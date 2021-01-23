#version 450

// Inputs
layout(location = 0) in vec3 Vertex_Position;
// layout(location = 1) in vec3 Vertex_Normal; // Normals aren't needed
layout(location = 2) in vec2 Vertex_Uv;

// Outputs
layout(location = 0) out vec2 v_Uv;

// World Uniforms
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

// Tileset uniforms
layout(set = 2, binding = 0) uniform LdtkTilemapMaterial_scale {
    float map_scale;
};
layout(set = 2, binding = 1) uniform LdtkTilemapMaterial_map_info {
    uint map_width_tiles;
    uint map_height_tiles;
};
layout(set = 2, binding = 2) uniform LdtkTilemapMaterial_tileset_info {
    uint tileset_width_tiles;
    uint tileset_height_tiles;
};
layout(set = 2, binding = 3) uniform texture2D LdtkTilemapMaterial_texture;
layout(set = 2, binding = 4) uniform sampler LdtkTilemapMaterial_texture_sampler;
struct TileInfo {
    uint tile_index;
    bool flip_x;
    bool flip_y;
};
layout(set = 2, binding = 5) buffer LdtkTilemapMaterial_tiles {
    uint[] map_tiles;
};

void main() {
    // Scale the mesh up to mach the size of the map
    vec3 pos = vec3(
        Vertex_Position.x * map_width_tiles,
        Vertex_Position.y * map_height_tiles,
        Vertex_Position.z
    );

    // Output the vertex UV for use in the fragment shader
    v_Uv = Vertex_Uv;

    // Set the vertice positions
    gl_Position = ViewProj * Model * vec4(pos * 50, 1);
}
