@group(0) @binding(2) 
var global: sampler;
var<private> global_1: vec4<f32>;
var<private> global_2: vec3<f32>;
@group(0) @binding(1) 
var global_3: texture_2d<f32>;

fn function() {
    var phi_153_: vec3<f32>;

    let _e10 = global_2;
    let _e17 = sqrt(fma(_e10.z, _e10.z, fma(_e10.x, _e10.x, (_e10.y * _e10.y))));
    if (_e17 == 0f) {
        phi_153_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_153_ = (_e10 * (1f / _e17));
    }
    let _e22 = phi_153_;
    let _e31 = textureSample(global_3, global, vec2<f32>(fma(atan2(_e22.z, _e22.x), 0.1591f, 0.5f), fma(asin(_e22.y), 0.31830987f, 0.5f)));
    global_1 = vec4<f32>(_e31.x, _e31.y, _e31.z, 1f);
    return;
}

@fragment 
fn skyboxfragment_equirectangular(@location(0) param: vec3<f32>) -> @location(0) vec4<f32> {
    global_2 = param;
    function();
    let _e3 = global_1;
    return _e3;
}
