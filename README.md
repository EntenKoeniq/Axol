# Axol
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]

Axol is a minimal 64-bit kernel project for learning operating system development.

| :warning: This is just a hobby project and anyone can contribute, but it probably won't get big and professional! |
| --- |

<details>
<summary>Screenshot of the current state</summary>
<br>
<p align="center">
  <img src="screenshot.png?raw=true" alt="QEMU screenshot"/>
</p>
</details>

# 📋 Prerequisites
- A text editor such as [VS Code](https://code.visualstudio.com/).
- [Docker](https://www.docker.com/) for creating our build-environment.
- [QEMU](https://www.qemu.org/) for emulating our operating system.

# 💾 Getting Started
Start by cloning the repository with:
`git clone --recursive https://github.com/EntenKoeniq/Axol.git`

## Setup
Build an image for our build-environment:
`docker build buildenv -t axol-buildenv`

## Build
Enter build environment:
- Linux or MacOS: `docker run --rm -it -v "$(pwd)":/root/env axol-buildenv`
- Windows (PowerShell): `docker run --rm -it -v "${pwd}:/root/env" axol-buildenv`

### Commands
Build kernel:
`make`

Remove all build files:
`make clean`

To leave the build environment, enter `exit`.

## Emulate
You can emulate Axol with [QEMU](https://www.qemu.org/) and:
`qemu-system-x86_64 -drive format=raw,file=build/kernel.iso`

## Cleanup
Remove the build-evironment image:
`docker rmi axol-buildenv -f`

# 📝 License
This project is licensed under [Apache License 2.0](https://github.com/EntenKoeniq/Axol/blob/main/LICENSE)

[contributors-shield]: https://img.shields.io/github/contributors/EntenKoeniq/Axol.svg?style=for-the-badge
[contributors-url]: https://github.com/EntenKoeniq/Axol/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/EntenKoeniq/Axol.svg?style=for-the-badge
[forks-url]: https://github.com/EntenKoeniq/Axol/network/members
[stars-shield]: https://img.shields.io/github/stars/EntenKoeniq/Axol.svg?style=for-the-badge
[stars-url]: https://github.com/EntenKoeniq/Axol/stargazers
[issues-shield]: https://img.shields.io/github/issues/EntenKoeniq/Axol.svg?style=for-the-badge
[issues-url]: https://github.com/EntenKoeniq/Axol/issues
