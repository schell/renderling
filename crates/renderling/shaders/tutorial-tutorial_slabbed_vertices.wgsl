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
var<private> global_4: u32;

fn function() {
    var local: array<f32, 4>;
    var local_1: array<u32, 4>;
    var phi_365_: bool;
    var phi_148_: type_13;
    var phi_391_: u32;
    var phi_410_: bool;
    var phi_239_: type_13;
    var phi_240_: type_13;
    var phi_255_: type_13;
    var phi_268_: bool;
    var phi_274_: type_13;
    var phi_275_: type_13;
    var phi_290_: type_13;
    var phi_304_: bool;
    var phi_308_: type_18;

    let _e45 = global_4;
    let _e46 = global;
    let _e48 = arrayLength((&global_1.member));
    if (_e48 >= 2u) {
        phi_365_ = (_e45 <= (_e48 - 2u));
    } else {
        phi_365_ = false;
    }
    let _e53 = phi_365_;
    if _e53 {
        let _e56 = global_1.member[_e45];
        let _e60 = global_1.member[(_e45 + 1u)];
        phi_148_ = type_13(_e56, _e60);
    } else {
        phi_148_ = type_13(4294967295u, 0u);
    }
    let _e63 = phi_148_;
    if (_e46 >= _e63.member_1) {
        phi_391_ = 4294967295u;
    } else {
        phi_391_ = (_e63.member + (26u * _e46));
    }
    let _e70 = phi_391_;
    if (_e48 >= 26u) {
        phi_410_ = (_e70 <= (_e48 - 26u));
    } else {
        phi_410_ = false;
    }
    let _e75 = phi_410_;
    if _e75 {
        let _e78 = global_1.member[_e70];
        let _e83 = global_1.member[(_e70 + 1u)];
        let _e88 = global_1.member[(_e70 + 2u)];
        let _e94 = global_1.member[(_e70 + 3u)];
        let _e99 = global_1.member[(_e70 + 4u)];
        let _e104 = global_1.member[(_e70 + 5u)];
        let _e109 = global_1.member[(_e70 + 6u)];
        let _e115 = global_1.member[(_e70 + 7u)];
        let _e120 = global_1.member[(_e70 + 8u)];
        let _e126 = global_1.member[(_e70 + 9u)];
        let _e131 = global_1.member[(_e70 + 10u)];
        let _e137 = global_1.member[(_e70 + 11u)];
        let _e142 = global_1.member[(_e70 + 12u)];
        let _e147 = global_1.member[(_e70 + 13u)];
        let _e153 = global_1.member[(_e70 + 14u)];
        let _e158 = global_1.member[(_e70 + 15u)];
        let _e163 = global_1.member[(_e70 + 16u)];
        let _e168 = global_1.member[(_e70 + 17u)];
        local_1 = array<u32, 4>(0u, 0u, 0u, 0u);
        phi_239_ = type_13(0u, 4u);
        loop {
            let _e173 = phi_239_;
            if (_e173.member < _e173.member_1) {
                phi_240_ = type_13((_e173.member + 1u), _e173.member_1);
                phi_255_ = type_13(1u, _e173.member);
            } else {
                phi_240_ = _e173;
                phi_255_ = type_13(0u, type_13().member_1);
            }
            let _e186 = phi_240_;
            let _e188 = phi_255_;
            switch bitcast<i32>(_e188.member) {
                case 0: {
                    phi_268_ = false;
                    break;
                }
                case 1: {
                    let _e195 = global_1.member[((_e70 + 18u) + _e188.member_1)];
                    local_1[_e188.member_1] = _e195;
                    phi_268_ = true;
                    break;
                }
                default: {
                    phi_268_ = bool();
                    break;
                }
            }
            let _e198 = phi_268_;
            continue;
            continuing {
                phi_239_ = _e186;
                break if !(_e198);
            }
        }
        let _e200 = local_1;
        local = array<f32, 4>(0f, 0f, 0f, 0f);
        phi_274_ = type_13(0u, 4u);
        loop {
            let _e203 = phi_274_;
            if (_e203.member < _e203.member_1) {
                phi_275_ = type_13((_e203.member + 1u), _e203.member_1);
                phi_290_ = type_13(1u, _e203.member);
            } else {
                phi_275_ = _e203;
                phi_290_ = type_13(0u, type_13().member_1);
            }
            let _e216 = phi_275_;
            let _e218 = phi_290_;
            switch bitcast<i32>(_e218.member) {
                case 0: {
                    phi_304_ = false;
                    break;
                }
                case 1: {
                    let _e225 = global_1.member[((_e70 + 22u) + _e218.member_1)];
                    local[_e218.member_1] = bitcast<f32>(_e225);
                    phi_304_ = true;
                    break;
                }
                default: {
                    phi_304_ = bool();
                    break;
                }
            }
            let _e229 = phi_304_;
            continue;
            continuing {
                phi_274_ = _e216;
                break if !(_e229);
            }
        }
        let _e231 = local;
        phi_308_ = type_18(vec3<f32>(bitcast<f32>(_e78), bitcast<f32>(_e83), bitcast<f32>(_e88)), vec4<f32>(bitcast<f32>(_e94), bitcast<f32>(_e99), bitcast<f32>(_e104), bitcast<f32>(_e109)), vec3<f32>(bitcast<f32>(_e137), bitcast<f32>(_e142), bitcast<f32>(_e147)), vec4<f32>(bitcast<f32>(_e153), bitcast<f32>(_e158), bitcast<f32>(_e163), bitcast<f32>(_e168)), _e200, _e231, vec2<f32>(bitcast<f32>(_e115), bitcast<f32>(_e120)), vec2<f32>(bitcast<f32>(_e126), bitcast<f32>(_e131)));
    } else {
        phi_308_ = type_18(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), vec3<f32>(0f, 0f, 1f), vec4<f32>(0f, 1f, 0f, 0f), array<u32, 4>(0u, 0u, 0u, 0u), array<f32, 4>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
    }
    let _e234 = phi_308_;
    global_2 = vec4<f32>(_e234.member.x, _e234.member.y, _e234.member.z, 1f);
    global_3 = _e234.member_1;
    return;
}

@vertex 
fn tutorialtutorial_slabbed_vertices(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_4 = param;
    global = param_1;
    function();
    let _e7 = global_2.y;
    global_2.y = -(_e7);
    let _e9 = global_3;
    let _e10 = global_2;
    return VertexOutput(_e9, _e10);
}
