# ERDP
[![Crates.io Version](https://img.shields.io/crates/v/erdp)](https://crates.io/crates/erdp)

ERDP is a small Rust crate with zero dependencies to help you display an error. If you use `std::fmt::Display` to display a `std::error::Error` like the following code:

```rust
use std::fs::File;
use std::path::{Path, PathBuf};
use thiserror::Error;

fn main() {
    let path = PathBuf::from("config.json");

    if let Err(e) = load_config(&path) {
        eprintln!("Failed to load {}: {}.", path.display(), e);
    }
}

fn load_config(path: &Path) -> Result<(), MyError> {
    let file = match File::open(path) {
        Ok(v) => v,
        Err(e) => return Err(MyError::OpenFileFailed(e)),
    };

    Ok(())
}

#[derive(Debug, Error)]
enum MyError {
    #[error("couldn't open the specified file")]
    OpenFileFailed(#[source] std::io::Error),
}
```

What you get is just a message from a top-level error:

```
Failed to load config.json: couldn't open the specified file.
```

With this crate you can use `display` method on the error value like:

```rust
use erdp::ErrorDisplay;

eprintln!("Failed to load {}: {}.", path.display(), e.display());
```

Then the output will change to something like:

```
Failed to load config.json: couldn't open the specified file -> No such file or directory.
```

## Breaking changes in 0.2

- ERDP now depend on `alloc` crate.
- Rust minimum version now 1.85.

## License

MIT
