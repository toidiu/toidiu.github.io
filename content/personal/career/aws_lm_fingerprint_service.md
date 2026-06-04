+++
title = "Fingerprint service"
date = 2019-12-01

[extra]
company = "aws_lm"
lp = ["invent and simplify", "bias for action"]
+++

- invent and simplify: re use hibernate codebase, especially the tests and
validation. also simplify the logic to make it more adaptable.
- bias for action: research other team's usecase; simplify and structure codebase
to allow for extensibility. drove consensus when technical issues(java type
generics) or framework choices made a right choice non obvious.

FAQ: https://quip-amazon.com/3ujuAnetkBr1
Dashboard: https://w.amazon.com/bin/view/EC2/LiveMigration/Dashboards/DropletFingerprintService/

### S
- KaOS owns fingerprint rules.
- KaOS deletes and then creates a DynamoDB table with updated rules. This causes elevated ICEing.
- LM consumed these rules.

### T
- Create a single highly available service to serve fingerprint rules.
- De-risk KaOS rolling out new fingerprint rules.
- Take the opportunity to change the format of fingerprint rules. This was a high risk item since a change in format made it difficult to detect mistakes in parsing logic.
- Emit metrics for fingerprint matching rules to track the flow migrations.

### A
- Wrote a FAQ to convince leadership: https://quip-amazon.com/3ujuAnetkBr1
- Coordinated with the KaOS team and launch a multi-region LM service.
- Wrote a lot of tests to ensure finger format change were equivalent.
- Mentored junior teammate (karthiga) on creating a detailed service health dashboard.
- Conducted ORR to ensure service best practices.
- Created a fully automated service using LPT, managed fleets, and paperwork.

### R
- Launched the service under MCM.
- There was zero impact to LM during the migration (due to the detailed metrics and tests).
- The ORR and automation served as a template for future service launches for the team.


---
### Extra info:

- wrote prfaq to convince leadership
- wrote a deserializer to safely transition from old parser + old rule format ->
  new parser + new rule format
  - multiple unit tests to ensure rules were exactly the same during migration
- automated release of service with pipeline + rollback alarms
  - replaced MCM practice (manual inspection of metrics over ~2 weeks)
- regional dashboards along with alarms for automated and manual debugging
  - set a template for regional dashboards
- managed SDE1 who helped create alarms for the service
- learn and bootstrap out paperwork
  - unlocking team to move from substrate to prod
- unblocked self by communicating cross teams for paperwork in substrate
- provision tod workers for lm team
  - lpt managed
  - quilt pipelines
- conducted an ORR for service pre/post launch (balancing the risk of doing some
  task post launch)
- successfully transfered the ownership of 'rules' to sister team (8hr time
  difference)
  - email + meetings + ongoing support
- released under MCM
  - set team standards for not having false alarms prior to launch

