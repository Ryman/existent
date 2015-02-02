Convenience traits to help clarify/rubify a common pattern in Rust code.

# Examples

## When
```rust
use existent::When;
let x = 100u32;

assert_eq!(None, x.when(x > 200));
assert_eq!(Some(x), x.when(x < 200));
```

## Unless
```rust
use existent::Unless;
let xs = vec!["", "Three", "", "Two", "One"];

let filtered = xs.into_iter()
                 .filter_map(|s| s.unless(s.is_empty()))
                 .collect::<Vec<&str>>();

assert_eq!(filtered, vec!["Three", "Two", "One"])
```

[dependencies.existent]

git = "https://github.com/Ryman/existent.git"
```
