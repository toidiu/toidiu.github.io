+++
title = "Meetup: Sans I/O"
date = 2025-06-01

[taxonomies]
tag = ["presentation"]

[extra]
company = ""
highlight = false
+++

https://github.com/toidiu/raft-rs/blob/main/sansio_presentation.pdf 

In this presentation, I wanted to share lessons learned while building sans I/O protocol
implementations. sans I/O advocates for implementing the core protocol logic as synchronous code and
avoiding all use of I/O (async, timers, socket, etc.). The I/O then gets "bolted" on top of the
protocol logic. The primary benefit of this is maintainable code, robust error handling, and testing
at the protocol layer. Additionally, the I/O layer becomes swappable and can easily integrate with
various language runtimes.

I wanted to share this because while sans I/O is specific to protocols, it should be possible to use
this design for other software. Finally, I feel even though async is powerful, it can lead to
complicated code (channel hell) that is hard to reason about if not careful.
