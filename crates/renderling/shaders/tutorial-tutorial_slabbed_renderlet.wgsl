struct type_11 {
    member: array<u32>,
}

struct type_18 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_19 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
    member_2: vec3<f32>,
}

struct type_20 {
    member: type_18,
    member_1: type_18,
    member_2: vec3<f32>,
    member_3: type_19,
}

struct type_22 {
    member: u32,
    member_1: u32,
}

struct type_25 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_28 {
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
var<storage> global_1: type_11;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: u32;
var<private> global_4: vec4<f32>;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var local_2: array<f32, 4>;
    var local_3: array<u32, 4>;
    var phi_613_: u32;
    var phi_1426_: bool;
    var phi_699_: type_22;
    var phi_700_: type_22;
    var phi_723_: type_22;
    var phi_736_: bool;
    var phi_742_: type_22;
    var phi_743_: type_22;
    var phi_766_: type_22;
    var phi_780_: bool;
    var phi_784_: type_28;
    var phi_1456_: bool;
    var phi_835_: type_25;
    var phi_1526_: bool;
    var phi_996_: type_22;
    var phi_997_: type_22;
    var phi_1020_: type_22;
    var phi_1047_: bool;
    var phi_1053_: type_22;
    var phi_1054_: type_22;
    var phi_1077_: type_22;
    var phi_1100_: bool;
    var phi_1121_: type_20;

