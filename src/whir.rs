//! Concrete WHIR proof-size estimation for the proved Johnson-bound and
//! conjectural capacity-bound regimes.
//!
//! The radius is `1 - sqrt(rho) - eta`, with the reference implementation's
//! conservative canonical slack `eta = sqrt(rho) / 20`. The capacity variant
//! follows STIR Conjecture 5.6 with `eta = rho / 20`. No proof of work is used.

use std::collections::BTreeSet;

use anyhow::{ensure, Context, Result};
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::Serialize;

use crate::merkle::frontier_hash_count;
use crate::HASH_BYTES;

/// Algebraic and hash-collision security target for the paper experiments.
pub const DEFAULT_SECURITY_BITS: usize = 128;
pub const DEFAULT_FIELD_BITS: usize = 256;
pub const DEFAULT_FIELD_LOG2_SIZE: f64 = FIELD_256.log2_size;
pub const DEFAULT_FOLDING_LOG: usize = 4;
pub const DEFAULT_FOLDING_FACTOR: usize = 1 << DEFAULT_FOLDING_LOG;
/// The executable reference keeps folding while at least one full four-variable
/// fold remains, so the terminal polynomial has at most three variables.
pub const MAX_TERMINAL_MESSAGE_VARIABLES: usize = DEFAULT_FOLDING_LOG - 1;

/// Reed-Solomon proximity regime used by WHIR.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WhirDecodingRegime {
    /// Proved Johnson-bound list decoding.
    Johnson,
    /// Capacity-bound list decoding under STIR Conjecture 5.6.
    Capacity,
}

impl WhirDecodingRegime {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Johnson => "johnson",
            Self::Capacity => "capacity_conjecture",
        }
    }

    pub const fn is_conjectural(self) -> bool {
        matches!(self, Self::Capacity)
    }
}

/// A concrete prime-field profile used by the proof-size estimator.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct WhirFieldProfile {
    pub bits: usize,
    pub modulus: &'static str,
    pub log2_size: f64,
    pub element_bytes: usize,
    pub two_adicity: usize,
    pub domain_kind: &'static str,
    pub max_domain_log: usize,
}

/// Idealized `GF(2^24)` profile for additive, foldable RS domains. The
/// experiment restricts each block to at most `2^23`, strictly below the field
/// cardinality.
pub const FIELD_24: WhirFieldProfile = WhirFieldProfile {
    bits: 24,
    modulus: "GF(2^24)",
    log2_size: 24.0,
    element_bytes: 3,
    two_adicity: 0,
    domain_kind: "additive binary-extension-field",
    max_domain_log: 23,
};

pub const FIELD_32: WhirFieldProfile = WhirFieldProfile {
    bits: 32,
    modulus: "3221225473",
    log2_size: 31.584_962_501_169_027,
    element_bytes: 4,
    two_adicity: 30,
    domain_kind: "multiplicative radix-2",
    max_domain_log: 30,
};

pub const FIELD_48: WhirFieldProfile = WhirFieldProfile {
    bits: 48,
    modulus: "281467460517889",
    log2_size: 47.999_961_475_371_85,
    element_bytes: 6,
    two_adicity: 30,
    domain_kind: "multiplicative radix-2",
    max_domain_log: 30,
};

pub const FIELD_64: WhirFieldProfile = WhirFieldProfile {
    bits: 64,
    modulus: "18446744069414584321",
    log2_size: 63.999_999_999_664_1,
    element_bytes: 8,
    two_adicity: 32,
    domain_kind: "multiplicative radix-2",
    max_domain_log: 32,
};

/// Concrete 128-bit prime field used by the executable Canopy path and the
/// default WHIR estimator. It has modulus
/// `237684487542793012780631866933 * 2^30 + 1`.
pub const FIELD_128: WhirFieldProfile = WhirFieldProfile {
    bits: 128,
    modulus: "255211775190703847597530972673164705793",
    log2_size: 127.584_962_500_721_16,
    element_bytes: 16,
    two_adicity: 30,
    domain_kind: "multiplicative radix-2",
    max_domain_log: 30,
};

/// The executable field selected by the crate containing this shared module.
pub const FIELD_256: WhirFieldProfile = WhirFieldProfile {
    bits: 256,
    modulus: crate::FIELD_MODULUS_LABEL,
    log2_size: 256.0,
    element_bytes: 32,
    two_adicity: crate::FIELD_TWO_ADICITY,
    domain_kind: crate::FIELD_DOMAIN_KIND,
    max_domain_log: crate::FIELD_MAX_DOMAIN_LOG,
};

pub fn field_profile(bits: usize, codeword_log_length: usize) -> Result<WhirFieldProfile> {
    let profile = match bits {
        24 => FIELD_24,
        32 => FIELD_32,
        48 => FIELD_48,
        64 => FIELD_64,
        128 => FIELD_128,
        256 => FIELD_256,
        16 => {
            anyhow::bail!("a 16-bit field cannot contain a 2^{codeword_log_length} radix-2 domain")
        }
        _ => anyhow::bail!("no concrete WHIR field profile configured for {bits} bits"),
    };
    ensure!(
        profile.max_domain_log >= codeword_log_length,
        "the {}-bit field supports {} domains only through log length {}, below the required {codeword_log_length}",
        profile.bits,
        profile.domain_kind,
        profile.max_domain_log,
    );
    Ok(profile)
}

