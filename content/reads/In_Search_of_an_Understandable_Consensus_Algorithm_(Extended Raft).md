+++
title = "In Search of an Understandable Consensus Algorithm (Extended Raft)"
date = 2019-09-17

[extra]
paper = "In_Search_of_an_Understandable_Consensus_Algorithm_(Extended_Raft).pdf"
short = "Raft is a concensus protocol that aims to improve upon Paxos by being more understandable. It claims to be as efficient as Paxos. The paper outlines an implementation in great detail. To quote: 'The greatest difference between Raft and Paxos is Raft’s strong leadership: Raft uses leader election as an essential part of the consensus protocol, and it concentrates as much functionality as possible in the leader.' By basing actions on a 'consistent leader', subsequent actions and state space is simplified"
+++

Note: It is not possible to summarize the details of the Raft implementation so I will instead cover some high-level points/decision/tradeoffs by by the authors(take a look at the paper which highlights important content). Also sections (6)Log Compaction and (7)Cluster membership changes are currently left out.

It should be noted that there is distinction between the entry `log` and `state machine`. The log is a mechanism used to make progress while an update to the state machine(an index pointing to the log) precludes that a entry and all previous entries have been `committed`(a decision was made).

Raft seperates the protocol into 3 different modes (leader election, log replication and safety). It first elects a leader(choose a new one if current one fails) and gives it complete control for managing the replicated log(accept entries from client, replicating entries and committing entries). The safety property guarantees that if an entry is committed to the state machine, all members of the cluster agreed upon the value.

A member of the cluster can only be in one of 3 states: leader, follower, candidate. A `term` is used as a logical clock in Raft(clock skew makes this a good choice). There are two RPC in Raft (RequestVote and AppendEntries) and they are idempotent.

Leader Election: A heartbeat mechanism is used to track the liveliness of the leader. There is only ever 1 leader (term is used to decide the which leader is valid). A failure to elect a leader(split vote, packet loss) will cause members to timout (150-300ms) and issue a RequestVote of their own and thus starting a new election.

Log Replication: A leader is the only one that can modify the logs. It services client request and sends AppendEntries RPC in parallel to members. Once an entry has been safely replicated, it will answer the client. At the start of leadership, the leader will force all members to match its logs and therefore delete log entries from followers if necessary. The log on the leader is only ever append only.

Safety: A restriction for voting for a leader is that the RequestVote RPC includes log information and the voter denies the vote if its own log is more up-to-date than that of the candidate. This ensures that the winning candidate must contain all committed entries since a majority vote is needed.

A caveat: clients send all interactions to the leader(a follower will reject and provide information about the current leader). Therefore the leader is responsible for handling all reads and writes which could cause hot-spot on a single server. Additionally, even a read-only operation requires the leader to verify it has the latest information and thus contact a majority of the members.

A caveat: Paxos is a paper and implementations vary out in the field. Egalitarian Paxos (EPaxos) can achieve higher performance under some conditions with a leaderless approach; however it is more complex.

