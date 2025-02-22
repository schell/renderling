struct type_9 {
    member: vec2<f32>,
    member_1: vec2<f32>,
}

struct VertexOutput {
    @location(0) member: vec2<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
var<private> global_1: vec2<f32>;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    var local: array<type_9, 6>;

    switch bitcast<i32>(0u) {
        default: {
            let _e23 = global;
            local = array<type_9, 6>(type_9(vec2<f32>(-1f, 1f), vec2<f32>(0f, 0f)), type_9(vec2<f32>(-1f, -1f), vec2<f32>(0f, 1f)), type_9(vec2<f32>(1f, -1f), vec2<f32>(1f, 1f)), type_9(vec2<f32>(-1f, 1f), vec2<f32>(0f, 0f)), type_9(vec2<f32>(1f, -1f), vec2<f32>(1f, 1f)), type_9(vec2<f32>(1f, 1f), vec2<f32>(1f, 0f)));
            if (_e23 < 6u) {
            } else {
                break;
            }
            let _e27 = local[_e23].member;
            let _e30 = local[_e23].member_1;
            global_1 = _e30;
            global_2 = vec4<f32>(_e27.x, _e27.y, 0f, 1f);
            break;
        }
    }
    return;
}

@vertex 
fn tonemappingtonemapping_vertex(@builtin(vertex_index) param: u32) -> VertexOutput {
    global = param;
    function();
    let _e5 = global_2.y;
    global_2.y = -(_e5);
    let _e7 = global_1;
    let _e8 = global_2;
    return VertexOutput(_e7, _e8);
}
