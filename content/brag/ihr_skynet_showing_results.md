+++
title = "Skynet - delivering results for hackday project"
date = 2018-06-11

[extra]
company = "ihr"
lp = ["deliver results", "right alot"]
+++

right alot: choosing java and solving part of the process was the correct small step to allow for adoption

deliver results: deliver a project despite it not being the original all encompassing solution

#### metrics
- reduced manual testing from 10-30 minutes to seconds
- removed human error during testing
- created a extensible framework, initially targeting 2 types of test
- agnostic testing framework for both android and ios

#### S
This was during a hack week project and the goal of our 3 person team was to
help automate QA-testing done for apps at the company.

The current way to do testing was to open the app, in a simulator and use
charles proxy to collect http traffic logs. The logs would then be parsed
manually.

This could take entire days and sometimes resulted in incomplete testing if
an urgent release was scheduled. It also meant long hours and sometimes
weekend work for the QA team.

The team was composed of a non-technical QA member, an iOS dev and myself (backend).

#### T
The initial goal was to use a UI testing framework for iOS and therefore automate
the collection and then verification of the tests. We spent 1.5 days trying
to get the UI framework to work, however it was unpredictable on different computers
and eventually just didnt work.

#### A
Realizing that we were at risk of not having any product I decided to take a step
back and see if we could produce something useful.

I took half the day to create a simple POC; to tackle the testing and not the data
gathering portion.

This was a much simpler and predictable problem to solve. The log data could be ingested
as Json and a series of tests could be run on it. Java was chosen since it was a
language most people would be comfortable with, it was typed and the non-technical
QA memebers could augment it.

We created a somewhat abstract testing framework. It had 3 broad scopes: ingest data from file,
filter data based on test to be run (user id, header), verify data based on custom test rules.

At some point I devoted majority of my time to training and guiding the QA member on
understanding and augmenting to the codebase. Transfering owenership to the QA team
was an important goal for me if the project was to succeed.

#### R
We were also tackle time sensitive and manually arduous tests (token refresh every 5min).
By the end of the week we were able to execute 6 different tests, implement
2 different testing scenarios (find a value, stateful testing).

Needless to say the QA team was very happy and with the testing abstraction in place they
were then able to implement more tests themselves.

