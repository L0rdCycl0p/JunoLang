use std::path::Path;

use pest::Parser;

use crate::*;

#[derive(Clone, Debug)]
pub enum SymbolDecl {
    Function {
        id: u32,
        name: String,
        params: Vec<MetaParam>,
        ret: Option<MetaType>,
    },
    Struct {
        id: u32,
        name: String,
        fields: Vec<MetaField>,
    },
}
#[derive(Clone, Debug)]
pub struct SymbolDeclTable {
    pub symbols: Vec<SymbolDecl>,
    pub program: MetaProgram,
}

pub fn get_symbols_from_file(p: &Path, pkg_name: String) -> SymbolDeclTable {
    let input = match std::fs::read_to_string(p) {
        Err(e) => panic!("Error while reading file {} (ERR: {})", p.to_str().unwrap(), e),
        Ok(s) => s,
    };
    let namespace = path_to_namespace(p, Some(pkg_name));
    get_symbols(input, namespace)
}

pub fn get_symbols(input: String, namespace: String) -> SymbolDeclTable {
    let pairs = match JunoParser::parse(Rule::program, &input.as_str()) {
        Ok(pairs) => pairs,
        Err(e) => {
            panic!("{e}");
        }
    };
    let expr_owned = parse_program(pairs.into_iter().next().unwrap(), namespace).unwrap();
    let expr = Box::leak(Box::new(expr_owned));
    let metairgen = Box::leak(Box::new(MetaIRGen::new(expr)));
    let metair = Box::leak(Box::new(metairgen.lower_program(expr)));
    let mut symbols = vec![];
    for f in &metair.functions {
        symbols.push(SymbolDecl::Function {
            id: f.id,
            name: metairgen.symbol_list.get(f.id as usize).unwrap().clone(),
            params: f.params.clone(),
            ret: f.ret.clone(),
        });
    }
    for s in &metair.structs {
        symbols.push(SymbolDecl::Struct {
            id: s.id,
            name: metairgen.symbol_list.get(s.id as usize).unwrap().clone(),
            fields: s.fields.clone(),
        });
    }
    SymbolDeclTable {
        symbols: symbols,
        program: metair.clone(),
    }
}
