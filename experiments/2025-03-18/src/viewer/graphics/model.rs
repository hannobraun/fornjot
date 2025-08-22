use std::ops::Range;

use tobj::LoadError;
use wgpu::util::DeviceExt;

use super::texture::{self, LoadTextureError};
use crate::viewer::assets::Assets;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl ModelVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

#[derive(Debug)]
pub struct Material {
    pub bind_group: wgpu::BindGroup,
}

#[derive(Debug)]
pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum LoadModelError {
    #[error("Object loading error")]
    ObjLoad(#[from] LoadError),

    #[error("Load texture error")]
    Texture(#[from] LoadTextureError),
}

pub fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
) -> Result<Model, LoadModelError> {
    let assets = Assets::get_instance();

    let (models, obj_materials) = tobj::load_obj_buf(
        &mut assets.get_asset(file_name),
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| {
            tobj::load_mtl_buf(
                &mut assets.get_asset(
                    p.file_name()
                        .unwrap()
                        .to_str()
                        .expect("OsStr could not be converted to a str"),
                ),
            )
        },
    )?;

    let mut materials = Vec::new();
    for m in obj_materials? {
        let texture_data: &[u8] = match m.diffuse_texture {
            Some(diffuse_texture) => assets.get_asset(diffuse_texture.as_str()),
            None => continue,
        };

        let diffuse_texture = texture::Texture::from_bytes(
            device,
            queue,
            texture_data,
            file_name,
        )?;

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &diffuse_texture.view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(
                        &diffuse_texture.sampler,
                    ),
                },
            ],
            label: None,
        });

        materials.push(Material { bind_group })
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [
                        m.mesh.texcoords[i * 2],
                        1.0 - m.mesh.texcoords[i * 2 + 1],
                    ],
                })
                .collect::<Vec<_>>();

            let vertex_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{file_name:?} Vertex Buffer")),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });
            let index_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{file_name:?} Index Buffer")),
                    contents: bytemuck::cast_slice(&m.mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

            Mesh {
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

    Ok(Model { meshes, materials })
}

pub trait DrawModel<'a> {
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        instances: Range<u32>,
    );

    fn draw_model(&mut self, model: &'a Model);
    fn draw_model_instanced(&mut self, model: &'a Model, instances: Range<u32>);
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        instances: Range<u32>,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(
            mesh.index_buffer.slice(..),
            wgpu::IndexFormat::Uint32,
        );
        self.set_bind_group(0, &material.bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_model(&mut self, model: &'b Model) {
        self.draw_model_instanced(model, 0..1);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.draw_mesh_instanced(mesh, material, instances.clone());
        }
    }
}
