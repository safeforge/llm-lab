use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;

pub trait Example: Send + Sync {
    fn main(&self) -> Result<()>;
}

pub static REGISTER: LazyLock<HashMap<&'static str, Box<dyn Example>>> = LazyLock::new(|| {
    let m: HashMap<&'static str, Box<dyn Example + 'static>> = HashMap::new();

    m
});