    let _e79 = global_3;
    let _e80 = global;
    let _e82 = arrayLength((&global_1.member));
    let _e86 = global_1.member[(_e79 + 1u)];
    let _e90 = global_1.member[(_e79 + 2u)];
    let _e94 = global_1.member[(_e79 + 9u)];
    let _e98 = global_1.member[(_e79 + 10u)];
    if (_e80 >= _e90) {
        phi_613_ = 4294967295u;
    } else {
        phi_613_ = (_e86 + (26u * _e80));
    }
    let _e103 = phi_613_;
    if (_e82 >= 26u) {
        phi_1426_ = (_e103 <= (_e82 - 26u));
    } else {
        phi_1426_ = false;
    }
    let _e108 = phi_1426_;
    if _e108 {
        let _e111 = global_1.member[_e103];
        let _e116 = global_1.member[(_e103 + 1u)];
        let _e121 = global_1.member[(_e103 + 2u)];
        let _e127 = global_1.member[(_e103 + 3u)];
        let _e132 = global_1.member[(_e103 + 4u)];
        let _e137 = global_1.member[(_e103 + 5u)];
        let _e142 = global_1.member[(_e103 + 6u)];
        let _e148 = global_1.member[(_e103 + 7u)];
        let _e153 = global_1.member[(_e103 + 8u)];
        let _e159 = global_1.member[(_e103 + 9u)];
        let _e164 = global_1.member[(_e103 + 10u)];
        let _e170 = global_1.member[(_e103 + 11u)];
        let _e175 = global_1.member[(_e103 + 12u)];
        let _e180 = global_1.member[(_e103 + 13u)];
        let _e186 = global_1.member[(_e103 + 14u)];
        let _e191 = global_1.member[(_e103 + 15u)];
        let _e196 = global_1.member[(_e103 + 16u)];
        let _e201 = global_1.member[(_e103 + 17u)];
        local_3 = array<u32, 4>(0u, 0u, 0u, 0u);
        phi_699_ = type_22(0u, 4u);
        loop {
            let _e206 = phi_699_;
            if (_e206.member < _e206.member_1) {
                phi_700_ = type_22((_e206.member + 1u), _e206.member_1);
                phi_723_ = type_22(1u, _e206.member);
            } else {
                phi_700_ = _e206;
                phi_723_ = type_22(0u, type_22().member_1);
            }
            let _e219 = phi_700_;
            let _e221 = phi_723_;
            switch bitcast<i32>(_e221.member) {
                case 0: {
                    phi_736_ = false;
                    break;
                }
                case 1: {
                    let _e228 = global_1.member[((_e103 + 18u) + _e221.member_1)];
                    local_3[_e221.member_1] = _e228;
                    phi_736_ = true;
                    break;
                }
                default: {
                    phi_736_ = bool();
                    break;
                }
            }
            let _e231 = phi_736_;
            continue;
            continuing {
                phi_699_ = _e219;
                break if !(_e231);
            }
        }
        let _e233 = local_3;
        local_2 = array<f32, 4>(0f, 0f, 0f, 0f);
        phi_742_ = type_22(0u, 4u);
        loop {
            let _e236 = phi_742_;
            if (_e236.member < _e236.member_1) {
                phi_743_ = type_22((_e236.member + 1u), _e236.member_1);
                phi_766_ = type_22(1u, _e236.member);
            } else {
                phi_743_ = _e236;
                phi_766_ = type_22(0u, type_22().member_1);
            }
            let _e249 = phi_743_;
            let _e251 = phi_766_;
            switch bitcast<i32>(_e251.member) {
                case 0: {
                    phi_780_ = false;
                    break;
                }
                case 1: {
                    let _e258 = global_1.member[((_e103 + 22u) + _e251.member_1)];
                    local_2[_e251.member_1] = bitcast<f32>(_e258);
                    phi_780_ = true;
                    break;
                }
                default: {
                    phi_780_ = bool();
                    break;
                }
            }
            let _e262 = phi_780_;
            continue;
            continuing {
                phi_742_ = _e249;
                break if !(_e262);
            }
        }
        let _e264 = local_2;
        phi_784_ = type_28(vec3<f32>(bitcast<f32>(_e111), bitcast<f32>(_e116), bitcast<f32>(_e121)), vec4<f32>(bitcast<f32>(_e127), bitcast<f32>(_e132), bitcast<f32>(_e137), bitcast<f32>(_e142)), vec3<f32>(bitcast<f32>(_e170), bitcast<f32>(_e175), bitcast<f32>(_e180)), vec4<f32>(bitcast<f32>(_e186), bitcast<f32>(_e191), bitcast<f32>(_e196), bitcast<f32>(_e201)), _e233, _e264, vec2<f32>(bitcast<f32>(_e148), bitcast<f32>(_e153)), vec2<f32>(bitcast<f32>(_e159), bitcast<f32>(_e164)));
    } else {
        phi_784_ = type_28(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), vec3<f32>(0f, 0f, 1f), vec4<f32>(0f, 1f, 0f, 0f), array<u32, 4>(0u, 0u, 0u, 0u), array<f32, 4>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
    }
    let _e267 = phi_784_;
    global_4 = _e267.member_1;
    if (_e82 >= 10u) {
        phi_1456_ = (_e98 <= (_e82 - 10u));
    } else {
        phi_1456_ = false;
    }
    let _e273 = phi_1456_;
    if _e273 {
        let _e276 = global_1.member[_e98];
        let _e281 = global_1.member[(_e98 + 1u)];
        let _e286 = global_1.member[(_e98 + 2u)];
        let _e292 = global_1.member[(_e98 + 3u)];
        let _e297 = global_1.member[(_e98 + 4u)];
        let _e302 = global_1.member[(_e98 + 5u)];
        let _e307 = global_1.member[(_e98 + 6u)];
        let _e313 = global_1.member[(_e98 + 7u)];
        let _e318 = global_1.member[(_e98 + 8u)];
        let _e323 = global_1.member[(_e98 + 9u)];
        phi_835_ = type_25(vec3<f32>(bitcast<f32>(_e276), bitcast<f32>(_e281), bitcast<f32>(_e286)), vec4<f32>(bitcast<f32>(_e292), bitcast<f32>(_e297), bitcast<f32>(_e302), bitcast<f32>(_e307)), vec3<f32>(bitcast<f32>(_e313), bitcast<f32>(_e318), bitcast<f32>(_e323)));
    } else {
        phi_835_ = type_25(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
    }
    let _e328 = phi_835_;
    let _e336 = (_e328.member_1.x + _e328.member_1.x);
    let _e337 = (_e328.member_1.y + _e328.member_1.y);
    let _e338 = (_e328.member_1.z + _e328.member_1.z);
    let _e340 = (_e328.member_1.z * _e338);
    let _e341 = (_e328.member_1.w * _e336);
    let _e342 = (_e328.member_1.w * _e337);
    let _e343 = (_e328.member_1.w * _e338);
    let _e363 = (vec4<f32>((1f - fma(_e328.member_1.y, _e337, _e340)), fma(_e328.member_1.x, _e337, _e343), fma(_e328.member_1.x, _e338, -(_e342)), 0f) * _e328.member_2.x);
    let _e365 = (vec4<f32>(fma(_e328.member_1.x, _e337, -(_e343)), (1f - fma(_e328.member_1.x, _e336, _e340)), fma(_e328.member_1.y, _e338, _e341), 0f) * _e328.member_2.y);
    let _e367 = (vec4<f32>(fma(_e328.member_1.x, _e338, _e342), fma(_e328.member_1.y, _e338, -(_e341)), (1f - fma(_e328.member_1.x, _e336, (_e328.member_1.y * _e337))), 0f) * _e328.member_2.z);
    if (_e82 >= 86u) {
        phi_1526_ = (_e94 <= (_e82 - 86u));
    } else {
        phi_1526_ = false;
    }
    let _e375 = phi_1526_;
    if _e375 {
        let _e378 = global_1.member[_e94];
        let _e383 = global_1.member[(_e94 + 1u)];
        let _e388 = global_1.member[(_e94 + 2u)];
        let _e393 = global_1.member[(_e94 + 3u)];
        let _e399 = global_1.member[(_e94 + 4u)];
        let _e404 = global_1.member[(_e94 + 5u)];
        let _e409 = global_1.member[(_e94 + 6u)];
        let _e414 = global_1.member[(_e94 + 7u)];
        let _e420 = global_1.member[(_e94 + 8u)];
        let _e425 = global_1.member[(_e94 + 9u)];
        let _e430 = global_1.member[(_e94 + 10u)];
        let _e435 = global_1.member[(_e94 + 11u)];
        let _e441 = global_1.member[(_e94 + 12u)];
        let _e446 = global_1.member[(_e94 + 13u)];
        let _e451 = global_1.member[(_e94 + 14u)];
        let _e456 = global_1.member[(_e94 + 15u)];
        let _e463 = global_1.member[(_e94 + 16u)];
        let _e468 = global_1.member[(_e94 + 17u)];
        let _e473 = global_1.member[(_e94 + 18u)];
        let _e478 = global_1.member[(_e94 + 19u)];
        let _e484 = global_1.member[(_e94 + 20u)];
        let _e489 = global_1.member[(_e94 + 21u)];
        let _e494 = global_1.member[(_e94 + 22u)];
        let _e499 = global_1.member[(_e94 + 23u)];
        let _e505 = global_1.member[(_e94 + 24u)];
        let _e510 = global_1.member[(_e94 + 25u)];
        let _e515 = global_1.member[(_e94 + 26u)];
        let _e520 = global_1.member[(_e94 + 27u)];
        let _e526 = global_1.member[(_e94 + 28u)];
        let _e531 = global_1.member[(_e94 + 29u)];
        let _e536 = global_1.member[(_e94 + 30u)];
        let _e541 = global_1.member[(_e94 + 31u)];
        let _e548 = global_1.member[(_e94 + 32u)];
        let _e553 = global_1.member[(_e94 + 33u)];
        let _e558 = global_1.member[(_e94 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_996_ = type_22(0u, 6u);
        loop {
            let _e563 = phi_996_;
            if (_e563.member < _e563.member_1) {
                phi_997_ = type_22((_e563.member + 1u), _e563.member_1);
                phi_1020_ = type_22(1u, _e563.member);
            } else {
                phi_997_ = _e563;
                phi_1020_ = type_22(0u, type_22().member_1);
            }
            let _e576 = phi_997_;
            let _e578 = phi_1020_;
            switch bitcast<i32>(_e578.member) {
                case 0: {
                    phi_1047_ = false;
                    break;
                }
                case 1: {
                    let _e583 = ((_e94 + 35u) + (_e578.member_1 * 4u));
                    let _e586 = global_1.member[_e583];
                    let _e591 = global_1.member[(_e583 + 1u)];
                    let _e596 = global_1.member[(_e583 + 2u)];
                    let _e601 = global_1.member[(_e583 + 3u)];
                    local_1[_e578.member_1] = vec4<f32>(bitcast<f32>(_e586), bitcast<f32>(_e591), bitcast<f32>(_e596), bitcast<f32>(_e601));
                    phi_1047_ = true;
                    break;
                }
                default: {
                    phi_1047_ = bool();
                    break;
                }
            }
            let _e606 = phi_1047_;
            continue;
            continuing {
                phi_996_ = _e576;
                break if !(_e606);
            }
        }
        let _e608 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_1053_ = type_22(0u, 8u);
        loop {
            let _e611 = phi_1053_;
            if (_e611.member < _e611.member_1) {
                phi_1054_ = type_22((_e611.member + 1u), _e611.member_1);
                phi_1077_ = type_22(1u, _e611.member);
            } else {
                phi_1054_ = _e611;
                phi_1077_ = type_22(0u, type_22().member_1);
            }
            let _e624 = phi_1054_;
            let _e626 = phi_1077_;
            switch bitcast<i32>(_e626.member) {
                case 0: {
                    phi_1100_ = false;
                    break;
                }
                case 1: {
                    let _e631 = ((_e94 + 59u) + (_e626.member_1 * 3u));
                    let _e634 = global_1.member[_e631];
                    let _e639 = global_1.member[(_e631 + 1u)];
                    let _e644 = global_1.member[(_e631 + 2u)];
                    local[_e626.member_1] = vec3<f32>(bitcast<f32>(_e634), bitcast<f32>(_e639), bitcast<f32>(_e644));
                    phi_1100_ = true;
                    break;
                }
                default: {
                    phi_1100_ = bool();
                    break;
                }
            }
            let _e649 = phi_1100_;
            continue;
            continuing {
                phi_1053_ = _e624;
                break if !(_e649);
            }
        }
        let _e651 = local;
        let _e655 = global_1.member[(_e94 + 83u)];
        let _e660 = global_1.member[(_e94 + 84u)];
        let _e665 = global_1.member[(_e94 + 85u)];
        phi_1121_ = type_20(type_18(vec4<f32>(bitcast<f32>(_e378), bitcast<f32>(_e383), bitcast<f32>(_e388), bitcast<f32>(_e393)), vec4<f32>(bitcast<f32>(_e399), bitcast<f32>(_e404), bitcast<f32>(_e409), bitcast<f32>(_e414)), vec4<f32>(bitcast<f32>(_e420), bitcast<f32>(_e425), bitcast<f32>(_e430), bitcast<f32>(_e435)), vec4<f32>(bitcast<f32>(_e441), bitcast<f32>(_e446), bitcast<f32>(_e451), bitcast<f32>(_e456))), type_18(vec4<f32>(bitcast<f32>(_e463), bitcast<f32>(_e468), bitcast<f32>(_e473), bitcast<f32>(_e478)), vec4<f32>(bitcast<f32>(_e484), bitcast<f32>(_e489), bitcast<f32>(_e494), bitcast<f32>(_e499)), vec4<f32>(bitcast<f32>(_e505), bitcast<f32>(_e510), bitcast<f32>(_e515), bitcast<f32>(_e520)), vec4<f32>(bitcast<f32>(_e526), bitcast<f32>(_e531), bitcast<f32>(_e536), bitcast<f32>(_e541))), vec3<f32>(bitcast<f32>(_e548), bitcast<f32>(_e553), bitcast<f32>(_e558)), type_19(_e651, _e608, vec3<f32>(bitcast<f32>(_e655), bitcast<f32>(_e660), bitcast<f32>(_e665))));
    } else {
        phi_1121_ = type_20(type_18(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_18(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_19(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e671 = phi_1121_;
    let _e711 = fma(_e671.member.member_3.x, _e671.member_1.member.w, fma(_e671.member.member_2.x, _e671.member_1.member.z, fma(_e671.member.member.x, _e671.member_1.member.x, (_e671.member.member_1.x * _e671.member_1.member.y))));
    let _e712 = fma(_e671.member.member_3.y, _e671.member_1.member.w, fma(_e671.member.member_2.y, _e671.member_1.member.z, fma(_e671.member.member.y, _e671.member_1.member.x, (_e671.member.member_1.y * _e671.member_1.member.y))));
    let _e713 = fma(_e671.member.member_3.z, _e671.member_1.member.w, fma(_e671.member.member_2.z, _e671.member_1.member.z, fma(_e671.member.member.z, _e671.member_1.member.x, (_e671.member.member_1.z * _e671.member_1.member.y))));
    let _e714 = fma(_e671.member.member_3.w, _e671.member_1.member.w, fma(_e671.member.member_2.w, _e671.member_1.member.z, fma(_e671.member.member.w, _e671.member_1.member.x, (_e671.member.member_1.w * _e671.member_1.member.y))));
    let _e732 = fma(_e671.member.member_3.x, _e671.member_1.member_1.w, fma(_e671.member.member_2.x, _e671.member_1.member_1.z, fma(_e671.member.member.x, _e671.member_1.member_1.x, (_e671.member.member_1.x * _e671.member_1.member_1.y))));
    let _e733 = fma(_e671.member.member_3.y, _e671.member_1.member_1.w, fma(_e671.member.member_2.y, _e671.member_1.member_1.z, fma(_e671.member.member.y, _e671.member_1.member_1.x, (_e671.member.member_1.y * _e671.member_1.member_1.y))));
    let _e734 = fma(_e671.member.member_3.z, _e671.member_1.member_1.w, fma(_e671.member.member_2.z, _e671.member_1.member_1.z, fma(_e671.member.member.z, _e671.member_1.member_1.x, (_e671.member.member_1.z * _e671.member_1.member_1.y))));
    let _e735 = fma(_e671.member.member_3.w, _e671.member_1.member_1.w, fma(_e671.member.member_2.w, _e671.member_1.member_1.z, fma(_e671.member.member.w, _e671.member_1.member_1.x, (_e671.member.member_1.w * _e671.member_1.member_1.y))));
    let _e753 = fma(_e671.member.member_3.x, _e671.member_1.member_2.w, fma(_e671.member.member_2.x, _e671.member_1.member_2.z, fma(_e671.member.member.x, _e671.member_1.member_2.x, (_e671.member.member_1.x * _e671.member_1.member_2.y))));
    let _e754 = fma(_e671.member.member_3.y, _e671.member_1.member_2.w, fma(_e671.member.member_2.y, _e671.member_1.member_2.z, fma(_e671.member.member.y, _e671.member_1.member_2.x, (_e671.member.member_1.y * _e671.member_1.member_2.y))));
    let _e755 = fma(_e671.member.member_3.z, _e671.member_1.member_2.w, fma(_e671.member.member_2.z, _e671.member_1.member_2.z, fma(_e671.member.member.z, _e671.member_1.member_2.x, (_e671.member.member_1.z * _e671.member_1.member_2.y))));
    let _e756 = fma(_e671.member.member_3.w, _e671.member_1.member_2.w, fma(_e671.member.member_2.w, _e671.member_1.member_2.z, fma(_e671.member.member.w, _e671.member_1.member_2.x, (_e671.member.member_1.w * _e671.member_1.member_2.y))));
    let _e774 = fma(_e671.member.member_3.x, _e671.member_1.member_3.w, fma(_e671.member.member_2.x, _e671.member_1.member_3.z, fma(_e671.member.member.x, _e671.member_1.member_3.x, (_e671.member.member_1.x * _e671.member_1.member_3.y))));
    let _e775 = fma(_e671.member.member_3.y, _e671.member_1.member_3.w, fma(_e671.member.member_2.y, _e671.member_1.member_3.z, fma(_e671.member.member.y, _e671.member_1.member_3.x, (_e671.member.member_1.y * _e671.member_1.member_3.y))));
    let _e776 = fma(_e671.member.member_3.z, _e671.member_1.member_3.w, fma(_e671.member.member_2.z, _e671.member_1.member_3.z, fma(_e671.member.member.z, _e671.member_1.member_3.x, (_e671.member.member_1.z * _e671.member_1.member_3.y))));
    let _e777 = fma(_e671.member.member_3.w, _e671.member_1.member_3.w, fma(_e671.member.member_2.w, _e671.member_1.member_3.z, fma(_e671.member.member.w, _e671.member_1.member_3.x, (_e671.member.member_1.w * _e671.member_1.member_3.y))));
    global_2 = vec4<f32>((fma(fma(_e774, _e367.w, fma(_e753, _e367.z, fma(_e711, _e367.x, (_e732 * _e367.y)))), _e267.member.z, fma(fma(_e774, _e363.w, fma(_e753, _e363.z, fma(_e711, _e363.x, (_e732 * _e363.y)))), _e267.member.x, (fma(_e774, _e365.w, fma(_e753, _e365.z, fma(_e711, _e365.x, (_e732 * _e365.y)))) * _e267.member.y))) + (fma(_e753, _e328.member.z, fma(_e711, _e328.member.x, (_e732 * _e328.member.y))) + _e774)), (fma(fma(_e775, _e367.w, fma(_e754, _e367.z, fma(_e712, _e367.x, (_e733 * _e367.y)))), _e267.member.z, fma(fma(_e775, _e363.w, fma(_e754, _e363.z, fma(_e712, _e363.x, (_e733 * _e363.y)))), _e267.member.x, (fma(_e775, _e365.w, fma(_e754, _e365.z, fma(_e712, _e365.x, (_e733 * _e365.y)))) * _e267.member.y))) + (fma(_e754, _e328.member.z, fma(_e712, _e328.member.x, (_e733 * _e328.member.y))) + _e775)), (fma(fma(_e776, _e367.w, fma(_e755, _e367.z, fma(_e713, _e367.x, (_e734 * _e367.y)))), _e267.member.z, fma(fma(_e776, _e363.w, fma(_e755, _e363.z, fma(_e713, _e363.x, (_e734 * _e363.y)))), _e267.member.x, (fma(_e776, _e365.w, fma(_e755, _e365.z, fma(_e713, _e365.x, (_e734 * _e365.y)))) * _e267.member.y))) + (fma(_e755, _e328.member.z, fma(_e713, _e328.member.x, (_e734 * _e328.member.y))) + _e776)), (fma(fma(_e777, _e367.w, fma(_e756, _e367.z, fma(_e714, _e367.x, (_e735 * _e367.y)))), _e267.member.z, fma(fma(_e777, _e363.w, fma(_e756, _e363.z, fma(_e714, _e363.x, (_e735 * _e363.y)))), _e267.member.x, (fma(_e777, _e365.w, fma(_e756, _e365.z, fma(_e714, _e365.x, (_e735 * _e365.y)))) * _e267.member.y))) + (fma(_e756, _e328.member.z, fma(_e714, _e328.member.x, (_e735 * _e328.member.y))) + _e777)));
    return;
}

@vertex 
fn tutorialtutorial_slabbed_renderlet(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_3 = param;
    global = param_1;
    function();
    let _e7 = global_2.y;
    global_2.y = -(_e7);
    let _e9 = global_4;
    let _e10 = global_2;
    return VertexOutput(_e9, _e10);
}
