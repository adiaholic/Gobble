# Gobble - fashionably grep with Rust

Gobble is a command line grep like tool but in Rust


## Installation

Install Cargo on MacOS-X and Linux
```
curl https://sh.rustup.rs -sSf | sh
```

Clone using 
```bash
https://github.com/adiaholic/Gobble.git
```

Build & Install
```bash
cargo build
cargo install
```

## Usage via CLI
```
cargo run <text_to_search> <file_path>

eg - cargo run "Hennessy" poem.txt
adiaholic@Adityas-MacBook-Pro Gobble % cargo run "Hennessy" poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/gobble Hennessy poem.txt`
Hennessy. Dancing
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)