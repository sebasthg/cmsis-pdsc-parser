use std::io::Read;

use log::{trace, debug, info};
use zip;

use cmsis_pdsc_parser::Package;

const PDSC_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/smoke_test/pdsc_files.zip"
);

fn main() {
    env_logger::init();

    // Open the zip file
    let zip_file = std::fs::File::open(PDSC_PATH).unwrap();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();

    // Assert that we have the expected number of files
    info!("Openened {PDSC_PATH} and found {} files", archive.len());
    assert_eq!(archive.len(), 10_006, "Wrong number of files in PDSC archive");

    // Parse each PDSC file entry
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        debug!("Testing {}", file.name());
        if !file.is_file() {
            debug!("{} is not a file, skipping...", file.name());
            continue;
        }

        // Read the contents
        let mut pdsc_content: String = String::new();
        file.read_to_string(&mut pdsc_content).unwrap();

        // Parse the file
        let document = roxmltree::Document::parse(&pdsc_content).unwrap();
        let pdsc = Package::new(&document).unwrap();
        trace!("Got: {pdsc:#?}");
    }
}
