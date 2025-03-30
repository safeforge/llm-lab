use std::{collections::HashMap, sync::LazyLock};
use types::Example;


pub static REGISTER: LazyLock<HashMap<&'static str, Box<dyn Example>>> = LazyLock::new(|| {
    let m: HashMap<&'static str, Box<dyn Example + 'static>> = HashMap::new();

    m
});
