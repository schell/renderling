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
var<private> global_4: type_14 = type_14(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));

fn function() {
    var local: array<f32, 4>;
    var local_1: array<u32, 4>;
    var local_2: array<f32, 4>;
    var local_3: array<u32, 4>;
    var phi_1365_: u32;
    var phi_2647_: bool;
    var phi_1372_: u32;
    var phi_1373_: u32;
    var phi_1383_: u32;
    var phi_1465_: type_11;
    var phi_1466_: type_11;
    var phi_1489_: type_11;
    var phi_1502_: bool;
    var phi_1508_: type_11;
    var phi_1509_: type_11;
    var phi_1532_: type_11;
    var phi_1546_: bool;
    var phi_1552_: type_11;
    var phi_1555_: type_20;
    var phi_1553_: type_11;
    var phi_1578_: type_11;
    var phi_1595_: u32;
    var phi_2677_: bool;
    var phi_1613_: type_11;
    var phi_2703_: u32;
    var phi_2722_: bool;
    var phi_1663_: type_23;
    var phi_1673_: u32;
    var phi_2744_: bool;
    var phi_1681_: f32;
    var phi_1556_: type_20;
    var phi_1734_: bool;
    var phi_2766_: bool;
    var phi_1834_: type_24;
    var local_4: type_20;
    var phi_1837_: type_11;
    var phi_1840_: type_14;
    var phi_1838_: type_11;
    var phi_1863_: type_11;
    var local_5: type_20;
    var phi_1887_: u32;
    var phi_2800_: bool;
    var phi_1896_: u32;
    var phi_2824_: bool;
    var phi_1945_: type_17;
    var phi_1955_: u32;
    var phi_2849_: bool;
    var phi_2028_: type_14;
    var phi_1841_: type_14;
    var phi_2265_: bool;
    var phi_3528_: bool;
    var local_6: type_14;
    var local_7: type_14;
    var local_8: type_14;
    var local_9: type_14;
    var phi_2292_: bool;
    var phi_2294_: bool;
    var phi_2295_: bool;
    var phi_2296_: bool;
    var phi_2297_: bool;
    var local_10: type_14;
    var local_11: type_14;
    var local_12: type_14;
    var local_13: type_14;
    var phi_2331_: bool;
    var phi_2333_: bool;
    var phi_2334_: bool;
    var phi_2335_: bool;
    var phi_2336_: bool;
    var local_14: type_14;
    var local_15: type_14;
    var local_16: type_14;
    var local_17: type_14;
    var phi_2370_: bool;
    var phi_2372_: bool;
    var phi_2373_: bool;
    var phi_2374_: bool;
    var phi_2375_: bool;
    var local_18: type_14;
    var local_19: type_14;
    var local_20: type_14;
    var local_21: type_14;
    var phi_2409_: bool;
    var phi_2411_: bool;
    var phi_2412_: bool;
    var phi_2413_: bool;
    var phi_2414_: bool;
    var phi_2419_: bool;
    var phi_2421_: bool;
    var phi_2422_: bool;
    var phi_2423_: bool;
    var phi_2424_: bool;
    var phi_2432_: type_14;
    var phi_2984_: bool;
    var phi_3049_: vec4<f32>;
    var phi_3079_: vec4<f32>;
    var phi_3081_: vec4<f32>;
    var phi_3090_: type_17;
    var phi_3091_: type_17;
    var phi_3096_: type_17;
    var phi_3097_: type_17;
    var phi_3098_: bool;
    var phi_3102_: type_17;
    var phi_2434_: type_17;
    var phi_2436_: type_17;
    var phi_2437_: bool;
    var phi_3196_: bool;
    var phi_2490_: type_17;
    var phi_2491_: type_17;
    var local_22: type_20;
    var local_23: type_14;

    switch bitcast<i32>(0u) {
        default: {
            let _e73 = global_2;
            let _e74 = global;
            let _e76 = arrayLength((&global_3.member));
            let _e79 = global_3.member[_e73];
            let _e84 = global_3.member[(_e73 + 1u)];
            let _e88 = global_3.member[(_e73 + 2u)];
            let _e92 = global_3.member[(_e73 + 7u)];
            let _e96 = global_3.member[(_e73 + 8u)];
            let _e100 = global_3.member[(_e73 + 9u)];
            let _e104 = global_3.member[(_e73 + 11u)];
            let _e108 = global_3.member[(_e73 + 12u)];
            let _e112 = global_3.member[(_e73 + 13u)];
            let _e116 = global_3.member[(_e73 + 14u)];
            let _e120 = global_3.member[(_e73 + 15u)];
            let _e124 = global_3.member[(_e73 + 16u)];
            if (_e79 == 1u) {
                let _e127 = global_3.member[0u];
                let _e130 = global_3.member[_e127];
                let _e131 = bitcast<f32>(_e130);
                let _e135 = global_3.member[(_e127 + 1u)];
                let _e136 = bitcast<f32>(_e135);
                let _e140 = global_3.member[(_e127 + 2u)];
                let _e141 = bitcast<f32>(_e140);
                let _e145 = global_3.member[(_e127 + 3u)];
                let _e146 = bitcast<f32>(_e145);
                let _e150 = global_3.member[(_e127 + 4u)];
                let _e151 = bitcast<f32>(_e150);
                let _e155 = global_3.member[(_e127 + 5u)];
                let _e156 = bitcast<f32>(_e155);
                let _e160 = global_3.member[(_e127 + 6u)];
                let _e161 = bitcast<f32>(_e160);
                let _e165 = global_3.member[(_e127 + 7u)];
                let _e166 = bitcast<f32>(_e165);
                let _e170 = global_3.member[(_e127 + 8u)];
                let _e171 = bitcast<f32>(_e170);
                let _e175 = global_3.member[(_e127 + 9u)];
                let _e176 = bitcast<f32>(_e175);
                let _e180 = global_3.member[(_e127 + 10u)];
                let _e181 = bitcast<f32>(_e180);
                let _e185 = global_3.member[(_e127 + 11u)];
                let _e186 = bitcast<f32>(_e185);
                let _e190 = global_3.member[(_e127 + 12u)];
                let _e191 = bitcast<f32>(_e190);
                let _e195 = global_3.member[(_e127 + 13u)];
                let _e196 = bitcast<f32>(_e195);
                let _e200 = global_3.member[(_e127 + 14u)];
                let _e201 = bitcast<f32>(_e200);
                let _e205 = global_3.member[(_e127 + 15u)];
                let _e206 = bitcast<f32>(_e205);
                let _e210 = global_3.member[(_e127 + 16u)];
                let _e211 = bitcast<f32>(_e210);
                let _e215 = global_3.member[(_e127 + 17u)];
                let _e216 = bitcast<f32>(_e215);
                let _e220 = global_3.member[(_e127 + 18u)];
                let _e221 = bitcast<f32>(_e220);
                let _e225 = global_3.member[(_e127 + 19u)];
                let _e226 = bitcast<f32>(_e225);
                let _e230 = global_3.member[(_e127 + 20u)];
                let _e231 = bitcast<f32>(_e230);
                let _e235 = global_3.member[(_e127 + 21u)];
                let _e236 = bitcast<f32>(_e235);
                let _e240 = global_3.member[(_e127 + 22u)];
                let _e241 = bitcast<f32>(_e240);
                let _e245 = global_3.member[(_e127 + 23u)];
                let _e246 = bitcast<f32>(_e245);
                let _e250 = global_3.member[(_e127 + 24u)];
                let _e251 = bitcast<f32>(_e250);
                let _e255 = global_3.member[(_e127 + 25u)];
                let _e256 = bitcast<f32>(_e255);
                let _e260 = global_3.member[(_e127 + 26u)];
                let _e261 = bitcast<f32>(_e260);
                let _e265 = global_3.member[(_e127 + 27u)];
                let _e266 = bitcast<f32>(_e265);
                let _e270 = global_3.member[(_e127 + 28u)];
                let _e271 = bitcast<f32>(_e270);
                let _e275 = global_3.member[(_e127 + 29u)];
                let _e276 = bitcast<f32>(_e275);
                let _e280 = global_3.member[(_e127 + 30u)];
                let _e281 = bitcast<f32>(_e280);
                let _e285 = global_3.member[(_e127 + 31u)];
                let _e286 = bitcast<f32>(_e285);
                if (_e92 == 4294967295u) {
                    phi_1373_ = _e74;
                } else {
                    if (_e74 >= _e96) {
                        phi_1365_ = 4294967295u;
                    } else {
                        phi_1365_ = (_e92 + _e74);
                    }
                    let _e291 = phi_1365_;
                    if (_e76 >= 1u) {
                        phi_2647_ = (_e291 <= (_e76 - 1u));
                    } else {
                        phi_2647_ = false;
                    }
                    let _e296 = phi_2647_;
                    if _e296 {
                        let _e299 = global_3.member[_e291];
                        phi_1372_ = _e299;
                    } else {
                        phi_1372_ = 0u;
                    }
                    let _e301 = phi_1372_;
                    phi_1373_ = _e301;
                }
                let _e303 = phi_1373_;
                if (_e303 >= _e88) {
                    phi_1383_ = 4294967295u;
                } else {
                    phi_1383_ = (_e84 + (26u * _e303));
                }
                let _e308 = phi_1383_;
                let _e311 = global_3.member[_e308];
                let _e316 = global_3.member[(_e308 + 1u)];
                let _e321 = global_3.member[(_e308 + 2u)];
                let _e327 = global_3.member[(_e308 + 3u)];
                let _e332 = global_3.member[(_e308 + 4u)];
                let _e337 = global_3.member[(_e308 + 5u)];
                let _e342 = global_3.member[(_e308 + 6u)];
                let _e348 = global_3.member[(_e308 + 7u)];
                let _e353 = global_3.member[(_e308 + 8u)];
                let _e359 = global_3.member[(_e308 + 9u)];
                let _e364 = global_3.member[(_e308 + 10u)];
                let _e370 = global_3.member[(_e308 + 11u)];
                let _e375 = global_3.member[(_e308 + 12u)];
                let _e380 = global_3.member[(_e308 + 13u)];
                let _e386 = global_3.member[(_e308 + 14u)];
                let _e391 = global_3.member[(_e308 + 15u)];
                let _e396 = global_3.member[(_e308 + 16u)];
                let _e401 = global_3.member[(_e308 + 17u)];
                local_3 = array<u32, 4>(0u, 0u, 0u, 0u);
                phi_1465_ = type_11(0u, 4u);
                loop {
                    let _e406 = phi_1465_;
                    if (_e406.member < _e406.member_1) {
                        phi_1466_ = type_11((_e406.member + 1u), _e406.member_1);
                        phi_1489_ = type_11(1u, _e406.member);
                    } else {
                        phi_1466_ = _e406;
                        phi_1489_ = type_11(0u, type_11().member_1);
                    }
                    let _e419 = phi_1466_;
                    let _e421 = phi_1489_;
                    switch bitcast<i32>(_e421.member) {
                        case 0: {
                            phi_1502_ = false;
                            break;
                        }
                        case 1: {
                            let _e428 = global_3.member[((_e308 + 18u) + _e421.member_1)];
                            local_3[_e421.member_1] = _e428;
                            phi_1502_ = true;
                            break;
                        }
                        default: {
                            phi_1502_ = bool();
                            break;
                        }
                    }
                    let _e431 = phi_1502_;
                    continue;
                    continuing {
                        phi_1465_ = _e419;
                        break if !(_e431);
                    }
                }
                let _e433 = local_3;
                local_2 = array<f32, 4>(0f, 0f, 0f, 0f);
                phi_1508_ = type_11(0u, 4u);
                loop {
                    let _e436 = phi_1508_;
                    if (_e436.member < _e436.member_1) {
                        phi_1509_ = type_11((_e436.member + 1u), _e436.member_1);
                        phi_1532_ = type_11(1u, _e436.member);
                    } else {
                        phi_1509_ = _e436;
                        phi_1532_ = type_11(0u, type_11().member_1);
                    }
                    let _e449 = phi_1509_;
                    let _e451 = phi_1532_;
                    switch bitcast<i32>(_e451.member) {
                        case 0: {
                            phi_1546_ = false;
                            break;
                        }
                        case 1: {
                            let _e458 = global_3.member[((_e308 + 22u) + _e451.member_1)];
                            local_2[_e451.member_1] = bitcast<f32>(_e458);
                            phi_1546_ = true;
                            break;
                        }
                        default: {
                            phi_1546_ = bool();
                            break;
                        }
                    }
                    let _e462 = phi_1546_;
                    continue;
                    continuing {
                        phi_1508_ = _e449;
                        break if !(_e462);
                    }
                }
                let _e464 = local_2;
                phi_1552_ = type_11(0u, _e112);
                phi_1555_ = type_20(vec3<f32>(bitcast<f32>(_e311), bitcast<f32>(_e316), bitcast<f32>(_e321)), vec4<f32>(bitcast<f32>(_e327), bitcast<f32>(_e332), bitcast<f32>(_e337), bitcast<f32>(_e342)), vec3<f32>(bitcast<f32>(_e370), bitcast<f32>(_e375), bitcast<f32>(_e380)), vec4<f32>(bitcast<f32>(_e386), bitcast<f32>(_e391), bitcast<f32>(_e396), bitcast<f32>(_e401)), _e433, _e464, vec2<f32>(bitcast<f32>(_e348), bitcast<f32>(_e353)), vec2<f32>(bitcast<f32>(_e359), bitcast<f32>(_e364)));
                loop {
                    let _e468 = phi_1552_;
                    let _e470 = phi_1555_;
                    local_4 = _e470;
                    local_5 = _e470;
                    local_22 = _e470;
                    if (_e468.member < _e468.member_1) {
                        phi_1553_ = type_11((_e468.member + 1u), _e468.member_1);
                        phi_1578_ = type_11(1u, _e468.member);
                    } else {
                        phi_1553_ = _e468;
                        phi_1578_ = type_11(0u, type_11().member_1);
                    }
                    let _e483 = phi_1553_;
                    let _e485 = phi_1578_;
                    switch bitcast<i32>(_e485.member) {
                        case 0: {
                            phi_1556_ = type_20();
                            phi_1734_ = false;
                            break;
                        }
                        case 1: {
                            if (_e485.member_1 >= _e112) {
                                phi_1595_ = 4294967295u;
                            } else {
                                phi_1595_ = (_e108 + (2u * _e485.member_1));
                            }
                            let _e493 = phi_1595_;
                            if (_e76 >= 2u) {
                                phi_2677_ = (_e493 <= (_e76 - 2u));
                            } else {
                                phi_2677_ = false;
                            }
                            let _e498 = phi_2677_;
                            if _e498 {
                                let _e501 = global_3.member[_e493];
                                let _e505 = global_3.member[(_e493 + 1u)];
                                phi_1613_ = type_11(_e501, _e505);
                            } else {
                                phi_1613_ = type_11(4294967295u, 0u);
                            }
                            let _e508 = phi_1613_;
                            if (_e303 >= _e508.member_1) {
                                phi_2703_ = 4294967295u;
                            } else {
                                phi_2703_ = (_e508.member + (9u * _e303));
                            }
                            let _e515 = phi_2703_;
                            if (_e76 >= 9u) {
                                phi_2722_ = (_e515 <= (_e76 - 9u));
                            } else {
                                phi_2722_ = false;
                            }
                            let _e520 = phi_2722_;
                            if _e520 {
                                let _e523 = global_3.member[_e515];
                                let _e528 = global_3.member[(_e515 + 1u)];
                                let _e533 = global_3.member[(_e515 + 2u)];
                                let _e539 = global_3.member[(_e515 + 3u)];
                                let _e544 = global_3.member[(_e515 + 4u)];
                                let _e549 = global_3.member[(_e515 + 5u)];
                                let _e555 = global_3.member[(_e515 + 6u)];
                                let _e560 = global_3.member[(_e515 + 7u)];
                                let _e565 = global_3.member[(_e515 + 8u)];
                                phi_1663_ = type_23(vec3<f32>(bitcast<f32>(_e523), bitcast<f32>(_e528), bitcast<f32>(_e533)), vec3<f32>(bitcast<f32>(_e539), bitcast<f32>(_e544), bitcast<f32>(_e549)), vec3<f32>(bitcast<f32>(_e555), bitcast<f32>(_e560), bitcast<f32>(_e565)));
                            } else {
                                phi_1663_ = type_23(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e570 = phi_1663_;
                            if (_e485.member_1 >= _e120) {
                                phi_1673_ = 4294967295u;
                            } else {
                                phi_1673_ = (_e116 + _e485.member_1);
                            }
                            let _e574 = phi_1673_;
                            if (_e76 >= 1u) {
                                phi_2744_ = (_e574 <= (_e76 - 1u));
                            } else {
                                phi_2744_ = false;
                            }
                            let _e579 = phi_2744_;
                            if _e579 {
                                let _e582 = global_3.member[_e574];
                                phi_1681_ = bitcast<f32>(_e582);
                            } else {
                                phi_1681_ = 0f;
                            }
                            let _e585 = phi_1681_;
                            let _e608 = type_20(vec3<f32>(fma(_e585, _e570.member.x, _e470.member.x), fma(_e585, _e570.member.y, _e470.member.y), fma(_e585, _e570.member.z, _e470.member.z)), _e470.member_1, _e470.member_2, _e470.member_3, _e470.member_4, _e470.member_5, _e470.member_6, _e470.member_7);
                            let _e631 = type_20(_e608.member, _e608.member_1, vec3<f32>(fma(_e585, _e570.member_1.x, _e470.member_2.x), fma(_e585, _e570.member_1.y, _e470.member_2.y), fma(_e585, _e570.member_1.z, _e470.member_2.z)), _e608.member_3, _e608.member_4, _e608.member_5, _e608.member_6, _e608.member_7);
                            phi_1556_ = type_20(_e631.member, _e631.member_1, _e631.member_2, vec4<f32>(fma(_e585, _e570.member_2.x, _e470.member_3.x), fma(_e585, _e570.member_2.y, _e470.member_3.y), fma(_e585, _e570.member_2.z, _e470.member_3.z), _e470.member_3.w), _e631.member_4, _e631.member_5, _e631.member_6, _e631.member_7);
                            phi_1734_ = true;
                            break;
                        }
                        default: {
                            phi_1556_ = type_20();
                            phi_1734_ = bool();
                            break;
                        }
                    }
                    let _e658 = phi_1556_;
                    let _e660 = phi_1734_;
                    continue;
                    continuing {
                        phi_1552_ = _e483;
                        phi_1555_ = _e658;
                        break if !(_e660);
                    }
                }
                let _e665 = global_3.member[(_e124 + 7u)];
                if (_e665 == 1u) {
                    let _e668 = ((_e104 == 4294967295u) != true);
                    if _e668 {
                        if (_e76 >= 4u) {
                            phi_2766_ = (_e104 <= (_e76 - 4u));
                        } else {
                            phi_2766_ = false;
                        }
                        let _e673 = phi_2766_;
                        if _e673 {
                            let _e676 = global_3.member[_e104];
                            let _e680 = global_3.member[(_e104 + 1u)];
                            let _e684 = global_3.member[(_e104 + 2u)];
                            let _e688 = global_3.member[(_e104 + 3u)];
                            phi_1834_ = type_24(type_11(_e676, _e680), type_11(_e684, _e688));
                        } else {
                            phi_1834_ = type_24(type_11(4294967295u, 0u), type_11(4294967295u, 0u));
                        }
                        let _e693 = phi_1834_;
                        let _e695 = local_4;
                        local = _e695.member_5;
                        phi_1837_ = type_11(0u, 4u);
                        phi_1840_ = type_14(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        loop {
                            let _e698 = phi_1837_;
                            let _e700 = phi_1840_;
                            local_6 = _e700;
                            local_7 = _e700;
                            local_8 = _e700;
                            local_9 = _e700;
                            local_10 = _e700;
                            local_11 = _e700;
                            local_12 = _e700;
                            local_13 = _e700;
                            local_14 = _e700;
                            local_15 = _e700;
                            local_16 = _e700;
                            local_17 = _e700;
                            local_18 = _e700;
                            local_19 = _e700;
                            local_20 = _e700;
                            local_21 = _e700;
                            local_23 = _e700;
                            if (_e698.member < _e698.member_1) {
                                phi_1838_ = type_11((_e698.member + 1u), _e698.member_1);
                                phi_1863_ = type_11(1u, _e698.member);
                            } else {
                                phi_1838_ = _e698;
                                phi_1863_ = type_11(0u, type_11().member_1);
                            }
                            let _e713 = phi_1838_;
                            let _e715 = phi_1863_;
                            switch bitcast<i32>(_e715.member) {
                                case 0: {
                                    phi_1841_ = type_14();
                                    phi_2265_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e720 = local_5;
                                    local_1 = _e720.member_4;
                                    let _e722 = (_e715.member_1 < 4u);
                                    if _e722 {
                                    } else {
                                        phi_3528_ = true;
                                        break;
                                    }
                                    let _e724 = local_1[_e715.member_1];
                                    if (_e724 >= _e693.member.member_1) {
                                        phi_1887_ = 4294967295u;
                                    } else {
                                        phi_1887_ = (_e693.member.member + _e724);
                                    }
                                    let _e732 = phi_1887_;
                                    if (_e76 >= 1u) {
                                        phi_2800_ = (_e732 <= (_e76 - 1u));
                                    } else {
                                        phi_2800_ = false;
                                    }
                                    let _e737 = phi_2800_;
                                    if _e737 {
                                        let _e740 = global_3.member[_e732];
                                        phi_1896_ = _e740;
                                    } else {
                                        phi_1896_ = 4294967295u;
                                    }
                                    let _e742 = phi_1896_;
                                    if (_e76 >= 10u) {
                                        phi_2824_ = (_e742 <= (_e76 - 10u));
                                    } else {
                                        phi_2824_ = false;
                                    }
                                    let _e747 = phi_2824_;
                                    if _e747 {
                                        let _e750 = global_3.member[_e742];
                                        let _e755 = global_3.member[(_e742 + 1u)];
                                        let _e760 = global_3.member[(_e742 + 2u)];
                                        let _e766 = global_3.member[(_e742 + 3u)];
                                        let _e771 = global_3.member[(_e742 + 4u)];
                                        let _e776 = global_3.member[(_e742 + 5u)];
                                        let _e781 = global_3.member[(_e742 + 6u)];
                                        let _e787 = global_3.member[(_e742 + 7u)];
                                        let _e792 = global_3.member[(_e742 + 8u)];
                                        let _e797 = global_3.member[(_e742 + 9u)];
                                        phi_1945_ = type_17(vec3<f32>(bitcast<f32>(_e750), bitcast<f32>(_e755), bitcast<f32>(_e760)), vec4<f32>(bitcast<f32>(_e766), bitcast<f32>(_e771), bitcast<f32>(_e776), bitcast<f32>(_e781)), vec3<f32>(bitcast<f32>(_e787), bitcast<f32>(_e792), bitcast<f32>(_e797)));
                                    } else {
                                        phi_1945_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e802 = phi_1945_;
                                    if (_e724 >= _e693.member_1.member_1) {
                                        phi_1955_ = 4294967295u;
                                    } else {
                                        phi_1955_ = (_e693.member_1.member + (16u * _e724));
                                    }
                                    let _e811 = phi_1955_;
                                    if (_e76 >= 16u) {
                                        phi_2849_ = (_e811 <= (_e76 - 16u));
                                    } else {
                                        phi_2849_ = false;
                                    }
                                    let _e816 = phi_2849_;
                                    if _e816 {
                                        let _e819 = global_3.member[_e811];
                                        let _e824 = global_3.member[(_e811 + 1u)];
                                        let _e829 = global_3.member[(_e811 + 2u)];
                                        let _e834 = global_3.member[(_e811 + 3u)];
                                        let _e840 = global_3.member[(_e811 + 4u)];
                                        let _e845 = global_3.member[(_e811 + 5u)];
                                        let _e850 = global_3.member[(_e811 + 6u)];
                                        let _e855 = global_3.member[(_e811 + 7u)];
                                        let _e861 = global_3.member[(_e811 + 8u)];
                                        let _e866 = global_3.member[(_e811 + 9u)];
                                        let _e871 = global_3.member[(_e811 + 10u)];
                                        let _e876 = global_3.member[(_e811 + 11u)];
                                        let _e882 = global_3.member[(_e811 + 12u)];
                                        let _e887 = global_3.member[(_e811 + 13u)];
                                        let _e892 = global_3.member[(_e811 + 14u)];
                                        let _e897 = global_3.member[(_e811 + 15u)];
                                        phi_2028_ = type_14(vec4<f32>(bitcast<f32>(_e819), bitcast<f32>(_e824), bitcast<f32>(_e829), bitcast<f32>(_e834)), vec4<f32>(bitcast<f32>(_e840), bitcast<f32>(_e845), bitcast<f32>(_e850), bitcast<f32>(_e855)), vec4<f32>(bitcast<f32>(_e861), bitcast<f32>(_e866), bitcast<f32>(_e871), bitcast<f32>(_e876)), vec4<f32>(bitcast<f32>(_e882), bitcast<f32>(_e887), bitcast<f32>(_e892), bitcast<f32>(_e897)));
                                    } else {
                                        phi_2028_ = type_14(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                    }
                                    let _e902 = phi_2028_;
                                    let _e910 = (_e802.member_1.x + _e802.member_1.x);
                                    let _e911 = (_e802.member_1.y + _e802.member_1.y);
                                    let _e912 = (_e802.member_1.z + _e802.member_1.z);
                                    let _e914 = (_e802.member_1.z * _e912);
                                    let _e915 = (_e802.member_1.w * _e910);
                                    let _e916 = (_e802.member_1.w * _e911);
                                    let _e917 = (_e802.member_1.w * _e912);
                                    let _e937 = (vec4<f32>((1f - fma(_e802.member_1.y, _e911, _e914)), fma(_e802.member_1.x, _e911, _e917), fma(_e802.member_1.x, _e912, -(_e916)), 0f) * _e802.member_2.x);
                                    let _e939 = (vec4<f32>(fma(_e802.member_1.x, _e911, -(_e917)), (1f - fma(_e802.member_1.x, _e910, _e914)), fma(_e802.member_1.y, _e912, _e915), 0f) * _e802.member_2.y);
                                    let _e941 = (vec4<f32>(fma(_e802.member_1.x, _e912, _e916), fma(_e802.member_1.y, _e912, -(_e915)), (1f - fma(_e802.member_1.x, _e910, (_e802.member_1.y * _e911))), 0f) * _e802.member_2.z);
                                    if _e722 {
                                    } else {
                                        phi_3528_ = true;
                                        break;
                                    }
                                    let _e1046 = local[_e715.member_1];
                                    phi_1841_ = type_14((_e700.member + (vec4<f32>(fma(_e802.member.x, _e902.member.w, fma(_e941.x, _e902.member.z, fma(_e937.x, _e902.member.x, (_e939.x * _e902.member.y)))), fma(_e802.member.y, _e902.member.w, fma(_e941.y, _e902.member.z, fma(_e937.y, _e902.member.x, (_e939.y * _e902.member.y)))), fma(_e802.member.z, _e902.member.w, fma(_e941.z, _e902.member.z, fma(_e937.z, _e902.member.x, (_e939.z * _e902.member.y)))), (fma(_e941.w, _e902.member.z, fma(_e937.w, _e902.member.x, (_e939.w * _e902.member.y))) + _e902.member.w)) * _e1046)), (_e700.member_1 + (vec4<f32>(fma(_e802.member.x, _e902.member_1.w, fma(_e941.x, _e902.member_1.z, fma(_e937.x, _e902.member_1.x, (_e939.x * _e902.member_1.y)))), fma(_e802.member.y, _e902.member_1.w, fma(_e941.y, _e902.member_1.z, fma(_e937.y, _e902.member_1.x, (_e939.y * _e902.member_1.y)))), fma(_e802.member.z, _e902.member_1.w, fma(_e941.z, _e902.member_1.z, fma(_e937.z, _e902.member_1.x, (_e939.z * _e902.member_1.y)))), (fma(_e941.w, _e902.member_1.z, fma(_e937.w, _e902.member_1.x, (_e939.w * _e902.member_1.y))) + _e902.member_1.w)) * _e1046)), (_e700.member_2 + (vec4<f32>(fma(_e802.member.x, _e902.member_2.w, fma(_e941.x, _e902.member_2.z, fma(_e937.x, _e902.member_2.x, (_e939.x * _e902.member_2.y)))), fma(_e802.member.y, _e902.member_2.w, fma(_e941.y, _e902.member_2.z, fma(_e937.y, _e902.member_2.x, (_e939.y * _e902.member_2.y)))), fma(_e802.member.z, _e902.member_2.w, fma(_e941.z, _e902.member_2.z, fma(_e937.z, _e902.member_2.x, (_e939.z * _e902.member_2.y)))), (fma(_e941.w, _e902.member_2.z, fma(_e937.w, _e902.member_2.x, (_e939.w * _e902.member_2.y))) + _e902.member_2.w)) * _e1046)), (_e700.member_3 + (vec4<f32>(fma(_e802.member.x, _e902.member_3.w, fma(_e941.x, _e902.member_3.z, fma(_e937.x, _e902.member_3.x, (_e939.x * _e902.member_3.y)))), fma(_e802.member.y, _e902.member_3.w, fma(_e941.y, _e902.member_3.z, fma(_e937.y, _e902.member_3.x, (_e939.y * _e902.member_3.y)))), fma(_e802.member.z, _e902.member_3.w, fma(_e941.z, _e902.member_3.z, fma(_e937.z, _e902.member_3.x, (_e939.z * _e902.member_3.y)))), (fma(_e941.w, _e902.member_3.z, fma(_e937.w, _e902.member_3.x, (_e939.w * _e902.member_3.y))) + _e902.member_3.w)) * _e1046)));
                                    phi_2265_ = true;
                                    break;
                                }
                                default: {
                                    phi_1841_ = type_14();
                                    phi_2265_ = bool();
                                    break;
                                }
                            }
                            let _e1061 = phi_1841_;
                            let _e1063 = phi_2265_;
                            continue;
                            continuing {
                                phi_1837_ = _e713;
                                phi_1840_ = _e1061;
                                phi_3528_ = false;
                                break if !(_e1063);
                            }
                        }
                        let _e1066 = phi_3528_;
                        if _e1066 {
                            break;
                        }
                        let _e1068 = local_6;
                        let _e1073 = global_4.member[0u];
                        if (_e1068.member.x == _e1073) {
                            let _e1076 = local_7;
                            let _e1081 = global_4.member[1u];
                            if (_e1076.member.y == _e1081) {
                                let _e1084 = local_8;
                                let _e1089 = global_4.member[2u];
                                let _e1090 = (_e1084.member.z == _e1089);
                                if _e1090 {
                                    let _e1092 = local_9;
                                    let _e1097 = global_4.member[3u];
                                    phi_2292_ = (_e1092.member.w == _e1097);
                                } else {
                                    phi_2292_ = bool();
                                }
                                let _e1100 = phi_2292_;
                                phi_2294_ = _e1100;
                                phi_2295_ = select(true, false, _e1090);
                            } else {
                                phi_2294_ = bool();
                                phi_2295_ = true;
                            }
                            let _e1103 = phi_2294_;
                            let _e1105 = phi_2295_;
                            phi_2296_ = _e1103;
                            phi_2297_ = _e1105;
                        } else {
                            phi_2296_ = bool();
                            phi_2297_ = true;
                        }
                        let _e1107 = phi_2296_;
                        let _e1109 = phi_2297_;
                        if select(_e1107, false, _e1109) {
                            let _e1112 = local_10;
                            let _e1117 = global_4.member_1[0u];
                            if (_e1112.member_1.x == _e1117) {
                                let _e1120 = local_11;
                                let _e1125 = global_4.member_1[1u];
                                if (_e1120.member_1.y == _e1125) {
                                    let _e1128 = local_12;
                                    let _e1133 = global_4.member_1[2u];
                                    let _e1134 = (_e1128.member_1.z == _e1133);
                                    if _e1134 {
                                        let _e1136 = local_13;
                                        let _e1141 = global_4.member_1[3u];
                                        phi_2331_ = (_e1136.member_1.w == _e1141);
                                    } else {
                                        phi_2331_ = bool();
                                    }
                                    let _e1144 = phi_2331_;
                                    phi_2333_ = _e1144;
                                    phi_2334_ = select(true, false, _e1134);
                                } else {
                                    phi_2333_ = bool();
                                    phi_2334_ = true;
                                }
                                let _e1147 = phi_2333_;
                                let _e1149 = phi_2334_;
                                phi_2335_ = _e1147;
                                phi_2336_ = _e1149;
                            } else {
                                phi_2335_ = bool();
                                phi_2336_ = true;
                            }
                            let _e1151 = phi_2335_;
                            let _e1153 = phi_2336_;
                            if select(_e1151, false, _e1153) {
                                let _e1156 = local_14;
                                let _e1161 = global_4.member_2[0u];
                                if (_e1156.member_2.x == _e1161) {
                                    let _e1164 = local_15;
                                    let _e1169 = global_4.member_2[1u];
                                    if (_e1164.member_2.y == _e1169) {
                                        let _e1172 = local_16;
                                        let _e1177 = global_4.member_2[2u];
                                        let _e1178 = (_e1172.member_2.z == _e1177);
                                        if _e1178 {
                                            let _e1180 = local_17;
                                            let _e1185 = global_4.member_2[3u];
                                            phi_2370_ = (_e1180.member_2.w == _e1185);
                                        } else {
                                            phi_2370_ = bool();
                                        }
                                        let _e1188 = phi_2370_;
                                        phi_2372_ = _e1188;
                                        phi_2373_ = select(true, false, _e1178);
                                    } else {
                                        phi_2372_ = bool();
                                        phi_2373_ = true;
                                    }
                                    let _e1191 = phi_2372_;
                                    let _e1193 = phi_2373_;
                                    phi_2374_ = _e1191;
                                    phi_2375_ = _e1193;
                                } else {
                                    phi_2374_ = bool();
                                    phi_2375_ = true;
                                }
                                let _e1195 = phi_2374_;
                                let _e1197 = phi_2375_;
                                let _e1198 = select(_e1195, false, _e1197);
                                if _e1198 {
                                    let _e1200 = local_18;
                                    let _e1205 = global_4.member_3[0u];
                                    if (_e1200.member_3.x == _e1205) {
                                        let _e1208 = local_19;
                                        let _e1213 = global_4.member_3[1u];
                                        if (_e1208.member_3.y == _e1213) {
                                            let _e1216 = local_20;
                                            let _e1221 = global_4.member_3[2u];
                                            let _e1222 = (_e1216.member_3.z == _e1221);
                                            if _e1222 {
                                                let _e1224 = local_21;
                                                let _e1229 = global_4.member_3[3u];
                                                phi_2409_ = (_e1224.member_3.w == _e1229);
                                            } else {
                                                phi_2409_ = bool();
                                            }
                                            let _e1232 = phi_2409_;
                                            phi_2411_ = _e1232;
                                            phi_2412_ = select(true, false, _e1222);
                                        } else {
                                            phi_2411_ = bool();
                                            phi_2412_ = true;
                                        }
                                        let _e1235 = phi_2411_;
                                        let _e1237 = phi_2412_;
                                        phi_2413_ = _e1235;
                                        phi_2414_ = _e1237;
                                    } else {
                                        phi_2413_ = bool();
                                        phi_2414_ = true;
                                    }
                                    let _e1239 = phi_2413_;
                                    let _e1241 = phi_2414_;
                                    phi_2419_ = select(_e1239, false, _e1241);
                                } else {
                                    phi_2419_ = bool();
                                }
                                let _e1244 = phi_2419_;
                                phi_2421_ = _e1244;
                                phi_2422_ = select(true, false, _e1198);
                            } else {
                                phi_2421_ = bool();
                                phi_2422_ = true;
                            }
                            let _e1247 = phi_2421_;
                            let _e1249 = phi_2422_;
                            phi_2423_ = _e1247;
                            phi_2424_ = _e1249;
                        } else {
                            phi_2423_ = bool();
                            phi_2424_ = true;
                        }
                        let _e1251 = phi_2423_;
                        let _e1253 = phi_2424_;
                        if select(_e1251, false, _e1253) {
                            phi_2432_ = type_14(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        } else {
                            let _e1746 = local_23;
                            phi_2432_ = _e1746;
                        }
                        let _e1256 = phi_2432_;
                        let _e1279 = fma(_e1256.member_2.z, _e1256.member_3.w, -((_e1256.member_2.w * _e1256.member_3.z)));
                        let _e1282 = fma(_e1256.member_2.y, _e1256.member_3.w, -((_e1256.member_2.w * _e1256.member_3.y)));
                        let _e1285 = fma(_e1256.member_2.y, _e1256.member_3.z, -((_e1256.member_2.z * _e1256.member_3.y)));
                        let _e1288 = fma(_e1256.member_2.x, _e1256.member_3.w, -((_e1256.member_2.w * _e1256.member_3.x)));
                        let _e1291 = fma(_e1256.member_2.x, _e1256.member_3.z, -((_e1256.member_2.z * _e1256.member_3.x)));
                        let _e1294 = fma(_e1256.member_2.x, _e1256.member_3.y, -((_e1256.member_2.y * _e1256.member_3.x)));
                        let _e1316 = fma(-(_e1256.member.w), fma(_e1256.member_1.z, _e1294, fma(_e1256.member_1.x, _e1285, -((_e1256.member_1.y * _e1291)))), fma(_e1256.member.z, fma(_e1256.member_1.w, _e1294, fma(_e1256.member_1.x, _e1282, -((_e1256.member_1.y * _e1288)))), fma(_e1256.member.x, fma(_e1256.member_1.w, _e1285, fma(_e1256.member_1.y, _e1279, -((_e1256.member_1.z * _e1282)))), -((_e1256.member.y * fma(_e1256.member_1.w, _e1291, fma(_e1256.member_1.x, _e1279, -((_e1256.member_1.z * _e1288)))))))));
                        if (_e1316 == 0f) {
                            phi_3096_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            phi_3097_ = type_17();
                            phi_3098_ = true;
                        } else {
                            let _e1325 = (sqrt(fma(_e1256.member.w, _e1256.member.w, fma(_e1256.member.z, _e1256.member.z, fma(_e1256.member.x, _e1256.member.x, (_e1256.member.y * _e1256.member.y))))) * select(-1f, 1f, (_e1316 >= 0f)));
                            let _e1330 = sqrt(fma(_e1256.member_1.w, _e1256.member_1.w, fma(_e1256.member_1.z, _e1256.member_1.z, fma(_e1256.member_1.x, _e1256.member_1.x, (_e1256.member_1.y * _e1256.member_1.y)))));
                            let _e1335 = sqrt(fma(_e1256.member_2.w, _e1256.member_2.w, fma(_e1256.member_2.z, _e1256.member_2.z, fma(_e1256.member_2.x, _e1256.member_2.x, (_e1256.member_2.y * _e1256.member_2.y)))));
                            if (_e1325 != 0f) {
                                phi_2984_ = select(true, false, (_e1330 != 0f));
                            } else {
                                phi_2984_ = true;
                            }
                            let _e1342 = phi_2984_;
                            let _e1343 = select((_e1335 != 0f), false, _e1342);
                            if _e1343 {
                                let _e1344 = (1f / _e1325);
                                let _e1345 = (1f / _e1330);
                                let _e1346 = (1f / _e1335);
                                let _e1347 = (_e1256.member.x * _e1344);
                                let _e1348 = (_e1256.member.z * _e1344);
                                let _e1349 = (_e1256.member_1.x * _e1345);
                                let _e1350 = (_e1256.member_2.x * _e1346);
                                let _e1351 = (_e1256.member_2.y * _e1346);
                                if ((_e1256.member_2.z * _e1346) <= 0f) {
                                    let _e1386 = fma(_e1256.member_1.y, _e1345, -(_e1347));
                                    let _e1388 = fma(-(_e1256.member_2.z), _e1346, 1f);
                                    if (_e1386 <= 0f) {
                                        let _e1402 = (_e1388 - _e1386);
                                        let _e1404 = (0.5f / sqrt(_e1402));
                                        phi_3079_ = vec4<f32>((_e1402 * _e1404), (fma(_e1256.member.y, _e1344, _e1349) * _e1404), (fma(_e1256.member.z, _e1344, _e1350) * _e1404), (fma(_e1256.member_1.z, _e1345, -(_e1351)) * _e1404));
                                    } else {
                                        let _e1390 = (_e1388 + _e1386);
                                        let _e1392 = (0.5f / sqrt(_e1390));
                                        phi_3079_ = vec4<f32>((fma(_e1256.member.y, _e1344, _e1349) * _e1392), (_e1390 * _e1392), (fma(_e1256.member_1.z, _e1345, _e1351) * _e1392), (fma(_e1256.member_2.x, _e1346, -(_e1348)) * _e1392));
                                    }
                                    let _e1415 = phi_3079_;
                                    phi_3081_ = _e1415;
                                } else {
                                    let _e1354 = fma(_e1256.member_1.y, _e1345, _e1347);
                                    let _e1355 = fma(_e1256.member_2.z, _e1346, 1f);
                                    if (_e1354 <= 0f) {
                                        let _e1371 = (_e1355 - _e1354);
                                        let _e1373 = (0.5f / sqrt(_e1371));
                                        phi_3049_ = vec4<f32>((fma(_e1256.member.z, _e1344, _e1350) * _e1373), (fma(_e1256.member_1.z, _e1345, _e1351) * _e1373), (_e1371 * _e1373), (fma(_e1256.member.y, _e1344, -(_e1349)) * _e1373));
                                    } else {
                                        let _e1357 = (_e1355 + _e1354);
                                        let _e1359 = (0.5f / sqrt(_e1357));
                                        phi_3049_ = vec4<f32>((fma(_e1256.member_1.z, _e1345, -(_e1351)) * _e1359), (fma(_e1256.member_2.x, _e1346, -(_e1348)) * _e1359), (fma(_e1256.member.y, _e1344, -(_e1349)) * _e1359), (_e1357 * _e1359));
                                    }
                                    let _e1384 = phi_3049_;
                                    phi_3081_ = _e1384;
                                }
                                let _e1417 = phi_3081_;
                                phi_3090_ = type_17(vec3<f32>(_e1325, _e1330, _e1335), _e1417, vec3<f32>(_e1256.member_3.x, _e1256.member_3.y, _e1256.member_3.z));
                                phi_3091_ = type_17();
                            } else {
                                phi_3090_ = type_17();
                                phi_3091_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e1421 = phi_3090_;
                            let _e1423 = phi_3091_;
                            phi_3096_ = _e1423;
                            phi_3097_ = _e1421;
                            phi_3098_ = select(true, false, _e1343);
                        }
                        let _e1426 = phi_3096_;
                        let _e1428 = phi_3097_;
                        let _e1430 = phi_3098_;
                        if _e1430 {
                            phi_3102_ = _e1426;
                        } else {
                            phi_3102_ = _e1428;
                        }
                        let _e1432 = phi_3102_;
                        phi_2434_ = type_17(_e1432.member_2, _e1432.member_1, _e1432.member);
                    } else {
                        phi_2434_ = type_17();
                    }
                    let _e1438 = phi_2434_;
                    phi_2436_ = _e1438;
                    phi_2437_ = select(true, false, _e668);
                } else {
                    phi_2436_ = type_17();
                    phi_2437_ = true;
                }
                let _e1441 = phi_2436_;
                let _e1443 = phi_2437_;
                if _e1443 {
                    if (_e76 >= 10u) {
                        phi_3196_ = (_e100 <= (_e76 - 10u));
                    } else {
                        phi_3196_ = false;
                    }
                    let _e1448 = phi_3196_;
                    if _e1448 {
                        let _e1451 = global_3.member[_e100];
                        let _e1456 = global_3.member[(_e100 + 1u)];
                        let _e1461 = global_3.member[(_e100 + 2u)];
                        let _e1467 = global_3.member[(_e100 + 3u)];
                        let _e1472 = global_3.member[(_e100 + 4u)];
                        let _e1477 = global_3.member[(_e100 + 5u)];
                        let _e1482 = global_3.member[(_e100 + 6u)];
                        let _e1488 = global_3.member[(_e100 + 7u)];
                        let _e1493 = global_3.member[(_e100 + 8u)];
                        let _e1498 = global_3.member[(_e100 + 9u)];
                        phi_2490_ = type_17(vec3<f32>(bitcast<f32>(_e1451), bitcast<f32>(_e1456), bitcast<f32>(_e1461)), vec4<f32>(bitcast<f32>(_e1467), bitcast<f32>(_e1472), bitcast<f32>(_e1477), bitcast<f32>(_e1482)), vec3<f32>(bitcast<f32>(_e1488), bitcast<f32>(_e1493), bitcast<f32>(_e1498)));
                    } else {
                        phi_2490_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e1503 = phi_2490_;
                    phi_2491_ = _e1503;
                } else {
                    phi_2491_ = _e1441;
                }
                let _e1505 = phi_2491_;
                let _e1513 = (_e1505.member_1.x + _e1505.member_1.x);
                let _e1514 = (_e1505.member_1.y + _e1505.member_1.y);
                let _e1515 = (_e1505.member_1.z + _e1505.member_1.z);
                let _e1517 = (_e1505.member_1.z * _e1515);
                let _e1518 = (_e1505.member_1.w * _e1513);
                let _e1519 = (_e1505.member_1.w * _e1514);
                let _e1520 = (_e1505.member_1.w * _e1515);
                let _e1540 = (vec4<f32>((1f - fma(_e1505.member_1.y, _e1514, _e1517)), fma(_e1505.member_1.x, _e1514, _e1520), fma(_e1505.member_1.x, _e1515, -(_e1519)), 0f) * _e1505.member_2.x);
                let _e1542 = (vec4<f32>(fma(_e1505.member_1.x, _e1514, -(_e1520)), (1f - fma(_e1505.member_1.x, _e1513, _e1517)), fma(_e1505.member_1.y, _e1515, _e1518), 0f) * _e1505.member_2.y);
                let _e1544 = (vec4<f32>(fma(_e1505.member_1.x, _e1515, _e1519), fma(_e1505.member_1.y, _e1515, -(_e1518)), (1f - fma(_e1505.member_1.x, _e1513, (_e1505.member_1.y * _e1514))), 0f) * _e1505.member_2.z);
                let _e1549 = local_22;
                let _e1572 = (_e1505.member.x + fma(_e1544.x, _e1549.member.z, fma(_e1542.x, _e1549.member.y, (_e1540.x * _e1549.member.x))));
                let _e1573 = (_e1505.member.y + fma(_e1544.y, _e1549.member.z, fma(_e1542.y, _e1549.member.y, (_e1540.y * _e1549.member.x))));
                let _e1574 = (_e1505.member.z + fma(_e1544.z, _e1549.member.z, fma(_e1542.z, _e1549.member.y, (_e1540.z * _e1549.member.x))));
                global_1 = vec4<f32>((fma(fma(_e191, _e266, fma(_e171, _e261, fma(_e131, _e251, (_e151 * _e256)))), _e1574, fma(fma(_e191, _e226, fma(_e171, _e221, fma(_e131, _e211, (_e151 * _e216)))), _e1572, (fma(_e191, _e246, fma(_e171, _e241, fma(_e131, _e231, (_e151 * _e236)))) * _e1573))) + fma(_e191, _e286, fma(_e171, _e281, fma(_e131, _e271, (_e151 * _e276))))), (fma(fma(_e196, _e266, fma(_e176, _e261, fma(_e136, _e251, (_e156 * _e256)))), _e1574, fma(fma(_e196, _e226, fma(_e176, _e221, fma(_e136, _e211, (_e156 * _e216)))), _e1572, (fma(_e196, _e246, fma(_e176, _e241, fma(_e136, _e231, (_e156 * _e236)))) * _e1573))) + fma(_e196, _e286, fma(_e176, _e281, fma(_e136, _e271, (_e156 * _e276))))), (fma(fma(_e201, _e266, fma(_e181, _e261, fma(_e141, _e251, (_e161 * _e256)))), _e1574, fma(fma(_e201, _e226, fma(_e181, _e221, fma(_e141, _e211, (_e161 * _e216)))), _e1572, (fma(_e201, _e246, fma(_e181, _e241, fma(_e141, _e231, (_e161 * _e236)))) * _e1573))) + fma(_e201, _e286, fma(_e181, _e281, fma(_e141, _e271, (_e161 * _e276))))), (fma(fma(_e206, _e266, fma(_e186, _e261, fma(_e146, _e251, (_e166 * _e256)))), _e1574, fma(fma(_e206, _e226, fma(_e186, _e221, fma(_e146, _e211, (_e166 * _e216)))), _e1572, (fma(_e206, _e246, fma(_e186, _e241, fma(_e146, _e231, (_e166 * _e236)))) * _e1573))) + fma(_e206, _e286, fma(_e186, _e281, fma(_e146, _e271, (_e166 * _e276))))));
            } else {
                global_1 = vec4<f32>(100f, 100f, 100f, 1f);
            }
            break;
        }
    }
    return;
}

@vertex 
fn lightlight_tiling_depth_pre_pass(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> @builtin(position) vec4<f32> {
    global_2 = param;
    global = param_1;
    function();
    let _e6 = global_1.y;
    global_1.y = -(_e6);
    let _e8 = global_1;
    return _e8;
}
