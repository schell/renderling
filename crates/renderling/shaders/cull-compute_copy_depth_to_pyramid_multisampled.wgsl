struct type_5 {
    member: array<u32>,
}

struct type_8 {
    member: u32,
    member_1: u32,
}

var<private> global: vec3<u32>;
@group(0) @binding(0) 
var<storage, read_write> global_1: type_5;
@group(0) @binding(1) 
var global_2: texture_depth_multisampled_2d;

fn function() {
    var phi_285_: bool;
    var phi_312_: bool;
    var phi_233_: type_8;
    var phi_338_: u32;

    let _e15 = arrayLength((&global_1.member));
    let _e16 = global;
    let _e19 = global_1.member[0u];
    let _e22 = global_1.member[1u];
    let _e25 = global_1.member[2u];
    let _e28 = global_1.member[3u];
    let _e31 = global_1.member[4u];
    let _e32 = (_e25 & 31u);
    if (_e16.x < (_e19 >> bitcast<u32>(_e32))) {
        phi_285_ = (_e16.y < (_e22 >> bitcast<u32>(_e32)));
    } else {
        phi_285_ = false;
    }
    let _e42 = phi_285_;
    if (_e42 != true) {
    } else {
        let _e46 = textureLoad(global_2, vec2<u32>(_e16.x, _e16.y), 0i);
        let _e50 = select(_e28, 4294967295u, (0u >= _e31));
        if (_e15 >= 2u) {
            phi_312_ = (_e50 <= (_e15 - 2u));
        } else {
            phi_312_ = false;
        }
        let _e55 = phi_312_;
        if _e55 {
            let _e58 = global_1.member[_e50];
            let _e62 = global_1.member[(_e50 + 1u)];
            phi_233_ = type_8(_e58, _e62);
        } else {
            phi_233_ = type_8(4294967295u, 0u);
        }
        let _e65 = phi_233_;
        let _e71 = ((_e16.y * (_e19 >> bitcast<u32>(0u))) + _e16.x);
        if (_e71 >= _e65.member_1) {
            phi_338_ = 4294967295u;
        } else {
            phi_338_ = (_e65.member + _e71);
        }
        let _e75 = phi_338_;
        global_1.member[_e75] = bitcast<u32>(vec4(_e46).x);
    }
    return;
}

@compute @workgroup_size(32, 32, 1) 
fn cullcompute_copy_depth_to_pyramid_multisampled(@builtin(global_invocation_id) param: vec3<u32>) {
    global = param;
    function();
}
