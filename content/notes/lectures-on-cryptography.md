+++
title = "Lecture Notes on Cryptography by Shafi Goldwasser and Mihir Bellare"
date = 2020-12-26


[taxonomies]
tag = ["crypto"]

[extra]
id = blog-single
+++

Lectures notes on cryptography. See pdf for highlighted section
<!-- more -->


## preparation
- write bench criteron for fin
- experiment with bolero fuzzing
- articulate ownership
- articulate guarantees of rust
- review network communications
  - TCP/IP Protocol Architecture Model: https://docs.oracle.com/cd/E19683-01/806-4075/ipov-10/index.html
- write some unsafe
- Sized
  - https://stackoverflow.com/questions/30938499/why-is-the-sized-bound-necessary-in-this-trait


## video lectures by Christof Paar https://www.youtube.com/channel/UC1usFRN4LCMcfIV7UjHNuQg/videos
- MAC and HMAC https://www.youtube.com/watch?v=DiLPn_ldAAQ
- SHA3 https://www.youtube.com/watch?v=JWskjzgiIa4

how to encrypt data over the wire
- transport layer
- mac vs ip vs applicaiton level

open ssl
s2n libary
open source

## 3 projects that the team works on
- s2n
    open source
- quic impl
    q1 first release - re-invent full protocol and 3 customers
- salty
    s3 internal tool. encryption in transit between their nodes
    certificate/identity
    rust
    identiy and certifiate
- sassy (maintenance mode)
    goal is to deprecate in end of 2021


## concepts
hashing
hmac
constant time attack
private-key encryption
public-key encryption
