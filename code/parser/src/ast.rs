use std::{fmt::format};

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperatorKind {
    Plus,
    Minus,
    Mul,
    Div,
    SmallerThen,
    GreaterThen,
}

impl BinaryOperatorKind {
    pub fn to_string(&self) -> String {
        match &self {
            BinaryOperatorKind::Plus => "+",
            BinaryOperatorKind::Minus => "-",
            BinaryOperatorKind::Mul => "*",
            BinaryOperatorKind::Div => "/",
            BinaryOperatorKind::SmallerThen => "<",
            BinaryOperatorKind::GreaterThen => ">",
        }
        .to_owned()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryExpr {
    pub operator: BinaryOperatorKind,
    pub left: Box<ExprKind>,
    pub right: Box<ExprKind>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperatorAssociativity {
    Left,
    Right,
}

impl BinaryExpr {
    fn precedence(&self) -> i32 {
        match self.operator {
            BinaryOperatorKind::SmallerThen => 0,
            BinaryOperatorKind::GreaterThen => 0,
            BinaryOperatorKind::Plus => 20,
            BinaryOperatorKind::Minus => 20,
            BinaryOperatorKind::Mul => 40,
            BinaryOperatorKind::Div => 40,
        }
    }

    fn associativity(&self) -> OperatorAssociativity {
        match self.operator {
            BinaryOperatorKind::SmallerThen => OperatorAssociativity::Left,
            BinaryOperatorKind::GreaterThen => OperatorAssociativity::Left,
            BinaryOperatorKind::Plus => OperatorAssociativity::Left,
            BinaryOperatorKind::Minus => OperatorAssociativity::Left,
            BinaryOperatorKind::Mul => OperatorAssociativity::Left,
            BinaryOperatorKind::Div => OperatorAssociativity::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuildinTypeKind {
    Int,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AggregateField {}

#[derive(Debug, PartialEq, Eq)]
pub struct AggregateType {
    pub name: String,
    pub fields: Vec<AggregateField>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeKind {
    Buildin(BuildinTypeKind),
    Aggregate(AggregateType),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LiteralExpr {
    pub constant_type: BuildinTypeKind,
    pub value: i32, // Only one kind of constant at this time.
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expr {
    kind: ExprKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExprKind {
    Identifier(String),
    Literal(LiteralExpr),
    Binary(BinaryExpr),
}

impl ExprKind {
    pub fn to_string(&self) -> String {
        match self {
            ExprKind::Identifier(s) => s.clone(),
            ExprKind::Literal(lit_expr) => format!("{}", lit_expr.value),
            ExprKind::Binary(bin_expr) => format!(
                "({} {} {})",
                bin_expr.operator.to_string(),
                bin_expr.left.to_string(),
                bin_expr.right.to_string()
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AssignExpr {
    pub left: Box<ExprKind>,
    pub right: Box<ExprKind>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StmtKind {
    Assign(AssignExpr),
    VarDecl(String),
}

impl StmtKind {
    pub fn to_string(&self) -> String {
        match self {
            StmtKind::Assign(asgExpr) => format!("(assign {0} {1})", asgExpr.left.to_string(), asgExpr.right.to_string()),
            StmtKind::VarDecl(name) => name.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnresolvedType{
    pub name: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunArg{
    pub name: String,
    pub arg_type: UnresolvedType
}

impl FunArg{
    pub fn to_string(&self) -> String{
        format!("(arg {0} {1})", self.arg_type.name, self.name).to_owned()
    }
}



#[derive(Debug, PartialEq, Eq)]
pub enum Ast {
    Expr(ExprKind),
    BasicBlock{implementation: Vec<Box<Ast>>},
    Stmt(StmtKind),
    FunDecl{name: String, args: Vec<FunArg>, returns: UnresolvedType, implementation: Box<Ast>},
}

impl Ast{
    pub fn to_string(&self) -> String{
        match  self {
            Ast::Expr(expr) => expr.to_string(),
            Ast::BasicBlock{implementation} => 
            {
                let code = implementation.iter()
                .map(|x|  x.to_string())
                .fold("".to_owned(), |acc, new| acc + &new);
                format!("(BasicBlock {0}) \n", code).to_owned()
            },
            Ast::Stmt(stmt) => stmt.to_string(),
            Ast::FunDecl { name , args, returns, implementation} 
            => {
                let args = args.iter()
                    .map(|a| a.to_string()) 
                    .fold("".to_owned(), |acc , a_s| acc + &a_s);
                let code = implementation.to_string();
                format!("(FunDecl {0} (args {1}) (returns {2}))\n {3}", name, args, returns.name, implementation.to_string())
            }
        }
    }
}