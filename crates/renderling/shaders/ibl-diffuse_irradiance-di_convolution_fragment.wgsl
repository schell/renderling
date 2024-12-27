var<private> global: vec4<f32>;
var<private> global_1: vec3<f32>;
@group(0) @binding(2) 
var global_2: sampler;
@group(0) @binding(1) 
var global_3: texture_cube<f32>;

fn function() {
    var phi_273_: vec3<f32>;
    var phi_308_: vec3<f32>;
    var phi_343_: vec3<f32>;
    var phi_138_: vec3<f32>;
    var phi_141_: f32;
    var phi_143_: f32;
    var phi_153_: vec3<f32>;
    var phi_156_: f32;
    var phi_158_: f32;
    var phi_378_: vec3<f32>;
    var phi_154_: vec3<f32>;
    var phi_157_: f32;
    var phi_159_: f32;
    var phi_139_: vec3<f32>;
    var phi_142_: f32;
    var phi_144_: f32;
    var local: f32;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: f32;

    let _e14 = global_1;
    let _e21 = sqrt(fma(_e14.z, _e14.z, fma(_e14.x, _e14.x, (_e14.y * _e14.y))));
    if (_e21 == 0f) {
        phi_273_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_273_ = (_e14 * (1f / _e21));
    }
    let _e26 = phi_273_;
    let _e28 = (_e26.y * -1f);
    let _e31 = -(_e26.x);
    let _e35 = sqrt(fma(_e26.z, _e26.z, (_e31 * _e31)));
    if (_e35 == 0f) {
        phi_308_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_308_ = (vec3<f32>(_e26.z, 0f, _e31) * (1f / _e35));
    }
    let _e40 = phi_308_;
    let _e45 = fma(_e28, _e40.z, -((_e40.y * _e26.z)));
    let _e49 = fma(_e26.z, _e40.x, -((_e40.z * _e26.x)));
    let _e52 = fma(_e26.x, _e40.y, -((_e40.x * _e28)));
    let _e57 = sqrt(fma(_e52, _e52, fma(_e45, _e45, (_e49 * _e49))));
    if (_e57 == 0f) {
        phi_343_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_343_ = (vec3<f32>(_e45, _e49, _e52) * (1f / _e57));
    }
    let _e62 = phi_343_;
    phi_138_ = vec3<f32>(0f, 0f, 0f);
    phi_141_ = 0f;
    phi_143_ = 0f;
    loop {
        let _e64 = phi_138_;
        let _e66 = phi_141_;
        let _e68 = phi_143_;
        local = _e68;
        local_1 = _e64;
        local_2 = _e64;
        local_3 = _e64;
        let _e69 = (_e66 < 6.2831855f);
        if _e69 {
            phi_153_ = _e64;
            phi_156_ = 0f;
            phi_158_ = _e68;
            loop {
                let _e71 = phi_153_;
                let _e73 = phi_156_;
                let _e75 = phi_158_;
                local_4 = _e71;
                local_5 = _e75;
                let _e76 = (_e73 < 1.5707964f);
                if _e76 {
                    let _e77 = sin(_e73);
                    let _e79 = (_e77 * cos(_e66));
                    let _e81 = (_e77 * sin(_e66));
                    let _e82 = cos(_e73);
                    let _e92 = fma(_e82, _e26.x, fma(_e79, _e40.x, (_e81 * _e62.x)));
                    let _e93 = fma(_e82, _e28, fma(_e79, _e40.y, (_e81 * _e62.y)));
                    let _e94 = fma(_e82, _e26.z, fma(_e79, _e40.z, (_e81 * _e62.z)));
                    let _e99 = sqrt(fma(_e94, _e94, fma(_e92, _e92, (_e93 * _e93))));
                    if (_e99 == 0f) {
                        phi_378_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_378_ = (vec3<f32>(_e92, _e93, _e94) * (1f / _e99));
                    }
                    let _e104 = phi_378_;
                    let _e105 = textureSample(global_3, global_2, _e104);
                    phi_154_ = vec3<f32>(fma((_e105.x * _e82), _e77, _e71.x), fma((_e105.y * _e82), _e77, _e71.y), fma((_e105.z * _e82), _e77, _e71.z));
                    phi_157_ = (_e73 + 0.025f);
                    phi_159_ = (_e75 + 1f);
                } else {
                    phi_154_ = vec3<f32>();
                    phi_157_ = f32();
                    phi_159_ = f32();
                }
                let _e122 = phi_154_;
                let _e124 = phi_157_;
                let _e126 = phi_159_;
                continue;
                continuing {
                    phi_153_ = _e122;
                    phi_156_ = _e124;
                    phi_158_ = _e126;
                    break if !(_e76);
                }
            }
            let _e167 = local_4;
            phi_139_ = _e167;
            phi_142_ = (_e66 + 0.025f);
            let _e171 = local_5;
            phi_144_ = _e171;
        } else {
            phi_139_ = vec3<f32>();
            phi_142_ = f32();
            phi_144_ = f32();
        }
        let _e130 = phi_139_;
        let _e132 = phi_142_;
        let _e134 = phi_144_;
        continue;
        continuing {
            phi_138_ = _e130;
            phi_141_ = _e132;
            phi_143_ = _e134;
            break if !(_e69);
        }
    }
    let _e137 = local;
    let _e138 = (3.1415927f / _e137);
    let _e140 = local_1;
    let _e144 = local_2;
    let _e148 = local_3;
    global = vec4<f32>((_e140.x * _e138), (_e144.y * _e138), (_e148.z * _e138), 1f);
    return;
}

@fragment 
fn ibldiffuse_irradiancedi_convolution_fragment(@location(0) param: vec3<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    function();
    let _e3 = global;
    return _e3;
}
