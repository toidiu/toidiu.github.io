+++
title = "Fastly blacklist to whitelist migration"
date = 2019-08-11

[extra]
company = "iHeartRadio"
lp = ["highest standard", "earn trust"]
+++

highest standard
  rather than fix the symptom at hand fix the core issue
earn trust
  outages mean lower oncall morale
  rolling out the changes in % meant gaining the trust of the team

customer obsession
  also bad for customers
reduce risk
  rolling in % meant detecting issues early

#### S
Realized that our recs were being incorrectly cached. The reason for this was
the historic configuration of specifying the blacklisted paths.

#### T
The inital fix for the recs service was an easy config change. However to avoid
such a mistake in the future I proposed that we should change from blacklist to
whitelist.

#### A
Initially this seems like a very risky manuver, expecially when changing this for
live production traffic. Therefore I took a few precautions to eliminate the risk.

I decided the a % based rollout, and then progressed to do it over 3 weeks.

#### R
At each 10% increase I gained additional confidence, about the new configs. Watching
the cache hit ratio (89%), the traffic pattern also remained the same. Additionally,
by communicating with the rest of my team as well as being aware of other company
events, I was able to aviod being suspect of any failures by mistake.

The result was a clean migration, a more resilient system and probably a higher
morale since there were no outages.

