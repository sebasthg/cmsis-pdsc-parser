//! Contains the types required to represent a [PDSC Taxonomy](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_taxonomy.html#element_taxonomy) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC taxonomy](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_taxonomy.html#element_taxonomy) element
///
/// Groups `description` entries that define the component classes and group names used in a pack.
pub struct Taxonomy {
    /// Component class and group descriptions (1..*)
    #[serde(rename = "description", default)]
    pub descriptions: Vec<TaxonomyDescription>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC taxonomy description](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_taxonomy.html#element_taxonomyDescription) entry
///
/// Defines a component class or a class-and-group combination used to categorise components.
pub struct TaxonomyDescription {
    /// Component class name (e.g. `Device`, `CMSIS`, `Board Support`)
    #[serde(rename = "Cclass")]
    pub class: String,

    /// Component group name within the class
    #[serde(rename = "Cgroup")]
    pub group: Option<String>,

    /// Path or URL to documentation for this class/group
    pub doc: Option<String>,

    /// Generator identifier associated with this class/group
    pub generator: Option<String>,

    /// Publishing permission; default `true`
    pub public: Option<bool>,

    /// Human-readable description of the component class or group; empty string if absent
    #[serde(rename = "#content")]
    pub content: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// Predefined component class names per
/// [CclassType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_taxonomy.html#CclassType)
pub enum CclassType {
    Audio,
    BoardSupport,
    BoardPart,
    Compiler,
    /// "CMSIS"
    Cmsis,
    /// "CMSIS Driver"
    CmsisDriver,
    Device,
    DataExchange,
    ExtensionBoard,
    FileSystem,
    Graphics,
    /// "IoT Client"
    IotClient,
    /// "IoT Utility"
    IotUtility,
    Network,
    /// "RTOS"
    Rtos,
    Security,
    /// "USB"
    Usb,
    Utility,
}

impl TryFrom<&str> for CclassType {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Audio" => Ok(Self::Audio),
            "Board Support" => Ok(Self::BoardSupport),
            "Board Part" => Ok(Self::BoardPart),
            "Compiler" => Ok(Self::Compiler),
            "CMSIS" => Ok(Self::Cmsis),
            "CMSIS Driver" => Ok(Self::CmsisDriver),
            "Device" => Ok(Self::Device),
            "Data Exchange" => Ok(Self::DataExchange),
            "Extension Board" => Ok(Self::ExtensionBoard),
            "File System" => Ok(Self::FileSystem),
            "Graphics" => Ok(Self::Graphics),
            "IoT Client" => Ok(Self::IotClient),
            "IoT Utility" => Ok(Self::IotUtility),
            "Network" => Ok(Self::Network),
            "RTOS" => Ok(Self::Rtos),
            "Security" => Ok(Self::Security),
            "USB" => Ok(Self::Usb),
            "Utility" => Ok(Self::Utility),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for CclassType {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> { Self::try_from(s.as_str()) }
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
/// Predefined component group names per
/// [CgroupType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_taxonomy.html#CgroupType)
pub enum CgroupType {
    /// "CORE"
    Core,
    /// "DSP"
    Dsp,
    /// "NN Lib"
    NnLib,
    /// "RTOS"
    Rtos,
    /// "RTOS2"
    Rtos2,
    Startup,
}

impl TryFrom<&str> for CgroupType {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "CORE" => Ok(Self::Core),
            "DSP" => Ok(Self::Dsp),
            "NN Lib" => Ok(Self::NnLib),
            "RTOS" => Ok(Self::Rtos),
            "RTOS2" => Ok(Self::Rtos2),
            "Startup" => Ok(Self::Startup),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for CgroupType {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> { Self::try_from(s.as_str()) }
}

#[cfg(test)]
mod tests {
    use crate::taxonomy::{Taxonomy, TaxonomyDescription};

    #[test]
    fn parse_taxonomy() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<taxonomy>
    <description Cclass="Device" Cgroup="Startup" doc="Device/Doc/startup.htm"
                 generator="MyGen" public="true">ARM Cortex-M device startup</description>
    <description Cclass="Board Support"/>
</taxonomy>"#;

        let taxonomy: Taxonomy = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(taxonomy.descriptions.len(), 2);

        assert_eq!(taxonomy.descriptions[0], TaxonomyDescription {
            class: "Device".to_string(),
            group: Some("Startup".to_string()),
            doc: Some("Device/Doc/startup.htm".to_string()),
            generator: Some("MyGen".to_string()),
            public: Some(true),
            content: "ARM Cortex-M device startup".to_string(),
        });
        assert_eq!(taxonomy.descriptions[1], TaxonomyDescription {
            class: "Board Support".to_string(),
            group: None,
            doc: None,
            generator: None,
            public: None,
            content: "".to_string(),
        });
    }

    #[test]
    fn parse_taxonomy_minimal() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<taxonomy>
    <description Cclass="CMSIS"/>
</taxonomy>"#;

        let taxonomy: Taxonomy = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(taxonomy.descriptions.len(), 1);

        let desc = &taxonomy.descriptions[0];
        assert_eq!(desc.class, "CMSIS");
        assert_eq!(desc.group, None);
        assert_eq!(desc.doc, None);
        assert_eq!(desc.generator, None);
        assert_eq!(desc.public, None);
        assert_eq!(desc.content, "");
    }

    #[test]
    fn parse_taxonomy_content() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<taxonomy>
    <description Cclass="Compiler">ARM Compiler software components</description>
</taxonomy>"#;

        let taxonomy: Taxonomy = serde_roxmltree::from_str(xml_str).unwrap();
        let desc = &taxonomy.descriptions[0];

        assert_eq!(desc.class, "Compiler");
        assert_eq!(desc.group, None);
        assert_eq!(desc.content, "ARM Compiler software components");
    }

    #[test]
    fn cclass_type_try_from() {
        use crate::taxonomy::CclassType;

        assert_eq!(CclassType::try_from("Audio"), Ok(CclassType::Audio));
        assert_eq!(CclassType::try_from("CMSIS"), Ok(CclassType::Cmsis));
        assert_eq!(CclassType::try_from("CMSIS Driver"), Ok(CclassType::CmsisDriver));
        assert_eq!(CclassType::try_from("RTOS"), Ok(CclassType::Rtos));
        assert_eq!(CclassType::try_from("USB"), Ok(CclassType::Usb));
        assert_eq!(CclassType::try_from("IoT Client"), Ok(CclassType::IotClient));
        assert_eq!(CclassType::try_from("Unknown Class"), Err(()));

        let s = "Board Support".to_string();
        assert_eq!(CclassType::try_from(s), Ok(CclassType::BoardSupport));
    }

    #[test]
    fn cgroup_type_try_from() {
        use crate::taxonomy::CgroupType;

        assert_eq!(CgroupType::try_from("CORE"), Ok(CgroupType::Core));
        assert_eq!(CgroupType::try_from("DSP"), Ok(CgroupType::Dsp));
        assert_eq!(CgroupType::try_from("NN Lib"), Ok(CgroupType::NnLib));
        assert_eq!(CgroupType::try_from("RTOS"), Ok(CgroupType::Rtos));
        assert_eq!(CgroupType::try_from("RTOS2"), Ok(CgroupType::Rtos2));
        assert_eq!(CgroupType::try_from("Startup"), Ok(CgroupType::Startup));
        assert_eq!(CgroupType::try_from("Unknown Group"), Err(()));

        let s = "NN Lib".to_string();
        assert_eq!(CgroupType::try_from(s), Ok(CgroupType::NnLib));
    }
}
