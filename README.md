## 📦️⚙️ quick-stack
<div align="center">

A simple command line file organiser for cleaning directories that tend to get cluttered very often
with files of similar names.

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)
</div>


## Installing
### With Nix
This project is a flake, consume it your way.

Try it out with:
```bash
nix run github:comfybyte/quick-stack
```

Or install it imperatively with:
```bash
nix profile install github:comfybyte/quick-stack
```

Or consume it in your flake (recommended).

### From source
You can `git clone` this repository and run `cargo install --path .` if you have Rust installed.
Requires a nightly toolchain (`rustup toolchain install nightly`).

## Usage
We work with rules here, define what should go where, then snap your fingers and everything gets done.

#### Creating a rule
Use `quick-stack add` to add a new rule, it takes three arguments: `-m` is a regular expression to match files ,
`-f` is a path to read files from, and `-t` is a path to place files at. For example:
```sh
quick-stack add -m ".png$" -f ~/downloads -t ~/imgs
# ...or
quick-stack add -m "(?i)(*.)gundam(.*).mkv$" -f ~/downloads -t ~/anime/gundam
```


#### Sorting
Use `quick-stack sort` to go over all rules, sorting files accordingly.
This skips rules that read from directories that can't be accessed, and creates ones
that don't exist.

See `quick-stack --help` for other useful commands.

****
*Yes, the name is a [Terraria reference](https://terraria.wiki.gg/wiki/Storage_items#Quick_Stack)*.
