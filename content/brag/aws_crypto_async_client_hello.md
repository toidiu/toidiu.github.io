+++
title = "s2n-quic async client hello"
date = 2022-02-01

[extra]
company = "aws_crypto"
lp = []
+++

https://github.com/aws/s2n-quic/issues/1137

#### S
Currently s2n-quic does the certificate lookup/loading operations synchronously.

Non ideal for application which server multiple domains concurrently and need to
load multiple certificates, since that would block the thread.

#### T
Allowed for certificate lookup/loading operations to be performed asynchronously
and enable non-blocking behavior.

#### A
- s2n-quic:
  - pass the connection waker down to the tls libraries so that they could wake
    on progress

- s2n-tls:
  - The work involved converting the callback only-invoked-once to a
    poll-the-callback model in s2n-tls.
  - s2n-tls by default did not allow for poll callbacks.
  - s2n-tls previously only called the callback once, which not the Rust model
    and has quite a few drawbacks.
    - Polling only once means the application/s2n-quic has to schedule (separate
      thread possibly) the completion of the callback.
    - It also needs to manage the state associated with the callback separately.
    - The Rust polling model allows for all state associated with the future to
      live within the object bing polled.
    - Additionally, the future can make progress as part of the already existing
      runtime that s2n-quic starts with.
- s2n-tls bindings:
  - gluing the new callback polling behavior in an extensible way for other
    callbacks.

#### R
- setup the framework for future async callbacks in s2n-tls

