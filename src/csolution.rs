//! Contains the types required to represent a [PDSC csolution](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_csolution_pg.html#element_csolution) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC csolution](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_csolution_pg.html#element_csolution) element
///
/// Groups software layers and project templates published by a pack for use with csolution-based projects.
pub struct Csolution {
    /// Software layer definitions (0..*)
    #[serde(rename = "clayer", default)]
    pub clayers: Vec<Clayer>,

    /// Project template definitions (0..*)
    #[serde(rename = "template", default)]
    pub templates: Vec<CsolutionTemplate>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [clayer](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_csolution_pg.html#element_clayer) element
///
/// Provides access information for a software layer stored in a pack.
pub struct Clayer {
    /// Layer type identifier
    #[serde(rename = "type")]
    pub layer_type: String,

    /// Path to the layer directory relative to the pack root
    pub path: String,

    /// Layer project file name (e.g. `.clayer.yml`)
    pub file: String,

    /// Optional destination path when copying the layer into a project
    #[serde(rename = "copy-to")]
    pub copy_to: Option<String>,

    /// References a condition ID; layer is available only if the condition is met
    pub condition: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [template](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_csolution_pg.html#element_cs_template) element
///
/// Defines a project template for initialising new csolution projects in an IDE.
pub struct CsolutionTemplate {
    /// Template display name
    pub name: String,

    /// Path to the template directory relative to the pack root
    pub path: String,

    /// Template project file name (e.g. `.csolution.yml`)
    pub file: String,

    /// Optional destination path when copying the template into a project
    #[serde(rename = "copy-to")]
    pub copy_to: Option<String>,

    /// References a condition ID; template is available only if the condition is met
    pub condition: Option<String>,

    /// Brief description of the template
    pub description: String,
}

#[cfg(test)]
mod tests {
    use crate::csolution::{Clayer, CsolutionTemplate, Csolution};

    #[test]
    fn parse_csolution() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<csolution>
    <clayer type="Board" path="layers/board" file="Board.clayer.yml"
            copy-to="Board" condition="CM4"/>
    <clayer type="Shield" path="layers/shield" file="Shield.clayer.yml"/>
    <template name="Blinky" path="templates/Blinky" file="Blinky.csolution.yml"
              copy-to="MyBlinky" condition="GCC">
        <description>Simple LED blinking template</description>
    </template>
    <template name="Empty" path="templates/Empty" file="Empty.csolution.yml">
        <description>Minimal empty project</description>
    </template>
</csolution>"#;

        let cs: Csolution = serde_roxmltree::from_str(xml_str).unwrap();

        assert_eq!(cs.clayers.len(), 2);
        assert_eq!(cs.clayers[0], Clayer {
            layer_type: "Board".to_string(),
            path: "layers/board".to_string(),
            file: "Board.clayer.yml".to_string(),
            copy_to: Some("Board".to_string()),
            condition: Some("CM4".to_string()),
        });
        assert_eq!(cs.clayers[1], Clayer {
            layer_type: "Shield".to_string(),
            path: "layers/shield".to_string(),
            file: "Shield.clayer.yml".to_string(),
            copy_to: None,
            condition: None,
        });

        assert_eq!(cs.templates.len(), 2);
        assert_eq!(cs.templates[0], CsolutionTemplate {
            name: "Blinky".to_string(),
            path: "templates/Blinky".to_string(),
            file: "Blinky.csolution.yml".to_string(),
            copy_to: Some("MyBlinky".to_string()),
            condition: Some("GCC".to_string()),
            description: "Simple LED blinking template".to_string(),
        });
        assert_eq!(cs.templates[1], CsolutionTemplate {
            name: "Empty".to_string(),
            path: "templates/Empty".to_string(),
            file: "Empty.csolution.yml".to_string(),
            copy_to: None,
            condition: None,
            description: "Minimal empty project".to_string(),
        });
    }

    #[test]
    fn parse_csolution_clayers_only() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<csolution>
    <clayer type="App" path="layers/app" file="App.clayer.yml"/>
</csolution>"#;

        let cs: Csolution = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(cs.clayers.len(), 1);
        assert_eq!(cs.templates.len(), 0);

        let l = &cs.clayers[0];
        assert_eq!(l.layer_type, "App");
        assert_eq!(l.path, "layers/app");
        assert_eq!(l.file, "App.clayer.yml");
        assert_eq!(l.copy_to, None);
        assert_eq!(l.condition, None);
    }

    #[test]
    fn parse_csolution_templates_only() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<csolution>
    <template name="Full" path="templates/Full" file="Full.csolution.yml"
              copy-to="MyProject">
        <description>Full-featured project template</description>
    </template>
</csolution>"#;

        let cs: Csolution = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(cs.clayers.len(), 0);
        assert_eq!(cs.templates.len(), 1);

        let t = &cs.templates[0];
        assert_eq!(t.name, "Full");
        assert_eq!(t.path, "templates/Full");
        assert_eq!(t.file, "Full.csolution.yml");
        assert_eq!(t.copy_to, Some("MyProject".to_string()));
        assert_eq!(t.condition, None);
        assert_eq!(t.description, "Full-featured project template");
    }
}
