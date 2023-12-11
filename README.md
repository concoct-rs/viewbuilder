<div align="center">
<h1>Viewbuilder</h1>
 <a href="https://crates.io/crates/viewbuilder">
    <img src="https://img.shields.io/crates/v/viewbuilder?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/viewbuilder/latest/viewbuilder/">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
   <a href="https://github.com/matthunz/viewbuilder/actions">
    <img src="https://github.com/matthunz/viewbuilder/actions/workflows/ci.yml/badge.svg"
      alt="CI status" />
  </a>
</div>

<div align="center">
 <a href="https://github.com/matthunz/viewbuilder/tree/main/examples">Examples</a>
</div>

<br>

A cross-platform user interface framework for Rust.

Viewbuilder is a moduler GUI library that can be used as an entire framework, or with individual parts.

```rust
use viewbuilder::{object, Object, Runtime};

#[derive(Default)]
pub struct Counter {
    value: i32,
}

#[object]
impl Counter {
    fn value_changed(&mut self, value: i32);

    #[slot]
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
        self.value_changed(value);
    }
}

#[tokio::main]
async fn main() {
    let rt = Runtime::default();
    let _guard = rt.enter();

    let a = Counter::default().spawn();
    let b = Counter::default().spawn();

    a.value_changed().bind(&b, Counter::set);
    a.set_value(2);

    rt.run().await;

    assert_eq!(a.borrow().value, 2);
    assert_eq!(b.borrow().value, 2);
}
```

## Getting started

Instatllation is simple with:

```sh
cargo add viewbuilder --features full
```
