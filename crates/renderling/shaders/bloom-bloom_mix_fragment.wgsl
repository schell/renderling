struct type_3 {
    member: array<u32>,
}

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: vec2<f32>;
var<private> global_2: vec4<f32>;
var<private> global_3: u32;
@group(0) @binding(2) 
var global_4: sampler;
@group(0) @binding(1) 
var global_5: texture_2d<f32>;
@group(0) @binding(4) 
var global_6: sampler;
@group(0) @binding(3) 
var global_7: texture_2d<f32>;

fn function() {
    var phi_133_: bool;
    var phi_81_: f32;

    let _e14 = arrayLength((&global.member));
    let _e15 = global_1;
    let _e16 = global_3;
    if (_e14 >= 1u) {
        phi_133_ = (_e16 <= (_e14 - 1u));
    } else {
        phi_133_ = false;
    }
    let _e21 = phi_133_;
    if _e21 {
        let _e24 = global.member[_e16];
        phi_81_ = bitcast<f32>(_e24);
    } else {
        phi_81_ = 0f;
    }
    let _e27 = phi_81_;
    let _e28 = textureSample(global_5, global_4, _e15);
    let _e32 = textureSample(global_7, global_6, _e15);
    let _e36 = (1f - _e27);
    global_2 = vec4<f32>(fma(_e28.x, _e36, (_e32.x * _e27)), fma(_e28.y, _e36, (_e32.y * _e27)), fma(_e28.z, _e36, (_e32.z * _e27)), 1f);
    return;
}

@fragment 
fn bloombloom_mix_fragment(@location(0) param: vec2<f32>, @location(1) @interpolate(flat) param_1: u32) -> @location(0) vec4<f32> {
    global_1 = param;
    global_3 = param_1;
    function();
    let _e5 = global_2;
    return _e5;
}
