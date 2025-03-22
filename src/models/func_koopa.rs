use super::my_struct::*;

pub trait AstTree {
    fn tree(&self) -> String;
}

impl AstTree for CompUnit {
    fn tree(&self) -> String {
        self.func_def.tree()
    }
}

impl AstTree for FuncDef {
    fn tree(&self) -> String {
        let temp0 = self.func_type.tree();
        let temp1 = self.ident.tree();
        let temp2 = self.block.tree();
        format!("fun @{}(): {} {}", temp1, temp0, temp2)
    }
}

#[allow(unreachable_patterns)]
impl AstTree for FuncType {
    fn tree(&self) -> String {
        match *self {
            Self::Int => String::from("i32"),
            _ => String::from("i32")
        }
    }
}

impl AstTree for String {
    fn tree(&self) -> String {
        self.clone()
    }
}

impl AstTree for Block {
    fn tree(&self) -> String {
        format!("{{\n%entry:\n{}}}", self.stmt.tree())
    }
}

impl AstTree for Stmt {
    fn tree(&self) -> String {
        format!("\tret {}\n", self.num)
    }
}
