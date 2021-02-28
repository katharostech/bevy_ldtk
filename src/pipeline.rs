use bevy::{
    core::Byteable,
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::PipelineDescriptor,
        render_graph::{base, RenderGraph, RenderResourcesNode},
        renderer::{RenderResource, RenderResources},
    },
};

// Create a handle to our pipeline that we can use later when we want to spawn our tilemap. We just
// have to create a unique ID for our pipeline and then we will register our pipeline with the
// `Assets<Pipeline>` using this handle.
pub const LDTK_TILEMAP_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 10348532193540037685);

/// Builds the pipeline used to render a tilemap layer. The configuration here is taken from the
/// pipeline configuration used to render Bevy sprites in the `bevy_sprite` crate. The difference is
/// our custom shaders.
fn build_ldtk_tilemap_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    use bevy::render::{
        pipeline::*,
        shader::{ShaderStage, ShaderStages},
        texture::*,
    };

    #[cfg(not(feature = "bevy-unstable"))]
    return PipelineDescriptor {
        rasterization_state: Some(RasterizationStateDescriptor {
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        }),
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
        sample_count: 0,
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("pipeline/tilemap.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("pipeline/tilemap.frag"),
            ))),
        })
    };

    #[cfg(feature = "bevy-unstable")]
    return PipelineDescriptor {
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilState {
                front: StencilFaceState::IGNORE,
                back: StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            },
            clamp_depth: false,
        }),
        color_target_states: vec![ColorTargetState {
            format: TextureFormat::default(),
            color_blend: BlendState {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: BlendState {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            write_mask: ColorWrite::ALL,
        }],
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::Back,
            polygon_mode: PolygonMode::Fill,
        },
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("pipeline/tilemap.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("pipeline/tilemap.frag"),
            ))),
        })
    };
}

/// This is the struct containing all of the information sent to the shaders that will render our
/// tilemap layer.
///
/// Each map is rendered in layers that are a child of the main `Handle<LdtkMap>` entity.
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c620"]
pub struct LdtkTilemapLayer {
    /// The scale of the map
    pub scale: f32,
    /// Information about the map itself
    pub map_info: LdtkTilemapMapInfo,
    /// Information about this layer's tileset
    pub tileset_info: LdtkTilemapTilesetInfo,
    /// The handle to the texture for the layer's tileset
    pub texture: Handle<Texture>,
    /// The list of all of the tiles in the map
    #[render_resources(buffer)]
    pub tiles: Vec<LdtkTilemapTileInfo>,
}

/// Information about the tilemap used by the GPU shaders
#[repr(C)]
#[derive(RenderResource, Default, Debug, Clone, Copy)]
pub struct LdtkTilemapMapInfo {
    /// The number of tiles wide the map is
    pub width: u32,
    /// The number of tiles tall the map is
    pub height: u32,
    /// The layer number for this map layer, counted starting at 0, with 0 being the lowest layer
    pub layer_index: u32,
    /// Whether or not to center the map around the origin ( Using the `u32` type because bools
    /// don't seem to work right for some reason. `0` means `false` and `1` means `true` )
    pub center_map: u32,
}
unsafe impl Byteable for LdtkTilemapMapInfo {}

/// Information about a layer's tileset used by the GPU shaders
#[repr(C)]
#[derive(RenderResource, Default, Debug, Clone, Copy)]
pub struct LdtkTilemapTilesetInfo {
    /// The number of tiles wide the tileset is
    pub width: u32,
    /// The number of tiles tall the tileset is
    pub height: u32,
    /// The number of pixels wide ( and tall ) a tile in the tileset grid is
    pub grid_size: u32,
}
unsafe impl Byteable for LdtkTilemapTilesetInfo {}

/// The information about a specific tile in a map layer
#[repr(C)]
#[derive(RenderResource, Default, Debug, Clone, Copy)]
pub struct LdtkTilemapTileInfo {
    /// The index of the tile image in the tileset texture
    pub tile_index: u32,
    /// First bit means flip x second bit means flip y:
    /// 0 == no flip
    /// 1 == flip x
    /// 2 == flip y
    /// 3 == flip both
    pub flip_bits: u32,
}
unsafe impl Byteable for LdtkTilemapTileInfo {}

/// This module is created just to hold the constants for our render graph node names
pub mod node {
    /// The name of the tilemap render graph node
    pub const LDTK_TILEMAP: &'static str = "ldtk_tile_map";
}

/// Configure the render pipeline for LDtk maps
pub(crate) fn configure_pipeline(app: &AppBuilder) {
    // Get the app resources
    let resources = app.resources();
    let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
    let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
    let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();

    // Add our pipeline asset using the handle we created above. This will allow us to access our
    // pipeline when spawning our tilemap layers using the handle.
    pipelines.set_untracked(
        LDTK_TILEMAP_PIPELINE_HANDLE,
        build_ldtk_tilemap_pipeline(&mut shaders),
    );

    // Add our render LdtkTilemap render resources to the render graph. This makes sure that if we
    // create an entity with an `LdtkTilemapLayer` component, that the data will be sent to our
    // custom shaders.
    render_graph.add_system_node(
        node::LDTK_TILEMAP,
        RenderResourcesNode::<LdtkTilemapLayer>::new(false),
    );

    // We also connect our new render node to the main pass node so that it will get applied when
    // rendering the main pass that all rendered objects are on by default.
    render_graph
        .add_node_edge(node::LDTK_TILEMAP, base::node::MAIN_PASS)
        .unwrap();
}
