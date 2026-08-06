# Smoke test

This test runs the parser against ~10 000 generated PDSC files that
all pass the [PACK XSD](https://github.com/Open-CMSIS-Pack/Open-CMSIS-Pack-Spec/blob/main/schema/PACK.xsd) definition.

To reduce the amount of individual files on disk the PDSC files have been zipped.
This also demostrates how you could unzip a `.pack` file and read the
PDSC file contained inside.

# License

The fuzzing test files are licensed under MIT, the same as the rest
of the project codebase.