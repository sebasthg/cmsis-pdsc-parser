//! Contains the types required to represent a [PDSC APIs](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_apis_pg.html#element_apis) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC apis](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_apis_pg.html#element_apis) element
///
/// Groups all API definitions published by a pack. At most one `<apis>` section may exist per package.
pub struct Apis {
    /// API definitions (1..*)
    #[serde(rename = "api", default)]
    pub apis: Vec<Api>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC api](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_apis_pg.html#element_api) element
///
/// Defines a software API identified by a component class, group, and optional version.
pub struct Api {
    /// Component class identifier
    #[serde(rename = "Cclass")]
    pub class: String,

    /// Component group identifier
    #[serde(rename = "Cgroup")]
    pub group: String,

    /// API version; part of the API ID when present
    #[serde(rename = "Capiversion")]
    pub api_version: Option<String>,

    /// If `false`, multiple implementations of the API may coexist; default is `true`
    pub exclusive: Option<bool>,

    /// References a condition ID; this API applies only if the condition is met
    pub condition: Option<String>,

    /// References a `licenseSet` identifier governing usage rights
    #[serde(rename = "licenseSet")]
    pub license_set: Option<String>,

    /// References a changelog ID with the API change history
    pub changelog: Option<String>,

    /// Brief description of the API (max 256 characters)
    pub description: Option<String>,

    /// Header and documentation files that define the API interface
    pub files: Option<ApiFiles>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the `<files>` grouping element inside a [PDSC api](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_apis_pg.html#element_api)
pub struct ApiFiles {
    /// Individual file entries (1..*)
    #[serde(rename = "file", default)]
    pub files: Vec<ApiFile>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a `<file>` entry within the API files group
///
/// Attributes follow the [PDSC FileType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_file) definition.
pub struct ApiFile {
    /// File path relative to the pack root
    pub name: String,

    /// File category (e.g. `header`, `include`, `doc`, `sourceC`)
    pub category: String,

    /// File attribute (e.g. `config`, `template`)
    pub attr: Option<String>,

    /// References a condition ID; file is included only if condition is met
    pub condition: Option<String>,

    /// File version
    pub version: Option<String>,

    /// Selection string used when multiple template files are offered
    pub select: Option<String>,

    /// Source file for generated or templated files
    pub src: Option<String>,

    /// Alternate path for the file
    pub path: Option<String>,

    /// Programming language associated with the file
    pub language: Option<String>,

    /// Scope of the file within the project
    pub scope: Option<String>,

    /// Publishing permission; default `true`
    pub public: Option<bool>,

    /// Project-relative path override for the file
    pub projectpath: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::apis::{Api, ApiFile, ApiFiles, Apis};

    #[test]
    fn parse_apis() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<apis>
    <api Cclass="CMSIS" Cgroup="RTOS2" Capiversion="2.1.3" exclusive="false"
         condition="ARMCC6" licenseSet="all" changelog="Changelog.txt">
        <description>CMSIS-RTOS2 API for real-time operating systems</description>
        <files>
            <file category="header" name="CMSIS/RTOS2/Include/cmsis_os2.h"/>
            <file category="doc" name="CMSIS/RTOS2/Doc/index.html" public="true"/>
        </files>
    </api>
    <api Cclass="Device" Cgroup="Startup"/>
</apis>"#;

        let apis: Apis = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(apis.apis.len(), 2);

        assert_eq!(apis.apis[0], Api {
            class: "CMSIS".to_string(),
            group: "RTOS2".to_string(),
            api_version: Some("2.1.3".to_string()),
            exclusive: Some(false),
            condition: Some("ARMCC6".to_string()),
            license_set: Some("all".to_string()),
            changelog: Some("Changelog.txt".to_string()),
            description: Some("CMSIS-RTOS2 API for real-time operating systems".to_string()),
            files: Some(ApiFiles {
                files: vec![
                    ApiFile {
                        name: "CMSIS/RTOS2/Include/cmsis_os2.h".to_string(),
                        category: "header".to_string(),
                        attr: None,
                        condition: None,
                        version: None,
                        select: None,
                        src: None,
                        path: None,
                        language: None,
                        scope: None,
                        public: None,
                        projectpath: None,
                    },
                    ApiFile {
                        name: "CMSIS/RTOS2/Doc/index.html".to_string(),
                        category: "doc".to_string(),
                        attr: None,
                        condition: None,
                        version: None,
                        select: None,
                        src: None,
                        path: None,
                        language: None,
                        scope: None,
                        public: Some(true),
                        projectpath: None,
                    },
                ],
            }),
        });
        assert_eq!(apis.apis[1], Api {
            class: "Device".to_string(),
            group: "Startup".to_string(),
            api_version: None,
            exclusive: None,
            condition: None,
            license_set: None,
            changelog: None,
            description: None,
            files: None,
        });
    }

    #[test]
    fn parse_api_minimal() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<apis>
    <api Cclass="Security" Cgroup="mbed TLS"/>
</apis>"#;

        let apis: Apis = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(apis.apis.len(), 1);

        let api = &apis.apis[0];
        assert_eq!(api.class, "Security");
        assert_eq!(api.group, "mbed TLS");
        assert_eq!(api.api_version, None);
        assert_eq!(api.exclusive, None);
        assert_eq!(api.condition, None);
        assert_eq!(api.license_set, None);
        assert_eq!(api.changelog, None);
        assert_eq!(api.description, None);
        assert_eq!(api.files, None);
    }

    #[test]
    fn parse_api_files() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<apis>
    <api Cclass="USB" Cgroup="Core">
        <files>
            <file category="header" name="USB/Include/usb_core.h"
                  condition="USB_Cond" version="1.0.0" attr="config"
                  select="USB Core Header" src="USB/Src/usb_core.c"
                  public="false"/>
        </files>
    </api>
</apis>"#;

        let apis: Apis = serde_roxmltree::from_str(xml_str).unwrap();
        let api = &apis.apis[0];

        assert_eq!(api.class, "USB");
        assert_eq!(api.group, "Core");
        assert_eq!(api.description, None);

        let files = api.files.as_ref().unwrap();
        assert_eq!(files.files.len(), 1);

        let file = &files.files[0];
        assert_eq!(file.name, "USB/Include/usb_core.h");
        assert_eq!(file.category, "header");
        assert_eq!(file.condition, Some("USB_Cond".to_string()));
        assert_eq!(file.version, Some("1.0.0".to_string()));
        assert_eq!(file.attr, Some("config".to_string()));
        assert_eq!(file.select, Some("USB Core Header".to_string()));
        assert_eq!(file.src, Some("USB/Src/usb_core.c".to_string()));
        assert_eq!(file.public, Some(false));
        assert_eq!(file.path, None);
        assert_eq!(file.language, None);
        assert_eq!(file.scope, None);
        assert_eq!(file.projectpath, None);
    }
}
