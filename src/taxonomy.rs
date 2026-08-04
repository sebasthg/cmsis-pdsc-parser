//! Contains the types required to represent a [PDSC Taxonomy](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_taxonomy.html#element_taxonomy) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC taxonomy](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_taxonomy.html#element_taxonomy) element
///
/// Groups `description` entries that define the component classes and group names used in a pack.
pub struct Taxonomy {
    /// Component class and group descriptions (1..*)
    #[serde(rename = "description", default)]
    pub descriptions: Vec<TaxonomyDescription>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
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
}
