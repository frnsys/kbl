mod format;
mod keymap;
mod keys;
mod parse;

use std::{path::PathBuf, str::FromStr};

use bpaf::Bpaf;
use keymap::KeyMap;

use format::*;

#[derive(Debug, Clone, Bpaf)]
enum Target {
    QMK,
    ZMK,
}
impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "qmk" => Ok(Self::QMK),
            "zmk" => Ok(Self::ZMK),
            _ => Err(format!("Invalid target: {s}")),
        }
    }
}
impl Target {
    fn format(&self, keymap: &KeyMap) -> String {
        match self {
            Target::QMK => QMK::format(keymap),
            Target::ZMK => ZMK::format(keymap),
        }
    }
}

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
/// Parse a keyboard layout to a keymap file.
struct Args {
    /// The target, either `qmk` or `zmk`.
    #[bpaf(positional)]
    target: Target,

    /// Keyboard layout to parse.
    #[bpaf(positional)]
    path: PathBuf,
}

fn main() {
    let opts = args().run();
    let keymap = KeyMap::load(&opts.path);
    let output = opts.target.format(&keymap);
    println!("{output}");
}
