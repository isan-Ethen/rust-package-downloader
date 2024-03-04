# rust-package-downloader

This is an downloader for collect neccesary packages for installing software by apt.

## Installation
Get the latest version.
```sh
git clone https://github.com/isan-Ethen/rust-package-downloader.git
```

## Usage
1. Show packages are needed by software.
```sh
sudo apt install --print-uris SoftwareName
```

2. Copy only rows related the packages from output and paste on list.txt.
> This is an example of "rows related the packages"
>```
>'http://ports.ubuntu.com/ubuntu-ports/pool/universe/a/autogen/libopts25_5.18.16-3_arm64.deb' libopts25_1%3a5.18.16-3_arm64.deb 55536 MD5Sum:9f0d852a9f64373bf5f0e832b2bc2e1f
>```
3. Execute.
```sh
cargo run
```
4. Move packages to /var/cache/apt/archives
5. Execute apt install
```sh
sudo apt install SoftwareName
```

## License
Distributed under the Apache-2.0 License. See LICENSE for more information.
