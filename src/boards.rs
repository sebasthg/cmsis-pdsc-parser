//! Contains the types required to represent a [PDSC Boards](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_boards) element

use serde::{Deserialize, Deserializer, Serialize};

/// Deserializes a `u64` from either a hex string (`"0x..."`) or a decimal string.
fn deserialize_hex_u64<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    let s = String::deserialize(d)?;
    s.strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .map_or_else(
            || s.parse::<u64>().map_err(serde::de::Error::custom),
            |hex| u64::from_str_radix(hex, 16).map_err(serde::de::Error::custom),
        )
}

/// Called only when the optional attribute is present; wraps the parsed value in `Some`.
fn deserialize_opt_hex_u64<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u64>, D::Error> {
    deserialize_hex_u64(d).map(Some)
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC boards](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_boards) element
pub struct Boards {
    /// The list of board descriptions
    #[serde(rename = "board")]
    pub boards: Vec<Board>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC board](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board) element
pub struct Board {
    /// Board vendor name
    pub vendor: String,

    /// Development board name
    pub name: String,

    /// Board revision suited for the BSP
    pub revision: Option<String>,

    /// 128-bit embedded debugger firmware identifier (format: 8-4-4-4-12)
    pub uuid: Option<String>,

    /// Email or webpage for sales enquiries
    #[serde(rename = "salesContact")]
    pub sales_contact: Option<String>,

    /// Webpage for ordering the board
    #[serde(rename = "orderForm")]
    pub order_form: Option<String>,

    /// Brief board description (max 256 characters); optional per spec (0..1)
    pub description: Option<String>,

    /// Board features and capabilities (1..*)
    #[serde(rename = "feature", default)]
    pub features: Vec<Feature>,

    /// Microcontroller devices mounted on the board (1..*)
    #[serde(rename = "mountedDevice", default)]
    pub mounted_devices: Vec<MountedDevice>,

    /// Microcontroller devices compatible with the board (1..*)
    #[serde(rename = "compatibleDevice", default)]
    pub compatible_devices: Vec<CompatibleDevice>,

    /// Non-MCU parts mounted on the board (0..*)
    #[serde(rename = "mountedPart", default)]
    pub mounted_parts: Vec<MountedPart>,

    /// Board images (top/bottom/perspective)
    pub image: Option<Image>,

    /// On-board debug interface capabilities (0..*)
    #[serde(rename = "debugInterface", default)]
    pub debug_interfaces: Vec<DebugInterface>,

    /// Documentation files (0..*)
    #[serde(rename = "book", default)]
    pub books: Vec<Book>,

    /// On-board debug probe configuration
    #[serde(rename = "debugProbe")]
    pub debug_probe: Option<DebugProbe>,

    /// Additional board memory regions (0..*)
    #[serde(rename = "memory", default)]
    pub memories: Vec<Memory>,

    /// Flash programming algorithms for board memory (0..*)
    #[serde(rename = "algorithm", default)]
    pub algorithms: Vec<Algorithm>,

    /// IDE-specific tool environments for this board (0..*)
    pub environments: Option<BoardEnvironments>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC board feature](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board_feature) element
pub struct Feature {
    /// Predefined board feature type (e.g. `LED`, `Button`, `XTAL`, `USB`, `Ethernet`); valid values: [BoardFeatureType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html)
    #[serde(rename = "type")]
    pub feature_type: String,

    /// Quantity or primary numeric parameter; meaning depends on `feature_type`
    pub n: Option<String>,

    /// Secondary numeric parameter; meaning depends on `feature_type`
    pub m: Option<String>,

    /// Descriptive feature name
    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC mountedDevice](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board_mountedDevice) element
pub struct MountedDevice {
    /// Device index for boards with multiple devices
    #[serde(rename = "deviceIndex")]
    pub device_index: Option<String>,

    /// Device vendor (use `"NO_VENDOR:0"` if there is no MCU); valid values: [DeviceVendorEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html)
    #[serde(rename = "Dvendor")]
    pub device_vendor: String,

    /// Device name (use `"NO_MCU"` if there is no MCU)
    #[serde(rename = "Dname")]
    pub device_name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC compatibleDevice](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board_compatibleDevice) element
pub struct CompatibleDevice {
    /// Device index for multi-device boards
    #[serde(rename = "deviceIndex")]
    pub device_index: Option<String>,

    /// Device vendor (use `"NO_VENDOR:0"` for incompatible configurations); valid values: [DeviceVendorEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html)
    #[serde(rename = "Dvendor")]
    pub device_vendor: String,

