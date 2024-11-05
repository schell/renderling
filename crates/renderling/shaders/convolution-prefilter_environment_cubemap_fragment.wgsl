struct type_7 {
    member: u32,
    member_1: u32,
}

@group(0) @binding(2) 
var global: sampler;
var<private> global_1: vec4<f32>;
var<private> global_2: vec3<f32>;
var<private> global_3: f32;
@group(0) @binding(1) 
var global_4: texture_cube<f32>;

fn function() {
    var phi_446_: vec3<f32>;
    var phi_103_: type_7;
    var phi_106_: vec3<f32>;
    var phi_108_: f32;
    var phi_123_: type_7;
    var phi_124_: type_7;
    var phi_627_: vec3<f32>;
    var phi_662_: vec3<f32>;
    var phi_697_: vec3<f32>;
    var phi_187_: f32;
    var phi_206_: vec3<f32>;
    var phi_207_: f32;
    var phi_208_: bool;
    var phi_104_: type_7;
    var phi_107_: vec3<f32>;
    var phi_109_: f32;

    let _e41 = global_2;
    let _e42 = global_3;
    let _e49 = sqrt(fma(_e41.z, _e41.z, fma(_e41.x, _e41.x, (_e41.y * _e41.y))));
    if (_e49 == 0f) {
        phi_446_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_446_ = (_e41 * (1f / _e49));
    }
    let _e54 = phi_446_;
    let _e56 = (_e54.y * -1f);
    phi_103_ = type_7(0u, 1024u);
    phi_106_ = vec3<f32>(0f, 0f, 0f);
    phi_108_ = 0f;
    loop {
        let _e58 = phi_103_;
        let _e60 = phi_106_;
        let _e62 = phi_108_;
        if (_e58.member < _e58.member_1) {
            phi_123_ = type_7((_e58.member + 1u), _e58.member_1);
            phi_124_ = type_7(1u, _e58.member);
        } else {
            phi_123_ = _e58;
            phi_124_ = type_7(0u, type_7().member_1);
        }
        let _e75 = phi_123_;
        let _e77 = phi_124_;
        switch bitcast<i32>(_e77.member) {
            case 0: {
                global_1 = vec4<f32>((_e60.x / _e62), (_e60.y / _e62), (_e60.z / _e62), 1f);
                phi_208_ = false;
                phi_104_ = type_7();
                phi_107_ = vec3<f32>();
                phi_109_ = f32();
                break;
            }
            case 1: {
                let _e93 = ((_e77.member_1 << bitcast<u32>(16u)) | (_e77.member_1 >> bitcast<u32>(16u)));
                let _e100 = (((_e93 & 1431655765u) << bitcast<u32>(1u)) | ((_e93 & 2863311530u) >> bitcast<u32>(1u)));
                let _e107 = (((_e100 & 858993459u) << bitcast<u32>(2u)) | ((_e100 & 3435973836u) >> bitcast<u32>(2u)));
                let _e114 = (((_e107 & 252645135u) << bitcast<u32>(4u)) | ((_e107 & 4042322160u) >> bitcast<u32>(4u)));
                let _e122 = f32((((_e114 & 16711935u) << bitcast<u32>(8u)) | ((_e114 & 4278255360u) >> bitcast<u32>(8u))));
                let _e124 = (_e42 * _e42);
                let _e125 = (f32(_e77.member_1) * 0.0061359233f);
                let _e131 = sqrt((fma(-(_e122), 0.00000000023283064f, 1f) / fma(fma(_e124, _e124, -1f), (_e122 * 0.00000000023283064f), 1f)));
                let _e134 = sqrt(fma(-(_e131), _e131, 1f));
                let _e136 = (cos(_e125) * _e134);
                let _e138 = (sin(_e125) * _e134);
                let _e143 = select(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 0f, 1f), vec3((abs(_e54.z) < 0.999f)));
                let _e148 = fma(_e143.y, _e54.z, -((_e56 * _e143.z)));
                let _e153 = fma(_e143.z, _e54.x, -((_e54.z * _e143.x)));
                let _e156 = fma(_e143.x, _e56, -((_e54.x * _e143.y)));
                let _e161 = sqrt(fma(_e156, _e156, fma(_e148, _e148, (_e153 * _e153))));
                if (_e161 == 0f) {
                    phi_627_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_627_ = (vec3<f32>(_e148, _e153, _e156) * (1f / _e161));
                }
                let _e166 = phi_627_;
                let _e185 = fma(_e54.x, _e131, fma(_e166.x, _e136, (fma(_e56, _e166.z, -((_e166.y * _e54.z))) * _e138)));
                let _e186 = fma(_e56, _e131, fma(_e166.y, _e136, (fma(_e54.z, _e166.x, -((_e166.z * _e54.x))) * _e138)));
                let _e187 = fma(_e54.z, _e131, fma(_e166.z, _e136, (fma(_e54.x, _e166.y, -((_e166.x * _e56))) * _e138)));
                let _e192 = sqrt(fma(_e187, _e187, fma(_e185, _e185, (_e186 * _e186))));
                if (_e192 == 0f) {
                    phi_662_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_662_ = (vec3<f32>(_e185, _e186, _e187) * (1f / _e192));
                }
                let _e197 = phi_662_;
                let _e204 = (2f * fma(_e54.z, _e197.z, fma(_e54.x, _e197.x, (_e56 * _e197.y))));
                let _e206 = fma(_e204, _e197.x, -(_e54.x));
                let _e207 = fma(_e204, _e197.y, _e54.y);
                let _e209 = fma(_e204, _e197.z, -(_e54.z));
                let _e214 = sqrt(fma(_e209, _e209, fma(_e206, _e206, (_e207 * _e207))));
                if (_e214 == 0f) {
                    phi_697_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_697_ = (vec3<f32>(_e206, _e207, _e209) * (1f / _e214));
                }
                let _e219 = phi_697_;
                let _e226 = max(fma(_e54.z, _e219.z, fma(_e54.x, _e219.x, (_e56 * _e219.y))), 0f);
                if (_e226 > 0f) {
                    if (_e42 == 0f) {
                        phi_187_ = 0f;
                    } else {
                        phi_187_ = (0.5f * log2((1572864f / max((1024f * max((_e226 * 0.31830987f), 0f)), 0.00000011920929f))));
                    }
                    let _e237 = phi_187_;
                    let _e238 = textureSampleLevel(global_4, global, _e219, _e237);
                    phi_206_ = vec3<f32>(fma(_e238.x, _e226, _e60.x), fma(_e238.y, _e226, _e60.y), fma(_e238.z, _e226, _e60.z));
                    phi_207_ = (_e62 + _e226);
                } else {
                    phi_206_ = _e60;
                    phi_207_ = _e62;
                }
                let _e251 = phi_206_;
                let _e253 = phi_207_;
                phi_208_ = true;
                phi_104_ = _e75;
                phi_107_ = _e251;
                phi_109_ = _e253;
                break;
            }
            default: {
                phi_208_ = false;
                phi_104_ = type_7();
                phi_107_ = vec3<f32>();
                phi_109_ = f32();
                break;
            }
        }
        let _e255 = phi_208_;
        let _e257 = phi_104_;
        let _e259 = phi_107_;
        let _e261 = phi_109_;
        continue;
        continuing {
            phi_103_ = _e257;
            phi_106_ = _e259;
            phi_108_ = _e261;
            break if !(_e255);
        }
    }
    return;
}

@fragment 
fn convolutionprefilter_environment_cubemap_fragment(@location(0) param: vec3<f32>, @location(1) param_1: f32) -> @location(0) vec4<f32> {
    global_2 = param;
    global_3 = param_1;
    function();
    let _e5 = global_1;
    return _e5;
}
