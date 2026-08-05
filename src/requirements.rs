//! Contains the types required for the [PDSC Requirements](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_requirements_pg.html) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC Requirements](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_requirements_pg.html#element_requirements)
/// element
pub struct Requirements {
    /// List of software packs required for the project to build
    pub packages: Option<PackagesList>,

    /// List of compilers required for the project to build
    pub compilers: Option<CompilersList>,

    /// List of language standards required for the project to build.
    pub languages: Option<LanguagesList>,

    /// Restrict a software pack to a list of devices, boards, or processor cores.
    pub targets: Option<TargetsList>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Wrapper type for the `packages` xml element
pub struct PackagesList {
    #[serde(rename = "package")]
    pub packages: Vec<Package>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Wrapper type for the `compilers` xml element
pub struct CompilersList {
    #[serde(rename = "compiler")]
    pub compilers: Vec<Compiler>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Wrapper type for the `languages` xml element
pub struct LanguagesList {
    #[serde(rename = "language")]
    pub languages: Vec<Language>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Wrapper type for the `targets` xml element
pub struct TargetsList {
    #[serde(rename = "target")]
    pub targets: Vec<Target>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC PackagesType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_requirements_pg.html#element_packages) element
pub struct Package {
    /// The package vendor; pattern: `RestrictedString` = `[A-Za-z0-9_\-]+`
    pub vendor: String,

    /// Name of the pack
    pub name: String,

    /// Version of the required pack; pattern: `VersionType` (e.g. `1.2.3` or `>=1.0.0`)
    pub version: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC CompilersType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_requirements_pg.html#element_compilers) element
pub struct Compiler {
    /// Name of the required compiler
    pub name: String,

    /// Version of the required compiler
    pub version: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC Languages](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_requirements_pg.html#element_languages) element
pub struct Language {
    /// Name of the porgramming language, i.e. "C", "C++", "Rust"
    pub name: String,

    /// Version of the language standard
    pub version: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC Targets](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_requirements_pg.html#element_targets) element
pub struct Target {
    #[serde(rename = "Dvendor")]
    /// The silicon vendor of the device
    pub device_vendor: Option<String>,

    #[serde(rename = "Dname")]
    /// The name of the device (wildcards possible)
    pub device_name: Option<String>,

    #[serde(rename = "Dcore")]
    /// The name of the processor core
    pub device_core: Option<String>,

    #[serde(rename = "Bvendor")]
    /// The name of the board vendor
    pub board_vendor: Option<String>,

    #[serde(rename = "Bname")]
    /// The name of the board
    pub board_name: Option<String>,

    #[serde(rename = "Brevision")]
    /// Specifies the board revision
    pub board_revision: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::requirements::{
        Compiler, CompilersList, Language, LanguagesList, Package, PackagesList, Requirements,
        Target, TargetsList,
    };

    #[test]
    fn isolate_fields() {
        let xml_packages = r#"<?xml version="1.0"?><requirements><packages><package name="A" vendor="B"/></packages></requirements>"#;
        let xml_compilers = r#"<?xml version="1.0"?><requirements><compilers><compiler name="ARMCC" version="5.0"/></compilers></requirements>"#;
        let xml_languages = r#"<?xml version="1.0"?><requirements><languages><language name="C" version="99"/></languages></requirements>"#;
        let xml_targets = r#"<?xml version="1.0"?><requirements><targets><target Dvendor="X:1" Dname="Y"/></targets></requirements>"#;

        let r: Result<Requirements, _> = serde_roxmltree::from_str(xml_packages);
        println!("packages: {:?}", r.err());
        let r: Result<Requirements, _> = serde_roxmltree::from_str(xml_compilers);
        println!("compilers: {:?}", r.err());
        let r: Result<Requirements, _> = serde_roxmltree::from_str(xml_languages);
        println!("languages: {:?}", r.err());
        let r: Result<Requirements, _> = serde_roxmltree::from_str(xml_targets);
        println!("targets: {:?}", r.err());
    }

    #[test]
    /// Test that all requiremets fields parse correctly
    fn parse_requirements() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<requirements>
    <packages>
        <package name="STM32F4xx_DFP" vendor="Keil" version="2.8.0:2.8.0"/>
    </packages>
    <compilers>
        <compiler name="ARMCC" version="5.0.0:6.0.0-0"/>
    </compilers>
    <languages>
        <language name="C" version="99"/>
    </languages>
    <targets>
        <target Dvendor="STMicroelectronics:13" Dname="STM32H7*"/>    <!-- supports STM32H7 device series -->
        <target Dvendor="STMicroelectronics:13" Dname="STM32U585"/>   <!-- and supports STM32U585 device -->
    </targets>
</requirements>"#;

        let requirements: Requirements = serde_roxmltree::from_str(xml_str).unwrap();

        assert_eq!(
            requirements.packages,
            Some(PackagesList {
                packages: vec![Package {
                    name: "STM32F4xx_DFP".to_string(),
                    vendor: "Keil".to_string(),
                    version: Some("2.8.0:2.8.0".to_string())
                }]
            })
        );
        assert_eq!(
            requirements.compilers,
            Some(CompilersList {
                compilers: vec![Compiler {
                    name: "ARMCC".to_string(),
                    version: "5.0.0:6.0.0-0".to_string()
                }]
            })
        );
        assert_eq!(
            requirements.languages,
            Some(LanguagesList {
                languages: vec![Language {
                    name: "C".to_string(),
                    version: "99".to_string()
                }]
            })
        );
        assert_eq!(
            requirements.targets,
            Some(TargetsList {
                targets: vec![
                    Target {
                        device_vendor: Some("STMicroelectronics:13".to_string()),
                        device_name: Some("STM32H7*".to_string()),
                        device_core: None,
                        board_name: None,
                        board_revision: None,
                        board_vendor: None
                    },
                    Target {
                        device_vendor: Some("STMicroelectronics:13".to_string()),
                        device_name: Some("STM32U585".to_string()),
                        device_core: None,
                        board_name: None,
                        board_revision: None,
                        board_vendor: None
                    }
                ]
            })
        );
    }
}
