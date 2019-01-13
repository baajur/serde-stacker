Serde stack growth adapter
==========================

[![Build Status](https://api.travis-ci.com/dtolnay/serde-stacker.svg?branch=master)](https://travis-ci.com/dtolnay/serde-stacker)
[![Latest Version](https://img.shields.io/crates/v/serde_stacker.svg)](https://crates.io/crates/serde_stacker)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/serde_stacker)

This crate provides a Serde adapter that avoids stack overflow by dynamically
growing the stack.

Be aware that you may need to protect against other recursive operations outside
of serialization and deserialization when working with deeply nested data,
including, but not limited to, Display and Debug and Drop impls.

```toml
[dependencies]
serde = "1.0"
serde_stacker = "0.0"
```

## Deserialization example

```rust
use serde::Deserialize;
use serde_json::Value;

fn main() {
    let mut json = String::new();
    for _ in 0..10000 {
        json = format!("[{}]", json);
    }

    let mut deserializer = serde_json::Deserializer::from_str(&json);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let value = Value::deserialize(deserializer).unwrap();

    carefully_drop_nested_arrays(value);
}

fn carefully_drop_nested_arrays(value: Value) {
    let mut stack = vec![value];
    while let Some(value) = stack.pop() {
        if let Value::Array(array) = value {
            stack.extend(array);
        }
    }
}
```

## Serialization example

```rust
use serde::Serialize;
use serde_json::Value;

fn main() {
    let mut value = Value::Null;
    for _ in 0..10000 {
        value = Value::Array(vec![value]);
    }

    let mut out = Vec::new();
    let mut serializer = serde_json::Serializer::new(&mut out);
    let serializer = serde_stacker::Serializer::new(&mut serializer);
    let result = value.serialize(serializer);

    carefully_drop_nested_arrays(value);

    result.unwrap();
    assert_eq!(out.len(), 10000 + 4 + 10000);
}

fn carefully_drop_nested_arrays(value: Value) {
    let mut stack = vec![value];
    while let Some(value) = stack.pop() {
        if let Value::Array(array) = value {
            stack.extend(array);
        }
    }
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
