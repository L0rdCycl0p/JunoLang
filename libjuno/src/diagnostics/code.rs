//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Hexadecimal based code!

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    JN0000, // NO ERROR
    // Syntax
    JN0001,

    JN0002,

    JN0003,

    // AST
    JN0101,

    // Type
    JN2001,
}
