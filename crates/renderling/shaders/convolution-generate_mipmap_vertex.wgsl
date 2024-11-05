struct VertexOutput {
    @location(0) member: vec2<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: vec2<f32>;
var<private> global_1: u32;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    var local: array<vec2<f32>, 6>;
    var local_1: array<vec4<f32>, 6>;

    let _e20 = global_1;
    local = array<vec2<f32>, 6>(vec2<f32>(0f, 1f), vec2<f32>(1f, 1f), vec2<f32>(1f, 0f), vec2<f32>(1f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 1f));
    let _e21 = (_e20 < 6u);
    if _e21 {
        let _e23 = local[_e20];
        global = _e23;
        local_1 = array<vec4<f32>, 6>(vec4<f32>(-1f, -1f, 0.5f, 1f), vec4<f32>(1f, -1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(-1f, 1f, 0.5f, 1f), vec4<f32>(-1f, -1f, 0.5f, 1f));
        if _e21 {
            let _e25 = local_1[_e20];
            global_2 = _e25;
        }
    }
    return;
}

@vertex 
fn convolutiongenerate_mipmap_vertex(@builtin(vertex_index) param: u32) -> VertexOutput {
    global_1 = param;
    function();
    let _e5 = global_2.y;
    global_2.y = -(_e5);
    let _e7 = global;
    let _e8 = global_2;
    return VertexOutput(_e7, _e8);
}
