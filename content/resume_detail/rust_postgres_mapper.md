+++
title = "PostgresMapper - increasing code realibility"
date = 2018-10-11

[extra]
company = "rust"
lp = ["learn and be curious", "think big"]
+++

learn and be curious: learn how to do proc macros. read other source code

#### metrics
- added 2 methods
- selectively replaced old code with new features
- fixed broken functionality with tokio-postgres
- reduced code update locations from 3 to 1
  - hit 0 runtime errors because of mismatch fields thereafter
- technique allowed for slow adoption rather than breaking code

#### S
The crate was a side project written by someone to provide some simple deserialization capabilites for postgres in rust. It addressed a important need but failed to do any field name checks at compile time.

#### T
Add additional methods to reduce runtime errors.

#### A
Added simple methods that read annotations and provided two methogs `get_sql_table` and `get_fields`. this was something that the compiler could do really well and not error.

#### R
As a result, the runtime errors due to table migrations went down to zero.
