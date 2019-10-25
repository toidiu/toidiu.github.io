+++
title = "Building the Real-Time Big Data Database: Seven Design Principles behind Scylla"
date = 2019-01-10

[extra]
paper = "Scylla_Seven_Design_Principles.pdf"
short = "This paper is structured more like a whitepaper than a research paper and gives really nice insight into engineering a high performant DB. Scylla is a drop-in replacement for Cassandra and promises to provide better performance. It claims to do so by leveraging modern harware features and automation. The paper explores 7 key design decisions that helped guide Scylla's development."
+++

1) Using C++ instead of Java meant avoiding GC latencies in JVM, while gaining precise control over memory and being able to access [advance kernal features](https://lwn.net/Articles/743714/).

2) Scylla chose to be compatible with Cassandra drivers, query language and ecosystem and was therefore able to market to the existing vast Cassandra community.

3) Async architecture was used for all I/O and CPU operations in Scylla. This not only avoided traditional concurrency problems and also capped the performance to system resources rather than the application framework. See [Seastar](https://github.com/scylladb/seastar) for more info on the async engine used in Scylla.

4) A shared-nothing (one shard/core)architecture was used to avoid using locks. This lets the application developer avoid context, cache invalidation, and locks. Instead shared memory queues are used to communicated between shards. So like Cassandra, the entire dataset of the cluster is sharded to nodes(machines), but additionally, the data on the node is sharded per core.

5) Cassandra uses a key cache and row cache in addition to the general purpose Linux page cache. This adds complixity with unpredictable performance. Scylla chose to implement a singe unified cache which can tune itself depending on the current workload. It also uses things like direct memory access(DMA) to access disk when there is a cache miss. DMA operates independent of CPU.

6) Scylla utilizes I/O schedulers to self tune and balance between foreground and background I/O operations. The tool `scylla_ io_setup` is used to ensure maximum useful disk concurrency while maintaining good latency and not overloading drives.

7) The self-tuning capabilities allow Scylla to make informed decisions without manual intervention or needing to hire an expert. This design is used in compaction, caches and many other parts of the system.

