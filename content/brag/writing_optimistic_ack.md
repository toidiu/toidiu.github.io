+++
title = "Blog: Optimistic ACK attack"
date = 2026-06-01

[taxonomies]
tag = ["writing"]

[extra]
company = ""
highlight = false
+++


https://blog.cloudflare.com/defending-quic-from-acknowledgement-based-ddos-attacks/

I loved Cloudflare's culture of transparency and the detailed blogs they publish. When I got the
opportunity to talk about my work patching a security issue, I took it.

My goal with the blog post was to make the topic approachable to all engineers and teach them a
little about the networking domain. I dedicated many of the earlier sections to building up the
reader's intuition about protocol design, congestion control, and the idea of fairness in
networking. Finally, I discussed the Optimistic ACK attack and presented the dynamic packet-skip
patch that mitigated the vulnerability.
