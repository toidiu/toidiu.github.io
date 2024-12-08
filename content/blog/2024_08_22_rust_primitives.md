+++
title = "Rust primitives"
date = 2024-08-22

[taxonomies]
tag = ["rust"]

[extra]
id = "blog-single"
+++

An attempt to plainly explain some Rust primitives you are likely to come across.

<!-- more -->

## Table of Contents
- [Borrow Checker](#borrow)
- [Shared mutability/ownership](#mutability)
- [Traits](#traits)
- [Asyn](#async)

## <a name="borrow">Borrow Checker</a>
### Ownership:
prevents double free, accessing memory after free and ensuring memory safety

### Borrowing:
prevents aliasing and data races. ensure single writer and multiple writes.

### Lifetimes:
prevents invalid or dangling references. ensures that references remain valid
and dont outlive the data they point to.


## <a name="mutability">Shared mutability/ownership</a>

cell (shared mutability) vs rc (shared ownership) vs sync (synchronization)

### cell: shared mutability
- [Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html): interior mutability by copying/moving the interior value
```
// assert_eq!(min_path(&[&[1, 2], &[3, 4]]), 3);
let cell = Cell::new(3);
assert_eq!(cell.get(), 3); // duplicate inner value
assert_eq!(cell.take(), 3); // take and replace with default value
assert_eq!(cell.get(), 0); // assert default value
```

- [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html): runtime "dynamic borrowing" to avoid copy/move and give exclusive mutable access
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

### rc: shared ownership
- [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)
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

### sync
- [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)

## <a name="async">Async</a>

### Send:
todo
### Sync:
https://doc.rust-lang.org/nomicon/send-and-sync.html
https://doc.rust-lang.org/std/marker/trait.Send.html
https://doc.rust-lang.org/std/marker/trait.Sync.html

Types which are safe to share references between threads.

Auto Impl
`impl Sync for Arc`
`impl<T: ?Sized + Send> Sync for Mutex<T>`

> The precise definition is: a type T is Sync if and only if &T is Send. In
other words, if there is no possibility of undefined behavior (including data
races) when passing &T references between threads.


> A shorter overview of how Sync and Send relate to referencing:
>
> - &T is Send if and only if T is Sync
> - &mut T is Send if and only if T is Send
> - &T and &mut T are Sync if and only if T is Sync



### Future/Task:
Check out the dedicated post on Futures: https://toidiu.com/blog/rust-future-ecosystem/

### Pin:
Definition of “pinning"
- Not be moved out of its memory location
- More generally, remain valid at that same memory location

“Pinning” allows us to put a value which exists at some location in memory into
a state where safe code cannot move that value to a different location in memory
or otherwise invalidate it at its current location

### Unpin
The vast majority of Rust types have no address-sensitive states. These types
implement the Unpin auto-trait, which cancels the restrictive effects of Pin
when the pointee type T is Unpin.

When T: Unpin, Pin<Box<T>> functions identically to a non-pinning Box<T>;
similarly, Pin<&mut T> would impose no additional restrictions above a regular
&mut T.

## <a name="traits">Traits</a>
### Deref:
todo
### Drop:
todo
### PhantomData:
todo

