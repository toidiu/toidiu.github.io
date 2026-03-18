+++
title = "quiche invalid state error"
date = 2026-01-01

[extra]
company = "cf_proto"
lp = []
+++

https://github.com/cloudflare/quiche/pull/2366

#### S
- bastion conn/sec:                 700,000
- error per day (1% sampling):      1,000,000
- error per day (100% sampling):    100,000,000
- sec in day:                       86,400
- error per sec:                    1,157
- bastion conn error %:            .17%

- InvalidState is a little like a catch all for quiche getting into invalid state.
- It is currently emitted when quiche detects an inconsistency with internal state (path, cid, crypto and a few other scenarios).
- The common cases made it hard to tell if this was simply junk internet traffic or if there was a real issue.

#### T
- Investigate why there were so many errors.
- There is a lot of junk on the internet so this could be that.

#### A
- multiple iterations of adding logs and reasoning about the error path.
- initial changes to add logs resulted in too many logs so we reverted the
change and switched to internal debugging.
- shifted to deploying changes on 10 test staging metals to root cause.
- there were valid reasons for the error (receiving invalid packets from the
peer)
- isolated the pmtu code as culprit and created a minimal fix

#### R
- Saw a 80% reduction in errors.
- This translates to approx 1k connections succeeding that were previously failing.
- With reduced noise it now possible to tell if there are real errors due to
regression.
- quiche is used cross team so it also helped other teams/products.
