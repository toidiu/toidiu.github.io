+++
title = "Invalid State Error"
date = 2026-01-01

[taxonomies]
tag = ["work"]

[extra]
company = "Cloudflare"
highlight = true
+++

https://github.com/cloudflare/quiche/pull/2366

While developing the new Rust proxy at Cloudflare, one of my coworkers noticed an increase in the
number of InvalidState errors. The InvalidState error was a generic catch-all emitted when a quiche
connection was in a bad state. We expected a non-zero number of errors since CDN traffic contains
malformed or malicious clients, but an increase in the number of errors needed investigation.
Attempts to find the root cause were not successful and we filed a ticket as we continued to work on
other critical components. I picked up the ticket almost a year later, as we neared the launch date.

The main challenge with this investigation was the limited telemetry and the use of loosely typed
errors. Since the proxy served a high volume of traffic, any increase in logging could be dangerous
to the logging infrastructure (and amplified by a DDoS attack). I felt this pain as my first attempt
to annotate errors with additional context resulted in a behavior change to the connection close
reason and an increase in log volume. The change was reverted and I had to rethink my strategy.

The original goal of contextualizing the errors was to segment them without blowing up the logging
pipeline. However, since what I really needed were detailed logs, I proposed switching to a
"staging" build (deployed on ~10 bare-metal servers). With the build isolated to a smaller fleet but
still being cautious not to overload the pipeline, I iteratively added more logs and traced the
various sources of the InvalidState error. Most of the error sources were reasonable and stemmed
from handling malformed client payloads (e.g. invalid CID).

A few iterations later I hit an interesting log line that roughly looked like: "paths: 2,
active_path: x". This "smelled" funny because all these errors were happening in the presence of '2'
paths, but path migration is not a common feature. Having previously implemented connection
migration in s2n-quic, I quickly deduced that NAT rebinds can cause passive path migration
independent of the client implementation. The error occurred when the PMTU probing logic (a
performance optimization) attempted to access a non-existent active path after the migration. My
proposed fix was to skip PMTU probes in these cases and avoid closing the connection.

Once fixed, the logs showed an 80% reduction in InvalidState errors. Although the PMTU bug affected
only 0.17% of the total proxy traffic, that amounted to roughly a thousand customer connections
being torn down every second on the new proxy. Given that quiche was used across other services, the
real impact was much larger.

