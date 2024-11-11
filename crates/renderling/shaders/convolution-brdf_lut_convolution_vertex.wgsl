struct type_10 {
    member: array<f32, 3>,
    member_1: array<f32, 2>,
}

struct VertexOutput {
    @location(0) member: vec2<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: vec2<f32>;
var<private> global_1: u32;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    var local: array<type_10, 6>;

    switch bitcast<i32>(0u) {
        default: {
            let _e26 = global_1;
            local = array<type_10, 6>(type_10(array<f32, 3>(-1f, -1f, 0f), array<f32, 2>(0f, 1f)), type_10(array<f32, 3>(1f, -1f, 0f), array<f32, 2>(1f, 1f)), type_10(array<f32, 3>(1f, 1f, 0f), array<f32, 2>(1f, 0f)), type_10(array<f32, 3>(-1f, -1f, 0f), array<f32, 2>(0f, 1f)), type_10(array<f32, 3>(1f, 1f, 0f), array<f32, 2>(1f, 0f)), type_10(array<f32, 3>(-1f, 1f, 0f), array<f32, 2>(0f, 0f)));
            if (_e26 < 6u) {
            } else {
                break;
            }
            let _e30 = local[_e26].member;
            let _e33 = local[_e26].member_1;
            global = vec2<f32>(_e33[0], _e33[1]);
            global_2 = vec4<f32>(_e30[0], _e30[1], _e30[2], 1f);
            break;
        }
    }
    return;
}

@vertex 
fn convolutionbrdf_lut_convolution_vertex(@builtin(vertex_index) param: u32) -> VertexOutput {
    global_1 = param;
    function();
    let _e5 = global_2.y;
    global_2.y = -(_e5);
    let _e7 = global;
    let _e8 = global_2;
    return VertexOutput(_e7, _e8);
}
