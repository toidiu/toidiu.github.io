+++
title = "Encoding keys and certs"
date = 2024-05-30

[taxonomies]
tag = ["crypto"]

[extra]
id = "blog-single"
+++

Exploring the confusing concepts of ASN.1, pkcs1, pkcs8, pem, and der and how they
relate to cryptography.

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

// 5: The pem encoded asn1 object is the base64 encoding of the raw bytes
//
// pem = cat key.der | base64
let pem = der.to_base64();
```

In the snippet above we are attempting to encode a private key.
1. `key`: secret key generate perhaps from a key exchange
2. `big_num_key`: generate a bignum from the secret key
3.  create a `pkcs8` encoded ASN.1 object: `asn1_obj`
  - [ASN.1](https://en.wikipedia.org/wiki/ASN.1): language for defining data structure (interface description language, IDL).
    In cryptography, we care about a few encodings:
      - `pkcs1`: format for encoding and decoding RSA private and public keys
      - `pkcs8`: format for encoding cryptographic private keys, often containing pairs of private
        and public keys. Can encode multiple key types (RSA, ECDSA, etc) and should be prefered
        over pkcs1.
      - `X.509`: format for encoding digital certificates
4. Represent the asn1_obj as either der or pem:
    1. `der`: the raw bytes of the ans1 object
    1. `pem`: base64 representation of the der ans1 object

### What is ASN.1?
But what is ASN.1 in the example above? Please read the wonderful article
[Warm Welcome to ASN.1 and DER](https://letsencrypt.org/docs/a-warm-welcome-to-asn1-and-der),
but in summary:

> An interface description language or interface definition language (IDL) is a generic term for
> a language that lets a program or object written in one language communicate with another program
> written in an unknown language. IDLs are usually used to describe data types and interfaces in a
> language-independent way, for example, between those written in C++ and those written in Java.

An [IDL](https://en.wikipedia.org/wiki/Interface_description_language) is essentially a
a generic data serialization format. Think a language agnostic way to represent some data encoding.

> There are some other languages that do the same things as ASN.1.
> For instance, Protocol Buffers offer both a language for defining types and a serialization
> format for encoding objects of the types you’ve defined. Thrift also has both a language and
> a serialization format. Either Protocol Buffers or Thrift could have just as easily been used
> to define the format for HTTPS certificates, but ASN.1 (1984) had the significant advantage of
> already existing when certificates (1988) and HTTPS (1994) were invented.


As as example lets represent a coordinate type in C vs ANS.1:

```
// C
struct point {
  int x, y;
  char label[10];
};

