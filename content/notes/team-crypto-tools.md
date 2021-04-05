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


### short term ramp up
D - quic rfc
D - meeting invites - standup, com office hr, oncall, sprint, sprint demo
D - https://www.youtube.com/watch?v=B1SQFjIXJtc&feature=emb_title
D - read tls 1.3
- read tls 1.2
- read s2n-tls code
- start with api to level yourself with customer
  - set of api is called 'async'
  - io
  - send vector and receive vector

::::why read tls code::::
- very well written code
- tls will stay with you for entire career

::::divide your day/week into three parts::::
coding
meetings
code reviews/tls learning

## short term read list
- tls walkthu: https://tls13.ulfheim.net/
- quic performance: http://cs.brown.edu/~tab/papers/QUIC_WWW21.pdf
D - intro to crypto: https://web.cs.ship.edu/~cdgira/courses/CSC434/Fall2004/docs/course_docs/IntroToCrypto.pdf

-------------------

## long term read list
- intrusive list: https://www.data-structures-in-practice.com/intrusive-linked-lists/
- siphash: http://cr.yp.to/siphash/siphash-20120918.pdf
- min perfect hash: https://blog.gopheracademy.com/advent-2017/mphf/
- rust ref: https://doc.rust-lang.org/reference/introduction.html
- read up on kernel bypass
- https://mitls.org/pages/attacks/SMACK
- https://github.com/SalusaSecondus/CryptoGotchas/blob/master/README.md
- https://www.cs.auckland.ac.nz/~pgut001/pubs/pkitutorial.pdf
- coursera

### long term ramp up
- cryptopals
- read bbr
- tls rfc
- https://github.com/quicwg/base-drafts
- constance time: https://www.bearssl.org/constanttime.html

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
