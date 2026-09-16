use ark_ff::{AdditiveGroup, Field};
use ark_std::rand::{distributions::Standard, prelude::Distribution, CryptoRng, RngCore};
#[cfg(feature = "parallel")]
use rayon::prelude::*;
#[cfg(feature = "tracing")]
use tracing::instrument;

use super::{Config, Witness};
use crate::{
    algebra::{
        embedding::{Embedding, Identity},
        linear_form::LinearForm,
        ntt::{self, PolynomialSegment, Polynomials},
    },
    buffer::{Buffer, BufferMath, BufferOps},
    hash::Hash,
    protocols::{
        geometric_challenge::{geometric_challenge_buffer, geometric_challenge_groups},
        irs_commit,
        whir::FinalClaim,
    },
    transcript::{
        codecs::U64, Codec, Decoding, DuplexSpongeInterface, ProverMessage, ProverState,
        VerifierMessage,
    },
    utils::zip_strict,
};

enum RoundWitness<'a, F: Field, M: Embedding<Target = F>> {
    Initial(Vec<&'a Witness<F, M>>),
    Round(irs_commit::Witness<F>),
}

/// Materialize `sum_j scalars[j] * points[j]^i` using the fact that every
/// point is a power of one RS-domain generator. The sparse query weights are
/// treated as polynomial coefficients, so one NTT produces every required
/// power sum at once.
#[cfg_attr(feature = "tracing", instrument(skip_all, fields(
    points = points.len(),
    domain_length = domain.length,
    output_length = prefix_len,
)))]
fn sparse_domain_covector<F: Field>(
    points: &[F],
    domain: &irs_commit::EvaluationDomain<F>,
    scalars: &Buffer<F>,
    prefix_len: usize,
) -> Option<Buffer<F>> {
    if points.len() != scalars.len()
        || points.len() != domain.exponents.len()
        || prefix_len > domain.length
    {
        return None;
    }

    // Avoid an NTT when direct accumulation has a lower arithmetic count.
    let transform_cost = domain
        .length
        .saturating_mul(domain.length.next_power_of_two().ilog2() as usize)
        / 2
        + prefix_len;
    if transform_cost >= points.len().saturating_mul(prefix_len) {
        return None;
    }

    if domain.generator.is_none() {
        return ntt::NTT.get::<F>()?.additive_power_sums(
            points, &domain.exponents, scalars.to_slice(), domain.length, prefix_len,
        );
    }
    let generator = ntt::generator::<F>(domain.length);
    if Some(generator) != domain.generator {
        // This can happen when source-field RS points were embedded into an
        // extension whose registered NTT uses a different compatible root.
        return None;
    }
    debug_assert!(points
        .iter()
        .zip(&domain.exponents)
        .all(|(&point, &exponent)| generator.pow([exponent as u64]) == point));

    let sparse = {
        #[cfg(feature = "tracing")]
        let _span = tracing::trace_span!("sparse_domain_scatter").entered();
        let mut coefficients = vec![F::ZERO; domain.length];
        for (&exponent, &scalar) in domain.exponents.iter().zip(scalars.to_slice()) {
            coefficients[exponent] += scalar;
        }
        Buffer::from(coefficients)
    };

    let transformed = {
        #[cfg(feature = "tracing")]
        let _span = tracing::trace_span!("sparse_domain_ntt").entered();
        let sparse_refs = [&sparse];
        let segments = [PolynomialSegment::from_rows(&sparse_refs, 1)];
        let polynomials = Polynomials::from_segments(&segments);
        ntt::interleaved_rs_encode(polynomials, domain.length)
    };

    Some(Buffer::from(transformed.read_range(0..prefix_len)))
}

