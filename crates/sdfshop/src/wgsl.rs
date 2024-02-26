//! An attempt at a embedding wgsl in Rust.
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    path::PathBuf, collections::HashMap,
};

use glam::Vec4;

pub struct TypeDefinition<T> {
    _phantom: PhantomData<T>,
}

// WgslStruct auto-derives things, and produces some functions
pub trait IsTypeDefinition: Any + Sized {
    fn type_definition() -> Wgsl<TypeDefinition<Self>>;
}

impl<T: Any + Sized> IsTypeDefinition for T {
    fn type_definition() -> Wgsl<TypeDefinition<Self>> {
        Wgsl {
            _phantom: PhantomData,
        }
    }
}

pub struct FnDefinition<T> {
    _phantom: PhantomData<T>
}

pub struct Wgsl<T> {
    _phantom: PhantomData<T>,
}

pub struct Module {
    pub path: PathBuf,
    pub type_definitions: HashMap<TypeId, >,
    pub fn_definitions: Vec<String, naga::Function>,
}

impl Module {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Module {
            path: path.as_ref().into(),
            type_definitions: vec![],
        }
    }

    pub fn define_type<T: IsTypeDefinition>(&mut self) -> Wgsl<TypeDefinition<T>> {
        self.type_definitions.push(std::any::TypeId::of::<T>());
        <T as IsTypeDefinition>::type_definition()
    }

    pub fn fn<T: IsFnDefinition>(name: &str, f: F) -> Wgsl<FnDefinition<T>> {
            self.fn_definitions.push()
    }
}

#[derive(Clone, Copy, Default /*, WgslStruct */)]
pub struct VertexInput {
    //#[wgsl(location(0))]
    pub position: Vec4,
    //#[wgsl(location(1))]
    pub color: Vec4,
}

#[derive(Clone, Copy, Default /*, WgslStruct */)]
pub struct VertexOutput {
    //#[wgsl(builtin(position))]
    pub position: Vec4,
    //#[wgsl(location(0))]
    pub color: Vec4,
}

pub fn var<T>(_name: &str) -> Wgsl<T> {
    Wgsl {
        _phantom: PhantomData,
    }
}

pub fn wgsl_module() -> Module {
    let mut m = Module::new("intro.wgsl");

    let _vertex_input = m.define_type::<VertexInput>();
    let _vertex_outut = m.define_type::<VertexOutput>();

    let _vertex_main = m.fn("vertex_main", |vert: Wgsl<VertexInput>| -> Wgsl<VertexOutput> {
        let out: Wgsl<VertexOutput> = var("out");
        out.set("color", vert.get("color"));
        out.set("position", vert.get("position"));
        out
    });

    m
}
