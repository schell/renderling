struct type_3 {
    member: array<u32>,
}

struct type_14 {
    member: u32,
    member_1: u32,
}

struct type_21 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_22 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
    member_2: vec3<f32>,
}

struct type_23 {
    member: type_21,
    member_1: type_21,
    member_2: vec3<f32>,
    member_3: type_22,
}

struct type_27 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_30 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
    member_3: vec4<f32>,
    member_4: array<u32, 4>,
    member_5: array<f32, 4>,
    member_6: vec2<f32>,
    member_7: vec2<f32>,
}

struct type_33 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_34 {
    member: type_14,
    member_1: type_14,
}

struct VertexOutput {
    @location(0) @interpolate(flat) member: u32,
    @location(1) member_1: vec4<f32>,
    @location(2) member_2: vec2<f32>,
    @location(3) member_3: vec2<f32>,
    @location(4) member_4: vec3<f32>,
    @location(5) member_5: vec3<f32>,
    @location(6) member_6: vec3<f32>,
    @location(7) member_7: vec3<f32>,
    @builtin(position) member_8: vec4<f32>,
}

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: u32;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: u32;
var<private> global_4: type_21 = type_21(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
var<private> global_5: u32;
var<private> global_6: vec4<f32>;
var<private> global_7: vec2<f32>;
var<private> global_8: vec2<f32>;
var<private> global_9: vec3<f32>;
var<private> global_10: vec3<f32>;
var<private> global_11: vec3<f32>;
var<private> global_12: vec3<f32>;

fn function() {
    var local: array<f32, 4>;
    var local_1: array<u32, 4>;
    var local_2: array<vec3<f32>, 8>;
    var local_3: array<vec4<f32>, 6>;
    var local_4: array<f32, 4>;
    var local_5: array<u32, 4>;
    var phi_1156_: u32;
    var phi_2871_: bool;
    var phi_1163_: u32;
    var phi_1164_: u32;
    var phi_1174_: u32;
    var phi_1256_: type_14;
    var phi_1257_: type_14;
    var phi_1280_: type_14;
    var phi_1293_: bool;
    var phi_1299_: type_14;
    var phi_1300_: type_14;
    var phi_1323_: type_14;
    var phi_1337_: bool;
    var phi_1343_: type_14;
    var phi_1346_: type_30;
    var phi_1344_: type_14;
    var phi_1369_: type_14;
    var phi_1386_: u32;
    var phi_2901_: bool;
    var phi_1404_: type_14;
    var phi_2927_: u32;
    var phi_2946_: bool;
    var phi_1454_: type_33;
    var phi_1464_: u32;
    var phi_2968_: bool;
    var phi_1472_: f32;
    var phi_1347_: type_30;
    var phi_1525_: bool;
    var phi_2988_: bool;
    var phi_1621_: type_34;
    var local_6: type_30;
    var phi_1624_: type_14;
    var phi_1627_: type_21;
    var phi_1625_: type_14;
    var phi_1650_: type_14;
    var local_7: type_30;
    var phi_1674_: u32;
    var phi_3022_: bool;
    var phi_1683_: u32;
    var phi_3046_: bool;
    var phi_1732_: type_27;
    var phi_1742_: u32;
    var phi_3071_: bool;
    var phi_1815_: type_21;
    var phi_1628_: type_21;
    var phi_2052_: bool;
    var phi_3887_: bool;
    var local_8: type_21;
    var local_9: type_21;
    var local_10: type_21;
    var local_11: type_21;
    var phi_2079_: bool;
    var phi_2081_: bool;
    var phi_2082_: bool;
    var phi_2083_: bool;
    var phi_2084_: bool;
    var local_12: type_21;
    var local_13: type_21;
    var local_14: type_21;
    var local_15: type_21;
    var phi_2118_: bool;
    var phi_2120_: bool;
    var phi_2121_: bool;
    var phi_2122_: bool;
    var phi_2123_: bool;
    var local_16: type_21;
    var local_17: type_21;
    var local_18: type_21;
    var local_19: type_21;
    var phi_2157_: bool;
    var phi_2159_: bool;
    var phi_2160_: bool;
    var phi_2161_: bool;
    var phi_2162_: bool;
    var local_20: type_21;
    var local_21: type_21;
    var local_22: type_21;
    var local_23: type_21;
    var phi_2196_: bool;
    var phi_2198_: bool;
    var phi_2199_: bool;
    var phi_2200_: bool;
    var phi_2201_: bool;
    var phi_2206_: bool;
    var phi_2208_: bool;
    var phi_2209_: bool;
    var phi_2210_: bool;
    var phi_2211_: bool;
    var phi_2219_: type_21;
    var phi_3206_: bool;
    var phi_3271_: vec4<f32>;
    var phi_3301_: vec4<f32>;
    var phi_3303_: vec4<f32>;
    var phi_3312_: type_27;
    var phi_3313_: type_27;
    var phi_3318_: type_27;
    var phi_3319_: type_27;
    var phi_3320_: bool;
    var phi_3324_: type_27;
    var phi_2221_: type_27;
    var phi_2223_: type_27;
    var phi_2224_: bool;
    var phi_3418_: bool;
    var phi_2277_: type_27;
    var phi_2278_: type_27;
    var local_24: type_30;
    var local_25: type_30;
    var local_26: type_30;
    var local_27: type_30;
    var local_28: type_30;
    var phi_2365_: vec3<f32>;
    var local_29: type_30;
    var phi_3509_: vec3<f32>;
    var phi_3544_: vec3<f32>;
    var phi_3579_: vec3<f32>;
    var local_30: type_30;
    var phi_3592_: bool;
    var phi_2649_: type_14;
    var phi_2650_: type_14;
    var phi_2673_: type_14;
    var phi_2700_: bool;
    var phi_2706_: type_14;
    var phi_2707_: type_14;
    var phi_2730_: type_14;
    var phi_2753_: bool;
    var phi_2774_: type_23;
    var local_31: type_21;

    switch bitcast<i32>(0u) {
        default: {
            let _e98 = global_3;
            let _e99 = global_1;
            let _e101 = arrayLength((&global.member));
            let _e104 = global.member[_e98];
            let _e109 = global.member[(_e98 + 1u)];
            let _e113 = global.member[(_e98 + 2u)];
            let _e117 = global.member[(_e98 + 7u)];
            let _e121 = global.member[(_e98 + 8u)];
            let _e125 = global.member[(_e98 + 9u)];
            let _e129 = global.member[(_e98 + 10u)];
            let _e133 = global.member[(_e98 + 12u)];
            let _e137 = global.member[(_e98 + 13u)];
            let _e141 = global.member[(_e98 + 14u)];
            let _e145 = global.member[(_e98 + 15u)];
            let _e149 = global.member[(_e98 + 16u)];
            let _e153 = global.member[(_e98 + 17u)];
            if (_e104 == 1u) {
                global_5 = _e98;
                if (_e117 == 4294967295u) {
                    phi_1164_ = _e99;
                } else {
                    if (_e99 >= _e121) {
                        phi_1156_ = 4294967295u;
                    } else {
                        phi_1156_ = (_e117 + _e99);
                    }
                    let _e158 = phi_1156_;
                    if (_e101 >= 1u) {
                        phi_2871_ = (_e158 <= (_e101 - 1u));
                    } else {
                        phi_2871_ = false;
                    }
                    let _e163 = phi_2871_;
                    if _e163 {
                        let _e166 = global.member[_e158];
                        phi_1163_ = _e166;
                    } else {
                        phi_1163_ = 0u;
                    }
                    let _e168 = phi_1163_;
                    phi_1164_ = _e168;
                }
                let _e170 = phi_1164_;
                if (_e170 >= _e113) {
                    phi_1174_ = 4294967295u;
                } else {
                    phi_1174_ = (_e109 + (26u * _e170));
                }
                let _e175 = phi_1174_;
                let _e178 = global.member[_e175];
                let _e183 = global.member[(_e175 + 1u)];
                let _e188 = global.member[(_e175 + 2u)];
                let _e194 = global.member[(_e175 + 3u)];
                let _e199 = global.member[(_e175 + 4u)];
                let _e204 = global.member[(_e175 + 5u)];
                let _e209 = global.member[(_e175 + 6u)];
                let _e215 = global.member[(_e175 + 7u)];
                let _e220 = global.member[(_e175 + 8u)];
                let _e226 = global.member[(_e175 + 9u)];
                let _e231 = global.member[(_e175 + 10u)];
                let _e237 = global.member[(_e175 + 11u)];
                let _e242 = global.member[(_e175 + 12u)];
                let _e247 = global.member[(_e175 + 13u)];
                let _e253 = global.member[(_e175 + 14u)];
                let _e258 = global.member[(_e175 + 15u)];
                let _e263 = global.member[(_e175 + 16u)];
                let _e268 = global.member[(_e175 + 17u)];
                local_5 = array<u32, 4>(0u, 0u, 0u, 0u);
                phi_1256_ = type_14(0u, 4u);
                loop {
                    let _e273 = phi_1256_;
                    if (_e273.member < _e273.member_1) {
                        phi_1257_ = type_14((_e273.member + 1u), _e273.member_1);
                        phi_1280_ = type_14(1u, _e273.member);
                    } else {
                        phi_1257_ = _e273;
                        phi_1280_ = type_14(0u, type_14().member_1);
                    }
                    let _e286 = phi_1257_;
                    let _e288 = phi_1280_;
                    switch bitcast<i32>(_e288.member) {
                        case 0: {
                            phi_1293_ = false;
                            break;
                        }
                        case 1: {
                            let _e295 = global.member[((_e175 + 18u) + _e288.member_1)];
                            local_5[_e288.member_1] = _e295;
                            phi_1293_ = true;
                            break;
                        }
                        default: {
                            phi_1293_ = bool();
                            break;
                        }
                    }
                    let _e298 = phi_1293_;
                    continue;
                    continuing {
                        phi_1256_ = _e286;
                        break if !(_e298);
                    }
                }
                let _e300 = local_5;
                local_4 = array<f32, 4>(0f, 0f, 0f, 0f);
                phi_1299_ = type_14(0u, 4u);
                loop {
                    let _e303 = phi_1299_;
                    if (_e303.member < _e303.member_1) {
                        phi_1300_ = type_14((_e303.member + 1u), _e303.member_1);
                        phi_1323_ = type_14(1u, _e303.member);
                    } else {
                        phi_1300_ = _e303;
                        phi_1323_ = type_14(0u, type_14().member_1);
                    }
                    let _e316 = phi_1300_;
                    let _e318 = phi_1323_;
                    switch bitcast<i32>(_e318.member) {
                        case 0: {
                            phi_1337_ = false;
                            break;
                        }
                        case 1: {
                            let _e325 = global.member[((_e175 + 22u) + _e318.member_1)];
                            local_4[_e318.member_1] = bitcast<f32>(_e325);
                            phi_1337_ = true;
                            break;
                        }
                        default: {
                            phi_1337_ = bool();
                            break;
                        }
                    }
                    let _e329 = phi_1337_;
                    continue;
                    continuing {
                        phi_1299_ = _e316;
                        break if !(_e329);
                    }
                }
                let _e331 = local_4;
                phi_1343_ = type_14(0u, _e141);
                phi_1346_ = type_30(vec3<f32>(bitcast<f32>(_e178), bitcast<f32>(_e183), bitcast<f32>(_e188)), vec4<f32>(bitcast<f32>(_e194), bitcast<f32>(_e199), bitcast<f32>(_e204), bitcast<f32>(_e209)), vec3<f32>(bitcast<f32>(_e237), bitcast<f32>(_e242), bitcast<f32>(_e247)), vec4<f32>(bitcast<f32>(_e253), bitcast<f32>(_e258), bitcast<f32>(_e263), bitcast<f32>(_e268)), _e300, _e331, vec2<f32>(bitcast<f32>(_e215), bitcast<f32>(_e220)), vec2<f32>(bitcast<f32>(_e226), bitcast<f32>(_e231)));
                loop {
                    let _e335 = phi_1343_;
                    let _e337 = phi_1346_;
                    local_6 = _e337;
                    local_7 = _e337;
                    local_24 = _e337;
                    local_25 = _e337;
                    local_26 = _e337;
                    local_27 = _e337;
                    local_28 = _e337;
                    local_29 = _e337;
                    local_30 = _e337;
                    if (_e335.member < _e335.member_1) {
                        phi_1344_ = type_14((_e335.member + 1u), _e335.member_1);
                        phi_1369_ = type_14(1u, _e335.member);
                    } else {
                        phi_1344_ = _e335;
                        phi_1369_ = type_14(0u, type_14().member_1);
                    }
                    let _e350 = phi_1344_;
                    let _e352 = phi_1369_;
                    switch bitcast<i32>(_e352.member) {
                        case 0: {
                            phi_1347_ = type_30();
                            phi_1525_ = false;
                            break;
                        }
                        case 1: {
                            if (_e352.member_1 >= _e141) {
                                phi_1386_ = 4294967295u;
                            } else {
                                phi_1386_ = (_e137 + (2u * _e352.member_1));
                            }
                            let _e360 = phi_1386_;
                            if (_e101 >= 2u) {
                                phi_2901_ = (_e360 <= (_e101 - 2u));
                            } else {
                                phi_2901_ = false;
                            }
                            let _e365 = phi_2901_;
                            if _e365 {
                                let _e368 = global.member[_e360];
                                let _e372 = global.member[(_e360 + 1u)];
                                phi_1404_ = type_14(_e368, _e372);
                            } else {
                                phi_1404_ = type_14(4294967295u, 0u);
                            }
                            let _e375 = phi_1404_;
                            if (_e170 >= _e375.member_1) {
                                phi_2927_ = 4294967295u;
                            } else {
                                phi_2927_ = (_e375.member + (9u * _e170));
                            }
                            let _e382 = phi_2927_;
                            if (_e101 >= 9u) {
                                phi_2946_ = (_e382 <= (_e101 - 9u));
                            } else {
                                phi_2946_ = false;
                            }
                            let _e387 = phi_2946_;
                            if _e387 {
                                let _e390 = global.member[_e382];
                                let _e395 = global.member[(_e382 + 1u)];
                                let _e400 = global.member[(_e382 + 2u)];
                                let _e406 = global.member[(_e382 + 3u)];
                                let _e411 = global.member[(_e382 + 4u)];
                                let _e416 = global.member[(_e382 + 5u)];
                                let _e422 = global.member[(_e382 + 6u)];
                                let _e427 = global.member[(_e382 + 7u)];
                                let _e432 = global.member[(_e382 + 8u)];
                                phi_1454_ = type_33(vec3<f32>(bitcast<f32>(_e390), bitcast<f32>(_e395), bitcast<f32>(_e400)), vec3<f32>(bitcast<f32>(_e406), bitcast<f32>(_e411), bitcast<f32>(_e416)), vec3<f32>(bitcast<f32>(_e422), bitcast<f32>(_e427), bitcast<f32>(_e432)));
                            } else {
                                phi_1454_ = type_33(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e437 = phi_1454_;
                            if (_e352.member_1 >= _e149) {
                                phi_1464_ = 4294967295u;
                            } else {
                                phi_1464_ = (_e145 + _e352.member_1);
                            }
                            let _e441 = phi_1464_;
                            if (_e101 >= 1u) {
                                phi_2968_ = (_e441 <= (_e101 - 1u));
                            } else {
                                phi_2968_ = false;
                            }
                            let _e446 = phi_2968_;
                            if _e446 {
                                let _e449 = global.member[_e441];
                                phi_1472_ = bitcast<f32>(_e449);
                            } else {
                                phi_1472_ = 0f;
                            }
                            let _e452 = phi_1472_;
                            let _e475 = type_30(vec3<f32>(fma(_e452, _e437.member.x, _e337.member.x), fma(_e452, _e437.member.y, _e337.member.y), fma(_e452, _e437.member.z, _e337.member.z)), _e337.member_1, _e337.member_2, _e337.member_3, _e337.member_4, _e337.member_5, _e337.member_6, _e337.member_7);
                            let _e498 = type_30(_e475.member, _e475.member_1, vec3<f32>(fma(_e452, _e437.member_1.x, _e337.member_2.x), fma(_e452, _e437.member_1.y, _e337.member_2.y), fma(_e452, _e437.member_1.z, _e337.member_2.z)), _e475.member_3, _e475.member_4, _e475.member_5, _e475.member_6, _e475.member_7);
                            phi_1347_ = type_30(_e498.member, _e498.member_1, _e498.member_2, vec4<f32>(fma(_e452, _e437.member_2.x, _e337.member_3.x), fma(_e452, _e437.member_2.y, _e337.member_3.y), fma(_e452, _e437.member_2.z, _e337.member_3.z), _e337.member_3.w), _e498.member_4, _e498.member_5, _e498.member_6, _e498.member_7);
                            phi_1525_ = true;
                            break;
                        }
                        default: {
                            phi_1347_ = type_30();
                            phi_1525_ = bool();
                            break;
                        }
                    }
                    let _e525 = phi_1347_;
                    let _e527 = phi_1525_;
                    continue;
                    continuing {
                        phi_1343_ = _e350;
                        phi_1346_ = _e525;
                        break if !(_e527);
                    }
                }
                let _e532 = global.member[(_e153 + 6u)];
                if (_e532 == 1u) {
                    let _e535 = ((_e133 == 4294967295u) != true);
                    if _e535 {
                        if (_e101 >= 4u) {
                            phi_2988_ = (_e133 <= (_e101 - 4u));
                        } else {
                            phi_2988_ = false;
                        }
                        let _e540 = phi_2988_;
                        if _e540 {
                            let _e543 = global.member[_e133];
                            let _e547 = global.member[(_e133 + 1u)];
                            let _e551 = global.member[(_e133 + 2u)];
                            let _e555 = global.member[(_e133 + 3u)];
                            phi_1621_ = type_34(type_14(_e543, _e547), type_14(_e551, _e555));
                        } else {
                            phi_1621_ = type_34(type_14(4294967295u, 0u), type_14(4294967295u, 0u));
                        }
                        let _e560 = phi_1621_;
                        let _e562 = local_6;
                        local = _e562.member_5;
                        phi_1624_ = type_14(0u, 4u);
                        phi_1627_ = type_21(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        loop {
                            let _e565 = phi_1624_;
                            let _e567 = phi_1627_;
                            local_8 = _e567;
                            local_9 = _e567;
                            local_10 = _e567;
                            local_11 = _e567;
                            local_12 = _e567;
                            local_13 = _e567;
                            local_14 = _e567;
                            local_15 = _e567;
                            local_16 = _e567;
                            local_17 = _e567;
                            local_18 = _e567;
                            local_19 = _e567;
                            local_20 = _e567;
                            local_21 = _e567;
                            local_22 = _e567;
                            local_23 = _e567;
                            local_31 = _e567;
                            if (_e565.member < _e565.member_1) {
                                phi_1625_ = type_14((_e565.member + 1u), _e565.member_1);
                                phi_1650_ = type_14(1u, _e565.member);
                            } else {
                                phi_1625_ = _e565;
                                phi_1650_ = type_14(0u, type_14().member_1);
                            }
                            let _e580 = phi_1625_;
                            let _e582 = phi_1650_;
                            switch bitcast<i32>(_e582.member) {
                                case 0: {
                                    phi_1628_ = type_21();
                                    phi_2052_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e587 = local_7;
                                    local_1 = _e587.member_4;
                                    let _e589 = (_e582.member_1 < 4u);
                                    if _e589 {
                                    } else {
                                        phi_3887_ = true;
                                        break;
                                    }
                                    let _e591 = local_1[_e582.member_1];
                                    if (_e591 >= _e560.member.member_1) {
                                        phi_1674_ = 4294967295u;
                                    } else {
                                        phi_1674_ = (_e560.member.member + _e591);
                                    }
                                    let _e599 = phi_1674_;
                                    if (_e101 >= 1u) {
                                        phi_3022_ = (_e599 <= (_e101 - 1u));
                                    } else {
                                        phi_3022_ = false;
                                    }
                                    let _e604 = phi_3022_;
                                    if _e604 {
                                        let _e607 = global.member[_e599];
                                        phi_1683_ = _e607;
                                    } else {
                                        phi_1683_ = 4294967295u;
                                    }
                                    let _e609 = phi_1683_;
                                    if (_e101 >= 10u) {
                                        phi_3046_ = (_e609 <= (_e101 - 10u));
                                    } else {
                                        phi_3046_ = false;
                                    }
                                    let _e614 = phi_3046_;
                                    if _e614 {
                                        let _e617 = global.member[_e609];
                                        let _e622 = global.member[(_e609 + 1u)];
                                        let _e627 = global.member[(_e609 + 2u)];
                                        let _e633 = global.member[(_e609 + 3u)];
                                        let _e638 = global.member[(_e609 + 4u)];
                                        let _e643 = global.member[(_e609 + 5u)];
                                        let _e648 = global.member[(_e609 + 6u)];
                                        let _e654 = global.member[(_e609 + 7u)];
                                        let _e659 = global.member[(_e609 + 8u)];
                                        let _e664 = global.member[(_e609 + 9u)];
                                        phi_1732_ = type_27(vec3<f32>(bitcast<f32>(_e617), bitcast<f32>(_e622), bitcast<f32>(_e627)), vec4<f32>(bitcast<f32>(_e633), bitcast<f32>(_e638), bitcast<f32>(_e643), bitcast<f32>(_e648)), vec3<f32>(bitcast<f32>(_e654), bitcast<f32>(_e659), bitcast<f32>(_e664)));
                                    } else {
                                        phi_1732_ = type_27(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e669 = phi_1732_;
                                    if (_e591 >= _e560.member_1.member_1) {
                                        phi_1742_ = 4294967295u;
                                    } else {
                                        phi_1742_ = (_e560.member_1.member + (16u * _e591));
                                    }
                                    let _e678 = phi_1742_;
                                    if (_e101 >= 16u) {
                                        phi_3071_ = (_e678 <= (_e101 - 16u));
                                    } else {
                                        phi_3071_ = false;
                                    }
                                    let _e683 = phi_3071_;
                                    if _e683 {
                                        let _e686 = global.member[_e678];
                                        let _e691 = global.member[(_e678 + 1u)];
                                        let _e696 = global.member[(_e678 + 2u)];
                                        let _e701 = global.member[(_e678 + 3u)];
                                        let _e707 = global.member[(_e678 + 4u)];
                                        let _e712 = global.member[(_e678 + 5u)];
                                        let _e717 = global.member[(_e678 + 6u)];
                                        let _e722 = global.member[(_e678 + 7u)];
                                        let _e728 = global.member[(_e678 + 8u)];
                                        let _e733 = global.member[(_e678 + 9u)];
                                        let _e738 = global.member[(_e678 + 10u)];
                                        let _e743 = global.member[(_e678 + 11u)];
                                        let _e749 = global.member[(_e678 + 12u)];
                                        let _e754 = global.member[(_e678 + 13u)];
                                        let _e759 = global.member[(_e678 + 14u)];
                                        let _e764 = global.member[(_e678 + 15u)];
                                        phi_1815_ = type_21(vec4<f32>(bitcast<f32>(_e686), bitcast<f32>(_e691), bitcast<f32>(_e696), bitcast<f32>(_e701)), vec4<f32>(bitcast<f32>(_e707), bitcast<f32>(_e712), bitcast<f32>(_e717), bitcast<f32>(_e722)), vec4<f32>(bitcast<f32>(_e728), bitcast<f32>(_e733), bitcast<f32>(_e738), bitcast<f32>(_e743)), vec4<f32>(bitcast<f32>(_e749), bitcast<f32>(_e754), bitcast<f32>(_e759), bitcast<f32>(_e764)));
                                    } else {
                                        phi_1815_ = type_21(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                    }
                                    let _e769 = phi_1815_;
                                    let _e777 = (_e669.member_1.x + _e669.member_1.x);
                                    let _e778 = (_e669.member_1.y + _e669.member_1.y);
                                    let _e779 = (_e669.member_1.z + _e669.member_1.z);
                                    let _e781 = (_e669.member_1.z * _e779);
                                    let _e782 = (_e669.member_1.w * _e777);
                                    let _e783 = (_e669.member_1.w * _e778);
                                    let _e784 = (_e669.member_1.w * _e779);
                                    let _e804 = (vec4<f32>((1f - fma(_e669.member_1.y, _e778, _e781)), fma(_e669.member_1.x, _e778, _e784), fma(_e669.member_1.x, _e779, -(_e783)), 0f) * _e669.member_2.x);
                                    let _e806 = (vec4<f32>(fma(_e669.member_1.x, _e778, -(_e784)), (1f - fma(_e669.member_1.x, _e777, _e781)), fma(_e669.member_1.y, _e779, _e782), 0f) * _e669.member_2.y);
                                    let _e808 = (vec4<f32>(fma(_e669.member_1.x, _e779, _e783), fma(_e669.member_1.y, _e779, -(_e782)), (1f - fma(_e669.member_1.x, _e777, (_e669.member_1.y * _e778))), 0f) * _e669.member_2.z);
                                    if _e589 {
                                    } else {
                                        phi_3887_ = true;
                                        break;
                                    }
                                    let _e913 = local[_e582.member_1];
                                    phi_1628_ = type_21((_e567.member + (vec4<f32>(fma(_e669.member.x, _e769.member.w, fma(_e808.x, _e769.member.z, fma(_e804.x, _e769.member.x, (_e806.x * _e769.member.y)))), fma(_e669.member.y, _e769.member.w, fma(_e808.y, _e769.member.z, fma(_e804.y, _e769.member.x, (_e806.y * _e769.member.y)))), fma(_e669.member.z, _e769.member.w, fma(_e808.z, _e769.member.z, fma(_e804.z, _e769.member.x, (_e806.z * _e769.member.y)))), (fma(_e808.w, _e769.member.z, fma(_e804.w, _e769.member.x, (_e806.w * _e769.member.y))) + _e769.member.w)) * _e913)), (_e567.member_1 + (vec4<f32>(fma(_e669.member.x, _e769.member_1.w, fma(_e808.x, _e769.member_1.z, fma(_e804.x, _e769.member_1.x, (_e806.x * _e769.member_1.y)))), fma(_e669.member.y, _e769.member_1.w, fma(_e808.y, _e769.member_1.z, fma(_e804.y, _e769.member_1.x, (_e806.y * _e769.member_1.y)))), fma(_e669.member.z, _e769.member_1.w, fma(_e808.z, _e769.member_1.z, fma(_e804.z, _e769.member_1.x, (_e806.z * _e769.member_1.y)))), (fma(_e808.w, _e769.member_1.z, fma(_e804.w, _e769.member_1.x, (_e806.w * _e769.member_1.y))) + _e769.member_1.w)) * _e913)), (_e567.member_2 + (vec4<f32>(fma(_e669.member.x, _e769.member_2.w, fma(_e808.x, _e769.member_2.z, fma(_e804.x, _e769.member_2.x, (_e806.x * _e769.member_2.y)))), fma(_e669.member.y, _e769.member_2.w, fma(_e808.y, _e769.member_2.z, fma(_e804.y, _e769.member_2.x, (_e806.y * _e769.member_2.y)))), fma(_e669.member.z, _e769.member_2.w, fma(_e808.z, _e769.member_2.z, fma(_e804.z, _e769.member_2.x, (_e806.z * _e769.member_2.y)))), (fma(_e808.w, _e769.member_2.z, fma(_e804.w, _e769.member_2.x, (_e806.w * _e769.member_2.y))) + _e769.member_2.w)) * _e913)), (_e567.member_3 + (vec4<f32>(fma(_e669.member.x, _e769.member_3.w, fma(_e808.x, _e769.member_3.z, fma(_e804.x, _e769.member_3.x, (_e806.x * _e769.member_3.y)))), fma(_e669.member.y, _e769.member_3.w, fma(_e808.y, _e769.member_3.z, fma(_e804.y, _e769.member_3.x, (_e806.y * _e769.member_3.y)))), fma(_e669.member.z, _e769.member_3.w, fma(_e808.z, _e769.member_3.z, fma(_e804.z, _e769.member_3.x, (_e806.z * _e769.member_3.y)))), (fma(_e808.w, _e769.member_3.z, fma(_e804.w, _e769.member_3.x, (_e806.w * _e769.member_3.y))) + _e769.member_3.w)) * _e913)));
                                    phi_2052_ = true;
                                    break;
                                }
                                default: {
                                    phi_1628_ = type_21();
                                    phi_2052_ = bool();
                                    break;
                                }
                            }
                            let _e928 = phi_1628_;
                            let _e930 = phi_2052_;
                            continue;
                            continuing {
                                phi_1624_ = _e580;
                                phi_1627_ = _e928;
                                phi_3887_ = false;
                                break if !(_e930);
                            }
                        }
                        let _e933 = phi_3887_;
                        if _e933 {
                            break;
                        }
                        let _e935 = local_8;
                        let _e940 = global_4.member[0u];
                        if (_e935.member.x == _e940) {
                            let _e943 = local_9;
                            let _e948 = global_4.member[1u];
                            if (_e943.member.y == _e948) {
                                let _e951 = local_10;
                                let _e956 = global_4.member[2u];
                                let _e957 = (_e951.member.z == _e956);
                                if _e957 {
                                    let _e959 = local_11;
                                    let _e964 = global_4.member[3u];
                                    phi_2079_ = (_e959.member.w == _e964);
                                } else {
                                    phi_2079_ = bool();
                                }
                                let _e967 = phi_2079_;
                                phi_2081_ = _e967;
                                phi_2082_ = select(true, false, _e957);
                            } else {
                                phi_2081_ = bool();
                                phi_2082_ = true;
                            }
                            let _e970 = phi_2081_;
                            let _e972 = phi_2082_;
                            phi_2083_ = _e970;
                            phi_2084_ = _e972;
                        } else {
                            phi_2083_ = bool();
                            phi_2084_ = true;
                        }
                        let _e974 = phi_2083_;
                        let _e976 = phi_2084_;
                        if select(_e974, false, _e976) {
                            let _e979 = local_12;
                            let _e984 = global_4.member_1[0u];
                            if (_e979.member_1.x == _e984) {
                                let _e987 = local_13;
                                let _e992 = global_4.member_1[1u];
                                if (_e987.member_1.y == _e992) {
                                    let _e995 = local_14;
                                    let _e1000 = global_4.member_1[2u];
                                    let _e1001 = (_e995.member_1.z == _e1000);
                                    if _e1001 {
                                        let _e1003 = local_15;
                                        let _e1008 = global_4.member_1[3u];
                                        phi_2118_ = (_e1003.member_1.w == _e1008);
                                    } else {
                                        phi_2118_ = bool();
                                    }
                                    let _e1011 = phi_2118_;
                                    phi_2120_ = _e1011;
                                    phi_2121_ = select(true, false, _e1001);
                                } else {
                                    phi_2120_ = bool();
                                    phi_2121_ = true;
                                }
                                let _e1014 = phi_2120_;
                                let _e1016 = phi_2121_;
                                phi_2122_ = _e1014;
                                phi_2123_ = _e1016;
                            } else {
                                phi_2122_ = bool();
                                phi_2123_ = true;
                            }
                            let _e1018 = phi_2122_;
                            let _e1020 = phi_2123_;
                            if select(_e1018, false, _e1020) {
                                let _e1023 = local_16;
                                let _e1028 = global_4.member_2[0u];
                                if (_e1023.member_2.x == _e1028) {
                                    let _e1031 = local_17;
                                    let _e1036 = global_4.member_2[1u];
                                    if (_e1031.member_2.y == _e1036) {
                                        let _e1039 = local_18;
                                        let _e1044 = global_4.member_2[2u];
                                        let _e1045 = (_e1039.member_2.z == _e1044);
                                        if _e1045 {
                                            let _e1047 = local_19;
                                            let _e1052 = global_4.member_2[3u];
                                            phi_2157_ = (_e1047.member_2.w == _e1052);
                                        } else {
                                            phi_2157_ = bool();
                                        }
                                        let _e1055 = phi_2157_;
                                        phi_2159_ = _e1055;
                                        phi_2160_ = select(true, false, _e1045);
                                    } else {
                                        phi_2159_ = bool();
                                        phi_2160_ = true;
                                    }
                                    let _e1058 = phi_2159_;
                                    let _e1060 = phi_2160_;
                                    phi_2161_ = _e1058;
                                    phi_2162_ = _e1060;
                                } else {
                                    phi_2161_ = bool();
                                    phi_2162_ = true;
                                }
                                let _e1062 = phi_2161_;
                                let _e1064 = phi_2162_;
                                let _e1065 = select(_e1062, false, _e1064);
                                if _e1065 {
                                    let _e1067 = local_20;
                                    let _e1072 = global_4.member_3[0u];
                                    if (_e1067.member_3.x == _e1072) {
                                        let _e1075 = local_21;
                                        let _e1080 = global_4.member_3[1u];
                                        if (_e1075.member_3.y == _e1080) {
                                            let _e1083 = local_22;
                                            let _e1088 = global_4.member_3[2u];
                                            let _e1089 = (_e1083.member_3.z == _e1088);
                                            if _e1089 {
                                                let _e1091 = local_23;
                                                let _e1096 = global_4.member_3[3u];
                                                phi_2196_ = (_e1091.member_3.w == _e1096);
                                            } else {
                                                phi_2196_ = bool();
                                            }
                                            let _e1099 = phi_2196_;
                                            phi_2198_ = _e1099;
                                            phi_2199_ = select(true, false, _e1089);
                                        } else {
                                            phi_2198_ = bool();
                                            phi_2199_ = true;
                                        }
                                        let _e1102 = phi_2198_;
                                        let _e1104 = phi_2199_;
                                        phi_2200_ = _e1102;
                                        phi_2201_ = _e1104;
                                    } else {
                                        phi_2200_ = bool();
                                        phi_2201_ = true;
                                    }
                                    let _e1106 = phi_2200_;
                                    let _e1108 = phi_2201_;
                                    phi_2206_ = select(_e1106, false, _e1108);
                                } else {
                                    phi_2206_ = bool();
                                }
                                let _e1111 = phi_2206_;
                                phi_2208_ = _e1111;
                                phi_2209_ = select(true, false, _e1065);
                            } else {
                                phi_2208_ = bool();
                                phi_2209_ = true;
                            }
                            let _e1114 = phi_2208_;
                            let _e1116 = phi_2209_;
                            phi_2210_ = _e1114;
                            phi_2211_ = _e1116;
                        } else {
                            phi_2210_ = bool();
                            phi_2211_ = true;
                        }
                        let _e1118 = phi_2210_;
                        let _e1120 = phi_2211_;
                        if select(_e1118, false, _e1120) {
                            phi_2219_ = type_21(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        } else {
                            let _e2072 = local_31;
                            phi_2219_ = _e2072;
                        }
                        let _e1123 = phi_2219_;
                        let _e1146 = fma(_e1123.member_2.z, _e1123.member_3.w, -((_e1123.member_2.w * _e1123.member_3.z)));
                        let _e1149 = fma(_e1123.member_2.y, _e1123.member_3.w, -((_e1123.member_2.w * _e1123.member_3.y)));
                        let _e1152 = fma(_e1123.member_2.y, _e1123.member_3.z, -((_e1123.member_2.z * _e1123.member_3.y)));
                        let _e1155 = fma(_e1123.member_2.x, _e1123.member_3.w, -((_e1123.member_2.w * _e1123.member_3.x)));
                        let _e1158 = fma(_e1123.member_2.x, _e1123.member_3.z, -((_e1123.member_2.z * _e1123.member_3.x)));
                        let _e1161 = fma(_e1123.member_2.x, _e1123.member_3.y, -((_e1123.member_2.y * _e1123.member_3.x)));
                        let _e1183 = fma(-(_e1123.member.w), fma(_e1123.member_1.z, _e1161, fma(_e1123.member_1.x, _e1152, -((_e1123.member_1.y * _e1158)))), fma(_e1123.member.z, fma(_e1123.member_1.w, _e1161, fma(_e1123.member_1.x, _e1149, -((_e1123.member_1.y * _e1155)))), fma(_e1123.member.x, fma(_e1123.member_1.w, _e1152, fma(_e1123.member_1.y, _e1146, -((_e1123.member_1.z * _e1149)))), -((_e1123.member.y * fma(_e1123.member_1.w, _e1158, fma(_e1123.member_1.x, _e1146, -((_e1123.member_1.z * _e1155)))))))));
                        if (_e1183 == 0f) {
                            phi_3318_ = type_27(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            phi_3319_ = type_27();
                            phi_3320_ = true;
                        } else {
                            let _e1192 = (sqrt(fma(_e1123.member.w, _e1123.member.w, fma(_e1123.member.z, _e1123.member.z, fma(_e1123.member.x, _e1123.member.x, (_e1123.member.y * _e1123.member.y))))) * select(-1f, 1f, (_e1183 >= 0f)));
                            let _e1197 = sqrt(fma(_e1123.member_1.w, _e1123.member_1.w, fma(_e1123.member_1.z, _e1123.member_1.z, fma(_e1123.member_1.x, _e1123.member_1.x, (_e1123.member_1.y * _e1123.member_1.y)))));
                            let _e1202 = sqrt(fma(_e1123.member_2.w, _e1123.member_2.w, fma(_e1123.member_2.z, _e1123.member_2.z, fma(_e1123.member_2.x, _e1123.member_2.x, (_e1123.member_2.y * _e1123.member_2.y)))));
                            if (_e1192 != 0f) {
                                phi_3206_ = select(true, false, (_e1197 != 0f));
                            } else {
                                phi_3206_ = true;
                            }
                            let _e1209 = phi_3206_;
                            let _e1210 = select((_e1202 != 0f), false, _e1209);
                            if _e1210 {
                                let _e1211 = (1f / _e1192);
                                let _e1212 = (1f / _e1197);
                                let _e1213 = (1f / _e1202);
                                let _e1214 = (_e1123.member.x * _e1211);
                                let _e1215 = (_e1123.member.z * _e1211);
                                let _e1216 = (_e1123.member_1.x * _e1212);
                                let _e1217 = (_e1123.member_2.x * _e1213);
                                let _e1218 = (_e1123.member_2.y * _e1213);
                                if ((_e1123.member_2.z * _e1213) <= 0f) {
                                    let _e1253 = fma(_e1123.member_1.y, _e1212, -(_e1214));
                                    let _e1255 = fma(-(_e1123.member_2.z), _e1213, 1f);
                                    if (_e1253 <= 0f) {
                                        let _e1269 = (_e1255 - _e1253);
                                        let _e1271 = (0.5f / sqrt(_e1269));
                                        phi_3301_ = vec4<f32>((_e1269 * _e1271), (fma(_e1123.member.y, _e1211, _e1216) * _e1271), (fma(_e1123.member.z, _e1211, _e1217) * _e1271), (fma(_e1123.member_1.z, _e1212, -(_e1218)) * _e1271));
                                    } else {
                                        let _e1257 = (_e1255 + _e1253);
                                        let _e1259 = (0.5f / sqrt(_e1257));
                                        phi_3301_ = vec4<f32>((fma(_e1123.member.y, _e1211, _e1216) * _e1259), (_e1257 * _e1259), (fma(_e1123.member_1.z, _e1212, _e1218) * _e1259), (fma(_e1123.member_2.x, _e1213, -(_e1215)) * _e1259));
                                    }
                                    let _e1282 = phi_3301_;
                                    phi_3303_ = _e1282;
                                } else {
                                    let _e1221 = fma(_e1123.member_1.y, _e1212, _e1214);
                                    let _e1222 = fma(_e1123.member_2.z, _e1213, 1f);
                                    if (_e1221 <= 0f) {
                                        let _e1238 = (_e1222 - _e1221);
                                        let _e1240 = (0.5f / sqrt(_e1238));
                                        phi_3271_ = vec4<f32>((fma(_e1123.member.z, _e1211, _e1217) * _e1240), (fma(_e1123.member_1.z, _e1212, _e1218) * _e1240), (_e1238 * _e1240), (fma(_e1123.member.y, _e1211, -(_e1216)) * _e1240));
                                    } else {
                                        let _e1224 = (_e1222 + _e1221);
                                        let _e1226 = (0.5f / sqrt(_e1224));
                                        phi_3271_ = vec4<f32>((fma(_e1123.member_1.z, _e1212, -(_e1218)) * _e1226), (fma(_e1123.member_2.x, _e1213, -(_e1215)) * _e1226), (fma(_e1123.member.y, _e1211, -(_e1216)) * _e1226), (_e1224 * _e1226));
                                    }
                                    let _e1251 = phi_3271_;
                                    phi_3303_ = _e1251;
                                }
                                let _e1284 = phi_3303_;
                                phi_3312_ = type_27(vec3<f32>(_e1192, _e1197, _e1202), _e1284, vec3<f32>(_e1123.member_3.x, _e1123.member_3.y, _e1123.member_3.z));
                                phi_3313_ = type_27();
                            } else {
                                phi_3312_ = type_27();
                                phi_3313_ = type_27(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e1288 = phi_3312_;
                            let _e1290 = phi_3313_;
                            phi_3318_ = _e1290;
                            phi_3319_ = _e1288;
                            phi_3320_ = select(true, false, _e1210);
                        }
                        let _e1293 = phi_3318_;
                        let _e1295 = phi_3319_;
                        let _e1297 = phi_3320_;
                        if _e1297 {
                            phi_3324_ = _e1293;
                        } else {
                            phi_3324_ = _e1295;
                        }
                        let _e1299 = phi_3324_;
                        phi_2221_ = type_27(_e1299.member_2, _e1299.member_1, _e1299.member);
                    } else {
                        phi_2221_ = type_27();
                    }
                    let _e1305 = phi_2221_;
                    phi_2223_ = _e1305;
                    phi_2224_ = select(true, false, _e535);
                } else {
                    phi_2223_ = type_27();
                    phi_2224_ = true;
                }
                let _e1308 = phi_2223_;
                let _e1310 = phi_2224_;
                if _e1310 {
                    if (_e101 >= 10u) {
                        phi_3418_ = (_e129 <= (_e101 - 10u));
                    } else {
                        phi_3418_ = false;
                    }
                    let _e1315 = phi_3418_;
                    if _e1315 {
                        let _e1318 = global.member[_e129];
                        let _e1323 = global.member[(_e129 + 1u)];
                        let _e1328 = global.member[(_e129 + 2u)];
                        let _e1334 = global.member[(_e129 + 3u)];
                        let _e1339 = global.member[(_e129 + 4u)];
                        let _e1344 = global.member[(_e129 + 5u)];
                        let _e1349 = global.member[(_e129 + 6u)];
                        let _e1355 = global.member[(_e129 + 7u)];
                        let _e1360 = global.member[(_e129 + 8u)];
                        let _e1365 = global.member[(_e129 + 9u)];
                        phi_2277_ = type_27(vec3<f32>(bitcast<f32>(_e1318), bitcast<f32>(_e1323), bitcast<f32>(_e1328)), vec4<f32>(bitcast<f32>(_e1334), bitcast<f32>(_e1339), bitcast<f32>(_e1344), bitcast<f32>(_e1349)), vec3<f32>(bitcast<f32>(_e1355), bitcast<f32>(_e1360), bitcast<f32>(_e1365)));
                    } else {
                        phi_2277_ = type_27(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e1370 = phi_2277_;
                    phi_2278_ = _e1370;
                } else {
                    phi_2278_ = _e1308;
                }
                let _e1372 = phi_2278_;
                let _e1380 = (_e1372.member_1.x + _e1372.member_1.x);
                let _e1381 = (_e1372.member_1.y + _e1372.member_1.y);
                let _e1382 = (_e1372.member_1.z + _e1372.member_1.z);
                let _e1384 = (_e1372.member_1.z * _e1382);
                let _e1385 = (_e1372.member_1.w * _e1380);
                let _e1386 = (_e1372.member_1.w * _e1381);
                let _e1387 = (_e1372.member_1.w * _e1382);
                let _e1407 = (vec4<f32>((1f - fma(_e1372.member_1.y, _e1381, _e1384)), fma(_e1372.member_1.x, _e1381, _e1387), fma(_e1372.member_1.x, _e1382, -(_e1386)), 0f) * _e1372.member_2.x);
                let _e1409 = (vec4<f32>(fma(_e1372.member_1.x, _e1381, -(_e1387)), (1f - fma(_e1372.member_1.x, _e1380, _e1384)), fma(_e1372.member_1.y, _e1382, _e1385), 0f) * _e1372.member_2.y);
                let _e1411 = (vec4<f32>(fma(_e1372.member_1.x, _e1382, _e1386), fma(_e1372.member_1.y, _e1382, -(_e1385)), (1f - fma(_e1372.member_1.x, _e1380, (_e1372.member_1.y * _e1381))), 0f) * _e1372.member_2.z);
                let _e1416 = local_24;
                let _e1439 = (_e1372.member.x + fma(_e1411.x, _e1416.member.z, fma(_e1409.x, _e1416.member.y, (_e1407.x * _e1416.member.x))));
                let _e1440 = (_e1372.member.y + fma(_e1411.y, _e1416.member.z, fma(_e1409.y, _e1416.member.y, (_e1407.y * _e1416.member.x))));
                let _e1441 = (_e1372.member.z + fma(_e1411.z, _e1416.member.z, fma(_e1409.z, _e1416.member.y, (_e1407.z * _e1416.member.x))));
                let _e1444 = local_25;
                global_6 = _e1444.member_1;
                let _e1447 = local_26;
                global_7 = _e1447.member_6;
                let _e1450 = local_27;
                global_8 = _e1450.member_7;
                global_9 = vec3<f32>(_e1439, _e1440, _e1441);
                let _e1456 = local_28;
                let _e1464 = sqrt(fma(_e1456.member_2.z, _e1456.member_2.z, fma(_e1456.member_2.x, _e1456.member_2.x, (_e1456.member_2.y * _e1456.member_2.y))));
                if (_e1464 == 0f) {
                    phi_2365_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_2365_ = (_e1456.member_2 * (1f / _e1464));
                }
                let _e1469 = phi_2365_;
                let _e1471 = local_29;
                let _e1480 = sqrt(fma(_e1471.member_3.z, _e1471.member_3.z, fma(_e1471.member_3.x, _e1471.member_3.x, (_e1471.member_3.y * _e1471.member_3.y))));
                if (_e1480 == 0f) {
                    phi_3509_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3509_ = (vec3<f32>(_e1471.member_3.x, _e1471.member_3.y, _e1471.member_3.z) * (1f / _e1480));
                }
                let _e1485 = phi_3509_;
                let _e1487 = (_e1469.x / (_e1372.member_2.x * _e1372.member_2.x));
                let _e1489 = (_e1469.y / (_e1372.member_2.y * _e1372.member_2.y));
                let _e1491 = (_e1469.z / (_e1372.member_2.z * _e1372.member_2.z));
                let _e1498 = fma(_e1411.x, _e1491, fma(_e1407.x, _e1487, (_e1409.x * _e1489)));
                let _e1499 = fma(_e1411.y, _e1491, fma(_e1407.y, _e1487, (_e1409.y * _e1489)));
                let _e1500 = fma(_e1411.z, _e1491, fma(_e1407.z, _e1487, (_e1409.z * _e1489)));
                let _e1505 = sqrt(fma(_e1500, _e1500, fma(_e1498, _e1498, (_e1499 * _e1499))));
                if (_e1505 == 0f) {
                    phi_3544_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3544_ = (vec3<f32>(_e1498, _e1499, _e1500) * (1f / _e1505));
                }
                let _e1510 = phi_3544_;
                global_10 = _e1510;
                let _e1520 = fma(_e1411.x, _e1485.z, fma(_e1407.x, _e1485.x, (_e1409.x * _e1485.y)));
                let _e1521 = fma(_e1411.y, _e1485.z, fma(_e1407.y, _e1485.x, (_e1409.y * _e1485.y)));
                let _e1522 = fma(_e1411.z, _e1485.z, fma(_e1407.z, _e1485.x, (_e1409.z * _e1485.y)));
                let _e1527 = sqrt(fma(_e1522, _e1522, fma(_e1520, _e1520, (_e1521 * _e1521))));
                if (_e1527 == 0f) {
                    phi_3579_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3579_ = (vec3<f32>(_e1520, _e1521, _e1522) * (1f / _e1527));
                }
                let _e1532 = phi_3579_;
                global_11 = _e1532;
                let _e1549 = local_30;
                let _e1553 = select(-1f, 1f, (_e1549.member_3.w >= 0f));
                global_12 = vec3<f32>((fma(_e1510.y, _e1532.z, -((_e1532.y * _e1510.z))) * _e1553), (fma(_e1510.z, _e1532.x, -((_e1532.z * _e1510.x))) * _e1553), (fma(_e1510.x, _e1532.y, -((_e1532.x * _e1510.y))) * _e1553));
                if (_e101 >= 86u) {
                    phi_3592_ = (_e125 <= (_e101 - 86u));
                } else {
                    phi_3592_ = false;
                }
                let _e1562 = phi_3592_;
                if _e1562 {
                    let _e1565 = global.member[_e125];
                    let _e1570 = global.member[(_e125 + 1u)];
                    let _e1575 = global.member[(_e125 + 2u)];
                    let _e1580 = global.member[(_e125 + 3u)];
                    let _e1586 = global.member[(_e125 + 4u)];
                    let _e1591 = global.member[(_e125 + 5u)];
                    let _e1596 = global.member[(_e125 + 6u)];
                    let _e1601 = global.member[(_e125 + 7u)];
                    let _e1607 = global.member[(_e125 + 8u)];
                    let _e1612 = global.member[(_e125 + 9u)];
                    let _e1617 = global.member[(_e125 + 10u)];
                    let _e1622 = global.member[(_e125 + 11u)];
                    let _e1628 = global.member[(_e125 + 12u)];
                    let _e1633 = global.member[(_e125 + 13u)];
                    let _e1638 = global.member[(_e125 + 14u)];
                    let _e1643 = global.member[(_e125 + 15u)];
                    let _e1650 = global.member[(_e125 + 16u)];
                    let _e1655 = global.member[(_e125 + 17u)];
                    let _e1660 = global.member[(_e125 + 18u)];
                    let _e1665 = global.member[(_e125 + 19u)];
                    let _e1671 = global.member[(_e125 + 20u)];
                    let _e1676 = global.member[(_e125 + 21u)];
                    let _e1681 = global.member[(_e125 + 22u)];
                    let _e1686 = global.member[(_e125 + 23u)];
                    let _e1692 = global.member[(_e125 + 24u)];
                    let _e1697 = global.member[(_e125 + 25u)];
                    let _e1702 = global.member[(_e125 + 26u)];
                    let _e1707 = global.member[(_e125 + 27u)];
                    let _e1713 = global.member[(_e125 + 28u)];
                    let _e1718 = global.member[(_e125 + 29u)];
                    let _e1723 = global.member[(_e125 + 30u)];
                    let _e1728 = global.member[(_e125 + 31u)];
                    let _e1735 = global.member[(_e125 + 32u)];
                    let _e1740 = global.member[(_e125 + 33u)];
                    let _e1745 = global.member[(_e125 + 34u)];
                    local_3 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    phi_2649_ = type_14(0u, 6u);
                    loop {
                        let _e1750 = phi_2649_;
                        if (_e1750.member < _e1750.member_1) {
                            phi_2650_ = type_14((_e1750.member + 1u), _e1750.member_1);
                            phi_2673_ = type_14(1u, _e1750.member);
                        } else {
                            phi_2650_ = _e1750;
                            phi_2673_ = type_14(0u, type_14().member_1);
                        }
                        let _e1763 = phi_2650_;
                        let _e1765 = phi_2673_;
                        switch bitcast<i32>(_e1765.member) {
                            case 0: {
                                phi_2700_ = false;
                                break;
                            }
                            case 1: {
                                let _e1770 = ((_e125 + 35u) + (_e1765.member_1 * 4u));
                                let _e1773 = global.member[_e1770];
                                let _e1778 = global.member[(_e1770 + 1u)];
                                let _e1783 = global.member[(_e1770 + 2u)];
                                let _e1788 = global.member[(_e1770 + 3u)];
                                local_3[_e1765.member_1] = vec4<f32>(bitcast<f32>(_e1773), bitcast<f32>(_e1778), bitcast<f32>(_e1783), bitcast<f32>(_e1788));
                                phi_2700_ = true;
                                break;
                            }
                            default: {
                                phi_2700_ = bool();
                                break;
                            }
                        }
                        let _e1793 = phi_2700_;
                        continue;
                        continuing {
                            phi_2649_ = _e1763;
                            break if !(_e1793);
                        }
                    }
                    let _e1795 = local_3;
                    local_2 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                    phi_2706_ = type_14(0u, 8u);
                    loop {
                        let _e1798 = phi_2706_;
                        if (_e1798.member < _e1798.member_1) {
                            phi_2707_ = type_14((_e1798.member + 1u), _e1798.member_1);
                            phi_2730_ = type_14(1u, _e1798.member);
                        } else {
                            phi_2707_ = _e1798;
                            phi_2730_ = type_14(0u, type_14().member_1);
                        }
                        let _e1811 = phi_2707_;
                        let _e1813 = phi_2730_;
                        switch bitcast<i32>(_e1813.member) {
                            case 0: {
                                phi_2753_ = false;
                                break;
                            }
                            case 1: {
                                let _e1818 = ((_e125 + 59u) + (_e1813.member_1 * 3u));
                                let _e1821 = global.member[_e1818];
                                let _e1826 = global.member[(_e1818 + 1u)];
                                let _e1831 = global.member[(_e1818 + 2u)];
                                local_2[_e1813.member_1] = vec3<f32>(bitcast<f32>(_e1821), bitcast<f32>(_e1826), bitcast<f32>(_e1831));
                                phi_2753_ = true;
                                break;
                            }
                            default: {
                                phi_2753_ = bool();
                                break;
                            }
                        }
                        let _e1836 = phi_2753_;
                        continue;
                        continuing {
                            phi_2706_ = _e1811;
                            break if !(_e1836);
                        }
                    }
                    let _e1838 = local_2;
                    let _e1842 = global.member[(_e125 + 83u)];
                    let _e1847 = global.member[(_e125 + 84u)];
                    let _e1852 = global.member[(_e125 + 85u)];
                    phi_2774_ = type_23(type_21(vec4<f32>(bitcast<f32>(_e1565), bitcast<f32>(_e1570), bitcast<f32>(_e1575), bitcast<f32>(_e1580)), vec4<f32>(bitcast<f32>(_e1586), bitcast<f32>(_e1591), bitcast<f32>(_e1596), bitcast<f32>(_e1601)), vec4<f32>(bitcast<f32>(_e1607), bitcast<f32>(_e1612), bitcast<f32>(_e1617), bitcast<f32>(_e1622)), vec4<f32>(bitcast<f32>(_e1628), bitcast<f32>(_e1633), bitcast<f32>(_e1638), bitcast<f32>(_e1643))), type_21(vec4<f32>(bitcast<f32>(_e1650), bitcast<f32>(_e1655), bitcast<f32>(_e1660), bitcast<f32>(_e1665)), vec4<f32>(bitcast<f32>(_e1671), bitcast<f32>(_e1676), bitcast<f32>(_e1681), bitcast<f32>(_e1686)), vec4<f32>(bitcast<f32>(_e1692), bitcast<f32>(_e1697), bitcast<f32>(_e1702), bitcast<f32>(_e1707)), vec4<f32>(bitcast<f32>(_e1713), bitcast<f32>(_e1718), bitcast<f32>(_e1723), bitcast<f32>(_e1728))), vec3<f32>(bitcast<f32>(_e1735), bitcast<f32>(_e1740), bitcast<f32>(_e1745)), type_22(_e1838, _e1795, vec3<f32>(bitcast<f32>(_e1842), bitcast<f32>(_e1847), bitcast<f32>(_e1852))));
                } else {
                    phi_2774_ = type_23(type_21(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_21(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_22(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
                }
                let _e1858 = phi_2774_;
                global_2 = vec4<f32>((fma(fma(_e1858.member.member_3.x, _e1858.member_1.member_2.w, fma(_e1858.member.member_2.x, _e1858.member_1.member_2.z, fma(_e1858.member.member.x, _e1858.member_1.member_2.x, (_e1858.member.member_1.x * _e1858.member_1.member_2.y)))), _e1441, fma(fma(_e1858.member.member_3.x, _e1858.member_1.member.w, fma(_e1858.member.member_2.x, _e1858.member_1.member.z, fma(_e1858.member.member.x, _e1858.member_1.member.x, (_e1858.member.member_1.x * _e1858.member_1.member.y)))), _e1439, (fma(_e1858.member.member_3.x, _e1858.member_1.member_1.w, fma(_e1858.member.member_2.x, _e1858.member_1.member_1.z, fma(_e1858.member.member.x, _e1858.member_1.member_1.x, (_e1858.member.member_1.x * _e1858.member_1.member_1.y)))) * _e1440))) + fma(_e1858.member.member_3.x, _e1858.member_1.member_3.w, fma(_e1858.member.member_2.x, _e1858.member_1.member_3.z, fma(_e1858.member.member.x, _e1858.member_1.member_3.x, (_e1858.member.member_1.x * _e1858.member_1.member_3.y))))), (fma(fma(_e1858.member.member_3.y, _e1858.member_1.member_2.w, fma(_e1858.member.member_2.y, _e1858.member_1.member_2.z, fma(_e1858.member.member.y, _e1858.member_1.member_2.x, (_e1858.member.member_1.y * _e1858.member_1.member_2.y)))), _e1441, fma(fma(_e1858.member.member_3.y, _e1858.member_1.member.w, fma(_e1858.member.member_2.y, _e1858.member_1.member.z, fma(_e1858.member.member.y, _e1858.member_1.member.x, (_e1858.member.member_1.y * _e1858.member_1.member.y)))), _e1439, (fma(_e1858.member.member_3.y, _e1858.member_1.member_1.w, fma(_e1858.member.member_2.y, _e1858.member_1.member_1.z, fma(_e1858.member.member.y, _e1858.member_1.member_1.x, (_e1858.member.member_1.y * _e1858.member_1.member_1.y)))) * _e1440))) + fma(_e1858.member.member_3.y, _e1858.member_1.member_3.w, fma(_e1858.member.member_2.y, _e1858.member_1.member_3.z, fma(_e1858.member.member.y, _e1858.member_1.member_3.x, (_e1858.member.member_1.y * _e1858.member_1.member_3.y))))), (fma(fma(_e1858.member.member_3.z, _e1858.member_1.member_2.w, fma(_e1858.member.member_2.z, _e1858.member_1.member_2.z, fma(_e1858.member.member.z, _e1858.member_1.member_2.x, (_e1858.member.member_1.z * _e1858.member_1.member_2.y)))), _e1441, fma(fma(_e1858.member.member_3.z, _e1858.member_1.member.w, fma(_e1858.member.member_2.z, _e1858.member_1.member.z, fma(_e1858.member.member.z, _e1858.member_1.member.x, (_e1858.member.member_1.z * _e1858.member_1.member.y)))), _e1439, (fma(_e1858.member.member_3.z, _e1858.member_1.member_1.w, fma(_e1858.member.member_2.z, _e1858.member_1.member_1.z, fma(_e1858.member.member.z, _e1858.member_1.member_1.x, (_e1858.member.member_1.z * _e1858.member_1.member_1.y)))) * _e1440))) + fma(_e1858.member.member_3.z, _e1858.member_1.member_3.w, fma(_e1858.member.member_2.z, _e1858.member_1.member_3.z, fma(_e1858.member.member.z, _e1858.member_1.member_3.x, (_e1858.member.member_1.z * _e1858.member_1.member_3.y))))), (fma(fma(_e1858.member.member_3.w, _e1858.member_1.member_2.w, fma(_e1858.member.member_2.w, _e1858.member_1.member_2.z, fma(_e1858.member.member.w, _e1858.member_1.member_2.x, (_e1858.member.member_1.w * _e1858.member_1.member_2.y)))), _e1441, fma(fma(_e1858.member.member_3.w, _e1858.member_1.member.w, fma(_e1858.member.member_2.w, _e1858.member_1.member.z, fma(_e1858.member.member.w, _e1858.member_1.member.x, (_e1858.member.member_1.w * _e1858.member_1.member.y)))), _e1439, (fma(_e1858.member.member_3.w, _e1858.member_1.member_1.w, fma(_e1858.member.member_2.w, _e1858.member_1.member_1.z, fma(_e1858.member.member.w, _e1858.member_1.member_1.x, (_e1858.member.member_1.w * _e1858.member_1.member_1.y)))) * _e1440))) + fma(_e1858.member.member_3.w, _e1858.member_1.member_3.w, fma(_e1858.member.member_2.w, _e1858.member_1.member_3.z, fma(_e1858.member.member.w, _e1858.member_1.member_3.x, (_e1858.member.member_1.w * _e1858.member_1.member_3.y))))));
            } else {
                global_2 = vec4<f32>(10f, 10f, 10f, 1f);
            }
            break;
        }
    }
    return;
}

@vertex 
fn stagerenderlet_vertex(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_3 = param;
    global_1 = param_1;
    function();
    let _e14 = global_2.y;
    global_2.y = -(_e14);
    let _e16 = global_5;
    let _e17 = global_6;
    let _e18 = global_7;
    let _e19 = global_8;
    let _e20 = global_10;
    let _e21 = global_11;
    let _e22 = global_12;
    let _e23 = global_9;
    let _e24 = global_2;
    return VertexOutput(_e16, _e17, _e18, _e19, _e20, _e21, _e22, _e23, _e24);
}
