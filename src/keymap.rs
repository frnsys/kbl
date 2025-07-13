use std::path::Path;

use crate::{keys::*, parse};

#[derive(Default)]
pub struct KeyMap {
    pub layers: Vec<Layer>,
    pub shifts: Vec<Shifted>,
    pub include: String,
}
impl KeyMap {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        parse::parse_keymap(path)
    }

    pub fn combos(&self) -> impl Iterator<Item = &Combo> {
        self.layers.iter().flat_map(|layer| layer.combos.iter())
    }
}

pub struct Layer {
    pub name: String,
    pub layout: Layout,
    pub combos: Vec<Combo>,
}

pub type Layout = Vec<KeyDef>;

/// Represents a particular key and position in a layer.
pub struct LayerKey {
    pub key: KeyDef,

    /// Key's position in the layer.
    pub position: usize,
}

pub struct Combo {
    pub inputs: Vec<LayerKey>,
    pub output: KeyDef,
}

/// An override for a `Shift+Key` behavior.
pub struct Shifted {
    pub input: KeyDef,
    pub output: KeyDef,
}
