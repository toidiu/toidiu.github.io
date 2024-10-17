+++
title = "AWS: Transport libs"
date = 2024-03-01

[taxonomies]
tag = ["brag"]

[extra]
id = "blog-single"
+++

In this role I was responsible for maintaining the secure transport libraries
used to power AWS data centers.  These included s2n-quic and s2n-tls, which are
implementations of the QUIC and TLS protocols respectively.

As part of maintenance, my responsibilities included adding new library
features, accessing and mitigating security vulnerability, on-boarding new
customer integrations, helping debug existing customer integrations, helping
onboard new team members, updating public and internal documentation.

I joined the team when we were in the process of implementing s2n-quic so I am
also one of the original core maintainers of s2n-quic who helped build and
launch the library. In this capacity I designed and implemented core features in
the library, coordinated 3rd party pentests, helped with customer integrations,
etc.

### s2n-quic: Implement Optimistic ACK Attack mitigations
Tracking issue: https://github.com/aws/s2n-quic/issues/1962
Feature PR: https://github.com/aws/s2n-quic/pull/1986

This was tricky since the RFC is very sparse in its guidance. How often should
we skip packets? How many packets to skip? How do we track/clear skipped
packets?

> An endpoint that acknowledges packets it has not received might cause a
> congestion controller to permit sending at rates beyond what the network
> supports. An endpoint MAY skip packet numbers when sending packets to detect
> this behavior. An endpoint can then immediately close the connection with a
> connection error of type PROTOCOL_VIOLATION

We choose to skip 1 packet in the range [cwnd/2, cwnd*2]. Since we are trying to
control cwnd explosion, basing the skip value on cwnd scaled with the cwnd state
over the lifetime of the connection.

Other implementations were choosing this value statically.

> Only skip a packet number after verifying the peer did not send the previous one.
Many of the implementations were doing this check naively. They would skip
packets at a regular interval, clearing the previous skipped pn. Only tracking a
single skipped packet and clearing the skipped pn, before verifying the peer did
not ack the previous skipped packet means that they were not necessarily doing
the mitigation (depending on which packed was skipped).

The above method allowed us to only skip a single pn (small mem footprint),
while also effectively mitigating the attack.

Example:
```
So lets say an impl skips every 5th packet. It would skip 5, 10, 15. But the
impl should only skip
[1,2,3,4]; skip 5
< ack [1,2,3,4]

// the skip packet is cleared so the peer can send an ack for 5
[1,2,3,4,6,7,8,9,11]; skip 10
< ack [5,6,7]
```

Actual calculation:
```
// bound the random value based on congestion window
let pkt_per_cwnd = path.congestion_window() / path.mtu();
let lower = pkt_per_cwnd / 2;
let upper = pkt_per_cwnd.saturating_mul(2);
let cardinality = upper - lower + 1;
let mut rand = u32::rand() & cardinality;

// skip counter
skip = (lower + MIN_SKIP_COUNTER_VALUE).saturating_add(rand);
*skip_counter = Some(Counter::new(skip));
```

### s2n-quic: Onboarded CloudFront to s2n-quic for HTTP/3 support
> Time to First Byte (TTFB) improved by 23.5% overall, but especially on Android with a 37.1%
improvement. Additionally, customer engagement metrics from Snap showed improvement to story view
times, ad impressions, and a reduction in viewer cancellation counts.

s2n-quic was more performant due to the following reasons.
- cc algorithm (one of the main driver of the TTFB)
  - custom amplification factor
  - gain value
  - loss threshold values
  - initial cwnd value
- mtu probing
- packet pacing
- gso
- ecn
- testing
  - performance via flamegraph
  - netbench
- zero cost event framework
- custom data structures
  - ack range: interval set
  - packet number map: ring buffer which stores consequetive values and allows
    for ranged delete

### s2n-quic/s2n-tls: Async cert loading
https://github.com/aws/s2n-quic/issues/1137
Currently s2n-quic does the certificate lookup/loading operations synchronously.
Non ideal for application which handle multiple connections concurrently. The
work allowed for certificate lookup/loading operations to be performed
asynchronously and enable non-blocking behavior.

- s2n-quic: pass the connection waker down to the tls libraries so that they
  could wake on progress
- s2n-tls: The work involved converting the callback is only invoked once to a
  poll the callback model in s2n-tls. s2n-tls by default did not allow for poll
  callbacks. s2n-tls previously only called the callback once, which not the
  Rust model and has quite a few drawbacks. Polling only once means the
  application/s2n-quic has to schedule (separate thread possibly) the completion
  of the callback. It also needs to manage the state associated with the
  callback separately. The Rust polling model allows for all state associated
  with the future to live within the object bing polled. Additionally, the
  future can make progress as part of the already existing runtime that s2n-quic
  starts with.
- s2n-tls bindings: gluing the new callback polling behavior in an extensible
  way for other callbacks.

### s2n-quic: ACK frequency optimizations
link: https://github.com/aws/s2n-quic/issues/1276

