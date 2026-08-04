//! Types representing  [PDSC Debug Access](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#block_DebugSyntaxRules)

use serde::{Deserialize, Serialize};

/// Parse error for debug access XML elements.
#[derive(Debug, PartialEq)]
pub enum DebugAccessParseError {
    /// A required attribute or structural element was absent.
    MissingAttribute(String),
    /// An unrecognised statement or function name was encountered.
    UnknownStatement(String),
}

impl std::fmt::Display for DebugAccessParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingAttribute(msg) => write!(f, "missing attribute: {msg}"),
            Self::UnknownStatement(name) => write!(f, "unknown statement: {name}"),
        }
    }
}

impl std::error::Error for DebugAccessParseError {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Types representing the valid [PDSC Debug Access](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#block_DebugSyntaxRules)
/// statements.
pub enum Statement {
    /// A sole expression, e.g. `expression;`
    Expression(Expression),

    /// A variable assignment, e.g. `variable = expression;`
    Assignment(Assignment),

    /// A variable definition, e.g. `__var variable = 0;`
    Definition(Assignment),

    /// Comment, e.g. `// This is a comment`
    Comment(String),
}

impl From<String> for Statement {
    fn from(value: String) -> Self {
        // If present trim any whitespace
        let input = value.trim().to_string();

        // Check if it is a comment
        if input.starts_with("//") {
            return Statement::Comment(input);
        }

        // If present remove the semicolon
        let input = input.strip_suffix(";").unwrap_or(&input).to_string();

        // Check if this is an assignment or declaration
        let split: Vec<&str> = input.split_inclusive("=").collect();
        match split.len() {
            0 => unreachable!("String split should never return 0 elements"),
            1 => {
                // No '=', must be a standalone expression
                let expression: Expression = input.try_into().unwrap();
                return Self::Expression(expression);
            },
            _ => {
                let variable = split[0]
                    .strip_suffix('=')
                    // Safety: This should be unreachable
                    .expect("Missing '=' after inclusive split on '='")
                    .trim();

                let expression: Expression = split[1..]
                    .iter()
                    .copied()
                    .collect::<String>()
                    .trim()
                    .into();

                match variable.strip_prefix("__var ") {
                    Some(variable) => Statement::Definition(
                        Assignment { variable: variable.to_string(), expression }
                    ),
                    None => Statement::Assignment(
                        Assignment { variable: variable.to_string(), expression }
                    )
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// A variable assignment, e.g. `variable = expression;`
pub struct Assignment {
    pub variable: String,
    pub expression: Expression
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// A variable representing a [PDSC Expression](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#block_ExpressionType)
pub enum Expression {
    /// An arithmetic, bitwise, or comparison expression, e.g. `2 + 2`, `reg & 0xFF`, `x == 1`, or a bare variable reference
    Normal(String),

    /// An expression representing an inline if statement, e.g. `(x < y) ? a : b`
    ///
    /// # Note
    ///
    /// The parser currently does not handle nested conditionals, e.g. `(x < y) ? ( (a < b) ? c : d ) : e`
    /// I hope noone has written a PDSC file which does this, if so this can be implemented.
    Conditional(Box<Conditional>),

    /// A call to a predefined [PDSC debug access function](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/debug_description.html#DebugFunctions),
    /// e.g. `Read32(0x40000000)` or `Sequence("ResetAndHalt")`
    FunctionCall(Box<DebugFunction>),
}

impl From<String> for Expression {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        if let Ok(condition) = Conditional::try_from(value) {
            return Self::Conditional(Box::new(condition));
        }

        if let Some((name, args_str)) = detect_function_call(value) {
            let args: Vec<Expression> = split_args(args_str)
                .into_iter()
                .map(Expression::from)
                .collect();
            let func = DebugFunction::try_from((name.to_string(), args))
                .unwrap_or_else(|e| panic!("{}", e));
            return Expression::FunctionCall(Box::new(func));
        }

        Expression::Normal(value.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// An expression representing an inline if statement, e.g. `(x < y) ? a : b`
///
/// # Note
///
/// The parser currently does not handle nested conditionals, e.g. `(x < y) ? ( (a < b) ? c : d ) : e`
/// I hope noone has written a PDSC file which does this, if so this can be implemented.
pub struct Conditional {
    /// The conditional part, `(x < y) ? a : b -> x < y`
    pub condition: Expression,
    /// The value when the conditional evaluates to true, `(x < y) ? a : b -> a`
    pub true_value: Expression,
    /// The value when the conditional evaluates to false, `(x < y) ? a : b -> b`
    pub false_value: Expression
}

impl TryFrom<String> for Conditional {
    type Error = DebugAccessParseError;

    /// Performs the conversion between [String] and [Conditional]
    ///
    /// # Note
    ///
    /// The parser currently does not handle nested conditionals, e.g. `(x < y) ? ( (a < b) ? c : d ) : e`
    /// I hope noone has written a PDSC file which does this, if so this can be implemented.
    /// This will return a valid type with a garbage value.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Conditional {
    type Error = DebugAccessParseError;

    /// Performs the conversion between [&str] and [Conditional]
    ///
    /// # Note
    ///
    /// The parser currently does not handle nested conditionals, e.g. `(x < y) ? ( (a < b) ? c : d ) : e`
    /// I hope noone has written a PDSC file which does this, if so this can be implemented.
    /// This will return a valid type with a garbage value.
    fn try_from(value: &str) -> Result<Self, Self::Error> {

        // Create the sates for the state machine
        #[derive(Debug, PartialEq)]
        enum WalkerProgress {
            None,
            ParenOpen,
            ParenClose,
            Question,
            Colon
        }

        // Variables to store the result
        let mut condition_str: String = String::new();
        let mut truthy_str: String = String::new();
        let mut falsey_str: String = String::new();

        // Use a state machine to walk the string
        let mut progress = WalkerProgress::None;
        for c in value.chars() {
            match progress {
                WalkerProgress::None => if c == '(' { progress = WalkerProgress::ParenOpen; },
                WalkerProgress::ParenOpen => if c == ')' { progress = WalkerProgress::ParenClose; } else { condition_str.push(c); },
                WalkerProgress::ParenClose => if c == '?' { progress = WalkerProgress::Question; },
                WalkerProgress::Question => if c == ':' { progress = WalkerProgress::Colon; } else { truthy_str.push(c); },
                WalkerProgress::Colon => if c == ';' { break; } else { falsey_str.push(c); }
            }
        }

        let walk_ok = progress == WalkerProgress::Colon && !falsey_str.is_empty();

        if walk_ok {
            let condition: Expression = condition_str.trim().into();
            let true_value: Expression = truthy_str.trim().into();
            let false_value: Expression = falsey_str.trim().into();

            Ok(Conditional { condition, true_value, false_value })
        } else {
            Err(DebugAccessParseError::MissingAttribute(
                "conditional syntax: expected '(condition) ? truthy : falsy'".to_string()
            ))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// A predefined [PDSC debug access function](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/debug_description.html#DebugFunctions).
/// Unknown function names are a parse error — if the spec adds new functions they will surface as panics.
pub enum DebugFunction {
    // Memory access
    /// Read 8-bit value from target memory
    Read8  { addr: Expression },
    /// Read 16-bit value from target memory
    Read16 { addr: Expression },
    /// Read 32-bit value from target memory
    Read32 { addr: Expression },
    /// Read 64-bit value from target memory
    Read64 { addr: Expression },
    /// Write 8-bit value to target memory
    Write8  { addr: Expression, val: Expression },
    /// Write 16-bit value to target memory
    Write16 { addr: Expression, val: Expression },
    /// Write 32-bit value to target memory
    Write32 { addr: Expression, val: Expression },
    /// Write 64-bit value to target memory
    Write64 { addr: Expression, val: Expression },

    // Register access
    /// Read access port register
    ReadAP  { addr: Expression },
    /// Write access port register
    WriteAP { addr: Expression, val: Expression },
    /// Read debug port register
    ReadDP  { addr: Expression },
    /// Write debug port register
    WriteDP { addr: Expression, val: Expression },
    /// APv2/ADIv6 access port read
    ReadAccessAP  { addr: Expression },
    /// APv2/ADIv6 access port write
    WriteAccessAP { addr: Expression, val: Expression },

    // Debug port / probe
    /// Wait for a specific delay (microseconds)
    DapDelay      { delay: Expression },
    /// Write abort request to CoreSight register
    DapWriteAbort { value: Expression },
    /// Monitor and control debugger I/O pins
    DapSwjPins    { pinout: Expression, pinselect: Expression, pinwait: Expression },
    /// Set JTAG/SWD clock frequency (Hz)
    DapSwjClock   { val: Expression },
    /// Generate SWJ sequences
    DapSwjSequence  { cnt: Expression, val: Expression },
    /// Generate JTAG sequences
    DapJtagSequence { cnt: Expression, tms: Expression, tdi: Expression },

    // Sequence control
    /// Execute a debug access sequence by name
    Sequence   { name: Expression },
    /// Prompt user for confirmation or selection
    Query      { query_type: Expression, message: Expression, default: Expression },
    /// Query an input value from the user
    QueryValue { message: Expression, default: Expression },
    /// Output a formatted message to the debug log (variadic: `msg_type`, `format`, then optional extra args)
    Message    { msg_type: Expression, format: Expression, args: Vec<Expression> },

    // Flash operations
    /// Write flash buffer contents into target memory
    FlashWriteBuffer   { addr: Expression, offs: Expression, len: Expression, mode: Expression },
    /// Select FLM flash algorithm for operations
    FlashLoadAlgorithm { algo_path: Expression, ram_start: Expression, ram_size: Expression },

    // Buffer management
    /// Fill buffer with a value pattern
    BufferSet   { buff_id: Expression, buff_offset: Expression, count: Expression, size: Expression, value: Expression },
    /// Retrieve an item from a buffer
    BufferGet   { buff_id: Expression, buff_offset: Expression, size: Expression },
    /// Get current buffer size in bytes
    BufferSize  { buff_id: Expression },
    /// Read target data into a buffer
    BufferRead  { buff_id: Expression, buff_offset: Expression, addr: Expression, length: Expression, mode: Expression },
    /// Transfer buffer data to target
    BufferWrite { buff_id: Expression, buff_offset: Expression, addr: Expression, length: Expression, mode: Expression },

    // External tool integration
    /// Stream data from an external source into a buffer
    BufferStreamIn  { buff_id: Expression, buff_offset: Expression, length: Expression, path: Expression, mode: Expression, timeout: Expression },
    /// Transfer buffer data to an external sink
    BufferStreamOut { buff_id: Expression, buff_offset: Expression, length: Expression, dest_path: Expression, dest_mode: Expression, timeout: Expression },
    /// Execute an external application
    RunApplication  { app_path: Expression, arguments: Expression, work_directory: Expression, timeout: Expression },
    /// Run a Python script on the host system
    RunPythonScript { script_path: Expression, arguments: Expression, work_directory: Expression, timeout: Expression },
    /// Check if a path exists on the host filesystem
    FilePathExists  { path: Expression, timeout: Expression },
    /// Load DWARF debug information
    LoadDebugInfo   { file: Expression },
}

impl TryFrom<(String, Vec<Expression>)> for DebugFunction {
    type Error = DebugAccessParseError;

    /// Parses a debug access function by name and argument list.
    ///
    /// Returns [Err] if the function name is not in the CMSIS-Pack spec or the argument count is wrong.
    fn try_from((name, args): (String, Vec<Expression>)) -> Result<Self, Self::Error> {
        match name.as_str() {
            // Memory — 1 arg (addr)
            "Read8"  => match <[Expression; 1]>::try_from(args) {
                Ok([addr]) => Ok(DebugFunction::Read8  { addr }),
                Err(v)     => Err(DebugAccessParseError::MissingAttribute(format!("Read8 expects 1 argument, got {}", v.len()))),
            },
            "Read16" => match <[Expression; 1]>::try_from(args) {
                Ok([addr]) => Ok(DebugFunction::Read16 { addr }),
                Err(v)     => Err(DebugAccessParseError::MissingAttribute(format!("Read16 expects 1 argument, got {}", v.len()))),
            },
            "Read32" => match <[Expression; 1]>::try_from(args) {
                Ok([addr]) => Ok(DebugFunction::Read32 { addr }),
                Err(v)     => Err(DebugAccessParseError::MissingAttribute(format!("Read32 expects 1 argument, got {}", v.len()))),
            },
            "Read64" => match <[Expression; 1]>::try_from(args) {
                Ok([addr]) => Ok(DebugFunction::Read64 { addr }),
                Err(v)     => Err(DebugAccessParseError::MissingAttribute(format!("Read64 expects 1 argument, got {}", v.len()))),
            },
            // Memory — 2 args (addr, val)
            "Write8"  => match <[Expression; 2]>::try_from(args) {
                Ok([addr, val]) => Ok(DebugFunction::Write8  { addr, val }),
                Err(v)          => Err(DebugAccessParseError::MissingAttribute(format!("Write8 expects 2 arguments, got {}", v.len()))),
            },
            "Write16" => match <[Expression; 2]>::try_from(args) {
                Ok([addr, val]) => Ok(DebugFunction::Write16 { addr, val }),
                Err(v)          => Err(DebugAccessParseError::MissingAttribute(format!("Write16 expects 2 arguments, got {}", v.len()))),
            },
            "Write32" => match <[Expression; 2]>::try_from(args) {
                Ok([addr, val]) => Ok(DebugFunction::Write32 { addr, val }),
                Err(v)          => Err(DebugAccessParseError::MissingAttribute(format!("Write32 expects 2 arguments, got {}", v.len()))),
            },
            "Write64" => match <[Expression; 2]>::try_from(args) {
                Ok([addr, val]) => Ok(DebugFunction::Write64 { addr, val }),
                Err(v)          => Err(DebugAccessParseError::MissingAttribute(format!("Write64 expects 2 arguments, got {}", v.len()))),
            },
            // Register — 1 arg (addr)
            "ReadAP" => match <[Expression; 1]>::try_from(args) {
                Ok([addr]) => Ok(DebugFunction::ReadAP { addr }),
                Err(v)     => Err(DebugAccessParseError::MissingAttribute(format!("ReadAP expects 1 argument, got {}", v.len()))),
            },
            "ReadDP" => match <[Expression; 1]>::try_from(args) {
                Ok([addr]) => Ok(DebugFunction::ReadDP { addr }),
                Err(v)     => Err(DebugAccessParseError::MissingAttribute(format!("ReadDP expects 1 argument, got {}", v.len()))),
            },
            "ReadAccessAP" => match <[Expression; 1]>::try_from(args) {
                Ok([addr]) => Ok(DebugFunction::ReadAccessAP { addr }),
                Err(v)     => Err(DebugAccessParseError::MissingAttribute(format!("ReadAccessAP expects 1 argument, got {}", v.len()))),
            },
            // Register — 2 args (addr, val)
            "WriteAP" => match <[Expression; 2]>::try_from(args) {
                Ok([addr, val]) => Ok(DebugFunction::WriteAP { addr, val }),
                Err(v)          => Err(DebugAccessParseError::MissingAttribute(format!("WriteAP expects 2 arguments, got {}", v.len()))),
            },
            "WriteDP" => match <[Expression; 2]>::try_from(args) {
                Ok([addr, val]) => Ok(DebugFunction::WriteDP { addr, val }),
                Err(v)          => Err(DebugAccessParseError::MissingAttribute(format!("WriteDP expects 2 arguments, got {}", v.len()))),
            },
            "WriteAccessAP" => match <[Expression; 2]>::try_from(args) {
                Ok([addr, val]) => Ok(DebugFunction::WriteAccessAP { addr, val }),
                Err(v)          => Err(DebugAccessParseError::MissingAttribute(format!("WriteAccessAP expects 2 arguments, got {}", v.len()))),
            },
            // Debug port — 1 arg
            "DAP_Delay" => match <[Expression; 1]>::try_from(args) {
                Ok([delay]) => Ok(DebugFunction::DapDelay { delay }),
                Err(v)      => Err(DebugAccessParseError::MissingAttribute(format!("DAP_Delay expects 1 argument, got {}", v.len()))),
            },
            "DAP_WriteABORT" => match <[Expression; 1]>::try_from(args) {
                Ok([value]) => Ok(DebugFunction::DapWriteAbort { value }),
                Err(v)      => Err(DebugAccessParseError::MissingAttribute(format!("DAP_WriteABORT expects 1 argument, got {}", v.len()))),
            },
            "DAP_SWJ_Clock" => match <[Expression; 1]>::try_from(args) {
                Ok([val]) => Ok(DebugFunction::DapSwjClock { val }),
                Err(v)    => Err(DebugAccessParseError::MissingAttribute(format!("DAP_SWJ_Clock expects 1 argument, got {}", v.len()))),
            },
            // Debug port — 2 args
            "DAP_SWJ_Sequence" => match <[Expression; 2]>::try_from(args) {
                Ok([cnt, val]) => Ok(DebugFunction::DapSwjSequence { cnt, val }),
                Err(v)         => Err(DebugAccessParseError::MissingAttribute(format!("DAP_SWJ_Sequence expects 2 arguments, got {}", v.len()))),
            },
            // Debug port — 3 args
            "DAP_SWJ_Pins" => match <[Expression; 3]>::try_from(args) {
                Ok([pinout, pinselect, pinwait]) => Ok(DebugFunction::DapSwjPins { pinout, pinselect, pinwait }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("DAP_SWJ_Pins expects 3 arguments, got {}", v.len()))),
            },
            "DAP_JTAG_Sequence" => match <[Expression; 3]>::try_from(args) {
                Ok([cnt, tms, tdi]) => Ok(DebugFunction::DapJtagSequence { cnt, tms, tdi }),
                Err(v)              => Err(DebugAccessParseError::MissingAttribute(format!("DAP_JTAG_Sequence expects 3 arguments, got {}", v.len()))),
            },
            // Sequence control — 1 arg
            "Sequence" => match <[Expression; 1]>::try_from(args) {
                Ok([name]) => Ok(DebugFunction::Sequence { name }),
                Err(v)     => Err(DebugAccessParseError::MissingAttribute(format!("Sequence expects 1 argument, got {}", v.len()))),
            },
            // Sequence control — 2 args
            "QueryValue" => match <[Expression; 2]>::try_from(args) {
                Ok([message, default]) => Ok(DebugFunction::QueryValue { message, default }),
                Err(v)                 => Err(DebugAccessParseError::MissingAttribute(format!("QueryValue expects 2 arguments, got {}", v.len()))),
            },
            // Sequence control — 3 args
            "Query" => match <[Expression; 3]>::try_from(args) {
                Ok([query_type, message, default]) => Ok(DebugFunction::Query { query_type, message, default }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("Query expects 3 arguments, got {}", v.len()))),
            },
            // Sequence control — variadic (2+ args)
            "Message" => {
                if args.len() < 2 {
                    return Err(DebugAccessParseError::MissingAttribute(format!("Message expects at least 2 arguments, got {}", args.len())));
                }
                let mut it = args.into_iter();
                let msg_type = it.next().unwrap();
                let format_expr = it.next().unwrap();
                Ok(DebugFunction::Message { msg_type, format: format_expr, args: it.collect() })
            },
            // Flash — 3 args
            "FlashLoadAlgorithm" => match <[Expression; 3]>::try_from(args) {
                Ok([algo_path, ram_start, ram_size]) => Ok(DebugFunction::FlashLoadAlgorithm { algo_path, ram_start, ram_size }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("FlashLoadAlgorithm expects 3 arguments, got {}", v.len()))),
            },
            // Flash — 4 args
            "FlashWriteBuffer" => match <[Expression; 4]>::try_from(args) {
                Ok([addr, offs, len, mode]) => Ok(DebugFunction::FlashWriteBuffer { addr, offs, len, mode }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("FlashWriteBuffer expects 4 arguments, got {}", v.len()))),
            },
            // Buffer — 1 arg
            "BufferSize" => match <[Expression; 1]>::try_from(args) {
                Ok([buff_id]) => Ok(DebugFunction::BufferSize { buff_id }),
                Err(v)        => Err(DebugAccessParseError::MissingAttribute(format!("BufferSize expects 1 argument, got {}", v.len()))),
            },
            // Buffer — 3 args
            "BufferGet" => match <[Expression; 3]>::try_from(args) {
                Ok([buff_id, buff_offset, size]) => Ok(DebugFunction::BufferGet { buff_id, buff_offset, size }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("BufferGet expects 3 arguments, got {}", v.len()))),
            },
            // Buffer — 5 args
            "BufferSet" => match <[Expression; 5]>::try_from(args) {
                Ok([buff_id, buff_offset, count, size, value]) => Ok(DebugFunction::BufferSet { buff_id, buff_offset, count, size, value }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("BufferSet expects 5 arguments, got {}", v.len()))),
            },
            "BufferRead" => match <[Expression; 5]>::try_from(args) {
                Ok([buff_id, buff_offset, addr, length, mode]) => Ok(DebugFunction::BufferRead { buff_id, buff_offset, addr, length, mode }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("BufferRead expects 5 arguments, got {}", v.len()))),
            },
            "BufferWrite" => match <[Expression; 5]>::try_from(args) {
                Ok([buff_id, buff_offset, addr, length, mode]) => Ok(DebugFunction::BufferWrite { buff_id, buff_offset, addr, length, mode }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("BufferWrite expects 5 arguments, got {}", v.len()))),
            },
            // External — 1 arg
            "LoadDebugInfo" => match <[Expression; 1]>::try_from(args) {
                Ok([file]) => Ok(DebugFunction::LoadDebugInfo { file }),
                Err(v)     => Err(DebugAccessParseError::MissingAttribute(format!("LoadDebugInfo expects 1 argument, got {}", v.len()))),
            },
            // External — 2 args
            "FilePathExists" => match <[Expression; 2]>::try_from(args) {
                Ok([path, timeout]) => Ok(DebugFunction::FilePathExists { path, timeout }),
                Err(v)              => Err(DebugAccessParseError::MissingAttribute(format!("FilePathExists expects 2 arguments, got {}", v.len()))),
            },
            // External — 4 args
            "RunApplication" => match <[Expression; 4]>::try_from(args) {
                Ok([app_path, arguments, work_directory, timeout]) => Ok(DebugFunction::RunApplication { app_path, arguments, work_directory, timeout }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("RunApplication expects 4 arguments, got {}", v.len()))),
            },
            "RunPythonScript" => match <[Expression; 4]>::try_from(args) {
                Ok([script_path, arguments, work_directory, timeout]) => Ok(DebugFunction::RunPythonScript { script_path, arguments, work_directory, timeout }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("RunPythonScript expects 4 arguments, got {}", v.len()))),
            },
            // External — 6 args
            "BufferStreamIn" => match <[Expression; 6]>::try_from(args) {
                Ok([buff_id, buff_offset, length, path, mode, timeout]) => Ok(DebugFunction::BufferStreamIn { buff_id, buff_offset, length, path, mode, timeout }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("BufferStreamIn expects 6 arguments, got {}", v.len()))),
            },
            "BufferStreamOut" => match <[Expression; 6]>::try_from(args) {
                Ok([buff_id, buff_offset, length, dest_path, dest_mode, timeout]) => Ok(DebugFunction::BufferStreamOut { buff_id, buff_offset, length, dest_path, dest_mode, timeout }),
                Err(v) => Err(DebugAccessParseError::MissingAttribute(format!("BufferStreamOut expects 6 arguments, got {}", v.len()))),
            },
            _ => Err(DebugAccessParseError::UnknownStatement(name)),
        }
    }
}

/// Returns `Some((name, args_str))` if `s` matches `identifier(...)`, otherwise `None`.
///
/// `name` is the function name; `args_str` is the raw content between the outer parentheses.
fn detect_function_call(s: &str) -> Option<(&str, &str)> {
    if !s.ends_with(')') {
        return None;
    }

    let paren_pos = s.find('(')?;
    let name = &s[..paren_pos];

    // Validate name is a non-empty identifier [A-Za-z_][A-Za-z0-9_]*
    let mut name_chars = name.chars();
    let first = name_chars.next()?;
    if !first.is_alphabetic() && first != '_' {
        return None;
    }
    if !name_chars.all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    let args_str = &s[paren_pos + 1..s.len() - 1];
    Some((name, args_str))
}

/// Splits a comma-separated argument string into trimmed segments, respecting nested parentheses.
///
/// e.g. `"addr, Read32(base)"` → `["addr", "Read32(base)"]`
fn split_args(args_str: &str) -> Vec<&str> {
    if args_str.trim().is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut depth = 0u32;
    let mut start = 0;

    for (i, c) in args_str.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                result.push(args_str[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }

    let last = args_str[start..].trim();
    if !last.is_empty() {
        result.push(last);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::debug_access::{Assignment, Conditional, DebugAccessParseError, DebugFunction, Expression, Statement};

    #[test]
    fn parse_comment() {
        let line = "// This is a comment!".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Comment("// This is a comment!".to_string()));
    }

    #[test]
    fn semicolon_handling() {
        let line1 = "Read32(0x10)".to_string();
        let line2 = "Read32(0x10);".to_string();

        let statement1: Statement = line1.into();
        let statement2: Statement = line2.into();

        assert_eq!(statement1, statement2);
    }

    #[test]
    fn parse_expression_normal() {
        let line = "addr + offset;".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Expression(
            Expression::Normal("addr + offset".to_string())
        ));
    }

    #[test]
    fn parse_expression_normal_variable() {
        let line = "doIfBlock".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Expression(
            Expression::Normal("doIfBlock".to_string())
        ));
    }

    #[test]
    fn parse_expression_conditional() {
        let line = "(x < y) ? a : b".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Expression(
            Expression::Conditional(Box::new(Conditional {
                condition: Expression::Normal("x < y".to_string()),
                true_value: Expression::Normal("a".to_string()),
                false_value: Expression::Normal("b".to_string())
            }))
        ));
    }

