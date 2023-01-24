struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(2) position: vec2<f32>,
    @location(3) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

struct Globals {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Globals;

@vertex
fn vs_main(
    input: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let instance_pos = vec4<f32>(instance.position, 0.0, 0.0);
    let vertex_pos = vec4<f32>(input.position, 1.0);

    out.clip_position = camera.view_proj * (instance_pos + vertex_pos);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}