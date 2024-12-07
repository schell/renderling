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

struct type_18 {
    member: u32,
}

struct type_22 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
    member_3: vec4<f32>,
    member_4: array<u32, 4>,
    member_5: array<f32, 4>,
    member_6: vec2<f32>,
    member_7: vec2<f32>,
}

struct type_25 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_26 {
    member: type_14,
    member_1: type_14,
}

var<private> global: u32;
var<private> global_1: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
@group(0) @binding(0) 
var<storage> global_2: type_8;
var<private> global_3: u32;
@group(0) @binding(1) 
var<storage> global_4: type_18;
var<private> global_5: type_13 = type_13(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));

fn function() {
    var local: array<f32, 4>;
    var local_1: array<u32, 4>;
    var local_2: array<f32, 4>;
    var local_3: array<u32, 4>;
    var phi_437_: u32;
    var phi_2929_: bool;
    var phi_444_: u32;
    var phi_445_: u32;
    var phi_455_: u32;
    var phi_537_: type_14;
    var phi_538_: type_14;
    var phi_553_: type_14;
    var phi_566_: bool;
    var phi_572_: type_14;
    var phi_573_: type_14;
    var phi_588_: type_14;
    var phi_602_: bool;
    var phi_608_: type_14;
    var phi_611_: type_22;
    var phi_609_: type_14;
    var phi_626_: type_14;
    var phi_643_: u32;
    var phi_2959_: bool;
    var phi_661_: type_14;
    var phi_2985_: u32;
    var phi_3004_: bool;
    var phi_711_: type_25;
    var phi_721_: u32;
    var phi_3026_: bool;
    var phi_729_: f32;
    var phi_612_: type_22;
    var phi_782_: bool;
    var phi_3049_: bool;
    var phi_903_: type_26;
    var local_4: type_22;
    var phi_906_: type_14;
    var phi_909_: type_13;
    var phi_907_: type_14;
    var phi_924_: type_14;
    var local_5: type_22;
    var phi_948_: u32;
    var phi_3083_: bool;
    var phi_957_: u32;
    var phi_3107_: bool;
    var phi_1006_: type_17;
    var phi_1016_: u32;
    var phi_3132_: bool;
    var phi_1089_: type_13;
    var phi_910_: type_13;
    var phi_1326_: bool;
    var phi_3632_: bool;
    var local_6: type_13;
    var local_7: type_13;
    var local_8: type_13;
    var local_9: type_13;
    var phi_1353_: bool;
    var phi_1355_: bool;
    var phi_1356_: bool;
    var phi_1357_: bool;
    var phi_1358_: bool;
    var local_10: type_13;
    var local_11: type_13;
    var local_12: type_13;
    var local_13: type_13;
    var phi_1392_: bool;
    var phi_1394_: bool;
    var phi_1395_: bool;
    var phi_1396_: bool;
    var phi_1397_: bool;
    var local_14: type_13;
    var local_15: type_13;
    var local_16: type_13;
    var local_17: type_13;
    var phi_1431_: bool;
    var phi_1433_: bool;
    var phi_1434_: bool;
    var phi_1435_: bool;
    var phi_1436_: bool;
    var local_18: type_13;
    var local_19: type_13;
    var local_20: type_13;
    var local_21: type_13;
    var phi_1470_: bool;
    var phi_1472_: bool;
    var phi_1473_: bool;
    var phi_1474_: bool;
    var phi_1475_: bool;
    var phi_1480_: bool;
    var phi_1482_: bool;
    var phi_1483_: bool;
    var phi_1484_: bool;
    var phi_1485_: bool;
    var phi_1493_: type_13;
    var phi_3269_: bool;
    var phi_3332_: vec4<f32>;
    var phi_3362_: vec4<f32>;
    var phi_3364_: vec4<f32>;
    var phi_3375_: type_17;
    var phi_3376_: type_17;
    var phi_3379_: type_17;
    var phi_3380_: type_17;
    var phi_3381_: bool;
    var phi_3385_: type_17;
    var phi_1495_: type_17;
    var phi_1497_: type_17;
    var phi_1498_: bool;
    var phi_3479_: bool;
    var phi_1551_: type_17;
    var phi_1552_: type_17;
    var local_22: type_22;
    var phi_1600_: u32;
    var phi_3550_: bool;
    var phi_1656_: type_17;
    var phi_2403_: type_13;
    var local_23: type_13;

    switch bitcast<i32>(0u) {
        default: {
            let _e70 = global_3;
            let _e71 = global;
            let _e73 = arrayLength((&global_2.member));
            let _e77 = global_2.member[_e70];
            let _e82 = global_2.member[(_e70 + 1u)];
            let _e86 = global_2.member[(_e70 + 2u)];
            let _e90 = global_2.member[(_e70 + 7u)];
            let _e94 = global_2.member[(_e70 + 8u)];
            let _e98 = global_2.member[(_e70 + 10u)];
            let _e102 = global_2.member[(_e70 + 12u)];
            let _e106 = global_2.member[(_e70 + 13u)];
            let _e110 = global_2.member[(_e70 + 14u)];
            let _e114 = global_2.member[(_e70 + 15u)];
            let _e118 = global_2.member[(_e70 + 16u)];
            let _e122 = global_2.member[(_e70 + 17u)];
            if (_e77 == 1u) {
                if (_e90 == 4294967295u) {
                    phi_445_ = _e71;
                } else {
                    if (_e71 >= _e94) {
                        phi_437_ = 4294967295u;
                    } else {
                        phi_437_ = (_e90 + _e71);
                    }
                    let _e127 = phi_437_;
                    if (_e73 >= 1u) {
                        phi_2929_ = (_e127 <= (_e73 - 1u));
                    } else {
                        phi_2929_ = false;
                    }
                    let _e132 = phi_2929_;
                    if _e132 {
                        let _e135 = global_2.member[_e127];
                        phi_444_ = _e135;
                    } else {
                        phi_444_ = 0u;
                    }
                    let _e137 = phi_444_;
                    phi_445_ = _e137;
                }
                let _e139 = phi_445_;
                if (_e139 >= _e86) {
                    phi_455_ = 4294967295u;
                } else {
                    phi_455_ = (_e82 + (26u * _e139));
                }
                let _e144 = phi_455_;
                let _e147 = global_2.member[_e144];
                let _e152 = global_2.member[(_e144 + 1u)];
                let _e157 = global_2.member[(_e144 + 2u)];
                let _e163 = global_2.member[(_e144 + 3u)];
                let _e168 = global_2.member[(_e144 + 4u)];
                let _e173 = global_2.member[(_e144 + 5u)];
                let _e178 = global_2.member[(_e144 + 6u)];
                let _e184 = global_2.member[(_e144 + 7u)];
                let _e189 = global_2.member[(_e144 + 8u)];
                let _e195 = global_2.member[(_e144 + 9u)];
                let _e200 = global_2.member[(_e144 + 10u)];
                let _e206 = global_2.member[(_e144 + 11u)];
                let _e211 = global_2.member[(_e144 + 12u)];
                let _e216 = global_2.member[(_e144 + 13u)];
                let _e222 = global_2.member[(_e144 + 14u)];
                let _e227 = global_2.member[(_e144 + 15u)];
                let _e232 = global_2.member[(_e144 + 16u)];
                let _e237 = global_2.member[(_e144 + 17u)];
                local_3 = array<u32, 4>(0u, 0u, 0u, 0u);
                phi_537_ = type_14(0u, 4u);
                loop {
                    let _e242 = phi_537_;
                    if (_e242.member < _e242.member_1) {
                        phi_538_ = type_14((_e242.member + 1u), _e242.member_1);
                        phi_553_ = type_14(1u, _e242.member);
                    } else {
                        phi_538_ = _e242;
                        phi_553_ = type_14(0u, type_14().member_1);
                    }
                    let _e255 = phi_538_;
                    let _e257 = phi_553_;
                    switch bitcast<i32>(_e257.member) {
                        case 0: {
                            phi_566_ = false;
                            break;
                        }
                        case 1: {
                            let _e264 = global_2.member[((_e144 + 18u) + _e257.member_1)];
                            local_3[_e257.member_1] = _e264;
                            phi_566_ = true;
                            break;
                        }
                        default: {
                            phi_566_ = bool();
                            break;
                        }
                    }
                    let _e267 = phi_566_;
                    continue;
                    continuing {
                        phi_537_ = _e255;
                        break if !(_e267);
                    }
                }
                let _e269 = local_3;
                local_2 = array<f32, 4>(0f, 0f, 0f, 0f);
                phi_572_ = type_14(0u, 4u);
                loop {
                    let _e272 = phi_572_;
                    if (_e272.member < _e272.member_1) {
                        phi_573_ = type_14((_e272.member + 1u), _e272.member_1);
                        phi_588_ = type_14(1u, _e272.member);
                    } else {
                        phi_573_ = _e272;
                        phi_588_ = type_14(0u, type_14().member_1);
                    }
                    let _e285 = phi_573_;
                    let _e287 = phi_588_;
                    switch bitcast<i32>(_e287.member) {
                        case 0: {
                            phi_602_ = false;
                            break;
                        }
                        case 1: {
                            let _e294 = global_2.member[((_e144 + 22u) + _e287.member_1)];
                            local_2[_e287.member_1] = bitcast<f32>(_e294);
                            phi_602_ = true;
                            break;
                        }
                        default: {
                            phi_602_ = bool();
                            break;
                        }
                    }
                    let _e298 = phi_602_;
                    continue;
                    continuing {
                        phi_572_ = _e285;
                        break if !(_e298);
                    }
                }
                let _e300 = local_2;
                phi_608_ = type_14(0u, _e110);
                phi_611_ = type_22(vec3<f32>(bitcast<f32>(_e147), bitcast<f32>(_e152), bitcast<f32>(_e157)), vec4<f32>(bitcast<f32>(_e163), bitcast<f32>(_e168), bitcast<f32>(_e173), bitcast<f32>(_e178)), vec3<f32>(bitcast<f32>(_e206), bitcast<f32>(_e211), bitcast<f32>(_e216)), vec4<f32>(bitcast<f32>(_e222), bitcast<f32>(_e227), bitcast<f32>(_e232), bitcast<f32>(_e237)), _e269, _e300, vec2<f32>(bitcast<f32>(_e184), bitcast<f32>(_e189)), vec2<f32>(bitcast<f32>(_e195), bitcast<f32>(_e200)));
                loop {
                    let _e304 = phi_608_;
                    let _e306 = phi_611_;
                    local_4 = _e306;
                    local_5 = _e306;
                    local_22 = _e306;
                    if (_e304.member < _e304.member_1) {
                        phi_609_ = type_14((_e304.member + 1u), _e304.member_1);
                        phi_626_ = type_14(1u, _e304.member);
                    } else {
                        phi_609_ = _e304;
                        phi_626_ = type_14(0u, type_14().member_1);
                    }
                    let _e319 = phi_609_;
                    let _e321 = phi_626_;
                    switch bitcast<i32>(_e321.member) {
                        case 0: {
                            phi_612_ = type_22();
                            phi_782_ = false;
                            break;
                        }
                        case 1: {
                            if (_e321.member_1 >= _e110) {
                                phi_643_ = 4294967295u;
                            } else {
                                phi_643_ = (_e106 + (2u * _e321.member_1));
                            }
                            let _e329 = phi_643_;
                            if (_e73 >= 2u) {
                                phi_2959_ = (_e329 <= (_e73 - 2u));
                            } else {
                                phi_2959_ = false;
                            }
                            let _e334 = phi_2959_;
                            if _e334 {
                                let _e337 = global_2.member[_e329];
                                let _e341 = global_2.member[(_e329 + 1u)];
                                phi_661_ = type_14(_e337, _e341);
                            } else {
                                phi_661_ = type_14(4294967295u, 0u);
                            }
                            let _e344 = phi_661_;
                            if (_e139 >= _e344.member_1) {
                                phi_2985_ = 4294967295u;
                            } else {
                                phi_2985_ = (_e344.member + (9u * _e139));
                            }
                            let _e351 = phi_2985_;
                            if (_e73 >= 9u) {
                                phi_3004_ = (_e351 <= (_e73 - 9u));
                            } else {
                                phi_3004_ = false;
                            }
                            let _e356 = phi_3004_;
                            if _e356 {
                                let _e359 = global_2.member[_e351];
                                let _e364 = global_2.member[(_e351 + 1u)];
                                let _e369 = global_2.member[(_e351 + 2u)];
                                let _e375 = global_2.member[(_e351 + 3u)];
                                let _e380 = global_2.member[(_e351 + 4u)];
                                let _e385 = global_2.member[(_e351 + 5u)];
                                let _e391 = global_2.member[(_e351 + 6u)];
                                let _e396 = global_2.member[(_e351 + 7u)];
                                let _e401 = global_2.member[(_e351 + 8u)];
                                phi_711_ = type_25(vec3<f32>(bitcast<f32>(_e359), bitcast<f32>(_e364), bitcast<f32>(_e369)), vec3<f32>(bitcast<f32>(_e375), bitcast<f32>(_e380), bitcast<f32>(_e385)), vec3<f32>(bitcast<f32>(_e391), bitcast<f32>(_e396), bitcast<f32>(_e401)));
                            } else {
                                phi_711_ = type_25(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e406 = phi_711_;
                            if (_e321.member_1 >= _e118) {
                                phi_721_ = 4294967295u;
                            } else {
                                phi_721_ = (_e114 + _e321.member_1);
                            }
                            let _e410 = phi_721_;
                            if (_e73 >= 1u) {
                                phi_3026_ = (_e410 <= (_e73 - 1u));
                            } else {
                                phi_3026_ = false;
                            }
                            let _e415 = phi_3026_;
                            if _e415 {
                                let _e418 = global_2.member[_e410];
                                phi_729_ = bitcast<f32>(_e418);
                            } else {
                                phi_729_ = 0f;
                            }
                            let _e421 = phi_729_;
                            let _e444 = type_22(vec3<f32>(fma(_e421, _e406.member.x, _e306.member.x), fma(_e421, _e406.member.y, _e306.member.y), fma(_e421, _e406.member.z, _e306.member.z)), _e306.member_1, _e306.member_2, _e306.member_3, _e306.member_4, _e306.member_5, _e306.member_6, _e306.member_7);
                            let _e467 = type_22(_e444.member, _e444.member_1, vec3<f32>(fma(_e421, _e406.member_1.x, _e306.member_2.x), fma(_e421, _e406.member_1.y, _e306.member_2.y), fma(_e421, _e406.member_1.z, _e306.member_2.z)), _e444.member_3, _e444.member_4, _e444.member_5, _e444.member_6, _e444.member_7);
                            phi_612_ = type_22(_e467.member, _e467.member_1, _e467.member_2, vec4<f32>(fma(_e421, _e406.member_2.x, _e306.member_3.x), fma(_e421, _e406.member_2.y, _e306.member_3.y), fma(_e421, _e406.member_2.z, _e306.member_3.z), _e306.member_3.w), _e467.member_4, _e467.member_5, _e467.member_6, _e467.member_7);
                            phi_782_ = true;
                            break;
                        }
                        default: {
                            phi_612_ = type_22();
                            phi_782_ = bool();
                            break;
                        }
                    }
                    let _e494 = phi_612_;
                    let _e496 = phi_782_;
                    continue;
                    continuing {
                        phi_608_ = _e319;
                        phi_611_ = _e494;
                        break if !(_e496);
                    }
                }
                let _e501 = global_2.member[(_e122 + 6u)];
                if (_e501 == 1u) {
                    let _e504 = ((_e102 == 4294967295u) != true);
                    if _e504 {
                        if (_e73 >= 4u) {
                            phi_3049_ = (_e102 <= (_e73 - 4u));
                        } else {
                            phi_3049_ = false;
                        }
                        let _e509 = phi_3049_;
                        if _e509 {
                            let _e512 = global_2.member[_e102];
                            let _e516 = global_2.member[(_e102 + 1u)];
                            let _e520 = global_2.member[(_e102 + 2u)];
                            let _e524 = global_2.member[(_e102 + 3u)];
                            phi_903_ = type_26(type_14(_e512, _e516), type_14(_e520, _e524));
                        } else {
                            phi_903_ = type_26(type_14(4294967295u, 0u), type_14(4294967295u, 0u));
                        }
                        let _e529 = phi_903_;
                        let _e531 = local_4;
                        local = _e531.member_5;
                        phi_906_ = type_14(0u, 4u);
                        phi_909_ = type_13(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        loop {
                            let _e534 = phi_906_;
                            let _e536 = phi_909_;
                            local_6 = _e536;
                            local_7 = _e536;
                            local_8 = _e536;
                            local_9 = _e536;
                            local_10 = _e536;
                            local_11 = _e536;
                            local_12 = _e536;
                            local_13 = _e536;
                            local_14 = _e536;
                            local_15 = _e536;
                            local_16 = _e536;
                            local_17 = _e536;
                            local_18 = _e536;
                            local_19 = _e536;
                            local_20 = _e536;
                            local_21 = _e536;
                            local_23 = _e536;
                            if (_e534.member < _e534.member_1) {
                                phi_907_ = type_14((_e534.member + 1u), _e534.member_1);
                                phi_924_ = type_14(1u, _e534.member);
                            } else {
                                phi_907_ = _e534;
                                phi_924_ = type_14(0u, type_14().member_1);
                            }
                            let _e549 = phi_907_;
                            let _e551 = phi_924_;
                            switch bitcast<i32>(_e551.member) {
                                case 0: {
                                    phi_910_ = type_13();
                                    phi_1326_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e556 = local_5;
                                    local_1 = _e556.member_4;
                                    let _e558 = (_e551.member_1 < 4u);
                                    if _e558 {
                                    } else {
                                        phi_3632_ = true;
                                        break;
                                    }
                                    let _e560 = local_1[_e551.member_1];
                                    if (_e560 >= _e529.member.member_1) {
                                        phi_948_ = 4294967295u;
                                    } else {
                                        phi_948_ = (_e529.member.member + _e560);
                                    }
                                    let _e568 = phi_948_;
                                    if (_e73 >= 1u) {
                                        phi_3083_ = (_e568 <= (_e73 - 1u));
                                    } else {
                                        phi_3083_ = false;
                                    }
                                    let _e573 = phi_3083_;
                                    if _e573 {
                                        let _e576 = global_2.member[_e568];
                                        phi_957_ = _e576;
                                    } else {
                                        phi_957_ = 4294967295u;
                                    }
                                    let _e578 = phi_957_;
                                    if (_e73 >= 10u) {
                                        phi_3107_ = (_e578 <= (_e73 - 10u));
                                    } else {
                                        phi_3107_ = false;
                                    }
                                    let _e583 = phi_3107_;
                                    if _e583 {
                                        let _e586 = global_2.member[_e578];
                                        let _e591 = global_2.member[(_e578 + 1u)];
                                        let _e596 = global_2.member[(_e578 + 2u)];
                                        let _e602 = global_2.member[(_e578 + 3u)];
                                        let _e607 = global_2.member[(_e578 + 4u)];
                                        let _e612 = global_2.member[(_e578 + 5u)];
                                        let _e617 = global_2.member[(_e578 + 6u)];
                                        let _e623 = global_2.member[(_e578 + 7u)];
                                        let _e628 = global_2.member[(_e578 + 8u)];
                                        let _e633 = global_2.member[(_e578 + 9u)];
                                        phi_1006_ = type_17(vec3<f32>(bitcast<f32>(_e586), bitcast<f32>(_e591), bitcast<f32>(_e596)), vec4<f32>(bitcast<f32>(_e602), bitcast<f32>(_e607), bitcast<f32>(_e612), bitcast<f32>(_e617)), vec3<f32>(bitcast<f32>(_e623), bitcast<f32>(_e628), bitcast<f32>(_e633)));
                                    } else {
                                        phi_1006_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e638 = phi_1006_;
                                    if (_e560 >= _e529.member_1.member_1) {
                                        phi_1016_ = 4294967295u;
                                    } else {
                                        phi_1016_ = (_e529.member_1.member + (16u * _e560));
                                    }
                                    let _e647 = phi_1016_;
                                    if (_e73 >= 16u) {
                                        phi_3132_ = (_e647 <= (_e73 - 16u));
                                    } else {
                                        phi_3132_ = false;
                                    }
                                    let _e652 = phi_3132_;
                                    if _e652 {
                                        let _e655 = global_2.member[_e647];
                                        let _e660 = global_2.member[(_e647 + 1u)];
                                        let _e665 = global_2.member[(_e647 + 2u)];
                                        let _e670 = global_2.member[(_e647 + 3u)];
                                        let _e676 = global_2.member[(_e647 + 4u)];
                                        let _e681 = global_2.member[(_e647 + 5u)];
                                        let _e686 = global_2.member[(_e647 + 6u)];
                                        let _e691 = global_2.member[(_e647 + 7u)];
                                        let _e697 = global_2.member[(_e647 + 8u)];
                                        let _e702 = global_2.member[(_e647 + 9u)];
                                        let _e707 = global_2.member[(_e647 + 10u)];
                                        let _e712 = global_2.member[(_e647 + 11u)];
                                        let _e718 = global_2.member[(_e647 + 12u)];
                                        let _e723 = global_2.member[(_e647 + 13u)];
                                        let _e728 = global_2.member[(_e647 + 14u)];
                                        let _e733 = global_2.member[(_e647 + 15u)];
                                        phi_1089_ = type_13(vec4<f32>(bitcast<f32>(_e655), bitcast<f32>(_e660), bitcast<f32>(_e665), bitcast<f32>(_e670)), vec4<f32>(bitcast<f32>(_e676), bitcast<f32>(_e681), bitcast<f32>(_e686), bitcast<f32>(_e691)), vec4<f32>(bitcast<f32>(_e697), bitcast<f32>(_e702), bitcast<f32>(_e707), bitcast<f32>(_e712)), vec4<f32>(bitcast<f32>(_e718), bitcast<f32>(_e723), bitcast<f32>(_e728), bitcast<f32>(_e733)));
                                    } else {
                                        phi_1089_ = type_13(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                    }
                                    let _e738 = phi_1089_;
                                    let _e746 = (_e638.member_1.x + _e638.member_1.x);
                                    let _e747 = (_e638.member_1.y + _e638.member_1.y);
                                    let _e748 = (_e638.member_1.z + _e638.member_1.z);
                                    let _e750 = (_e638.member_1.z * _e748);
                                    let _e751 = (_e638.member_1.w * _e746);
                                    let _e752 = (_e638.member_1.w * _e747);
                                    let _e753 = (_e638.member_1.w * _e748);
                                    let _e773 = (vec4<f32>((1f - fma(_e638.member_1.y, _e747, _e750)), fma(_e638.member_1.x, _e747, _e753), fma(_e638.member_1.x, _e748, -(_e752)), 0f) * _e638.member_2.x);
                                    let _e775 = (vec4<f32>(fma(_e638.member_1.x, _e747, -(_e753)), (1f - fma(_e638.member_1.x, _e746, _e750)), fma(_e638.member_1.y, _e748, _e751), 0f) * _e638.member_2.y);
                                    let _e777 = (vec4<f32>(fma(_e638.member_1.x, _e748, _e752), fma(_e638.member_1.y, _e748, -(_e751)), (1f - fma(_e638.member_1.x, _e746, (_e638.member_1.y * _e747))), 0f) * _e638.member_2.z);
                                    if _e558 {
                                    } else {
                                        phi_3632_ = true;
                                        break;
                                    }
                                    let _e882 = local[_e551.member_1];
                                    phi_910_ = type_13((_e536.member + (vec4<f32>(fma(_e638.member.x, _e738.member.w, fma(_e777.x, _e738.member.z, fma(_e773.x, _e738.member.x, (_e775.x * _e738.member.y)))), fma(_e638.member.y, _e738.member.w, fma(_e777.y, _e738.member.z, fma(_e773.y, _e738.member.x, (_e775.y * _e738.member.y)))), fma(_e638.member.z, _e738.member.w, fma(_e777.z, _e738.member.z, fma(_e773.z, _e738.member.x, (_e775.z * _e738.member.y)))), (fma(_e777.w, _e738.member.z, fma(_e773.w, _e738.member.x, (_e775.w * _e738.member.y))) + _e738.member.w)) * _e882)), (_e536.member_1 + (vec4<f32>(fma(_e638.member.x, _e738.member_1.w, fma(_e777.x, _e738.member_1.z, fma(_e773.x, _e738.member_1.x, (_e775.x * _e738.member_1.y)))), fma(_e638.member.y, _e738.member_1.w, fma(_e777.y, _e738.member_1.z, fma(_e773.y, _e738.member_1.x, (_e775.y * _e738.member_1.y)))), fma(_e638.member.z, _e738.member_1.w, fma(_e777.z, _e738.member_1.z, fma(_e773.z, _e738.member_1.x, (_e775.z * _e738.member_1.y)))), (fma(_e777.w, _e738.member_1.z, fma(_e773.w, _e738.member_1.x, (_e775.w * _e738.member_1.y))) + _e738.member_1.w)) * _e882)), (_e536.member_2 + (vec4<f32>(fma(_e638.member.x, _e738.member_2.w, fma(_e777.x, _e738.member_2.z, fma(_e773.x, _e738.member_2.x, (_e775.x * _e738.member_2.y)))), fma(_e638.member.y, _e738.member_2.w, fma(_e777.y, _e738.member_2.z, fma(_e773.y, _e738.member_2.x, (_e775.y * _e738.member_2.y)))), fma(_e638.member.z, _e738.member_2.w, fma(_e777.z, _e738.member_2.z, fma(_e773.z, _e738.member_2.x, (_e775.z * _e738.member_2.y)))), (fma(_e777.w, _e738.member_2.z, fma(_e773.w, _e738.member_2.x, (_e775.w * _e738.member_2.y))) + _e738.member_2.w)) * _e882)), (_e536.member_3 + (vec4<f32>(fma(_e638.member.x, _e738.member_3.w, fma(_e777.x, _e738.member_3.z, fma(_e773.x, _e738.member_3.x, (_e775.x * _e738.member_3.y)))), fma(_e638.member.y, _e738.member_3.w, fma(_e777.y, _e738.member_3.z, fma(_e773.y, _e738.member_3.x, (_e775.y * _e738.member_3.y)))), fma(_e638.member.z, _e738.member_3.w, fma(_e777.z, _e738.member_3.z, fma(_e773.z, _e738.member_3.x, (_e775.z * _e738.member_3.y)))), (fma(_e777.w, _e738.member_3.z, fma(_e773.w, _e738.member_3.x, (_e775.w * _e738.member_3.y))) + _e738.member_3.w)) * _e882)));
                                    phi_1326_ = true;
                                    break;
                                }
                                default: {
                                    phi_910_ = type_13();
                                    phi_1326_ = bool();
                                    break;
                                }
                            }
                            let _e897 = phi_910_;
                            let _e899 = phi_1326_;
                            continue;
                            continuing {
                                phi_906_ = _e549;
                                phi_909_ = _e897;
                                phi_3632_ = false;
                                break if !(_e899);
                            }
                        }
                        let _e902 = phi_3632_;
                        if _e902 {
                            break;
                        }
                        let _e904 = local_6;
                        let _e909 = global_5.member[0u];
                        if (_e904.member.x == _e909) {
                            let _e912 = local_7;
                            let _e917 = global_5.member[1u];
                            if (_e912.member.y == _e917) {
                                let _e920 = local_8;
                                let _e925 = global_5.member[2u];
                                let _e926 = (_e920.member.z == _e925);
                                if _e926 {
                                    let _e928 = local_9;
                                    let _e933 = global_5.member[3u];
                                    phi_1353_ = (_e928.member.w == _e933);
                                } else {
                                    phi_1353_ = bool();
                                }
                                let _e936 = phi_1353_;
                                phi_1355_ = _e936;
                                phi_1356_ = select(true, false, _e926);
                            } else {
                                phi_1355_ = bool();
                                phi_1356_ = true;
                            }
                            let _e939 = phi_1355_;
                            let _e941 = phi_1356_;
                            phi_1357_ = _e939;
                            phi_1358_ = _e941;
                        } else {
                            phi_1357_ = bool();
                            phi_1358_ = true;
                        }
                        let _e943 = phi_1357_;
                        let _e945 = phi_1358_;
                        if select(_e943, false, _e945) {
                            let _e948 = local_10;
                            let _e953 = global_5.member_1[0u];
                            if (_e948.member_1.x == _e953) {
                                let _e956 = local_11;
                                let _e961 = global_5.member_1[1u];
                                if (_e956.member_1.y == _e961) {
                                    let _e964 = local_12;
                                    let _e969 = global_5.member_1[2u];
                                    let _e970 = (_e964.member_1.z == _e969);
                                    if _e970 {
                                        let _e972 = local_13;
                                        let _e977 = global_5.member_1[3u];
                                        phi_1392_ = (_e972.member_1.w == _e977);
                                    } else {
                                        phi_1392_ = bool();
                                    }
                                    let _e980 = phi_1392_;
                                    phi_1394_ = _e980;
                                    phi_1395_ = select(true, false, _e970);
                                } else {
                                    phi_1394_ = bool();
                                    phi_1395_ = true;
                                }
                                let _e983 = phi_1394_;
                                let _e985 = phi_1395_;
                                phi_1396_ = _e983;
                                phi_1397_ = _e985;
                            } else {
                                phi_1396_ = bool();
                                phi_1397_ = true;
                            }
                            let _e987 = phi_1396_;
                            let _e989 = phi_1397_;
                            if select(_e987, false, _e989) {
                                let _e992 = local_14;
                                let _e997 = global_5.member_2[0u];
                                if (_e992.member_2.x == _e997) {
                                    let _e1000 = local_15;
                                    let _e1005 = global_5.member_2[1u];
                                    if (_e1000.member_2.y == _e1005) {
                                        let _e1008 = local_16;
                                        let _e1013 = global_5.member_2[2u];
                                        let _e1014 = (_e1008.member_2.z == _e1013);
                                        if _e1014 {
                                            let _e1016 = local_17;
                                            let _e1021 = global_5.member_2[3u];
                                            phi_1431_ = (_e1016.member_2.w == _e1021);
                                        } else {
                                            phi_1431_ = bool();
                                        }
                                        let _e1024 = phi_1431_;
                                        phi_1433_ = _e1024;
                                        phi_1434_ = select(true, false, _e1014);
                                    } else {
                                        phi_1433_ = bool();
                                        phi_1434_ = true;
                                    }
                                    let _e1027 = phi_1433_;
                                    let _e1029 = phi_1434_;
                                    phi_1435_ = _e1027;
                                    phi_1436_ = _e1029;
                                } else {
                                    phi_1435_ = bool();
                                    phi_1436_ = true;
                                }
                                let _e1031 = phi_1435_;
                                let _e1033 = phi_1436_;
                                let _e1034 = select(_e1031, false, _e1033);
                                if _e1034 {
                                    let _e1036 = local_18;
                                    let _e1041 = global_5.member_3[0u];
                                    if (_e1036.member_3.x == _e1041) {
                                        let _e1044 = local_19;
                                        let _e1049 = global_5.member_3[1u];
                                        if (_e1044.member_3.y == _e1049) {
                                            let _e1052 = local_20;
                                            let _e1057 = global_5.member_3[2u];
                                            let _e1058 = (_e1052.member_3.z == _e1057);
                                            if _e1058 {
                                                let _e1060 = local_21;
                                                let _e1065 = global_5.member_3[3u];
                                                phi_1470_ = (_e1060.member_3.w == _e1065);
                                            } else {
                                                phi_1470_ = bool();
                                            }
                                            let _e1068 = phi_1470_;
                                            phi_1472_ = _e1068;
                                            phi_1473_ = select(true, false, _e1058);
                                        } else {
                                            phi_1472_ = bool();
                                            phi_1473_ = true;
                                        }
                                        let _e1071 = phi_1472_;
                                        let _e1073 = phi_1473_;
                                        phi_1474_ = _e1071;
                                        phi_1475_ = _e1073;
                                    } else {
                                        phi_1474_ = bool();
                                        phi_1475_ = true;
                                    }
                                    let _e1075 = phi_1474_;
                                    let _e1077 = phi_1475_;
                                    phi_1480_ = select(_e1075, false, _e1077);
                                } else {
                                    phi_1480_ = bool();
                                }
                                let _e1080 = phi_1480_;
                                phi_1482_ = _e1080;
                                phi_1483_ = select(true, false, _e1034);
                            } else {
                                phi_1482_ = bool();
                                phi_1483_ = true;
                            }
                            let _e1083 = phi_1482_;
                            let _e1085 = phi_1483_;
                            phi_1484_ = _e1083;
                            phi_1485_ = _e1085;
                        } else {
                            phi_1484_ = bool();
                            phi_1485_ = true;
                        }
                        let _e1087 = phi_1484_;
                        let _e1089 = phi_1485_;
                        if select(_e1087, false, _e1089) {
                            phi_1493_ = type_13(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        } else {
                            let _e1854 = local_23;
                            phi_1493_ = _e1854;
                        }
                        let _e1092 = phi_1493_;
                        let _e1115 = fma(_e1092.member_2.z, _e1092.member_3.w, -((_e1092.member_2.w * _e1092.member_3.z)));
                        let _e1118 = fma(_e1092.member_2.y, _e1092.member_3.w, -((_e1092.member_2.w * _e1092.member_3.y)));
                        let _e1121 = fma(_e1092.member_2.y, _e1092.member_3.z, -((_e1092.member_2.z * _e1092.member_3.y)));
                        let _e1124 = fma(_e1092.member_2.x, _e1092.member_3.w, -((_e1092.member_2.w * _e1092.member_3.x)));
                        let _e1127 = fma(_e1092.member_2.x, _e1092.member_3.z, -((_e1092.member_2.z * _e1092.member_3.x)));
                        let _e1130 = fma(_e1092.member_2.x, _e1092.member_3.y, -((_e1092.member_2.y * _e1092.member_3.x)));
                        let _e1152 = fma(-(_e1092.member.w), fma(_e1092.member_1.z, _e1130, fma(_e1092.member_1.x, _e1121, -((_e1092.member_1.y * _e1127)))), fma(_e1092.member.z, fma(_e1092.member_1.w, _e1130, fma(_e1092.member_1.x, _e1118, -((_e1092.member_1.y * _e1124)))), fma(_e1092.member.x, fma(_e1092.member_1.w, _e1121, fma(_e1092.member_1.y, _e1115, -((_e1092.member_1.z * _e1118)))), -((_e1092.member.y * fma(_e1092.member_1.w, _e1127, fma(_e1092.member_1.x, _e1115, -((_e1092.member_1.z * _e1124)))))))));
                        if (_e1152 == 0f) {
                            phi_3379_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            phi_3380_ = type_17();
                            phi_3381_ = true;
                        } else {
                            let _e1161 = (sqrt(fma(_e1092.member.w, _e1092.member.w, fma(_e1092.member.z, _e1092.member.z, fma(_e1092.member.x, _e1092.member.x, (_e1092.member.y * _e1092.member.y))))) * select(-1f, 1f, (_e1152 >= 0f)));
                            let _e1166 = sqrt(fma(_e1092.member_1.w, _e1092.member_1.w, fma(_e1092.member_1.z, _e1092.member_1.z, fma(_e1092.member_1.x, _e1092.member_1.x, (_e1092.member_1.y * _e1092.member_1.y)))));
                            let _e1171 = sqrt(fma(_e1092.member_2.w, _e1092.member_2.w, fma(_e1092.member_2.z, _e1092.member_2.z, fma(_e1092.member_2.x, _e1092.member_2.x, (_e1092.member_2.y * _e1092.member_2.y)))));
                            if (_e1161 != 0f) {
                                phi_3269_ = select(true, false, (_e1166 != 0f));
                            } else {
                                phi_3269_ = true;
                            }
                            let _e1178 = phi_3269_;
                            let _e1179 = select((_e1171 != 0f), false, _e1178);
                            if _e1179 {
                                let _e1180 = (1f / _e1161);
                                let _e1181 = (1f / _e1166);
                                let _e1182 = (1f / _e1171);
                                let _e1183 = (_e1092.member.x * _e1180);
                                let _e1184 = (_e1092.member.z * _e1180);
                                let _e1185 = (_e1092.member_1.x * _e1181);
                                let _e1186 = (_e1092.member_2.x * _e1182);
                                let _e1187 = (_e1092.member_2.y * _e1182);
                                if ((_e1092.member_2.z * _e1182) <= 0f) {
                                    let _e1191 = fma(_e1092.member_1.y, _e1181, -(_e1183));
                                    let _e1193 = fma(-(_e1092.member_2.z), _e1182, 1f);
                                    if (_e1191 <= 0f) {
                                        let _e1195 = (_e1193 - _e1191);
                                        let _e1197 = (0.5f / sqrt(_e1195));
                                        phi_3332_ = vec4<f32>((_e1195 * _e1197), (fma(_e1092.member.y, _e1180, _e1185) * _e1197), (fma(_e1092.member.z, _e1180, _e1186) * _e1197), (fma(_e1092.member_1.z, _e1181, -(_e1187)) * _e1197));
                                    } else {
                                        let _e1207 = (_e1193 + _e1191);
                                        let _e1209 = (0.5f / sqrt(_e1207));
                                        phi_3332_ = vec4<f32>((fma(_e1092.member.y, _e1180, _e1185) * _e1209), (_e1207 * _e1209), (fma(_e1092.member_1.z, _e1181, _e1187) * _e1209), (fma(_e1092.member_2.x, _e1182, -(_e1184)) * _e1209));
                                    }
                                    let _e1220 = phi_3332_;
                                    phi_3364_ = _e1220;
                                } else {
                                    let _e1221 = fma(_e1092.member_1.y, _e1181, _e1183);
                                    let _e1222 = fma(_e1092.member_2.z, _e1182, 1f);
                                    if (_e1221 <= 0f) {
                                        let _e1224 = (_e1222 - _e1221);
                                        let _e1226 = (0.5f / sqrt(_e1224));
                                        phi_3362_ = vec4<f32>((fma(_e1092.member.z, _e1180, _e1186) * _e1226), (fma(_e1092.member_1.z, _e1181, _e1187) * _e1226), (_e1224 * _e1226), (fma(_e1092.member.y, _e1180, -(_e1185)) * _e1226));
                                    } else {
                                        let _e1236 = (_e1222 + _e1221);
                                        let _e1238 = (0.5f / sqrt(_e1236));
                                        phi_3362_ = vec4<f32>((fma(_e1092.member_1.z, _e1181, -(_e1187)) * _e1238), (fma(_e1092.member_2.x, _e1182, -(_e1184)) * _e1238), (fma(_e1092.member.y, _e1180, -(_e1185)) * _e1238), (_e1236 * _e1238));
                                    }
                                    let _e1251 = phi_3362_;
                                    phi_3364_ = _e1251;
                                }
                                let _e1253 = phi_3364_;
                                phi_3375_ = type_17(vec3<f32>(_e1161, _e1166, _e1171), _e1253, vec3<f32>(_e1092.member_3.x, _e1092.member_3.y, _e1092.member_3.z));
                                phi_3376_ = type_17();
                            } else {
                                phi_3375_ = type_17();
                                phi_3376_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e1257 = phi_3375_;
                            let _e1259 = phi_3376_;
                            phi_3379_ = _e1259;
                            phi_3380_ = _e1257;
                            phi_3381_ = select(true, false, _e1179);
                        }
                        let _e1262 = phi_3379_;
                        let _e1264 = phi_3380_;
                        let _e1266 = phi_3381_;
                        if _e1266 {
                            phi_3385_ = _e1262;
                        } else {
                            phi_3385_ = _e1264;
                        }
                        let _e1268 = phi_3385_;
                        phi_1495_ = type_17(_e1268.member_2, _e1268.member_1, _e1268.member);
                    } else {
                        phi_1495_ = type_17();
                    }
                    let _e1274 = phi_1495_;
                    phi_1497_ = _e1274;
                    phi_1498_ = select(true, false, _e504);
                } else {
                    phi_1497_ = type_17();
                    phi_1498_ = true;
                }
                let _e1277 = phi_1497_;
                let _e1279 = phi_1498_;
                if _e1279 {
                    if (_e73 >= 10u) {
                        phi_3479_ = (_e98 <= (_e73 - 10u));
                    } else {
                        phi_3479_ = false;
                    }
                    let _e1284 = phi_3479_;
                    if _e1284 {
                        let _e1287 = global_2.member[_e98];
                        let _e1292 = global_2.member[(_e98 + 1u)];
                        let _e1297 = global_2.member[(_e98 + 2u)];
                        let _e1303 = global_2.member[(_e98 + 3u)];
                        let _e1308 = global_2.member[(_e98 + 4u)];
                        let _e1313 = global_2.member[(_e98 + 5u)];
                        let _e1318 = global_2.member[(_e98 + 6u)];
                        let _e1324 = global_2.member[(_e98 + 7u)];
                        let _e1329 = global_2.member[(_e98 + 8u)];
                        let _e1334 = global_2.member[(_e98 + 9u)];
                        phi_1551_ = type_17(vec3<f32>(bitcast<f32>(_e1287), bitcast<f32>(_e1292), bitcast<f32>(_e1297)), vec4<f32>(bitcast<f32>(_e1303), bitcast<f32>(_e1308), bitcast<f32>(_e1313), bitcast<f32>(_e1318)), vec3<f32>(bitcast<f32>(_e1324), bitcast<f32>(_e1329), bitcast<f32>(_e1334)));
                    } else {
                        phi_1551_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e1339 = phi_1551_;
                    phi_1552_ = _e1339;
                } else {
                    phi_1552_ = _e1277;
                }
                let _e1341 = phi_1552_;
                let _e1349 = (_e1341.member_1.x + _e1341.member_1.x);
                let _e1350 = (_e1341.member_1.y + _e1341.member_1.y);
                let _e1351 = (_e1341.member_1.z + _e1341.member_1.z);
                let _e1353 = (_e1341.member_1.z * _e1351);
                let _e1354 = (_e1341.member_1.w * _e1349);
                let _e1355 = (_e1341.member_1.w * _e1350);
                let _e1356 = (_e1341.member_1.w * _e1351);
                let _e1376 = (vec4<f32>((1f - fma(_e1341.member_1.y, _e1350, _e1353)), fma(_e1341.member_1.x, _e1350, _e1356), fma(_e1341.member_1.x, _e1351, -(_e1355)), 0f) * _e1341.member_2.x);
                let _e1378 = (vec4<f32>(fma(_e1341.member_1.x, _e1350, -(_e1356)), (1f - fma(_e1341.member_1.x, _e1349, _e1353)), fma(_e1341.member_1.y, _e1351, _e1354), 0f) * _e1341.member_2.y);
                let _e1380 = (vec4<f32>(fma(_e1341.member_1.x, _e1351, _e1355), fma(_e1341.member_1.y, _e1351, -(_e1354)), (1f - fma(_e1341.member_1.x, _e1349, (_e1341.member_1.y * _e1350))), 0f) * _e1341.member_2.z);
                let _e1385 = local_22;
                let _e1408 = (_e1341.member.x + fma(_e1380.x, _e1385.member.z, fma(_e1378.x, _e1385.member.y, (_e1376.x * _e1385.member.x))));
                let _e1409 = (_e1341.member.y + fma(_e1380.y, _e1385.member.z, fma(_e1378.y, _e1385.member.y, (_e1376.y * _e1385.member.x))));
                let _e1410 = (_e1341.member.z + fma(_e1380.z, _e1385.member.z, fma(_e1378.z, _e1385.member.y, (_e1376.z * _e1385.member.x))));
                let _e1411 = global_4.member;
                let _e1414 = global_2.member[_e1411];
                switch bitcast<i32>(_e1414) {
                    case 0: {
                        phi_1600_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1600_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1600_ = 2u;
                        break;
                    }
                    default: {
                        phi_1600_ = 0u;
                        break;
                    }
                }
                let _e1417 = phi_1600_;
                let _e1421 = global_2.member[(_e1411 + 1u)];
                let _e1425 = global_2.member[(_e1411 + 2u)];
                if (_e73 >= 10u) {
                    phi_3550_ = (_e1425 <= (_e73 - 10u));
                } else {
                    phi_3550_ = false;
                }
                let _e1430 = phi_3550_;
                if _e1430 {
                    let _e1433 = global_2.member[_e1425];
                    let _e1438 = global_2.member[(_e1425 + 1u)];
                    let _e1443 = global_2.member[(_e1425 + 2u)];
                    let _e1449 = global_2.member[(_e1425 + 3u)];
                    let _e1454 = global_2.member[(_e1425 + 4u)];
                    let _e1459 = global_2.member[(_e1425 + 5u)];
                    let _e1464 = global_2.member[(_e1425 + 6u)];
                    let _e1470 = global_2.member[(_e1425 + 7u)];
                    let _e1475 = global_2.member[(_e1425 + 8u)];
                    let _e1480 = global_2.member[(_e1425 + 9u)];
                    phi_1656_ = type_17(vec3<f32>(bitcast<f32>(_e1433), bitcast<f32>(_e1438), bitcast<f32>(_e1443)), vec4<f32>(bitcast<f32>(_e1449), bitcast<f32>(_e1454), bitcast<f32>(_e1459), bitcast<f32>(_e1464)), vec3<f32>(bitcast<f32>(_e1470), bitcast<f32>(_e1475), bitcast<f32>(_e1480)));
                } else {
                    phi_1656_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                }
                let _e1485 = phi_1656_;
                let _e1493 = (_e1485.member_1.x + _e1485.member_1.x);
                let _e1494 = (_e1485.member_1.y + _e1485.member_1.y);
                let _e1495 = (_e1485.member_1.z + _e1485.member_1.z);
                let _e1497 = (_e1485.member_1.z * _e1495);
                let _e1498 = (_e1485.member_1.w * _e1493);
                let _e1499 = (_e1485.member_1.w * _e1494);
                let _e1500 = (_e1485.member_1.w * _e1495);
                let _e1520 = (vec4<f32>((1f - fma(_e1485.member_1.y, _e1494, _e1497)), fma(_e1485.member_1.x, _e1494, _e1500), fma(_e1485.member_1.x, _e1495, -(_e1499)), 0f) * _e1485.member_2.x);
                let _e1522 = (vec4<f32>(fma(_e1485.member_1.x, _e1494, -(_e1500)), (1f - fma(_e1485.member_1.x, _e1493, _e1497)), fma(_e1485.member_1.y, _e1495, _e1498), 0f) * _e1485.member_2.y);
                let _e1524 = (vec4<f32>(fma(_e1485.member_1.x, _e1495, _e1499), fma(_e1485.member_1.y, _e1495, -(_e1498)), (1f - fma(_e1485.member_1.x, _e1493, (_e1485.member_1.y * _e1494))), 0f) * _e1485.member_2.z);
                switch bitcast<i32>(_e1417) {
                    case 0: {
                        let _e1531 = global_2.member[_e1421];
                        let _e1532 = bitcast<f32>(_e1531);
                        let _e1536 = global_2.member[(_e1421 + 1u)];
                        let _e1537 = bitcast<f32>(_e1536);
                        let _e1541 = global_2.member[(_e1421 + 2u)];
                        let _e1542 = bitcast<f32>(_e1541);
                        let _e1558 = fma(_e1524.x, _e1542, fma(_e1522.x, _e1537, (_e1520.x * _e1532)));
                        let _e1559 = fma(_e1524.y, _e1542, fma(_e1522.y, _e1537, (_e1520.y * _e1532)));
                        let _e1560 = fma(_e1524.z, _e1542, fma(_e1522.z, _e1537, (_e1520.z * _e1532)));
                        let _e1561 = -(_e1558);
                        let _e1562 = -(_e1559);
                        let _e1563 = -(_e1560);
                        let _e1568 = (1f / sqrt(fma(_e1563, _e1563, fma(_e1561, _e1561, (_e1562 * _e1562)))));
                        let _e1569 = (_e1561 * _e1568);
                        let _e1570 = (_e1562 * _e1568);
                        let _e1571 = (_e1563 * _e1568);
                        let _e1572 = -(_e1571);
                        let _e1576 = (1f / sqrt(fma(_e1572, _e1572, (_e1569 * _e1569))));
                        let _e1577 = (_e1572 * _e1576);
                        let _e1578 = (_e1569 * _e1576);
                        let _e1579 = (_e1570 * _e1578);
                        let _e1583 = fma(_e1578, _e1569, -((_e1571 * _e1577)));
                        let _e1584 = (_e1577 * _e1570);
                        phi_2403_ = type_13(vec4<f32>((0.1f * _e1577), (_e1579 * -0.1f), (_e1569 * 0.15384616f), 0f), vec4<f32>(0f, (0.1f * _e1583), (_e1570 * 0.15384616f), 0f), vec4<f32>((0.1f * _e1578), (0.1f * _e1584), (_e1571 * 0.15384616f), 0f), vec4<f32>((fma(_e1558, _e1577, (_e1560 * _e1578)) * -0.1f), (fma(_e1560, _e1584, fma(_e1558, -(_e1579), (_e1559 * _e1583))) * -0.1f), fma(-0.15384616f, fma(_e1560, _e1571, fma(_e1558, _e1569, (_e1559 * _e1570))), -0.15384616f), 1f));
                        break;
                    }
                    case 1: {
                        phi_2403_ = type_13(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        break;
                    }
                    case 2: {
                        let _e1611 = global_2.member[_e1421];
                        let _e1612 = bitcast<f32>(_e1611);
                        let _e1616 = global_2.member[(_e1421 + 1u)];
                        let _e1617 = bitcast<f32>(_e1616);
                        let _e1621 = global_2.member[(_e1421 + 2u)];
                        let _e1622 = bitcast<f32>(_e1621);
                        let _e1626 = global_2.member[(_e1421 + 3u)];
                        let _e1627 = bitcast<f32>(_e1626);
                        let _e1631 = global_2.member[(_e1421 + 4u)];
                        let _e1632 = bitcast<f32>(_e1631);
                        let _e1636 = global_2.member[(_e1421 + 5u)];
                        let _e1637 = bitcast<f32>(_e1636);
                        let _e1641 = global_2.member[(_e1421 + 7u)];
                        let _e1643 = (0.5f * bitcast<f32>(_e1641));
                        let _e1646 = (cos(_e1643) / sin(_e1643));
                        let _e1662 = fma(_e1524.x, _e1637, fma(_e1522.x, _e1632, (_e1520.x * _e1627)));
                        let _e1663 = fma(_e1524.y, _e1637, fma(_e1522.y, _e1632, (_e1520.y * _e1627)));
                        let _e1664 = fma(_e1524.z, _e1637, fma(_e1522.z, _e1632, (_e1520.z * _e1627)));
                        let _e1674 = (_e1485.member.x + fma(_e1524.x, _e1622, fma(_e1522.x, _e1617, (_e1520.x * _e1612))));
                        let _e1675 = (_e1485.member.y + fma(_e1524.y, _e1622, fma(_e1522.y, _e1617, (_e1520.y * _e1612))));
                        let _e1676 = (_e1485.member.z + fma(_e1524.z, _e1622, fma(_e1522.z, _e1617, (_e1520.z * _e1612))));
                        let _e1681 = (1f / sqrt(fma(_e1664, _e1664, fma(_e1662, _e1662, (_e1663 * _e1663)))));
                        let _e1682 = (_e1662 * _e1681);
                        let _e1683 = (_e1663 * _e1681);
                        let _e1684 = (_e1664 * _e1681);
                        let _e1685 = -(_e1684);
                        let _e1689 = (1f / sqrt(fma(_e1685, _e1685, (_e1682 * _e1682))));
                        let _e1690 = (_e1685 * _e1689);
                        let _e1691 = (_e1682 * _e1689);
                        let _e1693 = -((_e1683 * _e1691));
                        let _e1696 = fma(_e1691, _e1682, -((_e1684 * _e1690)));
                        let _e1697 = (_e1690 * _e1683);
                        let _e1707 = fma(_e1676, _e1684, fma(_e1674, _e1682, (_e1675 * _e1683)));
                        phi_2403_ = type_13(vec4<f32>((_e1646 * _e1690), (_e1646 * _e1693), (_e1682 * 1.0001f), _e1682), vec4<f32>(0f, (_e1646 * _e1696), (_e1683 * 1.0001f), _e1683), vec4<f32>((_e1646 * _e1691), (_e1646 * _e1697), (_e1684 * 1.0001f), _e1684), vec4<f32>((_e1646 * -(fma(_e1674, _e1690, (_e1676 * _e1691)))), (_e1646 * -(fma(_e1676, _e1697, fma(_e1674, _e1693, (_e1675 * _e1696))))), fma(-1.0001f, _e1707, -0.010001f), (-1f * _e1707)));
                        break;
                    }
                    default: {
                        phi_2403_ = type_13();
                        break;
                    }
                }
                let _e1726 = phi_2403_;
                global_1 = vec4<f32>((fma(_e1726.member_2.x, _e1410, fma(_e1726.member.x, _e1408, (_e1726.member_1.x * _e1409))) + _e1726.member_3.x), (fma(_e1726.member_2.y, _e1410, fma(_e1726.member.y, _e1408, (_e1726.member_1.y * _e1409))) + _e1726.member_3.y), (fma(_e1726.member_2.z, _e1410, fma(_e1726.member.z, _e1408, (_e1726.member_1.z * _e1409))) + _e1726.member_3.z), (fma(_e1726.member_2.w, _e1410, fma(_e1726.member.w, _e1408, (_e1726.member_1.w * _e1409))) + _e1726.member_3.w));
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
    global_3 = param;
    global = param_1;
    function();
    let _e6 = global_1.y;
    global_1.y = -(_e6);
    let _e8 = global_1;
    return _e8;
}
