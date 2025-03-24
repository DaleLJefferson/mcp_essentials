use insta::assert_snapshot;
use map::map;

#[test]
fn test_everything() {
    let source_code = include_str!("../test_assets/everything.rs");

    let result = map(source_code);

    assert_snapshot!(result);
}
