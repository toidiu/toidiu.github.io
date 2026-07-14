+++
title = "Go Proxy TLS hangs"
date = 2025-11-01

[taxonomies]
tag = ["work"]

[extra]
company = "Cloudflare"
highlight = true
+++

While on-call at Cloudflare, I was paged for an incident where customers were experiencing failures
when trying to establish new connections to the Go proxy. Existing connections continued to work
fine and the only useful information in the core dump was a long-running goroutine, whose stack
trace pointed at a specific source file.

While technically owned by our team, the Go proxy was in unofficial "maintenance" mode since all the
original authors were on different teams. Even though my weekly on-call shift was coming to an end,
I decided to pick up the investigation because our team needed to own our software, there was real
customer impact, and this would be a fantastic opportunity to learn Go.

I started with the source file mentioned in the core dump and focused on a complex loop which was
using locks to make RPC calls and establish a new connection on failure. I started to suspect a
deadlock, but being new to Go, I first had to confirm this by investigating the library internals.
In my investigation into the RPC and new connection calls, I did not find any timeouts, which
deepened my suspicion. The fix later confirmed this: a goroutine establishing a new connection would
hang, never release the lock, and block all new connections.

Pursuing the deadlock theory, I started by adding observability to try and detect the issue and also
measure the customer impact. However, a deadlock is tricky to observe: I was trying to measure a
lack of metrics, which is hard to differentiate from low traffic volume. I ended up adding histogram
metrics to measure the distribution of time spent in the RPC loop, and collaborated with the
original authors to estimate the current impact (~0.2% of the fleet, or 200 proxy instances, stuck
at any given time).

Next, I prepared a small patch to add timeouts to both the RPC and new connection calls.
Unfortunately, this was blocked due to a multi-month production freeze. Motivated by the facts that
this was likely the fix, that real customers were impacted, and that a long freeze would not be
acceptable, I set out to convince my manager and leadership. I wrote a concise document describing
the problem, my investigation, estimated customer impact, why the proposed fix was urgent, and why
the fix was low risk. I was able to get approvals up the leadership chain (up to director), and I
confirmed the fix via the histogram metrics once the patch was released.
