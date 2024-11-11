struct type_9 {
    member: array<u32>,
}

struct type_15 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_16 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
}

struct type_17 {
    member: type_15,
    member_1: type_15,
    member_2: type_16,
    member_3: vec3<f32>,
}

struct type_19 {
    member: u32,
    member_1: u32,
}

struct type_23 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_25 {
    member: array<type_23>,
}

struct type_31 {
    member: vec2<u32>,
    member_1: vec2<u32>,
    member_2: type_19,
    member_3: u32,
    member_4: bool,
    member_5: bool,
    member_6: bool,
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
var<storage> global: type_9;
@group(0) @binding(1) 
var<storage> global_1: type_9;
@group(0) @binding(2) 
var<storage, read_write> global_2: type_25;
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
    var phi_662_: type_31;
    var phi_2063_: bool;
    var phi_827_: type_19;
    var phi_828_: type_19;
    var phi_843_: type_19;
    var phi_870_: bool;
    var phi_876_: type_19;
    var phi_877_: type_19;
    var phi_892_: type_19;
    var phi_915_: bool;
    var phi_923_: type_17;
    var phi_2095_: bool;
    var phi_974_: type_32;
    var phi_1106_: type_19;
    var phi_1107_: type_19;
    var phi_1122_: type_19;
    var phi_1166_: bool;
    var phi_1170_: bool;
    var phi_1171_: bool;
    var phi_2659_: bool;
    var phi_1965_: bool;
    var phi_2685_: bool;
    var phi_1178_: type_19;
    var phi_1179_: type_19;
    var phi_1194_: type_19;
    var phi_1204_: type_19;
    var phi_1207_: i32;
    var phi_1205_: type_19;
    var phi_1222_: type_19;
    var phi_1263_: i32;
    var phi_1208_: i32;
    var phi_1264_: bool;
    var phi_2680_: bool;
    var local_8: i32;
    var phi_1271_: type_19;
    var phi_1274_: i32;
    var phi_1272_: type_19;
    var phi_1289_: type_19;
    var phi_1330_: i32;
    var phi_1275_: i32;
    var phi_1331_: bool;
    var phi_2687_: bool;
    var local_9: i32;
    var phi_2694_: bool;
    var phi_1337_: bool;
    var phi_1338_: bool;
    var phi_2693_: bool;
    var phi_1339_: bool;
    var phi_1340_: bool;
    var phi_1341_: bool;
    var phi_1342_: bool;
    var phi_2692_: bool;
    var phi_1968_: bool;
    var phi_1967_: bool;
    var phi_1966_: bool;
    var phi_1365_: type_33;
    var phi_1366_: type_33;
    var local_10: u32;
    var phi_1372_: type_33;
    var phi_1373_: type_33;
    var phi_1632_: u32;
    var phi_2478_: bool;
    var phi_1650_: type_19;
    var phi_2504_: u32;

