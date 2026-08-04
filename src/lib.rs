//! # CMSIS PDSC Parser
//!
//! This is a Rust crate that aims to provide a convenient abstraction to parse
//! [CMSIS Pack Description Format](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html) (PDSC)
//! files.  
//! This project takes in a PDSC file and parses into a Rust datastructure.
//!
//! ## Usage
//!
//! Add the dependency with the following command:
//!
//! ```shell
//! cargo add cmsis-pdsc-parser
//! cargo add roxmltree
//! ```
//!
//! Minimal example:
//!
//! ```rust
//! use cmsis_pdsc_parser;
//! use std::io::read;
//!
//! const PDSC_PATH: &str = "Microchip.PIC32CM-PL_DFP.pdsc";
//!
//! fn main() {
//!     // Read the document conent into memory
//!     let mut f = std::fs::File::open(PDSC_PATH).unwrap();
//!     let mut pdsc_content: String = String::new();
//!     f.read_to_string(&mut pdsc_content).unwrap();
//!
//!     // Parse the XML document
//!     let document = roxmltree::Document::parse(&pdsc_content).unwrap();
//!     // Parse the PDSC file as the root `Package` element.
//!     let pdsc = cmsis_pdsc_parser::Package::new(&document);
//!
//!     println!("{:#?}", pdsc);
//! }
//! ```

use serde::Deserialize;

pub mod pdsc;
pub mod family;
pub mod debug_access;
pub mod requirements;
pub mod generators;
pub mod boards;
pub mod parts;
pub mod taxonomy;
pub mod part_taxonomy;
pub mod apis;
pub mod components;
pub mod conditions;
pub mod csolution;
pub mod examples;

#[derive(Debug, PartialEq, Deserialize)]
/// Represents [PDSC Package](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_package_pg.html)
/// which is the root element of the PDSC file
#[serde(rename_all = "camelCase")]
pub struct Package<'a> {
    /// Name of the software pack
    pub name: String,

    /// Name of the software pack supplier/vendor
    pub vendor: String,

    /// PDSC schema version; valid values defined by [PDSC schema versioning](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    pub schema_version: Option<String>,

    /// Restricts pack to a specific core; valid values: [DcoreEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dcore")]
    pub d_core: Option<String>,

    /// Restricts pack to a specific silicon vendor; valid values: [DeviceVendorEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dvendor")]
    pub d_vendor: Option<String>,

    /// Restricts pack to a specific device name; wildcards allowed
    #[serde(rename = "Dname")]
    pub d_name: Option<String>,

    /// Restricts pack to a specific toolchain; valid values: [CompilerEnumType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Tcompiler")]
    pub t_compiler: Option<String>,

    /// Brief description of the sofware pack
    pub description: pdsc::Description,

    /// Export Control Classification Numbers for the EU and US
    pub eccn: Option<pdsc::Eccn>,

    /// URL or file URI of the sotware pack
    pub url: String,

    /// URL or e-main for users to get support for the Pack content
    pub support_contact: Option<String>,

    /// Path to the license document of the Pack
    pub license: Option<String>,

    /// Listing containing the collection of license fils
    pub license_sets: Option<pdsc::LicenseSets>,

    /// A pack that has dominate attribute overrules other packs
    pub dominate: Option<pdsc::Dominate>,

    /// Specifies other CMSIS-Packs, programming languages, and compilers required by pack components
    pub requirements: Option<requirements::Requirements>,

    // The deprecated `create` element is intentionally not modelled.

    /// HTTPS URL of a public repository tat the pack originates from
    pub repository: Option<pdsc::Repository>,

    /// Version release history with brief information about a software pack
    pub releases: pdsc::Releases,

    /// Section describing one or more changelog files
    pub changelogs: Option<pdsc::Changelogs>,

    /// Keywords that might be used to find a software pack
    pub keywords: Option<pdsc::Keywords>,

    /// Grouping elements for environments information.
    pub environments: Option<pdsc::Environments>,

    /// Specifies generator tools that have been used to generate components
    pub generators: Option<generators::Generators>,

    /// Development boards described in this pack
    pub boards: Option<boards::Boards>,

    /// Hardware parts described in this pack
    pub parts: Option<parts::Parts>,

    /// Component class and group taxonomy for this pack
    pub taxonomy: Option<taxonomy::Taxonomy>,

    /// Hardware part class and group taxonomy for this pack
    #[serde(rename = "part-taxonomy")]
    pub part_taxonomy: Option<part_taxonomy::PartTaxonomy>,

    /// Application programming interfaces defined by this pack
    pub apis: Option<apis::Apis>,

    #[serde(borrow)]
    /// The device family, the devices, and variants
    pub devices: pdsc::Devices<'a>,

    /// Conditions defined for use throughout this pack
    pub conditions: Option<conditions::Conditions>,

    /// Example projects included in this pack
    pub examples: Option<examples::Examples>,

    /// Software layers and project templates for csolution-based projects
    pub csolution: Option<csolution::Csolution>,

    /// Components published by this pack
    pub components: Option<components::Components>,
}

impl<'a> Package<'a> {
    pub fn new(document: &'a roxmltree::Document) -> Self {
        // Parse the content
        let mut package: Package = serde_roxmltree::from_doc(&document).unwrap();

        // Parse the "wild" string conents into structured data
        for family in &mut package.devices.families {
            family.debugvars.parse_debugvars();
            family.sequences.parse_sequences();
        }

        // Return the data
        package
    }
}