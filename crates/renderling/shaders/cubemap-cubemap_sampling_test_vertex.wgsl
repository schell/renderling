struct type_11 {
    member: vec3<f32>,
}

struct VertexOutput {
    @location(0) member: vec3<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
@group(0) @binding(0) 
var<storage> global_1: type_11;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: vec3<f32>;

fn function() {
    var local: array<vec4<f32>, 6>;

    switch bitcast<i32>(0u) {
        default: {
            let _e16 = global;
            let _e18 = (_e16 % 6u);
            local = array<vec4<f32>, 6>(vec4<f32>(-1f, -1f, 0.5f, 1f), vec4<f32>(1f, -1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(-1f, 1f, 0.5f, 1f), vec4<f32>(-1f, -1f, 0.5f, 1f));
            if (_e18 < 6u) {
            } else {
                break;
            }
            let _e21 = local[_e18];
            global_2 = _e21;
            let _e22 = global_1.member;
            global_3 = _e22;
            break;
        }
    }
    return;
}

@vertex 
fn cubemapcubemap_sampling_test_vertex(@builtin(vertex_index) param: u32) -> VertexOutput {
    global = param;
    function();
    let _e5 = global_2.y;
    global_2.y = -(_e5);
    let _e7 = global_3;
    let _e8 = global_2;
    return VertexOutput(_e7, _e8);
}
