//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::diagnostics::{
    DiagnosticCode, Severity, Span, Suggestion,
};

#[derive(Debug, Clone)]
pub struct Diagnostic {

    //pub code: DiagnosticCode,

    pub severity: Severity,

    pub span: Span,

    pub message: String,

    pub notes: Vec<String>,

    pub suggestions: Vec<Suggestion>,
}


