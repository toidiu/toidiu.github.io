---
title: "Android low memory testing"
nav: blog
id: blog
categories:
  - Code
  - Android
tags:
  - testing
  - memory
---

There are a few things that separate mobile development from other types of development. Yes, actually your code does have to be smaller (in the previous version of the Android compiler, there was a limit on the number of methods that your application could have)... but really you are dealing with two main struggles `loss of network connectivity`, `low memory`. We will focus on the low memory issue and how you can go about testing this ephemeral state.

### Out of sight, out of mind
It is often easy to ignore a problem that is unlikely to occur. Of course as the consumer market steadily moves towards more powerful phones, low memory issue should be a thing of the past. However, if each app developer thinks along these lines then we make no progress. Additionally, low memory issues can happen quite often: start a camera intent, or the user navigates to another app and then comes back.

### Living on the edge: destroy everything
Enable the setting: `Settings -> Developer options -> Don't keep activities`. By checking this option Android will simulate low memory behavior, and destroy each activity as the user navigates away from it. This is what the OS does to reclaim memory and can result in invalid state within or loss of state.

### The Good news
Android actually gives you a lot (such as views), when you return to an activity that was destroyed. However if you are setting field variables then its upon you to restore those. Use the method `onSaveInstanceState()` to save any variables and `onRestoreInstanceState()` to restore the variables back.


Happy developing :)
