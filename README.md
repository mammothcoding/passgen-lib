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

## Usage

#### Include library to your project `Cargo.toml`:

```toml
[dependencies]
passgenlib = "1.0.3"
```

#### You can create a strong token including all leterals, numbers and special symbols with length 30 charasters:

```rust
let result = Passgen::default().generate(30);
```

#### You can create a strong and usability password with length 8 charasters:

```rust
let result = Passgen::default_strong_and_usab().generate(8);
```
#### You can create a set from your custom charset with length 8 charasters:

```rust
let result = Passgen::new().set_custom_charset("bla@.321").generate(8);
```

### More information in docs:
[Doc](https://docs.rs/passgen-lib/)

## License

[MIT](https://choosealicense.com/licenses/mit/)