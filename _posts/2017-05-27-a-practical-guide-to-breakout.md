---
title: "A practical guide to breakOut"
categories:
  - Code
  - Scala collection
tags:
  - Scala
---

In this post I will be reviewing `scala.collection.breakOut` from an application developer's perspective rather than a functional programming enthusiast. I will discuss why one would want to use it and then a simple rule on how to recognize when it is applicable. The full code sample can be found at github.

When I first saw `breakOut`, it seemed like an extremely intimidating concept and a little like black magic. It certainly didn't help that I found explanations of `breakOut` intermingled with the equally intimidating concept of `CanBuildFrom`. This is a shame because the proper use of `breakOut` can make your code run faster. Let's consider the following code sample:


//IGNORE============
The Scala collection library is a deceptively complicated piece of machinery that is hidden below the a very nice interface. I consider this essential for productivity but it can sometimes result in the following code:

```scala
val list = List("one", "two", "three", "four")

val expensiveMap: Map[Int, String] = list
  .map(n => n.length -> n)     // 1
  .toMap                    // 2
```

The problem with this code is that it is memory inefficient. We first create an intermediate list at line (1) and then convert it to a Map on line (2). Ideally we could do it all in one step. Well thanks to `breakOut` we can!

```scala
import scala.collection.breakOut
val list = List("one", "two", "three", "four")

val efficientMap: Map[Int, String] = list
  .map(n => (n.length, n))(breakOut)
```

This results in the `Map` we want without the intermediate list.

So when should we use `breakOut`? An easy way to remember when to use it is if we have the following list of conditions:

- we have a traversable collection (List, Seq, Vector)
- we want to transform the elements (pairing the string length with the string)
- we want the final output to be a `Map`



