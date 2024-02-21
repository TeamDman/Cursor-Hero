#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct TaskbarMaterial {
    taskbar_height: f32,
    taskbar_color: vec4<f32>,
    wallpaper_size: vec2<f32>,
};


@group(1) @binding(0) var<uniform> material: TaskbarMaterial;
@group(1) @binding(1) var wallpaper_texture: texture_2d<f32>;
@group(1) @binding(2) var wallpaper_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var texSize = textureDimensions(wallpaper_texture, 0);
    var cover_ratio = material.taskbar_height / f32(texSize.y) * 2.0;
    var uv = vec2<f32>(1.0 - mesh.uv.x, 1.0 - cover_ratio + mesh.uv.y * cover_ratio);
    // return textureSample(wallpaper_texture, wallpaper_sampler, uv);

    var d = 0.001;
    var mult = 0.7;
    var blurred_color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    blurred_color += textureSample(wallpaper_texture, wallpaper_sampler, uv + vec2<f32>(0.0, 0.0)) * mult;
    blurred_color += textureSample(wallpaper_texture, wallpaper_sampler, uv + vec2<f32>(0.0, d)) * mult;
    blurred_color += textureSample(wallpaper_texture, wallpaper_sampler, uv + vec2<f32>(0.0, -d)) * mult;
    blurred_color += textureSample(wallpaper_texture, wallpaper_sampler, uv + vec2<f32>(d, 0.0)) * mult;
    blurred_color += textureSample(wallpaper_texture, wallpaper_sampler, uv + vec2<f32>(-d, 0.0)) * mult;
    blurred_color += textureSample(wallpaper_texture, wallpaper_sampler, uv + vec2<f32>(-d, -d)) * mult;
    blurred_color += textureSample(wallpaper_texture, wallpaper_sampler, uv + vec2<f32>(d, -d)) * mult;
    blurred_color += textureSample(wallpaper_texture, wallpaper_sampler, uv + vec2<f32>(-d, d)) * mult;
    blurred_color += textureSample(wallpaper_texture, wallpaper_sampler, uv + vec2<f32>(d, d)) * mult;
    blurred_color /= 9.0;
    
    blurred_color += textureSample(wallpaper_texture, wallpaper_sampler, uv) * 0.55;
    blurred_color[3] = 1.0;

    // return blurred_color;
    // return material.taskbar_color;
    var out = material.taskbar_color / 2.0 + blurred_color / 2.0;
    // out[3] = 0.75;
    return out;
}
