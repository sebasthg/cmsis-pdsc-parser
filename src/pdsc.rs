//! Contains types representing debug sequences

use std::{collections::HashMap, fmt::Debug};
use roxmltree::{Document, Node};
use serde::{Deserialize, Serialize, de::value};
use log::{debug, error, trace, warn};
use serde_roxmltree::RawNode;

use crate::debug_access::{self, Statement};

#[derive(Debug, PartialEq, Deserialize)]
/// Represents [PDSC Package](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_package_pg.html)
/// which is the root element of the PDSC file
#[serde(rename_all = "camelCase")]
pub struct Package<'a> {
    /// Name of the software pack
    pub name: String,

    /// Name of the software pack supplier/vendor
    pub vendor: String,

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

    // TODO: Add requirements

    // TODO: Add deprecated option create

    // TODO: Add repository

    // TODO: Add releases

    // TODO: Add changelogs

    // TODO: Add keywords

    // TODO: Add environments

    // TODO: Add genertators

    #[serde(borrow)]
    /// The device family, the devices, and variants
    pub devices: Devices<'a>,

    // TODO: Add boards

    // TODO: Add parts

    // TODO: Add taxonomy

    // TODO: Add part-taxonomy

    // TODO: Add APIs

    // TODO: Add conditions

    // TODO: Add examples

    // TODO: Add csolution

    // TODO: Add components
}

impl<'a> Package<'a> {
    pub fn new(document: &'a Document) -> Self {
        // Parse the content
        let mut package: Package = serde_roxmltree::from_doc(&document).unwrap();

        // Parse the "wild" string conents into structured data
        package.devices.family.debugvars.parse_debugvars();
        package.devices.family.sequences.parse_sequences();

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
    info: Option<String>
}

#[derive(Debug, PartialEq, Deserialize)]
/// Represents [PDSC Devices](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_devices_pg.html)
pub struct Devices<'a> {
    #[serde(borrow)]
    pub family: Family<'a>
}

#[derive(Debug, PartialEq, Deserialize)]
/// Represents [PDSC Family](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html)
pub struct Family<'a> {
    #[serde(rename = "Dfamily")]
    /// The device family name
    pub device_family: String,

    #[serde(rename = "Dvendor")]
    /// The device manufacturer/vendor
    pub vendor: String, // TODO: Make enum

    /// Global debug variables
    pub debugvars: Debugvars,

    /// Debug sequences
    #[serde(borrow)]
    pub sequences: Sequences<'a>
}

#[derive(Debug, PartialEq, Deserialize)]
/// Represents the `traceSetput` attribute in [PDSC sequences](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_sequences)
enum TraceSetup {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "legacy")]
    Legacy
}

#[derive(Debug, PartialEq, Deserialize)]
/// Represents [PDSC Debugvars](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_debugvars)
pub struct Debugvars {
    /// The relative path to the configuration file containing debugvars
    pub configfile: Option<String>,

    /// Debugvars version
    pub version: Option<String>,

    #[serde(rename = "#content")]
    /// Debugvars variable declarations
    content: String,

    /// Parsed debugvars, initially `None`, generated by [Debugvars::parse_debugvars]
    pub parsed_debugvars: Option<HashMap<String, u64>>
}

impl Debugvars {
    /// Parses a debugvar value string into u64
    fn parse_value_string(value: &str) -> Option<u64> {
            // Try to parse the value as a hex value firs
            if let Some(hex_value_str) = value.strip_prefix("0x") {
                if let Ok(k) = u64::from_str_radix(hex_value_str, 16) {
                    return Some(k);
                }
            };

            // If not hex, parse as base-10
            match value.parse() {
                Ok(k) => return Some(k),
                Err(e) => {
                    error!("Failed to parse debugvar value ({}) to u64 with error: {}", value, e);
                    return None
                }
            }
    }

    /// Parses a single variable declaration entry
    fn parse_single_debugvar(line: &str) -> Option<(String, u64)> { // TODO: Make Result
        trace!("Parsing debugvar line: {}", line);

        // Remove comments
        let declaration = if line.trim_start().starts_with("//") {
            let parts: Vec<&str> = line.split("\n").collect();
            parts[1].trim()
        } else {
            line.trim()
        };

        let stripped_declaration = match declaration.strip_prefix("__var ") {
            Some(val) => val,
            None => {
                if declaration.len() != 0 {
                    warn!("Variable in debugvars does not start with \"__var \": {:?}", declaration);
                };
                return None;
            }
        };

        let parts: Vec<&str> = stripped_declaration.split("=").collect();
        if parts.len() != 2 {
            warn!("Got something other than 2 fields when parsing debugvar: {:?}", parts);
            return None;
        }

        let name: String = parts[0].trim().to_string();
        let value_str = parts[1].trim();

        if let Some(value) = Self::parse_value_string(value_str) {
            Some((name, value))
        } else {
            None
        }
    }

