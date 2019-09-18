+++
title = "other"
date = 2019-08-11

[extra]
company = "iHeartRadio"
lp = ["highest standard"]
+++

- jvm debugging
- production outage - tracking list of services and restarting them

- commiting to rust lang issue and following thru on it (7 months)

 - Worked closely with Data Science/Design/iOS to help launch music recommendaRon service and make it more resilient.
     - migrating old recs api to new micro service logic
     - HA for data published by DS
     - realizing fastly was caching recs
       - change from blacklist to whitelist and rolling it out in production

 - debugging and optimizing Mongo query performance across different services
 - write unit and mongo embed tests across the many services
 - I interface with kubernetes on a daily basis. k8s is quite complex so many of the debugging tasks involve creating and refining the mental picture of how it actually works. This is then used to debug and further improve upon service performance.


