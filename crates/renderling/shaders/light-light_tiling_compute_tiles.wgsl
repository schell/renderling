struct type_2 {
    member: array<u32>,
}

struct type_16 {
    member: array<atomic<u32>>,
}

var<private> global: vec3<u32>;
@group(0) @binding(0) 
var<storage> global_1: type_2;
@group(0) @binding(1) 
var<storage> global_2: type_2;
@group(0) @binding(2) 
var<storage, read_write> global_3: type_16;
@group(0) @binding(3) 
var global_4: texture_depth_2d;

fn function() {
    var phi_228_: u32;
    var phi_239_: u32;

    switch bitcast<i32>(0u) {
        default: {
            let _e22 = arrayLength((&global_3.member));
            let _e23 = global;
            let _e26 = global_1.member[3u];
            let _e29 = global_1.member[4u];
            if (_e23.x < _e26) {
                if (_e23.y < _e29) {
                    let _e35 = textureLoad(global_4, vec2<u32>(_e23.x, _e23.y), 0i);
                    let _e38 = (4294967300f * vec4(_e35).x);
                    let _e43 = select(select(u32(_e38), 0u, (_e38 < 0f)), 4294967295u, (_e38 > 4294967000f));
                    let _e47 = (((_e23.x / 16u) * 16u) + (_e23.y / 16u));
                    let _e50 = global_2.member[5u];
                    let _e53 = global_2.member[6u];
                    let _e56 = global_2.member[7u];
                    let _e59 = global_2.member[8u];
                    if (_e47 >= _e53) {
                        phi_228_ = 4294967295u;
                    } else {
                        phi_228_ = (_e50 + _e47);
                    }
                    let _e63 = phi_228_;
                    if (_e47 >= _e59) {
                        phi_239_ = 4294967295u;
                    } else {
                        phi_239_ = (_e56 + _e47);
                    }
                    let _e67 = phi_239_;
                    if (_e63 < _e22) {
                    } else {
                        break;
                    }
                    let _e71 = atomicMin((&global_3.member[_e63]), _e43);
                    if (_e67 < _e22) {
                    } else {
                        break;
                    }
                    let _e75 = atomicMax((&global_3.member[_e67]), _e43);
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(16, 16, 1) 
fn lightlight_tiling_compute_tiles(@builtin(global_invocation_id) param: vec3<u32>) {
    global = param;
    function();
}
