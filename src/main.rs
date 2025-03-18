mod keymap;
mod keys;
mod parse;

use std::path::PathBuf;

use bpaf::Bpaf;
use genco::prelude::*;
use keymap::{Combo, KeyMap, Shifted};
use keys::KeyDef;

impl FormatInto<C> for &KeyDef {
    fn format_into(self, tokens: &mut Tokens<C>) {
        let s = self.to_string();
        quote_in!(*tokens => $s);
    }
}

impl KeyMap {
    fn to_c(&self) -> String {
        let include = &self.include;
        let tokens: Tokens<C> = quote! {
            #include QMK_KEYBOARD_H
            #define LAYOUT LAYOUT_split_3x5_2

            $include

            enum layers {
                $(for layer_name in self.layers.iter().map(|(name, _)| name) join(,) => $layer_name)
            };

            const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
                $(for (layer_name, layout) in &self.layers join(,$['\r']) =>
                    [$layer_name] = LAYOUT(
                        $(for key in layout join(, ) =>
                            $key
                        )
                    )
                )
            };

            // Combos
            $(for (i, combo) in self.combos.iter().enumerate() join($['\r']) {
                const uint16_t PROGMEM combo_$i[]   = {$(for inp in &combo.inputs join(, ) => $inp), COMBO_END};
            })
            combo_t key_combos[] = {
                $(for (i, Combo { output, .. }) in self.combos.iter().enumerate() join(,$['\r']) {
                    COMBO(combo_$i, $output)
                })
            };

            // Shift overrides
            $(for (i, Shifted { input, output }) in self.shifts.iter().enumerate() join($['\r']) {
                const key_override_t shift_$i = ko_make_basic(MOD_MASK_SHIFT, $input, $output);
            })
            const key_override_t *key_overrides[] = {
                $(for (i, _) in self.shifts.iter().enumerate() join(,$['\r']) => &shift_$i)
            };
        };
        tokens.to_file_string().unwrap()
    }
}

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
/// Parse a keyboard layout to a QMK keymap.c file.
struct Args {
    /// Keyboard layout to parse.
    #[bpaf(positional)]
    path: PathBuf,
}

fn main() {
    let opts = args().run();
    let keymap = KeyMap::load(&opts.path);
    println!("{}", keymap.to_c());
}
