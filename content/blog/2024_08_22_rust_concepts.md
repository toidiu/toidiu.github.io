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

**Ownership:**

prevents double free, accessing memory after free and ensuring memory safety

**Borrowing:** prevents aliasing and data races. ensure single writer and multiple
  writes.

**Lifetimes:** prevents invalid or dangling references. ensures that references
  remain valid and dont outlive the data they point to.

**Deref:**

**Drop:**

**Arc:**

**Mutex:**

**Send:**

**Sync:**

**PhantomData:**

**Pin:**

Definition of “pinning"
- Not be moved out of its memory location
- More generally, remain valid at that same memory location

“Pinning” allows us to put a value which exists at some location in memory into
a state where safe code cannot move that value to a different location in memory
or otherwise invalidate it at its current location

**Unpin**
The vast majority of Rust types have no address-sensitive states. These types
implement the Unpin auto-trait, which cancels the restrictive effects of Pin
when the pointee type T is Unpin.

When T: Unpin, Pin<Box<T>> functions identically to a non-pinning Box<T>;
similarly, Pin<&mut T> would impose no additional restrictions above a regular
&mut T.

**Future:**

**Task:**
