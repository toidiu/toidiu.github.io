+++
title = "s2n-quic default TLS 1.3 support"
date = 2022-05-01

[extra]
company = "aws_crypto"
lp = []
+++

#### metrics
- 4000 tests
- ~1500 scoped down tls 1.2 tests
- ~33 "default" policy tests

#### S
- include tls 1.3 support by default
- good modern default. tls 1.3 is provably secure, modern ciphers,


#### T
- which can be risky because customers will see change in behavior
- it would also affect test coverage for tls 1.2
  - tests that **test the tls 1.2 policy**
    - can pass when switched to tls 1.3 because written previous to tls 1.3
      support and might not assert protocol
  - tests that **test the "default" policy**

It is not possible to detect the difference between these.

#### A
- assess different options
- balance the risk of security vs not making progress
- lay out the options, make a case for it


#### R
- manually audit 4000 tests
- scope bad tls 1.2 to certain files
- pin all tests that are using default to a numbered policy
  - test regression for **"default" policy tests**
- don't pin tests
  - test regression for **tls 1.2 tests**
- run all tests twice (different platform, libcrypto, fuzz, valgrind)
- run a single tls 1.2 test and accept a minimal risk of tls 1.2 test regression
