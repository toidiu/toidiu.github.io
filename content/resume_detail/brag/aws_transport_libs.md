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
features, accessing and mitigating security vurneabilities, onboarding new
customer integrations, helping debug existing customer integrations, helping
onboard new team members, updating public and internal documentation.

I joined the team when we were in the process of implementing s2n-quic so I am
also one of the original core maintainers of s2n-quic who helped build and
launch the library. In this capacity I designed and implemented core features in
the library, coordinated 3rd party pentests, helped with customer integrations,
etc.

### s2n-quic Onboarded CloudFront to s2n-quic for HTTP/3 support
s2n-quic was more performant due to the following reasons.
- cc algorithm
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

### s2n-quic/s2n-tls Async cert loading
**TODO expand on this work.**
https://github.com/aws/s2n-quic/issues/1137
Currently s2n-quic does the certificate lookup/loading operations synchronously.
Non ideal for application which handle multiple connections concurrently. The
work allowed for certificate lookup/loading operations to be performed
asynchronously and enable non-blocking behavior.

- s2n-tls work
- s2n-tls bindings work
- s2n-quic work

### s2n-quic ACK frequency optimizations
link: https://github.com/aws/s2n-quic/issues/1276

Initial experiments with single batch did not show much improvements. Theory
was that having to aggregate acks and then process it separately was just as
expensive (cost savings of 2-3% simply didn't materialize) as simply processing
the acks individually.

**Delaying ack meant:**
- delayed signals to CC and loss recovery
- delayed processing of ECN signals (mitigated by processing immediately)
- mtu discovery (mitigated)
- handshake and anti-amplificaiton (mitigated)

**Proposed solutions:**
- single payload batch would aggregate acks from multiple packets (10) and
  process them once (gso by default batches 10 packets). projected cost savings
  of 2-3% based on flamegraph.
- multi payload batch would store the acks and process them at some delay. this
  would be equivalent to implementing the RFC, except it can be implemented
  locally without having to negotiate with the peer. (batch acks from multiple
  payloads which need a separate data structure)
- ack freq rfc (the full rfc, which requires the peer to negotiate and be
  compliant)

### s2n-quic Event framework
https://github.com/aws/s2n-quic/issues/439

The event framework is a zero-cost abstraction which allows customers to enable
logging for s2n-quic.

The default implementation of each event is a noop, which means it is compiled
away. Users can override the default behavior if they care to consume a
particular event.

Originally implemented using syntax macros, the final implementation was a
standalone Rust token parse (via syn crate) and outputed a generated.rs file.
The parser was responsible for correctly constructing events, their default
impl, testing impl and builder pattern for each event.

### s2n-quic Connection Migration
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

### s2n-quic client implementation
https://github.com/aws/s2n-quic/issues/1009 AWS is primarily a datacenter
company so the initial implementation of s2n-quic only supported the server
usecase. I added client support to the library.

Previously integration tests were based on a third-party QUIC library(quinn)
which made introspection and configuration more difficult. Adding client support
meant owning the entire QUIC stack for both the client and server. This
eventually enabled more sophisticated integration and fuzz testing.

The task involved reading the QUIC RFC for all requirements and implementing the
features to support to the library.

### s2n-quic Implement Optimistic ACK Attack mitigations
Tracking issue: https://github.com/aws/s2n-quic/issues/1962
Feature PR: https://github.com/aws/s2n-quic/pull/1986

This was tricky since the RFC didn't really mention the exact implementation.
How often should we skip packets and how many packets?

We choose to skip 1 packet in the range [cwnd/2, cwnd*2]. Since we are trying
to control cwnd explosion, basing the skip value on cwnd made sense.

> Only skip a packet number after verifying the peer did not send the previous one.
Many of the implementations were doing this check naively. They would skip
packets at a regular interval. However, other impls were only tracking a single
skipped packet and not waiting to verify if the peer did not ack the previous
skipped packet.

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

### s2n-tls Pedantic Valgrind checks
```
// a comma separated list of one or more of: definite indirect possible reachable.
--errors-for-leak-kinds=<set> [default: definite,possible]
--errors-for-leak-kinds=all
```

Leaks of kind "Still reachable" were not caught by Vangrind because by default
Valgrind enables "definite and possible". While this might be a safe assumption
for applications it is not for libraries that need to cleanup possibly stateful
resources. Solution was to run `--errors-for-leak-kinds=all` in tests.

### s2n-tls Openssl 3.0 support
**TODO expand on this work.** low

### s2n-netbench orchestrator
**TODO expand on this work.** low

## Not mentioned in resume

### s2n-quic Support min, initial, max MTU Some mobile networks will fragment
and reassemble packets despite the DNF (do no fragment) flag. While this is a
good thing for availability, it skews MTU probing measurements and results in
dereased throughput due to packet loss.

s2n-quic probes for the max mtu values on each network path and uses that to
that when sending packets. Since mobile networks are essentially claiming larger
MTU values than the path actually supports, s2n-quic continues to send larger
MTU values. Additionally, fragmenting/reassembing is an expensive operation
(io/cpu/time), which requies the fragmented packets to be buffered and then
reassemble.

The result is that s2n-quic sends larger and larger packets, which take longer
and longer for the mobile network to process. This results in packet loss and an
overall decrease in throughput.

### s2n-quic Presented a talk at CryptoCon I presented a talk about s2n-quic and
the QUIC protocol at CryptoCon, an internal AWS cryptography conference. The
talk was recored and shared in the internal Amazon video repository.

The goal of the talk was to illuminate what makes QUIC a novel protocol over the
previous iterations: TLS1.3, TLS1.2, etc. Imo the biggest shift in QUIC was
moving critical protocol components from the kernel-space to user-space (CC,
recovery, pacing). This shift allowed for quicker iterations but comes at the
higher cost and complexity of the user-space implementation.

### s2n-quic Updating rustls to use aws-lc by default rustls doesnt have a
stable API and is pre-1.x. Additionally, it frequently publishes breaking
changes (although the introduction of https://github.com/rustls/pki-types might
mean a more stable API).

- s2n-quic is 1.x
- s2n-quic took a dep on rustls.
- s2n-quic re-exported some rustls types.
- rustls broke API in new versions.
- Upgrading rustls would mean breaking API for s2n-quic.

### created internal customer list
### s2n-quic rustls testing and parity
### s2n-quic advocate better slowloris mitigation
### s2n-quic handshake status
### s2n-quic path challenge

---


## TEMPLATE

### Goals for this year:

    List your major goals here! Sharing your goals with your manager & coworkers is really nice
    because it helps them see how they can support you in accomplishing those goals!

### Goals for next year

    If it’s getting towards the end of the year, maybe start writing down what you think your goals
    for next year might be.

### Projects

For each one, go through:

    What your contributions were (did you come up with the design? Which
    components did you build? Was there some useful insight like “wait, we can
    cut scope and do what we want by doing way less work” that you came up
    with?) The impact of the project – who was it for? Are there numbers you can
    attach to it? (saved X dollars? shipped new feature that has helped sell Y
    big deals? Improved performance by X%? Used by X internal users every day?).
    Did it support some important non-numeric company goal (required to pass an
    audit? helped retain an important user?)

Remember: don’t forget to explain what the results of you work actually were!
It’s often important to go back a few months later and fill in what actually
happened after you launched the project.

### Collaboration & mentorship

Examples of things in this category:

    Helping others in an area you’re an expert in (like “other engineers
    regularly ask me for one-off help solving weird bugs in their CSS” or
    “quoting from the C standard at just the right moment”) Mentoring interns /
    helping new team members get started Writing really clear emails/meeting
    notes Foundational code that other people built on top of Improving
    monitoring / dashboards / on call Any code review that you spent a
    particularly long time on / that you think was especially important
    Important questions you answered (“helped Risha from OTHER_TEAM with a lot
    of questions related to Y”) Mentoring someone on a project (“gave Ben advice
    from time to time on leading his first big project”) Giving an internal talk
    or workshop

### Design & documentation

List design docs & documentation that you worked on

    Design docs: I usually just say “wrote design for X” or “reviewed design for
    X” Documentation: maybe briefly explain the goal behind this documentation
    (for example “we were getting a lot of questions about X, so I documented it
    and now we can answer the questions more quickly”)

### Company building

This is a category we have at work – it basically means “things you did to help
the company overall, not just your project / team”. Some things that go in here:

    Going above & beyond with interviewing or recruiting (doing campus
    recruiting, etc) Improving important processes, like the interview process
    or writing better onboarding materials

### What you learned

My friend Julian suggested this section and I think it’s a great idea – try
listing important things you learned or skills you’ve acquired recently! Some
examples of skills you might be learning or improving:

    how to do performance analysis & make code run faster internals of an
    important piece of software (like the JVM or Postgres or Linux) how to use a
    library (like React) how to use an important tool (like the command line or
    Firefox dev tools) about a specific area of programming (like localization
    or timezones) an area like product management / UX design how to write a
    clear design doc a new programming language

It’s really easy to lose track of what skills you’re learning, and usually when
I reflect on this I realize I learned a lot more than I thought and also notice
things that I’m not learning that I wish I was. Outside of work

It’s also often useful to track accomplishments outside of work, like:

    blog posts talks/panels open source work Industry recognition

I think this can be a nice way to highlight how you’re thinking about your
career outside of strictly what you’re doing at work.

This can also include other non-career-related things you’re proud of, if that
feels good to you! Some people like to keep a combined personal + work brag
document.

### General prompts

### If you’re feeling stuck for things to mention, try:

    If you were trying to convince a friend to come join your company/team, what
    would you tell them about your work? Did anybody tell you that you did
    something well recently?
