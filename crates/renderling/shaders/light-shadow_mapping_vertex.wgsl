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
    var phi_414_: u32;
    var phi_2251_: bool;
    var phi_421_: u32;
    var phi_422_: u32;
    var phi_432_: u32;
    var phi_514_: type_11;
    var phi_515_: type_11;
    var phi_538_: type_11;
    var phi_551_: bool;
    var phi_557_: type_11;
    var phi_558_: type_11;
    var phi_581_: type_11;
    var phi_595_: bool;
    var phi_601_: type_11;
    var phi_604_: type_20;
    var phi_602_: type_11;
    var phi_627_: type_11;
    var phi_644_: u32;
    var phi_2281_: bool;
    var phi_662_: type_11;
    var phi_2307_: u32;
    var phi_2326_: bool;
    var phi_712_: type_23;
    var phi_722_: u32;
    var phi_2348_: bool;
    var phi_730_: f32;
    var phi_605_: type_20;
    var phi_783_: bool;
    var phi_2370_: bool;
    var phi_883_: type_24;
    var local_4: type_20;
    var phi_886_: type_11;
    var phi_889_: type_14;
    var phi_887_: type_11;
    var phi_912_: type_11;
    var local_5: type_20;
    var phi_936_: u32;
    var phi_2404_: bool;
    var phi_945_: u32;
    var phi_2428_: bool;
    var phi_994_: type_17;
    var phi_1004_: u32;
    var phi_2453_: bool;
    var phi_1077_: type_14;
    var phi_890_: type_14;
    var phi_1314_: bool;
    var phi_2895_: bool;
    var local_6: type_14;
    var local_7: type_14;
    var local_8: type_14;
    var local_9: type_14;
    var phi_1341_: bool;
    var phi_1343_: bool;
    var phi_1344_: bool;
    var phi_1345_: bool;
    var phi_1346_: bool;
    var local_10: type_14;
    var local_11: type_14;
    var local_12: type_14;
    var local_13: type_14;
    var phi_1380_: bool;
    var phi_1382_: bool;
    var phi_1383_: bool;
    var phi_1384_: bool;
    var phi_1385_: bool;
    var local_14: type_14;
    var local_15: type_14;
    var local_16: type_14;
    var local_17: type_14;
    var phi_1419_: bool;
    var phi_1421_: bool;
    var phi_1422_: bool;
    var phi_1423_: bool;
    var phi_1424_: bool;
    var local_18: type_14;
    var local_19: type_14;
    var local_20: type_14;
    var local_21: type_14;
    var phi_1458_: bool;
    var phi_1460_: bool;
    var phi_1461_: bool;
    var phi_1462_: bool;
    var phi_1463_: bool;
    var phi_1468_: bool;
    var phi_1470_: bool;
    var phi_1471_: bool;
    var phi_1472_: bool;
    var phi_1473_: bool;
    var phi_1481_: type_14;
    var phi_2588_: bool;
    var phi_2653_: vec4<f32>;
    var phi_2683_: vec4<f32>;
    var phi_2685_: vec4<f32>;
    var phi_2694_: type_17;
    var phi_2695_: type_17;
    var phi_2700_: type_17;
    var phi_2701_: type_17;
    var phi_2702_: bool;
    var phi_2706_: type_17;
    var phi_1483_: type_17;
    var phi_1485_: type_17;
    var phi_1486_: bool;
    var phi_2800_: bool;
    var phi_1539_: type_17;
    var phi_1540_: type_17;
    var local_22: type_20;
    var phi_1653_: u32;
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
            let _e90 = global_3.member[(_e63 + 9u)];
            let _e94 = global_3.member[(_e63 + 11u)];
            let _e98 = global_3.member[(_e63 + 12u)];
            let _e102 = global_3.member[(_e63 + 13u)];
            let _e106 = global_3.member[(_e63 + 14u)];
            let _e110 = global_3.member[(_e63 + 15u)];
            let _e114 = global_3.member[(_e63 + 16u)];
            if (_e69 == 1u) {
                if (_e82 == 4294967295u) {
                    phi_422_ = _e64;
                } else {
                    if (_e64 >= _e86) {
                        phi_414_ = 4294967295u;
                    } else {
                        phi_414_ = (_e82 + _e64);
                    }
                    let _e119 = phi_414_;
                    if (_e66 >= 1u) {
                        phi_2251_ = (_e119 <= (_e66 - 1u));
                    } else {
                        phi_2251_ = false;
                    }
                    let _e124 = phi_2251_;
                    if _e124 {
                        let _e127 = global_3.member[_e119];
                        phi_421_ = _e127;
                    } else {
                        phi_421_ = 0u;
                    }
                    let _e129 = phi_421_;
                    phi_422_ = _e129;
                }
                let _e131 = phi_422_;
                if (_e131 >= _e78) {
                    phi_432_ = 4294967295u;
                } else {
                    phi_432_ = (_e74 + (26u * _e131));
                }
                let _e136 = phi_432_;
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
                phi_514_ = type_11(0u, 4u);
                loop {
                    let _e234 = phi_514_;
                    if (_e234.member < _e234.member_1) {
                        phi_515_ = type_11((_e234.member + 1u), _e234.member_1);
                        phi_538_ = type_11(1u, _e234.member);
                    } else {
                        phi_515_ = _e234;
                        phi_538_ = type_11(0u, type_11().member_1);
                    }
                    let _e247 = phi_515_;
                    let _e249 = phi_538_;
                    switch bitcast<i32>(_e249.member) {
                        case 0: {
                            phi_551_ = false;
                            break;
                        }
                        case 1: {
                            let _e256 = global_3.member[((_e136 + 18u) + _e249.member_1)];
                            local_3[_e249.member_1] = _e256;
                            phi_551_ = true;
                            break;
                        }
                        default: {
                            phi_551_ = bool();
                            break;
                        }
                    }
                    let _e259 = phi_551_;
                    continue;
                    continuing {
                        phi_514_ = _e247;
                        break if !(_e259);
                    }
                }
                let _e261 = local_3;
                local_2 = array<f32, 4>(0f, 0f, 0f, 0f);
                phi_557_ = type_11(0u, 4u);
                loop {
                    let _e264 = phi_557_;
                    if (_e264.member < _e264.member_1) {
                        phi_558_ = type_11((_e264.member + 1u), _e264.member_1);
                        phi_581_ = type_11(1u, _e264.member);
                    } else {
                        phi_558_ = _e264;
                        phi_581_ = type_11(0u, type_11().member_1);
                    }
                    let _e277 = phi_558_;
                    let _e279 = phi_581_;
                    switch bitcast<i32>(_e279.member) {
                        case 0: {
                            phi_595_ = false;
                            break;
                        }
                        case 1: {
                            let _e286 = global_3.member[((_e136 + 22u) + _e279.member_1)];
                            local_2[_e279.member_1] = bitcast<f32>(_e286);
                            phi_595_ = true;
                            break;
                        }
                        default: {
                            phi_595_ = bool();
                            break;
                        }
                    }
                    let _e290 = phi_595_;
                    continue;
                    continuing {
                        phi_557_ = _e277;
                        break if !(_e290);
                    }
                }
                let _e292 = local_2;
                phi_601_ = type_11(0u, _e102);
                phi_604_ = type_20(vec3<f32>(bitcast<f32>(_e139), bitcast<f32>(_e144), bitcast<f32>(_e149)), vec4<f32>(bitcast<f32>(_e155), bitcast<f32>(_e160), bitcast<f32>(_e165), bitcast<f32>(_e170)), vec3<f32>(bitcast<f32>(_e198), bitcast<f32>(_e203), bitcast<f32>(_e208)), vec4<f32>(bitcast<f32>(_e214), bitcast<f32>(_e219), bitcast<f32>(_e224), bitcast<f32>(_e229)), _e261, _e292, vec2<f32>(bitcast<f32>(_e176), bitcast<f32>(_e181)), vec2<f32>(bitcast<f32>(_e187), bitcast<f32>(_e192)));
                loop {
                    let _e296 = phi_601_;
                    let _e298 = phi_604_;
                    local_4 = _e298;
                    local_5 = _e298;
                    local_22 = _e298;
                    if (_e296.member < _e296.member_1) {
                        phi_602_ = type_11((_e296.member + 1u), _e296.member_1);
                        phi_627_ = type_11(1u, _e296.member);
                    } else {
                        phi_602_ = _e296;
                        phi_627_ = type_11(0u, type_11().member_1);
                    }
                    let _e311 = phi_602_;
                    let _e313 = phi_627_;
                    switch bitcast<i32>(_e313.member) {
                        case 0: {
                            phi_605_ = type_20();
                            phi_783_ = false;
                            break;
                        }
                        case 1: {
                            if (_e313.member_1 >= _e102) {
                                phi_644_ = 4294967295u;
                            } else {
                                phi_644_ = (_e98 + (2u * _e313.member_1));
                            }
                            let _e321 = phi_644_;
                            if (_e66 >= 2u) {
                                phi_2281_ = (_e321 <= (_e66 - 2u));
                            } else {
                                phi_2281_ = false;
                            }
                            let _e326 = phi_2281_;
                            if _e326 {
                                let _e329 = global_3.member[_e321];
                                let _e333 = global_3.member[(_e321 + 1u)];
                                phi_662_ = type_11(_e329, _e333);
                            } else {
                                phi_662_ = type_11(4294967295u, 0u);
                            }
                            let _e336 = phi_662_;
                            if (_e131 >= _e336.member_1) {
                                phi_2307_ = 4294967295u;
                            } else {
                                phi_2307_ = (_e336.member + (9u * _e131));
                            }
                            let _e343 = phi_2307_;
                            if (_e66 >= 9u) {
                                phi_2326_ = (_e343 <= (_e66 - 9u));
                            } else {
                                phi_2326_ = false;
                            }
                            let _e348 = phi_2326_;
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
                                phi_712_ = type_23(vec3<f32>(bitcast<f32>(_e351), bitcast<f32>(_e356), bitcast<f32>(_e361)), vec3<f32>(bitcast<f32>(_e367), bitcast<f32>(_e372), bitcast<f32>(_e377)), vec3<f32>(bitcast<f32>(_e383), bitcast<f32>(_e388), bitcast<f32>(_e393)));
                            } else {
                                phi_712_ = type_23(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e398 = phi_712_;
                            if (_e313.member_1 >= _e110) {
                                phi_722_ = 4294967295u;
                            } else {
                                phi_722_ = (_e106 + _e313.member_1);
                            }
                            let _e402 = phi_722_;
                            if (_e66 >= 1u) {
                                phi_2348_ = (_e402 <= (_e66 - 1u));
                            } else {
                                phi_2348_ = false;
                            }
                            let _e407 = phi_2348_;
                            if _e407 {
                                let _e410 = global_3.member[_e402];
                                phi_730_ = bitcast<f32>(_e410);
                            } else {
                                phi_730_ = 0f;
                            }
                            let _e413 = phi_730_;
                            let _e436 = type_20(vec3<f32>(fma(_e413, _e398.member.x, _e298.member.x), fma(_e413, _e398.member.y, _e298.member.y), fma(_e413, _e398.member.z, _e298.member.z)), _e298.member_1, _e298.member_2, _e298.member_3, _e298.member_4, _e298.member_5, _e298.member_6, _e298.member_7);
                            let _e459 = type_20(_e436.member, _e436.member_1, vec3<f32>(fma(_e413, _e398.member_1.x, _e298.member_2.x), fma(_e413, _e398.member_1.y, _e298.member_2.y), fma(_e413, _e398.member_1.z, _e298.member_2.z)), _e436.member_3, _e436.member_4, _e436.member_5, _e436.member_6, _e436.member_7);
                            phi_605_ = type_20(_e459.member, _e459.member_1, _e459.member_2, vec4<f32>(fma(_e413, _e398.member_2.x, _e298.member_3.x), fma(_e413, _e398.member_2.y, _e298.member_3.y), fma(_e413, _e398.member_2.z, _e298.member_3.z), _e298.member_3.w), _e459.member_4, _e459.member_5, _e459.member_6, _e459.member_7);
                            phi_783_ = true;
                            break;
                        }
                        default: {
                            phi_605_ = type_20();
                            phi_783_ = bool();
                            break;
                        }
                    }
                    let _e486 = phi_605_;
                    let _e488 = phi_783_;
                    continue;
                    continuing {
                        phi_601_ = _e311;
                        phi_604_ = _e486;
                        break if !(_e488);
                    }
                }
                let _e493 = global_3.member[(_e114 + 7u)];
                if (_e493 == 1u) {
                    let _e496 = ((_e94 == 4294967295u) != true);
                    if _e496 {
                        if (_e66 >= 4u) {
                            phi_2370_ = (_e94 <= (_e66 - 4u));
                        } else {
                            phi_2370_ = false;
                        }
                        let _e501 = phi_2370_;
                        if _e501 {
                            let _e504 = global_3.member[_e94];
                            let _e508 = global_3.member[(_e94 + 1u)];
                            let _e512 = global_3.member[(_e94 + 2u)];
                            let _e516 = global_3.member[(_e94 + 3u)];
                            phi_883_ = type_24(type_11(_e504, _e508), type_11(_e512, _e516));
                        } else {
                            phi_883_ = type_24(type_11(4294967295u, 0u), type_11(4294967295u, 0u));
                        }
                        let _e521 = phi_883_;
                        let _e523 = local_4;
                        local = _e523.member_5;
                        phi_886_ = type_11(0u, 4u);
                        phi_889_ = type_14(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        loop {
                            let _e526 = phi_886_;
                            let _e528 = phi_889_;
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
                                phi_887_ = type_11((_e526.member + 1u), _e526.member_1);
                                phi_912_ = type_11(1u, _e526.member);
                            } else {
                                phi_887_ = _e526;
                                phi_912_ = type_11(0u, type_11().member_1);
                            }
                            let _e541 = phi_887_;
                            let _e543 = phi_912_;
                            switch bitcast<i32>(_e543.member) {
                                case 0: {
                                    phi_890_ = type_14();
                                    phi_1314_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e548 = local_5;
                                    local_1 = _e548.member_4;
                                    let _e550 = (_e543.member_1 < 4u);
                                    if _e550 {
                                    } else {
                                        phi_2895_ = true;
                                        break;
                                    }
                                    let _e552 = local_1[_e543.member_1];
                                    if (_e552 >= _e521.member.member_1) {
                                        phi_936_ = 4294967295u;
                                    } else {
                                        phi_936_ = (_e521.member.member + _e552);
                                    }
                                    let _e560 = phi_936_;
                                    if (_e66 >= 1u) {
                                        phi_2404_ = (_e560 <= (_e66 - 1u));
                                    } else {
                                        phi_2404_ = false;
                                    }
                                    let _e565 = phi_2404_;
                                    if _e565 {
                                        let _e568 = global_3.member[_e560];
                                        phi_945_ = _e568;
                                    } else {
                                        phi_945_ = 4294967295u;
                                    }
                                    let _e570 = phi_945_;
                                    if (_e66 >= 10u) {
                                        phi_2428_ = (_e570 <= (_e66 - 10u));
                                    } else {
                                        phi_2428_ = false;
                                    }
                                    let _e575 = phi_2428_;
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
                                        phi_994_ = type_17(vec3<f32>(bitcast<f32>(_e578), bitcast<f32>(_e583), bitcast<f32>(_e588)), vec4<f32>(bitcast<f32>(_e594), bitcast<f32>(_e599), bitcast<f32>(_e604), bitcast<f32>(_e609)), vec3<f32>(bitcast<f32>(_e615), bitcast<f32>(_e620), bitcast<f32>(_e625)));
                                    } else {
                                        phi_994_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e630 = phi_994_;
                                    if (_e552 >= _e521.member_1.member_1) {
                                        phi_1004_ = 4294967295u;
                                    } else {
                                        phi_1004_ = (_e521.member_1.member + (16u * _e552));
                                    }
                                    let _e639 = phi_1004_;
                                    if (_e66 >= 16u) {
                                        phi_2453_ = (_e639 <= (_e66 - 16u));
                                    } else {
                                        phi_2453_ = false;
                                    }
                                    let _e644 = phi_2453_;
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
                                        phi_1077_ = type_14(vec4<f32>(bitcast<f32>(_e647), bitcast<f32>(_e652), bitcast<f32>(_e657), bitcast<f32>(_e662)), vec4<f32>(bitcast<f32>(_e668), bitcast<f32>(_e673), bitcast<f32>(_e678), bitcast<f32>(_e683)), vec4<f32>(bitcast<f32>(_e689), bitcast<f32>(_e694), bitcast<f32>(_e699), bitcast<f32>(_e704)), vec4<f32>(bitcast<f32>(_e710), bitcast<f32>(_e715), bitcast<f32>(_e720), bitcast<f32>(_e725)));
                                    } else {
                                        phi_1077_ = type_14(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                    }
                                    let _e730 = phi_1077_;
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
                                        phi_2895_ = true;
                                        break;
                                    }
                                    let _e874 = local[_e543.member_1];
                                    phi_890_ = type_14((_e528.member + (vec4<f32>(fma(_e630.member.x, _e730.member.w, fma(_e769.x, _e730.member.z, fma(_e765.x, _e730.member.x, (_e767.x * _e730.member.y)))), fma(_e630.member.y, _e730.member.w, fma(_e769.y, _e730.member.z, fma(_e765.y, _e730.member.x, (_e767.y * _e730.member.y)))), fma(_e630.member.z, _e730.member.w, fma(_e769.z, _e730.member.z, fma(_e765.z, _e730.member.x, (_e767.z * _e730.member.y)))), (fma(_e769.w, _e730.member.z, fma(_e765.w, _e730.member.x, (_e767.w * _e730.member.y))) + _e730.member.w)) * _e874)), (_e528.member_1 + (vec4<f32>(fma(_e630.member.x, _e730.member_1.w, fma(_e769.x, _e730.member_1.z, fma(_e765.x, _e730.member_1.x, (_e767.x * _e730.member_1.y)))), fma(_e630.member.y, _e730.member_1.w, fma(_e769.y, _e730.member_1.z, fma(_e765.y, _e730.member_1.x, (_e767.y * _e730.member_1.y)))), fma(_e630.member.z, _e730.member_1.w, fma(_e769.z, _e730.member_1.z, fma(_e765.z, _e730.member_1.x, (_e767.z * _e730.member_1.y)))), (fma(_e769.w, _e730.member_1.z, fma(_e765.w, _e730.member_1.x, (_e767.w * _e730.member_1.y))) + _e730.member_1.w)) * _e874)), (_e528.member_2 + (vec4<f32>(fma(_e630.member.x, _e730.member_2.w, fma(_e769.x, _e730.member_2.z, fma(_e765.x, _e730.member_2.x, (_e767.x * _e730.member_2.y)))), fma(_e630.member.y, _e730.member_2.w, fma(_e769.y, _e730.member_2.z, fma(_e765.y, _e730.member_2.x, (_e767.y * _e730.member_2.y)))), fma(_e630.member.z, _e730.member_2.w, fma(_e769.z, _e730.member_2.z, fma(_e765.z, _e730.member_2.x, (_e767.z * _e730.member_2.y)))), (fma(_e769.w, _e730.member_2.z, fma(_e765.w, _e730.member_2.x, (_e767.w * _e730.member_2.y))) + _e730.member_2.w)) * _e874)), (_e528.member_3 + (vec4<f32>(fma(_e630.member.x, _e730.member_3.w, fma(_e769.x, _e730.member_3.z, fma(_e765.x, _e730.member_3.x, (_e767.x * _e730.member_3.y)))), fma(_e630.member.y, _e730.member_3.w, fma(_e769.y, _e730.member_3.z, fma(_e765.y, _e730.member_3.x, (_e767.y * _e730.member_3.y)))), fma(_e630.member.z, _e730.member_3.w, fma(_e769.z, _e730.member_3.z, fma(_e765.z, _e730.member_3.x, (_e767.z * _e730.member_3.y)))), (fma(_e769.w, _e730.member_3.z, fma(_e765.w, _e730.member_3.x, (_e767.w * _e730.member_3.y))) + _e730.member_3.w)) * _e874)));
                                    phi_1314_ = true;
                                    break;
                                }
                                default: {
                                    phi_890_ = type_14();
                                    phi_1314_ = bool();
                                    break;
                                }
                            }
                            let _e889 = phi_890_;
                            let _e891 = phi_1314_;
                            continue;
                            continuing {
                                phi_886_ = _e541;
                                phi_889_ = _e889;
                                phi_2895_ = false;
                                break if !(_e891);
                            }
                        }
                        let _e894 = phi_2895_;
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
                                    phi_1341_ = (_e920.member.w == _e925);
                                } else {
                                    phi_1341_ = bool();
                                }
                                let _e928 = phi_1341_;
                                phi_1343_ = _e928;
                                phi_1344_ = select(true, false, _e918);
                            } else {
                                phi_1343_ = bool();
                                phi_1344_ = true;
                            }
                            let _e931 = phi_1343_;
                            let _e933 = phi_1344_;
                            phi_1345_ = _e931;
                            phi_1346_ = _e933;
                        } else {
                            phi_1345_ = bool();
                            phi_1346_ = true;
                        }
                        let _e935 = phi_1345_;
                        let _e937 = phi_1346_;
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
                                        phi_1380_ = (_e964.member_1.w == _e969);
                                    } else {
                                        phi_1380_ = bool();
                                    }
                                    let _e972 = phi_1380_;
                                    phi_1382_ = _e972;
                                    phi_1383_ = select(true, false, _e962);
                                } else {
                                    phi_1382_ = bool();
                                    phi_1383_ = true;
                                }
                                let _e975 = phi_1382_;
                                let _e977 = phi_1383_;
                                phi_1384_ = _e975;
                                phi_1385_ = _e977;
                            } else {
                                phi_1384_ = bool();
                                phi_1385_ = true;
                            }
                            let _e979 = phi_1384_;
                            let _e981 = phi_1385_;
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
                                            phi_1419_ = (_e1008.member_2.w == _e1013);
                                        } else {
                                            phi_1419_ = bool();
                                        }
                                        let _e1016 = phi_1419_;
                                        phi_1421_ = _e1016;
                                        phi_1422_ = select(true, false, _e1006);
                                    } else {
                                        phi_1421_ = bool();
                                        phi_1422_ = true;
                                    }
                                    let _e1019 = phi_1421_;
                                    let _e1021 = phi_1422_;
                                    phi_1423_ = _e1019;
                                    phi_1424_ = _e1021;
                                } else {
                                    phi_1423_ = bool();
                                    phi_1424_ = true;
                                }
                                let _e1023 = phi_1423_;
                                let _e1025 = phi_1424_;
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
                                                phi_1458_ = (_e1052.member_3.w == _e1057);
                                            } else {
                                                phi_1458_ = bool();
                                            }
                                            let _e1060 = phi_1458_;
                                            phi_1460_ = _e1060;
                                            phi_1461_ = select(true, false, _e1050);
                                        } else {
                                            phi_1460_ = bool();
                                            phi_1461_ = true;
                                        }
                                        let _e1063 = phi_1460_;
                                        let _e1065 = phi_1461_;
                                        phi_1462_ = _e1063;
                                        phi_1463_ = _e1065;
                                    } else {
                                        phi_1462_ = bool();
                                        phi_1463_ = true;
                                    }
                                    let _e1067 = phi_1462_;
                                    let _e1069 = phi_1463_;
                                    phi_1468_ = select(_e1067, false, _e1069);
                                } else {
                                    phi_1468_ = bool();
                                }
                                let _e1072 = phi_1468_;
                                phi_1470_ = _e1072;
                                phi_1471_ = select(true, false, _e1026);
                            } else {
                                phi_1470_ = bool();
                                phi_1471_ = true;
                            }
                            let _e1075 = phi_1470_;
                            let _e1077 = phi_1471_;
                            phi_1472_ = _e1075;
                            phi_1473_ = _e1077;
                        } else {
                            phi_1472_ = bool();
                            phi_1473_ = true;
                        }
                        let _e1079 = phi_1472_;
                        let _e1081 = phi_1473_;
                        if select(_e1079, false, _e1081) {
                            phi_1481_ = type_14(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        } else {
                            let _e1607 = local_23;
                            phi_1481_ = _e1607;
                        }
                        let _e1084 = phi_1481_;
                        let _e1107 = fma(_e1084.member_2.z, _e1084.member_3.w, -((_e1084.member_2.w * _e1084.member_3.z)));
                        let _e1110 = fma(_e1084.member_2.y, _e1084.member_3.w, -((_e1084.member_2.w * _e1084.member_3.y)));
                        let _e1113 = fma(_e1084.member_2.y, _e1084.member_3.z, -((_e1084.member_2.z * _e1084.member_3.y)));
                        let _e1116 = fma(_e1084.member_2.x, _e1084.member_3.w, -((_e1084.member_2.w * _e1084.member_3.x)));
                        let _e1119 = fma(_e1084.member_2.x, _e1084.member_3.z, -((_e1084.member_2.z * _e1084.member_3.x)));
                        let _e1122 = fma(_e1084.member_2.x, _e1084.member_3.y, -((_e1084.member_2.y * _e1084.member_3.x)));
                        let _e1144 = fma(-(_e1084.member.w), fma(_e1084.member_1.z, _e1122, fma(_e1084.member_1.x, _e1113, -((_e1084.member_1.y * _e1119)))), fma(_e1084.member.z, fma(_e1084.member_1.w, _e1122, fma(_e1084.member_1.x, _e1110, -((_e1084.member_1.y * _e1116)))), fma(_e1084.member.x, fma(_e1084.member_1.w, _e1113, fma(_e1084.member_1.y, _e1107, -((_e1084.member_1.z * _e1110)))), -((_e1084.member.y * fma(_e1084.member_1.w, _e1119, fma(_e1084.member_1.x, _e1107, -((_e1084.member_1.z * _e1116)))))))));
                        if (_e1144 == 0f) {
                            phi_2700_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            phi_2701_ = type_17();
                            phi_2702_ = true;
                        } else {
                            let _e1153 = (sqrt(fma(_e1084.member.w, _e1084.member.w, fma(_e1084.member.z, _e1084.member.z, fma(_e1084.member.x, _e1084.member.x, (_e1084.member.y * _e1084.member.y))))) * select(-1f, 1f, (_e1144 >= 0f)));
                            let _e1158 = sqrt(fma(_e1084.member_1.w, _e1084.member_1.w, fma(_e1084.member_1.z, _e1084.member_1.z, fma(_e1084.member_1.x, _e1084.member_1.x, (_e1084.member_1.y * _e1084.member_1.y)))));
                            let _e1163 = sqrt(fma(_e1084.member_2.w, _e1084.member_2.w, fma(_e1084.member_2.z, _e1084.member_2.z, fma(_e1084.member_2.x, _e1084.member_2.x, (_e1084.member_2.y * _e1084.member_2.y)))));
                            if (_e1153 != 0f) {
                                phi_2588_ = select(true, false, (_e1158 != 0f));
                            } else {
                                phi_2588_ = true;
                            }
                            let _e1170 = phi_2588_;
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
                                        phi_2683_ = vec4<f32>((_e1230 * _e1232), (fma(_e1084.member.y, _e1172, _e1177) * _e1232), (fma(_e1084.member.z, _e1172, _e1178) * _e1232), (fma(_e1084.member_1.z, _e1173, -(_e1179)) * _e1232));
                                    } else {
                                        let _e1218 = (_e1216 + _e1214);
                                        let _e1220 = (0.5f / sqrt(_e1218));
                                        phi_2683_ = vec4<f32>((fma(_e1084.member.y, _e1172, _e1177) * _e1220), (_e1218 * _e1220), (fma(_e1084.member_1.z, _e1173, _e1179) * _e1220), (fma(_e1084.member_2.x, _e1174, -(_e1176)) * _e1220));
                                    }
                                    let _e1243 = phi_2683_;
                                    phi_2685_ = _e1243;
                                } else {
                                    let _e1182 = fma(_e1084.member_1.y, _e1173, _e1175);
                                    let _e1183 = fma(_e1084.member_2.z, _e1174, 1f);
                                    if (_e1182 <= 0f) {
                                        let _e1199 = (_e1183 - _e1182);
                                        let _e1201 = (0.5f / sqrt(_e1199));
                                        phi_2653_ = vec4<f32>((fma(_e1084.member.z, _e1172, _e1178) * _e1201), (fma(_e1084.member_1.z, _e1173, _e1179) * _e1201), (_e1199 * _e1201), (fma(_e1084.member.y, _e1172, -(_e1177)) * _e1201));
                                    } else {
                                        let _e1185 = (_e1183 + _e1182);
                                        let _e1187 = (0.5f / sqrt(_e1185));
                                        phi_2653_ = vec4<f32>((fma(_e1084.member_1.z, _e1173, -(_e1179)) * _e1187), (fma(_e1084.member_2.x, _e1174, -(_e1176)) * _e1187), (fma(_e1084.member.y, _e1172, -(_e1177)) * _e1187), (_e1185 * _e1187));
                                    }
                                    let _e1212 = phi_2653_;
                                    phi_2685_ = _e1212;
                                }
                                let _e1245 = phi_2685_;
                                phi_2694_ = type_17(vec3<f32>(_e1153, _e1158, _e1163), _e1245, vec3<f32>(_e1084.member_3.x, _e1084.member_3.y, _e1084.member_3.z));
                                phi_2695_ = type_17();
                            } else {
                                phi_2694_ = type_17();
                                phi_2695_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e1249 = phi_2694_;
                            let _e1251 = phi_2695_;
                            phi_2700_ = _e1251;
                            phi_2701_ = _e1249;
                            phi_2702_ = select(true, false, _e1171);
                        }
                        let _e1254 = phi_2700_;
                        let _e1256 = phi_2701_;
                        let _e1258 = phi_2702_;
                        if _e1258 {
                            phi_2706_ = _e1254;
                        } else {
                            phi_2706_ = _e1256;
                        }
                        let _e1260 = phi_2706_;
                        phi_1483_ = type_17(_e1260.member_2, _e1260.member_1, _e1260.member);
                    } else {
                        phi_1483_ = type_17();
                    }
                    let _e1266 = phi_1483_;
                    phi_1485_ = _e1266;
                    phi_1486_ = select(true, false, _e496);
                } else {
                    phi_1485_ = type_17();
                    phi_1486_ = true;
                }
                let _e1269 = phi_1485_;
                let _e1271 = phi_1486_;
                if _e1271 {
                    if (_e66 >= 10u) {
                        phi_2800_ = (_e90 <= (_e66 - 10u));
                    } else {
                        phi_2800_ = false;
                    }
                    let _e1276 = phi_2800_;
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
                        phi_1539_ = type_17(vec3<f32>(bitcast<f32>(_e1279), bitcast<f32>(_e1284), bitcast<f32>(_e1289)), vec4<f32>(bitcast<f32>(_e1295), bitcast<f32>(_e1300), bitcast<f32>(_e1305), bitcast<f32>(_e1310)), vec3<f32>(bitcast<f32>(_e1316), bitcast<f32>(_e1321), bitcast<f32>(_e1326)));
                    } else {
                        phi_1539_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e1331 = phi_1539_;
                    phi_1540_ = _e1331;
                } else {
                    phi_1540_ = _e1269;
                }
                let _e1333 = phi_1540_;
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
                    phi_1653_ = 4294967295u;
                } else {
                    phi_1653_ = (_e1411 + (16u * _e1408));
                }
                let _e1420 = phi_1653_;
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
                global_1 = vec4<f32>(100f, 100f, 100f, 1f);
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