    /// Parses the debugvars content and returns a hashmap with the variable name as the key and the value as the value
    pub fn parse_debugvars_content(&self) -> HashMap<String, u64> {
        // Remove any lines starting with "//"
        let content: String = self.content.split("\n").filter_map(|line| {
            if line.trim().starts_with("//") {
                trace!("Ignoring line: {}", line);
                None
            } else {
                Some(line.to_owned() + "\n") // Add back the newline
            }
        }).collect();

        // Variables are split with a ';'
        // While unlikely the spec does not forbid multiple inline assignments, e.g.:
        //    __var foo = 5; __var bar = 0x42;
        // As such we must split on ';' rather than '\n' for actual parsing.
        let variables: Vec<&str> = content.split(';').collect();

        variables.iter().filter_map(|var| {
            // Parse the variable
            Self::parse_single_debugvar(var)
        }).collect()
    }

    /// Performs the parsing and stores values in [Self::parsed_debugvars]
    pub fn parse_debugvars(&mut self) {
        let vars = self.parse_debugvars_content();

        self.parsed_debugvars = Some(vars);
    }
}

#[derive(Debug, PartialEq, Deserialize)]
/// Represents [PDSC sequences](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_sequences)
pub struct Sequences<'a> {
    /// Trace setup configuration
    #[serde(rename = "traceSetup")]
    trace_setup: Option<TraceSetup>,

    /// Raw XML nodes representing debug sequences
    ///
    /// These are stored as [RawNode] due to [serde_roxmltree] not supporting decoding elements as
    /// a vector of enums and we neet do be able to preresent both [control](SequenceElement::Control)
    /// and [block](SequenceElement::Block) elements with their order perserved.
    #[serde(rename = "sequence")]
    #[serde(borrow)]
    raw_nodes: Vec<RawNode<'a>>,

    /// Debug Sequences
    #[serde(skip)]
    sequences: Vec<Sequence>
}

impl<'a> Sequences<'a> {
    /// Iteates through the raw nodes and parses the sequences
    pub fn parse_raw_nodes_content(&self) -> Vec<Sequence> {
        self.raw_nodes.iter().map(|node| {
            node.0.try_into().unwrap()
        }).collect()
    }

    /// Parses the raw XML Sequence nodes and stores the parsed sequences in [Self::sequences]
    pub fn parse_sequences(&mut self) {
        let sequences = Self::parse_raw_nodes_content(&self);

        self.sequences = sequences;
    }
}


#[derive(Debug, PartialEq, Deserialize)]
/// Represents [PDSC Sequence](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_sequence)
pub struct Sequence {
    /// The sequence name
    name: String,

    /// Processor name, if set only use the debug sequence for this processor
    processor_name: Option<String>,

    /// If set disable the [Predefined Debug Access Sequence](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/debug_description.html#default_sequences) of the same name
    disable: Option<bool>,

    /// Descriptive text about the sequence
    info: Option<String>,

    #[serde(skip)]
    elements: Vec<SequenceElement>
}

impl<'a, 'input: 'a> TryFrom<Node<'a, 'input>> for Sequence {
    type Error = String; // TODO: Proper error

    fn try_from(value: Node<'a, 'input>) -> Result<Self, Self::Error> {
        // Validate that this is a sequence node
        let node_name = value.tag_name().name();
        assert_eq!(node_name, "sequence");

        // Get the name
        let sequence_name = value.attribute("name").expect("Missing requeired field name");

        // Get the optional attributes
        let sequence_processor_name = value.attribute("Pname")
            .map_or_else(
                || None,
                |v| Some(v.to_string())
            );
        let sequence_disable = {
            if let Some(v) = value.attribute("disable") {
                let disable_value: bool = v.parse().expect("Non boolean value in disable field");
                Some(disable_value)
            } else {
                None
            }
        };
        let sequence_info = value.attribute("info")
            .map_or_else(
                || None,
                |v| Some(v.to_string())
            );

        // The sequence elements
        let mut elements: Vec<SequenceElement> = Vec::new();

        // Try to parse the child nodes
        for child in value.children().filter(|c| c.is_element()) {
            let element: SequenceElement = child.try_into().expect("Illegal child");

            elements.push(element);
        }

        Ok(Sequence {
            name: sequence_name.to_string(),
            processor_name: sequence_processor_name,
            disable: sequence_disable,
            info: sequence_info,
            elements
        })
    }
}

