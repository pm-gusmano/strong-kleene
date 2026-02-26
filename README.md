# strong-kleene

`strong-kleene` provides `Trit`, a three-valued logic value with strong
Kleene semantics.

Compatibility: this crate is `no_std`.

## Quickstart

```rust
use strong_kleene::Trit;

let a = Trit::True;
let b = Trit::Unknown;

assert_eq!(!a, Trit::False);
assert_eq!(a & b, Trit::Unknown);
assert_eq!(Trit::False | b, Trit::Unknown);
assert_eq!(a ^ Trit::False, Trit::True);
```

## How-To

### Convert from `bool`

```rust
use strong_kleene::Trit;

let trit_true: Trit = true.into();
let trit_false: Trit = false.into();
assert_eq!(trit_true, Trit::True);
assert_eq!(trit_false, Trit::False);
```

### Convert into `bool` safely

```rust
use strong_kleene::{Trit, UnknownToBoolError};

let known = Trit::True;
let unknown = Trit::Unknown;

assert_eq!(bool::try_from(known), Ok(true));
assert_eq!(bool::try_from(unknown), Err(UnknownToBoolError));
```

### Branch on `Unknown`

```rust
use strong_kleene::Trit;

let value = Trit::Unknown;

match value {
    Trit::True => {}
    Trit::False => {}
    Trit::Unknown => {}
}
assert!(value.is_unknown());
```

## Semantics

[Strong Kleene logic](https://en.wikipedia.org/wiki/Three-valued_logic#Kleene_and_Priest_logics)
extends classical two-valued logic with `Unknown` for values that are not
currently known.

Legend: `F = false`, `U = unknown`, `T = true`.

### `NOT(A)`

| A | `!A` |
|:-:|:----:|
| F |  T   |
| U |  U   |
| T |  F   |

### `AND(A, B)`

| A \\ B | F | U | T |
|:------:|:--:|:--:|:--:|
| **F**  | F | F | F |
| **U**  | F | U | U |
| **T**  | F | U | T |

### `OR(A, B)`

| A \\ B | F | U | T |
|:------:|:--:|:--:|:--:|
| **F**  | F | U | T |
| **U**  | U | U | T |
| **T**  | T | T | T |

### `XOR(A, B)`

| A \\ B | F | U | T |
|:------:|:--:|:--:|:--:|
| **F**  | F | U | T |
| **U**  | U | U | U |
| **T**  | T | U | F |

## Reference

- `Trit` is the primary type and supports `!`, `&`, `|`, and `^`.
- `UnknownToBoolError` is returned by `bool::try_from(Trit::Unknown)`.
