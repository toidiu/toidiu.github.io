+++
title = "The role of ANS.1 in Cryptography"
date = 2024-05-30

[taxonomies]
tag = ["crypto"]

[extra]
id = "blog-single"
+++

ASN.1, pkcs1, pkcs8, pem, der, Pika Pika Chuuuu!

Ever feel like cryptography is just a
bunch of random acronyms that some just made up? I felt the same way and decided to change
that by figuring out how these things actually get used in real world cryptography.

<!-- more -->

Let us start with some example pseudo code:

```rust
// 1. generate cryptrographic key (ex. ecdhe)
let key = ecdhe_handshake();

// 2. generate a BIG_NUM
let big_num_key: BIG_NUM = BN_set_word(key);

// 3.
// asn1_obj = ASN.1: ()
//   - pkcs1: defines the RSA format
//   - pkcs8: defines the key format (RSA, EC, ..)
let asn1_obj = Pkcs8::new(private_key);

// 4: The der encoded ans1 object is simply the raw bytes
let der = asn1_obj.to_bytes();

// 5: The pem encoded asn1 object is the base64 encoding
// of the raw bytes
//
// pem = cat key.der | base64
let pem = der.to_base64();
```

In the snippet above we are attempting to encode a private key and encode it as `der` or
`pem` file. We can do something similar with other interesting objects as well such as
public key, certificates, etc.

1. `key`: secret key generate perhaps from a key exchange
2. `big_num_key`: generate a [bignum](https://docs.openssl.org/1.0.2/man3/bn/#synopsis) from the
   secret key
3.  create a `pkcs8` encoded ASN.1 object: `asn1_obj`
  - [ASN.1](https://en.wikipedia.org/wiki/ASN.1): a language for defining data structure.
    In cryptography, we care about a few encodings:
      - `pkcs1`: format for encoding and decoding RSA private and public keys
      - `pkcs8`: format for encoding cryptographic private keys, often containing pairs of private
        and public keys. Can encode multiple key types (RSA, ECDSA, etc) and should be preferred
        over pkcs1.
      - `X.509`: format for encoding digital certificates
4. `der` is just the raw bytes of the ans1 object
5. `pem` is the base64 representation of the der bytes

### What is ASN.1?
First, take a look at the ASN.1 object representation of the certificate of [this site](https://lapo.it/asn1js/#MIIDoTCCA0egAwIBAgIQXQaw3dOjYeUOAEXcx7NKfzAKBggqhkjOPQQDAjA7MQswCQYDVQQGEwJVUzEeMBwGA1UEChMVR29vZ2xlIFRydXN0IFNlcnZpY2VzMQwwCgYDVQQDEwNXRTEwHhcNMjQwOTIzMTUwMDI4WhcNMjQxMjIyMTUwMDI3WjAVMRMwEQYDVQQDEwp0b2lkaXUuY29tMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEk5N_4mwUFBbfq_DwqGxTtZDXQ-G_F9y9e5NYaXWS2HQsh6UwPMLrQRwlQ77OsWBW_z-HKtRwsY9QgrMVISYVlqOCAlEwggJNMA4GA1UdDwEB_wQEAwIHgDATBgNVHSUEDDAKBggrBgEFBQcDATAMBgNVHRMBAf8EAjAAMB0GA1UdDgQWBBSfYjReoEA85TK6jSVwxd2HjyfElzAfBgNVHSMEGDAWgBSQd5I1Z8T_qMyp5nvZgHl7zJP5ODBeBggrBgEFBQcBAQRSMFAwJwYIKwYBBQUHMAGGG2h0dHA6Ly9vLnBraS5nb29nL3Mvd2UxL1hRWTAlBggrBgEFBQcwAoYZaHR0cDovL2kucGtpLmdvb2cvd2UxLmNydDAjBgNVHREEHDAaggp0b2lkaXUuY29tggwqLnRvaWRpdS5jb20wEwYDVR0gBAwwCjAIBgZngQwBAgEwNgYDVR0fBC8wLTAroCmgJ4YlaHR0cDovL2MucGtpLmdvb2cvd2UxL1BDVWVRVmlRbFljLmNybDCCAQQGCisGAQQB1nkCBAIEgfUEgfIA8AB2AHb_iD8KtvuVUcJhzPWHujS0pM27KdxoQgqf5mdMWjp0AAABkh-c4_QAAAQDAEcwRQIhAJwMeBGNP9ofyl-PQ0AuL4qSkz9clmmZ175jDZYcNPSFAiBtZVemYmFbhFOch99Kq1EvAX4i_CroxuMRCJowzxitUgB2AEiw42vapkc0D-VqAvqdMOscUgHLVt0sgdm7v6s52IRzAAABkh-c5B4AAAQDAEcwRQIhAIh7iC-IxstglYu3qnIplFopHD6ixr3aAHyv5sZWTKEeAiAY_zjE9cj-pVehys2Sx0MZMnRVmmecrhNu0bDaA2P3HDAKBggqhkjOPQQDAgNIADBFAiEAvXutcWdEDhwh0yA6wxuYjWK-Z_ESF-apfTM8UZ340psCIAy2V8z3q5dPnyJ-hLfwQDh4yX5mD8yyTmwCirr12FwK)

For a detailed dive into ASN.1, I can highly recommend reading the post
[Warm Welcome to ASN.1 and DER](https://letsencrypt.org/docs/a-warm-welcome-to-asn1-and-der). In summary,
ASN.1 is an interface definition language
([IDL](https://en.wikipedia.org/wiki/Interface_description_language)) (it defines an interface to a
system). An IDL needs to be able to do 2 things:
1. defining data structures (ie. define structure of a certificate)
1. serialization and deserialization of those data structures


> The advantage of writing ASN.1 definitions instead of Go or C definitions is that they are
> language-independent.

While it is possible to represent a data structure in the language your application is written in
(eg. Rust), the ASN.1 representation is language agnostic and can be used across multiple languages
(ASN.1 parser implementations are available for multiple languages).  I like to use the analogy of
Java, which allows you to "write code once, run anywhere". ANS.1 allows your to "define object and
serialize in one application, deserialize and reconstruct in any other application".

```
// C representation
struct point {
  int x, y;
  char label[10];
};

// Go representation
type point struct {
  x, y int
  label string
}

// ASN.1 representation is language agnostic
Point ::= SEQUENCE {
  x INTEGER,
  y INTEGER,
  label UTF8String
}
```

> There are some other languages that do the same things as ASN.1. For instance, Protocol Buffers
> offer both a language for defining types and a serialization format for encoding objects of the
> types you’ve defined... ASN.1 (1984) had the significant advantage of already existing when
> certificates (1988) and HTTPS (1994) were invented.


ASN.1, while not perfect (a ASN.1 parser is complicated to implement), has become the
standard in cryptography.

### Resources:
- https://letsencrypt.org/docs/a-warm-welcome-to-asn1-and-der/
- http://luca.ntop.org/Teaching/Appunti/asn1.html
- https://lapo.it/asn1js/
- [Internet X.509 Public Key Infrastructure Certificate and Certificate Revocation List (CRL) Profile](https://datatracker.ietf.org/doc/html/rfc5280)
- [ASN.1 lang](https://www.itu.int/rec/T-REC-X.680)
- [ASN.1 serialization format](https://www.itu.int/rec/T-REC-X.690)
- [oid](http://oid-info.com/get/1.3.6.1.4.1.11129)
