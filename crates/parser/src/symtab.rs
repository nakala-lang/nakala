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
    Function { arity: usize },
    Class { methods: HashMap<String, Symbol> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    inner: Vec<HashMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new(builtins: Vec<Symbol>) -> Self {
        let mut symtab = SymbolTable {
            inner: vec![HashMap::default()],
        };

        symtab.define_builtins(builtins);

        symtab
    }

    fn define_builtins(&mut self, builtins: Vec<Symbol>) {
        for builtin in builtins {
            self.insert(builtin);
        }
    }

    pub fn merge_with(&mut self, other: SymbolTable) {
        //FIXME I'm pretty sure we only need to merge the first level of each since we use this
        //function for the REPL and never in a nested scope
        let self_map = self
            .inner
            .get_mut(0)
            .expect("symtabs must have at least one level");
        self_map.extend(
            other
                .inner
                .into_iter()
                .next()
                .expect("symtabs must have atleast one level"),
        );
    }

    pub fn at_global_scope(&self) -> bool {
        self.inner.len() == 1
    }

    pub fn level_up(&mut self) {
        self.inner.push(HashMap::default());
    }

    pub fn level_down(&mut self) {
        self.inner.pop();
    }

    pub fn insert(&mut self, sym: Symbol) {
        if let Some(map) = self.inner.last_mut() {
            map.insert(sym.name.clone(), sym);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for map in self.inner.iter().rev() {
            if let Some(entry) = map.get(name) {
                return Some(entry);
            }
        }

        None
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        for map in self.inner.iter_mut().rev() {
            if let Some(entry) = map.get_mut(name) {
                return Some(entry);
            }
        }

        None
    }
}
