+++
title = "Proxy H3 Migration"
date = 2026-03-01

[taxonomies]
tag = ["work"]

[extra]
company = "Cloudflare"
highlight = true
+++

I led a complex and high-risk production migration of CDN traffic at Cloudflare. The project was
challenging because it required significant effort to align a diverse set of stakeholders while
ensuring the technical decisions were sound. The goal was to migrate 4.5M+ unique customers (25M+
req/sec) from nginx to the new Rust proxy. With stakeholders spanning directors, PMs, customer
support, and engineers across the organization, I had to understand our system both at the technical
and operational level. I started by developing a deep understanding of how our proxy integrated with
the networking infrastructure at Cloudflare (Addressing, DNS, and load-balancing). For example, this
included understanding how the IP Addressing team routed traffic for different customer tiers and
how they load-balanced requests across different processes.

At multiple stages I explored various technical solutions, weighing the “soft” vs “hard”
requirements for different stakeholders while building alignment. For example, due to customer
requirements, it was not possible to change IPs for existing customers. Since IPs could only be
assigned to a single proxy process, this meant customer migration would require migrating IPs
between the old and new proxy. Critically, this meant we could not write clean logic around customer
IDs and would instead need to integrate with the Addressing team to migrate IPs. While IP stability
was critical to a successful migration from the customer's perspective, it introduced major
technical challenges. Since most of the 4.5M+ customers had single-address prefixes (/32 and /128),
this means I would have to create batches from 4.5M IPs while considering customer tier, blast
radius, and rollback status. I was able to gain consensus for this design by drafting several
proposals and asking stakeholders to flag "hard" blockers. This helped surface non-viable solutions
early and kept the project moving.

Once we had rough consensus on a technical solution, I began drafting a detailed specification of
the migration plan. Because this was such a large and dynamic project, my goal was to create a
living document and iteratively build toward the final plan. I completed the first draft within a
week and set the plan in motion. I got initial feedback from my principal engineer to confirm the
general engineering direction made sense. Then I expanded the review to the entire team, the PM, and
my skip-level manager. Importantly, throughout this entire process I captured all feedback as
outstanding "TODO(owner)" items, assigning each to the correct owner (usually myself). This approach
added visibility into all decisions and let me keep the momentum going.

At 4.5M IPs, I very quickly realized the need for a software-based solution (migration-utility) that
could manage the migration state, add guardrails, and most importantly act as the single source of
truth. The migration-utility turned out to be critical for building trust. It enabled safe migration
operations (progress, rollback) for the entire on-call team and gave customer-support teams
visibility into the migration state. I was also able to build trust with the Addressing team by
building a "staging" feature flag in the migration-utility and executing test migrations against the
staging version of the Addressing database.

The migration was approved for execution but unfortunately I was impacted by layoffs just before I
could execute it. I am unaware of the current migration status but believe my work should be
reusable since I wrote a detailed specification outlining and justifying the plan, an Engineering
SOP, a Customer Support SOP and a migration-utility that automates the dangerous part of the
migration.

