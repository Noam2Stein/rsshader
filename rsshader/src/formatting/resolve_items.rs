use crate::desc::{GPUFnDesc, GPUItemDesc, GPUStructDesc, GPUTypeDesc};

pub fn resolve_items(items: impl Iterator<Item = GPUItemDesc>) -> Vec<GPUItemDesc> {
    let mut output = Vec::new();

    for item in items {
        match item {
            GPUItemDesc::Struct(item) => push_struct(&item, &mut output),
            GPUItemDesc::Fn(item) => push_fn(&item, &mut output),
        }
    }

    output
}

fn push_struct(item: &GPUStructDesc, output: &mut Vec<GPUItemDesc>) {
    for field in item.fields {
        push_type(field.ty, output);
    }

    output.push(GPUItemDesc::Struct(*item));
}

fn push_fn(item: &GPUFnDesc, output: &mut Vec<GPUItemDesc>) {}

fn push_type(item: &GPUTypeDesc, output: &mut Vec<GPUItemDesc>) {}
