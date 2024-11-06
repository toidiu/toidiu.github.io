+++
title = "s2n-quic optimistic ack mitigation"
date = 2022-07-01

[extra]
company = "aws_crypto"
lp = []
+++

https://github.com/aws/s2n-quic/pull/1986

#### S
- Add mitigation for optimistic acks
- The rfc was clear how to do this (skip packets), but also vague because it didnt mention how many packets to skip and how often.

#### T
- Come up with a strategy for skipping packet.

#### A
- Audited other QUIC implementations and conducted analysis to answer two key questions:
  - How many packets to skip?
  - How often should packets be skipped?
- There were 2 other implementations that were using a "static" (skipping did not evolve with cwnd)
  approach. Their strategy was to:
  - Track skipping 1 pn. Overwrite value when a new pn needs to be skipped.
  - Skip a random pn in some static range.
- The "static" approach was network dependent (DC vs public wifi):
  - Does overriting skip pn nullify the mitigation?
  - Is a static skip range effective for all networks?
    - A cwnd is very different in a DS vs public internet
- Considered the option of skipping multiple packets:
  - Pro: This would allow us to skip more frequenly
  - Con: Require storing multiple skip pn. How many pn should we store?
- Analyzed the purpose of the mitigation to come up with an optimal solution.
  - The goal of the mitigation was to prevent cwnd bloat.
  - Which could be done if packets were acked prior to the peer receiving them.
  - By basing skip range on the number of packets that could be received with an cwnd.
- Solution: evolve skip packet range based on cwnd and only store 1 skip pn.
- Calculate range based on packets we expect to send in a single period.
  - `pkt_per_cwnd = cwnd / mtu`
  - `rand = pkt_per_cwnd/2..pkt_per_cwnd*2`

#### R
- Implemented the mitigation.
- Only had to store 1 skip pn.
- By evolving the skip range based on cwnd, s2n-quic would scale to all networks.
