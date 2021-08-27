use hir::Database;

use crate::env::Env;
use crate::error::EngineError;
use crate::func::Function;
use crate::val::Val;

#[derive(Clone)]
pub struct ClassDef {
    pub name: String,
    pub fields: Vec<String>,
    pub methods: Vec<Function>,
}

impl ClassDef {
    pub fn new(class_def: hir::ClassDef, db: &Database) -> Self {
        ClassDef {
            name: class_def.name.into(),
            fields: class_def.fields.into_iter().map(|s| s.into()).collect(),
            methods: class_def
                .methods
                .into_iter()
                // FIXME: db shouldn't have to be cloned here
                .map(|func| Function::new(func, db.clone()))
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct Class {
    pub self_env: Env,
    pub def: ClassDef,
}

impl Class {
    pub fn new(def: ClassDef, init_value_list: Vec<Val>) -> Result<Self, EngineError> {
        let mut self_env = Env::new(None);
        for (index, field) in def.clone().fields.into_iter().enumerate() {
            self_env.define_variable(field.as_str(), init_value_list[index].to_owned())?;
        }

        for func in def.clone().methods.into_iter() {
            self_env.set_function(func)?;
        }

        Ok(Class {
            self_env: Env::new(None),
            def,
        })
    }
}

impl PartialEq for Class {
    fn eq(&self, _other: &Self) -> bool {
        todo!("partial eq for classes")
    }

    fn ne(&self, _other: &Self) -> bool {
        todo!("partial eq for classes")
    }
}

impl std::fmt::Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Class: def = {:?}, fields = {:?}",
            self.def.name,
            self.self_env.get_all_bindings()
        )
    }
}
