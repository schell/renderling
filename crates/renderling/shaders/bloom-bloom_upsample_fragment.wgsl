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
    var phi_235_: bool;
    var phi_108_: vec2<f32>;

    let _e17 = arrayLength((&global.member));
    let _e18 = global_1;
    let _e19 = global_4;
    if (_e17 >= 2u) {
        phi_235_ = (_e19 <= (_e17 - 2u));
    } else {
        phi_235_ = false;
    }
    let _e24 = phi_235_;
    if _e24 {
        let _e27 = global.member[_e19];
        let _e32 = global.member[(_e19 + 1u)];
        phi_108_ = vec2<f32>(bitcast<f32>(_e27), bitcast<f32>(_e32));
    } else {
        phi_108_ = vec2<f32>(0f, 0f);
    }
    let _e36 = phi_108_;
    let _e40 = (_e18.x - _e36.x);
    let _e42 = (_e18.y + _e36.y);
    let _e44 = textureSample(global_3, global_2, vec2<f32>(_e40, _e42));
    let _e49 = textureSample(global_3, global_2, vec2<f32>(_e18.x, _e42));
    let _e54 = textureSample(global_3, global_2, (_e18 + _e36));
    let _e59 = textureSample(global_3, global_2, vec2<f32>(_e40, _e18.y));
    let _e63 = textureSample(global_3, global_2, _e18);
    let _e67 = (_e18.x + _e36.x);
    let _e69 = textureSample(global_3, global_2, vec2<f32>(_e67, _e18.y));
    let _e74 = textureSample(global_3, global_2, (_e18 - _e36));
    let _e78 = (_e18.y - _e36.y);
    let _e80 = textureSample(global_3, global_2, vec2<f32>(_e18.x, _e78));
    let _e85 = textureSample(global_3, global_2, vec2<f32>(_e67, _e78));
    global_5 = vec4<f32>(((fma(_e63.x, 4f, ((((_e49.x + _e59.x) + _e69.x) + _e80.x) * 2f)) + (((_e44.x + _e54.x) + _e74.x) + _e85.x)) * 0.0625f), ((fma(_e63.y, 4f, ((((_e49.y + _e59.y) + _e69.y) + _e80.y) * 2f)) + (((_e44.y + _e54.y) + _e74.y) + _e85.y)) * 0.0625f), ((fma(_e63.z, 4f, ((((_e49.z + _e59.z) + _e69.z) + _e80.z) * 2f)) + (((_e44.z + _e54.z) + _e74.z) + _e85.z)) * 0.0625f), 0.5f);
    return;
}

@fragment 
fn bloombloom_upsample_fragment(@location(0) param: vec2<f32>, @location(1) @interpolate(flat) param_1: u32) -> @location(0) vec4<f32> {
    global_1 = param;
    global_4 = param_1;
    function();
    let _e5 = global_5;
    return _e5;
}
