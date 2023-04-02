//! Creating and storing meshes using `wgpu`.
use bytemuck::Pod;
use wgpu::util::DeviceExt;

/// An unbuffered mesh.
pub struct UnbufferedMesh<T> {
    pub vertices: Vec<T>,
    pub indices: Option<Vec<u16>>,
}

impl<T> Default for UnbufferedMesh<T> {
    fn default() -> Self {
        UnbufferedMesh {
            vertices: vec![],
            indices: None,
        }
    }
}

#[derive(Debug)]
pub struct MeshBuffer {
    pub buffer: wgpu::Buffer,
    pub len: usize,
}

/// A mesh that has been buffered to the GPU.
#[derive(Debug)]
pub struct Mesh {
    pub vertex_buffer: MeshBuffer,
    pub index_buffer: Option<MeshBuffer>,
}

impl Mesh {
    pub fn buffer<T: Pod>(
        name: Option<&str>,
        device: &wgpu::Device,
        vertices: &[T],
        indices: Option<&[u16]>,
    ) -> Self {
        let name = name.unwrap_or("unknown");
        let len = vertices.len();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Mesh::buffer {} vertex", name)),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let vertex_buffer = MeshBuffer { buffer, len };
        let index_buffer = indices.map(|ndxs| {
            let len = ndxs.len();
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Mesh::buffer {} index", name)),
                contents: bytemuck::cast_slice(ndxs),
                usage: wgpu::BufferUsages::INDEX,
            });
            MeshBuffer { buffer, len }
        });

        Mesh {
            vertex_buffer,
            index_buffer,
        }
    }
}

pub struct MeshBuilder<T> {
    unbuffered_mesh: UnbufferedMesh<T>,
}

impl<T> Default for MeshBuilder<T> {
    fn default() -> Self {
        Self {
            unbuffered_mesh: Default::default(),
        }
    }
}

impl<T: Pod> MeshBuilder<T> {
    pub fn with_vertex(mut self, v: T) -> Self {
        self.unbuffered_mesh.vertices.push(v);
        self
    }

    pub fn with_vertices(mut self, vs: impl IntoIterator<Item = T>) -> Self {
        self.unbuffered_mesh.vertices.extend(vs);
        self
    }

    pub fn with_indices(mut self, is: impl IntoIterator<Item = u16>) -> Self {
        if self.unbuffered_mesh.indices.is_none() {
            self.unbuffered_mesh.indices = Some(vec![]);
        }
        self.unbuffered_mesh.indices.as_mut().unwrap().extend(is);
        self
    }

    pub fn build(self, label: Option<&str>, device: &wgpu::Device) -> Mesh {
        Mesh::buffer(
            label,
            device,
            &self.unbuffered_mesh.vertices,
            self.unbuffered_mesh.indices.as_deref(),
        )
    }
}
