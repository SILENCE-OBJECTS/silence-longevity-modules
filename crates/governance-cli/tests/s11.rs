use governance_validator::scan_s11;

#[test]
fn clean_structure_language_passes() {
    scan_s11("structure-only continuity vector; no raw stream; injected clock").unwrap();
}

#[test]
fn diagnostic_language_fails() {
    assert!(scan_s11("offers a diagnostic readout").is_err());
}
