fn main() {
    println!("{}", SHADER.replace("\t", "  "));
}

use rsshader::{
    ir::{EntryPointInfo, Expr, Function, Literal, Shader, Stmt, Variable, VertexFunctionInfo},
    shader_item, wgsl,
};

#[shader_item]
#[derive(Debug, Copy, Clone)]
struct Vertex {
    pos: f32,
    test: Helper,
}

#[shader_item(fragment)]
#[derive(Debug, Copy, Clone)]
struct Helper {
    #[position]
    a: f32,
    b: i32,
    c: f32,
}

const SHADER: &str = wgsl!(Shader {
    entry_points: &[&Function::UserDefined {
        entry_point_info: Some(EntryPointInfo::Vertex(VertexFunctionInfo {
            input_attrs: &[],
            output_attrs: &[],
        })),
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
