var<private> global: vec4<f32>;
var<private> global_1: vec4<f32>;

fn function() {
    let _e2 = global_1;
    global = _e2;
    return;
}

@fragment 
fn tutorialtutorial_passthru_fragment(@location(0) param: vec4<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    function();
    let _e3 = global;
    return _e3;
}
