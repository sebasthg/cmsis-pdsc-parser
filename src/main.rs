use std::io::Read;

use log::{debug, info};

const PDSC_PATH: &str = "Microchip.PIC32CM-PL_DFP.pdsc";

mod pdsc;
mod debug_access;


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
    ("RAM_BUFFER", 0x20000000)
];

fn main() {
    env_logger::init();

    let mut f = std::fs::File::open(PDSC_PATH).unwrap();
    let mut pdsc_content: String = String::new();
    f.read_to_string(&mut pdsc_content).unwrap();

    let document = roxmltree::Document::parse(&pdsc_content).unwrap();
    let pdsc = pdsc::Package::new(&document);

    debug!("{:#?}", pdsc);

    // Validate the desrciption field
    assert_eq!(pdsc.description, pdsc::Description {
        overview: Some("./OVERVIEW.md".to_string()),
        content: Some("Microchip PIC32CM-PL Series Device Support".to_string())
    });

    // Validate that we don't have an ECCN field
    assert_eq!(pdsc.eccn, None);

    // Validate family info
    let family = pdsc.devices.family;
    assert_eq!(&family.device_family, "PIC32CM-PL");
    assert_eq!(&family.vendor, "Microchip:3");

    // Validate debugvars
    let debugvars = &family.debugvars;
    assert_eq!(&debugvars.configfile, &Some("debug/PIC32CM-PL.dbgconf".to_string()));
    assert_eq!(&debugvars.version, &Some("1.0.0".to_string()));

    let parsed_debugvars = debugvars.parsed_debugvars.clone().unwrap();
    for (name, value) in EXPECTED_DEBUGVARS {
        let stored_value = parsed_debugvars.get(name).expect(&format!("Failed to get debugvar {:?}", name)).to_owned();
        assert_eq!(stored_value, value)
    }

}
