struct type_9 {
    member: vec2<f32>,
    member_1: vec2<f32>,
}

struct VertexOutput {
    @location(0) member: vec2<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
var<private> global_1: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_2: vec2<f32>;

fn function() {
    var local: array<type_9, 6>;

    let _e22 = global;
    local = array<type_9, 6>(type_9(vec2<f32>(-1f, 1f), vec2<f32>(0f, 0f)), type_9(vec2<f32>(-1f, -1f), vec2<f32>(0f, 1f)), type_9(vec2<f32>(1f, -1f), vec2<f32>(1f, 1f)), type_9(vec2<f32>(-1f, 1f), vec2<f32>(0f, 0f)), type_9(vec2<f32>(1f, -1f), vec2<f32>(1f, 1f)), type_9(vec2<f32>(1f, 1f), vec2<f32>(1f, 0f)));
    if (_e22 < 6u) {
        let _e26 = local[_e22].member;
        let _e29 = local[_e22].member_1;
        global_2 = _e29;
        global_1 = vec4<f32>(_e26.x, _e26.y, 0f, 1f);
    }
    return;
}

@vertex 
fn tonemappingvertex(@builtin(vertex_index) param: u32) -> VertexOutput {
    global = param;
    function();
    let _e5 = global_1.y;
    global_1.y = -(_e5);
    let _e7 = global_2;
    let _e8 = global_1;
    return VertexOutput(_e7, _e8);
}
