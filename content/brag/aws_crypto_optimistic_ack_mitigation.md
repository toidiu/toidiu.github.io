+++
title = "s2n-quic optimistic ack mitigation"
date = 2022-07-01

[extra]
company = "aws_crypto"
lp = []
+++

https://github.com/aws/s2n-quic/pull/1986

#### S
- add mitigation for optimistic acks
- the risk would be to inflate cc and create unfair network conditions

#### T
- the rfc was clear how to do this (skip packets)
- but also vague because it didnt mention how many packets to skip and how often

#### A
- took a look at other implementations
- there were 2 and they were using a static approach
  - track skipping 1 pn. overwrite value when a new pn needs to be skipped
  - skip a random pn in some static range
- it was hard to assess if this approach was effective
  - does overriting skip pn affect mitigation?
  - does a static range work for all networks?
    - a cc window would be very different in a DS vs public internet
- instead of storing 1 skip pn we could store many
- this would allow us to skip more frequenly
  - but require us to store these pn
  - how many pn should we store? how fast should we send?

#### R
- i analyzed the purpose of the mitigation
  - prevent cc window bloat
  - this could be done if packets were acked prior to peer receiving them
- we expect an ack every RTT time (RTT being the **period**)
  - this is the **period** for sending packet and reciving an ack
- since the # of packets we send within a **period** can change, we should skip
  packets based on the period
- how many packets do we expect to send in a **period**?
  - cwnd / mtu = packets per cwnd
  - rand = cwnd/2..cwnd*2

