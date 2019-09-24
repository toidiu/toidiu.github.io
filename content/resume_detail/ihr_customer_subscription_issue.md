+++
title = "Amazon subscription not enabling features"
date = 2018-08-11

[extra]
company = "ihr"
lp = ["customer obsession", "dive deep"]
+++

customer obsession: subscription is the core of customer experience

dive deep: look at neighbouring code for amazon, google and apple

#### metrics
- solved issue across 2 different services (amazon and heroku)
- added a test to cover the edge case
- traced and rebated a few thousand customers affected

#### S
Subscription miroservie was responsible for determining subscription status for users.
A CS report indicated that a customer was charged for subscription but was not seeing
premium features.

#### T
Verify that the customer was actually experiencing an error. Figure out a fix to the issue.

#### A
First I verified that the error was actually happening. This took using a combination of
two internal endpoints to check on the subscription and paid status of the user.
I also verified that the subscription had not expired.

The error was occuring for a Amazon user.

Explored the other subscription services since the code should be similar, including
Amazon, Apple, Google...

Oddly enough, the code was slightly different for Amazon and Heroku users compared to
the Google subscription code. Looking at git history one could see that the two had
been added afterwards.

The problem involved finding a shared piece of code logic that was incorrect.

#### R
I was able to fix the subscription code and fix user experience for future users.
This fix got applied not only to Amazon, but also Amazon users.

A follow up task was created to track other users affected and provide them with
rebates/extended experience.

