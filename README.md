# adbexplorer
**IN DEVELOPMENT! - breakable changes and random bugs are inevitable. Use it on own risk and report bugs in "issues" section!**

A simple file-explorer for transfering files using [ADB](https://developer.android.com/studio/command-line/adb) (Android Debugging Bridge). The advantage is no need to download anything to your phone and supports **any** android version.

## Installation

1. install ADB server. On your PC

**Linux:**
Usally this can be done installing ```android-tools``` or ```adb``` package from [package manager](https://command-not-found.com/adb).

**macOS:**
You can install from homebrew:
```brew install android-platform-tools```

**Windows:** TODO

Install adbexplorer from [cargo](https://crates.io/) with this command: ``cargo install --git https://github.com/LaineZ/adbexplorer.git``

## Building
1. [Download Rust]([https://www.rust-lang.org/learn/get-started) and run these commands
2. ```git clone https://github.com/LaineZ/adbexplorer.git```
3. ```cd adbexplorer```
4. ```cargo build --release```
5. ```cd target/release```
6. DONE