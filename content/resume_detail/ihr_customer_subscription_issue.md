+++
title = "other"
date = 2019-08-11

[extra]
company = "iHeartRadio"
lp = ["customer obsession", "highest standard"]
+++

### S
Subscription miroservie was responsible for determining subscription status for users.
A CS report indicated that a customer was charged for subscription but was not seeing
premium features.

### T
Verify that the customer was actually experiencing an error. Figure out a fix to the issue.

### A
First I verified that the error was actually happening. This took using a combination of
two internal endpoints to check on the subscription and paid status of the user.
I also verified that the subscription had not expired.

The error was occuring for a Google user.

Explored the other subscription services since the code should be similar, including
Google, Apple, Amazon...

Oddly enough, the code was slightly different for Google and Amazon users compared to
the Apple subscription code. Looking at git history one could see that the two had
been added afterwards.

The problem involved finding a shared piece of code logic that was incorrect.

### R
I was able to fix the subscription code and fix user experience for future users.
This fix got applied not only to Google, but also Amazon users.

A follow up task was created to track other users affected and provide them with
rebates/extended experience.

