use crate::*;
use std::fmt::Formatter;

#[derive(Default, Eq, PartialEq, Clone, Hash)]
pub struct Bytecode {
    source_instructions: Vec<Instruction>,
    step_instructions: Vec<Instruction>,
}

impl std::fmt::Debug for Bytecode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.step_instructions[0])?;

        for step in self.step_instructions.iter().skip(1) {
            write!(f, ".{:?}", &step)?;
        }

        Ok(())
    }
}

impl Bytecode {
    pub fn new() -> Bytecode {
        Default::default()
    }

    pub fn add_source<S>(&mut self, source_name: S, args: Vec<GValue>)
    where
        S: AsRef<str>,
    {
        self.source_instructions
            .push(Instruction::new(source_name.as_ref().to_string(), args));
    }
    pub fn add_step<S>(&mut self, step_name: S, args: Vec<GValue>)
    where
        S: AsRef<str>,
    {
        self.step_instructions
            .push(Instruction::new(step_name.as_ref().to_string(), args));
    }

    pub fn sources(&self) -> &Vec<Instruction> {
        &self.source_instructions
    }

    pub fn steps(&self) -> &Vec<Instruction> {
        &self.step_instructions
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Instruction {
    pub(crate) operator: String,
    pub(crate) args: Vec<GValue>,
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.args.is_empty() {
            match &self.args[0] {
                GValue::P(_) => write!(f, "P{:?}", &self.args[0])?,
                GValue::T(_) => write!(f, "T{:?}", &self.args[0])?,
                _ => {
                    write!(f, "{}", &self.operator)?;
                    write!(f, "(")?;
                    write!(f, "{:?}", &self.args[0])?
                }
            }

            for arg in self.args.iter().skip(1) {
                write!(f, ", {:?}", arg)?;
            }
        }

        if !self.args.is_empty() {
            match &self.args[0] {
                GValue::P(_) | GValue::T(_) => Ok(()),
                _ => {
                    write!(f, ")")
                }
            }
        } else {
            Ok(())
        }
    }
}

impl Instruction {
    pub fn new(operator: String, args: Vec<GValue>) -> Instruction {
        Instruction { operator, args }
    }
}
