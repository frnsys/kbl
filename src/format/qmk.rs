use super::Format;
use crate::{
    keymap::{Combo, KeyMap, Layer, LayerKey, Shifted},
    keys::{HoldKey, Key, KeyDef, Modifier, TapKey},
};

use genco::prelude::*;

pub struct QMK;

impl Format for QMK {
    fn format(keymap: &KeyMap) -> String {
        let include = &keymap.include;
        let tokens: Tokens<C> = quote! {
            #include QMK_KEYBOARD_H
            #define LAYOUT LAYOUT_split_3x5_2

            $include

            enum layers {
                $(for Layer { name, .. } in &keymap.layers join(,) => $name)
            };

            const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
                $(for Layer { name, layout, .. } in &keymap.layers join(,$['\r']) =>
                    [$name] = LAYOUT(
                        $(for key in layout join(, ) =>
                            $(kd(key))
                        )
                    )
                )
            };

            // Combos
            $(for (i, combo) in keymap.combos().enumerate() join($['\r']) {
                const uint16_t PROGMEM combo_$i[] = {$(for LayerKey { key, .. } in &combo.inputs join(, ) => $(kd(key))), COMBO_END};
            })
            combo_t key_combos[] = {
                $(for (i, Combo { output, .. }) in keymap.combos().enumerate() join(,$['\r']) {
                    COMBO(combo_$i, $(kd(output)))
                })
            };

            // Shift overrides
            $(for (i, Shifted { input, output }) in keymap.shifts.iter().enumerate() join($['\r']) {
                const key_override_t shift_$i = ko_make_basic(MOD_MASK_SHIFT, $(kd(input)), $(kd(output)));
            })
            const key_override_t *key_overrides[] = {
                $(for (i, _) in keymap.shifts.iter().enumerate() join(,$['\r']) => &shift_$i)
            };
        };
        tokens.to_file_string().unwrap()
    }
}

