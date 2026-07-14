+++
title = "Session Resumption"
date = 2025-09-01

[taxonomies]
tag = ["work"]

[extra]
company = "Cloudflare"
+++

One of the most technically challenging projects I undertook was adding session resumption and 0-RTT
to our new Rust proxy at Cloudflare. The project was particularly challenging because it was my
first project in the new role, in addition to the complicated nature of session resumption.

The high-level goal of session resumption is to cache session state in order to skip some expensive
aspects of the full TLS handshake for subsequent connections. The savings come with protocol and API
complexity, so it's even more important to design resilient APIs that are easy to audit and hard to
misuse.

I started by deeply understanding the session resumption feature, studying how nginx used it, and
writing a specification based on my previous experience with protocol design. Since the entire stack
was new to me, I relied on my team to review my specification and surface legacy decisions that
needed to be ported from the old proxy.

The first tricky technical decision was handling rotating session ticket keys based on a TTL. This
logic needed to be resilient because of all the potential edge cases due to missing keys. I had to
handle loading keys asynchronously, generating session ticket keys from secrets, and fallback logic
to deal with missing keys (empty or partially missing key set). Testing this logic was quite tricky
because it was difficult to isolate the rotation logic from the session ticket caching layer.

The most technically difficult aspect of the project was understanding the BoringSSL API and
exposing Rust bindings for it. Crossing the FFI boundary makes development much riskier because you
are dealing with unsafe boundaries that can panic, or worse, fail silently. Considering that we were
building a secure proxy, a bug could have serious implications. I implemented a safe Rust interface
for session resumption, added tests, and made sure to document the implicit C contract. While I
could have used the unsafe low-level C bindings directly, the safe bindings enabled cleaner control
flow and made the code easier to test and instrument.

Integrating session resumption was simple compared with testing the feature. Most TLS features tend
to be stateless, so adding testing support for session resumption involved diving into the entire
stack, five layers deep (BoringSSL, quiche, tokio-quiche, oxy framework, Rust proxy).

Implementing 0-RTT didn't require much code but demanded deep investigation of the C logic in
BoringSSL. The TLS state machine during a 0-RTT resumption is complicated since it bypasses many of
the procedures of a full handshake and instead derives 0-RTT keys from the session ticket. I had to
build a comprehensive model of TLS, 0-RTT, QUIC and all the proxy layers in between to get the
feature working.

I pride myself on creating systems that are simple, and a large part of that is exposing rich
metrics. I am especially proud of the observability I added for the session resumption feature. The
Cloudflare proxy serves millions of req/sec (more during a DDoS attack), so good observability is
critical to sane operation. I added POP-specific metrics for session ticket keys and resumption
status. The session ticket key metrics tracked key expiration and the use of fallback keys, and were
useful for detecting infrastructure issues. The resumption status metrics were useful for detecting
customer issues and finding gaps in our resumption design. Based on these metrics, I estimated that
it was possible to further increase the resumption rate by up to 15% compared to the current global
rate. I wrote an internal blog post suggesting the improvement and estimating the effort.
