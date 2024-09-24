pub trait Type {

}

pub trait Struct {

}
impl<T: Struct> Type for T {

}

pub trait Vertex: Struct {

}
pub trait Fragment: Struct {

}