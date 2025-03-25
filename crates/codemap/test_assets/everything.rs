pub struct Simple {
    pub public_field: i32,
}

pub struct PublicPrivatePublic {
    pub public_field: i32,
    private_field: i32,
    pub pub_field: String,
}

struct PrivateNoFields;

struct PrivateWithFields {
    private_field: i32,
}

enum PrivateEnum {
    Variant1,
    Variant2,
    Variant3(String),
    Variant4 { field: i32 },
}

pub enum PublicEnum {
    Variant1,
    Variant2,
    Variant3(String),
    Variant4 { field: i32 },
}

pub const CONSTANT: i32 = 42;

pub struct ImplStruct;

impl ImplStruct {
    pub fn public_method(&self, param: i32) -> i32 {
        todo!()
    }

    fn private_method(&self, param: i32) -> i32 {
        todo!()
    }
}

pub fn public_function(param: i32) -> i32 {
    todo!()
}

fn private_function(param: i32) -> i32 {
    todo!()
}

pub mod public_mod;

mod private_mod;

pub type PublicType = i32;

type PrivateType = i32;

pub trait PublicTrait {
    fn public_method(&self, param: i32) -> i32;
}

trait PrivateTrait {
    fn private_method(&self, param: i32) -> i32;
}

pub use anyhow::Result;

use anyhow::Ok;
