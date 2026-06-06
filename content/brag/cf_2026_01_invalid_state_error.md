+++
title = "Invalid State Error"
date = 2026-01-01

[extra]
company = "Cloudflare"
highlight = true
+++

https://github.com/cloudflare/quiche/pull/2366

As we built the new proxy at Cloudflare, it came with better observability. One
of the puzzling metrics was the high number of InvalidState errors that the
proxy was emitting. The InvalidState error was a generic catch-all error emitted
from quiche. It wasn't possible to tell if the errors were due to a quiche bug,
malicious traffic or simply malformed client traffic from the internet. Another
troubling signal was that an engineer had noticed an increase in this traffic a
year earlier but had not been able to identify the root cause of the increase.

The challenge with this investigation was the vast volume of traffic and adding
non-invasive telemetry to a production system. The first attempt at logging more
information about the bug actually resulted in a behavior change and a large
increase in logs. It turned out that the downstream system was actually relying
on the error type to determine close behavior. While the increase in logs was
not detrimental, it did leave the logging pipeline vulnerable in the case of a
DOS attack.

Since I needed detailed logs, I proposed that we switch the investigation to an
ad-hoc build (~10 machines). Still being careful not to overload the logging
pipeline, I incrementally added more logs and tracked the error to the packet
receive flow. A few layers deeper, logging the total number of QUIC paths per
connection revealed the root cause. All connection errors had 2 paths
registered. The bug was coming from the PMTU probing logic. I quickly deduced
that packets on a connection could accidentally have been affected by a path
change (NAT rebind). NAT rebinds would result in the perception of multiple
paths but a QUIC connection requires path validation before becoming the active
path. The PMTU code was not considering this edge case and incorrectly expecting
an active path.

The fix was trivial; since PMTU is a performance optimization the correct
solution was to simply skip it until an active path was available. Analysis
showed an 80% reduction in InvalidState errors (1K connections/day) in our new
proxy logs.

Put another way, the PMTU bug was 0.17% of the total proxy traffic. Since quiche
was also being used for the old proxy and products across the company, the real
savings were much larger.
