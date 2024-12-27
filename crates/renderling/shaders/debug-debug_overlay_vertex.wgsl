var<private> global: u32;
var<private> global_1: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    var local: array<vec4<f32>, 6>;

    switch bitcast<i32>(0u) {
        default: {
            let _e14 = global;
            local = array<vec4<f32>, 6>(vec4<f32>(-1f, -1f, 0.5f, 1f), vec4<f32>(1f, -1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(-1f, 1f, 0.5f, 1f), vec4<f32>(-1f, -1f, 0.5f, 1f));
            let _e15 = (_e14 % 6u);
            if (_e15 < 6u) {
            } else {
                break;
            }
            let _e18 = local[_e15];
            global_1 = _e18;
            break;
        }
    }
    return;
}

@vertex 
fn debugdebug_overlay_vertex(@builtin(vertex_index) param: u32) -> @builtin(position) vec4<f32> {
    global = param;
    function();
    let _e4 = global_1.y;
    global_1.y = -(_e4);
    let _e6 = global_1;
    return _e6;
}
