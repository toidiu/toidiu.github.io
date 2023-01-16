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

cryptography is a rigorous science:
- specify a threat model (how the attacker will attack and what his goals are)
- propose a construction (construction of the crypto)
- prove that breaking connstruction under threat model will solve an underlying hard problem (prove that its hard to break encrpytion)

### discrete prob
**vid 1:**
- probability distribution: a func that defines a probability over the domains space. the sum of all the prob is equal to 1
- uniform distribution: all elements in domain space is equal
- point distribution: a point gets a prob of 1 and all others get a probability of 0
- event: given an event A, the probailty of A is the sum of all occurance of the set A
- union bound: what the probability that either A or B occurs. P(A) U P(B) <= P(A) + P(B)
- uniform random variable: a variable x from the set U that has uniform probability

- random algorithms:
F(m, r) where r is a uniform random variable
the output of a random algorithm is random and changes based on r
so given a message m, the output of the function will be different because of the uniform random variable

independence: event A and event B are independent if P[A and B] = P[A] * P[B]
  - A happening doesnt tell you anything about B
  - same can be said about random variables P[rA and rB] = P[rA] * P[rB]

'XOR': either or but not both
x y  xor
0 0  0
0 1  1
1 0  1
1 1  0

Theorem: Y a random variable over {0,1}^n
         X an independent uniform variable on {0,1}^n
         Z := Y xor X is a uniform variable

XOR has the property: that we can make a rand variable uniform prob by XORing with a uniform independent variable.


birthday paradox: probability that a collision will be found
          given r1..rn in U. independent identically distributed variables
          when you take n samples prob that two are equal is 1/2
          n = 1.2 |U|^1/2 then P[ri = rj] >= 1/2

### stream cipher 1
**vid 1:**
symmetric cipher is defined over a triple (K, M, C) (keys, message, ciphertext)
  a pair of efficient algorithms (E, D) encryption and decryption

D(K, E(E, m)) = m  => decryption needs to be inverse of encryption
E is often a randomized algo (generate random bits) but D is deterministic

**OTP** One time pad OTP: first secure cipher
M = C = K = {0,1}^n
C := E(K,m) = K xor m (ciphertext is xor of key and message)
the key needs to be as long as the message so its not very practical

why is OTP secure?
Information Theory - Claude Shannon
ciphertext should reveal no 'info' about plaintext (ciphertext and plaintext)

a cipher (E,D) over (K,M,C) has **perfect secrecy** if
given m0 and m1 in M and c in C
P[ E(k,m0) = c ] = P[ E(k,m1) = c ]
  prob of the two ciphertext being equal
  given ciphertext cant tell if msg is m0 or m1 or
  given ciphertext i cant tell what the original msg was
  an advisary learns nothing about plaintext from ciphertext
  **NO ciphertext attack is possible**

given ciphertext it is not possible to tell plaintext because the probility for all possible plaintext is the same (xor makes this easy to see)

perfect secrecy is an ideal goal, becuase is says that |K| >= |M|, length of the keys must be equal or more than messages
again for OTP the keys and message are the same length, each message has one key

### PRG

We define a statistical test for the generator G of the random distribution.

We then define an advantage as how likely the generator will output 1 compared to truly random generator.
Adv (PRG) : [A, G] : Pr[ A(G(k)) = 1 ] - Pr[ A(r) ]

**Unpredictable**
An unpredictable PRG is a secure PRG.
If the 'next-bit' predictor cannot distinguish G from random then no statistical test can.

**Computationally indistinguishable**: notation to compare two distributions
| Pr[ A(P1((x))) = 1 ] - Pr[ A(P2((x))) = 1 ] | < negligible


### Semantic Security

The attacker should learn no information about the plaintext.

**Semantic Security for one time key:**

    Experiment 1(Exp1) and Experiment 2(Exp2)
    b= 0,1

```
[Challenger]    <- m0 or m1     [Adversary]
                E(k, mb) ->        v
                                guess m0 or m1
```

    W(b) = event that Exp(b) = 1
    Advantage(SS) = | Pr[ W(0) ] - Pr[ W(1) ] |


## Week 2: Block Ciphers

## Week 3: Message Integrity

## Week 4: Authenticated Encryption

## Week 5: Key Exchange

## Week 6: Public-Key Encryption




