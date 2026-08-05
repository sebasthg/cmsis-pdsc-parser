//! Contains the types required to represent a [PDSC Family](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html) element

use std::{collections::HashMap, fmt::Debug};
use roxmltree::Node;
use serde::{Deserialize, Serialize};
use log::{error, trace, warn};
use serde_roxmltree::RawNode;
use crate::debug_access::{self, Statement};

/// Deserializes a `u32` from a decimal or `0x`-prefixed hex string.
fn de_uint<'de, D>(d: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    let trimmed = s.trim();
    if let Some(hex) = trimmed.strip_prefix("0x").or_else(|| trimmed.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).map_err(serde::de::Error::custom)
    } else {
        trimmed.parse::<u32>().map_err(serde::de::Error::custom)
    }
}

/// Deserializes an `Option<bool>` from an xs:boolean string ("true", "false", "1", "0").
/// Only called when the field is present; absent fields use `default` (None).
fn de_opt_bool<'de, D>(d: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    match s.as_str() {
        "true" | "1" => Ok(Some(true)),
        "false" | "0" => Ok(Some(false)),
        other => Err(serde::de::Error::custom(format!("expected xs:boolean, got: {other}"))),
    }
}

/// Parse error for family XML elements.
#[derive(Debug, PartialEq)]
pub enum FamilyParseError {
    /// A required XML attribute was absent.
    MissingAttribute(String),
    /// An unrecognised element type was encountered.
    UnknownElementType(String),
    /// A debugvar declaration could not be parsed.
    MalformedDebugvar(String),
    /// Hit an unimplemented content-parse branch.
    UnimplementedContent,
}

impl std::fmt::Display for FamilyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingAttribute(attr) => write!(f, "missing attribute: {attr}"),
            Self::UnknownElementType(tag) => write!(f, "unknown element type: {tag}"),
            Self::MalformedDebugvar(msg) => write!(f, "malformed debugvar: {msg}"),
            Self::UnimplementedContent => write!(f, "unimplemented content-parse branch"),
        }
    }
}

impl std::error::Error for FamilyParseError {}

#[derive(Debug, PartialEq, Deserialize)]
/// Represents [PDSC Family](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html)
pub struct Family<'a> {
    #[serde(rename = "Dfamily")]
    /// The device family name
    pub device_family: String,

    #[serde(rename = "Dvendor")]
    /// The device manufacturer/vendor; valid values: [DeviceVendorEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    pub vendor: String,

    /// Brief family description
    pub description: Option<String>,

    /// Processor definitions (0..*)
    #[serde(rename = "processor", default)]
    pub processor: Vec<Processor>,

    /// Compile-time definitions (0..*)
    #[serde(rename = "compile", default)]
    pub compile: Vec<Compile>,

    /// Debug unit assignments (0..*)
    #[serde(rename = "debug", default)]
    pub debug: Vec<FamilyDebug>,

    /// Debug configuration (0..1)
    pub debugconfig: Option<DebugConfig>,

    /// Debug port definitions (0..*)
    #[serde(rename = "debugport", default)]
    pub debugport: Vec<DebugPort>,

    /// Access port v1 definitions (0..*)
    #[serde(rename = "accessportV1", default)]
    pub access_port_v1: Vec<AccessPortV1>,

    /// Access port v2 definitions (0..*)
    #[serde(rename = "accessportV2", default)]
    pub access_port_v2: Vec<AccessPortV2>,

    /// Flash programming algorithms (0..*)
    #[serde(rename = "algorithm", default)]
    pub algorithm: Vec<Algorithm>,

    /// Flash information (0..*)
    #[serde(rename = "flashinfo", default)]
    pub flashinfo: Vec<FlashInfo>,

    /// Memory regions (0..*)
    #[serde(rename = "memory", default)]
    pub memory: Vec<Memory>,

    /// Trace unit definitions (0..*)
    #[serde(rename = "trace", default)]
    pub trace: Vec<Trace>,

    /// Reference documentation (0..*)
    #[serde(rename = "book", default)]
    pub book: Vec<Book>,

    /// Feature descriptors (0..*)
    #[serde(rename = "feature", default)]
    pub feature: Vec<Feature>,

    /// Tool environment entries (0..*)
    #[serde(rename = "environment", default)]
    pub environment: Vec<FamilyEnvironment>,

    /// Sub-family groupings within this family (0..*)
    #[serde(rename = "subFamily", default)]
    pub sub_families: Vec<SubFamily<'a>>,

    /// Devices directly in this family, without a sub-family grouping (0..*)
    #[serde(rename = "device", default)]
    pub devices: Vec<Device<'a>>,

    /// Global debug variables
    pub debugvars: Debugvars,

    /// Debug sequences
    #[serde(borrow)]
    pub sequences: Sequences<'a>
}

#[derive(Debug, PartialEq, Deserialize)]
/// Represents the `traceSetput` attribute in [PDSC sequences](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_sequences)
pub enum TraceSetup {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "legacy")]
    Legacy
}

#[derive(Debug, PartialEq, Deserialize, Default)]
/// Represents [PDSC Debugvars](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_debugvars)
pub struct Debugvars {
    /// The relative path to the configuration file containing debugvars
    pub configfile: Option<String>,

    /// Debugvars version
    pub version: Option<String>,

    #[serde(rename = "#content")]
    /// Debugvars variable declarations
    pub content: String,

    /// Parsed debugvars, initially `None`, generated by [Debugvars::parse_debugvars]
    pub parsed_debugvars: Option<HashMap<String, u64>>
}

impl Debugvars {
    /// Parses a debugvar value string into u64
    fn parse_value_string(value: &str) -> Option<u64> {
            // Try to parse the value as a hex value firs
            if let Some(hex_value_str) = value.strip_prefix("0x")
               && let Ok(k) = u64::from_str_radix(hex_value_str, 16) {
                    return Some(k);
            };

            // If not hex, parse as base-10
            match value.parse() {
                Ok(k) => Some(k),
                Err(e) => {
                    error!("Failed to parse debugvar value ({}) to u64 with error: {}", value, e);
                    None
                }
            }
    }

    /// Parses a single variable declaration entry.
    ///
    /// Returns `Ok((name, value))` for a valid `__var name = value;` declaration,
    /// or `Err(FamilyParseError::MalformedDebugvar(...))` for any malformed input.
    pub fn parse_single_debugvar(line: &str) -> Result<(String, u64), FamilyParseError> {
        trace!("Parsing debugvar line: {}", line);

        // Remove comments
        let declaration = if line.trim_start().starts_with("//") {
            let parts: Vec<&str> = line.split("\n").collect();
            parts[1].trim()
        } else {
            line.trim()
        };

        if declaration.is_empty() {
            return Err(FamilyParseError::MalformedDebugvar("empty declaration".to_string()));
        }

        let stripped_declaration = match declaration.strip_prefix("__var ") {
            Some(val) => val,
            None => {
                warn!("Variable in debugvars does not start with \"__var \": {:?}", declaration);
                return Err(FamilyParseError::MalformedDebugvar(
                    format!("missing '__var' prefix: {declaration:?}")
                ));
            }
        };

        let parts: Vec<&str> = stripped_declaration.split("=").collect();
        if parts.len() != 2 {
            warn!("Got something other than 2 fields when parsing debugvar: {:?}", parts);
            return Err(FamilyParseError::MalformedDebugvar(
                format!("expected exactly one '=' separator, got {} fields", parts.len())
            ));
        }

        let name: String = parts[0].trim().to_string();
        let value_str = parts[1].trim();

        if let Some(value) = Self::parse_value_string(value_str) {
            Ok((name, value))
        } else {
            Err(FamilyParseError::MalformedDebugvar(
                format!("could not parse value as u64: {value_str:?}")
            ))
        }
    }