// ANS.1
Point ::= SEQUENCE {
  x INTEGER,
  y INTEGER,
  label UTF8String
}
```

> The advantage of writing ASN.1 definitions instead of Go or C definitions is that they are
> language-independent.


Sample ASN.1 object for the [amazon.com certificate](https://lapo.it/asn1js/#MIIIITCCBwmgAwIBAgIQCsTTq_eL3icrK2ufOHpz7jANBgkqhkiG9w0BAQsFADBEMQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMR4wHAYDVQQDExVEaWdpQ2VydCBHbG9iYWwgQ0EgRzIwHhcNMjQwOTEzMDAwMDAwWhcNMjUwODIzMjM1OTU5WjAZMRcwFQYDVQQDEw53d3cuYW1hem9uLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBALEkdnzcYIhElp0ec-rd9H3KQzT6qtN7WTctiY0EI7kMqYU7fLPn0PQ644L4_zlgYWzhyAPNLYAfe3MrN2cLPD-KQlGRahoZJhmfvjMzS0F5pLtJcQlx8evcya65SUrklaoMiYDX0StubbjkbeHB2AOBeTPpKftwfpqQAC1iLoimSuA3JzTpOpYQKUrrGNv6fSJ38_LugomLQ7q36hAA_cBynj4rlSLUObyFWJN4XCS8fNQjsZjlV1OWpLAsuJTbqciooc10-cX95dePitm0g7IhBlLFfxbmJEhv6OAcS-qtsL2nWChbnRDxfcsNNmZdHPh3H-YanoufYsieGc88IPECAwEAAaOCBTgwggU0MB8GA1UdIwQYMBaAFCRuKy3QapJRUSVpAaqaR6aJ50AgMB0GA1UdDgQWBBRvPH8sbFhy0FTv_dFmUjYp5TPP1jCCAgEGA1UdEQSCAfgwggH0ggphbWF6b24uY29tgghhbXpuLmNvbYIRdWVkYXRhLmFtYXpvbi5jb22CDXVzLmFtYXpvbi5jb22CDnd3dy5hbWF6b24uY29tggx3d3cuYW16bi5jb22CFGNvcnBvcmF0ZS5hbWF6b24uY29tghFidXlib3guYW1hem9uLmNvbYIRaXBob25lLmFtYXpvbi5jb22CDXlwLmFtYXpvbi5jb22CD2hvbWUuYW1hem9uLmNvbYIVb3JpZ2luLXd3dy5hbWF6b24uY29tgiFidWNrZXllLXJldGFpbC13ZWJzaXRlLmFtYXpvbi5jb22CEmh1ZGRsZXMuYW1hem9uLmNvbYIlcC1udC13d3ctYW1hem9uLWNvbS1rYWxpYXMuYW1hem9uLmNvbYIlcC15by13d3ctYW1hem9uLWNvbS1rYWxpYXMuYW1hem9uLmNvbYIlcC15My13d3ctYW1hem9uLWNvbS1rYWxpYXMuYW1hem9uLmNvbYIWeWVsbG93cGFnZXMuYW1hem9uLmNvbYIQd3d3Lm0uYW1hem9uLmNvbYISd3d3LmNkbi5hbWF6b24uY29tghN0ZXN0LXd3dy5hbWF6b24uY29tghJtcDNyZWNzLmFtYXpvbi5jb22CFmtvbnJhZC10ZXN0LmFtYXpvbi5jb20wPgYDVR0gBDcwNTAzBgZngQwBAgEwKTAnBggrBgEFBQcCARYbaHR0cDovL3d3dy5kaWdpY2VydC5jb20vQ1BTMA4GA1UdDwEB_wQEAwIFoDAdBgNVHSUEFjAUBggrBgEFBQcDAQYIKwYBBQUHAwIwdwYDVR0fBHAwbjA1oDOgMYYvaHR0cDovL2NybDMuZGlnaWNlcnQuY29tL0RpZ2lDZXJ0R2xvYmFsQ0FHMi5jcmwwNaAzoDGGL2h0dHA6Ly9jcmw0LmRpZ2ljZXJ0LmNvbS9EaWdpQ2VydEdsb2JhbENBRzIuY3JsMHQGCCsGAQUFBwEBBGgwZjAkBggrBgEFBQcwAYYYaHR0cDovL29jc3AuZGlnaWNlcnQuY29tMD4GCCsGAQUFBzAChjJodHRwOi8vY2FjZXJ0cy5kaWdpY2VydC5jb20vRGlnaUNlcnRHbG9iYWxDQUcyLmNydDAMBgNVHRMBAf8EAjAAMIIBfwYKKwYBBAHWeQIEAgSCAW8EggFrAWkAdgAS8U40vVNyTIQGGcOPP3oT-Oe1YoeInG0wBYTr5YYmOgAAAZHpF8flAAAEAwBHMEUCIEFQqK7SZS8h6Xu2NYpin9h9soCw2U8x674OdVvTbsaLAiEAoGOTz-SQay1r-1TCLltyXQ1pyBvsuTLOM_INma2H3WYAdgB9WR4S4XgqexxhZ3xe_fjQh1wUoE6VnrkDL9kOjC55uAAAAZHpF8f-AAAEAwBHMEUCIAldwHI3rxAlQYjJ9ysLXAGQDjfsvJRWVHBf2mj5jWBCAiEAmi12ds2GPAIgK6lc0lTzhYMMkqMApJ9IT9YydI6BS88AdwDm0jFjQHeMwRBBBtdxuc7B0kD2loSG-7qHMh39HjeOUAAAAZHpF8gfAAAEAwBIMEYCIQDAsOmKNz5Hv5Rnzeq7Bs-3Zvyz1w-8mkZOxAceg0hGKQIhAMSXvJvaTkqNxni-mrEyRMnt7sNyGRV7ca2c8XlYwPsgMA0GCSqGSIb3DQEBCwUAA4IBAQC-NuDOnNf-fyE8lq33E5EABylKdK7RNnYyIdm8Wc4mw--X-ieFsd9kyE9GOn9x3wPeq0BZy0Qt9j12CZUmhIA9bh85Dx8zMzcDopqR4ptxAJjRLU9Ue98evnkACoXIBiBxM0Mq1JF90ivlNgCy6Ba4RioWKaKpR2-RHbadzTtwLrgki3qPyNRazztqHomwMwfhiW4pTKIT7K4OoiPmsvklk5QaMBiZMu85fQgg0qCaf8bh0YRm9Arf1pxrFe8ygetCCHi0UrA6izBC8TKDsDY5LLfJdSlFpPB5eIMebrmcekFFMQ1WODodvREdkRiQQ0M8oEBv1S4TjFieityqNOGQ)

### Resources:
- https://letsencrypt.org/docs/a-warm-welcome-to-asn1-and-der/
- http://luca.ntop.org/Teaching/Appunti/asn1.html
- https://lapo.it/asn1js/
- [Internet X.509 Public Key Infrastructure Certificate and Certificate Revocation List (CRL) Profile](https://datatracker.ietf.org/doc/html/rfc5280)
- [ASN.1 lang](https://www.itu.int/rec/T-REC-X.680)
- [ASN.1 serialization format](https://www.itu.int/rec/T-REC-X.690)
- [oid](http://oid-info.com/get/1.3.6.1.4.1.11129)
