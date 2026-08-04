//! Contains types representing debug sequences

use std::fmt::Debug;
use roxmltree::Document;
use serde::{Deserialize, Serialize};

use crate::requirements::Requirements;
use crate::generators::Generators;
use crate::boards::Boards;
use crate::parts::Parts;
use crate::taxonomy::Taxonomy;
use crate::part_taxonomy::PartTaxonomy;
use crate::apis::Apis;
use crate::components::Components;
use crate::conditions::Conditions;
use crate::csolution::Csolution;
use crate::examples::Examples;
use crate::family::Family;

#[derive(Debug, PartialEq, Deserialize)]
/// Represents [PDSC Package](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_package_pg.html)
/// which is the root element of the PDSC file
#[serde(rename_all = "camelCase")]
pub struct Package<'a> {
    /// Name of the software pack
    pub name: String,

    /// Name of the software pack supplier/vendor
    pub vendor: String,

    /// PDSC schema version; valid values defined by [PDSC schema versioning](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    pub schema_version: Option<String>,

    /// Restricts pack to a specific core; valid values: [DcoreEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dcore")]
    pub d_core: Option<String>,

    /// Restricts pack to a specific silicon vendor; valid values: [DeviceVendorEnum](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Dvendor")]
    pub d_vendor: Option<String>,

    /// Restricts pack to a specific device name; wildcards allowed
    #[serde(rename = "Dname")]
    pub d_name: Option<String>,

    /// Restricts pack to a specific toolchain; valid values: [CompilerEnumType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/packFormat.html)
    #[serde(rename = "Tcompiler")]
    pub t_compiler: Option<String>,

    /// Brief description of the sofware pack
    pub description: Description,

    /// Export Control Classification Numbers for the EU and US
    pub eccn: Option<Eccn>,

    /// URL or file URI of the sotware pack
    pub url: String,

    /// URL or e-main for users to get support for the Pack content
    pub support_contact: Option<String>,

    /// Path to the license document of the Pack
    pub license: Option<String>,

    /// Listing containing the collection of license fils
    pub license_sets: Option<LicenseSets>,

    /// A pack that has dominate attribute overrules other packs
    pub dominate: Option<Dominate>,

    /// Specifies other CMSIS-Packs, programming languages, and compilers required by pack components
    pub requirements: Option<Requirements>,

    // The deprecated `create` element is intentionally not modelled.

    /// HTTPS URL of a public repository tat the pack originates from
    pub repository: Option<Repository>,

    /// Version release history with brief information about a software pack
    pub releases: Releases,

    /// Section describing one or more changelog files
    pub changelogs: Option<Changelogs>,

    /// Keywords that might be used to find a software pack
    pub keywords: Option<Keywords>,

    /// Grouping elements for environments information.
    pub environments: Option<Environments>,

    /// Specifies generator tools that have been used to generate components
    pub generators: Option<Generators>,

    /// Development boards described in this pack
    pub boards: Option<Boards>,

    /// Hardware parts described in this pack
    pub parts: Option<Parts>,

    /// Component class and group taxonomy for this pack
    pub taxonomy: Option<Taxonomy>,

    /// Hardware part class and group taxonomy for this pack
    #[serde(rename = "part-taxonomy")]
    pub part_taxonomy: Option<PartTaxonomy>,

    /// Application programming interfaces defined by this pack
    pub apis: Option<Apis>,

    #[serde(borrow)]
    /// The device family, the devices, and variants
    pub devices: Devices<'a>,

    /// Conditions defined for use throughout this pack
    pub conditions: Option<Conditions>,

    /// Example projects included in this pack
    pub examples: Option<Examples>,

    /// Software layers and project templates for csolution-based projects
    pub csolution: Option<Csolution>,

    /// Components published by this pack
    pub components: Option<Components>,
}