    /// Parses the debugvars content and returns a hashmap with the variable name as the key and the value as the value
    pub fn parse_debugvars_content(&self) -> HashMap<String, u64> {
        // Remove any lines starting with "//"
        let content: String = self.content.split("\n").filter_map(|line| {
            if line.trim().starts_with("//") {
                trace!("Ignoring line: {}", line);
                None
            } else {
                Some(line.to_owned() + "\n") // Add back the newline
            }
        }).collect();

        // Variables are split with a ';'
        // While unlikely the spec does not forbid multiple inline assignments, e.g.:
        //    __var foo = 5; __var bar = 0x42;
        // As such we must split on ';' rather than '\n' for actual parsing.
        let variables: Vec<&str> = content.split(';').collect();

        variables.iter().filter_map(|var| {
            // Silently skip empty segments and malformed lines
            Self::parse_single_debugvar(var).ok()
        }).collect()
    }

    /// Performs the parsing and stores values in [Self::parsed_debugvars]
    pub fn parse_debugvars(&mut self) {
        let vars = self.parse_debugvars_content();

        self.parsed_debugvars = Some(vars);
    }
}

/// Represents a [PDSC processor](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_processor) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Processor {
    /// Processor instance name for multi-core devices
    #[serde(rename = "Pname")]
    pub pname: Option<String>,
    /// Number of processor units for multi-core devices
    #[serde(rename = "Punits")]
    pub punits: Option<u32>,
    /// Processor core; valid values: [DcoreEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dcore")]
    pub dcore: Option<String>,
    /// Processor core architecture version
    #[serde(rename = "DcoreVersion")]
    pub dcore_version: Option<String>,
    /// Floating point unit; valid values: [DfpuEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dfpu")]
    pub dfpu: Option<String>,
    /// Memory protection unit; valid values: [DmpuEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dmpu")]
    pub dmpu: Option<String>,
    /// DSP instructions support; valid values: [DdspEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Ddsp")]
    pub ddsp: Option<String>,
    /// M-Profile Vector Extension (Helium); valid values: [DmveEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dmve")]
    pub dmve: Option<String>,
    /// Pointer Authentication and Branch Target Identification; valid values: [DpacbtiEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dpacbti")]
    pub dpacbti: Option<String>,
    /// TrustZone support; valid values: [DtzEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dtz")]
    pub dtz: Option<String>,
    /// Endianness; valid values: [DendianEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dendian")]
    pub dendian: Option<String>,
    /// Maximum processor clock frequency in Hz
    #[serde(rename = "Dclock")]
    pub dclock: Option<u64>,
}

/// Represents a [PDSC compile](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_compile) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Compile {
    /// Processor instance name for multi-core devices
    #[serde(rename = "Pname")]
    pub pname: Option<String>,
    /// Include file path injected into all projects
    pub header: Option<String>,
    /// Preprocessor define injected into all projects
    pub define: Option<String>,
    /// Processor-specific preprocessor define
    #[serde(rename = "Pdefine")]
    pub pdefine: Option<String>,
}

/// Represents a [PDSC debug](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_debug) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FamilyDebug {
    /// Processor instance name
    #[serde(rename = "Pname")]
    pub pname: Option<String>,
    /// Processor unit index for multi-core devices
    #[serde(rename = "Punit")]
    pub punit: Option<u32>,
    /// Path to the SVD file describing the device registers
    pub svd: Option<String>,
    /// Debug port index
    #[serde(rename = "__dp")]
    pub dp: Option<u32>,
    /// Access port index (legacy; use `apid` when possible)
    #[serde(rename = "__ap")]
    pub ap: Option<u32>,
    /// Access port identifier
    #[serde(rename = "__apid")]
    pub apid: Option<u32>,
    /// Base address of the debug component
    pub address: Option<String>,
}

/// Represents a [PDSC debugconfig](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_debugconfig) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DebugConfig {
    /// Default debug interface (`swd`, `jtag`, `cjtag`)
    pub default: Option<String>,
    /// Default debug clock frequency in Hz
    pub clock: Option<u64>,
    /// SWJ-DP is available (supports both SWD and JTAG)
    #[serde(default, deserialize_with = "de_opt_bool")]
    pub swj: Option<bool>,
    /// Dormant state is supported
    #[serde(default, deserialize_with = "de_opt_bool")]
    pub dormant: Option<bool>,
}

/// Represents a [PDSC debugport](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_debugport) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DebugPort {
    /// Debug port index
    #[serde(rename = "__dp")]
    pub dp: u32,
    /// JTAG port configuration
    pub jtag: Option<DebugPortJtag>,
    /// SWD port configuration
    pub swd: Option<DebugPortSwd>,
    /// cJTAG port configuration
    pub cjtag: Option<DebugPortCjtag>,
}

/// JTAG debug port parameters
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DebugPortJtag {
    /// TAP index on the JTAG chain
    #[serde(rename = "tapindex")]
    pub tapindex: Option<u32>,
}

/// SWD debug port parameters
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DebugPortSwd {
    /// Maximum SWD clock in Hz (informational)
    pub clockmax: Option<u64>,
}

/// cJTAG debug port parameters
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DebugPortCjtag {
    /// Maximum cJTAG clock in Hz (informational)
    pub clockmax: Option<u64>,
}

/// Represents a [PDSC accessportV1](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_accessportV1) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AccessPortV1 {
    /// Access port identifier
    #[serde(rename = "__apid")]
    pub apid: u32,
    /// Parent debug port index
    #[serde(rename = "__dp")]
    pub dp: Option<u32>,
    /// APv1 index on the debug port
    pub index: u32,
}

/// Represents a [PDSC accessportV2](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_accessportV2) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AccessPortV2 {
    /// Access port identifier
    #[serde(rename = "__apid")]
    pub apid: u32,
    /// Parent debug port index
    #[serde(rename = "__dp")]
    pub dp: Option<u32>,
    /// Base address of the access port
    pub address: String,
    /// HPROT bus attribute bits
    #[serde(rename = "HPROT")]
    pub hprot: Option<u32>,
    /// Secure privileged access enable
    #[serde(rename = "SPROT")]
    pub sprot: Option<u32>,
    /// Parent access port identifier for hierarchical APv2
    pub parent: Option<u32>,
}

