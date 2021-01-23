#version 450

// Input
layout(location = 0) in vec2 v_Uv;

// Output
layout(location = 0) out vec4 o_Color;

// Tilemap uniforms
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
    uint index;
    uint flip_bits;
};
layout(set = 2, binding = 5) buffer LdtkTilemapMaterial_tiles {
    TileInfo[] map_tiles;
};

void main() {
    // TileInfo[] map_tiles = TileInfo[9](
    //     TileInfo(0, 0),
    //     TileInfo(63, 0),
    //     TileInfo(51, 0),
    //     TileInfo(62, 0),
    //     TileInfo(62, 0),
    //     TileInfo(1, 0),
    //     TileInfo(0, 0),
    //     TileInfo(0, 0),
    //     TileInfo(0, 0)
    // );

    vec2 map_size = vec2(map_width_tiles, map_height_tiles);

    // Get the x index of the tile in the map by rounding which square this fragment is in
    uint map_tile_x = map_width_tiles - uint(floor(v_Uv.x * map_width_tiles)) - 1;
    // Get the y index of the tile in the map
    uint map_tile_y = uint(floor(v_Uv.y * map_height_tiles));
    // combine that into our map tile vector
    vec2 map_tile = vec2(map_tile_x, map_tile_y);

    // Get the index of the tile in the map as counted left to right, top to bottom
    uint map_tile_idx = uint(map_tile_x + (map_tile_y * map_width_tiles));
    // Use that tile index to read into our map tiles buffer and get the info for the current
    // tile.
    TileInfo tile_info = map_tiles[map_tile_idx];

    // Get the index of the tileset tile that we should fill this map tile with
    uint tileset_tile_idx = tile_info.index;
    // Calculate the tileset tile y value from the tileset tile index
    uint tileset_tile_y = uint(floor(tileset_tile_idx / tileset_width_tiles));
    // And the tileset tile x value 
    uint tileset_tile_x = tileset_tile_idx - tileset_tile_y * tileset_width_tiles;
    // And combine that to our tileset tile vector
    vec2 tileset_tile = vec2(tileset_tile_x, tileset_tile_y);

    // Next calculate the size of a map tile, as a fraction of the total size of the mesh,
    // which is betwen 0 and 1.
    vec2 map_tile_size = vec2(1 / map_width_tiles, 1 / map_height_tiles);
    // And calculate the size of a tileset tile as a fraction of its texture size
    vec2 tileset_tile_size = vec2(1 / float(tileset_width_tiles), 1 / float(tileset_height_tiles));

    // Flip the x UV of the whole tileset so that it lines up with our left-to-right interpretation
    // of the tilesheet indexes
    vec2 uv = vec2(1 - v_Uv.x, v_Uv.y);
    // Get the Uv across the tile for this part of the map.
    // For instance, 0, 0 for the tile_uv would mean that we need to sample the top left
    // of the tile on the tilemap.
    vec2 tile_uv = (uv * map_size - map_tile);

    // If the flip x bit is set, flip the tile UV along the x axis
    if ((tile_info.flip_bits & 1) != 0) {
        tile_uv.x = 1 - tile_uv.x;
    }
    // And the same for the y axis
    if ((tile_info.flip_bits & 2) != 0) {
        tile_uv.y = 1 - tile_uv.y;
    }

    // Sample our fragment from the tileset texture
    o_Color = texture(
        sampler2D(LdtkTilemapMaterial_texture, LdtkTilemapMaterial_texture_sampler),
        // The UV coordinate calculated here is the location from the tileset that we take
        // our pixels. We calculate it by offsetting the UV according to the location of the
        // tile in the tileset, and then adding the tile UV scaled to the size of a tilemap
        // tile.
        tileset_tile * tileset_tile_size + tile_uv * tileset_tile_size
        // v_Uv
    );

    // o_Color = vec4(tileset_tile * tileset_tile_size + tile_uv * tileset_tile_size, 0, 1);
    // o_Color = vec4(v_Uv, 0, 1);
}
