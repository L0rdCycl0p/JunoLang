//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod builder;
pub mod code;
pub mod context;
pub mod diagnostic;
pub mod engine;
pub mod parser;
pub mod provider;
pub mod providers;
pub mod rule;
pub mod severity;

pub use builder::*;
pub use code::*;
pub use context::*;
pub use diagnostic::*;
pub use engine::*;
pub use parser::*;
pub use provider::*;
pub use providers::*;
pub use rule::*;
pub use severity::*;

pub fn analyze(source: &str) -> Vec<Diagnostic> {
    let mut engine = DiagnosticEngine::new(source);

    SyntaxProvider.collect(&mut engine);

    engine.finish()
}
