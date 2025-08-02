use genco::prelude::*;

use crate::{
    keymap::{KeyMap, Layer, Shifted},
    keys::{HoldKey, Key, KeyDef, Modifier, TapKey},
};

use super::Format;

pub struct ZMK;

impl Format for ZMK {
    fn format(keymap: &KeyMap) -> String {
        let include = &keymap.include;
        let tokens: Tokens<C> = quote! {
            #include <behaviors.dtsi>
            #include <dt-bindings/zmk/keys.h>
            #include <dt-bindings/zmk/bt.h>

            $(for (i, Layer { name, .. }) in keymap.layers.iter().enumerate() join($['\r']) =>
                #define $name $i
            )

            / {
                behaviors {
                    $include

                    $(for Shifted { name, input, output } in keymap.shifts() join($['\r']) {
                        $name: $name {
                            compatible = "zmk,behavior-mod-morph";
                            #binding-cells = <0>;
                            bindings = <$(kd(input))>, <$(kd(output))>;
                            mods = <(MOD_LSFT|MOD_RSFT)>;
                        };
                    })
                };

                combos {
                    compatible = "zmk,combos";
                    $(for ComboDef { name, positions, output, layer } in combos(&keymap.layers) join($['\r']) =>
                        $name {
                            timeout-ms = <200>;
                            key-positions = <$positions>;
                            bindings = <$(kd(output))>;
                            layers = <$layer>;
                        };
                    )
                };

                keymap {
                    compatible = "zmk,keymap";

                    $(for Layer { name, layout, shifts, .. } in &keymap.layers join($['\r']) =>
                        $name {
                            bindings = <
                                $(for key in layout join(  ) =>
                                    $(skd(key, shifts))
                                )
                            >;
                        };
                    )
                };
            };
        };
        tokens.to_file_string().unwrap()
    }
}

struct ComboDef<'a> {
    name: String,
    positions: String,
    output: &'a KeyDef,
    layer: &'a str,
}

fn combos<'a>(layers: &'a [Layer]) -> impl Iterator<Item = ComboDef<'a>> {
    layers.iter().enumerate().flat_map(|(i, layer)| {
        layer
            .combos
            .iter()
            .enumerate()
            .map(move |(j, combo)| ComboDef {
                layer: &layer.name,
                name: format!("combo_{i}_{j}"),
                output: &combo.output,
                positions: combo
                    .inputs
                    .iter()
                    .map(|inp| inp.position.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
            })
    })
}

fn kc(key: &Key) -> &'static str {
    match key {
        Key::A => "A",
        Key::B => "B",
        Key::C => "C",
        Key::D => "D",
        Key::E => "E",
        Key::F => "F",
        Key::G => "G",
        Key::H => "H",
        Key::I => "I",
        Key::J => "J",
        Key::K => "K",
        Key::L => "L",
        Key::M => "M",
        Key::N => "N",
        Key::O => "O",
        Key::P => "P",
        Key::Q => "Q",
        Key::R => "R",
        Key::S => "S",
        Key::T => "T",
        Key::U => "U",
        Key::V => "V",
        Key::W => "W",
        Key::X => "X",
        Key::Y => "Y",
        Key::Z => "Z",

        Key::Num0 => "N0",
        Key::Num1 => "N1",
        Key::Num2 => "N2",
        Key::Num3 => "N3",
        Key::Num4 => "N4",
        Key::Num5 => "N5",
        Key::Num6 => "N6",
        Key::Num7 => "N7",
        Key::Num8 => "N8",
        Key::Num9 => "N9",

        Key::Pipe => "LS(BACKSLASH)",
        Key::Plus => "LS(EQUAL)",
        Key::Minus => "MINUS",
        Key::Equal => "EQUAL",
        Key::Comma => "COMMA",
        Key::Period => "DOT",
        Key::Colon => "LS(SEMICOLON)",
        Key::Caret => "LS(N6)",
        Key::Dollar => "LS(N4)",
        Key::Percent => "LS(N5)",
        Key::Semicolon => "SEMICOLON",
        Key::Question => "LS(SLASH)",
        Key::Exclamation => "LS(N1)",
        Key::Octothorpe => "LS(N3)",
        Key::Ampersand => "LS(N7)",
        Key::Arobase => "LS(N2)",
        Key::Asterisk => "LS(N8)",
        Key::Slash => "SLASH",
        Key::Tilde => "LS(GRAVE)",
        Key::Backtick => "GRAVE",
        Key::LessThan => "LS(COMMA)",
        Key::GreaterThan => "LS(DOT)",
        Key::ParensRight => "LS(N0)",
        Key::ParensLeft => "LS(N9)",
        Key::BracketRight => "RBKT",
        Key::BracketLeft => "LBKT",
        Key::BraceRight => "LS(RBKT)",
        Key::BraceLeft => "LS(LBKT)",
        Key::Underscore => "LS(MINUS)",
        Key::Apostrophe => "SINGLE_QUOTE",
        Key::Backslash => "BACKSLASH",
        Key::Quote => "LS(SINGLE_QUOTE)",

        Key::Up => "UP",
        Key::Down => "DOWN",
        Key::Left => "LEFT",
        Key::Right => "RIGHT",
        Key::PageUp => "PG_UP",
        Key::PageDown => "PG_DN",

        Key::VolumeUp => "C_VOL_UP",
        Key::VolumeDown => "C_VOL_DN",
        Key::VolumeMute => "C_MUTE",
        Key::BrightnessUp => "C_BRI_INC",
        Key::BrightnessDown => "C_BRI_DEC",

        Key::Aigu => "RA(SINGLE_QUOTE)",
        Key::Grave => "RA(GRAVE)",
        Key::Cedille => "RA(COMMA)",
        Key::Circonflex => "RA(LS(N6))",
        Key::Trema => "RA(LS(SINGLE_QUOTE))",
        Key::Euro => "RA(EQUAL)",

        Key::Esc => "ESC",
        Key::Tab => "TAB",
        Key::Space => "SPACE",
        Key::Backspace => "BSPC",
        Key::Enter => "ENTER",
        Key::Meta => "LGUI",
        Key::Shift => "LSHIFT",
        Key::Ctrl => "LCTRL",
        Key::Alt => "LALT",

        // No appropriate keycode.
        // NOTE: This may indicate that the intermediate
        // representation should not treat these as normal keys?
        // See for example how `NotAllowed`, `Unassigned`, and `CapsWord`
        // are handled below. They are more like normal keys in QMK
        // but less so in ZMK.
        Key::MouseWheelUp => "none",
        Key::MouseWheelDown => "none",
        Key::CapsWord => "none",
        Key::NotAllowed => "none",
        Key::Unassigned => "none",
    }
}

