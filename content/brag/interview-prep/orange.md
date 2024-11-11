+++
title = "orange"
date = 2024-04-01

[taxonomies]
tag = ["brag"]

[extra]
id = "blog-single"
+++

## Project retrospective: 1hr
TODO:
- pick two projects and be able to dive into project details

### ACK frequency
- tracking issue: https://github.com/aws/s2n-quic/issues/1276
- cpu increase: https://github.com/aws/s2n-quic/pull/1298
- revert pr with some analysis shows : https://github.com/aws/s2n-quic/pull/1368

#### RFC:
- Acks uses:
  - progress: acknowledge packet received
  - loss recovery: detect a packet was not received and retransmit
  - cc: regular acks help establish good RTT estimates and drive CC
  - ECN: congestion signal by the network
```
ACK Frame {
  Type (i) = 0x02..0x03,
  Largest Acknowledged (i),
  ACK Delay (i),
  ACK Range Count (i),
  First ACK Range (i),
  ACK Range (..) ...,
  [ECN Counts (..)],
}
```
- motivation: expensive cpu to send/recv. asymmetric links like satellite. ack
  rate can affect link efficiency.
- negotiating the extension: min_ack_delay. the minimum amount of time, in
  microseconds, that the endpoint sending this value is willing to delay an
  acknowledgment
- ACK_FREQUENCY frame:
  - Ack-Eliciting Threshold: max ack eliciting **packets** received before sending an ack
  - Requested Max Ack Delay:  max **time** to wait before sending an ack
  - Reordering Threshold: max number of **reordered packet** before sending an ack
- IMMEDIATE_ACK frame: asks peer to send an immediate ack
- Expedite ECN: ecn marked packets should not be delayed
- other concerns:
  - congestion control:
  - application limited:
  - burst mitigation: if sending is dependent on recieving acks. use pacing
  - loss detection is hard: RFC doesnt provide a good solution
  - connecion migration: send IMMEDIATE_ACK to expedite path validation. ack
    frequency is NOT reset!! on new path.. but should be update to the new path

### optimistic ack mitigation
- pr https://github.com/aws/s2n-quic/pull/1986
- rfc link https://www.rfc-editor.org/rfc/rfc9000#section-21.4

### netbench

## Technical: 1hr

## Orange cloud: Behavioral: 30 min
TODO:
- pick 5 projects and STAR them.
- read https://www.hellointerview.com/learn/behavioral/overview/introduction

**TOP**
- A design choice that encured technical debt but you think it was a good choice.
- Tell me about a time you worked well within a team.
  - Helped co-worker in CLOUDFRONT integrate with async cert loading feature.
  - helped deliver quic over 2 years. specifically worked in a team of three to review prs, make design
    decisions etc.
  - connection migration pentest
  - orchestrator. i implemented the library logic and he helped me with the automated infra
- Tell me about a time you dealt with conflict on a team. How did you solve it?
  - prioritize features for the orchestrator. we met with the stake holder and figured out which
    features they actually wanted to help prioritize the project
  -
    - How do you respond when you disagree with a coworker?
        - disagrements usually stem form mis-matched assumptions or lack of
          communication. so i like to start with having a conversation and
          trying to understand why there is a disagreemet. usually when this
          happens we both end up learning something.
- Tell me about a time you failed at work.
  - ack frequency project. the cpu usage went up after implementing the solution
- Can you give me an example of how you set goals for yourself?
  - daily task list. mark what got done and what did not
- **Tell me about a time you showed leadership.**
  - spoke up about technical decisions that I spoke out on
  - representing someone external to yourself
    - took initiative to fix handshake status. added fuzz testing verify the solution.
    - automate release versioning
    - customer contact list
- Tell me about your biggest weakness.

**MID**
- Tell me about a time you faced a really hard problem / a challenge at work.
- Tell me about an interesting project you’ve worked on recently.
- Tell me about a time you had to meet a tight deadline.
- Tell me about a time you had to prioritize projects under pressure.

**LOW**
- What is the most helpful feedback you've ever gotten?
- As a software engineer, you must be both predictable and innovative. How can these traits coexist
  in your work?
- Tell me about a time you adapted to a new situation or environment.
- Tell me about a time you needed information from someone who wasn't responsive. How did you
    handle it?

**Other**
- Why do you want to work here?
- Walk me through your resume and relevant experience.

## System Design: 1hr
TODO:
- read https://www.hellointerview.com/learn/system-design/in-a-hurry/introduction
- design 2 systems as practice

## Executive call

- It seems to me that CLOUDFLARE is in a crowded market space (cdn/securing the internet (fastly),
  paas (aws), data security (crowdstrike)). What do you believe is your single biggest technical or
  non-technical advantage that will make you successful over your competitors?
- What is the top kpi/metric you personally pay attention to, to make sure CLOUDFLARE is maintaining
  its momentum?
- What is the most imporant thing CLOUDFLARE does to build high-functioning and talented teams?
- In one of you blog posts you mentioned: "we believe our continued success should be primarily
  driven by new innovation, not by milking old features for revenue."
  https://blog.cloudflare.com/adjusting-pricing-introducing-annual-plans-and-accelerating-innovation/
- As you continute to release new features and tools how do you ensure a seamless user experience
  and avoid overloading your users? (AWS has over 200 services).

