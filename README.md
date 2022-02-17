# garen
Rune setter for league of legends.

## Installation

Use the package manager [cargo](https://crates.io/) to install garen.

```rust
cargo build
```

## Usage

```rust
// sets default ARAM runes
cargo run champion <CHAMPION>
// sets default for mode and default role
cargo run champion <CHAMPION> mode <MODE>
// sets runes for given champion for a specific mode and role
cargo run champion <CHAMPION> mode <MODE> role <ROLE>
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
