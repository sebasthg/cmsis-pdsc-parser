//! Contains the types required to represent a [PDSC Components](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_components) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC components](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_components) element
///
/// Groups all software bundles and standalone components published by a pack.
pub struct Components {
    /// Generator ID applied to all enclosed components when set
    pub generator: Option<String>,

    /// Component bundle definitions (0..*)
    #[serde(rename = "bundle", default)]
    pub bundles: Vec<Bundle>,

    /// Standalone component definitions (0..*)
    #[serde(rename = "component", default)]
    pub components: Vec<Component>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [bundle](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_bundle) element
///
/// Groups a set of interdependent components under a shared class, version, and name.
/// The `Cclass` and `Cversion` attributes are inherited by all enclosed components.
pub struct Bundle {
    /// Bundle name; becomes part of each enclosed component's ID
    #[serde(rename = "Cbundle")]
    pub bundle: String,

    /// Component vendor; derives from package vendor if omitted
    #[serde(rename = "Cvendor")]
    pub vendor: Option<String>,

    /// Component class shared by all enclosed components
    #[serde(rename = "Cclass")]
    pub class: String,

    /// Version shared by all enclosed components unless individually overridden
    #[serde(rename = "Cversion")]
    pub version: String,

    /// References a `licenseSet` identifier governing usage rights
    #[serde(rename = "licenseSet")]
    pub license_set: Option<String>,

    /// References a changelog ID with the bundle change history
    pub changelog: Option<String>,

    /// Brief description of the bundle (max 256 characters)
    pub description: String,

    /// Path to the bundle documentation file relative to the pack root
    pub doc: String,

    /// Components enclosed in this bundle (1..*)
    #[serde(rename = "component", default)]
    pub components: Vec<Component>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [component](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_component) element
///
/// Defines a single software component. Used for both top-level components and components
/// nested inside a bundle. When inside a bundle, `class` and `version` are absent (inherited).
pub struct Component {
    /// Component vendor; derives from package vendor if omitted
    #[serde(rename = "Cvendor")]
    pub vendor: Option<String>,

    /// Component class; absent when nested in a bundle (inherited)
    #[serde(rename = "Cclass")]
    pub class: Option<String>,

    /// Component group
    #[serde(rename = "Cgroup")]
    pub group: String,

    /// Component sub-group (3–32 characters)
    #[serde(rename = "Csub")]
    pub sub: Option<String>,

    /// Variant name (e.g. `release`, `debug`); mutually exclusive with other variants
    #[serde(rename = "Cvariant")]
    pub variant: Option<String>,

    /// Component version; absent when nested in a bundle unless overriding
    #[serde(rename = "Cversion")]
    pub version: Option<String>,

    /// API version consumed by this component
    #[serde(rename = "Capiversion")]
    pub api_version: Option<String>,

    /// References a condition ID; component is included only if the condition is met
    pub condition: Option<String>,

    /// If `true`, suppresses the automatic resolver; component requires manual selection
    pub custom: Option<bool>,

    /// Number of simultaneous instances allowed (1–10); default is 1
    #[serde(rename = "maxInstances")]
    pub max_instances: Option<u32>,

    /// Marks this variant as the preferred choice for automated updates
    #[serde(rename = "isDefaultVariant")]
    pub is_default_variant: Option<bool>,

    /// Links to a `<generator>` entry in the same pack
    pub generator: Option<String>,

    /// References a `licenseSet` identifier governing usage rights
    #[serde(rename = "licenseSet")]
    pub license_set: Option<String>,

    /// User-facing visibility (`always`, `never`, `maskable`); default is `always`
    pub view: Option<String>,

    /// References a changelog ID with the component change history
    pub changelog: Option<String>,

    /// Marks the component as deprecated; deprecated components should not be used in new designs
    pub deprecated: Option<bool>,

    /// Brief description of the component (max 256 characters)
    pub description: String,

    /// C preprocessor definitions injected verbatim into `RTE_Components.h`
    #[serde(rename = "RTE_Components_h")]
    pub rte_components_h: Option<String>,

    /// Content pre-included globally for all project modules via `Pre_Include_Global.h`
    #[serde(rename = "Pre_Include_Global_h")]
    pub pre_include_global_h: Option<String>,

    /// Content pre-included for this component's modules only via `Pre_Include_<Cclass>_<component>.h`
    #[serde(rename = "Pre_Include_Local_Component_h")]
    pub pre_include_local_component_h: Option<String>,

