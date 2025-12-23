use std::{fs, io::Error, path::Path};

use math::{quaternion::Quat, transform::Transform, vec3::Vec3};

use crate::src::{
    cpu::{
        material::Material,
        mesh::Mesh,
        model::Model,
        primitive::Primitive,
        skin::Skin,
        texture::{Texture, TextureFormat},
    },
    scene::Scene,
};

use crate::src::cpu::node::Node;

pub fn load_model_from_gltf(file: &GltfFile) -> Result<Model, Box<dyn std::error::Error>> {
    let meshes = load_meshes(&file)?;
    let textures = load_textures(&file)?;
    let materials = load_materials(&file)?;
    let skins = load_skins(&file)?;
    let nodes = load_nodes(&file)?;

    Ok(Model {
        meshes,
        skins,
        textures,
        materials,
        nodes,
    })
}

pub struct GltfFile {
    /// parent folder holding gltf/glb assets
    folder: String,

    document: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    images: Vec<gltf::image::Data>,
}

pub fn load_gltf(folder: &Path) -> Result<GltfFile, Error> {
    let paths = fs::read_dir(folder).unwrap();

    let mut gltf_file = String::new();
    for entry in paths {
        let path = entry.unwrap().path();

        if let Some(extension) = path.extension() {
            if extension.eq("gltf") || extension.eq("glb") {
                gltf_file = String::from(path.to_str().unwrap());
            }
        }
    }
    let folder = String::from(folder.to_str().unwrap());

    let (document, buffers, images) = gltf::import(gltf_file.clone())
        .expect(format!("Failed to import gltf from {}", gltf_file).as_str());

    Ok(GltfFile {
        folder,
        document,
        buffers,
        images,
    })
}

/// helper functions

fn load_meshes(file: &GltfFile) -> Result<Vec<Mesh>, Error> {
    let mut meshes = Vec::new();
    let document = &file.document;
    let buffers = &file.buffers;

    document.meshes().for_each(|mesh| {
        let mut temp_mesh = Mesh::new();

        mesh.primitives().for_each(|primitive| {
            let mut temp_primitive = Primitive::new();

            temp_primitive.material = primitive.material().index().unwrap();
            //temp_primitive.name = primitive.mode().as_gl_enum();

            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            if let Some(positions) = reader.read_positions() {
                temp_primitive.set_positions(positions.collect::<Vec<[f32; 3]>>());
            }

            if let Some(normals) = reader.read_normals() {
                temp_primitive.set_normals(normals.collect::<Vec<[f32; 3]>>());
            }

            if let Some(tex_coords) = reader.read_tex_coords(0) {
                temp_primitive.set_uvs(tex_coords.into_f32().collect::<Vec<[f32; 2]>>());
            }

            if let Some(weights) = reader.read_weights(0) {
                temp_primitive.set_bone_weights(weights.into_f32().collect::<Vec<[f32; 4]>>())
            }

            if let Some(joints) = reader.read_joints(0) {
                temp_primitive.set_bone_ids(
                    joints
                        .into_u16()
                        .map(|joint| joint.map(|id| id as i32))
                        .collect::<Vec<[i32; 4]>>(),
                );
            }

            if let Some(indices) = reader.read_indices() {
                temp_primitive.indices = Some(indices.into_u32().collect::<Vec<u32>>());
            }

            temp_mesh.primitives.push(temp_primitive);
        });

        meshes.push(temp_mesh);
    });

    Ok(meshes)
}

fn load_materials(file: &GltfFile) -> Result<Vec<Material>, Error> {
    let doc = &file.document;

    Ok(doc
        .materials()
        .map(|material| {
            let mut mat = Material::default();
            if let Some(index) = material.index() {
                mat.id = Some(index);
                mat.base_color_factor = material.pbr_metallic_roughness().base_color_factor();
                mat.metallic_factor = material.pbr_metallic_roughness().metallic_factor();
                mat.roughness_factor = material.pbr_metallic_roughness().roughness_factor();
                if let Some(base_color_texture) =
                    material.pbr_metallic_roughness().base_color_texture()
                {
                    mat.base_color_texture = Some(base_color_texture.texture().index());
                }

                if let Some(metallic_roughness) = material
                    .pbr_metallic_roughness()
                    .metallic_roughness_texture()
                {
                    mat.metallic_roughness_texture = Some(metallic_roughness.texture().index());
                }
            }

            return mat;
        })
        .collect::<Vec<Material>>())
}