/// Represents a [PDSC algorithm](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_algorithm) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Algorithm {
    /// Path to the flash algorithm file (.FLM)
    pub name: String,
    /// Start address of the flash region (hex string, e.g. `"0x00000000"`)
    pub start: String,
    /// Size of the flash region in bytes (hex string, e.g. `"0x00010000"`)
    pub size: String,
    /// Start address of the RAM buffer used by the algorithm
    #[serde(rename = "RAMstart")]
    pub ram_start: Option<String>,
    /// Size of the RAM buffer in bytes
    #[serde(rename = "RAMsize")]
    pub ram_size: Option<String>,
    /// If `true`, this is the default algorithm for the device
    #[serde(default, deserialize_with = "de_opt_bool")]
    pub default: Option<bool>,
    /// Algorithm style (`Keil` or `IAR`)
    pub style: Option<String>,
    /// Processor instance name this algorithm applies to
    #[serde(rename = "Pname")]
    pub pname: Option<String>,
}

/// Represents a [PDSC flashinfo](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_flashinfo) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FlashInfo {
    /// Path to the flash device description file
    pub name: Option<String>,
    /// Start address of the flash device (hex string)
    pub start: String,
    /// Flash page size in bytes (hex string)
    pub pagesize: Option<String>,
    /// Erased-byte value (hex string, usually `"0xFF"`)
    pub blankval: Option<String>,
    /// Fill byte value for gaps (hex string)
    pub filler: Option<String>,
    /// Page program time in microseconds
    pub ptime: Option<u32>,
    /// Sector erase time in microseconds
    pub etime: Option<u32>,
    /// Processor instance name this info applies to
    #[serde(rename = "Pname")]
    pub pname: Option<String>,
    /// Contiguous flash blocks (0..*)
    #[serde(rename = "block", default)]
    pub blocks: Vec<FlashBlock>,
    /// Gaps between flash regions (0..*)
    #[serde(rename = "gap", default)]
    pub gaps: Vec<FlashGap>,
}

/// A contiguous block within a flash device
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FlashBlock {
    /// Count of sectors in this block
    #[serde(deserialize_with = "de_uint")]
    pub count: u32,
    /// Sector size in bytes (hex string)
    pub size: String,
}

/// A gap between flash regions
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FlashGap {
    /// Count of gap sectors
    #[serde(deserialize_with = "de_uint")]
    pub count: u32,
    /// Gap sector size in bytes (hex string)
    pub size: String,
}

/// Represents a [PDSC memory](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_memory) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Memory {
    /// Unique name for this memory region
    pub name: Option<String>,
    /// Access permissions (`r`, `rw`, `rwx`, etc.)
    pub access: Option<String>,
    /// Start address of the memory region (hex string)
    pub start: String,
    /// Size of the memory region in bytes (hex string)
    pub size: String,
    /// If `true`, this is the default memory region
    #[serde(default, deserialize_with = "de_opt_bool")]
    pub default: Option<bool>,
    /// If `true`, this region contains the startup code
    #[serde(default, deserialize_with = "de_opt_bool")]
    pub startup: Option<bool>,
    /// If `true`, do not zero-initialise this region
    #[serde(default, deserialize_with = "de_opt_bool")]
    pub uninit: Option<bool>,
    /// Name of the memory region this region aliases
    pub alias: Option<String>,
    /// Processor instance name this region applies to
    #[serde(rename = "Pname")]
    pub pname: Option<String>,
}

/// Represents a [PDSC trace](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_trace) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Trace {
    /// Processor instance name
    #[serde(rename = "Pname")]
    pub pname: Option<String>,
    /// Debug port index
    #[serde(rename = "__dp")]
    pub dp: Option<u32>,
    /// Access port identifier for the trace funnel
    #[serde(rename = "__apid")]
    pub apid: Option<u32>,
    /// Base address of the trace component
    pub address: Option<String>,
    /// Trace type (`ETM`, `MTB`, `SWO`, `TPIU`)
    #[serde(rename = "type")]
    pub trace_type: Option<String>,
}

/// Represents a [PDSC book](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_book) element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Book {
    /// Path or URL to the document
    pub name: String,
    /// Human-readable document title
    pub title: String,
    /// Processor instance name this book applies to
    #[serde(rename = "Pname")]
    pub pname: Option<String>,
    /// If `true`, this document is public
    #[serde(default, deserialize_with = "de_opt_bool")]
    pub public: Option<bool>,
}

/// Represents a [PDSC feature](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_feature) element;
/// valid `feature_type` values: [DeviceFeatureEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Feature {
    /// Feature type identifier
    #[serde(rename = "type")]
    pub feature_type: String,
    /// Primary parameter — numeric for most types, label string for `Application`
    pub n: Option<String>,
    /// Secondary parameter — numeric for most types, label string for `Application`
    pub m: Option<String>,
    /// Human-readable feature name
    pub name: Option<String>,
    /// Processor instance name this feature applies to
    #[serde(rename = "Pname")]
    pub pname: Option<String>,
}

/// Represents an [environment](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_environment) entry within a family
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FamilyEnvironment {
    /// Tool environment identifier
    pub name: String,
    /// Processor instance name
    #[serde(rename = "Pname")]
    pub pname: Option<String>,
}

/// Represents a [PDSC variant](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_variant) element
///
/// Defines a named variant of a device, overriding or extending the parent device's properties.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Variant {
    /// Variant name
    #[serde(rename = "Dvariant")]
    pub variant_name: String,
    /// Brief variant description
    pub description: Option<String>,
    /// Processor definitions (0..*)
    #[serde(rename = "processor", default)]
    pub processor: Vec<Processor>,
    /// Compile-time definitions (0..*)
    #[serde(rename = "compile", default)]
    pub compile: Vec<Compile>,
    /// Debug unit assignments (0..*)
    #[serde(rename = "debug", default)]
    pub debug: Vec<FamilyDebug>,
    /// Debug configuration (0..1)
    pub debugconfig: Option<DebugConfig>,
    /// Debug port definitions (0..*)
    #[serde(rename = "debugport", default)]
    pub debugport: Vec<DebugPort>,
    /// Access port v1 definitions (0..*)
    #[serde(rename = "accessportV1", default)]
    pub access_port_v1: Vec<AccessPortV1>,
    /// Access port v2 definitions (0..*)
    #[serde(rename = "accessportV2", default)]
    pub access_port_v2: Vec<AccessPortV2>,
    /// Flash programming algorithms (0..*)
    #[serde(rename = "algorithm", default)]
    pub algorithm: Vec<Algorithm>,
    /// Flash information (0..*)
    #[serde(rename = "flashinfo", default)]
    pub flashinfo: Vec<FlashInfo>,
    /// Memory regions (0..*)
    #[serde(rename = "memory", default)]
    pub memory: Vec<Memory>,
    /// Trace unit definitions (0..*)
    #[serde(rename = "trace", default)]
    pub trace: Vec<Trace>,
    /// Reference documentation (0..*)
    #[serde(rename = "book", default)]
    pub book: Vec<Book>,
    /// Feature descriptors (0..*)
    #[serde(rename = "feature", default)]
    pub feature: Vec<Feature>,
    /// Tool environment entries (0..*)
    #[serde(rename = "environment", default)]
    pub environment: Vec<FamilyEnvironment>,
}

