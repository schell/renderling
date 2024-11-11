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

    switch bitcast<i32>(0u) {
        default: {
            let _e22 = global_1;
            local = array<vec2<f32>, 6>(vec2<f32>(0f, 1f), vec2<f32>(1f, 1f), vec2<f32>(1f, 0f), vec2<f32>(1f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 1f));
            let _e23 = (_e22 < 6u);
            if _e23 {
            } else {
                break;
            }
            let _e25 = local[_e22];
            global = _e25;
            local_1 = array<vec4<f32>, 6>(vec4<f32>(-1f, -1f, 0.5f, 1f), vec4<f32>(1f, -1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(-1f, 1f, 0.5f, 1f), vec4<f32>(-1f, -1f, 0.5f, 1f));
            if _e23 {
            } else {
                break;
            }
            let _e27 = local_1[_e22];
            global_2 = _e27;
            break;
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