fn load_textures(file: &GltfFile) -> Result<Vec<Texture>, Error> {
    let doc = &file.document;
    let buffers = &file.buffers;

    Ok(doc
        .textures()
        .map(|texture| {
            // let mut cpu_tex =
            let src = texture.source().source();

            match src {
                gltf::image::Source::Uri { uri, .. } => {
                    let parent = Path::new(&file.folder[..]);
                    Texture::from_path(parent.join(uri).as_path(), texture.index(), false).unwrap()
                }

                gltf::image::Source::View { view, mime_type } => {
                    let buffer = &buffers[view.buffer().index()];
                    let start = view.offset();
                    let end = start + view.length();
                    let image_bytes = &buffer[start..end];

                    match mime_type {
                        "image/jpeg" => Texture::from_dynamic_image(
                            image::load_from_memory_with_format(
                                image_bytes,
                                image::ImageFormat::Jpeg,
                            )
                            .unwrap(),
                            texture.index(),
                            TextureFormat::Rgb8,
                        )
                        .unwrap(),
                        "image/png" => Texture::from_dynamic_image(
                            image::load_from_memory_with_format(
                                image_bytes,
                                image::ImageFormat::Png,
                            )
                            .unwrap(),
                            texture.index(),
                            TextureFormat::Rgba8,
                        )
                        .unwrap(),
                        _ => panic!("unsupported image type"),
                    }
                }
            }
        })
        .collect::<Vec<Texture>>())
}

fn load_skins(file: &GltfFile) -> Result<Vec<Skin>, Error> {
    let doc = &file.document;

    Ok(doc
        .skins()
        .map(|skin| {
            let mut final_skin = Skin::new(skin.index());

            if let Some(skeleton) = skin.skeleton() {
                final_skin.skeleton = Some(skeleton.index());
            };

            // if let Some(inverse_bind_poses) = skin.inverse_bind_matrices() {}

            skin.joints()
                .for_each(|joint| final_skin.joints.push(joint.index()));

            final_skin
        })
        .collect::<Vec<Skin>>())
}

fn load_nodes(file: &GltfFile) -> Result<Vec<Node>, Error> {
    let doc = &file.document;

    Ok(doc
        .nodes()
        .map(|node| {
            let mut final_node = Node::new();

            let children = node
                .children()
                .map(|child| child.index())
                .collect::<Vec<usize>>();

            final_node.with_children(children);

            if let Some(name) = node.name() {
                final_node.with_name(String::from(name));
            }

            if let Some(mesh) = node.mesh() {
                final_node.with_mesh(mesh.index());
            }

            if let Some(skin) = node.skin() {
                final_node.with_skin(skin.index());
            }

            let mut final_transform = Transform::DEFAULT;
            let tranform = node.transform().decomposed();
            final_transform.translation = Vec3::from(&tranform.0);
            final_transform.orientation = Quat::from(&tranform.1);
            final_transform.scaling = Vec3::from(&tranform.2);

            final_node.transform = final_transform;

            return final_node;
        })
        .collect::<Vec<Node>>())
}

pub fn load_scenes(file: &GltfFile) -> Vec<Scene> {
    let doc = &file.document;
    doc.scenes()
        .map(|scene| {
            let mut final_scene = Scene::new();
            final_scene.set_nodes(
                scene
                    .nodes()
                    .map(|node| node.index())
                    .collect::<Vec<usize>>(),
            );

            final_scene
        })
        .collect::<Vec<Scene>>()
}
