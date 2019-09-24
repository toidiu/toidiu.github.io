+++
title = "Batch job alerting and little knowledge of system (on-call)"
date = 2018-08-11

[extra]
company = "ihr"
lp = ["bias for action"]
+++

bias for action: took action with limited info, while evaluating risk.

#### metrics
- 10K messages queued
- 10pm with no response for rest of team
- silenced alarms for 2 hr increments
- after 3 occurance and 2am alarm made decision to terminate job

#### S
I was on call got a page at approx 10pm. There was a rabbitMq that had backlogged and was alerting.
The first hurdle was that there was no login/password information about how to access the queue
and see what was going on.

#### T
I tried to reach the secondary in the hopes of gathering more information but to no avail.
I tried the tertiary(boss) and also the primary on the ingestion team but received no answer.

My task at this point became deciding how to proceed.

#### A
I was able to get login information from the SRE oncall and able to inspect the queue. At this point
I saw that a message was causing a backlog so I cleared it manually.

Once the queue started draining I silenced the specific alarm and went about the night. However the
error happened again and I noticed that there was another message now causing the backlog.

It became apparent that manually skipping the error message was not a solution. There were approx 10k
messages queued, which was the limit of the queue but I suspected that if the backlog continued it could
fill up the drives causing more damage.

#### R
It was also apparent that the batch job was not correct, in the sense that it was unable to handle
all message types, which was resulting in the backlog. I therefore decided to cancel the batch job
and drain the queue.

Since it was a batch job and it was being developed (not facing), there was little harm in stopping
the alert and job, which could be looked at in the morning.

