struct type_7 {
    member: array<u32>,
}

struct type_14 {
    member: u32,
    member_1: f32,
}

@group(0) @binding(0) 
var<storage> global: type_7;
var<private> global_1: vec2<f32>;
@group(0) @binding(2) 
var global_2: sampler;
@group(0) @binding(1) 
var global_3: texture_2d<f32>;
var<private> global_4: vec4<f32>;

fn function() {
    var phi_142_: type_14;
    var phi_190_: vec4<f32>;

    let _e53 = global_1;
    let _e54 = textureSample(global_3, global_2, _e53);
    if select(false, true, (arrayLength((&global.member)) >= 2u)) {
        let _e59 = global.member[0u];
        let _e62 = global.member[1u];
        phi_142_ = type_14(_e59, bitcast<f32>(_e62));
    } else {
        phi_142_ = type_14(0u, 1f);
    }
    let _e66 = phi_142_;
    let _e70 = (_e54.x * _e66.member_1);
    let _e72 = (_e54.y * _e66.member_1);
    let _e74 = (_e54.z * _e66.member_1);
    let _e76 = (_e54.w * _e66.member_1);
    switch bitcast<i32>(_e66.member) {
        case 1: {
            let _e98 = min(vec3<f32>(max(((_e70 * fma(2.51f, _e70, 0.03f)) / fma(_e70, fma(2.43f, _e70, 0.59f), 0.14f)), 0f), max(((_e72 * fma(2.51f, _e72, 0.03f)) / fma(_e72, fma(2.43f, _e72, 0.59f), 0.14f)), 0f), max(((_e74 * fma(2.51f, _e74, 0.03f)) / fma(_e74, fma(2.43f, _e74, 0.59f), 0.14f)), 0f)), vec3<f32>(1f, 1f, 1f));
            phi_190_ = vec4<f32>(_e98.x, _e98.y, _e98.z, _e76);
            break;
        }
        case 2: {
            let _e109 = fma(0.04823f, _e74, fma(0.59719f, _e70, (0.35458f * _e72)));
            let _e110 = fma(0.01566f, _e74, fma(0.076f, _e70, (0.90834f * _e72)));
            let _e111 = fma(0.83777f, _e74, fma(0.0284f, _e70, (0.13383f * _e72)));
            let _e124 = (fma(_e109, (_e109 + 0.0245786f), -0.000090537f) / fma(_e109, fma(0.983729f, _e109, 0.432951f), 0.238081f));
            let _e125 = (fma(_e110, (_e110 + 0.0245786f), -0.000090537f) / fma(_e110, fma(0.983729f, _e110, 0.432951f), 0.238081f));
            let _e126 = (fma(_e111, (_e111 + 0.0245786f), -0.000090537f) / fma(_e111, fma(0.983729f, _e111, 0.432951f), 0.238081f));
            let _e140 = min(vec3<f32>(max(fma(-0.07367f, _e126, fma(1.60475f, _e124, (-0.53108f * _e125))), 0f), max(fma(-0.00605f, _e126, fma(-0.10208f, _e124, (1.10813f * _e125))), 0f), max(fma(1.07602f, _e126, fma(-0.00327f, _e124, (-0.07276f * _e125))), 0f)), vec3<f32>(1f, 1f, 1f));
            phi_190_ = vec4<f32>(_e140.x, _e140.y, _e140.z, _e76);
            break;
        }
        case 3: {
            let _e151 = fma(_e74, 0.08038333f, fma(_e70, 0.9953167f, (_e72 * 0.59096664f)));
            let _e152 = fma(_e74, 0.026099999f, fma(_e70, 0.12666667f, (_e72 * 1.5138999f)));
            let _e153 = fma(_e74, 1.3962833f, fma(_e70, 0.047333334f, (_e72 * 0.22304998f)));
            let _e166 = (fma(_e151, (_e151 + 0.0245786f), -0.000090537f) / fma(_e151, fma(0.983729f, _e151, 0.432951f), 0.238081f));
            let _e167 = (fma(_e152, (_e152 + 0.0245786f), -0.000090537f) / fma(_e152, fma(0.983729f, _e152, 0.432951f), 0.238081f));
            let _e168 = (fma(_e153, (_e153 + 0.0245786f), -0.000090537f) / fma(_e153, fma(0.983729f, _e153, 0.432951f), 0.238081f));
            let _e182 = min(vec3<f32>(max(fma(-0.07367f, _e168, fma(1.60475f, _e166, (-0.53108f * _e167))), 0f), max(fma(-0.00605f, _e168, fma(-0.10208f, _e166, (1.10813f * _e167))), 0f), max(fma(1.07602f, _e168, fma(-0.00327f, _e166, (-0.07276f * _e167))), 0f)), vec3<f32>(1f, 1f, 1f));
            phi_190_ = vec4<f32>(_e182.x, _e182.y, _e182.z, _e76);
            break;
        }
        case 4: {
            let _e192 = (vec3<f32>(_e70, _e72, _e74) / vec3<f32>(fma(_e54.x, _e66.member_1, 1f), fma(_e54.y, _e66.member_1, 1f), fma(_e54.z, _e66.member_1, 1f)));
            phi_190_ = vec4<f32>(_e192.x, _e192.y, _e192.z, _e76);
            break;
        }
        default: {
            phi_190_ = vec4<f32>(_e70, _e72, _e74, _e76);
            break;
        }
    }
    let _e198 = phi_190_;
    global_4 = _e198;
    return;
}

@fragment 
fn tonemappingtonemapping_fragment(@location(0) param: vec2<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    function();
    let _e3 = global_4;
    return _e3;
}
