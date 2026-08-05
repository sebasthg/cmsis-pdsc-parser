//! Contains the types required to represent a [PDSC Parts](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_parts_pg.html#element_parts) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC parts](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_parts_pg.html#element_parts) element
pub struct Parts {
    /// The list of hardware part descriptions
    #[serde(rename = "part")]
    pub parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC part](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_parts_pg.html#element_part) element
pub struct Part {
    /// Hardware part vendor name
    #[serde(rename = "Hvendor")]
    pub vendor: Option<String>,

    /// Hardware part name
    #[serde(rename = "Hname")]
    pub name: String,

    /// Hardware part class
    #[serde(rename = "Hclass")]
    pub class: Option<String>,

    /// Hardware part group
    #[serde(rename = "Hgroup")]
    pub group: Option<String>,

    /// Hardware part sub-group
    #[serde(rename = "Hsub")]
    pub sub: Option<String>,

    /// Exact commercial part name (variant)
    #[serde(rename = "Hvariant")]
    pub variant: Option<String>,

    /// Part revision identifier
    #[serde(rename = "Hrevision")]
    pub revision: Option<String>,

    /// Brief part description (max 256 characters)
    pub description: Option<String>,

    /// Part features and capabilities (0..*)
    #[serde(rename = "feature", default)]
    pub features: Vec<Feature>,

    /// Documentation files for this part (0..*)
    #[serde(rename = "book", default)]
    pub books: Vec<Book>,

    /// Part images (top/bottom/perspective)
    pub image: Option<Image>,

    /// IDE-specific tool environments for this part (0..*)
    pub environments: Option<PartEnvironments>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC part feature](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_feature) element
pub struct Feature {
    /// Processor identifier for multi-core parts
    #[serde(rename = "Pname")]
    pub processor_name: Option<String>,

    /// Predefined feature type (e.g. `CAN`, `DMA`, `Timer`, `UART`)
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
/// Represents a [PDSC part book](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_book) element
pub struct Book {
    /// Processor identifier for multi-core parts
    #[serde(rename = "Pname")]
    pub processor_name: Option<String>,

    /// Document file path or external URL
    pub name: String,

    /// Display title for the document
    pub title: String,

    /// Publishing permission; default `true`
    pub public: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC part image](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_parts_pg.html#element_part_image) element
pub struct Image {
    /// Path to the top-side part image
    pub top: String,

    /// Path to the bottom-side part image
    pub bottom: Option<String>,

    /// Path to a perspective-view part image
    pub perspective: Option<String>,

    /// Publishing permission; default `true`
    pub public: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [part environment](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_parts_pg.html#element_part) entry
pub struct PartEnvironment {
    /// IDE environment name (e.g. `uvision`, `iar`, `eclipse`)
    pub name: String,

    /// Processor name for multi-core parts; limits this environment entry to one core
    #[serde(rename = "Pname")]
    pub processor_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Groups part environment entries for a [PDSC part](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_parts_pg.html#element_part)
pub struct PartEnvironments {
    /// Individual environment entries (0..*)
    #[serde(rename = "environment", default)]
    pub environments: Vec<PartEnvironment>,
}

#[cfg(test)]
mod tests {
    use crate::parts::{Book, Feature, Image, PartEnvironment, Parts};

    #[test]
    fn parse_parts() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<parts>
    <part Hvendor="Microchip" Hname="ATSAMD21G18A" Hclass="Microcontroller" Hgroup="SAM D21"
          Hsub="SAMD21G18A" Hvariant="TQFP48" Hrevision="A">
        <description>SAM D21 ARM Cortex-M0+ microcontroller with 256 KB Flash</description>
        <feature Pname="Cortex-M0+" type="CoreOther" n="1" name="Cortex-M0+ processor"/>
        <feature type="UART" n="6"/>
        <book name="docs/SAM_D21_DS.pdf" title="SAM D21 Datasheet" public="true"/>
        <image top="images/samd21_top.png" bottom="images/samd21_bottom.png" public="true"/>
    </part>
</parts>"#;

        let parts: Parts = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(parts.parts.len(), 1);

        let part = &parts.parts[0];
        assert_eq!(part.vendor, Some("Microchip".to_string()));
        assert_eq!(part.name, "ATSAMD21G18A");
        assert_eq!(part.class, Some("Microcontroller".to_string()));
        assert_eq!(part.group, Some("SAM D21".to_string()));
        assert_eq!(part.sub, Some("SAMD21G18A".to_string()));
        assert_eq!(part.variant, Some("TQFP48".to_string()));
        assert_eq!(part.revision, Some("A".to_string()));
        assert_eq!(
            part.description,
            Some("SAM D21 ARM Cortex-M0+ microcontroller with 256 KB Flash".to_string())
        );
        assert_eq!(
            part.features,
            vec![
                Feature {
                    processor_name: Some("Cortex-M0+".to_string()),
                    feature_type: "CoreOther".to_string(),
                    n: Some("1".to_string()),
                    m: None,
                    name: Some("Cortex-M0+ processor".to_string()),
                },
                Feature {
                    processor_name: None,
                    feature_type: "UART".to_string(),
                    n: Some("6".to_string()),
                    m: None,
                    name: None,
                },
            ]
        );
        assert_eq!(
            part.books,
            vec![Book {
                processor_name: None,
                name: "docs/SAM_D21_DS.pdf".to_string(),
                title: "SAM D21 Datasheet".to_string(),
                public: Some(true),
            }]
        );
        assert_eq!(
            part.image,
            Some(Image {
                top: "images/samd21_top.png".to_string(),
                bottom: Some("images/samd21_bottom.png".to_string()),
                perspective: None,
                public: Some(true),
            })
        );
    }

    #[test]
    fn parse_part_minimal() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<parts>
    <part Hname="MY-PART-001"/>
</parts>"#;

        let parts: Parts = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(parts.parts.len(), 1);

        let part = &parts.parts[0];
        assert_eq!(part.name, "MY-PART-001");
        assert_eq!(part.vendor, None);
        assert_eq!(part.class, None);
        assert_eq!(part.group, None);
        assert_eq!(part.sub, None);
        assert_eq!(part.variant, None);
        assert_eq!(part.revision, None);
        assert_eq!(part.description, None);
        assert_eq!(part.features, vec![]);
        assert_eq!(part.books, vec![]);
        assert_eq!(part.image, None);
    }

    #[test]
    fn parse_parts_multiple() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<parts>
    <part Hvendor="ARM" Hname="CM4-CORE">
        <feature type="CoreOther" n="1"/>
    </part>
    <part Hvendor="ARM" Hname="CM0-CORE" Hrevision="r0p1">
        <book name="docs/cm0.pdf" title="Cortex-M0 Technical Reference Manual"/>
    </part>
</parts>"#;

        let parts: Parts = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(parts.parts.len(), 2);

        assert_eq!(parts.parts[0].vendor, Some("ARM".to_string()));
        assert_eq!(parts.parts[0].name, "CM4-CORE");
        assert_eq!(parts.parts[0].features.len(), 1);
        assert_eq!(parts.parts[0].books, vec![]);

        assert_eq!(parts.parts[1].vendor, Some("ARM".to_string()));
        assert_eq!(parts.parts[1].name, "CM0-CORE");
        assert_eq!(parts.parts[1].revision, Some("r0p1".to_string()));
        assert_eq!(parts.parts[1].features, vec![]);
        assert_eq!(parts.parts[1].books.len(), 1);
        assert_eq!(parts.parts[1].books[0].name, "docs/cm0.pdf");
        assert_eq!(
            parts.parts[1].books[0].title,
            "Cortex-M0 Technical Reference Manual"
        );
    }

    #[test]
    fn parse_part_environments() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<parts>
    <part Hname="MY-CHIP-001">
        <environments>
            <environment name="uvision"/>
            <environment name="iar" Pname="Core0"/>
        </environments>
    </part>
</parts>"#;

        let parts: Parts = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(parts.parts.len(), 1);

        let part = &parts.parts[0];
        assert_eq!(part.name, "MY-CHIP-001");
        let envs = part
            .environments
            .as_ref()
            .expect("environments should be present");
        assert_eq!(envs.environments.len(), 2);
        assert_eq!(
            envs.environments[0],
            PartEnvironment {
                name: "uvision".to_string(),
                processor_name: None,
            }
        );
        assert_eq!(
            envs.environments[1],
            PartEnvironment {
                name: "iar".to_string(),
                processor_name: Some("Core0".to_string()),
            }
        );
    }
}
