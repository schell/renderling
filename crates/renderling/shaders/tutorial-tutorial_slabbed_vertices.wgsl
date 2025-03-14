struct type_3 {
    member: array<u32>,
}

struct type_11 {
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

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: u32;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: vec4<f32>;
var<private> global_4: u32;

fn function() {
    var local: array<f32, 4>;
    var local_1: array<u32, 4>;
    var phi_381_: bool;
    var phi_148_: type_11;
    var phi_407_: u32;
    var phi_426_: bool;
    var phi_239_: type_11;
    var phi_240_: type_11;
    var phi_263_: type_11;
    var phi_276_: bool;
    var phi_282_: type_11;
    var phi_283_: type_11;
    var phi_306_: type_11;
    var phi_320_: bool;
    var phi_324_: type_18;

    let _e45 = global_4;
    let _e46 = global_1;
    let _e48 = arrayLength((&global.member));
    if (_e48 >= 2u) {
        phi_381_ = (_e45 <= (_e48 - 2u));
    } else {
        phi_381_ = false;
    }
    let _e53 = phi_381_;
    if _e53 {
        let _e56 = global.member[_e45];
        let _e60 = global.member[(_e45 + 1u)];
        phi_148_ = type_11(_e56, _e60);
    } else {
        phi_148_ = type_11(4294967295u, 0u);
    }
    let _e63 = phi_148_;
    if (_e46 >= _e63.member_1) {
        phi_407_ = 4294967295u;
    } else {
        phi_407_ = (_e63.member + (26u * _e46));
    }
    let _e70 = phi_407_;
    if (_e48 >= 26u) {
        phi_426_ = (_e70 <= (_e48 - 26u));
    } else {
        phi_426_ = false;
    }
    let _e75 = phi_426_;
    if _e75 {
        let _e78 = global.member[_e70];
        let _e83 = global.member[(_e70 + 1u)];
        let _e88 = global.member[(_e70 + 2u)];
        let _e94 = global.member[(_e70 + 3u)];
        let _e99 = global.member[(_e70 + 4u)];
        let _e104 = global.member[(_e70 + 5u)];
        let _e109 = global.member[(_e70 + 6u)];
        let _e115 = global.member[(_e70 + 7u)];
        let _e120 = global.member[(_e70 + 8u)];
        let _e126 = global.member[(_e70 + 9u)];
        let _e131 = global.member[(_e70 + 10u)];
        let _e137 = global.member[(_e70 + 11u)];
        let _e142 = global.member[(_e70 + 12u)];
        let _e147 = global.member[(_e70 + 13u)];
        let _e153 = global.member[(_e70 + 14u)];
        let _e158 = global.member[(_e70 + 15u)];
        let _e163 = global.member[(_e70 + 16u)];
        let _e168 = global.member[(_e70 + 17u)];
        local_1 = array<u32, 4>(0u, 0u, 0u, 0u);
        phi_239_ = type_11(0u, 4u);
        loop {
            let _e173 = phi_239_;
            if (_e173.member < _e173.member_1) {
                phi_240_ = type_11((_e173.member + 1u), _e173.member_1);
                phi_263_ = type_11(1u, _e173.member);
            } else {
                phi_240_ = _e173;
                phi_263_ = type_11(0u, type_11().member_1);
            }
            let _e186 = phi_240_;
            let _e188 = phi_263_;
            switch bitcast<i32>(_e188.member) {
                case 0: {
                    phi_276_ = false;
                    break;
                }
                case 1: {
                    let _e195 = global.member[((_e70 + 18u) + _e188.member_1)];
                    local_1[_e188.member_1] = _e195;
                    phi_276_ = true;
                    break;
                }
                default: {
                    phi_276_ = bool();
                    break;
                }
            }
            let _e198 = phi_276_;
            continue;
            continuing {
                phi_239_ = _e186;
                break if !(_e198);
            }
        }
        let _e200 = local_1;
        local = array<f32, 4>(0f, 0f, 0f, 0f);
        phi_282_ = type_11(0u, 4u);
        loop {
            let _e203 = phi_282_;
            if (_e203.member < _e203.member_1) {
                phi_283_ = type_11((_e203.member + 1u), _e203.member_1);
                phi_306_ = type_11(1u, _e203.member);
            } else {
                phi_283_ = _e203;
                phi_306_ = type_11(0u, type_11().member_1);
            }
            let _e216 = phi_283_;
            let _e218 = phi_306_;
            switch bitcast<i32>(_e218.member) {
                case 0: {
                    phi_320_ = false;
                    break;
                }
                case 1: {
                    let _e225 = global.member[((_e70 + 22u) + _e218.member_1)];
                    local[_e218.member_1] = bitcast<f32>(_e225);
                    phi_320_ = true;
                    break;
                }
                default: {
                    phi_320_ = bool();
                    break;
                }
            }
            let _e229 = phi_320_;
            continue;
            continuing {
                phi_282_ = _e216;
                break if !(_e229);
            }
        }
        let _e231 = local;
        phi_324_ = type_18(vec3<f32>(bitcast<f32>(_e78), bitcast<f32>(_e83), bitcast<f32>(_e88)), vec4<f32>(bitcast<f32>(_e94), bitcast<f32>(_e99), bitcast<f32>(_e104), bitcast<f32>(_e109)), vec3<f32>(bitcast<f32>(_e137), bitcast<f32>(_e142), bitcast<f32>(_e147)), vec4<f32>(bitcast<f32>(_e153), bitcast<f32>(_e158), bitcast<f32>(_e163), bitcast<f32>(_e168)), _e200, _e231, vec2<f32>(bitcast<f32>(_e115), bitcast<f32>(_e120)), vec2<f32>(bitcast<f32>(_e126), bitcast<f32>(_e131)));
    } else {
        phi_324_ = type_18(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), vec3<f32>(0f, 0f, 1f), vec4<f32>(0f, 1f, 0f, 0f), array<u32, 4>(0u, 0u, 0u, 0u), array<f32, 4>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
    }
    let _e234 = phi_324_;
    global_2 = vec4<f32>(_e234.member.x, _e234.member.y, _e234.member.z, 1f);
    global_3 = _e234.member_1;
    return;
}

@vertex 
fn tutorialtutorial_slabbed_vertices(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_4 = param;
    global_1 = param_1;
    function();
    let _e7 = global_2.y;
    global_2.y = -(_e7);
    let _e9 = global_3;
    let _e10 = global_2;
    return VertexOutput(_e9, _e10);
}
