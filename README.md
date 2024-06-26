## 📦️⚙️ quick-stack

a simple command line file organiser for cleaning directories that often get cluttered
with files of similar names.
<div align="right">

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)
</div>


## installing
### with Nix
this project is a flake, consume it your way.

try it out with:
```bash
nix run github:mtlhrt/quick-stack
```

or install it imperatively with:
```bash
nix profile install github:mtlhrt/quick-stack
```

or however else you want, the package is exposed as `packages.default`.

### from source
you can `git clone` this repository and run `cargo install --path .` if you have Rust installed.
requires a nightly toolchain (`rustup toolchain install nightly`).

## usage
we work with rules here, define what should go where, then snap your fingers and everything gets done.

#### creating a rule
use `quick-stack add` to add a new rule, it takes three arguments: `-m` is a regular expression to match files ,
`-i` is a path to read files from, and `-o` is a path to place files at. For example:
```sh
quick-stack add -m ".png$" -i ~/downloads -o ~/imgs
# ...or
quick-stack add -m "(?i)(*.)gundam(.*).mkv$" -i ~/downloads -o ~/anime/gundam
```


#### sorting
use `quick-stack sort` to go over all rules, sorting files accordingly.
this skips rules that read from directories that can't be accessed, and creates ones
that don't exist.

see `quick-stack --help` for other useful commands.
