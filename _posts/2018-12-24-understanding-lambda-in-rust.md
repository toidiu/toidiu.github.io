---
title: "Understanding Lambda in Rust"
nav: blog
id: blog
categories:
  - Rust
  - lambda
  - aws
---


Given the announcement of the Lambda runtime, there is now an officially supported story around writing lambda functions in Rust. I wanted to try for myself and see the amount of effort needed to get a lambda function working, while also diving deeper into whats involved.

This [blog post](https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/) describes how to use the lambda runtime to create a lambda function in Rust, so I wont detail the actual process. Instead I will try to expand on what is actually happening behind the scenes and reasons for certain steps.

## Why we need "[[bin]] bootstrap" and "autobins = false"
Since we are going to include a custom runtime(tokio runtime), we need the executable to be called 'bootstrap'. These settings allow us to dictate these settings.

## What is "glibc" and "musl" and whats the difference
This is a much larger topic but I will try to cover the relevant pieces. A computer comprises of layers of abstractions. So for example, if we wish to execute a task in our application, we request a thread from the kernel, which schedules our task to run on a core.

```
  hardware(cores) <-> kernel(thread management) <-> user space(application)
```

So to access resources of a computer we need to communicate with the kernel. The kernel in turn exposes the API via a C standard called the *libc*. `glibc` (GNU C Library) and `musl` are two different implementations of this standard.

`glibc` is the most commonly supported libc but Amazon instances require `musl`. From it's site: `musl` is lightweight, fast, simple, free, and strives to be correct in the sense of standards-conformance and safety.

## static vs dynamic lib
Not sure if musl is statically or dynamically linked???

## what is a lambda runtime?
The `lambda!` macro takes an optional field: tokio runtime. As per [docs](https://docs.rs/tokio/0.1.13/tokio/runtime/index.html) creating a runtime does the following:
- Spawn a background thread running a Reactor instance.
- Start a ThreadPool for executing futures.
- Run an instance of Timer per thread pool worker thread.

One can imagine configuring a runtime to specify the number of threads. Configuration options for the tokio runtime can be found in [docs](https://docs.rs/tokio/0.1.13/tokio/runtime/struct.Builder.html)

As per AWS [docs](https://docs.aws.amazon.com/lambda/latest/dg/limits.html) there is a default limit of 1024 threads/processes per function.

## rustc supported targets

## Why we need target.x86_64-unknown-linux-musl
The amazon linux instances require `musl`. For reference here is a [list](https://forge.rust-lang.org/platform-support.html) of all the target that the Rust compiler supports!

## Why we need linker = "x86_64-linux-musl-gcc"
We need to download the linker so that we can locally compile for the musl target.

#### References:
- https://www.reddit.com/r/linuxmasterrace/comments/41q2m9/eli5_what_is_musl_and_glibc/cz4cy3k
