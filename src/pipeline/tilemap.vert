#version 450

//
// # Vertex Shader
//
// The vertex shader maps the vertices in the plain quad that the tilemap layer is rendered on and
// positions the vertices in the quad according to size and scale of the map.

// ## Shader Inputs

// ### Vertex attributs
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

// ### World Uniforms
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

// ### Tileset uniforms
//
// These tileset uiniforms are added to the shader inputs in `pipeline.rs` and correspond directly
// to our `LdtkTilemapMaterial` struct. 
layout(set = 2, binding = 0) uniform LdtkTilemapMaterial_scale {
    float map_scale;
};
layout(set = 2, binding = 1) uniform LdtkTilemapMaterial_map_info {
    uint map_width_tiles;
    uint map_height_tiles;
    uint layer_index;
};
layout(set = 2, binding = 2) uniform LdtkTilemapMaterial_tileset_info {
    uint tileset_width_tiles;
    uint tileset_height_tiles;
    uint tileset_grid_size;
};
// These texture uniforms are automatically added by Bevy to represent the `Handle<Texture>` that
// was in our corresponding Rust struct.
layout(set = 2, binding = 3) uniform texture2D LdtkTilemapMaterial_texture;
layout(set = 2, binding = 4) uniform sampler LdtkTilemapMaterial_texture_sampler;
struct TileInfo {
    uint index;
    uint flip_bits;
};
layout(set = 2, binding = 5) buffer LdtkTilemapMaterial_tiles {
    TileInfo[] map_tiles;
};

// ## Outputs
//
// We output the vertice UV for use in the fragment shader.
layout(location = 0) out vec2 v_Uv;

void main() {
    // Calculate a base position for the vertice, scaling it to match the aspect ratio of the
    // tilemap.
    vec3 pos = vec3(
        Vertex_Position.x * map_width_tiles,
        Vertex_Position.y * map_height_tiles,
        // We also push layers down ( away from the camera, into the distance ) based on their layer
        // index to position layers one behind the other. Each layer will be one unit apart.
        Vertex_Position.z + 1 * layer_index
    );

    // Simply forward our v_Uv out variable from the input Vertex_Uv unchanged.
    v_Uv = Vertex_Uv;

    // Set the position of the vertex
    gl_Position = 
        // Add the view and model projections, and multiply the position by the map scale and the
        // tileset grid size. The grid size multiplication makes sure that grid pixels correspond to
        // pixels on the screen, assuming the map_scale is set to 1.
        ViewProj * Model * vec4(pos * map_scale * tileset_grid_size, 1);
}
