+++
title = "Go Proxy TLS hangs"
date = 2025-11-01

[taxonomies]
tag = ["work"]

[extra]
company = "Cloudflare"
highlight = true
+++

While oncall I received an incident that customers were experiencing failure when trying to
establish new connections to the Go proxy. Existing connections worked fine and the only abnormality
in the logs were some long running goroutines.

While technically owned by our team, this proxy was in maintenance mode and the original authors
were on different teams. Even though my weekly oncall shift was coming to an end, I decided this was
going to be a very good opportunity to take ownership of the proxy and learn Go at the same time.

Following the logs I ended up in a loop which was using locks to either make RPC calls or establish
a new connection on failure. Being new to Go, I deeply investigated the internals of the standard
library. I specifically attempted to find if there were timeouts and if there was support for
connection pooling.

After this investigation I strongly started to suspect that the new connection establishment was
holding the lock. I added observability to try and detect the long held lock. However, the root
cause was difficult to detect since we were trying to detect a lack of metrics, which could also
occur due to low traffic volume.

I prepared a patch to add timeout to the new Connection call, which was most likely the fix.
However, during this time the company started a change freeze and required justification to make any
changes.

I  colloborated with the original authors to analyze the metrics and conclude that 1K+ proxy
instances (~0.2% of fleet) was being affected. I drafted a 1 page document detailing the risks of
the patch, the importance of the customer issue, roughly the customer impact based on the
observability I added, and a detailed rollback plan.

Based on my analysis, I got approvals from my manager, her skip manager and the director. Adding a
lower (10 sec) timeout around new Connection resolved the issue the observability confirmed the fix.
