+++
title = "TLS 1.3"
date = 2021-01-09

[taxonomies]
tag = ["crypto", "rfc"]

[extra]
id = blog-single
+++

Understanding TLS 1.3

The primary goal of TLS is to provide a secure channel given a reliable, in-order data stream.

<!-- more -->

Find the annotated rfc at: [paper](...)

## 1 Intro
The secure channel provides:
- Authentication (server side is always and client side is optionally authenticate)
- Confidentiality (data after establishment is only visible to the endpoints)
- Integrity (data after establishment cannoy be modified without detection).

TLS consists of 2 primary components:
- handshake protocol: auths the peers, negotiates crypto modes and establishes shared keys. It is designed to resist tampering.
- record protocol: uses keys established in handshake to protect the traffic between the peers. Divides the traffic into series of records.

TLS is application protocol independent and higher-level protocols can layer on top of TLS.

### 1.2 Differences from TLS 1.2
- Static RSA and DHE have been removed. All public-key based exchanges now provide forward secrecy.

## 2 Protocol overview
TLS support 3 basic key exchange modes:
- (EC)DHE (Diffie-Helman over either finite fields or elliptic curves)
- PSK-only
- PSK with (EC)DHE

The handshake can be thought of having 3 phases:
- Key exchange: establish shared key and select crypto params. Everything after this phase is encrypted.
- Server parameters: establish other handshake params (applicaiton layer support, if the client is authenticated, etc)
- Authentiction: authenticate the server (optionally client) and key confirmation and handshake integrity.




