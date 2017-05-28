---
title: "A practical guide to breakOut"
categories:
  - Code
  - collection library
tags:
  - Scala
---

In this post I will be reviewing `scala.collection.breakOut` from an application developer's perspective rather than a functional programming enthusiast. I will discuss why one would want to use it and how to recognize when it is applicable. The full code sample can be found at github.

When I first saw `breakOut`, it seemed like an extremely intimidating concept and a little like black magic. It certainly didn't help that I found explanations of `breakOut` intermingled with the equally intimidating concept of `CanBuildFrom`. This is a shame because, with proper application, `breakOut` can make your code run faster. Let's consider the following code sample:

#### Why we care

```scala
val list = List("one", "two", "three", "four")

val expensiveMap = list      //type Map[Int, String] is infered by the compiler
  .map(n => n.length -> n)     //1
  .toMap                    //2
```

The problem with this code is that it is memory inefficient. We first create an intermediate list at line (1) and then convert it to a Map on line (2). Ideally we could do it all in one step. Well thanks to `breakOut` we can!

```scala
import scala.collection.breakOut
val list = List("one", "two", "three", "four")

val efficientMap: Map[Int, String] = list   //you need to specify the type
  .map(n => (n.length, n))(breakOut)
```

This results in the `Map` we want without the intermediate list. Win!

#### When to use

So when should we use `breakOut`? An easy way to remember when to use it is if we have the following list of conditions:

- we have a traversable collection (List, Seq, Vector)
- we want to transform the elements (pairing the string length with the string)
- we want the final output to be a `Map`


#### Conclusion
Scala has a robust immutalbe collections library, which has a tendency to make it more GC heavy. However, with some attention to the implementation details, it is possible to remain efficient and functional.

As always, thanks for reading. If you find any mistakes please feel free to contact me at apoorv[aat]toidiu[dott]com
