//! Contains the types required to represent a [PDSC Part-Taxonomy](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_part-taxonomy.html#element_part-taxonomy) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC part-taxonomy](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_part-taxonomy.html#element_part-taxonomy) element
///
/// Groups `description` entries that define the hardware part classes and group names used in a pack.
pub struct PartTaxonomy {
    /// Hardware part class and group descriptions (1..*)
    #[serde(rename = "description", default)]
    pub descriptions: Vec<PartTaxonomyDescription>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC part-taxonomy description](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_part-taxonomy.html#element_part-taxonomyDescription) entry
///
/// Defines a hardware part class or a class-and-group combination used to categorise parts.
pub struct PartTaxonomyDescription {
    /// Hardware part class name (e.g. `Microcontroller`, `Memory`, `Sensor`)
    #[serde(rename = "Hclass")]
    pub class: String,

    /// Hardware part group name within the class
    #[serde(rename = "Hgroup")]
    pub group: Option<String>,

    /// Path or URL to documentation for this class/group
    pub doc: Option<String>,

    /// Generator identifier associated with this class/group
    pub generator: Option<String>,

    /// Publishing permission; default `true`
    pub public: Option<bool>,

    /// Condition ID that applies to this class/group
    pub condition: Option<String>,

    /// Human-readable description of the hardware part class or group; empty string if absent
    #[serde(rename = "#content")]
    pub content: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// Predefined hardware part class names per
/// [HclassType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_part-taxonomy.html#HclassType)
pub enum HclassType {
    AmplifiersAndComparators,
    /// "Audio ICs"
    AudioIcs,
    Automotive,
    ClocksAndTimers,
    DataConverters,
    /// "Digital Set-Top Box ICs"
    DigitalSetTopBoxIcs,
    DiodesAndRectifiers,
    ImagingAndPhotonicsDevices,
    InterfacesAndTransceivers,
    Memories,
    /// "MEMS and Sensors"
    MemsAndSensors,
    MotorDrivers,
    /// "NFC"
    Nfc,
    Other,
    Positioning,
    PowerManagement,
    PowerModules,
    PowerTransistors,
    Protections,
    RadioFrequency,
    /// "Reset and Supervisor ICs"
    ResetAndSupervisorIcs,
    /// "Secure MCUs"
    SecureMcus,
    /// "SiC Devices"
    SicDevices,
    SpaceDevices,
    SwitchesAndMultiplexers,
    /// "Thyristors and AC Switches"
    ThyristorsAndAcSwitches,
    TouchAndDisplayControllers,
    Wireless,
}

impl TryFrom<&str> for HclassType {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Amplifiers and Comparators" => Ok(Self::AmplifiersAndComparators),
            "Audio ICs" => Ok(Self::AudioIcs),
            "Automotive" => Ok(Self::Automotive),
            "Clocks and Timers" => Ok(Self::ClocksAndTimers),
            "Data Converters" => Ok(Self::DataConverters),
            "Digital Set-Top Box ICs" => Ok(Self::DigitalSetTopBoxIcs),
            "Diodes and Rectifiers" => Ok(Self::DiodesAndRectifiers),
            "Imaging and Photonics Devices" => Ok(Self::ImagingAndPhotonicsDevices),
            "Interfaces and Transceivers" => Ok(Self::InterfacesAndTransceivers),
            "Memories" => Ok(Self::Memories),
            "MEMS and Sensors" => Ok(Self::MemsAndSensors),
            "Motor Drivers" => Ok(Self::MotorDrivers),
            "NFC" => Ok(Self::Nfc),
            "Other" => Ok(Self::Other),
            "Positioning" => Ok(Self::Positioning),
            "Power Management" => Ok(Self::PowerManagement),
            "Power Modules" => Ok(Self::PowerModules),
            "Power Transistors" => Ok(Self::PowerTransistors),
            "Protections" => Ok(Self::Protections),
            "Radio Frequency" => Ok(Self::RadioFrequency),
            "Reset and Supervisor ICs" => Ok(Self::ResetAndSupervisorIcs),
            "Secure MCUs" => Ok(Self::SecureMcus),
            "SiC Devices" => Ok(Self::SicDevices),
            "Space Devices" => Ok(Self::SpaceDevices),
            "Switches and Multiplexers" => Ok(Self::SwitchesAndMultiplexers),
            "Thyristors and AC Switches" => Ok(Self::ThyristorsAndAcSwitches),
            "Touch and Display Controllers" => Ok(Self::TouchAndDisplayControllers),
            "Wireless" => Ok(Self::Wireless),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for HclassType {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// Predefined hardware part group names per
/// [HgroupType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_part-taxonomy.html#HgroupType)
pub enum HgroupType {
    Comparators,
    CurrentSensing,
    OpAmp,
    PowerOpAmp,
    VideoAmplifiers,
    Amplifiers,
    /// "MEMS Sensor"
    MemsSensor,
    Processors,
    /// "Sound Terminal ICs"
    SoundTerminalIcs,
    /// "ADAS (Advanced Driver Assistance Systems)"
    Adas,
    AnalogAndPower,
    InfotainmentAndTelematics,
    /// "Logic ICs"
    LogicIcs,
    Microcontrollers,
    /// "RTC"
    Rtc,
    Timers,
    /// "A2D - D2A"
    A2dD2a,
    /// "Isolated ADCs"
    IsolatedAdcs,
    /// "Metering ICs"
    MeteringIcs,
    DemodulatorsAndTuners,
    SetTopBoxProcessors,
    Diodes,
    Rectifiers,
    AmbientLightSensors,
    ImageSensors,
    ImagingProcessors,
    TimeOfFlightSensors,
    Interfaces,
    /// "I/O Expanders"
    IoExpanders,
    LevelTranslators,
    Transceivers,
    /// "EEPROM"
    Eeprom,
    Flash,
    /// "RAM"
    Ram,
    Environmental,
    Infrared,
    /// "MEMS Hybrid"
    MemsHybrid,
    /// "MEMS Microphone"
    MemsMicrophone,
    /// "MEMS Motion"
    MemsMotion,
    Proximity,
    Brushed,
    Brushless,
    GateDrivers,
    Stepper,
    Secure,
    Reader,
    Tag,
    TagAndReader,
    /// "GNSS ICs"
    GnssIcs,
    /// "GNSS Modules"
    GnssModules,
    /// "AC-DC Converters"
    AcDcConverters,
    /// "Battery Management ICs"
    BatteryManagementIcs,
    DcDcSwitchingConverters,
    DisplaySuppliesAndControllers,
    /// "eFuses and hot-swap ICs"
    EFusesAndHotSwapIcs,
    EnergyHarvesting,
    /// "GaN Power ICs"
    GanPowerIcs,
    HighDensityPowerDrivers,
    IntelligentPowerSwitches,
    /// "LED Drivers"
    LedDrivers,
    /// "Lighting ICs"
    LightingIcs,
    LinearVoltageRegulators,
    /// "LNB Supplies"
    LnbSupplies,
    /// "Photovoltaic ICs"
    PhotovoltaicIcs,
    /// "Power Over Ethernet ICs"
    PowerOverEthernetIcs,
    VoltageReferences,
    /// "Wireless Charger ICs"
    WirelessChargerIcs,
    /// "ACEPACK"
    Acepack,
    /// "SLLIMM"
    Sllimm,
    /// "IGBTs"
    Igbts,
    Bipolar,
    /// "MOSFETs"
    Mosfets,
    WideBandgap,
    /// "ASIP"
    Asip,
    /// "EMI Filters"
    EmiFilters,
    /// "TSS"
    Tss,
    /// "TVS"
    Tvs,
    /// "RF DMOS"
    RfDmos,
    /// "RF LDMOS"
    RfLdmos,
    MicroprocessorSupervisors,
    OnOffControllers,
    ResetAndVoltageDetectors,
    /// "Smart Reset ICs"
    SmartResetIcs,
    /// "Voltage Protection ICs"
    VoltageProtectionIcs,
    WatchdogTimers,
    Authentication,
    /// "SIM"
    Sim,
    /// "SiC Diodes"
    SicDiodes,
    /// "SiC MOSFETs"
    SicMosfets,
    /// "LEO Rad-Hard ICs"
    LeoRadHardIcs,
    /// "Rad-Hard Analog ICs"
    RadHardAnalogIcs,
    /// "Rad-Hard ASIC Platforms"
    RadHardAsicPlatforms,
    RadHardDiscretes,
    RadHardInterfaces,
    /// "Rad-Hard Logic ICs"
    RadHardLogicIcs,
    RadHardPowerManagement,
    Switches,
    Multiplexers,
    Thyristors,
    Triacs,
    LongRange,
    /// "RF Front-end"
    RfFrontEnd,
    /// "RF Solutions"
    RfSolutions,
    ShortRange,
}

impl TryFrom<&str> for HgroupType {
    type Error = ();
    #[allow(clippy::too_many_lines)]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Comparators" => Ok(Self::Comparators),
            "Current Sensing" => Ok(Self::CurrentSensing),
            "Op Amp" => Ok(Self::OpAmp),
            "Power Op Amp" => Ok(Self::PowerOpAmp),
            "Video Amplifiers" => Ok(Self::VideoAmplifiers),
            "Amplifiers" => Ok(Self::Amplifiers),
            "MEMS Sensor" => Ok(Self::MemsSensor),
            "Processors" => Ok(Self::Processors),
            "Sound Terminal ICs" => Ok(Self::SoundTerminalIcs),
            "ADAS (Advanced Driver Assistance Systems)" => Ok(Self::Adas),
            "Analog and Power" => Ok(Self::AnalogAndPower),
            "Infotainment and Telematics" => Ok(Self::InfotainmentAndTelematics),
            "Logic ICs" => Ok(Self::LogicIcs),
            "Microcontrollers" => Ok(Self::Microcontrollers),
            "RTC" => Ok(Self::Rtc),
            "Timers" => Ok(Self::Timers),
            "A2D - D2A" => Ok(Self::A2dD2a),
            "Isolated ADCs" => Ok(Self::IsolatedAdcs),
            "Metering ICs" => Ok(Self::MeteringIcs),
            "Demodulators and Tuners" => Ok(Self::DemodulatorsAndTuners),
            "Set-Top-Box Processors" => Ok(Self::SetTopBoxProcessors),
            "Diodes" => Ok(Self::Diodes),
            "Rectifiers" => Ok(Self::Rectifiers),
            "Ambient Light Sensors" => Ok(Self::AmbientLightSensors),
            "Image Sensors" => Ok(Self::ImageSensors),
            "Imaging Processors" => Ok(Self::ImagingProcessors),
            "Time-of-Flight Sensors" => Ok(Self::TimeOfFlightSensors),
            "Interfaces" => Ok(Self::Interfaces),
            "I/O Expanders" => Ok(Self::IoExpanders),
            "Level Translators" => Ok(Self::LevelTranslators),
            "Transceivers" => Ok(Self::Transceivers),
            "EEPROM" => Ok(Self::Eeprom),
            "Flash" => Ok(Self::Flash),
            "RAM" => Ok(Self::Ram),
            "Environmental" => Ok(Self::Environmental),
            "Infrared" => Ok(Self::Infrared),
            "MEMS Hybrid" => Ok(Self::MemsHybrid),
            "MEMS Microphone" => Ok(Self::MemsMicrophone),
            "MEMS Motion" => Ok(Self::MemsMotion),
            "Proximity" => Ok(Self::Proximity),
            "Brushed" => Ok(Self::Brushed),
            "Brushless" => Ok(Self::Brushless),
            "Gate Drivers" => Ok(Self::GateDrivers),
            "Stepper" => Ok(Self::Stepper),
            "Secure" => Ok(Self::Secure),
            "Reader" => Ok(Self::Reader),
            "Tag" => Ok(Self::Tag),
            "Tag and Reader" => Ok(Self::TagAndReader),
            "GNSS ICs" => Ok(Self::GnssIcs),
            "GNSS Modules" => Ok(Self::GnssModules),
            "AC-DC Converters" => Ok(Self::AcDcConverters),
            "Battery Management ICs" => Ok(Self::BatteryManagementIcs),
            "DC-DC Switching Converters" => Ok(Self::DcDcSwitchingConverters),
            "Display Supplies and Controllers" => Ok(Self::DisplaySuppliesAndControllers),
            "eFuses and hot-swap ICs" => Ok(Self::EFusesAndHotSwapIcs),
            "Energy Harvesting" => Ok(Self::EnergyHarvesting),
            "GaN Power ICs" => Ok(Self::GanPowerIcs),
            "High-Density Power Drivers" => Ok(Self::HighDensityPowerDrivers),
            "Intelligent Power Switches" => Ok(Self::IntelligentPowerSwitches),
            "LED Drivers" => Ok(Self::LedDrivers),
            "Lighting ICs" => Ok(Self::LightingIcs),
            "Linear Voltage Regulators" => Ok(Self::LinearVoltageRegulators),
            "LNB Supplies" => Ok(Self::LnbSupplies),
            "Photovoltaic ICs" => Ok(Self::PhotovoltaicIcs),
            "Power Over Ethernet ICs" => Ok(Self::PowerOverEthernetIcs),
            "Voltage References" => Ok(Self::VoltageReferences),
            "Wireless Charger ICs" => Ok(Self::WirelessChargerIcs),
            "ACEPACK" => Ok(Self::Acepack),
            "SLLIMM" => Ok(Self::Sllimm),
            "IGBTs" => Ok(Self::Igbts),
            "Bipolar" => Ok(Self::Bipolar),
            "MOSFETs" => Ok(Self::Mosfets),
            "Wide Bandgap" => Ok(Self::WideBandgap),
            "ASIP" => Ok(Self::Asip),
            "EMI Filters" => Ok(Self::EmiFilters),
            "TSS" => Ok(Self::Tss),
            "TVS" => Ok(Self::Tvs),
            "RF DMOS" => Ok(Self::RfDmos),
            "RF LDMOS" => Ok(Self::RfLdmos),
            "Microprocessor Supervisors" => Ok(Self::MicroprocessorSupervisors),
            "On-Off Controllers" => Ok(Self::OnOffControllers),
            "Reset and Voltage Detectors" => Ok(Self::ResetAndVoltageDetectors),
            "Smart Reset ICs" => Ok(Self::SmartResetIcs),
            "Voltage Protection ICs" => Ok(Self::VoltageProtectionIcs),
            "Watchdog Timers" => Ok(Self::WatchdogTimers),
            "Authentication" => Ok(Self::Authentication),
            "SIM" => Ok(Self::Sim),
            "SiC Diodes" => Ok(Self::SicDiodes),
            "SiC MOSFETs" => Ok(Self::SicMosfets),
            "LEO Rad-Hard ICs" => Ok(Self::LeoRadHardIcs),
            "Rad-Hard Analog ICs" => Ok(Self::RadHardAnalogIcs),
            "Rad-Hard ASIC Platforms" => Ok(Self::RadHardAsicPlatforms),
            "Rad-Hard Discretes" => Ok(Self::RadHardDiscretes),
            "Rad-Hard Interfaces" => Ok(Self::RadHardInterfaces),
            "Rad-Hard Logic ICs" => Ok(Self::RadHardLogicIcs),
            "Rad-Hard Power Management" => Ok(Self::RadHardPowerManagement),
            "Switches" => Ok(Self::Switches),
            "Multiplexers" => Ok(Self::Multiplexers),
            "Thyristors" => Ok(Self::Thyristors),
            "Triacs" => Ok(Self::Triacs),
            "Long Range" => Ok(Self::LongRange),
            "RF Front-end" => Ok(Self::RfFrontEnd),
            "RF Solutions" => Ok(Self::RfSolutions),
            "Short Range" => Ok(Self::ShortRange),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for HgroupType {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::part_taxonomy::{PartTaxonomy, PartTaxonomyDescription};

    #[test]
    fn parse_part_taxonomy() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<part-taxonomy>
    <description Hclass="Microcontroller" Hgroup="ARM Cortex-M" doc="Docs/MCU/index.html"
                 generator="MyGen" public="true" condition="MyCondition">ARM Cortex-M microcontrollers</description>
    <description Hclass="Memory"/>
</part-taxonomy>"#;

        let pt: PartTaxonomy = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(pt.descriptions.len(), 2);

        assert_eq!(
            pt.descriptions[0],
            PartTaxonomyDescription {
                class: "Microcontroller".to_string(),
                group: Some("ARM Cortex-M".to_string()),
                doc: Some("Docs/MCU/index.html".to_string()),
                generator: Some("MyGen".to_string()),
                public: Some(true),
                condition: Some("MyCondition".to_string()),
                content: "ARM Cortex-M microcontrollers".to_string(),
            }
        );
        assert_eq!(
            pt.descriptions[1],
            PartTaxonomyDescription {
                class: "Memory".to_string(),
                group: None,
                doc: None,
                generator: None,
                public: None,
                condition: None,
                content: "".to_string(),
            }
        );
    }

    #[test]
    fn parse_part_taxonomy_minimal() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<part-taxonomy>
    <description Hclass="Sensor"/>
</part-taxonomy>"#;

        let pt: PartTaxonomy = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(pt.descriptions.len(), 1);

        let desc = &pt.descriptions[0];
        assert_eq!(desc.class, "Sensor");
        assert_eq!(desc.group, None);
        assert_eq!(desc.doc, None);
        assert_eq!(desc.generator, None);
        assert_eq!(desc.public, None);
        assert_eq!(desc.content, "");
    }

    #[test]
    fn parse_part_taxonomy_content() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<part-taxonomy>
    <description Hclass="Wireless">Wireless connectivity modules</description>
</part-taxonomy>"#;

        let pt: PartTaxonomy = serde_roxmltree::from_str(xml_str).unwrap();
        let desc = &pt.descriptions[0];

        assert_eq!(desc.class, "Wireless");
        assert_eq!(desc.group, None);
        assert_eq!(desc.content, "Wireless connectivity modules");
    }

    #[test]
    fn hclass_type_try_from() {
        use crate::part_taxonomy::HclassType;

        assert_eq!(HclassType::try_from("Wireless"), Ok(HclassType::Wireless));
        assert_eq!(
            HclassType::try_from("MEMS and Sensors"),
            Ok(HclassType::MemsAndSensors)
        );
        assert_eq!(HclassType::try_from("NFC"), Ok(HclassType::Nfc));
        assert_eq!(
            HclassType::try_from("SiC Devices"),
            Ok(HclassType::SicDevices)
        );
        assert_eq!(
            HclassType::try_from("Thyristors and AC Switches"),
            Ok(HclassType::ThyristorsAndAcSwitches)
        );
        assert_eq!(HclassType::try_from("Unknown Class"), Err(()));

        let s = "Secure MCUs".to_string();
        assert_eq!(HclassType::try_from(s), Ok(HclassType::SecureMcus));
    }

    #[test]
    fn hgroup_type_try_from() {
        use crate::part_taxonomy::HgroupType;

        assert_eq!(HgroupType::try_from("A2D - D2A"), Ok(HgroupType::A2dD2a));
        assert_eq!(
            HgroupType::try_from("ADAS (Advanced Driver Assistance Systems)"),
            Ok(HgroupType::Adas)
        );
        assert_eq!(
            HgroupType::try_from("eFuses and hot-swap ICs"),
            Ok(HgroupType::EFusesAndHotSwapIcs)
        );
        assert_eq!(
            HgroupType::try_from("I/O Expanders"),
            Ok(HgroupType::IoExpanders)
        );
        assert_eq!(
            HgroupType::try_from("SiC MOSFETs"),
            Ok(HgroupType::SicMosfets)
        );
        assert_eq!(
            HgroupType::try_from("RF Front-end"),
            Ok(HgroupType::RfFrontEnd)
        );
        assert_eq!(HgroupType::try_from("Unknown Group"), Err(()));

        let s = "LEO Rad-Hard ICs".to_string();
        assert_eq!(HgroupType::try_from(s), Ok(HgroupType::LeoRadHardIcs));
    }
}
