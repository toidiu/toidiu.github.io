+++
title = "Quic overview"
date = 2021-01-12


[taxonomies]
tag = ["crypto", "quic"]

[extra]
id = "blog-single"
+++

Understanding QUIC transport protocol

explanations: https://quicwg.org/ops-drafts/
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



### RFC Questions:
2.1
- successive streams of each type are created with numerically incresing stream IDs
Q: there are 4 types.. does that mean we can have a stream id 5 of type 1 and type 2? or does stream id incude the type bits also?

3.1
- the endpoint can safely ignore any MAX_STREAM_DATA frames it receives from its peer for a stream in this state (Data Sent state)
Q: so the sender can essentially ignore any data that is sent to it? since it is the sender it doenst expect any data.

3.2
- Before a stream is created, all streams of the same type with lower-numbered stream IDs MUST be created.  This ensures that the creation order for streams is consistent on both endpoints.
Q: does that mean we can end up creating many unused streams if a high value stream is received? why does creating order need to be consistent?

3.2
- It is possible that all stream data has already been received when a RESET_STREAM is received... possible for remaining stream data to arrive after receiving a RESET_STREAM frame. implementation is free to manage this situation as it chooses
Q: how do we handle this situation?

3.4
- whats the difference between remote and local (half-closed) composite state?

4.1
- MAX_DATA frame, which indicates the maximum of the sum of the absolute byte offsets of all the streams... receiver MUST close the connection with a FLOW_CONTROL_ERROR if the sender violates the limits.
so is the data calculated as stream id offset? does/can the sender ever maintain the limit? does a FLOW_CONTROL_ERROR mean that the connection is closed or the sender is then responsible for requesting a limit update via STREAM_DATA_BLOCKED/DATA_BLOCKED.
Q: does rertanmission of lost frames count towards the limit?

4.6
- A receiver MUST ignore any MAX_STREAMS frame that does not increase the stream limit
Q: why in multiple places do we only want to accept an increase and reject a decrease in limits? is there a security concern or is this a simpler model?

5.2.2
- large enough to initiate a new connection for any supported version... must drop smaller packets
Q: what defines a large/small packet

6.2
- A client MUST discard a Version Negotiation packet that lists the QUIC version selected by the client.
Q: does this indicate a condition that should not occur (server should simply have accepted it) and therefore an error state?

7.2
- Initial packet is sent by a client... client populates the Destination Connection ID field with an unpredictable value
Q: why do we need an unpredictable value here? if the packet protection keys are derived from this.. does this need to be cryptographicly secure?

7.3
- Initial vs Retry
Q: why would a server send a Retry packet?

7.4.1
- server MAY store and recover the previously sent values (cannot lower the values specified previously).
Q: does that mean we need to store all connection values/states.. or store 1 set by the timestamp? what happens if the client and server disagree on the value?

7.5
- Implementations MUST support buffering at least 4096 bytes of data received in out-of-order CRYPTO frames
Q: any insight into why was 4096 chosen as a value? why not half or double that value?

7.5
- Once the handshake completes, if an endpoint is unable to buffer all data in a CRYPTO frame, it MAY discard that CRYPTO frame and all CRYPTO frames received in the future
Q: so the handshake does not include a crypto handshake? how can we proceed with the connection if the crypto handshake has not completed.. are we in insecure land?

8.1.2
- A server can also use a Retry packet to defer the state and processing costs of connection establishment
Q: how does Retry packet defer the cost? wont we need to do this eentually and since we dont establish a connection this just seems to waste a packet.

8.1.3
A server SHOULD encode tokens provided with NEW_TOKEN frames and Retry packets differently, and validate the latter more strictly.
Q: - why  one more strictly over the other? is it because NEW_TOKEN can only be used with 0-RTT and therefore more restricted?

9.3.2
- If an endpoint skips validation of a peer address as described above, it does not need to limit its sending rate.
Q: why would peer address validation be skipped, this seems dangerous?

9.6.1
9.6.2
- preferred_address transport parameter in the TLS handshake
- client that migrates to a preferred address..
Q: so the server can only set the preferred address at the start of a connection (TLS). and only a client initiate a migation? does this mean that a server cannot update the preferred address and send all connections to a single address by mistake?

10.1.2
- applications with an option to defer an idle timeout
Q: is this something that has to be specified at the start of the connection or can it be configured adhoc?

10.2.3
- CONNECTION_CLOSE of type 0x1d in an Initial or Handshake packet could expose application state. CONNECTION_CLOSE of type 0x1d MUST be replaced by a CONNECTION_CLOSE of type 0x1c
Q: what state can be revealed and why is that bad?

11.2
- Application protocols SHOULD define rules for handling streams that are prematurely cancelled by either endpoint.
Q: thats super general.. what do we do here?

13
- Implementations are advised to include as few streams as necessary in outgoing packets without losing transmission efficiency to underfilled packets.
Q: since multiple stream in a single packet can result in all those streams being bocked.. do we do anything here?

