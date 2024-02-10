struct type_3 {
    member: array<u32>,
}

struct type_12 {
    member: u32,
    member_1: u32,
}

struct VertexOutput {
    @location(0) @interpolate(flat) member: u32,
    @location(1) @interpolate(flat) member_1: u32,
    @location(2) @interpolate(flat) member_2: u32,
    @location(3) member_3: vec4<f32>,
    @location(4) member_4: vec2<f32>,
    @location(5) member_5: vec2<f32>,
    @location(6) member_6: vec3<f32>,
    @location(7) member_7: vec3<f32>,
    @location(8) member_8: vec3<f32>,
    @location(9) member_9: vec3<f32>,
    @location(10) member_10: vec3<f32>,
    @builtin(position) member_11: vec4<f32>,
}

var<private> global: u32;
@group(0) @binding(0) 
var<storage> global_1: type_3;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: u32;
var<private> global_4: vec3<f32>;
var<private> global_5: u32;
var<private> global_6: u32;
var<private> global_7: vec4<f32>;
var<private> global_8: vec2<f32>;
var<private> global_9: vec2<f32>;
var<private> global_10: vec3<f32>;
var<private> global_11: vec3<f32>;
var<private> global_12: vec3<f32>;
var<private> global_13: vec3<f32>;

fn function() {
    var phi_85_: type_12;
    var phi_86_: type_12;

    let _e19 = global;
    let _e21 = arrayLength((&global_1.member));
    global_3 = _e19;
    if (_e19 < _e21) {
        let _e25 = global_1.member[_e19];
        let _e26 = (_e19 + 1u);
        switch bitcast<i32>(_e25) {
            case 0: {
                phi_86_ = type_12(0u, type_12().member_1);
                break;
            }
            case 1: {
                if (_e26 < _e21) {
                    let _e36 = global_1.member[_e26];
                    phi_85_ = type_12(1u, _e36);
                } else {
                    phi_85_ = type_12();
                }
                let _e39 = phi_85_;
                phi_86_ = _e39;
                break;
            }
            default: {
                phi_86_ = type_12(0u, type_12().member_1);
                break;
            }
        }
        let _e41 = phi_86_;
        switch bitcast<i32>(_e41.member) {
            case 0: {
                break;
            }
            case 1: {
                global_4 = vec3<f32>(0f, 0f, 0f);
                break;
            }
            default: {
                break;
            }
        }
    }
    return;
}

@vertex 
fn stagevertex(@builtin(instance_index) param: u32) -> VertexOutput {
    global = param;
    function();
    let _e15 = global_2.y;
    global_2.y = -(_e15);
    let _e17 = global_3;
    let _e18 = global_5;
    let _e19 = global_6;
    let _e20 = global_7;
    let _e21 = global_8;
    let _e22 = global_9;
    let _e23 = global_10;
    let _e24 = global_11;
    let _e25 = global_12;
    let _e26 = global_4;
    let _e27 = global_13;
    let _e28 = global_2;
    return VertexOutput(_e17, _e18, _e19, _e20, _e21, _e22, _e23, _e24, _e25, _e26, _e27, _e28);
}
