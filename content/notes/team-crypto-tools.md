+++
title = "Learning Cryptography"
date = 2020-12-26


[taxonomies]
tag = ["crypto"]

[extra]
id = blog-single
+++

Ramp up on cryptography and crypto tools team
<!-- more -->

-----------------

## tls1.3
skipped:
- 4.2.11

## qlog main
- 4

### short term ramp up
D - quic rfc
D - meeting invites - standup, com office hr, oncall, sprint, sprint demo
D - https://www.youtube.com/watch?v=B1SQFjIXJtc&feature=emb_title
D - read tls 1.3
- consume quic and write server impl
- read s2n-tls code
- start with api to level yourself with customer
  - set of api is called 'async'
  - io
  - send vector and receive vector
- read up on kernel bypass
- https://www.2uo.de/myths-about-urandom/

::::why read tls code::::
- very well written code
- tls will stay with you for entire career

::::divide your day/week into three parts::::
coding
meetings
code reviews/tls learning

## short term read list
- read tls 1.2
- quic performance: http://cs.brown.edu/~tab/papers/QUIC_WWW21.pdf
D - intro to crypto: https://web.cs.ship.edu/~cdgira/courses/CSC434/Fall2004/docs/course_docs/IntroToCrypto.pdf

-------------------

## long term read list
- coursera
- https://inst.eecs.berkeley.edu/~cs161/fa08/papers/stack_smashing.pdf
- https://mitls.org/pages/attacks/SMACK
- https://eprint.iacr.org/2010/264
- intrusive list: https://www.data-structures-in-practice.com/intrusive-linked-lists/
- siphash: http://cr.yp.to/siphash/siphash-20120918.pdf
- min perfect hash: https://blog.gopheracademy.com/advent-2017/mphf/
- rust ref: https://doc.rust-lang.org/reference/introduction.html
- https://github.com/SalusaSecondus/CryptoGotchas/blob/master/README.md
- https://www.cs.auckland.ac.nz/~pgut001/pubs/pkitutorial.pdf

### long term ramp up
- cryptopals
- read bbr
- tls rfc
- https://github.com/quicwg/base-drafts
- constance time: https://www.bearssl.org/constanttime.html

-----------------

### tools
- tls walkthu: https://tls13.ulfheim.net/

### long term projects
- unit testing
- de-macro stuff
- more usecase testing (send vs receive, short vs large payload, congestion, attacker)

-----------------

## other
### major attacks
- shellshock
- heartbleed
- spectre
- meltdown

### resources
#### book
- http://cacr.uwaterloo.ca/hac/
#### lecture notes
- http://cseweb.ucsd.edu/~mihir/papers/gb.pdf
- ship.edu: https://web.cs.ship.edu/~cdgira/courses/CSC434/Fall2004/docs/course_docs/

-----------------

## ciphersuite
A ciphersuite is a combination of a few algorithms that allow you to do variour operations.

- `TLS13_AES_256_GCM_SHA384`
AES is the encryption algo, GCM is the counter mode + mac algorithm, SHA is the KDF

- `TLS13_CHACHA20_POLY1305_SHA256`
CHACHA20 is a stream cipher, POLY1305 is the mac algo, SHA is the KDF

AES_(MAC-COUNTER):
The mac-counter allows you to transform the block cipher (AES) into a stream cipher. This is useful so that you can encrypt unknown length of data.
- GCM: counter + gmac
- CCM: counter + cmac

KDF is used for 4 things:
- entropy extraction - hash it
- expansion - generate more than one key (generating the same key is secure, a bigger key is not secure. 256 -> 256 is ok. 256 -> 512 is not)
- ratcheting - one way part of the key generation
- strengthening IKM - slow execution to make it difficult to run. checking password pbkdf2, bcrypt, scrypt






