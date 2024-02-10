struct type_3 {
    member: array<u32>,
}

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: vec4<f32>;
var<private> global_2: u32;

fn function() {
    let _e8 = global_2;
    if (_e8 < arrayLength((&global.member))) {
        let _e12 = global.member[_e8];
        global_1 = vec4<f32>(bitcast<f32>(_e12), 0f, 0f, 1f);
    }
    return;
}

@fragment 
fn shader_testarray_test(@location(0) @interpolate(flat) param: u32) -> @location(0) vec4<f32> {
    global_2 = param;
    function();
    let _e3 = global_1;
    return _e3;
}
