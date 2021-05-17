#!/bin/bash
rm build/kernel.img
./scripts/build.sh
./scripts/build_limine.sh
./scripts/run_qemu.sh