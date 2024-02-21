#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct TaskbarMaterial {
    taskbar_height: f32,
    taskbar_color: vec4<f32>,
    wallpaper_size: vec2<f32>,
};

const amount = 5;

//TODO: optimize
@group(1) @binding(0) var<uniform> material: TaskbarMaterial;
@group(1) @binding(1) var wallpaper_texture: texture_2d<f32>;
@group(1) @binding(2) var wallpaper_sampler: sampler;
@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var texSize = textureDimensions(wallpaper_texture, 0);
    var cover_ratio = material.taskbar_height / f32(texSize.y) * 2.0;
    var uv = vec2<f32>(1.0 - mesh.uv.x, 1.0 - cover_ratio + mesh.uv.y * cover_ratio);

    let d = 1.0 / max(f32(texSize.x), f32(texSize.y));
    var blurred_color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    var total_samples = 0.0;

    for (var i: i32 = -amount; i <= amount; i = i + 1) {
        for (var j: i32 = -amount; j <= amount; j = j + 1) {
            let offset = vec2<f32>(d * f32(i), d * f32(j));
            blurred_color = blurred_color + textureSample(wallpaper_texture, wallpaper_sampler, uv + offset);
            total_samples = total_samples + 1.0;
        }
    }

    blurred_color = blurred_color / total_samples;

    // You can adjust the final mix between the blurred color and the taskbar color
    var out = mix(material.taskbar_color * vec4<f32>(1.0 / 2.5, 2.0, 0.5, 1.0), blurred_color, 0.102);

    return out;
}