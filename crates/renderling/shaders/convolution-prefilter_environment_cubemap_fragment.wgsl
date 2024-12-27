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
    var phi_104_: type_7;
    var phi_107_: vec3<f32>;
    var phi_109_: f32;
    var phi_105_: type_7;
    var phi_124_: type_7;
    var phi_627_: vec3<f32>;
    var phi_662_: vec3<f32>;
    var phi_697_: vec3<f32>;
    var phi_178_: f32;
    var phi_197_: vec3<f32>;
    var phi_198_: f32;
    var phi_108_: vec3<f32>;
    var phi_110_: f32;
    var phi_199_: bool;
    var local: vec3<f32>;
    var local_1: f32;
    var local_2: vec3<f32>;
    var local_3: f32;
    var local_4: vec3<f32>;
    var local_5: f32;

    let _e42 = global_2;
    let _e43 = global_3;
    let _e50 = sqrt(fma(_e42.z, _e42.z, fma(_e42.x, _e42.x, (_e42.y * _e42.y))));
    if (_e50 == 0f) {
        phi_446_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_446_ = (_e42 * (1f / _e50));
    }
    let _e55 = phi_446_;
    let _e57 = (_e55.y * -1f);
    phi_104_ = type_7(0u, 1024u);
    phi_107_ = vec3<f32>(0f, 0f, 0f);
    phi_109_ = 0f;
    loop {
        let _e59 = phi_104_;
        let _e61 = phi_107_;
        let _e63 = phi_109_;
        local = _e61;
        local_1 = _e63;
        local_2 = _e61;
        local_3 = _e63;
        local_4 = _e61;
        local_5 = _e63;
        if (_e59.member < _e59.member_1) {
            phi_105_ = type_7((_e59.member + 1u), _e59.member_1);
            phi_124_ = type_7(1u, _e59.member);
        } else {
            phi_105_ = _e59;
            phi_124_ = type_7(0u, type_7().member_1);
        }
        let _e76 = phi_105_;
        let _e78 = phi_124_;
        switch bitcast<i32>(_e78.member) {
            case 0: {
                phi_108_ = vec3<f32>();
                phi_110_ = f32();
                phi_199_ = false;
                break;
            }
            case 1: {
                let _e87 = ((_e78.member_1 << bitcast<u32>(16u)) | (_e78.member_1 >> bitcast<u32>(16u)));
                let _e94 = (((_e87 & 1431655765u) << bitcast<u32>(1u)) | ((_e87 & 2863311530u) >> bitcast<u32>(1u)));
                let _e101 = (((_e94 & 858993459u) << bitcast<u32>(2u)) | ((_e94 & 3435973836u) >> bitcast<u32>(2u)));
                let _e108 = (((_e101 & 252645135u) << bitcast<u32>(4u)) | ((_e101 & 4042322160u) >> bitcast<u32>(4u)));
                let _e116 = f32((((_e108 & 16711935u) << bitcast<u32>(8u)) | ((_e108 & 4278255360u) >> bitcast<u32>(8u))));
                let _e118 = (_e43 * _e43);
                let _e119 = (f32(_e78.member_1) * 0.0061359233f);
                let _e125 = sqrt((fma(-(_e116), 0.00000000023283064f, 1f) / fma(fma(_e118, _e118, -1f), (_e116 * 0.00000000023283064f), 1f)));
                let _e128 = sqrt(fma(-(_e125), _e125, 1f));
                let _e130 = (cos(_e119) * _e128);
                let _e132 = (sin(_e119) * _e128);
                let _e137 = select(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 0f, 1f), vec3((abs(_e55.z) < 0.999f)));
                let _e142 = fma(_e137.y, _e55.z, -((_e57 * _e137.z)));
                let _e147 = fma(_e137.z, _e55.x, -((_e55.z * _e137.x)));
                let _e150 = fma(_e137.x, _e57, -((_e55.x * _e137.y)));
                let _e155 = sqrt(fma(_e150, _e150, fma(_e142, _e142, (_e147 * _e147))));
                if (_e155 == 0f) {
                    phi_627_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_627_ = (vec3<f32>(_e142, _e147, _e150) * (1f / _e155));
                }
                let _e160 = phi_627_;
                let _e179 = fma(_e55.x, _e125, fma(_e160.x, _e130, (fma(_e57, _e160.z, -((_e160.y * _e55.z))) * _e132)));
                let _e180 = fma(_e57, _e125, fma(_e160.y, _e130, (fma(_e55.z, _e160.x, -((_e160.z * _e55.x))) * _e132)));
                let _e181 = fma(_e55.z, _e125, fma(_e160.z, _e130, (fma(_e55.x, _e160.y, -((_e160.x * _e57))) * _e132)));
                let _e186 = sqrt(fma(_e181, _e181, fma(_e179, _e179, (_e180 * _e180))));
                if (_e186 == 0f) {
                    phi_662_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_662_ = (vec3<f32>(_e179, _e180, _e181) * (1f / _e186));
                }
                let _e191 = phi_662_;
                let _e198 = (2f * fma(_e55.z, _e191.z, fma(_e55.x, _e191.x, (_e57 * _e191.y))));
                let _e200 = fma(_e198, _e191.x, -(_e55.x));
                let _e201 = fma(_e198, _e191.y, _e55.y);
                let _e203 = fma(_e198, _e191.z, -(_e55.z));
                let _e208 = sqrt(fma(_e203, _e203, fma(_e200, _e200, (_e201 * _e201))));
                if (_e208 == 0f) {
                    phi_697_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_697_ = (vec3<f32>(_e200, _e201, _e203) * (1f / _e208));
                }
                let _e213 = phi_697_;
                let _e220 = max(fma(_e55.z, _e213.z, fma(_e55.x, _e213.x, (_e57 * _e213.y))), 0f);
                if (_e220 > 0f) {
                    if (_e43 == 0f) {
                        phi_178_ = 0f;
                    } else {
                        phi_178_ = (0.5f * log2((1572864f / max((1024f * max((_e220 * 0.31830987f), 0f)), 0.00000011920929f))));
                    }
                    let _e231 = phi_178_;
                    let _e232 = textureSampleLevel(global_4, global, _e213, _e231);
                    phi_197_ = vec3<f32>(fma(_e232.x, _e220, _e61.x), fma(_e232.y, _e220, _e61.y), fma(_e232.z, _e220, _e61.z));
                    phi_198_ = (_e63 + _e220);
                } else {
                    phi_197_ = _e61;
                    phi_198_ = _e63;
                }
                let _e245 = phi_197_;
                let _e247 = phi_198_;
                phi_108_ = _e245;
                phi_110_ = _e247;
                phi_199_ = true;
                break;
            }
            default: {
                phi_108_ = vec3<f32>();
                phi_110_ = f32();
                phi_199_ = bool();
                break;
            }
        }
        let _e249 = phi_108_;
        let _e251 = phi_110_;
        let _e253 = phi_199_;
        continue;
        continuing {
            phi_104_ = _e76;
            phi_107_ = _e249;
            phi_109_ = _e251;
            break if !(_e253);
        }
    }
    let _e256 = local;
    let _e259 = local_1;
    let _e262 = local_2;
    let _e265 = local_3;
    let _e268 = local_4;
    let _e271 = local_5;
    global_1 = vec4<f32>((_e256.x / _e259), (_e262.y / _e265), (_e268.z / _e271), 1f);
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
