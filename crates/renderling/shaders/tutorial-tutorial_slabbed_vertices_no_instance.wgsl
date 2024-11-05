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
    var phi_300_: bool;
    var phi_179_: type_13;
    var phi_195_: type_13;
    var phi_196_: type_13;
    var phi_209_: type_13;
    var phi_225_: type_13;
    var phi_226_: type_13;
    var phi_242_: type_18;
    var phi_243_: bool;
    var phi_210_: type_13;
    var phi_249_: type_18;
    var phi_250_: bool;
    var phi_180_: type_13;
    var phi_252_: type_18;
    var local_2: type_18;
    var local_3: type_18;

    let _e42 = global;
    let _e44 = arrayLength((&global_1.member));
    let _e45 = (_e42 * 26u);
    if (_e44 >= 26u) {
        phi_300_ = (_e45 <= (_e44 - 26u));
    } else {
        phi_300_ = false;
    }
    let _e50 = phi_300_;
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
        phi_179_ = type_13(0u, 4u);
        loop {
            let _e148 = phi_179_;
            if (_e148.member < _e148.member_1) {
                phi_195_ = type_13((_e148.member + 1u), _e148.member_1);
                phi_196_ = type_13(1u, _e148.member);
            } else {
                phi_195_ = _e148;
                phi_196_ = type_13(0u, type_13().member_1);
            }
            let _e161 = phi_195_;
            let _e163 = phi_196_;
            switch bitcast<i32>(_e163.member) {
                case 0: {
                    let _e167 = local_1;
                    local = array<f32, 4>(0f, 0f, 0f, 0f);
                    phi_209_ = type_13(0u, 4u);
                    loop {
                        let _e170 = phi_209_;
                        if (_e170.member < _e170.member_1) {
                            phi_225_ = type_13((_e170.member + 1u), _e170.member_1);
                            phi_226_ = type_13(1u, _e170.member);
                        } else {
                            phi_225_ = _e170;
                            phi_226_ = type_13(0u, type_13().member_1);
                        }
                        let _e183 = phi_225_;
                        let _e185 = phi_226_;
                        switch bitcast<i32>(_e185.member) {
                            case 0: {
                                let _e189 = local;
                                phi_242_ = type_18(vec3<f32>(bitcast<f32>(_e53), bitcast<f32>(_e58), bitcast<f32>(_e63)), vec4<f32>(bitcast<f32>(_e69), bitcast<f32>(_e74), bitcast<f32>(_e79), bitcast<f32>(_e84)), vec3<f32>(bitcast<f32>(_e112), bitcast<f32>(_e117), bitcast<f32>(_e122)), vec4<f32>(bitcast<f32>(_e128), bitcast<f32>(_e133), bitcast<f32>(_e138), bitcast<f32>(_e143)), _e167, _e189, vec2<f32>(bitcast<f32>(_e90), bitcast<f32>(_e95)), vec2<f32>(bitcast<f32>(_e101), bitcast<f32>(_e106)));
                                phi_243_ = false;
                                phi_210_ = type_13();
                                break;
                            }
                            case 1: {
                                let _e194 = global_1.member[((_e45 + 22u) + _e185.member_1)];
                                local[_e185.member_1] = bitcast<f32>(_e194);
                                phi_242_ = type_18();
                                phi_243_ = true;
                                phi_210_ = _e183;
                                break;
                            }
                            default: {
                                phi_242_ = type_18();
                                phi_243_ = false;
                                phi_210_ = type_13();
                                break;
                            }
                        }
                        let _e198 = phi_242_;
                        let _e200 = phi_243_;
                        let _e202 = phi_210_;
                        local_2 = _e198;
                        continue;
                        continuing {
                            phi_209_ = _e202;
                            break if !(_e200);
                        }
                    }
                    let _e236 = local_2;
                    phi_249_ = _e236;
                    phi_250_ = false;
                    phi_180_ = type_13();
                    break;
                }
                case 1: {
                    let _e207 = global_1.member[((_e45 + 18u) + _e163.member_1)];
                    local_1[_e163.member_1] = _e207;
                    phi_249_ = type_18();
                    phi_250_ = true;
                    phi_180_ = _e161;
                    break;
                }
                default: {
                    phi_249_ = type_18();
                    phi_250_ = false;
                    phi_180_ = type_13();
                    break;
                }
            }
            let _e210 = phi_249_;
            let _e212 = phi_250_;
            let _e214 = phi_180_;
            local_3 = _e210;
            continue;
            continuing {
                phi_179_ = _e214;
                break if !(_e212);
            }
        }
        let _e241 = local_3;
        phi_252_ = _e241;
    } else {
        phi_252_ = type_18(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), vec3<f32>(0f, 0f, 1f), vec4<f32>(0f, 1f, 0f, 0f), array<u32, 4>(0u, 0u, 0u, 0u), array<f32, 4>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
    }
    let _e217 = phi_252_;
    global_2 = vec4<f32>(_e217.member.x, _e217.member.y, _e217.member.z, 1f);
    global_3 = _e217.member_1;
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
