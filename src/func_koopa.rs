use crate::my_struct::*;

trait AstTree{
    fn tree(&self) -> String;
}

impl AstTree for CompUnit{
    fn tree(&self) -> String {
        self.func_def.tree()
    }
}

impl AstTree for FuncDef{
    fn tree(&self) -> String{
        
    }
}