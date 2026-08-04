## Contributing

Contributions are welcome!

This project aims to follow the CMSIS Pack standard as closely as possible.  
However, if required the project is not opposed to modifications to make it
work with non-standrd compilant PDSC files, within reason.

### Development setup

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/my-useful-feature`)
3. Commit your changes (`git commit -m 'feat: Added useful feature'`)
4. Push to your branch (`git push origin feature/my-useful-feature`)
5. Make sure the code follows the code style
6. Make sure the code has tests
7. Create a pull-request

### Code style

- `cargo fmt` has been run, requires [rustfmt](https://github.com/rust-lang/rustfmt)
- `cargo clippy` should output no warnings
- All struct/enum fields and definitions should have a doc comments
- All modules have doc comments
- All modules have tests, where it makes sense