use inkwell::types::{BasicTypeEnum, StructType};

use crate::metair::*;
use super::*;

impl<'ctx> LLVMBackend<'ctx> {
    fn struct_type(&mut self, s: &MetaStruct) -> Result<StructType<'ctx>, LLVMError> {
        let mut fields = Vec::<BasicTypeEnum<'ctx>>::new();

        for field in &s.fields {
            fields.push(self.lower_type(&field.ty)?);
        }

        Ok(self.context.struct_type(&fields, false))
    }

    pub fn lower_struct(&mut self, s: &MetaStruct) -> Result<(), LLVMError> {
        let ty = self.struct_type(s)?;

        self.add_struct(s.name, &ty)?;

        Ok(())
    }

    pub fn add_struct_in_namespace(
        &mut self,
        namespace: u16,
        id: u16,
        ty: &StructType<'ctx>,
    ) -> Result<(), LLVMError> {
        let id = ((namespace as u32) << 16) | id as u32;
        self.structs.insert(id, *ty);
        Ok(())
    }

    pub fn add_struct(
        &mut self,
        id: u32,
        ty: &StructType<'ctx>,
    ) -> Result<(), LLVMError> {
        self.structs.insert(id, *ty);
        Ok(())
    }

    pub fn get_struct_in_namespace(
        &self,
        namespace: u16,
        id: u16,
    ) -> Result<StructType<'ctx>, LLVMError> {
        let id = ((namespace as u32) << 16) | id as u32;

        self.structs
            .get(&id)
            .copied()
            .ok_or_else(|| LLVMError::Message(format!("unknown struct {}", id)))
    }

    pub fn get_struct(
        &self,
        target: &[SymbolId],
    ) -> Result<StructType<'ctx>, LLVMError> {
        if target.len() != 1 {
            return Err(LLVMError::Message(
                "qualified struct lookup is not implemented".into(),
            ));
        }

        let id = target[0];

        self.structs
            .get(&id)
            .copied()
            .ok_or_else(|| {
                LLVMError::Message(format!(
                    "unknown struct '{}'",
                    self.program.symbol_table[id as usize]
                ))
            })
    }
}