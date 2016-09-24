ncurses-static-rs
==========

This is a very thin wrapper around the ncurses TUI lib.

It is a fork of [ncurses-rs](https://github.com/jeaye/ncurses-rs) that
statically links to `ncurses`.

To add as a dependency to your project, use the following in your
`Cargo.toml`:

```toml
[dependencies.ncurses]
git = "https://github.com/mjhoy/ncurses-static-rs"
features = ["wide"]
```

## Building

The compiled library will go to the `target` directory.

```
cargo build
```

This also downloads and builds `ncurses`. It may take a minute or
two. (See the included git submodule for what gets built.)

## Examples

Examples are built by `cargo build`. To run them, use `cargo run --example ex_<NUMBER>`. Example numbers increase along with the complexity of the example.

Current examples:  
**1.** [Hello World](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_1.rs)  
**2.** [Basic Input & Attributes](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_2.rs)  
**3.** [Simple Pager](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_3.rs)  
**4.** [Window Movement](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_4.rs)  
**5.** [Menu Library](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_5.rs) (requires rust nightly)  
**6.** [Pager & Syntax Highlighting](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_6.rs)  
**7.** [Basic Input & Attributes (Unicode)](https://github.com/jeaye/ncurses-rs/blob/master/examples/ex_7.rs)  