    /// Source and header files that implement this component
    #[serde(default)]
    pub files: ComponentFiles,

    /// Key/value metadata extensions for toolchain or IDE integration
    #[serde(default)]
    pub extensions: ComponentExtensions,

    /// IDE-specific tool integration environments (0..1)
    pub environments: Option<ComponentEnvironments>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, Default)]
/// Represents the [files](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_files) grouping element inside a component
pub struct ComponentFiles {
    /// Individual file entries (1..*)
    #[serde(rename = "file", default)]
    pub files: Vec<ComponentFile>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [file](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_file) entry within a component
///
/// Attributes follow the PDSC `FileType` definition shared across components and APIs.
pub struct ComponentFile {
    /// File path relative to the pack root; may be a URL for `category="doc"`
    pub name: String,

    /// File category (e.g. `header`, `sourceC`, `doc`, `library`)
    pub category: String,

    /// Special handling: `config` (copied to project, user-editable) or `template`
    pub attr: Option<String>,

    /// References a condition ID; file included only when condition evaluates true
    pub condition: Option<String>,

    /// File-specific version; component version used if omitted
    pub version: Option<String>,

    /// Description/purpose required when `attr="template"`; groups template options
    pub select: Option<String>,

    /// Source path relative to PDSC; semicolon-separated list for libraries
    pub src: Option<String>,

    /// For `category="header"`: an incomplete include path for project-relative includes
    pub path: Option<String>,

    /// Target compiler/assembler (`c`, `cpp`, `c-cpp`, `asm`, `link`); inferred from extension if absent
    pub language: Option<String>,

    /// Header visibility (`public` or `private`); default is `public`
    pub scope: Option<String>,

    /// Publishing permission; default `true`
    pub public: Option<bool>,

    /// IDE project explorer location override
    pub projectpath: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, Default)]
/// Represents the [extensions](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_extensions) grouping element inside a component
pub struct ComponentExtensions {
    /// Key/value extension entries (1..*)
    #[serde(rename = "extension", default)]
    pub extensions: Vec<ComponentExtension>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents an [extension](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_extension) entry within the extensions group
///
/// Provides arbitrary key/value metadata for toolchain or IDE integration.
pub struct ComponentExtension {
    /// Extension identifier; unique within the component
    pub key: String,

    /// Value associated with the key
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [environments](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_component_environments) grouping element inside a component
pub struct ComponentEnvironments {
    /// Tool environment entries (1..*)
    #[serde(rename = "environment", default)]
    pub environments: Vec<ComponentEnvironment>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [component environment](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_component_environment) entry
///
/// Identifies a specific development tool (e.g. `uv`, `iar`). Tool-specific child
/// elements (often namespace-qualified) are silently ignored by the parser.
pub struct ComponentEnvironment {
    /// Development tool identifier
    pub name: String,

    /// Processor selector for multi-core devices
    #[serde(rename = "Pname")]
    pub processor_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::components::{
        ComponentEnvironment, ComponentExtension, ComponentFile, Components,
    };

    #[test]
    fn parse_components() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<components generator="MyGen">
    <component Cclass="Device" Cgroup="Startup" Cversion="1.0.0"
               Cvendor="ARM" condition="CM4" licenseSet="all" changelog="CHANGES.txt"
               custom="false" maxInstances="2" isDefaultVariant="true"
               generator="StartupGen" view="always">
        <description>Device startup files</description>
        <files>
            <file category="sourceC" name="Device/Source/startup.c"/>
            <file category="header" name="Device/Include/system.h" public="true"/>
        </files>
        <extensions>
            <extension key="schemaVersion" value="1.0"/>
        </extensions>
    </component>
    <bundle Cbundle="MyRTOS" Cclass="RTOS" Cversion="5.6.0"
            Cvendor="ARM" licenseSet="rtosLicense" changelog="RTOS_CHANGES.txt">
        <description>ARM MyRTOS bundle</description>
        <doc>documentation/MyRTOS.html</doc>
        <component Cgroup="Kernel" Csub="Source">
            <description>RTOS kernel source</description>
            <files>
                <file category="sourceC" name="RTOS/Source/kernel.c"/>
            </files>
            <extensions/>
        </component>
    </bundle>
</components>"#;

