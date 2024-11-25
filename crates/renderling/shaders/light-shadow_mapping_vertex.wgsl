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
    member: type_13,
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

struct type_26 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_27 {
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
    var phi_431_: u32;
    var phi_2144_: bool;
    var phi_438_: u32;
    var phi_439_: u32;
    var phi_449_: u32;
    var phi_531_: type_14;
    var phi_532_: type_14;
    var phi_555_: type_14;
    var phi_568_: bool;
    var phi_574_: type_14;
    var phi_575_: type_14;
    var phi_598_: type_14;
    var phi_612_: bool;
    var phi_618_: type_14;
    var phi_621_: type_22;
    var phi_619_: type_14;
    var phi_644_: type_14;
    var phi_661_: u32;
    var phi_2174_: bool;
    var phi_679_: type_14;
    var phi_2200_: u32;
    var phi_2219_: bool;
    var phi_729_: type_26;
    var phi_739_: u32;
    var phi_2241_: bool;
    var phi_747_: f32;
    var phi_622_: type_22;
    var phi_800_: bool;
    var phi_2264_: bool;
    var phi_921_: type_27;
    var local_4: type_22;
    var phi_924_: type_14;
    var phi_927_: type_13;
    var phi_925_: type_14;
    var phi_950_: type_14;
    var local_5: type_22;
    var phi_974_: u32;
    var phi_2298_: bool;
    var phi_983_: u32;
    var phi_2322_: bool;
    var phi_1032_: type_17;
    var phi_1042_: u32;
    var phi_2347_: bool;
    var phi_1115_: type_13;
    var phi_928_: type_13;
    var phi_1352_: bool;
    var phi_2772_: bool;
    var local_6: type_13;
    var local_7: type_13;
    var local_8: type_13;
    var local_9: type_13;
    var phi_1379_: bool;
    var phi_1381_: bool;
    var phi_1382_: bool;
    var phi_1383_: bool;
    var phi_1384_: bool;
    var local_10: type_13;
    var local_11: type_13;
    var local_12: type_13;
    var local_13: type_13;
    var phi_1418_: bool;
    var phi_1420_: bool;
    var phi_1421_: bool;
    var phi_1422_: bool;
    var phi_1423_: bool;
    var local_14: type_13;
    var local_15: type_13;
    var local_16: type_13;
    var local_17: type_13;
    var phi_1457_: bool;
    var phi_1459_: bool;
    var phi_1460_: bool;
    var phi_1461_: bool;
    var phi_1462_: bool;
    var local_18: type_13;
    var local_19: type_13;
    var local_20: type_13;
    var local_21: type_13;
    var phi_1496_: bool;
    var phi_1498_: bool;
    var phi_1499_: bool;
    var phi_1500_: bool;
    var phi_1501_: bool;
    var phi_1506_: bool;
    var phi_1508_: bool;
    var phi_1509_: bool;
    var phi_1510_: bool;
    var phi_1511_: bool;
    var phi_1519_: type_13;
    var phi_2482_: bool;
    var phi_2547_: vec4<f32>;
    var phi_2577_: vec4<f32>;
    var phi_2579_: vec4<f32>;
    var phi_2588_: type_17;
    var phi_2589_: type_17;
    var phi_2594_: type_17;
    var phi_2595_: type_17;
    var phi_2596_: bool;
    var phi_2600_: type_17;
    var phi_1521_: type_17;
    var phi_1523_: type_17;
    var phi_1524_: bool;
    var phi_2694_: bool;
    var phi_1577_: type_17;
    var phi_1578_: type_17;
    var local_22: type_22;
    var local_23: type_13;

