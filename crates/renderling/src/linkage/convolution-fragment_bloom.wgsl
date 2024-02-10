struct type_7 {
    member: u32,
    member_1: u32,
}

struct type_12 {
    member: u32,
}

struct type_15 {
    member: vec2<u32>,
}

var<private> global: vec4<f32>;
var<private> global_1: vec2<f32>;
@group(0) @binding(0) 
var<uniform> global_2: type_12;
@group(0) @binding(1) 
var<uniform> global_3: type_15;
@group(0) @binding(3) 
var global_4: sampler;
@group(0) @binding(2) 
var global_5: texture_2d<f32>;

fn function() {
    var local: array<f32, 5>;
    var phi_117_: type_7;
    var phi_120_: vec3<f32>;
    var phi_136_: type_7;
    var phi_137_: type_7;
    var phi_200_: type_7;
    var phi_201_: vec3<f32>;
    var phi_202_: type_7;
    var phi_203_: vec3<f32>;
    var phi_204_: bool;
    var phi_118_: type_7;
    var phi_121_: vec3<f32>;

    let _e28 = global_1;
    let _e33 = global_2.member;
    local[0u] = 0.227027f;
    local[1u] = 0.1945946f;
    local[2u] = 0.1216216f;
    local[3u] = 0.054054f;
    local[4u] = 0.016216f;
    let _e40 = global_3.member[0u];
    let _e42 = global_3.member[1u];
    let _e46 = textureSample(global_5, global_4, _e28);
    let _e55 = select(vec2<f32>(0f, 1f), vec2<f32>(1f, 0f), vec2((_e33 != 0u)));
    phi_117_ = type_7(1u, 5u);
    phi_120_ = vec3<f32>((_e46.x * 0.227027f), (_e46.y * 0.227027f), (_e46.z * 0.227027f));
    loop {
        let _e57 = phi_117_;
        let _e59 = phi_120_;
        if (_e57.member < _e57.member_1) {
            phi_136_ = type_7((_e57.member + 1u), _e57.member_1);
            phi_137_ = type_7(1u, _e57.member);
        } else {
            phi_136_ = _e57;
            phi_137_ = type_7(0u, type_7().member_1);
        }
        let _e72 = phi_136_;
        let _e74 = phi_137_;
        switch bitcast<i32>(_e74.member) {
            case 0: {
                global = vec4<f32>(_e59.x, _e59.y, _e59.z, 1f);
                phi_204_ = false;
                phi_118_ = type_7();
                phi_121_ = vec3<f32>();
                break;
            }
            case 1: {
                let _e86 = f32(_e74.member_1);
                let _e89 = vec2<f32>(((_e55.x * (1f / f32(_e40))) * _e86), ((_e55.y * (1f / f32(_e42))) * _e86));
                let _e91 = textureSampleLevel(global_5, global_4, (_e28 + _e89), 0f);
                let _e95 = (_e74.member_1 < 5u);
                if _e95 {
                    let _e97 = local[_e74.member_1];
                    let _e105 = textureSampleLevel(global_5, global_4, (_e28 - _e89), 0f);
                    if _e95 {
                        let _e109 = local[_e74.member_1];
                        phi_200_ = _e72;
                        phi_201_ = vec3<f32>(fma(_e105.x, _e109, fma(_e91.x, _e97, _e59.x)), fma(_e105.y, _e109, fma(_e91.y, _e97, _e59.y)), fma(_e105.z, _e109, fma(_e91.z, _e97, _e59.z)));
                    } else {
                        phi_200_ = type_7();
                        phi_201_ = vec3<f32>();
                    }
                    let _e115 = phi_200_;
                    let _e117 = phi_201_;
                    phi_202_ = _e115;
                    phi_203_ = _e117;
                } else {
                    phi_202_ = type_7();
                    phi_203_ = vec3<f32>();
                }
                let _e119 = phi_202_;
                let _e121 = phi_203_;
                phi_204_ = true;
                phi_118_ = _e119;
                phi_121_ = _e121;
                break;
            }
            default: {
                phi_204_ = false;
                phi_118_ = type_7();
                phi_121_ = vec3<f32>();
                break;
            }
        }
        let _e123 = phi_204_;
        let _e125 = phi_118_;
        let _e127 = phi_121_;
        continue;
        continuing {
            phi_117_ = _e125;
            phi_120_ = _e127;
            break if !(_e123);
        }
    }
    return;
}

@fragment 
fn convolutionfragment_bloom(@location(0) param: vec2<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    function();
    let _e3 = global;
    return _e3;
}
