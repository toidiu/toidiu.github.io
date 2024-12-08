+++
title = "Rust's Future ecosystem"
date = 2022-01-25

[taxonomies]
tag = ["Rust", "future"]

[extra]
id = "blog-single"
+++

The Rust future ecosystem is quite complex and in this post I hope to explore
and shed light on the different pieces.
<!-- more -->

Note: the Futures ecosystem is still evolving and will likely change as the
Rust community agrees on best practices. It is therefore best to consider this
post as a snapshot in time.

Why I think the ecosystem is so complicated:
- futures/asynchronous programming is complicated
- the ecosystem was developed and used outside the standard library and then
  slowly introduced into the library
- async is a lot of syntax sugar. It hides the complexity but just because its
  hidden doesn't mean its not there
- the ecosystem is being developed and is currently a bit fragmented

### Different future crates
- [futures-rs](https://docs.rs/futures/latest/futures/) from the repo
  [https://github.com/rust-lang/futures-rs](https://github.com/rust-lang/futures-rs) and
  website https://rust-lang.github.io/futures-rs/
  - [futures-core](https://docs.rs/futures-core/latest/futures_core/)
  - [futures-util](https://docs.rs/futures-util/latest/futures_util/)
  - [futures-channel](https://docs.rs/futures-channel/latest/futures_channel/)
- [std future](https://doc.rust-lang.org/std/future/) comes from the repo
  [https://github.com/rust-lang/rust](https://github.com/rust-lang/rust)
- [task module](https://doc.rust-lang.org/std/task/index.html)


### futures-rs
futures-rs is a project before there were futures in Rust. The goal of the
project was to experiment with futures before officially supporting it in the
Rust standard library. The project was a success and from it came the [Future
trait](https://doc.rust-lang.org/std/future/trait.Future.html). The trait is
what allowed the rust team to then build an deliver the async feature that the
community very much wanted for the 2018 edition.

However, there is still so much more functionality that is part of the
futures-rs project and a lot is already being used. The futures crate is the
entire collection of all functionality exposed by futures-rs while the
futures-* is the project broken into smaller modular pieces.

So what does futures-rs offer? Stream, Channels, IO, task, and helpers to make
it more convenient to work with futures!

### std future
The standard future crate (which is part of the Rust standard library) offers
the **Future** trait. This trait is what allows the Rust community to write
interoperable code.

### task module
While not actually a future, this module contains some core concepts necessary
for writing asynchronous programs.

We need a runtime to actually make progress in a program. For this post lets consider the
work stealing tokio runtime. Rather than spawn threads, tokio, utilizes tasks (or green
threads) to complete work. So within a single thread a task might run until it makes an
async non-blocking call (file read, network send, timeout, etc.).

The non-blocking is important because it allows the runtime to utilize the thread to run
another task instead (ie. the network call is waiting for some packets to arrive and
doesn't actually need cpu resources). This cooperative non-blocking scheduling by the
executor allows for much denser workloads to occur compared to the thread per task model.

In this cooperative scheduling, a task (as it is known in tokio) is the unit of work
(rather than a thread). A task should contains all the information to start, make progress
and finish the work. Being asynchronous, a task expects Future, which it can start, and
then `poll()` to completion.

- **Poll**: the result returned from calling `poll` on a Future. Can either be Ready or
  Pending.
- **Context**: each task calls `poll(ctx: Context)`. Context contains a Waker.
- **Waker**: notifies the runtime executor that a task is ready and should be polled
  again.
