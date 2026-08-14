struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) colour: vec3<f32>,
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(input.colour, 1.0);
}
