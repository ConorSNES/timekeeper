# ![timekeeper icon](media/icon/icon_48.png) Timekeeper

[![run-tests](https://github.com/ConorSNES/timekeeper/actions/workflows/run-tests.yml/badge.svg)](https://github.com/ConorSNES/timekeeper/actions/workflows/run-tests.yml)

|![main screen](_screenshots/main.png)|
|---|

Large display, widget-like clock application with rich keyboard shortcut support. 
Remake of an older python script I used to use for time management.

## Features

- Extremely portable
- Highly keyboard-accessible
- Persistent config
- Borderless mode
- Light/dark theme support

## Installing

Automated builds for Windows and Linux (64-bit) are available from [repository releases](https://github.com/ConorSNES/timekeeper/releases/latest). Other platforms are currently untested.

Building from source will require the [Rust/Cargo toolchain](https://rust-lang.org/learn/get-started/). The below code should automate creation of a release binary in a directory of your choice;

```bash
git clone https://github.com/ConorSNES/timekeeper.git
cd ./timekeeper
cargo build -r
cp ./target/release/timekeeper ..
```

(On Windows, the output binary will be "timekeeper.exe" instead of "timekeeper". Modify accordingly.)

## Licence

All repository-authored source code is licenced under [apache-2.0](LICENCE.txt) ([web copy](https://www.apache.org/licenses/LICENSE-2.0)).

Fonts (Open Sans [bold](src/font/OpenSans-Bold.ttf), [medium](src/font/OpenSans-Medium.ttf) and [light](src/font/OpenSans-Light.ttf)) included with codebase and with compiled program under OFL. Full licence details are included within [`opensans-ofl.txt`](src/font/opensans-ofl.txt) next to files, as provided with [the source of the font](https://github.com/googlefonts/opensans).