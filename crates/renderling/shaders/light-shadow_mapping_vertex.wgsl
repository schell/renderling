struct type_3 {
    member: array<u32>,
}

struct type_11 {
    member: u32,
    member_1: u32,
}

struct type_14 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_17 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_20 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
    member_3: vec4<f32>,
    member_4: array<u32, 4>,
    member_5: array<f32, 4>,
    member_6: vec2<f32>,
    member_7: vec2<f32>,
}

struct type_23 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_24 {
    member: type_11,
    member_1: type_11,
}

var<private> global: u32;
var<private> global_1: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_2: u32;
@group(0) @binding(0) 
var<storage> global_3: type_3;
@group(0) @binding(1) 
var<storage> global_4: type_3;
var<private> global_5: type_14 = type_14(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));

fn function() {
    var local: array<f32, 4>;
    var local_1: array<u32, 4>;
    var local_2: array<f32, 4>;
    var local_3: array<u32, 4>;
    var phi_419_: u32;
    var phi_2254_: bool;
    var phi_426_: u32;
    var phi_427_: u32;
    var phi_437_: u32;
    var phi_519_: type_11;
    var phi_520_: type_11;
    var phi_543_: type_11;
    var phi_556_: bool;
    var phi_562_: type_11;
    var phi_563_: type_11;
    var phi_586_: type_11;
    var phi_600_: bool;
    var phi_606_: type_11;
    var phi_609_: type_20;
    var phi_607_: type_11;
    var phi_632_: type_11;
    var phi_649_: u32;
    var phi_2284_: bool;
    var phi_667_: type_11;
    var phi_2310_: u32;
    var phi_2329_: bool;
    var phi_717_: type_23;
    var phi_727_: u32;
    var phi_2351_: bool;
    var phi_735_: f32;
    var phi_610_: type_20;
    var phi_788_: bool;
    var phi_2371_: bool;
    var phi_884_: type_24;
    var local_4: type_20;
    var phi_887_: type_11;
    var phi_890_: type_14;
    var phi_888_: type_11;
    var phi_913_: type_11;
    var local_5: type_20;
    var phi_937_: u32;
    var phi_2405_: bool;
    var phi_946_: u32;
    var phi_2429_: bool;
    var phi_995_: type_17;
    var phi_1005_: u32;
    var phi_2454_: bool;
    var phi_1078_: type_14;
    var phi_891_: type_14;
    var phi_1315_: bool;
    var phi_2896_: bool;
    var local_6: type_14;
    var local_7: type_14;
    var local_8: type_14;
    var local_9: type_14;
    var phi_1342_: bool;
    var phi_1344_: bool;
    var phi_1345_: bool;
    var phi_1346_: bool;
    var phi_1347_: bool;
    var local_10: type_14;
    var local_11: type_14;
    var local_12: type_14;
    var local_13: type_14;
    var phi_1381_: bool;
    var phi_1383_: bool;
    var phi_1384_: bool;
    var phi_1385_: bool;
    var phi_1386_: bool;
    var local_14: type_14;
    var local_15: type_14;
    var local_16: type_14;
    var local_17: type_14;
    var phi_1420_: bool;
    var phi_1422_: bool;
    var phi_1423_: bool;
    var phi_1424_: bool;
    var phi_1425_: bool;
    var local_18: type_14;
    var local_19: type_14;
    var local_20: type_14;
    var local_21: type_14;
    var phi_1459_: bool;
    var phi_1461_: bool;
    var phi_1462_: bool;
    var phi_1463_: bool;
    var phi_1464_: bool;
    var phi_1469_: bool;
    var phi_1471_: bool;
    var phi_1472_: bool;
    var phi_1473_: bool;
    var phi_1474_: bool;
    var phi_1482_: type_14;
    var phi_2589_: bool;
    var phi_2654_: vec4<f32>;
    var phi_2684_: vec4<f32>;
    var phi_2686_: vec4<f32>;
    var phi_2695_: type_17;
    var phi_2696_: type_17;
    var phi_2701_: type_17;
    var phi_2702_: type_17;
    var phi_2703_: bool;
    var phi_2707_: type_17;
    var phi_1484_: type_17;
    var phi_1486_: type_17;
    var phi_1487_: bool;
    var phi_2801_: bool;
    var phi_1540_: type_17;
    var phi_1541_: type_17;
    var local_22: type_20;
    var phi_1654_: u32;
    var local_23: type_14;

    switch bitcast<i32>(0u) {
        default: {
            let _e63 = global_2;
            let _e64 = global;
            let _e66 = arrayLength((&global_3.member));
            let _e69 = global_3.member[_e63];
            let _e74 = global_3.member[(_e63 + 1u)];
            let _e78 = global_3.member[(_e63 + 2u)];
            let _e82 = global_3.member[(_e63 + 7u)];
            let _e86 = global_3.member[(_e63 + 8u)];
            let _e90 = global_3.member[(_e63 + 10u)];
            let _e94 = global_3.member[(_e63 + 12u)];
            let _e98 = global_3.member[(_e63 + 13u)];
            let _e102 = global_3.member[(_e63 + 14u)];
            let _e106 = global_3.member[(_e63 + 15u)];
            let _e110 = global_3.member[(_e63 + 16u)];
            let _e114 = global_3.member[(_e63 + 17u)];
            if (_e69 == 1u) {
                if (_e82 == 4294967295u) {
                    phi_427_ = _e64;
                } else {
                    if (_e64 >= _e86) {
                        phi_419_ = 4294967295u;
                    } else {
                        phi_419_ = (_e82 + _e64);
                    }
                    let _e119 = phi_419_;
                    if (_e66 >= 1u) {
                        phi_2254_ = (_e119 <= (_e66 - 1u));
                    } else {
                        phi_2254_ = false;
                    }
                    let _e124 = phi_2254_;
                    if _e124 {
                        let _e127 = global_3.member[_e119];
                        phi_426_ = _e127;
                    } else {
                        phi_426_ = 0u;
                    }
                    let _e129 = phi_426_;
                    phi_427_ = _e129;
                }
                let _e131 = phi_427_;
                if (_e131 >= _e78) {
                    phi_437_ = 4294967295u;
                } else {
                    phi_437_ = (_e74 + (26u * _e131));
                }
                let _e136 = phi_437_;
                let _e139 = global_3.member[_e136];
                let _e144 = global_3.member[(_e136 + 1u)];
                let _e149 = global_3.member[(_e136 + 2u)];
                let _e155 = global_3.member[(_e136 + 3u)];
                let _e160 = global_3.member[(_e136 + 4u)];
                let _e165 = global_3.member[(_e136 + 5u)];
                let _e170 = global_3.member[(_e136 + 6u)];
                let _e176 = global_3.member[(_e136 + 7u)];
                let _e181 = global_3.member[(_e136 + 8u)];
                let _e187 = global_3.member[(_e136 + 9u)];
                let _e192 = global_3.member[(_e136 + 10u)];
                let _e198 = global_3.member[(_e136 + 11u)];
                let _e203 = global_3.member[(_e136 + 12u)];
                let _e208 = global_3.member[(_e136 + 13u)];
                let _e214 = global_3.member[(_e136 + 14u)];
                let _e219 = global_3.member[(_e136 + 15u)];
                let _e224 = global_3.member[(_e136 + 16u)];
                let _e229 = global_3.member[(_e136 + 17u)];
                local_3 = array<u32, 4>(0u, 0u, 0u, 0u);
                phi_519_ = type_11(0u, 4u);
                loop {
                    let _e234 = phi_519_;
                    if (_e234.member < _e234.member_1) {
                        phi_520_ = type_11((_e234.member + 1u), _e234.member_1);
                        phi_543_ = type_11(1u, _e234.member);
                    } else {
                        phi_520_ = _e234;
                        phi_543_ = type_11(0u, type_11().member_1);
                    }
                    let _e247 = phi_520_;
                    let _e249 = phi_543_;
                    switch bitcast<i32>(_e249.member) {
                        case 0: {
                            phi_556_ = false;
                            break;
                        }
                        case 1: {
                            let _e256 = global_3.member[((_e136 + 18u) + _e249.member_1)];
                            local_3[_e249.member_1] = _e256;
                            phi_556_ = true;
                            break;
                        }
                        default: {
                            phi_556_ = bool();
                            break;
                        }
                    }
                    let _e259 = phi_556_;
                    continue;
                    continuing {
                        phi_519_ = _e247;
                        break if !(_e259);
                    }
                }
                let _e261 = local_3;
                local_2 = array<f32, 4>(0f, 0f, 0f, 0f);
                phi_562_ = type_11(0u, 4u);
                loop {
                    let _e264 = phi_562_;
                    if (_e264.member < _e264.member_1) {
                        phi_563_ = type_11((_e264.member + 1u), _e264.member_1);
                        phi_586_ = type_11(1u, _e264.member);
                    } else {
                        phi_563_ = _e264;
                        phi_586_ = type_11(0u, type_11().member_1);
                    }
                    let _e277 = phi_563_;
                    let _e279 = phi_586_;
                    switch bitcast<i32>(_e279.member) {
                        case 0: {
                            phi_600_ = false;
                            break;
                        }
                        case 1: {
                            let _e286 = global_3.member[((_e136 + 22u) + _e279.member_1)];
                            local_2[_e279.member_1] = bitcast<f32>(_e286);
                            phi_600_ = true;
                            break;
                        }
                        default: {
                            phi_600_ = bool();
                            break;
                        }
                    }
                    let _e290 = phi_600_;
                    continue;
                    continuing {
                        phi_562_ = _e277;
                        break if !(_e290);
                    }
                }
                let _e292 = local_2;
                phi_606_ = type_11(0u, _e102);
                phi_609_ = type_20(vec3<f32>(bitcast<f32>(_e139), bitcast<f32>(_e144), bitcast<f32>(_e149)), vec4<f32>(bitcast<f32>(_e155), bitcast<f32>(_e160), bitcast<f32>(_e165), bitcast<f32>(_e170)), vec3<f32>(bitcast<f32>(_e198), bitcast<f32>(_e203), bitcast<f32>(_e208)), vec4<f32>(bitcast<f32>(_e214), bitcast<f32>(_e219), bitcast<f32>(_e224), bitcast<f32>(_e229)), _e261, _e292, vec2<f32>(bitcast<f32>(_e176), bitcast<f32>(_e181)), vec2<f32>(bitcast<f32>(_e187), bitcast<f32>(_e192)));
                loop {
                    let _e296 = phi_606_;
                    let _e298 = phi_609_;
                    local_4 = _e298;
                    local_5 = _e298;
                    local_22 = _e298;
                    if (_e296.member < _e296.member_1) {
                        phi_607_ = type_11((_e296.member + 1u), _e296.member_1);
                        phi_632_ = type_11(1u, _e296.member);
                    } else {
                        phi_607_ = _e296;
                        phi_632_ = type_11(0u, type_11().member_1);
                    }
                    let _e311 = phi_607_;
                    let _e313 = phi_632_;
                    switch bitcast<i32>(_e313.member) {
                        case 0: {
                            phi_610_ = type_20();
                            phi_788_ = false;
                            break;
                        }
                        case 1: {
                            if (_e313.member_1 >= _e102) {
                                phi_649_ = 4294967295u;
                            } else {
                                phi_649_ = (_e98 + (2u * _e313.member_1));
                            }
                            let _e321 = phi_649_;
                            if (_e66 >= 2u) {
                                phi_2284_ = (_e321 <= (_e66 - 2u));
                            } else {
                                phi_2284_ = false;
                            }
                            let _e326 = phi_2284_;
                            if _e326 {
                                let _e329 = global_3.member[_e321];
                                let _e333 = global_3.member[(_e321 + 1u)];
                                phi_667_ = type_11(_e329, _e333);
                            } else {
                                phi_667_ = type_11(4294967295u, 0u);
                            }
                            let _e336 = phi_667_;
                            if (_e131 >= _e336.member_1) {
                                phi_2310_ = 4294967295u;
                            } else {
                                phi_2310_ = (_e336.member + (9u * _e131));
                            }
                            let _e343 = phi_2310_;
                            if (_e66 >= 9u) {
                                phi_2329_ = (_e343 <= (_e66 - 9u));
                            } else {
                                phi_2329_ = false;
                            }
                            let _e348 = phi_2329_;
                            if _e348 {
                                let _e351 = global_3.member[_e343];
                                let _e356 = global_3.member[(_e343 + 1u)];
                                let _e361 = global_3.member[(_e343 + 2u)];
                                let _e367 = global_3.member[(_e343 + 3u)];
                                let _e372 = global_3.member[(_e343 + 4u)];
                                let _e377 = global_3.member[(_e343 + 5u)];
                                let _e383 = global_3.member[(_e343 + 6u)];
                                let _e388 = global_3.member[(_e343 + 7u)];
                                let _e393 = global_3.member[(_e343 + 8u)];
                                phi_717_ = type_23(vec3<f32>(bitcast<f32>(_e351), bitcast<f32>(_e356), bitcast<f32>(_e361)), vec3<f32>(bitcast<f32>(_e367), bitcast<f32>(_e372), bitcast<f32>(_e377)), vec3<f32>(bitcast<f32>(_e383), bitcast<f32>(_e388), bitcast<f32>(_e393)));
                            } else {
                                phi_717_ = type_23(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e398 = phi_717_;
                            if (_e313.member_1 >= _e110) {
                                phi_727_ = 4294967295u;
                            } else {
                                phi_727_ = (_e106 + _e313.member_1);
                            }
                            let _e402 = phi_727_;
                            if (_e66 >= 1u) {
                                phi_2351_ = (_e402 <= (_e66 - 1u));
                            } else {
                                phi_2351_ = false;
                            }
                            let _e407 = phi_2351_;
                            if _e407 {
                                let _e410 = global_3.member[_e402];
                                phi_735_ = bitcast<f32>(_e410);
                            } else {
                                phi_735_ = 0f;
                            }
                            let _e413 = phi_735_;
                            let _e436 = type_20(vec3<f32>(fma(_e413, _e398.member.x, _e298.member.x), fma(_e413, _e398.member.y, _e298.member.y), fma(_e413, _e398.member.z, _e298.member.z)), _e298.member_1, _e298.member_2, _e298.member_3, _e298.member_4, _e298.member_5, _e298.member_6, _e298.member_7);
                            let _e459 = type_20(_e436.member, _e436.member_1, vec3<f32>(fma(_e413, _e398.member_1.x, _e298.member_2.x), fma(_e413, _e398.member_1.y, _e298.member_2.y), fma(_e413, _e398.member_1.z, _e298.member_2.z)), _e436.member_3, _e436.member_4, _e436.member_5, _e436.member_6, _e436.member_7);
                            phi_610_ = type_20(_e459.member, _e459.member_1, _e459.member_2, vec4<f32>(fma(_e413, _e398.member_2.x, _e298.member_3.x), fma(_e413, _e398.member_2.y, _e298.member_3.y), fma(_e413, _e398.member_2.z, _e298.member_3.z), _e298.member_3.w), _e459.member_4, _e459.member_5, _e459.member_6, _e459.member_7);
                            phi_788_ = true;
                            break;
                        }
                        default: {
                            phi_610_ = type_20();
                            phi_788_ = bool();
                            break;
                        }
                    }
                    let _e486 = phi_610_;
                    let _e488 = phi_788_;
                    continue;
                    continuing {
                        phi_606_ = _e311;
                        phi_609_ = _e486;
                        break if !(_e488);
                    }
                }
                let _e493 = global_3.member[(_e114 + 6u)];
                if (_e493 == 1u) {
                    let _e496 = ((_e94 == 4294967295u) != true);
                    if _e496 {
                        if (_e66 >= 4u) {
                            phi_2371_ = (_e94 <= (_e66 - 4u));
                        } else {
                            phi_2371_ = false;
                        }
                        let _e501 = phi_2371_;
                        if _e501 {
                            let _e504 = global_3.member[_e94];
                            let _e508 = global_3.member[(_e94 + 1u)];
                            let _e512 = global_3.member[(_e94 + 2u)];
                            let _e516 = global_3.member[(_e94 + 3u)];
                            phi_884_ = type_24(type_11(_e504, _e508), type_11(_e512, _e516));
                        } else {
                            phi_884_ = type_24(type_11(4294967295u, 0u), type_11(4294967295u, 0u));
                        }
                        let _e521 = phi_884_;
                        let _e523 = local_4;
                        local = _e523.member_5;
                        phi_887_ = type_11(0u, 4u);
                        phi_890_ = type_14(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        loop {
                            let _e526 = phi_887_;
                            let _e528 = phi_890_;
                            local_6 = _e528;
                            local_7 = _e528;
                            local_8 = _e528;
                            local_9 = _e528;
                            local_10 = _e528;
                            local_11 = _e528;
                            local_12 = _e528;
                            local_13 = _e528;
                            local_14 = _e528;
                            local_15 = _e528;
                            local_16 = _e528;
                            local_17 = _e528;
                            local_18 = _e528;
                            local_19 = _e528;
                            local_20 = _e528;
                            local_21 = _e528;
                            local_23 = _e528;
                            if (_e526.member < _e526.member_1) {
                                phi_888_ = type_11((_e526.member + 1u), _e526.member_1);
                                phi_913_ = type_11(1u, _e526.member);
                            } else {
                                phi_888_ = _e526;
                                phi_913_ = type_11(0u, type_11().member_1);
                            }
                            let _e541 = phi_888_;
                            let _e543 = phi_913_;
                            switch bitcast<i32>(_e543.member) {
                                case 0: {
                                    phi_891_ = type_14();
                                    phi_1315_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e548 = local_5;
                                    local_1 = _e548.member_4;
                                    let _e550 = (_e543.member_1 < 4u);
                                    if _e550 {
                                    } else {
                                        phi_2896_ = true;
                                        break;
                                    }
                                    let _e552 = local_1[_e543.member_1];
                                    if (_e552 >= _e521.member.member_1) {
                                        phi_937_ = 4294967295u;
                                    } else {
                                        phi_937_ = (_e521.member.member + _e552);
                                    }
                                    let _e560 = phi_937_;
                                    if (_e66 >= 1u) {
                                        phi_2405_ = (_e560 <= (_e66 - 1u));
                                    } else {
                                        phi_2405_ = false;
                                    }
                                    let _e565 = phi_2405_;
                                    if _e565 {
                                        let _e568 = global_3.member[_e560];
                                        phi_946_ = _e568;
                                    } else {
                                        phi_946_ = 4294967295u;
                                    }
                                    let _e570 = phi_946_;
                                    if (_e66 >= 10u) {
                                        phi_2429_ = (_e570 <= (_e66 - 10u));
                                    } else {
                                        phi_2429_ = false;
                                    }
                                    let _e575 = phi_2429_;
                                    if _e575 {
                                        let _e578 = global_3.member[_e570];
                                        let _e583 = global_3.member[(_e570 + 1u)];
                                        let _e588 = global_3.member[(_e570 + 2u)];
                                        let _e594 = global_3.member[(_e570 + 3u)];
                                        let _e599 = global_3.member[(_e570 + 4u)];
                                        let _e604 = global_3.member[(_e570 + 5u)];
                                        let _e609 = global_3.member[(_e570 + 6u)];
                                        let _e615 = global_3.member[(_e570 + 7u)];
                                        let _e620 = global_3.member[(_e570 + 8u)];
                                        let _e625 = global_3.member[(_e570 + 9u)];
                                        phi_995_ = type_17(vec3<f32>(bitcast<f32>(_e578), bitcast<f32>(_e583), bitcast<f32>(_e588)), vec4<f32>(bitcast<f32>(_e594), bitcast<f32>(_e599), bitcast<f32>(_e604), bitcast<f32>(_e609)), vec3<f32>(bitcast<f32>(_e615), bitcast<f32>(_e620), bitcast<f32>(_e625)));
                                    } else {
                                        phi_995_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e630 = phi_995_;
                                    if (_e552 >= _e521.member_1.member_1) {
                                        phi_1005_ = 4294967295u;
                                    } else {
                                        phi_1005_ = (_e521.member_1.member + (16u * _e552));
                                    }
                                    let _e639 = phi_1005_;
                                    if (_e66 >= 16u) {
                                        phi_2454_ = (_e639 <= (_e66 - 16u));
                                    } else {
                                        phi_2454_ = false;
                                    }
                                    let _e644 = phi_2454_;
                                    if _e644 {
                                        let _e647 = global_3.member[_e639];
                                        let _e652 = global_3.member[(_e639 + 1u)];
                                        let _e657 = global_3.member[(_e639 + 2u)];
                                        let _e662 = global_3.member[(_e639 + 3u)];
                                        let _e668 = global_3.member[(_e639 + 4u)];
                                        let _e673 = global_3.member[(_e639 + 5u)];
                                        let _e678 = global_3.member[(_e639 + 6u)];
                                        let _e683 = global_3.member[(_e639 + 7u)];
                                        let _e689 = global_3.member[(_e639 + 8u)];
                                        let _e694 = global_3.member[(_e639 + 9u)];
                                        let _e699 = global_3.member[(_e639 + 10u)];
                                        let _e704 = global_3.member[(_e639 + 11u)];
                                        let _e710 = global_3.member[(_e639 + 12u)];
                                        let _e715 = global_3.member[(_e639 + 13u)];
                                        let _e720 = global_3.member[(_e639 + 14u)];
                                        let _e725 = global_3.member[(_e639 + 15u)];
                                        phi_1078_ = type_14(vec4<f32>(bitcast<f32>(_e647), bitcast<f32>(_e652), bitcast<f32>(_e657), bitcast<f32>(_e662)), vec4<f32>(bitcast<f32>(_e668), bitcast<f32>(_e673), bitcast<f32>(_e678), bitcast<f32>(_e683)), vec4<f32>(bitcast<f32>(_e689), bitcast<f32>(_e694), bitcast<f32>(_e699), bitcast<f32>(_e704)), vec4<f32>(bitcast<f32>(_e710), bitcast<f32>(_e715), bitcast<f32>(_e720), bitcast<f32>(_e725)));
                                    } else {
                                        phi_1078_ = type_14(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                    }
                                    let _e730 = phi_1078_;
                                    let _e738 = (_e630.member_1.x + _e630.member_1.x);
                                    let _e739 = (_e630.member_1.y + _e630.member_1.y);
                                    let _e740 = (_e630.member_1.z + _e630.member_1.z);
                                    let _e742 = (_e630.member_1.z * _e740);
                                    let _e743 = (_e630.member_1.w * _e738);
                                    let _e744 = (_e630.member_1.w * _e739);
                                    let _e745 = (_e630.member_1.w * _e740);
                                    let _e765 = (vec4<f32>((1f - fma(_e630.member_1.y, _e739, _e742)), fma(_e630.member_1.x, _e739, _e745), fma(_e630.member_1.x, _e740, -(_e744)), 0f) * _e630.member_2.x);
                                    let _e767 = (vec4<f32>(fma(_e630.member_1.x, _e739, -(_e745)), (1f - fma(_e630.member_1.x, _e738, _e742)), fma(_e630.member_1.y, _e740, _e743), 0f) * _e630.member_2.y);
                                    let _e769 = (vec4<f32>(fma(_e630.member_1.x, _e740, _e744), fma(_e630.member_1.y, _e740, -(_e743)), (1f - fma(_e630.member_1.x, _e738, (_e630.member_1.y * _e739))), 0f) * _e630.member_2.z);
                                    if _e550 {
                                    } else {
                                        phi_2896_ = true;
                                        break;
                                    }
                                    let _e874 = local[_e543.member_1];
                                    phi_891_ = type_14((_e528.member + (vec4<f32>(fma(_e630.member.x, _e730.member.w, fma(_e769.x, _e730.member.z, fma(_e765.x, _e730.member.x, (_e767.x * _e730.member.y)))), fma(_e630.member.y, _e730.member.w, fma(_e769.y, _e730.member.z, fma(_e765.y, _e730.member.x, (_e767.y * _e730.member.y)))), fma(_e630.member.z, _e730.member.w, fma(_e769.z, _e730.member.z, fma(_e765.z, _e730.member.x, (_e767.z * _e730.member.y)))), (fma(_e769.w, _e730.member.z, fma(_e765.w, _e730.member.x, (_e767.w * _e730.member.y))) + _e730.member.w)) * _e874)), (_e528.member_1 + (vec4<f32>(fma(_e630.member.x, _e730.member_1.w, fma(_e769.x, _e730.member_1.z, fma(_e765.x, _e730.member_1.x, (_e767.x * _e730.member_1.y)))), fma(_e630.member.y, _e730.member_1.w, fma(_e769.y, _e730.member_1.z, fma(_e765.y, _e730.member_1.x, (_e767.y * _e730.member_1.y)))), fma(_e630.member.z, _e730.member_1.w, fma(_e769.z, _e730.member_1.z, fma(_e765.z, _e730.member_1.x, (_e767.z * _e730.member_1.y)))), (fma(_e769.w, _e730.member_1.z, fma(_e765.w, _e730.member_1.x, (_e767.w * _e730.member_1.y))) + _e730.member_1.w)) * _e874)), (_e528.member_2 + (vec4<f32>(fma(_e630.member.x, _e730.member_2.w, fma(_e769.x, _e730.member_2.z, fma(_e765.x, _e730.member_2.x, (_e767.x * _e730.member_2.y)))), fma(_e630.member.y, _e730.member_2.w, fma(_e769.y, _e730.member_2.z, fma(_e765.y, _e730.member_2.x, (_e767.y * _e730.member_2.y)))), fma(_e630.member.z, _e730.member_2.w, fma(_e769.z, _e730.member_2.z, fma(_e765.z, _e730.member_2.x, (_e767.z * _e730.member_2.y)))), (fma(_e769.w, _e730.member_2.z, fma(_e765.w, _e730.member_2.x, (_e767.w * _e730.member_2.y))) + _e730.member_2.w)) * _e874)), (_e528.member_3 + (vec4<f32>(fma(_e630.member.x, _e730.member_3.w, fma(_e769.x, _e730.member_3.z, fma(_e765.x, _e730.member_3.x, (_e767.x * _e730.member_3.y)))), fma(_e630.member.y, _e730.member_3.w, fma(_e769.y, _e730.member_3.z, fma(_e765.y, _e730.member_3.x, (_e767.y * _e730.member_3.y)))), fma(_e630.member.z, _e730.member_3.w, fma(_e769.z, _e730.member_3.z, fma(_e765.z, _e730.member_3.x, (_e767.z * _e730.member_3.y)))), (fma(_e769.w, _e730.member_3.z, fma(_e765.w, _e730.member_3.x, (_e767.w * _e730.member_3.y))) + _e730.member_3.w)) * _e874)));
                                    phi_1315_ = true;
                                    break;
                                }
                                default: {
                                    phi_891_ = type_14();
                                    phi_1315_ = bool();
                                    break;
                                }
                            }
                            let _e889 = phi_891_;
                            let _e891 = phi_1315_;
                            continue;
                            continuing {
                                phi_887_ = _e541;
                                phi_890_ = _e889;
                                phi_2896_ = false;
                                break if !(_e891);
                            }
                        }
                        let _e894 = phi_2896_;
                        if _e894 {
                            break;
                        }
                        let _e896 = local_6;
                        let _e901 = global_5.member[0u];
                        if (_e896.member.x == _e901) {
                            let _e904 = local_7;
                            let _e909 = global_5.member[1u];
                            if (_e904.member.y == _e909) {
                                let _e912 = local_8;
                                let _e917 = global_5.member[2u];
                                let _e918 = (_e912.member.z == _e917);
                                if _e918 {
                                    let _e920 = local_9;
                                    let _e925 = global_5.member[3u];
                                    phi_1342_ = (_e920.member.w == _e925);
                                } else {
                                    phi_1342_ = bool();
                                }
                                let _e928 = phi_1342_;
                                phi_1344_ = _e928;
                                phi_1345_ = select(true, false, _e918);
                            } else {
                                phi_1344_ = bool();
                                phi_1345_ = true;
                            }
                            let _e931 = phi_1344_;
                            let _e933 = phi_1345_;
                            phi_1346_ = _e931;
                            phi_1347_ = _e933;
                        } else {
                            phi_1346_ = bool();
                            phi_1347_ = true;
                        }
                        let _e935 = phi_1346_;
                        let _e937 = phi_1347_;
                        if select(_e935, false, _e937) {
                            let _e940 = local_10;
                            let _e945 = global_5.member_1[0u];
                            if (_e940.member_1.x == _e945) {
                                let _e948 = local_11;
                                let _e953 = global_5.member_1[1u];
                                if (_e948.member_1.y == _e953) {
                                    let _e956 = local_12;
                                    let _e961 = global_5.member_1[2u];
                                    let _e962 = (_e956.member_1.z == _e961);
                                    if _e962 {
                                        let _e964 = local_13;
                                        let _e969 = global_5.member_1[3u];
                                        phi_1381_ = (_e964.member_1.w == _e969);
                                    } else {
                                        phi_1381_ = bool();
                                    }
                                    let _e972 = phi_1381_;
                                    phi_1383_ = _e972;
                                    phi_1384_ = select(true, false, _e962);
                                } else {
                                    phi_1383_ = bool();
                                    phi_1384_ = true;
                                }
                                let _e975 = phi_1383_;
                                let _e977 = phi_1384_;
                                phi_1385_ = _e975;
                                phi_1386_ = _e977;
                            } else {
                                phi_1385_ = bool();
                                phi_1386_ = true;
                            }
                            let _e979 = phi_1385_;
                            let _e981 = phi_1386_;
                            if select(_e979, false, _e981) {
                                let _e984 = local_14;
                                let _e989 = global_5.member_2[0u];
                                if (_e984.member_2.x == _e989) {
                                    let _e992 = local_15;
                                    let _e997 = global_5.member_2[1u];
                                    if (_e992.member_2.y == _e997) {
                                        let _e1000 = local_16;
                                        let _e1005 = global_5.member_2[2u];
                                        let _e1006 = (_e1000.member_2.z == _e1005);
                                        if _e1006 {
                                            let _e1008 = local_17;
                                            let _e1013 = global_5.member_2[3u];
                                            phi_1420_ = (_e1008.member_2.w == _e1013);
                                        } else {
                                            phi_1420_ = bool();
                                        }
                                        let _e1016 = phi_1420_;
                                        phi_1422_ = _e1016;
                                        phi_1423_ = select(true, false, _e1006);
                                    } else {
                                        phi_1422_ = bool();
                                        phi_1423_ = true;
                                    }
                                    let _e1019 = phi_1422_;
                                    let _e1021 = phi_1423_;
                                    phi_1424_ = _e1019;
                                    phi_1425_ = _e1021;
                                } else {
                                    phi_1424_ = bool();
                                    phi_1425_ = true;
                                }
                                let _e1023 = phi_1424_;
                                let _e1025 = phi_1425_;
                                let _e1026 = select(_e1023, false, _e1025);
                                if _e1026 {
                                    let _e1028 = local_18;
                                    let _e1033 = global_5.member_3[0u];
                                    if (_e1028.member_3.x == _e1033) {
                                        let _e1036 = local_19;
                                        let _e1041 = global_5.member_3[1u];
                                        if (_e1036.member_3.y == _e1041) {
                                            let _e1044 = local_20;
                                            let _e1049 = global_5.member_3[2u];
                                            let _e1050 = (_e1044.member_3.z == _e1049);
                                            if _e1050 {
                                                let _e1052 = local_21;
                                                let _e1057 = global_5.member_3[3u];
                                                phi_1459_ = (_e1052.member_3.w == _e1057);
                                            } else {
                                                phi_1459_ = bool();
                                            }
                                            let _e1060 = phi_1459_;
                                            phi_1461_ = _e1060;
                                            phi_1462_ = select(true, false, _e1050);
                                        } else {
                                            phi_1461_ = bool();
                                            phi_1462_ = true;
                                        }
                                        let _e1063 = phi_1461_;
                                        let _e1065 = phi_1462_;
                                        phi_1463_ = _e1063;
                                        phi_1464_ = _e1065;
                                    } else {
                                        phi_1463_ = bool();
                                        phi_1464_ = true;
                                    }
                                    let _e1067 = phi_1463_;
                                    let _e1069 = phi_1464_;
                                    phi_1469_ = select(_e1067, false, _e1069);
                                } else {
                                    phi_1469_ = bool();
                                }
                                let _e1072 = phi_1469_;
                                phi_1471_ = _e1072;
                                phi_1472_ = select(true, false, _e1026);
                            } else {
                                phi_1471_ = bool();
                                phi_1472_ = true;
                            }
                            let _e1075 = phi_1471_;
                            let _e1077 = phi_1472_;
                            phi_1473_ = _e1075;
                            phi_1474_ = _e1077;
                        } else {
                            phi_1473_ = bool();
                            phi_1474_ = true;
                        }
                        let _e1079 = phi_1473_;
                        let _e1081 = phi_1474_;
                        if select(_e1079, false, _e1081) {
                            phi_1482_ = type_14(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        } else {
                            let _e1607 = local_23;
                            phi_1482_ = _e1607;
                        }
                        let _e1084 = phi_1482_;
                        let _e1107 = fma(_e1084.member_2.z, _e1084.member_3.w, -((_e1084.member_2.w * _e1084.member_3.z)));
                        let _e1110 = fma(_e1084.member_2.y, _e1084.member_3.w, -((_e1084.member_2.w * _e1084.member_3.y)));
                        let _e1113 = fma(_e1084.member_2.y, _e1084.member_3.z, -((_e1084.member_2.z * _e1084.member_3.y)));
                        let _e1116 = fma(_e1084.member_2.x, _e1084.member_3.w, -((_e1084.member_2.w * _e1084.member_3.x)));
                        let _e1119 = fma(_e1084.member_2.x, _e1084.member_3.z, -((_e1084.member_2.z * _e1084.member_3.x)));
                        let _e1122 = fma(_e1084.member_2.x, _e1084.member_3.y, -((_e1084.member_2.y * _e1084.member_3.x)));
                        let _e1144 = fma(-(_e1084.member.w), fma(_e1084.member_1.z, _e1122, fma(_e1084.member_1.x, _e1113, -((_e1084.member_1.y * _e1119)))), fma(_e1084.member.z, fma(_e1084.member_1.w, _e1122, fma(_e1084.member_1.x, _e1110, -((_e1084.member_1.y * _e1116)))), fma(_e1084.member.x, fma(_e1084.member_1.w, _e1113, fma(_e1084.member_1.y, _e1107, -((_e1084.member_1.z * _e1110)))), -((_e1084.member.y * fma(_e1084.member_1.w, _e1119, fma(_e1084.member_1.x, _e1107, -((_e1084.member_1.z * _e1116)))))))));
                        if (_e1144 == 0f) {
                            phi_2701_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            phi_2702_ = type_17();
                            phi_2703_ = true;
                        } else {
                            let _e1153 = (sqrt(fma(_e1084.member.w, _e1084.member.w, fma(_e1084.member.z, _e1084.member.z, fma(_e1084.member.x, _e1084.member.x, (_e1084.member.y * _e1084.member.y))))) * select(-1f, 1f, (_e1144 >= 0f)));
                            let _e1158 = sqrt(fma(_e1084.member_1.w, _e1084.member_1.w, fma(_e1084.member_1.z, _e1084.member_1.z, fma(_e1084.member_1.x, _e1084.member_1.x, (_e1084.member_1.y * _e1084.member_1.y)))));
                            let _e1163 = sqrt(fma(_e1084.member_2.w, _e1084.member_2.w, fma(_e1084.member_2.z, _e1084.member_2.z, fma(_e1084.member_2.x, _e1084.member_2.x, (_e1084.member_2.y * _e1084.member_2.y)))));
                            if (_e1153 != 0f) {
                                phi_2589_ = select(true, false, (_e1158 != 0f));
                            } else {
                                phi_2589_ = true;
                            }
                            let _e1170 = phi_2589_;
                            let _e1171 = select((_e1163 != 0f), false, _e1170);
                            if _e1171 {
                                let _e1172 = (1f / _e1153);
                                let _e1173 = (1f / _e1158);
                                let _e1174 = (1f / _e1163);
                                let _e1175 = (_e1084.member.x * _e1172);
                                let _e1176 = (_e1084.member.z * _e1172);
                                let _e1177 = (_e1084.member_1.x * _e1173);
                                let _e1178 = (_e1084.member_2.x * _e1174);
                                let _e1179 = (_e1084.member_2.y * _e1174);
                                if ((_e1084.member_2.z * _e1174) <= 0f) {
                                    let _e1214 = fma(_e1084.member_1.y, _e1173, -(_e1175));
                                    let _e1216 = fma(-(_e1084.member_2.z), _e1174, 1f);
                                    if (_e1214 <= 0f) {
                                        let _e1230 = (_e1216 - _e1214);
                                        let _e1232 = (0.5f / sqrt(_e1230));
                                        phi_2684_ = vec4<f32>((_e1230 * _e1232), (fma(_e1084.member.y, _e1172, _e1177) * _e1232), (fma(_e1084.member.z, _e1172, _e1178) * _e1232), (fma(_e1084.member_1.z, _e1173, -(_e1179)) * _e1232));
                                    } else {
                                        let _e1218 = (_e1216 + _e1214);
                                        let _e1220 = (0.5f / sqrt(_e1218));
                                        phi_2684_ = vec4<f32>((fma(_e1084.member.y, _e1172, _e1177) * _e1220), (_e1218 * _e1220), (fma(_e1084.member_1.z, _e1173, _e1179) * _e1220), (fma(_e1084.member_2.x, _e1174, -(_e1176)) * _e1220));
                                    }
                                    let _e1243 = phi_2684_;
                                    phi_2686_ = _e1243;
                                } else {
                                    let _e1182 = fma(_e1084.member_1.y, _e1173, _e1175);
                                    let _e1183 = fma(_e1084.member_2.z, _e1174, 1f);
                                    if (_e1182 <= 0f) {
                                        let _e1199 = (_e1183 - _e1182);
                                        let _e1201 = (0.5f / sqrt(_e1199));
                                        phi_2654_ = vec4<f32>((fma(_e1084.member.z, _e1172, _e1178) * _e1201), (fma(_e1084.member_1.z, _e1173, _e1179) * _e1201), (_e1199 * _e1201), (fma(_e1084.member.y, _e1172, -(_e1177)) * _e1201));
                                    } else {
                                        let _e1185 = (_e1183 + _e1182);
                                        let _e1187 = (0.5f / sqrt(_e1185));
                                        phi_2654_ = vec4<f32>((fma(_e1084.member_1.z, _e1173, -(_e1179)) * _e1187), (fma(_e1084.member_2.x, _e1174, -(_e1176)) * _e1187), (fma(_e1084.member.y, _e1172, -(_e1177)) * _e1187), (_e1185 * _e1187));
                                    }
                                    let _e1212 = phi_2654_;
                                    phi_2686_ = _e1212;
                                }
                                let _e1245 = phi_2686_;
                                phi_2695_ = type_17(vec3<f32>(_e1153, _e1158, _e1163), _e1245, vec3<f32>(_e1084.member_3.x, _e1084.member_3.y, _e1084.member_3.z));
                                phi_2696_ = type_17();
                            } else {
                                phi_2695_ = type_17();
                                phi_2696_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e1249 = phi_2695_;
                            let _e1251 = phi_2696_;
                            phi_2701_ = _e1251;
                            phi_2702_ = _e1249;
                            phi_2703_ = select(true, false, _e1171);
                        }
                        let _e1254 = phi_2701_;
                        let _e1256 = phi_2702_;
                        let _e1258 = phi_2703_;
                        if _e1258 {
                            phi_2707_ = _e1254;
                        } else {
                            phi_2707_ = _e1256;
                        }
                        let _e1260 = phi_2707_;
                        phi_1484_ = type_17(_e1260.member_2, _e1260.member_1, _e1260.member);
                    } else {
                        phi_1484_ = type_17();
                    }
                    let _e1266 = phi_1484_;
                    phi_1486_ = _e1266;
                    phi_1487_ = select(true, false, _e496);
                } else {
                    phi_1486_ = type_17();
                    phi_1487_ = true;
                }
                let _e1269 = phi_1486_;
                let _e1271 = phi_1487_;
                if _e1271 {
                    if (_e66 >= 10u) {
                        phi_2801_ = (_e90 <= (_e66 - 10u));
                    } else {
                        phi_2801_ = false;
                    }
                    let _e1276 = phi_2801_;
                    if _e1276 {
                        let _e1279 = global_3.member[_e90];
                        let _e1284 = global_3.member[(_e90 + 1u)];
                        let _e1289 = global_3.member[(_e90 + 2u)];
                        let _e1295 = global_3.member[(_e90 + 3u)];
                        let _e1300 = global_3.member[(_e90 + 4u)];
                        let _e1305 = global_3.member[(_e90 + 5u)];
                        let _e1310 = global_3.member[(_e90 + 6u)];
                        let _e1316 = global_3.member[(_e90 + 7u)];
                        let _e1321 = global_3.member[(_e90 + 8u)];
                        let _e1326 = global_3.member[(_e90 + 9u)];
                        phi_1540_ = type_17(vec3<f32>(bitcast<f32>(_e1279), bitcast<f32>(_e1284), bitcast<f32>(_e1289)), vec4<f32>(bitcast<f32>(_e1295), bitcast<f32>(_e1300), bitcast<f32>(_e1305), bitcast<f32>(_e1310)), vec3<f32>(bitcast<f32>(_e1316), bitcast<f32>(_e1321), bitcast<f32>(_e1326)));
                    } else {
                        phi_1540_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e1331 = phi_1540_;
                    phi_1541_ = _e1331;
                } else {
                    phi_1541_ = _e1269;
                }
                let _e1333 = phi_1541_;
                let _e1341 = (_e1333.member_1.x + _e1333.member_1.x);
                let _e1342 = (_e1333.member_1.y + _e1333.member_1.y);
                let _e1343 = (_e1333.member_1.z + _e1333.member_1.z);
                let _e1345 = (_e1333.member_1.z * _e1343);
                let _e1346 = (_e1333.member_1.w * _e1341);
                let _e1347 = (_e1333.member_1.w * _e1342);
                let _e1348 = (_e1333.member_1.w * _e1343);
                let _e1368 = (vec4<f32>((1f - fma(_e1333.member_1.y, _e1342, _e1345)), fma(_e1333.member_1.x, _e1342, _e1348), fma(_e1333.member_1.x, _e1343, -(_e1347)), 0f) * _e1333.member_2.x);
                let _e1370 = (vec4<f32>(fma(_e1333.member_1.x, _e1342, -(_e1348)), (1f - fma(_e1333.member_1.x, _e1341, _e1345)), fma(_e1333.member_1.y, _e1343, _e1346), 0f) * _e1333.member_2.y);
                let _e1372 = (vec4<f32>(fma(_e1333.member_1.x, _e1343, _e1347), fma(_e1333.member_1.y, _e1343, -(_e1346)), (1f - fma(_e1333.member_1.x, _e1341, (_e1333.member_1.y * _e1342))), 0f) * _e1333.member_2.z);
                let _e1377 = local_22;
                let _e1400 = (_e1333.member.x + fma(_e1372.x, _e1377.member.z, fma(_e1370.x, _e1377.member.y, (_e1368.x * _e1377.member.x))));
                let _e1401 = (_e1333.member.y + fma(_e1372.y, _e1377.member.z, fma(_e1370.y, _e1377.member.y, (_e1368.y * _e1377.member.x))));
                let _e1402 = (_e1333.member.z + fma(_e1372.z, _e1377.member.z, fma(_e1370.z, _e1377.member.y, (_e1368.z * _e1377.member.x))));
                let _e1405 = global_4.member[3u];
                let _e1408 = global_4.member[4u];
                let _e1411 = global_4.member[_e1405];
                let _e1415 = global_4.member[(_e1405 + 1u)];
                if (_e1408 >= _e1415) {
                    phi_1654_ = 4294967295u;
                } else {
                    phi_1654_ = (_e1411 + (16u * _e1408));
                }
                let _e1420 = phi_1654_;
                let _e1423 = global_4.member[_e1420];
                let _e1428 = global_4.member[(_e1420 + 1u)];
                let _e1433 = global_4.member[(_e1420 + 2u)];
                let _e1438 = global_4.member[(_e1420 + 3u)];
                let _e1443 = global_4.member[(_e1420 + 4u)];
                let _e1448 = global_4.member[(_e1420 + 5u)];
                let _e1453 = global_4.member[(_e1420 + 6u)];
                let _e1458 = global_4.member[(_e1420 + 7u)];
                let _e1463 = global_4.member[(_e1420 + 8u)];
                let _e1468 = global_4.member[(_e1420 + 9u)];
                let _e1473 = global_4.member[(_e1420 + 10u)];
                let _e1478 = global_4.member[(_e1420 + 11u)];
                let _e1483 = global_4.member[(_e1420 + 12u)];
                let _e1488 = global_4.member[(_e1420 + 13u)];
                let _e1493 = global_4.member[(_e1420 + 14u)];
                let _e1498 = global_4.member[(_e1420 + 15u)];
                global_1 = vec4<f32>((fma(bitcast<f32>(_e1463), _e1402, fma(bitcast<f32>(_e1423), _e1400, (bitcast<f32>(_e1443) * _e1401))) + bitcast<f32>(_e1483)), (fma(bitcast<f32>(_e1468), _e1402, fma(bitcast<f32>(_e1428), _e1400, (bitcast<f32>(_e1448) * _e1401))) + bitcast<f32>(_e1488)), (fma(bitcast<f32>(_e1473), _e1402, fma(bitcast<f32>(_e1433), _e1400, (bitcast<f32>(_e1453) * _e1401))) + bitcast<f32>(_e1493)), (fma(bitcast<f32>(_e1478), _e1402, fma(bitcast<f32>(_e1438), _e1400, (bitcast<f32>(_e1458) * _e1401))) + bitcast<f32>(_e1498)));
            } else {
                global_1 = vec4<f32>(10f, 10f, 10f, 1f);
            }
            break;
        }
    }
    return;
}

@vertex 
fn lightshadow_mapping_vertex(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> @builtin(position) vec4<f32> {
    global_2 = param;
    global = param_1;
    function();
    let _e6 = global_1.y;
    global_1.y = -(_e6);
    let _e8 = global_1;
    return _e8;
}
