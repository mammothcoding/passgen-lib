![alt text](./McDev_thin_900x70.png "McDev_thin_900x70.png")

[![Latest version](https://img.shields.io/crates/v/passgen-lib.svg)](https://crates.io/crates/passgen-lib)
[![Download](https://img.shields.io/crates/d/passgen-lib.svg)](https://crates.io/crates/passgen-lib)
[![docs.rs](https://docs.rs/passgen-lib/badge.svg)](https://docs.rs/passgen-lib/)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://choosealicense.com/licenses/mit/)
[![Build Status](https://github.com/mammothcoding/passgen-lib/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/mammothcoding/passgen-lib/actions/workflows/rust.yml)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)

Readme in different languages:
[EN](https://github.com/mammothcoding/passgen-lib/blob/master/README.md)
[RU](https://github.com/mammothcoding/passgen-lib/blob/master/README.ru.md)

# ⚙ Passgen-lib

Library for generating cryptographically secure passwords/tokens and other sets and sequences.

[CSPRNGs](https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs) Isaac64Rng and Hc128Rng are used.

![alt text](./passgen-lib_demo.gif "passgen-lib_demo.gif")

## Usage

#### Include library to your project `Cargo.toml`:
```toml
[dependencies]
passgen-lib = "1.2.1"
```

#### You can create a token that includes lowercase letters and numbers up to 30 characters long:
```rust
let result = Passgen::new().set_enabled_letters(true).set_enabled_numbers(true).generate(30);
```

#### You can create a default strong password including all literals, numbers and symbols:
```rust
let result = Passgen::default().generate(12);
```

#### You can create a strong and usability password with 8 characters long.
Including all characters, but
the first position in the password is a capital or small letter,
the last position is the symbol.

🔸 Excluded ambiguous characters `"0oOiIlL1"`.
```rust
let result = Passgen::default_strong_and_usab().generate(8);
```

#### You can create a set from your custom charset 12 characters long:
```rust
let result = Passgen::new().set_custom_charset("abcABC123⭕➖❎⚫⬛п₼⁂🙂").generate(12);
```

### Example of library integration in the [passgen-cmd](https://github.com/mammothcoding/passgen-cmd) tool and [passgen-telegram](https://github.com/mammothcoding/passgen-telegram) service.

### Library [doc](https://docs.rs/passgen-lib/).

## License

[MIT](https://choosealicense.com/licenses/mit/)

### Our other passgen projects:
[passgen-desktop](https://github.com/mammothcoding/passgen-desktop)

[passgen-console-linuxwin](https://github.com/mammothcoding/passgen-console-linuxwin)

[passgen-cmd](https://github.com/mammothcoding/passgen-cmd)

[passgen-telegram](https://github.com/mammothcoding/passgen-telegram)
