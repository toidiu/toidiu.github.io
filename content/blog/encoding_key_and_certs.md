+++
title = "Making plans"
date = 2024-05-25

[taxonomies]
tag = ["focus"]

[extra]
id = "blog-single"
+++

The difference between ASN.1, pkcs1, pkcs8, pem, der.

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

Finally, the raw bytes of the asn1_obj is the der representation, while the base64 representation is the pem format.