/// Represents a [PDSC device](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_device) element
#[derive(Debug, PartialEq, Deserialize)]
pub struct Device<'a> {
    /// Device name
    #[serde(rename = "Dname")]
    pub device_name: String,
    /// Brief device description
    pub description: Option<String>,
    /// Processor definitions (0..*)
    #[serde(rename = "processor", default)]
    pub processor: Vec<Processor>,
    /// Compile-time definitions (0..*)
    #[serde(rename = "compile", default)]
    pub compile: Vec<Compile>,
    /// Debug unit assignments (0..*)
    #[serde(rename = "debug", default)]
    pub debug: Vec<FamilyDebug>,
    /// Debug configuration (0..1)
    pub debugconfig: Option<DebugConfig>,
    /// Debug port definitions (0..*)
    #[serde(rename = "debugport", default)]
    pub debugport: Vec<DebugPort>,
    /// Access port v1 definitions (0..*)
    #[serde(rename = "accessportV1", default)]
    pub access_port_v1: Vec<AccessPortV1>,
    /// Access port v2 definitions (0..*)
    #[serde(rename = "accessportV2", default)]
    pub access_port_v2: Vec<AccessPortV2>,
    /// Flash programming algorithms (0..*)
    #[serde(rename = "algorithm", default)]
    pub algorithm: Vec<Algorithm>,
    /// Flash information (0..*)
    #[serde(rename = "flashinfo", default)]
    pub flashinfo: Vec<FlashInfo>,
    /// Memory regions (0..*)
    #[serde(rename = "memory", default)]
    pub memory: Vec<Memory>,
    /// Trace unit definitions (0..*)
    #[serde(rename = "trace", default)]
    pub trace: Vec<Trace>,
    /// Reference documentation (0..*)
    #[serde(rename = "book", default)]
    pub book: Vec<Book>,
    /// Feature descriptors (0..*)
    #[serde(rename = "feature", default)]
    pub feature: Vec<Feature>,
    /// Tool environment entries (0..*)
    #[serde(rename = "environment", default)]
    pub environment: Vec<FamilyEnvironment>,
    /// Device-level debug variables (rare; most packs define these at family level)
    #[serde(default)]
    pub debugvars: Debugvars,
    /// Device-level debug sequences (rare; most packs define these at family level)
    #[serde(borrow, default)]
    pub sequences: Sequences<'a>,
    /// Device variants (0..*)
    #[serde(rename = "variant", default)]
    pub variants: Vec<Variant>,
}

/// Represents a [PDSC subFamily](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_subFamily) element
#[derive(Debug, PartialEq, Deserialize)]
pub struct SubFamily<'a> {
    /// Sub-family name
    #[serde(rename = "DsubFamily")]
    pub sub_family_name: String,
    /// Brief sub-family description
    pub description: Option<String>,
    /// Processor definitions (0..*)
    #[serde(rename = "processor", default)]
    pub processor: Vec<Processor>,
    /// Compile-time definitions (0..*)
    #[serde(rename = "compile", default)]
    pub compile: Vec<Compile>,
    /// Debug unit assignments (0..*)
    #[serde(rename = "debug", default)]
    pub debug: Vec<FamilyDebug>,
    /// Debug configuration (0..1)
    pub debugconfig: Option<DebugConfig>,
    /// Debug port definitions (0..*)
    #[serde(rename = "debugport", default)]
    pub debugport: Vec<DebugPort>,
    /// Access port v1 definitions (0..*)
    #[serde(rename = "accessportV1", default)]
    pub access_port_v1: Vec<AccessPortV1>,
    /// Access port v2 definitions (0..*)
    #[serde(rename = "accessportV2", default)]
    pub access_port_v2: Vec<AccessPortV2>,
    /// Flash programming algorithms (0..*)
    #[serde(rename = "algorithm", default)]
    pub algorithm: Vec<Algorithm>,
    /// Flash information (0..*)
    #[serde(rename = "flashinfo", default)]
    pub flashinfo: Vec<FlashInfo>,
    /// Memory regions (0..*)
    #[serde(rename = "memory", default)]
    pub memory: Vec<Memory>,
    /// Trace unit definitions (0..*)
    #[serde(rename = "trace", default)]
    pub trace: Vec<Trace>,
    /// Reference documentation (0..*)
    #[serde(rename = "book", default)]
    pub book: Vec<Book>,
    /// Feature descriptors (0..*)
    #[serde(rename = "feature", default)]
    pub feature: Vec<Feature>,
    /// Tool environment entries (0..*)
    #[serde(rename = "environment", default)]
    pub environment: Vec<FamilyEnvironment>,
    /// Sub-family-level debug variables
    #[serde(default)]
    pub debugvars: Debugvars,
    /// Sub-family-level debug sequences
    #[serde(borrow, default)]
    pub sequences: Sequences<'a>,
    /// Devices within this sub-family (0..*)
    #[serde(rename = "device", default)]
    pub devices: Vec<Device<'a>>,
}

#[derive(Debug, PartialEq, Deserialize, Default)]
/// Represents [PDSC sequences](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_sequences)
pub struct Sequences<'a> {
    /// Trace setup configuration
    #[serde(rename = "traceSetup")]
    pub trace_setup: Option<TraceSetup>,

    /// Raw XML nodes representing debug sequences
    ///
    /// These are stored as [RawNode] due to [serde_roxmltree] not supporting decoding elements as
    /// a vector of enums and we neet do be able to preresent both [control](SequenceElement::Control)
    /// and [block](SequenceElement::Block) elements with their order perserved.
    #[serde(rename = "sequence", default)]
    #[serde(borrow)]
    pub raw_nodes: Vec<RawNode<'a>>,

    /// Debug Sequences
    #[serde(skip)]
    pub sequences: Vec<Sequence>
}

impl<'a> Sequences<'a> {
    /// Iterates through the raw nodes and parses the sequences
    pub fn parse_raw_nodes_content(&self) -> Vec<Sequence> {
        self.raw_nodes.iter().map(|node| {
            node.0.try_into().expect("Failed to parse sequence node")
        }).collect()
    }

    /// Parses the raw XML Sequence nodes and stores the parsed sequences in [Self::sequences]
    pub fn parse_sequences(&mut self) {
        let sequences = Self::parse_raw_nodes_content(self);

        self.sequences = sequences;
    }
}


#[derive(Debug, PartialEq, Deserialize)]
/// Represents [PDSC Sequence](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_sequence)
pub struct Sequence {
    /// The sequence name
    pub name: String,

    /// Processor name, if set only use the debug sequence for this processor
    pub processor_name: Option<String>,

    /// If set disable the [Predefined Debug Access Sequence](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/debug_description.html#default_sequences) of the same name
    pub disable: Option<bool>,

    /// Descriptive text about the sequence
    pub info: Option<String>,

    #[serde(skip)]
    pub elements: Vec<SequenceElement>
}

