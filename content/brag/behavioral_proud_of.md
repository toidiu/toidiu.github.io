+++
title = "What work have you done that you are particularly proud of and why?"
date = 2026-06-01

[taxonomies]
tag = ["behavioral"]

[extra]
company = ""
highlight = false
+++

I am proud of my work in launching a new EC2 Fingerprint service at AWS. The service replaced core
legacy logic which was used to determine EC2 instance compatibility. I am especially proud because
the migration to the new service went smoothly due to the extensive testing and observability I
added.

My team owned an orchestrator which consumed the compatibility rules from a DynamoDB table owned by
the hardware team. Rules were frequently updated by the hardware team but would occasionally result
in broken metals and customer impact. One of the motivations behind the Fingerprint service was to
address this customer impact, scale to support more rules, and act as a single source of truth for
EC2 compatibility for the organization.

There were a few reasons why this project was especially risky. First, due to the large number of
compatibility rules (1500), a manual audit of the rules was impractical. Second, the hardware team
wanted to change the rule format, which meant ensuring parity when switching to the new service was
a lot harder. Finally, any mistakes in compatibility meant serious customer impact.

The development work was tricky due to the changing format, and I spent most of my time writing
tests. To build confidence, I wrote a test to generate all possible compatibility combinations and
compare them between the new and old format. This testing ended up catching a few mistakes, which
the hardware team then fixed.

After the testing, I was very confident about the logic, but given the high potential impact for
customers, I decided to integrate safely and double-check the logic in production. I integrated the
new service into the orchestrator but only enabled it in "observability mode". Each call would
generate a compatibility result from both the new and old service and emit a metric if there was a
difference. After running for a week without any mismatch, I started the production release. As
expected, there was zero customer impact.
