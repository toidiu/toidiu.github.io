+++
title = "Cryptography 1"
date = 2023-01-11

[taxonomies]
tag = ["cryptography"]

[extra]
id = "blog-single"
+++

In this post I'll be recaping the [coursera course](https://www.coursera.org/learn/crypto) on Crpytography 1.

<!-- more -->

## Week1: Stream ciphers

### intro
**vid 1:**
- handshake protocol: secret key establishment
  public-key crypto
- record protocol: secure the communication
  ensure confidentiality and integrity

- symetric encryption
E and D algotrithms, secret key k, m message and c ciphertext

**vid 2:**
- digital signatures: hash the content plus the signature
- anonymous communication: mix net
- digital cash: if money is spent once it remains anonymous, however if its spent twice the identity is revealed
- secure multi-party communication: they communicate amongst each other in an encrpyted way so that at the end only the result is known
  `theorem`: anything you can do with a trusted authoriy, can also be done without!!
  - election: (voter 1) -> (voter 2)
  - private auction: reveal only the winner and the second highest bid
zero knowledge:
  (Alice) N = p * q     -> Alice can prove that she knows factors of N, with sharing p or q ->       only knows N (Bob)

rigorous science:
- specify a threat model (how the attacker will attack and what his goals are)
- propose a construction (construction of the crypto)
- prove that breaking connstruction under threat model will solve an underlying hard problem (its hard to break encrpytion)

**vid 3:**
history of cryptography..

### discrete prob
**vid 1:**
discrete probability

probability:
probability distribution: a func that defines a probability over the domains space. the sum of all the prob is equal to 1
uniform distribution: all elements in domain space is equal
point distribution: a point gets a prob of 1 and all others get a probability of 0

event:
given an event A, the probailty of A is the sum of all occurance of the set A



## Week 2: Block Ciphers

## Week 3: Message Integrity

## Week 4: Authenticated Encryption

## Week 5: Key Exchange

## Week 6: Public-Key Encryption




