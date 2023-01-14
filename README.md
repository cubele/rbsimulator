# RB poor
Brand new Reflec Beat simulator.

Created using rust and [BEVY](https://bevyengine.org/)

## How to run
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. run `cargo run --release [OPTIONS] <FUMENPATH> <SONGPATH>` or download the binary and run `rbsimulator.exe [OPTIONS] <FUMENPATH> <SONGPATH>` *with relative path*
```
Usage: rbsimulator.exe [OPTIONS] <FUMENPATH> <SONGPATH>

Arguments:
  <FUMENPATH>  The relative path to the fumen file, supports ply and json
  <SONGPATH>   The relative path to the song file, supports mp3 and ogg

Options:
  -d, --delay <DELAY>      The delay in milliseconds between the start of the song and the start of the fumen
  -s, --start <STARTTIME>  Make the song start at this time in miliseconds instead of the beginning
  -m, --meta <METAPATH>    The path to the metadata file, not supported yet!
  -h, --help               Print help
```

## TODO List
- [x] basic game
- [x] chain objects
- [ ] just reflecs
- [x] VO
- [x] SO
- [x] LO
- [x] chords
- [x] precise RB physics(?)
- [x] precise RB fumen generation(?)
- [x] better UI
- [x] PLY parser(PLY->JSON->fumen)
- [x] CLI interface
- [ ] menu/song selection
- [ ] Fumen maker
- [ ] be able to play
- [ ] latency and performance

## references
[Rhythm game in Rust using Bevy](https://caballerocoll.com/blog/bevy-rhythm-game/)

[Creating a Snake Clone in Rust, with Bevy](https://mbuffett.com/posts/bevy-snake-tutorial/)

[Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)

## credits
Team SCHWARZSCHILD