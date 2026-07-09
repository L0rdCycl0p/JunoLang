//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::diagnostics::Span;

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub message: String,

    pub replacement: String,

    pub span: Span,

    pub applicability: Applicability,
}
#[derive(Debug, Clone)]
pub enum Applicability {
    MachineApplicable,

    MaybeIncorrect,

    Manual,
}
