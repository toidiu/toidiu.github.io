+++
title = "Rust primitives"
date = 2024-08-22

[taxonomies]
tag = ["rust"]

[extra]
id = "blog-single"
+++

**WIP**. An attempt to plainly explain Rust primitives and why they matter.

<!-- more -->

## Table of Contents
- [Borrow Checker](#borrow-checker)
  - [Ownership](#ownership) todo
  - [Lifetimes](#lifetimes) todo
- [Memory Aliasing modules](#memory-aliasing-modules)
  - [Shared mutability](#shared-mutability) todo
    - [Cell](#cell) todo
    - [RefCell](#refcell) todo
  - [Shared ownership](#shared-ownership) todo
    - [Rc](#rc) todo
  - [Synchronization](#synchronization)
    - [Mutex](#mutex) todo
    - [Arc](#arc) todo
- [Thread-safety](#thread-safety) todo
  - [Send](#send)
  - [Sync](#sync)
- [Traits](#traits)
  - [Deref](#deref) todo
  - [Drop](#drop) todo
  - [PhantomData](#phantomdata) todo
  - [Unpin](#unpin)
    - [Pin](#pin-struct)
    - [PhantomPinned](#phantompinned) todo

## Borrow Checker

> In computing, aliasing describes a situation in which a data location in memory can be
> accessed through different symbolic names in the program. Thus, modifying the data
> through one name implicitly modifies the values associated with all aliased names, which
> may not be expected by the programmer. As a result, aliasing makes it particularly
> difficult to understand, analyze and optimize programs.

In my opinion, the biggest value that Rust provides is making it easier to reason about
[aliasing](https://en.wikipedia.org/wiki/Aliasing_(computing)). At the end of the day it
comes down to where is your data, who owns it and who is accessing it. The Borrow Checker
provides abstractions which allows the Rust compiler to track and reason about these
aliasing rules and prevent misuse.

Unlike C, Rust attempts to automatically track if some data is being referenced, or read
or written to or already cleaned up. By tracking this automatically, it is able to prevent
common pitfalls we run into in C and also able to make performance optimizations that
would be very risky to do when tracking aliasing manually.

The following post describes some core Rust primitives that Rust provides to help us
reason about aliasing.

### Ownership:
Prevents double free, accessing memory after free and ensuring memory safety.

### Borrowing:
Prevents aliasing and data races. ensure single writer and multiple writes.

### Lifetimes:
prevents invalid or dangling references. ensures that references remain valid and dont
outlive the data they point to.

## Memory Aliasing modules
To help us deal with aliasing rules Rust provides some containers that a allow us to
safetly alias memory while also allowing the compiler to reason about them.

- `mod cell` gives us shared mutability
- `mod rc` gives us shared ownership
- `mod sync` gives us synchronization

### Shared mutability

#### Cell
[Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html): interior mutability by
copying/moving the interior value

```
// assert_eq!(min_path(&[&[1, 2], &[3, 4]]), 3);
let cell = Cell::new(3);
assert_eq!(cell.get(), 3); // duplicate inner value
assert_eq!(cell.take(), 3); // take and replace with default value
assert_eq!(cell.get(), 0); // assert default value
```

#### RefCell
[RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html): runtime "dynamic
borrowing" to avoid copy/move and give exclusive mutable access

```
// RefCell::replace()
// RefCell::swap()
// RefCell::borrow()
// RefCell::borrow_mut()

use std::cell::RefCell;

#[derive(Debug)]
struct NoCopyu32(u32);

let ref_cell = RefCell::new(NoCopyu32(3));
let num: &NoCopyu32 = &ref_cell.borrow();
assert_eq!(num.0, 3);

let num_mut: &mut NoCopyu32 = &mut ref_cell.borrow_mut();
num_mut.0 = 4;
assert_eq!(num.0, 4);
```

### Shared ownership

#### Rc
[Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)
```
use std::rc::Rc;

#[derive(Debug)]
struct NoCopyu32(u32);

let rc = Rc::new(NoCopyu32(3));
let clone = rc.clone();

assert!(Rc::ptr_eq(&rc, &clone));
assert!(rc.0 == clone.0);
assert!(Rc::strong_count(&rc) == 2);

let rc_other = Rc::new(NoCopyu32(3));
assert!(rc.0 == rc_other.0);
assert!(!Rc::ptr_eq(&rc, &rc_other));
```

### Synchronization
Synchronization is useful when a program is operating with multiple threads and wishes to
share data across those threads. Shared mutation across threads requires excusive access
(i.e. atomics or locks).

The [sync](https://doc.rust-lang.org/std/sync/index.html) module also has a pretty decent
explanation which I recommend reading.

#### Mutex
[Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
todo

#### Arc
[Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
todo


## Thread-safety

You know its complicated when there is a
[chapter](https://doc.rust-lang.org/nomicon/send-and-sync.html) for it in the
Rust Nomicon. Send and Sync traits help the Rust compiler reason about thread safety. They
are used to mark data that can be shared or moved across different threads. Typically this
is not safe to do for and requires the use of synchronization.


### Send:
[Send](https://doc.rust-lang.org/std/marker/trait.Send.html) marks objects that can be
sent to different threads (its ok to move ownership to a different thread).

Some types that are marked Send:
- RefCell is a type that [is
Send](https://doc.rust-lang.org/std/cell/struct.RefCell.html#impl-Send-for-RefCell%3CT%3E)
but [not
Sync](https://doc.rust-lang.org/std/cell/struct.RefCell.html#impl-Sync-for-RefCell%3CT%3E).

The reason why RefCell doesn't implement Sync is because it doesn't synchronize interior
mutability (mutating internal state). This means that multiple threads could concurrenlty
attempt to mutate the internal state and result in a race condition. RefCell is allowed to
be Send since its fine to mutate internal state within one thread if we transfer ownership
of RefCell to that thread.

### Sync:
[Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html) marks objects which are safe
to share among threads (its ok to borrow a reference across threads).

> The precise definition is: a type T is Sync if and only if &T is Send. In other words,
> if there is no possibility of undefined behavior (including data races) when passing &T
> references between threads.

Or more concisely:

> &T and &mut T are Sync if and only if T is Sync

Some types that are marked Sync:
- `impl Sync for Arc`
- `impl<T: ?Sized + Send> Sync for Mutex<T>`

Arc and Mutex implement Sync since they use synchronization (i.e. atomics and locks) to
ensure access to the data is excusive even across multiple threads.


## <a name="traits">Traits</a>
The Rust type system is rich and allows us to express complex concepts to the compiler.

### Deref:
todo
https://doc.rust-lang.org/std/ops/trait.Deref.html

### Drop:
todo
https://doc.rust-lang.org/std/ops/trait.Drop.html

### PhantomData:
todo
https://doc.rust-lang.org/std/marker/struct.PhantomData.html

### Unpin
The vast majority of Rust types have no address-sensitive states. For example, an integer
can be copies to another address while mainting the correct semantic. Most types implement
the [Unpin](https://doc.rust-lang.org/std/marker/trait.Unpin.html) auto-trait by default.

On the contrary, an object which is self-referential (contains a pointer to itself), will
become invalid if the struct is moved to a different address. These objects need to be
`Pin`ed (see below) to prevent them from being accidentally move.

#### Pin (struct):
If an object should not be moved, then the user can `Pin` it by wrapping the object in a
`Pin<T>`. The [pin](https://doc.rust-lang.org/std/pin/index.html) module defines the
concept of "pinning" and has more detailed info.

But do we really need to care about pining? It seems complicated with little benefits. Why
does the compiler care if a value is moved or not? If a user is writing self-referential
objects, they should be responsible for doing a proper copy/move.

> All values in Rust are trivially moveable. This means that the address at which a value
> is located is not necessarily stable in between borrows. The compiler is allowed to move
> a value to a new address without running any code to notify that value that its address
> has changed.

Ok so the compiler can "move" an object at will. Sure that makes sense but its not very
satisfying and doesn't explain why the compiler need to move objects. The answer is
`async` programming in Rust.

In-order to support `async`, Rust had to essentially crate implicit Future objects that
could be polled, yield, and return. These objects also had to store state about the
progress of the Future (aka. self-referential objects). To seamlessly support async
programming, the compile needed to reason about object moves and hence the need for
`Unpin` auto-trait.

See [comment](https://users.rust-lang.org/t/high-level-pin-async-future/71876/2) and book
[chapter](https://rust-lang.github.io/async-book/04_pinning/01_chapter.html) for further
details.

#### PhantomPinned:
https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html