#[derive(Clone, Debug, Serialize)]
pub struct WhirRoundParameters {
    pub round: usize,
    pub message_variables: usize,
    pub log_inverse_rate_before: usize,
    pub log_inverse_rate_after: usize,
    pub requested_queries: usize,
    pub query_distance: f64,
    pub query_security_bits_per_query: f64,
    pub ood_samples: usize,
    pub combination_security_bits_per_challenge: f64,
    /// Variables folded after committing the new oracle in this round.
    pub folding_log: usize,
    pub folding_factor: usize,
    pub folding_security_bits_per_challenge: f64,
    /// Number of degree-2 sumcheck transcripts in this fold. The current
    /// 256-bit-field model fixes this to one. Verifier challenges are not
    /// serialized.
    pub sumcheck_repetitions: usize,
    /// Width of a leaf in the previous oracle opened by this round.
    pub query_leaf_width: usize,
    pub oracle_leaf_count: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct WhirRbrEvent {
    pub event: String,
    pub security_bits: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct WhirParameters {
    pub decoding_regime: WhirDecodingRegime,
    pub max_terminal_message_variables: usize,
    pub security_bits: usize,
    pub field_bits: usize,
    pub field_modulus: &'static str,
    pub field_log2_size: f64,
    pub field_bytes: usize,
    pub field_two_adicity: usize,
    pub message_variables: usize,
    pub log_inverse_rate: usize,
    pub codeword_log_length: usize,
    pub folding_log: usize,
    pub folding_factor: usize,
    /// Initial and recursive folding widths in transcript order. Empty means
    /// the committed polynomial is already at the terminal size.
    pub folding_schedule: Vec<usize>,
    /// Logarithmic shrinkage from the initial codeword to the first
    /// recursively committed codeword. Standard WHIR uses one; the
    /// zero-shortening Canopy alignment uses zero.
    pub initial_codeword_log_reduction: usize,
    pub terminal_message_variables: usize,
    pub hash_bits: usize,
    pub hash_collision_security_bits: usize,
    pub effective_security_bits: usize,
    pub commitment_ood_samples: usize,
    pub initial_folding_security_bits_per_challenge: f64,
    pub initial_sumcheck_repetitions: usize,
    pub rounds: Vec<WhirRoundParameters>,
    pub final_requested_queries: usize,
    pub final_query_distance: f64,
    pub final_query_security_bits_per_query: f64,
    pub final_sumcheck_rounds: usize,
    pub final_sumcheck_security_bits_per_challenge: f64,
    pub final_sumcheck_repetitions: usize,
    pub final_log_inverse_rate: usize,
    pub final_oracle_leaf_width: usize,
    pub final_oracle_leaf_count: usize,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct WhirProofSize {
    pub hash_count: usize,
    pub field_count: usize,
    pub intermediate_roots: usize,
    pub merkle_auth_hashes: usize,
    pub query_answer_fields: usize,
    pub ood_fields: usize,
    pub sumcheck_fields: usize,
    /// Auxiliary field elements used to batch claims before the recursive
    /// WHIR proof. Ordinary and parallel-interleaved proofs use none.
    pub batching_fields: usize,
    pub final_coefficient_fields: usize,
    pub unique_queries_by_oracle: Vec<usize>,
    pub auth_hashes_by_oracle: Vec<usize>,
    pub field_bytes_per_element: usize,
}

/// A WHIR proof whose initial oracle is virtual and bound by external
/// commitments. The caller authenticates `initial_query_indices` against
/// those commitments and supplies the queried values to the verifier.
#[derive(Clone, Debug)]
pub struct VirtualInitialWhirProofSize {
    pub proof: WhirProofSize,
    pub initial_query_indices: Vec<usize>,
}

impl WhirProofSize {
    pub fn hash_bytes(&self) -> usize {
        self.hash_count * HASH_BYTES
    }

    pub fn field_bytes(&self) -> usize {
        self.field_count * self.field_bytes_per_element
    }

    pub fn total_bytes(&self) -> usize {
        self.hash_bytes() + self.field_bytes()
    }
}

fn effective_security_bits() -> usize {
    DEFAULT_SECURITY_BITS.min(HASH_BYTES * 8 / 2)
}

fn log_eta(regime: WhirDecodingRegime, log_inverse_rate: usize) -> f64 {
    match regime {
        // eta = sqrt(rho) / 20.
        WhirDecodingRegime::Johnson => -(0.5 * log_inverse_rate as f64 + 20.0_f64.log2()),
        // eta = rho / 20.
        WhirDecodingRegime::Capacity => -(log_inverse_rate as f64 + 20.0_f64.log2()),
    }
}

fn list_size_bits(
    regime: WhirDecodingRegime,
    message_variables: usize,
    log_inverse_rate: usize,
) -> f64 {
    match regime {
        // Johnson: |Lambda| = 1 / (2 eta sqrt(rho)) = 10 / rho.
        WhirDecodingRegime::Johnson => {
            0.5 * log_inverse_rate as f64 - (1.0 + log_eta(regime, log_inverse_rate))
        }
        // Capacity (STIR Conj. 5.6): |Lambda| = d / (rho eta).
        WhirDecodingRegime::Capacity => {
            message_variables as f64 + log_inverse_rate as f64 - log_eta(regime, log_inverse_rate)
        }
    }
}

fn rbr_ood_sample(
    regime: WhirDecodingRegime,
    message_variables: usize,
    log_inverse_rate: usize,
    field_log2_size: f64,
    samples: usize,
) -> f64 {
    let list = list_size_bits(regime, message_variables, log_inverse_rate);
    let error = 2.0 * list + (message_variables * samples) as f64;
    samples as f64 * field_log2_size + 1.0 - error
}

fn ood_samples(
    regime: WhirDecodingRegime,
    security_bits: usize,
    message_variables: usize,
    log_inverse_rate: usize,
    field_log2_size: f64,
) -> usize {
    let list = list_size_bits(regime, message_variables, log_inverse_rate);
    let gain_per_sample = field_log2_size - message_variables as f64;
    assert!(
        gain_per_sample > 0.0,
        "OOD repetition cannot amplify when the field is no larger than the message degree"
    );
    (((security_bits as f64 - 1.0 + 2.0 * list) / gain_per_sample).ceil() as usize).max(1)
}

fn folding_security_bits_per_challenge(
    regime: WhirDecodingRegime,
    field_log2_size: f64,
    message_variables: usize,
    log_inverse_rate: usize,
) -> f64 {
    let list = list_size_bits(regime, message_variables, log_inverse_rate);
    let prox_gaps_error = match regime {
        // BCSS25, Theorem 4.6. Here gamma is the agreement gap used by
        // WHIR, M=1 because the folding curve is linear, and n is the RS
        // block length. We retain eta=sqrt(rho)/20, hence m=20.
        WhirDecodingRegime::Johnson => {
            field_log2_size - johnson_exceptional_set_log2(message_variables, log_inverse_rate)
        }
        // STIR Conj. 5.6: eps ~= d / (eta rho^2 |F|), eta = rho/20.
        WhirDecodingRegime::Capacity => {
            field_log2_size
                - (message_variables as f64 + 3.0 * log_inverse_rate as f64 + 10.0_f64.log2() + 1.0)
        }
    };
    let sumcheck_error = field_log2_size - (list + 1.0);
    prox_gaps_error.min(sumcheck_error)
}

fn johnson_exceptional_set_log2(message_variables: usize, log_inverse_rate: usize) -> f64 {
    let rho = 2.0_f64.powi(-(log_inverse_rate as i32));
    let sqrt_rho = rho.sqrt();
    let eta = sqrt_rho / 20.0;
    let gamma = 1.0 - sqrt_rho - eta;
    let multiplicity = (sqrt_rho / eta).ceil().max(3.0);
    let shifted = multiplicity + 0.5;
    let block_length = 2.0_f64.powi((message_variables + log_inverse_rate) as i32);
    let leading = (2.0 * shifted.powi(5) + 3.0 * shifted * gamma * rho) / (3.0 * rho.powf(1.5))
        * block_length;
    let additive = shifted / sqrt_rho;
    (leading + additive).log2()
}

fn query_distance(regime: WhirDecodingRegime, log_inverse_rate: usize) -> f64 {
    let rate = 2.0_f64.powi(-(log_inverse_rate as i32));
    match regime {
        WhirDecodingRegime::Johnson => 1.0 - rate.sqrt() - rate.sqrt() / 20.0,
        WhirDecodingRegime::Capacity => 1.0 - rate - rate / 20.0,
    }
}

fn query_security_bits_per_query(regime: WhirDecodingRegime, log_inverse_rate: usize) -> f64 {
    let distance = query_distance(regime, log_inverse_rate);
    -(1.0 - distance).log2()
}

fn queries(regime: WhirDecodingRegime, security_bits: usize, log_inverse_rate: usize) -> usize {
    (security_bits as f64 / query_security_bits_per_query(regime, log_inverse_rate)).ceil() as usize
}

fn query_combination_security(
    regime: WhirDecodingRegime,
    field_log2_size: f64,
    message_variables: usize,
    log_inverse_rate: usize,
    ood_samples: usize,
    num_queries: usize,
) -> f64 {
    let list = list_size_bits(regime, message_variables, log_inverse_rate);
    let log_combination = ((ood_samples + num_queries) as f64).log2();
    field_log2_size - (log_combination + list + 1.0)
}

impl WhirParameters {
    /// Conservative correlated-list exponent used by the concrete RBR model
    /// at this invocation's initial code and proximity radius.
    pub fn initial_list_size_bits(&self) -> f64 {
        list_size_bits(
            self.decoding_regime,
            self.message_variables,
            self.log_inverse_rate,
        )
    }

    /// Concrete bad-event exponents used by the round-by-round model.
    pub fn rbr_security_events(&self) -> Vec<WhirRbrEvent> {
        let mut events = Vec::new();
        let mut message_variables = self.message_variables;
        let mut log_inverse_rate = self.log_inverse_rate;

        if !self.folding_schedule.is_empty() {
            events.push(WhirRbrEvent {
                event: "initial folding / sumcheck".to_owned(),
                security_bits: self.initial_folding_security_bits_per_challenge,
            });
            events.push(WhirRbrEvent {
                event: "initial OOD sampling".to_owned(),
                security_bits: rbr_ood_sample(
                    self.decoding_regime,
                    message_variables,
                    log_inverse_rate,
                    self.field_log2_size,
                    self.commitment_ood_samples,
                ),
            });
        }

        for (oracle, requested_queries) in self.all_query_counts().into_iter().enumerate() {
            let query_bits = if oracle < self.rounds.len() {
                self.rounds[oracle].query_security_bits_per_query
            } else {
                self.final_query_security_bits_per_query
            };
            events.push(WhirRbrEvent {
                event: format!("oracle {oracle} proximity queries"),
                security_bits: requested_queries as f64 * query_bits,
            });
        }

        for round in &self.rounds {
            message_variables = round.message_variables;
            log_inverse_rate = round.log_inverse_rate_after;
            events.push(WhirRbrEvent {
                event: format!("recursive round {} folding / sumcheck", round.round + 1),
                security_bits: round.folding_security_bits_per_challenge,
            });
            events.push(WhirRbrEvent {
                event: format!("recursive round {} claim combination", round.round + 1),
                security_bits: round.combination_security_bits_per_challenge,
            });
            events.push(WhirRbrEvent {
                event: format!("recursive round {} OOD sampling", round.round + 1),
                security_bits: rbr_ood_sample(
                    self.decoding_regime,
                    message_variables,
                    log_inverse_rate,
                    self.field_log2_size,
                    round.ood_samples,
                ),
            });
        }
        events.push(WhirRbrEvent {
            event: "terminal sumcheck".to_owned(),
            security_bits: self.final_sumcheck_security_bits_per_challenge,
        });
        events
    }

    /// Initial-transition events only. This is useful when a caller replaces
    /// all later rounds with a separately parameterized recursive proof.
    pub fn initial_rbr_security_events(&self) -> Vec<WhirRbrEvent> {
        self.rbr_security_events()
            .into_iter()
            .filter(|event| {
                event.event == "initial folding / sumcheck"
                    || event.event == "initial OOD sampling"
                    || event.event == "oracle 0 proximity queries"
            })
            .collect()
    }

    /// Smallest concrete round-by-round security exponent recorded for this
    /// invocation. Query, OOD, folding, claim-combination, and terminal
    /// sumcheck events are included. This is a per-event minimum, not an
    /// end-to-end union bound.
    pub fn minimum_rbr_security_bits(&self) -> f64 {
        self.rbr_security_events()
            .into_iter()
            .map(|event| event.security_bits)
            .reduce(f64::min)
            .unwrap_or(f64::INFINITY)
    }

    /// Create the Canopy-specific WHIR configuration.
    ///
    /// `message_variables = n-h`, `log_inverse_rate = h`, and hence the
    /// committed RS codeword has logarithmic length `n`.
    pub fn johnson_bound(message_variables: usize, log_inverse_rate: usize) -> Result<Self> {
        Self::with_regime(
            message_variables,
            log_inverse_rate,
            WhirDecodingRegime::Johnson,
        )
    }

    pub fn capacity_bound(message_variables: usize, log_inverse_rate: usize) -> Result<Self> {
        Self::with_regime(
            message_variables,
            log_inverse_rate,
            WhirDecodingRegime::Capacity,
        )
    }

    pub fn with_regime(
        message_variables: usize,
        log_inverse_rate: usize,
        decoding_regime: WhirDecodingRegime,
    ) -> Result<Self> {
        let codeword_log_length = message_variables + log_inverse_rate;
        let field = field_profile(DEFAULT_FIELD_BITS, codeword_log_length)?;
        Self::with_regime_and_field(message_variables, log_inverse_rate, field, decoding_regime)
    }

    pub fn with_regime_and_terminal(
        message_variables: usize,
        log_inverse_rate: usize,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
    ) -> Result<Self> {
        let codeword_log_length = message_variables + log_inverse_rate;
        let field = field_profile(DEFAULT_FIELD_BITS, codeword_log_length)?;
        Self::with_regime_and_field_and_terminal(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            max_terminal_message_variables,
        )
    }

    /// Construct WHIR with the same selected folding width in the initial
    /// and every recursive folding round.
    pub fn with_regime_and_terminal_and_folding(
        message_variables: usize,
        log_inverse_rate: usize,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
        folding_log: usize,
    ) -> Result<Self> {
        let codeword_log_length = message_variables + log_inverse_rate;
        let field = field_profile(DEFAULT_FIELD_BITS, codeword_log_length)?;
        Self::with_regime_and_field_and_terminal_and_folding(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            max_terminal_message_variables,
            folding_log,
        )
    }

    /// Construct WHIR with an independently selected initial folding width.
    /// Recursive folding rounds continue to use [`DEFAULT_FOLDING_LOG`].
    pub fn with_regime_and_terminal_and_initial_folding(
        message_variables: usize,
        log_inverse_rate: usize,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
        initial_folding_log: usize,
    ) -> Result<Self> {
        let codeword_log_length = message_variables + log_inverse_rate;
        let field = field_profile(DEFAULT_FIELD_BITS, codeword_log_length)?;
        Self::with_regime_and_field_and_terminal_and_initial_folding(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            max_terminal_message_variables,
            initial_folding_log,
        )
    }

    /// Construct WHIR with independently selected first-fold width and first
    /// codeword shrinkage. Recursive rounds use the standard one-bit
    /// codeword shrinkage and [`DEFAULT_FOLDING_LOG`].
    pub fn with_regime_and_terminal_and_initial_transition(
        message_variables: usize,
        log_inverse_rate: usize,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
        initial_folding_log: usize,
        initial_codeword_log_reduction: usize,
    ) -> Result<Self> {
        let codeword_log_length = message_variables + log_inverse_rate;
        let field = field_profile(DEFAULT_FIELD_BITS, codeword_log_length)?;
        Self::with_regime_and_field_and_terminal_and_initial_transition(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            max_terminal_message_variables,
            initial_folding_log,
            initial_codeword_log_reduction,
        )
    }

    /// Construct WHIR with independently selected first-fold width, first
    /// codeword shrinkage, and recursive folding width.
    #[allow(clippy::too_many_arguments)]
    pub fn with_regime_and_terminal_and_initial_transition_and_recursive_folding(
        message_variables: usize,
        log_inverse_rate: usize,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
        initial_folding_log: usize,
        initial_codeword_log_reduction: usize,
        recursive_folding_log: usize,
    ) -> Result<Self> {
        let codeword_log_length = message_variables + log_inverse_rate;
        let field = field_profile(DEFAULT_FIELD_BITS, codeword_log_length)?;
        Self::with_regime_and_field_and_terminal_and_initial_transition_and_recursive_folding(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            max_terminal_message_variables,
            initial_folding_log,
            initial_codeword_log_reduction,
            recursive_folding_log,
        )
    }

    /// Construct WHIR from an explicit folding schedule and require that it
    /// terminates at exactly `terminal_message_variables`.
    pub fn with_regime_and_terminal_and_schedule(
        message_variables: usize,
        log_inverse_rate: usize,
        decoding_regime: WhirDecodingRegime,
        terminal_message_variables: usize,
        folding_schedule: &[usize],
        initial_codeword_log_reduction: usize,
    ) -> Result<Self> {
        let codeword_log_length = message_variables + log_inverse_rate;
        let field = field_profile(DEFAULT_FIELD_BITS, codeword_log_length)?;
        Self::with_regime_and_field_and_terminal_and_schedule(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            terminal_message_variables,
            folding_schedule,
            initial_codeword_log_reduction,
        )
    }

    pub fn johnson_bound_with_field(
        message_variables: usize,
        log_inverse_rate: usize,
        field: WhirFieldProfile,
    ) -> Result<Self> {
        Self::with_regime_and_field(
            message_variables,
            log_inverse_rate,
            field,
            WhirDecodingRegime::Johnson,
        )
    }

    pub fn capacity_bound_with_field(
        message_variables: usize,
        log_inverse_rate: usize,
        field: WhirFieldProfile,
    ) -> Result<Self> {
        Self::with_regime_and_field(
            message_variables,
            log_inverse_rate,
            field,
            WhirDecodingRegime::Capacity,
        )
    }

    pub fn with_regime_and_field(
        message_variables: usize,
        log_inverse_rate: usize,
        field: WhirFieldProfile,
        decoding_regime: WhirDecodingRegime,
    ) -> Result<Self> {
        Self::with_regime_and_field_and_terminal(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            MAX_TERMINAL_MESSAGE_VARIABLES,
        )
    }

    pub fn with_regime_and_field_and_terminal(
        message_variables: usize,
        log_inverse_rate: usize,
        field: WhirFieldProfile,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
    ) -> Result<Self> {
        Self::with_regime_and_field_and_terminal_and_initial_folding(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            max_terminal_message_variables,
            DEFAULT_FOLDING_LOG,
        )
    }

    pub fn with_regime_and_field_and_terminal_and_folding(
        message_variables: usize,
        log_inverse_rate: usize,
        field: WhirFieldProfile,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
        folding_log: usize,
    ) -> Result<Self> {
        Self::with_regime_and_field_and_terminal_and_initial_transition_and_recursive_folding(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            max_terminal_message_variables,
            folding_log,
            1,
            folding_log,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_regime_and_field_and_terminal_and_schedule(
        message_variables: usize,
        log_inverse_rate: usize,
        field: WhirFieldProfile,
        decoding_regime: WhirDecodingRegime,
        terminal_message_variables: usize,
        folding_schedule: &[usize],
        initial_codeword_log_reduction: usize,
    ) -> Result<Self> {
        Self::with_regime_and_field_and_schedule_internal(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            terminal_message_variables,
            folding_schedule,
            initial_codeword_log_reduction,
            true,
        )
    }

    pub fn with_regime_and_field_and_terminal_and_initial_folding(
        message_variables: usize,
        log_inverse_rate: usize,
        field: WhirFieldProfile,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
        initial_folding_log: usize,
    ) -> Result<Self> {
        Self::with_regime_and_field_and_terminal_and_initial_transition(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            max_terminal_message_variables,
            initial_folding_log,
            1,
        )
    }

    pub fn with_regime_and_field_and_terminal_and_initial_transition(
        message_variables: usize,
        log_inverse_rate: usize,
        field: WhirFieldProfile,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
        initial_folding_log: usize,
        initial_codeword_log_reduction: usize,
    ) -> Result<Self> {
        Self::with_regime_and_field_and_terminal_and_initial_transition_and_recursive_folding(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            max_terminal_message_variables,
            initial_folding_log,
            initial_codeword_log_reduction,
            DEFAULT_FOLDING_LOG,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_regime_and_field_and_terminal_and_initial_transition_and_recursive_folding(
        message_variables: usize,
        log_inverse_rate: usize,
        field: WhirFieldProfile,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
        initial_folding_log: usize,
        initial_codeword_log_reduction: usize,
        recursive_folding_log: usize,
    ) -> Result<Self> {
        ensure!(initial_folding_log > 0, "initial fold must be nonempty");
        ensure!(recursive_folding_log > 0, "recursive fold must be nonempty");
        ensure!(
            max_terminal_message_variables + 1 >= recursive_folding_log,
            "terminal cap must stop before a recursive fold becomes too wide"
        );
        ensure!(
            message_variables >= initial_folding_log,
            "message must contain enough variables for the initial fold"
        );
        let mut folding_schedule = vec![initial_folding_log];
        let mut remaining_variables = message_variables - initial_folding_log;
        while remaining_variables > max_terminal_message_variables {
            ensure!(
                remaining_variables >= recursive_folding_log,
                "recursive fold exceeds the remaining message"
            );
            folding_schedule.push(recursive_folding_log);
            remaining_variables -= recursive_folding_log;
        }
        Self::with_regime_and_field_and_schedule_internal(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            max_terminal_message_variables,
            &folding_schedule,
            initial_codeword_log_reduction,
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn with_regime_and_field_and_schedule_internal(
        message_variables: usize,
        log_inverse_rate: usize,
        field: WhirFieldProfile,
        decoding_regime: WhirDecodingRegime,
        max_terminal_message_variables: usize,
        folding_schedule: &[usize],
        initial_codeword_log_reduction: usize,
        require_exact_terminal: bool,
    ) -> Result<Self> {
        ensure!(
            folding_schedule.iter().all(|&fold| fold > 0),
            "folding widths must be positive"
        );
        let folded_variables = folding_schedule
            .iter()
            .try_fold(0usize, |sum, &fold| sum.checked_add(fold))
            .context("folding schedule length overflow")?;
        ensure!(
            folded_variables <= message_variables,
            "folding schedule exceeds the message"
        );
        let terminal_message_variables = message_variables - folded_variables;
        if require_exact_terminal {
            ensure!(
                terminal_message_variables == max_terminal_message_variables,
                "folding schedule terminates at 2^{terminal_message_variables}, not exactly 2^{max_terminal_message_variables}"
            );
        } else {
            ensure!(
                terminal_message_variables <= max_terminal_message_variables,
                "folding schedule does not reach the terminal cap"
            );
        }
        ensure!(log_inverse_rate > 0, "WHIR rate must be below one");
        if let Some(&initial_folding_log) = folding_schedule.first() {
            ensure!(
                initial_codeword_log_reduction < log_inverse_rate + initial_folding_log,
                "first recursive codeword must have rate below one"
            );
        }
        let codeword_log_length = message_variables + log_inverse_rate;
        ensure!(
            field.max_domain_log >= codeword_log_length,
            "the {}-bit field supports {} domains only through log length {}, below the required {codeword_log_length}",
            field.bits,
            field.domain_kind,
            field.max_domain_log,
        );
        ensure!(
            field.log2_size > message_variables as f64,
            "the field must be larger than the message degree for OOD amplification"
        );
        let security_bits = DEFAULT_SECURITY_BITS;
        let field_bits = field.bits;
        let field_log2_size = field.log2_size;
        let hash_bits = HASH_BYTES * 8;
        let hash_collision_security_bits = hash_bits / 2;
        let effective_security_bits = effective_security_bits();
        let has_initial_fold = !folding_schedule.is_empty();
        let commitment_ood_samples = if has_initial_fold {
            ood_samples(
                decoding_regime,
                effective_security_bits,
                message_variables,
                log_inverse_rate,
                field_log2_size,
            )
        } else {
            0
        };
        let initial_folding_security_bits_per_challenge = if has_initial_fold {
            folding_security_bits_per_challenge(
                decoding_regime,
                field_log2_size,
                message_variables,
                log_inverse_rate,
            )
        } else {
            f64::INFINITY
        };
        // One large-field sumcheck transcript. We deliberately do not model
        // parallel repetition: independent folds produce different recursive
        // oracles, so multiplying only the scalar sumcheck messages would
        // undercount the transcript.
        let initial_sumcheck_repetitions = usize::from(has_initial_fold);

        let mut rounds = Vec::new();
        let initial_folding_log = folding_schedule.first().copied().unwrap_or(0);
        let mut variables = message_variables - initial_folding_log;
        let mut rate = log_inverse_rate;
        let mut previous_folding_log = initial_folding_log;
        let mut previous_oracle_leaf_width = 1usize << initial_folding_log;
        let mut previous_oracle_leaf_count = 1usize << (codeword_log_length - previous_folding_log);
        for (round, &round_folding_log) in folding_schedule.iter().skip(1).enumerate() {
            let codeword_log_reduction = if round == 0 {
                initial_codeword_log_reduction
            } else {
                1
            };
            let next_rate = rate + previous_folding_log - codeword_log_reduction;
            let query_distance = query_distance(decoding_regime, rate);
            let query_security_bits_per_query =
                query_security_bits_per_query(decoding_regime, rate);
            let requested_queries = queries(decoding_regime, effective_security_bits, rate);
            let round_ood = ood_samples(
                decoding_regime,
                effective_security_bits,
                variables,
                next_rate,
                field_log2_size,
            );
            let combination_security_bits_per_challenge = query_combination_security(
                decoding_regime,
                field_log2_size,
                variables,
                next_rate,
                round_ood,
                requested_queries,
            );
            let folding_security_bits_per_challenge = folding_security_bits_per_challenge(
                decoding_regime,
                field_log2_size,
                variables,
                next_rate,
            );
            let sumcheck_repetitions = 1;
            rounds.push(WhirRoundParameters {
                round,
                message_variables: variables,
                log_inverse_rate_before: rate,
                log_inverse_rate_after: next_rate,
                requested_queries,
                query_distance,
                query_security_bits_per_query,
                ood_samples: round_ood,
                combination_security_bits_per_challenge,
                folding_log: round_folding_log,
                folding_factor: 1usize << round_folding_log,
                folding_security_bits_per_challenge,
                sumcheck_repetitions,
                query_leaf_width: previous_oracle_leaf_width,
                oracle_leaf_count: previous_oracle_leaf_count,
            });

            // The new oracle encodes `variables` message variables at
            // `next_rate` and groups all values needed by its next fold
            // into one column.
            previous_oracle_leaf_width = 1usize << round_folding_log;
            previous_oracle_leaf_count = 1usize << (variables + next_rate - round_folding_log);
            variables -= round_folding_log;
            rate = next_rate;
            previous_folding_log = round_folding_log;
        }
        debug_assert_eq!(variables, terminal_message_variables);
        let final_sumcheck_rounds = variables;

        let final_query_distance = query_distance(decoding_regime, rate);
        let final_query_security_bits_per_query =
            query_security_bits_per_query(decoding_regime, rate);
        let final_requested_queries = queries(decoding_regime, effective_security_bits, rate);
        let final_sumcheck_security_bits_per_challenge = field_log2_size - 1.0;
        let final_sumcheck_repetitions = 1;
        let final_oracle_leaf_count = previous_oracle_leaf_count;

        Ok(Self {
            decoding_regime,
            max_terminal_message_variables,
            security_bits,
            field_bits,
            field_modulus: field.modulus,
            field_log2_size,
            field_bytes: field.element_bytes,
            field_two_adicity: field.two_adicity,
            message_variables,
            log_inverse_rate,
            codeword_log_length,
            folding_log: initial_folding_log,
            folding_factor: 1usize << initial_folding_log,
            folding_schedule: folding_schedule.to_vec(),
            initial_codeword_log_reduction,
            terminal_message_variables,
            hash_bits,
            hash_collision_security_bits,
            effective_security_bits,
            commitment_ood_samples,
            initial_folding_security_bits_per_challenge,
            initial_sumcheck_repetitions,
            rounds,
            final_requested_queries,
            final_query_distance,
            final_query_security_bits_per_query,
            final_sumcheck_rounds,
            final_sumcheck_security_bits_per_challenge,
            final_sumcheck_repetitions,
            final_log_inverse_rate: rate,
            final_oracle_leaf_width: previous_oracle_leaf_width,
            final_oracle_leaf_count,
        })
    }

    pub fn all_query_counts(&self) -> Vec<usize> {
        self.rounds
            .iter()
            .map(|round| round.requested_queries)
            .chain(std::iter::once(self.final_requested_queries))
            .collect()
    }

    pub fn all_query_distances(&self) -> Vec<f64> {
        self.rounds
            .iter()
            .map(|round| round.query_distance)
            .chain(std::iter::once(self.final_query_distance))
            .collect()
    }

    pub fn all_query_security_bits_per_query(&self) -> Vec<f64> {
        self.rounds
            .iter()
            .map(|round| round.query_security_bits_per_query)
            .chain(std::iter::once(self.final_query_security_bits_per_query))
            .collect()
    }

    pub fn all_ood_counts(&self) -> Vec<usize> {
        if self.folding_schedule.is_empty() {
            Vec::new()
        } else {
            std::iter::once(self.commitment_ood_samples)
                .chain(self.rounds.iter().map(|round| round.ood_samples))
                .collect()
        }
    }

    pub fn all_folding_logs(&self) -> Vec<usize> {
        self.folding_schedule.clone()
    }

    pub fn all_sumcheck_repetitions(&self) -> Vec<usize> {
        if self.folding_schedule.is_empty() {
            Vec::new()
        } else {
            std::iter::once(self.initial_sumcheck_repetitions)
                .chain(self.rounds.iter().map(|round| round.sumcheck_repetitions))
                .collect()
        }
    }

    pub fn all_folding_security_bits_per_challenge(&self) -> Vec<f64> {
        if self.folding_schedule.is_empty() {
            Vec::new()
        } else {
            std::iter::once(self.initial_folding_security_bits_per_challenge)
                .chain(
                    self.rounds
                        .iter()
                        .map(|round| round.folding_security_bits_per_challenge),
                )
                .collect()
        }
    }

    /// Smallest analytic bit estimate among the single-challenge folding,
    /// recursive constraint-combination, and terminal sumcheck checks.
    ///
    /// This deliberately excludes the proximity-query and OOD components,
    /// whose counts are independently selected to meet
    /// `effective_security_bits`.
    pub fn minimum_single_challenge_security_bits(&self) -> f64 {
        self.all_folding_security_bits_per_challenge()
            .into_iter()
            .chain(
                self.rounds
                    .iter()
                    .map(|round| round.combination_security_bits_per_challenge),
            )
            .chain(std::iter::once(
                self.final_sumcheck_security_bits_per_challenge,
            ))
            .fold(f64::INFINITY, f64::min)
    }

    pub fn all_query_leaf_widths(&self) -> Vec<usize> {
        self.rounds
            .iter()
            .map(|round| round.query_leaf_width)
            .chain(std::iter::once(self.final_oracle_leaf_width))
            .collect()
    }
}

fn sample_with_replacement<R: RngCore>(rng: &mut R, universe: usize, draws: usize) -> Vec<usize> {
    let mut set = BTreeSet::new();
    for _ in 0..draws {
        set.insert(rng.gen_range(0..universe));
    }
    set.into_iter().collect()
}

/// Deterministically sample the first committed oracle's packed-leaf queries.
/// This is used to replace that oracle's Merkle authentication with Canopy
/// bottom-tree authentication in the hybrid experiments.
pub fn sample_initial_oracle_queries(params: &WhirParameters, seed: [u8; 32]) -> Vec<usize> {
    let (leaf_count, requested_queries) = params.rounds.first().map_or(
        (
            params.final_oracle_leaf_count,
            params.final_requested_queries,
        ),
        |round| (round.oracle_leaf_count, round.requested_queries),
    );
    let mut rng = ChaCha20Rng::from_seed(seed);
    sample_with_replacement(&mut rng, leaf_count, requested_queries)
}

fn account_oracle_queries<R: RngCore>(
    result: &mut WhirProofSize,
    rng: &mut R,
    leaf_count: usize,
    requested_queries: usize,
    leaf_width: usize,
    parallel_instances: usize,
    preopened: Option<&BTreeSet<usize>>,
) -> Result<()> {
    let queries: BTreeSet<_> = sample_with_replacement(rng, leaf_count, requested_queries)
        .into_iter()
        .collect();
    let opened: BTreeSet<_> = preopened.map_or_else(
        || queries.clone(),
        |already_opened| queries.union(already_opened).copied().collect(),
    );
    let auth = frontier_hash_count(
        &opened.iter().copied().collect::<Vec<_>>(),
        leaf_count,
        leaf_count.ilog2() as usize,
    )?;
    let transmitted_queries = preopened.map_or(queries.len(), |already_opened| {
        queries.difference(already_opened).count()
    });

    result.unique_queries_by_oracle.push(opened.len());
    result.auth_hashes_by_oracle.push(auth);
    result.merkle_auth_hashes += auth;
    result.hash_count += auth;
    result.query_answer_fields += transmitted_queries * leaf_width * parallel_instances;
    Ok(())
}

fn estimate_proof_internal(
    params: &WhirParameters,
    seed: [u8; 32],
    parallel_instances: usize,
    initial_preopened_indices: &[usize],
) -> Result<WhirProofSize> {
    ensure!(
        parallel_instances > 0,
        "at least one WHIR instance is required"
    );
    let initial_leaf_count = params
        .rounds
        .first()
        .map_or(params.final_oracle_leaf_count, |round| {
            round.oracle_leaf_count
        });
    let preopened: BTreeSet<_> = initial_preopened_indices.iter().copied().collect();
    ensure!(
        preopened.len() == initial_preopened_indices.len(),
        "preopened initial-oracle indices must be distinct"
    );
    ensure!(
        preopened.iter().all(|&index| index < initial_leaf_count),
        "preopened initial-oracle index out of range"
    );

    let mut rng = ChaCha20Rng::from_seed(seed);
    let mut result = WhirProofSize {
        field_bytes_per_element: params.field_bytes,
        ..WhirProofSize::default()
    };

    // Existing initial-oracle root is not counted. Parallel instances share
    // interleaved oracle commitments and Merkle paths, but retain independent
    // algebraic prover messages.
    result.ood_fields += params.commitment_ood_samples * parallel_instances;
    result.sumcheck_fields +=
        3 * params.folding_log * params.initial_sumcheck_repetitions * parallel_instances;

    let mut initial_oracle_pending = true;
    for round in &params.rounds {
        // The parallel instances interleave this newly committed folded
        // oracle, so one root authenticates all of them.
        result.intermediate_roots += 1;
        result.hash_count += 1;
        result.ood_fields += round.ood_samples * parallel_instances;

        let already_opened = if initial_oracle_pending {
            initial_oracle_pending = false;
            Some(&preopened)
        } else {
            None
        };
        account_oracle_queries(
            &mut result,
            &mut rng,
            round.oracle_leaf_count,
            round.requested_queries,
            round.query_leaf_width,
            parallel_instances,
            already_opened,
        )?;

        result.sumcheck_fields +=
            3 * round.folding_log * round.sumcheck_repetitions * parallel_instances;
    }

    let already_opened = if initial_oracle_pending {
        Some(&preopened)
    } else {
        None
    };
    account_oracle_queries(
        &mut result,
        &mut rng,
        params.final_oracle_leaf_count,
        params.final_requested_queries,
        params.final_oracle_leaf_width,
        parallel_instances,
        already_opened,
    )?;

    result.final_coefficient_fields += (1 << params.final_sumcheck_rounds) * parallel_instances;
    result.sumcheck_fields +=
        3 * params.final_sumcheck_rounds * params.final_sumcheck_repetitions * parallel_instances;

    recompute_field_count(&mut result);
    Ok(result)
}

fn recompute_field_count(result: &mut WhirProofSize) {
    result.field_count = result.ood_fields
        + result.sumcheck_fields
        + result.batching_fields
        + result.query_answer_fields
        + result.final_coefficient_fields;
}

/// Estimate one concrete transcript using verifier queries derived from `seed`.
/// Verifier randomness and verifier-generated indices occupy zero proof bytes.
pub fn estimate_proof(params: &WhirParameters, seed: [u8; 32]) -> Result<WhirProofSize> {
    estimate_proof_internal(params, seed, 1, &[])
}

/// Estimate parallel WHIR instances whose committed oracles are interleaved.
///
/// All instances use the same query positions and therefore share each oracle
/// root and minimal Merkle frontier. Algebraic messages and queried oracle
/// values scale with `parallel_instances`. `initial_preopened_indices` are
/// leaves already revealed by a preceding consistency check; their values are
/// derived rather than serialized, and their authentication paths are merged
/// with the first WHIR opening.
pub fn estimate_parallel_interleaved_proofs(
    params: &WhirParameters,
    parallel_instances: usize,
    initial_preopened_indices: &[usize],
    seed: [u8; 32],
) -> Result<WhirProofSize> {
    estimate_proof_internal(params, seed, parallel_instances, initial_preopened_indices)
}

/// Estimate a batched WHIR proof whose initial vectors retain independent
/// Merkle roots. Initial query positions are shared but each initial tree has
/// its own authentication frontier; every recursive oracle is shared after
/// the random linear combination.
pub fn estimate_parallel_separate_initial_proofs(
    params: &WhirParameters,
    parallel_instances: usize,
    seed: [u8; 32],
) -> Result<WhirProofSize> {
    let mut result = estimate_proof_internal(params, seed, parallel_instances, &[])?;
    if parallel_instances > 1 {
        let initial_auth = result.auth_hashes_by_oracle[0];
        let extra = initial_auth * (parallel_instances - 1);
        result.auth_hashes_by_oracle[0] += extra;
        result.merkle_auth_hashes += extra;
        result.hash_count += extra;
    }
    Ok(result)
}

/// Estimate the one-challenge Schwartz--Zippel batch of aligned claims.
///
/// The `source_claims` initial vectors retain independent Merkle roots. Their
/// initial query positions are shared, but every authentication frontier and
/// folding column is transmitted. Before the batching challenge the prover
/// sends the `2*source_claims-3` non-endpoint cross-term coefficients when
/// there are multiple claims (and none for a single claim). The
/// verifier then checks one degree-`2*(source_claims-1)` identity, after which
/// WHIR carries only one combined vector, linear form, and recursive oracle.
pub fn estimate_schwartz_zippel_batched_initial_proofs(
    params: &WhirParameters,
    source_claims: usize,
    seed: [u8; 32],
) -> Result<WhirProofSize> {
    ensure!(source_claims >= 1, "at least one source claim is required");
    let mut result = estimate_proof_internal(params, seed, 1, &[])?;
    let initial_auth = result.auth_hashes_by_oracle[0];
    let extra_auth = initial_auth * (source_claims - 1);
    result.auth_hashes_by_oracle[0] += extra_auth;
    result.merkle_auth_hashes += extra_auth;
    result.hash_count += extra_auth;

    let initial_leaf_width = params
        .rounds
        .first()
        .map_or(params.final_oracle_leaf_width, |round| {
            round.query_leaf_width
        });
    result.query_answer_fields +=
        (source_claims - 1) * result.unique_queries_by_oracle[0] * initial_leaf_width;

    // Both input commitments provide their initial OOD evaluations. All OOD
    // constraints attached to recursively committed oracles are for the one
    // combined vector and are already counted by the single proof above.
    result.ood_fields += (source_claims - 1) * params.commitment_ood_samples;
    if source_claims > 1 {
        result.batching_fields += 2 * source_claims - 3;
    }
    recompute_field_count(&mut result);
    Ok(result)
}

/// Estimate parallel WHIR instances when the initial oracle is virtual.
///
/// The external commitments expose one scalar coordinate of every source
/// oracle at a queried index. In the interleaved-RS protocol, those source
/// values supply the effective first folding column, so the initial WHIR
/// opening has additional width one rather than another width-16 column.
/// The first recursive WHIR root and every later width-16 fold are retained.
/// Initial authentication hashes and values are returned to the caller rather
/// than counted in `proof`.
pub fn estimate_parallel_virtual_initial_proofs(
    params: &WhirParameters,
    parallel_instances: usize,
    seed: [u8; 32],
) -> Result<VirtualInitialWhirProofSize> {
    ensure!(
        parallel_instances > 0,
        "at least one WHIR instance is required"
    );

    let virtual_initial_leaf_count = 1usize << params.codeword_log_length;
    let mut rng = ChaCha20Rng::from_seed(seed);
    let mut result = WhirProofSize {
        field_bytes_per_element: params.field_bytes,
        ..WhirProofSize::default()
    };
    let mut initial_query_indices = None;

    result.ood_fields += params.commitment_ood_samples * parallel_instances;
    result.sumcheck_fields +=
        3 * params.folding_log * params.initial_sumcheck_repetitions * parallel_instances;

    for round in &params.rounds {
        result.intermediate_roots += 1;
        result.hash_count += 1;
        result.ood_fields += round.ood_samples * parallel_instances;

        if initial_query_indices.is_none() {
            let queries = sample_with_replacement(
                &mut rng,
                virtual_initial_leaf_count,
                round.requested_queries,
            );
            result.unique_queries_by_oracle.push(queries.len());
            result.auth_hashes_by_oracle.push(0);
            initial_query_indices = Some(queries);
        } else {
            account_oracle_queries(
                &mut result,
                &mut rng,
                round.oracle_leaf_count,
                round.requested_queries,
                round.query_leaf_width,
                parallel_instances,
                None,
            )?;
        }

        result.sumcheck_fields +=
            3 * round.folding_log * round.sumcheck_repetitions * parallel_instances;
    }

    if initial_query_indices.is_none() {
        let queries = sample_with_replacement(
            &mut rng,
            virtual_initial_leaf_count,
            params.final_requested_queries,
        );
        result.unique_queries_by_oracle.push(queries.len());
        result.auth_hashes_by_oracle.push(0);
        initial_query_indices = Some(queries);
    } else {
        account_oracle_queries(
            &mut result,
            &mut rng,
            params.final_oracle_leaf_count,
            params.final_requested_queries,
            params.final_oracle_leaf_width,
            parallel_instances,
            None,
        )?;
    }

    result.final_coefficient_fields += (1 << params.final_sumcheck_rounds) * parallel_instances;
    result.sumcheck_fields +=
        3 * params.final_sumcheck_rounds * params.final_sumcheck_repetitions * parallel_instances;
    recompute_field_count(&mut result);

    Ok(VirtualInitialWhirProofSize {
        proof: result,
        initial_query_indices: initial_query_indices.unwrap(),
    })
}

/// Convenience helper for deterministic tests and callers using a `u64` seed.
pub fn expand_seed(seed: u64) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[..8].copy_from_slice(&seed.to_le_bytes());
    *blake3::hash(&bytes).as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modeled_24_bit_field_supports_strictly_shorter_additive_domains() {
        let field = field_profile(24, 23).unwrap();
        assert_eq!(field.element_bytes, 3);
        assert_eq!(field.domain_kind, "additive binary-extension-field");
        assert!(field_profile(24, 24).is_err());
        let params = WhirParameters::johnson_bound_with_field(20, 3, field).unwrap();
        assert_eq!(params.codeword_log_length, 23);
    }

    #[test]
    fn retired_192_bit_profile_is_unavailable() {
        assert!(field_profile(192, 20).is_err());
    }

    #[test]
    fn canopy_parameters_have_expected_rate_and_length() {
        let params = WhirParameters::johnson_bound(18, 4).unwrap();
        assert_eq!(params.codeword_log_length, 22);
        assert_eq!(params.folding_factor, 16);
        assert_eq!(params.security_bits, 128);
        assert_eq!(params.field_bits, 256);
        assert!((params.field_log2_size - DEFAULT_FIELD_LOG2_SIZE).abs() < f64::EPSILON);
        assert_eq!(params.hash_bits, 256);
        assert_eq!(params.hash_collision_security_bits, 128);
        assert_eq!(params.effective_security_bits, 128);
        assert_eq!(params.commitment_ood_samples, 1);
        assert_eq!(params.rounds.len(), 3);
        assert_eq!(params.all_folding_logs(), vec![4, 4, 4, 4]);
        assert_eq!(params.all_query_counts(), vec![67, 38, 26, 20]);
        assert_eq!(params.initial_sumcheck_repetitions, 1);
        assert!(params
            .rounds
            .iter()
            .all(|round| round.sumcheck_repetitions == 1));
        assert_eq!(params.final_sumcheck_repetitions, 1);
        assert_eq!(params.terminal_message_variables, 2);
        assert_eq!(params.final_sumcheck_rounds, 2);
        assert_eq!(params.final_oracle_leaf_width, 16);
    }

    #[test]
    fn estimate_accounts_for_all_components() {
        let params = WhirParameters::johnson_bound(18, 4).unwrap();
        let proof = estimate_proof(&params, expand_seed(7)).unwrap();
        assert!(proof.hash_count > proof.intermediate_roots);
        assert!(proof.query_answer_fields > 0);
        assert_eq!(
            proof.field_count,
            proof.ood_fields
                + proof.sumcheck_fields
                + proof.batching_fields
                + proof.query_answer_fields
                + proof.final_coefficient_fields
        );
        assert_eq!(
            proof.unique_queries_by_oracle.len(),
            params.rounds.len() + 1
        );
    }

    #[test]
    fn parallel_interleaved_proofs_share_hashes_and_scale_fields() {
        let params = WhirParameters::johnson_bound(8, 4).unwrap();
        let seed = expand_seed(11);
        let single = estimate_proof(&params, seed).unwrap();
        let parallel = estimate_parallel_interleaved_proofs(&params, 5, &[], seed).unwrap();

        assert_eq!(parallel.hash_count, single.hash_count);
        assert_eq!(parallel.merkle_auth_hashes, single.merkle_auth_hashes);
        assert_eq!(parallel.ood_fields, 5 * single.ood_fields);
        assert_eq!(parallel.sumcheck_fields, 5 * single.sumcheck_fields);
        assert_eq!(
            parallel.final_coefficient_fields,
            5 * single.final_coefficient_fields
        );
        assert_eq!(parallel.query_answer_fields, 5 * single.query_answer_fields);

        let initial_leaf_count = params
            .rounds
            .first()
            .map_or(params.final_oracle_leaf_count, |round| {
                round.oracle_leaf_count
            });
        let all_initial_leaves: Vec<_> = (0..initial_leaf_count).collect();
        let deduplicated =
            estimate_parallel_interleaved_proofs(&params, 5, &all_initial_leaves, seed).unwrap();
        assert_eq!(deduplicated.auth_hashes_by_oracle[0], 0);
        assert_eq!(deduplicated.unique_queries_by_oracle[0], initial_leaf_count);
        assert_eq!(
            deduplicated.merkle_auth_hashes,
            deduplicated.auth_hashes_by_oracle[1..].iter().sum()
        );
        assert!(deduplicated.query_answer_fields < parallel.query_answer_fields);
    }

    #[test]
    fn initial_fold_can_differ_from_recursive_folds() {
        let params = WhirParameters::with_regime_and_terminal_and_initial_folding(
            22,
            1,
            WhirDecodingRegime::Capacity,
            8,
            6,
        )
        .unwrap();
        assert_eq!(params.folding_log, 6);
        assert_eq!(params.folding_factor, 64);
        assert_eq!(params.initial_codeword_log_reduction, 1);
        assert_eq!(params.all_folding_logs(), vec![6, 4, 4]);
        assert!(params
            .rounds
            .iter()
            .all(|round| round.folding_log == DEFAULT_FOLDING_LOG));
    }

    #[test]
    fn fixed_three_variable_folds_apply_to_every_round() {
        let params = WhirParameters::with_regime_and_terminal_and_folding(
            22,
            1,
            WhirDecodingRegime::Capacity,
            8,
            3,
        )
        .unwrap();
        assert_eq!(params.all_folding_logs(), vec![3, 3, 3, 3, 3]);
        assert!(params.rounds.iter().all(|round| round.folding_log == 3));
    }

    #[test]
    fn explicit_schedule_terminates_at_exact_target() {
        let schedule = [3, 4, 3, 4, 3, 3];
        let params = WhirParameters::with_regime_and_terminal_and_schedule(
            30,
            1,
            WhirDecodingRegime::Johnson,
            10,
            &schedule,
            1,
        )
        .unwrap();
        assert_eq!(params.all_folding_logs(), schedule);
        assert_eq!(params.terminal_message_variables, 10);
        assert_eq!(params.final_sumcheck_rounds, 10);
    }

    #[test]
    fn exact_terminal_without_folding_is_supported() {
        let params = WhirParameters::with_regime_and_terminal_and_schedule(
            10,
            5,
            WhirDecodingRegime::Capacity,
            10,
            &[],
            1,
        )
        .unwrap();
        assert!(params.all_folding_logs().is_empty());
        assert_eq!(params.commitment_ood_samples, 0);
        assert_eq!(params.final_oracle_leaf_width, 1);
        let proof = estimate_proof(&params, expand_seed(17)).unwrap();
        assert_eq!(proof.final_coefficient_fields, 1 << 10);
    }

    #[test]
    fn zero_reduction_keeps_the_first_recursive_codeword_length() {
        let params = WhirParameters::with_regime_and_terminal_and_initial_transition(
            22,
            1,
            WhirDecodingRegime::Capacity,
            8,
            3,
            0,
        )
        .unwrap();
        assert_eq!(params.initial_codeword_log_reduction, 0);
        assert_eq!(params.rounds[0].message_variables, 19);
        assert_eq!(params.rounds[0].log_inverse_rate_after, 4);
        assert_eq!(
            params.rounds[0].message_variables + params.rounds[0].log_inverse_rate_after,
            params.codeword_log_length
        );
    }

    #[test]
    fn schwartz_zippel_batch_only_doubles_the_initial_opening() {
        let params =
            WhirParameters::with_regime_and_terminal(18, 4, WhirDecodingRegime::Capacity, 8)
                .unwrap();
        let seed = expand_seed(13);
        let single = estimate_proof(&params, seed).unwrap();
        let batched = estimate_schwartz_zippel_batched_initial_proofs(&params, 2, seed).unwrap();
        assert_eq!(batched.batching_fields, 1);
        assert_eq!(
            batched.ood_fields,
            single.ood_fields + params.commitment_ood_samples
        );
        assert_eq!(batched.sumcheck_fields, single.sumcheck_fields);
        assert_eq!(
            batched.final_coefficient_fields,
            single.final_coefficient_fields
        );
        assert_eq!(
            batched.auth_hashes_by_oracle[0],
            2 * single.auth_hashes_by_oracle[0]
        );
        assert_eq!(
            batched.auth_hashes_by_oracle[1..],
            single.auth_hashes_by_oracle[1..]
        );
    }

    #[test]
    fn generalized_schwartz_zippel_batch_scales_only_the_source_opening() {
        let params =
            WhirParameters::with_regime_and_terminal(18, 4, WhirDecodingRegime::Capacity, 8)
                .unwrap();
        let seed = expand_seed(14);
        let single = estimate_proof(&params, seed).unwrap();
        let source_claims = 8;
        let batched =
            estimate_schwartz_zippel_batched_initial_proofs(&params, source_claims, seed).unwrap();

        assert_eq!(batched.batching_fields, 2 * source_claims - 3);
        assert_eq!(
            batched.ood_fields,
            single.ood_fields + (source_claims - 1) * params.commitment_ood_samples
        );
        assert_eq!(
            batched.auth_hashes_by_oracle[0],
            source_claims * single.auth_hashes_by_oracle[0]
        );
        assert_eq!(
            batched.query_answer_fields,
            single.query_answer_fields
                + (source_claims - 1)
                    * single.unique_queries_by_oracle[0]
                    * params.rounds[0].query_leaf_width
        );
        assert_eq!(
            batched.auth_hashes_by_oracle[1..],
            single.auth_hashes_by_oracle[1..]
        );
    }

    #[test]
    fn one_claim_batch_is_the_ordinary_proof() {
        let params =
            WhirParameters::with_regime_and_terminal(18, 4, WhirDecodingRegime::Capacity, 8)
                .unwrap();
        let seed = expand_seed(15);
        let single = estimate_proof(&params, seed).unwrap();
        let batched = estimate_schwartz_zippel_batched_initial_proofs(&params, 1, seed).unwrap();
        assert_eq!(batched.hash_count, single.hash_count);
        assert_eq!(batched.field_count, single.field_count);
        assert_eq!(batched.batching_fields, 0);
    }

    #[test]
    fn theorem_4_6_uses_multiplicity_twenty_for_the_canonical_slack() {
        let message_variables = 30;
        let log_inverse_rate = 1;
        let rho = 0.5_f64;
        let sqrt_rho = rho.sqrt();
        let eta = sqrt_rho / 20.0;
        assert_eq!((sqrt_rho / eta).ceil(), 20.0);

        let exact = johnson_exceptional_set_log2(message_variables, log_inverse_rate);
        let old_approximation =
            (2.0 * 10.5_f64.powi(5) / 3.0).log2() + message_variables as f64 + 2.5;
        assert!(exact > old_approximation + 4.0);
        assert!(exact < old_approximation + 6.0);
    }

    #[test]
    fn virtual_initial_oracle_defers_its_opening_accounting_to_the_caller() {
        let params = WhirParameters::johnson_bound(18, 4).unwrap();
        let seed = expand_seed(19);
        let ordinary = estimate_parallel_interleaved_proofs(&params, 6, &[], seed).unwrap();
        let virtual_proof = estimate_parallel_virtual_initial_proofs(&params, 6, seed).unwrap();

        assert!(virtual_proof
            .initial_query_indices
            .iter()
            .all(|&index| index < (1usize << params.codeword_log_length)));
        assert_eq!(
            virtual_proof.proof.unique_queries_by_oracle[0],
            virtual_proof.initial_query_indices.len()
        );
        assert_eq!(virtual_proof.proof.auth_hashes_by_oracle[0], 0);
        assert!(virtual_proof.proof.merkle_auth_hashes < ordinary.merkle_auth_hashes);
        assert!(virtual_proof.proof.query_answer_fields < ordinary.query_answer_fields);
        assert_eq!(
            virtual_proof.proof.intermediate_roots,
            ordinary.intermediate_roots
        );
    }

    #[test]
    fn unrestricted_ood_counts_meet_the_target_and_are_minimal() {
        for n in [22usize, 24, 26, 28, 30] {
            for h in 1..=15 {
                let params = WhirParameters::johnson_bound(n - h, h).unwrap();
                let mut message_variables = n - h;
                let mut rate = h;
                for (position, count) in params.all_ood_counts().into_iter().enumerate() {
                    if position > 0 {
                        message_variables -= DEFAULT_FOLDING_LOG;
                        rate += DEFAULT_FOLDING_LOG - 1;
                    }
                    assert!(
                        rbr_ood_sample(
                            WhirDecodingRegime::Johnson,
                            message_variables,
                            rate,
                            DEFAULT_FIELD_LOG2_SIZE,
                            count,
                        ) >= DEFAULT_SECURITY_BITS as f64
                    );
                    if count > 1 {
                        assert!(
                            rbr_ood_sample(
                                WhirDecodingRegime::Johnson,
                                message_variables,
                                rate,
                                DEFAULT_FIELD_LOG2_SIZE,
                                count - 1
                            ) < DEFAULT_SECURITY_BITS as f64
                        );
                    }
                }
            }
        }

        // This is the worst initial commitment in the requested grid.
        assert_eq!(
            ood_samples(
                WhirDecodingRegime::Johnson,
                DEFAULT_SECURITY_BITS,
                29,
                1,
                DEFAULT_FIELD_LOG2_SIZE,
            ),
            1
        );
    }

    #[test]
    fn capacity_uses_the_conjectural_radius_and_smaller_query_count() {
        let johnson = WhirParameters::johnson_bound(18, 4).unwrap();
        let capacity = WhirParameters::capacity_bound(18, 4).unwrap();
        assert_eq!(capacity.decoding_regime, WhirDecodingRegime::Capacity);
        assert!(capacity.final_query_distance > johnson.final_query_distance);
        assert!(capacity
            .all_query_counts()
            .iter()
            .zip(johnson.all_query_counts())
            .all(|(&capacity, johnson)| capacity < johnson));
        assert!(capacity.minimum_single_challenge_security_bits() >= 128.0);
    }

    #[test]
    fn terminal_cap_eight_stops_one_round_earlier() {
        let executable = WhirParameters::capacity_bound(21, 9).unwrap();
        let terminal_eight =
            WhirParameters::with_regime_and_terminal(21, 9, WhirDecodingRegime::Capacity, 8)
                .unwrap();
        assert_eq!(executable.terminal_message_variables, 1);
        assert_eq!(terminal_eight.terminal_message_variables, 5);
        assert_eq!(terminal_eight.rounds.len() + 1, executable.rounds.len());
    }

    #[test]
    fn every_sumcheck_uses_one_256_bit_field_transcript() {
        for n in [22usize, 24, 26, 28, 30] {
            for h in 1..=15 {
                let params = WhirParameters::johnson_bound(n - h, h).unwrap();
                assert_eq!(params.initial_sumcheck_repetitions, 1);
                assert!(params
                    .rounds
                    .iter()
                    .all(|round| round.sumcheck_repetitions == 1));
                assert_eq!(params.final_sumcheck_repetitions, 1);
                assert!(
                    params.final_sumcheck_security_bits_per_challenge
                        >= DEFAULT_SECURITY_BITS as f64
                );
            }
        }
    }
}
