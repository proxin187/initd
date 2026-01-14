#!/bin/bash

set -e

cargo build --release

if [ ! -d "rootfs" ]; then
    echo "creating rootfs"

    rm -f rootfs.cpio.gz
    rm -rf rootfs

    mkdir -p rootfs/{bin,sbin,etc,proc,sys,dev,run,tmp}
    mkdir -p rootfs/etc/initd/{boot,supervise,shutdown,disabled}

    cp /usr/bin/busybox rootfs/bin/busybox
    chmod +x rootfs/bin/busybox

    for cmd in $(rootfs/bin/busybox --list | tr ' ' '\n' | tail -n +2); do
        if [ -n "$cmd" ]; then
            ln -sf /bin/busybox rootfs/bin/$cmd
        fi
    done

    cp /usr/bin/dash rootfs/bin/dash
    chmod +x rootfs/bin/dash
    ln -sf /bin/dash rootfs/bin/sh
else
    echo "using existing rootfs"
fi

cp target/x86_64-unknown-linux-musl/release/initd rootfs/sbin/init
chmod +x rootfs/sbin/init

cd rootfs
find . -print0 | cpio --null -ov --format=newc 2>/dev/null | gzip -9 > ../rootfs.cpio.gz
cd ..

qemu-system-x86_64 \
    -kernel /boot/vmlinuz-linux \
    -initrd rootfs.cpio.gz \
    -append "console=ttyS0 rdinit=/sbin/init rw quiet -- initd.services=/etc/initd" \
    -m 512M \
    -smp 2 \
    -nographic \
    -no-reboot


