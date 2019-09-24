+++
title = "Own production outage when on-call was in transit"
date = 2018-03-11

[extra]
company = "ihr"
lp = ["ownership", "customer obsession"]
+++

ownership: the api is owned by the team and not only the responsibility of an individual.

customer obsession: prod outage means users are being affected

#### metrics
- less than 1/2 hr outage
- restarted 3-4 failing services
- restarted up to 15 pods across all services that were in a bad state

#### S
Towards the end of the work day a sestamatic outage started to happen. The oncall member was in-transit and not available to handle the issue.

Later we found out that the outage was due to a combination of AWS upgrade event and the weave CNI not being able to maintain a network mesh.

#### T
Step up and represent the API team in fixing the outage.

#### A
Restarted services that were showing errors in prometheus. Also tracked each service individually to ensure that there were no lingering bad pods.

#### R
Outage lasted less than 1/2 hour. Monitored the state of the system for a total of 1 hour.

