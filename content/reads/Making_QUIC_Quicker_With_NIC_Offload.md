+++
title = "Making QUIC Quicker With NIC Offload"
date = 2024-01-03

[extra]
paper = "Making_QUIC_Quicker_With_NIC_Offload.pdf"
short = "The authors measured different components of the QUIC protocol. They identified kernel to userspace communication (data copy), crypto, and packer reordering as CPU hungry tasks which could be offloaded to a NIC."
+++

Kernel-userspace data copy accounts for 40-50% of total CPU usage. When kernel bypass is used, crypto operations become the bottleneck and consume up to 40% of CPU per connection. Packet re-ordering accounts for 5-20% of total CPU.

> CPU usage in the multi-connection scenario and saw that the major difference compared to the single connection scenario is that the CPU time spent on packet sending, e.g., pkt formatting, crypto, pkt I/O, has increased by about 10%.

The cost of going from single connection to multi-connections is ~10% CPU.

For the crypto task, `enc_aead()` and `dec_aead()` calls were responsible for approx 90% of CPU time.

The QUIC impls Quant, Quicly, Picoquic and Facebook’s Mvfst were used for the analysis.
- Mvfst seems to be the most balanced in terms of functionality and performance.
- Quicly seems to lack performance compared to other impls.
- Quant however is a research tool and lacks alot of real functionality.
- Picoquic's CC seems to ignore packet loss?? or is this a packet re-ordering functionality?
  - Picoquic implements packet re-ordering engine which could help it maintain throughput when faced with packet loss/re-ordering. They had two different algorithms for the re-ordering engine (splay tree and linear search).

`Netmap` is an efficient I/O framework for sumulating network. `TLEM` is a utility which allows for controlling traffic perturbations, i.e., loss, delay, re-ordering.. etc. TLEM is an extension on Netmap.

> Network Interface Cards (NICs) with programmable hardware components, e.g., network processor, FPGA, can help by easing the host CPU from expensive computation tasks.

Seems like the industry is looking into programable NICs.
