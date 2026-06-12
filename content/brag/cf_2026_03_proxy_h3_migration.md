+++
title = "Proxy H3 Migration"
date = 2026-03-01

[taxonomies]
tag = ["work"]

[extra]
company = "Cloudflare"
highlight = true
+++

Led the migration of 4.5M+ unique customers (25M+ req/sec) to the new proxy. With stakeholders
spanning directors, PMs, customer support, and engineers across the organization, I started by
deeply understanding our system and how it integrated with the networking infrastructure. Next I
explored various technical solutions, considering both the "soft" vs "hard" requirements for various
stakeholders and building project alignment.

The challenges included noisy observability (due to DDoS traffic) and the inability to perfectly
isolate customers via software due to performance requirements. To manage risks, I built a utility
to progress, query and roll back the migration with per-customer granularity. I included guardrails
to prevent accidental misuse and led a mini workshop for our On-call engineers. I used AI to
accelerate batch creation and utility development, saving approximately 2-4 weeks of effort.
