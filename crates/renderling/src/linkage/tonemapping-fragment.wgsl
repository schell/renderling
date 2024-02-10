struct type_2 {
    member: array<u32>,
}

@group(0) @binding(0) 
var<storage> global: type_2;
@group(0) @binding(2) 
var global_1: sampler;
var<private> global_2: vec2<f32>;
var<private> global_3: vec4<f32>;
@group(0) @binding(1) 
var global_4: texture_2d<f32>;

fn function() {
    var phi_180_: vec4<f32>;

    let _e48 = arrayLength((&global.member));
    let _e49 = global_2;
    let _e50 = textureSample(global_4, global_1, _e49);
    if (0u < _e48) {
        let _e54 = global.member[0u];
        if (1u < _e48) {
            let _e58 = global.member[1u];
            let _e59 = bitcast<f32>(_e58);
            let _e61 = (_e50.x * _e59);
            let _e63 = (_e50.y * _e59);
            let _e65 = (_e50.z * _e59);
            let _e67 = (_e50.w * _e59);
            switch bitcast<i32>(_e54) {
                case 1: {
                    let _e89 = min(vec3<f32>(max(((_e61 * fma(2.51f, _e61, 0.03f)) / fma(_e61, fma(2.43f, _e61, 0.59f), 0.14f)), 0f), max(((_e63 * fma(2.51f, _e63, 0.03f)) / fma(_e63, fma(2.43f, _e63, 0.59f), 0.14f)), 0f), max(((_e65 * fma(2.51f, _e65, 0.03f)) / fma(_e65, fma(2.43f, _e65, 0.59f), 0.14f)), 0f)), vec3<f32>(1f, 1f, 1f));
                    phi_180_ = vec4<f32>(_e89.x, _e89.y, _e89.z, _e67);
                    break;
                }
                case 2: {
                    let _e100 = fma(0.04823f, _e65, fma(0.59719f, _e61, (0.35458f * _e63)));
                    let _e101 = fma(0.01566f, _e65, fma(0.076f, _e61, (0.90834f * _e63)));
                    let _e102 = fma(0.83777f, _e65, fma(0.0284f, _e61, (0.13383f * _e63)));
                    let _e115 = (fma(_e100, (_e100 + 0.0245786f), -0.000090537f) / fma(_e100, fma(0.983729f, _e100, 0.432951f), 0.238081f));
                    let _e116 = (fma(_e101, (_e101 + 0.0245786f), -0.000090537f) / fma(_e101, fma(0.983729f, _e101, 0.432951f), 0.238081f));
                    let _e117 = (fma(_e102, (_e102 + 0.0245786f), -0.000090537f) / fma(_e102, fma(0.983729f, _e102, 0.432951f), 0.238081f));
                    let _e131 = min(vec3<f32>(max(fma(-0.07367f, _e117, fma(1.60475f, _e115, (-0.53108f * _e116))), 0f), max(fma(-0.00605f, _e117, fma(-0.10208f, _e115, (1.10813f * _e116))), 0f), max(fma(1.07602f, _e117, fma(-0.00327f, _e115, (-0.07276f * _e116))), 0f)), vec3<f32>(1f, 1f, 1f));
                    phi_180_ = vec4<f32>(_e131.x, _e131.y, _e131.z, _e67);
                    break;
                }
                case 3: {
                    let _e142 = fma(_e65, 0.08038333f, fma(_e61, 0.9953167f, (_e63 * 0.59096664f)));
                    let _e143 = fma(_e65, 0.026099999f, fma(_e61, 0.12666667f, (_e63 * 1.5138999f)));
                    let _e144 = fma(_e65, 1.3962833f, fma(_e61, 0.047333334f, (_e63 * 0.22304998f)));
                    let _e157 = (fma(_e142, (_e142 + 0.0245786f), -0.000090537f) / fma(_e142, fma(0.983729f, _e142, 0.432951f), 0.238081f));
                    let _e158 = (fma(_e143, (_e143 + 0.0245786f), -0.000090537f) / fma(_e143, fma(0.983729f, _e143, 0.432951f), 0.238081f));
                    let _e159 = (fma(_e144, (_e144 + 0.0245786f), -0.000090537f) / fma(_e144, fma(0.983729f, _e144, 0.432951f), 0.238081f));
                    let _e173 = min(vec3<f32>(max(fma(-0.07367f, _e159, fma(1.60475f, _e157, (-0.53108f * _e158))), 0f), max(fma(-0.00605f, _e159, fma(-0.10208f, _e157, (1.10813f * _e158))), 0f), max(fma(1.07602f, _e159, fma(-0.00327f, _e157, (-0.07276f * _e158))), 0f)), vec3<f32>(1f, 1f, 1f));
                    phi_180_ = vec4<f32>(_e173.x, _e173.y, _e173.z, _e67);
                    break;
                }
                case 4: {
                    let _e183 = (vec3<f32>(_e61, _e63, _e65) / vec3<f32>(fma(_e50.x, _e59, 1f), fma(_e50.y, _e59, 1f), fma(_e50.z, _e59, 1f)));
                    phi_180_ = vec4<f32>(_e183.x, _e183.y, _e183.z, _e67);
                    break;
                }
                default: {
                    phi_180_ = vec4<f32>(_e61, _e63, _e65, _e67);
                    break;
                }
            }
            let _e189 = phi_180_;
            global_3 = _e189;
        }
    }
    return;
}

@fragment 
fn tonemappingfragment(@location(0) param: vec2<f32>) -> @location(0) vec4<f32> {
    global_2 = param;
    function();
    let _e3 = global_3;
    return _e3;
}
