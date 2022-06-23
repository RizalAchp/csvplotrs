# Simple Program for generating Plot Image from CSV file

## library used: 

- **[plotters](https://docs.rs/plotters/latest/plotters/)**
- **[csv](https://docs.rs/csv/latest/csv/)**
- **[clap](https://docs.rs/clap/2.32.0/clap/)**


## build
- **build it with cargo**
```bash
#testing
cargo build && cargo test
cargo run -- --help

#install
cargo build --release && cargo install

#run it
csvplot --help #for help
csvplot --input input.csv generate # generate image data
csvplot --input input.csv --lists # list data in csv
```


## enjoy!
