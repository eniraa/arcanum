# arcanum

arcanum is a "useful" hobby programming language.

## Background

In my first attempt at a programming language, I made [boxscript](https://github.com/eniraa/boxscript-py), an esolang that exclusively used boxes as code, which resulted in it essentially being a less ergonomic version of brainfuck. The parsing was a mess, because the syntax was quite different from traditional programming language syntax, and also because I had to make it in the span of a single week with no prior experience or knowledge creating languages.

This will be my second attempt at a programming language, and I'm hoping to make it a bit more useful. It will definitely be ambitious, but rather than having a week to complete it, I have no hard deadlines. This time, I'll do things right: using LLVM for its optimizations, using LALRPOP for parsing rather than hacky regexes, etcetera.

## Usage

If you are not on a *nix OS, things may or may not work. I'd suggest you use a VM to avoid needless troubleshooting.

### Dockerless

0. Download the repository.
1. `cd` into the repository.
2. Run `sh install.sh`.
    - Note that this only temporarily aliases the binary to `arcanum`, you will need to rerun this for every new terminal session.
3. Run `arcanum`.

### Docker

0. Download the repository.
1. `cd` into the repository.
2. Build using the Dockerfile which will be made soon™.

## (Planned) Features

- [ ] Obvious language features that need to be in any good modern programming language
- [ ] Edgy and chuunibyou names for things that already have good names
- [ ] Support for custom parsing at runtime
- [ ] Actual documentation

Obviously, the language is not very planned out at this moment. Ideally, it should be Julia-like, in that it's faster than Python but has a bunch of the good stuff. Custom parsing at runtime is also planned, because that allows custom syntaxes, like Ruby's regex syntax, to be used without coding it into the core language. I mean, this project will probably die in a month or so, so why not plan ambitiously?

## License

This project uses the GNU GPLv3. TL;DR: just don't make a closed-source version of this. I'd be flattered if you do, but why would you even want to?
