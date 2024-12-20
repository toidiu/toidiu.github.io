+++
title = "Access and expose recs generated by Data Science"
date = 2019-01-11

[extra]
company = "ihr"
lp = ["invent and simplify", "frugality", "earn trust"]
+++

invent and simplify: come up with a solution and then simplify

earn trust: negotiate a solution with DS to further the relationship

frugality: reuse dynamo, jenkins rather than invent new solution

#### metrics
- nightly job; system designed for time agnostic release
- kept 7 days of backups
- provide interface for testing
- reuse existing infrastructure (dynamo, jenkins, schema models)
- poll for new dataset every 5 min

#### S
- Data Science (DS) ran a nightly job to generate music recommendations for
users.
- The dataset would live in DynamoDB and the old DS workflow was to rewrite the
  same dynamo table with new dataset each night.
- This was a high risk operation for the my team (APIs). Additionally it caused
  some outages due to accidental schema changes.

#### T
- I was in charge of creating a HA and resilient workflow.
- The ownership of the data would reside with them.
- We simply wanted assurances that there were some sanity for newly published
  data.
- Treat data as immutable
  - Enforce that DS publish the dataset to a new table each night.
  - Maintain a backlog of 7 datasets into the past.
  - This gives the benefit of 'rollback' if a new dataset was broken
- Given multiple datasets (a,b,c); DS can 'point' to latest dataset
  - DS can maintain a few versions of the data. "Version Table"
  - DS can run tests on new datasets to confirm schema compatibility
  - rollbacks are as easy as pointing to an older known dataset
- Maintain a log of actions (publish new dataset, test pass/fail, point to new
  dataset)

#### A
- Work closely with DS to come up with the system design.
- Added a Jenkins tests which could be triggered by DS
  - used to verify that data schema would not be a breaking change
  - extensible model for other types of tests
  - potential to track failing/passing metrics
- Poll based mechanism within API code to look at "Version Table" and start
  using the latest dataset.
  - A poll interval of 5 min was used since 'older' data would still produce
    good enough data.
  - Log when a new dataset was detected (help correlate errors with new dataset)

#### R
- We saw a system that could be used to audit the creation of new datasets.
- Outages due to breaking schema's were eliminated.

