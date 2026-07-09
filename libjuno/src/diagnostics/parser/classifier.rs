//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use pest::error::ErrorVariant;

use crate::{Rule, diagnostics::DiagnosticRule};

pub fn classify(error: &pest::error::Error<Rule>) -> DiagnosticRule {
    match &error.variant {
        ErrorVariant::ParsingError { positives, .. } => {
            if positives.contains(&Rule::ident) {
                return DiagnosticRule::MissingIdentifier;
            }

            if positives.contains(&Rule::type_) {
                return DiagnosticRule::MissingType;
            }

            if positives.contains(&Rule::expr) {
                return DiagnosticRule::MissingExpression;
            }

            DiagnosticRule::Unknown
        }

        ErrorVariant::CustomError { .. } => DiagnosticRule::Unknown,
    }
}
