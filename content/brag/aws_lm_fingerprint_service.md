+++
title = "Fingerprint service"
date = 2019-12-01

[extra]
company = "aws_lm"
lp = ["invent and simplify", "bias for action"]
+++

invent and simplify: re use hibernate codebase, especially the tests and
validation. also simplify the logic to make it more adaptable.

bias for action: research other team's usecase; simplify and structure codebase
to allow for extensibility. drove consensus when technical issues(java type
generics) or framework choices made a right choice non obvious.

#### metrics
- eliminate 5-6 abstractions across 30 files
- structure code to support various types of fingerprints for various teams
- deliver a working first version of the API and provide value to the team.
  added testing for places where java type erasure made it difficult to add
  fully typed checked code.

#### S
- fingerprinting ec2 instances was done across multiple services (3 services at
  the time)
- this was a set of rules loaded into memory
- the rules were updated ever so often but changes were scary since they were
  manually done for each service

#### T
- create a fingerprint service that could
  - support a new format of fingerprint
  - support multiple services (with possibly slight variants of fingerprints)
- swap the fingerprint service for the static rules in a critical service

#### A
- write a prfaq to convince leadership and tech lead
- write generic java code that could represent various types of fingerprints
- write **alot of tests** to confirm that the old vs new fingerprint format was the
  same.
  - nervous about this. was writing test in my car on a camping trip. felt a
    relief once i had this test in place
- mentor a new SDE to create alarms for the service (how to think about
  thresholds, which metrics to look at, sev 2 vs sev 3, create a centralized
  dashboard)
- alarms included:
  - application (consuming fingerprint) error rate
  - request per second (by region, fingerprint type)
  - host alarms
    - jvm memory
    - cpu
    - disk
    - cert expiration
- create pipeline definition to release the service
  - automated managed fleets
  -
- lead an ORR (operation readiness review), included security review
  - first ORR for the team. This has become a requirement across many teams in
    AWS since then

#### R
- released the service under MCM (change management)
- integrate with application and emit success metric (monitor for 1 week)
- after seeing 0 errors we swapped to the service with 0 impact

---

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

