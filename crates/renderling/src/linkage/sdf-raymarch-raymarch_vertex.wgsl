struct VertexOutput {
    @location(0) @interpolate(flat) member: u32,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
var<private> global_1: u32;
var<private> global_2: u32;
var<private> global_3: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    var local: array<vec3<f32>, 6>;

    let _e14 = global;
    let _e15 = global_1;
    global_2 = _e15;
    local = array<vec3<f32>, 6>(vec3<f32>(-1f, -1f, 0f), vec3<f32>(1f, -1f, 0f), vec3<f32>(1f, 1f, 0f), vec3<f32>(1f, 1f, 0f), vec3<f32>(-1f, 1f, 0f), vec3<f32>(-1f, -1f, 0f));
    let _e16 = (_e14 % 6u);
    if (_e16 < 6u) {
        let _e19 = local[_e16];
        global_3 = vec4<f32>(_e19.x, _e19.y, _e19.z, 1f);
    }
    return;
}

@vertex 
fn sdfraymarchraymarch_vertex(@builtin(vertex_index) param: u32, @builtin(instance_index) param_1: u32) -> VertexOutput {
    global = param;
    global_1 = param_1;
    function();
    let _e7 = global_3.y;
    global_3.y = -(_e7);
    let _e9 = global_2;
    let _e10 = global_3;
    return VertexOutput(_e9, _e10);
}
