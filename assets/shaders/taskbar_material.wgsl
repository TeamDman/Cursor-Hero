#import bevy_sprite::mesh2d_vertex_output::VertexOutput


struct TaskbarMaterial {
    color: vec4<f32>,
};


@group(1) @binding(0) var<uniform> material: TaskbarMaterial;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return material.color;
}
