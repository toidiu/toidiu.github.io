+++
title = "On privacy and control"
date = 2025-12-25

[taxonomies]
tag = ["privacy"]

[extra]
id = "blog-single"
+++

"I don't need to care about privacy because I have nothing to hide." is an
argument that I have heard countless times. It's not unreasonable argument that
I found difficult to dismiss. Yet somehow I knew it was wrong and only recently
realized that the real cost of not caring is control.

<!-- more -->

Here is a conversation I had with a friend:

For me privacy is not the primary driver, because like you mentioned, it doesn't
make sense and doesn't actually fall into people's threat model (journalists on
the other hand should care). I am personally motivated by the notion of
"control". Can someone else meditate how I experience the world and what
information I consume? Whether that is censoring, influencing how much time I
have to spend watching ads or which ads I am allowed to watch.

I have a personal domain and use my personal domain for email i.e.
hello@toidiu.com so I can switch providers if I want (google can terminate your
account, ProtonMail can raise their prices). To this same effect I limit cloud
storage for critical things (not 100% true, I use github and google docs for
taxes/finances with Anna). I use GNU pass because I don't want to hand over my
passwords to a 3rd party. I use firefox with privacy badger and uOrigin because
I do not opt into companies personally targeting me (can be contentious since
some sites use this to make money).

I host my calendar and contacts on a raspberrypi, which you can only access on
localhost. The caldav server is https://sabre.io/baikal, which is not perfect
but gets the job done.

I use graphene os to sandbox my apps on android and privilege google/ISP apps
from doing what ever on my personal device. Graphene OS also allows you to
restrict permissions at a granular level, including network access. Did you know
disabling google play also increases your battery life by alot! Not updating
phone OS also allows you keep using your phone for many years. I don't install
any social media on my phone (whatsapp is borderline and I hate that I need it).
Signal for messaging is preferable. Venmo is disabled. No rideshare apps. I try
to use opensource alternatives for apps on F-Droid.

I use Cloudflare's (disclaimer: this is now my current employer) DNS because I
trust them more than other companies; purely based on their business and how
their incentives align (https://one.one.one.one/). Questioning how "incentives
align" is a really good litmus test for alot of things.
