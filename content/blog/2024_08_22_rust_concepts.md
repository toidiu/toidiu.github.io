+++
title = "Rust concepts"
date = 2024-08-22

[taxonomies]
tag = ["rust"]

[extra]
id = "blog-single"
+++

What is ownership, borrowing, and lifetimes.

<!-- more -->

- Ownership: prevents double free, accessing memory after free and ensuring
  memory safety
- Borrowing: prevents aliasing and data races. ensure single writer and multiple
  writes.
- Lifetimes: prevents invalid or dangling references. ensures that references
  remain valid and dont outlive the data they point to.

- Deref:
- Drop:
- Arc:
- Mutex:
- Send:
- Sync:
- PhantomData:
- Pin:
- Future:
- Task:
