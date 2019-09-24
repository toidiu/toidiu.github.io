+++
title = "Templating kube config"
date = 2018-05-11

[extra]
company = "iHeartRadio"
lp = ["think big", "disagree and commit"]
+++

#### metrics
- goods:
  - used `helm template` command to avoid creating new tool
  - would be able to slowly template and move over existing configs
  - poc worked and was able to represent current config
- bads:
  - would be 1 additional tool to install
  - helm template was no longer supported
  - go templating had complicated syntax
- failures:
  - project got hijacked from under me (loss of trust)
  - should have consulted seniors more (get buy in)
  - should have demonstrated small example rather go for the cake
  - should have asked for more feedback from team
  - should have demonstrated the stability of heml template

#### S
instead of updating configs across multiple envs and regions (qa, stg, us, au, nz) I wanted to create a template that would allow us to update the value in a single file. The remaining content could then be defined once.

#### T

#### A

#### R
Once the implementation had was done and the team clearly supported the other implementatino, I made it a point to verbally commend it in a meeting and show support.

