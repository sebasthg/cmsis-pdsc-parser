use std::io::Read;

use log::info;

use cmsis_pdsc_parser::{
    Package,
    family::{FlashBlock, FlashInfoElement},
    pdsc::{self, License, LicenseSet},
    requirements::{self, PackagesList},
};

const PDSC_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/PIC32CM_PL/pdsc/Microchip.PIC32CM-PL_DFP.pdsc"
);

const EXPECTED_DEBUGVARS: [(&str, u64); 26] = [
    ("AIRCR_Addr", 0xE000ED0C),
    ("DHCSR_Addr", 0xE000EDF0),
    ("DEMCR_Addr", 0xE000EDFC),
    ("DCRSR_Addr", 0xE000EDF4),
    ("DCRDR_Addr", 0xE000EDF8),
    ("DSU_BASE_ADDR", 0x41002000),
    /*
    ("DSU_STATUSA_ADDR", DSU_BASE_ADDR + 0x104),
    ("DSU_STATUSB_ADDR", DSU_BASE_ADDR + 0x108),
    ("DSU_DAL_ADDR", DSU_BASE_ADDR + 0x124),
    ("DSU_BCC0_ADDR", DSU_BASE_ADDR + 0x110),
    ("DSU_BCC1_ADDR", DSU_BASE_ADDR + 0x114),
    */
    ("DSU_STATUSB_BCCD1_BIT", 0x2),
    ("DSU_STATUSB_BCCD0_BIT", 0x1),
    ("CRSTEXTBIT", 0x100),
    ("BREXTBIT", 0x10000),
    ("BOOTROM_STATUS_INITCHECK_OK", 0x3),
    ("BOOTROM_STATUS_BOOTOK", 0x4),
    ("BOOTROM_STATUS_OK", 0x9),
    ("BOOTROM_STATUS_CMD_VALID", 0x5),
    ("BOOTROM_STATUS_CHALLENGE", 0xB),
    ("DEBUGGER_CMD_EXIT", 0x444247AA),
    ("DEBUGGER_CMD_IMODE", 0x44424755),
    ("NVMCTRL_INTFLAG_ADDR", 0x41004014),
    ("NVMCTRL_STATUS_ADDR", 0x4100401C),
    ("NVMCTRL_CTRLB_ADDR", 0x41004004),
    ("FP_CTRL_Addr", 0xE0002000),
    ("FP_COMP0_Addr", 0xE0002008),
    ("FPB_KEY", 0x00000002),
    ("FPB_ENABLE", 0x00000001),
    ("FPB_REPLACE", 0xC0000000),
    ("RAM_BUFFER", 0x20000000),
];

const EXPECTED_KEYWORDS: [&str; 34] = [
    "Microchip",
    "Device Family Package Microchip",
    "Device Support",
    "PIC32CM-PL",
    "AVR-DA",
    "AVR",
    "8-bit",
    "64KB Flash",
    "PTC",
    "Peripheral Touch Controller",
    "Capacitive Touch",
    "12-bit differential ADC",
    "Comparator",
    "Zero Cross Detect",
    "ZCD",
    "10-bit DAC",
    "PWM",
    "Timer/Counter",
    "MIPS",
    "24 MHz",
    "PLL",
    "32",
    "Bit",
    "Microcontrollers",
    "32-Bit Microcontrollers",
    "Touch",
    "MotorControl",
    "Functional Safety Ready",
    "Motor Control",
    "Microcontrollers And Processors",
    "UltraLowPower",
    "Ultra Low Power",
    "8",
    "8-Bit Microcontrollers",
];

