+++
title = "PostgresMapper - increasing code realibility"
date = 2019-08-11

[extra]
company = "rust"
lp = ["learn and be curious", "think big"]
+++

learn and be curious
  learn how to do proc macros. read other source code

#### S
The crate was a side project written by someone to provide some simple deserialization capabilites for postgres in rust. It addressed a important need but failed to do any field name checks at compile time.

#### T
Add additional methods to reduce runtime errors.

#### A
Added simple methods that read annotations and provided two methogs `get_sql_table` and `get_fields`. this was something that the compiler could do really well and not error.

#### R
As a result, the runtime errors due to table migrations went down to zero.

