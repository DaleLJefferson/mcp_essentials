use codemap::codemap;
use insta::assert_snapshot;

#[test]
fn test_everything() {
    let source_code = include_str!("../test_assets/everything.rs");

    let result = codemap(source_code);

    assert_snapshot!(result);
}
