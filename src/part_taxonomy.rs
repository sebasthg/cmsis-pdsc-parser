//! Contains the types required to represent a [PDSC Part-Taxonomy](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_part-taxonomy.html#element_part-taxonomy) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC part-taxonomy](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_part-taxonomy.html#element_part-taxonomy) element
///
/// Groups `description` entries that define the hardware part classes and group names used in a pack.
pub struct PartTaxonomy {
    /// Hardware part class and group descriptions (1..*)
    #[serde(rename = "description", default)]
    pub descriptions: Vec<PartTaxonomyDescription>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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

    /// Human-readable description of the hardware part class or group; empty string if absent
    #[serde(rename = "#content")]
    pub content: String,
}

#[cfg(test)]
mod tests {
    use crate::part_taxonomy::{PartTaxonomy, PartTaxonomyDescription};

    #[test]
    fn parse_part_taxonomy() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<part-taxonomy>
    <description Hclass="Microcontroller" Hgroup="ARM Cortex-M" doc="Docs/MCU/index.html"
                 generator="MyGen" public="true">ARM Cortex-M microcontrollers</description>
    <description Hclass="Memory"/>
</part-taxonomy>"#;

        let pt: PartTaxonomy = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(pt.descriptions.len(), 2);

        assert_eq!(pt.descriptions[0], PartTaxonomyDescription {
            class: "Microcontroller".to_string(),
            group: Some("ARM Cortex-M".to_string()),
            doc: Some("Docs/MCU/index.html".to_string()),
            generator: Some("MyGen".to_string()),
            public: Some(true),
            content: "ARM Cortex-M microcontrollers".to_string(),
        });
        assert_eq!(pt.descriptions[1], PartTaxonomyDescription {
            class: "Memory".to_string(),
            group: None,
            doc: None,
            generator: None,
            public: None,
            content: "".to_string(),
        });
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
}
