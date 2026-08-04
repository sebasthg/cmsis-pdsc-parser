//! Contains the types required to represent a [PDSC Conditions](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_conditions_pg.html#element_conditions) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC conditions](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_conditions_pg.html#element_conditions) element
///
/// Groups all condition definitions used to conditionally include components, files, and other pack content.
pub struct Conditions {
    /// Condition definitions (1..*)
    #[serde(rename = "condition", default)]
    pub conditions: Vec<Condition>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents a [PDSC condition](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_conditions_pg.html#element_condition) element
///
/// A condition groups `accept`, `require`, and `deny` filter rules identified by a unique `id`.
pub struct Condition {
    /// Unique identifier for this condition within the pack
    pub id: String,

    /// Human-readable explanation of what the condition tests
    pub description: Option<String>,

    /// OR-rules: at least one `accept` filter must evaluate to true
    #[serde(rename = "accept", default)]
    pub accept: Vec<Filter>,

    /// AND-rules: all `require` filters must evaluate to true
    #[serde(rename = "require", default)]
    pub require: Vec<Filter>,

    /// AND-NOT-rules: no `deny` filter may evaluate to true
    #[serde(rename = "deny", default)]
    pub deny: Vec<Filter>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the shared filter attributes used by
/// [`<accept>`](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_conditions_pg.html#element_accept),
/// [`<require>`](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_conditions_pg.html#element_require), and
/// [`<deny>`](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_conditions_pg.html#element_deny) elements
///
/// All attributes are optional; at least one must be set in a valid filter.
pub struct Filter {
    /// Device vendor (e.g. `ARM:82`, `STMicroelectronics:13`)
    #[serde(rename = "Dvendor")]
    pub device_vendor: Option<String>,

    /// Device name; supports wildcards `*`, `?`, `[abc]`
    #[serde(rename = "Dname")]
    pub device_name: Option<String>,

    /// Processor name for multi-core devices
    #[serde(rename = "Pname")]
    pub processor_name: Option<String>,

    /// Processor core (e.g. `Cortex-M0`, `Cortex-M4`)
    #[serde(rename = "Dcore")]
    pub device_core: Option<String>,

    /// Floating-point unit type
    #[serde(rename = "Dfpu")]
    pub fpu: Option<String>,

    /// Memory protection unit presence
    #[serde(rename = "Dmpu")]
    pub mpu: Option<String>,

    /// TrustZone support
    #[serde(rename = "Dtz")]
    pub trustzone: Option<String>,

    /// Secure state support
    #[serde(rename = "Dsecure")]
    pub secure: Option<String>,

    /// DSP instruction set support
    #[serde(rename = "Ddsp")]
    pub dsp: Option<String>,

    /// M-Profile Vector Extension (Helium) support
    #[serde(rename = "Dmve")]
    pub mve: Option<String>,

    /// Custom Datapath Extension co-processor support
    #[serde(rename = "Dcdecp")]
    pub cdecp: Option<String>,

    /// Pointer Authentication and Branch Target Identification support
    #[serde(rename = "Dpacbti")]
    pub pacbti: Option<String>,

    /// Byte endianness (e.g. `Little-endian`, `Big-endian`)
    #[serde(rename = "Dendian")]
    pub endian: Option<String>,

    /// Component vendor
    #[serde(rename = "Cvendor")]
    pub component_vendor: Option<String>,

    /// Component bundle name
    #[serde(rename = "Cbundle")]
    pub bundle: Option<String>,

    /// Component class
    #[serde(rename = "Cclass")]
    pub class: Option<String>,

    /// Component group
    #[serde(rename = "Cgroup")]
    pub group: Option<String>,

    /// Component sub-group
    #[serde(rename = "Csub")]
    pub sub: Option<String>,

    /// Component variant
    #[serde(rename = "Cvariant")]
    pub variant: Option<String>,

    /// Component version range
    #[serde(rename = "Cversion")]
    pub version: Option<String>,

    /// API version range
    #[serde(rename = "Capiversion")]
    pub api_version: Option<String>,

    /// Board vendor
    #[serde(rename = "Bvendor")]
    pub board_vendor: Option<String>,

    /// Board name
    #[serde(rename = "Bname")]
    pub board_name: Option<String>,

    /// Board revision
    #[serde(rename = "Brevision")]
    pub board_revision: Option<String>,

    /// Hardware part vendor
    #[serde(rename = "Hvendor")]
    pub part_vendor: Option<String>,

    /// Hardware part name
    #[serde(rename = "Hname")]
    pub part_name: Option<String>,

    /// Compiler toolchain (e.g. `GCC`, `ARMCC`, `IAR`)
    #[serde(rename = "Tcompiler")]
    pub compiler: Option<String>,

    /// Compiler options or mode (e.g. `AC5`, `AC6`)
    #[serde(rename = "Toptions")]
    pub compiler_options: Option<String>,

    /// References another condition by its `id`; this filter is true only if that condition is true
    pub condition: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::conditions::Conditions;

    #[test]
    fn parse_conditions() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<conditions>
    <condition id="CM4">
        <description>Requires ARM Cortex-M4 with FPU</description>
        <accept Dcore="Cortex-M4" Dfpu="SP_FPU"/>
        <accept Dcore="Cortex-M4" Dfpu="DP_FPU"/>
        <require Tcompiler="GCC"/>
        <deny Dcore="Cortex-M0"/>
    </condition>
    <condition id="GCC_Only">
        <require Tcompiler="GCC"/>
    </condition>
</conditions>"#;

        let conds: Conditions = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(conds.conditions.len(), 2);

        let c0 = &conds.conditions[0];
        assert_eq!(c0.id, "CM4");
        assert_eq!(c0.description, Some("Requires ARM Cortex-M4 with FPU".to_string()));
        assert_eq!(c0.accept.len(), 2);
        assert_eq!(c0.accept[0].device_core, Some("Cortex-M4".to_string()));
        assert_eq!(c0.accept[0].fpu, Some("SP_FPU".to_string()));
        assert_eq!(c0.accept[1].device_core, Some("Cortex-M4".to_string()));
        assert_eq!(c0.accept[1].fpu, Some("DP_FPU".to_string()));
        assert_eq!(c0.require.len(), 1);
        assert_eq!(c0.require[0].compiler, Some("GCC".to_string()));
        assert_eq!(c0.deny.len(), 1);
        assert_eq!(c0.deny[0].device_core, Some("Cortex-M0".to_string()));

        let c1 = &conds.conditions[1];
        assert_eq!(c1.id, "GCC_Only");
        assert_eq!(c1.description, None);
        assert_eq!(c1.accept.len(), 0);
        assert_eq!(c1.require.len(), 1);
        assert_eq!(c1.deny.len(), 0);
    }

    #[test]
    fn parse_condition_minimal() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<conditions>
    <condition id="Empty"/>
</conditions>"#;

        let conds: Conditions = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(conds.conditions.len(), 1);

        let c = &conds.conditions[0];
        assert_eq!(c.id, "Empty");
        assert_eq!(c.description, None);
        assert_eq!(c.accept, vec![]);
        assert_eq!(c.require, vec![]);
        assert_eq!(c.deny, vec![]);
    }

    #[test]
    fn parse_condition_filters() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<conditions>
    <condition id="Complex">
        <accept Dvendor="ARM:82" Dname="ARMCM4*" Dcore="Cortex-M4"
                Dfpu="SP_FPU" Dmpu="MPU" Dtz="TZ" Dsecure="Secure"
                Ddsp="DSP" Dmve="MVE" Dcdecp="CDECP" Dpacbti="PACBTI"
                Dendian="Little-endian" Pname="Core0"/>
        <require Cclass="CMSIS" Cgroup="RTOS2" Capiversion="2.0.0"
                 Cvendor="ARM" Cbundle="CMSIS" Csub="Core"
                 Cvariant="Release" Cversion="5.0.0"/>
        <require Bvendor="ARM" Bname="V2M-MPS2" Brevision="Rev.C"
                 Hvendor="NXP" Hname="LPC1768"
                 Tcompiler="GCC" Toptions="AC6"/>
        <deny condition="NoARM"/>
    </condition>
</conditions>"#;

        let conds: Conditions = serde_roxmltree::from_str(xml_str).unwrap();
        let c = &conds.conditions[0];
        assert_eq!(c.id, "Complex");

        // Accept filter — device attributes
        let a = &c.accept[0];
        assert_eq!(a.device_vendor, Some("ARM:82".to_string()));
        assert_eq!(a.device_name, Some("ARMCM4*".to_string()));
        assert_eq!(a.device_core, Some("Cortex-M4".to_string()));
        assert_eq!(a.fpu, Some("SP_FPU".to_string()));
        assert_eq!(a.mpu, Some("MPU".to_string()));
        assert_eq!(a.trustzone, Some("TZ".to_string()));
        assert_eq!(a.secure, Some("Secure".to_string()));
        assert_eq!(a.dsp, Some("DSP".to_string()));
        assert_eq!(a.mve, Some("MVE".to_string()));
        assert_eq!(a.cdecp, Some("CDECP".to_string()));
        assert_eq!(a.pacbti, Some("PACBTI".to_string()));
        assert_eq!(a.endian, Some("Little-endian".to_string()));
        assert_eq!(a.processor_name, Some("Core0".to_string()));
        assert_eq!(a.condition, None);

        // First require filter — component attributes
        let r0 = &c.require[0];
        assert_eq!(r0.class, Some("CMSIS".to_string()));
        assert_eq!(r0.group, Some("RTOS2".to_string()));
        assert_eq!(r0.api_version, Some("2.0.0".to_string()));
        assert_eq!(r0.component_vendor, Some("ARM".to_string()));
        assert_eq!(r0.bundle, Some("CMSIS".to_string()));
        assert_eq!(r0.sub, Some("Core".to_string()));
        assert_eq!(r0.variant, Some("Release".to_string()));
        assert_eq!(r0.version, Some("5.0.0".to_string()));
        assert_eq!(r0.device_vendor, None);

        // Second require filter — board/part/toolchain attributes
        let r1 = &c.require[1];
        assert_eq!(r1.board_vendor, Some("ARM".to_string()));
        assert_eq!(r1.board_name, Some("V2M-MPS2".to_string()));
        assert_eq!(r1.board_revision, Some("Rev.C".to_string()));
        assert_eq!(r1.part_vendor, Some("NXP".to_string()));
        assert_eq!(r1.part_name, Some("LPC1768".to_string()));
        assert_eq!(r1.compiler, Some("GCC".to_string()));
        assert_eq!(r1.compiler_options, Some("AC6".to_string()));
        assert_eq!(r1.class, None);

        // Deny filter — condition cross-reference
        let d = &c.deny[0];
        assert_eq!(d.condition, Some("NoARM".to_string()));
        assert_eq!(d.device_vendor, None);
    }
}
