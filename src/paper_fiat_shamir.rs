//! Fiat–Shamir schedule for the composed paper protocol (three coefficients
//! per sumcheck, matching its size model). This is a transcript-cost model,
//! not a replacement for the component-based WHIR timing harness.
use super::*;
use crate::fiat_shamir::Ledger;

#[derive(Deserialize)]
struct QueryRow {
    scheme: String,
    input_oracles: usize,
    log_message_length: usize,
    oracle_role: String,
    mean_auth_hashes: f64,
    mean_answer_fields: f64,
}

fn ood(l: &mut Ledger, count: usize) {
    l.challenges(count);
    l.fields(count);
}

/// Initial roots/OOD and per-source constraint combinations precede this
/// shared tail. Authenticated WHIR openings are hints, as in native WHIR.
fn tail(l: &mut Ledger, p: &WhirParameters, sources: usize) {
    if sources > 1 {
        l.fields(2 * sources - 3);
        l.challenges(1); // generalized Schwartz--Zippel batching
    }
    l.sumcheck(p.folding_log * p.initial_sumcheck_repetitions, 3);
    for r in &p.rounds {
        l.absorb(32);
        ood(l, r.ood_samples);
        l.indices(r.oracle_leaf_count, r.requested_queries);
        l.geometric(r.ood_samples + r.requested_queries);
        l.sumcheck(r.folding_log * r.sumcheck_repetitions, 3);
    }
    l.fields(1 << p.final_sumcheck_rounds);
    l.indices(p.final_oracle_leaf_count, p.final_requested_queries);
    l.sumcheck(p.final_sumcheck_rounds * p.final_sumcheck_repetitions, 3);
}

/// A single transcript for the composed argument, not a sum of independent
/// component transcripts. Each front runs in input order, then one shared
/// tail runs. Canopy bottom openings enter the sponge before the SEP
/// combination challenge; Merkle-authenticated WHIR openings remain hints.
fn model(row: &HybridProofSizeRow, bottom_words_per_input: usize) -> Result<Ledger> {
    let canopy = row.scheme == "canopy_schwartz_zippel";
    let schedule = row
        .outer_folding_schedule
        .split(';')
        .map(str::parse)
        .collect::<std::result::Result<Vec<usize>, _>>()?;
    let rate = if row.outer_starting_rate == "1/4" {
        2
    } else {
        1
    };
    let outer = paper_parameters(
        row.log_message_length,
        rate,
        WhirDecodingRegime::Johnson,
        row.outer_terminal_message_variables,
        &schedule,
        usize::from(!canopy),
    )?;
    let mut l = Ledger::domain();
    if row.input_oracles == 1 && !canopy {
        l.absorb(32);
        ood(&mut l, outer.commitment_ood_samples);
        l.geometric(1 + outer.commitment_ood_samples);
        tail(&mut l, &outer, 1);
        return Ok(l);
    }
    let f = row.outer_initial_folding_log;
    let shared = paper_parameters(
        row.log_message_length - f,
        rate + f - usize::from(!canopy),
        WhirDecodingRegime::Johnson,
        row.outer_terminal_message_variables,
        &schedule[1..],
        1,
    )?;
    let first = query_geometry(&outer)[0];
    for _ in 0..row.input_oracles {
        l.absorb(32); // initial Merkle root or Canopy top root
        ood(&mut l, outer.commitment_ood_samples);
        l.geometric(1 + outer.commitment_ood_samples);
        l.sumcheck(f * outer.initial_sumcheck_repetitions, 3);
        l.absorb(32); // aligned second oracle
        ood(&mut l, shared.commitment_ood_samples);
        l.indices(first.4, first.5);
        l.geometric(first.5 + shared.commitment_ood_samples);
        if canopy {
            l.fields(bottom_words_per_input);
            l.challenges(1); // combine the opened Canopy SEP constraints
            ood(&mut l, shared.commitment_ood_samples);
            l.geometric(1 + shared.commitment_ood_samples);
        }
    }
    tail(&mut l, &shared, row.batched_source_oracles);
    Ok(l)
}

#[derive(Serialize)]
struct Row {
    log_message_length: usize,
    input_oracles: usize,
    scheme: String,
    fs_finalizations: usize,
    fs_absorb_bytes: f64,
    fs_squeeze_bytes: usize,
    fs_proof_bytes: usize,
    verifier_hashes: f64,
}

pub fn account(input: &Path, queries: &Path, output: &Path) -> Result<()> {
    let rows = csv::Reader::from_path(input)?
        .deserialize::<HybridProofSizeRow>()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let queries = csv::Reader::from_path(queries)?
        .deserialize::<QueryRow>()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let mut out = csv::Writer::from_path(output)?;
    for row in rows {
        let bottom = queries
            .iter()
            .find(|q| {
                q.scheme == row.scheme
                    && q.input_oracles == row.input_oracles
                    && q.log_message_length == row.log_message_length
                    && q.oracle_role == "outer first oracle via Canopy"
            })
            .map_or(0.0, |q| q.mean_auth_hashes + q.mean_answer_fields);
        let per_input = (bottom / row.input_oracles as f64).round() as usize;
        let l = model(&row, per_input)?;
        let c = l.counts();
        let counted_bottom = (per_input * row.input_oracles * 32) as f64;
        let absorbed = c.absorb_bytes as f64 - counted_bottom + bottom * 32.0;
        // Reconcile every absorbed root/algebraic message to the independently
        // generated proof-size components. Query hints cost no absorption.
        let expected = 96.0
            + 32.0 * row.mean_root_hashes
            + row.mean_ood_bytes
            + row.mean_sumcheck_bytes
            + row.mean_batching_bytes
            + row.mean_final_coefficient_bytes
            + bottom * 32.0;
        ensure!(
            (absorbed - expected).abs() < 1e-6,
            "WHIR FS message reconciliation failed: {} n={} t={} ({absorbed} != {expected})",
            row.scheme,
            row.log_message_length,
            row.input_oracles
        );
        out.serialize(Row {
            log_message_length: row.log_message_length,
            input_oracles: row.input_oracles,
            scheme: row.scheme,
            fs_finalizations: c.finalizations,
            fs_absorb_bytes: absorbed,
            fs_squeeze_bytes: c.squeeze_bytes,
            fs_proof_bytes: 0,
            verifier_hashes: row.verifier_auth_hashes
                + row.verifier_hash_to_field
                + c.finalizations as f64,
        })?;
    }
    out.flush()?;
    Ok(())
}
