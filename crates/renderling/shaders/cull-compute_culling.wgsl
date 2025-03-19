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
    var phi_627_: u32;
    var phi_646_: type_31;
    var phi_2088_: bool;
    var phi_810_: type_12;
    var phi_811_: type_12;
    var phi_834_: type_12;
    var phi_861_: bool;
    var phi_867_: type_12;
    var phi_868_: type_12;
    var phi_891_: type_12;
    var phi_914_: bool;
    var phi_935_: type_19;
    var phi_2120_: bool;
    var phi_986_: type_32;
    var phi_1118_: type_12;
    var phi_1119_: type_12;
    var phi_1142_: type_12;
    var phi_1186_: bool;
    var phi_1190_: bool;
    var phi_1191_: bool;
    var phi_2682_: bool;
    var phi_2006_: bool;
    var phi_2709_: bool;
    var phi_1198_: type_12;
    var phi_1199_: type_12;
    var phi_1222_: type_12;
    var phi_1232_: type_12;
    var phi_1235_: i32;
    var phi_1233_: type_12;
    var phi_1258_: type_12;
    var phi_1299_: i32;
    var phi_1236_: i32;
    var phi_1300_: bool;
    var phi_2703_: bool;
    var local_8: i32;
    var phi_1307_: type_12;
    var phi_1310_: i32;
    var phi_1308_: type_12;
    var phi_1333_: type_12;
    var phi_1374_: i32;
    var phi_1311_: i32;
    var phi_1375_: bool;
    var phi_2710_: bool;
    var local_9: i32;
    var phi_2717_: bool;
    var phi_1381_: bool;
    var phi_1382_: bool;
    var phi_2716_: bool;
    var phi_1383_: bool;
    var phi_1384_: bool;
    var phi_1385_: bool;
    var phi_1386_: bool;
    var phi_2715_: bool;
    var phi_2009_: bool;
    var phi_2008_: bool;
    var phi_2007_: bool;
    var phi_1409_: type_33;
    var phi_1410_: type_33;
    var local_10: u32;
    var phi_1416_: type_33;
    var phi_1417_: type_33;
    var phi_1681_: u32;
    var phi_2503_: bool;
    var phi_1699_: type_12;
    var phi_2529_: u32;

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
                global_2.member[_e98.x].member = select(_e140, _e112, (_e136 == 4294967295u));
                global_2.member[_e98.x].member_1 = select(0u, 1u, (_e107 == 1u));
                if (_e132 == 0f) {
                } else {
                    let _e155 = (_e93 >= 10u);
                    if select(false, true, _e155) {
                        let _e159 = global.member[0u];
                        let _e162 = global.member[1u];
                        let _e165 = global.member[2u];
                        let _e169 = global.member[3u];
                        let _e172 = global.member[4u];
                        let _e176 = global.member[5u];
                        switch bitcast<i32>(_e176) {
                            case 0: {
                                phi_627_ = 0u;
                                break;
                            }
                            case 1: {
                                phi_627_ = 1u;
                                break;
                            }
                            case 2: {
                                phi_627_ = 2u;
                                break;
                            }
                            case 3: {
                                phi_627_ = 3u;
                                break;
                            }
                            case 4: {
                                phi_627_ = 4u;
                                break;
                            }
                            case 5: {
                                phi_627_ = 5u;
                                break;
                            }
                            case 6: {
                                phi_627_ = 6u;
                                break;
                            }
                            case 7: {
                                phi_627_ = 7u;
                                break;
                            }
                            case 8: {
                                phi_627_ = 8u;
                                break;
                            }
                            case 9: {
                                phi_627_ = 9u;
                                break;
                            }
                            case 10: {
                                phi_627_ = 10u;
                                break;
                            }
                            case 11: {
                                phi_627_ = 11u;
                                break;
                            }
                            case 12: {
                                phi_627_ = 12u;
                                break;
                            }
                            case 13: {
                                phi_627_ = 13u;
                                break;
                            }
                            case 14: {
                                phi_627_ = 14u;
                                break;
                            }
                            case 15: {
                                phi_627_ = 15u;
                                break;
                            }
                            case 16: {
                                phi_627_ = 16u;
                                break;
                            }
                            case 17: {
                                phi_627_ = 17u;
                                break;
                            }
                            case 18: {
                                phi_627_ = 18u;
                                break;
                            }
                            case 19: {
                                phi_627_ = 19u;
                                break;
                            }
                            default: {
                                phi_627_ = 0u;
                                break;
                            }
                        }
                        let _e179 = phi_627_;
                        let _e182 = global.member[6u];
                        let _e186 = global.member[7u];
                        let _e190 = global.member[8u];
                        let _e194 = global.member[9u];
                        phi_646_ = type_31(vec2<u32>(_e162, _e165), vec2<u32>(_e169, _e172), _e159, _e179, (_e182 == 1u), (_e186 == 1u), (_e190 == 1u), (_e194 == 1u));
                    } else {
                        phi_646_ = type_31(vec2<u32>(0u, 0u), vec2<u32>(1u, 1u), 4294967295u, 0u, true, true, true, false);
                    }
                    let _e198 = phi_646_;
                    if _e198.member_6 {
                        if (_e93 >= 86u) {
                            phi_2088_ = (_e198.member_2 <= (_e93 - 86u));
                        } else {
                            phi_2088_ = false;
                        }
                        let _e205 = phi_2088_;
                        if _e205 {
                            let _e208 = global.member[_e198.member_2];
                            let _e213 = global.member[(_e198.member_2 + 1u)];
                            let _e218 = global.member[(_e198.member_2 + 2u)];
                            let _e223 = global.member[(_e198.member_2 + 3u)];
                            let _e229 = global.member[(_e198.member_2 + 4u)];
                            let _e234 = global.member[(_e198.member_2 + 5u)];
                            let _e239 = global.member[(_e198.member_2 + 6u)];
                            let _e244 = global.member[(_e198.member_2 + 7u)];
                            let _e250 = global.member[(_e198.member_2 + 8u)];
                            let _e255 = global.member[(_e198.member_2 + 9u)];
                            let _e260 = global.member[(_e198.member_2 + 10u)];
                            let _e265 = global.member[(_e198.member_2 + 11u)];
                            let _e271 = global.member[(_e198.member_2 + 12u)];
                            let _e276 = global.member[(_e198.member_2 + 13u)];
                            let _e281 = global.member[(_e198.member_2 + 14u)];
                            let _e286 = global.member[(_e198.member_2 + 15u)];
                            let _e293 = global.member[(_e198.member_2 + 16u)];
                            let _e298 = global.member[(_e198.member_2 + 17u)];
                            let _e303 = global.member[(_e198.member_2 + 18u)];
                            let _e308 = global.member[(_e198.member_2 + 19u)];
                            let _e314 = global.member[(_e198.member_2 + 20u)];
                            let _e319 = global.member[(_e198.member_2 + 21u)];
                            let _e324 = global.member[(_e198.member_2 + 22u)];
                            let _e329 = global.member[(_e198.member_2 + 23u)];
                            let _e335 = global.member[(_e198.member_2 + 24u)];
                            let _e340 = global.member[(_e198.member_2 + 25u)];
                            let _e345 = global.member[(_e198.member_2 + 26u)];
                            let _e350 = global.member[(_e198.member_2 + 27u)];
                            let _e356 = global.member[(_e198.member_2 + 28u)];
                            let _e361 = global.member[(_e198.member_2 + 29u)];
                            let _e366 = global.member[(_e198.member_2 + 30u)];
                            let _e371 = global.member[(_e198.member_2 + 31u)];
                            let _e378 = global.member[(_e198.member_2 + 32u)];
                            let _e383 = global.member[(_e198.member_2 + 33u)];
                            let _e388 = global.member[(_e198.member_2 + 34u)];
                            local_7 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            phi_810_ = type_12(0u, 6u);
                            loop {
                                let _e393 = phi_810_;
                                if (_e393.member < _e393.member_1) {
                                    phi_811_ = type_12((_e393.member + 1u), _e393.member_1);
                                    phi_834_ = type_12(1u, _e393.member);
                                } else {
                                    phi_811_ = _e393;
                                    phi_834_ = type_12(0u, type_12().member_1);
                                }
                                let _e406 = phi_811_;
                                let _e408 = phi_834_;
                                switch bitcast<i32>(_e408.member) {
                                    case 0: {
                                        phi_861_ = false;
                                        break;
                                    }
                                    case 1: {
                                        let _e413 = ((_e198.member_2 + 35u) + (_e408.member_1 * 4u));
                                        let _e416 = global.member[_e413];
                                        let _e421 = global.member[(_e413 + 1u)];
                                        let _e426 = global.member[(_e413 + 2u)];
                                        let _e431 = global.member[(_e413 + 3u)];
                                        local_7[_e408.member_1] = vec4<f32>(bitcast<f32>(_e416), bitcast<f32>(_e421), bitcast<f32>(_e426), bitcast<f32>(_e431));
                                        phi_861_ = true;
                                        break;
                                    }
                                    default: {
                                        phi_861_ = bool();
                                        break;
                                    }
                                }
                                let _e436 = phi_861_;
                                continue;
                                continuing {
                                    phi_810_ = _e406;
                                    break if !(_e436);
                                }
                            }
                            let _e438 = local_7;
                            local_6 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            phi_867_ = type_12(0u, 8u);
                            loop {
                                let _e441 = phi_867_;
                                if (_e441.member < _e441.member_1) {
                                    phi_868_ = type_12((_e441.member + 1u), _e441.member_1);
                                    phi_891_ = type_12(1u, _e441.member);
                                } else {
                                    phi_868_ = _e441;
                                    phi_891_ = type_12(0u, type_12().member_1);
                                }
                                let _e454 = phi_868_;
                                let _e456 = phi_891_;
                                switch bitcast<i32>(_e456.member) {
                                    case 0: {
                                        phi_914_ = false;
                                        break;
                                    }
                                    case 1: {
                                        let _e461 = ((_e198.member_2 + 59u) + (_e456.member_1 * 3u));
                                        let _e464 = global.member[_e461];
                                        let _e469 = global.member[(_e461 + 1u)];
                                        let _e474 = global.member[(_e461 + 2u)];
                                        local_6[_e456.member_1] = vec3<f32>(bitcast<f32>(_e464), bitcast<f32>(_e469), bitcast<f32>(_e474));
                                        phi_914_ = true;
                                        break;
                                    }
                                    default: {
                                        phi_914_ = bool();
                                        break;
                                    }
                                }
                                let _e479 = phi_914_;
                                continue;
                                continuing {
                                    phi_867_ = _e454;
                                    break if !(_e479);
                                }
                            }
                            let _e481 = local_6;
                            let _e485 = global.member[(_e198.member_2 + 83u)];
                            let _e490 = global.member[(_e198.member_2 + 84u)];
                            let _e495 = global.member[(_e198.member_2 + 85u)];
                            phi_935_ = type_19(type_17(vec4<f32>(bitcast<f32>(_e208), bitcast<f32>(_e213), bitcast<f32>(_e218), bitcast<f32>(_e223)), vec4<f32>(bitcast<f32>(_e229), bitcast<f32>(_e234), bitcast<f32>(_e239), bitcast<f32>(_e244)), vec4<f32>(bitcast<f32>(_e250), bitcast<f32>(_e255), bitcast<f32>(_e260), bitcast<f32>(_e265)), vec4<f32>(bitcast<f32>(_e271), bitcast<f32>(_e276), bitcast<f32>(_e281), bitcast<f32>(_e286))), type_17(vec4<f32>(bitcast<f32>(_e293), bitcast<f32>(_e298), bitcast<f32>(_e303), bitcast<f32>(_e308)), vec4<f32>(bitcast<f32>(_e314), bitcast<f32>(_e319), bitcast<f32>(_e324), bitcast<f32>(_e329)), vec4<f32>(bitcast<f32>(_e335), bitcast<f32>(_e340), bitcast<f32>(_e345), bitcast<f32>(_e350)), vec4<f32>(bitcast<f32>(_e356), bitcast<f32>(_e361), bitcast<f32>(_e366), bitcast<f32>(_e371))), vec3<f32>(bitcast<f32>(_e378), bitcast<f32>(_e383), bitcast<f32>(_e388)), type_18(_e481, _e438, vec3<f32>(bitcast<f32>(_e485), bitcast<f32>(_e490), bitcast<f32>(_e495))));
                        } else {
                            phi_935_ = type_19(type_17(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_17(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_18(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
                        }
                        let _e501 = phi_935_;
                        if _e155 {
                            phi_2120_ = (_e144 <= (_e93 - 10u));
                        } else {
                            phi_2120_ = false;
                        }
                        let _e510 = phi_2120_;
                        if _e510 {
                            let _e513 = global.member[_e144];
                            let _e518 = global.member[(_e144 + 1u)];
                            let _e523 = global.member[(_e144 + 2u)];
                            let _e529 = global.member[(_e144 + 3u)];
                            let _e534 = global.member[(_e144 + 4u)];
                            let _e539 = global.member[(_e144 + 5u)];
                            let _e544 = global.member[(_e144 + 6u)];
                            let _e550 = global.member[(_e144 + 7u)];
                            let _e555 = global.member[(_e144 + 8u)];
                            let _e560 = global.member[(_e144 + 9u)];
                            phi_986_ = type_32(vec3<f32>(bitcast<f32>(_e513), bitcast<f32>(_e518), bitcast<f32>(_e523)), vec4<f32>(bitcast<f32>(_e529), bitcast<f32>(_e534), bitcast<f32>(_e539), bitcast<f32>(_e544)), vec3<f32>(bitcast<f32>(_e550), bitcast<f32>(_e555), bitcast<f32>(_e560)));
                        } else {
                            phi_986_ = type_32(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                        }
                        let _e565 = phi_986_;
                        let _e573 = (_e565.member_1.x + _e565.member_1.x);
                        let _e574 = (_e565.member_1.y + _e565.member_1.y);
                        let _e575 = (_e565.member_1.z + _e565.member_1.z);
                        let _e577 = (_e565.member_1.z * _e575);
                        let _e578 = (_e565.member_1.w * _e573);
                        let _e579 = (_e565.member_1.w * _e574);
                        let _e580 = (_e565.member_1.w * _e575);
                        let _e600 = (vec4<f32>((1f - fma(_e565.member_1.y, _e574, _e577)), fma(_e565.member_1.x, _e574, _e580), fma(_e565.member_1.x, _e575, -(_e579)), 0f) * _e565.member_2.x);
                        let _e602 = (vec4<f32>(fma(_e565.member_1.x, _e574, -(_e580)), (1f - fma(_e565.member_1.x, _e573, _e577)), fma(_e565.member_1.y, _e575, _e578), 0f) * _e565.member_2.y);
                        let _e604 = (vec4<f32>(fma(_e565.member_1.x, _e575, _e579), fma(_e565.member_1.y, _e575, -(_e578)), (1f - fma(_e565.member_1.x, _e573, (_e565.member_1.y * _e574))), 0f) * _e565.member_2.z);
                        let _e626 = (_e565.member.x + fma(_e604.x, _e127, fma(_e602.x, _e122, (_e600.x * _e117))));
                        let _e627 = (_e565.member.y + fma(_e604.y, _e127, fma(_e602.y, _e122, (_e600.y * _e117))));
                        let _e628 = (_e565.member.z + fma(_e604.z, _e127, fma(_e602.z, _e122, (_e600.z * _e117))));
                        let _e629 = vec3<f32>(_e626, _e627, _e628);
                        let _e632 = (max(_e565.member_2.x, max(_e565.member_2.y, _e565.member_2.z)) * _e132);
                        let _e634 = sqrt((_e632 * _e632));
                        local_1 = _e501.member_3.member;
                        local = _e501.member_3.member_1;
                        let _e637 = local[0u][0u];
                        let _e640 = local[0u][1u];
                        let _e645 = local[0u][2u];
                        let _e649 = local[0u][3u];
                        let _e651 = -(_e634);
                        if ((fma(_e645, _e628, fma(_e637, _e626, (_e640 * _e627))) + _e649) < _e651) {
                            phi_1417_ = type_33(true, 0u);
                        } else {
                            phi_1118_ = type_12(0u, 6u);
                            loop {
                                let _e654 = phi_1118_;
                                if (_e654.member < _e654.member_1) {
                                    phi_1119_ = type_12((_e654.member + 1u), _e654.member_1);
                                    phi_1142_ = type_12(1u, _e654.member);
                                } else {
                                    phi_1119_ = _e654;
                                    phi_1142_ = type_12(0u, type_12().member_1);
                                }
                                let _e667 = phi_1119_;
                                let _e669 = phi_1142_;
                                local_10 = _e669.member_1;
                                switch bitcast<i32>(_e669.member) {
                                    case 0: {
                                        phi_1190_ = false;
                                        phi_1191_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if (_e669.member_1 != 0u) {
                                            if (_e669.member_1 < 6u) {
                                            } else {
                                                phi_2682_ = true;
                                                phi_2006_ = bool();
                                                break;
                                            }
                                            let _e677 = local[_e669.member_1][0u];
                                            let _e680 = local[_e669.member_1][1u];
                                            let _e685 = local[_e669.member_1][2u];
                                            let _e689 = local[_e669.member_1][3u];
                                            phi_1186_ = select(true, false, ((fma(_e685, _e628, fma(_e677, _e626, (_e680 * _e627))) + _e689) < _e651));
                                        } else {
                                            phi_1186_ = true;
                                        }
                                        let _e694 = phi_1186_;
                                        phi_1190_ = _e694;
                                        phi_1191_ = false;
                                        break;
                                    }
                                    default: {
                                        phi_1190_ = bool();
                                        phi_1191_ = bool();
                                        break;
                                    }
                                }
                                let _e696 = phi_1190_;
                                let _e698 = phi_1191_;
                                continue;
                                continuing {
                                    phi_1118_ = _e667;
                                    phi_2682_ = false;
                                    phi_2006_ = _e698;
                                    break if !(_e696);
                                }
                            }
                            let _e701 = phi_2682_;
                            let _e703 = phi_2006_;
                            if _e701 {
                                break;
                            }
                            if _e703 {
                                let _e704 = vec3(_e634);
                                let _e705 = (_e629 - _e704);
                                let _e706 = (_e629 + _e704);
                                phi_2709_ = _e701;
                                phi_1198_ = type_12(0u, 3u);
                                loop {
                                    let _e708 = phi_2709_;
                                    let _e710 = phi_1198_;
                                    if (_e710.member < _e710.member_1) {
                                        phi_1199_ = type_12((_e710.member + 1u), _e710.member_1);
                                        phi_1222_ = type_12(1u, _e710.member);
                                    } else {
                                        phi_1199_ = _e710;
                                        phi_1222_ = type_12(0u, type_12().member_1);
                                    }
                                    let _e723 = phi_1199_;
                                    let _e725 = phi_1222_;
                                    switch bitcast<i32>(_e725.member) {
                                        case 0: {
                                            phi_2716_ = _e708;
                                            phi_1383_ = false;
                                            phi_1384_ = true;
                                            phi_1385_ = false;
                                            phi_1386_ = false;
                                            break;
                                        }
                                        case 1: {
                                            phi_1232_ = type_12(0u, 8u);
                                            phi_1235_ = 0i;
                                            loop {
                                                let _e730 = phi_1232_;
                                                let _e732 = phi_1235_;
                                                local_8 = _e732;
                                                if (_e730.member < _e730.member_1) {
                                                    phi_1233_ = type_12((_e730.member + 1u), _e730.member_1);
                                                    phi_1258_ = type_12(1u, _e730.member);
                                                } else {
                                                    phi_1233_ = _e730;
                                                    phi_1258_ = type_12(0u, type_12().member_1);
                                                }
                                                let _e745 = phi_1233_;
                                                let _e747 = phi_1258_;
                                                switch bitcast<i32>(_e747.member) {
                                                    case 0: {
                                                        phi_1236_ = i32();
                                                        phi_1300_ = false;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if (_e747.member_1 < 8u) {
                                                        } else {
                                                            phi_2703_ = true;
                                                            break;
                                                        }
                                                        let _e754 = local_1[_e747.member_1][0u];
                                                        let _e757 = local_1[_e747.member_1][1u];
                                                        let _e760 = local_1[_e747.member_1][2u];
                                                        local_2 = array<f32, 3>(_e754, _e757, _e760);
                                                        let _e762 = (_e725.member_1 < 3u);
                                                        if _e762 {
                                                        } else {
                                                            phi_2703_ = true;
                                                            break;
                                                        }
                                                        let _e764 = local_2[_e725.member_1];
                                                        local_3 = array<f32, 3>(_e705.x, _e705.y, _e705.z);
                                                        if _e762 {
                                                        } else {
                                                            phi_2703_ = true;
                                                            break;
                                                        }
                                                        let _e770 = local_3[_e725.member_1];
                                                        if (_e764 < _e770) {
                                                            phi_1299_ = (_e732 + 1i);
                                                        } else {
                                                            phi_1299_ = _e732;
                                                        }
                                                        let _e774 = phi_1299_;
                                                        phi_1236_ = _e774;
                                                        phi_1300_ = true;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_1236_ = i32();
                                                        phi_1300_ = bool();
                                                        break;
                                                    }
                                                }
                                                let _e776 = phi_1236_;
                                                let _e778 = phi_1300_;
                                                continue;
                                                continuing {
                                                    phi_1232_ = _e745;
                                                    phi_1235_ = _e776;
                                                    phi_2703_ = _e708;
                                                    break if !(_e778);
                                                }
                                            }
                                            let _e781 = phi_2703_;
                                            phi_2715_ = _e781;
                                            phi_2009_ = bool();
                                            phi_2008_ = bool();
                                            phi_2007_ = bool();
                                            if _e781 {
                                                break;
                                            }
                                            let _e783 = local_8;
                                            let _e784 = (_e783 == 8i);
                                            if _e784 {
                                                phi_2717_ = _e781;
                                                phi_1381_ = false;
                                                phi_1382_ = false;
                                            } else {
                                                phi_1307_ = type_12(0u, 8u);
                                                phi_1310_ = 0i;
                                                loop {
                                                    let _e786 = phi_1307_;
                                                    let _e788 = phi_1310_;
                                                    local_9 = _e788;
                                                    if (_e786.member < _e786.member_1) {
                                                        phi_1308_ = type_12((_e786.member + 1u), _e786.member_1);
                                                        phi_1333_ = type_12(1u, _e786.member);
                                                    } else {
                                                        phi_1308_ = _e786;
                                                        phi_1333_ = type_12(0u, type_12().member_1);
                                                    }
                                                    let _e801 = phi_1308_;
                                                    let _e803 = phi_1333_;
                                                    switch bitcast<i32>(_e803.member) {
                                                        case 0: {
                                                            phi_1311_ = i32();
                                                            phi_1375_ = false;
                                                            break;
                                                        }
                                                        case 1: {
                                                            if (_e803.member_1 < 8u) {
                                                            } else {
                                                                phi_2710_ = true;
                                                                break;
                                                            }
                                                            let _e810 = local_1[_e803.member_1][0u];
                                                            let _e813 = local_1[_e803.member_1][1u];
                                                            let _e816 = local_1[_e803.member_1][2u];
                                                            local_4 = array<f32, 3>(_e810, _e813, _e816);
                                                            let _e818 = (_e725.member_1 < 3u);
                                                            if _e818 {
                                                            } else {
                                                                phi_2710_ = true;
                                                                break;
                                                            }
                                                            let _e820 = local_4[_e725.member_1];
                                                            local_5 = array<f32, 3>(_e706.x, _e706.y, _e706.z);
                                                            if _e818 {
                                                            } else {
                                                                phi_2710_ = true;
                                                                break;
                                                            }
                                                            let _e826 = local_5[_e725.member_1];
                                                            if (_e820 > _e826) {
                                                                phi_1374_ = (_e788 + 1i);
                                                            } else {
                                                                phi_1374_ = _e788;
                                                            }
                                                            let _e830 = phi_1374_;
                                                            phi_1311_ = _e830;
                                                            phi_1375_ = true;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_1311_ = i32();
                                                            phi_1375_ = bool();
                                                            break;
                                                        }
                                                    }
                                                    let _e832 = phi_1311_;
                                                    let _e834 = phi_1375_;
                                                    continue;
                                                    continuing {
                                                        phi_1307_ = _e801;
                                                        phi_1310_ = _e832;
                                                        phi_2710_ = _e781;
                                                        break if !(_e834);
                                                    }
                                                }
                                                let _e837 = phi_2710_;
                                                phi_2715_ = _e837;
                                                phi_2009_ = bool();
                                                phi_2008_ = bool();
                                                phi_2007_ = bool();
                                                if _e837 {
                                                    break;
                                                }
                                                let _e839 = local_9;
                                                let _e840 = (_e839 == 8i);
                                                phi_2717_ = _e837;
                                                phi_1381_ = select(true, false, _e840);
                                                phi_1382_ = _e840;
                                            }
                                            let _e843 = phi_2717_;
                                            let _e845 = phi_1381_;
                                            let _e847 = phi_1382_;
                                            phi_2716_ = _e843;
                                            phi_1383_ = _e845;
                                            phi_1384_ = false;
                                            phi_1385_ = _e784;
                                            phi_1386_ = _e847;
                                            break;
                                        }
                                        default: {
                                            phi_2716_ = _e708;
                                            phi_1383_ = bool();
                                            phi_1384_ = bool();
                                            phi_1385_ = bool();
                                            phi_1386_ = bool();
                                            break;
                                        }
                                    }
                                    let _e849 = phi_2716_;
                                    let _e851 = phi_1383_;
                                    let _e853 = phi_1384_;
                                    let _e855 = phi_1385_;
                                    let _e857 = phi_1386_;
                                    continue;
                                    continuing {
                                        phi_2709_ = _e849;
                                        phi_1198_ = _e723;
                                        phi_2715_ = _e849;
                                        phi_2009_ = _e857;
                                        phi_2008_ = _e855;
                                        phi_2007_ = _e853;
                                        break if !(_e851);
                                    }
                                }
                                let _e860 = phi_2715_;
                                let _e862 = phi_2009_;
                                let _e864 = phi_2008_;
                                let _e866 = phi_2007_;
                                if _e860 {
                                    break;
                                }
                                let _e867 = select(_e864, false, _e866);
                                if select(true, false, select(_e867, true, select(select(_e862, false, _e866), false, _e867))) {
                                    phi_1409_ = type_33(false, 0u);
                                } else {
                                    phi_1409_ = type_33(true, 0u);
                                }
                                let _e873 = phi_1409_;
                                phi_1410_ = _e873;
                            } else {
                                phi_1410_ = type_33();
                            }
                            let _e875 = phi_1410_;
                            if select(true, false, _e703) {
                                let _e878 = local_10;
                                phi_1416_ = type_33(true, _e878);
                            } else {
                                phi_1416_ = _e875;
                            }
                            let _e881 = phi_1416_;
                            phi_1417_ = _e881;
                        }
                        let _e883 = phi_1417_;
                        if (_e883.member != true) {
                            global_2.member[_e98.x].member_1 = 1u;
                            if _e198.member_7 {
                                let _e889 = global_1.member[0u];
                                let _e892 = global_1.member[1u];
                                let _e895 = global_1.member[3u];
                                let _e898 = global_1.member[4u];
                                let _e899 = f32(_e889);
                                let _e941 = fma(_e501.member.member_3.z, _e501.member_1.member.w, fma(_e501.member.member_2.z, _e501.member_1.member.z, fma(_e501.member.member.z, _e501.member_1.member.x, (_e501.member.member_1.z * _e501.member_1.member.y))));
                                let _e942 = fma(_e501.member.member_3.w, _e501.member_1.member.w, fma(_e501.member.member_2.w, _e501.member_1.member.z, fma(_e501.member.member.w, _e501.member_1.member.x, (_e501.member.member_1.w * _e501.member_1.member.y))));
                                let _e962 = fma(_e501.member.member_3.z, _e501.member_1.member_1.w, fma(_e501.member.member_2.z, _e501.member_1.member_1.z, fma(_e501.member.member.z, _e501.member_1.member_1.x, (_e501.member.member_1.z * _e501.member_1.member_1.y))));
                                let _e963 = fma(_e501.member.member_3.w, _e501.member_1.member_1.w, fma(_e501.member.member_2.w, _e501.member_1.member_1.z, fma(_e501.member.member.w, _e501.member_1.member_1.x, (_e501.member.member_1.w * _e501.member_1.member_1.y))));
                                let _e983 = fma(_e501.member.member_3.z, _e501.member_1.member_2.w, fma(_e501.member.member_2.z, _e501.member_1.member_2.z, fma(_e501.member.member.z, _e501.member_1.member_2.x, (_e501.member.member_1.z * _e501.member_1.member_2.y))));
                                let _e984 = fma(_e501.member.member_3.w, _e501.member_1.member_2.w, fma(_e501.member.member_2.w, _e501.member_1.member_2.z, fma(_e501.member.member.w, _e501.member_1.member_2.x, (_e501.member.member_1.w * _e501.member_1.member_2.y))));
                                let _e1004 = fma(_e501.member.member_3.z, _e501.member_1.member_3.w, fma(_e501.member.member_2.z, _e501.member_1.member_3.z, fma(_e501.member.member.z, _e501.member_1.member_3.x, (_e501.member.member_1.z * _e501.member_1.member_3.y))));
                                let _e1005 = fma(_e501.member.member_3.w, _e501.member_1.member_3.w, fma(_e501.member.member_2.w, _e501.member_1.member_3.z, fma(_e501.member.member.w, _e501.member_1.member_3.x, (_e501.member.member_1.w * _e501.member_1.member_3.y))));
                                let _e1017 = (fma(_e984, _e628, fma(_e942, _e626, (_e963 * _e627))) + _e1005);
                                let _e1022 = fma(_e634, _e501.member_3.member_1[5].x, _e626);
                                let _e1023 = fma(_e634, _e501.member_3.member_1[5].y, _e627);
                                let _e1024 = fma(_e634, _e501.member_3.member_1[5].z, _e628);
                                let _e1038 = fma(_e634, _e501.member_3.member_1[0].x, _e626);
                                let _e1039 = fma(_e634, _e501.member_3.member_1[0].y, _e627);
                                let _e1040 = fma(_e634, _e501.member_3.member_1[0].z, _e628);
                                let _e1057 = (vec2<f32>(((((fma(fma(_e501.member.member_3.x, _e501.member_1.member_2.w, fma(_e501.member.member_2.x, _e501.member_1.member_2.z, fma(_e501.member.member.x, _e501.member_1.member_2.x, (_e501.member.member_1.x * _e501.member_1.member_2.y)))), _e628, fma(fma(_e501.member.member_3.x, _e501.member_1.member.w, fma(_e501.member.member_2.x, _e501.member_1.member.z, fma(_e501.member.member.x, _e501.member_1.member.x, (_e501.member.member_1.x * _e501.member_1.member.y)))), _e626, (fma(_e501.member.member_3.x, _e501.member_1.member_1.w, fma(_e501.member.member_2.x, _e501.member_1.member_1.z, fma(_e501.member.member.x, _e501.member_1.member_1.x, (_e501.member.member_1.x * _e501.member_1.member_1.y)))) * _e627))) + fma(_e501.member.member_3.x, _e501.member_1.member_3.w, fma(_e501.member.member_2.x, _e501.member_1.member_3.z, fma(_e501.member.member.x, _e501.member_1.member_3.x, (_e501.member.member_1.x * _e501.member_1.member_3.y))))) / _e1017) + 1f) * 0.5f), fma((-1f - ((fma(fma(_e501.member.member_3.y, _e501.member_1.member_2.w, fma(_e501.member.member_2.y, _e501.member_1.member_2.z, fma(_e501.member.member.y, _e501.member_1.member_2.x, (_e501.member.member_1.y * _e501.member_1.member_2.y)))), _e628, fma(fma(_e501.member.member_3.y, _e501.member_1.member.w, fma(_e501.member.member_2.y, _e501.member_1.member.z, fma(_e501.member.member.y, _e501.member_1.member.x, (_e501.member.member_1.y * _e501.member_1.member.y)))), _e626, (fma(_e501.member.member_3.y, _e501.member_1.member_1.w, fma(_e501.member.member_2.y, _e501.member_1.member_1.z, fma(_e501.member.member.y, _e501.member_1.member_1.x, (_e501.member.member_1.y * _e501.member_1.member_1.y)))) * _e627))) + fma(_e501.member.member_3.y, _e501.member_1.member_3.w, fma(_e501.member.member_2.y, _e501.member_1.member_3.z, fma(_e501.member.member.y, _e501.member_1.member_3.x, (_e501.member.member_1.y * _e501.member_1.member_3.y))))) / _e1017)), 0.5f, 1f)) * vec2<f32>(_e899, f32(_e892)));
                                let _e1058 = (_e634 / _e1017);
                                let _e1060 = -(_e899);
                                let _e1064 = vec3<f32>(fma(_e1060, _e1058, _e1057.x), fma(_e1060, _e1058, _e1057.y), ((_e1004 + fma(_e983, _e1024, fma(_e962, _e1023, (_e941 * _e1022)))) / (_e1005 + fma(_e984, _e1024, fma(_e963, _e1023, (_e942 * _e1022))))));
                                let _e1067 = vec3<f32>(fma(_e899, _e1058, _e1057.x), fma(_e899, _e1058, _e1057.y), ((_e1004 + fma(_e983, _e1040, fma(_e962, _e1039, (_e941 * _e1038)))) / (_e1005 + fma(_e984, _e1040, fma(_e963, _e1039, (_e942 * _e1038))))));
                                let _e1068 = min(_e1064, _e1067);
                                let _e1069 = max(_e1064, _e1067);
                                let _e1074 = (_e1069.x - _e1068.x);
                                let _e1075 = (_e1069.y - _e1068.y);
                                let _e1079 = floor(log2(select(_e1075, _e1074, (_e1074 > _e1075))));
                                let _e1084 = select(select(u32(_e1079), 0u, (_e1079 < 0f)), 4294967295u, (_e1079 > 4294967000f));
                                let _e1085 = (_e898 - 1u);
                                let _e1087 = select(_e1084, _e1085, (_e1084 > _e1085));
                                let _e1093 = round(((_e1068.x + _e1069.x) * 0.5f));
                                let _e1099 = (_e1087 & 31u);
                                let _e1102 = round(((_e1068.y + _e1069.y) * 0.5f));
                                if (_e1087 >= _e898) {
                                    phi_1681_ = 4294967295u;
                                } else {
                                    phi_1681_ = (_e895 + (2u * _e1087));
                                }
                                let _e1114 = phi_1681_;
                                if (_e95 >= 2u) {
                                    phi_2503_ = (_e1114 <= (_e95 - 2u));
                                } else {
                                    phi_2503_ = false;
                                }
                                let _e1119 = phi_2503_;
                                if _e1119 {
                                    let _e1122 = global_1.member[_e1114];
                                    let _e1126 = global_1.member[(_e1114 + 1u)];
                                    phi_1699_ = type_12(_e1122, _e1126);
                                } else {
                                    phi_1699_ = type_12(4294967295u, 0u);
                                }
                                let _e1129 = phi_1699_;
                                let _e1135 = (((select(select(u32(_e1102), 0u, (_e1102 < 0f)), 4294967295u, (_e1102 > 4294967000f)) >> bitcast<u32>(_e1099)) * (_e889 >> bitcast<u32>(_e1099))) + (select(select(u32(_e1093), 0u, (_e1093 < 0f)), 4294967295u, (_e1093 > 4294967000f)) >> bitcast<u32>(_e1099)));
                                if (_e1135 >= _e1129.member_1) {
                                    phi_2529_ = 4294967295u;
                                } else {
                                    phi_2529_ = (_e1129.member + _e1135);
                                }
                                let _e1139 = phi_2529_;
                                let _e1142 = global_1.member[_e1139];
                                if select((_e1068.z > 1f), true, (_e1068.z > bitcast<f32>(_e1142))) {
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

@compute @workgroup_size(16, 1, 1) 
fn cullcompute_culling(@builtin(global_invocation_id) param: vec3<u32>) {
    global_3 = param;
    function();
}