impl<'a, 'input: 'a> TryFrom<Node<'a, 'input>> for Sequence {
    type Error = FamilyParseError;

    fn try_from(value: Node<'a, 'input>) -> Result<Self, Self::Error> {
        // Validate that this is a sequence node
        let node_name = value.tag_name().name();
        assert_eq!(node_name, "sequence");

        // Get the name
        let sequence_name = value.attribute("name")
            .ok_or_else(|| FamilyParseError::MissingAttribute("name".to_string()))?;

        // Get the optional attributes
        let sequence_processor_name = value.attribute("Pname")
            .map_or_else(
                || None,
                |v| Some(v.to_string())
            );
        let sequence_disable = {
            if let Some(v) = value.attribute("disable") {
                let disable_value: bool = v.parse().expect("Non boolean value in disable field");
                Some(disable_value)
            } else {
                None
            }
        };
        let sequence_info = value.attribute("info")
            .map_or_else(
                || None,
                |v| Some(v.to_string())
            );

        // The sequence elements
        let mut elements: Vec<SequenceElement> = Vec::new();

        // Try to parse the child nodes
        for child in value.children().filter(|c| c.is_element()) {
            let element: SequenceElement = child.try_into()?;

            elements.push(element);
        }

        Ok(Sequence {
            name: sequence_name.to_string(),
            processor_name: sequence_processor_name,
            disable: sequence_disable,
            info: sequence_info,
            elements
        })
    }
}

impl<'a, 'input: 'a> TryFrom<Node<'a, 'input>> for SequenceElement {
    type Error = FamilyParseError;

    fn try_from(value: Node<'a, 'input>) -> Result<Self, Self::Error> {
        if !value.is_element() {
            // Text/comment nodes are not sequence elements; content-parse is unimplemented
            return Err(FamilyParseError::UnimplementedContent);
        }
        match value.tag_name().name().to_lowercase().as_str() {
            "block" => Ok(
                <Node<'_, '_> as TryInto<SequenceBlock>>::try_into(value)?.into()
            ),
            "control" => Ok(
                <Node<'_, '_> as TryInto<SequenceControl>>::try_into(value)?.into()
            ),
            other => Err(FamilyParseError::UnknownElementType(other.to_string())),
        }
    }
}

impl<'a, 'input: 'a> TryFrom<Node<'a, 'input>> for SequenceBlock {
    type Error = FamilyParseError;

    fn try_from(value: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let mut block: Self = serde_roxmltree::from_node(value).unwrap();
        block.parse_statements();
        Ok(block)
    }
}

impl<'a, 'input: 'a> TryFrom<Node<'a, 'input>> for SequenceControl {
    type Error = FamilyParseError;

    fn try_from(value: Node<'a, 'input>) -> Result<Self, Self::Error> {
        // Use serde_roxmltree to parse the basic elements
        let mut block: Self = serde_roxmltree::from_node(value).unwrap();

        // Try to parse the child nodes
        for child in value.children().filter(|c| c.is_element()) {
            let element: SequenceElement = child.try_into()?;

            block.elements.push(element);
        }

        // Parse the conditional into an Expression
        let conditional_string: &str;
        if let Some(ref val) = block.conditional_if {
            conditional_string = val.as_str()
        } else if let Some(ref val) = block.conditional_while {
            conditional_string = val.as_str()
        } else {
            return Err(FamilyParseError::MissingAttribute(
                "conditional 'if' or 'while' attribute".to_string()
            ));
        };

        let conditional: debug_access::Expression = conditional_string.into();
        block.conditional = Some(conditional);

        Ok(block)
    }
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
 /// Represents the valid sequence child elements as defined in the "Child Elements" section of the [PDSC sequence element](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_sequence)
pub enum SequenceElement {
    Control(SequenceControl),
    Block(SequenceBlock),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
/// Represents a [PDSC Control Sequence](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_seq_control)
pub struct SequenceControl {
    /// If conditional
    #[serde(rename = "if")]
    pub conditional_if: Option<String>,

    /// While conditional
    #[serde(rename = "while")]
    pub conditional_while: Option<String>,

    /// Timeout in microseconds, a value of 0 is the same as None
    pub timeout: Option<u64>,

    /// Decsriptive text, e.g. for diagnostics
    pub info: Option<String>,

    #[serde(skip_deserializing)]
    /// The elements contained by the control block
    pub elements: Vec<SequenceElement>,

    #[serde(skip_deserializing)]
    /// The conditional parsed as an Expression
    ///
    /// The [Expression](debug_access::Expression) is wrapped in an [Option] due to
    /// provoding a default value for [serde] when deserializing. It should be safe
    /// to unwrap this value.
    pub conditional: Option<debug_access::Expression>
}

impl From<SequenceControl> for SequenceElement {
    fn from(value: SequenceControl) -> Self {
        SequenceElement::Control(value)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
/// Represents a [PDSC Block Sequence](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_seq_block)
pub struct SequenceBlock {
    /// If `Some(true)` the block must be executed atomically, see the description on CMSIS Pack website.
    pub atomic: Option<bool>,

    /// Decsriptive text, e.g. for diagnostics
    pub info: Option<String>,

    #[serde(rename = "#content")]
    /// Sequence block content
    pub content: String,

    #[serde(skip)]
    /// [Statement]s resulting from the parsing of [Self::content]
    pub statements: Vec<Statement>
}

impl SequenceBlock {
    /// Parses [Self::content] into a list of [Statement]s
    pub fn parse_statements_content(&self) -> Vec<Statement> {
        self.content.lines()
            .flat_map(|line| line.split(';'))
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| Statement::from(s.to_string()))
            .collect()
    }

    /// Parses the block content and stores the result in [Self::statements]
    pub fn parse_statements(&mut self) {
        self.statements = self.parse_statements_content();
    }
}

impl From<SequenceBlock> for SequenceElement {
    fn from(value: SequenceBlock) -> Self {
        SequenceElement::Block(value)
    }
}

#[cfg(test)]
mod tests {
    use std::default;

use roxmltree::Document;
use serde_roxmltree::RawNode;

use crate::{debug_access::{Assignment, DebugFunction, Expression, Statement::{self}}, pdsc::{Eccn, License, LicenseSet, Release, Releases, Repository}, family::{FamilyParseError, Sequence, SequenceBlock, SequenceControl, SequenceElement}};

    #[test]
    fn basic_sequence() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<sequence name="ResetSystem">
    <block>
        Sequence("ResetAndHalt");
    </block>
</sequence>"#;

        let document = Document::parse(xml_str).unwrap();
        let sequence_node = document.root_element();
        let raw_node: RawNode = RawNode(sequence_node);

        let sequence: Sequence = raw_node.0.try_into().unwrap();

        assert_eq!(sequence.name, "ResetSystem".to_string());
        assert_eq!(sequence.elements, vec![
            SequenceBlock {
                atomic: None,
                info: None,
                content: "\n        Sequence(\"ResetAndHalt\");\n    ".to_string(),
                statements: vec![
                    Statement::Expression(Expression::FunctionCall(Box::new(DebugFunction::Sequence {
                        name: Expression::Normal("\"ResetAndHalt\"".to_string())
                    })))
                ]
            }.into()
        ]);
    }

