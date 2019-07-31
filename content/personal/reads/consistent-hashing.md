+++
title = "Consistent Hashing and Random Trees: Distributed Caching Protocols for Relieving Hot Spots on the World Wide Web"
date = 2018-12-24

[extra]
paper = "consistent_hashing_and_random_trees_distributed_caching_protocols_for_relieving_hot_spots_on_the_world_wide_web_technical_publication.pdf"
short = "In distributed systems there exists a problem of how to spread load across many servers in a scalable manner. Consistent hashing is a hashing design that allows for distribution with minimal coordination between the machines."
code = "https://github.com/toidiu/consistent-rs"
+++

A first solution might be to hash the requests across the different servers. In terms of a caching, this means to take item (`i`), hash it (good distribution) and then assign it to the available caches (`C`), maybe with modulo: `hash\_fn(i) % C`. There are two problems with this approach. First is that the distribution is a function of the servers, so as we add or remove servers, we now need to recalculate the modulo and therefore the distribution. The second problem, which derives from the first, is that now each server node now needs to be updated - said differently, each server node now needs to be aware of every node. Consensus is hard, so this is not ideal.

Section 4 contained proof and discussion of Consistent Hashing.

The authors define 4 properties that define their notion of consistency: **balance** requires that the hash function distribute the objects in a balanced fashion amongst the buckets. **monotonicity** says if items are initially assigned to a set of buckets and then some new buckets are added, then an item may move from an old bucket to a new bucket, but not from one old bucket to another. **spread** implies that references for a given object are only directed to a small number of caching machines. **load** implies that no one cache is assigned an unreasonable number of objects.objects.

Next they define some properties of a good ranged hash function: Given `Rb`(hash for buckets) and `Ri`(hash for items). An item `i` should be assigned to the closest bucket. Given max of `C` buckets. For each bucket create `k*log(C)`, some constant `k` virtual buckets and map them using the hash function `Rb`. To save space, have `Rb` and `Ri` map to the range `[0,1]`. To differentiate a point from another, we only needs to have `log(number of points)` random bits (decimal precision) identifying a point.

Lastly they share an efficient implementation of a possible consistent hash function: Use a balanced binary search tree to store the buckets and their assignment in the range. If there are `C` buckets, then there will be `k*C*log(C)` entries, which gives a worse tree depth of `log(C)` and a worse possible calculation of `log(C)`. The time for adding and removing a bucket is `log^2(C)` since we would need to remove `k*log(C)` points. To reduce this time to constant lookup, the authors suggest dividing the range into `k*C*log(C)` equal length segments and have a separate tree for each one. This allows us to more predictably have one bucket per segment. The problem with segments is that it requires smaller segments as more buckets are added. An amortized way recommended by the authors is to choose intervals of size `1/2^x` such that `1/2^x < 1/k*C*log(C)`. Then as new buckets are added, gradually bisect each section.

