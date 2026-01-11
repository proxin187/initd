# initd

initd is a minimalist init system for linux, similar to runit it has 3 stages, the boot stage, the supervision stage and the shutdown stage.

## Features
- Emergency shell on unrecoverable errors, enabling manual recovery

## Development
To test initd during development you can either use the [automated script](scripts/run-qemu.sh) or follow the following steps:
- Create a rootfs directory with all the needed directories (eg. /bin, /sbin, /etc/initd, and so on)
- Copy [toybox](https://landley.net/toybox/) utilities into /bin
- Copy the initd binary from target/x86_64-unknown-linux-musl/release/initd into /sbin/init
- Build initramfs archive of rootfs
- Boot qemu with initrd set to the initramfs archive

## License
initd is licensed under the MIT License.


