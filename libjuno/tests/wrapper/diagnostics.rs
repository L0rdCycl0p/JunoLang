use std::{fs::File, io::Read, path::Path};

#[test]
fn test_diagnostics_ok_file() {
    let mut source = "".to_string();
    File::open(Path::new("../test/test.juno"))
        .unwrap()
        .read_to_string(&mut source)
        .unwrap();
    let diagnostics = libjuno::diagnostics::analyze(source.as_str());
    assert!(
        diagnostics.is_empty(),
        "Expected no diagnostics, got:\n{:#?}",
        diagnostics
    );
}

#[test]
fn test_diagnostics_wrong_file() {
    let mut source = "".to_string();
    File::open(Path::new("../test/wrong_syntax/semicolon.juno"))
        .unwrap()
        .read_to_string(&mut source)
        .unwrap();
    let diagnostics = libjuno::diagnostics::analyze(source.as_str());
    assert!(
        !diagnostics.is_empty(),
        "Expected diagnostics, got:\n{:#?}",
        diagnostics
    );
}
