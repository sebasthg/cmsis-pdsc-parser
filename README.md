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

```rust
use cmsis_pdsc_parser;
use roxmltree;

const PDSC_PATH: &str = "Microchip.PIC32CM-PL_DFP.pdsc";

fn main() {
    // Read the document content into memory
    let pdsc_content: String = std::fs::read_to_string(PDSC_PATH).unwrap();

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
