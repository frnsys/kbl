use std::{collections::HashMap, path::Path};

use crate::{
    keymap::{Combo, KeyMap, Layer, LayerKey, Shifted},
    keys::*,
};

#[derive(Debug, Default, serde::Deserialize)]
struct LayerConfig {
    /// Define optional hold actions for keys.
    #[serde(default)]
    holds: HashMap<char, HoldKey>,

    /// If a key is more complex than a "normal"
    /// key, e.g. mods plus a key, define it here.
    #[serde(default)]
    defs: HashMap<char, TapKey>,

    /// Define combos associated with this layer.
    #[serde(default)]
    combos: HashMap<Vec<char>, KeyDef>,

    /// Shift overrides associated with this layer.
    #[serde(default)]
    shifts: HashMap<char, KeyDef>,
}

pub fn parse_keymap<P: AsRef<Path>>(path: P) -> KeyMap {
    let keymap = fs_err::read_to_string(path).expect("Unable to read file");
    let (keymap, include) = keymap
        .split_once("\n===\n")
        .unwrap_or_else(|| (keymap.as_str(), ""));
    let layers = keymap.split("\n---\n");

    let mut keymap = KeyMap {
        include: include.to_string(),
        ..Default::default()
    };

    for layer in layers {
        // Expect three parts delimited by two line breaks:
        // 1. The layer name
        // 2. The layer layout
        // 3. The layer config (optional)
        let parts: Vec<&str> = layer.trim().splitn(3, "\n\n").collect();

        let name = parts[0];
        let layout = parts[1];
        let mut config: LayerConfig = parts
            .get(2)
            .map(|config| serde_yaml::from_str(config).unwrap())
            .unwrap_or_default();

        // Figure out mapping of chars to the key definitions.
        let mut layer_def = vec![];
        let mut keys: HashMap<char, KeyDef> = HashMap::default();
        for ch in layout.chars().filter(|ch| !ch.is_whitespace()) {
            // If this char is mapped to a specific definition, use that.
            // Otherwise use the default definition.
            let tap = if let Some(def) = config.defs.get(&ch) {
                def.clone()
            } else {
                TapKey::try_from(ch).unwrap()
            };

            // Include a hold action, if one's specified.
            let key = match config.holds.remove(&ch) {
                Some(hold) => KeyDef::TapHold(tap, hold),
                None => KeyDef::Tap(tap),
            };

            keys.insert(ch, key.clone());
            layer_def.push(key);
        }

        let combos = config
            .combos
            .into_iter()
            .map(|(inps, output)| {
                let inputs: Vec<_> = inps
                .into_iter()
                .map(|ch| {
                    if let Some(key) = keys.get(&ch) {
                        let position = layer_def.iter().position(|k| k == key).unwrap();
                        LayerKey {
                            key: key.clone(),
                            position,
                        }
                    } else {
                        panic!(
                            "Layer {name}: Expected {:?} for a combo, but it isn't in the layer.",
                            ch
                        );
                    }
                })
                .collect();
                Combo { inputs, output }
            })
            .collect();
        keymap
            .shifts
            .extend(config.shifts.into_iter().map(|(input, output)| {
                let input = keys
                    .get(&input)
                    .cloned()
                    .unwrap_or_else(|| KeyDef::Tap(TapKey::try_from(input).unwrap()));
                Shifted { input, output }
            }));
        keymap.layers.push(Layer {
            name: name.to_string(),
            layout: layer_def,
            combos,
        });
    }
    keymap
}

// FIXME: Hacky
impl<'de> serde::Deserialize<'de> for TapKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;

        if let Ok(key) = Key::try_from(s) {
            return Ok(TapKey::Key(key));
        }

        if let Some(layer) = s.strip_prefix("Enable(").and_then(|s| s.strip_suffix(")")) {
            return Ok(TapKey::Layer(layer.to_string()));
        }

        if let Some(layer) = s.strip_prefix("Toggle(").and_then(|s| s.strip_suffix(")")) {
            return Ok(TapKey::ToggleLayer(layer.to_string()));
        }

        if let Some(s) = s.strip_prefix("OneShot(").and_then(|s| s.strip_suffix(")")) {
            if let Ok(modifier) = serde_yaml::from_str(s) {
                return Ok(TapKey::OneShotMod(modifier));
            } else {
                return Ok(TapKey::OneShotLayer(s.to_string()));
            }
        }

        let (mods, key) = parse_nested(s);
        let mods = mods
            .into_iter()
            .map(serde_yaml::from_str::<Modifier>)
            .collect::<Result<Vec<Modifier>, _>>();

        let key_def =
            mods.and_then(|mods| Key::try_from(key).map(|key| TapKey::Modified(mods, key)));

        key_def.map_err(|_err| serde::de::Error::custom(format!("Invalid enum variant: {}", s)))
    }
}

// FIXME: Hacky
impl<'de> serde::Deserialize<'de> for HoldKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        if let Ok(key) = serde_yaml::from_str::<Modifier>(s) {
            return Ok(HoldKey::Modifier(key));
        }
        if let Some(layer) = s.strip_prefix("Layer(").and_then(|s| s.strip_suffix(")")) {
            return Ok(HoldKey::Layer(layer.to_string()));
        }
        Err(serde::de::Error::custom(format!(
            "Invalid enum variant: {}",
            s
        )))
    }
}

impl TryFrom<&str> for Key {
    type Error = serde_yaml::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // FIXME feels hacky
        let inp = if s == "\\" {
            "'\\'".to_string()
        } else if s == "\"" {
            "'\"'".to_string()
        } else {
            format!(r#""{s}""#)
        };
        serde_yaml::from_str::<Key>(&inp)
    }
}

impl TryFrom<char> for Key {
    type Error = serde_yaml::Error;
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        (ch.to_string().as_str()).try_into()
    }
}

impl TryFrom<char> for TapKey {
    type Error = serde_yaml::Error;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        Key::try_from(ch).map(TapKey::Key)
    }
}

/// Given "A(B(C(X)))", this extracts
/// `([A, B, C], X)`
fn parse_nested(s: &str) -> (Vec<&str>, &str) {
    let mut stack = Vec::new();
    let mut start = 0;
    let mut last_value = "";

    for (i, c) in s.char_indices() {
        match c {
            '(' => {
                stack.push(&s[start..i]);
                start = i + 1; // Move past '('
            }
            ')' => {
                last_value = &s[start..i]; // Capture the final value
                break;
            }
            _ => {}
        }
    }

    (stack, last_value)
}
