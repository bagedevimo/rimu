pub struct AstNode {
    pub kind: AstNodeKind,
}

impl std::fmt::Debug for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AstNode {{ Kind: {:?} }}", self.kind)
    }
}

#[derive(Debug)]
pub enum AstNodeKind {
    Function(Option<FunctionTypedef>, FunctionBody)
}

#[derive(Debug)]
pub struct Identifier {
    pub identifier : String
}

#[derive(Debug)]
pub struct FunctionTypedef {
    pub parameter_type : Vec<Identifier>,
    pub return_type : Vec<Identifier>,
}

#[derive(Debug)]
pub struct FunctionBody {
}

