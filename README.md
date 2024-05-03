## 📦️⚙️ quick-stack
A specific file organisation tool for people with a downloads folder that gets really cluttered really fast, especially with files of similar names.
Yes, the name is a [Terraria reference](https://terraria.wiki.gg/wiki/Storage_items#Quick_Stack).

**Status**: stable, but still pretty work-in-progress.

## Installing
### With [Nix](https://nixos.org/)
This project is a flake, consume it your way.

### From source
You can `git clone` this repository and run `cargo install --path .` if you have Rust installed.
Requires a nightly toolchain (`rustup toolchain install nightly`).

### From binary
Planned.

## Usage
You can create rules for how files should be sorted (i.e. what files should go, from where, and to where),
then run a command to sort everything defined in those rules. They're all saved in a file at `$XDG_DATA_HOME/quick-stack/rulefile`.

#### Creating a rule
Use `quick-stack add` to add a new rule, it takes three arguments: `-m` is a regular expression to match files ,
`-f` is a path to read files from, and `-t` is a path to place files at. For example:
```sh
# To move any file ending in .png from your downloads folder into your images folder.
quick-stack add -m ".png$" -f ~/downloads -t ~/imgs
# To move all your (legally obtained) Gundam episodes into a separate folder.
quick-stack add -m "(?i)(*.)gundam(.*).mkv$" -f ~/downloads -t ~/anime/gundam
```


#### Quick stacking
Use `quick-stack sort`. It will skip rules that read from or write to directories that it can't access, be it because of permission errors or 
because they don't exist (It won't create directories, yet.).

See `quick-stack --help` for other useful commands.