    #[test]
    fn parse_assignment_comparison() {
        let line = "thisValue = (readTheCoolRegister(0x248) == 5);".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Assignment(
            Assignment {
                variable: "thisValue".to_string(),
                expression: Expression::Normal("(readTheCoolRegister(0x248) == 5)".to_string())
            }
        ));
    }

    #[test]
    fn parse_assignment() {
        let line = "variable = expression;".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Assignment(
            Assignment {
                expression: Expression::Normal("expression".to_string()),
                variable: "variable".to_string(),
            }
        ))
    }

    #[test]
    fn parse_definition() {
        let line = "__var variable = 0;".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Definition(
            Assignment {
                expression: Expression::Normal("0".to_string()),
                variable: "variable".to_string(),
            }
        ))
    }

    #[test]
    fn parse_function_call_single_arg() {
        let line = "Read32(0x40000000);".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Expression(Expression::FunctionCall(Box::new(
            DebugFunction::Read32 { addr: Expression::Normal("0x40000000".to_string()) }
        ))));
    }

    #[test]
    fn parse_function_call_two_args() {
        let line = "Write32(addr, val);".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Expression(Expression::FunctionCall(Box::new(
            DebugFunction::Write32 {
                addr: Expression::Normal("addr".to_string()),
                val:  Expression::Normal("val".to_string()),
            }
        ))));
    }

    #[test]
    fn parse_function_call_string_arg() {
        let line = "Sequence(\"ResetAndHalt\");".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Expression(Expression::FunctionCall(Box::new(
            DebugFunction::Sequence { name: Expression::Normal("\"ResetAndHalt\"".to_string()) }
        ))));
    }

    #[test]
    fn parse_function_call_three_args() {
        let line = "DAP_SWJ_Pins(pinout, pinselect, pinwait);".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Expression(Expression::FunctionCall(Box::new(
            DebugFunction::DapSwjPins {
                pinout:    Expression::Normal("pinout".to_string()),
                pinselect: Expression::Normal("pinselect".to_string()),
                pinwait:   Expression::Normal("pinwait".to_string()),
            }
        ))));
    }

    #[test]
    fn parse_function_call_variadic() {
        let line = "Message(1, \"debug message\");".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Expression(Expression::FunctionCall(Box::new(
            DebugFunction::Message {
                msg_type: Expression::Normal("1".to_string()),
                format:   Expression::Normal("\"debug message\"".to_string()),
                args:     vec![],
            }
        ))));
    }

    #[test]
    fn parse_function_call_nested_arg() {
        // Read32(base) is an argument to Write32 — split_args must not split on the inner comma
        let line = "Write32(addr, Read32(base));".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Expression(Expression::FunctionCall(Box::new(
            DebugFunction::Write32 {
                addr: Expression::Normal("addr".to_string()),
                val:  Expression::FunctionCall(Box::new(DebugFunction::Read32 {
                    addr: Expression::Normal("base".to_string()),
                })),
            }
        ))));
    }

    #[test]
    #[should_panic(expected = "unknown statement: GetBase")]
    fn unknown_function_panics() {
        let _: Expression = "GetBase()".into();
    }

    #[test]
    fn conditional_missing_syntax() {
        let result = Conditional::try_from("no parentheses here");
        assert!(matches!(result, Err(DebugAccessParseError::MissingAttribute(_))));
    }

    #[test]
    fn unknown_function_returns_unknown_statement() {
        use crate::debug_access::Expression;
        let result = DebugFunction::try_from(("GetBase".to_string(), vec![]));
        assert_eq!(result.unwrap_err(), DebugAccessParseError::UnknownStatement("GetBase".to_string()));
    }
}
