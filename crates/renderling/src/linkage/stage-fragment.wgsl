struct type_3 {
    member: array<u32>,
}

struct type_12 {
    member: u32,
    member_1: u32,
}

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: vec4<f32>;
var<private> global_2: u32;
var<private> global_3: vec3<f32>;

fn function() {
    var phi_104_: type_12;
    var phi_105_: type_12;

    let _e9 = arrayLength((&global.member));
    let _e10 = global_2;
    let _e11 = global_3;
    if (_e10 < _e9) {
        let _e15 = global.member[_e10];
        let _e16 = (_e10 + 1u);
        switch bitcast<i32>(_e15) {
            case 0: {
                phi_105_ = type_12(0u, type_12().member_1);
                break;
            }
            case 1: {
                if (_e16 < _e9) {
                    let _e26 = global.member[_e16];
                    phi_104_ = type_12(1u, _e26);
                } else {
                    phi_104_ = type_12();
                }
                let _e29 = phi_104_;
                phi_105_ = _e29;
                break;
            }
            default: {
                phi_105_ = type_12(0u, type_12().member_1);
                break;
            }
        }
        let _e31 = phi_105_;
        switch bitcast<i32>(_e31.member) {
            case 0: {
                break;
            }
            case 1: {
                global_1 = vec4<f32>(_e11.x, _e11.y, _e11.z, 1f);
                break;
            }
            default: {
                break;
            }
        }
    }
    return;
}

@fragment 
fn stagefragment(@location(0) @interpolate(flat) param: u32, @location(9) param_1: vec3<f32>) -> @location(0) vec4<f32> {
    global_2 = param;
    global_3 = param_1;
    function();
    let _e5 = global_1;
    return _e5;
}
