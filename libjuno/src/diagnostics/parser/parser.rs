//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{ classify, message, from_pest };

use crate::diagnostics::{ Diagnostic, DiagnosticBuilder, DiagnosticContext, Severity };

use crate::Rule;

pub fn parse(_ctx: &DiagnosticContext, error: pest::error::Error<Rule>) -> Diagnostic {
    let span = from_pest(&error);

    let rule = classify(&error);

    let message = message(rule);

    DiagnosticBuilder::new(Severity::Error, span, message).build()
}