impl<'a, 'input: 'a> TryFrom<Node<'a, 'input>> for SequenceElement {
    type Error = String; // TODO: Proper error

    fn try_from(value: Node<'a, 'input>) -> Result<Self, Self::Error> {
        match value.tag_name().name().to_lowercase().as_str() {
            "block" => {
                Ok(
                    <Node<'_, '_> as TryInto<SequenceBlock>>::try_into(value)
                        .unwrap().into()
                )
            },
            "control" => {
                Ok(
                    <Node<'_, '_> as TryInto<SequenceControl>>::try_into(value)
                        .unwrap().into()
                )
            },
            _ => panic!("Failed to convert to sequence element")
        }
        // TODO: Parse content

    }
}

impl<'a, 'input: 'a> TryFrom<Node<'a, 'input>> for SequenceBlock {
    type Error = String; // TODO: Proper error

    fn try_from(value: Node<'a, 'input>) -> Result<Self, Self::Error> {
        let mut block: Self = serde_roxmltree::from_node(value).unwrap();
        block.parse_statements();
        Ok(block)
    }
}

impl<'a, 'input: 'a> TryFrom<Node<'a, 'input>> for SequenceControl {
    type Error = String; // TODO: Proper error

    fn try_from(value: Node<'a, 'input>) -> Result<Self, Self::Error> {
        // Use serde_roxmltree to parse the basic elements
        let mut block: Self = serde_roxmltree::from_node(value).unwrap();

        // Try to parse the child nodes
        for child in value.children().filter(|c| c.is_element()) {
            let element: SequenceElement = child.try_into().expect("Illegal child");

            block.elements.push(element);
        }

        // Parse the conditional into an Expression
        let conditional_string: &str;
        if let Some(ref val) = block.conditional_if {
            conditional_string = val.as_str()
        } else if let Some(ref val) = block.conditional_while {
            conditional_string = val.as_str()
        } else {
            return Err("Failed to get conditional string".to_string());
        };

        let conditional: debug_access::Expression = conditional_string.try_into().expect("Failed to parse conditional string");
        block.conditional = Some(conditional);

        Ok(block)
    }
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
 /// Represents the valid sequence child elements as defined in the "Child Elements" section of the [PDSC sequence element](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_sequence)
pub enum SequenceElement {
    Control(SequenceControl),
    Block(SequenceBlock),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
/// Represents a [PDSC Control Sequence](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_seq_control)
pub struct SequenceControl {
    /// If conditional
    #[serde(rename = "if")]
    conditional_if: Option<String>,

    /// While conditional
    #[serde(rename = "while")]
    conditional_while: Option<String>,

    /// Timeout in microseconds, a value of 0 is the same as None
    timeout: Option<u64>,

    /// Decsriptive text, e.g. for diagnostics
    info: Option<String>,

    #[serde(skip_deserializing)]
    /// The elements contained by the control block
    elements: Vec<SequenceElement>,

    #[serde(skip_deserializing)]
    /// The conditional parsed as an Expression
    ///
    /// The [Expression](debug_access::Expression) is wrapped in an [Option] due to
    /// provoding a default value for [serde] when deserializing. It should be safe
    /// to unwrap this value.
    conditional: Option<debug_access::Expression>
}

impl From<SequenceControl> for SequenceElement {
    fn from(value: SequenceControl) -> Self {
        SequenceElement::Control(value)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
/// Represents a [PDSC Block Sequence](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_seq_block)
pub struct SequenceBlock {
    /// If `Some(true)` the block must be executed atomically, see the description on CMSIS Pack website.
    atomic: Option<bool>,

    /// Decsriptive text, e.g. for diagnostics
    info: Option<String>,

    #[serde(rename = "#content")]
    /// Sequence block content
    content: String,

    #[serde(skip)]
    /// [Statement]s resulting from the parsing of [Self::content]
    statements: Vec<Statement>
}

impl SequenceBlock {
    /// Parses [Self::content] into a list of [Statement]s
    pub fn parse_statements_content(&self) -> Vec<Statement> {
        self.content.lines()
            .flat_map(|line| line.split(';'))
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| Statement::from(s.to_string()))
            .collect()
    }

    /// Parses the block content and stores the result in [Self::statements]
    pub fn parse_statements(&mut self) {
        self.statements = self.parse_statements_content();
    }
}

impl From<SequenceBlock> for SequenceElement {
    fn from(value: SequenceBlock) -> Self {
        SequenceElement::Block(value)
    }
}

#[cfg(test)]
mod sequence_tests {
    use roxmltree::Document;
use serde_roxmltree::RawNode;

use crate::{debug_access::{Assignment, DebugFunction, Expression, Statement::{self}}, pdsc::{Eccn, License, LicenseSet, Sequence, SequenceBlock, SequenceControl, SequenceElement}};

    #[test]
    fn basic_sequence() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<sequence name="ResetSystem">
    <block>
        Sequence("ResetAndHalt");
    </block>
</sequence>"#;

        let document = Document::parse(xml_str).unwrap();
        let sequence_node = document.root_element();
        let raw_node: RawNode = RawNode(sequence_node);

        let sequence: Sequence = raw_node.0.try_into().unwrap();

        assert_eq!(sequence.name, "ResetSystem".to_string());
        assert_eq!(sequence.elements, vec![
            SequenceBlock {
                atomic: None,
                info: None,
                content: "\n        Sequence(\"ResetAndHalt\");\n    ".to_string(),
                statements: vec![
                    Statement::Expression(Expression::FunctionCall(Box::new(DebugFunction::Sequence {
                        name: Expression::Normal("\"ResetAndHalt\"".to_string())
                    })))
                ]
            }.into()
        ]);
    }

    #[test]
    /// Tests the example sequence from https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#element_sequence
    fn full_sequence() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<sequence name="UserSequence">
    <block info="Define variables and do debug accesses">
        __var tpWidth = (__traceout &amp; 0x003F0000) >> 16;
    </block>

    <control if="__traceout &amp; 0x2" info="Parallel Trace Port enabled">
        <block>
            // Do something generic for parallel trace port trace
        </block>

        <control if="tpWidth == 1" info="Configure device for 1-bit TPIU trace.">
            <block>
                // Do debug accesses
            </block>
        </control>

        <control if="tpWidth == 2" info="Configure device for 2-bit TPIU trace.">
            <block>
                // Do debug accesses
            </block>
        </control>

        <control if="tpWidth == 4" info="Configure device for 4-bit TPIU trace.">
            <block>
                // Do debug accesses
            </block>
        </control>
    </control>
</sequence>"#;

        let document = Document::parse(xml_str).unwrap();
        let sequence_node = document.root_element();
        let raw_node: RawNode = RawNode(sequence_node);

        let sequence: Sequence = raw_node.0.try_into().unwrap();

        // Check basic info
        assert_eq!(sequence.name, "UserSequence".to_string());
        assert_eq!(sequence.disable, None);
        assert_eq!(sequence.info, None);
        assert_eq!(sequence.processor_name, None);

        // Define a repeated element for future use
        let debug_access_block: SequenceElement = SequenceBlock {
            atomic: None,
            info: None,
            content: r#"
                // Do debug accesses
            "#.to_string(),
            statements: vec![
                Statement::Comment("// Do debug accesses".to_string())
            ]
        }.into();

        // Check that the elements are correct and in the correct order
        let expected_elements: Vec<crate::pdsc::SequenceElement> = vec![
            SequenceBlock {
                atomic: None,
                info: Some("Define variables and do debug accesses".to_string()),
                content: r#"
        __var tpWidth = (__traceout & 0x003F0000) >> 16;
    "#.to_string(),
                statements: vec![
                    Statement::Definition(Assignment {
                        variable: "tpWidth".to_string(),
                        expression: Expression::Normal("(__traceout & 0x003F0000) >> 16".to_string())
                    })
                ]
            }.into(),
            SequenceControl {
                conditional_if: Some("__traceout & 0x2".to_string()),
                conditional_while: None,
                timeout: None,
                info: Some("Parallel Trace Port enabled".to_string()),
                elements: vec![
                    SequenceBlock {
                        atomic: None,
                        info: None,
                        content: r#"
            // Do something generic for parallel trace port trace
        "#.to_string(),
                        statements: vec![
                            Statement::Comment("// Do something generic for parallel trace port trace".to_string())
                        ]
                    }.into(),
                    SequenceControl {
                        conditional_if: Some("tpWidth == 1".to_string()),
                        conditional_while: None,
                        timeout: None,
                        info: Some("Configure device for 1-bit TPIU trace.".to_string()),
                        elements: vec![
                            debug_access_block.clone().into()
                        ],
                        conditional: Some(
                            Expression::Normal("tpWidth == 1".to_string())
                        )
                    }.into(),
                    SequenceControl {
                        conditional_if: Some("tpWidth == 2".to_string()),
                        conditional_while: None,
                        timeout: None,
                        info: Some("Configure device for 2-bit TPIU trace.".to_string()),
                        elements: vec![
                            debug_access_block.clone().into()
                        ],
                        conditional: Some(
                            Expression::Normal("tpWidth == 2".to_string())
                        )
                    }.into(),
                    SequenceControl {
                        conditional_if: Some("tpWidth == 4".to_string()),
                        conditional_while: None,
                        timeout: None,
                        info: Some("Configure device for 4-bit TPIU trace.".to_string()),
                        elements: vec![
                            debug_access_block.clone().into()
                        ],
                        conditional: Some(
                            Expression::Normal("tpWidth == 4".to_string())
                        )
                    }.into()
                ],
                conditional: Some(Expression::Normal("__traceout & 0x2".to_string()))
            }.into()
        ];

        println!("Expected: {:#?}", expected_elements);
        println!("Actual: {:#?}", sequence.elements);

        assert_eq!(sequence.elements, expected_elements);

    }

    #[test]
    fn basic_sequence_block() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<block info="Define condition variales for later use in block elements.">
    // Variable definition by __var keyword
    __var doIfBlock      = 1;
    __var whileCondition = 1;
</block>"#;

        let document = Document::parse(xml_str).unwrap();
        let sequence_node = document.root_element();
        let raw_node: RawNode = RawNode(sequence_node);

        let block: SequenceBlock = raw_node.0.try_into().unwrap();

        assert_eq!(block.info, Some("Define condition variales for later use in block elements.".to_string()));
        assert_eq!(block.atomic, None);
        assert_eq!(block.content, r#"
    // Variable definition by __var keyword
    __var doIfBlock      = 1;
    __var whileCondition = 1;
"#
        );
        assert_eq!(block.statements, vec![
            Statement::Comment("// Variable definition by __var keyword".to_string()),
            Statement::Definition(Assignment {
                variable: "doIfBlock".to_string(),
                expression: Expression::Normal("1".to_string())
            }),
            Statement::Definition(Assignment {
                variable: "whileCondition".to_string(),
                expression: Expression::Normal("1".to_string())
            })
        ]);
    }

    #[test]
    fn parse_control_element_if() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<control if="doIfBlock">
    <block>
        // Do debug accesses
    </block>
</control>"#;

        let document = Document::parse(xml_str).unwrap();
        let sequence_node = document.root_element();
        let raw_node: RawNode = RawNode(sequence_node);

        let block: SequenceControl = raw_node.0.try_into().unwrap();

        println!("{:#?}", block);
        assert_eq!(block.info, None);
        assert_eq!(block.conditional_if, Some("doIfBlock".to_string()));
        assert_eq!(block.conditional_while, None);
        assert_eq!(block.timeout, None);
        assert_eq!(block.elements, vec![
            SequenceBlock{
                atomic: None,
                info: None,
                content: "\n        // Do debug accesses\n    ".to_string(),
                statements: vec![
                    Statement::Comment("// Do debug accesses".to_string())
                ]
            }.into()
        ]);
    }

    #[test]
    fn parse_control_element_while() {
        let xml_str =
r#"<?xml version="1.0" encoding="UTF-8"?>
<control while="whileCondition" timeout="5000">
    <block>
        // Execute while "whileCondition" different from '0' with a timeout of 5ms
        whileCondition = 0;
    </block>
</control>"#;

        let document = Document::parse(xml_str).unwrap();
        let sequence_node = document.root_element();
        let raw_node: RawNode = RawNode(sequence_node);

        let block: SequenceControl = raw_node.0.try_into().unwrap();

        println!("{:#?}", block);
        assert_eq!(block.info, None);
        assert_eq!(block.conditional_if, None);
        assert_eq!(block.conditional_while, Some("whileCondition".to_string()));
        assert_eq!(block.timeout, Some(5000));
        assert_eq!(block.elements, vec![
            SequenceBlock{
                atomic: None,
                info: None,
                content: r#"
        // Execute while "whileCondition" different from '0' with a timeout of 5ms
        whileCondition = 0;
    "#.to_string(),
                statements: vec![
                    Statement::Comment("// Execute while \"whileCondition\" different from '0' with a timeout of 5ms".to_string()),
                    Statement::Assignment(Assignment {
                        variable: "whileCondition".to_string(),
                        expression: Expression::Normal("0".to_string())
                    })
                ]
            }.into()
        ]);
    }

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
}