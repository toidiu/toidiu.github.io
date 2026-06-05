+++
title = "For one of Oxide's values, describe an example of how it was reflected in a particular body of your work."
date = 2026-06-01

[taxonomies]
tag = ["behavioral"]

[extra]
company = ""
highlight = false
+++

**Oxide values:** Candor, Humor, Teamwork, Courage, Optimism, Thriftiness, Curiosity, Resilience, Transparency, Diversity, Responsibility, Urgency, Empathy, Rigor, Versatility

Making sure the software we ship is secure is one of my core principles. Two months into the Protocols role at Cloudflare, I was tasked with patching a security vulnerability in our QUIC library. At the same time the Principle engineers had a planned vacation for two weeks so I would need to make progress by myself.

Protocol-level vulnerabilities are notoriously difficult to debug: implementations tend to be RFC-compliant, but reproducing a security issue requires a deliberately non-compliant counterpart. Reproduction therefore requires a deep understanding of the RFC and codebase to implement the non-compliant behavior.

I owned the full remediation lifecycle: reproducing the issue, implementing the fix, patching the proxy, and coordinating patches across three other infrastructure teams. I had the fixes ready for review when my principle engineers returned and got them merged internally with minimal feedback. I patched our proxy and verified the vulnerability was fixed. Next I cut tickets for 3 other teams and tracked the fix over the next month. For more details, check out the blog post.
