+++
title = "PDT build"
date = 2020-12-01

[extra]
company = "aws_lm"
lp = []
+++

Project plan: https://quip-amazon.com/NtptAZ55eOpb/LM-in-PDT-Work

### S
- LM was tasked to make the service available in PDT.
- PDT should be the last region; LM should therefore also be available in all other regions.

### T
- Build the entire LM stack in PDT to meet gov compliance. (canaries, dispatcher, alarms, metrics)
- Created a project plan to identify all components and missing regions.
- Automate the infrastructure to make builds reproducible.

### A
- Cleaned up our LPT rules to make service launches reqproducible, extensible and maintainable.
- Synthesized new alarms, cleaning-up/creating dashboards along the way.

### R
- Full LM availability in all regions, including PDT (met gov compliance).
- A clean, normalized LPT stack for all services owned by LM.

#### metrics
- helped surface questions and drive conversation for support in restricted regions

- create and manage doc to track work for PDT region build
- debug various issues as they came up during rebuild