    switch bitcast<i32>(0u) {
        default: {
            let _e90 = arrayLength((&global.member));
            let _e92 = arrayLength((&global_1.member));
            let _e95 = global_3;
            if (_e95.x >= arrayLength((&global_2.member))) {
            } else {
                let _e101 = global_2.member[_e95.x].member_3;
                let _e104 = global.member[_e101];
                let _e109 = global.member[(_e101 + 2u)];
                let _e113 = global.member[(_e101 + 3u)];
                let _e114 = bitcast<f32>(_e113);
                let _e118 = global.member[(_e101 + 4u)];
                let _e119 = bitcast<f32>(_e118);
                let _e123 = global.member[(_e101 + 5u)];
                let _e124 = bitcast<f32>(_e123);
                let _e128 = global.member[(_e101 + 6u)];
                let _e129 = bitcast<f32>(_e128);
                let _e133 = global.member[(_e101 + 7u)];
                let _e137 = global.member[(_e101 + 8u)];
                let _e141 = global.member[(_e101 + 9u)];
                let _e145 = global.member[(_e101 + 10u)];
                global_2.member[_e95.x].member = select(_e137, _e109, (_e133 == 4294967295u));
                global_2.member[_e95.x].member_1 = select(0u, 1u, (_e104 == 1u));
                if (_e129 == 0f) {
                } else {
                    let _e156 = (_e90 >= 10u);
                    if select(false, true, _e156) {
                        let _e160 = global.member[0u];
                        let _e163 = global.member[1u];
                        let _e167 = global.member[2u];
                        let _e170 = global.member[3u];
                        let _e174 = global.member[4u];
                        switch bitcast<i32>(_e174) {
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
                        let _e177 = phi_631_;
                        let _e180 = global.member[5u];
                        let _e184 = global.member[6u];
                        let _e188 = global.member[7u];
                        let _e192 = global.member[8u];
                        let _e195 = global.member[9u];
                        phi_662_ = type_31(vec2<u32>(_e160, _e163), vec2<u32>(_e167, _e170), type_19(_e192, _e195), _e177, (_e180 == 1u), (_e184 == 1u), (_e188 == 1u));
                    } else {
                        phi_662_ = type_31(vec2<u32>(0u, 0u), vec2<u32>(1u, 1u), type_19(4294967295u, 0u), 0u, true, true, false);
                    }
                    let _e199 = phi_662_;
                    if _e199.member_6 {
                        if (_e90 >= 83u) {
                            phi_2063_ = (_e141 <= (_e90 - 83u));
                        } else {
                            phi_2063_ = false;
                        }
                        let _e205 = phi_2063_;
                        if _e205 {
                            let _e208 = global.member[_e141];
                            let _e213 = global.member[(_e141 + 1u)];
                            let _e218 = global.member[(_e141 + 2u)];
                            let _e223 = global.member[(_e141 + 3u)];
                            let _e229 = global.member[(_e141 + 4u)];
                            let _e234 = global.member[(_e141 + 5u)];
                            let _e239 = global.member[(_e141 + 6u)];
                            let _e244 = global.member[(_e141 + 7u)];
                            let _e250 = global.member[(_e141 + 8u)];
                            let _e255 = global.member[(_e141 + 9u)];
                            let _e260 = global.member[(_e141 + 10u)];
                            let _e265 = global.member[(_e141 + 11u)];
                            let _e271 = global.member[(_e141 + 12u)];
                            let _e276 = global.member[(_e141 + 13u)];
                            let _e281 = global.member[(_e141 + 14u)];
                            let _e286 = global.member[(_e141 + 15u)];
                            let _e293 = global.member[(_e141 + 16u)];
                            let _e298 = global.member[(_e141 + 17u)];
                            let _e303 = global.member[(_e141 + 18u)];
                            let _e308 = global.member[(_e141 + 19u)];
                            let _e314 = global.member[(_e141 + 20u)];
                            let _e319 = global.member[(_e141 + 21u)];
                            let _e324 = global.member[(_e141 + 22u)];
                            let _e329 = global.member[(_e141 + 23u)];
                            let _e335 = global.member[(_e141 + 24u)];
                            let _e340 = global.member[(_e141 + 25u)];
                            let _e345 = global.member[(_e141 + 26u)];
                            let _e350 = global.member[(_e141 + 27u)];
                            let _e356 = global.member[(_e141 + 28u)];
                            let _e361 = global.member[(_e141 + 29u)];
                            let _e366 = global.member[(_e141 + 30u)];
                            let _e371 = global.member[(_e141 + 31u)];
                            let _e378 = global.member[(_e141 + 32u)];
                            let _e383 = global.member[(_e141 + 33u)];
                            let _e388 = global.member[(_e141 + 34u)];
                            local_7 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            phi_827_ = type_19(0u, 6u);
                            loop {
                                let _e393 = phi_827_;
                                if (_e393.member < _e393.member_1) {
                                    phi_828_ = type_19((_e393.member + 1u), _e393.member_1);
                                    phi_843_ = type_19(1u, _e393.member);
                                } else {
                                    phi_828_ = _e393;
                                    phi_843_ = type_19(0u, type_19().member_1);
                                }
                                let _e406 = phi_828_;
                                let _e408 = phi_843_;
                                switch bitcast<i32>(_e408.member) {
                                    case 0: {
                                        phi_870_ = false;
                                        break;
                                    }
                                    case 1: {
                                        let _e413 = ((_e141 + 35u) + (_e408.member_1 * 4u));
                                        let _e416 = global.member[_e413];
                                        let _e421 = global.member[(_e413 + 1u)];
                                        let _e426 = global.member[(_e413 + 2u)];
                                        let _e431 = global.member[(_e413 + 3u)];
                                        local_7[_e408.member_1] = vec4<f32>(bitcast<f32>(_e416), bitcast<f32>(_e421), bitcast<f32>(_e426), bitcast<f32>(_e431));
                                        phi_870_ = true;
                                        break;
                                    }
                                    default: {
                                        phi_870_ = bool();
                                        break;
                                    }
                                }
                                let _e436 = phi_870_;
                                continue;
                                continuing {
                                    phi_827_ = _e406;
                                    break if !(_e436);
                                }
                            }
                            let _e438 = local_7;
                            local_6 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            phi_876_ = type_19(0u, 8u);
                            loop {
                                let _e441 = phi_876_;
                                if (_e441.member < _e441.member_1) {
                                    phi_877_ = type_19((_e441.member + 1u), _e441.member_1);
                                    phi_892_ = type_19(1u, _e441.member);
                                } else {
                                    phi_877_ = _e441;
                                    phi_892_ = type_19(0u, type_19().member_1);
                                }
                                let _e454 = phi_877_;
                                let _e456 = phi_892_;
                                switch bitcast<i32>(_e456.member) {
                                    case 0: {
                                        phi_915_ = false;
                                        break;
                                    }
                                    case 1: {
                                        let _e461 = ((_e141 + 59u) + (_e456.member_1 * 3u));
                                        let _e464 = global.member[_e461];
                                        let _e469 = global.member[(_e461 + 1u)];
                                        let _e474 = global.member[(_e461 + 2u)];
                                        local_6[_e456.member_1] = vec3<f32>(bitcast<f32>(_e464), bitcast<f32>(_e469), bitcast<f32>(_e474));
                                        phi_915_ = true;
                                        break;
                                    }
                                    default: {
                                        phi_915_ = bool();
                                        break;
                                    }
                                }
                                let _e479 = phi_915_;
                                continue;
                                continuing {
                                    phi_876_ = _e454;
                                    break if !(_e479);
                                }
                            }
                            let _e481 = local_6;
                            phi_923_ = type_17(type_15(vec4<f32>(bitcast<f32>(_e208), bitcast<f32>(_e213), bitcast<f32>(_e218), bitcast<f32>(_e223)), vec4<f32>(bitcast<f32>(_e229), bitcast<f32>(_e234), bitcast<f32>(_e239), bitcast<f32>(_e244)), vec4<f32>(bitcast<f32>(_e250), bitcast<f32>(_e255), bitcast<f32>(_e260), bitcast<f32>(_e265)), vec4<f32>(bitcast<f32>(_e271), bitcast<f32>(_e276), bitcast<f32>(_e281), bitcast<f32>(_e286))), type_15(vec4<f32>(bitcast<f32>(_e293), bitcast<f32>(_e298), bitcast<f32>(_e303), bitcast<f32>(_e308)), vec4<f32>(bitcast<f32>(_e314), bitcast<f32>(_e319), bitcast<f32>(_e324), bitcast<f32>(_e329)), vec4<f32>(bitcast<f32>(_e335), bitcast<f32>(_e340), bitcast<f32>(_e345), bitcast<f32>(_e350)), vec4<f32>(bitcast<f32>(_e356), bitcast<f32>(_e361), bitcast<f32>(_e366), bitcast<f32>(_e371))), type_16(_e481, _e438), vec3<f32>(bitcast<f32>(_e378), bitcast<f32>(_e383), bitcast<f32>(_e388)));
                        } else {
                            phi_923_ = type_17(type_15(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_15(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_16(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
                        }
                        let _e485 = phi_923_;
                        if _e156 {
                            phi_2095_ = (_e145 <= (_e90 - 10u));
                        } else {
                            phi_2095_ = false;
                        }
                        let _e492 = phi_2095_;
                        if _e492 {
                            let _e495 = global.member[_e145];
                            let _e500 = global.member[(_e145 + 1u)];
                            let _e505 = global.member[(_e145 + 2u)];
                            let _e511 = global.member[(_e145 + 3u)];
                            let _e516 = global.member[(_e145 + 4u)];
                            let _e521 = global.member[(_e145 + 5u)];
                            let _e526 = global.member[(_e145 + 6u)];
                            let _e532 = global.member[(_e145 + 7u)];
                            let _e537 = global.member[(_e145 + 8u)];
                            let _e542 = global.member[(_e145 + 9u)];
                            phi_974_ = type_32(vec3<f32>(bitcast<f32>(_e495), bitcast<f32>(_e500), bitcast<f32>(_e505)), vec4<f32>(bitcast<f32>(_e511), bitcast<f32>(_e516), bitcast<f32>(_e521), bitcast<f32>(_e526)), vec3<f32>(bitcast<f32>(_e532), bitcast<f32>(_e537), bitcast<f32>(_e542)));
                        } else {
                            phi_974_ = type_32(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                        }
                        let _e547 = phi_974_;
                        let _e555 = (_e547.member_1.x + _e547.member_1.x);
                        let _e556 = (_e547.member_1.y + _e547.member_1.y);
                        let _e557 = (_e547.member_1.z + _e547.member_1.z);
                        let _e559 = (_e547.member_1.z * _e557);
                        let _e560 = (_e547.member_1.w * _e555);
                        let _e561 = (_e547.member_1.w * _e556);
                        let _e562 = (_e547.member_1.w * _e557);
                        let _e582 = (vec4<f32>((1f - fma(_e547.member_1.y, _e556, _e559)), fma(_e547.member_1.x, _e556, _e562), fma(_e547.member_1.x, _e557, -(_e561)), 0f) * _e547.member_2.x);
                        let _e584 = (vec4<f32>(fma(_e547.member_1.x, _e556, -(_e562)), (1f - fma(_e547.member_1.x, _e555, _e559)), fma(_e547.member_1.y, _e557, _e560), 0f) * _e547.member_2.y);
                        let _e586 = (vec4<f32>(fma(_e547.member_1.x, _e557, _e561), fma(_e547.member_1.y, _e557, -(_e560)), (1f - fma(_e547.member_1.x, _e555, (_e547.member_1.y * _e556))), 0f) * _e547.member_2.z);
                        let _e608 = (_e547.member.x + fma(_e586.x, _e124, fma(_e584.x, _e119, (_e582.x * _e114))));
                        let _e609 = (_e547.member.y + fma(_e586.y, _e124, fma(_e584.y, _e119, (_e582.y * _e114))));
                        let _e610 = (_e547.member.z + fma(_e586.z, _e124, fma(_e584.z, _e119, (_e582.z * _e114))));
                        let _e611 = vec3<f32>(_e608, _e609, _e610);
                        let _e614 = (max(_e547.member_2.x, max(_e547.member_2.y, _e547.member_2.z)) * _e129);
                        let _e616 = sqrt((_e614 * _e614));
                        local_1 = _e485.member_2.member;
                        local = _e485.member_2.member_1;
                        let _e621 = local[0u][0u];
                        let _e624 = local[0u][1u];
                        let _e629 = local[0u][2u];
                        let _e633 = local[0u][3u];
                        let _e635 = -(_e616);
                        if ((fma(_e629, _e610, fma(_e621, _e608, (_e624 * _e609))) + _e633) < _e635) {
                            phi_1373_ = type_33(true, 0u);
                        } else {
                            phi_1106_ = type_19(0u, 6u);
                            loop {
                                let _e638 = phi_1106_;
                                if (_e638.member < _e638.member_1) {
                                    phi_1107_ = type_19((_e638.member + 1u), _e638.member_1);
                                    phi_1122_ = type_19(1u, _e638.member);
                                } else {
                                    phi_1107_ = _e638;
                                    phi_1122_ = type_19(0u, type_19().member_1);
                                }
                                let _e651 = phi_1107_;
                                let _e653 = phi_1122_;
                                local_10 = _e653.member_1;
                                switch bitcast<i32>(_e653.member) {
                                    case 0: {
                                        phi_1170_ = false;
                                        phi_1171_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if (_e653.member_1 != 0u) {
                                            if (_e653.member_1 < 6u) {
                                            } else {
                                                phi_2659_ = true;
                                                phi_1965_ = bool();
                                                break;
                                            }
                                            let _e661 = local[_e653.member_1][0u];
                                            let _e664 = local[_e653.member_1][1u];
                                            let _e669 = local[_e653.member_1][2u];
                                            let _e673 = local[_e653.member_1][3u];
                                            phi_1166_ = select(true, false, ((fma(_e669, _e610, fma(_e661, _e608, (_e664 * _e609))) + _e673) < _e635));
                                        } else {
                                            phi_1166_ = true;
                                        }
                                        let _e678 = phi_1166_;
                                        phi_1170_ = _e678;
                                        phi_1171_ = false;
                                        break;
                                    }
                                    default: {
                                        phi_1170_ = bool();
                                        phi_1171_ = bool();
                                        break;
                                    }
                                }
                                let _e680 = phi_1170_;
                                let _e682 = phi_1171_;
                                continue;
                                continuing {
                                    phi_1106_ = _e651;
                                    phi_2659_ = false;
                                    phi_1965_ = _e682;
                                    break if !(_e680);
                                }
                            }
                            let _e685 = phi_2659_;
                            let _e687 = phi_1965_;
                            if _e685 {
                                break;
                            }
                            if _e687 {
                                let _e688 = vec3(_e616);
                                let _e689 = (_e611 - _e688);
                                let _e690 = (_e611 + _e688);
                                phi_2685_ = _e685;
                                phi_1178_ = type_19(0u, 3u);
                                loop {
                                    let _e692 = phi_2685_;
                                    let _e694 = phi_1178_;
                                    if (_e694.member < _e694.member_1) {
                                        phi_1179_ = type_19((_e694.member + 1u), _e694.member_1);
                                        phi_1194_ = type_19(1u, _e694.member);
                                    } else {
                                        phi_1179_ = _e694;
                                        phi_1194_ = type_19(0u, type_19().member_1);
                                    }
                                    let _e707 = phi_1179_;
                                    let _e709 = phi_1194_;
                                    switch bitcast<i32>(_e709.member) {
                                        case 0: {
                                            phi_2693_ = _e692;
                                            phi_1339_ = false;
                                            phi_1340_ = true;
                                            phi_1341_ = false;
                                            phi_1342_ = false;
                                            break;
                                        }
                                        case 1: {
                                            phi_1204_ = type_19(0u, 8u);
                                            phi_1207_ = 0i;
                                            loop {
                                                let _e714 = phi_1204_;
                                                let _e716 = phi_1207_;
                                                local_8 = _e716;
                                                if (_e714.member < _e714.member_1) {
                                                    phi_1205_ = type_19((_e714.member + 1u), _e714.member_1);
                                                    phi_1222_ = type_19(1u, _e714.member);
                                                } else {
                                                    phi_1205_ = _e714;
                                                    phi_1222_ = type_19(0u, type_19().member_1);
                                                }
                                                let _e729 = phi_1205_;
                                                let _e731 = phi_1222_;
                                                switch bitcast<i32>(_e731.member) {
                                                    case 0: {
                                                        phi_1208_ = i32();
                                                        phi_1264_ = false;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if (_e731.member_1 < 8u) {
                                                        } else {
                                                            phi_2680_ = true;
                                                            break;
                                                        }
                                                        let _e738 = local_1[_e731.member_1][0u];
                                                        let _e741 = local_1[_e731.member_1][1u];
                                                        let _e744 = local_1[_e731.member_1][2u];
                                                        local_2 = array<f32, 3>(_e738, _e741, _e744);
                                                        let _e746 = (_e709.member_1 < 3u);
                                                        if _e746 {
                                                        } else {
                                                            phi_2680_ = true;
                                                            break;
                                                        }
                                                        let _e748 = local_2[_e709.member_1];
                                                        local_3 = array<f32, 3>(_e689.x, _e689.y, _e689.z);
                                                        if _e746 {
                                                        } else {
                                                            phi_2680_ = true;
                                                            break;
                                                        }
                                                        let _e754 = local_3[_e709.member_1];
                                                        if (_e748 < _e754) {
                                                            phi_1263_ = (_e716 + 1i);
                                                        } else {
                                                            phi_1263_ = _e716;
                                                        }
                                                        let _e758 = phi_1263_;
                                                        phi_1208_ = _e758;
                                                        phi_1264_ = true;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_1208_ = i32();
                                                        phi_1264_ = bool();
                                                        break;
                                                    }
                                                }
                                                let _e760 = phi_1208_;
                                                let _e762 = phi_1264_;
                                                continue;
                                                continuing {
                                                    phi_1204_ = _e729;
                                                    phi_1207_ = _e760;
                                                    phi_2680_ = _e692;
                                                    break if !(_e762);
                                                }
                                            }
                                            let _e765 = phi_2680_;
                                            phi_2692_ = _e765;
                                            phi_1968_ = bool();
                                            phi_1967_ = bool();
                                            phi_1966_ = bool();
                                            if _e765 {
                                                break;
                                            }
                                            let _e767 = local_8;
                                            let _e768 = (_e767 == 8i);
                                            if _e768 {
                                                phi_2694_ = _e765;
                                                phi_1337_ = false;
                                                phi_1338_ = false;
                                            } else {
                                                phi_1271_ = type_19(0u, 8u);
                                                phi_1274_ = 0i;
                                                loop {
                                                    let _e770 = phi_1271_;
                                                    let _e772 = phi_1274_;
                                                    local_9 = _e772;
                                                    if (_e770.member < _e770.member_1) {
                                                        phi_1272_ = type_19((_e770.member + 1u), _e770.member_1);
                                                        phi_1289_ = type_19(1u, _e770.member);
                                                    } else {
                                                        phi_1272_ = _e770;
                                                        phi_1289_ = type_19(0u, type_19().member_1);
                                                    }
                                                    let _e785 = phi_1272_;
                                                    let _e787 = phi_1289_;
                                                    switch bitcast<i32>(_e787.member) {
                                                        case 0: {
                                                            phi_1275_ = i32();
                                                            phi_1331_ = false;
                                                            break;
                                                        }
                                                        case 1: {
                                                            if (_e787.member_1 < 8u) {
                                                            } else {
                                                                phi_2687_ = true;
                                                                break;
                                                            }
                                                            let _e794 = local_1[_e787.member_1][0u];
                                                            let _e797 = local_1[_e787.member_1][1u];
                                                            let _e800 = local_1[_e787.member_1][2u];
                                                            local_4 = array<f32, 3>(_e794, _e797, _e800);
                                                            let _e802 = (_e709.member_1 < 3u);
                                                            if _e802 {
                                                            } else {
                                                                phi_2687_ = true;
                                                                break;
                                                            }
                                                            let _e804 = local_4[_e709.member_1];
                                                            local_5 = array<f32, 3>(_e690.x, _e690.y, _e690.z);
                                                            if _e802 {
                                                            } else {
                                                                phi_2687_ = true;
                                                                break;
                                                            }
                                                            let _e810 = local_5[_e709.member_1];
                                                            if (_e804 > _e810) {
                                                                phi_1330_ = (_e772 + 1i);
                                                            } else {
                                                                phi_1330_ = _e772;
                                                            }
                                                            let _e814 = phi_1330_;
                                                            phi_1275_ = _e814;
                                                            phi_1331_ = true;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_1275_ = i32();
                                                            phi_1331_ = bool();
                                                            break;
                                                        }
                                                    }
                                                    let _e816 = phi_1275_;
                                                    let _e818 = phi_1331_;
                                                    continue;
                                                    continuing {
                                                        phi_1271_ = _e785;
                                                        phi_1274_ = _e816;
                                                        phi_2687_ = _e765;
                                                        break if !(_e818);
                                                    }
                                                }
                                                let _e821 = phi_2687_;
                                                phi_2692_ = _e821;
                                                phi_1968_ = bool();
                                                phi_1967_ = bool();
                                                phi_1966_ = bool();
                                                if _e821 {
                                                    break;
                                                }
                                                let _e823 = local_9;
                                                let _e824 = (_e823 == 8i);
                                                phi_2694_ = _e821;
                                                phi_1337_ = select(true, false, _e824);
                                                phi_1338_ = _e824;
                                            }
                                            let _e827 = phi_2694_;
                                            let _e829 = phi_1337_;
                                            let _e831 = phi_1338_;
                                            phi_2693_ = _e827;
                                            phi_1339_ = _e829;
                                            phi_1340_ = false;
                                            phi_1341_ = _e768;
                                            phi_1342_ = _e831;
                                            break;
                                        }
                                        default: {
                                            phi_2693_ = _e692;
                                            phi_1339_ = bool();
                                            phi_1340_ = bool();
                                            phi_1341_ = bool();
                                            phi_1342_ = bool();
                                            break;
                                        }
                                    }
                                    let _e833 = phi_2693_;
                                    let _e835 = phi_1339_;
                                    let _e837 = phi_1340_;
                                    let _e839 = phi_1341_;
                                    let _e841 = phi_1342_;
                                    continue;
                                    continuing {
                                        phi_2685_ = _e833;
                                        phi_1178_ = _e707;
                                        phi_2692_ = _e833;
                                        phi_1968_ = _e841;
                                        phi_1967_ = _e839;
                                        phi_1966_ = _e837;
                                        break if !(_e835);
                                    }
                                }
                                let _e844 = phi_2692_;
                                let _e846 = phi_1968_;
                                let _e848 = phi_1967_;
                                let _e850 = phi_1966_;
                                if _e844 {
                                    break;
                                }
                                let _e851 = select(_e848, false, _e850);
                                if select(true, false, select(_e851, true, select(select(_e846, false, _e850), false, _e851))) {
                                    phi_1365_ = type_33(false, 0u);
                                } else {
                                    phi_1365_ = type_33(true, 0u);
                                }
                                let _e857 = phi_1365_;
                                phi_1366_ = _e857;
                            } else {
                                phi_1366_ = type_33();
                            }
                            let _e859 = phi_1366_;
                            if select(true, false, _e687) {
                                let _e862 = local_10;
                                phi_1372_ = type_33(true, _e862);
                            } else {
                                phi_1372_ = _e859;
                            }
                            let _e865 = phi_1372_;
                            phi_1373_ = _e865;
                        }
                        let _e867 = phi_1373_;
                        if (_e867.member != true) {
                            let _e872 = global_1.member[0u];
                            let _e875 = global_1.member[1u];
                            let _e878 = global_1.member[3u];
                            let _e881 = global_1.member[4u];
                            let _e882 = f32(_e872);
                            let _e924 = fma(_e485.member.member_3.z, _e485.member_1.member.w, fma(_e485.member.member_2.z, _e485.member_1.member.z, fma(_e485.member.member.z, _e485.member_1.member.x, (_e485.member.member_1.z * _e485.member_1.member.y))));
                            let _e925 = fma(_e485.member.member_3.w, _e485.member_1.member.w, fma(_e485.member.member_2.w, _e485.member_1.member.z, fma(_e485.member.member.w, _e485.member_1.member.x, (_e485.member.member_1.w * _e485.member_1.member.y))));
                            let _e945 = fma(_e485.member.member_3.z, _e485.member_1.member_1.w, fma(_e485.member.member_2.z, _e485.member_1.member_1.z, fma(_e485.member.member.z, _e485.member_1.member_1.x, (_e485.member.member_1.z * _e485.member_1.member_1.y))));
                            let _e946 = fma(_e485.member.member_3.w, _e485.member_1.member_1.w, fma(_e485.member.member_2.w, _e485.member_1.member_1.z, fma(_e485.member.member.w, _e485.member_1.member_1.x, (_e485.member.member_1.w * _e485.member_1.member_1.y))));
                            let _e966 = fma(_e485.member.member_3.z, _e485.member_1.member_2.w, fma(_e485.member.member_2.z, _e485.member_1.member_2.z, fma(_e485.member.member.z, _e485.member_1.member_2.x, (_e485.member.member_1.z * _e485.member_1.member_2.y))));
                            let _e967 = fma(_e485.member.member_3.w, _e485.member_1.member_2.w, fma(_e485.member.member_2.w, _e485.member_1.member_2.z, fma(_e485.member.member.w, _e485.member_1.member_2.x, (_e485.member.member_1.w * _e485.member_1.member_2.y))));
                            let _e987 = fma(_e485.member.member_3.z, _e485.member_1.member_3.w, fma(_e485.member.member_2.z, _e485.member_1.member_3.z, fma(_e485.member.member.z, _e485.member_1.member_3.x, (_e485.member.member_1.z * _e485.member_1.member_3.y))));
                            let _e988 = fma(_e485.member.member_3.w, _e485.member_1.member_3.w, fma(_e485.member.member_2.w, _e485.member_1.member_3.z, fma(_e485.member.member.w, _e485.member_1.member_3.x, (_e485.member.member_1.w * _e485.member_1.member_3.y))));
                            let _e1000 = (fma(_e967, _e610, fma(_e925, _e608, (_e946 * _e609))) + _e988);
                            let _e1006 = fma(_e616, _e485.member_2.member_1[5].x, _e608);
                            let _e1007 = fma(_e616, _e485.member_2.member_1[5].y, _e609);
                            let _e1008 = fma(_e616, _e485.member_2.member_1[5].z, _e610);
                            let _e1023 = fma(_e616, _e485.member_2.member_1[0].x, _e608);
                            let _e1024 = fma(_e616, _e485.member_2.member_1[0].y, _e609);
                            let _e1025 = fma(_e616, _e485.member_2.member_1[0].z, _e610);
                            let _e1042 = (vec2<f32>(((((fma(fma(_e485.member.member_3.x, _e485.member_1.member_2.w, fma(_e485.member.member_2.x, _e485.member_1.member_2.z, fma(_e485.member.member.x, _e485.member_1.member_2.x, (_e485.member.member_1.x * _e485.member_1.member_2.y)))), _e610, fma(fma(_e485.member.member_3.x, _e485.member_1.member.w, fma(_e485.member.member_2.x, _e485.member_1.member.z, fma(_e485.member.member.x, _e485.member_1.member.x, (_e485.member.member_1.x * _e485.member_1.member.y)))), _e608, (fma(_e485.member.member_3.x, _e485.member_1.member_1.w, fma(_e485.member.member_2.x, _e485.member_1.member_1.z, fma(_e485.member.member.x, _e485.member_1.member_1.x, (_e485.member.member_1.x * _e485.member_1.member_1.y)))) * _e609))) + fma(_e485.member.member_3.x, _e485.member_1.member_3.w, fma(_e485.member.member_2.x, _e485.member_1.member_3.z, fma(_e485.member.member.x, _e485.member_1.member_3.x, (_e485.member.member_1.x * _e485.member_1.member_3.y))))) / _e1000) + 1f) * 0.5f), fma((-1f - ((fma(fma(_e485.member.member_3.y, _e485.member_1.member_2.w, fma(_e485.member.member_2.y, _e485.member_1.member_2.z, fma(_e485.member.member.y, _e485.member_1.member_2.x, (_e485.member.member_1.y * _e485.member_1.member_2.y)))), _e610, fma(fma(_e485.member.member_3.y, _e485.member_1.member.w, fma(_e485.member.member_2.y, _e485.member_1.member.z, fma(_e485.member.member.y, _e485.member_1.member.x, (_e485.member.member_1.y * _e485.member_1.member.y)))), _e608, (fma(_e485.member.member_3.y, _e485.member_1.member_1.w, fma(_e485.member.member_2.y, _e485.member_1.member_1.z, fma(_e485.member.member.y, _e485.member_1.member_1.x, (_e485.member.member_1.y * _e485.member_1.member_1.y)))) * _e609))) + fma(_e485.member.member_3.y, _e485.member_1.member_3.w, fma(_e485.member.member_2.y, _e485.member_1.member_3.z, fma(_e485.member.member.y, _e485.member_1.member_3.x, (_e485.member.member_1.y * _e485.member_1.member_3.y))))) / _e1000)), 0.5f, 1f)) * vec2<f32>(_e882, f32(_e875)));
                            let _e1043 = (_e616 / _e1000);
                            let _e1045 = -(_e882);
                            let _e1049 = vec3<f32>(fma(_e1045, _e1043, _e1042.x), fma(_e1045, _e1043, _e1042.y), ((_e987 + fma(_e966, _e1008, fma(_e945, _e1007, (_e924 * _e1006)))) / (_e988 + fma(_e967, _e1008, fma(_e946, _e1007, (_e925 * _e1006))))));
                            let _e1052 = vec3<f32>(fma(_e882, _e1043, _e1042.x), fma(_e882, _e1043, _e1042.y), ((_e987 + fma(_e966, _e1025, fma(_e945, _e1024, (_e924 * _e1023)))) / (_e988 + fma(_e967, _e1025, fma(_e946, _e1024, (_e925 * _e1023))))));
                            let _e1053 = min(_e1049, _e1052);
                            let _e1054 = max(_e1049, _e1052);
                            let _e1059 = (_e1054.x - _e1053.x);
                            let _e1060 = (_e1054.y - _e1053.y);
                            let _e1064 = floor(log2(select(_e1060, _e1059, (_e1059 > _e1060))));
                            let _e1069 = select(select(u32(_e1064), 0u, (_e1064 < 0f)), 4294967295u, (_e1064 > 4294967000f));
                            let _e1070 = (_e881 - 1u);
                            let _e1072 = select(_e1069, _e1070, (_e1069 > _e1070));
                            let _e1078 = round(((_e1053.x + _e1054.x) * 0.5f));
                            let _e1084 = (_e1072 & 31u);
                            let _e1087 = round(((_e1053.y + _e1054.y) * 0.5f));
                            if (_e1072 >= _e881) {
                                phi_1632_ = 4294967295u;
                            } else {
                                phi_1632_ = (_e878 + (2u * _e1072));
                            }
                            let _e1099 = phi_1632_;
                            if (_e92 >= 2u) {
                                phi_2478_ = (_e1099 <= (_e92 - 2u));
                            } else {
                                phi_2478_ = false;
                            }
                            let _e1104 = phi_2478_;
                            if _e1104 {
                                let _e1107 = global_1.member[_e1099];
                                let _e1111 = global_1.member[(_e1099 + 1u)];
                                phi_1650_ = type_19(_e1107, _e1111);
                            } else {
                                phi_1650_ = type_19(4294967295u, 0u);
                            }
                            let _e1114 = phi_1650_;
                            let _e1120 = (((select(select(u32(_e1087), 0u, (_e1087 < 0f)), 4294967295u, (_e1087 > 4294967000f)) >> bitcast<u32>(_e1084)) * (_e872 >> bitcast<u32>(_e1084))) + (select(select(u32(_e1078), 0u, (_e1078 < 0f)), 4294967295u, (_e1078 > 4294967000f)) >> bitcast<u32>(_e1084)));
                            if (_e1120 >= _e1114.member_1) {
                                phi_2504_ = 4294967295u;
                            } else {
                                phi_2504_ = (_e1114.member + _e1120);
                            }
                            let _e1124 = phi_2504_;
                            let _e1127 = global_1.member[_e1124];
                            if select((_e1053.z > 1f), true, (_e1053.z > bitcast<f32>(_e1127))) {
                                global_2.member[_e95.x].member_1 = 0u;
                            }
                        } else {
                            global_2.member[_e95.x].member_1 = 0u;
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
