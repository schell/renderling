struct type_10 {
    member: array<f32, 3>,
    member_1: array<f32, 2>,
}

struct VertexOutput {
    @location(0) member: vec2<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
var<private> global_1: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_2: vec2<f32>;

fn function() {
    var local: array<type_10, 6>;

    let _e25 = global;
    local = array<type_10, 6>(type_10(array<f32, 3>(-1f, -1f, 0f), array<f32, 2>(0f, 1f)), type_10(array<f32, 3>(1f, -1f, 0f), array<f32, 2>(1f, 1f)), type_10(array<f32, 3>(1f, 1f, 0f), array<f32, 2>(1f, 0f)), type_10(array<f32, 3>(-1f, -1f, 0f), array<f32, 2>(0f, 1f)), type_10(array<f32, 3>(1f, 1f, 0f), array<f32, 2>(1f, 0f)), type_10(array<f32, 3>(-1f, 1f, 0f), array<f32, 2>(0f, 0f)));
    if (_e25 < 6u) {
        let _e29 = local[_e25].member;
        let _e32 = local[_e25].member_1;
        global_2 = vec2<f32>(_e32[0], _e32[1]);
        global_1 = vec4<f32>(_e29[0], _e29[1], _e29[2], 1f);
    }
    return;
}

@vertex 
fn convolutionvertex_brdf_lut_convolution(@builtin(vertex_index) param: u32) -> VertexOutput {
    global = param;
    function();
    let _e5 = global_1.y;
    global_1.y = -(_e5);
    let _e7 = global_2;
    let _e8 = global_1;
    return VertexOutput(_e7, _e8);
}