    /// Device name or wildcard pattern
    #[serde(rename = "Dname")]
    pub device_name: Option<String>,

    /// Device family name
    #[serde(rename = "Dfamily")]
    pub device_family: Option<String>,

    /// Device sub-family name
    #[serde(rename = "DsubFamily")]
    pub device_sub_family: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC mountedPart](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board_mountedPart) element
pub struct MountedPart {
    /// Quantity of parts with this name and vendor
    pub n: String,

    /// Part vendor name
    #[serde(rename = "Hvendor")]
    pub part_vendor: String,

    /// Part name
    #[serde(rename = "Hname")]
    pub part_name: String,

    /// Exact commercial part name (variant)
    #[serde(rename = "Hvariant")]
    pub part_variant: Option<String>,

    /// Part revision
    #[serde(rename = "Hrevision")]
    pub part_revision: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC board image](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board_image) element
pub struct Image {
    /// Path to the large top-side board image
    pub large: Option<String>,

    /// Path to the small top-side board image (lower resolution)
    pub small: Option<String>,

    /// Path to the bottom-side board image
    pub bottom: Option<String>,

    /// Path to a perspective-view board image
    pub perspective: Option<String>,

    /// Publishing permission; default `true`
    pub public: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC debugInterface](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board_debugInterface) element
pub struct DebugInterface {
    /// Debug adapter type (e.g. `CMSIS-DAP`, `JTAG/SW`)
    pub adapter: Option<String>,

    /// Physical connector description
    pub connector: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC board book](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board_book) element
pub struct Book {
    /// Documentation category (e.g. `setup`, `schematic`, `manual`, `other`)
    pub category: Option<String>,

    /// Document file path or external URL
    pub name: Option<String>,

    /// Display title for the document
    pub title: Option<String>,

    /// Publishing permission; default `true`
    pub public: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC debugProbe](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board_debugProbe) element
pub struct DebugProbe {
    /// Device index for the probe on multi-device boards
    #[serde(rename = "deviceIndex")]
    pub device_index: Option<String>,

    /// Probe type (e.g. `CMSIS-DAP`, `DAP-Link`, `ST-Link`, `J-Link`)
    pub name: String,

    /// Probe firmware version
    pub version: String,

    /// Connection type: `jtag` or `swd`
    #[serde(rename = "debugLink")]
    pub debug_link: String,

    /// Default debug clock speed in Hz
    #[serde(rename = "debugClock", deserialize_with = "deserialize_hex_u64")]
    pub debug_clock: u64,

    /// Physical connector type (e.g. `Mini-USB`, `Micro-USB`, `USB-C`)
    pub connector: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC board memory](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board_memory) element
pub struct Memory {
    /// Processor identifier for multi-processor boards
    #[serde(rename = "Pname")]
    pub processor_name: Option<String>,

    /// Unique memory region name
    pub name: Option<String>,

    /// Access permissions (e.g. `rx`, `rw`, `rwx`)
    pub access: Option<String>,

    /// Base address of the memory region (hex or decimal)
    #[serde(deserialize_with = "deserialize_hex_u64")]
    pub start: u64,

    /// Size of the memory region in bytes (hex or decimal)
    #[serde(deserialize_with = "deserialize_hex_u64")]
    pub size: u64,

    /// Whether this is the general-purpose memory for the linker (default: `false`)
    pub default: Option<bool>,

    /// Whether startup code should be placed here (default: `false`)
    pub startup: Option<bool>,

    /// Whether the region should remain uninitialized (default: `false`)
    pub uninit: Option<bool>,

    /// Name of another memory region this region aliases
    pub alias: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC board algorithm](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board_algorithm) element
pub struct Algorithm {
    /// Processor identifier for multi-processor boards
    #[serde(rename = "Pname")]
    pub processor_name: Option<String>,

    /// Path to the flash programming algorithm file
    pub name: String,

    /// Base address of the flash region covered by this algorithm (hex or decimal)
    #[serde(deserialize_with = "deserialize_hex_u64")]
    pub start: u64,

    /// Size of the flash region covered by this algorithm in bytes (hex or decimal)
    #[serde(deserialize_with = "deserialize_hex_u64")]
    pub size: u64,

