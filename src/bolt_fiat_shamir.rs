//! FS transcript model for the serialized public-H Bolt adapter.
use super::*;
use crate::fiat_shamir::{native_commit, native_verify, Ledger};

pub(super) fn reduce(
    l: &mut Ledger,
    mut cfg: WhirConfig,
    common: &WhirConfig,
    forms: usize,
) -> Result<()> {
    l.geometric(forms + cfg.initial_out_domain_samples);
    while cfg.initial_size() > common.initial_size() {
        let next = batching::next_config(&cfg, common)?;
        l.sumcheck(cfg.initial_sumcheck.num_rounds(), 2);
        native_commit(l, &next);
        l.indices(
            cfg.initial_committer.codeword_length(),
            cfg.initial_committer.in_domain_samples(),
        );
        l.geometric(cfg.initial_committer.in_domain_samples());
        if next.initial_size() > common.initial_size() {
            l.geometric(next.initial_out_domain_samples);
        }
        cfg = next;
    }
    Ok(())
}

/// `outer_words` includes opened row elements and authentication digests.
/// `query_bytes` includes retries when outer sampling seeks distinct rows.
fn model(
    row: &batching::ComparisonRow,
    outer_words: &[usize],
    query_bytes: &[usize],
) -> Result<Ledger> {
    let eta = row.input_oracles;
    ensure!(
        outer_words.len() == eta && query_bytes.len() == eta,
        "wrong transcript batch shape"
    );
    let k = (1usize << row.log_input) / row.interleaving;
    let heights = row.systematic_h.zip(row.parity_h);
    let common = batching::common_config(k, heights, row.layout, row.aligned_log.unwrap())?;
    let x = batching::initial_config(k.ilog2() as usize, &common)?;
    let u = batching::initial_config(k.ilog2() as usize - 3, &common)?;
    let nonconstant = heights.is_some_and(|h| h.0 < k.ilog2() as usize);
    let mut l = Ledger::domain();
    l.fields(2 * eta); // all initial X/Y roots precede opening
    for i in 0..eta {
        l.fields(row.interleaving); // g
        l.challenges(row.interleaving);
        native_commit(&mut l, &x);
        native_commit(&mut l, &u);
        l.challenges(k.ilog2() as usize - 3);
        l.fields(1);
        l.push('s', 1, query_bytes[i]);
        l.fields(outer_words[i]);
        l.challenges(2);
        if heights.is_some() {
            if nonconstant {
                for _ in 0..2 {
                    l.challenges(1);
                    l.challenges(common.initial_out_domain_samples);
                    l.fields(common.initial_out_domain_samples);
                }
            } else {
                l.fields(2);
            }
        }
    }
    for _ in 0..eta {
        reduce(&mut l, x.clone(), &common, 3)?;
        reduce(&mut l, u.clone(), &common, 2)?;
        if nonconstant {
            for _ in 0..2 {
                reduce(&mut l, common.clone(), &common, 1)?;
            }
        }
    }
    let count = eta * if nonconstant { 4 } else { 2 };
    l.fields(count * (count - 1));
    native_verify(&mut l, &common, count, count);
    Ok(l)
}

pub(super) fn expected_query_bytes(rows: usize, count: usize) -> f64 {
    if count >= rows {
        return 0.;
    }
    let draws = (0..count)
        .map(|i| rows as f64 / (rows - i) as f64)
        .sum::<f64>();
    draws * (rows.ilog2() as usize).div_ceil(8) as f64
}
#[derive(Serialize)]
pub struct Row {
    input_oracles: usize,
    log_input: usize,
    interleaving: usize,
    layout: CanopyLayout,
    scheme: String,
    systematic_h: Option<usize>,
    parity_h: Option<usize>,
    aligned_log: Option<usize>,
    fs_finalizations: usize,
    fs_absorb_bytes: f64,
    fs_squeeze_bytes: f64,
    fs_proof_bytes: usize,
    verifier_hashes: f64,
}
pub fn account(input: &Path, output: &Path) -> Result<()> {
    let rows = csv::Reader::from_path(input)?
        .deserialize::<batching::ComparisonRow>()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let mut result = Vec::new();
    for r in rows {
        let k = (1usize << r.log_input) / r.interleaving;
        // The baseline's external_auth_bytes includes its two roots; those
        // are separate initial transcript messages in the model.
        let root_bytes = if r.systematic_h.is_none() {
            64. * r.input_oracles as f64
        } else {
            0.
        };
        let outer = (r.external_row_bytes + r.external_auth_bytes - root_bytes)
            / (32. * r.input_oracles as f64);
        let qbytes = expected_query_bytes(k, r.systematic_queries)
            + expected_query_bytes(k / 4, r.parity_queries);
        let ledger = model(
            &r,
            &vec![outer.round() as usize; r.input_oracles],
            &vec![qbytes.round() as usize; r.input_oracles],
        )?;
        let c = ledger.counts();
        let absorb = c.absorb_bytes as f64 + 32. * (outer - outer.round()) * r.input_oracles as f64;
        let squeezed = c.squeeze_bytes as f64 + (qbytes - qbytes.round()) * r.input_oracles as f64;
        result.push(Row {
            input_oracles: r.input_oracles,
            log_input: r.log_input,
            interleaving: r.interleaving,
            layout: r.layout,
            scheme: r.scheme,
            systematic_h: r.systematic_h,
            parity_h: r.parity_h,
            aligned_log: r.aligned_log,
            fs_finalizations: c.finalizations,
            fs_absorb_bytes: absorb,
            fs_squeeze_bytes: squeezed,
            fs_proof_bytes: 0,
            verifier_hashes: r.verifier_auth_hashes
                + r.verifier_hash_to_field
                + c.finalizations as f64,
        });
    }
    write_csv(output, &result)
}