14.2
- PMTU.. has fallen below the smallest allowed.. 1200 bytes.. immediatey cease sending QUIC packets.. endpoint MAY terminate the connection
Q: this sounds like a great way to kill all connections if PMTU falls (need to read up on PMTU and what can cause it to fall). is there a good recovery stradegy here?

17
this entire section seems very dense. can someone share alternative explanation for this.

17.3
- Fixed Bit/Resered Bit
Q: why do we have fixed bits and reserved bits? does this server a crypto function or error detection or future proofing?

17.3.1
- disable their use of the spin bit for a random selection of at least one in every 16 network paths, or for one in every 16 connection IDs
Q: why do we need to disable this bit randomly? crypto related? why 1 in 16?

19.3
- ACK Ranges identify acknowledged packets.
Q: what is an ACK range? can we specify and acknowledged a range of packets? from what i can tell its just a single packet number
An ACK range do allow for ACKing ranges of packets. it alternates from acknowledged to not acknowledged.

19.6
- CRYPTO frames... do not bear a stream identifier
Q: why not? doesnt each stream need to have separate keys?

19.8
- OFF bit. final size of the stream is the sum of te offset and the length of this frame.
Q: is this essentially an incremental sequence? why is the final size calculate like this?
A: offset refers to byte offset. so 0 means this is first byte of the stream and 80 means this is the 80th byte of the stream. so final size is 80 + len of this frame since we got 80 bytes before. BUT WHY SUM OF THE OFFSET?

19.10
- maximum amount of data that can be sent on the identified stream
Q: what happens if the client keeps sending the same packet over and over.. does this count towards the MAX_STREAM_DATA limit?

19.11
- MAX_STREAMS do not describe the number of streams that can be opened concurrently
Q: what is the mechanism to describe the concurrent limit?

19.15
- Retire Prior to: indicating which connection IDs should be retired
Q: is this just a terrible name since it i think in unit of time when reading this? what exactly does this field mean?

19.15
- NEW_CONNECTION_ID::Length; Values less than 1 and greater than 20 are invalid and MUST be treated as a connection error
Q: i remember reading that future versions of quic can change the length of the connection id. was i mistaken?

19.19
- CONNECTION_CLOSE frame cannot be split between packets, any limits on packet size will also limit the space available for a reason phrase.
Q: do we truncate the reason if its too long? i didnt see this in code, but possibly missed it (quic/s2n-quic-core/src/frame/connection_close.rs#L33)

19.21
- QUIC frames do not use a self-describing encoding.
Q: what does this mean? whats an example of self-describing encoding vs non-self-describing encoding? is it to define a new frame at runtime?

### General questions
Q: - what is a reset_stream frame? and when should one use it?
Q: - how did we decide to set MAX_ACTIVE_CONNECTION_ID_LIMIT to 3?
Q: - what exactly is the state that needs to be maintained and what is the "probe more paths". can this simply not be done via more concurrent streams
Q: - section 6.2.1 did not get highlighted.. bug?
Q: - whats the strategy for preventing connection id exhaustion? given 64bit length, 1 conn/sec => years = 583,386,176,284
Q: - do we impl 0-RTT? will we offer it in the future?
Q: - what exactly is the difference between frame and packet? is a packet a UDP datagram? do they provide overlapping functionality and if so when do you use one over the other (for example NEW_TOKEN vs Retry packet in 8.1.1)?
Q: - is a path a different physical network path? does the client konw that a new network path was created and thus initialte a path migration?
Q: - i foresee high latency if path migration is initiated too frequently. 'stuckness' can also happen if connection_ids are used up and abandoned too quickly.
Q: - whats the difference between path and connection migration?
Q: - since violations of the protocol cause immediate close of the connection, is there an attack vector here to be concerned about or is that simply necessary?
Q: - do packets and frames both have incrementing identifiers?
Q: - an attacker can send a spoofed packet with a higher id; this would result in the duplicate packet not being processed. can quic make progress in this state? (the client should issue a PROTOCOL_VIOLATION)
Q: - instead of sending an ACK for each packet, can the server send an ACK for ever 10 packets received and the client can infer that all previous packets were also received? there could be implications for congestion control and loss detection with this scheme.
rfc: every packet MUST be acknowledged at least once
Q: - how frequently to send ACKs seems like an entire subject. do we have anything to measure this? the rfc also mentioned asymmetric links and I wonder how that would impact the implementation
Q: - does the the congestion control (bbr) also live in application space? dont you need a common one between the peers? does/how does quic communicate with congestion control and provide it info?
Q: - how many other implementation out there of quic are written in rust? i counted (neqo, pico, quiche, quinn). have we thought about what will set s2n apart for the rest?
Q: - what does the crate zerocopy do?
A: https://doc.rust-lang.org/nomicon/other-reprs.html
trait: unaligned: struct is repr C - doesnt have strict alignment aligned means have to store it every usize
       asBytes:
       toBytes:
Q: - what does the crate byteorder do?
Q: - why would one want to use a zero length connection id?




start at 21.4.1
