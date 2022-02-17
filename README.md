# garen
Rune setter for league of legends.

## Installation

Use the package manager [cargo](https://crates.io/) to install garen.

```rust
cargo build -- release
```

I recommend to set some alias for it:

```bash
alias fetchme="/PATH/garen/target/release/./garen"
```


## Usage

```rust
// sets default ARAM runes
fetchme champion <CHAMPION>
// sets default for mode and default role
fetchme champion <CHAMPION> mode <MODE>
// sets runes for given champion for a specific mode and role
fetchme champion <CHAMPION> mode <MODE> role <ROLE>
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
