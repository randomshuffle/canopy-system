"""Keep historical and steady-state Section 8 benchmark workloads distinct."""

LEGACY_WORKLOAD = "legacy_fresh_only_v1"
CURRENT_WORKLOAD = "steady_state_v2"
WORKLOADS = (LEGACY_WORKLOAD, CURRENT_WORKLOAD)


def workload_name(value):
    # Historical CSVs predate the workload column and omit the prior Canopy.
    if value is not None and not isinstance(value, str):
        raise ValueError(f"Invalid Section 8 workload {value!r}")
    name = value.strip() if value else LEGACY_WORKLOAD
    if name not in WORKLOADS:
        raise ValueError(f"Unknown Section 8 workload {name!r}")
    return name


def validate_sources(workload, scheme, ell_cast, ell_acc, sources):
    if scheme == "canopy_total":
        return ""
    expected = ell_cast + ell_acc
    if scheme == "ifold":
        expected = ell_cast
    if workload == CURRENT_WORKLOAD and scheme in ("ifold", "iacc"):
        expected += 1
    try:
        actual = int(sources)
    except (TypeError, ValueError):
        raise ValueError(f"{workload}/{scheme}: missing or invalid source count {sources!r}") from None
    if str(sources).strip() != str(actual) or actual != expected:
        raise ValueError(f"{workload}/{scheme}: expected {expected} sources, got {sources!r}")
    return actual


def single_workload(rows):
    names = {row["workload"] for row in rows}
    if not names:
        raise ValueError("No Section 8 workload rows were supplied")
    if len(names) != 1:
        raise ValueError("Cannot mix Section 8 workloads: " + ", ".join(sorted(names)))
    return names.pop()


def validate_metadata(workload, records):
    for record in records:
        metadata = record["metadata"]
        proof_configuration = metadata.get("proof_configuration", {})
        if not isinstance(proof_configuration, dict):
            raise ValueError("Supplied proof_configuration metadata must be an object")
        for value in (metadata.get("workload"), proof_configuration.get("workload")):
            if value and workload_name(value) != workload:
                raise ValueError(f"Supplied provenance workload {value!r} differs from CSV workload {workload!r}")


def workload_description(workload):
    if workload == CURRENT_WORKLOAD:
        return ("Workload `steady_state_v2`: ifold batches ell_cast fresh Canopy claims and one prior Canopy claim, "
                "returning ell_cast + 1 Merkle residuals; iacc batches those residuals and ell_acc prior Merkle claims. "
                "The Merkle controls batch ell_cast fresh claims and ell_acc prior claims.")
    return ("Historical workload `legacy_fresh_only_v1`: ifold batches only ell_cast fresh Canopy claims and "
            "iacc batches ell_cast residuals plus ell_acc prior Merkle claims. It omits the prior Canopy accumulator "
            "and does not measure the corrected steady-state recurrence. Missing workload columns identify this historical format.")
