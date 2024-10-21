+++
title = "s2n-quic ack freq analysis"
date = 2022-05-01

[extra]
company = "aws_crypto"
lp = []
+++

#### summary
- tracking issue: https://github.com/aws/s2n-quic/issues/1276
- cpu increase: https://github.com/aws/s2n-quic/pull/1298
- revert pr with some analysis shows : https://github.com/aws/s2n-quic/pull/1368
- measure batching multi-packets: https://user-images.githubusercontent.com/4350690/174196918-6af428e4-9ab7-4458-b3b9-e27ed89c3318.png

#### metrics

#### S
Pros of delay:
- send/recv acks is CPU expensive
  - on the send heavy server it takes 24.33%
- asymmetric links like satellite

Cons of delay:
- progress: delayed acknowledgment can delay data being sent from peer
  - the RFC lets you adjust thresholds
- ECN: congestion signal by the network
  - ack packets with ECN markings immediately
- loss recovery: detect a packet was not received and retransmit
- cc: regular acks help establish good RTT estimates and drive CC
- BBR: requires a high precision signal to work so it was unclear how delaying
  acks would affect this

RFC features:
- negotiating the extension: min_ack_delay. the minimum amount of time, that the
  endpoint sending this value is willing to delay an acknowledgment
- ACK_FREQUENCY frame:
  - Ack-Eliciting Threshold: max ack eliciting **packets** received before sending an ack
  - Requested Max Ack Delay:  max **time** to wait before sending an ack
  - Reordering Threshold: max number of **reordered packet** before sending an ack
- IMMEDIATE_ACK frame: asks peer to send an immediate ack
- Expedite ECN: ecn marked packets should not be delayed

#### T
Choose on an implementation.

Options:
- single round batching (10 packet GSO payload)
  - easy to implement (1-2 sprints)
  - this solution creates building blocks for others after it
- multi round batching (multi-gso payload) (2-4 sprints)
  - medium difficulty to implement
  - requires tuning with application, production usecase, traffic pattern
- implement Ack delay RFC (4-8 sprints)
  - difficult to implement
  - requires tuning with application, production usecase, traffic pattern
  - requires negotiating the extension

#### A
[Impl: Batch ACK Processing (single round)](https://github.com/aws/s2n-quic/issues/1277)

- single round batching (10 packet GSO payload)
  - connection can signal interest in processing acks
  - store pending acks on connection
  - refactor CC and LR to accept signals from batched ack processing (timestamp)
  - swap from single processing to batched processing
  - emit metrics

#### R
- Flamegraph result 24.33% -> 25.81% https://github.com/aws/s2n-quic/pull/1298
- Batching different amount of packets (2, 10, 40, 80): https://user-images.githubusercontent.com/4350690/174196918-6af428e4-9ab7-4458-b3b9-e27ed89c3318.png

**lessons learned:**
- we had to be cautious about delaying acks because we were operating on public
  internet
- I wonder if an environment like DC would be better for delaying acks
- acks are a signal within the noise of loss,delay,congestion,etc
- within a DC there is less noise so it makes sense that we can have less signal
  and get away with it

