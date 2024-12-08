![alt text](./McDev_thin_900x70.png "McDev_thin_900x70.png")

[![Latest version](https://img.shields.io/crates/v/passgen-lib.svg)](https://crates.io/crates/passgen-lib)
[![Download](https://img.shields.io/crates/d/passgen-lib.svg)](https://crates.io/crates/passgen-lib)
[![docs.rs](https://docs.rs/passgen-lib/badge.svg)](https://docs.rs/passgen-lib/)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://choosealicense.com/licenses/mit/)
[![Build Status](https://github.com/mammothcoding/passgen-lib/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/mammothcoding/passgen-lib/actions/workflows/rust.yml)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)

Readme на разных языках:
[EN](https://github.com/mammothcoding/passgen-lib/blob/master/README.md)
[RU](https://github.com/mammothcoding/passgen-lib/blob/master/README.ru.md)

# ⚙ Passgen-lib

Библиотека для генерации криптографически защищенных паролей/токенов и других наборов и последовательностей.

Используются [CSPRNGs](https://rust-random.github.io/book/guide-rngs.html#cryptographically-secure-pseudo-random-number-generators-csprngs) Isaac64Rng и Hc128Rng.

## Использование

#### Включение библиотеки в свой проект `Cargo.toml`:

```toml
[dependencies]
passgenlib = "1.0.4"
```

#### Создать стойкий токен, включающий все летералы, цифры и специальные символы длиной 30 символов:

```rust
let result = Passgen::default().generate(30);
```

#### Создать стойкий и удобный пароль длиной 8 символов:

```rust
let result = Passgen::default_strong_and_usab().generate(8);
```
#### Сгенерировать криптостойкую случайную строку из вашего набора длиной 8 символов:

```rust
let result = Passgen::new().set_custom_charset("bla@.321").generate(8);
```

### Пример интеграции библиотеки в нашем инструментарии [passgen-cmd](https://github.com/mammothcoding/passgen-cmd).

### Документация [тут](https://docs.rs/passgen-lib/).

## Лицензия

[MIT](https://choosealicense.com/licenses/mit/)