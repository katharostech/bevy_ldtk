#version 450

// # Fragment Shader
//
// Our fragment shader is responsible for rendering the pixels ( fragments ) in our tilemap by
// selecting pixels from our tileset texture and placing them on the surface of our quad.
//
// It works by taking the tilemap information in our `LdtkTilemapLayer` render resource, and then
// working out the color that the current fragment should be based on the UV position the fragment
// is in on the quad.

// ## Inputs

// We take the UV value from the vertex shader
layout(location = 0) in vec2 v_Uv;

// We output the color of this fragment
layout(location = 0) out vec4 o_Color;

// ### Tileset uniforms
//
// These tileset uiniforms are added to the shader inputs in `pipeline.rs` and correspond directly
// to our `LdtkTilemapLayer` struct. Bevy automaticaly maps our struct to these bindings based on
// the naming convention of `StructName_field_name`.
layout(set = 2, binding = 0) uniform LdtkTilemapLayer_scale {
    float map_scale;
};
layout(set = 2, binding = 1) uniform LdtkTilemapLayer_map_info {
    uint map_width_tiles;
    uint map_height_tiles;
    uint layer_index;
};
layout(set = 2, binding = 2) uniform LdtkTilemapLayer_tileset_info {
    uint tileset_width_tiles;
    uint tileset_height_tiles;
    uint tileset_grid_size;
};
// These texture uniforms are automatically added by Bevy to represent the `Handle<Texture>` that
// was in our corresponding Rust struct.
layout(set = 2, binding = 3) uniform texture2D LdtkTilemapLayer_texture;
layout(set = 2, binding = 4) uniform sampler LdtkTilemapLayer_texture_sampler;
struct TileInfo {
    uint index;
    uint flip_bits;
};
layout(set = 2, binding = 5) buffer LdtkTilemapLayer_tiles {
    TileInfo[] map_tiles;
};

void main() {
    // We use the maximum number in a uint to represent an empty tile, and assign it to a constant
    uint EMPTY_TILE_IDX = 4294967295;

    // Create a map size vector from the width and height of the map
    vec2 map_size = vec2(map_width_tiles, map_height_tiles);

    // Get the x index of the tile in the map by rounding which square this fragment is in
    uint map_tile_x = map_width_tiles - uint(floor(v_Uv.x * map_width_tiles)) - 1;
    // Get the y index of the tile in the map
    uint map_tile_y = uint(floor(v_Uv.y * map_height_tiles));
    // combine that into our map tile vector
    vec2 map_tile = vec2(map_tile_x, map_tile_y);

    // Get the index of the tile in the map as counted left to right, top to bottom
    uint map_tile_idx = uint(map_tile_x + (map_tile_y * map_width_tiles));

    // Use that tile index to read from our map tiles buffer and get the info for the current
    // tile.
    TileInfo tile_info = map_tiles[map_tile_idx];

    // Get the index of the tileset tile that we should fill this map tile with
    uint tileset_tile_idx = tile_info.index;

    // Check whether or not the tileset index for this tile in the map is not the empty tile.
    // If it isn't we can render the color for this fragment.
    if (tileset_tile_idx != EMPTY_TILE_IDX) {

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

        // If the flip x bit is not set, flip the tile UV along the x axis,
        // ( for some reason it is backward by default ).
        if (!((tile_info.flip_bits & 1) != 0)) {
            tile_uv.x = 1 - tile_uv.x;
        }
        // And the same for the y axis
        if ((tile_info.flip_bits & 2) != 0) {
            tile_uv.y = 1 - tile_uv.y;
        }

        // Take the tile UV and convert it to a pixelated tile UV, that samples the same single
        // coordinate from the tileset for the whole pixel in the map. In other words, grab the
        // center of the pixel in our tileset to get the color. This helps prevent bleeding colors
        // in between tiles in the map.
        vec2 pixel_tile_uv = 
            // round the tile coordinate down to the closest pixel
            floor(tile_uv * tileset_grid_size) / tileset_grid_size
            // and add half a pixel's width to grab the center of the pixel in the tileset
            + 0.5 / float(tileset_grid_size);

        // Sample our fragment from the tileset texture
        o_Color = texture(
            sampler2D(LdtkTilemapLayer_texture, LdtkTilemapLayer_texture_sampler),
            // The UV coordinate calculated here is the location from the tileset that we take our
            // pixels. We calculate it by offsetting the UV according to the location of the tile in
            // the tileset, and then adding the tile UV scaled to the size of a tilemap tile.
            tileset_tile * tileset_tile_size + pixel_tile_uv * tileset_tile_size
        );

    // If this is an empty tile, just make it transparent
    } else {
        o_Color = vec4(0, 1, 0, 0);
    }
}
