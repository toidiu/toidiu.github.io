+++
title = "Quic TLS overview"
date = 2021-03-01


[taxonomies]
tag = ["crypto", "quic"]

[extra]
id = "blog-single"
+++

Understanding QUIC TLS

rfc: https://tools.ietf.org/html/draft-ietf-quic-tls-32

### RFC Questions:
4.1.4 As keys at a given encryption level become available to TLS, TLS indicates to QUIC that reading or writing keys at that encryption level are available.

Q: seems like QUIC does crypto exchange and key generation multiple times?? this seems not efficient but important for security maybe??

---






### General questions
Q: - what is an encryption level? there are 4 encryption levels.. does this mean that each level has its own keys and that we generate 4 set of keys? because we only want 1-RTT max for encryption

---

Q: its not clear how levels and keys are generate and stored at each level. for that matter why are levels necessary?

---




