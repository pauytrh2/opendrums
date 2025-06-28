# Opendrums

> An independent, FOSS, alternative to Aerodrums

# Vision

Opendrums comes to fix many things in Aerodrums:
- First and foremost, Aerodrum's product costs money, and is not open source.
- Aerodrums's software is not cross-compatible, being only availabe to Windows and MacOS.
- Aerodrums's software is notorious for having a bad UI and UX.

# Installation and usage

## [GitHub releases](https://github.com/pauytrh2/opendrums/releases)
Every major release of the software will be available to Windows, MacOS and linux through GitHub releases.

## Manual compilation
verify rust and cargo are installed
```bash
rustc --version
cargo --version
```
clone, cd, build, and run
```bash
git clone https://github.com/pauytrh2/opendrums
cd opendrums
cargo fmt
cargo run
```
or in one line
```bash
git clone https://github.com/pauytrh2/opendrums && cd opendrums && cargo fmt && cargo run
```