macro_rules! define_keys {
    { $($str:literal => $variant:ident => $code:literal),* $(,)? } => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
        pub enum Key {
            $(
                #[serde(alias = $str)]
                $variant,
            )*
        }

        impl std::fmt::Display for Key {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $(
                        Self::$variant => $code,
                    )*
                })
            }
        }
    };
}

define_keys! {
    "A" => A => "KC_A",
    "B" => B => "KC_B",
    "C" => C => "KC_C",
    "D" => D => "KC_D",
    "E" => E => "KC_E",
    "F" => F => "KC_F",
    "G" => G => "KC_G",
    "H" => H => "KC_H",
    "I" => I => "KC_I",
    "J" => J => "KC_J",
    "K" => K => "KC_K",
    "L" => L => "KC_L",
    "M" => M => "KC_M",
    "N" => N => "KC_N",
    "O" => O => "KC_O",
    "P" => P => "KC_P",
    "Q" => Q => "KC_Q",
    "R" => R => "KC_R",
    "S" => S => "KC_S",
    "T" => T => "KC_T",
    "U" => U => "KC_U",
    "V" => V => "KC_V",
    "W" => W => "KC_W",
    "X" => X => "KC_X",
    "Y" => Y => "KC_Y",
    "Z" => Z => "KC_Z",

    "0" => Num0 => "KC_0",
    "1" => Num1 => "KC_1",
    "2" => Num2 => "KC_2",
    "3" => Num3 => "KC_3",
    "4" => Num4 => "KC_4",
    "5" => Num5 => "KC_5",
    "6" => Num6 => "KC_6",
    "7" => Num7 => "KC_7",
    "8" => Num8 => "KC_8",
    "9" => Num9 => "KC_9",

    "|" => Pipe => "LSFT(KC_BSLS)",
    "+" => Plus => "LSFT(KC_EQL)",
    "-" => Minus => "KC_MINUS",
    "=" => Equal => "KC_EQL",
    "," => Comma => "KC_COMMA",
    "." => Period => "KC_DOT",
    ":" => Colon => "LSFT(KC_SCLN)",
    "^" => Caret => "LSFT(KC_6)",
    "$" => Dollar => "LSFT(KC_4)",
    "%" => Percent => "LSFT(KC_5)",
    ";" => Semicolon => "KC_SCLN",
    "?" => Question => "LSFT(KC_SLSH)",
    "!" => Exclamation => "LSFT(KC_1)",
    "#" => Octothorpe => "LSFT(KC_3)",
    "&" => Ampersand => "LSFT(KC_7)",
    "@" => Arobase => "LSFT(KC_2)",
    "*" => Asterisk => "LSFT(KC_8)",
    "/" => Slash => "KC_SLSH",
    "~" => Tilde => "LSFT(KC_GRV)",
    "`" => Backtick => "KC_GRAVE",
    "<" => LessThan => "LSFT(KC_COMMA)",
    ">" => GreaterThan => "LSFT(KC_DOT)",
    ")" => ParensRight => "LSFT(KC_0)",
    "(" => ParensLeft => "LSFT(KC_9)",
    "]" => BracketRight => "KC_RBRC",
    "[" => BracketLeft => "KC_LBRC",
    "}" => BraceRight => "LSFT(KC_RBRC)",
    "{" => BraceLeft => "LSFT(KC_LBRC)",
    "_" => Underscore => "LSFT(KC_MINUS)",
    "'" => Apostrophe => "KC_QUOTE",
    "\\" => Backslash => "KC_BACKSLASH",
    "\"" => Quote => "LSFT(KC_QUOTE)",

    "↑" => Up => "KC_UP",
    "↓" => Down => "KC_DOWN",
    "←" => Left => "KC_LEFT",
    "→" => Right => "KC_RIGHT",
    "⇞" => PageUp => "KC_PGUP",
    "⇟" => PageDown => "KC_PGDN",

    "⏶" => VolumeUp => "KC_VOLU",
    "⏷" => VolumeDown => "KC_VOLD",
    "⨯" => VolumeMute => "KC_MUTE",
    "☼" => BrightnessUp => "KC_BRIGHTNESS_UP",
    "☾" => BrightnessDown => "KC_BRIGHTNESS_DOWN",

    "´" => Aigu => "RALT(KC_QUOTE)",
    "ˋ" => Grave => "RALT(KC_GRAVE)",
    "¸" => Cedille => "RALT(KC_COMMA)",
    "ˆ" => Circonflex => "RALT(LSFT(KC_6))",
    "¨" => Trema => "RALT(LSFT(KC_QUOTE))",
    "€" => Euro => "RALT(KC_EQUAL)",

    "⎋" => Esc => "KC_ESC",
    "⇥" => Tab => "KC_TAB",
    "␣" => Space => "KC_SPACE",
    "⌫" => Backspace => "KC_BSPC",
    "⏎" => Enter => "KC_ENTER",
    "◆" => Meta => "KC_LGUI",
    "⇧" => Shift => "KC_LSFT",
    "⎈" => Ctrl => "KC_LCTL",
    "⎇" => Alt => "KC_LALT",

    "⇫" => CapsWord => "QK_CAPS_WORD_TOGGLE",

    "∅" => NotAllowed => "KC_NO",
    "⬚" => Unassigned => "KC_TRNS",
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub enum Modifier {
    Alt,
    Meta,
    Ctrl,
    Shift,
}
impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alt => "LALT",
                Self::Meta => "LGUI",
                Self::Ctrl => "LCTL",
                Self::Shift => "LSFT",
            }
        )
    }
}

/// A key for a tap action.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TapKey {
    Key(Key),
    Layer(String),
    ToggleLayer(String),
    OneShotMod(Modifier),
    OneShotLayer(String),
    Modified(Vec<Modifier>, Key),
}
impl std::fmt::Display for TapKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Key(key) => key.to_string(),
                Self::Layer(layer) => format!("MO({layer})"),
                Self::ToggleLayer(layer) => format!("TG({layer})"),
                Self::OneShotMod(modifier) => {
                    format!("OSM(MOD_{modifier})")
                }
                Self::OneShotLayer(layer) => {
                    format!("OSL({layer})")
                }
                Self::Modified(modifiers, key) => {
                    let mut s = String::new();
                    let mut depth = 0;
                    for modifier in modifiers {
                        s.push_str(&format!("{modifier}("));
                        depth += 1;
                    }
                    s.push_str(&key.to_string());
                    for _ in 0..depth {
                        s.push(')');
                    }
                    s
                }
            }
        )
    }
}

/// A key for a hold action.
#[derive(Debug, Clone)]
pub enum HoldKey {
    Layer(String),
    Modifier(Modifier),
}

/// A complete definition for a key.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum KeyDef {
    Tap(TapKey),
    TapHold(TapKey, HoldKey),
}
impl std::fmt::Display for KeyDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Tap(key) => key.to_string(),
                Self::TapHold(tap, hold) => {
                    match hold {
                        HoldKey::Layer(layer) => format!("LT({layer}, {tap})"),
                        HoldKey::Modifier(modifier) => format!("MT(MOD_{modifier}, {tap})"),
                    }
                }
            }
        )
    }
}
