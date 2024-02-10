struct VertexOutput {
    @location(0) @interpolate(flat) member: u32,
    @location(1) member_1: vec3<f32>,
    @builtin(position) member_2: vec4<f32>,
}

var<private> global: u32;
var<private> global_1: u32;
var<private> global_2: u32;
var<private> global_3: vec3<f32>;
var<private> global_4: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    var local: array<vec3<f32>, 6>;

    let _e15 = global_1;
    let _e16 = global;
    global_2 = _e15;
    local = array<vec3<f32>, 6>(vec3<f32>(-1f, -1f, 0f), vec3<f32>(1f, -1f, 0f), vec3<f32>(1f, 1f, 0f), vec3<f32>(1f, 1f, 0f), vec3<f32>(-1f, 1f, 0f), vec3<f32>(-1f, -1f, 0f));
    let _e17 = (_e16 % 6u);
    if (_e17 < 6u) {
        let _e20 = local[_e17];
        global_3 = _e20;
        global_4 = vec4<f32>(_e20.x, _e20.y, _e20.z, 1f);
    }
    return;
}

@vertex 
fn sdfsdf_shape_vertex(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_1 = param;
    global = param_1;
    function();
    let _e8 = global_4.y;
    global_4.y = -(_e8);
    let _e10 = global_2;
    let _e11 = global_3;
    let _e12 = global_4;
    return VertexOutput(_e10, _e11, _e12);
}
