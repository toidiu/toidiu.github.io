+++
title = "TLS overview"
date = 2024-07-12

[taxonomies]
tag = ["crypto"]

[extra]
id = "blog-single"
+++

An brief overview of a TLS in simple language.

<!-- more -->

### The TLS protocol provides 3 properties
- Authentication: is the peer who they say they are. the client authenticates the server. server can optionally authenticate the client.
- Confidentiality: messages sent to the peer are secret and only visible to the peer.
- Integrity: messages sent to peer are tamper proof. If a message is modified then this tamper can be detected.

### How are these properties enforced
- Authentication: during the handshake, the server sends a certificate which ties its private key to the public key on the certificate. The certificate is in turn signed by a root CA's certificate, which both the peers need to trust to establich "a chain of trust". The root cert might sign a intermediate cert, which in turn will sign the peer's cert. The "root cert" -> "intermediate cert" -> "peer cert", is the chain and the trust is established by cryptographically verifying the signatures on the cert. The [RFC](https://datatracker.ietf.org/doc/html/rfc5280) defines x.509 and PKI, which are the components which define certificate infrastructure.
- Confidentiality: during the handshake, the client and server perform a key exchange (kex). There are many kex algorithms (RSA, DHE, EDHE). ECDHE (elliptic curve diffie-hellman ephemeral) is prefered because it is forward-secret. Note the word epehemeral in the title. The responsibility of kex is to allow for each peer to contribute some public key material and together come up with a secret key for the TLS connection. "client public key material" + "server public key material" => "connection secret key". The connection secret key is a strong cryptographic secret, which when used to encrpty messages provides confidentiality.
- Integrity: along with using the "connection secret key" for encryption, we also generate a MAC (message authentication code) for the ciphertext we generate. This tag can be though of as a "signature" on the ciphertext, and guarantees that only a peer which possesses the connection secret key could have generated the encryption/tag. Previously the encryption and authentication were separate steps and algorithms, but today it is good practice to use cipher schemes that offer Authenticated Encryption (authenticaion and encryption).

### A full TLS handshake
todo

### A PSK TLS handshake
todo