impl<M: Embedding> Config<M> {
    /// Prove a WHIR opening.
    ///
    /// * `prover_state` the mutable transcript to write the proof to.
    /// * `vectors` all the vectors we are opening.
    /// * `witnesses` witnesses corresponding to the `vectors`, in the same
    ///   order. Multiple vectors may share the same witness, in which case
    ///   only one witness should be provided.
    /// * `linear_forms` the covectors (if any) to evaluate each vector at.
    /// * `evaluations` a matrix of each vector evaluated at each linear form.
    ///
    /// The `evaluations` matrix is in row-major order with the number of rows
    /// equal to the `linear_forms.len()` and the number of columns equal to
    /// `vectors.len()`.
    ///
    #[cfg_attr(feature = "tracing", instrument(skip_all))]
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    pub fn prove<'a, H, R>(
        &self,
        prover_state: &mut ProverState<H, R>,
        vectors: &[&Buffer<M::Source>],
        witnesses: Vec<&'a Witness<M::Target, M>>,
        linear_forms: Vec<Box<dyn LinearForm<M::Target>>>,
        evaluations: Buffer<M::Target>,
    ) -> FinalClaim<M::Target>
    where
        Standard: Distribution<M::Source> + Distribution<M::Target>,
        H: DuplexSpongeInterface,
        R: RngCore + CryptoRng,
        M::Target: Codec<[H::U]>,
        [u8; 32]: Decoding<[H::U]>,
        U64: Codec<[H::U]>,
        u8: Decoding<[H::U]>,
        Hash: ProverMessage<[H::U]>,
    {
        let num_vectors = vectors.len();

        // Input validation
        assert_eq!(
            num_vectors,
            witnesses.len() * self.initial_committer.num_vectors()
        );
        assert_eq!(evaluations.len(), num_vectors * linear_forms.len());
        for vector in vectors {
            assert_eq!(vector.len(), self.initial_size());
        }
        for linear_form in &linear_forms {
            assert_eq!(linear_form.size(), self.initial_size());
        }
        #[cfg(debug_assertions)]
        for (linear_form, evaluations) in zip_strict(
            linear_forms.iter(),
            evaluations.to_slice().chunks_exact(num_vectors),
        ) {
            use crate::algebra::linear_form::Covector;
            let covector = Covector::from(&**linear_form);
            for (vector, evaluation) in zip_strict(vectors, evaluations) {
                debug_assert_eq!(
                    vector.mixed_dot(self.embedding(), &Buffer::from(covector.vector.as_slice())),
                    *evaluation
                );
            }
        }
        if vectors.is_empty() {
            // TODO: Should we draw a random evaluation point of the right size?
            return FinalClaim::default();
        }

        // Complete evaluations of EVERY vector at EVERY linear form.
        let (oods_evals, oods_matrix) = {
            let mut oods_points = Vec::new();
            let mut oods_cells = Vec::new();

            // Collect the matrix in transcript order first. Missing cross terms
            // are independent once all OOD points are known, so compute them in
            // parallel and only serialize the transcript writes afterward.
            let mut vector_offset = 0;
            for witness in &witnesses {
                for (oods_eval, oods_row) in zip_strict(
                    witness.out_of_domain.points.iter().copied(),
                    witness.out_of_domain.rows(),
                ) {
                    for (j, vector) in vectors.iter().enumerate() {
                        if j >= vector_offset && j < oods_row.len() + vector_offset {
                            debug_assert_eq!(
                                oods_row[j - vector_offset],
                                vector.mixed_univariate_evaluate(self.embedding(), oods_eval)
                            );
                            oods_cells.push((Some(oods_row[j - vector_offset]), j, oods_eval));
                        } else {
                            oods_cells.push((None, j, oods_eval));
                        }
                    }
                    oods_points.push(oods_eval);
                }
                vector_offset += witness.num_vectors();
            }

            #[cfg(feature = "parallel")]
            let oods_matrix = oods_cells
                .par_iter()
                .map(|(known, vector_index, point)| {
                    known.unwrap_or_else(|| {
                        vectors[*vector_index].mixed_univariate_evaluate(self.embedding(), *point)
                    })
                })
                .collect::<Vec<_>>();
            #[cfg(not(feature = "parallel"))]
            let oods_matrix = oods_cells
                .iter()
                .map(|(known, vector_index, point)| {
                    known.unwrap_or_else(|| {
                        vectors[*vector_index].mixed_univariate_evaluate(self.embedding(), *point)
                    })
                })
                .collect::<Vec<_>>();
            for ((known, _, _), value) in oods_cells.iter().zip(&oods_matrix) {
                if known.is_none() {
                    prover_state.prover_message(value);
                }
            }
            (oods_points, oods_matrix)
        };

        // Random linear combination of the vectors.
        let mut vector_rlc_coeffs = geometric_challenge_buffer(prover_state, num_vectors);
        let mut vector = Buffer::<M::Source>::mixed_linear_combination(
            self.embedding(),
            vectors,
            &vector_rlc_coeffs,
        );

        let mut prev_witness: RoundWitness<'a, M::Target, M> = RoundWitness::Initial(witnesses);

        // Random linear combination of the constraints. Split the geometric
        // challenge into a run for the linear forms followed by a contiguous
        // run for the OODS constraints; the verifier splits the same sequence.
        let total_constraints = linear_forms.len() + oods_evals.len();
        let has_constraints = total_constraints > 0;
        let mut rlc_groups = geometric_challenge_groups::<_, M::Target>(
            prover_state,
            &[linear_forms.len(), oods_evals.len()],
        )
        .into_iter();
        let initial_forms_rlc_coeffs = rlc_groups.next().unwrap();
        let oods_rlc_coeffs = rlc_groups.next().unwrap();

        let mut linear_forms = linear_forms;
        let mut covector = if has_constraints {
            Buffer::<M::Target>::linear_forms_rlc(
                self.initial_size(),
                &mut linear_forms,
                &initial_forms_rlc_coeffs,
            )
        } else {
            Buffer::<M::Target>::zeros(0)
        };
        drop(linear_forms);

        // Compute "The Sum": initial_forms_rlc_coeffsᵀ · evaluations · vector_rlc_coeffs
        let mut the_sum = evaluations.bilinear_form(&initial_forms_rlc_coeffs, &vector_rlc_coeffs);
        drop(evaluations);

        debug_assert!(!has_constraints || vector.dot(&covector) == the_sum);

        // Add OODS constraints
        covector.accumulate_geometric(&oods_evals, &oods_rlc_coeffs, self.initial_size());
        let oods_matrix = Buffer::from(oods_matrix);
        the_sum += oods_matrix.bilinear_form(&oods_rlc_coeffs, &vector_rlc_coeffs);
        drop(oods_evals);
        drop(oods_matrix);

        debug_assert!(!has_constraints || vector.dot(&covector) == the_sum);

        // Run initial sumcheck on batched vectors with combined statement
        let mut folding_randomness = if has_constraints {
            let (folded, opening) = self.initial_sumcheck.prove(
                prover_state,
                &Identity::new(),
                vector,
                &mut covector,
                &mut the_sum,
                &[],
            );
            vector = folded;
            opening.round_challenges
        } else {
            // There are no constraints yet, so we can skip the sumcheck.
            // (If we did run it, all sumcheck vectors would be constant zero)
            // TODO: Don't compute evaluations and constraints in the first place.
            let folding_randomness = (0..self.initial_sumcheck.num_rounds())
                .map(|_| prover_state.verifier_message())
                .collect();
            self.initial_skip_pow.prove(prover_state);
            // Fold vector
            for &f in &folding_randomness {
                vector.fold(f);
            }
            // Covector must be all zeros.
            covector = Buffer::<M::Target>::zeros(self.initial_sumcheck.final_size());
            folding_randomness
        };
        let mut evaluation_point = folding_randomness.clone();

        debug_assert_eq!(vector.dot(&covector), the_sum);

        // Execute standard WHIR rounds on the batched vectors
        for (round_index, round_config) in self.round_configs.iter().enumerate() {
            // Commit to the folded vector and run the per-round OOD step.
            let (new_witness, out_of_domain) = round_config.irs_committer.commit_with_ood(
                prover_state,
                &[&vector],
                round_config.out_domain_samples,
            );

            // Proof of work before in-domain challenges
            round_config.pow.prove(prover_state);

            // Open the previous round's witness.
            let in_domain = match prev_witness {
                RoundWitness::Initial(init_witnesses) => {
                    let irs_refs: Vec<&_> = init_witnesses.iter().map(|c| &c.irs).collect();
                    self.initial_committer
                        .open(prover_state, &irs_refs)
                        .lift(self.embedding())
                }
                RoundWitness::Round(old_witness) => {
                    let prev_round_config = &self.round_configs[round_index - 1];
                    prev_round_config
                        .irs_committer
                        .open(prover_state, &[&old_witness])
                }
            };

            // Collect constraints for this round and RLC them in
            let stir_points = out_of_domain
                .points
                .iter()
                .chain(&in_domain.points)
                .copied()
                .collect::<Vec<_>>();
            // Weights for the in-domain rows: vector_rlc_coeffs ⊗ eq(folding_randomness),
            // built directly on the backend so no readback is needed.
            let stir_weights =
                vector_rlc_coeffs.tensor_product(&Buffer::eq_weights(&folding_randomness));
            let stir_evaluations = out_of_domain
                .values_buffer(&Identity::new(), &Buffer::ones(1))
                .concat(&in_domain.values_buffer(&Identity::new(), &stir_weights));
            let stir_rlc_coeffs = geometric_challenge_buffer(prover_state, stir_points.len());
            let out_count = out_of_domain.points.len();
            let (out_scalars, in_scalars) = stir_rlc_coeffs.to_slice().split_at(out_count);
            covector.accumulate_geometric(
                &out_of_domain.points,
                &Buffer::from(out_scalars),
                round_config.initial_size(),
            );
            let in_scalars = Buffer::from(in_scalars);
            let domain_covector = in_domain.domain.as_ref().and_then(|domain| {
                sparse_domain_covector(
                    &in_domain.points,
                    domain,
                    &in_scalars,
                    round_config.initial_size(),
                )
            });
            if let Some(domain_covector) = domain_covector {
                #[cfg(feature = "tracing")]
                let _span = tracing::trace_span!("sparse_domain_accumulate").entered();
                #[cfg(feature = "parallel")]
                let combined = covector
                    .to_slice()
                    .par_iter()
                    .zip(domain_covector.to_slice().par_iter())
                    .map(|(&left, &right)| left + right)
                    .collect::<Vec<_>>();
                #[cfg(not(feature = "parallel"))]
                let combined = covector
                    .to_slice()
                    .iter()
                    .zip(domain_covector.to_slice())
                    .map(|(&left, &right)| left + right)
                    .collect::<Vec<_>>();
                covector = Buffer::from(combined);
            } else {
                covector.accumulate_geometric(
                    &in_domain.points,
                    &in_scalars,
                    round_config.initial_size(),
                );
            }
            the_sum += stir_rlc_coeffs.dot(&stir_evaluations);
            debug_assert_eq!(vector.dot(&covector), the_sum);

            // Run sumcheck for this round
            let (folded, opening) = round_config.sumcheck.prove(
                prover_state,
                &Identity::new(),
                vector,
                &mut covector,
                &mut the_sum,
                &[],
            );
            vector = folded;
            folding_randomness = opening.round_challenges;

            evaluation_point.extend(folding_randomness.iter().copied());
            debug_assert_eq!(vector.dot(&covector), the_sum);

            prev_witness = RoundWitness::Round(new_witness);
            vector_rlc_coeffs = Buffer::ones(1);
        }

        // Directly send the vector to the verifier.
        assert_eq!(vector.len(), self.final_sumcheck.initial_size());
        for coeff in vector.to_slice() {
            prover_state.prover_message(coeff);
        }

        // PoW
        self.final_pow.prove(prover_state);

        // Open and consume the final previous witness.
        match prev_witness {
            RoundWitness::Initial(init_witnesses) => {
                let irs_refs: Vec<&_> = init_witnesses.iter().map(|c| &c.irs).collect();
                let _in_domain = self.initial_committer.open(prover_state, &irs_refs);
            }
            RoundWitness::Round(old_witness) => {
                let prev_config = self.round_configs.last().unwrap();
                let _in_domain = prev_config
                    .irs_committer
                    .open(prover_state, &[&old_witness]);
            }
        }

        // Final sumcheck
        let (_, opening) = self.final_sumcheck.prove(
            prover_state,
            &Identity::new(),
            vector,
            &mut covector,
            &mut the_sum,
            &[],
        );
        let final_folding_randomness = opening.round_challenges;
        evaluation_point.extend(final_folding_randomness.iter().copied());

        FinalClaim {
            evaluation_point,
            rlc_coefficients: initial_forms_rlc_coeffs.to_slice().to_vec(),
            linear_form_rlc: M::Target::ZERO,
            polynomial_evaluation: M::Target::ONE,
        }
    }
}

#[cfg(test)]
mod sparse_domain_tests {
    use super::*;
    use crate::algebra::{fields::Field256, geometric_accumulate};

    #[test]
    fn sparse_ntt_matches_geometric_accumulation() {
        let prefix_len = 32;
        let domain_length = 64;
        let indices = (0..32)
            .map(|index| index * 2 % domain_length)
            .collect::<Vec<_>>();
        let points = ntt::evaluation_points::<Field256>(prefix_len, domain_length, &indices);
        let exponents = ntt::evaluation_exponents::<Field256>(prefix_len, domain_length, &indices);
        let scalars = (1..=indices.len() as u64)
            .map(Field256::from)
            .collect::<Vec<_>>();
        let domain = irs_commit::EvaluationDomain {
            length: domain_length,
            generator: Some(ntt::generator::<Field256>(domain_length)),
            exponents,
        };

        let actual = sparse_domain_covector(
            &points,
            &domain,
            &Buffer::from(scalars.as_slice()),
            prefix_len,
        )
        .expect("the NTT path should be selected");
        let mut expected = vec![Field256::ZERO; prefix_len];
        geometric_accumulate(&mut expected, scalars, &points);
        assert_eq!(actual.to_slice(), expected);
    }
}
