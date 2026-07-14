+++
title = "QUIC OACK Security Vulnerability"
date = 2025-04-01

[taxonomies]
tag = ["work"]

[extra]
company = "Cloudflare"
highlight = true
+++

Two months into the Protocols role at Cloudflare, I was tasked with patching a security
vulnerability in our QUIC library. At the same time, my two principal engineers had planned vacation
for two weeks, so I would need to make progress by myself.

Protocol-level vulnerabilities are notoriously difficult to debug: implementations tend to be
RFC-compliant, but reproducing a security issue requires a deliberately non-compliant counterpart.
Reproduction therefore requires a deep understanding of the RFC and codebase to implement the
non-compliant behavior.

I owned the full remediation lifecycle: reproducing the issue, implementing the fix, patching the
proxy, and coordinating patches across three other infrastructure teams. I had the fixes ready for
review when my principal engineers returned and got them merged internally with minimal feedback. I
patched our proxy and verified the vulnerability was fixed. Next, I cut tickets for three other
teams and tracked the fix over the next month.

