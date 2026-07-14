+++
title = "Meetup: boar CC analysis"
date = 2026-06-01

[taxonomies]
tag = ["presentation"]

[extra]
company = ""
highlight = false
+++

https://github.com/toidiu/boar/blob/main/A_statistical_analysis_of_Congestion_Control_algorithms.pdf 

I love sharing networking concepts, and since I do most of my work in Rust, I usually end up
presenting my work to this group. In this talk, I shared my experience building a local 'tc' based
network simulator. I took the opportunity to share what congestion control algorithms are, why they
are critical to networking, and why they are so difficult to measure due to the non-determinism.

The core insight in the presentation was performing statistical analysis to find the signal in the
noise, particularly through the use of CDFs. By using CDFs, I found it possible to easily visualize
the entire probability distribution, which can be hard when looking at isolated p-values (e.g. p50,
p99).
