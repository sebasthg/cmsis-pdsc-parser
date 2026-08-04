# CMSIS PDSC Parser

This is a Rust crate that aims to provide a convenient abstraction to parse
[CMSIS Pack Description Format](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html) (PDSC)
files.  
This project takes in a PDSC file and parses into a Rust datastructure.

## Usage

Add the dependency with the following command:

```shell
cargo add cmsis-pdsc-parser
cargo add roxmltree
```

Minimal example:

```Rust
use cmsis_pdsc_parser;
use std::io::read;

const PDSC_PATH: &str = "Microchip.PIC32CM-PL_DFP.pdsc";

fn main() {
    // Read the document conent into memory
    let mut f = std::fs::File::open(PDSC_PATH).unwrap();
    let mut pdsc_content: String = String::new();
    f.read_to_string(&mut pdsc_content).unwrap();

    // Parse the XML document
    let document = roxmltree::Document::parse(&pdsc_content).unwrap();
    // Parse the PDSC file as the root `Package` element.
    let pdsc = cmsis_pdsc_parser::Package::new(&document);

    println!("{:#?}", pdsc);
}
```

## Contributing

Contributions are welcome!

For more information see [`CONTRIBUTING.md`](./CONTRIBUTING.md).

## License

This project is MIT Licensed.
See the `LICENSE` file.