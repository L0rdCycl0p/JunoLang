//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod parser;
pub mod providers;
pub mod builder;
pub mod diagnostic;
pub mod severity;
pub mod context;
pub mod engine;
pub mod provider;
pub mod rule;
pub mod code;

pub use code::*;
pub use parser::*;
pub use providers::*;
pub use builder::*;
pub use diagnostic::*;
pub use severity::*;
pub use context::*;
pub use engine::*;
pub use provider::*;
pub use rule::*;

pub fn analyze(source: &str) -> Vec<Diagnostic> {
    let mut engine = DiagnosticEngine::new(source);

    SyntaxProvider.collect(&mut engine);

    engine.finish()
}
