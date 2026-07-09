//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use pest::Parser;

use crate::diagnostics::{ DiagnosticEngine, DiagnosticProvider, parse };
use crate::{JunoParser, Rule};
pub struct SyntaxProvider;

impl DiagnosticProvider for SyntaxProvider {
    fn collect(&self, engine: &mut DiagnosticEngine) {
        let source = engine.context().source();

        match JunoParser::parse(Rule::program, source) {
            Ok(_) => {}

            Err(error) => {
                let diagnostic = parse(engine.context(), error);

                engine.push(diagnostic);
            }
        }
    }
}