    /// RAM execution base address for the algorithm (hex or decimal)
    #[serde(
        rename = "RAMstart",
        default,
        deserialize_with = "deserialize_opt_hex_u64"
    )]
    pub ram_start: Option<u64>,

    /// Maximum RAM available for algorithm execution (hex or decimal)
    #[serde(
        rename = "RAMsize",
        default,
        deserialize_with = "deserialize_opt_hex_u64"
    )]
    pub ram_size: Option<u64>,

    /// Whether this is the default algorithm for the covered region (default: `false`)
    pub default: Option<bool>,

    /// Algorithm style; defaults to `Keil`
    pub style: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [board environment](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board) entry
pub struct BoardEnvironment {
    /// IDE environment name (e.g. `uvision`, `iar`, `eclipse`)
    pub name: String,

    /// Processor name for multi-core boards; limits this environment entry to one core
    #[serde(rename = "Pname")]
    pub processor_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Groups board environment entries for a [PDSC board](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_boards_pg.html#element_board)
pub struct BoardEnvironments {
    /// Individual environment entries (0..*)
    #[serde(rename = "environment", default)]
    pub environments: Vec<BoardEnvironment>,
}

#[cfg(test)]
mod tests {
    use crate::boards::{
        Algorithm, BoardEnvironment, Boards, Book, CompatibleDevice, DebugProbe, Feature, Image,
        Memory, MountedDevice, MountedPart,
    };

    #[test]
    fn parse_boards() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<boards>
    <board vendor="STMicroelectronics" name="NUCLEO-F401RE" revision="Rev.C">
        <description>STM32 Nucleo-64 development board with STM32F401RE MCU</description>
        <feature type="XTAL" n="8" name="High-speed crystal oscillator"/>
        <mountedDevice Dvendor="STMicroelectronics:13" Dname="STM32F401RETx"/>
        <compatibleDevice Dvendor="STMicroelectronics:13" Dname="STM32F401*"/>
        <image large="images/nucleo_large.png" small="images/nucleo_small.png"/>
        <book category="setup" name="docs/setup.pdf" title="Getting Started" public="true"/>
        <memory name="FLASH" access="rx" start="0x08000000" size="0x80000" default="true" startup="true"/>
        <algorithm name="Flash/STM32F4xx.FLM" start="0x08000000" size="0x80000" default="true"/>
    </board>
</boards>"#;

        let boards: Boards = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(boards.boards.len(), 1);

        let board = &boards.boards[0];
        assert_eq!(board.vendor, "STMicroelectronics");
        assert_eq!(board.name, "NUCLEO-F401RE");
        assert_eq!(board.revision, Some("Rev.C".to_string()));
        assert_eq!(
            board.description,
            Some("STM32 Nucleo-64 development board with STM32F401RE MCU".to_string())
        );
        assert_eq!(
            board.features,
            vec![Feature {
                feature_type: "XTAL".to_string(),
                n: Some("8".to_string()),
                m: None,
                name: Some("High-speed crystal oscillator".to_string()),
            }]
        );
        assert_eq!(
            board.mounted_devices,
            vec![MountedDevice {
                device_index: None,
                device_vendor: "STMicroelectronics:13".to_string(),
                device_name: "STM32F401RETx".to_string(),
            }]
        );
        assert_eq!(
            board.compatible_devices,
            vec![CompatibleDevice {
                device_index: None,
                device_vendor: "STMicroelectronics:13".to_string(),
                device_name: Some("STM32F401*".to_string()),
                device_family: None,
                device_sub_family: None,
            }]
        );
        assert_eq!(
            board.image,
            Some(Image {
                large: Some("images/nucleo_large.png".to_string()),
                small: Some("images/nucleo_small.png".to_string()),
                bottom: None,
                perspective: None,
                public: None,
            })
        );
        assert_eq!(
            board.books,
            vec![Book {
                category: Some("setup".to_string()),
                name: Some("docs/setup.pdf".to_string()),
                title: Some("Getting Started".to_string()),
                public: Some(true),
            }]
        );
        assert_eq!(
            board.memories,
            vec![Memory {
                processor_name: None,
                name: Some("FLASH".to_string()),
                access: Some("rx".to_string()),
                start: 0x08000000,
                size: 0x80000,
                default: Some(true),
                startup: Some(true),
                uninit: None,
                alias: None,
            }]
        );
        assert_eq!(
            board.algorithms,
            vec![Algorithm {
                processor_name: None,
                name: "Flash/STM32F4xx.FLM".to_string(),
                start: 0x08000000,
                size: 0x80000,
                ram_start: None,
                ram_size: None,
                default: Some(true),
                style: None,
            }]
        );
        assert_eq!(board.mounted_parts, vec![]);
        assert_eq!(board.debug_interfaces, vec![]);
        assert_eq!(board.debug_probe, None);
    }

    #[test]
    fn parse_board_minimal() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<boards>
    <board vendor="Example" name="MyBoard">
        <description>A minimal test board</description>
        <feature type="LED" n="2"/>
        <mountedDevice Dvendor="ARM:82" Dname="ARMCM0"/>
    </board>
</boards>"#;

        let boards: Boards = serde_roxmltree::from_str(xml_str).unwrap();
        let board = &boards.boards[0];

        assert_eq!(board.vendor, "Example");
        assert_eq!(board.name, "MyBoard");
        assert_eq!(board.revision, None);
        assert_eq!(board.uuid, None);
        assert_eq!(board.description, Some("A minimal test board".to_string()));
        assert_eq!(
            board.features,
            vec![Feature {
                feature_type: "LED".to_string(),
                n: Some("2".to_string()),
                m: None,
                name: None,
            }]
        );
        assert_eq!(
            board.mounted_devices,
            vec![MountedDevice {
                device_index: None,
                device_vendor: "ARM:82".to_string(),
                device_name: "ARMCM0".to_string(),
            }]
        );
        assert_eq!(board.compatible_devices, vec![]);
        assert_eq!(board.mounted_parts, vec![]);
        assert_eq!(board.books, vec![]);
        assert_eq!(board.memories, vec![]);
        assert_eq!(board.algorithms, vec![]);
        assert_eq!(board.image, None);
        assert_eq!(board.debug_probe, None);
    }

