use std::collections::HashMap;

pub enum Error {
    UnknownVariable { name: String },
    VariableAlreadyDefined { name: String },
}

pub struct EnvironmentStack<'a> {
    stack: Vec<Environment<'a>>,
}

impl<'a> EnvironmentStack<'a> {
    pub fn get(&self, key: &str) -> Option<&DataType> {
        for env in self.stack.iter().rev() {
            if let Some(val) = env.get(key) {
                return Some(val);
            }
        }

        None
    }
}

#[derive(Debug)]
pub enum DataType {
    Integer(i32),
    Float(f32),
}

#[derive(Debug, Default)]
pub struct Environment<'a> {
    mappings: HashMap<&'a str, DataType>,
}

impl<'a> Environment<'a> {
    pub fn define(&mut self, key: &'a str, value: DataType) -> Result<(), Error> {
        if !self.mappings.contains_key(key) {
            self.mappings.insert(key, value);
            Ok(())
        } else {
            Err(Error::VariableAlreadyDefined { name: key.into() })
        }
    }

    pub fn get(&self, key: &str) -> Option<&DataType> {
        self.mappings.get(key)
    }

    pub fn set(&mut self, key: &'a str, val: DataType) -> Result<DataType, Error> {
        match self.mappings.entry(key) {
            std::collections::hash_map::Entry::Occupied(mut o) => Ok(o.insert(val)),
            std::collections::hash_map::Entry::Vacant(_) => {
                Err(Error::UnknownVariable { name: key.into() })
            }
        }
    }
}
