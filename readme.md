# kbl

Allows QMK and ZMK keymaps to be written in (what I find to be) a nicer way, compiling to the standard `keymap.c` for QMK and `*.keymap` for ZMK.

The idea is that keymaps are defined visually, and additional configuration such as combos are grouped with their layers.

## Installation

```bash
cargo install --git https://github.com/frnsys/kbl
```

## Usage

A layout file might look like this, with each layer separated by `---`:

```
ALPHA

  G M L      Y F U
D S T N R  I A E O '
  W C P      H , .
      ⇧ ⌫  ⏎ ␣

holds:
  ⌫: Meta
  ⏎: Layer(NAVCTL)
  ␣: Ctrl
  T: Layer(SYMBOL)
  E: Layer(NUMBER)
combos:
  [E, O]: ":"
  [S, N]: Esc
  [A, O]: =
  [G, M]: Q
  [W, C]: Z
  [C, P]: V
  [S, T]: B
  [A, E]: K
  [M, L]: J
  [F, U]: [X, Layer(SYSTEM)]
shifts:
  ",": "-"
  ".": _
  "'": '"'

---

SYMBOL

  c ⬚ p      # [ ]
/ * ∅ ` ~  @ { ( ) }
  ⬚ ⬚ ⬚      ? < >
      ⇥ &  ! ;

defs:
    c: Ctrl(C)
    p: Ctrl(V)

---

NUMBER

  1 2 3      ¸ ⬚ ˆ
^ 4 5 6 $  € + ∅ % ´
  7 8 9      ˋ ⬚ ¨
      0 |  ⬚ \

---

NAVCTL

  ⇲ ⍐ ⇱      ❮ ↑ ❯
⇧ 1 2 3 ↶  ⎶ ← ↓ → ⊕
  ⏷ ▶ ⏶      ◱ ✛ ⌕
      ⍜ ⍈  ∅ ⬚

defs:
  1: Meta(1)        # Desktop 1
  2: Meta(2)        # Desktop 2
  3: Meta(3)        # Desktop 3
  ❮: Alt([)         # Prev term tab
  ❯: Alt(])         # Next term tab
  ⊕: Alt(T)         # New tab
  ⎶: Alt(-)         # Horizontal split
  ◱: Alt(M)         # Maximize term window
  ✛: Alt(W)         # Focus term window
  ⌕: Alt(')         # Fuzzy search
  ⍐: Meta(Ctrl(K))  # Focus monitor
  ⍈: Meta(H)        # Focus window
  ↶: Ctrl(Z)        # Undo
  ⇱: Meta(Shift(←)) # Size up window
  ⇲: Meta(Shift(→)) # Size down window
  ⍜: Meta(␣)        # Launcher
  ▶: Meta(=)        # Play/pause music
combos:
  [⏷, ⏶]: ⨯         # Mute
shifts:
  ⍐: Meta(Ctrl(M))  # Move window to monitor
  1: Meta(Shift(1)) # Move window to desktop 1
  2: Meta(Shift(2)) # Move window to desktop 2
  3: Meta(Shift(3)) # Move window to desktop 3
  ▶: Meta(=)        # Play/pause mpv
  ⏶: ☼              # Brightness up
  ⏷: ☾              # Brightness down
  ⊕: Alt(E)         # Terminal scrollback history
  ⎶: Alt(\)         # Terminal vertical split
  ↶: Ctrl(Shift(Z)) # Redo
```

which can then be compiled:

```bash
kbl qmk layout.kbl > /path/to/my/qmk/keymaps/keymap.c
kbl zmk layout.kbl > /path/to/my/zmk/config/something.keymap
```

To include literal code into the generated file you can add a final section starting with `===` and include the code after it, e.g. for QMK:

```
===

// Disable Liatris power LED
#include "gpio.h"
void keyboard_pre_init_user(void) {
    gpio_set_pin_output(24);
    gpio_write_pin_high(24);
}
```


## Notes & Limitations

- The visual layout definition relies on each key being defined by a single character. As such many keys are mapped to special characters, e.g. "␣" for "space". Refer to `keys.rs` to see these mappings.
  - Note that `⬚` is used to indicate an unassigned key.
- This only supports a small portion of QMK and ZMK features; i.e. the ones that I use, which also happen to only be features shared between both. So far this includes tap-hold, shift key overrides, and combos.
- The layer config is parsed as YAML, so the usual YAML caveats apply. In particular there are some characters which will need to be quoted, such as `:`, `-`, and `"`; basically any characters that are special for YAML. Numbers may also need to be quoted so they're parsed as strings.
