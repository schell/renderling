struct type_3 {
    member: array<u32>,
}

struct type_13 {
    member: u32,
    member_1: u32,
}

struct type_19 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_20 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
    member_2: vec3<f32>,
}

struct type_21 {
    member: type_19,
    member_1: type_19,
    member_2: vec3<f32>,
    member_3: type_20,
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

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: u32;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: u32;
var<private> global_4: vec4<f32>;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var local_2: array<f32, 4>;
    var local_3: array<u32, 4>;
    var phi_619_: u32;
    var phi_1433_: bool;
    var phi_705_: type_13;
    var phi_706_: type_13;
    var phi_729_: type_13;
    var phi_742_: bool;
    var phi_748_: type_13;
    var phi_749_: type_13;
    var phi_772_: type_13;
    var phi_786_: bool;
    var phi_790_: type_28;
    var phi_1463_: bool;
    var phi_841_: type_25;
    var phi_1541_: bool;
    var phi_1005_: type_13;
    var phi_1006_: type_13;
    var phi_1029_: type_13;
    var phi_1056_: bool;
    var phi_1062_: type_13;
    var phi_1063_: type_13;
    var phi_1086_: type_13;
    var phi_1109_: bool;
    var phi_1130_: type_21;

    let _e79 = global_3;
    let _e80 = global_1;
    let _e82 = arrayLength((&global.member));
    let _e86 = global.member[(_e79 + 1u)];
    let _e90 = global.member[(_e79 + 2u)];
    let _e94 = global.member[(_e79 + 9u)];
    let _e98 = global.member[(_e79 + 16u)];
    if (_e80 >= _e90) {
        phi_619_ = 4294967295u;
    } else {
        phi_619_ = (_e86 + (26u * _e80));
    }
    let _e103 = phi_619_;
    if (_e82 >= 26u) {
        phi_1433_ = (_e103 <= (_e82 - 26u));
    } else {
        phi_1433_ = false;
    }
    let _e108 = phi_1433_;
    if _e108 {
        let _e111 = global.member[_e103];
        let _e116 = global.member[(_e103 + 1u)];
        let _e121 = global.member[(_e103 + 2u)];
        let _e127 = global.member[(_e103 + 3u)];
        let _e132 = global.member[(_e103 + 4u)];
        let _e137 = global.member[(_e103 + 5u)];
        let _e142 = global.member[(_e103 + 6u)];
        let _e148 = global.member[(_e103 + 7u)];
        let _e153 = global.member[(_e103 + 8u)];
        let _e159 = global.member[(_e103 + 9u)];
        let _e164 = global.member[(_e103 + 10u)];
        let _e170 = global.member[(_e103 + 11u)];
        let _e175 = global.member[(_e103 + 12u)];
        let _e180 = global.member[(_e103 + 13u)];
        let _e186 = global.member[(_e103 + 14u)];
        let _e191 = global.member[(_e103 + 15u)];
        let _e196 = global.member[(_e103 + 16u)];
        let _e201 = global.member[(_e103 + 17u)];
        local_3 = array<u32, 4>(0u, 0u, 0u, 0u);
        phi_705_ = type_13(0u, 4u);
        loop {
            let _e206 = phi_705_;
            if (_e206.member < _e206.member_1) {
                phi_706_ = type_13((_e206.member + 1u), _e206.member_1);
                phi_729_ = type_13(1u, _e206.member);
            } else {
                phi_706_ = _e206;
                phi_729_ = type_13(0u, type_13().member_1);
            }
            let _e219 = phi_706_;
            let _e221 = phi_729_;
            switch bitcast<i32>(_e221.member) {
                case 0: {
                    phi_742_ = false;
                    break;
                }
                case 1: {
                    let _e228 = global.member[((_e103 + 18u) + _e221.member_1)];
                    local_3[_e221.member_1] = _e228;
                    phi_742_ = true;
                    break;
                }
                default: {
                    phi_742_ = bool();
                    break;
                }
            }
            let _e231 = phi_742_;
            continue;
            continuing {
                phi_705_ = _e219;
                break if !(_e231);
            }
        }
        let _e233 = local_3;
        local_2 = array<f32, 4>(0f, 0f, 0f, 0f);
        phi_748_ = type_13(0u, 4u);
        loop {
            let _e236 = phi_748_;
            if (_e236.member < _e236.member_1) {
                phi_749_ = type_13((_e236.member + 1u), _e236.member_1);
                phi_772_ = type_13(1u, _e236.member);
            } else {
                phi_749_ = _e236;
                phi_772_ = type_13(0u, type_13().member_1);
            }
            let _e249 = phi_749_;
            let _e251 = phi_772_;
            switch bitcast<i32>(_e251.member) {
                case 0: {
                    phi_786_ = false;
                    break;
                }
                case 1: {
                    let _e258 = global.member[((_e103 + 22u) + _e251.member_1)];
                    local_2[_e251.member_1] = bitcast<f32>(_e258);
                    phi_786_ = true;
                    break;
                }
                default: {
                    phi_786_ = bool();
                    break;
                }
            }
            let _e262 = phi_786_;
            continue;
            continuing {
                phi_748_ = _e249;
                break if !(_e262);
            }
        }
        let _e264 = local_2;
        phi_790_ = type_28(vec3<f32>(bitcast<f32>(_e111), bitcast<f32>(_e116), bitcast<f32>(_e121)), vec4<f32>(bitcast<f32>(_e127), bitcast<f32>(_e132), bitcast<f32>(_e137), bitcast<f32>(_e142)), vec3<f32>(bitcast<f32>(_e170), bitcast<f32>(_e175), bitcast<f32>(_e180)), vec4<f32>(bitcast<f32>(_e186), bitcast<f32>(_e191), bitcast<f32>(_e196), bitcast<f32>(_e201)), _e233, _e264, vec2<f32>(bitcast<f32>(_e148), bitcast<f32>(_e153)), vec2<f32>(bitcast<f32>(_e159), bitcast<f32>(_e164)));
    } else {
        phi_790_ = type_28(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), vec3<f32>(0f, 0f, 1f), vec4<f32>(0f, 1f, 0f, 0f), array<u32, 4>(0u, 0u, 0u, 0u), array<f32, 4>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
    }
    let _e267 = phi_790_;
    global_4 = _e267.member_1;
    if (_e82 >= 10u) {
        phi_1463_ = (_e94 <= (_e82 - 10u));
    } else {
        phi_1463_ = false;
    }
    let _e273 = phi_1463_;
    if _e273 {
        let _e276 = global.member[_e94];
        let _e281 = global.member[(_e94 + 1u)];
        let _e286 = global.member[(_e94 + 2u)];
        let _e292 = global.member[(_e94 + 3u)];
        let _e297 = global.member[(_e94 + 4u)];
        let _e302 = global.member[(_e94 + 5u)];
        let _e307 = global.member[(_e94 + 6u)];
        let _e313 = global.member[(_e94 + 7u)];
        let _e318 = global.member[(_e94 + 8u)];
        let _e323 = global.member[(_e94 + 9u)];
        phi_841_ = type_25(vec3<f32>(bitcast<f32>(_e276), bitcast<f32>(_e281), bitcast<f32>(_e286)), vec4<f32>(bitcast<f32>(_e292), bitcast<f32>(_e297), bitcast<f32>(_e302), bitcast<f32>(_e307)), vec3<f32>(bitcast<f32>(_e313), bitcast<f32>(_e318), bitcast<f32>(_e323)));
    } else {
        phi_841_ = type_25(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
    }
    let _e328 = phi_841_;
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
    let _e373 = global.member[_e98];
    if (_e82 >= 86u) {
        phi_1541_ = (_e373 <= (_e82 - 86u));
    } else {
        phi_1541_ = false;
    }
    let _e378 = phi_1541_;
    if _e378 {
        let _e381 = global.member[_e373];
        let _e386 = global.member[(_e373 + 1u)];
        let _e391 = global.member[(_e373 + 2u)];
        let _e396 = global.member[(_e373 + 3u)];
        let _e402 = global.member[(_e373 + 4u)];
        let _e407 = global.member[(_e373 + 5u)];
        let _e412 = global.member[(_e373 + 6u)];
        let _e417 = global.member[(_e373 + 7u)];
        let _e423 = global.member[(_e373 + 8u)];
        let _e428 = global.member[(_e373 + 9u)];
        let _e433 = global.member[(_e373 + 10u)];
        let _e438 = global.member[(_e373 + 11u)];
        let _e444 = global.member[(_e373 + 12u)];
        let _e449 = global.member[(_e373 + 13u)];
        let _e454 = global.member[(_e373 + 14u)];
        let _e459 = global.member[(_e373 + 15u)];
        let _e466 = global.member[(_e373 + 16u)];
        let _e471 = global.member[(_e373 + 17u)];
        let _e476 = global.member[(_e373 + 18u)];
        let _e481 = global.member[(_e373 + 19u)];
        let _e487 = global.member[(_e373 + 20u)];
        let _e492 = global.member[(_e373 + 21u)];
        let _e497 = global.member[(_e373 + 22u)];
        let _e502 = global.member[(_e373 + 23u)];
        let _e508 = global.member[(_e373 + 24u)];
        let _e513 = global.member[(_e373 + 25u)];
        let _e518 = global.member[(_e373 + 26u)];
        let _e523 = global.member[(_e373 + 27u)];
        let _e529 = global.member[(_e373 + 28u)];
        let _e534 = global.member[(_e373 + 29u)];
        let _e539 = global.member[(_e373 + 30u)];
        let _e544 = global.member[(_e373 + 31u)];
        let _e551 = global.member[(_e373 + 32u)];
        let _e556 = global.member[(_e373 + 33u)];
        let _e561 = global.member[(_e373 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_1005_ = type_13(0u, 6u);
        loop {
            let _e566 = phi_1005_;
            if (_e566.member < _e566.member_1) {
                phi_1006_ = type_13((_e566.member + 1u), _e566.member_1);
                phi_1029_ = type_13(1u, _e566.member);
            } else {
                phi_1006_ = _e566;
                phi_1029_ = type_13(0u, type_13().member_1);
            }
            let _e579 = phi_1006_;
            let _e581 = phi_1029_;
            switch bitcast<i32>(_e581.member) {
                case 0: {
                    phi_1056_ = false;
                    break;
                }
                case 1: {
                    let _e586 = ((_e373 + 35u) + (_e581.member_1 * 4u));
                    let _e589 = global.member[_e586];
                    let _e594 = global.member[(_e586 + 1u)];
                    let _e599 = global.member[(_e586 + 2u)];
                    let _e604 = global.member[(_e586 + 3u)];
                    local_1[_e581.member_1] = vec4<f32>(bitcast<f32>(_e589), bitcast<f32>(_e594), bitcast<f32>(_e599), bitcast<f32>(_e604));
                    phi_1056_ = true;
                    break;
                }
                default: {
                    phi_1056_ = bool();
                    break;
                }
            }
            let _e609 = phi_1056_;
            continue;
            continuing {
                phi_1005_ = _e579;
                break if !(_e609);
            }
        }
        let _e611 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_1062_ = type_13(0u, 8u);
        loop {
            let _e614 = phi_1062_;
            if (_e614.member < _e614.member_1) {
                phi_1063_ = type_13((_e614.member + 1u), _e614.member_1);
                phi_1086_ = type_13(1u, _e614.member);
            } else {
                phi_1063_ = _e614;
                phi_1086_ = type_13(0u, type_13().member_1);
            }
            let _e627 = phi_1063_;
            let _e629 = phi_1086_;
            switch bitcast<i32>(_e629.member) {
                case 0: {
                    phi_1109_ = false;
                    break;
                }
                case 1: {
                    let _e634 = ((_e373 + 59u) + (_e629.member_1 * 3u));
                    let _e637 = global.member[_e634];
                    let _e642 = global.member[(_e634 + 1u)];
                    let _e647 = global.member[(_e634 + 2u)];
                    local[_e629.member_1] = vec3<f32>(bitcast<f32>(_e637), bitcast<f32>(_e642), bitcast<f32>(_e647));
                    phi_1109_ = true;
                    break;
                }
                default: {
                    phi_1109_ = bool();
                    break;
                }
            }
            let _e652 = phi_1109_;
            continue;
            continuing {
                phi_1062_ = _e627;
                break if !(_e652);
            }
        }
        let _e654 = local;
        let _e658 = global.member[(_e373 + 83u)];
        let _e663 = global.member[(_e373 + 84u)];
        let _e668 = global.member[(_e373 + 85u)];
        phi_1130_ = type_21(type_19(vec4<f32>(bitcast<f32>(_e381), bitcast<f32>(_e386), bitcast<f32>(_e391), bitcast<f32>(_e396)), vec4<f32>(bitcast<f32>(_e402), bitcast<f32>(_e407), bitcast<f32>(_e412), bitcast<f32>(_e417)), vec4<f32>(bitcast<f32>(_e423), bitcast<f32>(_e428), bitcast<f32>(_e433), bitcast<f32>(_e438)), vec4<f32>(bitcast<f32>(_e444), bitcast<f32>(_e449), bitcast<f32>(_e454), bitcast<f32>(_e459))), type_19(vec4<f32>(bitcast<f32>(_e466), bitcast<f32>(_e471), bitcast<f32>(_e476), bitcast<f32>(_e481)), vec4<f32>(bitcast<f32>(_e487), bitcast<f32>(_e492), bitcast<f32>(_e497), bitcast<f32>(_e502)), vec4<f32>(bitcast<f32>(_e508), bitcast<f32>(_e513), bitcast<f32>(_e518), bitcast<f32>(_e523)), vec4<f32>(bitcast<f32>(_e529), bitcast<f32>(_e534), bitcast<f32>(_e539), bitcast<f32>(_e544))), vec3<f32>(bitcast<f32>(_e551), bitcast<f32>(_e556), bitcast<f32>(_e561)), type_20(_e654, _e611, vec3<f32>(bitcast<f32>(_e658), bitcast<f32>(_e663), bitcast<f32>(_e668))));
    } else {
        phi_1130_ = type_21(type_19(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_19(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_20(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e674 = phi_1130_;
    let _e714 = fma(_e674.member.member_3.x, _e674.member_1.member.w, fma(_e674.member.member_2.x, _e674.member_1.member.z, fma(_e674.member.member.x, _e674.member_1.member.x, (_e674.member.member_1.x * _e674.member_1.member.y))));
    let _e715 = fma(_e674.member.member_3.y, _e674.member_1.member.w, fma(_e674.member.member_2.y, _e674.member_1.member.z, fma(_e674.member.member.y, _e674.member_1.member.x, (_e674.member.member_1.y * _e674.member_1.member.y))));
    let _e716 = fma(_e674.member.member_3.z, _e674.member_1.member.w, fma(_e674.member.member_2.z, _e674.member_1.member.z, fma(_e674.member.member.z, _e674.member_1.member.x, (_e674.member.member_1.z * _e674.member_1.member.y))));
    let _e717 = fma(_e674.member.member_3.w, _e674.member_1.member.w, fma(_e674.member.member_2.w, _e674.member_1.member.z, fma(_e674.member.member.w, _e674.member_1.member.x, (_e674.member.member_1.w * _e674.member_1.member.y))));
    let _e735 = fma(_e674.member.member_3.x, _e674.member_1.member_1.w, fma(_e674.member.member_2.x, _e674.member_1.member_1.z, fma(_e674.member.member.x, _e674.member_1.member_1.x, (_e674.member.member_1.x * _e674.member_1.member_1.y))));
    let _e736 = fma(_e674.member.member_3.y, _e674.member_1.member_1.w, fma(_e674.member.member_2.y, _e674.member_1.member_1.z, fma(_e674.member.member.y, _e674.member_1.member_1.x, (_e674.member.member_1.y * _e674.member_1.member_1.y))));
    let _e737 = fma(_e674.member.member_3.z, _e674.member_1.member_1.w, fma(_e674.member.member_2.z, _e674.member_1.member_1.z, fma(_e674.member.member.z, _e674.member_1.member_1.x, (_e674.member.member_1.z * _e674.member_1.member_1.y))));
    let _e738 = fma(_e674.member.member_3.w, _e674.member_1.member_1.w, fma(_e674.member.member_2.w, _e674.member_1.member_1.z, fma(_e674.member.member.w, _e674.member_1.member_1.x, (_e674.member.member_1.w * _e674.member_1.member_1.y))));
    let _e756 = fma(_e674.member.member_3.x, _e674.member_1.member_2.w, fma(_e674.member.member_2.x, _e674.member_1.member_2.z, fma(_e674.member.member.x, _e674.member_1.member_2.x, (_e674.member.member_1.x * _e674.member_1.member_2.y))));
    let _e757 = fma(_e674.member.member_3.y, _e674.member_1.member_2.w, fma(_e674.member.member_2.y, _e674.member_1.member_2.z, fma(_e674.member.member.y, _e674.member_1.member_2.x, (_e674.member.member_1.y * _e674.member_1.member_2.y))));
    let _e758 = fma(_e674.member.member_3.z, _e674.member_1.member_2.w, fma(_e674.member.member_2.z, _e674.member_1.member_2.z, fma(_e674.member.member.z, _e674.member_1.member_2.x, (_e674.member.member_1.z * _e674.member_1.member_2.y))));
    let _e759 = fma(_e674.member.member_3.w, _e674.member_1.member_2.w, fma(_e674.member.member_2.w, _e674.member_1.member_2.z, fma(_e674.member.member.w, _e674.member_1.member_2.x, (_e674.member.member_1.w * _e674.member_1.member_2.y))));
    let _e777 = fma(_e674.member.member_3.x, _e674.member_1.member_3.w, fma(_e674.member.member_2.x, _e674.member_1.member_3.z, fma(_e674.member.member.x, _e674.member_1.member_3.x, (_e674.member.member_1.x * _e674.member_1.member_3.y))));
    let _e778 = fma(_e674.member.member_3.y, _e674.member_1.member_3.w, fma(_e674.member.member_2.y, _e674.member_1.member_3.z, fma(_e674.member.member.y, _e674.member_1.member_3.x, (_e674.member.member_1.y * _e674.member_1.member_3.y))));
    let _e779 = fma(_e674.member.member_3.z, _e674.member_1.member_3.w, fma(_e674.member.member_2.z, _e674.member_1.member_3.z, fma(_e674.member.member.z, _e674.member_1.member_3.x, (_e674.member.member_1.z * _e674.member_1.member_3.y))));
    let _e780 = fma(_e674.member.member_3.w, _e674.member_1.member_3.w, fma(_e674.member.member_2.w, _e674.member_1.member_3.z, fma(_e674.member.member.w, _e674.member_1.member_3.x, (_e674.member.member_1.w * _e674.member_1.member_3.y))));
    global_2 = vec4<f32>((fma(fma(_e777, _e367.w, fma(_e756, _e367.z, fma(_e714, _e367.x, (_e735 * _e367.y)))), _e267.member.z, fma(fma(_e777, _e363.w, fma(_e756, _e363.z, fma(_e714, _e363.x, (_e735 * _e363.y)))), _e267.member.x, (fma(_e777, _e365.w, fma(_e756, _e365.z, fma(_e714, _e365.x, (_e735 * _e365.y)))) * _e267.member.y))) + (fma(_e756, _e328.member.z, fma(_e714, _e328.member.x, (_e735 * _e328.member.y))) + _e777)), (fma(fma(_e778, _e367.w, fma(_e757, _e367.z, fma(_e715, _e367.x, (_e736 * _e367.y)))), _e267.member.z, fma(fma(_e778, _e363.w, fma(_e757, _e363.z, fma(_e715, _e363.x, (_e736 * _e363.y)))), _e267.member.x, (fma(_e778, _e365.w, fma(_e757, _e365.z, fma(_e715, _e365.x, (_e736 * _e365.y)))) * _e267.member.y))) + (fma(_e757, _e328.member.z, fma(_e715, _e328.member.x, (_e736 * _e328.member.y))) + _e778)), (fma(fma(_e779, _e367.w, fma(_e758, _e367.z, fma(_e716, _e367.x, (_e737 * _e367.y)))), _e267.member.z, fma(fma(_e779, _e363.w, fma(_e758, _e363.z, fma(_e716, _e363.x, (_e737 * _e363.y)))), _e267.member.x, (fma(_e779, _e365.w, fma(_e758, _e365.z, fma(_e716, _e365.x, (_e737 * _e365.y)))) * _e267.member.y))) + (fma(_e758, _e328.member.z, fma(_e716, _e328.member.x, (_e737 * _e328.member.y))) + _e779)), (fma(fma(_e780, _e367.w, fma(_e759, _e367.z, fma(_e717, _e367.x, (_e738 * _e367.y)))), _e267.member.z, fma(fma(_e780, _e363.w, fma(_e759, _e363.z, fma(_e717, _e363.x, (_e738 * _e363.y)))), _e267.member.x, (fma(_e780, _e365.w, fma(_e759, _e365.z, fma(_e717, _e365.x, (_e738 * _e365.y)))) * _e267.member.y))) + (fma(_e759, _e328.member.z, fma(_e717, _e328.member.x, (_e738 * _e328.member.y))) + _e780)));
    return;
}

@vertex 
fn tutorialtutorial_slabbed_renderlet(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_3 = param;
    global_1 = param_1;
    function();
    let _e7 = global_2.y;
    global_2.y = -(_e7);
    let _e9 = global_4;
    let _e10 = global_2;
    return VertexOutput(_e9, _e10);
}