impl<'a> Package<'a> {
    pub fn new(document: &'a Document) -> Self {
        // Parse the content
        let mut package: Package = serde_roxmltree::from_doc(&document).unwrap();

        // Parse the "wild" string conents into structured data
        for family in &mut package.devices.families {
            family.debugvars.parse_debugvars();
            family.sequences.parse_sequences();
        }

        // Return the data
        package
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC Descrpiton](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_package_description.html)
/// element.
pub struct Description {
    /// File path, file name, and file extension with an overview of documentation in markdown format
    pub overview: Option<String>,

    /// The description body contaning a brief markdown desrciption
    #[serde(rename = "#content")]
    pub content: Option<String>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC ECCN](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_ECCN.html) element
pub struct Eccn {
    #[serde(rename = "ECCN-EU")]
    pub eu: String,
    #[serde(rename = "ECCN-US")]
    pub us: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Representation of the [PDSC LicenseSets](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_licenseSets_pg.html#element_licenseSets)
/// element
pub struct LicenseSets {
    #[serde(rename = "licenseSet")]
    pub license_set: Vec<LicenseSet>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC LicenseSetsType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_licenseSets_pg.html#element_licenseSets)
pub struct LicenseSet {
    /// License set identifier string, must be uniqe in the PDSC file
    pub id: String,
    /// If set to true this license set is associated with all content, not explicitly referencing another license set
    pub default: Option<bool>,
    /// If set to true this license set is required to be accepted by the user before installation starts.
    pub gating: Option<bool>,
    /// Description of the license file refeneces
    pub license: Vec<License>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC License](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_licenseSets_pg.html#element_licensefile)
/// element.
///
/// Contains a description of an individual license file
pub struct License {
    /// License filename with pack base directory relative path
    pub name: String,

    /// Display sting used by tools to represent the license
    pub title: String,

    /// Machine readable licence ID string according to the [SPDX License List](https://spdx.org/licenses/)
    pub spdx: Option<String>,

    /// Public web link to the license text
    pub url: Option<String>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC Dominate](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_dominate.html) element
pub struct Dominate {
    /// Descriptive text that explains the reason for dominate
    pub info: Option<String>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC Repository](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_repository.html) element
pub struct Repository {
    #[serde(rename = "type")]
    pub repository_type: String,

    #[serde(rename = "#content")]
    pub url: String
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
/// Represents the [PDSC Releases](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_releases.html) element
pub struct Releases {
    pub release: Vec<Release>
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
/// Represents the [PDSC Releaes](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_releases.html#element_release) element
pub struct Release {
    /// Release version string
    pub version: String,
    /// Release date in xs:date format (e.g. `2023-01-15`)
    pub date: Option<String>,
    /// VCS tag for this release
    pub tag: Option<String>,
    /// URL of the release archive
    pub url: Option<String>,
    /// Date this release was deprecated, in xs:date format (e.g. `2023-01-15`); tools should warn users when the pack is installed
    pub deprecated: Option<String>,
    /// Pack name (`Vendor.Name`) that replaces this deprecated release
    pub replacement: Option<String>,
    #[serde(rename = "#content")]
    pub content: String
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC ChangelogsType](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_changelogs_pg.html#element_changelogs) element
pub struct Changelogs {
    pub changelog: Vec<Changelog>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC Changelog](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_changelogs_pg.html#element_changelog) element
pub struct Changelog {
    /// Changelog identifier string, must be uniqe within the PDSC file
    pub id: String,

    /// A path relative to the PDSC fil and the filename of the changelog file
    pub name: String,

    /// If `true` this changelog is associated with all APIs and components that do not explicitly reference another changelog; xs:boolean (`"true"` / `"false"`)
    pub default: Option<bool>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC Keywords](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_keywords.html) element
pub struct Keywords {
    /// The vector of keywords
    #[serde(rename = "keyword")]
    pub keywords: Vec<String>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC Envrionments](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_components_pg.html#element_component_environments) element
pub struct Environments {
    /// The vector of environments
    #[serde(rename = "environment")]
    pub environments: Vec<Environment>
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// Represents the [PDSC Environment](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_environment) element
pub struct Environment {
    pub name: String,
    #[serde(rename = "Pname")]
    pub processor_name: Option<String>,
    // TODO: Handle the `anyAttribute` children
}

/// Represents [PDSC Devices](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_devices_pg.html)
#[derive(Debug, PartialEq, Deserialize)]
pub struct Devices<'a> {
    /// Device family definitions (1..*)
    #[serde(rename = "family", default)]
    #[serde(borrow)]
    pub families: Vec<Family<'a>>
}

#[cfg(test)]
mod tests {
    use std::default;

use roxmltree::Document;
use serde_roxmltree::RawNode;

use crate::{debug_access::{Assignment, DebugFunction, Expression, Statement::{self}}, pdsc::{Changelog, Changelogs, Devices, Eccn, License, LicenseSet, Release, Releases, Repository}};
    #[test]
    fn parse_eccn() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<ECCN>
    <ECCN-EU>NEC</ECCN-EU>
    <ECCN-US>5D992.c</ECCN-US>
</ECCN>"#;

        let eccn: Eccn = serde_roxmltree::from_str(xml_str).unwrap();

        assert_eq!(eccn.eu, "NEC".to_string());
        assert_eq!(eccn.us, "5D992.c".to_string());
    }

    #[test]
    /// Test that ECCN parsing fails if either the EU or US field is missing.
    ///
    /// As per the Open [CMSIS Pack specification](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/element_ECCN.html)
    /// if the ECCN field is present it must contain both the US and EU ECCN entries.
    fn parse_eccn_missing_field() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<ECCN>
    <ECCN-US>5D992.c</ECCN-US>
</ECCN>"#;

        let eccn: Result<Eccn, _> = serde_roxmltree::from_str(xml_str);

        assert!(eccn.is_err());

        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<ECCN>
    <ECCN-EU>NEC</ECCN-EU>
</ECCN>"#;

        let eccn: Result<Eccn, _> = serde_roxmltree::from_str(xml_str);

        assert!(eccn.is_err());
    }

    #[test]
    fn parse_license_set() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<licenseSet id="all" default="true" gating="false">
    <license name="LICENSE.txt"
        title="Apache License, Version 2.0"
        spdx="Apache-2.0"
        url="https://www.apache.org/licenses/LICENSE-2.0"/>
</licenseSet>"#;

        let license_set: LicenseSet = serde_roxmltree::from_str(xml_str).unwrap();

        assert_eq!(license_set, LicenseSet {
            id: "all".to_string(),
            default: Some(true),
            gating: Some(false),
            license: vec![
                License {
                    name: "LICENSE.txt".to_string(),
                    title: "Apache License, Version 2.0".to_string(),
                    spdx: Some("Apache-2.0".to_string()),
                    url: Some("https://www.apache.org/licenses/LICENSE-2.0".to_string())
                }
            ]
        });
    }

    #[test]
    fn parse_repository() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<repository type="git">https://github.com/ARM-software/CMSIS-Driver.git</repository>"#;

        let repo: Repository = serde_roxmltree::from_str(xml_str).unwrap();

        assert_eq!(repo.repository_type, "git".to_string());
        assert_eq!(repo.url, "https://github.com/ARM-software/CMSIS-Driver.git".to_string());

    }

    #[test]
    fn parse_releases() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<releases>
  <release version="1.1.1" date="2020-05-12">Fixed a problem with the feature xyz.</release>
  <release version="1.1.0" date="2020-03-13">Introduces a new feature xyz.</release>
  <release version="1.0.0" date="2020-02-23">First published version.</release>
</releases>
"#;

        let releases: Releases = serde_roxmltree::from_str(xml_str).unwrap();

        assert_eq!(releases, Releases { release: vec![
            Release {
                version: "1.1.1".to_string(),
                date: Some("2020-05-12".to_string()),
                content: "Fixed a problem with the feature xyz.".to_string(),
                ..default::Default::default()
            },
            Release {
                version: "1.1.0".to_string(),
                date: Some("2020-03-13".to_string()),
                content: "Introduces a new feature xyz.".to_string(),
                ..default::Default::default()
            },
            Release {
                version: "1.0.0".to_string(),
                date: Some("2020-02-23".to_string()),
                content: "First published version.".to_string(),
                ..default::Default::default()
            },
        ]});
    }

    #[test]
    fn parse_releases_public_repo() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<releases>
    <release version="2.1.0" tag="2.1.0" url="https://github.com/ARM-software/CMSIS-Driver/archive/2.1.0.zip">
      Added LAN9220 Ethernet MAC+PHY driver.
    </release>
    <release version="2.0.0" tag="2.0.0" url="https://github.com/ARM-software/CMSIS-Driver/archive/2.0.0.zip">
      First published version.
    </release>
</releases>"#;

        let releases: Releases = serde_roxmltree::from_str(xml_str).unwrap();

        assert_eq!(releases, Releases { release: vec![
            Release {
                version: "2.1.0".to_string(),
                tag: Some("2.1.0".to_string()),
                url: Some("https://github.com/ARM-software/CMSIS-Driver/archive/2.1.0.zip".to_string()),
                content: "\n      Added LAN9220 Ethernet MAC+PHY driver.\n    ".to_string(),
                ..default::Default::default()
            },
            Release {
                version: "2.0.0".to_string(),
                tag: Some("2.0.0".to_string()),
                url: Some("https://github.com/ARM-software/CMSIS-Driver/archive/2.0.0.zip".to_string()),
                content: "\n      First published version.\n    ".to_string(),
                ..default::Default::default()
            },
        ]});
    }

    #[test]
    fn parse_releases_deprecated() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<releases>
  <release version="1.0.1" date="2020-04-18" deprecated="2020-04-18" replacement="Vendor.pack_name">
  </release>
  <release version="1.0.0" date="2020-03-24">Initial version.
  </release>
</releases>"#;

        let releases: Releases = serde_roxmltree::from_str(xml_str).unwrap();

        assert_eq!(releases, Releases { release: vec![
            Release {
                version: "1.0.1".to_string(),
                date: Some("2020-04-18".to_string()),
                deprecated: Some("2020-04-18".to_string()),
                replacement: Some("Vendor.pack_name".to_string()),
                content: "\n  ".to_string(),
                ..default::Default::default()
            },
            Release {
                version: "1.0.0".to_string(),
                date: Some("2020-03-24".to_string()),
                content: "Initial version.\n  ".to_string(),
                ..default::Default::default()
            },
        ]});
    }

    #[test]
    fn parse_devices_multiple_families() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<devices>
    <family Dfamily="FamilyA" Dvendor="ARM:82">
        <debugvars configfile="a.dbgconf" version="1.0.0"></debugvars>
        <sequences/>
    </family>
    <family Dfamily="FamilyB" Dvendor="ARM:82">
        <debugvars configfile="b.dbgconf" version="2.0.0"></debugvars>
        <sequences/>
    </family>
</devices>"#;
        // Devices borrows from the Document so the document must outlive the parsed value
        let document = roxmltree::Document::parse(xml_str).unwrap();
        let devices: Devices = serde_roxmltree::from_doc(&document).unwrap();
        assert_eq!(devices.families.len(), 2);
        assert_eq!(devices.families[0].device_family, "FamilyA");
        assert_eq!(devices.families[1].device_family, "FamilyB");
    }

    #[test]
    fn parse_changelog_default_bool() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<changelogs>
    <changelog id="all" name="CHANGELOG.md" default="true"/>
    <changelog id="other" name="OTHER.md"/>
</changelogs>"#;

        let changelogs: Changelogs = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(changelogs.changelog.len(), 2);
        assert_eq!(changelogs.changelog[0].id, "all");
        assert_eq!(changelogs.changelog[0].name, "CHANGELOG.md");
        assert_eq!(changelogs.changelog[0].default, Some(true));
        assert_eq!(changelogs.changelog[1].id, "other");
        assert_eq!(changelogs.changelog[1].default, None);
    }
}