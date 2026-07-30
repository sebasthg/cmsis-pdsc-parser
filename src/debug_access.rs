//! Types representing  [PDSC Debug Access](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#block_DebugSyntaxRules)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Types representing the valid [PDSC Debug Access](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#block_DebugSyntaxRules)
/// statements.
pub enum Statement {
    /// A sole expression, e.g. `expression;`
    Expression(Expression),

    /// A variable assignment, e.g. `variable = expression;`
    Assignment(Assignment),

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

        // Check if this is an assignment
        let split: Vec<&str> = input.split_inclusive("=").collect();
        println!("{:?}", split);
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
                    .expect("Missing '=' after inclusive split on '='")
                    .trim()
                    .to_string();

                let expression = split[1..]
                    .iter()
                    .copied()
                    .collect::<String>()
                    .trim()
                    .to_string();

                return Statement::Assignment(
                    Assignment { variable, expression }
                )
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// A variable assignment, e.g. `variable = expression;`
pub struct Assignment {
    pub variable: String,
    pub expression: String
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// A variable representing a [PDSC Expression](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#block_ExpressionType)
pub enum Expression {
    /// An expression with a normal operator, e.g. 2 + 2, foo == true, reg_val & 0xFF, myFunc()
    ///
    /// Note: Operator Expressions are stored as strings and not broken down further due to
    /// the [PDSC Expression Rules](https://open-cmsis-pack.github.io/Open-CMSIS-Pack-Spec/main/html/pdsc_family_pg.html#block_ExpressionType)
    /// being quite sensible and should be compatible with most target languages.
    /// If the need arises this can be implemented
    Normal(String),

    /// An expression representing an inline if statement, e.g. `(x < y) ? a : b`
    ///
    /// # Note
    ///
    /// The parser currently does not handle nested conditionals, e.g. `(x < y) ? ( (a < b) ? c : d ) : e`  
    /// I hope noone has written a PDSC file which does this, if so this can be implemented.
    Conditional(Box<Conditional>)
}

impl From<String> for Expression {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        if let Ok(condition) = Conditional::try_from(value) {
            Self::Conditional(Box::new(condition))
        } else {
            Expression::Normal(value.to_string())
        }
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
    type Error = String;

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
    type Error = String; // TODO: Better errors

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
            Err("Walking string did not yield a valid value".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::debug_access::{Assignment, Conditional, Expression, Statement};


    #[test]
    fn parse_comment() {
        let line = "// This is a comment!".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Comment("// This is a comment!".to_string()));
    }

    #[test]
    fn semicolon_handling() {
        let line1 = "myFunction(foo, bar)".to_string();
        let line2 = "myFunction(foo, bar);".to_string();

        let statement1: Statement = line1.into();
        let statement2: Statement = line2.into();

        assert_eq!(statement1, statement2);
    }

    #[test]
    fn parse_expression_normal() {
        let line = "myFunction(foo, bar);".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Expression(
            Expression::Normal("myFunction(foo, bar)".to_string())
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
                expression: "(readTheCoolRegister(0x248) == 5)".to_string()
            }
        ));
    }

    #[test]
    fn parse_assignment() {
        let line = "variable = expression;".to_string();

        let statement: Statement = line.into();

        assert_eq!(statement, Statement::Assignment(
            Assignment {
                expression: "expression".to_string(),
                variable: "variable".to_string(),
            }
        ))
    }
}