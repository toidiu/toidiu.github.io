+++
title = "Cryptography Course"
date = 2021-01-12


[taxonomies]
tag = ["crypto"]

[extra]
id = "blog-single"
+++

Coursera course https://www.coursera.org/learn/crypto/lecture/lboqg/course-overview
<!-- more -->


# week 1:
- intro
- discrete prob
- stream cipher 1
- stream cipher 2
- stream cipher 3
- stream cipher 4

## section 1: intro
### vid 1:
- handshake protocol: secret key establishment
  public-key crypto
- record protocol: secure the communication
  ensure confidentiality and integrity

- symetric encryption
E and D algotrithms, secret key k, m message and c ciphertext

### vid 2:
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

### vid 3:
history of cryptography..

## section 2: discrete prob
### vid 1:
discrete probability

probability:
probability distribution: a func that defines a probability over the domains space. the sum of all the prob is equal to 1
uniform distribution: all elements in domain space is equal
point distribution: a point gets a prob of 1 and all others get a probability of 0

event:
given an event A, the probailty of A is the sum of all occurance of the set A

union bound:
what the probability that either A or B occurs
P(A) U P(B) <= P(A) + P(B)

uniform random variable:
a variable x from the set U that has uniform probability

random algorithms:
F(m, r) where r is a uniform random variable
the output of a random algorithm is random and changes based on r
so given a message m, the output of the function will be different because of the uniform random variable

### vid 2:
descrete probility

independence: event A and event B are independent if P[A and B] = P[A] * P[B]
  - A happening doesnt tell you anything about B
  - same can be said about random variables P[rA and rB] = P[rA] * P[rB]

'XOR': either or but not both
x y  xor
0 0  0
0 1  1
1 0  1
1 1  0

Theorem: Y a rand. var. over {0,1}^n and X an indep. unifor. var. on {0,1}^n
         Z := Y xor X is a uniform variable

XOR has the property: that we can make a rand variable uniform prob by XORing with a uniform independent variable.


birthday paradox: probability that a collision will be found
          given r1..rn in U. indep. identically distributed vars.
          when you take n samples prob that two are equal is 1/2
          n = 1.2 |U|^1/2 then P[ri = rj] >= 1/2

## section 3: stream cipher 1
### vid 1:
symmetric cipher is defined over a triple (K, M, C) (keys, message, ciphertext)
  a pair of efficient algorithms (E, D) encryption and decryption

D(K, E(E, m)) = m  => decryption needs to be inverse of encryption
E is often a randomized algo (generate random bits) but D is deterministic

One time pad OTP: first secure cipher
M = C = K = {0,1}^n
C := E(K,m) = K xor m (ciphertext is xor of key and message)
the key needs to be as long as the message so its not very practical

why is OTP secure?
Information Theory - Claude Shannon
CT should reveal no 'info' about PT (ciphertext and plaintext)

a cipher (E,D) over (K,M,C) has **perfect secrecy** if
given m0 and m1 in M and c in C
P[ E(k,m0) = c ] = P[ E(k,m1) = c ]
  prob of the two ciphertext being equal
  given CT cant tell if msg is m0 or m1 or
  given CT i cant tell what the original msg was
  an advisary learns nothing about PT from CT
  **NO CT attack is possible**

given CT it is not possible to tell PT because the probility for all possible PT is the same (xor makes this easy to see)

perfect secrecy is an ideal goal, becuase is says that |K| >= |M|, length of the keys must be equal or more than messages
again for OTP the keys and message are the same length, each message has one key

### vid 2:


--------------------------

# week 2:
## section 1: ...
### vid 1:
