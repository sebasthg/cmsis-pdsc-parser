//! Contains the types required to represent a [PDSC Generators](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html) element

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC generators](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_generators) element
pub struct Generators {
    /// The list of generator tool descriptions
    #[serde(rename = "generator")]
    pub generators: Vec<Generator>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC generator](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_generator) element
pub struct Generator {
    /// Unique identifier for this generator, referenced by components via `Gname`
    pub id: String,

    /// Silicon vendor associated with the generator tool
    #[serde(rename = "Gvendor")]
    pub generator_vendor: Option<String>,

    /// Plain-text name of the generator tool
    #[serde(rename = "Gtool")]
    pub generator_tool: Option<String>,

    /// Version of the generator tool
    #[serde(rename = "Gversion")]
    pub generator_version: Option<String>,

    /// Brief description of the generator (max 256 characters)
    pub description: Option<String>,

    /// Device or variant the generator targets
    pub select: Option<Select>,

    /// Output directory for generated files; supports `$P`, `$S` substitution variables
    #[serde(rename = "workingDir")]
    pub working_dir: Option<String>,

    /// Path and filename of the generated GPDSC file produced by the tool
    pub gpdsc: Option<Gpdsc>,

    /// Native executable invocation configuration
    pub exe: Option<Exe>,

    /// Eclipse plug-in invocation configuration
    pub eclipse: Option<Eclipse>,

    /// Web service invocation configuration
    pub web: Option<Web>,

    /// Generated project files that the IDE should add to the project after generation
    pub project_files: Option<ProjectFiles>,

    /// Generator tool files (executables, libraries, etc.) that ship inside the pack
    pub files: Option<Files>,

    /// Deprecated; use `exe.command` instead
    pub command: Option<String>,

    /// Deprecated; use `exe.argument` inside `exe` instead
    pub arguments: Option<GeneratorArguments>,
    // TODO: extensions — vendor-specific extension section, requires RawNode handling
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC select](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_gen_select) element
///
/// Specifies the device or device variant this generator targets.
/// Either `device_name` or `device_variant` must be present.
pub struct Select {
    /// Silicon vendor of the targeted device (e.g. `"STMicroelectronics:13"`)
    #[serde(rename = "Dvendor")]
    pub device_vendor: String,

    /// Device name or wildcard pattern; required if `device_variant` is absent
    #[serde(rename = "Dname")]
    pub device_name: Option<String>,

    /// Device variant; required if `device_name` is absent
    #[serde(rename = "Dvariant")]
    pub device_variant: Option<String>,

    /// Processor name for multi-core devices
    #[serde(rename = "Pname")]
    pub processor_name: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC gpdsc](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_gen_gpdsc) element
///
/// Identifies the GPDSC file the generator produces. Supports `$P` substitution.
pub struct Gpdsc {
    /// Path and filename of the generated GPDSC file relative to `workingDir`
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC exe](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_gen_exe) element
///
/// Defines native executable invocation of the generator tool. Up to four
/// platform-specific `<command>` entries may be provided.
pub struct Exe {
    /// Platform-specific command lines used to invoke the generator (1..4)
    #[serde(rename = "command")]
    pub commands: Vec<Command>,

    /// Arguments appended to the command line
    #[serde(rename = "argument")]
    pub arguments: Vec<Argument>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC eclipse](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_gen_eclipse) element
///
/// Defines an Eclipse plug-in invocation of the generator tool.
pub struct Eclipse {
    /// Eclipse plug-in identifier (e.g. `"com.vendor.generator"`)
    pub plugin: String,

    /// Fully-qualified Java class within the plug-in to invoke
    pub class: String,

    /// Method within the class to call
    pub method: String,

    /// Arguments passed to the Eclipse plug-in method
    #[serde(rename = "argument")]
    pub arguments: Vec<Argument>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents the [PDSC web](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_gen_web) element
///
/// Defines a web service invocation of the generator tool.
pub struct Web {
    /// URL of the web service endpoint to invoke
    pub url: String,

    /// Query parameters or arguments passed to the web service
    #[serde(rename = "argument", default)]
    pub arguments: Vec<Argument>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC command](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_gen_command) element
///
/// A single platform-specific command line that invokes the generator executable.
pub struct Command {
    /// Target host platform; one of `all`, `win`, `linux`, `mac`, `other`
    pub host: Option<String>,

    /// Command line string, including path to the executable; supports substitution variables
    #[serde(rename = "#content")]
    pub command: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC argument](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_gen_argument) element
///
/// A single argument passed to an `exe`, `eclipse`, or `web` invocation.
pub struct Argument {
    /// Invocation mode; one of `normal` (default) or `dry-run`
    pub mode: Option<String>,

    /// Target host platform (exe only); one of `all`, `win`, `linux`, `mac`, `other`
    pub host: Option<String>,

    /// Command-line switch prefix prepended to the argument value (e.g. `"--project"`)
    pub switch: Option<String>,

    /// The argument value; supports substitution variables
    #[serde(rename = "#content")]
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Wrapper for the [PDSC project_files](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_gen_project_files) element
///
/// Lists the files produced by the generator that the IDE should include in the project.
pub struct ProjectFiles {
    /// Generated files to add to the project after generation completes
    #[serde(rename = "file", default)]
    pub files: Vec<File>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Wrapper for the [PDSC files](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_gen_files) element
///
/// Lists generator tool files (executables, libraries, scripts) that ship inside the pack.
pub struct Files {
    /// Generator tool files included in the pack
    #[serde(rename = "file", default)]
    pub files: Vec<File>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Represents a [PDSC file](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_generators_pg.html#element_gen_file) entry in [`ProjectFiles`] or [`Files`]
pub struct File {
    /// File path relative to the pack base directory; supports substitution variables
    pub name: String,

    /// File category (e.g. `sourceC`, `sourceAsm`, `header`, `library`, `other`)
    pub category: String,

    /// Condition identifier that controls when this file is included
    pub condition: Option<String>,

    /// Version of the file
    pub version: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
/// Deprecated arguments wrapper; use `exe.argument` instead
pub struct GeneratorArguments {
    /// Individual argument strings (0..*)
    #[serde(rename = "argument", default)]
    pub argument: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::generators::{
        Argument, Command, Eclipse, Exe, File, Files, Generators, Gpdsc, ProjectFiles, Select, Web,
    };

    #[test]
    fn parse_generators() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<generators>
    <generator id="STCubeMX" Gvendor="STMicroelectronics" Gtool="STM32CubeMX" Gversion="6.0.0">
        <description>STM32CubeMX code generator</description>
        <select Dvendor="STMicroelectronics:13" Dname="STM32*" Pname="Cortex-M4"/>
        <workingDir>$P</workingDir>
        <gpdsc name="$P/MyProject.gpdsc"/>
        <exe>
            <command host="win">$S/CubeMX/cubemx.exe</command>
            <command host="linux">$S/CubeMX/cubemx</command>
            <argument switch="--project">$P</argument>
        </exe>
        <project_files>
            <file name="main.c" category="sourceC"/>
        </project_files>
        <files>
            <file name="cubemx.exe" category="other" condition="Win" version="6.0.0"/>
        </files>
    </generator>
</generators>"#;

        let generators: Generators = serde_roxmltree::from_str(xml_str).unwrap();

        assert_eq!(generators.generators.len(), 1);

        let generator = &generators.generators[0];
        assert_eq!(generator.id, "STCubeMX");
        assert_eq!(
            generator.generator_vendor,
            Some("STMicroelectronics".to_string())
        );
        assert_eq!(generator.generator_tool, Some("STM32CubeMX".to_string()));
        assert_eq!(generator.generator_version, Some("6.0.0".to_string()));
        assert_eq!(
            generator.description,
            Some("STM32CubeMX code generator".to_string())
        );
        assert_eq!(generator.working_dir, Some("$P".to_string()));
        assert_eq!(
            generator.select,
            Some(Select {
                device_vendor: "STMicroelectronics:13".to_string(),
                device_name: Some("STM32*".to_string()),
                device_variant: None,
                processor_name: Some("Cortex-M4".to_string()),
            })
        );
        assert_eq!(
            generator.gpdsc,
            Some(Gpdsc {
                name: "$P/MyProject.gpdsc".to_string()
            })
        );
        assert_eq!(
            generator.exe,
            Some(Exe {
                commands: vec![
                    Command {
                        host: Some("win".to_string()),
                        command: "$S/CubeMX/cubemx.exe".to_string()
                    },
                    Command {
                        host: Some("linux".to_string()),
                        command: "$S/CubeMX/cubemx".to_string()
                    },
                ],
                arguments: vec![Argument {
                    mode: None,
                    host: None,
                    switch: Some("--project".to_string()),
                    value: "$P".to_string()
                },],
            })
        );
        assert_eq!(
            generator.project_files,
            Some(ProjectFiles {
                files: vec![File {
                    name: "main.c".to_string(),
                    category: "sourceC".to_string(),
                    condition: None,
                    version: None,
                }],
            })
        );
        assert_eq!(
            generator.files,
            Some(Files {
                files: vec![File {
                    name: "cubemx.exe".to_string(),
                    category: "other".to_string(),
                    condition: Some("Win".to_string()),
                    version: Some("6.0.0".to_string()),
                }],
            })
        );
        assert_eq!(generator.eclipse, None);
        assert_eq!(generator.web, None);
    }

    #[test]
    fn parse_generator_eclipse() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<generators>
    <generator id="MyEclipseGen">
        <eclipse plugin="com.example.generator" class="com.example.Generator" method="generate">
            <argument switch="--device">$D</argument>
            <argument mode="dry-run">--dry-run</argument>
        </eclipse>
    </generator>
</generators>"#;

        let generators: Generators = serde_roxmltree::from_str(xml_str).unwrap();
        let generator = &generators.generators[0];

        assert_eq!(generator.id, "MyEclipseGen");
        assert_eq!(
            generator.eclipse,
            Some(Eclipse {
                plugin: "com.example.generator".to_string(),
                class: "com.example.Generator".to_string(),
                method: "generate".to_string(),
                arguments: vec![
                    Argument {
                        mode: None,
                        host: None,
                        switch: Some("--device".to_string()),
                        value: "$D".to_string()
                    },
                    Argument {
                        mode: Some("dry-run".to_string()),
                        host: None,
                        switch: None,
                        value: "--dry-run".to_string()
                    },
                ],
            })
        );
        assert_eq!(generator.exe, None);
        assert_eq!(generator.web, None);
    }

    #[test]
    fn parse_generator_web() {
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<generators>
    <generator id="MyWebGen">
        <web url="https://generator.example.com/api">
            <argument switch="--board">$B</argument>
        </web>
    </generator>
</generators>"#;

        let generators: Generators = serde_roxmltree::from_str(xml_str).unwrap();
        let generator = &generators.generators[0];

        assert_eq!(generator.id, "MyWebGen");
        assert_eq!(
            generator.web,
            Some(Web {
                url: "https://generator.example.com/api".to_string(),
                arguments: vec![Argument {
                    mode: None,
                    host: None,
                    switch: Some("--board".to_string()),
                    value: "$B".to_string()
                },],
            })
        );
        assert_eq!(generator.exe, None);
        assert_eq!(generator.eclipse, None);
    }

    #[test]
    fn parse_generator_deprecated_command_arguments() {
        // Tests that the deprecated top-level <command> and <arguments> children
        // of a <generator> element are captured correctly.
        let xml_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<generators>
    <generator id="OldStyleGen">
        <description>Legacy generator using deprecated command/arguments elements</description>
        <command>$S/tools/oldgen</command>
        <arguments>
            <argument>--project</argument>
            <argument>$P</argument>
        </arguments>
    </generator>
</generators>"#;

        let generators: Generators = serde_roxmltree::from_str(xml_str).unwrap();
        assert_eq!(generators.generators.len(), 1);

        let generator = &generators.generators[0];
        assert_eq!(generator.id, "OldStyleGen");
        assert_eq!(generator.command, Some("$S/tools/oldgen".to_string()));
        let args = generator
            .arguments
            .as_ref()
            .expect("arguments should be present");
        assert_eq!(
            args.argument,
            vec!["--project".to_string(), "$P".to_string(),]
        );
        assert_eq!(generator.exe, None);
    }
}
