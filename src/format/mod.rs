mod qmk;
mod zmk;

use crate::keymap::KeyMap;
pub use qmk::QMK;
pub use zmk::ZMK;

pub trait Format {
    fn format(keymap: &KeyMap) -> String;
}
