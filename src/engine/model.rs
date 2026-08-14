use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: glam::Vec3,
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                shader_location: 0,
                offset: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    position: glam::Vec3,
    colour: glam::Vec3,
    radius: f32,
}

impl Instance {
    pub fn new(position: glam::Vec3, radius: f32, colour: glam::Vec3) -> Self {
        Self {
            position,
            colour,
            radius,
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    shader_location: 1,
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    shader_location: 2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    shader_location: 3,
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

#[derive(Clone)]
pub struct Model {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl Model {
    pub fn new(device: &wgpu::Device, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let index_count = indices.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }

    pub fn uv_sphere(device: &wgpu::Device, radius: f32, stacks: u32, slices: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for stack in 0..=stacks {
            let theta = stack as f32 / stacks as f32 * std::f32::consts::PI;
            let (sin_theta, cos_theta) = theta.sin_cos();

            for slice in 0..=slices {
                let phi = slice as f32 / slices as f32 * std::f32::consts::TAU;
                let (sin_phi, cos_phi) = phi.sin_cos();

                let x = sin_theta * cos_phi;
                let y = cos_theta;
                let z = sin_theta * sin_phi;

                let position = glam::Vec3::new(x, y, z) * radius;

                vertices.push(Vertex { position });
            }

            let verts_per_ring = slices + 1;
            for stack in 0..stacks {
                for slice in 0..slices {
                    let top_left = stack * verts_per_ring + slice;
                    let top_right = top_left + 1;
                    let bottom_left = (stack + 1) * verts_per_ring + slice;
                    let bottom_right = bottom_left + 1;

                    indices.push(top_left);
                    indices.push(bottom_left);
                    indices.push(top_right);

                    indices.push(top_right);
                    indices.push(bottom_left);
                    indices.push(bottom_right);
                }
            }
        }
        Model::new(device, vertices, indices)
    }

    pub fn vertex_buffer(&self) -> wgpu::Buffer {
        self.vertex_buffer.clone()
    }

    pub fn index_buffer(&self) -> wgpu::Buffer {
        self.index_buffer.clone()
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }
}
