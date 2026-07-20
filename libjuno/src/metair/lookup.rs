use crate::ast::Function;
use crate::metair::generator::MetaIRGen;
use crate::metair::metair::*;

impl<'a> MetaIRGen<'a> {
    // =======================
    // Function Lookup
    // =======================

    pub(crate) fn find_function(&self, name: &str) -> Option<&'a Function> {
        self.program.items.iter().find_map(|item| {
            match item {
                crate::ast::Item::Function(function, _)
                    if function.name == name =>
                {
                    Some(function)
                }

                _ => None,
            }
        })
    }

    // =======================
    // Local Variables
    // =======================

    pub(crate) fn insert_local(
        &mut self,
        name: SymbolId,
        ty: MetaType,
    ) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name, ty);
        } else {
            panic!(
                "attempted to insert local variable without active scope"
            );
        }
    }

    pub(crate) fn lookup_local_type(
        &self,
        name: SymbolId,
    ) -> MetaType {
        for scope in self.locals.iter().rev() {
            if let Some(ty) = scope.get(&name) {
                return ty.clone();
            }
        }

        panic!("unknown local variable {}", name);
    }
}