+++
title = "Impossibility of Distributed Consensus with One Faulty Process"
date = 2017-09-24

[extra]
paper = "FLP_imposibility.pdf"
short = "The consensus problem involves a system of asynchronous processes, some of which may be unreliable/faulty (die or crash). This paper proves that every solution, with even a single faulty process, has the possibility of non-termination. The important takeaway: we can design our systems to make the possibility of a livelock small, but the probability is non-zero."
+++

Despite being a short paper, the proofs themselves were pretty confusing. This <a href="https://www.the-paper-trail.org/post/2008-08-13-a-brief-tour-of-flp-impossibility/" target="_blank" rel="noopener">post</a> was critical for my understanding.

The proof is comprised of two lemmas. The first lemma showed that there must be some initial configuration where consensus is not yet determined (caused by errors or message delays). Think of this as **bivalent** (having two possible truths) state. The second lemma shows that it is always possible to remain in a **bivalent** state by delaying a message.

Lets note the assumptions made by the proof: The underlying transport protocol is reliable; messages are delivered correctly and exactly once. There is no synchronized clock, and it is not possible to detect a slow process vs a dead process. Additionally, the proof relaxes the **termination** requirement and requires that only some process eventually decide on a value (weak consensus). Put another way, a message will always be delivered but can be delayed and can be delivered out-of-order.

For Strong Consensus these 3 properties must hold: **termination**: eventually, every correct process decides some value. **agreement**: all processes that decide do so on the same value. **validity**: the value must have been proposed by some process.

