struct VertexOutput {
    @location(0) member: vec4<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
var<private> global_1: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_2: vec4<f32>;

fn function() {
    let _e11 = global;
    global_2 = vec4<f32>(1f, 0f, 0f, 1f);
    global_1 = vec4<f32>((f32((1i - bitcast<i32>(_e11))) * 0.5f), (fma(f32((_e11 & 1u)), 2f, -1f) * 0.5f), 0f, 1f);
    return;
}

@vertex 
fn tutorialtutorial_implicit_isosceles_vertex(@builtin(vertex_index) param: u32) -> VertexOutput {
    global = param;
    function();
    let _e5 = global_1.y;
    global_1.y = -(_e5);
    let _e7 = global_2;
    let _e8 = global_1;
    return VertexOutput(_e7, _e8);
}