fn mc(key: &Modifier) -> &'static str {
    match key {
        Modifier::Alt => "LALT",
        Modifier::Meta => "LGUI",
        Modifier::Ctrl => "LCTRL",
        Modifier::Shift => "LSHIFT",
    }
}

fn m(key: &Modifier) -> &'static str {
    match key {
        Modifier::Alt => "LA",
        Modifier::Meta => "LG",
        Modifier::Ctrl => "LC",
        Modifier::Shift => "LS",
    }
}

fn tk(key: &TapKey) -> String {
    match key {
        TapKey::Key(key) => match key {
            Key::CapsWord => "&caps_word".to_string(),
            Key::Unassigned | Key::NotAllowed => "&none".to_string(),
            _ => format!("&kp {}", kc(key)),
        },
        TapKey::Layer(layer) => format!("&mo {layer}"),
        TapKey::ToggleLayer(layer) => format!("&tog {layer}"),
        TapKey::OneShotMod(modifier) => {
            let modifier = mc(modifier);
            format!("&sk {modifier}")
        }
        TapKey::OneShotLayer(layer) => {
            format!("&sl {layer}")
        }
        TapKey::Modified(modifiers, key) => {
            let mut s = String::from("&kp ");
            let mut depth = 0;
            for modifier in modifiers {
                let modifier = m(modifier);
                s.push_str(&format!("{modifier}("));
                depth += 1;
            }
            s.push_str(kc(&key));
            for _ in 0..depth {
                s.push(')');
            }
            s
        }
    }
}

fn hk(hold: &HoldKey, tap: &TapKey) -> String {
    let key = match tap {
        TapKey::Key(key) => kc(key),
        _ => panic!("The tap key in a hold-tap definition must be a regular key."),
    };
    match hold {
        HoldKey::Layer(layer) => format!("&lt {layer} {key}"),
        HoldKey::Modifier(modifier) => {
            let modifier = mc(modifier);
            format!("&mt {modifier} {key}")
        }
    }
}

fn kd(keydef: &KeyDef) -> String {
    match keydef {
        KeyDef::Tap(key) => tk(key),
        KeyDef::TapHold(tap, hold) => hk(hold, tap),
    }
}

/// A key that may have a shift override.
fn skd(key: &KeyDef, shifts: &[Shifted]) -> String {
    if let Some(Shifted { name, .. }) = shifts.iter().find(|s| &s.input == key) {
        format!("&{name}")
    } else {
        kd(key)
    }
}
