---
title: "Polymorphism in Scala"
nav: blog
id: blog
categories:
  - Code
  - Scala
tags:
  - polymorphism
---

I have found that I learn much better when I have to write about it and explain it to others. This is an exercise for my understanding but hopefully it will also help others. The accompanying code for this post can be found on [github](https://github.com/toidiu/ScalaPolymorphism).

Polymorphism is a word that means 'having  different forms'. In terms of OOP, this mean that a class can have many different forms and behave in different manners depending on the context.

Scala has 3 types of Polymorphism that we will explore further below.

* Subtype Polymorphism
* Parametric Polymorphism
* Ad-hoc Polymorphism

> As an extra bit of info, the compiler evaluates subtype at Runtime
> while parametric and ad-hoc are evaluated at compile time.


### Subtype Polymorphism

In Java we have subtype polymorphism and this is the one that most people learn about. This is when we define a super/parent class or interface(traits in Scala) and then extend this behavior in a child class. In the example below, we can think of Taco as a _sub-type_ of the type Food.

```scala
//Define a trait that other child classes can extend. We could also have declared Food as an abstract class.
trait Food {
  final val isEdible = true
  def name: String
  def minToPrep: Int
}

class Taco extends Food {
  val name: String = "taco"
  def minToPrep: Int = 6
}

class Cereal extends Food {
  val name: String = "cereal"
  def minToPrep: Int = 1
}

val t = new Taco
val c = new Cereal

t.isEdible //true
c.isEdible //true
t.minToPrep //6
c.minToPrep //1
```

As seen from the example above, both `Taco` and `Cereal` are child classes that extend the behavior of trait `Food`.

```scala
def whatsForLunch(f: Food) {
  println(f.name)
}

whatsForLunch(new Taco) //prints: taco
```

We can now substitute instances of Taco for a value that requires Food. A way to think about this is that by extending the class Food, Taco essentially guarantees that it will fulfill a contract and define the values isEdible and minToPrep; and therefore makes a perfectly acceptable substitute for Food.

In this case we define the value for isEdible as true for all foods. However, the name and minToPrep value is customized based on the different types of food.

As a note, sub-type polymorphism is determined at Runtime. The type of the class is looked up at Runtime and then the appropriate method is called. For more information about this read the Wikipedia article on [Dynamic Dispatch](https://en.wikipedia.org/wiki/Dynamic_dispatch).


### Parametric Polymorphism

In Java the name for parametric polymorphism is 'Generics'. The idea is to create data structures and write code that can be generic or type agnostic. A good example to demonstrate this is List. Below we construct a List for holding Int which **doesn't** use parametric polymorphism.

```scala
trait List

object Nil extends List {
  val isEmpty = true
}

class Cons(val head: Int, val tail: List) extends List {
  val isEmpty = false
}

val list = new Cons(2, new Cons(1, Nil)) //list {2->1->Nil}
println(list.head)  //prints: 2
```

The above code is fine if we only want to create a list of Int but we quickly run into problems when we want to create a list of String, Long, Taco!! We now have to re-write the same code and create multiple copies for each data type.

  > This violates the DRY(Don't Repeat Yourself) principle of good 
  > programming. To understand the reason behind DRY, just consider
  > what happens if we want to now add a method to append lists 
  > together. We would have to add the same code to the implementation 
  > for each type of List.

Instead we could re-write List data structure using parametric polymorphism, in a more generic manner(pun intended :P).

```scala
trait List[+A]

object Nil extends List[Nothing]

class Cons[A](val head: A, val tail: List[A]) extends List[A]


val intList = new Cons(2 , new Cons(1 , Nil)) //list of Int
val foodList: List[Food] = new Cons(new Taco, new Cons(new Cereal, Nil)) //list of food
```

  > The + sign on the definition of the List declares List structure
  > as covariant. This means that if we declare a list of Food 
  > `val list = List[Food]` we can add Taco and Cereal to the list.

The advantages are obvious... we can define the List data structure once and utilize it with any type of object!


### Ad-hoc Polymorphism

Let me admit, this one was confusing for me. Therefore I will first explain the concept in plain english to get a general understanding. I will then demonstrate the concept with code and relate it back to the english definition.

Wikipedia defines [ad-hoc](https://en.wikipedia.org/wiki/Ad_hoc_polymorphism) polymorphism as follows:

  > Ad hoc polymorphism is a kind of polymorphism in which 
  > polymorphic **[1]functions can be applied to arguments of** 
  > **different types**, because a polymorphic function can 
  > **[2]denote a number of distinct and potentially heterogeneous** 
  > **implementations** **[3]depending on the type of argument(s)** 
  > **to which it is applied**. It is also known as function 
  > overloading or operator overloading... This is in **[4]contrast**
  > **to parametric polymorphism, in which polymorphic functions** 
  > **are written without mention of any specific type**, and can 
  > thus apply a single abstract implementation to any number of 
  > types in a transparent way.

1. An ad-hoc polymorphic function can take different types of arguments. ex. Int or String.

2. The implementation of the ad-hoc polymorphic function function can vary.

3. Combining points 1 and 2, the implementation of the function will vary depending on the type of the argument passed to the function.

    **Thats really it! Another way to think about this is that we are doing function overloading and the function does different things based on the type we pass it.**

4. The statement helps us to differentiate ad-hoc from parametric polymorphism. Remember from before, in parametric polymorphism the implementation of the function(List constructor) was the same regardless of the type passed in.


#### Some code:

We first define a Texture trait and a function that prints the Texture. **Note: `printTexture` is our ad-hoc polymorphic function.** printTexture takes a type parameter `t: T` and a texture implementation of that type `o: Texture[T]`. 

**Ahh ha.. from bullet (1), the function takes different types as arguments.**

  > Ignore the `implicit` keyword for now. I will how this is later used
  > to automatically pull in the correct implementation of Texture.

```scala
trait Texture[T] {
  def getTexture(t: T): String
}

def printTexture[T](t: T)(implicit o: Texture[T]): Unit = {
  println(o.getTexture(t))
}
```

Next we need to define the different implementation depending on the the class type. Note how in the Texture object we define `TacoTexture` and `WetTacoTexture` which define the texture for the class Taco that we defined above.

We also define a new class called `Silk` which can also have a texture and give it a texture implementation `SilkTexture`.

**Ahh ha.. from bullet (2), we have varying definitions based on the types.**

  > Again notice but ignore the `implicit` keyword. We will make the 
  > connection later.

```scala
//A new class for which we will define texture
class Silk

object Texture {
  implicit object TacoTexture extends Texture[Taco] {
    override def getTexture(t: Taco) = "crunchy"
  }

  object WetTacoTexture extends Texture[Taco] {
    override def getTexture(t: Taco) = "soggy"
  }

  implicit object SilkTexture extends Texture[Silk] {
    override def getTexture(s: Silk) = "soft"
  }
}
```

We can now create instances of Taco and Silk and pass them to our ad-hoc polymorphic function `printTexture`. The function, depending on the type prints the appropriate texture!

Now you might be asking how did `printTexture` get the appropriate implementation for Texture? The answer is `implicits`; we import the implicits with the statement `import Texture._` and the Scala compiler takes care of the rest. Note, however that we can always provide the `Texture[T]` explicitly and get the desired implementation. 

  > I should also mention that we can only have one type of
  > implicit definition in the same scope because otherwise the
  > Scala compiler get mad. ex. We don't declare WetTacoTexture as
  > an implicit.

**Ahh ha.. we demonstrate bullet (3), the function implementation varies depending on the type!!**

```scala
import Texture._

val taco = new Taco
printTexture(taco) //prints: crunchy
val silk = new Silk
printTexture(silk) //prints: soft

//explicitly declare the Texture[Taco] implementation
printTexture(taco)(WetTacoTexture) //prints: soggy
```

#### Different Syntax:

As an extra bit let's look at another way we can write our ad-hoc polymorphism function. The Scala compiler provides syntactical sugar so that we can re-write the function as follows. So don't get confused when you see either representation.

```scala
//def printTexture[T](t: T)(implicit o: Texture[T]): Unit = {
//  println(o.getTexture(t))
//}


def printTexture[A: Texture](t: A): Unit = {
  val o = implicitly[Texture[A]]
  println(o.getTexture(t))
}
```

## Addendum
Thanks to **Wil Lee** for clarifying this and highlighting the power of Ad-hoc polymorphism.

You might have noticed something cool about ad-hoc polymorphism; at no point did we modify the class `Food` and yet we were able to give it a `Texture`! Ad-hoc polymorphism allows us to extend the functionality of a piece of code, even if we don't have access to the source (library, legacy code).

And to quote Wil, who explains it eloquently:

> With Java interfaces / Scala traits, you can't claim that some type implements such an interface unless you have access to that type's source code and can modify it. With ad-hoc polymorphism, you don't need to be able change the type's source code. Using your post's example, `Taco` and `Silk` can come from some 3rd party library, and due to the implicit pattern you can still define a `Texture` for them.

## Conclusion
Well now you know the different types of polymorphism in Scala. As always, thanks for reading. If you find any mistakes please feel free to contact me at apoorv[aat]toidiu[dott]com