Initial experiments with single batch did not show much improvements. Theory
was that having to aggregate acks and then process it separately was just as
expensive (cost savings of 2-3% simply didn't materialize) as simply processing
the acks individually.

**Delaying ack meant:**
- delayed signals to CC and loss recovery
- delayed processing of ECN signals (mitigated by processing immediately)
- mtu discovery (mitigated by not delaying mtu probes)
- handshake and anti-amplificaiton (mitigated by delaying after the handshake)

**Proposed solutions: 3 possible solutions**
- single payload batch would aggregate acks from multiple packets (10) and
  process them once (gso by default batches 10 packets). projected cost savings
  of 2-3% based on flamegraph.
  - Projected Pros:
    - relative CPU savings of ~20%
    - bytes/instruction (150k → 125k)
    - syscalls savings (20k → 6k)
    - This solution does not affect LR and CC!!
- multi payload batch would store the acks and process them at some delay. this
  would be equivalent to implementing the RFC, except it can be implemented
  locally without having to negotiate with the peer. (batch acks from multiple
  payloads would require additional storage and tracking)
  - Projected Pros:
    - more sophisticated delay strategy based on time/packets
    - doesn't require negotiating extension with peer
    - a stepping stone towards full RFC compliance
  - Cons:
    - Affects CC and LR
    - requires tuning and experimental data to fine-tune
- ack freq rfc (the full rfc, which requires the peer to negotiate)
  - Projected Pros:
    - can influence peer ack delay
  - Cons:
    - Affects CC and LR
    - requires tuning and experimental data to fine-tune
    - requires peer to negotiate

### s2n-quic: Connection Migration
https://github.com/aws/s2n-quic/pulls?q=is%3Apr+connection+migration+is%3Aclosed+author%3Atoidiu
Connection migration is one of the selling features of QUIC. It addresses the
modern network usage pattern where clients running on phones switch from one cell
tower to another. Reestablishing a new connection requires a new handshake to
establish new keys. Additionally, any in-progress transfers might have to start
over.

QUIC facilitates Connection Migration by introducing an explicit ConnectionId,
rather than using ip/port to identify a connection. This allows a server to tie
a client connection which is changing ip/ports back to the same connection
context and avoid a new handshake.

### s2n-quic: client implementation
https://github.com/aws/s2n-quic/issues/1009

AWS is primarily a datacenter company so the initial implementation of s2n-quic
only supported the server usecase. I added client support to the library.

Previously integration tests were based on a third-party QUIC library(quinn)
which made introspection and configuration more difficult. Adding client support
meant owning the entire QUIC stack for both the client and server. This
eventually enabled more sophisticated integration and fuzz testing.

The task involved reading the QUIC RFC for all requirements and implementing the
features to support to the library.

### s2n-quic: Event framework
https://github.com/aws/s2n-quic/issues/439

The event framework is a zero-cost abstraction which allows customers to enable
logging for s2n-quic. This kind of falls out of the sans I/O methodology.

The original verion of this was done with syntax macros, which I worked on
extensively. The final implementation was a standalone Rust token parse (via syn
crate) and outputed a generated.rs file. The parser was responsible for
correctly constructing events, their default impl, testing impl and builder
pattern for each event.

### s2n-tls: Pedantic Valgrind checks
https://github.com/aws/s2n-tls/issues/3758

Inspired by s2n_cleanup not properly cleaning up.

```
// a comma separated list of one or more of: definite indirect possible reachable.
--errors-for-leak-kinds=<set> [default: definite,possible]
--errors-for-leak-kinds=all
```

Leaks of kind "Still reachable" were not caught by Vangrind because by default
Valgrind enables "definite and possible". While this might be a safe assumption
for applications it is not for libraries that need to cleanup possibly stateful
resources. Solution was to run `--errors-for-leak-kinds=all` in tests.

### s2n-tls: Openssl 3.0 support
**TODO expand on this work.** low
https://github.com/aws/s2n-tls/issues/3442

### s2n-netbench: orchestrator
**TODO expand on this work.** low

## Not mentioned in resume

### s2n-quic: Support min, initial, max MTU per connection
Some mobile networks will fragment and reassemble packets despite the DNF (do no
fragment) flag. While this is a good thing for availability, it skews MTU
probing measurements and results in dereased throughput due to packet loss.

s2n-quic probes for the max mtu values on each network path and uses that to
that when sending packets. Since mobile networks are essentially claiming larger
MTU values than the path actually supports, s2n-quic continues to send larger
MTU values. Additionally, fragmenting/reassembing is an expensive operation
(io/cpu/time), which requies the fragmented packets to be buffered and then
reassemble.

The result is that s2n-quic sends larger and larger packets, which take longer
and longer for the mobile network to process. This results in packet loss and an
overall decrease in throughput.

### s2n-quic Presented a talk at CryptoCon
I presented a talk about s2n-quic and the QUIC protocol at CryptoCon, an internal AWS cryptography
conference. The talk was recored and shared in the internal Amazon video repository.

The goal of the talk was to illuminate what makes QUIC a novel protocol over the
previous iterations: TLS1.3, TLS1.2, etc. Imo the biggest shift in QUIC was
moving critical protocol components from the kernel-space to user-space (CC,
recovery, pacing). This shift allowed for quicker iterations but comes at the
higher cost and complexity of the user-space implementation.

### s2n-quic:  Add fips support which lead to issues when updating rustls to use aws-lc by default
rustls doesnt have a stable API and is pre-1.x. Additionally, it frequently
publishes breaking changes (although the introduction of
https://github.com/rustls/pki-types might mean a more stable API).

- s2n-quic is 1.x
- s2n-quic took a dep on rustls.
- s2n-quic re-exported some rustls types.
- rustls broke API in new versions.
- Upgrading rustls would mean breaking API for s2n-quic.

### created internal customer list

### s2n-quic:  rustls testing and parity

### s2n-quic:  advocate better slowloris mitigation

### s2n-quic:  handshake status
https://github.com/aws/s2n-quic/pull/960

Identified missing loose requirement with the handshake status. Took initiative to figure out the
correct requirement for client vs server. Added data typed enum to capture the complex handshake
transition. Added fuzz testing alongside the fix which increased confidence in the solution.

### s2n-quic path challenge

