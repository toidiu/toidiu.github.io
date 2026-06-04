+++
title = "QUIC OACK Security Vulnerability"
date = 2025-04-01

[extra]
company = "Cloudflare"
highlight = true
+++

Two months into the Protocols role at Cloudflare, I patched a security vulnerability in our QUIC library. Protocol-level vulnerabilities are notoriously difficult to debug: implementations tend to be RFC-compliant, but reproducing a security issue requires a deliberately non-compliant counterpart. Reproduction therefore requires a deep understanding of the RFC and codebase to implement the non-compliant behavior.

I owned the full remediation lifecycle: reproducing the issue, implementing the fix, patching the proxy, and coordinating patches across three other infrastructure teams. I implemented the fix along with tests within two weeks, and our proxy was patched within the month.
