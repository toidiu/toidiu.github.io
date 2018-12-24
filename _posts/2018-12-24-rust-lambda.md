---
title: "Rust Lambda"
nav: blog
id: blog
categories:
  - lambda
  - aws
---


Given the announcement of the Lambda runtime, there is now an officially supported story around writing lambda functions in Rust. I wanted to try for myself and see the amount of effort needed to get a lambda function working, while also exploring some of the intracites involved.

This [blog post](https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/)describes how to use the lambda runtime to create a lambda function in Rust, so I wont detail the actua process. Instead I will try to expand on what is actually happening behind the scenes.


## why we need [[bin]] bootstrap and autobins = false
Since we are going to include a custom runtime(tokio runtime), we need the executable to be called 'bootstrap'. These settings allow us to dictate these settings.

## the difference between glibc and musl
hardware <-> kernel <-> user space

The kernel has exposes its api via a C standard. glibc and musl are two different implementations of this standard.
https://www.reddit.com/r/linuxmasterrace/comments/41q2m9/eli5_what_is_musl_and_glibc/cz4cy3k

`glibc` (GNU C Library) is the more commonly supported libc but Amazon instances require `musl`.
`musl` is lightweight, fast, simple, free, and strives to be correct in the sense of standards-conformance and safety.

## static vs dynamic lib
Not sure if musl is statically or dynamically linked.

## what is a lambda runtime?
The `lambda!` macro takes an optional field: tokio runtime. as per docs creating a runtime does the following:
(https://docs.rs/tokio/0.1.13/tokio/runtime/index.html)
- Spawn a background thread running a Reactor instance.
- Start a ThreadPool for executing futures.
- Run an instance of Timer per thread pool worker thread.

One can configue a runtime for example to increase the number of threads: https://docs.rs/tokio/0.1.13/tokio/runtime/struct.Builder.html

Again as per AWS docs the limits say that we can have 1024 threads: https://docs.aws.amazon.com/lambda/latest/dg/limits.html


## rustc supported targets
https://forge.rust-lang.org/platform-support.html

## why we need target.x86_64-unknown-linux-musl
The amazon linux instances require musl

## why we need linker = "x86_64-linux-musl-gcc"
We need to download the linker so that we can then compile for the musl target.


