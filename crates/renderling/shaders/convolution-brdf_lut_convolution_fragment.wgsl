struct type_6 {
    member: u32,
    member_1: u32,
}

var<private> global: vec2<f32>;
var<private> global_1: vec2<f32>;

fn function() {
    var phi_415_: type_6;
    var phi_416_: f32;
    var phi_417_: f32;
    var phi_430_: type_6;
    var phi_431_: type_6;
    var phi_680_: vec3<f32>;
    var phi_715_: vec3<f32>;
    var phi_750_: vec3<f32>;
    var phi_792_: f32;
    var phi_805_: f32;
    var phi_488_: f32;
    var phi_489_: f32;
    var phi_491_: f32;
    var phi_492_: f32;
    var phi_493_: bool;
    var local: f32;
    var local_1: f32;

    let _e37 = global;
    let _e40 = max(_e37.x, 0.00000011920929f);
    let _e41 = -(_e40);
    let _e43 = sqrt(fma(_e41, _e40, 1f));
    phi_415_ = type_6(1u, 1024u);
    phi_416_ = 0f;
    phi_417_ = 0f;
    loop {
        let _e45 = phi_415_;
        let _e47 = phi_416_;
        let _e49 = phi_417_;
        local = _e49;
        local_1 = _e47;
        if (_e45.member < _e45.member_1) {
            phi_430_ = type_6((_e45.member + 1u), _e45.member_1);
            phi_431_ = type_6(1u, _e45.member);
        } else {
            phi_430_ = _e45;
            phi_431_ = type_6(0u, type_6().member_1);
        }
        let _e62 = phi_430_;
        let _e64 = phi_431_;
        switch bitcast<i32>(_e64.member) {
            case 0: {
                phi_491_ = f32();
                phi_492_ = f32();
                phi_493_ = false;
                break;
            }
            case 1: {
                let _e73 = ((_e64.member_1 << bitcast<u32>(16u)) | (_e64.member_1 >> bitcast<u32>(16u)));
                let _e80 = (((_e73 & 1431655765u) << bitcast<u32>(1u)) | ((_e73 & 2863311530u) >> bitcast<u32>(1u)));
                let _e87 = (((_e80 & 858993459u) << bitcast<u32>(2u)) | ((_e80 & 3435973836u) >> bitcast<u32>(2u)));
                let _e94 = (((_e87 & 252645135u) << bitcast<u32>(4u)) | ((_e87 & 4042322160u) >> bitcast<u32>(4u)));
                let _e102 = f32((((_e94 & 16711935u) << bitcast<u32>(8u)) | ((_e94 & 4278255360u) >> bitcast<u32>(8u))));
                let _e104 = (_e37.y * _e37.y);
                let _e105 = (f32(_e64.member_1) * 0.0061359233f);
                let _e111 = sqrt((fma(-(_e102), 0.00000000023283064f, 1f) / fma(fma(_e104, _e104, -1f), (_e102 * 0.00000000023283064f), 1f)));
                let _e114 = sqrt(fma(-(_e111), _e111, 1f));
                let _e116 = (cos(_e105) * _e114);
                let _e118 = (sin(_e105) * _e114);
                let _e122 = select(vec3<f32>(1f, 0f, 0f), vec3<f32>(0f, 0f, 1f), vec3((abs(1f) < 0.999f)));
                let _e125 = -(_e122.x);
                let _e129 = sqrt(fma(_e122.y, _e122.y, (_e125 * _e125)));
                if (_e129 == 0f) {
                    phi_680_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_680_ = (vec3<f32>(_e122.y, _e125, 0f) * (1f / _e129));
                }
                let _e134 = phi_680_;
                let _e141 = fma(_e134.x, _e116, (-(_e134.y) * _e118));
                let _e142 = fma(_e134.y, _e116, (_e134.x * _e118));
                let _e143 = fma(_e134.z, _e116, _e111);
                let _e148 = sqrt(fma(_e143, _e143, fma(_e141, _e141, (_e142 * _e142))));
                if (_e148 == 0f) {
                    phi_715_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_715_ = (vec3<f32>(_e141, _e142, _e143) * (1f / _e148));
                }
                let _e153 = phi_715_;
                let _e158 = fma(_e43, _e153.x, (_e40 * _e153.z));
                let _e159 = (2f * _e158);
                let _e160 = (_e159 * _e153.y);
                let _e162 = fma(_e159, _e153.x, -(_e43));
                let _e163 = fma(_e159, _e153.z, _e41);
                let _e168 = sqrt(fma(_e163, _e163, fma(_e162, _e162, (_e160 * _e160))));
                if (_e168 == 0f) {
                    phi_750_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_750_ = (vec3<f32>(_e162, _e160, _e163) * (1f / _e168));
                }
                let _e173 = phi_750_;
                let _e175 = max(_e173.z, 0f);
                let _e177 = max(_e158, 0f);
                if (_e175 > 0f) {
                    let _e179 = max(_e40, 0f);
                    let _e180 = (_e104 * 0.5f);
                    let _e182 = fma(-(_e104), 0.5f, 1f);
                    let _e183 = fma(_e179, _e182, _e180);
                    if (_e183 == 0f) {
                        phi_792_ = 0f;
                    } else {
                        phi_792_ = (_e179 / _e183);
                    }
                    let _e187 = phi_792_;
                    let _e188 = fma(_e175, _e182, _e180);
                    if (_e188 == 0f) {
                        phi_805_ = 0f;
                    } else {
                        phi_805_ = (_e175 / _e188);
                    }
                    let _e192 = phi_805_;
                    let _e196 = (((_e187 * _e192) * _e177) / (max(_e153.z, 0f) * _e40));
                    let _e198 = pow((1f - _e177), 5f);
                    phi_488_ = fma(_e198, _e196, _e47);
                    phi_489_ = fma((1f - _e198), _e196, _e49);
                } else {
                    phi_488_ = _e47;
                    phi_489_ = _e49;
                }
                let _e203 = phi_488_;
                let _e205 = phi_489_;
                phi_491_ = _e203;
                phi_492_ = _e205;
                phi_493_ = true;
                break;
            }
            default: {
                phi_491_ = f32();
                phi_492_ = f32();
                phi_493_ = bool();
                break;
            }
        }
        let _e207 = phi_491_;
        let _e209 = phi_492_;
        let _e211 = phi_493_;
        continue;
        continuing {
            phi_415_ = _e62;
            phi_416_ = _e207;
            phi_417_ = _e209;
            break if !(_e211);
        }
    }
    let _e214 = local;
    let _e217 = local_1;
    global_1 = vec2<f32>((_e214 * 0.0009765625f), (_e217 * 0.0009765625f));
    return;
}

@fragment 
fn convolutionbrdf_lut_convolution_fragment(@location(0) param: vec2<f32>) -> @location(0) vec2<f32> {
    global = param;
    function();
    let _e3 = global_1;
    return _e3;
}
