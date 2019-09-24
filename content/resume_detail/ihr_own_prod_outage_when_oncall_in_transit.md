+++
title = "Own production outage when on-call was in transit"
date = 2019-08-11

[extra]
company = "iHeartRadio"
lp = ["ownership", "customer obsession"]
+++

ownership
  the api is owned by the team and not only the responsibility of an individual.
customer obsession
  prod outage means users are being affected

#### S
Towards the end of the work day a sestamatic outage started to happen. The oncall member was in-transit and not available to handle the issue.

#### T
Step up and represent the API team in fixing the outage.

#### A
Restarted services that were showing errors in prometheus. Also tracked each service individually to ensure that there were no lingering bad pods.

#### R
Outage lasted less than 1 hour.