        let cs: Components = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(cs.generator, Some("MyGen".to_string()));
        assert_eq!(cs.components.len(), 1);
        assert_eq!(cs.bundles.len(), 1);

        let c = &cs.components[0];
        assert_eq!(c.vendor, Some("ARM".to_string()));
        assert_eq!(c.class, Some("Device".to_string()));
        assert_eq!(c.group, "Startup");
        assert_eq!(c.version, Some("1.0.0".to_string()));
        assert_eq!(c.condition, Some("CM4".to_string()));
        assert_eq!(c.license_set, Some("all".to_string()));
        assert_eq!(c.changelog, Some("CHANGES.txt".to_string()));
        assert_eq!(c.custom, Some(false));
        assert_eq!(c.max_instances, Some(2));
        assert_eq!(c.is_default_variant, Some(true));
        assert_eq!(c.generator, Some("StartupGen".to_string()));
        assert_eq!(c.view, Some("always".to_string()));
        assert_eq!(c.description, "Device startup files");
        assert_eq!(c.deprecated, None);
        assert_eq!(c.sub, None);
        assert_eq!(c.variant, None);
        assert_eq!(c.api_version, None);
        assert_eq!(c.rte_components_h, None);
        assert_eq!(c.files.files.len(), 2);
        assert_eq!(c.extensions.extensions.len(), 1);
        assert_eq!(c.extensions.extensions[0], ComponentExtension {
            key: "schemaVersion".to_string(),
            value: Some("1.0".to_string()),
        });
        assert_eq!(c.environments, None);

        let b = &cs.bundles[0];
        assert_eq!(b.bundle, "MyRTOS");
        assert_eq!(b.class, "RTOS");
        assert_eq!(b.version, "5.6.0");
        assert_eq!(b.vendor, Some("ARM".to_string()));
        assert_eq!(b.license_set, Some("rtosLicense".to_string()));
        assert_eq!(b.changelog, Some("RTOS_CHANGES.txt".to_string()));
        assert_eq!(b.description, "ARM MyRTOS bundle");
        assert_eq!(b.doc, "documentation/MyRTOS.html");
        assert_eq!(b.components.len(), 1);

        let bc = &b.components[0];
        assert_eq!(bc.class, None);
        assert_eq!(bc.version, None);
        assert_eq!(bc.group, "Kernel");
        assert_eq!(bc.sub, Some("Source".to_string()));
        assert_eq!(bc.description, "RTOS kernel source");
    }

    #[test]
    fn parse_component_bundle() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<components>
    <bundle Cbundle="CMSIS" Cclass="CMSIS" Cversion="5.9.0">
        <description>CMSIS software framework</description>
        <doc>CMSIS/Documentation/html/index.html</doc>
        <component Cgroup="CORE">
            <description>CMSIS-CORE support for Cortex-M</description>
            <files>
                <file category="header" name="CMSIS/Core/Include/cmsis_compiler.h"/>
                <file category="header" name="CMSIS/Core/Include/core_cm4.h" condition="CM4"/>
            </files>
            <extensions/>
        </component>
        <component Cgroup="DSP" Cvariant="Source" Cversion="1.10.1"
                   isDefaultVariant="true">
            <description>CMSIS-DSP library source</description>
            <files>
                <file category="sourceC" name="CMSIS/DSP/Source/BasicMathFunctions/arm_abs_f32.c"/>
            </files>
            <extensions>
                <extension key="dsplicense" value="Apache-2.0"/>
            </extensions>
        </component>
    </bundle>
</components>"#;

        let cs: Components = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(cs.generator, None);
        assert_eq!(cs.components.len(), 0);
        assert_eq!(cs.bundles.len(), 1);

        let b = &cs.bundles[0];
        assert_eq!(b.bundle, "CMSIS");
        assert_eq!(b.class, "CMSIS");
        assert_eq!(b.version, "5.9.0");
        assert_eq!(b.vendor, None);
        assert_eq!(b.license_set, None);
        assert_eq!(b.changelog, None);
        assert_eq!(b.description, "CMSIS software framework");
        assert_eq!(b.doc, "CMSIS/Documentation/html/index.html");
        assert_eq!(b.components.len(), 2);

