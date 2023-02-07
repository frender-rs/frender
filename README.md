# frender

[![Crates.io](https://img.shields.io/crates/v/frender?style=for-the-badge)](https://crates.io/crates/frender)
[![docs.rs](https://img.shields.io/docsrs/frender/latest?style=for-the-badge)](https://docs.rs/frender)
[![GitHub license](https://img.shields.io/github/license/frender-rs/frender?style=for-the-badge)](https://github.com/frender-rs/frender/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/frender-rs/frender?style=for-the-badge)](https://github.com/frender-rs/frender/stargazers)

<div style="text-align:center;margin:16px">

![frender logo](logo.svg)

Functional Rendering: `React` in `Rust`

</div>

_**f**render_ is still in alpha and it's api might change.
For now it is recommended to specify the exact version in `Cargo.toml`.
Before updating, please see the full [changelog](https://github.com/frender-rs/frender/blob/alpha/CHANGELOG.md) in case there are breaking changes.

There are some example apps in
[`examples`](https://github.com/frender-rs/frender/tree/alpha/examples)
folder. You can preview them at [this site](https://frender-rs.github.io/frender/).

You can quick start with the
[frender book](https://frender-rs.github.io/frender/book).

## Features

- Functional components and hooks
- Statically typed props
- Forced immutability with [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)

## Future Development Plans

- [ ] Documentation
- [ ] Intrinsic svg components
- [ ] Export `frender` components to js
- [ ] Server Side Rendering
- [ ] Type checking for `CssProperties`
- [ ] Css-in-rust (For example, integrate with [`emotion/react`](https://emotion.sh/docs/@emotion/react))
- [ ] Performance benchmarking

## Contributing

`frender` is open sourced at [GitHub](https://github.com/frender-rs/frender).
Pull requests and issues are welcomed.

You can also [sponsor me](https://ko-fi.com/equalma) and I would be very grateful :heart:

[![Buy Me a Coffee at ko-fi.com](https://cdn.ko-fi.com/cdn/kofi2.png?v=3)](https://ko-fi.com/N4N26J11L)

## Development

```
cargo run --bin frender-html-expand
```
