+++
title = "Fingerprint service"
date = 2019-12-01

[extra]
company = "amzn_lm"
lp = ["invent and simplify", "bias for action"]
+++

invent and simplify: re use hibernate codebase, especially the tests and validation. also simplify the logic to make it more adaptable.

bias for action: research other team's usecase; simplify and structure codebase to allow for extensibility. drove consensus when technical issues(java type generics) or framework choices made a right choice non obvious.

#### metrics
- eliminate 5-6 abstractions across 30 files
- structure code to support various types of fingerprints for various teams
- deliver a working first version of the API and provide value to the team. added testing for places where java type erasure made it difficult to add fully typed checked code.

- wrote prfaq to convince leadership

- wrote a deserializer to safely transition from old parser + old rule format -> new parser + new rule format
  - multiple unit tests to ensure rules were exactly the same during migration

- automated release of service with pipeline + rollback alarms
  - replaced MCM practice (manual inspection of metrics over ~2 weeks)

- regional dashboards along with alarms for automated and manual debugging
  - set a template for regional dashboards
- managed SDE1 who helped with some tasks

- learn and bootstrap out paperwork
  - unlocking team to move from substrate to prod
- unblocked self by communicating cross teams for paperwork in substrate

- provision tod workers for lm team
  - lpt managed
  - quilt pipelines

- conducted an ORR for service pre/post launch (balancing the risk of doing some task post launch)
- successfully transfered the ownership of 'rules' to sister team (8hr time difference)
  - email + meetings + ongoing support

- released under MCM
  - set team standards for not having false alarms prior to launch

#### S

#### T

#### A

#### R

