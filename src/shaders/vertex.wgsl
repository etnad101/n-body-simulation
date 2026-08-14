struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct InstanceInput {
    @location(1) position: vec3<f32>,
    @location(2) colour: vec3<f32>,
    @location(3) radius: f32,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) colour: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_pos = camera.view_proj * vec4(model.position * instance.radius + instance.position, 1.0);
    out.colour = instance.colour;
    return out;
}
