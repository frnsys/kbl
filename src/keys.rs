macro_rules! define_keys {
    { $($str:literal => $variant:ident),* $(,)? } => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
        pub enum Key {
            $(
                #[serde(alias = $str)]
                $variant,
            )*
        }
    };
}

define_keys! {
    "A" => A,
    "B" => B,
    "C" => C,
    "D" => D,
    "E" => E,
    "F" => F,
    "G" => G,
    "H" => H,
    "I" => I,
    "J" => J,
    "K" => K,
    "L" => L,
    "M" => M,
    "N" => N,
    "O" => O,
    "P" => P,
    "Q" => Q,
    "R" => R,
    "S" => S,
    "T" => T,
    "U" => U,
    "V" => V,
    "W" => W,
    "X" => X,
    "Y" => Y,
    "Z" => Z,

    "0" => Num0,
    "1" => Num1,
    "2" => Num2,
    "3" => Num3,
    "4" => Num4,
    "5" => Num5,
    "6" => Num6,
    "7" => Num7,
    "8" => Num8,
    "9" => Num9,

    "|" => Pipe,
    "+" => Plus,
    "-" => Minus,
    "=" => Equal,
    "," => Comma,
    "." => Period,
    ":" => Colon,
    "^" => Caret,
    "$" => Dollar,
    "%" => Percent,
    ";" => Semicolon,
    "?" => Question,
    "!" => Exclamation,
    "#" => Octothorpe,
    "&" => Ampersand,
    "@" => Arobase,
    "*" => Asterisk,
    "/" => Slash,
    "~" => Tilde,
    "`" => Backtick,
    "<" => LessThan,
    ">" => GreaterThan,
    ")" => ParensRight,
    "(" => ParensLeft,
    "]" => BracketRight,
    "[" => BracketLeft,
    "}" => BraceRight,
    "{" => BraceLeft,
    "_" => Underscore,
    "'" => Apostrophe,
    "\\" => Backslash,
    "\"" => Quote,

    "↑" => Up,
    "↓" => Down,
    "←" => Left,
    "→" => Right,
    "⇞" => PageUp,
    "⇟" => PageDown,

    "⏶" => VolumeUp,
    "⏷" => VolumeDown,
    "⨯" => VolumeMute,
    "☼" => BrightnessUp,
    "☾" => BrightnessDown,

    "´" => Aigu,
    "ˋ" => Grave,
    "¸" => Cedille,
    "ˆ" => Circonflex,
    "¨" => Trema,
    "€" => Euro,

    "⎋" => Esc,
    "⇥" => Tab,
    "␣" => Space,
    "⌫" => Backspace,
    "⏎" => Enter,
    "◆" => Meta,
    "⇧" => Shift,
    "⎈" => Ctrl,
    "⎇" => Alt,

    "⤊" => MouseWheelUp,
    "⤋" => MouseWheelDown,

    "⇫" => CapsWord,

    "◐" => Bt0,
    "◑" => Bt1,
    "◌" => BtClear,

    "∅" => NotAllowed,
    "⬚" => Unassigned,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub enum Modifier {
    Alt,
    Meta,
    Ctrl,
    Shift,
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

/// A key for a hold action.
#[derive(Debug, Clone, PartialEq)]
pub enum HoldKey {
    Layer(String),
    Modifier(Modifier),
}

/// A complete definition for a key.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum KeyDef {
    Tap(TapKey),
    TapHold(TapKey, HoldKey),
}
