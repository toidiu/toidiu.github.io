+++
title = "VM"
date = 2018-12-21

[taxonomies]
tag = ["vm", "hypervisor"]

[extra]
id = blog-single
+++

There are two types of VM: system VM and process/language VM. A language VM is also called a runtime. One abstracts out the OS while the other abstracts out a language runtime. A system VM might be good for isolation, security, portability, etc. A language VM is good for portability of a language (think JVM). For the purpose of this article we will consider system VMs.
<!-- more -->

Note: these are some thoughts I compiled awhile ago and only now getting to posting them. I am trying to get in the habit of pushing out content more consistently rather than waiting for it to be perfect.. so please let me know if there are errors or if I have failed to give proper credit.

#### Hypervisor: (https://en.m.wikipedia.org/wiki/Hypervisor)
A hypervisor or virtual machine manager (VMM) is computer software, firmware or hardware that creates and runs virtual machines. A computer on which a hypervisor runs one or more virtual machines is called a host machine, and each virtual machine is called a guest machine. The hypervisor presents the guest operating systems with a virtual operating platform and manages the execution of the guest operating systems. Multiple instances of a variety of operating systems may share the virtualized hardware resources.

#### KVM: (https://en.m.wikipedia.org/wiki/Kernel-based_Virtual_Machine)
Kernel-based Virtual Machine (KVM) is a virtualization infrastructure for the Linux kernel that turns it into a hypervisor. It was merged into the Linux kernel mainline in kernel version 2.6.20, which was released on February 5, 2007.


#### Stack machine: (https://en.wikipedia.org/wiki/Stack_machine)
Instructions in a computer can be implemented via a stack or via a register. A register machine will specify register where its operands and results are. The stack machine on the other hand will assume operands are on the stack and will place the result on the stack as well. For example an `Add` instruction will pop its operands of the stack and will push the result onto the stack to prepare for the next instruction.

Some advantages of stack machines are:
- compact object code(smaller code size)
- simple copilers
- simple interpreters
- fast operand access
- minimal processor state

Some disadvantages of stack machines are:
- more memory references
- common evaluations can't easily be factored out
- ridgid code order

#### Firecracker:
Firecracker is a microVM written in Rust :) Being mico, it is lightweight and offers all the benefits of a VM without the overhead (consumes about 5Mib memory). Because it is written in Rust, it offers safety and performances guarantees. It is currently being used by Amazon to power their Lambda and Fargate services.

https://github.com/firecracker-microvm/firecracker/blob/master/docs/design.md

## tutorial
[30] https://www.youtube.com/watch?v=BNXP0w4Ppto


Resources
## Firecracker:
[1] http://firecracker-microvm.io
[3] https://aws.amazon.com/blogs/aws/firecracker-lightweight-virtualization-for-serverless-computing/

## KVM
[10] https://en.m.wikipedia.org/wiki/Kernel-based_Virtual_Machine

## stack machine
[40] https://en.wikipedia.org/wiki/Stack_machine


