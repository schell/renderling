struct type_2 {
    member: array<u32>,
}

struct type_12 {
    member: u32,
    member_1: u32,
}

struct type_17 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_18 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
    member_2: vec3<f32>,
}

struct type_19 {
    member: type_17,
    member_1: type_17,
    member_2: vec3<f32>,
    member_3: type_18,
}

struct type_24 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_26 {
    member: array<type_24>,
}

struct type_31 {
    member: vec2<u32>,
    member_1: vec2<u32>,
    member_2: type_12,
    member_3: u32,
    member_4: bool,
    member_5: bool,
    member_6: bool,
    member_7: bool,
}

struct type_32 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_33 {
    member: bool,
    member_1: u32,
}

@group(0) @binding(0) 
var<storage> global: type_2;
@group(0) @binding(1) 
var<storage> global_1: type_2;
@group(0) @binding(2) 
var<storage, read_write> global_2: type_26;
var<private> global_3: vec3<u32>;

fn function() {
    var local: array<vec4<f32>, 6>;
    var local_1: array<vec3<f32>, 8>;
    var local_2: array<f32, 3>;
    var local_3: array<f32, 3>;
    var local_4: array<f32, 3>;
    var local_5: array<f32, 3>;
    var local_6: array<vec3<f32>, 8>;
    var local_7: array<vec4<f32>, 6>;
    var phi_631_: u32;
    var phi_667_: type_31;
    var phi_2144_: bool;
    var phi_832_: type_12;
    var phi_833_: type_12;
    var phi_856_: type_12;
    var phi_883_: bool;
    var phi_889_: type_12;
    var phi_890_: type_12;
    var phi_913_: type_12;
    var phi_936_: bool;
    var phi_957_: type_19;
    var phi_2176_: bool;
    var phi_1008_: type_32;
    var phi_1140_: type_12;
    var phi_1141_: type_12;
    var phi_1164_: type_12;
    var phi_1208_: bool;
    var phi_1212_: bool;
    var phi_1213_: bool;
    var phi_2740_: bool;
    var phi_2045_: bool;
    var phi_2767_: bool;
    var phi_1220_: type_12;
    var phi_1221_: type_12;
    var phi_1244_: type_12;
    var phi_1254_: type_12;
    var phi_1257_: i32;
    var phi_1255_: type_12;
    var phi_1280_: type_12;
    var phi_1321_: i32;
    var phi_1258_: i32;
    var phi_1322_: bool;
    var phi_2761_: bool;
    var local_8: i32;
    var phi_1329_: type_12;
    var phi_1332_: i32;
    var phi_1330_: type_12;
    var phi_1355_: type_12;
    var phi_1396_: i32;
    var phi_1333_: i32;
    var phi_1397_: bool;
    var phi_2768_: bool;
    var local_9: i32;
    var phi_2775_: bool;
    var phi_1403_: bool;
    var phi_1404_: bool;
    var phi_2774_: bool;
    var phi_1405_: bool;
    var phi_1406_: bool;
    var phi_1407_: bool;
    var phi_1408_: bool;
    var phi_2773_: bool;
    var phi_2048_: bool;
    var phi_2047_: bool;
    var phi_2046_: bool;
    var phi_1431_: type_33;
    var phi_1432_: type_33;
    var local_10: u32;
    var phi_1438_: type_33;
    var phi_1439_: type_33;
    var phi_1703_: u32;
    var phi_2559_: bool;
    var phi_1721_: type_12;
    var phi_2585_: u32;

    switch bitcast<i32>(0u) {
        default: {
            let _e93 = arrayLength((&global.member));
            let _e95 = arrayLength((&global_1.member));
            let _e98 = global_3;
            if (_e98.x >= arrayLength((&global_2.member))) {
            } else {
                let _e104 = global_2.member[_e98.x].member_3;
                let _e107 = global.member[_e104];
                let _e112 = global.member[(_e104 + 2u)];
                let _e116 = global.member[(_e104 + 3u)];
                let _e117 = bitcast<f32>(_e116);
                let _e121 = global.member[(_e104 + 4u)];
                let _e122 = bitcast<f32>(_e121);
                let _e126 = global.member[(_e104 + 5u)];
                let _e127 = bitcast<f32>(_e126);
                let _e131 = global.member[(_e104 + 6u)];
                let _e132 = bitcast<f32>(_e131);
                let _e136 = global.member[(_e104 + 7u)];
                let _e140 = global.member[(_e104 + 8u)];
                let _e144 = global.member[(_e104 + 9u)];
                let _e148 = global.member[(_e104 + 10u)];
                global_2.member[_e98.x].member = select(_e140, _e112, (_e136 == 4294967295u));
                global_2.member[_e98.x].member_1 = select(0u, 1u, (_e107 == 1u));
                if (_e132 == 0f) {
                } else {
                    if select(false, true, (_e93 >= 11u)) {
                        let _e163 = global.member[0u];
                        let _e166 = global.member[1u];
                        let _e170 = global.member[2u];
                        let _e173 = global.member[3u];
                        let _e177 = global.member[4u];
                        switch bitcast<i32>(_e177) {
                            case 0: {
                                phi_631_ = 0u;
                                break;
                            }
                            case 1: {
                                phi_631_ = 1u;
                                break;
                            }
                            case 2: {
                                phi_631_ = 2u;
                                break;
                            }
                            case 3: {
                                phi_631_ = 3u;
                                break;
                            }
                            case 4: {
                                phi_631_ = 4u;
                                break;
                            }
                            case 5: {
                                phi_631_ = 5u;
                                break;
                            }
                            case 6: {
                                phi_631_ = 6u;
                                break;
                            }
                            case 7: {
                                phi_631_ = 7u;
                                break;
                            }
                            case 8: {
                                phi_631_ = 8u;
                                break;
                            }
                            case 9: {
                                phi_631_ = 9u;
                                break;
                            }
                            case 10: {
                                phi_631_ = 10u;
                                break;
                            }
                            case 11: {
                                phi_631_ = 11u;
                                break;
                            }
                            case 12: {
                                phi_631_ = 12u;
                                break;
                            }
                            case 13: {
                                phi_631_ = 13u;
                                break;
                            }
                            case 14: {
                                phi_631_ = 14u;
                                break;
                            }
                            case 15: {
                                phi_631_ = 15u;
                                break;
                            }
                            case 16: {
                                phi_631_ = 16u;
                                break;
                            }
                            case 17: {
                                phi_631_ = 17u;
                                break;
                            }
                            case 18: {
                                phi_631_ = 18u;
                                break;
                            }
                            case 19: {
                                phi_631_ = 19u;
                                break;
                            }
                            default: {
                                phi_631_ = 0u;
                                break;
                            }
                        }
                        let _e180 = phi_631_;
                        let _e183 = global.member[5u];
                        let _e187 = global.member[6u];
                        let _e191 = global.member[7u];
                        let _e195 = global.member[8u];
                        let _e199 = global.member[9u];
                        let _e202 = global.member[10u];
                        phi_667_ = type_31(vec2<u32>(_e163, _e166), vec2<u32>(_e170, _e173), type_12(_e199, _e202), _e180, (_e183 == 1u), (_e187 == 1u), (_e191 == 1u), (_e195 == 1u));
                    } else {
                        phi_667_ = type_31(vec2<u32>(0u, 0u), vec2<u32>(1u, 1u), type_12(4294967295u, 0u), 0u, true, true, true, false);
                    }
                    let _e206 = phi_667_;
                    if _e206.member_6 {
                        if (_e93 >= 86u) {
                            phi_2144_ = (_e144 <= (_e93 - 86u));
                        } else {
                            phi_2144_ = false;
                        }
                        let _e212 = phi_2144_;
                        if _e212 {
                            let _e215 = global.member[_e144];
                            let _e220 = global.member[(_e144 + 1u)];
                            let _e225 = global.member[(_e144 + 2u)];
                            let _e230 = global.member[(_e144 + 3u)];
                            let _e236 = global.member[(_e144 + 4u)];
                            let _e241 = global.member[(_e144 + 5u)];
                            let _e246 = global.member[(_e144 + 6u)];
                            let _e251 = global.member[(_e144 + 7u)];
                            let _e257 = global.member[(_e144 + 8u)];
                            let _e262 = global.member[(_e144 + 9u)];
                            let _e267 = global.member[(_e144 + 10u)];
                            let _e272 = global.member[(_e144 + 11u)];
                            let _e278 = global.member[(_e144 + 12u)];
                            let _e283 = global.member[(_e144 + 13u)];
                            let _e288 = global.member[(_e144 + 14u)];
                            let _e293 = global.member[(_e144 + 15u)];
                            let _e300 = global.member[(_e144 + 16u)];
                            let _e305 = global.member[(_e144 + 17u)];
                            let _e310 = global.member[(_e144 + 18u)];
                            let _e315 = global.member[(_e144 + 19u)];
                            let _e321 = global.member[(_e144 + 20u)];
                            let _e326 = global.member[(_e144 + 21u)];
                            let _e331 = global.member[(_e144 + 22u)];
                            let _e336 = global.member[(_e144 + 23u)];
                            let _e342 = global.member[(_e144 + 24u)];
                            let _e347 = global.member[(_e144 + 25u)];
                            let _e352 = global.member[(_e144 + 26u)];
                            let _e357 = global.member[(_e144 + 27u)];
                            let _e363 = global.member[(_e144 + 28u)];
                            let _e368 = global.member[(_e144 + 29u)];
                            let _e373 = global.member[(_e144 + 30u)];
                            let _e378 = global.member[(_e144 + 31u)];
                            let _e385 = global.member[(_e144 + 32u)];
                            let _e390 = global.member[(_e144 + 33u)];
                            let _e395 = global.member[(_e144 + 34u)];
                            local_7 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            phi_832_ = type_12(0u, 6u);
                            loop {
                                let _e400 = phi_832_;
                                if (_e400.member < _e400.member_1) {
                                    phi_833_ = type_12((_e400.member + 1u), _e400.member_1);
                                    phi_856_ = type_12(1u, _e400.member);
                                } else {
                                    phi_833_ = _e400;
                                    phi_856_ = type_12(0u, type_12().member_1);
                                }
                                let _e413 = phi_833_;
                                let _e415 = phi_856_;
                                switch bitcast<i32>(_e415.member) {
                                    case 0: {
                                        phi_883_ = false;
                                        break;
                                    }
                                    case 1: {
                                        let _e420 = ((_e144 + 35u) + (_e415.member_1 * 4u));
                                        let _e423 = global.member[_e420];
                                        let _e428 = global.member[(_e420 + 1u)];
                                        let _e433 = global.member[(_e420 + 2u)];
                                        let _e438 = global.member[(_e420 + 3u)];
                                        local_7[_e415.member_1] = vec4<f32>(bitcast<f32>(_e423), bitcast<f32>(_e428), bitcast<f32>(_e433), bitcast<f32>(_e438));
                                        phi_883_ = true;
                                        break;
                                    }
                                    default: {
                                        phi_883_ = bool();
                                        break;
                                    }
                                }
                                let _e443 = phi_883_;
                                continue;
                                continuing {
                                    phi_832_ = _e413;
                                    break if !(_e443);
                                }
                            }
                            let _e445 = local_7;
                            local_6 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            phi_889_ = type_12(0u, 8u);
                            loop {
                                let _e448 = phi_889_;
                                if (_e448.member < _e448.member_1) {
                                    phi_890_ = type_12((_e448.member + 1u), _e448.member_1);
                                    phi_913_ = type_12(1u, _e448.member);
                                } else {
                                    phi_890_ = _e448;
                                    phi_913_ = type_12(0u, type_12().member_1);
                                }
                                let _e461 = phi_890_;
                                let _e463 = phi_913_;
                                switch bitcast<i32>(_e463.member) {
                                    case 0: {
                                        phi_936_ = false;
                                        break;
                                    }
                                    case 1: {
                                        let _e468 = ((_e144 + 59u) + (_e463.member_1 * 3u));
                                        let _e471 = global.member[_e468];
                                        let _e476 = global.member[(_e468 + 1u)];
                                        let _e481 = global.member[(_e468 + 2u)];
                                        local_6[_e463.member_1] = vec3<f32>(bitcast<f32>(_e471), bitcast<f32>(_e476), bitcast<f32>(_e481));
                                        phi_936_ = true;
                                        break;
                                    }
                                    default: {
                                        phi_936_ = bool();
                                        break;
                                    }
                                }
                                let _e486 = phi_936_;
                                continue;
                                continuing {
                                    phi_889_ = _e461;
                                    break if !(_e486);
                                }
                            }
                            let _e488 = local_6;
                            let _e492 = global.member[(_e144 + 83u)];
                            let _e497 = global.member[(_e144 + 84u)];
                            let _e502 = global.member[(_e144 + 85u)];
                            phi_957_ = type_19(type_17(vec4<f32>(bitcast<f32>(_e215), bitcast<f32>(_e220), bitcast<f32>(_e225), bitcast<f32>(_e230)), vec4<f32>(bitcast<f32>(_e236), bitcast<f32>(_e241), bitcast<f32>(_e246), bitcast<f32>(_e251)), vec4<f32>(bitcast<f32>(_e257), bitcast<f32>(_e262), bitcast<f32>(_e267), bitcast<f32>(_e272)), vec4<f32>(bitcast<f32>(_e278), bitcast<f32>(_e283), bitcast<f32>(_e288), bitcast<f32>(_e293))), type_17(vec4<f32>(bitcast<f32>(_e300), bitcast<f32>(_e305), bitcast<f32>(_e310), bitcast<f32>(_e315)), vec4<f32>(bitcast<f32>(_e321), bitcast<f32>(_e326), bitcast<f32>(_e331), bitcast<f32>(_e336)), vec4<f32>(bitcast<f32>(_e342), bitcast<f32>(_e347), bitcast<f32>(_e352), bitcast<f32>(_e357)), vec4<f32>(bitcast<f32>(_e363), bitcast<f32>(_e368), bitcast<f32>(_e373), bitcast<f32>(_e378))), vec3<f32>(bitcast<f32>(_e385), bitcast<f32>(_e390), bitcast<f32>(_e395)), type_18(_e488, _e445, vec3<f32>(bitcast<f32>(_e492), bitcast<f32>(_e497), bitcast<f32>(_e502))));
                        } else {
                            phi_957_ = type_19(type_17(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_17(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_18(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
                        }
                        let _e508 = phi_957_;
                        if (_e93 >= 10u) {
                            phi_2176_ = (_e148 <= (_e93 - 10u));
                        } else {
                            phi_2176_ = false;
                        }
                        let _e518 = phi_2176_;
                        if _e518 {
                            let _e521 = global.member[_e148];
                            let _e526 = global.member[(_e148 + 1u)];
                            let _e531 = global.member[(_e148 + 2u)];
                            let _e537 = global.member[(_e148 + 3u)];
                            let _e542 = global.member[(_e148 + 4u)];
                            let _e547 = global.member[(_e148 + 5u)];
                            let _e552 = global.member[(_e148 + 6u)];
                            let _e558 = global.member[(_e148 + 7u)];
                            let _e563 = global.member[(_e148 + 8u)];
                            let _e568 = global.member[(_e148 + 9u)];
                            phi_1008_ = type_32(vec3<f32>(bitcast<f32>(_e521), bitcast<f32>(_e526), bitcast<f32>(_e531)), vec4<f32>(bitcast<f32>(_e537), bitcast<f32>(_e542), bitcast<f32>(_e547), bitcast<f32>(_e552)), vec3<f32>(bitcast<f32>(_e558), bitcast<f32>(_e563), bitcast<f32>(_e568)));
                        } else {
                            phi_1008_ = type_32(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                        }
                        let _e573 = phi_1008_;
                        let _e581 = (_e573.member_1.x + _e573.member_1.x);
                        let _e582 = (_e573.member_1.y + _e573.member_1.y);
                        let _e583 = (_e573.member_1.z + _e573.member_1.z);
                        let _e585 = (_e573.member_1.z * _e583);
                        let _e586 = (_e573.member_1.w * _e581);
                        let _e587 = (_e573.member_1.w * _e582);
                        let _e588 = (_e573.member_1.w * _e583);
                        let _e608 = (vec4<f32>((1f - fma(_e573.member_1.y, _e582, _e585)), fma(_e573.member_1.x, _e582, _e588), fma(_e573.member_1.x, _e583, -(_e587)), 0f) * _e573.member_2.x);
                        let _e610 = (vec4<f32>(fma(_e573.member_1.x, _e582, -(_e588)), (1f - fma(_e573.member_1.x, _e581, _e585)), fma(_e573.member_1.y, _e583, _e586), 0f) * _e573.member_2.y);
                        let _e612 = (vec4<f32>(fma(_e573.member_1.x, _e583, _e587), fma(_e573.member_1.y, _e583, -(_e586)), (1f - fma(_e573.member_1.x, _e581, (_e573.member_1.y * _e582))), 0f) * _e573.member_2.z);
                        let _e634 = (_e573.member.x + fma(_e612.x, _e127, fma(_e610.x, _e122, (_e608.x * _e117))));
                        let _e635 = (_e573.member.y + fma(_e612.y, _e127, fma(_e610.y, _e122, (_e608.y * _e117))));
                        let _e636 = (_e573.member.z + fma(_e612.z, _e127, fma(_e610.z, _e122, (_e608.z * _e117))));
                        let _e637 = vec3<f32>(_e634, _e635, _e636);
                        let _e640 = (max(_e573.member_2.x, max(_e573.member_2.y, _e573.member_2.z)) * _e132);
                        let _e642 = sqrt((_e640 * _e640));
                        local_1 = _e508.member_3.member;
                        local = _e508.member_3.member_1;
                        let _e645 = local[0u][0u];
                        let _e648 = local[0u][1u];
                        let _e653 = local[0u][2u];
                        let _e657 = local[0u][3u];
                        let _e659 = -(_e642);
                        if ((fma(_e653, _e636, fma(_e645, _e634, (_e648 * _e635))) + _e657) < _e659) {
                            phi_1439_ = type_33(true, 0u);
                        } else {
                            phi_1140_ = type_12(0u, 6u);
                            loop {
                                let _e662 = phi_1140_;
                                if (_e662.member < _e662.member_1) {
                                    phi_1141_ = type_12((_e662.member + 1u), _e662.member_1);
                                    phi_1164_ = type_12(1u, _e662.member);
                                } else {
                                    phi_1141_ = _e662;
                                    phi_1164_ = type_12(0u, type_12().member_1);
                                }
                                let _e675 = phi_1141_;
                                let _e677 = phi_1164_;
                                local_10 = _e677.member_1;
                                switch bitcast<i32>(_e677.member) {
                                    case 0: {
                                        phi_1212_ = false;
                                        phi_1213_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if (_e677.member_1 != 0u) {
                                            if (_e677.member_1 < 6u) {
                                            } else {
                                                phi_2740_ = true;
                                                phi_2045_ = bool();
                                                break;
                                            }
                                            let _e685 = local[_e677.member_1][0u];
                                            let _e688 = local[_e677.member_1][1u];
                                            let _e693 = local[_e677.member_1][2u];
                                            let _e697 = local[_e677.member_1][3u];
                                            phi_1208_ = select(true, false, ((fma(_e693, _e636, fma(_e685, _e634, (_e688 * _e635))) + _e697) < _e659));
                                        } else {
                                            phi_1208_ = true;
                                        }
                                        let _e702 = phi_1208_;
                                        phi_1212_ = _e702;
                                        phi_1213_ = false;
                                        break;
                                    }
                                    default: {
                                        phi_1212_ = bool();
                                        phi_1213_ = bool();
                                        break;
                                    }
                                }
                                let _e704 = phi_1212_;
                                let _e706 = phi_1213_;
                                continue;
                                continuing {
                                    phi_1140_ = _e675;
                                    phi_2740_ = false;
                                    phi_2045_ = _e706;
                                    break if !(_e704);
                                }
                            }
                            let _e709 = phi_2740_;
                            let _e711 = phi_2045_;
                            if _e709 {
                                break;
                            }
                            if _e711 {
                                let _e712 = vec3(_e642);
                                let _e713 = (_e637 - _e712);
                                let _e714 = (_e637 + _e712);
                                phi_2767_ = _e709;
                                phi_1220_ = type_12(0u, 3u);
                                loop {
                                    let _e716 = phi_2767_;
                                    let _e718 = phi_1220_;
                                    if (_e718.member < _e718.member_1) {
                                        phi_1221_ = type_12((_e718.member + 1u), _e718.member_1);
                                        phi_1244_ = type_12(1u, _e718.member);
                                    } else {
                                        phi_1221_ = _e718;
                                        phi_1244_ = type_12(0u, type_12().member_1);
                                    }
                                    let _e731 = phi_1221_;
                                    let _e733 = phi_1244_;
                                    switch bitcast<i32>(_e733.member) {
                                        case 0: {
                                            phi_2774_ = _e716;
                                            phi_1405_ = false;
                                            phi_1406_ = true;
                                            phi_1407_ = false;
                                            phi_1408_ = false;
                                            break;
                                        }
                                        case 1: {
                                            phi_1254_ = type_12(0u, 8u);
                                            phi_1257_ = 0i;
                                            loop {
                                                let _e738 = phi_1254_;
                                                let _e740 = phi_1257_;
                                                local_8 = _e740;
                                                if (_e738.member < _e738.member_1) {
                                                    phi_1255_ = type_12((_e738.member + 1u), _e738.member_1);
                                                    phi_1280_ = type_12(1u, _e738.member);
                                                } else {
                                                    phi_1255_ = _e738;
                                                    phi_1280_ = type_12(0u, type_12().member_1);
                                                }
                                                let _e753 = phi_1255_;
                                                let _e755 = phi_1280_;
                                                switch bitcast<i32>(_e755.member) {
                                                    case 0: {
                                                        phi_1258_ = i32();
                                                        phi_1322_ = false;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if (_e755.member_1 < 8u) {
                                                        } else {
                                                            phi_2761_ = true;
                                                            break;
                                                        }
                                                        let _e762 = local_1[_e755.member_1][0u];
                                                        let _e765 = local_1[_e755.member_1][1u];
                                                        let _e768 = local_1[_e755.member_1][2u];
                                                        local_2 = array<f32, 3>(_e762, _e765, _e768);
                                                        let _e770 = (_e733.member_1 < 3u);
                                                        if _e770 {
                                                        } else {
                                                            phi_2761_ = true;
                                                            break;
                                                        }
                                                        let _e772 = local_2[_e733.member_1];
                                                        local_3 = array<f32, 3>(_e713.x, _e713.y, _e713.z);
                                                        if _e770 {
                                                        } else {
                                                            phi_2761_ = true;
                                                            break;
                                                        }
                                                        let _e778 = local_3[_e733.member_1];
                                                        if (_e772 < _e778) {
                                                            phi_1321_ = (_e740 + 1i);
                                                        } else {
                                                            phi_1321_ = _e740;
                                                        }
                                                        let _e782 = phi_1321_;
                                                        phi_1258_ = _e782;
                                                        phi_1322_ = true;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_1258_ = i32();
                                                        phi_1322_ = bool();
                                                        break;
                                                    }
                                                }
                                                let _e784 = phi_1258_;
                                                let _e786 = phi_1322_;
                                                continue;
                                                continuing {
                                                    phi_1254_ = _e753;
                                                    phi_1257_ = _e784;
                                                    phi_2761_ = _e716;
                                                    break if !(_e786);
                                                }
                                            }
                                            let _e789 = phi_2761_;
                                            phi_2773_ = _e789;
                                            phi_2048_ = bool();
                                            phi_2047_ = bool();
                                            phi_2046_ = bool();
                                            if _e789 {
                                                break;
                                            }
                                            let _e791 = local_8;
                                            let _e792 = (_e791 == 8i);
                                            if _e792 {
                                                phi_2775_ = _e789;
                                                phi_1403_ = false;
                                                phi_1404_ = false;
                                            } else {
                                                phi_1329_ = type_12(0u, 8u);
                                                phi_1332_ = 0i;
                                                loop {
                                                    let _e794 = phi_1329_;
                                                    let _e796 = phi_1332_;
                                                    local_9 = _e796;
                                                    if (_e794.member < _e794.member_1) {
                                                        phi_1330_ = type_12((_e794.member + 1u), _e794.member_1);
                                                        phi_1355_ = type_12(1u, _e794.member);
                                                    } else {
                                                        phi_1330_ = _e794;
                                                        phi_1355_ = type_12(0u, type_12().member_1);
                                                    }
                                                    let _e809 = phi_1330_;
                                                    let _e811 = phi_1355_;
                                                    switch bitcast<i32>(_e811.member) {
                                                        case 0: {
                                                            phi_1333_ = i32();
                                                            phi_1397_ = false;
                                                            break;
                                                        }
                                                        case 1: {
                                                            if (_e811.member_1 < 8u) {
                                                            } else {
                                                                phi_2768_ = true;
                                                                break;
                                                            }
                                                            let _e818 = local_1[_e811.member_1][0u];
                                                            let _e821 = local_1[_e811.member_1][1u];
                                                            let _e824 = local_1[_e811.member_1][2u];
                                                            local_4 = array<f32, 3>(_e818, _e821, _e824);
                                                            let _e826 = (_e733.member_1 < 3u);
                                                            if _e826 {
                                                            } else {
                                                                phi_2768_ = true;
                                                                break;
                                                            }
                                                            let _e828 = local_4[_e733.member_1];
                                                            local_5 = array<f32, 3>(_e714.x, _e714.y, _e714.z);
                                                            if _e826 {
                                                            } else {
                                                                phi_2768_ = true;
                                                                break;
                                                            }
                                                            let _e834 = local_5[_e733.member_1];
                                                            if (_e828 > _e834) {
                                                                phi_1396_ = (_e796 + 1i);
                                                            } else {
                                                                phi_1396_ = _e796;
                                                            }
                                                            let _e838 = phi_1396_;
                                                            phi_1333_ = _e838;
                                                            phi_1397_ = true;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_1333_ = i32();
                                                            phi_1397_ = bool();
                                                            break;
                                                        }
                                                    }
                                                    let _e840 = phi_1333_;
                                                    let _e842 = phi_1397_;
                                                    continue;
                                                    continuing {
                                                        phi_1329_ = _e809;
                                                        phi_1332_ = _e840;
                                                        phi_2768_ = _e789;
                                                        break if !(_e842);
                                                    }
                                                }
                                                let _e845 = phi_2768_;
                                                phi_2773_ = _e845;
                                                phi_2048_ = bool();
                                                phi_2047_ = bool();
                                                phi_2046_ = bool();
                                                if _e845 {
                                                    break;
                                                }
                                                let _e847 = local_9;
                                                let _e848 = (_e847 == 8i);
                                                phi_2775_ = _e845;
                                                phi_1403_ = select(true, false, _e848);
                                                phi_1404_ = _e848;
                                            }
                                            let _e851 = phi_2775_;
                                            let _e853 = phi_1403_;
                                            let _e855 = phi_1404_;
                                            phi_2774_ = _e851;
                                            phi_1405_ = _e853;
                                            phi_1406_ = false;
                                            phi_1407_ = _e792;
                                            phi_1408_ = _e855;
                                            break;
                                        }
                                        default: {
                                            phi_2774_ = _e716;
                                            phi_1405_ = bool();
                                            phi_1406_ = bool();
                                            phi_1407_ = bool();
                                            phi_1408_ = bool();
                                            break;
                                        }
                                    }
                                    let _e857 = phi_2774_;
                                    let _e859 = phi_1405_;
                                    let _e861 = phi_1406_;
                                    let _e863 = phi_1407_;
                                    let _e865 = phi_1408_;
                                    continue;
                                    continuing {
                                        phi_2767_ = _e857;
                                        phi_1220_ = _e731;
                                        phi_2773_ = _e857;
                                        phi_2048_ = _e865;
                                        phi_2047_ = _e863;
                                        phi_2046_ = _e861;
                                        break if !(_e859);
                                    }
                                }
                                let _e868 = phi_2773_;
                                let _e870 = phi_2048_;
                                let _e872 = phi_2047_;
                                let _e874 = phi_2046_;
                                if _e868 {
                                    break;
                                }
                                let _e875 = select(_e872, false, _e874);
                                if select(true, false, select(_e875, true, select(select(_e870, false, _e874), false, _e875))) {
                                    phi_1431_ = type_33(false, 0u);
                                } else {
                                    phi_1431_ = type_33(true, 0u);
                                }
                                let _e881 = phi_1431_;
                                phi_1432_ = _e881;
                            } else {
                                phi_1432_ = type_33();
                            }
                            let _e883 = phi_1432_;
                            if select(true, false, _e711) {
                                let _e886 = local_10;
                                phi_1438_ = type_33(true, _e886);
                            } else {
                                phi_1438_ = _e883;
                            }
                            let _e889 = phi_1438_;
                            phi_1439_ = _e889;
                        }
                        let _e891 = phi_1439_;
                        if (_e891.member != true) {
                            global_2.member[_e98.x].member_1 = 1u;
                            if _e206.member_7 {
                                let _e897 = global_1.member[0u];
                                let _e900 = global_1.member[1u];
                                let _e903 = global_1.member[3u];
                                let _e906 = global_1.member[4u];
                                let _e907 = f32(_e897);
                                let _e949 = fma(_e508.member.member_3.z, _e508.member_1.member.w, fma(_e508.member.member_2.z, _e508.member_1.member.z, fma(_e508.member.member.z, _e508.member_1.member.x, (_e508.member.member_1.z * _e508.member_1.member.y))));
                                let _e950 = fma(_e508.member.member_3.w, _e508.member_1.member.w, fma(_e508.member.member_2.w, _e508.member_1.member.z, fma(_e508.member.member.w, _e508.member_1.member.x, (_e508.member.member_1.w * _e508.member_1.member.y))));
                                let _e970 = fma(_e508.member.member_3.z, _e508.member_1.member_1.w, fma(_e508.member.member_2.z, _e508.member_1.member_1.z, fma(_e508.member.member.z, _e508.member_1.member_1.x, (_e508.member.member_1.z * _e508.member_1.member_1.y))));
                                let _e971 = fma(_e508.member.member_3.w, _e508.member_1.member_1.w, fma(_e508.member.member_2.w, _e508.member_1.member_1.z, fma(_e508.member.member.w, _e508.member_1.member_1.x, (_e508.member.member_1.w * _e508.member_1.member_1.y))));
                                let _e991 = fma(_e508.member.member_3.z, _e508.member_1.member_2.w, fma(_e508.member.member_2.z, _e508.member_1.member_2.z, fma(_e508.member.member.z, _e508.member_1.member_2.x, (_e508.member.member_1.z * _e508.member_1.member_2.y))));
                                let _e992 = fma(_e508.member.member_3.w, _e508.member_1.member_2.w, fma(_e508.member.member_2.w, _e508.member_1.member_2.z, fma(_e508.member.member.w, _e508.member_1.member_2.x, (_e508.member.member_1.w * _e508.member_1.member_2.y))));
                                let _e1012 = fma(_e508.member.member_3.z, _e508.member_1.member_3.w, fma(_e508.member.member_2.z, _e508.member_1.member_3.z, fma(_e508.member.member.z, _e508.member_1.member_3.x, (_e508.member.member_1.z * _e508.member_1.member_3.y))));
                                let _e1013 = fma(_e508.member.member_3.w, _e508.member_1.member_3.w, fma(_e508.member.member_2.w, _e508.member_1.member_3.z, fma(_e508.member.member.w, _e508.member_1.member_3.x, (_e508.member.member_1.w * _e508.member_1.member_3.y))));
                                let _e1025 = (fma(_e992, _e636, fma(_e950, _e634, (_e971 * _e635))) + _e1013);
                                let _e1030 = fma(_e642, _e508.member_3.member_1[5].x, _e634);
                                let _e1031 = fma(_e642, _e508.member_3.member_1[5].y, _e635);
                                let _e1032 = fma(_e642, _e508.member_3.member_1[5].z, _e636);
                                let _e1046 = fma(_e642, _e508.member_3.member_1[0].x, _e634);
                                let _e1047 = fma(_e642, _e508.member_3.member_1[0].y, _e635);
                                let _e1048 = fma(_e642, _e508.member_3.member_1[0].z, _e636);
                                let _e1065 = (vec2<f32>(((((fma(fma(_e508.member.member_3.x, _e508.member_1.member_2.w, fma(_e508.member.member_2.x, _e508.member_1.member_2.z, fma(_e508.member.member.x, _e508.member_1.member_2.x, (_e508.member.member_1.x * _e508.member_1.member_2.y)))), _e636, fma(fma(_e508.member.member_3.x, _e508.member_1.member.w, fma(_e508.member.member_2.x, _e508.member_1.member.z, fma(_e508.member.member.x, _e508.member_1.member.x, (_e508.member.member_1.x * _e508.member_1.member.y)))), _e634, (fma(_e508.member.member_3.x, _e508.member_1.member_1.w, fma(_e508.member.member_2.x, _e508.member_1.member_1.z, fma(_e508.member.member.x, _e508.member_1.member_1.x, (_e508.member.member_1.x * _e508.member_1.member_1.y)))) * _e635))) + fma(_e508.member.member_3.x, _e508.member_1.member_3.w, fma(_e508.member.member_2.x, _e508.member_1.member_3.z, fma(_e508.member.member.x, _e508.member_1.member_3.x, (_e508.member.member_1.x * _e508.member_1.member_3.y))))) / _e1025) + 1f) * 0.5f), fma((-1f - ((fma(fma(_e508.member.member_3.y, _e508.member_1.member_2.w, fma(_e508.member.member_2.y, _e508.member_1.member_2.z, fma(_e508.member.member.y, _e508.member_1.member_2.x, (_e508.member.member_1.y * _e508.member_1.member_2.y)))), _e636, fma(fma(_e508.member.member_3.y, _e508.member_1.member.w, fma(_e508.member.member_2.y, _e508.member_1.member.z, fma(_e508.member.member.y, _e508.member_1.member.x, (_e508.member.member_1.y * _e508.member_1.member.y)))), _e634, (fma(_e508.member.member_3.y, _e508.member_1.member_1.w, fma(_e508.member.member_2.y, _e508.member_1.member_1.z, fma(_e508.member.member.y, _e508.member_1.member_1.x, (_e508.member.member_1.y * _e508.member_1.member_1.y)))) * _e635))) + fma(_e508.member.member_3.y, _e508.member_1.member_3.w, fma(_e508.member.member_2.y, _e508.member_1.member_3.z, fma(_e508.member.member.y, _e508.member_1.member_3.x, (_e508.member.member_1.y * _e508.member_1.member_3.y))))) / _e1025)), 0.5f, 1f)) * vec2<f32>(_e907, f32(_e900)));
                                let _e1066 = (_e642 / _e1025);
                                let _e1068 = -(_e907);
                                let _e1072 = vec3<f32>(fma(_e1068, _e1066, _e1065.x), fma(_e1068, _e1066, _e1065.y), ((_e1012 + fma(_e991, _e1032, fma(_e970, _e1031, (_e949 * _e1030)))) / (_e1013 + fma(_e992, _e1032, fma(_e971, _e1031, (_e950 * _e1030))))));
                                let _e1075 = vec3<f32>(fma(_e907, _e1066, _e1065.x), fma(_e907, _e1066, _e1065.y), ((_e1012 + fma(_e991, _e1048, fma(_e970, _e1047, (_e949 * _e1046)))) / (_e1013 + fma(_e992, _e1048, fma(_e971, _e1047, (_e950 * _e1046))))));
                                let _e1076 = min(_e1072, _e1075);
                                let _e1077 = max(_e1072, _e1075);
                                let _e1082 = (_e1077.x - _e1076.x);
                                let _e1083 = (_e1077.y - _e1076.y);
                                let _e1087 = floor(log2(select(_e1083, _e1082, (_e1082 > _e1083))));
                                let _e1092 = select(select(u32(_e1087), 0u, (_e1087 < 0f)), 4294967295u, (_e1087 > 4294967000f));
                                let _e1093 = (_e906 - 1u);
                                let _e1095 = select(_e1092, _e1093, (_e1092 > _e1093));
                                let _e1101 = round(((_e1076.x + _e1077.x) * 0.5f));
                                let _e1107 = (_e1095 & 31u);
                                let _e1110 = round(((_e1076.y + _e1077.y) * 0.5f));
                                if (_e1095 >= _e906) {
                                    phi_1703_ = 4294967295u;
                                } else {
                                    phi_1703_ = (_e903 + (2u * _e1095));
                                }
                                let _e1122 = phi_1703_;
                                if (_e95 >= 2u) {
                                    phi_2559_ = (_e1122 <= (_e95 - 2u));
                                } else {
                                    phi_2559_ = false;
                                }
                                let _e1127 = phi_2559_;
                                if _e1127 {
                                    let _e1130 = global_1.member[_e1122];
                                    let _e1134 = global_1.member[(_e1122 + 1u)];
                                    phi_1721_ = type_12(_e1130, _e1134);
                                } else {
                                    phi_1721_ = type_12(4294967295u, 0u);
                                }
                                let _e1137 = phi_1721_;
                                let _e1143 = (((select(select(u32(_e1110), 0u, (_e1110 < 0f)), 4294967295u, (_e1110 > 4294967000f)) >> bitcast<u32>(_e1107)) * (_e897 >> bitcast<u32>(_e1107))) + (select(select(u32(_e1101), 0u, (_e1101 < 0f)), 4294967295u, (_e1101 > 4294967000f)) >> bitcast<u32>(_e1107)));
                                if (_e1143 >= _e1137.member_1) {
                                    phi_2585_ = 4294967295u;
                                } else {
                                    phi_2585_ = (_e1137.member + _e1143);
                                }
                                let _e1147 = phi_2585_;
                                let _e1150 = global_1.member[_e1147];
                                if select((_e1076.z > 1f), true, (_e1076.z > bitcast<f32>(_e1150))) {
                                    global_2.member[_e98.x].member_1 = 0u;
                                }
                            }
                        } else {
                            global_2.member[_e98.x].member_1 = 0u;
                        }
                    }
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(32, 1, 1) 
fn cullcompute_culling(@builtin(global_invocation_id) param: vec3<u32>) {
    global_3 = param;
    function();
}
