@group(0) @binding(2) 
var global: sampler;
var<private> global_1: vec3<f32>;
@group(0) @binding(1) 
var global_2: texture_cube<f32>;
var<private> global_3: vec4<f32>;

fn function() {
    var phi_129_: vec3<f32>;

    let _e7 = global_1;
    let _e14 = sqrt(fma(_e7.z, _e7.z, fma(_e7.x, _e7.x, (_e7.y * _e7.y))));
    if (_e14 == 0f) {
        phi_129_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_129_ = (_e7 * (1f / _e14));
    }
    let _e19 = phi_129_;
    let _e20 = textureSample(global_2, global, _e19);
    global_3 = vec4<f32>(_e20.x, _e20.y, _e20.z, 1f);
    return;
}

@fragment 
fn skyboxskybox_cubemap_fragment(@location(0) param: vec3<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    function();
    let _e3 = global_3;
    return _e3;
}
