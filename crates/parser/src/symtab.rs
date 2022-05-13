use ast::ty::Type;
use std::{collections::HashMap, usize};

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub sym: Sym,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sym {
    Variable,
    Function {
        arity: usize
    }
}

#[derive(Debug, PartialEq)]
pub struct SymbolTable {
    inner: Vec<HashMap<String, Symbol>>,
    level: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            inner: vec![HashMap::default()],
            level: 0,
        }
    }

    pub fn level_up(&mut self) {
        self.inner.push(HashMap::default());
        self.level = self.level + 1;
    }

    pub fn level_down(&mut self) {
        if self.level == 0 {
            unreachable!("ICE: symtab can't go below level 0!");
        }

        self.inner.pop();
        self.level = self.level - 1;
    }

    pub fn insert(&mut self, sym: Symbol) {
        if let Some(map) = self.inner.get_mut(self.level) {
            map.insert(sym.name.clone(), sym); 
        } else {
            panic!("ICE: symtab is out of sync. Trying to insert into level that doesn't exist.");
        }
    }

    pub fn lookup(&self, name: &String) -> Option<&Symbol> {
        if let Some(map) = self.inner.get(self.level) {
            map.get(name)
        } else {
            panic!("ICE: symtab is out of sync. Trying to lookup into level that doesn't exist.");
        }
    }
    
    pub fn lookup_mut(&mut self, name: &String) -> Option<&mut Symbol> {
        if let Some(map) = self.inner.get_mut(self.level) {
            map.get_mut(name)
        } else {
            panic!("ICE: symtab is out of sync. Trying to lookup into level that doesn't exist.");
        }
    }
}
