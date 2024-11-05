struct VertexOutput {
    @location(0) member: vec2<f32>,
    @location(1) @interpolate(flat) member_1: u32,
    @builtin(position) member_2: vec4<f32>,
}

var<private> global: u32;
var<private> global_1: u32;
var<private> global_2: vec2<f32>;
var<private> global_3: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_4: u32;

fn function() {
    var local: array<vec2<f32>, 6>;
    var local_1: array<vec4<f32>, 6>;

    let _e22 = global;
    let _e23 = global_1;
    let _e24 = (_e22 % 6u);
    local = array<vec2<f32>, 6>(vec2<f32>(0f, 1f), vec2<f32>(1f, 1f), vec2<f32>(1f, 0f), vec2<f32>(1f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 1f));
    let _e25 = (_e24 < 6u);
    if _e25 {
        let _e27 = local[_e24];
        global_2 = _e27;
        local_1 = array<vec4<f32>, 6>(vec4<f32>(-1f, -1f, 0.5f, 1f), vec4<f32>(1f, -1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(-1f, 1f, 0.5f, 1f), vec4<f32>(-1f, -1f, 0.5f, 1f));
        if _e25 {
            let _e29 = local_1[_e24];
            global_3 = _e29;
            global_4 = _e23;
        }
    }
    return;
}

@vertex 
fn bloombloom_vertex(@builtin(vertex_index) param: u32, @builtin(instance_index) param_1: u32) -> VertexOutput {
    global = param;
    global_1 = param_1;
    function();
    let _e8 = global_3.y;
    global_3.y = -(_e8);
    let _e10 = global_2;
    let _e11 = global_4;
    let _e12 = global_3;
    return VertexOutput(_e10, _e11, _e12);
}
