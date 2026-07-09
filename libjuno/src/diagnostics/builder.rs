//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::diagnostics::*;

pub struct DiagnosticBuilder {

    diagnostic: Diagnostic,
}

impl DiagnosticBuilder {

    pub fn new(
        severity: Severity,
        span: Span,
        message: impl Into<String>,
    ) -> Self {

        Self {
            diagnostic: Diagnostic {
                severity,
                span,
                message: message.into(),
                notes: Vec::new(),
                suggestions: Vec::new(),
                
            }
        }
    }

    pub fn note(
        mut self,
        note: impl Into<String>,
    ) -> Self {

        self.diagnostic.notes.push(note.into());
        self
    }

    pub fn suggestion(
        mut self,
        suggestion: Suggestion,
    ) -> Self {

        self.diagnostic.suggestions.push(suggestion);
        self
    }

    pub fn build(self) -> Diagnostic {
        self.diagnostic
    }
}