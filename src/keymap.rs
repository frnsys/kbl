use std::path::Path;

use crate::{keys::*, parse};

#[derive(Default)]
pub struct KeyMap {
    pub layers: Vec<Layer>,
    pub include: String,
}
impl KeyMap {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        parse::parse_keymap(path)
    }

    pub fn combos(&self) -> impl Iterator<Item = &Combo> {
        self.layers.iter().flat_map(|layer| layer.combos.iter())
    }

    pub fn shifts(&self) -> impl Iterator<Item = &Shifted> {
        self.layers.iter().flat_map(|layer| layer.shifts.iter())
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors: Vec<String> = vec![];
        let layers: Vec<_> = self.layers.iter().map(|layer| &layer.name).collect();

        for key in self.keys() {
            match key {
                KeyDef::Tap(tap_key) => match tap_key {
                    TapKey::Layer(layer)
                    | TapKey::ToggleLayer(layer)
                    | TapKey::OneShotLayer(layer) => {
                        if !layers.contains(&layer) {
                            errors.push(format!("Missing layer: {layer}"));
                        }
                    }
                    _ => {}
                },
                KeyDef::TapHold(tap_key, hold_key) => {
                    match tap_key {
                        TapKey::Layer(layer)
                        | TapKey::ToggleLayer(layer)
                        | TapKey::OneShotLayer(layer) => {
                            if !layers.contains(&layer) {
                                errors.push(format!("Missing layer: {layer}"));
                            }
                        }
                        _ => {}
                    }
                    if let HoldKey::Layer(layer) = hold_key
                        && !layers.contains(&layer)
                    {
                        errors.push(format!("Missing layer: {layer}"));
                    }
                }
            }
        }
        errors
    }

    /// Iterator over all keys defined in this keymap.
    fn keys(&self) -> impl Iterator<Item = &KeyDef> {
        self.layers.iter().flat_map(|layer| {
            layer
                .keys()
                .chain(layer.shifts.iter().map(|shifted| &shifted.output))
        })
    }
}

pub struct Layer {
    pub name: String,
    pub layout: Layout,
    pub combos: Vec<Combo>,
    pub shifts: Vec<Shifted>,
}
impl Layer {
    pub fn keys(&self) -> impl Iterator<Item = &KeyDef> {
        self.layout
            .iter()
            .chain(self.combos.iter().map(|combo| &combo.output))
    }
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
    pub name: String,
    pub input: KeyDef,
    pub output: KeyDef,
}