#[cfg(feature = "fs-accounting")]
pub fn validate(input: &Path, output: &Path) -> Result<()> {
    use crate::fiat_shamir::capture;
    #[derive(Serialize)]
    struct Validation {
        input_oracles: usize,
        log_input: usize,
        scheme: String,
        layout: CanopyLayout,
        proof_verified: bool,
        trace_matches_model: bool,
        finalizations: usize,
        absorb_bytes: usize,
        squeeze_bytes: usize,
        narg_bytes: usize,
        hint_bytes: usize,
    }
    let rows = csv::Reader::from_path(input)?
        .deserialize::<batching::ComparisonRow>()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let mut checks = Vec::new();
    for r in rows.iter().filter(|r| {
        r.log_input == 18
            || (r.log_input == 22
                && [1, 8].contains(&r.input_oracles)
                && (r.systematic_h.is_none() || r.layout == CanopyLayout::EqualFull))
    }) {
        let n = r.log_input;
        let eta = r.input_oracles;
        let encoder = BoltMaxEncoder::new((1 << n) / r.interleaving, r.interleaving, DEFAULT_SEED)?;
        let data = (0..eta)
            .map(|j| {
                (0..1usize << n)
                    .map(|i| {
                        crate::benchmark_field_element(
                            DEFAULT_SEED,
                            0x424f4c54 + j as u64,
                            i as u64,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        eprintln!(
            "FS trace validation: n={n}, eta={eta}, {} {:?}",
            r.scheme, r.layout
        );
        let heights = r.systematic_h.zip(r.parity_h);
        let proof = multi::prove(
            &encoder,
            &data,
            heights,
            DEFAULT_SEED,
            r.layout,
            r.aligned_log.unwrap(),
        )?;
        let (verified, traces) = capture(|| {
            multi::verify(
                &encoder,
                n,
                heights,
                DEFAULT_SEED,
                &proof.evaluations,
                &proof.proof,
                r.layout,
                r.aligned_log.unwrap(),
            )
        });
        verified?;
        ensure!(traces.len() == 1, "expected one verifier transcript");
        let trace = &traces[0];
        let mut openings = trace
            .events
            .iter()
            .enumerate()
            .filter(|(_, e)| e.operation == 'a' && e.bytes == 32)
            .collect::<Vec<_>>();
        openings.sort_by_key(|(_, e)| std::cmp::Reverse(e.count));
        openings.truncate(eta);
        openings.sort_by_key(|(i, _)| *i);
        let mut words = Vec::new();
        let mut queries = Vec::new();
        for (i, e) in openings {
            ensure!(
                trace.events[i - 1].operation == 's' && trace.events[i - 1].bytes == 1,
                "missing outer query segment"
            );
            words.push(e.count);
            queries.push(trace.events[i - 1].count);
        }
        let modeled = model(r, &words, &queries)?;
        if modeled.normalized() != trace.normalized() {
            write_csv(output.with_extension("actual.csv"), &trace.events)?;
            write_csv(output.with_extension("expected.csv"), &modeled.events)?;
            anyhow::bail!(
                "FS model differs from verifier at n={n}, eta={eta}, {} {:?}",
                r.scheme,
                r.layout
            );
        }
        let c = trace.counts();
        ensure!(
            c.absorb_bytes == 96 + proof.proof.narg_string.len(),
            "transcript bytes do not reconcile"
        );
        checks.push(Validation {
            input_oracles: eta,
            log_input: n,
            scheme: r.scheme.clone(),
            layout: r.layout,
            proof_verified: true,
            trace_matches_model: true,
            finalizations: c.finalizations,
            absorb_bytes: c.absorb_bytes,
            squeeze_bytes: c.squeeze_bytes,
            narg_bytes: proof.proof.narg_string.len(),
            hint_bytes: proof.proof.hints.len(),
        });
    }
    write_csv(output, &checks)
}
