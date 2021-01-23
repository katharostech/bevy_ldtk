use bevy::{
    core::Byteable,
    prelude::*,
    reflect::TypeUuid,
    render::pipeline::BlendFactor,
    render::pipeline::BlendOperation,
    render::{
        pipeline::{
            BlendDescriptor, ColorStateDescriptor, ColorWrite, CullMode, FrontFace,
            PipelineDescriptor, RasterizationStateDescriptor,
        },
        render_graph::{base, RenderGraph, RenderResourcesNode},
        renderer::{RenderResource, RenderResources},
        shader::{ShaderStage, ShaderStages},
        texture::TextureFormat,
    },
render::pipeline::DepthStencilStateDescriptor, render::pipeline::CompareFunction, render::pipeline::StencilStateFaceDescriptor, render::pipeline::StencilStateDescriptor};

pub const LDTK_TILEMAP_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 10348532193540037685);

fn build_ldtk_tilemap_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor {
        rasterization_state: Some(RasterizationStateDescriptor {
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        }),
        // TODO: This was taken from the Bevy Sprite rendering settings, but for some reason
        // the tiles dissapear when uncommenting it
        depth_stencil_state: Some(DepthStencilStateDescriptor {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilStateDescriptor {
                front: StencilStateFaceDescriptor::IGNORE,
                back: StencilStateFaceDescriptor::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
        }),
        color_states: vec![ColorStateDescriptor {
            format: TextureFormat::default(),
            color_blend: BlendDescriptor {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: BlendDescriptor {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            write_mask: ColorWrite::ALL,
        }],
        ..PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("pipeline/tilemap.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("pipeline/tilemap.frag"),
            ))),
            // vertex: asset_server.load("tilemap.vert"),
            // fragment: Some(asset_server.load("tilemap.frag")),
        })
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c620"]
pub struct LdtkTilemapMaterial {
    pub scale: f32,
    pub map_info: LdtkTilemapMapInfo,
    pub tileset_info: LdtkTilemapTilesetInfo,
    pub texture: Handle<Texture>,
    #[render_resources(buffer)]
    pub tiles: Vec<LdtkTilemapTileInfo>,
}

#[repr(C)]
#[derive(RenderResource, Default, Debug, Clone, Copy)]
pub struct LdtkTilemapMapInfo {
    pub width: u32,
    pub height: u32,
    pub layer_index: u32,
}
unsafe impl Byteable for LdtkTilemapMapInfo {}

#[repr(C)]
#[derive(RenderResource, Default, Debug, Clone, Copy)]
pub struct LdtkTilemapTilesetInfo {
    pub width: u32,
    pub height: u32,
}
unsafe impl Byteable for LdtkTilemapTilesetInfo {}

#[repr(C)]
#[derive(RenderResource, Default, Debug, Clone, Copy)]
pub struct LdtkTilemapTileInfo {
    pub tile_index: u32,
    /// First bit means flip x second bit means flip y:
    /// 0 == no flip
    /// 1 == flip x
    /// 2 == flip y
    /// 3 == flip both
    pub flip_bits: u32,
}
unsafe impl Byteable for LdtkTilemapTileInfo {}

pub mod node {
    pub const LDTK_TILEMAP: &'static str = "ldtk_tile_map";
}

pub(crate) fn configure_pipeline(app: &AppBuilder) {
    // Get the app resources
    let resources = app.resources();
    let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
    let asset_server = resources.get::<AssetServer>().unwrap();
    asset_server.watch_for_changes().unwrap();
    let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
    let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();

    // Add our pipeline asset
    pipelines.set_untracked(
        LDTK_TILEMAP_PIPELINE_HANDLE,
        build_ldtk_tilemap_pipeline(&mut shaders),
    );

    // Add our render LdtkTilemap render resources to the render graph  so that the data from that component
    // will be available in the tilemap shader
    render_graph.add_system_node(
        node::LDTK_TILEMAP,
        RenderResourcesNode::<LdtkTilemapMaterial>::new(false),
    );
    render_graph
        .add_node_edge(node::LDTK_TILEMAP, base::node::MAIN_PASS)
        .unwrap();
}
