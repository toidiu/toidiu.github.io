+++
title = "QUIC ACK frequency"
date = 2026-03-01

[taxonomies]
tag = ["work"]

[extra]
company = "AWS Transport Libs"
+++
## Context
ACK processing is one of the most CPU expensive aspects of the QUIC protocol since every packet
requires an ACK. For example, a send heavy workload spends ~24.33% processing ACKs. My task was
therefore to look at the ACK delay RFC and see if it would be possible to apply it to s2n-quic and
get some CPU savings.

As the project lead I started with a deep read of the RFC text, followed by a proposal doc that was
reviewed by the entire team. The RFC proposed a new QUIC extension (negotiated by both peers), which
would allow for delaying and batching packet ACKs.

The obvious benefit was the lower CPU cost. However the adverse affects included:
- Loss recovery: ACKs are critical for detecting and recovering from lost packets.
- Congestion control:
  - CCA: ACKs help establish RTT, which is critical for most congestion control algorithms.
  - BBR: requires high-precision so its not obvious how the algorithm might break.

## Action
In my proposal I proposed the following options:
- Single round batching of the 10 GSO packets.
  - Easy to implement (1-2 sprints).
  - The code changes would be a building blocks for the other solutions.
- Multi round batching (multi-gso payload) (2-4 sprints)
  - This solution was medium difficulty to implement.
  - Requires tuning with application, production use case, traffic pattern
- Implement the full ACK delay RFC (4-8 sprints)
  - Complicated to implement since it involves the packet processing plumbing.
  - Requires tuning with application, production use case, traffic pattern.
  - Requires negotiating the extension

I recommended implementing the single round batching since it was the quickest way to get benefits
in production and also would naturally lead to the other solutions.

The implementation involved the following tasks:
- Allowing connections to signal interest in batch ACK processing
- Queuing pending ACKs on the connection
- Refactoring Congestion Control and Loss Recovery to accept a ACK timestamp delay signal
- Emitting metrics

## Result
Once the feature was complete however it showed a CPU performance regression! The flamegraph result
showed a regression from 24.33% -> 25.81% (https://github.com/aws/s2n-quic/pull/1298).

Finally the PR was reverted and we determined that ACK delay was likely not going to yield the
desired benefits for s2n-quic (https://github.com/aws/s2n-quic/pull/1368).

## Learning
On the surface this might seem like a failure, however it was one of my most rewarding
engineering efforts. We were able to take a complex feature, implement a POC to demonstrate the
regression and avoid months of wasted effort. Additionally, its not trivial that we were able to
measure the tiny regression and abort the feature development.

ACK delay is probably too risky a feature for a protocol that needs to operate on the public
internet. Instead I think it might be a better fit for Datacenter traffic, which has much lower RTTs
and loss rates.

### References
- RFC https://datatracker.ietf.org/doc/html/draft-ietf-quic-ack-frequency
- Tracking issue: https://github.com/aws/s2n-quic/issues/1276
- CPU increase: https://github.com/aws/s2n-quic/pull/1298
- Revert PR with some analysis shows : https://github.com/aws/s2n-quic/pull/1368
