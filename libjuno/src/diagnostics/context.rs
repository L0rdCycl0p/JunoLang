//This Source Code Form is subject to the terms of the Mozilla Public
//License, v. 2.0. If a copy of the MPL was not distributed with this
//file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::diagnostics::{
    Position,
    Span,
};

pub struct DiagnosticContext<'a> {
    source: &'a str,
}

impl<'a> DiagnosticContext<'a> {

    pub fn new(source: &'a str) -> Self {
        Self {
            source,
        }
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn line(
        &self,
        line: usize,
    ) -> Option<&'a str> {

        self.source.lines().nth(line)
    }

    pub fn slice(
        &self,
        span: Span,
    ) -> Option<&'a str> {

        let start = self.offset(span.start)?;
        let end = self.offset(span.end)?;

        self.source.get(start..end)
    }

    pub fn offset(
        &self,
        pos: Position,
    ) -> Option<usize> {

        let mut offset = 0;

        for (i, line) in self.source.lines().enumerate() {

            if i == pos.line {
                return Some(offset + pos.column);
            }

            offset += line.len() + 1;
        }

        None
    }
}