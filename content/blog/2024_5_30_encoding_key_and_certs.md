+++
title = "Encoding keys and certs"
date = 2024-05-30

[taxonomies]
tag = ["crypto"]

[extra]
id = "blog-single"
+++

How do the following relate: ASN.1, pkcs1, pkcs8, pem, der.

<!-- more -->

```
BIG_NUM big_num = BN_set_word(key_bytes_num)

asn1_obj = ASN.1: (language that defines data structures)
  pkcs1: defines the RSA format
  pkcs8: defines the key format (RSA, EC, ..)

der = asn1_obj.to_bytes()
pem = der.to_base64() // contains labels
```

Assuming we want to encode a private key, we start out with a secret which is possibly a large integer: `key_bytes_num`

We can define that using the [ASN.1](https://en.wikipedia.org/wiki/ASN.1), which is an interface description language (IDL).

The key format, used for ASN.1 encoding can either be a pkcs1 or pkcs8.

Finally, the raw bytes of the asn1_obj is the `der` representation, while the base64 representation is the `pem` format.

pem = `cat key.der | base64`

### Warm Welcome to ASN.1 and DER
> There are some other languages that do the same things as ASN.1. For instance, Protocol Buffers offer both a language for defining types and a serialization format for encoding objects of the types you’ve defined.

#### String
PrintableString is a restricted subset of ASCII, allowing alphanumerics, spaces, and a specific handful of punctuation: ' () + , - . / : = ?. Notably it doesn’t include * or @.

### Resources:
- https://letsencrypt.org/docs/a-warm-welcome-to-asn1-and-der/
- http://luca.ntop.org/Teaching/Appunti/asn1.html
- https://lapo.it/asn1js/
- [Internet X.509 Public Key Infrastructure Certificate and Certificate Revocation List (CRL) Profile](https://datatracker.ietf.org/doc/html/rfc5280)
- [ASN.1 lang](https://www.itu.int/rec/T-REC-X.680)
- [ASN.1 serialization format](https://www.itu.int/rec/T-REC-X.690)
- [oid](http://oid-info.com/get/1.3.6.1.4.1.11129)
