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
  - [Ownership](#ownership)
  - [Borrowing](#borrowing)
  - [Lifetimes](#lifetimes)
- [Memory Aliasing modules](#memory-aliasing-modules)
  - [Shared ownership](#shared-ownership)
    - [Rc](#rc)
  - [Shared mutability](#shared-mutability)
    - [Cell](#cell)
    - [RefCell](#refcell)
  - [Synchronization](#synchronization)
    - [Arc](#arc)
    - [Mutex](#mutex)
- [Thread-safety](#thread-safety)
  - [Send](#send)
  - [Sync](#sync)
- [Traits](#traits)
  - [Deref](#deref)
  - [Drop](#drop)
  - [PhantomData](#phantomdata)
  - [Unpin](#unpin)
    - [Pin](#pin-struct)
    - [PhantomPinned](#phantompinned)

## Borrow Checker

> In computing, aliasing describes a situation in which a data location in memory can be
> accessed through different symbolic names in the program. Thus, modifying the data
> through one name implicitly modifies the values associated with all aliased names, which
> may not be expected by the programmer. As a result, aliasing makes it particularly
> difficult to understand, analyze and optimize programs.

A mental model for thinking about guarantees that Rust provides is that it makes it easy
to reason about [aliasing](https://en.wikipedia.org/wiki/Aliasing_(computing)). At the end
of the day it comes down to where is your data, who owns it and who is accessing it. The
Borrow Checker provides abstractions which allows the Rust compiler to track and reason
about these aliasing rules and prevent misuse.

Unlike C, Rust attempts to automatically track if some data is being referenced, or read
or written to or already cleaned up. By tracking this automatically, it is able to prevent
common pitfalls we run into in C and also able to make performance optimizations that
would be very risky to do when tracking aliasing manually.

The following post describes some core Rust primitives that Rust provides to help us
reason about aliasing.

### Ownership:
Rust has a concept of ownership which assigns "ownership" of data to a variable. Since
ownership is exclusive, the Rust compiler is able to reason about when data is valid vs
when it has been freed. Ownersip can be transferred, but that still maintains the exclusive
ownership rule.

Ownership rules allows the Rust compiler to:
- prevent double free
- prevent accessing memory after free
  - which ensures memory safety

### Borrowing:
Since each piece of data has an exclusive owner, borrowing is the mechanism that allows
for multiple functions to interact with some data. Multiple reads (`&` which is an
immutable reference) or a single write (`&mut` mutable reference) are allowed and enforced
by the Rust compiler.

Borrowing rules allows the Rust compiler to:
- prevent aliasing
- prevent data races
  - which ensures single writer **or** multiple reads

### Lifetimes:
In C, while it is possible to hand out references to data its also necessary to enforce
that the underlying data is valid for as long as the reference is in use. For example, it
would be undefined behavior if we were to hand out a reference to some data, and free the
data (dangling pointer). Accessing the data after free is undefined behavior and
considered catastrophic.

Tracking a single piece of data and its reference might seem feasible but quickly an
operational risk as the program grows in complexity.

Lifetime rules allows the Rust compiler to:
- prevent invalid or dangling references
  - which ensures that references remain valid
  - which ensures references don't outlive the data they point to

## Memory Aliasing modules
To help us deal with aliasing rules Rust provides some containers that a allow us to
safely alias memory while also allowing the compiler to reason about them.

- `mod cell` provides shared mutability
- `mod rc` provides shared ownership
- `mod sync` provides synchronization


### Shared ownership
In the Ownership section we discussed how the Rust compiler enforces exclusive ownership
of data. The Rust compiler enforces these rules at compile-time, but sometimes there is a
need to relax these rules to runtime. The
[`rc`](https://doc.rust-lang.org/std/rc/index.html) module doesn't eliminate the rules,
but instead "shifts" them to runtime.

#### Rc
[Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) allows us to shift how the Ownership
and Lifetime rules are enforced. Let see how Rc achieves the same guarantees provided by
Ownership and Lifetime:

Ownership:
1. prevent double free
2. prevent accessing memory after free

Lifetime:

3. prevent invalid or dangling references

Rc allocates data on the heap (by default Rust allocates on the stack) and then tracking
the number of access via a 'non-atomic' (single threaded) reference count. Each access,
`Rc::clone()`, increments the reference count and each `drop()` decrements the count . If
the count reaches '0', there are no outstanding references to the data and we can free it
(**1.** prevents double free). Once the last reference is dropped, its impossible to
create new references (**2.** prevents accessing memory after free). Together, the logic
behind Rc helps ensure that references are valid (**3.** prevents invalid or dangling
references).

Here is a Rust playground
[link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=21622aa0d3d60c9fd73aee58d9717f07)
which demonstrate some of these concepts.

I haven't covered [Weak](https://doc.rust-lang.org/std/rc/struct.Weak.html) which is
necessary when creating for creating cyclical structures using Rc but requires more user
vigilance to use correctly.


### Shared mutability
Similar to how the Shared ownership module, `rc`, shifts the Ownership/Lifetime rules to
runtime, the Shared mutability module
[`cell`](https://doc.rust-lang.org/std/cell/index.html), "shifts" Borrowing guarantees
from compile-time to runtime. Let list these guarantees so we can compare their runtime
equivalent.

Borrowing:
1. prevent aliasing
2. prevent data races


#### Cell
[Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html) offers us interior mutability
by copying (`Cell::get()`) or moving (`Cell::take()` and `Cell::replace()`) the interior
value. By copying/moving the data, Cell ensures that there is only ever a single source of
data. This simplistic yet effective data model helps ensure memory safety (**1.** prevent
aliasing. **2.** prevent data races).

Here is a Rust playground
[link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bf97fdc984b61b910e68c5293a2f43c4)
which demonstrates some of Cell's APIs.



#### RefCell
Instead of copying/moving values,
[RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) achieves interior
mutability by enforcing exclusive mutable access by runtime "dynamic borrowing".

Using RefCell, its possible to `try_borrow()` to get a `&T` or `try_borrow_mut()` to get a
`&mut T`. Underneath RefCell enforces memory safety (**1.** prevent aliasing. **2.**
prevent data races). Less safe version of these APIs which panic on violation are also
available but those seem risky for production use (`borrow()` and `borrow_mut()`).

List of mutable APIs:
- RefCell::borrow_mut()
- RefCell::replace()
- RefCell::swap()

Here is a Rust playground
[link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f78e6ea5441d3f13a0f076736ab654c4)
which demonstrates some of RefCell's APIs.


### Synchronization
Synchronization is useful when a program is operating with multiple threads and wishes to
share data across those threads. Shared mutation across threads requires primitives (i.e.
atomics or locks) to enforce exclusive ownership and usage guarantees.

The [sync](https://doc.rust-lang.org/std/sync/index.html) module provides these
synchronization primitives and has decent explanation which I recommend reading.

#### Arc
[Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) provides us with shared
ownership across threads while enforcing Rust's Ownership/Lifetime rules. This is similar
to Rc, except Rc is not thread-safe (Arc stands for Atomic Reference Counted or
Atomic-Rc).

Very similar to [Rc](#rc), the only difference is the atomic nature of Arc works within a
multi-threaded environment. The reference count increments when a new `clone()` is created
and decremented on `drop()`. The count allows for tracking the number of outstanding
instances of Arc and initiaing memory cleanup when the last one goes out of scope (atomic
reference count hits 0).

```
// Example modified from
// https://doc.rust-lang.org/std/sync/struct.Arc.html#method.strong_count

use std::sync::Arc;

let five = Arc::new(5);
let _also_five = Arc::clone(&five);

assert_eq!(2, Arc::strong_count(&five));
```


#### Mutex
[Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html) provides us with shared
mutability across threads while enforcing Rust's Borrow rules. This is similar to RefCell,
except [RefCell](#refcell) is not thread-safe.

A Mutex helps preserves Rusts Borrow guarantees by enforcing exclusive access to the
underlying mutable object. Exclusive access can be taken by calling `Mutex::try_lock()`
(`Mutext::lock()` will block). Here is an example of how to use a Mutex:

```
// Example taken from
// https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.lock

use std::sync::{Arc, Mutex};
use std::thread;

// Arc is used since we are "sharing" the Mutex across threads.
//
// Mutex is used to mutate the object. If we didn't need mutation
// we could simply use Arc.
let mutex = Arc::new(Mutex::new(0));
let c_mutex = Arc::clone(&mutex);

thread::spawn(move || {
    *c_mutex.lock().unwrap() = 10;
}).join().expect("thread::spawn failed");

// assert new value
assert_eq!(*mutex.lock().unwrap(), 10);
```



## Thread-safety

You know its complicated when there is a
[chapter](https://doc.rust-lang.org/nomicon/send-and-sync.html) for it in the Rust
Nomicon. The `Send` and `Sync` traits help the Rust compiler reason about thread safety.
They are used to mark data that can be shared or moved across different threads. Typically
this is not safe to do for and requires the use of synchronization.


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
ensure access to the data is exclusive even across multiple threads.


## <a name="traits">Traits</a>
The Rust type system is rich and allows us to express complex concepts to the compiler.

### Deref
todo
Rust encourages the use of types, which encourages safe usage enforced by the compiler.
For example, rather than representing distance as `usize`, one should create a new-type
`Distance(usize)`. This helps maintain type checking but also allows us to attach special
context around a Distance type.

```
struct Distance(usize);

impl Distance {
    fn add_len(&self, len: &usize) -> usize {
        self.0 + len
    }
}

fn main() {
    let dist1 = Distance(1);
    let dist2 = Distance(2);

    dist1.add_len(&dist2.0); // 1: explicitly access the inner field
}
```

However, notice how the usage (1) is more verbose now and require accessing the inner
field `dist2.0`. [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html) offers
automatic immutable deferencing so that its possible avoid access the inner data without
the explicit access. By implementing Deref we are able to simplify the usage to `&dist2`
below.

```
impl std::ops::Deref for Distance {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let dist1 = Distance(1);
    let dist2 = Distance(2);

    dist1.add_len(&dist2);
}
```

The full example code is available in this Rust playground
[link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=29fcde8dc36ceb60cde97d2035b61eb4)

This is a powerful tool but should be used carefully since it does mean we potentially
lose the benefit of the new-type.

### Drop:
In our programs one need to perform "cleanup" after use or risk resource (memory) leakage
or worse. This is a non-trivial problem in C where one needs a paranoid attitude (really
all aspects of C require a paranoid attitude) to make sure that all resources are
properly cleaned up.

Rust offers us the [Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html) trait to make
this task trivial.

The Drop trait requires `fn drop(&mut self)` function which is run when an object goes out
of scope is no longer used. It should then run and "destructor" methods to cleanup
associated resources.

The Rust compiler recursively calls the `drop()` function on all fields of an object and
implements the Drop trait for std types (u8, u16, etc...) so that most of the time proper
cleanup happens automatically. Here is a Rust playground
[link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c5c784713e26f1c957a772ec2edc76c6)
which shows how the recursive drop functionality works.

So when should users implement the Drop trait themselves? In this description I have been
using "resources" rather than memory because I wanted to be general. Resources could
include memory, network connection, file handle, locks, database cleanup, or any other
custom logic that users want to run at "cleanup".


### PhantomData
[PhantomData](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) is considered
a phantom type. A phantom type is a type not actually present at runtime but instead used
at compile time for static analysis.

> Adding a PhantomData<T> field to your type tells the compiler that your type acts as
> though it stores a value of type T, even though it doesn’t really. This information is
> used when computing certain safety properties.

Lets look at some example code:

```
struct MyType<'a, A> {
    field: usize,
    // PhantomData is used to avoid unused warning for 'a and A
    a: PhantomData<&'a A>,
}
```

So essentially, the Rust compiler will emit an unused warning if a generic type or
lifetime is not actually used. PhantomData can be used to mute these warning. But why
would we have a type that we are not using?


```
// Example taken from: https://cliffle.com/blog/rust-typestate
//
// S is the state parameter. We require it to impl
// our ResponseState trait (below) to prevent users
// from trying weird types like HttpResponse<u8>.
struct HttpResponse<S: ResponseState> {
    // This is the same field as in the previous example.
    state: Box<ActualResponseState>,
    // This reassures the compiler that the parameter
    // gets used.
    marker: std::marker::PhantomData<S>,
}

// State type options.
enum Start {} // expecting status line
enum Headers {} // expecting headers or body

trait ResponseState {}
impl ResponseState for Start {}
impl ResponseState for Headers {}
```

The best usecase which demonstrates PhantomData's is the
[Typestate](https://cliffle.com/blog/rust-typestate/) pattern. `PhantomData<T>` is used in
the Typestate pattern to switch implementation behavior based on the type `T`. However,
since there is no need to actually instiantiate T (we only need the type `T` to switch
behavior), it will emit an unused warning unless included in PhantomData.


### Unpin
The vast majority of Rust types have no address-sensitive states. For example, an integer
can be copies to another address while maintaining the correct semantic. Most types implement
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
Similar to PhantomData (discussed [above](#phantomdata)),
[PhantomPinned](https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html) is also a
phantom type which is only useful for the Rust compiler's static analysis.

Where PhantomData was used to mark "used" parameters, PhantomPinned is used to mark a type
as `Pin`ned. Since there is no Pin trait, a more accurate statement from the official
documentation states:

> A marker type which does not implement Unpin.
> If a type contains a PhantomPinned, it will not implement Unpin by default.

This type can be used when you need to ensure that a type should not be moved. You might
need this if you are implementing some unsafe self referential code and should throughly
understand pin.
