use anyhow::Result;
use std::path::Path;
fn main() -> Result<()> {
    let root = Path::new("results/gf256_batched_suite");
    let out = root.join("fiat_shamir");
    std::fs::create_dir_all(&out)?;
    canopy_system::bolt_experiment::fiat_shamir::account(
        &root.join("bolt/proof_sizes/proof_sizes.csv"),
        &out.join("bolt.csv"),
    )?;
    canopy_system::paper_experiments::fiat_shamir::account(
        &root.join("whir/proof_sizes/hybrid_proof_sizes.csv"),
        &root.join("whir/proof_sizes/oracle_query_breakdown.csv"),
        &out.join("whir.csv"),
    )?;
    #[cfg(feature = "fs-accounting")]
    if std::env::args().any(|a| a == "--validate") {
        canopy_system::bolt_experiment::fiat_shamir::validate(
            &root.join("bolt/proof_sizes/proof_sizes.csv"),
            &out.join("bolt_validation.csv"),
        )?;
    }
    Ok(())
}
