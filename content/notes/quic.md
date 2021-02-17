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

6.2
- A client MUST discard a Version Negotiation packet that lists the QUIC version selected by the client.
does this indicate a condition that should not occur (server should simply have accepted it) and therefore an error state?

7.2
- Initial packet is sent by a client... client populates the Destination Connection ID field with an unpredictable value
why do we need an unpredictable value here? if the packet protection keys are derived from this.. does this need to be cryptographicly secure?

7.3
- Initial vs Retry
why would a server send a Retry packet?

7.4.1
- server MAY store and recover the previously sent values (cannot lower the values specified previously).
does that mean we need to store all connection values/states.. or store 1 set by the timestamp? what happens if the client and server disagree on the value?

7.5
- Implementations MUST support buffering at least 4096 bytes of data received in out-of-order CRYPTO frames
any insight into why was 4096 chosen as a value? why not half or double that value?

7.5
- Once the handshake completes, if an endpoint is unable to buffer all data in a CRYPTO frame, it MAY discard that CRYPTO frame and all CRYPTO frames received in the future
so the handshake does not include a crypto handshake? how can we proceed with the connection if the crypto handshake has not completed.. are we in insecure land?

8.1.2
- A server can also use a Retry packet to defer the state and processing costs of connection establishment
how does Retry packet defer the cost? wont we need to do this eentually and since we dont establish a connection this just seems to waste a packet.

### General questions
- what is a reset_stream frame? and when should one use it?
- how did we decide to set MAX_ACTIVE_CONNECTION_ID_LIMIT to 3?
  - what exactly is the state that needs to be maintained and what is the "probe more paths". can this simply not be done via more concurrent streams
- section 6.2.1 did not get highlighted.. bug?
- whats the strategy for preventing connection id exhaustion? given 64bit length, 1 conn/sec => years = 583,386,176,284
- do we impl 0-RTT? will we offer it in the future?
- what exactly is the difference between frame and packet? is a packet a UDP datagram? do they provide overlapping functionality and if so when do you use one over the other (for example NEW_TOKEN vs Retry packet in 8.1.1)?

