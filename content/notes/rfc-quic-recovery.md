+++
title = "Quic Recovery overview"
date = 2021-03-01


[taxonomies]
tag = ["crypto", "quic"]

[extra]
id = blog-single
+++

Understanding QUIC recovery

explanations: https://quicwg.org/ops-drafts/
summary: https://quicwg.org/ops-drafts/draft-ietf-quic-manageability.html#name-using-the-spin-bit-for-pass
rfc: https://tools.ietf.org/html/draft-ietf-quic-recovery-32

### RFC Questions:
6.2.1 When ack-eliciting packets in multiple packet number spaces are in flight, the timer MUST be set to the earlier value of the Initial and Handshake packet number spaces.

Q: the code always takes the earliest timer. do we ever take not earliest timer? how many times do we maintain in out quic impl (one per packet number space or arbitraty amount)?

---

6.2.3 with regards to (Speeding Up Handshake Completion and loss of packet). Endpoints can also use coalesced packets (see Section 12.2 of [QUIC-TRANSPORT]) to ensure that each datagram elicits at least one acknowledgement.

Q: this seems like good advice to ensure that each packet gets acknowledged. do we follow this and/or why not?

---

7.3.3 A sender in congestion avoidance uses an Additive Increase Multiplicative Decrease (AIMD) approach that MUST limit the increase to the congestion window to at most one maximum datagram size for each congestion window that is acknowledged.

Q: Do we use AIMD or some other algorithm? Aslo what does "one maximum datagram size" mean?

---

7.7 Sending multiple packets into the network without any delay between them creates a packet burst that might cause short-term congestion and losses. Senders MUST either use pacing or limit such bursts.o

Q: do we implement pacing (why not)? based on compliance report it doesnt seem like we implement this but it seems important.



### General questions
Q: - what congestion control do we use in QUIC? Seems like CUBIC with Hybrid Slow Start.

Q: - more generally what will be the effect of middleboxes no longer being able to inspect and alter network traffic?

