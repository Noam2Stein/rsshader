fn main() {
    println!("{}", SHADER.replace("\t", "  "));
}

use rsshader::{
    ir::{EntryKind, Expr, Function, Literal, Shader, Stmt, Variable},
    shader_item, wgsl,
};

#[shader_item]
#[derive(Debug, Copy, Clone)]
struct Vertex {
    pos: f32,
    color: [f32; 5],
    test: Helper,
}

#[shader_item]
#[derive(Debug, Copy, Clone)]
struct Helper {
    a: f32,
    b: bool,
    c: f32,
}

const SHADER: &str = wgsl!(Shader {
    entries: &[&Function {
        entry_kind: Some(EntryKind::Vertex),
        parameters: &[Variable {
            id: 0,
            ty: &<Vertex as rsshader::reflection::ShaderType>::IR,
        }],
        return_type: None,
        stmts: &[0],
        expr_bank: &[],
        stmt_bank: &[Stmt::Return(Some(Expr::Literal(Literal::F32(1.0))))],
    }],
});
