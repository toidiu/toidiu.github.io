+++
title = "Go Proxy Deadlock Fix"
date = 2025-11-01

[extra]
company = "Cloudflare"
highlight = true
+++

While most of my recent work has been in Rust, I also helped maintain the Go proxy at Cloudflare. I fixed a nasty deadlock issue that was causing up to 1K+ proxy instances (~0.2% of fleet) to reject new connections.

The logs were cryptic and the only abnormality was a long-running tls.Dial goroutine. I added observability to a suspicious tls.Dial retry-loop to measure the impact and then root-caused the issue to an interaction where the "accept-timeout" (15 sec) terminated the goroutine before the tls.Dial could release its lock. Adding a lower (10 sec) timeout around tls.Dial resolved the issue as confirmed by the observability I added.
