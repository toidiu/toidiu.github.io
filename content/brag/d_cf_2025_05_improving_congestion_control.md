+++
title = "Improving Congestion Control"
date = 2025-05-01
draft = true

[taxonomies]
tag = ["work"]

[extra]
company = "Cloudflare"
+++

https://github.com/toidiu/boar
I worked to measure and improve the BBR congestion control algorithm at Cloudflare. While we did most of our analysis on sampled aggregate data, network performance depends so heavily on customer use case and network conditions. For this reason I wanted to simulate the network using tc, create program to automate multiple runs, and finally perfrom statistical analysis on the data. The main program runs simulations and generates reports while the viewer loads the reports and allows for side by side comparison of CDF graphs.
