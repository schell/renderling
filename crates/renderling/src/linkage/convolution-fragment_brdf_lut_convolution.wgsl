struct type_4 {
    member: u32,
    member_1: u32,
}

var<private> global: vec2<f32>;
var<private> global_1: vec2<f32>;

fn function() {
    var phi_418_: type_4;
    var phi_419_: f32;
    var phi_420_: f32;
    var phi_434_: type_4;
    var phi_435_: type_4;
    var phi_683_: vec3<f32>;
    var phi_718_: vec3<f32>;
    var phi_753_: vec3<f32>;
    var phi_795_: f32;
    var phi_808_: f32;
    var phi_495_: f32;
    var phi_496_: f32;
    var phi_498_: bool;
    var phi_499_: type_4;
    var phi_500_: f32;
    var phi_501_: f32;
    var phi_502_: vec2<f32>;
    var local: vec2<f32>;

    let _e37 = global;
    let _e40 = max(_e37.x, 0.00000011920929f);
    let _e41 = -(_e40);
    let _e43 = sqrt(fma(_e41, _e40, 1f));
    phi_418_ = type_4(1u, 1024u);
    phi_419_ = 0f;
    phi_420_ = 0f;
    loop {
        let _e45 = phi_418_;
        let _e47 = phi_419_;
        let _e49 = phi_420_;
        if (_e45.member < _e45.member_1) {
            phi_434_ = type_4((_e45.member + 1u), _e45.member_1);
            phi_435_ = type_4(1u, _e45.member);
        } else {
            phi_434_ = _e45;
            phi_435_ = type_4(0u, type_4().member_1);
        }
        let _e62 = phi_434_;
        let _e64 = phi_435_;
        switch bitcast<i32>(_e64.member) {
            case 0: {
                phi_498_ = false;
                phi_499_ = type_4();
                phi_500_ = f32();
                phi_501_ = f32();
                phi_502_ = vec2<f32>((_e49 * 0.0009765625f), (_e47 * 0.0009765625f));
                break;
            }
            case 1: {
                let _e76 = ((_e64.member_1 << bitcast<u32>(16u)) | (_e64.member_1 >> bitcast<u32>(16u)));
                let _e83 = (((_e76 & 1431655765u) << bitcast<u32>(1u)) | ((_e76 & 2863311530u) >> bitcast<u32>(1u)));
                let _e90 = (((_e83 & 858993459u) << bitcast<u32>(2u)) | ((_e83 & 3435973836u) >> bitcast<u32>(2u)));
                let _e97 = (((_e90 & 252645135u) << bitcast<u32>(4u)) | ((_e90 & 4042322160u) >> bitcast<u32>(4u)));
                let _e105 = f32((((_e97 & 16711935u) << bitcast<u32>(8u)) | ((_e97 & 4278255360u) >> bitcast<u32>(8u))));
                let _e107 = (_e37.y * _e37.y);
                let _e108 = (f32(_e64.member_1) * 0.0061359233f);
                let _e114 = sqrt((fma(-(_e105), 0.00000000023283064f, 1f) / fma(fma(_e107, _e107, -1f), (_e105 * 0.00000000023283064f), 1f)));
                let _e117 = sqrt(fma(-(_e114), _e114, 1f));
                let _e119 = (cos(_e108) * _e117);
                let _e121 = (sin(_e108) * _e117);
                let _e125 = select(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 0f, 1f), vec3((abs(1f) < 0.999f)));
                let _e128 = -(_e125.x);
                let _e132 = sqrt(fma(_e125.y, _e125.y, (_e128 * _e128)));
                if (_e132 == 0f) {
                    phi_683_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_683_ = (vec3<f32>(_e125.y, _e128, 0f) * (1f / _e132));
                }
                let _e137 = phi_683_;
                let _e144 = fma(_e137.x, _e119, (-(_e137.y) * _e121));
                let _e145 = fma(_e137.y, _e119, (_e137.x * _e121));
                let _e146 = fma(_e137.z, _e119, _e114);
                let _e151 = sqrt(fma(_e146, _e146, fma(_e144, _e144, (_e145 * _e145))));
                if (_e151 == 0f) {
                    phi_718_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_718_ = (vec3<f32>(_e144, _e145, _e146) * (1f / _e151));
                }
                let _e156 = phi_718_;
                let _e161 = fma(_e43, _e156.x, (_e40 * _e156.z));
                let _e162 = (2f * _e161);
                let _e163 = (_e162 * _e156.y);
                let _e165 = fma(_e162, _e156.x, -(_e43));
                let _e166 = fma(_e162, _e156.z, _e41);
                let _e171 = sqrt(fma(_e166, _e166, fma(_e165, _e165, (_e163 * _e163))));
                if (_e171 == 0f) {
                    phi_753_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_753_ = (vec3<f32>(_e165, _e163, _e166) * (1f / _e171));
                }
                let _e176 = phi_753_;
                let _e178 = max(_e176.z, 0f);
                let _e180 = max(_e161, 0f);
                if (_e178 > 0f) {
                    let _e182 = max(_e40, 0f);
                    let _e183 = (_e107 * 0.5f);
                    let _e185 = fma(-(_e107), 0.5f, 1f);
                    let _e186 = fma(_e182, _e185, _e183);
                    if (_e186 == 0f) {
                        phi_795_ = 0f;
                    } else {
                        phi_795_ = (_e182 / _e186);
                    }
                    let _e190 = phi_795_;
                    let _e191 = fma(_e178, _e185, _e183);
                    if (_e191 == 0f) {
                        phi_808_ = 0f;
                    } else {
                        phi_808_ = (_e178 / _e191);
                    }
                    let _e195 = phi_808_;
                    let _e199 = (((_e190 * _e195) * _e180) / (max(_e156.z, 0f) * _e40));
                    let _e201 = pow((1f - _e180), 5f);
                    phi_495_ = fma(_e201, _e199, _e47);
                    phi_496_ = fma((1f - _e201), _e199, _e49);
                } else {
                    phi_495_ = _e47;
                    phi_496_ = _e49;
                }
                let _e206 = phi_495_;
                let _e208 = phi_496_;
                phi_498_ = true;
                phi_499_ = _e62;
                phi_500_ = _e206;
                phi_501_ = _e208;
                phi_502_ = vec2<f32>();
                break;
            }
            default: {
                phi_498_ = false;
                phi_499_ = type_4();
                phi_500_ = f32();
                phi_501_ = f32();
                phi_502_ = vec2<f32>();
                break;
            }
        }
        let _e210 = phi_498_;
        let _e212 = phi_499_;
        let _e214 = phi_500_;
        let _e216 = phi_501_;
        let _e218 = phi_502_;
        local = _e218;
        continue;
        continuing {
            phi_418_ = _e212;
            phi_419_ = _e214;
            phi_420_ = _e216;
            break if !(_e210);
        }
    }
    let _e221 = local;
    global_1 = _e221;
    return;
}

@fragment 
fn convolutionfragment_brdf_lut_convolution(@location(0) param: vec2<f32>) -> @location(0) vec2<f32> {
    global = param;
    function();
    let _e3 = global_1;
    return _e3;
}
