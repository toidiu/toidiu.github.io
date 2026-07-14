+++
title = "QUIC ACK frequency"
date = 2022-04-01

[taxonomies]
tag = ["work"]

[extra]
company = "AWS Transport Libs"
+++

Tracking issue: https://github.com/aws/s2n-quic/issues/1276
Final PR to enable delayed ACKs: https://github.com/aws/s2n-quic/pull/1298

The project that best reflects "good engineering" for me was implementing the experimental ACK
frequency RFC in s2n-quic at AWS. Based on flamegraph analysis, ACK processing was an expensive part
of the QUIC stack, and the motivation behind the work was to reduce the ACK frequency to make the
protocol more efficient. The RFC proposed a custom protocol extension which required peer
negotiation and could result in less frequent ACK processing. I started by understanding the RFC and
writing a specification for team review.

The first proposal was to implement the full RFC, but this had many drawbacks. Since the extension
required peer negotiation and the RFC was experimental, it was unlikely that any peer would support
the feature. Additionally, tuning the frequency of ACKs would be non-trivial and would likely depend
on customer workload. A lower frequency of ACKs would also impact Congestion Control and Loss
Recovery in unpredictable ways, potentially harming connection stability. High implementation effort
and low reward meant this was the least desirable option.

The second proposal was to batch ACK information on the receive path across multiple GSO payloads
and process it in bulk: Receive path (multi-round). The theory was that bulk processing would mimic
the benefits of less frequent ACK delivery. Instead of negotiating the ACK frequency with the peer,
the local implementation would queue the ACK information from received packets across multiple GSO
payloads (tunable) and process it once. We suspected that the expensive part in the receive path was
the context-switching required for each QUIC packet. The benefit of this approach was the lack of
extension negotiation, which meant all connections could benefit from the work. The risk to
Congestion Control and Loss Recovery remained, however.

The third proposal, and my recommendation, was to batch ACK information across packets in a single
GSO payload: Receive path (single-round). Each GSO payload is essentially a collection of QUIC
packets (~10) that are transmitted and received together. The default implementation processed the
ACK information in each packet individually, and the insight was that we could extract CPU savings
by first queuing and then processing it once per payload. The benefits over the multi-round approach
were that CloudFront would not need to tune their systems and there would be no impact on Congestion
Control and Loss Recovery. Another benefit was that the work would naturally extend to the more
sophisticated proposals above.

After the team agreed with my recommendation, I started to implement the Receive path (single-round)
proposal. The work was broken down into small atomic PRs that exposed queuing and delayed packet
processing on the receive path. The final PR was simple and mostly enabled the feature.

However, to my surprise, the new code actually performed worse in CI tests, regressing from 24.33%
to 25.81% of CPU time in flamegraph analysis. I investigated the regression by running download
simulations with different payload sizes and confirmed there were no performance improvements.
Backed by data, we decided to abandon the feature. My theory is that queuing ACK information was
more expensive than processing it right away in s2n-quic.

While the project was not a success in the typical sense, it demonstrated the rigorous engineering
process and testing that caught the regression before it was merged. I also suspect that ACK
frequency would be more beneficial in a data center environment where network links are faster, peer
negotiation is possible, and there is less loss on the network.

