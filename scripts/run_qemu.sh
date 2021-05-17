#!/bin/bash
# -d cpu_reset,int
qemu-system-x86_64 build/kernel.img -serial stdio -no-reboot -no-shutdown -D ./log.txt -d int