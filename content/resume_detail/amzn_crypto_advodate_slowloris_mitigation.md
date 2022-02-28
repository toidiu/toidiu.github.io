+++
title = "s2n-quic advocate better slowloris mitigation"
date = 2021-11-01

[extra]
company = "amzn_crypto"
lp = []
+++

summary

#### metrics
- bla

#### S
The previous implementation involved closing the connection by default if the bytes transferred
dropped below some user specified amount.

#### T
While simple and effective implementation, this seems like an sharp edge and made me uneasy.
The reason for this was:
- the user specified value could become stale
- the default action closing the connection could be an availability risk
- in a worse case scenario this could lead to a stampede of closing all connections

#### A

#### R

