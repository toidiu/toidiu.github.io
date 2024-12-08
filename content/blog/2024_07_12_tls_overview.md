+++
title = "TLS overview"
date = 2024-07-12

[taxonomies]
tag = ["crypto"]

[extra]
id = "blog-single"
+++

An brief overview of a TLS in simple language. https://www.rfc-editor.org/rfc/rfc8446

<!-- more -->

## Table of Contents
- [TLS guarantees](#tls-guarantees)
  - [Authentication](#tls-authentication)
- [Sub protocols](#sub-protocols)
  - [Handshake protocols](#handshake-protocol)
  - [Record protocols](#record-protocol)
- [Handshake types](#handshake-types)
  - [A Full TLS handshake](#full-handshake)
  - [A PSK TLS handshake](#psk-handshake)

## <a name="tls-guarantees">TLS guarantees</a>
The goal of the TLS protocol is to secure network communication. More formally it aim to
provide 3 security properties:
- Authentication: is the peer who they say they are.
- Confidentiality: messages between two peers are secret and only visible to the peers
- Integrity: messages sent to peers are tamper proof. A tampered message can be detected.

But how exactly does TLS enforce these properties?

### <a name="tls-authentication">Authentication</a>
In TLS authentication is satisfied by the use of certificates.
A The server sends its certificate during the handshake.

Certificate trust is complicated and typically involves parsing a certificate chain
(root-cert => intermediate-cert => peer-cert). Trust is established if the server trusts
one of the certificates in the chain. [RFC
5280](https://datatracker.ietf.org/doc/html/rfc5280) defines the modern day certificate
infrastructure known as x.509 PKI (Public Key Infrastructure). Modern web browsers come
installed with well-established root certificates (Cloudflare root, Mozilla root, etc)
which helps facilitate the internet.

The certificate contains the server's public key pair
and can be used to verify [signatures](https://en.wikipedia.org/wiki/Digital_signature)
created by the server. Signatures are used by the TLS handshake to ensure that we are
speaking with the "real" peer (toidiu.com) and not an attacker. This works since only the
real "toidiu.com" website server will have the private key associated with the public key
on the certificate.

- Confidentiality:
  - During the handshake, the client and server perform a key exchange (kex).
  - There are many kex algorithms (RSA, DHE, EDHE).
  - ECDHE (elliptic curve diffie-hellman ephemeral) is preferred because it is
    forward-secret. Note the word ephemeral in the title.
  - The responsibility of kex is to allow for each peer to contribute some
    public key material (`kem`) and together come up with a secret key for the
    TLS connection.
  - `client-public-kem + server-public-kem => connection-secret-key`.
  - The connection secret key is a strong cryptographic secret, which when used
    to encrypt messages provides confidentiality.
- Integrity:
  - Along with using the "connection secret key" for encryption, we also
    generate a MAC (message authentication code) for the ciphertext when
    encrypting data.
  - This tag can be though of as a "signature" on the ciphertext, and guarantees
    that only a peer which possesses the connection secret key could have
    generated the encryption/tag.
  - Previously the encryption and authentication were separate steps, but today
    it is good practice to use cipher schemes that offer Authenticated
    Encryption (authentication and encryption).

## <a name="sub-protocols">Sub protocols</a>
The TLS protocol is divided into two sub protocols: Handshake and Record.

### <a name="handshake-protocols">Handshake protocol</a>
The Handshake protocol is responsible for performing a key exchange with the
peer. The secret material generated from the key exchange is then used in the
Record protocol to encrypt data.
[https://tls13.xargs.org/](https://tls13.xargs.org/) provides an interactive
diagram of the TLS 1.3 handshake.

```txt
    Client                                               Server

Handshake protocol:
    ClientHello               -------->
                                                    ServerHello
                                          [generate secret key]
                              <--------              {Finished}

    [generate secret key]
    {Finished}                -------->
```

The TLS protocol is extensible via the use of extesions. The peers can negotiate
extensions in order to enable certain features. Some of these extensions are
optional, while others are required for normal operation. A few of them include:
- server_name: specify the name of the server (useful for load balancing)
- heartbeat: remember the Heartbleed bug?
- supported_groups: allow a client to enumerate the elliptic curves it supports
  and/or the point formats it can parse.
- "signature extensions": I specifically choose these because they are subtly
  different and caused me much confusion in the past [RFC
  section](https://www.rfc-editor.org/rfc/rfc8446#section-4.2.3)
  - signature_algorithms: applies to signatures in the CertificateVerify
    message. This signature is generated by one of the TLS peers (client of
    server) and used to authenticate the handshake was tamper proof. More on
    this below.
  - signature_algorithms_cert: allows a client to indicate which signature
    algorithms it can validate in X.509 certificates. This signature is
    generated by CAs (certificate authorities) and used to validate the
    certificate chain.

**Difference between `signature_algorithms` and `signature_algorithms_cert`**

- signature_algorithms:
  - Certificate message: As part of the handshake, a Server provides the Client
    with its certificate (its identitiy).
  - CertificateVerify message: This message is used to provide explicit proof
    that the Server possesses the private key corresponding to its certificate.
    This is done by using the private key to **sign some data**. The algorithm
    used to sign this data must be in the `signature_algorithms` extension.
- signature_algorithms_cert:
  - We discussed certificate chains above. A chain could mean that the
    certificate for this website (leaf) is **signed** by some intermediatary,
    which is **signed** by some root certificate. root->intermediate->leaf.
  - My web browser might not trust every leaf certificate (every single website
    on the internet), but might trust some small number of root certificates.
  - My browser is able to validate that chain by verifying that the root
    signature on the leaf certificate.
  - The algorithm allowed for these certificate signature must be in the
    `signature_algorithms_cert` extension list.


### <a name="record-protocols">Record protocol</a>

Once the Handshake protocol is complete, the peers now share the same secret key
which they can use to encrypt/decrypt message. At this point it is possible for
the peers to send encrypted data to each another.

```txt
     Client                                               Server

Handshake protocol:

     [generate secret key]                 [generate secret key]
                                  ....

Record protocol:
     [Application Data]        <------->      [Application Data]

```

## <a name="handshake-types">Handshake Types</a>
A connection might perform a full handshake (perform a ECDHE key exchange). Or
it might have done so previously and already posesses a PSK (Pre-Shared Key),
which is can use for sending encrypted data.

### <a name="full-handshake">A full TLS handshake</a>

```txt
     Client                                               Server

Initial Handshake:
     ClientHello
     + key_share               -------->
                                                     ServerHello
                                                     + key_share
                                           {EncryptedExtensions}
                                           {CertificateRequest*}
                                                  {Certificate*}
                                            {CertificateVerify*}
                                                      {Finished}
                               <--------     [Application Data*]

     {Certificate*}
     {CertificateVerify*}
     {Finished}                -------->

                               <--------      [NewSessionTicket]

     [Application Data]        <------->      [Application Data]
```

### <a name="psk-handshake">A PSK TLS handshake</a>

```txt
Subsequent Handshake:
       ClientHello
       + key_share*
       + pre_shared_key          -------->

                                                       ServerHello
                                                  + pre_shared_key
                                                      + key_share*
                                             {EncryptedExtensions}
                                                        {Finished}
                                 <--------     [Application Data*]

       {Finished}                -------->

       [Application Data]        <------->      [Application Data]
```
