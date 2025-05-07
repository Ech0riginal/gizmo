use crate::structure::*;
use std::fmt::Formatter;

#[derive(PartialEq, Clone)]
pub struct Bytecode {
    source_instructions: Vec<Instruction>,
    pub(crate) step_instructions: Vec<Instruction>,
}

impl Default for Bytecode {
    fn default() -> Bytecode {
        Bytecode {
            source_instructions: vec![],
            step_instructions: vec![],
        }
    }
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

    pub fn add_source(&mut self, source_name: String, args: Vec<GValue>) {
        self.source_instructions
            .push(Instruction::new(source_name, args));
    }
    pub fn add_step(&mut self, step_name: String, args: Vec<GValue>) {
        self.step_instructions
            .push(Instruction::new(step_name, args));
    }

    pub fn steps(&self) -> &Vec<Instruction> {
        &self.step_instructions
    }

    pub fn sources(&self) -> &Vec<Instruction> {
        &self.source_instructions
    }
}

#[derive(PartialEq, Clone)]
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

    pub fn operator(&self) -> &String {
        &self.operator
    }

    pub fn args(&self) -> &Vec<GValue> {
        &self.args
    }
}
