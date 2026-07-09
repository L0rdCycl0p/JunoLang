
use std::path::Path;




#[test]
fn compile_test_files(){
    let ir_module = libjuno::compile::compile_file(Path::new("../test/test.juno"));
    ir_module.verify().unwrap();
}