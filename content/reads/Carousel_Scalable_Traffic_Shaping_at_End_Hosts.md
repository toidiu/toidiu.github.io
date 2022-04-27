+++
title = "Carousel: Scalable Traffic Shaping at End Hosts"
date = 2022-04-26

[extra]
paper = "Carousel_Scalable_Traffic_Shaping_at_End_Hosts.pdf"
short = "Traffic shaping is a critical feature for datacenters. It is used to apply policy-based bandwidth allocation, for packet pacing, and used by rate-based congestion control. Typical traffic shaping mechanisms can be CPU expensive, while not being very accurate. Carousel improves upon the state of the ary(8% CPU savings); i) a single queue shaper, ii) fine-grained, just-in-time freeing of resources coupled to packet departure iii) one shaper per CPU core with lock-free coordination."

+++

Network bandwidth is an expensive resource to overprovision and bursty links can lead to packet loss, less accurate bandwidth estimation, longer RTT times. Deep buffers, have typically been used but lead to poor latencies. Traffic shaping refers to pacing: injecting inter-packet gaps to smooth traffic and rate limiting: enforcing rate on flow-aggregate on connections. Within datacenters, the need to shape traffic is critial since there are multiple customer VMs which all compete for network bandwidth.

**traditional shapers**

Policers: a token bucket mechanism to assign resources to a flow, with zero buffering and packet dropping as a side effect.

HTB: a complex buffer/token bucket/tree structure to suport advanced traffic management. Backpressure is needed to avoid unbounded queue growth. Can group flos into individual or aggregate groups and thus apply complex logic to traffic shaping.

FQ/pacing: FQ tracks per-flow sate in an array of Red-Black trees indexed on flow hash IDs. Pacing is based on packet length and current pacing rate. Future packets are kept in a separate RB tree index.

**cost of shaping**
...

**carousel design principles**
...
