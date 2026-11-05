---
title: Install Layoutsmith
description: Download Layoutsmith 0.7.1 for Windows 11, macOS or Linux, or install it from Smith Suite Launcher.
order: 20
verified: 0.7.1
---

Download the file for your system from the [latest release](https://github.com/mahmudurrahman420/layoutsmith/releases/latest) and install it. In the file names, `<version>` stands for the version number, for example `0.7.1`. The release also has `SHA256SUMS.txt`, which lists a checksum for every file. The [Layoutsmith page on github.com/mahmudurrahman420](https://github.com/mahmudurrahman420/layoutsmith) links to the download as well.

## Windows 11

1. Open the [latest release](https://github.com/mahmudurrahman420/layoutsmith/releases/latest).
2. Download `layoutsmith-<version>-windows-x64.msi`. It fits most PCs (64-bit Intel and AMD).
3. Run the file and follow the installer.
4. Start **Layoutsmith** from the Start menu.

The release also has an `arm64` installer for ARM PCs such as Snapdragon, and a 32-bit `x86` installer. To run without installing, download `layoutsmith-<version>-windows-x64-portable.zip` (or the `arm64` or `x86` version), unzip it and start the program inside. The installers and executables are code-signed.

## macOS

1. Download `layoutsmith-<version>-macos-universal.dmg`. It works on Apple silicon and Intel Macs.
2. Open it and drag Layoutsmith to Applications.

The app is signed and notarized.

## Linux

Pick one file from the release. Replace `x86_64` with `aarch64` on ARM.

| File                                          | Notes                                                    |
|-----------------------------------------------|----------------------------------------------------------|
| `layoutsmith-<version>-linux-x86_64.AppImage` | Runs anywhere.                                           |
| `layoutsmith-<version>-linux-x86_64.flatpak`  | Sandboxed. Install with `flatpak install --user <file>`. |
| `layoutsmith-<version>-linux-x86_64.deb`      | Debian and Ubuntu.                                       |
| `layoutsmith-<version>-linux-x86_64.rpm`      | Fedora, RHEL and openSUSE.                               |
| `layoutsmith-<version>-linux-x86_64.tar.gz`   | Unpack anywhere.                                         |

For RISC-V there is a `.deb`, an `.rpm` and a tarball, named with `riscv64` in place of `x86_64`. There is no AppImage or Flatpak for RISC-V. The RISC-V build needs glibc 2.39 or newer (Ubuntu 24.04 or newer). A FreeBSD build (`layoutsmith-<version>-freebsd-x86_64.tar.gz`) is on the release page too.

## Install with Smith Suite Launcher

Smith Suite Launcher lists Layoutsmith and can install and update it for you. See [Smith Suite Launcher](https://github.com/mahmudurrahman420/docs/craft-launcher) for how it works.

## First run

Open Layoutsmith. You land on the start screen. Continue with [Getting started](index.md). If the window closes right away, see [Layoutsmith closes right after it starts](../troubleshooting/app-closes-at-start.md).
