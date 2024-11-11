struct type_8 {
    member: array<u32>,
}

struct type_14 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_15 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
}

struct type_16 {
    member: type_14,
    member_1: type_14,
    member_2: type_15,
    member_3: vec3<f32>,
}

struct type_18 {
    member: u32,
    member_1: u32,
}

struct type_22 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_24 {
    member: array<type_22>,
}

struct type_29 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_30 {
    member: bool,
    member_1: u32,
}

@group(0) @binding(0) 
var<storage, read_write> global: type_8;
@group(0) @binding(1) 
var<storage, read_write> global_1: type_24;
var<private> global_2: vec3<u32>;

fn function() {
    var local: array<vec4<f32>, 6>;
    var local_1: array<vec3<f32>, 8>;
    var local_2: array<f32, 3>;
    var local_3: array<f32, 3>;
    var local_4: array<f32, 3>;
    var local_5: array<f32, 3>;
    var local_6: array<vec3<f32>, 8>;
    var local_7: array<vec4<f32>, 6>;
    var phi_1208_: bool;
    var phi_460_: type_18;
    var phi_461_: type_18;
    var phi_476_: type_18;
    var phi_503_: bool;
    var phi_509_: type_18;
    var phi_510_: type_18;
    var phi_525_: type_18;
    var phi_548_: bool;
    var phi_556_: type_16;
    var phi_1240_: bool;
    var phi_607_: type_29;
    var phi_739_: type_18;
    var phi_740_: type_18;
    var phi_755_: type_18;
    var phi_799_: bool;
    var phi_803_: bool;
    var phi_804_: bool;
    var phi_1415_: bool;
    var phi_1154_: bool;
    var phi_1438_: bool;
    var phi_811_: type_18;
    var phi_812_: type_18;
    var phi_827_: type_18;
    var phi_837_: type_18;
    var phi_840_: i32;
    var phi_838_: type_18;
    var phi_855_: type_18;
    var phi_896_: i32;
    var phi_841_: i32;
    var phi_897_: bool;
    var phi_1433_: bool;
    var local_8: i32;
    var phi_904_: type_18;
    var phi_907_: i32;
    var phi_905_: type_18;
    var phi_922_: type_18;
    var phi_963_: i32;
    var phi_908_: i32;
    var phi_964_: bool;
    var phi_1440_: bool;
    var local_9: i32;
    var phi_1447_: bool;
    var phi_970_: bool;
    var phi_971_: bool;
    var phi_1446_: bool;
    var phi_972_: bool;
    var phi_973_: bool;
    var phi_974_: bool;
    var phi_975_: bool;
    var phi_1445_: bool;
    var phi_1157_: bool;
    var phi_1156_: bool;
    var phi_1155_: bool;
    var phi_998_: type_30;
    var phi_999_: type_30;
    var local_10: u32;
    var phi_1005_: type_30;
    var phi_1006_: type_30;

    switch bitcast<i32>(0u) {
        default: {
            let _e82 = arrayLength((&global.member));
            let _e85 = global_2;
            if (_e85.x >= arrayLength((&global_1.member))) {
            } else {
                let _e91 = global_1.member[_e85.x].member_3;
                let _e94 = global.member[_e91];
                let _e99 = global.member[(_e91 + 2u)];
                let _e103 = global.member[(_e91 + 3u)];
                let _e104 = bitcast<f32>(_e103);
                let _e108 = global.member[(_e91 + 4u)];
                let _e109 = bitcast<f32>(_e108);
                let _e113 = global.member[(_e91 + 5u)];
                let _e114 = bitcast<f32>(_e113);
                let _e118 = global.member[(_e91 + 6u)];
                let _e119 = bitcast<f32>(_e118);
                let _e123 = global.member[(_e91 + 7u)];
                let _e127 = global.member[(_e91 + 8u)];
                let _e131 = global.member[(_e91 + 9u)];
                let _e135 = global.member[(_e91 + 10u)];
                global_1.member[_e85.x].member = select(_e127, _e99, (_e123 == 4294967295u));
                global_1.member[_e85.x].member_1 = select(0u, 1u, (_e94 == 1u));
                if (_e119 == 0f) {
                } else {
                    if (_e82 >= 83u) {
                        phi_1208_ = (_e131 <= (_e82 - 83u));
                    } else {
                        phi_1208_ = false;
                    }
                    let _e150 = phi_1208_;
                    if _e150 {
                        let _e153 = global.member[_e131];
                        let _e158 = global.member[(_e131 + 1u)];
                        let _e163 = global.member[(_e131 + 2u)];
                        let _e168 = global.member[(_e131 + 3u)];
                        let _e174 = global.member[(_e131 + 4u)];
                        let _e179 = global.member[(_e131 + 5u)];
                        let _e184 = global.member[(_e131 + 6u)];
                        let _e189 = global.member[(_e131 + 7u)];
                        let _e195 = global.member[(_e131 + 8u)];
                        let _e200 = global.member[(_e131 + 9u)];
                        let _e205 = global.member[(_e131 + 10u)];
                        let _e210 = global.member[(_e131 + 11u)];
                        let _e216 = global.member[(_e131 + 12u)];
                        let _e221 = global.member[(_e131 + 13u)];
                        let _e226 = global.member[(_e131 + 14u)];
                        let _e231 = global.member[(_e131 + 15u)];
                        let _e238 = global.member[(_e131 + 16u)];
                        let _e243 = global.member[(_e131 + 17u)];
                        let _e248 = global.member[(_e131 + 18u)];
                        let _e253 = global.member[(_e131 + 19u)];
                        let _e259 = global.member[(_e131 + 20u)];
                        let _e264 = global.member[(_e131 + 21u)];
                        let _e269 = global.member[(_e131 + 22u)];
                        let _e274 = global.member[(_e131 + 23u)];
                        let _e280 = global.member[(_e131 + 24u)];
                        let _e285 = global.member[(_e131 + 25u)];
                        let _e290 = global.member[(_e131 + 26u)];
                        let _e295 = global.member[(_e131 + 27u)];
                        let _e301 = global.member[(_e131 + 28u)];
                        let _e306 = global.member[(_e131 + 29u)];
                        let _e311 = global.member[(_e131 + 30u)];
                        let _e316 = global.member[(_e131 + 31u)];
                        let _e323 = global.member[(_e131 + 32u)];
                        let _e328 = global.member[(_e131 + 33u)];
                        let _e333 = global.member[(_e131 + 34u)];
                        local_7 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        phi_460_ = type_18(0u, 6u);
                        loop {
                            let _e338 = phi_460_;
                            if (_e338.member < _e338.member_1) {
                                phi_461_ = type_18((_e338.member + 1u), _e338.member_1);
                                phi_476_ = type_18(1u, _e338.member);
                            } else {
                                phi_461_ = _e338;
                                phi_476_ = type_18(0u, type_18().member_1);
                            }
                            let _e351 = phi_461_;
                            let _e353 = phi_476_;
                            switch bitcast<i32>(_e353.member) {
                                case 0: {
                                    phi_503_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e358 = ((_e131 + 35u) + (_e353.member_1 * 4u));
                                    let _e361 = global.member[_e358];
                                    let _e366 = global.member[(_e358 + 1u)];
                                    let _e371 = global.member[(_e358 + 2u)];
                                    let _e376 = global.member[(_e358 + 3u)];
                                    local_7[_e353.member_1] = vec4<f32>(bitcast<f32>(_e361), bitcast<f32>(_e366), bitcast<f32>(_e371), bitcast<f32>(_e376));
                                    phi_503_ = true;
                                    break;
                                }
                                default: {
                                    phi_503_ = bool();
                                    break;
                                }
                            }
                            let _e381 = phi_503_;
                            continue;
                            continuing {
                                phi_460_ = _e351;
                                break if !(_e381);
                            }
                        }
                        let _e383 = local_7;
                        local_6 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                        phi_509_ = type_18(0u, 8u);
                        loop {
                            let _e386 = phi_509_;
                            if (_e386.member < _e386.member_1) {
                                phi_510_ = type_18((_e386.member + 1u), _e386.member_1);
                                phi_525_ = type_18(1u, _e386.member);
                            } else {
                                phi_510_ = _e386;
                                phi_525_ = type_18(0u, type_18().member_1);
                            }
                            let _e399 = phi_510_;
                            let _e401 = phi_525_;
                            switch bitcast<i32>(_e401.member) {
                                case 0: {
                                    phi_548_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e406 = ((_e131 + 59u) + (_e401.member_1 * 3u));
                                    let _e409 = global.member[_e406];
                                    let _e414 = global.member[(_e406 + 1u)];
                                    let _e419 = global.member[(_e406 + 2u)];
                                    local_6[_e401.member_1] = vec3<f32>(bitcast<f32>(_e409), bitcast<f32>(_e414), bitcast<f32>(_e419));
                                    phi_548_ = true;
                                    break;
                                }
                                default: {
                                    phi_548_ = bool();
                                    break;
                                }
                            }
                            let _e424 = phi_548_;
                            continue;
                            continuing {
                                phi_509_ = _e399;
                                break if !(_e424);
                            }
                        }
                        let _e426 = local_6;
                        phi_556_ = type_16(type_14(vec4<f32>(bitcast<f32>(_e153), bitcast<f32>(_e158), bitcast<f32>(_e163), bitcast<f32>(_e168)), vec4<f32>(bitcast<f32>(_e174), bitcast<f32>(_e179), bitcast<f32>(_e184), bitcast<f32>(_e189)), vec4<f32>(bitcast<f32>(_e195), bitcast<f32>(_e200), bitcast<f32>(_e205), bitcast<f32>(_e210)), vec4<f32>(bitcast<f32>(_e216), bitcast<f32>(_e221), bitcast<f32>(_e226), bitcast<f32>(_e231))), type_14(vec4<f32>(bitcast<f32>(_e238), bitcast<f32>(_e243), bitcast<f32>(_e248), bitcast<f32>(_e253)), vec4<f32>(bitcast<f32>(_e259), bitcast<f32>(_e264), bitcast<f32>(_e269), bitcast<f32>(_e274)), vec4<f32>(bitcast<f32>(_e280), bitcast<f32>(_e285), bitcast<f32>(_e290), bitcast<f32>(_e295)), vec4<f32>(bitcast<f32>(_e301), bitcast<f32>(_e306), bitcast<f32>(_e311), bitcast<f32>(_e316))), type_15(_e426, _e383), vec3<f32>(bitcast<f32>(_e323), bitcast<f32>(_e328), bitcast<f32>(_e333)));
                    } else {
                        phi_556_ = type_16(type_14(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_14(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_15(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
                    }
                    let _e430 = phi_556_;
                    if (_e82 >= 10u) {
                        phi_1240_ = (_e135 <= (_e82 - 10u));
                    } else {
                        phi_1240_ = false;
                    }
                    let _e436 = phi_1240_;
                    if _e436 {
                        let _e439 = global.member[_e135];
                        let _e444 = global.member[(_e135 + 1u)];
                        let _e449 = global.member[(_e135 + 2u)];
                        let _e455 = global.member[(_e135 + 3u)];
                        let _e460 = global.member[(_e135 + 4u)];
                        let _e465 = global.member[(_e135 + 5u)];
                        let _e470 = global.member[(_e135 + 6u)];
                        let _e476 = global.member[(_e135 + 7u)];
                        let _e481 = global.member[(_e135 + 8u)];
                        let _e486 = global.member[(_e135 + 9u)];
                        phi_607_ = type_29(vec3<f32>(bitcast<f32>(_e439), bitcast<f32>(_e444), bitcast<f32>(_e449)), vec4<f32>(bitcast<f32>(_e455), bitcast<f32>(_e460), bitcast<f32>(_e465), bitcast<f32>(_e470)), vec3<f32>(bitcast<f32>(_e476), bitcast<f32>(_e481), bitcast<f32>(_e486)));
                    } else {
                        phi_607_ = type_29(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e491 = phi_607_;
                    let _e499 = (_e491.member_1.x + _e491.member_1.x);
                    let _e500 = (_e491.member_1.y + _e491.member_1.y);
                    let _e501 = (_e491.member_1.z + _e491.member_1.z);
                    let _e503 = (_e491.member_1.z * _e501);
                    let _e504 = (_e491.member_1.w * _e499);
                    let _e505 = (_e491.member_1.w * _e500);
                    let _e506 = (_e491.member_1.w * _e501);
                    let _e526 = (vec4<f32>((1f - fma(_e491.member_1.y, _e500, _e503)), fma(_e491.member_1.x, _e500, _e506), fma(_e491.member_1.x, _e501, -(_e505)), 0f) * _e491.member_2.x);
                    let _e528 = (vec4<f32>(fma(_e491.member_1.x, _e500, -(_e506)), (1f - fma(_e491.member_1.x, _e499, _e503)), fma(_e491.member_1.y, _e501, _e504), 0f) * _e491.member_2.y);
                    let _e530 = (vec4<f32>(fma(_e491.member_1.x, _e501, _e505), fma(_e491.member_1.y, _e501, -(_e504)), (1f - fma(_e491.member_1.x, _e499, (_e491.member_1.y * _e500))), 0f) * _e491.member_2.z);
                    let _e552 = (_e491.member.x + fma(_e530.x, _e114, fma(_e528.x, _e109, (_e526.x * _e104))));
                    let _e553 = (_e491.member.y + fma(_e530.y, _e114, fma(_e528.y, _e109, (_e526.y * _e104))));
                    let _e554 = (_e491.member.z + fma(_e530.z, _e114, fma(_e528.z, _e109, (_e526.z * _e104))));
                    let _e555 = vec3<f32>(_e552, _e553, _e554);
                    let _e558 = (max(_e491.member_2.x, max(_e491.member_2.y, _e491.member_2.z)) * _e119);
                    let _e560 = sqrt((_e558 * _e558));
                    local_1 = _e430.member_2.member;
                    local = _e430.member_2.member_1;
                    let _e565 = local[0u][0u];
                    let _e568 = local[0u][1u];
                    let _e573 = local[0u][2u];
                    let _e577 = local[0u][3u];
                    let _e579 = -(_e560);
                    if ((fma(_e573, _e554, fma(_e565, _e552, (_e568 * _e553))) + _e577) < _e579) {
                        phi_1006_ = type_30(true, 0u);
                    } else {
                        phi_739_ = type_18(0u, 6u);
                        loop {
                            let _e582 = phi_739_;
                            if (_e582.member < _e582.member_1) {
                                phi_740_ = type_18((_e582.member + 1u), _e582.member_1);
                                phi_755_ = type_18(1u, _e582.member);
                            } else {
                                phi_740_ = _e582;
                                phi_755_ = type_18(0u, type_18().member_1);
                            }
                            let _e595 = phi_740_;
                            let _e597 = phi_755_;
                            local_10 = _e597.member_1;
                            switch bitcast<i32>(_e597.member) {
                                case 0: {
                                    phi_803_ = false;
                                    phi_804_ = true;
                                    break;
                                }
                                case 1: {
                                    if (_e597.member_1 != 0u) {
                                        if (_e597.member_1 < 6u) {
                                        } else {
                                            phi_1415_ = true;
                                            phi_1154_ = bool();
                                            break;
                                        }
                                        let _e605 = local[_e597.member_1][0u];
                                        let _e608 = local[_e597.member_1][1u];
                                        let _e613 = local[_e597.member_1][2u];
                                        let _e617 = local[_e597.member_1][3u];
                                        phi_799_ = select(true, false, ((fma(_e613, _e554, fma(_e605, _e552, (_e608 * _e553))) + _e617) < _e579));
                                    } else {
                                        phi_799_ = true;
                                    }
                                    let _e622 = phi_799_;
                                    phi_803_ = _e622;
                                    phi_804_ = false;
                                    break;
                                }
                                default: {
                                    phi_803_ = bool();
                                    phi_804_ = bool();
                                    break;
                                }
                            }
                            let _e624 = phi_803_;
                            let _e626 = phi_804_;
                            continue;
                            continuing {
                                phi_739_ = _e595;
                                phi_1415_ = false;
                                phi_1154_ = _e626;
                                break if !(_e624);
                            }
                        }
                        let _e629 = phi_1415_;
                        let _e631 = phi_1154_;
                        if _e629 {
                            break;
                        }
                        if _e631 {
                            let _e632 = vec3(_e560);
                            let _e633 = (_e555 - _e632);
                            let _e634 = (_e555 + _e632);
                            phi_1438_ = _e629;
                            phi_811_ = type_18(0u, 3u);
                            loop {
                                let _e636 = phi_1438_;
                                let _e638 = phi_811_;
                                if (_e638.member < _e638.member_1) {
                                    phi_812_ = type_18((_e638.member + 1u), _e638.member_1);
                                    phi_827_ = type_18(1u, _e638.member);
                                } else {
                                    phi_812_ = _e638;
                                    phi_827_ = type_18(0u, type_18().member_1);
                                }
                                let _e651 = phi_812_;
                                let _e653 = phi_827_;
                                switch bitcast<i32>(_e653.member) {
                                    case 0: {
                                        phi_1446_ = _e636;
                                        phi_972_ = false;
                                        phi_973_ = true;
                                        phi_974_ = false;
                                        phi_975_ = false;
                                        break;
                                    }
                                    case 1: {
                                        phi_837_ = type_18(0u, 8u);
                                        phi_840_ = 0i;
                                        loop {
                                            let _e658 = phi_837_;
                                            let _e660 = phi_840_;
                                            local_8 = _e660;
                                            if (_e658.member < _e658.member_1) {
                                                phi_838_ = type_18((_e658.member + 1u), _e658.member_1);
                                                phi_855_ = type_18(1u, _e658.member);
                                            } else {
                                                phi_838_ = _e658;
                                                phi_855_ = type_18(0u, type_18().member_1);
                                            }
                                            let _e673 = phi_838_;
                                            let _e675 = phi_855_;
                                            switch bitcast<i32>(_e675.member) {
                                                case 0: {
                                                    phi_841_ = i32();
                                                    phi_897_ = false;
                                                    break;
                                                }
                                                case 1: {
                                                    if (_e675.member_1 < 8u) {
                                                    } else {
                                                        phi_1433_ = true;
                                                        break;
                                                    }
                                                    let _e682 = local_1[_e675.member_1][0u];
                                                    let _e685 = local_1[_e675.member_1][1u];
                                                    let _e688 = local_1[_e675.member_1][2u];
                                                    local_2 = array<f32, 3>(_e682, _e685, _e688);
                                                    let _e690 = (_e653.member_1 < 3u);
                                                    if _e690 {
                                                    } else {
                                                        phi_1433_ = true;
                                                        break;
                                                    }
                                                    let _e692 = local_2[_e653.member_1];
                                                    local_3 = array<f32, 3>(_e633.x, _e633.y, _e633.z);
                                                    if _e690 {
                                                    } else {
                                                        phi_1433_ = true;
                                                        break;
                                                    }
                                                    let _e698 = local_3[_e653.member_1];
                                                    if (_e692 < _e698) {
                                                        phi_896_ = (_e660 + 1i);
                                                    } else {
                                                        phi_896_ = _e660;
                                                    }
                                                    let _e702 = phi_896_;
                                                    phi_841_ = _e702;
                                                    phi_897_ = true;
                                                    break;
                                                }
                                                default: {
                                                    phi_841_ = i32();
                                                    phi_897_ = bool();
                                                    break;
                                                }
                                            }
                                            let _e704 = phi_841_;
                                            let _e706 = phi_897_;
                                            continue;
                                            continuing {
                                                phi_837_ = _e673;
                                                phi_840_ = _e704;
                                                phi_1433_ = _e636;
                                                break if !(_e706);
                                            }
                                        }
                                        let _e709 = phi_1433_;
                                        phi_1445_ = _e709;
                                        phi_1157_ = bool();
                                        phi_1156_ = bool();
                                        phi_1155_ = bool();
                                        if _e709 {
                                            break;
                                        }
                                        let _e711 = local_8;
                                        let _e712 = (_e711 == 8i);
                                        if _e712 {
                                            phi_1447_ = _e709;
                                            phi_970_ = false;
                                            phi_971_ = false;
                                        } else {
                                            phi_904_ = type_18(0u, 8u);
                                            phi_907_ = 0i;
                                            loop {
                                                let _e714 = phi_904_;
                                                let _e716 = phi_907_;
                                                local_9 = _e716;
                                                if (_e714.member < _e714.member_1) {
                                                    phi_905_ = type_18((_e714.member + 1u), _e714.member_1);
                                                    phi_922_ = type_18(1u, _e714.member);
                                                } else {
                                                    phi_905_ = _e714;
                                                    phi_922_ = type_18(0u, type_18().member_1);
                                                }
                                                let _e729 = phi_905_;
                                                let _e731 = phi_922_;
                                                switch bitcast<i32>(_e731.member) {
                                                    case 0: {
                                                        phi_908_ = i32();
                                                        phi_964_ = false;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if (_e731.member_1 < 8u) {
                                                        } else {
                                                            phi_1440_ = true;
                                                            break;
                                                        }
                                                        let _e738 = local_1[_e731.member_1][0u];
                                                        let _e741 = local_1[_e731.member_1][1u];
                                                        let _e744 = local_1[_e731.member_1][2u];
                                                        local_4 = array<f32, 3>(_e738, _e741, _e744);
                                                        let _e746 = (_e653.member_1 < 3u);
                                                        if _e746 {
                                                        } else {
                                                            phi_1440_ = true;
                                                            break;
                                                        }
                                                        let _e748 = local_4[_e653.member_1];
                                                        local_5 = array<f32, 3>(_e634.x, _e634.y, _e634.z);
                                                        if _e746 {
                                                        } else {
                                                            phi_1440_ = true;
                                                            break;
                                                        }
                                                        let _e754 = local_5[_e653.member_1];
                                                        if (_e748 > _e754) {
                                                            phi_963_ = (_e716 + 1i);
                                                        } else {
                                                            phi_963_ = _e716;
                                                        }
                                                        let _e758 = phi_963_;
                                                        phi_908_ = _e758;
                                                        phi_964_ = true;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_908_ = i32();
                                                        phi_964_ = bool();
                                                        break;
                                                    }
                                                }
                                                let _e760 = phi_908_;
                                                let _e762 = phi_964_;
                                                continue;
                                                continuing {
                                                    phi_904_ = _e729;
                                                    phi_907_ = _e760;
                                                    phi_1440_ = _e709;
                                                    break if !(_e762);
                                                }
                                            }
                                            let _e765 = phi_1440_;
                                            phi_1445_ = _e765;
                                            phi_1157_ = bool();
                                            phi_1156_ = bool();
                                            phi_1155_ = bool();
                                            if _e765 {
                                                break;
                                            }
                                            let _e767 = local_9;
                                            let _e768 = (_e767 == 8i);
                                            phi_1447_ = _e765;
                                            phi_970_ = select(true, false, _e768);
                                            phi_971_ = _e768;
                                        }
                                        let _e771 = phi_1447_;
                                        let _e773 = phi_970_;
                                        let _e775 = phi_971_;
                                        phi_1446_ = _e771;
                                        phi_972_ = _e773;
                                        phi_973_ = false;
                                        phi_974_ = _e712;
                                        phi_975_ = _e775;
                                        break;
                                    }
                                    default: {
                                        phi_1446_ = _e636;
                                        phi_972_ = bool();
                                        phi_973_ = bool();
                                        phi_974_ = bool();
                                        phi_975_ = bool();
                                        break;
                                    }
                                }
                                let _e777 = phi_1446_;
                                let _e779 = phi_972_;
                                let _e781 = phi_973_;
                                let _e783 = phi_974_;
                                let _e785 = phi_975_;
                                continue;
                                continuing {
                                    phi_1438_ = _e777;
                                    phi_811_ = _e651;
                                    phi_1445_ = _e777;
                                    phi_1157_ = _e785;
                                    phi_1156_ = _e783;
                                    phi_1155_ = _e781;
                                    break if !(_e779);
                                }
                            }
                            let _e788 = phi_1445_;
                            let _e790 = phi_1157_;
                            let _e792 = phi_1156_;
                            let _e794 = phi_1155_;
                            if _e788 {
                                break;
                            }
                            let _e795 = select(_e792, false, _e794);
                            if select(true, false, select(_e795, true, select(select(_e790, false, _e794), false, _e795))) {
                                phi_998_ = type_30(false, 0u);
                            } else {
                                phi_998_ = type_30(true, 0u);
                            }
                            let _e801 = phi_998_;
                            phi_999_ = _e801;
                        } else {
                            phi_999_ = type_30();
                        }
                        let _e803 = phi_999_;
                        if select(true, false, _e631) {
                            let _e806 = local_10;
                            phi_1005_ = type_30(true, _e806);
                        } else {
                            phi_1005_ = _e803;
                        }
                        let _e809 = phi_1005_;
                        phi_1006_ = _e809;
                    }
                    let _e811 = phi_1006_;
                    if (_e811.member != true) {
                    } else {
                        global_1.member[_e85.x].member_1 = 0u;
                    }
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(32, 1, 1) 
fn cullcompute_frustum_culling(@builtin(global_invocation_id) param: vec3<u32>) {
    global_2 = param;
    function();
}
