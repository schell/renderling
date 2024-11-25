struct type_8 {
    member: array<u32>,
}

struct type_13 {
    member: u32,
    member_1: u32,
}

struct type_18 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
    member_3: vec4<f32>,
    member_4: array<u32, 4>,
    member_5: array<f32, 4>,
    member_6: vec2<f32>,
    member_7: vec2<f32>,
}

struct VertexOutput {
    @location(0) member: vec4<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
@group(0) @binding(0) 
var<storage> global_1: type_8;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: vec4<f32>;

fn function() {
    var local: array<f32, 4>;
    var local_1: array<u32, 4>;
    var phi_312_: bool;
    var phi_180_: type_13;
    var phi_181_: type_13;
    var phi_204_: type_13;
    var phi_217_: bool;
    var phi_223_: type_13;
    var phi_224_: type_13;
    var phi_247_: type_13;
    var phi_261_: bool;
    var phi_265_: type_18;

    let _e42 = global;
    let _e44 = arrayLength((&global_1.member));
    let _e45 = (_e42 * 26u);
    if (_e44 >= 26u) {
        phi_312_ = (_e45 <= (_e44 - 26u));
    } else {
        phi_312_ = false;
    }
    let _e50 = phi_312_;
    if _e50 {
        let _e53 = global_1.member[_e45];
        let _e58 = global_1.member[(_e45 + 1u)];
        let _e63 = global_1.member[(_e45 + 2u)];
        let _e69 = global_1.member[(_e45 + 3u)];
        let _e74 = global_1.member[(_e45 + 4u)];
        let _e79 = global_1.member[(_e45 + 5u)];
        let _e84 = global_1.member[(_e45 + 6u)];
        let _e90 = global_1.member[(_e45 + 7u)];
        let _e95 = global_1.member[(_e45 + 8u)];
        let _e101 = global_1.member[(_e45 + 9u)];
        let _e106 = global_1.member[(_e45 + 10u)];
        let _e112 = global_1.member[(_e45 + 11u)];
        let _e117 = global_1.member[(_e45 + 12u)];
        let _e122 = global_1.member[(_e45 + 13u)];
        let _e128 = global_1.member[(_e45 + 14u)];
        let _e133 = global_1.member[(_e45 + 15u)];
        let _e138 = global_1.member[(_e45 + 16u)];
        let _e143 = global_1.member[(_e45 + 17u)];
        local_1 = array<u32, 4>(0u, 0u, 0u, 0u);
        phi_180_ = type_13(0u, 4u);
        loop {
            let _e148 = phi_180_;
            if (_e148.member < _e148.member_1) {
                phi_181_ = type_13((_e148.member + 1u), _e148.member_1);
                phi_204_ = type_13(1u, _e148.member);
            } else {
                phi_181_ = _e148;
                phi_204_ = type_13(0u, type_13().member_1);
            }
            let _e161 = phi_181_;
            let _e163 = phi_204_;
            switch bitcast<i32>(_e163.member) {
                case 0: {
                    phi_217_ = false;
                    break;
                }
                case 1: {
                    let _e170 = global_1.member[((_e45 + 18u) + _e163.member_1)];
                    local_1[_e163.member_1] = _e170;
                    phi_217_ = true;
                    break;
                }
                default: {
                    phi_217_ = bool();
                    break;
                }
            }
            let _e173 = phi_217_;
            continue;
            continuing {
                phi_180_ = _e161;
                break if !(_e173);
            }
        }
        let _e175 = local_1;
        local = array<f32, 4>(0f, 0f, 0f, 0f);
        phi_223_ = type_13(0u, 4u);
        loop {
            let _e178 = phi_223_;
            if (_e178.member < _e178.member_1) {
                phi_224_ = type_13((_e178.member + 1u), _e178.member_1);
                phi_247_ = type_13(1u, _e178.member);
            } else {
                phi_224_ = _e178;
                phi_247_ = type_13(0u, type_13().member_1);
            }
            let _e191 = phi_224_;
            let _e193 = phi_247_;
            switch bitcast<i32>(_e193.member) {
                case 0: {
                    phi_261_ = false;
                    break;
                }
                case 1: {
                    let _e200 = global_1.member[((_e45 + 22u) + _e193.member_1)];
                    local[_e193.member_1] = bitcast<f32>(_e200);
                    phi_261_ = true;
                    break;
                }
                default: {
                    phi_261_ = bool();
                    break;
                }
            }
            let _e204 = phi_261_;
            continue;
            continuing {
                phi_223_ = _e191;
                break if !(_e204);
            }
        }
        let _e206 = local;
        phi_265_ = type_18(vec3<f32>(bitcast<f32>(_e53), bitcast<f32>(_e58), bitcast<f32>(_e63)), vec4<f32>(bitcast<f32>(_e69), bitcast<f32>(_e74), bitcast<f32>(_e79), bitcast<f32>(_e84)), vec3<f32>(bitcast<f32>(_e112), bitcast<f32>(_e117), bitcast<f32>(_e122)), vec4<f32>(bitcast<f32>(_e128), bitcast<f32>(_e133), bitcast<f32>(_e138), bitcast<f32>(_e143)), _e175, _e206, vec2<f32>(bitcast<f32>(_e90), bitcast<f32>(_e95)), vec2<f32>(bitcast<f32>(_e101), bitcast<f32>(_e106)));
    } else {
        phi_265_ = type_18(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), vec3<f32>(0f, 0f, 1f), vec4<f32>(0f, 1f, 0f, 0f), array<u32, 4>(0u, 0u, 0u, 0u), array<f32, 4>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
    }
    let _e209 = phi_265_;
    global_2 = vec4<f32>(_e209.member.x, _e209.member.y, _e209.member.z, 1f);
    global_3 = _e209.member_1;
    return;
}

@vertex 
fn tutorialtutorial_slabbed_vertices_no_instance(@builtin(vertex_index) param: u32) -> VertexOutput {
    global = param;
    function();
    let _e5 = global_2.y;
    global_2.y = -(_e5);
    let _e7 = global_3;
    let _e8 = global_2;
    return VertexOutput(_e7, _e8);
}