fn main() {
    env_logger::init();

    let mut f = std::fs::File::open(PDSC_PATH).unwrap();
    let mut pdsc_content: String = String::new();
    f.read_to_string(&mut pdsc_content).unwrap();

    let document = roxmltree::Document::parse(&pdsc_content).unwrap();
    let pdsc = Package::new(&document).unwrap();

    info!("{:#?}", pdsc);

    // Validate mics package fields
    assert_eq!(pdsc.name, "PIC32CM-PL_DFP".to_string());
    assert_eq!(pdsc.vendor, "Microchip".to_string());
    assert_eq!(
        pdsc.description,
        pdsc::Description {
            overview: Some("./OVERVIEW.md".to_string()),
            content: Some("Microchip PIC32CM-PL Series Device Support".to_string())
        }
    );
    assert_eq!(pdsc.eccn, None);
    assert_eq!(
        pdsc.url,
        "https://packs.download.microchip.com/".to_string()
    );
    assert_eq!(
        pdsc.support_contact,
        Some("https://www.microchip.com/en-us/support".to_string())
    );
    assert_eq!(pdsc.license, Some("LICENSE.txt".to_string()));
    assert_eq!(pdsc.dominate, None);
    assert_eq!(pdsc.repository, None);
    assert_eq!(pdsc.changelogs, None);

    // Validate the license set
    let license_sets = pdsc.license_sets.unwrap().license_set;
    assert_eq!(
        license_sets,
        vec![LicenseSet {
            id: "all".to_string(),
            default: Some(true),
            gating: Some(false),
            license: vec![License {
                name: "LICENSE.txt".to_string(),
                title: "Apache License, Version 2.0".to_string(),
                spdx: Some("Apache-2.0".to_string()),
                url: Some("https://www.apache.org/licenses/LICENSE-2.0".to_string())
            }]
        }]
    );

    // Validate the requirements
    let requirements = pdsc.requirements.unwrap();
    assert_eq!(
        requirements.packages,
        Some(PackagesList {
            packages: vec![requirements::Package {
                name: "CMSIS".to_string(),
                vendor: "ARM".to_string(),
                version: None
            }]
        })
    );
    assert_eq!(requirements.compilers, None);
    assert_eq!(requirements.languages, None);
    assert_eq!(requirements.targets, None);

    // Validate the relases
    let releases = pdsc.releases;
    assert_eq!(releases.release.len(), 6);
    assert_eq!(releases.release[0], pdsc::Release {
        version: "1.5.437".to_string(),
        date: Some("2026-07-15".to_string()),
        content: r#"
      - Switched from single flashinfo with blocks and gaps to multiple discrete flashinfo entries for improved flash programming compatibility.
      - Added protection against accidental permanent device lock when programming DAL=0.
      - Fixed csolution blank project template to correctly locate cproject file.
      "#.to_string(),
        ..Default::default()
    });

    // Validate keywords
    let keywords = pdsc.keywords.unwrap().keywords;
    let expected_keywords: Vec<String> = EXPECTED_KEYWORDS.iter().map(|s| s.to_string()).collect();
    assert_eq!(keywords, expected_keywords);

    // Validate family info
    let family = &pdsc.devices.as_ref().unwrap().families[0];
    assert_eq!(&family.device_family, "PIC32CM-PL");
    assert_eq!(&family.vendor, "Microchip:3");
    assert_eq!(
        family.devices.len(),
        11,
        "expected 11 devices directly under family"
    );
    assert_eq!(
        family.sub_families.len(),
        0,
        "expected no subFamily elements"
    );

    // Validate debugvars
    let debugvars = &family.debugvars;
    assert_eq!(
        &debugvars.configfile,
        &Some("debug/PIC32CM-PL.dbgconf".to_string())
    );
    assert_eq!(&debugvars.version, &Some("1.0.0".to_string()));

    let parsed_debugvars = debugvars.parsed_debugvars.clone().unwrap();
    for (name, value) in EXPECTED_DEBUGVARS {
        let stored_value = parsed_debugvars
            .get(name)
            .expect(&format!("Failed to get debugvar {:?}", name))
            .to_owned();
        assert_eq!(stored_value, value)
    }

    // Validate flashinfo for the first device (PIC32CM1216PL10028)
    let device = &family.devices[0];
    assert_eq!(&device.device_name, "PIC32CM1216PL10028");
    assert_eq!(device.flashinfo.len(), 3);

    let flash = &device.flashinfo[0];
    assert_eq!(flash.name, "Flash");
    assert_eq!(flash.start, "0x0C000000");
    assert_eq!(flash.pagesize, "0x200");
    assert_eq!(flash.blankval, Some("0xFFFFFFFF".to_string()));
    assert_eq!(flash.ptime, Some(1_000_000));
    assert_eq!(flash.etime, Some(1_000_000));
    assert_eq!(
        flash.elements,
        vec![FlashInfoElement::Block(FlashBlock {
            count: 0x100,
            size: "0x200".to_string(),
            arg: None,
        })]
    );

    let romcfg = &device.flashinfo[1];
    assert_eq!(romcfg.name, "ROMCFG");
    assert_eq!(romcfg.start, "0x0D000000");
    assert_eq!(romcfg.pagesize, "0x100");
    assert_eq!(
        romcfg.elements,
        vec![FlashInfoElement::Block(FlashBlock {
            count: 0x1,
            size: "0x100".to_string(),
            arg: None,
        })]
    );

    let bootcfg = &device.flashinfo[2];
    assert_eq!(bootcfg.name, "BOOTCFG");
    assert_eq!(bootcfg.start, "0x0D000400");
    assert_eq!(bootcfg.pagesize, "0x100");
    assert_eq!(
        bootcfg.elements,
        vec![FlashInfoElement::Block(FlashBlock {
            count: 0x1,
            size: "0x100".to_string(),
            arg: None,
        })]
    );
}
