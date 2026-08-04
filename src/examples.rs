//! Contains the types required to represent a [PDSC Examples](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_examples_pg.html#element_examples) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC examples](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_examples_pg.html#element_examples) element
///
/// Groups all example projects published by a pack.
pub struct Examples {
    /// Example project definitions (1..*)
    #[serde(rename = "example", default)]
    pub examples: Vec<Example>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents a [PDSC example](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_examples_pg.html#element_example) element
///
/// Defines a single example project, its boards, tool environments, and classification attributes.
pub struct Example {
    /// Short example identifier
    pub name: String,

    /// Path to the example folder relative to the pack root
    pub folder: String,

    /// Archive filename containing the example files
    pub archive: Option<String>,

    /// Path to the example documentation file
    pub doc: String,

    /// Example version
    pub version: Option<String>,

    /// Publishing permission; default `true`
    pub public: Option<bool>,

    /// Brief description of the example
    pub description: String,

    /// Target boards for this example (0..*)
    #[serde(rename = "board", default)]
    pub boards: Vec<ExampleBoard>,

    /// IDE/tool project files for this example
    pub project: ExampleProject,

    /// Classification attributes for discovery and filtering
    pub attributes: Option<ExampleAttributes>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents an [example board reference](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_examples_pg.html#element_example_board)
///
/// Identifies a development board on which the example has been tested.
pub struct ExampleBoard {
    /// Board vendor name
    pub vendor: String,

    /// Commercial board name
    pub name: String,

    /// Device vendor (deprecated since v1.1; prefer board's mounted device)
    #[serde(rename = "Dvendor")]
    pub device_vendor: Option<String>,

    /// Device family (deprecated since v1.1)
    #[serde(rename = "Dfamily")]
    pub device_family: Option<String>,

    /// Device sub-family (deprecated since v1.1)
    #[serde(rename = "DsubFamily")]
    pub device_sub_family: Option<String>,

    /// Device name (deprecated since v1.1)
    #[serde(rename = "Dname")]
    pub device_name: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [example project](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_examples_pg.html#element_example_project) element
///
/// Groups the tool-specific environment entries for this example.
pub struct ExampleProject {
    /// Tool environment entries (1..*)
    #[serde(rename = "environment", default)]
    pub environments: Vec<ExampleEnvironment>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents an [example project environment](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_examples_pg.html#element_example_project_env) entry
///
/// Identifies the project file and optional subfolder for a specific development tool
/// (e.g. `uv`, `iar`, `csolution`).
pub struct ExampleEnvironment {
    /// Development tool identifier (e.g. `uv`, `iar`, `csolution`)
    pub name: String,

    /// Project file path with extension, relative to the example folder
    pub load: String,

    /// Subdirectory containing tool-specific files, relative to the example folder
    pub folder: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [example attributes](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_examples_pg.html#element_example_attributes) element
///
/// Classification metadata used for example discovery and filtering.
pub struct ExampleAttributes {
    /// Free-form category labels (0..*)
    #[serde(rename = "category", default)]
    pub categories: Vec<String>,

    /// Component dependencies or tags (0..*)
    #[serde(rename = "component", default)]
    pub components: Vec<ExampleComponent>,

    /// Search keywords (0..*)
    #[serde(rename = "keyword", default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents a [component attribute](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_examples_pg.html#element_example_attribute_component) entry
///
/// Tags the example with a component class, group, and optional version for filtering.
pub struct ExampleComponent {
    /// Component class
    #[serde(rename = "Cclass")]
    pub class: String,

    /// Component group
    #[serde(rename = "Cgroup")]
    pub group: Option<String>,

    /// Component sub-group
    #[serde(rename = "Csub")]
    pub sub: Option<String>,

    /// Component version
    #[serde(rename = "Cversion")]
    pub version: Option<String>,

    /// Component vendor
    #[serde(rename = "Cvendor")]
    pub vendor: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::examples::{
        ExampleBoard, ExampleComponent, ExampleEnvironment, ExampleProject, Examples,
    };

    #[test]
    fn parse_examples() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<examples>
    <example name="Blinky" folder="examples/Blinky" doc="examples/Blinky/README.md"
             archive="Blinky.zip" version="1.2.0" public="true">
        <description>Blinky LED example for Cortex-M4</description>
        <board vendor="STMicroelectronics" name="NUCLEO-F401RE"
               Dvendor="STMicroelectronics:13" Dname="STM32F401RETx"/>
        <project>
            <environment name="uv" load="Blinky.uvprojx"/>
            <environment name="csolution" load="Blinky.csolution.yml" folder="csolution"/>
        </project>
        <attributes>
            <category>Getting Started</category>
            <component Cclass="CMSIS" Cgroup="RTOS2" Cversion="2.0.0" Cvendor="ARM"/>
            <keyword>LED</keyword>
            <keyword>Blinky</keyword>
        </attributes>
    </example>
    <example name="Hello" folder="examples/Hello" doc="examples/Hello/README.md">
        <description>Hello World via UART</description>
        <project>
            <environment name="uv" load="Hello.uvprojx"/>
        </project>
    </example>
</examples>"#;

        let examples: Examples = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(examples.examples.len(), 2);

        let e0 = &examples.examples[0];
        assert_eq!(e0.name, "Blinky");
        assert_eq!(e0.folder, "examples/Blinky");
        assert_eq!(e0.doc, "examples/Blinky/README.md");
        assert_eq!(e0.archive, Some("Blinky.zip".to_string()));
        assert_eq!(e0.version, Some("1.2.0".to_string()));
        assert_eq!(e0.public, Some(true));
        assert_eq!(e0.description, "Blinky LED example for Cortex-M4");
        assert_eq!(e0.boards, vec![ExampleBoard {
            vendor: "STMicroelectronics".to_string(),
            name: "NUCLEO-F401RE".to_string(),
            device_vendor: Some("STMicroelectronics:13".to_string()),
            device_family: None,
            device_sub_family: None,
            device_name: Some("STM32F401RETx".to_string()),
        }]);
        assert_eq!(e0.project, ExampleProject {
            environments: vec![
                ExampleEnvironment {
                    name: "uv".to_string(),
                    load: "Blinky.uvprojx".to_string(),
                    folder: None,
                },
                ExampleEnvironment {
                    name: "csolution".to_string(),
                    load: "Blinky.csolution.yml".to_string(),
                    folder: Some("csolution".to_string()),
                },
            ],
        });
        let attrs = e0.attributes.as_ref().unwrap();
        assert_eq!(attrs.categories, vec!["Getting Started".to_string()]);
        assert_eq!(attrs.components, vec![ExampleComponent {
            class: "CMSIS".to_string(),
            group: Some("RTOS2".to_string()),
            sub: None,
            version: Some("2.0.0".to_string()),
            vendor: Some("ARM".to_string()),
        }]);
        assert_eq!(attrs.keywords, vec!["LED".to_string(), "Blinky".to_string()]);

        let e1 = &examples.examples[1];
        assert_eq!(e1.name, "Hello");
        assert_eq!(e1.archive, None);
        assert_eq!(e1.version, None);
        assert_eq!(e1.boards, vec![]);
        assert_eq!(e1.attributes, None);
    }

    #[test]
    fn parse_example_minimal() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<examples>
    <example name="Minimal" folder="examples/Minimal" doc="examples/Minimal/README.md">
        <description>Minimal example</description>
        <project>
            <environment name="uv" load="Minimal.uvprojx"/>
        </project>
    </example>
</examples>"#;

        let examples: Examples = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(examples.examples.len(), 1);

        let e = &examples.examples[0];
        assert_eq!(e.name, "Minimal");
        assert_eq!(e.folder, "examples/Minimal");
        assert_eq!(e.doc, "examples/Minimal/README.md");
        assert_eq!(e.archive, None);
        assert_eq!(e.version, None);
        assert_eq!(e.public, None);
        assert_eq!(e.description, "Minimal example");
        assert_eq!(e.boards, vec![]);
        assert_eq!(e.project.environments.len(), 1);
        assert_eq!(e.project.environments[0].name, "uv");
        assert_eq!(e.project.environments[0].load, "Minimal.uvprojx");
        assert_eq!(e.project.environments[0].folder, None);
        assert_eq!(e.attributes, None);
    }

    #[test]
    fn parse_example_attributes() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<examples>
    <example name="Tagged" folder="examples/Tagged" doc="examples/Tagged/README.md">
        <description>Example with rich attributes</description>
        <project>
            <environment name="csolution" load="Tagged.csolution.yml"/>
        </project>
        <attributes>
            <category>Middleware</category>
            <category>Networking</category>
            <component Cclass="Network" Cgroup="Core" Csub="IPv4" Cversion="7.15.0" Cvendor="Keil"/>
            <component Cclass="CMSIS"/>
            <keyword>TCP/IP</keyword>
        </attributes>
    </example>
</examples>"#;

        let examples: Examples = serde_roxmltree::from_str(xml_str).unwrap();
        let e = &examples.examples[0];
        let attrs = e.attributes.as_ref().unwrap();

        assert_eq!(attrs.categories, vec!["Middleware".to_string(), "Networking".to_string()]);
        assert_eq!(attrs.components, vec![
            ExampleComponent {
                class: "Network".to_string(),
                group: Some("Core".to_string()),
                sub: Some("IPv4".to_string()),
                version: Some("7.15.0".to_string()),
                vendor: Some("Keil".to_string()),
            },
            ExampleComponent {
                class: "CMSIS".to_string(),
                group: None,
                sub: None,
                version: None,
                vendor: None,
            },
        ]);
        assert_eq!(attrs.keywords, vec!["TCP/IP".to_string()]);
    }
}
