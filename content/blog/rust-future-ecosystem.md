+++
title = "Rust Future ecosystem"
date = 2022-01-25

[taxonomies]
tag = ["Rust", "future"]

[extra]
id = "blog-single"
+++

The Rust future ecosystem is quite complex and in this post I hope to explore and shed light on the different pieces.
<!-- more -->

There are a few reasons why the ecosystem is so complicated:
- futures/asynchronous programming is complicated
- the ecosystem was developed and used outside the standard library and then slowly introduced into the library
- async is mostly syntax sugar. it hides the complexity but its not always possible to use async and just because its hidden doesnt mean it goes away

### Different future crates
- [futures-rs](https://docs.rs/futures/latest/futures/) from the repo [https://github.com/rust-lang/futures-rs](https://github.com/rust-lang/futures-rs) and website https://rust-lang.github.io/futures-rs/
  - [futures-core](https://docs.rs/futures-core/latest/futures_core/)
  - [futures-util](https://docs.rs/futures-util/latest/futures_util/)
  - [futures-channel](https://docs.rs/futures-channel/latest/futures_channel/)
- [std future](https://doc.rust-lang.org/std/future/) comes from the repo [https://github.com/rust-lang/rust](https://github.com/rust-lang/rust)

futures-rs is a project before there were futures in Rust. The goal of the project was to experiment with futures before officially supporting it in the Rust standard library. The project was a success and from it came the [Future trait](https://doc.rust-lang.org/std/future/trait.Future.html). The trait is what allowed the rust team to then build an deliver the async feature that the community very much wanted for the 2018 edition.

However, there is still so much more functionality that is part of the futures-rs project and alot is already being used. The futures crate is the entire collection of all funtionality exposed by futures-rs while the futures-* is the project broken into smaller modular pieces.

So what does futures-rs offer? Stream, Channels, IO, task, and helpers to make it more convenient to work with futures!
