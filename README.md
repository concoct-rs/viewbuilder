<div align="center">
<h1>Viewbuilder</h1>
 <a href="https://crates.io/crates/viewbuilder">
    <img src="https://img.shields.io/crates/v/viewbuilder?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://concoct-rs.github.io/viewbuilder/viewbuilder/index.html">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
   <a href="https://github.com/concoct-rs/viewbuilder/actions">
    <img src="https://github.com/concoct-rs/viewbuilder/actions/workflows/ci.yml/badge.svg"
      alt="CI status" />
  </a>
</div>

<div align="center">
 <a href="https://github.com/concoct-rs/viewbuilder/tree/main/examples">Examples</a>
</div>

<br>

Cross-platform user interface framework for Rust.

This crate provides primitives for the backend of a UI.
It's built for use as a backend for [concoct](https://github.com/concoct-rs/concoct),
but you can bring your own state management tools or build your own framework using this as a backend.

## Features

- Cross-platform with desktop and mobile support
- Event handling with an HTML-like API
- CSS flexbox and grid layout with [taffy](https://github.com/DioxusLabs/taffy/)
- Accessibility with [accesskit](https://github.com/AccessKit/accesskit)
- High performance rendering with [rust-skia](https://github.com/rust-skia/rust-skia)

## Getting started
Instatllation is simple with:
```sh
cargo add viewbuilder --features full
```
If you encounter errors, please check the instructions for building [rust-skia](https://github.com/rust-skia/rust-skia).

## Examples

### Layout
```rust
let mut tree = LayoutTree::default();
let root = Layout::builder()
    .size(Size::from_points(100., 100.))
    .is_listening(true)
    .build(&mut tree);

tree.build_with_listener(root, |key, node| {
    dbg!(key, node);
});
```