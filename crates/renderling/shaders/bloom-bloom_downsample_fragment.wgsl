struct type_3 {
    member: array<u32>,
}

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: vec2<f32>;
@group(0) @binding(2) 
var global_2: sampler;
@group(0) @binding(1) 
var global_3: texture_2d<f32>;
var<private> global_4: u32;
var<private> global_5: vec4<f32>;

fn function() {
    var phi_329_: bool;
    var phi_90_: vec2<f32>;

    let _e20 = arrayLength((&global.member));
    let _e21 = global_1;
    let _e22 = global_4;
    if (_e20 >= 2u) {
        phi_329_ = (_e22 <= (_e20 - 2u));
    } else {
        phi_329_ = false;
    }
    let _e27 = phi_329_;
    if _e27 {
        let _e30 = global.member[_e22];
        let _e35 = global.member[(_e22 + 1u)];
        phi_90_ = vec2<f32>(bitcast<f32>(_e30), bitcast<f32>(_e35));
    } else {
        phi_90_ = vec2<f32>(0f, 0f);
    }
    let _e39 = phi_90_;
    let _e44 = fma(-2f, _e39.x, _e21.x);
    let _e47 = fma(2f, _e39.y, _e21.y);
    let _e49 = textureSample(global_3, global_2, vec2<f32>(_e44, _e47));
    let _e51 = textureSample(global_3, global_2, vec2<f32>(_e21.x, _e47));
    let _e52 = vec2<f32>((2f * _e39.x), (2f * _e39.y));
    let _e54 = textureSample(global_3, global_2, (_e21 + _e52));
    let _e56 = textureSample(global_3, global_2, vec2<f32>(_e44, _e21.y));
    let _e57 = textureSample(global_3, global_2, _e21);
    let _e58 = fma(2f, _e39.x, _e21.x);
    let _e60 = textureSample(global_3, global_2, vec2<f32>(_e58, _e21.y));
    let _e62 = textureSample(global_3, global_2, (_e21 - _e52));
    let _e63 = fma(-2f, _e39.y, _e21.y);
    let _e65 = textureSample(global_3, global_2, vec2<f32>(_e21.x, _e63));
    let _e67 = textureSample(global_3, global_2, vec2<f32>(_e58, _e63));
    let _e71 = textureSample(global_3, global_2, vec2<f32>((_e21.x - _e39.x), (_e21.y + _e39.y)));
    let _e73 = textureSample(global_3, global_2, (_e21 + _e39));
    let _e75 = textureSample(global_3, global_2, (_e21 - _e39));
    let _e79 = textureSample(global_3, global_2, vec2<f32>((_e21.x + _e39.x), (_e21.y - _e39.y)));
    global_5 = vec4<f32>(max(fma((((_e49.x + _e54.x) + _e62.x) + _e67.x), 0.03125f, fma(0.125f, (_e57.x + (((_e71.x + _e73.x) + _e75.x) + _e79.x)), ((((_e51.x + _e56.x) + _e65.x) + _e60.x) * 0.0625f))), 0.00000011920929f), max(fma((((_e49.y + _e54.y) + _e62.y) + _e67.y), 0.03125f, fma(0.125f, (_e57.y + (((_e71.y + _e73.y) + _e75.y) + _e79.y)), ((((_e51.y + _e56.y) + _e65.y) + _e60.y) * 0.0625f))), 0.00000011920929f), max(fma((((_e49.z + _e54.z) + _e62.z) + _e67.z), 0.03125f, fma(0.125f, (_e57.z + (((_e71.z + _e73.z) + _e75.z) + _e79.z)), ((((_e51.z + _e56.z) + _e65.z) + _e60.z) * 0.0625f))), 0.00000011920929f), max(fma((((_e49.w + _e54.w) + _e62.w) + _e67.w), 0.03125f, fma(0.125f, (_e57.w + (((_e71.w + _e73.w) + _e75.w) + _e79.w)), ((((_e51.w + _e56.w) + _e65.w) + _e60.w) * 0.0625f))), 1f));
    return;
}

@fragment 
fn bloombloom_downsample_fragment(@location(0) param: vec2<f32>, @location(1) @interpolate(flat) param_1: u32) -> @location(0) vec4<f32> {
    global_1 = param;
    global_4 = param_1;
    function();
    let _e5 = global_5;
    return _e5;
}
