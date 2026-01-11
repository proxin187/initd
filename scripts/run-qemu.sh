#!/bin/bash

set -e

cargo build --release

rm -rf rootfs
mkdir -p rootfs/{bin,sbin,etc,proc,sys,dev,run,tmp}
mkdir -p rootfs/etc/initd/{boot,supervise,shutdown,disabled}

cp /usr/bin/toybox rootfs/bin/toybox
chmod +x rootfs/bin/toybox

for cmd in $(rootfs/bin/toybox | tr ' ' '\n' | tail -n +2); do
    if [ -n "$cmd" ]; then
        ln -sf rootfs/bin/toybox rootfs/bin/$cmd
    fi
done

cp /usr/bin/dash rootfs/bin/dash
chmod +x rootfs/bin/dash
ln -sf rootfs/bin/dash rootfs/bin/sh

cp target/x86_64-unknown-linux-musl/release/initd rootfs/sbin/init
chmod +x rootfs/sbin/init

cd rootfs
find . -print0 | cpio --null -ov --format=newc 2>/dev/null | gzip -9 > ../rootfs.cpio.gz
cd ..

qemu-system-x86_64 \
    -kernel /boot/vmlinuz-linux \
    -initrd rootfs.cpio.gz \
    -append "console=ttyS0 rw quiet rdinit=/sbin/init" \
    -m 512M \
    -smp 2 \
    -nographic \
    -no-reboot


