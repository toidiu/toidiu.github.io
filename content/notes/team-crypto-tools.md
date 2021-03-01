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

## ramp up on crypto
originally provided by SalusaSecondus

- [Dan Boneh's Cryptography I](https://www.coursera.org/learn/crypto)
- [Cryptopals Crypto Challenges](https://cryptopals.com)
- [Introduction to Cryptography](https://www.youtube.com/channel/UC1usFRN4LCMcfIV7UjHNuQg/videos) by Christof Paar (I haven't personally verified this one.)
- Read the source of the libraries you use.
- Find people working in spaces closer to professional cryptography than you and ask them to help you (buy them beers or beverages of choice)
- Look at public issues on GitHub for libraries you use and see if you can contribute, or at least understand them
- Read _tons_ of specifications. You use AES-GCM? Read [NIST SP 800-38D](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38d.pdf) You use HMAC? Read [RFC #2104](https://tools.ietf.org/html/rfc2104)
- Read [The Stick-Figure Guide to AES](http://www.moserware.com/2009/09/stick-figure-guide-to-advanced.html)
- Read the [Latacora Blog](https://latacora.singles) (especially the "Right Answers")
- Read my [Cryptographic Gotchas](https://github.com/SalusaSecondus/CryptoGotchas) list.
- Read [If You’re Typing the Letters A-E-S Into Your Code You’re Doing It Wrong(link is broken)](https://www.nccgroup.trust/us/about-us/newsroom-and-events/blog/2009/july/if-youre-typing-the-letters-a-e-s-into-your-code-youre-doing-it-wrong/). [archived link](https://web.archive.org/web/20150924090725/https://www.nccgroup.trust/us/about-us/newsroom-and-events/blog/2009/july/if-youre-typing-the-letters-a-e-s-into-your-code-youre-doing-it-wrong/) [alternate link](https://people.eecs.berkeley.edu/~daw/teaching/cs261-f12/misc/if.html) by the NCC group
- Remember (and try to follow) any company/person mentioned in this list.
- [How To Learn Cryptography as a Programmer](https://soatok.blog/2020/06/10/how-to-learn-cryptography-as-a-programmer)
- KNOW YOUR LIMITS - Let me re-emphasize that last one. As everyone tells you, doing crypto is hard and dangerous. If you are confident that you're doing things right with no problems, you can't be trusted. You need to treat cryptography with the caution and respect it deserves. I can think of at least one coworker whose cryptography I don't trust because he is over-confident in his own abilities. (And his skills are better than mine. Still, I'm more trusted because I'm more cautious.)


## ramp up on crypto-tools team


### short term ramp up
D - quic rfc
D - meeting invites - standup, com office hr, oncall, sprint, sprint demo
D - https://www.youtube.com/watch?v=B1SQFjIXJtc&feature=emb_title
- quic - varint (16)
- quic - packet decoding (17)
- endian
- coursera

### long term ramp up
- cryptopals
- read bbr
- tls rfc
- get involved in quic community
- https://github.com/quicwg/base-drafts
- https://mitls.org/pages/attacks/SMACK

### long term projects
- long term projects
- unit testing
- de-macro stuff
- more usecase testing (send vs receive, short vs large payload, congestion, attacker)
- read up on kernel bypass

### concepts
- diffie hellman
- rsa
- public vs private key

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