    #[test]
    fn parse_board_debug_probe() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<boards>
    <board vendor="ARM" name="MPS2" salesContact="support@arm.com">
        <description>ARM MPS2 FPGA prototyping board</description>
        <feature type="JTAG" n="1"/>
        <mountedDevice Dvendor="ARM:82" Dname="ARMCM3"/>
        <mountedPart n="1" Hvendor="Xilinx" Hname="XC7A200T" Hvariant="-1FBG484C"/>
        <debugProbe name="CMSIS-DAP" version="2.0" debugLink="swd" debugClock="10000000" connector="USB-C"/>
    </board>
</boards>"#;

        let boards: Boards = serde_roxmltree::from_str(xml_str).unwrap();
        let board = &boards.boards[0];

        assert_eq!(board.vendor, "ARM");
        assert_eq!(board.name, "MPS2");
        assert_eq!(board.sales_contact, Some("support@arm.com".to_string()));
        assert_eq!(
            board.mounted_parts,
            vec![MountedPart {
                n: "1".to_string(),
                part_vendor: "Xilinx".to_string(),
                part_name: "XC7A200T".to_string(),
                part_variant: Some("-1FBG484C".to_string()),
                part_revision: None,
            }]
        );
        assert_eq!(
            board.debug_probe,
            Some(DebugProbe {
                device_index: None,
                name: "CMSIS-DAP".to_string(),
                version: "2.0".to_string(),
                debug_link: "swd".to_string(),
                debug_clock: 10_000_000,
                connector: "USB-C".to_string(),
            })
        );
        assert_eq!(board.compatible_devices, vec![]);
        assert_eq!(board.memories, vec![]);
        assert_eq!(board.algorithms, vec![]);
    }

    #[test]
    fn parse_board_environments() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<boards>
    <board vendor="Example" name="EnvBoard">
        <description>Board with IDE environments</description>
        <mountedDevice Dvendor="ARM:82" Dname="ARMCM4"/>
        <environments>
            <environment name="uvision"/>
            <environment name="iar" Pname="Core0"/>
        </environments>
    </board>
</boards>"#;

        let boards: Boards = serde_roxmltree::from_str(xml_str).unwrap();
        let board = &boards.boards[0];

        assert_eq!(board.vendor, "Example");
        assert_eq!(board.name, "EnvBoard");
        let envs = board
            .environments
            .as_ref()
            .expect("environments should be present");
        assert_eq!(envs.environments.len(), 2);
        assert_eq!(
            envs.environments[0],
            BoardEnvironment {
                name: "uvision".to_string(),
                processor_name: None,
            }
        );
        assert_eq!(
            envs.environments[1],
            BoardEnvironment {
                name: "iar".to_string(),
                processor_name: Some("Core0".to_string()),
            }
        );
    }
}
