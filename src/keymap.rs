use std::path::Path;

use crate::{keys::*, parse};

#[derive(Default)]
pub struct KeyMap {
    pub layers: Vec<(String, Layout)>,
    pub combos: Vec<Combo>,
    pub shifts: Vec<Shifted>,
    pub include: String,
}
impl KeyMap {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        parse::parse_keymap(path)
    }
}

pub type Layout = Vec<KeyDef>;

pub struct Combo {
    pub inputs: Vec<KeyDef>,
    pub output: KeyDef,
}
pub struct Shifted {
    pub input: KeyDef,
    pub output: KeyDef,
}
