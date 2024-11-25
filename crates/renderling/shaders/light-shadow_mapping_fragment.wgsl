var<private> global: vec4<f32>;
var<private> global_1: vec4<f32>;

fn function() {
    let _e3 = global_1;
    global = vec4<f32>((_e3.x / _e3.w), (_e3.y / _e3.w), (_e3.z / _e3.w), 1f);
    return;
}

@fragment 
fn lightshadow_mapping_fragment(@location(0) param: vec4<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    function();
    let _e3 = global;
    return _e3;
}
