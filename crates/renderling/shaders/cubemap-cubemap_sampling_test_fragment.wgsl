@group(0) @binding(2) 
var global: sampler;
var<private> global_1: vec4<f32>;
var<private> global_2: vec3<f32>;
@group(0) @binding(1) 
var global_3: texture_cube<f32>;

fn function() {
    let _e4 = global_2;
    let _e5 = textureSample(global_3, global, _e4);
    global_1 = _e5;
    return;
}

@fragment 
fn cubemapcubemap_sampling_test_fragment(@location(0) param: vec3<f32>) -> @location(0) vec4<f32> {
    global_2 = param;
    function();
    let _e3 = global_1;
    return _e3;
}
