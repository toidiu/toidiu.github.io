+++
title = "Max connections and failing kube watch"
date = 2018-12-11

[extra]
company = "ihr"
lp = ["right alot", "bias for action", "dive deep"]
+++

right alot: pin point the differences between stg and prod

bias for action: execute a non invasive solution, which could go out quick while still preventing issues

#### metrics
- errors occured at 1-2 week increment
- random 500 errors for a particular api only
- different behavior across stg and prod envs
- 5 max connections
- fixed issue for 2 additional micro services

#### S
An API that I wrote and owned would start to 500 errors sporadically. When
Additionally, when I inspected the stg vs prod environments the results would
differ and not always align and would happen at seemingly random times.

Restarting the container would fix the issue however.

#### T
After 2-3 more occurances, it became apparent that there was a deeper issue.
In response I decided to add logging to the service around the relevant code.

At the next occurance we noticed from the logs that the IP of the service was
did not match the one being requested.

#### A
I then took a look at mechanism that was doing service discovery and added
additionally logs. This showed that not all the kube watch events were
working correctly.

Further dive revealed that this condition was met at a convenient number of
5 connections. Diving further into the http library used by the service
discovery library(pandorum) I discovered that OkHttp has a default limit
of 5 persistent connections.

#### R
I added a config change to increase the max connection limit. Additionally,
I added a check at startup to confirm that the limit matched the number of
services we were trying to connect to so as to prevent future failures.

