+++
title = "Quic overview"
date = 2021-01-12


[taxonomies]
tag = ["crypto", "quic"]

[extra]
id = blog-single
+++

Understanding QUIC transport protocol

summary: https://quicwg.org/ops-drafts/draft-ietf-quic-manageability.html#name-using-the-spin-bit-for-pass
rfc: https://tools.ietf.org/html/draft-ietf-quic-transport-33


### large sections:
- `StreamReceiveBuffer`
https://github.com/awslabs/s2n-quic/blob/4826027e85fef49ba87d9505494317dbe4e9d47e/quic/s2n-quic-transport/src/buffer/receive_buffer.rs#L61
//= https://tools.ietf.org/id/draft-ietf-quic-transport-32.txt#2.2
//# Endpoints MUST be able to deliver stream data to an application as an
//# ordered byte-stream.

- `SendStreamState`
https://github.com/awslabs/s2n-quic/blob/4826027e85fef49ba87d9505494317dbe4e9d47e/quic/s2n-quic-transport/src/stream/send_stream.rs#L30
/// Enumerates the possible states of the sending side of a stream.




### Questions:
2.1
Q: successive streams of each type are created with numerically incresing stream IDs
- there are 4 types.. does that mean we can have a stream id 5 of type 1 and type 2? or does stream id incude the type bits also?


