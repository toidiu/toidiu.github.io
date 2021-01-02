+++
title = "HMAC: Keyed-Hashing for Message Authentication (rfc 2104)"
date = 2021-01-01

[extra]
paper = "HMAC_Keyed-Hashing_for_Message_Authentication_rfc2104.pdf"
short = "MAC, message authentication codes, is a mechanism to check the integrity of information transmitted over unreliable medium. HMAC is an implementation of MAC using a cryptographic hash function. SHA-3 is probably a good algorithm to use. HMAC is designed to allows for inter-chaning hash functions seamlessly, not incur performance penalty on top of the hash function, have simple key usage and have cryptographic strength based on the strength of the hash function."
+++

original rfc: https://www.ietf.org/rfc/rfc2104.txt

**introduction**

The paper quotes MD5 and SHA-1 (SHA-3 should probably be used) as viable cryptographic hash functions. Along with a cryptographic algorithm, HMAC also requires a shared secret key.

HMAC has the following goals:
- use existing cryptographic function, which can be interchanged
- does not incur performance degradation on top of the cryptographic function
- uses and handles keys in a simple way
- cryptographic strength of the auth mechanism is based on the strength of the underlying hash function

**definition of HMAC**

H = hash function

K = secred key (it is recommended that the length of K be at minimum L)

B = length of input block

L = length of output block

ipad = the byte 0x36 repeated B times

opad = the byte 0x5C repeated B times

HMAC is calculated as = H(K XOR opad, H(K XOR ipad, text))

- 1: append zeros to the end of K to create a B byte string (e.g., if K is of length 20 bytes and B=64, then K will be appended with 44 zero bytes 0x00)
- 2: XOR (bitwise exclusive-OR) the B byte string computed in step (1) with ipad
- 3: append the stream of data ’text’ to the B byte string resulting from step (2)
- 4: apply H to the stream generated in step (3)
- 5: XOR (bitwise exclusive-OR) the B byte string computed in step (1) with opad
- 6: append the H result from step (4) to the B byte string resulting from step (5)
- 7: apply H to the stream generated in step (6) and output the result

**keys**

I is advices that the length of K be atleast L otherwise security is decreased. Keys need to be chosen at random using a cryptographically strong pseudo-random generator and periodically refreshed.

**truncated output**

Truncation of the output MAC is a known practice since the entire length is not need to verify that the hash was computed. This can save on transmission bandwidth I imagine but not sure how useful it is. The recommendation is that the truncation not be less than half the length of the output and not less than 80 bits (might be more in modern days).

**security**
- the construction of HMAC is independent of the details of a particular hash function
- message authentication, as opposed to encryption, has a "transient" effect. so that breaking a cryptographic function today will require us to change the function to prevent future MACs. However, past messages remain verified the present break doesn't affect the past verification.

