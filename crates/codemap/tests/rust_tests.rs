use codemap::codemap;

#[test]
fn test_public_struct_with_public_field() {
    let input = r#"pub struct Simple { pub public_field: i32 }"#;
    let expected = r#"pub struct Simple {
    pub public_field: i32
}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_struct_with_private_field() {
    let input = r#"pub struct Simple { private_field: i32 }"#;
    let expected = r#"pub struct Simple {}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_struct_with_mixed_fields() {
    let input = r#"pub struct PublicPrivatePublic {
    pub public_field: i32,
    private_field: i32,
    pub pub_field: String,
}"#;
    let expected = r#"pub struct PublicPrivatePublic {
    pub public_field: i32,
    pub pub_field: String
}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_private_struct_no_fields() {
    let input = r#"struct PrivateNoFields;"#;
    let expected = r#""#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_private_struct_with_fields() {
    let input = r#"struct PrivateWithFields { private_field: i32 }"#;
    let expected = r#""#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_generic_struct() {
    let input = r#"pub struct Invite<ID> { pub id: ID }"#;
    let expected = r#"pub struct Invite<ID> {
    pub id: ID
}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_private_enum() {
    let input = r#"enum PrivateEnum {
    Variant1,
    Variant2,
    Variant3(String),
    Variant4 { field: i32 },
}"#;
    let expected = r#""#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_enum() {
    let input = r#"pub enum PublicEnum {
    Variant1,
    Variant2,
    Variant3(String),
    Variant4 { field: i32 },
}"#;
    let expected = r#"pub enum PublicEnum {
    Variant1,
    Variant2,
    Variant3(String),
    Variant4 { field: i32 },
}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_constant() {
    let input = r#"pub const CONSTANT: i32 = 42;"#;
    let expected = r#"pub const CONSTANT: i32 = 42;"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_struct_with_impl() {
    let input = r#"pub struct ImplStruct;

impl ImplStruct {
    pub fn public_method(param: i32) -> i32 {
        todo!()
    }

    fn private_method(&self, param: i32) -> i32 {
        todo!()
    }
}"#;
    let expected = r#"pub struct ImplStruct;

impl ImplStruct {
    pub fn public_method(param: i32) -> i32;
}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_function() {
    let input = r#"pub fn public_function(param: i32) -> i32 { todo!() }"#;
    let expected = r#"pub fn public_function(param: i32) -> i32;"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_async_function() {
    let input = r#"pub async fn public_async_function(param: i32) -> i32 { todo!() }"#;
    let expected = r#"pub async fn public_async_function(param: i32) -> i32;"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_private_function() {
    let input = r#"fn private_function(param: i32) -> i32 { todo!() }"#;
    let expected = r#""#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_module() {
    let input = r#"pub mod public_mod;"#;
    let expected = r#"pub mod public_mod;"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_private_module() {
    let input = r#"mod private_mod;"#;
    let expected = r#""#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_type_alias() {
    let input = r#"pub type PublicType = i32;"#;
    let expected = r#"pub type PublicType = i32;"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_private_type_alias() {
    let input = r#"type PrivateType = i32;"#;
    let expected = r#""#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_trait() {
    let input = r#"pub trait PublicTrait {
    fn public_method(&self, param: i32) -> i32;
}"#;
    let expected = r#"pub trait PublicTrait {
    fn public_method(&self, param: i32) -> i32;
}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_private_trait() {
    let input = r#"trait PrivateTrait {
    fn private_method(&self, param: i32) -> i32;
}"#;
    let expected = r#""#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_use() {
    let input = r#"pub use anyhow::Result;"#;
    let expected = r#"pub use anyhow::Result;"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_private_use() {
    let input = r#"use anyhow::Ok;"#;
    let expected = r#""#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_async_method() {
    let input = r#"pub struct ImplStruct;

impl ImplStruct {
    pub async fn public_async_method(param: i32) -> i32 {
        todo!()
    }
}"#;
    let expected = r#"pub struct ImplStruct;

impl ImplStruct {
    pub async fn public_async_method(param: i32) -> i32;
}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_method_with_self_param() {
    let input = r#"pub struct ImplStruct;

impl ImplStruct {
    pub fn public_method_with_self_param(self, param: i32) -> i32 {
        todo!()
    }
}"#;
    let expected = r#"pub struct ImplStruct;

impl ImplStruct {
    pub fn public_method_with_self_param(self, param: i32) -> i32;
}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_method_with_self_reference() {
    let input = r#"pub struct ImplStruct;

impl ImplStruct {
    pub fn public_self_referencing_method(&self, param: i32) -> i32 {
        todo!()
    }
}"#;
    let expected = r#"pub struct ImplStruct;

impl ImplStruct {
    pub fn public_self_referencing_method(&self, param: i32) -> i32;
}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_public_method_with_mut_self_reference() {
    let input = r#"pub struct ImplStruct;

impl ImplStruct {
    pub fn public_self_mutating_reference_method(&mut self, param: i32) -> i32 {
        todo!()
    }
}"#;
    let expected = r#"pub struct ImplStruct;

impl ImplStruct {
    pub fn public_self_mutating_reference_method(&mut self, param: i32) -> i32;
}"#;
    assert_eq!(codemap(input), expected);
}

#[test]
fn test_struct_with_tuple_fields() {
    let input = r#"pub struct Parameter(pub String, pub String);"#;
    let expected = r#"pub struct Parameter(pub String, pub String);"#;
    assert_eq!(codemap(input), expected);
}