    #[test]
    /// Tests the example sequence from https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_sequence
    fn full_sequence() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<sequence name="UserSequence">
    <block info="Define variables and do debug accesses">
        __var tpWidth = (__traceout &amp; 0x003F0000) >> 16;
    </block>

    <control if="__traceout &amp; 0x2" info="Parallel Trace Port enabled">
        <block>
            // Do something generic for parallel trace port trace
        </block>

        <control if="tpWidth == 1" info="Configure device for 1-bit TPIU trace.">
            <block>
                // Do debug accesses
            </block>
        </control>

        <control if="tpWidth == 2" info="Configure device for 2-bit TPIU trace.">
            <block>
                // Do debug accesses
            </block>
        </control>

        <control if="tpWidth == 4" info="Configure device for 4-bit TPIU trace.">
            <block>
                // Do debug accesses
            </block>
        </control>
    </control>
</sequence>"#;

        let document = Document::parse(xml_str).unwrap();
        let sequence_node = document.root_element();
        let raw_node: RawNode = RawNode(sequence_node);

        let sequence: Sequence = raw_node.0.try_into().unwrap();

        // Check basic info
        assert_eq!(sequence.name, "UserSequence".to_string());
        assert_eq!(sequence.disable, None);
        assert_eq!(sequence.info, None);
        assert_eq!(sequence.processor_name, None);

        // Define a repeated element for future use
        let debug_access_block: SequenceElement = SequenceBlock {
            atomic: None,
            info: None,
            content: r#"
                // Do debug accesses
            "#.to_string(),
            statements: vec![
                Statement::Comment("// Do debug accesses".to_string())
            ]
        }.into();

        // Check that the elements are correct and in the correct order
        let expected_elements: Vec<SequenceElement> = vec![
            SequenceBlock {
                atomic: None,
                info: Some("Define variables and do debug accesses".to_string()),
                content: r#"
        __var tpWidth = (__traceout & 0x003F0000) >> 16;
    "#.to_string(),
                statements: vec![
                    Statement::Definition(Assignment {
                        variable: "tpWidth".to_string(),
                        expression: Expression::Normal("(__traceout & 0x003F0000) >> 16".to_string())
                    })
                ]
            }.into(),
            SequenceControl {
                conditional_if: Some("__traceout & 0x2".to_string()),
                conditional_while: None,
                timeout: None,
                info: Some("Parallel Trace Port enabled".to_string()),
                elements: vec![
                    SequenceBlock {
                        atomic: None,
                        info: None,
                        content: r#"
            // Do something generic for parallel trace port trace
        "#.to_string(),
                        statements: vec![
                            Statement::Comment("// Do something generic for parallel trace port trace".to_string())
                        ]
                    }.into(),
                    SequenceControl {
                        conditional_if: Some("tpWidth == 1".to_string()),
                        conditional_while: None,
                        timeout: None,
                        info: Some("Configure device for 1-bit TPIU trace.".to_string()),
                        elements: vec![
                            debug_access_block.clone().into()
                        ],
                        conditional: Some(
                            Expression::Normal("tpWidth == 1".to_string())
                        )
                    }.into(),
                    SequenceControl {
                        conditional_if: Some("tpWidth == 2".to_string()),
                        conditional_while: None,
                        timeout: None,
                        info: Some("Configure device for 2-bit TPIU trace.".to_string()),
                        elements: vec![
                            debug_access_block.clone().into()
                        ],
                        conditional: Some(
                            Expression::Normal("tpWidth == 2".to_string())
                        )
                    }.into(),
                    SequenceControl {
                        conditional_if: Some("tpWidth == 4".to_string()),
                        conditional_while: None,
                        timeout: None,
                        info: Some("Configure device for 4-bit TPIU trace.".to_string()),
                        elements: vec![
                            debug_access_block.clone().into()
                        ],
                        conditional: Some(
                            Expression::Normal("tpWidth == 4".to_string())
                        )
                    }.into()
                ],
                conditional: Some(Expression::Normal("__traceout & 0x2".to_string()))
            }.into()
        ];

        println!("Expected: {:#?}", expected_elements);
        println!("Actual: {:#?}", sequence.elements);

        assert_eq!(sequence.elements, expected_elements);

    }

    #[test]
    fn basic_sequence_block() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<block info="Define condition variales for later use in block elements.">
    // Variable definition by __var keyword
    __var doIfBlock      = 1;
    __var whileCondition = 1;
</block>"#;

        let document = Document::parse(xml_str).unwrap();
        let sequence_node = document.root_element();
        let raw_node: RawNode = RawNode(sequence_node);

        let block: SequenceBlock = raw_node.0.try_into().unwrap();

        assert_eq!(block.info, Some("Define condition variales for later use in block elements.".to_string()));
        assert_eq!(block.atomic, None);
        assert_eq!(block.content, r#"
    // Variable definition by __var keyword
    __var doIfBlock      = 1;
    __var whileCondition = 1;
"#
        );
        assert_eq!(block.statements, vec![
            Statement::Comment("// Variable definition by __var keyword".to_string()),
            Statement::Definition(Assignment {
                variable: "doIfBlock".to_string(),
                expression: Expression::Normal("1".to_string())
            }),
            Statement::Definition(Assignment {
                variable: "whileCondition".to_string(),
                expression: Expression::Normal("1".to_string())
            })
        ]);
    }

    #[test]
    fn parse_control_element_if() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<control if="doIfBlock">
    <block>
        // Do debug accesses
    </block>
</control>"#;

        let document = Document::parse(xml_str).unwrap();
        let sequence_node = document.root_element();
        let raw_node: RawNode = RawNode(sequence_node);

        let block: SequenceControl = raw_node.0.try_into().unwrap();

        println!("{:#?}", block);
        assert_eq!(block.info, None);
        assert_eq!(block.conditional_if, Some("doIfBlock".to_string()));
        assert_eq!(block.conditional_while, None);
        assert_eq!(block.timeout, None);
        assert_eq!(block.elements, vec![
            SequenceBlock{
                atomic: None,
                info: None,
                content: "\n        // Do debug accesses\n    ".to_string(),
                statements: vec![
                    Statement::Comment("// Do debug accesses".to_string())
                ]
            }.into()
        ]);
    }

    #[test]
    fn sequence_missing_name_attribute() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<sequence>
    <block>Write32(0x40000000, 0x1);</block>
