+++
title = "AWS: Transport libs"
date = 2024-03-01

[taxonomies]
tag = ["brag"]

[extra]
id = "blog-single"
+++

In this role I was responsible for maintaining the secure transport libraries used to power AWS data
centers.  These included s2n-quic and s2n-tls, which are implementations of the QUIC and TLS
protocols respectively.

As part of maintenance, my responsiblites included adding new library features, accessing and
mitigating security vurneabilities, onboarding new customer integrations, helping debug existing
customer integrations, helping onboard new team members, updating public and internal documentation.

I joined the team when we were in the process of implementing s2n-quic so I am also one of the
original core maintainers of s2n-quic who helped build and launch the library. In this capacity I
designed and implemented core features in the library, coordinated 3rd party pentests, helped with
customer integrations, etc.

<!-- ### project -->

<!-- - What your contributions were (did you come up with the design? Which components did you build? Was -->
<!--   there some useful insight like “wait, we can cut scope and do what we want by doing way less work” -->
<!--   that you came up with?) -->

<!-- - The impact of the project – who was it for? Are there numbers you can attach to it? (saved X -->
<!--   dollars? shipped new feature that has helped sell Y big deals? Improved performance by X%? Used by -->
<!--   X internal users every day?). Did it support some important non-numeric company goal (required to -->
<!--   pass an audit? helped retain an important user?) -->

<!-- Remember: don’t forget to explain what the results of you work actually were! It’s often important -->
<!-- to go back a few months later and fill in what actually happened after you launched the project. -->

### Onboarded CloudFront to s2n-quic for HTTP/3 support
- cc algorithm
  - custom amplification factor
  - gain value
  - loss threshold values
  - initial cwnd value
- mtu probing
- pacing
- gso
- ecn
- testing
  - performance via flamegraph
  - netbench

- zero cost event framework
- custom data structures
  - ack range: interval set
  - packet number map: ring buffer which stores consequetive values and allows for ranged delete

### support min, initial, max MTU
Some mobile networks will fragment and reassemble packets despite the DNF (do no fragment) flag.
While this is a good thing for availability, it skews MTU probing measurements and results in
dereased throughput due to packet loss.

s2n-quic probes for the max mtu values on each network path and uses that to that when sending
packets. Since mobile networks are essentially claiming larger MTU values than the path actually
supports, s2n-quic continues to send larger MTU values. Additionally, fragmenting/reassembing is an
expensive operation (io/cpu/time), which requies the fragmented packets to be buffered and then
reassemble.

The result is that s2n-quic sends larger and larger packets, which take longer and longer for the
mobile network to process. This results in packet loss and an overall decrease in throughput.

### Implement Optimistic ACK Attack mitigations
https://github.com/aws/s2n-quic/issues/1962

### ACK frequency optimizations
link: https://github.com/aws/s2n-quic/issues/1276

### presented a talk at CryptoCon
I presented a talk about s2n-quic and the QUIC protocol at CryptoCon, an internal AWS cryptography
conference. The talk was recored and shared in the internal Amazon video repository.

The goal of the talk was to illuminate what makes QUIC a novel protocol over the previous
iterations: TLS1.3, TLS1.2, etc. Imo the biggest shift in QUIC was moving critical protocol
components from the kernel-space to user-space (CC, recovery, pacing). This shift allowed for
quicker iterations but comes at the higher cost and complexity of the user-space implementation.

### Updating rustls to use aws-lc by default
rustls doesnt have a stable API and is pre-1.x. Additionally, it frequently publishes breaking
changes (although the introduction of https://github.com/rustls/pki-types might mean a more stable
API).

- s2n-quic is 1.x
- s2n-quic took a dep on rustls.
- s2n-quic re-exported some rustls types.
- rustls broke API in new versions.
- Upgrading rustls would mean breaking API for s2n-quic.

### openssl3 integation
### netbench orchestrator
### ktls
### created internal customer list

### s2n-quic async client hello
### s2n-quic rustls testing and parity
### s2n-quic advocate better slowloris mitigation
### s2n-quic handshake status
### s2n-quic path challenge
### s2n-quic client implementation
AWS is primarily a datacenter company so the initial implementation of s2n-quic only supported the
server usecase. I added client support to the library.

Previously integration tests were based on a third-party QUIC library(quinn) which made
introspection and configuration more difficult. Adding client support meant owning the entire QUIC
stack for both the client and server. This eventually enabled more sophisticated integration and
fuzz testing.

The task involved reading the QUIC RFC for all requirements and implementing the features to support
to the library.

### s2n-quic connection migration
### s2n-quic event framework

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

    What your contributions were (did you come up with the design? Which components did you build? Was there some useful insight like “wait, we can cut scope and do what we want by doing way less work” that you came up with?)
    The impact of the project – who was it for? Are there numbers you can attach to it? (saved X dollars? shipped new feature that has helped sell Y big deals? Improved performance by X%? Used by X internal users every day?). Did it support some important non-numeric company goal (required to pass an audit? helped retain an important user?)

Remember: don’t forget to explain what the results of you work actually were! It’s often important to go back a few months later and fill in what actually happened after you launched the project.

### Collaboration & mentorship

Examples of things in this category:

    Helping others in an area you’re an expert in (like “other engineers regularly ask me for one-off help solving weird bugs in their CSS” or “quoting from the C standard at just the right moment”)
    Mentoring interns / helping new team members get started
    Writing really clear emails/meeting notes
    Foundational code that other people built on top of
    Improving monitoring / dashboards / on call
    Any code review that you spent a particularly long time on / that you think was especially important
    Important questions you answered (“helped Risha from OTHER_TEAM with a lot of questions related to Y”)
    Mentoring someone on a project (“gave Ben advice from time to time on leading his first big project”)
    Giving an internal talk or workshop

### Design & documentation

List design docs & documentation that you worked on

    Design docs: I usually just say “wrote design for X” or “reviewed design for X”
    Documentation: maybe briefly explain the goal behind this documentation (for example “we were getting a lot of questions about X, so I documented it and now we can answer the questions more quickly”)

### Company building

This is a category we have at work – it basically means “things you did to help the company overall, not just your project / team”. Some things that go in here:

    Going above & beyond with interviewing or recruiting (doing campus recruiting, etc)
    Improving important processes, like the interview process or writing better onboarding materials

### What you learned

My friend Julian suggested this section and I think it’s a great idea – try listing important things you learned or skills you’ve acquired recently! Some examples of skills you might be learning or improving:

    how to do performance analysis & make code run faster
    internals of an important piece of software (like the JVM or Postgres or Linux)
    how to use a library (like React)
    how to use an important tool (like the command line or Firefox dev tools)
    about a specific area of programming (like localization or timezones)
    an area like product management / UX design
    how to write a clear design doc
    a new programming language

It’s really easy to lose track of what skills you’re learning, and usually when I reflect on this I realize I learned a lot more than I thought and also notice things that I’m not learning that I wish I was.
Outside of work

It’s also often useful to track accomplishments outside of work, like:

    blog posts
    talks/panels
    open source work
    Industry recognition

I think this can be a nice way to highlight how you’re thinking about your career outside of strictly what you’re doing at work.

This can also include other non-career-related things you’re proud of, if that feels good to you! Some people like to keep a combined personal + work brag document.

### General prompts

### If you’re feeling stuck for things to mention, try:

    If you were trying to convince a friend to come join your company/team, what would you tell them about your work?
    Did anybody tell you that you did something well recently?

