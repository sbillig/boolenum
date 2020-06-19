#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/good*.rs");
    t.compile_fail("tests/ui/bad*.rs");
}
