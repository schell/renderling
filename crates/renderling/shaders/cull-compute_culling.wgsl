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
    member_2: u32,
    member_3: bool,
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
    var phi_628_: u32;
    var phi_647_: type_31;
    var phi_2090_: bool;
    var phi_812_: type_12;
    var phi_813_: type_12;
    var phi_836_: type_12;
    var phi_863_: bool;
    var phi_869_: type_12;
    var phi_870_: type_12;
    var phi_893_: type_12;
    var phi_916_: bool;
    var phi_937_: type_19;
    var phi_2122_: bool;
    var phi_988_: type_32;
    var phi_1120_: type_12;
    var phi_1121_: type_12;
    var phi_1144_: type_12;
    var phi_1188_: bool;
    var phi_1192_: bool;
    var phi_1193_: bool;
    var phi_2686_: bool;
    var phi_2008_: bool;
    var phi_2713_: bool;
    var phi_1200_: type_12;
    var phi_1201_: type_12;
    var phi_1224_: type_12;
    var phi_1234_: type_12;
    var phi_1237_: i32;
    var phi_1235_: type_12;
    var phi_1260_: type_12;
    var phi_1301_: i32;
    var phi_1238_: i32;
    var phi_1302_: bool;
    var phi_2707_: bool;
    var local_8: i32;
    var phi_1309_: type_12;
    var phi_1312_: i32;
    var phi_1310_: type_12;
    var phi_1335_: type_12;
    var phi_1376_: i32;
    var phi_1313_: i32;
    var phi_1377_: bool;
    var phi_2714_: bool;
    var local_9: i32;
    var phi_2721_: bool;
    var phi_1383_: bool;
    var phi_1384_: bool;
    var phi_2720_: bool;
    var phi_1385_: bool;
    var phi_1386_: bool;
    var phi_1387_: bool;
    var phi_1388_: bool;
    var phi_2719_: bool;
    var phi_2011_: bool;
    var phi_2010_: bool;
    var phi_2009_: bool;
    var phi_1411_: type_33;
    var phi_1412_: type_33;
    var local_10: u32;
    var phi_1418_: type_33;
    var phi_1419_: type_33;
    var phi_1683_: u32;
    var phi_2505_: bool;
    var phi_1701_: type_12;
    var phi_2531_: u32;

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
                    if select(false, true, (_e93 >= 9u)) {
                        let _e163 = global.member[0u];
                        let _e166 = global.member[1u];
                        let _e170 = global.member[2u];
                        let _e173 = global.member[3u];
                        let _e177 = global.member[4u];
                        switch bitcast<i32>(_e177) {
                            case 0: {
                                phi_628_ = 0u;
                                break;
                            }
                            case 1: {
                                phi_628_ = 1u;
                                break;
                            }
                            case 2: {
                                phi_628_ = 2u;
                                break;
                            }
                            case 3: {
                                phi_628_ = 3u;
                                break;
                            }
                            case 4: {
                                phi_628_ = 4u;
                                break;
                            }
                            case 5: {
                                phi_628_ = 5u;
                                break;
                            }
                            case 6: {
                                phi_628_ = 6u;
                                break;
                            }
                            case 7: {
                                phi_628_ = 7u;
                                break;
                            }
                            case 8: {
                                phi_628_ = 8u;
                                break;
                            }
                            case 9: {
                                phi_628_ = 9u;
                                break;
                            }
                            case 10: {
                                phi_628_ = 10u;
                                break;
                            }
                            case 11: {
                                phi_628_ = 11u;
                                break;
                            }
                            case 12: {
                                phi_628_ = 12u;
                                break;
                            }
                            case 13: {
                                phi_628_ = 13u;
                                break;
                            }
                            case 14: {
                                phi_628_ = 14u;
                                break;
                            }
                            case 15: {
                                phi_628_ = 15u;
                                break;
                            }
                            case 16: {
                                phi_628_ = 16u;
                                break;
                            }
                            case 17: {
                                phi_628_ = 17u;
                                break;
                            }
                            case 18: {
                                phi_628_ = 18u;
                                break;
                            }
                            case 19: {
                                phi_628_ = 19u;
                                break;
                            }
                            default: {
                                phi_628_ = 0u;
                                break;
                            }
                        }
                        let _e180 = phi_628_;
                        let _e183 = global.member[5u];
                        let _e187 = global.member[6u];
                        let _e191 = global.member[7u];
                        let _e195 = global.member[8u];
                        phi_647_ = type_31(vec2<u32>(_e163, _e166), vec2<u32>(_e170, _e173), _e180, (_e183 == 1u), (_e187 == 1u), (_e191 == 1u), (_e195 == 1u));
                    } else {
                        phi_647_ = type_31(vec2<u32>(0u, 0u), vec2<u32>(1u, 1u), 0u, true, true, true, false);
                    }
                    let _e199 = phi_647_;
                    if _e199.member_5 {
                        if (_e93 >= 86u) {
                            phi_2090_ = (_e144 <= (_e93 - 86u));
                        } else {
                            phi_2090_ = false;
                        }
                        let _e205 = phi_2090_;
                        if _e205 {
                            let _e208 = global.member[_e144];
                            let _e213 = global.member[(_e144 + 1u)];
                            let _e218 = global.member[(_e144 + 2u)];
                            let _e223 = global.member[(_e144 + 3u)];
                            let _e229 = global.member[(_e144 + 4u)];
                            let _e234 = global.member[(_e144 + 5u)];
                            let _e239 = global.member[(_e144 + 6u)];
                            let _e244 = global.member[(_e144 + 7u)];
                            let _e250 = global.member[(_e144 + 8u)];
                            let _e255 = global.member[(_e144 + 9u)];
                            let _e260 = global.member[(_e144 + 10u)];
                            let _e265 = global.member[(_e144 + 11u)];
                            let _e271 = global.member[(_e144 + 12u)];
                            let _e276 = global.member[(_e144 + 13u)];
                            let _e281 = global.member[(_e144 + 14u)];
                            let _e286 = global.member[(_e144 + 15u)];
                            let _e293 = global.member[(_e144 + 16u)];
                            let _e298 = global.member[(_e144 + 17u)];
                            let _e303 = global.member[(_e144 + 18u)];
                            let _e308 = global.member[(_e144 + 19u)];
                            let _e314 = global.member[(_e144 + 20u)];
                            let _e319 = global.member[(_e144 + 21u)];
                            let _e324 = global.member[(_e144 + 22u)];
                            let _e329 = global.member[(_e144 + 23u)];
                            let _e335 = global.member[(_e144 + 24u)];
                            let _e340 = global.member[(_e144 + 25u)];
                            let _e345 = global.member[(_e144 + 26u)];
                            let _e350 = global.member[(_e144 + 27u)];
                            let _e356 = global.member[(_e144 + 28u)];
                            let _e361 = global.member[(_e144 + 29u)];
                            let _e366 = global.member[(_e144 + 30u)];
                            let _e371 = global.member[(_e144 + 31u)];
                            let _e378 = global.member[(_e144 + 32u)];
                            let _e383 = global.member[(_e144 + 33u)];
                            let _e388 = global.member[(_e144 + 34u)];
                            local_7 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            phi_812_ = type_12(0u, 6u);
                            loop {
                                let _e393 = phi_812_;
                                if (_e393.member < _e393.member_1) {
                                    phi_813_ = type_12((_e393.member + 1u), _e393.member_1);
                                    phi_836_ = type_12(1u, _e393.member);
                                } else {
                                    phi_813_ = _e393;
                                    phi_836_ = type_12(0u, type_12().member_1);
                                }
                                let _e406 = phi_813_;
                                let _e408 = phi_836_;
                                switch bitcast<i32>(_e408.member) {
                                    case 0: {
                                        phi_863_ = false;
                                        break;
                                    }
                                    case 1: {
                                        let _e413 = ((_e144 + 35u) + (_e408.member_1 * 4u));
                                        let _e416 = global.member[_e413];
                                        let _e421 = global.member[(_e413 + 1u)];
                                        let _e426 = global.member[(_e413 + 2u)];
                                        let _e431 = global.member[(_e413 + 3u)];
                                        local_7[_e408.member_1] = vec4<f32>(bitcast<f32>(_e416), bitcast<f32>(_e421), bitcast<f32>(_e426), bitcast<f32>(_e431));
                                        phi_863_ = true;
                                        break;
                                    }
                                    default: {
                                        phi_863_ = bool();
                                        break;
                                    }
                                }
                                let _e436 = phi_863_;
                                continue;
                                continuing {
                                    phi_812_ = _e406;
                                    break if !(_e436);
                                }
                            }
                            let _e438 = local_7;
                            local_6 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            phi_869_ = type_12(0u, 8u);
                            loop {
                                let _e441 = phi_869_;
                                if (_e441.member < _e441.member_1) {
                                    phi_870_ = type_12((_e441.member + 1u), _e441.member_1);
                                    phi_893_ = type_12(1u, _e441.member);
                                } else {
                                    phi_870_ = _e441;
                                    phi_893_ = type_12(0u, type_12().member_1);
                                }
                                let _e454 = phi_870_;
                                let _e456 = phi_893_;
                                switch bitcast<i32>(_e456.member) {
                                    case 0: {
                                        phi_916_ = false;
                                        break;
                                    }
                                    case 1: {
                                        let _e461 = ((_e144 + 59u) + (_e456.member_1 * 3u));
                                        let _e464 = global.member[_e461];
                                        let _e469 = global.member[(_e461 + 1u)];
                                        let _e474 = global.member[(_e461 + 2u)];
                                        local_6[_e456.member_1] = vec3<f32>(bitcast<f32>(_e464), bitcast<f32>(_e469), bitcast<f32>(_e474));
                                        phi_916_ = true;
                                        break;
                                    }
                                    default: {
                                        phi_916_ = bool();
                                        break;
                                    }
                                }
                                let _e479 = phi_916_;
                                continue;
                                continuing {
                                    phi_869_ = _e454;
                                    break if !(_e479);
                                }
                            }
                            let _e481 = local_6;
                            let _e485 = global.member[(_e144 + 83u)];
                            let _e490 = global.member[(_e144 + 84u)];
                            let _e495 = global.member[(_e144 + 85u)];
                            phi_937_ = type_19(type_17(vec4<f32>(bitcast<f32>(_e208), bitcast<f32>(_e213), bitcast<f32>(_e218), bitcast<f32>(_e223)), vec4<f32>(bitcast<f32>(_e229), bitcast<f32>(_e234), bitcast<f32>(_e239), bitcast<f32>(_e244)), vec4<f32>(bitcast<f32>(_e250), bitcast<f32>(_e255), bitcast<f32>(_e260), bitcast<f32>(_e265)), vec4<f32>(bitcast<f32>(_e271), bitcast<f32>(_e276), bitcast<f32>(_e281), bitcast<f32>(_e286))), type_17(vec4<f32>(bitcast<f32>(_e293), bitcast<f32>(_e298), bitcast<f32>(_e303), bitcast<f32>(_e308)), vec4<f32>(bitcast<f32>(_e314), bitcast<f32>(_e319), bitcast<f32>(_e324), bitcast<f32>(_e329)), vec4<f32>(bitcast<f32>(_e335), bitcast<f32>(_e340), bitcast<f32>(_e345), bitcast<f32>(_e350)), vec4<f32>(bitcast<f32>(_e356), bitcast<f32>(_e361), bitcast<f32>(_e366), bitcast<f32>(_e371))), vec3<f32>(bitcast<f32>(_e378), bitcast<f32>(_e383), bitcast<f32>(_e388)), type_18(_e481, _e438, vec3<f32>(bitcast<f32>(_e485), bitcast<f32>(_e490), bitcast<f32>(_e495))));
                        } else {
                            phi_937_ = type_19(type_17(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_17(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_18(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
                        }
                        let _e501 = phi_937_;
                        if (_e93 >= 10u) {
                            phi_2122_ = (_e148 <= (_e93 - 10u));
                        } else {
                            phi_2122_ = false;
                        }
                        let _e511 = phi_2122_;
                        if _e511 {
                            let _e514 = global.member[_e148];
                            let _e519 = global.member[(_e148 + 1u)];
                            let _e524 = global.member[(_e148 + 2u)];
                            let _e530 = global.member[(_e148 + 3u)];
                            let _e535 = global.member[(_e148 + 4u)];
                            let _e540 = global.member[(_e148 + 5u)];
                            let _e545 = global.member[(_e148 + 6u)];
                            let _e551 = global.member[(_e148 + 7u)];
                            let _e556 = global.member[(_e148 + 8u)];
                            let _e561 = global.member[(_e148 + 9u)];
                            phi_988_ = type_32(vec3<f32>(bitcast<f32>(_e514), bitcast<f32>(_e519), bitcast<f32>(_e524)), vec4<f32>(bitcast<f32>(_e530), bitcast<f32>(_e535), bitcast<f32>(_e540), bitcast<f32>(_e545)), vec3<f32>(bitcast<f32>(_e551), bitcast<f32>(_e556), bitcast<f32>(_e561)));
                        } else {
                            phi_988_ = type_32(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                        }
                        let _e566 = phi_988_;
                        let _e574 = (_e566.member_1.x + _e566.member_1.x);
                        let _e575 = (_e566.member_1.y + _e566.member_1.y);
                        let _e576 = (_e566.member_1.z + _e566.member_1.z);
                        let _e578 = (_e566.member_1.z * _e576);
                        let _e579 = (_e566.member_1.w * _e574);
                        let _e580 = (_e566.member_1.w * _e575);
                        let _e581 = (_e566.member_1.w * _e576);
                        let _e601 = (vec4<f32>((1f - fma(_e566.member_1.y, _e575, _e578)), fma(_e566.member_1.x, _e575, _e581), fma(_e566.member_1.x, _e576, -(_e580)), 0f) * _e566.member_2.x);
                        let _e603 = (vec4<f32>(fma(_e566.member_1.x, _e575, -(_e581)), (1f - fma(_e566.member_1.x, _e574, _e578)), fma(_e566.member_1.y, _e576, _e579), 0f) * _e566.member_2.y);
                        let _e605 = (vec4<f32>(fma(_e566.member_1.x, _e576, _e580), fma(_e566.member_1.y, _e576, -(_e579)), (1f - fma(_e566.member_1.x, _e574, (_e566.member_1.y * _e575))), 0f) * _e566.member_2.z);
                        let _e627 = (_e566.member.x + fma(_e605.x, _e127, fma(_e603.x, _e122, (_e601.x * _e117))));
                        let _e628 = (_e566.member.y + fma(_e605.y, _e127, fma(_e603.y, _e122, (_e601.y * _e117))));
                        let _e629 = (_e566.member.z + fma(_e605.z, _e127, fma(_e603.z, _e122, (_e601.z * _e117))));
                        let _e630 = vec3<f32>(_e627, _e628, _e629);
                        let _e633 = (max(_e566.member_2.x, max(_e566.member_2.y, _e566.member_2.z)) * _e132);
                        let _e635 = sqrt((_e633 * _e633));
                        local_1 = _e501.member_3.member;
                        local = _e501.member_3.member_1;
                        let _e638 = local[0u][0u];
                        let _e641 = local[0u][1u];
                        let _e646 = local[0u][2u];
                        let _e650 = local[0u][3u];
                        let _e652 = -(_e635);
                        if ((fma(_e646, _e629, fma(_e638, _e627, (_e641 * _e628))) + _e650) < _e652) {
                            phi_1419_ = type_33(true, 0u);
                        } else {
                            phi_1120_ = type_12(0u, 6u);
                            loop {
                                let _e655 = phi_1120_;
                                if (_e655.member < _e655.member_1) {
                                    phi_1121_ = type_12((_e655.member + 1u), _e655.member_1);
                                    phi_1144_ = type_12(1u, _e655.member);
                                } else {
                                    phi_1121_ = _e655;
                                    phi_1144_ = type_12(0u, type_12().member_1);
                                }
                                let _e668 = phi_1121_;
                                let _e670 = phi_1144_;
                                local_10 = _e670.member_1;
                                switch bitcast<i32>(_e670.member) {
                                    case 0: {
                                        phi_1192_ = false;
                                        phi_1193_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if (_e670.member_1 != 0u) {
                                            if (_e670.member_1 < 6u) {
                                            } else {
                                                phi_2686_ = true;
                                                phi_2008_ = bool();
                                                break;
                                            }
                                            let _e678 = local[_e670.member_1][0u];
                                            let _e681 = local[_e670.member_1][1u];
                                            let _e686 = local[_e670.member_1][2u];
                                            let _e690 = local[_e670.member_1][3u];
                                            phi_1188_ = select(true, false, ((fma(_e686, _e629, fma(_e678, _e627, (_e681 * _e628))) + _e690) < _e652));
                                        } else {
                                            phi_1188_ = true;
                                        }
                                        let _e695 = phi_1188_;
                                        phi_1192_ = _e695;
                                        phi_1193_ = false;
                                        break;
                                    }
                                    default: {
                                        phi_1192_ = bool();
                                        phi_1193_ = bool();
                                        break;
                                    }
                                }
                                let _e697 = phi_1192_;
                                let _e699 = phi_1193_;
                                continue;
                                continuing {
                                    phi_1120_ = _e668;
                                    phi_2686_ = false;
                                    phi_2008_ = _e699;
                                    break if !(_e697);
                                }
                            }
                            let _e702 = phi_2686_;
                            let _e704 = phi_2008_;
                            if _e702 {
                                break;
                            }
                            if _e704 {
                                let _e705 = vec3(_e635);
                                let _e706 = (_e630 - _e705);
                                let _e707 = (_e630 + _e705);
                                phi_2713_ = _e702;
                                phi_1200_ = type_12(0u, 3u);
                                loop {
                                    let _e709 = phi_2713_;
                                    let _e711 = phi_1200_;
                                    if (_e711.member < _e711.member_1) {
                                        phi_1201_ = type_12((_e711.member + 1u), _e711.member_1);
                                        phi_1224_ = type_12(1u, _e711.member);
                                    } else {
                                        phi_1201_ = _e711;
                                        phi_1224_ = type_12(0u, type_12().member_1);
                                    }
                                    let _e724 = phi_1201_;
                                    let _e726 = phi_1224_;
                                    switch bitcast<i32>(_e726.member) {
                                        case 0: {
                                            phi_2720_ = _e709;
                                            phi_1385_ = false;
                                            phi_1386_ = true;
                                            phi_1387_ = false;
                                            phi_1388_ = false;
                                            break;
                                        }
                                        case 1: {
                                            phi_1234_ = type_12(0u, 8u);
                                            phi_1237_ = 0i;
                                            loop {
                                                let _e731 = phi_1234_;
                                                let _e733 = phi_1237_;
                                                local_8 = _e733;
                                                if (_e731.member < _e731.member_1) {
                                                    phi_1235_ = type_12((_e731.member + 1u), _e731.member_1);
                                                    phi_1260_ = type_12(1u, _e731.member);
                                                } else {
                                                    phi_1235_ = _e731;
                                                    phi_1260_ = type_12(0u, type_12().member_1);
                                                }
                                                let _e746 = phi_1235_;
                                                let _e748 = phi_1260_;
                                                switch bitcast<i32>(_e748.member) {
                                                    case 0: {
                                                        phi_1238_ = i32();
                                                        phi_1302_ = false;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if (_e748.member_1 < 8u) {
                                                        } else {
                                                            phi_2707_ = true;
                                                            break;
                                                        }
                                                        let _e755 = local_1[_e748.member_1][0u];
                                                        let _e758 = local_1[_e748.member_1][1u];
                                                        let _e761 = local_1[_e748.member_1][2u];
                                                        local_2 = array<f32, 3>(_e755, _e758, _e761);
                                                        let _e763 = (_e726.member_1 < 3u);
                                                        if _e763 {
                                                        } else {
                                                            phi_2707_ = true;
                                                            break;
                                                        }
                                                        let _e765 = local_2[_e726.member_1];
                                                        local_3 = array<f32, 3>(_e706.x, _e706.y, _e706.z);
                                                        if _e763 {
                                                        } else {
                                                            phi_2707_ = true;
                                                            break;
                                                        }
                                                        let _e771 = local_3[_e726.member_1];
                                                        if (_e765 < _e771) {
                                                            phi_1301_ = (_e733 + 1i);
                                                        } else {
                                                            phi_1301_ = _e733;
                                                        }
                                                        let _e775 = phi_1301_;
                                                        phi_1238_ = _e775;
                                                        phi_1302_ = true;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_1238_ = i32();
                                                        phi_1302_ = bool();
                                                        break;
                                                    }
                                                }
                                                let _e777 = phi_1238_;
                                                let _e779 = phi_1302_;
                                                continue;
                                                continuing {
                                                    phi_1234_ = _e746;
                                                    phi_1237_ = _e777;
                                                    phi_2707_ = _e709;
                                                    break if !(_e779);
                                                }
                                            }
                                            let _e782 = phi_2707_;
                                            phi_2719_ = _e782;
                                            phi_2011_ = bool();
                                            phi_2010_ = bool();
                                            phi_2009_ = bool();
                                            if _e782 {
                                                break;
                                            }
                                            let _e784 = local_8;
                                            let _e785 = (_e784 == 8i);
                                            if _e785 {
                                                phi_2721_ = _e782;
                                                phi_1383_ = false;
                                                phi_1384_ = false;
                                            } else {
                                                phi_1309_ = type_12(0u, 8u);
                                                phi_1312_ = 0i;
                                                loop {
                                                    let _e787 = phi_1309_;
                                                    let _e789 = phi_1312_;
                                                    local_9 = _e789;
                                                    if (_e787.member < _e787.member_1) {
                                                        phi_1310_ = type_12((_e787.member + 1u), _e787.member_1);
                                                        phi_1335_ = type_12(1u, _e787.member);
                                                    } else {
                                                        phi_1310_ = _e787;
                                                        phi_1335_ = type_12(0u, type_12().member_1);
                                                    }
                                                    let _e802 = phi_1310_;
                                                    let _e804 = phi_1335_;
                                                    switch bitcast<i32>(_e804.member) {
                                                        case 0: {
                                                            phi_1313_ = i32();
                                                            phi_1377_ = false;
                                                            break;
                                                        }
                                                        case 1: {
                                                            if (_e804.member_1 < 8u) {
                                                            } else {
                                                                phi_2714_ = true;
                                                                break;
                                                            }
                                                            let _e811 = local_1[_e804.member_1][0u];
                                                            let _e814 = local_1[_e804.member_1][1u];
                                                            let _e817 = local_1[_e804.member_1][2u];
                                                            local_4 = array<f32, 3>(_e811, _e814, _e817);
                                                            let _e819 = (_e726.member_1 < 3u);
                                                            if _e819 {
                                                            } else {
                                                                phi_2714_ = true;
                                                                break;
                                                            }
                                                            let _e821 = local_4[_e726.member_1];
                                                            local_5 = array<f32, 3>(_e707.x, _e707.y, _e707.z);
                                                            if _e819 {
                                                            } else {
                                                                phi_2714_ = true;
                                                                break;
                                                            }
                                                            let _e827 = local_5[_e726.member_1];
                                                            if (_e821 > _e827) {
                                                                phi_1376_ = (_e789 + 1i);
                                                            } else {
                                                                phi_1376_ = _e789;
                                                            }
                                                            let _e831 = phi_1376_;
                                                            phi_1313_ = _e831;
                                                            phi_1377_ = true;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_1313_ = i32();
                                                            phi_1377_ = bool();
                                                            break;
                                                        }
                                                    }
                                                    let _e833 = phi_1313_;
                                                    let _e835 = phi_1377_;
                                                    continue;
                                                    continuing {
                                                        phi_1309_ = _e802;
                                                        phi_1312_ = _e833;
                                                        phi_2714_ = _e782;
                                                        break if !(_e835);
                                                    }
                                                }
                                                let _e838 = phi_2714_;
                                                phi_2719_ = _e838;
                                                phi_2011_ = bool();
                                                phi_2010_ = bool();
                                                phi_2009_ = bool();
                                                if _e838 {
                                                    break;
                                                }
                                                let _e840 = local_9;
                                                let _e841 = (_e840 == 8i);
                                                phi_2721_ = _e838;
                                                phi_1383_ = select(true, false, _e841);
                                                phi_1384_ = _e841;
                                            }
                                            let _e844 = phi_2721_;
                                            let _e846 = phi_1383_;
                                            let _e848 = phi_1384_;
                                            phi_2720_ = _e844;
                                            phi_1385_ = _e846;
                                            phi_1386_ = false;
                                            phi_1387_ = _e785;
                                            phi_1388_ = _e848;
                                            break;
                                        }
                                        default: {
                                            phi_2720_ = _e709;
                                            phi_1385_ = bool();
                                            phi_1386_ = bool();
                                            phi_1387_ = bool();
                                            phi_1388_ = bool();
                                            break;
                                        }
                                    }
                                    let _e850 = phi_2720_;
                                    let _e852 = phi_1385_;
                                    let _e854 = phi_1386_;
                                    let _e856 = phi_1387_;
                                    let _e858 = phi_1388_;
                                    continue;
                                    continuing {
                                        phi_2713_ = _e850;
                                        phi_1200_ = _e724;
                                        phi_2719_ = _e850;
                                        phi_2011_ = _e858;
                                        phi_2010_ = _e856;
                                        phi_2009_ = _e854;
                                        break if !(_e852);
                                    }
                                }
                                let _e861 = phi_2719_;
                                let _e863 = phi_2011_;
                                let _e865 = phi_2010_;
                                let _e867 = phi_2009_;
                                if _e861 {
                                    break;
                                }
                                let _e868 = select(_e865, false, _e867);
                                if select(true, false, select(_e868, true, select(select(_e863, false, _e867), false, _e868))) {
                                    phi_1411_ = type_33(false, 0u);
                                } else {
                                    phi_1411_ = type_33(true, 0u);
                                }
                                let _e874 = phi_1411_;
                                phi_1412_ = _e874;
                            } else {
                                phi_1412_ = type_33();
                            }
                            let _e876 = phi_1412_;
                            if select(true, false, _e704) {
                                let _e879 = local_10;
                                phi_1418_ = type_33(true, _e879);
                            } else {
                                phi_1418_ = _e876;
                            }
                            let _e882 = phi_1418_;
                            phi_1419_ = _e882;
                        }
                        let _e884 = phi_1419_;
                        if (_e884.member != true) {
                            global_2.member[_e98.x].member_1 = 1u;
                            if _e199.member_6 {
                                let _e890 = global_1.member[0u];
                                let _e893 = global_1.member[1u];
                                let _e896 = global_1.member[3u];
                                let _e899 = global_1.member[4u];
                                let _e900 = f32(_e890);
                                let _e942 = fma(_e501.member.member_3.z, _e501.member_1.member.w, fma(_e501.member.member_2.z, _e501.member_1.member.z, fma(_e501.member.member.z, _e501.member_1.member.x, (_e501.member.member_1.z * _e501.member_1.member.y))));
                                let _e943 = fma(_e501.member.member_3.w, _e501.member_1.member.w, fma(_e501.member.member_2.w, _e501.member_1.member.z, fma(_e501.member.member.w, _e501.member_1.member.x, (_e501.member.member_1.w * _e501.member_1.member.y))));
                                let _e963 = fma(_e501.member.member_3.z, _e501.member_1.member_1.w, fma(_e501.member.member_2.z, _e501.member_1.member_1.z, fma(_e501.member.member.z, _e501.member_1.member_1.x, (_e501.member.member_1.z * _e501.member_1.member_1.y))));
                                let _e964 = fma(_e501.member.member_3.w, _e501.member_1.member_1.w, fma(_e501.member.member_2.w, _e501.member_1.member_1.z, fma(_e501.member.member.w, _e501.member_1.member_1.x, (_e501.member.member_1.w * _e501.member_1.member_1.y))));
                                let _e984 = fma(_e501.member.member_3.z, _e501.member_1.member_2.w, fma(_e501.member.member_2.z, _e501.member_1.member_2.z, fma(_e501.member.member.z, _e501.member_1.member_2.x, (_e501.member.member_1.z * _e501.member_1.member_2.y))));
                                let _e985 = fma(_e501.member.member_3.w, _e501.member_1.member_2.w, fma(_e501.member.member_2.w, _e501.member_1.member_2.z, fma(_e501.member.member.w, _e501.member_1.member_2.x, (_e501.member.member_1.w * _e501.member_1.member_2.y))));
                                let _e1005 = fma(_e501.member.member_3.z, _e501.member_1.member_3.w, fma(_e501.member.member_2.z, _e501.member_1.member_3.z, fma(_e501.member.member.z, _e501.member_1.member_3.x, (_e501.member.member_1.z * _e501.member_1.member_3.y))));
                                let _e1006 = fma(_e501.member.member_3.w, _e501.member_1.member_3.w, fma(_e501.member.member_2.w, _e501.member_1.member_3.z, fma(_e501.member.member.w, _e501.member_1.member_3.x, (_e501.member.member_1.w * _e501.member_1.member_3.y))));
                                let _e1018 = (fma(_e985, _e629, fma(_e943, _e627, (_e964 * _e628))) + _e1006);
                                let _e1023 = fma(_e635, _e501.member_3.member_1[5].x, _e627);
                                let _e1024 = fma(_e635, _e501.member_3.member_1[5].y, _e628);
                                let _e1025 = fma(_e635, _e501.member_3.member_1[5].z, _e629);
                                let _e1039 = fma(_e635, _e501.member_3.member_1[0].x, _e627);
                                let _e1040 = fma(_e635, _e501.member_3.member_1[0].y, _e628);
                                let _e1041 = fma(_e635, _e501.member_3.member_1[0].z, _e629);
                                let _e1058 = (vec2<f32>(((((fma(fma(_e501.member.member_3.x, _e501.member_1.member_2.w, fma(_e501.member.member_2.x, _e501.member_1.member_2.z, fma(_e501.member.member.x, _e501.member_1.member_2.x, (_e501.member.member_1.x * _e501.member_1.member_2.y)))), _e629, fma(fma(_e501.member.member_3.x, _e501.member_1.member.w, fma(_e501.member.member_2.x, _e501.member_1.member.z, fma(_e501.member.member.x, _e501.member_1.member.x, (_e501.member.member_1.x * _e501.member_1.member.y)))), _e627, (fma(_e501.member.member_3.x, _e501.member_1.member_1.w, fma(_e501.member.member_2.x, _e501.member_1.member_1.z, fma(_e501.member.member.x, _e501.member_1.member_1.x, (_e501.member.member_1.x * _e501.member_1.member_1.y)))) * _e628))) + fma(_e501.member.member_3.x, _e501.member_1.member_3.w, fma(_e501.member.member_2.x, _e501.member_1.member_3.z, fma(_e501.member.member.x, _e501.member_1.member_3.x, (_e501.member.member_1.x * _e501.member_1.member_3.y))))) / _e1018) + 1f) * 0.5f), fma((-1f - ((fma(fma(_e501.member.member_3.y, _e501.member_1.member_2.w, fma(_e501.member.member_2.y, _e501.member_1.member_2.z, fma(_e501.member.member.y, _e501.member_1.member_2.x, (_e501.member.member_1.y * _e501.member_1.member_2.y)))), _e629, fma(fma(_e501.member.member_3.y, _e501.member_1.member.w, fma(_e501.member.member_2.y, _e501.member_1.member.z, fma(_e501.member.member.y, _e501.member_1.member.x, (_e501.member.member_1.y * _e501.member_1.member.y)))), _e627, (fma(_e501.member.member_3.y, _e501.member_1.member_1.w, fma(_e501.member.member_2.y, _e501.member_1.member_1.z, fma(_e501.member.member.y, _e501.member_1.member_1.x, (_e501.member.member_1.y * _e501.member_1.member_1.y)))) * _e628))) + fma(_e501.member.member_3.y, _e501.member_1.member_3.w, fma(_e501.member.member_2.y, _e501.member_1.member_3.z, fma(_e501.member.member.y, _e501.member_1.member_3.x, (_e501.member.member_1.y * _e501.member_1.member_3.y))))) / _e1018)), 0.5f, 1f)) * vec2<f32>(_e900, f32(_e893)));
                                let _e1059 = (_e635 / _e1018);
                                let _e1061 = -(_e900);
                                let _e1065 = vec3<f32>(fma(_e1061, _e1059, _e1058.x), fma(_e1061, _e1059, _e1058.y), ((_e1005 + fma(_e984, _e1025, fma(_e963, _e1024, (_e942 * _e1023)))) / (_e1006 + fma(_e985, _e1025, fma(_e964, _e1024, (_e943 * _e1023))))));
                                let _e1068 = vec3<f32>(fma(_e900, _e1059, _e1058.x), fma(_e900, _e1059, _e1058.y), ((_e1005 + fma(_e984, _e1041, fma(_e963, _e1040, (_e942 * _e1039)))) / (_e1006 + fma(_e985, _e1041, fma(_e964, _e1040, (_e943 * _e1039))))));
                                let _e1069 = min(_e1065, _e1068);
                                let _e1070 = max(_e1065, _e1068);
                                let _e1075 = (_e1070.x - _e1069.x);
                                let _e1076 = (_e1070.y - _e1069.y);
                                let _e1080 = floor(log2(select(_e1076, _e1075, (_e1075 > _e1076))));
                                let _e1085 = select(select(u32(_e1080), 0u, (_e1080 < 0f)), 4294967295u, (_e1080 > 4294967000f));
                                let _e1086 = (_e899 - 1u);
                                let _e1088 = select(_e1085, _e1086, (_e1085 > _e1086));
                                let _e1094 = round(((_e1069.x + _e1070.x) * 0.5f));
                                let _e1100 = (_e1088 & 31u);
                                let _e1103 = round(((_e1069.y + _e1070.y) * 0.5f));
                                if (_e1088 >= _e899) {
                                    phi_1683_ = 4294967295u;
                                } else {
                                    phi_1683_ = (_e896 + (2u * _e1088));
                                }
                                let _e1115 = phi_1683_;
                                if (_e95 >= 2u) {
                                    phi_2505_ = (_e1115 <= (_e95 - 2u));
                                } else {
                                    phi_2505_ = false;
                                }
                                let _e1120 = phi_2505_;
                                if _e1120 {
                                    let _e1123 = global_1.member[_e1115];
                                    let _e1127 = global_1.member[(_e1115 + 1u)];
                                    phi_1701_ = type_12(_e1123, _e1127);
                                } else {
                                    phi_1701_ = type_12(4294967295u, 0u);
                                }
                                let _e1130 = phi_1701_;
                                let _e1136 = (((select(select(u32(_e1103), 0u, (_e1103 < 0f)), 4294967295u, (_e1103 > 4294967000f)) >> bitcast<u32>(_e1100)) * (_e890 >> bitcast<u32>(_e1100))) + (select(select(u32(_e1094), 0u, (_e1094 < 0f)), 4294967295u, (_e1094 > 4294967000f)) >> bitcast<u32>(_e1100)));
                                if (_e1136 >= _e1130.member_1) {
                                    phi_2531_ = 4294967295u;
                                } else {
                                    phi_2531_ = (_e1130.member + _e1136);
                                }
                                let _e1140 = phi_2531_;
                                let _e1143 = global_1.member[_e1140];
                                if select((_e1069.z > 1f), true, (_e1069.z > bitcast<f32>(_e1143))) {
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
