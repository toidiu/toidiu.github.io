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

- `ReceiveStreamState`
https://github.com/awslabs/s2n-quic/blob/4826027e85fef49ba87d9505494317dbe4e9d47e/quic/s2n-quic-transport/src/stream/receive_stream.rs#L27
/// Enumerates the possible states of the receiving side of a stream.



### Questions:
2.1
- successive streams of each type are created with numerically incresing stream IDs
there are 4 types.. does that mean we can have a stream id 5 of type 1 and type 2? or does stream id incude the type bits also?

3.1
- the endpoint can safely ignore any MAX_STREAM_DATA frames it receives from its peer for a stream in this state (Data Sent state)
so the sender can essentially ignore any data that is sent to it? since it is the sender it doenst expect any data.

3.2
- Before a stream is created, all streams of the same type with lower-numbered stream IDs MUST be created.  This ensures that the creation order for streams is consistent on both endpoints.
does that mean we can end up creating many unused streams if a high value stream is received? why does creating order need to be consistent?

3.2
- It is possible that all stream data has already been received when a RESET_STREAM is received... possible for remaining stream data to arrive after receiving a RESET_STREAM frame. implementation is free to manage this situation as it chooses
how do we handle this situation?

3.4
- whats the difference between remote and local (half-closed) composite state?

4.1
- MAX_DATA frame, which indicates the maximum of the sum of the absolute byte offsets of all the streams... receiver MUST close the connection with a FLOW_CONTROL_ERROR if the sender violates the limits.
so is the data calculated as stream id offset? does/can the sender ever maintain the limit? does a FLOW_CONTROL_ERROR mean that the connection is closed or the sender is then responsible for requesting a limit update via STREAM_DATA_BLOCKED/DATA_BLOCKED.
does rertanmission of lost frames count towards the limit?

4.6
- A receiver MUST ignore any MAX_STREAMS frame that does not increase the stream limit
why in multiple places do we only want to accept an increase and reject a decrease in limits? is there a security concern or is this a simpler model?

5.2.2
- large enough to initiate a new connection for any supported version... must drop smaller packets
what defines a large/small packet

### General questions
- what is a reset_stream frame? and when should one use it?
- how did we decide to set MAX_ACTIVE_CONNECTION_ID_LIMIT to 3?
  - what exactly is the state that needs to be maintained and what is the "probe more paths". can this simply not be done via more concurrent streams


