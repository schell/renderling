struct type_8 {
    member: array<u32>,
}

struct type_13 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_14 {
    member: u32,
    member_1: u32,
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
    member: type_14,
    member_1: type_14,
}

var<private> global: u32;
var<private> global_1: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_2: u32;
@group(0) @binding(0) 
var<storage> global_3: type_8;
@group(0) @binding(1) 
var<storage> global_4: type_8;
var<private> global_5: type_13 = type_13(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));

fn function() {
    var local: array<f32, 4>;
    var local_1: array<u32, 4>;
    var local_2: array<f32, 4>;
    var local_3: array<u32, 4>;
    var phi_448_: u32;
    var phi_2533_: bool;
    var phi_455_: u32;
    var phi_456_: u32;
    var phi_466_: u32;
    var phi_548_: type_14;
    var phi_549_: type_14;
    var phi_572_: type_14;
    var phi_585_: bool;
    var phi_591_: type_14;
    var phi_592_: type_14;
    var phi_615_: type_14;
    var phi_629_: bool;
    var phi_635_: type_14;
    var phi_638_: type_20;
    var phi_636_: type_14;
    var phi_661_: type_14;
    var phi_678_: u32;
    var phi_2563_: bool;
    var phi_696_: type_14;
    var phi_2589_: u32;
    var phi_2608_: bool;
    var phi_746_: type_23;
    var phi_756_: u32;
    var phi_2630_: bool;
    var phi_764_: f32;
    var phi_639_: type_20;
    var phi_817_: bool;
    var phi_2653_: bool;
    var phi_938_: type_24;
    var local_4: type_20;
    var phi_941_: type_14;
    var phi_944_: type_13;
    var phi_942_: type_14;
    var phi_967_: type_14;
    var local_5: type_20;
    var phi_991_: u32;
    var phi_2687_: bool;
    var phi_1000_: u32;
    var phi_2711_: bool;
    var phi_1049_: type_17;
    var phi_1059_: u32;
    var phi_2736_: bool;
    var phi_1132_: type_13;
    var phi_945_: type_13;
    var phi_1369_: bool;
    var phi_3273_: bool;
    var local_6: type_13;
    var local_7: type_13;
    var local_8: type_13;
    var local_9: type_13;
    var phi_1396_: bool;
    var phi_1398_: bool;
    var phi_1399_: bool;
    var phi_1400_: bool;
    var phi_1401_: bool;
    var local_10: type_13;
    var local_11: type_13;
    var local_12: type_13;
    var local_13: type_13;
    var phi_1435_: bool;
    var phi_1437_: bool;
    var phi_1438_: bool;
    var phi_1439_: bool;
    var phi_1440_: bool;
    var local_14: type_13;
    var local_15: type_13;
    var local_16: type_13;
    var local_17: type_13;
    var phi_1474_: bool;
    var phi_1476_: bool;
    var phi_1477_: bool;
    var phi_1478_: bool;
    var phi_1479_: bool;
    var local_18: type_13;
    var local_19: type_13;
    var local_20: type_13;
    var local_21: type_13;
    var phi_1513_: bool;
    var phi_1515_: bool;
    var phi_1516_: bool;
    var phi_1517_: bool;
    var phi_1518_: bool;
    var phi_1523_: bool;
    var phi_1525_: bool;
    var phi_1526_: bool;
    var phi_1527_: bool;
    var phi_1528_: bool;
    var phi_1536_: type_13;
    var phi_2871_: bool;
    var phi_2936_: vec4<f32>;
    var phi_2966_: vec4<f32>;
    var phi_2968_: vec4<f32>;
    var phi_2977_: type_17;
    var phi_2978_: type_17;
    var phi_2983_: type_17;
    var phi_2984_: type_17;
    var phi_2985_: bool;
    var phi_2989_: type_17;
    var phi_3319_: bool;
    var phi_1538_: type_17;
    var phi_3318_: bool;
    var phi_1540_: type_17;
    var phi_1541_: bool;
    var phi_3083_: bool;
    var phi_1594_: type_17;
    var phi_1595_: type_17;
    var local_22: type_20;
    var phi_1761_: u32;
    var phi_1792_: u32;
    var phi_1801_: u32;
    var phi_3169_: f32;
    var phi_3182_: bool;
    var phi_1916_: f32;
    var phi_1911_: f32;
    var phi_1917_: f32;
    var phi_3199_: bool;
    var phi_1882_: f32;
    var phi_1919_: f32;
    var phi_3217_: f32;
    var phi_3230_: bool;
    var phi_1972_: f32;
    var phi_1967_: f32;
    var phi_1973_: f32;
    var phi_3247_: bool;
    var phi_1938_: f32;
    var phi_1975_: f32;
    var local_23: type_13;

    switch bitcast<i32>(0u) {
        default: {
            let _e70 = global_2;
            let _e71 = global;
            let _e73 = arrayLength((&global_3.member));
            let _e76 = global_3.member[_e70];
            let _e81 = global_3.member[(_e70 + 1u)];
            let _e85 = global_3.member[(_e70 + 2u)];
            let _e89 = global_3.member[(_e70 + 7u)];
            let _e93 = global_3.member[(_e70 + 8u)];
            let _e97 = global_3.member[(_e70 + 10u)];
            let _e101 = global_3.member[(_e70 + 12u)];
            let _e105 = global_3.member[(_e70 + 13u)];
            let _e109 = global_3.member[(_e70 + 14u)];
            let _e113 = global_3.member[(_e70 + 15u)];
            let _e117 = global_3.member[(_e70 + 16u)];
            let _e121 = global_3.member[(_e70 + 17u)];
            if (_e76 == 1u) {
                if (_e89 == 4294967295u) {
                    phi_456_ = _e71;
                } else {
                    if (_e71 >= _e93) {
                        phi_448_ = 4294967295u;
                    } else {
                        phi_448_ = (_e89 + _e71);
                    }
                    let _e126 = phi_448_;
                    if (_e73 >= 1u) {
                        phi_2533_ = (_e126 <= (_e73 - 1u));
                    } else {
                        phi_2533_ = false;
                    }
                    let _e131 = phi_2533_;
                    if _e131 {
                        let _e134 = global_3.member[_e126];
                        phi_455_ = _e134;
                    } else {
                        phi_455_ = 0u;
                    }
                    let _e136 = phi_455_;
                    phi_456_ = _e136;
                }
                let _e138 = phi_456_;
                if (_e138 >= _e85) {
                    phi_466_ = 4294967295u;
                } else {
                    phi_466_ = (_e81 + (26u * _e138));
                }
                let _e143 = phi_466_;
                let _e146 = global_3.member[_e143];
                let _e151 = global_3.member[(_e143 + 1u)];
                let _e156 = global_3.member[(_e143 + 2u)];
                let _e162 = global_3.member[(_e143 + 3u)];
                let _e167 = global_3.member[(_e143 + 4u)];
                let _e172 = global_3.member[(_e143 + 5u)];
                let _e177 = global_3.member[(_e143 + 6u)];
                let _e183 = global_3.member[(_e143 + 7u)];
                let _e188 = global_3.member[(_e143 + 8u)];
                let _e194 = global_3.member[(_e143 + 9u)];
                let _e199 = global_3.member[(_e143 + 10u)];
                let _e205 = global_3.member[(_e143 + 11u)];
                let _e210 = global_3.member[(_e143 + 12u)];
                let _e215 = global_3.member[(_e143 + 13u)];
                let _e221 = global_3.member[(_e143 + 14u)];
                let _e226 = global_3.member[(_e143 + 15u)];
                let _e231 = global_3.member[(_e143 + 16u)];
                let _e236 = global_3.member[(_e143 + 17u)];
                local_3 = array<u32, 4>(0u, 0u, 0u, 0u);
                phi_548_ = type_14(0u, 4u);
                loop {
                    let _e241 = phi_548_;
                    if (_e241.member < _e241.member_1) {
                        phi_549_ = type_14((_e241.member + 1u), _e241.member_1);
                        phi_572_ = type_14(1u, _e241.member);
                    } else {
                        phi_549_ = _e241;
                        phi_572_ = type_14(0u, type_14().member_1);
                    }
                    let _e254 = phi_549_;
                    let _e256 = phi_572_;
                    switch bitcast<i32>(_e256.member) {
                        case 0: {
                            phi_585_ = false;
                            break;
                        }
                        case 1: {
                            let _e263 = global_3.member[((_e143 + 18u) + _e256.member_1)];
                            local_3[_e256.member_1] = _e263;
                            phi_585_ = true;
                            break;
                        }
                        default: {
                            phi_585_ = bool();
                            break;
                        }
                    }
                    let _e266 = phi_585_;
                    continue;
                    continuing {
                        phi_548_ = _e254;
                        break if !(_e266);
                    }
                }
                let _e268 = local_3;
                local_2 = array<f32, 4>(0f, 0f, 0f, 0f);
                phi_591_ = type_14(0u, 4u);
                loop {
                    let _e271 = phi_591_;
                    if (_e271.member < _e271.member_1) {
                        phi_592_ = type_14((_e271.member + 1u), _e271.member_1);
                        phi_615_ = type_14(1u, _e271.member);
                    } else {
                        phi_592_ = _e271;
                        phi_615_ = type_14(0u, type_14().member_1);
                    }
                    let _e284 = phi_592_;
                    let _e286 = phi_615_;
                    switch bitcast<i32>(_e286.member) {
                        case 0: {
                            phi_629_ = false;
                            break;
                        }
                        case 1: {
                            let _e293 = global_3.member[((_e143 + 22u) + _e286.member_1)];
                            local_2[_e286.member_1] = bitcast<f32>(_e293);
                            phi_629_ = true;
                            break;
                        }
                        default: {
                            phi_629_ = bool();
                            break;
                        }
                    }
                    let _e297 = phi_629_;
                    continue;
                    continuing {
                        phi_591_ = _e284;
                        break if !(_e297);
                    }
                }
                let _e299 = local_2;
                phi_635_ = type_14(0u, _e109);
                phi_638_ = type_20(vec3<f32>(bitcast<f32>(_e146), bitcast<f32>(_e151), bitcast<f32>(_e156)), vec4<f32>(bitcast<f32>(_e162), bitcast<f32>(_e167), bitcast<f32>(_e172), bitcast<f32>(_e177)), vec3<f32>(bitcast<f32>(_e205), bitcast<f32>(_e210), bitcast<f32>(_e215)), vec4<f32>(bitcast<f32>(_e221), bitcast<f32>(_e226), bitcast<f32>(_e231), bitcast<f32>(_e236)), _e268, _e299, vec2<f32>(bitcast<f32>(_e183), bitcast<f32>(_e188)), vec2<f32>(bitcast<f32>(_e194), bitcast<f32>(_e199)));
                loop {
                    let _e303 = phi_635_;
                    let _e305 = phi_638_;
                    local_4 = _e305;
                    local_5 = _e305;
                    local_22 = _e305;
                    if (_e303.member < _e303.member_1) {
                        phi_636_ = type_14((_e303.member + 1u), _e303.member_1);
                        phi_661_ = type_14(1u, _e303.member);
                    } else {
                        phi_636_ = _e303;
                        phi_661_ = type_14(0u, type_14().member_1);
                    }
                    let _e318 = phi_636_;
                    let _e320 = phi_661_;
                    switch bitcast<i32>(_e320.member) {
                        case 0: {
                            phi_639_ = type_20();
                            phi_817_ = false;
                            break;
                        }
                        case 1: {
                            if (_e320.member_1 >= _e109) {
                                phi_678_ = 4294967295u;
                            } else {
                                phi_678_ = (_e105 + (2u * _e320.member_1));
                            }
                            let _e328 = phi_678_;
                            if (_e73 >= 2u) {
                                phi_2563_ = (_e328 <= (_e73 - 2u));
                            } else {
                                phi_2563_ = false;
                            }
                            let _e333 = phi_2563_;
                            if _e333 {
                                let _e336 = global_3.member[_e328];
                                let _e340 = global_3.member[(_e328 + 1u)];
                                phi_696_ = type_14(_e336, _e340);
                            } else {
                                phi_696_ = type_14(4294967295u, 0u);
                            }
                            let _e343 = phi_696_;
                            if (_e138 >= _e343.member_1) {
                                phi_2589_ = 4294967295u;
                            } else {
                                phi_2589_ = (_e343.member + (9u * _e138));
                            }
                            let _e350 = phi_2589_;
                            if (_e73 >= 9u) {
                                phi_2608_ = (_e350 <= (_e73 - 9u));
                            } else {
                                phi_2608_ = false;
                            }
                            let _e355 = phi_2608_;
                            if _e355 {
                                let _e358 = global_3.member[_e350];
                                let _e363 = global_3.member[(_e350 + 1u)];
                                let _e368 = global_3.member[(_e350 + 2u)];
                                let _e374 = global_3.member[(_e350 + 3u)];
                                let _e379 = global_3.member[(_e350 + 4u)];
                                let _e384 = global_3.member[(_e350 + 5u)];
                                let _e390 = global_3.member[(_e350 + 6u)];
                                let _e395 = global_3.member[(_e350 + 7u)];
                                let _e400 = global_3.member[(_e350 + 8u)];
                                phi_746_ = type_23(vec3<f32>(bitcast<f32>(_e358), bitcast<f32>(_e363), bitcast<f32>(_e368)), vec3<f32>(bitcast<f32>(_e374), bitcast<f32>(_e379), bitcast<f32>(_e384)), vec3<f32>(bitcast<f32>(_e390), bitcast<f32>(_e395), bitcast<f32>(_e400)));
                            } else {
                                phi_746_ = type_23(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e405 = phi_746_;
                            if (_e320.member_1 >= _e117) {
                                phi_756_ = 4294967295u;
                            } else {
                                phi_756_ = (_e113 + _e320.member_1);
                            }
                            let _e409 = phi_756_;
                            if (_e73 >= 1u) {
                                phi_2630_ = (_e409 <= (_e73 - 1u));
                            } else {
                                phi_2630_ = false;
                            }
                            let _e414 = phi_2630_;
                            if _e414 {
                                let _e417 = global_3.member[_e409];
                                phi_764_ = bitcast<f32>(_e417);
                            } else {
                                phi_764_ = 0f;
                            }
                            let _e420 = phi_764_;
                            let _e443 = type_20(vec3<f32>(fma(_e420, _e405.member.x, _e305.member.x), fma(_e420, _e405.member.y, _e305.member.y), fma(_e420, _e405.member.z, _e305.member.z)), _e305.member_1, _e305.member_2, _e305.member_3, _e305.member_4, _e305.member_5, _e305.member_6, _e305.member_7);
                            let _e466 = type_20(_e443.member, _e443.member_1, vec3<f32>(fma(_e420, _e405.member_1.x, _e305.member_2.x), fma(_e420, _e405.member_1.y, _e305.member_2.y), fma(_e420, _e405.member_1.z, _e305.member_2.z)), _e443.member_3, _e443.member_4, _e443.member_5, _e443.member_6, _e443.member_7);
                            phi_639_ = type_20(_e466.member, _e466.member_1, _e466.member_2, vec4<f32>(fma(_e420, _e405.member_2.x, _e305.member_3.x), fma(_e420, _e405.member_2.y, _e305.member_3.y), fma(_e420, _e405.member_2.z, _e305.member_3.z), _e305.member_3.w), _e466.member_4, _e466.member_5, _e466.member_6, _e466.member_7);
                            phi_817_ = true;
                            break;
                        }
                        default: {
                            phi_639_ = type_20();
                            phi_817_ = bool();
                            break;
                        }
                    }
                    let _e493 = phi_639_;
                    let _e495 = phi_817_;
                    continue;
                    continuing {
                        phi_635_ = _e318;
                        phi_638_ = _e493;
                        break if !(_e495);
                    }
                }
                let _e500 = global_3.member[(_e121 + 6u)];
                if (_e500 == 1u) {
                    let _e503 = ((_e101 == 4294967295u) != true);
                    if _e503 {
                        if (_e73 >= 4u) {
                            phi_2653_ = (_e101 <= (_e73 - 4u));
                        } else {
                            phi_2653_ = false;
                        }
                        let _e508 = phi_2653_;
                        if _e508 {
                            let _e511 = global_3.member[_e101];
                            let _e515 = global_3.member[(_e101 + 1u)];
                            let _e519 = global_3.member[(_e101 + 2u)];
                            let _e523 = global_3.member[(_e101 + 3u)];
                            phi_938_ = type_24(type_14(_e511, _e515), type_14(_e519, _e523));
                        } else {
                            phi_938_ = type_24(type_14(4294967295u, 0u), type_14(4294967295u, 0u));
                        }
                        let _e528 = phi_938_;
                        let _e530 = local_4;
                        local = _e530.member_5;
                        phi_941_ = type_14(0u, 4u);
                        phi_944_ = type_13(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        loop {
                            let _e533 = phi_941_;
                            let _e535 = phi_944_;
                            local_6 = _e535;
                            local_7 = _e535;
                            local_8 = _e535;
                            local_9 = _e535;
                            local_10 = _e535;
                            local_11 = _e535;
                            local_12 = _e535;
                            local_13 = _e535;
                            local_14 = _e535;
                            local_15 = _e535;
                            local_16 = _e535;
                            local_17 = _e535;
                            local_18 = _e535;
                            local_19 = _e535;
                            local_20 = _e535;
                            local_21 = _e535;
                            local_23 = _e535;
                            if (_e533.member < _e533.member_1) {
                                phi_942_ = type_14((_e533.member + 1u), _e533.member_1);
                                phi_967_ = type_14(1u, _e533.member);
                            } else {
                                phi_942_ = _e533;
                                phi_967_ = type_14(0u, type_14().member_1);
                            }
                            let _e548 = phi_942_;
                            let _e550 = phi_967_;
                            switch bitcast<i32>(_e550.member) {
                                case 0: {
                                    phi_945_ = type_13();
                                    phi_1369_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e555 = local_5;
                                    local_1 = _e555.member_4;
                                    let _e557 = (_e550.member_1 < 4u);
                                    if _e557 {
                                    } else {
                                        phi_3273_ = true;
                                        break;
                                    }
                                    let _e559 = local_1[_e550.member_1];
                                    if (_e559 >= _e528.member.member_1) {
                                        phi_991_ = 4294967295u;
                                    } else {
                                        phi_991_ = (_e528.member.member + _e559);
                                    }
                                    let _e567 = phi_991_;
                                    if (_e73 >= 1u) {
                                        phi_2687_ = (_e567 <= (_e73 - 1u));
                                    } else {
                                        phi_2687_ = false;
                                    }
                                    let _e572 = phi_2687_;
                                    if _e572 {
                                        let _e575 = global_3.member[_e567];
                                        phi_1000_ = _e575;
                                    } else {
                                        phi_1000_ = 4294967295u;
                                    }
                                    let _e577 = phi_1000_;
                                    if (_e73 >= 10u) {
                                        phi_2711_ = (_e577 <= (_e73 - 10u));
                                    } else {
                                        phi_2711_ = false;
                                    }
                                    let _e582 = phi_2711_;
                                    if _e582 {
                                        let _e585 = global_3.member[_e577];
                                        let _e590 = global_3.member[(_e577 + 1u)];
                                        let _e595 = global_3.member[(_e577 + 2u)];
                                        let _e601 = global_3.member[(_e577 + 3u)];
                                        let _e606 = global_3.member[(_e577 + 4u)];
                                        let _e611 = global_3.member[(_e577 + 5u)];
                                        let _e616 = global_3.member[(_e577 + 6u)];
                                        let _e622 = global_3.member[(_e577 + 7u)];
                                        let _e627 = global_3.member[(_e577 + 8u)];
                                        let _e632 = global_3.member[(_e577 + 9u)];
                                        phi_1049_ = type_17(vec3<f32>(bitcast<f32>(_e585), bitcast<f32>(_e590), bitcast<f32>(_e595)), vec4<f32>(bitcast<f32>(_e601), bitcast<f32>(_e606), bitcast<f32>(_e611), bitcast<f32>(_e616)), vec3<f32>(bitcast<f32>(_e622), bitcast<f32>(_e627), bitcast<f32>(_e632)));
                                    } else {
                                        phi_1049_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e637 = phi_1049_;
                                    if (_e559 >= _e528.member_1.member_1) {
                                        phi_1059_ = 4294967295u;
                                    } else {
                                        phi_1059_ = (_e528.member_1.member + (16u * _e559));
                                    }
                                    let _e646 = phi_1059_;
                                    if (_e73 >= 16u) {
                                        phi_2736_ = (_e646 <= (_e73 - 16u));
                                    } else {
                                        phi_2736_ = false;
                                    }
                                    let _e651 = phi_2736_;
                                    if _e651 {
                                        let _e654 = global_3.member[_e646];
                                        let _e659 = global_3.member[(_e646 + 1u)];
                                        let _e664 = global_3.member[(_e646 + 2u)];
                                        let _e669 = global_3.member[(_e646 + 3u)];
                                        let _e675 = global_3.member[(_e646 + 4u)];
                                        let _e680 = global_3.member[(_e646 + 5u)];
                                        let _e685 = global_3.member[(_e646 + 6u)];
                                        let _e690 = global_3.member[(_e646 + 7u)];
                                        let _e696 = global_3.member[(_e646 + 8u)];
                                        let _e701 = global_3.member[(_e646 + 9u)];
                                        let _e706 = global_3.member[(_e646 + 10u)];
                                        let _e711 = global_3.member[(_e646 + 11u)];
                                        let _e717 = global_3.member[(_e646 + 12u)];
                                        let _e722 = global_3.member[(_e646 + 13u)];
                                        let _e727 = global_3.member[(_e646 + 14u)];
                                        let _e732 = global_3.member[(_e646 + 15u)];
                                        phi_1132_ = type_13(vec4<f32>(bitcast<f32>(_e654), bitcast<f32>(_e659), bitcast<f32>(_e664), bitcast<f32>(_e669)), vec4<f32>(bitcast<f32>(_e675), bitcast<f32>(_e680), bitcast<f32>(_e685), bitcast<f32>(_e690)), vec4<f32>(bitcast<f32>(_e696), bitcast<f32>(_e701), bitcast<f32>(_e706), bitcast<f32>(_e711)), vec4<f32>(bitcast<f32>(_e717), bitcast<f32>(_e722), bitcast<f32>(_e727), bitcast<f32>(_e732)));
                                    } else {
                                        phi_1132_ = type_13(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                    }
                                    let _e737 = phi_1132_;
                                    let _e745 = (_e637.member_1.x + _e637.member_1.x);
                                    let _e746 = (_e637.member_1.y + _e637.member_1.y);
                                    let _e747 = (_e637.member_1.z + _e637.member_1.z);
                                    let _e749 = (_e637.member_1.z * _e747);
                                    let _e750 = (_e637.member_1.w * _e745);
                                    let _e751 = (_e637.member_1.w * _e746);
                                    let _e752 = (_e637.member_1.w * _e747);
                                    let _e772 = (vec4<f32>((1f - fma(_e637.member_1.y, _e746, _e749)), fma(_e637.member_1.x, _e746, _e752), fma(_e637.member_1.x, _e747, -(_e751)), 0f) * _e637.member_2.x);
                                    let _e774 = (vec4<f32>(fma(_e637.member_1.x, _e746, -(_e752)), (1f - fma(_e637.member_1.x, _e745, _e749)), fma(_e637.member_1.y, _e747, _e750), 0f) * _e637.member_2.y);
                                    let _e776 = (vec4<f32>(fma(_e637.member_1.x, _e747, _e751), fma(_e637.member_1.y, _e747, -(_e750)), (1f - fma(_e637.member_1.x, _e745, (_e637.member_1.y * _e746))), 0f) * _e637.member_2.z);
                                    if _e557 {
                                    } else {
                                        phi_3273_ = true;
                                        break;
                                    }
                                    let _e881 = local[_e550.member_1];
                                    phi_945_ = type_13((_e535.member + (vec4<f32>(fma(_e637.member.x, _e737.member.w, fma(_e776.x, _e737.member.z, fma(_e772.x, _e737.member.x, (_e774.x * _e737.member.y)))), fma(_e637.member.y, _e737.member.w, fma(_e776.y, _e737.member.z, fma(_e772.y, _e737.member.x, (_e774.y * _e737.member.y)))), fma(_e637.member.z, _e737.member.w, fma(_e776.z, _e737.member.z, fma(_e772.z, _e737.member.x, (_e774.z * _e737.member.y)))), (fma(_e776.w, _e737.member.z, fma(_e772.w, _e737.member.x, (_e774.w * _e737.member.y))) + _e737.member.w)) * _e881)), (_e535.member_1 + (vec4<f32>(fma(_e637.member.x, _e737.member_1.w, fma(_e776.x, _e737.member_1.z, fma(_e772.x, _e737.member_1.x, (_e774.x * _e737.member_1.y)))), fma(_e637.member.y, _e737.member_1.w, fma(_e776.y, _e737.member_1.z, fma(_e772.y, _e737.member_1.x, (_e774.y * _e737.member_1.y)))), fma(_e637.member.z, _e737.member_1.w, fma(_e776.z, _e737.member_1.z, fma(_e772.z, _e737.member_1.x, (_e774.z * _e737.member_1.y)))), (fma(_e776.w, _e737.member_1.z, fma(_e772.w, _e737.member_1.x, (_e774.w * _e737.member_1.y))) + _e737.member_1.w)) * _e881)), (_e535.member_2 + (vec4<f32>(fma(_e637.member.x, _e737.member_2.w, fma(_e776.x, _e737.member_2.z, fma(_e772.x, _e737.member_2.x, (_e774.x * _e737.member_2.y)))), fma(_e637.member.y, _e737.member_2.w, fma(_e776.y, _e737.member_2.z, fma(_e772.y, _e737.member_2.x, (_e774.y * _e737.member_2.y)))), fma(_e637.member.z, _e737.member_2.w, fma(_e776.z, _e737.member_2.z, fma(_e772.z, _e737.member_2.x, (_e774.z * _e737.member_2.y)))), (fma(_e776.w, _e737.member_2.z, fma(_e772.w, _e737.member_2.x, (_e774.w * _e737.member_2.y))) + _e737.member_2.w)) * _e881)), (_e535.member_3 + (vec4<f32>(fma(_e637.member.x, _e737.member_3.w, fma(_e776.x, _e737.member_3.z, fma(_e772.x, _e737.member_3.x, (_e774.x * _e737.member_3.y)))), fma(_e637.member.y, _e737.member_3.w, fma(_e776.y, _e737.member_3.z, fma(_e772.y, _e737.member_3.x, (_e774.y * _e737.member_3.y)))), fma(_e637.member.z, _e737.member_3.w, fma(_e776.z, _e737.member_3.z, fma(_e772.z, _e737.member_3.x, (_e774.z * _e737.member_3.y)))), (fma(_e776.w, _e737.member_3.z, fma(_e772.w, _e737.member_3.x, (_e774.w * _e737.member_3.y))) + _e737.member_3.w)) * _e881)));
                                    phi_1369_ = true;
                                    break;
                                }
                                default: {
                                    phi_945_ = type_13();
                                    phi_1369_ = bool();
                                    break;
                                }
                            }
                            let _e896 = phi_945_;
                            let _e898 = phi_1369_;
                            continue;
                            continuing {
                                phi_941_ = _e548;
                                phi_944_ = _e896;
                                phi_3273_ = false;
                                break if !(_e898);
                            }
                        }
                        let _e901 = phi_3273_;
                        if _e901 {
                            break;
                        }
                        let _e903 = local_6;
                        let _e908 = global_5.member[0u];
                        if (_e903.member.x == _e908) {
                            let _e911 = local_7;
                            let _e916 = global_5.member[1u];
                            if (_e911.member.y == _e916) {
                                let _e919 = local_8;
                                let _e924 = global_5.member[2u];
                                let _e925 = (_e919.member.z == _e924);
                                if _e925 {
                                    let _e927 = local_9;
                                    let _e932 = global_5.member[3u];
                                    phi_1396_ = (_e927.member.w == _e932);
                                } else {
                                    phi_1396_ = bool();
                                }
                                let _e935 = phi_1396_;
                                phi_1398_ = _e935;
                                phi_1399_ = select(true, false, _e925);
                            } else {
                                phi_1398_ = bool();
                                phi_1399_ = true;
                            }
                            let _e938 = phi_1398_;
                            let _e940 = phi_1399_;
                            phi_1400_ = _e938;
                            phi_1401_ = _e940;
                        } else {
                            phi_1400_ = bool();
                            phi_1401_ = true;
                        }
                        let _e942 = phi_1400_;
                        let _e944 = phi_1401_;
                        if select(_e942, false, _e944) {
                            let _e947 = local_10;
                            let _e952 = global_5.member_1[0u];
                            if (_e947.member_1.x == _e952) {
                                let _e955 = local_11;
                                let _e960 = global_5.member_1[1u];
                                if (_e955.member_1.y == _e960) {
                                    let _e963 = local_12;
                                    let _e968 = global_5.member_1[2u];
                                    let _e969 = (_e963.member_1.z == _e968);
                                    if _e969 {
                                        let _e971 = local_13;
                                        let _e976 = global_5.member_1[3u];
                                        phi_1435_ = (_e971.member_1.w == _e976);
                                    } else {
                                        phi_1435_ = bool();
                                    }
                                    let _e979 = phi_1435_;
                                    phi_1437_ = _e979;
                                    phi_1438_ = select(true, false, _e969);
                                } else {
                                    phi_1437_ = bool();
                                    phi_1438_ = true;
                                }
                                let _e982 = phi_1437_;
                                let _e984 = phi_1438_;
                                phi_1439_ = _e982;
                                phi_1440_ = _e984;
                            } else {
                                phi_1439_ = bool();
                                phi_1440_ = true;
                            }
                            let _e986 = phi_1439_;
                            let _e988 = phi_1440_;
                            if select(_e986, false, _e988) {
                                let _e991 = local_14;
                                let _e996 = global_5.member_2[0u];
                                if (_e991.member_2.x == _e996) {
                                    let _e999 = local_15;
                                    let _e1004 = global_5.member_2[1u];
                                    if (_e999.member_2.y == _e1004) {
                                        let _e1007 = local_16;
                                        let _e1012 = global_5.member_2[2u];
                                        let _e1013 = (_e1007.member_2.z == _e1012);
                                        if _e1013 {
                                            let _e1015 = local_17;
                                            let _e1020 = global_5.member_2[3u];
                                            phi_1474_ = (_e1015.member_2.w == _e1020);
                                        } else {
                                            phi_1474_ = bool();
                                        }
                                        let _e1023 = phi_1474_;
                                        phi_1476_ = _e1023;
                                        phi_1477_ = select(true, false, _e1013);
                                    } else {
                                        phi_1476_ = bool();
                                        phi_1477_ = true;
                                    }
                                    let _e1026 = phi_1476_;
                                    let _e1028 = phi_1477_;
                                    phi_1478_ = _e1026;
                                    phi_1479_ = _e1028;
                                } else {
                                    phi_1478_ = bool();
                                    phi_1479_ = true;
                                }
                                let _e1030 = phi_1478_;
                                let _e1032 = phi_1479_;
                                let _e1033 = select(_e1030, false, _e1032);
                                if _e1033 {
                                    let _e1035 = local_18;
                                    let _e1040 = global_5.member_3[0u];
                                    if (_e1035.member_3.x == _e1040) {
                                        let _e1043 = local_19;
                                        let _e1048 = global_5.member_3[1u];
                                        if (_e1043.member_3.y == _e1048) {
                                            let _e1051 = local_20;
                                            let _e1056 = global_5.member_3[2u];
                                            let _e1057 = (_e1051.member_3.z == _e1056);
                                            if _e1057 {
                                                let _e1059 = local_21;
                                                let _e1064 = global_5.member_3[3u];
                                                phi_1513_ = (_e1059.member_3.w == _e1064);
                                            } else {
                                                phi_1513_ = bool();
                                            }
                                            let _e1067 = phi_1513_;
                                            phi_1515_ = _e1067;
                                            phi_1516_ = select(true, false, _e1057);
                                        } else {
                                            phi_1515_ = bool();
                                            phi_1516_ = true;
                                        }
                                        let _e1070 = phi_1515_;
                                        let _e1072 = phi_1516_;
                                        phi_1517_ = _e1070;
                                        phi_1518_ = _e1072;
                                    } else {
                                        phi_1517_ = bool();
                                        phi_1518_ = true;
                                    }
                                    let _e1074 = phi_1517_;
                                    let _e1076 = phi_1518_;
                                    phi_1523_ = select(_e1074, false, _e1076);
                                } else {
                                    phi_1523_ = bool();
                                }
                                let _e1079 = phi_1523_;
                                phi_1525_ = _e1079;
                                phi_1526_ = select(true, false, _e1033);
                            } else {
                                phi_1525_ = bool();
                                phi_1526_ = true;
                            }
                            let _e1082 = phi_1525_;
                            let _e1084 = phi_1526_;
                            phi_1527_ = _e1082;
                            phi_1528_ = _e1084;
                        } else {
                            phi_1527_ = bool();
                            phi_1528_ = true;
                        }
                        let _e1086 = phi_1527_;
                        let _e1088 = phi_1528_;
                        if select(_e1086, false, _e1088) {
                            phi_1536_ = type_13(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        } else {
                            let _e1784 = local_23;
                            phi_1536_ = _e1784;
                        }
                        let _e1091 = phi_1536_;
                        let _e1114 = fma(_e1091.member_2.z, _e1091.member_3.w, -((_e1091.member_2.w * _e1091.member_3.z)));
                        let _e1117 = fma(_e1091.member_2.y, _e1091.member_3.w, -((_e1091.member_2.w * _e1091.member_3.y)));
                        let _e1120 = fma(_e1091.member_2.y, _e1091.member_3.z, -((_e1091.member_2.z * _e1091.member_3.y)));
                        let _e1123 = fma(_e1091.member_2.x, _e1091.member_3.w, -((_e1091.member_2.w * _e1091.member_3.x)));
                        let _e1126 = fma(_e1091.member_2.x, _e1091.member_3.z, -((_e1091.member_2.z * _e1091.member_3.x)));
                        let _e1129 = fma(_e1091.member_2.x, _e1091.member_3.y, -((_e1091.member_2.y * _e1091.member_3.x)));
                        let _e1151 = fma(-(_e1091.member.w), fma(_e1091.member_1.z, _e1129, fma(_e1091.member_1.x, _e1120, -((_e1091.member_1.y * _e1126)))), fma(_e1091.member.z, fma(_e1091.member_1.w, _e1129, fma(_e1091.member_1.x, _e1117, -((_e1091.member_1.y * _e1123)))), fma(_e1091.member.x, fma(_e1091.member_1.w, _e1120, fma(_e1091.member_1.y, _e1114, -((_e1091.member_1.z * _e1117)))), -((_e1091.member.y * fma(_e1091.member_1.w, _e1126, fma(_e1091.member_1.x, _e1114, -((_e1091.member_1.z * _e1123)))))))));
                        if (_e1151 == 0f) {
                            phi_2983_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            phi_2984_ = type_17();
                            phi_2985_ = true;
                        } else {
                            let _e1160 = (sqrt(fma(_e1091.member.w, _e1091.member.w, fma(_e1091.member.z, _e1091.member.z, fma(_e1091.member.x, _e1091.member.x, (_e1091.member.y * _e1091.member.y))))) * select(-1f, 1f, (_e1151 >= 0f)));
                            let _e1165 = sqrt(fma(_e1091.member_1.w, _e1091.member_1.w, fma(_e1091.member_1.z, _e1091.member_1.z, fma(_e1091.member_1.x, _e1091.member_1.x, (_e1091.member_1.y * _e1091.member_1.y)))));
                            let _e1170 = sqrt(fma(_e1091.member_2.w, _e1091.member_2.w, fma(_e1091.member_2.z, _e1091.member_2.z, fma(_e1091.member_2.x, _e1091.member_2.x, (_e1091.member_2.y * _e1091.member_2.y)))));
                            if (_e1160 != 0f) {
                                phi_2871_ = select(true, false, (_e1165 != 0f));
                            } else {
                                phi_2871_ = true;
                            }
                            let _e1177 = phi_2871_;
                            let _e1178 = select((_e1170 != 0f), false, _e1177);
                            if _e1178 {
                                let _e1179 = (1f / _e1160);
                                let _e1180 = (1f / _e1165);
                                let _e1181 = (1f / _e1170);
                                let _e1182 = (_e1091.member.x * _e1179);
                                let _e1183 = (_e1091.member.z * _e1179);
                                let _e1184 = (_e1091.member_1.x * _e1180);
                                let _e1185 = (_e1091.member_2.x * _e1181);
                                let _e1186 = (_e1091.member_2.y * _e1181);
                                if ((_e1091.member_2.z * _e1181) <= 0f) {
                                    let _e1221 = fma(_e1091.member_1.y, _e1180, -(_e1182));
                                    let _e1223 = fma(-(_e1091.member_2.z), _e1181, 1f);
                                    if (_e1221 <= 0f) {
                                        let _e1237 = (_e1223 - _e1221);
                                        let _e1239 = (0.5f / sqrt(_e1237));
                                        phi_2966_ = vec4<f32>((_e1237 * _e1239), (fma(_e1091.member.y, _e1179, _e1184) * _e1239), (fma(_e1091.member.z, _e1179, _e1185) * _e1239), (fma(_e1091.member_1.z, _e1180, -(_e1186)) * _e1239));
                                    } else {
                                        let _e1225 = (_e1223 + _e1221);
                                        let _e1227 = (0.5f / sqrt(_e1225));
                                        phi_2966_ = vec4<f32>((fma(_e1091.member.y, _e1179, _e1184) * _e1227), (_e1225 * _e1227), (fma(_e1091.member_1.z, _e1180, _e1186) * _e1227), (fma(_e1091.member_2.x, _e1181, -(_e1183)) * _e1227));
                                    }
                                    let _e1250 = phi_2966_;
                                    phi_2968_ = _e1250;
                                } else {
                                    let _e1189 = fma(_e1091.member_1.y, _e1180, _e1182);
                                    let _e1190 = fma(_e1091.member_2.z, _e1181, 1f);
                                    if (_e1189 <= 0f) {
                                        let _e1206 = (_e1190 - _e1189);
                                        let _e1208 = (0.5f / sqrt(_e1206));
                                        phi_2936_ = vec4<f32>((fma(_e1091.member.z, _e1179, _e1185) * _e1208), (fma(_e1091.member_1.z, _e1180, _e1186) * _e1208), (_e1206 * _e1208), (fma(_e1091.member.y, _e1179, -(_e1184)) * _e1208));
                                    } else {
                                        let _e1192 = (_e1190 + _e1189);
                                        let _e1194 = (0.5f / sqrt(_e1192));
                                        phi_2936_ = vec4<f32>((fma(_e1091.member_1.z, _e1180, -(_e1186)) * _e1194), (fma(_e1091.member_2.x, _e1181, -(_e1183)) * _e1194), (fma(_e1091.member.y, _e1179, -(_e1184)) * _e1194), (_e1192 * _e1194));
                                    }
                                    let _e1219 = phi_2936_;
                                    phi_2968_ = _e1219;
                                }
                                let _e1252 = phi_2968_;
                                phi_2977_ = type_17(vec3<f32>(_e1160, _e1165, _e1170), _e1252, vec3<f32>(_e1091.member_3.x, _e1091.member_3.y, _e1091.member_3.z));
                                phi_2978_ = type_17();
                            } else {
                                phi_2977_ = type_17();
                                phi_2978_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e1256 = phi_2977_;
                            let _e1258 = phi_2978_;
                            phi_2983_ = _e1258;
                            phi_2984_ = _e1256;
                            phi_2985_ = select(true, false, _e1178);
                        }
                        let _e1261 = phi_2983_;
                        let _e1263 = phi_2984_;
                        let _e1265 = phi_2985_;
                        if _e1265 {
                            phi_2989_ = _e1261;
                        } else {
                            phi_2989_ = _e1263;
                        }
                        let _e1267 = phi_2989_;
                        phi_3319_ = _e901;
                        phi_1538_ = type_17(_e1267.member_2, _e1267.member_1, _e1267.member);
                    } else {
                        phi_3319_ = false;
                        phi_1538_ = type_17();
                    }
                    let _e1273 = phi_3319_;
                    let _e1275 = phi_1538_;
                    phi_3318_ = _e1273;
                    phi_1540_ = _e1275;
                    phi_1541_ = select(true, false, _e503);
                } else {
                    phi_3318_ = false;
                    phi_1540_ = type_17();
                    phi_1541_ = true;
                }
                let _e1278 = phi_3318_;
                let _e1280 = phi_1540_;
                let _e1282 = phi_1541_;
                if _e1282 {
                    if (_e73 >= 10u) {
                        phi_3083_ = (_e97 <= (_e73 - 10u));
                    } else {
                        phi_3083_ = false;
                    }
                    let _e1287 = phi_3083_;
                    if _e1287 {
                        let _e1290 = global_3.member[_e97];
                        let _e1295 = global_3.member[(_e97 + 1u)];
                        let _e1300 = global_3.member[(_e97 + 2u)];
                        let _e1306 = global_3.member[(_e97 + 3u)];
                        let _e1311 = global_3.member[(_e97 + 4u)];
                        let _e1316 = global_3.member[(_e97 + 5u)];
                        let _e1321 = global_3.member[(_e97 + 6u)];
                        let _e1327 = global_3.member[(_e97 + 7u)];
                        let _e1332 = global_3.member[(_e97 + 8u)];
                        let _e1337 = global_3.member[(_e97 + 9u)];
                        phi_1594_ = type_17(vec3<f32>(bitcast<f32>(_e1290), bitcast<f32>(_e1295), bitcast<f32>(_e1300)), vec4<f32>(bitcast<f32>(_e1306), bitcast<f32>(_e1311), bitcast<f32>(_e1316), bitcast<f32>(_e1321)), vec3<f32>(bitcast<f32>(_e1327), bitcast<f32>(_e1332), bitcast<f32>(_e1337)));
                    } else {
                        phi_1594_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e1342 = phi_1594_;
                    phi_1595_ = _e1342;
                } else {
                    phi_1595_ = _e1280;
                }
                let _e1344 = phi_1595_;
                let _e1352 = (_e1344.member_1.x + _e1344.member_1.x);
                let _e1353 = (_e1344.member_1.y + _e1344.member_1.y);
                let _e1354 = (_e1344.member_1.z + _e1344.member_1.z);
                let _e1356 = (_e1344.member_1.z * _e1354);
                let _e1357 = (_e1344.member_1.w * _e1352);
                let _e1358 = (_e1344.member_1.w * _e1353);
                let _e1359 = (_e1344.member_1.w * _e1354);
                let _e1379 = (vec4<f32>((1f - fma(_e1344.member_1.y, _e1353, _e1356)), fma(_e1344.member_1.x, _e1353, _e1359), fma(_e1344.member_1.x, _e1354, -(_e1358)), 0f) * _e1344.member_2.x);
                let _e1381 = (vec4<f32>(fma(_e1344.member_1.x, _e1353, -(_e1359)), (1f - fma(_e1344.member_1.x, _e1352, _e1356)), fma(_e1344.member_1.y, _e1354, _e1357), 0f) * _e1344.member_2.y);
                let _e1383 = (vec4<f32>(fma(_e1344.member_1.x, _e1354, _e1358), fma(_e1344.member_1.y, _e1354, -(_e1357)), (1f - fma(_e1344.member_1.x, _e1352, (_e1344.member_1.y * _e1353))), 0f) * _e1344.member_2.z);
                let _e1388 = local_22;
                let _e1411 = (_e1344.member.x + fma(_e1383.x, _e1388.member.z, fma(_e1381.x, _e1388.member.y, (_e1379.x * _e1388.member.x))));
                let _e1412 = (_e1344.member.y + fma(_e1383.y, _e1388.member.z, fma(_e1381.y, _e1388.member.y, (_e1379.y * _e1388.member.x))));
                let _e1413 = (_e1344.member.z + fma(_e1383.z, _e1388.member.z, fma(_e1381.z, _e1388.member.y, (_e1379.z * _e1388.member.x))));
                let _e1416 = global_4.member[2u];
                let _e1419 = global_4.member[3u];
                let _e1422 = global_4.member[4u];
                let _e1425 = global_4.member[_e1416];
                let _e1429 = global_4.member[(_e1416 + 1u)];
                let _e1432 = global_4.member[_e1419];
                let _e1437 = global_4.member[(_e1419 + 1u)];
                let _e1442 = global_4.member[(_e1419 + 2u)];
                let _e1447 = global_4.member[(_e1419 + 3u)];
                let _e1452 = global_4.member[(_e1419 + 4u)];
                let _e1457 = global_4.member[(_e1419 + 5u)];
                let _e1462 = global_4.member[(_e1419 + 6u)];
                let _e1467 = global_4.member[(_e1419 + 7u)];
                let _e1472 = global_4.member[(_e1419 + 8u)];
                let _e1477 = global_4.member[(_e1419 + 9u)];
                let _e1482 = global_4.member[(_e1419 + 10u)];
                let _e1487 = global_4.member[(_e1419 + 11u)];
                let _e1492 = global_4.member[(_e1419 + 12u)];
                let _e1497 = global_4.member[(_e1419 + 13u)];
                let _e1502 = global_4.member[(_e1419 + 14u)];
                let _e1507 = global_4.member[(_e1419 + 15u)];
                let _e1512 = global_4.member[(_e1419 + 16u)];
                let _e1516 = global_4.member[(_e1419 + 17u)];
                if (_e1422 >= _e1516) {
                    phi_1761_ = 4294967295u;
                } else {
                    phi_1761_ = (_e1512 + _e1422);
                }
                let _e1520 = phi_1761_;
                let _e1523 = global_4.member[_e1520];
                let _e1526 = global_4.member[_e1523];
                let _e1530 = global_4.member[(_e1523 + 1u)];
                let _e1534 = global_4.member[(_e1523 + 2u)];
                let _e1538 = global_4.member[(_e1523 + 3u)];
                let _e1542 = global_4.member[(_e1523 + 6u)];
                switch bitcast<i32>(_e1542) {
                    case 0: {
                        phi_1792_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1792_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1792_ = 2u;
                        break;
                    }
                    default: {
                        phi_1792_ = 0u;
                        break;
                    }
                }
                let _e1545 = phi_1792_;
                let _e1549 = global_4.member[(_e1523 + 7u)];
                switch bitcast<i32>(_e1549) {
                    case 0: {
                        phi_1801_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1801_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1801_ = 2u;
                        break;
                    }
                    default: {
                        phi_1801_ = 0u;
                        break;
                    }
                }
                let _e1552 = phi_1801_;
                let _e1571 = (((fma(bitcast<f32>(_e1472), _e1413, fma(bitcast<f32>(_e1432), _e1411, (bitcast<f32>(_e1452) * _e1412))) + bitcast<f32>(_e1492)) + 1f) * 0.5f);
                let _e1572 = (((fma(bitcast<f32>(_e1477), _e1413, fma(bitcast<f32>(_e1437), _e1411, (bitcast<f32>(_e1457) * _e1412))) + bitcast<f32>(_e1497)) + 1f) * -0.5f);
                switch bitcast<i32>(_e1545) {
                    case 1: {
                        let _e1607 = abs(_e1571);
                        let _e1609 = (_e1607 % 1f);
                        if (_e1607 >= 1f) {
                            phi_3199_ = select(true, false, (_e1609 == 0f));
                        } else {
                            phi_3199_ = true;
                        }
                        let _e1613 = phi_3199_;
                        let _e1614 = select(1f, _e1609, _e1613);
                        if (select(-1f, 1f, (_e1571 >= 0f)) > 0f) {
                            phi_1882_ = _e1614;
                        } else {
                            phi_1882_ = (1f - _e1614);
                        }
                        let _e1618 = phi_1882_;
                        phi_1919_ = _e1618;
                        break;
                    }
                    case 2: {
                        let _e1581 = abs(_e1571);
                        let _e1588 = ((select(select(u32(_e1581), 0u, (_e1581 < 0f)), 4294967295u, (_e1581 > 4294967000f)) % 2u) == 0u);
                        let _e1590 = (_e1581 % 1f);
                        if (_e1581 >= 1f) {
                            phi_3182_ = select(true, false, (_e1590 == 0f));
                        } else {
                            phi_3182_ = true;
                        }
                        let _e1594 = phi_3182_;
                        let _e1595 = select(1f, _e1590, _e1594);
                        if (select(-1f, 1f, (_e1571 >= 0f)) > 0f) {
                            if _e1588 {
                                phi_1911_ = _e1595;
                            } else {
                                phi_1911_ = (1f - _e1595);
                            }
                            let _e1602 = phi_1911_;
                            phi_1917_ = _e1602;
                        } else {
                            if _e1588 {
                                phi_1916_ = (1f - _e1595);
                            } else {
                                phi_1916_ = _e1595;
                            }
                            let _e1599 = phi_1916_;
                            phi_1917_ = _e1599;
                        }
                        let _e1604 = phi_1917_;
                        phi_1919_ = _e1604;
                        break;
                    }
                    case 0: {
                        if (_e1571 > 1f) {
                            phi_3169_ = 0.9999999f;
                        } else {
                            phi_3169_ = select(_e1571, 0.00000011920929f, (_e1571 < 0f));
                        }
                        let _e1578 = phi_3169_;
                        phi_1919_ = _e1578;
                        break;
                    }
                    default: {
                        phi_1919_ = f32();
                        break;
                    }
                }
                let _e1620 = phi_1919_;
                if _e1278 {
                    break;
                }
                switch bitcast<i32>(_e1552) {
                    case 1: {
                        let _e1655 = abs(_e1572);
                        let _e1657 = (_e1655 % 1f);
                        if (_e1655 >= 1f) {
                            phi_3247_ = select(true, false, (_e1657 == 0f));
                        } else {
                            phi_3247_ = true;
                        }
                        let _e1661 = phi_3247_;
                        let _e1662 = select(1f, _e1657, _e1661);
                        if (select(-1f, 1f, (_e1572 >= 0f)) > 0f) {
                            phi_1938_ = _e1662;
                        } else {
                            phi_1938_ = (1f - _e1662);
                        }
                        let _e1666 = phi_1938_;
                        phi_1975_ = _e1666;
                        break;
                    }
                    case 2: {
                        let _e1629 = abs(_e1572);
                        let _e1636 = ((select(select(u32(_e1629), 0u, (_e1629 < 0f)), 4294967295u, (_e1629 > 4294967000f)) % 2u) == 0u);
                        let _e1638 = (_e1629 % 1f);
                        if (_e1629 >= 1f) {
                            phi_3230_ = select(true, false, (_e1638 == 0f));
                        } else {
                            phi_3230_ = true;
                        }
                        let _e1642 = phi_3230_;
                        let _e1643 = select(1f, _e1638, _e1642);
                        if (select(-1f, 1f, (_e1572 >= 0f)) > 0f) {
                            if _e1636 {
                                phi_1967_ = _e1643;
                            } else {
                                phi_1967_ = (1f - _e1643);
                            }
                            let _e1650 = phi_1967_;
                            phi_1973_ = _e1650;
                        } else {
                            if _e1636 {
                                phi_1972_ = (1f - _e1643);
                            } else {
                                phi_1972_ = _e1643;
                            }
                            let _e1647 = phi_1972_;
                            phi_1973_ = _e1647;
                        }
                        let _e1652 = phi_1973_;
                        phi_1975_ = _e1652;
                        break;
                    }
                    case 0: {
                        if (_e1572 > 1f) {
                            phi_3217_ = 0.9999999f;
                        } else {
                            phi_3217_ = select(_e1572, 0.00000011920929f, (_e1572 < 0f));
                        }
                        let _e1626 = phi_3217_;
                        phi_1975_ = _e1626;
                        break;
                    }
                    default: {
                        phi_1975_ = f32();
                        break;
                    }
                }
                let _e1668 = phi_1975_;
                if _e1278 {
                    break;
                }
                let _e1670 = (_e1620 * f32(_e1534));
                let _e1677 = (_e1668 * f32(_e1538));
                global_1 = vec4<f32>(fma((f32((select(select(u32(_e1670), 0u, (_e1670 < 0f)), 4294967295u, (_e1670 > 4294967000f)) + _e1526)) / f32(_e1425)), 2f, -1f), fma((f32((select(select(u32(_e1677), 0u, (_e1677 < 0f)), 4294967295u, (_e1677 > 4294967000f)) + _e1530)) / f32(_e1429)), -2f, -1f), (fma(bitcast<f32>(_e1482), _e1413, fma(bitcast<f32>(_e1442), _e1411, (bitcast<f32>(_e1462) * _e1412))) + bitcast<f32>(_e1502)), (fma(bitcast<f32>(_e1487), _e1413, fma(bitcast<f32>(_e1447), _e1411, (bitcast<f32>(_e1467) * _e1412))) + bitcast<f32>(_e1507)));
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