        let c0 = &b.components[0];
        assert_eq!(c0.group, "CORE");
        assert_eq!(c0.class, None);
        assert_eq!(c0.version, None);
        assert_eq!(c0.variant, None);
        assert_eq!(c0.is_default_variant, None);
        assert_eq!(c0.files.files.len(), 2);
        assert_eq!(c0.files.files[1].condition, Some("CM4".to_string()));
        assert_eq!(c0.extensions.extensions.len(), 0);

        let c1 = &b.components[1];
        assert_eq!(c1.group, "DSP");
        assert_eq!(c1.variant, Some("Source".to_string()));
        assert_eq!(c1.version, Some("1.10.1".to_string()));
        assert_eq!(c1.is_default_variant, Some(true));
        assert_eq!(c1.extensions.extensions[0].key, "dsplicense");
        assert_eq!(c1.extensions.extensions[0].value, Some("Apache-2.0".to_string()));
    }

    #[test]
    fn parse_component_files() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<components>
    <component Cclass="USB" Cgroup="Core" Cvariant="Device" Cversion="6.15.0">
        <deprecated>true</deprecated>
        <description>USB Device stack (deprecated; use MDK-Middleware instead)</description>
        <RTE_Components_h>#define RTE_USB_CORE</RTE_Components_h>
        <Pre_Include_Global_h>#include "usb_config.h"</Pre_Include_Global_h>
        <files>
            <file category="header" name="USB/Include/rl_usb.h" scope="public"/>
            <file category="header" name="USB/Config/USB_Config.h"
                  attr="config" version="6.15.0" select="USB Config"/>
            <file category="sourceC" name="USB/Source/usbd_core.c"
                  condition="USB_Cond" src="USB/Source/usbd_core.c"
                  language="c" public="false" projectpath="USB/Source"/>
            <file category="doc" name="https://www.keil.com/pack/doc/mw/USB/html/index.html"/>
        </files>
        <extensions>
            <extension key="schemaVersion" value="2.0"/>
            <extension key="category" value="middleware"/>
        </extensions>
        <environments>
            <environment name="uv" Pname="Core0"/>
        </environments>
    </component>
</components>"#;

        let cs: Components = serde_roxmltree::from_str(xml_str).unwrap();
        let c = &cs.components[0];

        assert_eq!(c.class, Some("USB".to_string()));
        assert_eq!(c.group, "Core");
        assert_eq!(c.variant, Some("Device".to_string()));
        assert_eq!(c.version, Some("6.15.0".to_string()));
        assert_eq!(c.deprecated, Some(true));
        assert_eq!(c.description, "USB Device stack (deprecated; use MDK-Middleware instead)");
        assert_eq!(c.rte_components_h, Some("#define RTE_USB_CORE".to_string()));
        assert_eq!(c.pre_include_global_h, Some("#include \"usb_config.h\"".to_string()));
        assert_eq!(c.pre_include_local_component_h, None);

        let files = &c.files.files;
        assert_eq!(files.len(), 4);
        assert_eq!(files[0], ComponentFile {
            name: "USB/Include/rl_usb.h".to_string(),
            category: "header".to_string(),
            attr: None, condition: None, version: None, select: None,
            src: None, path: None, language: None,
            scope: Some("public".to_string()),
            public: None, projectpath: None,
        });
        assert_eq!(files[1].attr, Some("config".to_string()));
        assert_eq!(files[1].version, Some("6.15.0".to_string()));
        assert_eq!(files[1].select, Some("USB Config".to_string()));
        assert_eq!(files[2].condition, Some("USB_Cond".to_string()));
        assert_eq!(files[2].src, Some("USB/Source/usbd_core.c".to_string()));
        assert_eq!(files[2].language, Some("c".to_string()));
        assert_eq!(files[2].public, Some(false));
        assert_eq!(files[2].projectpath, Some("USB/Source".to_string()));
        assert_eq!(files[3].name, "https://www.keil.com/pack/doc/mw/USB/html/index.html");
        assert_eq!(files[3].category, "doc");

        let exts = &c.extensions.extensions;
        assert_eq!(exts.len(), 2);
        assert_eq!(exts[0], ComponentExtension { key: "schemaVersion".to_string(), value: Some("2.0".to_string()) });
        assert_eq!(exts[1], ComponentExtension { key: "category".to_string(), value: Some("middleware".to_string()) });

        let envs = c.environments.as_ref().unwrap();
        assert_eq!(envs.environments, vec![ComponentEnvironment {
            name: "uv".to_string(),
            processor_name: Some("Core0".to_string()),
        }]);
    }
}