    switch bitcast<i32>(0u) {
        default: {
            let _e63 = global_3;
            let _e64 = global;
            let _e66 = arrayLength((&global_2.member));
            let _e70 = global_2.member[_e63];
            let _e75 = global_2.member[(_e63 + 1u)];
            let _e79 = global_2.member[(_e63 + 2u)];
            let _e83 = global_2.member[(_e63 + 7u)];
            let _e87 = global_2.member[(_e63 + 8u)];
            let _e91 = global_2.member[(_e63 + 10u)];
            let _e95 = global_2.member[(_e63 + 12u)];
            let _e99 = global_2.member[(_e63 + 13u)];
            let _e103 = global_2.member[(_e63 + 14u)];
            let _e107 = global_2.member[(_e63 + 15u)];
            let _e111 = global_2.member[(_e63 + 16u)];
            let _e115 = global_2.member[(_e63 + 17u)];
            if (_e70 == 1u) {
                if (_e83 == 4294967295u) {
                    phi_439_ = _e64;
                } else {
                    if (_e64 >= _e87) {
                        phi_431_ = 4294967295u;
                    } else {
                        phi_431_ = (_e83 + _e64);
                    }
                    let _e120 = phi_431_;
                    if (_e66 >= 1u) {
                        phi_2144_ = (_e120 <= (_e66 - 1u));
                    } else {
                        phi_2144_ = false;
                    }
                    let _e125 = phi_2144_;
                    if _e125 {
                        let _e128 = global_2.member[_e120];
                        phi_438_ = _e128;
                    } else {
                        phi_438_ = 0u;
                    }
                    let _e130 = phi_438_;
                    phi_439_ = _e130;
                }
                let _e132 = phi_439_;
                if (_e132 >= _e79) {
                    phi_449_ = 4294967295u;
                } else {
                    phi_449_ = (_e75 + (26u * _e132));
                }
                let _e137 = phi_449_;
                let _e140 = global_2.member[_e137];
                let _e145 = global_2.member[(_e137 + 1u)];
                let _e150 = global_2.member[(_e137 + 2u)];
                let _e156 = global_2.member[(_e137 + 3u)];
                let _e161 = global_2.member[(_e137 + 4u)];
                let _e166 = global_2.member[(_e137 + 5u)];
                let _e171 = global_2.member[(_e137 + 6u)];
                let _e177 = global_2.member[(_e137 + 7u)];
                let _e182 = global_2.member[(_e137 + 8u)];
                let _e188 = global_2.member[(_e137 + 9u)];
                let _e193 = global_2.member[(_e137 + 10u)];
                let _e199 = global_2.member[(_e137 + 11u)];
                let _e204 = global_2.member[(_e137 + 12u)];
                let _e209 = global_2.member[(_e137 + 13u)];
                let _e215 = global_2.member[(_e137 + 14u)];
                let _e220 = global_2.member[(_e137 + 15u)];
                let _e225 = global_2.member[(_e137 + 16u)];
                let _e230 = global_2.member[(_e137 + 17u)];
                local_3 = array<u32, 4>(0u, 0u, 0u, 0u);
                phi_531_ = type_14(0u, 4u);
                loop {
                    let _e235 = phi_531_;
                    if (_e235.member < _e235.member_1) {
                        phi_532_ = type_14((_e235.member + 1u), _e235.member_1);
                        phi_555_ = type_14(1u, _e235.member);
                    } else {
                        phi_532_ = _e235;
                        phi_555_ = type_14(0u, type_14().member_1);
                    }
                    let _e248 = phi_532_;
                    let _e250 = phi_555_;
                    switch bitcast<i32>(_e250.member) {
                        case 0: {
                            phi_568_ = false;
                            break;
                        }
                        case 1: {
                            let _e257 = global_2.member[((_e137 + 18u) + _e250.member_1)];
                            local_3[_e250.member_1] = _e257;
                            phi_568_ = true;
                            break;
                        }
                        default: {
                            phi_568_ = bool();
                            break;
                        }
                    }
                    let _e260 = phi_568_;
                    continue;
                    continuing {
                        phi_531_ = _e248;
                        break if !(_e260);
                    }
                }
                let _e262 = local_3;
                local_2 = array<f32, 4>(0f, 0f, 0f, 0f);
                phi_574_ = type_14(0u, 4u);
                loop {
                    let _e265 = phi_574_;
                    if (_e265.member < _e265.member_1) {
                        phi_575_ = type_14((_e265.member + 1u), _e265.member_1);
                        phi_598_ = type_14(1u, _e265.member);
                    } else {
                        phi_575_ = _e265;
                        phi_598_ = type_14(0u, type_14().member_1);
                    }
                    let _e278 = phi_575_;
                    let _e280 = phi_598_;
                    switch bitcast<i32>(_e280.member) {
                        case 0: {
                            phi_612_ = false;
                            break;
                        }
                        case 1: {
                            let _e287 = global_2.member[((_e137 + 22u) + _e280.member_1)];
                            local_2[_e280.member_1] = bitcast<f32>(_e287);
                            phi_612_ = true;
                            break;
                        }
                        default: {
                            phi_612_ = bool();
                            break;
                        }
                    }
                    let _e291 = phi_612_;
                    continue;
                    continuing {
                        phi_574_ = _e278;
                        break if !(_e291);
                    }
                }
                let _e293 = local_2;
                phi_618_ = type_14(0u, _e103);
                phi_621_ = type_22(vec3<f32>(bitcast<f32>(_e140), bitcast<f32>(_e145), bitcast<f32>(_e150)), vec4<f32>(bitcast<f32>(_e156), bitcast<f32>(_e161), bitcast<f32>(_e166), bitcast<f32>(_e171)), vec3<f32>(bitcast<f32>(_e199), bitcast<f32>(_e204), bitcast<f32>(_e209)), vec4<f32>(bitcast<f32>(_e215), bitcast<f32>(_e220), bitcast<f32>(_e225), bitcast<f32>(_e230)), _e262, _e293, vec2<f32>(bitcast<f32>(_e177), bitcast<f32>(_e182)), vec2<f32>(bitcast<f32>(_e188), bitcast<f32>(_e193)));
                loop {
                    let _e297 = phi_618_;
                    let _e299 = phi_621_;
                    local_4 = _e299;
                    local_5 = _e299;
                    local_22 = _e299;
                    if (_e297.member < _e297.member_1) {
                        phi_619_ = type_14((_e297.member + 1u), _e297.member_1);
                        phi_644_ = type_14(1u, _e297.member);
                    } else {
                        phi_619_ = _e297;
                        phi_644_ = type_14(0u, type_14().member_1);
                    }
                    let _e312 = phi_619_;
                    let _e314 = phi_644_;
                    switch bitcast<i32>(_e314.member) {
                        case 0: {
                            phi_622_ = type_22();
                            phi_800_ = false;
                            break;
                        }
                        case 1: {
                            if (_e314.member_1 >= _e103) {
                                phi_661_ = 4294967295u;
                            } else {
                                phi_661_ = (_e99 + (2u * _e314.member_1));
                            }
                            let _e322 = phi_661_;
                            if (_e66 >= 2u) {
                                phi_2174_ = (_e322 <= (_e66 - 2u));
                            } else {
                                phi_2174_ = false;
                            }
                            let _e327 = phi_2174_;
                            if _e327 {
                                let _e330 = global_2.member[_e322];
                                let _e334 = global_2.member[(_e322 + 1u)];
                                phi_679_ = type_14(_e330, _e334);
                            } else {
                                phi_679_ = type_14(4294967295u, 0u);
                            }
                            let _e337 = phi_679_;
                            if (_e132 >= _e337.member_1) {
                                phi_2200_ = 4294967295u;
                            } else {
                                phi_2200_ = (_e337.member + (9u * _e132));
                            }
                            let _e344 = phi_2200_;
                            if (_e66 >= 9u) {
                                phi_2219_ = (_e344 <= (_e66 - 9u));
                            } else {
                                phi_2219_ = false;
                            }
                            let _e349 = phi_2219_;
                            if _e349 {
                                let _e352 = global_2.member[_e344];
                                let _e357 = global_2.member[(_e344 + 1u)];
                                let _e362 = global_2.member[(_e344 + 2u)];
                                let _e368 = global_2.member[(_e344 + 3u)];
                                let _e373 = global_2.member[(_e344 + 4u)];
                                let _e378 = global_2.member[(_e344 + 5u)];
                                let _e384 = global_2.member[(_e344 + 6u)];
                                let _e389 = global_2.member[(_e344 + 7u)];
                                let _e394 = global_2.member[(_e344 + 8u)];
                                phi_729_ = type_26(vec3<f32>(bitcast<f32>(_e352), bitcast<f32>(_e357), bitcast<f32>(_e362)), vec3<f32>(bitcast<f32>(_e368), bitcast<f32>(_e373), bitcast<f32>(_e378)), vec3<f32>(bitcast<f32>(_e384), bitcast<f32>(_e389), bitcast<f32>(_e394)));
                            } else {
                                phi_729_ = type_26(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e399 = phi_729_;
                            if (_e314.member_1 >= _e111) {
                                phi_739_ = 4294967295u;
                            } else {
                                phi_739_ = (_e107 + _e314.member_1);
                            }
                            let _e403 = phi_739_;
                            if (_e66 >= 1u) {
                                phi_2241_ = (_e403 <= (_e66 - 1u));
                            } else {
                                phi_2241_ = false;
                            }
                            let _e408 = phi_2241_;
                            if _e408 {
                                let _e411 = global_2.member[_e403];
                                phi_747_ = bitcast<f32>(_e411);
                            } else {
                                phi_747_ = 0f;
                            }
                            let _e414 = phi_747_;
                            let _e437 = type_22(vec3<f32>(fma(_e414, _e399.member.x, _e299.member.x), fma(_e414, _e399.member.y, _e299.member.y), fma(_e414, _e399.member.z, _e299.member.z)), _e299.member_1, _e299.member_2, _e299.member_3, _e299.member_4, _e299.member_5, _e299.member_6, _e299.member_7);
                            let _e460 = type_22(_e437.member, _e437.member_1, vec3<f32>(fma(_e414, _e399.member_1.x, _e299.member_2.x), fma(_e414, _e399.member_1.y, _e299.member_2.y), fma(_e414, _e399.member_1.z, _e299.member_2.z)), _e437.member_3, _e437.member_4, _e437.member_5, _e437.member_6, _e437.member_7);
                            phi_622_ = type_22(_e460.member, _e460.member_1, _e460.member_2, vec4<f32>(fma(_e414, _e399.member_2.x, _e299.member_3.x), fma(_e414, _e399.member_2.y, _e299.member_3.y), fma(_e414, _e399.member_2.z, _e299.member_3.z), _e299.member_3.w), _e460.member_4, _e460.member_5, _e460.member_6, _e460.member_7);
                            phi_800_ = true;
                            break;
                        }
                        default: {
                            phi_622_ = type_22();
                            phi_800_ = bool();
                            break;
                        }
                    }
                    let _e487 = phi_622_;
                    let _e489 = phi_800_;
                    continue;
                    continuing {
                        phi_618_ = _e312;
                        phi_621_ = _e487;
                        break if !(_e489);
                    }
                }
                let _e494 = global_2.member[(_e115 + 6u)];
                if (_e494 == 1u) {
                    let _e497 = ((_e95 == 4294967295u) != true);
                    if _e497 {
                        if (_e66 >= 4u) {
                            phi_2264_ = (_e95 <= (_e66 - 4u));
                        } else {
                            phi_2264_ = false;
                        }
                        let _e502 = phi_2264_;
                        if _e502 {
                            let _e505 = global_2.member[_e95];
                            let _e509 = global_2.member[(_e95 + 1u)];
                            let _e513 = global_2.member[(_e95 + 2u)];
                            let _e517 = global_2.member[(_e95 + 3u)];
                            phi_921_ = type_27(type_14(_e505, _e509), type_14(_e513, _e517));
                        } else {
                            phi_921_ = type_27(type_14(4294967295u, 0u), type_14(4294967295u, 0u));
                        }
                        let _e522 = phi_921_;
                        let _e524 = local_4;
                        local = _e524.member_5;
                        phi_924_ = type_14(0u, 4u);
                        phi_927_ = type_13(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        loop {
                            let _e527 = phi_924_;
                            let _e529 = phi_927_;
                            local_6 = _e529;
                            local_7 = _e529;
                            local_8 = _e529;
                            local_9 = _e529;
                            local_10 = _e529;
                            local_11 = _e529;
                            local_12 = _e529;
                            local_13 = _e529;
                            local_14 = _e529;
                            local_15 = _e529;
                            local_16 = _e529;
                            local_17 = _e529;
                            local_18 = _e529;
                            local_19 = _e529;
                            local_20 = _e529;
                            local_21 = _e529;
                            local_23 = _e529;
                            if (_e527.member < _e527.member_1) {
                                phi_925_ = type_14((_e527.member + 1u), _e527.member_1);
                                phi_950_ = type_14(1u, _e527.member);
                            } else {
                                phi_925_ = _e527;
                                phi_950_ = type_14(0u, type_14().member_1);
                            }
                            let _e542 = phi_925_;
                            let _e544 = phi_950_;
                            switch bitcast<i32>(_e544.member) {
                                case 0: {
                                    phi_928_ = type_13();
                                    phi_1352_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e549 = local_5;
                                    local_1 = _e549.member_4;
                                    let _e551 = (_e544.member_1 < 4u);
                                    if _e551 {
                                    } else {
                                        phi_2772_ = true;
                                        break;
                                    }
                                    let _e553 = local_1[_e544.member_1];
                                    if (_e553 >= _e522.member.member_1) {
                                        phi_974_ = 4294967295u;
                                    } else {
                                        phi_974_ = (_e522.member.member + _e553);
                                    }
                                    let _e561 = phi_974_;
                                    if (_e66 >= 1u) {
                                        phi_2298_ = (_e561 <= (_e66 - 1u));
                                    } else {
                                        phi_2298_ = false;
                                    }
                                    let _e566 = phi_2298_;
                                    if _e566 {
                                        let _e569 = global_2.member[_e561];
                                        phi_983_ = _e569;
                                    } else {
                                        phi_983_ = 4294967295u;
                                    }
                                    let _e571 = phi_983_;
                                    if (_e66 >= 10u) {
                                        phi_2322_ = (_e571 <= (_e66 - 10u));
                                    } else {
                                        phi_2322_ = false;
                                    }
                                    let _e576 = phi_2322_;
                                    if _e576 {
                                        let _e579 = global_2.member[_e571];
                                        let _e584 = global_2.member[(_e571 + 1u)];
                                        let _e589 = global_2.member[(_e571 + 2u)];
                                        let _e595 = global_2.member[(_e571 + 3u)];
                                        let _e600 = global_2.member[(_e571 + 4u)];
                                        let _e605 = global_2.member[(_e571 + 5u)];
                                        let _e610 = global_2.member[(_e571 + 6u)];
                                        let _e616 = global_2.member[(_e571 + 7u)];
                                        let _e621 = global_2.member[(_e571 + 8u)];
                                        let _e626 = global_2.member[(_e571 + 9u)];
                                        phi_1032_ = type_17(vec3<f32>(bitcast<f32>(_e579), bitcast<f32>(_e584), bitcast<f32>(_e589)), vec4<f32>(bitcast<f32>(_e595), bitcast<f32>(_e600), bitcast<f32>(_e605), bitcast<f32>(_e610)), vec3<f32>(bitcast<f32>(_e616), bitcast<f32>(_e621), bitcast<f32>(_e626)));
                                    } else {
                                        phi_1032_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e631 = phi_1032_;
                                    if (_e553 >= _e522.member_1.member_1) {
                                        phi_1042_ = 4294967295u;
                                    } else {
                                        phi_1042_ = (_e522.member_1.member + (16u * _e553));
                                    }
                                    let _e640 = phi_1042_;
                                    if (_e66 >= 16u) {
                                        phi_2347_ = (_e640 <= (_e66 - 16u));
                                    } else {
                                        phi_2347_ = false;
                                    }
                                    let _e645 = phi_2347_;
                                    if _e645 {
                                        let _e648 = global_2.member[_e640];
                                        let _e653 = global_2.member[(_e640 + 1u)];
                                        let _e658 = global_2.member[(_e640 + 2u)];
                                        let _e663 = global_2.member[(_e640 + 3u)];
                                        let _e669 = global_2.member[(_e640 + 4u)];
                                        let _e674 = global_2.member[(_e640 + 5u)];
                                        let _e679 = global_2.member[(_e640 + 6u)];
                                        let _e684 = global_2.member[(_e640 + 7u)];
                                        let _e690 = global_2.member[(_e640 + 8u)];
                                        let _e695 = global_2.member[(_e640 + 9u)];
                                        let _e700 = global_2.member[(_e640 + 10u)];
                                        let _e705 = global_2.member[(_e640 + 11u)];
                                        let _e711 = global_2.member[(_e640 + 12u)];
                                        let _e716 = global_2.member[(_e640 + 13u)];
                                        let _e721 = global_2.member[(_e640 + 14u)];
                                        let _e726 = global_2.member[(_e640 + 15u)];
                                        phi_1115_ = type_13(vec4<f32>(bitcast<f32>(_e648), bitcast<f32>(_e653), bitcast<f32>(_e658), bitcast<f32>(_e663)), vec4<f32>(bitcast<f32>(_e669), bitcast<f32>(_e674), bitcast<f32>(_e679), bitcast<f32>(_e684)), vec4<f32>(bitcast<f32>(_e690), bitcast<f32>(_e695), bitcast<f32>(_e700), bitcast<f32>(_e705)), vec4<f32>(bitcast<f32>(_e711), bitcast<f32>(_e716), bitcast<f32>(_e721), bitcast<f32>(_e726)));
                                    } else {
                                        phi_1115_ = type_13(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                    }
                                    let _e731 = phi_1115_;
                                    let _e739 = (_e631.member_1.x + _e631.member_1.x);
                                    let _e740 = (_e631.member_1.y + _e631.member_1.y);
                                    let _e741 = (_e631.member_1.z + _e631.member_1.z);
                                    let _e743 = (_e631.member_1.z * _e741);
                                    let _e744 = (_e631.member_1.w * _e739);
                                    let _e745 = (_e631.member_1.w * _e740);
                                    let _e746 = (_e631.member_1.w * _e741);
                                    let _e766 = (vec4<f32>((1f - fma(_e631.member_1.y, _e740, _e743)), fma(_e631.member_1.x, _e740, _e746), fma(_e631.member_1.x, _e741, -(_e745)), 0f) * _e631.member_2.x);
                                    let _e768 = (vec4<f32>(fma(_e631.member_1.x, _e740, -(_e746)), (1f - fma(_e631.member_1.x, _e739, _e743)), fma(_e631.member_1.y, _e741, _e744), 0f) * _e631.member_2.y);
                                    let _e770 = (vec4<f32>(fma(_e631.member_1.x, _e741, _e745), fma(_e631.member_1.y, _e741, -(_e744)), (1f - fma(_e631.member_1.x, _e739, (_e631.member_1.y * _e740))), 0f) * _e631.member_2.z);
                                    if _e551 {
                                    } else {
                                        phi_2772_ = true;
                                        break;
                                    }
                                    let _e875 = local[_e544.member_1];
                                    phi_928_ = type_13((_e529.member + (vec4<f32>(fma(_e631.member.x, _e731.member.w, fma(_e770.x, _e731.member.z, fma(_e766.x, _e731.member.x, (_e768.x * _e731.member.y)))), fma(_e631.member.y, _e731.member.w, fma(_e770.y, _e731.member.z, fma(_e766.y, _e731.member.x, (_e768.y * _e731.member.y)))), fma(_e631.member.z, _e731.member.w, fma(_e770.z, _e731.member.z, fma(_e766.z, _e731.member.x, (_e768.z * _e731.member.y)))), (fma(_e770.w, _e731.member.z, fma(_e766.w, _e731.member.x, (_e768.w * _e731.member.y))) + _e731.member.w)) * _e875)), (_e529.member_1 + (vec4<f32>(fma(_e631.member.x, _e731.member_1.w, fma(_e770.x, _e731.member_1.z, fma(_e766.x, _e731.member_1.x, (_e768.x * _e731.member_1.y)))), fma(_e631.member.y, _e731.member_1.w, fma(_e770.y, _e731.member_1.z, fma(_e766.y, _e731.member_1.x, (_e768.y * _e731.member_1.y)))), fma(_e631.member.z, _e731.member_1.w, fma(_e770.z, _e731.member_1.z, fma(_e766.z, _e731.member_1.x, (_e768.z * _e731.member_1.y)))), (fma(_e770.w, _e731.member_1.z, fma(_e766.w, _e731.member_1.x, (_e768.w * _e731.member_1.y))) + _e731.member_1.w)) * _e875)), (_e529.member_2 + (vec4<f32>(fma(_e631.member.x, _e731.member_2.w, fma(_e770.x, _e731.member_2.z, fma(_e766.x, _e731.member_2.x, (_e768.x * _e731.member_2.y)))), fma(_e631.member.y, _e731.member_2.w, fma(_e770.y, _e731.member_2.z, fma(_e766.y, _e731.member_2.x, (_e768.y * _e731.member_2.y)))), fma(_e631.member.z, _e731.member_2.w, fma(_e770.z, _e731.member_2.z, fma(_e766.z, _e731.member_2.x, (_e768.z * _e731.member_2.y)))), (fma(_e770.w, _e731.member_2.z, fma(_e766.w, _e731.member_2.x, (_e768.w * _e731.member_2.y))) + _e731.member_2.w)) * _e875)), (_e529.member_3 + (vec4<f32>(fma(_e631.member.x, _e731.member_3.w, fma(_e770.x, _e731.member_3.z, fma(_e766.x, _e731.member_3.x, (_e768.x * _e731.member_3.y)))), fma(_e631.member.y, _e731.member_3.w, fma(_e770.y, _e731.member_3.z, fma(_e766.y, _e731.member_3.x, (_e768.y * _e731.member_3.y)))), fma(_e631.member.z, _e731.member_3.w, fma(_e770.z, _e731.member_3.z, fma(_e766.z, _e731.member_3.x, (_e768.z * _e731.member_3.y)))), (fma(_e770.w, _e731.member_3.z, fma(_e766.w, _e731.member_3.x, (_e768.w * _e731.member_3.y))) + _e731.member_3.w)) * _e875)));
                                    phi_1352_ = true;
                                    break;
                                }
                                default: {
                                    phi_928_ = type_13();
                                    phi_1352_ = bool();
                                    break;
                                }
                            }
                            let _e890 = phi_928_;
                            let _e892 = phi_1352_;
                            continue;
                            continuing {
                                phi_924_ = _e542;
                                phi_927_ = _e890;
                                phi_2772_ = false;
                                break if !(_e892);
                            }
                        }
                        let _e895 = phi_2772_;
                        if _e895 {
                            break;
                        }
                        let _e897 = local_6;
                        let _e902 = global_5.member[0u];
                        if (_e897.member.x == _e902) {
                            let _e905 = local_7;
                            let _e910 = global_5.member[1u];
                            if (_e905.member.y == _e910) {
                                let _e913 = local_8;
                                let _e918 = global_5.member[2u];
                                let _e919 = (_e913.member.z == _e918);
                                if _e919 {
                                    let _e921 = local_9;
                                    let _e926 = global_5.member[3u];
                                    phi_1379_ = (_e921.member.w == _e926);
                                } else {
                                    phi_1379_ = bool();
                                }
                                let _e929 = phi_1379_;
                                phi_1381_ = _e929;
                                phi_1382_ = select(true, false, _e919);
                            } else {
                                phi_1381_ = bool();
                                phi_1382_ = true;
                            }
                            let _e932 = phi_1381_;
                            let _e934 = phi_1382_;
                            phi_1383_ = _e932;
                            phi_1384_ = _e934;
                        } else {
                            phi_1383_ = bool();
                            phi_1384_ = true;
                        }
                        let _e936 = phi_1383_;
                        let _e938 = phi_1384_;
                        if select(_e936, false, _e938) {
                            let _e941 = local_10;
                            let _e946 = global_5.member_1[0u];
                            if (_e941.member_1.x == _e946) {
                                let _e949 = local_11;
                                let _e954 = global_5.member_1[1u];
                                if (_e949.member_1.y == _e954) {
                                    let _e957 = local_12;
                                    let _e962 = global_5.member_1[2u];
                                    let _e963 = (_e957.member_1.z == _e962);
                                    if _e963 {
                                        let _e965 = local_13;
                                        let _e970 = global_5.member_1[3u];
                                        phi_1418_ = (_e965.member_1.w == _e970);
                                    } else {
                                        phi_1418_ = bool();
                                    }
                                    let _e973 = phi_1418_;
                                    phi_1420_ = _e973;
                                    phi_1421_ = select(true, false, _e963);
                                } else {
                                    phi_1420_ = bool();
                                    phi_1421_ = true;
                                }
                                let _e976 = phi_1420_;
                                let _e978 = phi_1421_;
                                phi_1422_ = _e976;
                                phi_1423_ = _e978;
                            } else {
                                phi_1422_ = bool();
                                phi_1423_ = true;
                            }
                            let _e980 = phi_1422_;
                            let _e982 = phi_1423_;
                            if select(_e980, false, _e982) {
                                let _e985 = local_14;
                                let _e990 = global_5.member_2[0u];
                                if (_e985.member_2.x == _e990) {
                                    let _e993 = local_15;
                                    let _e998 = global_5.member_2[1u];
                                    if (_e993.member_2.y == _e998) {
                                        let _e1001 = local_16;
                                        let _e1006 = global_5.member_2[2u];
                                        let _e1007 = (_e1001.member_2.z == _e1006);
                                        if _e1007 {
                                            let _e1009 = local_17;
                                            let _e1014 = global_5.member_2[3u];
                                            phi_1457_ = (_e1009.member_2.w == _e1014);
                                        } else {
                                            phi_1457_ = bool();
                                        }
                                        let _e1017 = phi_1457_;
                                        phi_1459_ = _e1017;
                                        phi_1460_ = select(true, false, _e1007);
                                    } else {
                                        phi_1459_ = bool();
                                        phi_1460_ = true;
                                    }
                                    let _e1020 = phi_1459_;
                                    let _e1022 = phi_1460_;
                                    phi_1461_ = _e1020;
                                    phi_1462_ = _e1022;
                                } else {
                                    phi_1461_ = bool();
                                    phi_1462_ = true;
                                }
                                let _e1024 = phi_1461_;
                                let _e1026 = phi_1462_;
                                let _e1027 = select(_e1024, false, _e1026);
                                if _e1027 {
                                    let _e1029 = local_18;
                                    let _e1034 = global_5.member_3[0u];
                                    if (_e1029.member_3.x == _e1034) {
                                        let _e1037 = local_19;
                                        let _e1042 = global_5.member_3[1u];
                                        if (_e1037.member_3.y == _e1042) {
                                            let _e1045 = local_20;
                                            let _e1050 = global_5.member_3[2u];
                                            let _e1051 = (_e1045.member_3.z == _e1050);
                                            if _e1051 {
                                                let _e1053 = local_21;
                                                let _e1058 = global_5.member_3[3u];
                                                phi_1496_ = (_e1053.member_3.w == _e1058);
                                            } else {
                                                phi_1496_ = bool();
                                            }
                                            let _e1061 = phi_1496_;
                                            phi_1498_ = _e1061;
                                            phi_1499_ = select(true, false, _e1051);
                                        } else {
                                            phi_1498_ = bool();
                                            phi_1499_ = true;
                                        }
                                        let _e1064 = phi_1498_;
                                        let _e1066 = phi_1499_;
                                        phi_1500_ = _e1064;
                                        phi_1501_ = _e1066;
                                    } else {
                                        phi_1500_ = bool();
                                        phi_1501_ = true;
                                    }
                                    let _e1068 = phi_1500_;
                                    let _e1070 = phi_1501_;
                                    phi_1506_ = select(_e1068, false, _e1070);
                                } else {
                                    phi_1506_ = bool();
                                }
                                let _e1073 = phi_1506_;
                                phi_1508_ = _e1073;
                                phi_1509_ = select(true, false, _e1027);
                            } else {
                                phi_1508_ = bool();
                                phi_1509_ = true;
                            }
                            let _e1076 = phi_1508_;
                            let _e1078 = phi_1509_;
                            phi_1510_ = _e1076;
                            phi_1511_ = _e1078;
                        } else {
                            phi_1510_ = bool();
                            phi_1511_ = true;
                        }
                        let _e1080 = phi_1510_;
                        let _e1082 = phi_1511_;
                        if select(_e1080, false, _e1082) {
                            phi_1519_ = type_13(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        } else {
                            let _e1532 = local_23;
                            phi_1519_ = _e1532;
                        }
                        let _e1085 = phi_1519_;
                        let _e1108 = fma(_e1085.member_2.z, _e1085.member_3.w, -((_e1085.member_2.w * _e1085.member_3.z)));
                        let _e1111 = fma(_e1085.member_2.y, _e1085.member_3.w, -((_e1085.member_2.w * _e1085.member_3.y)));
                        let _e1114 = fma(_e1085.member_2.y, _e1085.member_3.z, -((_e1085.member_2.z * _e1085.member_3.y)));
                        let _e1117 = fma(_e1085.member_2.x, _e1085.member_3.w, -((_e1085.member_2.w * _e1085.member_3.x)));
                        let _e1120 = fma(_e1085.member_2.x, _e1085.member_3.z, -((_e1085.member_2.z * _e1085.member_3.x)));
                        let _e1123 = fma(_e1085.member_2.x, _e1085.member_3.y, -((_e1085.member_2.y * _e1085.member_3.x)));
                        let _e1145 = fma(-(_e1085.member.w), fma(_e1085.member_1.z, _e1123, fma(_e1085.member_1.x, _e1114, -((_e1085.member_1.y * _e1120)))), fma(_e1085.member.z, fma(_e1085.member_1.w, _e1123, fma(_e1085.member_1.x, _e1111, -((_e1085.member_1.y * _e1117)))), fma(_e1085.member.x, fma(_e1085.member_1.w, _e1114, fma(_e1085.member_1.y, _e1108, -((_e1085.member_1.z * _e1111)))), -((_e1085.member.y * fma(_e1085.member_1.w, _e1120, fma(_e1085.member_1.x, _e1108, -((_e1085.member_1.z * _e1117)))))))));
                        if (_e1145 == 0f) {
                            phi_2594_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            phi_2595_ = type_17();
                            phi_2596_ = true;
                        } else {
                            let _e1154 = (sqrt(fma(_e1085.member.w, _e1085.member.w, fma(_e1085.member.z, _e1085.member.z, fma(_e1085.member.x, _e1085.member.x, (_e1085.member.y * _e1085.member.y))))) * select(-1f, 1f, (_e1145 >= 0f)));
                            let _e1159 = sqrt(fma(_e1085.member_1.w, _e1085.member_1.w, fma(_e1085.member_1.z, _e1085.member_1.z, fma(_e1085.member_1.x, _e1085.member_1.x, (_e1085.member_1.y * _e1085.member_1.y)))));
                            let _e1164 = sqrt(fma(_e1085.member_2.w, _e1085.member_2.w, fma(_e1085.member_2.z, _e1085.member_2.z, fma(_e1085.member_2.x, _e1085.member_2.x, (_e1085.member_2.y * _e1085.member_2.y)))));
                            if (_e1154 != 0f) {
                                phi_2482_ = select(true, false, (_e1159 != 0f));
                            } else {
                                phi_2482_ = true;
                            }
                            let _e1171 = phi_2482_;
                            let _e1172 = select((_e1164 != 0f), false, _e1171);
                            if _e1172 {
                                let _e1173 = (1f / _e1154);
                                let _e1174 = (1f / _e1159);
                                let _e1175 = (1f / _e1164);
                                let _e1176 = (_e1085.member.x * _e1173);
                                let _e1177 = (_e1085.member.z * _e1173);
                                let _e1178 = (_e1085.member_1.x * _e1174);
                                let _e1179 = (_e1085.member_2.x * _e1175);
                                let _e1180 = (_e1085.member_2.y * _e1175);
                                if ((_e1085.member_2.z * _e1175) <= 0f) {
                                    let _e1215 = fma(_e1085.member_1.y, _e1174, -(_e1176));
                                    let _e1217 = fma(-(_e1085.member_2.z), _e1175, 1f);
                                    if (_e1215 <= 0f) {
                                        let _e1231 = (_e1217 - _e1215);
                                        let _e1233 = (0.5f / sqrt(_e1231));
                                        phi_2577_ = vec4<f32>((_e1231 * _e1233), (fma(_e1085.member.y, _e1173, _e1178) * _e1233), (fma(_e1085.member.z, _e1173, _e1179) * _e1233), (fma(_e1085.member_1.z, _e1174, -(_e1180)) * _e1233));
                                    } else {
                                        let _e1219 = (_e1217 + _e1215);
                                        let _e1221 = (0.5f / sqrt(_e1219));
                                        phi_2577_ = vec4<f32>((fma(_e1085.member.y, _e1173, _e1178) * _e1221), (_e1219 * _e1221), (fma(_e1085.member_1.z, _e1174, _e1180) * _e1221), (fma(_e1085.member_2.x, _e1175, -(_e1177)) * _e1221));
                                    }
                                    let _e1244 = phi_2577_;
                                    phi_2579_ = _e1244;
                                } else {
                                    let _e1183 = fma(_e1085.member_1.y, _e1174, _e1176);
                                    let _e1184 = fma(_e1085.member_2.z, _e1175, 1f);
                                    if (_e1183 <= 0f) {
                                        let _e1200 = (_e1184 - _e1183);
                                        let _e1202 = (0.5f / sqrt(_e1200));
                                        phi_2547_ = vec4<f32>((fma(_e1085.member.z, _e1173, _e1179) * _e1202), (fma(_e1085.member_1.z, _e1174, _e1180) * _e1202), (_e1200 * _e1202), (fma(_e1085.member.y, _e1173, -(_e1178)) * _e1202));
                                    } else {
                                        let _e1186 = (_e1184 + _e1183);
                                        let _e1188 = (0.5f / sqrt(_e1186));
                                        phi_2547_ = vec4<f32>((fma(_e1085.member_1.z, _e1174, -(_e1180)) * _e1188), (fma(_e1085.member_2.x, _e1175, -(_e1177)) * _e1188), (fma(_e1085.member.y, _e1173, -(_e1178)) * _e1188), (_e1186 * _e1188));
                                    }
                                    let _e1213 = phi_2547_;
                                    phi_2579_ = _e1213;
                                }
                                let _e1246 = phi_2579_;
                                phi_2588_ = type_17(vec3<f32>(_e1154, _e1159, _e1164), _e1246, vec3<f32>(_e1085.member_3.x, _e1085.member_3.y, _e1085.member_3.z));
                                phi_2589_ = type_17();
                            } else {
                                phi_2588_ = type_17();
                                phi_2589_ = type_17(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e1250 = phi_2588_;
                            let _e1252 = phi_2589_;
                            phi_2594_ = _e1252;
                            phi_2595_ = _e1250;
                            phi_2596_ = select(true, false, _e1172);
                        }
                        let _e1255 = phi_2594_;
                        let _e1257 = phi_2595_;
                        let _e1259 = phi_2596_;
                        if _e1259 {
                            phi_2600_ = _e1255;
                        } else {
                            phi_2600_ = _e1257;
                        }
                        let _e1261 = phi_2600_;
                        phi_1521_ = type_17(_e1261.member_2, _e1261.member_1, _e1261.member);
                    } else {
                        phi_1521_ = type_17();
                    }
                    let _e1267 = phi_1521_;
                    phi_1523_ = _e1267;
                    phi_1524_ = select(true, false, _e497);
                } else {
                    phi_1523_ = type_17();
                    phi_1524_ = true;
                }
                let _e1270 = phi_1523_;
                let _e1272 = phi_1524_;
                if _e1272 {
                    if (_e66 >= 10u) {
                        phi_2694_ = (_e91 <= (_e66 - 10u));
                    } else {
                        phi_2694_ = false;
                    }
                    let _e1277 = phi_2694_;
                    if _e1277 {
                        let _e1280 = global_2.member[_e91];
                        let _e1285 = global_2.member[(_e91 + 1u)];
                        let _e1290 = global_2.member[(_e91 + 2u)];
                        let _e1296 = global_2.member[(_e91 + 3u)];
                        let _e1301 = global_2.member[(_e91 + 4u)];
                        let _e1306 = global_2.member[(_e91 + 5u)];
                        let _e1311 = global_2.member[(_e91 + 6u)];
                        let _e1317 = global_2.member[(_e91 + 7u)];
                        let _e1322 = global_2.member[(_e91 + 8u)];
                        let _e1327 = global_2.member[(_e91 + 9u)];
                        phi_1577_ = type_17(vec3<f32>(bitcast<f32>(_e1280), bitcast<f32>(_e1285), bitcast<f32>(_e1290)), vec4<f32>(bitcast<f32>(_e1296), bitcast<f32>(_e1301), bitcast<f32>(_e1306), bitcast<f32>(_e1311)), vec3<f32>(bitcast<f32>(_e1317), bitcast<f32>(_e1322), bitcast<f32>(_e1327)));
                    } else {
                        phi_1577_ = type_17(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e1332 = phi_1577_;
                    phi_1578_ = _e1332;
                } else {
                    phi_1578_ = _e1270;
                }
                let _e1334 = phi_1578_;
                let _e1342 = (_e1334.member_1.x + _e1334.member_1.x);
                let _e1343 = (_e1334.member_1.y + _e1334.member_1.y);
                let _e1344 = (_e1334.member_1.z + _e1334.member_1.z);
                let _e1346 = (_e1334.member_1.z * _e1344);
                let _e1347 = (_e1334.member_1.w * _e1342);
                let _e1348 = (_e1334.member_1.w * _e1343);
                let _e1349 = (_e1334.member_1.w * _e1344);
                let _e1369 = (vec4<f32>((1f - fma(_e1334.member_1.y, _e1343, _e1346)), fma(_e1334.member_1.x, _e1343, _e1349), fma(_e1334.member_1.x, _e1344, -(_e1348)), 0f) * _e1334.member_2.x);
                let _e1371 = (vec4<f32>(fma(_e1334.member_1.x, _e1343, -(_e1349)), (1f - fma(_e1334.member_1.x, _e1342, _e1346)), fma(_e1334.member_1.y, _e1344, _e1347), 0f) * _e1334.member_2.y);
                let _e1373 = (vec4<f32>(fma(_e1334.member_1.x, _e1344, _e1348), fma(_e1334.member_1.y, _e1344, -(_e1347)), (1f - fma(_e1334.member_1.x, _e1342, (_e1334.member_1.y * _e1343))), 0f) * _e1334.member_2.z);
                let _e1378 = local_22;
                let _e1401 = (_e1334.member.x + fma(_e1373.x, _e1378.member.z, fma(_e1371.x, _e1378.member.y, (_e1369.x * _e1378.member.x))));
                let _e1402 = (_e1334.member.y + fma(_e1373.y, _e1378.member.z, fma(_e1371.y, _e1378.member.y, (_e1369.y * _e1378.member.x))));
                let _e1403 = (_e1334.member.z + fma(_e1373.z, _e1378.member.z, fma(_e1371.z, _e1378.member.y, (_e1369.z * _e1378.member.x))));
                let _e1404 = global_4.member;
                global_1 = vec4<f32>((fma(_e1404.member_2.x, _e1403, fma(_e1404.member.x, _e1401, (_e1404.member_1.x * _e1402))) + _e1404.member_3.x), (fma(_e1404.member_2.y, _e1403, fma(_e1404.member.y, _e1401, (_e1404.member_1.y * _e1402))) + _e1404.member_3.y), (fma(_e1404.member_2.z, _e1403, fma(_e1404.member.z, _e1401, (_e1404.member_1.z * _e1402))) + _e1404.member_3.z), (fma(_e1404.member_2.w, _e1403, fma(_e1404.member.w, _e1401, (_e1404.member_1.w * _e1402))) + _e1404.member_3.w));
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
