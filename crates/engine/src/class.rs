use hir::Database;

use crate::func::Function;

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