</sequence>"#;
        let document = Document::parse(xml_str).unwrap();
        let node = document.root_element();
        let result: Result<Sequence, _> = node.try_into();
        assert_eq!(result.unwrap_err(), FamilyParseError::MissingAttribute("name".to_string()));
    }

    #[test]
    fn sequence_element_unknown_tag() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<unknown/>"#;
        let document = Document::parse(xml_str).unwrap();
        let node = document.root_element();
        let result: Result<SequenceElement, _> = node.try_into();
        assert_eq!(result.unwrap_err(), FamilyParseError::UnknownElementType("unknown".to_string()));
    }

    #[test]
    fn debugvar_missing_var_prefix() {
        use crate::family::Debugvars;
        let result = Debugvars::parse_single_debugvar("  myVar = 0x1;");
        assert!(matches!(result, Err(FamilyParseError::MalformedDebugvar(_))));
    }

    #[test]
    fn debugvar_empty_segment() {
        use crate::family::Debugvars;
        let result = Debugvars::parse_single_debugvar("   ");
        assert!(matches!(result, Err(FamilyParseError::MalformedDebugvar(_))));
    }

    #[test]
    fn parse_control_element_while() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<control while="whileCondition" timeout="5000">
    <block>
        // Execute while "whileCondition" different from '0' with a timeout of 5ms
        whileCondition = 0;
    </block>
</control>"#;

        let document = Document::parse(xml_str).unwrap();
        let sequence_node = document.root_element();
        let raw_node: RawNode = RawNode(sequence_node);

        let block: SequenceControl = raw_node.0.try_into().unwrap();

        println!("{:#?}", block);
        assert_eq!(block.info, None);
        assert_eq!(block.conditional_if, None);
        assert_eq!(block.conditional_while, Some("whileCondition".to_string()));
        assert_eq!(block.timeout, Some(5000));
        assert_eq!(block.elements, vec![
            SequenceBlock{
                atomic: None,
                info: None,
                content: r#"
        // Execute while "whileCondition" different from '0' with a timeout of 5ms
        whileCondition = 0;
    "#.to_string(),
                statements: vec![
                    Statement::Comment("// Execute while \"whileCondition\" different from '0' with a timeout of 5ms".to_string()),
                    Statement::Assignment(Assignment {
                        variable: "whileCondition".to_string(),
                        expression: Expression::Normal("0".to_string())
                    })
                ]
            }.into()
        ]);
    }

    #[test]
    fn parse_family_with_description() {
        // Family with a description element
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <description>STM32F1 Arm Cortex-M3 microcontroller series</description>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.description, Some("STM32F1 Arm Cortex-M3 microcontroller series".to_string()));
        assert_eq!(family.processor.len(), 0);
    }

    #[test]
    fn parse_family_processor() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="LPC" Dvendor="NXP:11">
    <processor Pname="Cortex-M3" Dcore="Cortex-M3" Dfpu="NO_FPU" Dmpu="NO_MPU" Dclock="120000000"/>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.processor.len(), 1);
        let p = &family.processor[0];
        assert_eq!(p.pname, Some("Cortex-M3".to_string()));
        assert_eq!(p.dcore, Some("Cortex-M3".to_string()));
        assert_eq!(p.dfpu, Some("NO_FPU".to_string()));
        assert_eq!(p.dclock, Some(120_000_000));
    }

    #[test]
    fn parse_family_algorithm() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <algorithm name="Flash/STM32F1xx_128.FLM" start="0x08000000" size="0x00020000"
               RAMstart="0x20000000" RAMsize="0x00001000" default="true"/>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.algorithm.len(), 1);
        let a = &family.algorithm[0];
        assert_eq!(a.name, "Flash/STM32F1xx_128.FLM");
        assert_eq!(a.start, "0x08000000");
        assert_eq!(a.size, "0x00020000");
        assert_eq!(a.ram_start, Some("0x20000000".to_string()));
        assert_eq!(a.ram_size, Some("0x00001000".to_string()));
        assert_eq!(a.default, Some(true));
    }

    #[test]
    fn parse_family_memory() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <memory name="IROM1" access="rx" start="0x08000000" size="0x00020000" startup="true" default="true"/>
    <memory name="IRAM1" access="rw" start="0x20000000" size="0x00005000" uninit="true" default="true"/>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.memory.len(), 2);
        let rom = &family.memory[0];
        assert_eq!(rom.name, Some("IROM1".to_string()));
        assert_eq!(rom.access, Some("rx".to_string()));
        assert_eq!(rom.start, "0x08000000");
        assert_eq!(rom.startup, Some(true));
        assert_eq!(rom.default, Some(true));
        let ram = &family.memory[1];
        assert_eq!(ram.uninit, Some(true));
    }

    #[test]
    fn parse_family_book() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <book name="Docs/DM00031936.pdf" title="STM32F1 Reference Manual"/>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.book.len(), 1);
        assert_eq!(family.book[0].name, "Docs/DM00031936.pdf");
        assert_eq!(family.book[0].title, "STM32F1 Reference Manual");
        assert_eq!(family.book[0].public, None);
    }

    #[test]
    fn parse_family_feature() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <feature type="UART" n="3" name="Universal Asynchronous Receiver/Transmitter"/>
    <feature type="ADC" n="1" m="12"/>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.feature.len(), 2);
        assert_eq!(family.feature[0].feature_type, "UART");
        assert_eq!(family.feature[0].n, Some("3".to_string()));
        assert_eq!(family.feature[0].name, Some("Universal Asynchronous Receiver/Transmitter".to_string()));
        assert_eq!(family.feature[1].m, Some("12".to_string()));
    }

    #[test]
    fn parse_family_debug_and_debugconfig() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <debug svd="SVD/STM32F103xx.svd" __dp="0" __ap="0"/>
    <debugconfig default="swd" clock="5000000" swj="true"/>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.debug.len(), 1);
        assert_eq!(family.debug[0].svd, Some("SVD/STM32F103xx.svd".to_string()));
        assert_eq!(family.debug[0].dp, Some(0));
        assert_eq!(family.debug[0].ap, Some(0));
        let dc = family.debugconfig.as_ref().unwrap();
        assert_eq!(dc.default, Some("swd".to_string()));
        assert_eq!(dc.clock, Some(5_000_000));
        assert_eq!(dc.swj, Some(true));
    }

    #[test]
    fn parse_family_debugport() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <debugport __dp="0">
        <swd/>
        <jtag tapindex="0"/>
    </debugport>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.debugport.len(), 1);
        let dp = &family.debugport[0];
        assert_eq!(dp.dp, 0);
        assert!(dp.swd.is_some());
        assert!(dp.jtag.is_some());
        assert_eq!(dp.jtag.as_ref().unwrap().tapindex, Some(0));
        assert!(dp.cjtag.is_none());
    }

    #[test]
    fn parse_family_access_ports() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <accessportV1 __apid="0" __dp="0" index="0"/>
    <accessportV2 __apid="1" __dp="0" address="0xE0041000"/>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.access_port_v1.len(), 1);
        assert_eq!(family.access_port_v1[0].apid, 0);
        assert_eq!(family.access_port_v1[0].dp, Some(0));
        assert_eq!(family.access_port_v1[0].index, 0);
        assert_eq!(family.access_port_v2.len(), 1);
        assert_eq!(family.access_port_v2[0].apid, 1);
        assert_eq!(family.access_port_v2[0].address, "0xE0041000");
    }

    #[test]
    fn parse_family_compile() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <compile header="Include/stm32f1xx.h" define="STM32F1"/>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.compile.len(), 1);
        assert_eq!(family.compile[0].header, Some("Include/stm32f1xx.h".to_string()));
        assert_eq!(family.compile[0].define, Some("STM32F1".to_string()));
    }

    #[test]
    fn parse_family_flashinfo() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <flashinfo start="0x08000000" pagesize="0x400" blankval="0xFF">
        <block count="128" size="0x400"/>
    </flashinfo>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.flashinfo.len(), 1);
        let fi = &family.flashinfo[0];
        assert_eq!(fi.start, "0x08000000");
        assert_eq!(fi.pagesize, Some("0x400".to_string()));
        assert_eq!(fi.blankval, Some("0xFF".to_string()));
        assert_eq!(fi.blocks.len(), 1);
        assert_eq!(fi.blocks[0].count, 128);
        assert_eq!(fi.blocks[0].size, "0x400");
    }

    #[test]
    fn parse_family_trace() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <trace __dp="0" __apid="1" address="0xE0041000" type="ETM"/>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.trace.len(), 1);
        assert_eq!(family.trace[0].dp, Some(0));
        assert_eq!(family.trace[0].apid, Some(1));
        assert_eq!(family.trace[0].address, Some("0xE0041000".to_string()));
        assert_eq!(family.trace[0].trace_type, Some("ETM".to_string()));
    }

    #[test]
    fn parse_family_environment() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="STM32F1" Dvendor="STMicroelectronics:13">
    <environment name="uv" Pname="Core0"/>
    <debugvars></debugvars>
    <sequences/>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.environment.len(), 1);
        assert_eq!(family.environment[0].name, "uv");
        assert_eq!(family.environment[0].pname, Some("Core0".to_string()));
    }

    #[test]
    fn parse_device_basic() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<device Dname="PIC32CM1216PL10028">
    <processor Dcore="Cortex-M0+" Dendian="Little-endian" Dmpu="NO_MPU" Dfpu="NO_FPU"
               Ddsp="NO_DSP" Dtz="NO_TZ" Dmve="NO_MVE" Dclock="24000000" DcoreVersion="r0p0"/>
    <memory name="FLASH" start="0x0C000000" size="0x20000" access="rx" default="1" startup="1"/>
    <memory name="HSRAM" start="0x20000000" size="0x4000" default="1" access="rwx"/>
