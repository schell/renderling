struct VertexOutput {
    @location(0) color: vec4<f32>,
    @builtin(position) clip_pos: vec4<f32>,
}
@vertex 
fn main(@builtin(vertex_index) index: u32) -> VertexOutput {
    let x = f32(1i - bitcast<i32>(index)) * 0.5f;
    let y = (f32(index & 1u) * 2f - 1f) * 0.5f;
    let position = vec4<f32>(x, y, 0f, 1f);

    let color = vec4<f32>(1f, 0f, 0f, 1f);
    return VertexOutput(color, position);
}
