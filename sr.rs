#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use rsshader::{gpu, render_pipeline, RenderPipeline, WGSL};
fn main() {
    {
        ::std::io::_print(format_args!("{0}\n", HELLO_TRIANGLE.format::<WGSL>()));
    }
}
#[repr(C)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
unsafe impl rsshader::GPUType for Vertex {
    const GPU_TYPE_INFO: rsshader::GPUTypeInfo = rsshader::GPUTypeInfo {
        item_info: rsshader::GPUItemInfo {
            id: 31978747110337514149504010304491992806u128,
            dependencies: &[
                &<[f32; 2] as rsshader::GPUType>::GPU_TYPE_INFO.item_info,
                &<[f32; 3] as rsshader::GPUType>::GPU_TYPE_INFO.item_info,
            ],
            wgsl_declaration: ::const_format::pmr::__AssertStr {
                x: {
                    use ::const_format::__cf_osRcTFl4A;
                    {
                        ((/*ERROR*/));
                        ""
                    }
                },
            }
                .x,
        },
        wgsl_reference: "id2806_Vertex",
    };
}
const HELLO_TRIANGLE: RenderPipeline<Vertex> = (/*ERROR*/).optimize::<WGSL>();
#[repr(C)]
struct Fragment {
    position: [f32; 4],
    color: [f32; 4],
}
unsafe impl rsshader::GPUType for Fragment {
    const GPU_TYPE_INFO: rsshader::GPUTypeInfo = rsshader::GPUTypeInfo {
        item_info: rsshader::GPUItemInfo {
            id: 31978747110360939661934749181981913383u128,
            dependencies: &[
                &<[f32; 4] as rsshader::GPUType>::GPU_TYPE_INFO.item_info,
                &<[f32; 4] as rsshader::GPUType>::GPU_TYPE_INFO.item_info,
            ],
            wgsl_declaration: ::const_format::pmr::__AssertStr {
                x: {
                    use ::const_format::__cf_osRcTFl4A;
                    {
                        ((/*ERROR*/));
                        ""
                    }
                },
            }
                .x,
        },
        wgsl_reference: "id3383_Fragment",
    };
}
#[allow(non_camel_case_types)]
struct vertex_main;
impl std::ops::Deref for vertex_main {
    type Target = fn(Vertex) -> Fragment;
    fn deref(&self) -> &Self::Target {
        fn vertex_main(vertex: Vertex) -> Fragment {
            Fragment {
                position: [vertex.position[0], vertex.position[1], 0.0, 1.0],
                color: [vertex.color[0], vertex.color[1], vertex.color[2], 1.0],
            }
        }
        &(vertex_main as fn(Vertex) -> Fragment)
    }
}
unsafe impl rsshader::GPUFn for vertex_main {
    const FN_DESC: rsshader::desc::GPUFnDesc = rsshader::desc::GPUFnDesc {
        ident: rsshader::desc::GPUIdentDesc(
            31978747110371227412385732658207579177u128,
            "vertex_main",
        ),
        inputs: &[
            rsshader::desc::GPUFnInputDesc {
                ident: rsshader::desc::GPUIdentDesc(
                    31978747110371282766054056082127853391u128,
                    "vertex",
                ),
                ty: &<Vertex as rsshader::GPUType>::TYPE_DESC,
            },
        ],
        output: Some(&<Fragment as rsshader::GPUType>::TYPE_DESC),
        stmts: {
            #[allow(non_upper_case_globals)]
            const GPU_EXPR_vertex: rsshader::desc::GPUExprDesc = rsshader::desc::GPUExprDesc::Local(
                &rsshader::desc::GPUIdentDesc(
                    31978747110371282766054056082127853391u128,
                    "vertex",
                ),
            );
            {
                let stmt_0 = rsshader::desc::GPUStmtDesc::Expr(
                    rsshader::desc::GPUExprDesc::Struct(
                        &<Fragment as rsshader::GPUType>::TYPE_DESC,
                        &[
                            (
                                &<Fragment as rsshader::GPUType>::TYPE_DESC
                                    .field("\"position\""),
                                rsshader::desc::GPUExprDesc::Array(
                                    &[
                                        rsshader::desc::GPUExprDesc::Index(
                                            &rsshader::desc::GPUExprDesc::Field(
                                                &GPU_EXPR_vertex,
                                                "position",
                                            ),
                                            &rsshader::desc::GPUExprDesc::IntLiteral(0u128),
                                        ),
                                        rsshader::desc::GPUExprDesc::Index(
                                            &rsshader::desc::GPUExprDesc::Field(
                                                &GPU_EXPR_vertex,
                                                "position",
                                            ),
                                            &rsshader::desc::GPUExprDesc::IntLiteral(1u128),
                                        ),
                                        rsshader::desc::GPUExprDesc::FloatLiteral("0.0"),
                                        rsshader::desc::GPUExprDesc::FloatLiteral("1.0"),
                                    ],
                                ),
                            ),
                            (
                                &<Fragment as rsshader::GPUType>::TYPE_DESC
                                    .field("\"color\""),
                                rsshader::desc::GPUExprDesc::Array(
                                    &[
                                        rsshader::desc::GPUExprDesc::Index(
                                            &rsshader::desc::GPUExprDesc::Field(
                                                &GPU_EXPR_vertex,
                                                "color",
                                            ),
                                            &rsshader::desc::GPUExprDesc::IntLiteral(0u128),
                                        ),
                                        rsshader::desc::GPUExprDesc::Index(
                                            &rsshader::desc::GPUExprDesc::Field(
                                                &GPU_EXPR_vertex,
                                                "color",
                                            ),
                                            &rsshader::desc::GPUExprDesc::IntLiteral(1u128),
                                        ),
                                        rsshader::desc::GPUExprDesc::Index(
                                            &rsshader::desc::GPUExprDesc::Field(
                                                &GPU_EXPR_vertex,
                                                "color",
                                            ),
                                            &rsshader::desc::GPUExprDesc::IntLiteral(2u128),
                                        ),
                                        rsshader::desc::GPUExprDesc::FloatLiteral("1.0"),
                                    ],
                                ),
                            ),
                        ],
                    ),
                );
                &[stmt_0]
            }
        },
    };
}
#[allow(non_upper_case_globals)]
const GPU_EXPR_vertex_main: rsshader::desc::GPUUnsupportedType = rsshader::desc::GPUUnsupportedType;
#[allow(non_camel_case_types)]
struct fragment_main;
impl std::ops::Deref for fragment_main {
    type Target = fn(Fragment) -> [f32; 4];
    fn deref(&self) -> &Self::Target {
        fn fragment_main(fragment: Fragment) -> [f32; 4] {
            fragment.color
        }
        &(fragment_main as fn(Fragment) -> [f32; 4])
    }
}
unsafe impl rsshader::GPUFn for fragment_main {
    const FN_DESC: rsshader::desc::GPUFnDesc = rsshader::desc::GPUFnDesc {
        ident: rsshader::desc::GPUIdentDesc(
            31978747110391166510963238969909186175u128,
            "fragment_main",
        ),
        inputs: &[
            rsshader::desc::GPUFnInputDesc {
                ident: rsshader::desc::GPUIdentDesc(
                    31978747110391218148976625478485967571u128,
                    "fragment",
                ),
                ty: &<Fragment as rsshader::GPUType>::TYPE_DESC,
            },
        ],
        output: Some(&<[f32; 4] as rsshader::GPUType>::TYPE_DESC),
        stmts: {
            #[allow(non_upper_case_globals)]
            const GPU_EXPR_fragment: rsshader::desc::GPUExprDesc = rsshader::desc::GPUExprDesc::Local(
                &rsshader::desc::GPUIdentDesc(
                    31978747110391218148976625478485967571u128,
                    "fragment",
                ),
            );
            {
                let stmt_0 = rsshader::desc::GPUStmtDesc::Expr(
                    rsshader::desc::GPUExprDesc::Field(&GPU_EXPR_fragment, "color"),
                );
                &[stmt_0]
            }
        },
    };
}
#[allow(non_upper_case_globals)]
const GPU_EXPR_fragment_main: rsshader::desc::GPUUnsupportedType = rsshader::desc::GPUUnsupportedType;