</device>"#;
        let document = Document::parse(xml_str).unwrap();
        let device: crate::family::Device = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(device.device_name, "PIC32CM1216PL10028");
        assert_eq!(device.processor.len(), 1);
        assert_eq!(device.processor[0].dcore, Some("Cortex-M0+".to_string()));
        assert_eq!(device.processor[0].dclock, Some(24000000));
        assert_eq!(device.memory.len(), 2);
        assert_eq!(device.memory[0].name, Some("FLASH".to_string()));
        assert_eq!(device.memory[0].start, "0x0C000000");
        assert_eq!(device.memory[1].name, Some("HSRAM".to_string()));
        assert_eq!(device.variants.len(), 0);
    }

    #[test]
    fn parse_device_with_variant() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<device Dname="LPC1768">
    <processor Dcore="Cortex-M3" Dclock="100000000"/>
    <variant Dvariant="LPC1768FBD100">
        <memory name="FLASH" start="0x00000000" size="0x80000" access="rx"/>
    </variant>
    <variant Dvariant="LPC1768FET100">
        <processor Dcore="Cortex-M3" Dclock="120000000"/>
    </variant>
</device>"#;
        let document = Document::parse(xml_str).unwrap();
        let device: crate::family::Device = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(device.device_name, "LPC1768");
        assert_eq!(device.variants.len(), 2);
        assert_eq!(device.variants[0].variant_name, "LPC1768FBD100");
        assert_eq!(device.variants[0].memory.len(), 1);
        assert_eq!(device.variants[1].variant_name, "LPC1768FET100");
        assert_eq!(device.variants[1].processor[0].dclock, Some(120000000));
    }

    #[test]
    fn parse_subfamily_with_devices() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<subFamily DsubFamily="LPC176x">
    <processor Dcore="Cortex-M3"/>
    <device Dname="LPC1768">
        <processor Dclock="100000000"/>
        <memory name="FLASH" start="0x00000000" size="0x80000" access="rx"/>
    </device>
    <device Dname="LPC1766">
        <processor Dclock="100000000"/>
        <memory name="FLASH" start="0x00000000" size="0x40000" access="rx"/>
    </device>
</subFamily>"#;
        let document = Document::parse(xml_str).unwrap();
        let sf: crate::family::SubFamily = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(sf.sub_family_name, "LPC176x");
        assert_eq!(sf.processor.len(), 1);
        assert_eq!(sf.processor[0].dcore, Some("Cortex-M3".to_string()));
        assert_eq!(sf.devices.len(), 2);
        assert_eq!(sf.devices[0].device_name, "LPC1768");
        assert_eq!(sf.devices[0].memory[0].size, "0x80000");
        assert_eq!(sf.devices[1].device_name, "LPC1766");
    }

    #[test]
    fn parse_family_with_direct_devices() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<family Dfamily="PIC32CM-PL" Dvendor="Microchip:3">
    <debugvars configfile="debug.dbgconf" version="1.0.0">__var myVar = 0x1;</debugvars>
    <sequences/>
    <device Dname="PIC32CM1216PL10028">
        <processor Dcore="Cortex-M0+" Dclock="24000000"/>
        <debugconfig default="swd" clock="2000000"/>
        <compile header="pic32cm1216pl/include/pic32c.h" define="__PIC32CM1216PL10028__"/>
        <memory name="FLASH" start="0x0C000000" size="0x20000" access="rx"/>
        <algorithm name="keil/Flash/PIC32CM-PL_FLASH_128.FLM" start="0x0C000000" size="0x20000"
                   RAMstart="0x20000000" RAMsize="0x2000" default="1" style="Keil"/>
    </device>
    <device Dname="PIC32CM2532PL10028">
        <processor Dcore="Cortex-M0+" Dclock="24000000"/>
        <memory name="FLASH" start="0x0C000000" size="0x40000" access="rx"/>
    </device>
</family>"#;
        let document = Document::parse(xml_str).unwrap();
        let family: crate::family::Family = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(family.device_family, "PIC32CM-PL");
        assert_eq!(family.devices.len(), 2);
        assert_eq!(family.sub_families.len(), 0);

        let d0 = &family.devices[0];
        assert_eq!(d0.device_name, "PIC32CM1216PL10028");
        assert_eq!(d0.processor[0].dcore, Some("Cortex-M0+".to_string()));
        assert_eq!(d0.debugconfig.as_ref().unwrap().default, Some("swd".to_string()));
        assert_eq!(d0.compile[0].header, Some("pic32cm1216pl/include/pic32c.h".to_string()));
        assert_eq!(d0.memory[0].name, Some("FLASH".to_string()));
        assert_eq!(d0.algorithm[0].name, "keil/Flash/PIC32CM-PL_FLASH_128.FLM");
        assert_eq!(d0.algorithm[0].default, Some(true));

        let d1 = &family.devices[1];
        assert_eq!(d1.device_name, "PIC32CM2532PL10028");
        assert_eq!(d1.memory[0].size, "0x40000");
    }
}