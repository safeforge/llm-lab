use std::{collections::HashMap, sync::LazyLock};
use types::Example;

pub mod ch02;

pub static REGISTER: LazyLock<HashMap<&'static str, Box<dyn Example>>> = LazyLock::new(|| {
    let mut m: HashMap<&'static str, Box<dyn Example + 'static>> = HashMap::new();

    m.insert("02.01", Box::new(ch02::EG01));
    m
});