fn kc(key: &Key) -> &'static str {
    match key {
        Key::A => "KC_A",
        Key::B => "KC_B",
        Key::C => "KC_C",
        Key::D => "KC_D",
        Key::E => "KC_E",
        Key::F => "KC_F",
        Key::G => "KC_G",
        Key::H => "KC_H",
        Key::I => "KC_I",
        Key::J => "KC_J",
        Key::K => "KC_K",
        Key::L => "KC_L",
        Key::M => "KC_M",
        Key::N => "KC_N",
        Key::O => "KC_O",
        Key::P => "KC_P",
        Key::Q => "KC_Q",
        Key::R => "KC_R",
        Key::S => "KC_S",
        Key::T => "KC_T",
        Key::U => "KC_U",
        Key::V => "KC_V",
        Key::W => "KC_W",
        Key::X => "KC_X",
        Key::Y => "KC_Y",
        Key::Z => "KC_Z",

        Key::Num0 => "KC_0",
        Key::Num1 => "KC_1",
        Key::Num2 => "KC_2",
        Key::Num3 => "KC_3",
        Key::Num4 => "KC_4",
        Key::Num5 => "KC_5",
        Key::Num6 => "KC_6",
        Key::Num7 => "KC_7",
        Key::Num8 => "KC_8",
        Key::Num9 => "KC_9",

        Key::Pipe => "LSFT(KC_BSLS)",
        Key::Plus => "LSFT(KC_EQL)",
        Key::Minus => "KC_MINUS",
        Key::Equal => "KC_EQL",
        Key::Comma => "KC_COMMA",
        Key::Period => "KC_DOT",
        Key::Colon => "LSFT(KC_SCLN)",
        Key::Caret => "LSFT(KC_6)",
        Key::Dollar => "LSFT(KC_4)",
        Key::Percent => "LSFT(KC_5)",
        Key::Semicolon => "KC_SCLN",
        Key::Question => "LSFT(KC_SLSH)",
        Key::Exclamation => "LSFT(KC_1)",
        Key::Octothorpe => "LSFT(KC_3)",
        Key::Ampersand => "LSFT(KC_7)",
        Key::Arobase => "LSFT(KC_2)",
        Key::Asterisk => "LSFT(KC_8)",
        Key::Slash => "KC_SLSH",
        Key::Tilde => "LSFT(KC_GRV)",
        Key::Backtick => "KC_GRAVE",
        Key::LessThan => "LSFT(KC_COMMA)",
        Key::GreaterThan => "LSFT(KC_DOT)",
        Key::ParensRight => "LSFT(KC_0)",
        Key::ParensLeft => "LSFT(KC_9)",
        Key::BracketRight => "KC_RBRC",
        Key::BracketLeft => "KC_LBRC",
        Key::BraceRight => "LSFT(KC_RBRC)",
        Key::BraceLeft => "LSFT(KC_LBRC)",
        Key::Underscore => "LSFT(KC_MINUS)",
        Key::Apostrophe => "KC_QUOTE",
        Key::Backslash => "KC_BACKSLASH",
        Key::Quote => "LSFT(KC_QUOTE)",

        Key::Up => "KC_UP",
        Key::Down => "KC_DOWN",
        Key::Left => "KC_LEFT",
        Key::Right => "KC_RIGHT",
        Key::PageUp => "KC_PGUP",
        Key::PageDown => "KC_PGDN",

        Key::VolumeUp => "KC_VOLU",
        Key::VolumeDown => "KC_VOLD",
        Key::VolumeMute => "KC_MUTE",
        Key::BrightnessUp => "KC_BRIGHTNESS_UP",
        Key::BrightnessDown => "KC_BRIGHTNESS_DOWN",

        Key::Aigu => "RALT(KC_QUOTE)",
        Key::Grave => "RALT(KC_GRAVE)",
        Key::Cedille => "RALT(KC_COMMA)",
        Key::Circonflex => "RALT(LSFT(KC_6))",
        Key::Trema => "RALT(LSFT(KC_QUOTE))",
        Key::Euro => "RALT(KC_EQUAL)",

        Key::Esc => "KC_ESC",
        Key::Tab => "KC_TAB",
        Key::Space => "KC_SPACE",
        Key::Backspace => "KC_BSPC",
        Key::Enter => "KC_ENTER",
        Key::Meta => "KC_LGUI",
        Key::Shift => "KC_LSFT",
        Key::Ctrl => "KC_LCTL",
        Key::Alt => "KC_LALT",

        Key::MouseWheelUp => "QK_MOUSE_WHEEL_UP",
        Key::MouseWheelDown => "QK_MOUSE_WHEEL_DOWN",

        Key::CapsWord => "QK_CAPS_WORD_TOGGLE",

        Key::NotAllowed => "KC_NO",
        Key::Unassigned => "KC_TRNS",
    }
}

fn mc(key: &Modifier) -> &'static str {
    match key {
        Modifier::Alt => "LALT",
        Modifier::Meta => "LGUI",
        Modifier::Ctrl => "LCTL",
        Modifier::Shift => "LSFT",
    }
}

fn tk(key: &TapKey) -> String {
    match key {
        TapKey::Key(key) => kc(key).to_string(),
        TapKey::Layer(layer) => format!("MO({layer})"),
        TapKey::ToggleLayer(layer) => format!("TG({layer})"),
        TapKey::OneShotMod(modifier) => {
            let modifier = mc(modifier);
            format!("OSM(MOD_{modifier})")
        }
        TapKey::OneShotLayer(layer) => {
            format!("OSL({layer})")
        }
        TapKey::Modified(modifiers, key) => {
            let mut s = String::new();
            let mut depth = 0;
            for modifier in modifiers {
                let modifier = mc(modifier);
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
    let tap = tk(tap);
    match hold {
        HoldKey::Layer(layer) => format!("LT({layer}, {tap})"),
        HoldKey::Modifier(modifier) => {
            let modifier = mc(modifier);
            format!("MT(MOD_{modifier}, {tap})")
        }
    }
}

fn kd(keydef: &KeyDef) -> String {
    match keydef {
        KeyDef::Tap(key) => tk(key),
        KeyDef::TapHold(tap, hold) => hk(hold, tap),
    }
}

