+++
title = "Migrate jenkins from standalone to k8s"
date = 2019-08-11

[extra]
company = "iHeartRadio"
lp = ["think big", "invent and simplify"]
+++

think big
  improve reliability and reduce work exponentially
invent and simplify
  utilize helm chart and then customize for internal tools


#### S
Jenkins was the test automation server we used. However it was deployed on
on a VM without any way to recover, upgrade or replicate the instance and data.

This resulted in tech debt and a fear of doing anything new with the instance.

#### T
I took it upon myself to create a kube deployed instance which could be
replaced and therefore upgraded.

#### A
I created a decrarative instance of the server based on an existing jenkins helm
recipie. I then tweaked it to have custom values and secrets.

The secrets were applied via a api call to the jenkins server (decrypt KMS).

#### R
We were able to migrate to the latest version of Jenkins.

