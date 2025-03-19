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

var<private> global: u32;
var<private> global_1: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_2: u32;
@group(0) @binding(0) 
var<storage> global_3: type_3;
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
    var phi_1157_: u32;
    var phi_2882_: bool;
    var phi_1164_: u32;
    var phi_1165_: u32;
    var phi_1175_: u32;
    var phi_1257_: type_14;
    var phi_1258_: type_14;
    var phi_1281_: type_14;
    var phi_1294_: bool;
    var phi_1300_: type_14;
    var phi_1301_: type_14;
    var phi_1324_: type_14;
    var phi_1338_: bool;
    var phi_1344_: type_14;
    var phi_1347_: type_30;
    var phi_1345_: type_14;
    var phi_1370_: type_14;
    var phi_1387_: u32;
    var phi_2912_: bool;
    var phi_1405_: type_14;
    var phi_2938_: u32;
    var phi_2957_: bool;
    var phi_1455_: type_33;
    var phi_1465_: u32;
    var phi_2979_: bool;
    var phi_1473_: f32;
    var phi_1348_: type_30;
    var phi_1526_: bool;
    var phi_3001_: bool;
    var phi_1626_: type_34;
    var local_6: type_30;
    var phi_1629_: type_14;
    var phi_1632_: type_21;
    var phi_1630_: type_14;
    var phi_1655_: type_14;
    var local_7: type_30;
    var phi_1679_: u32;
    var phi_3035_: bool;
    var phi_1688_: u32;
    var phi_3059_: bool;
    var phi_1737_: type_27;
    var phi_1747_: u32;
    var phi_3084_: bool;
    var phi_1820_: type_21;
    var phi_1633_: type_21;
    var phi_2057_: bool;
    var phi_3908_: bool;
    var local_8: type_21;
    var local_9: type_21;
    var local_10: type_21;
    var local_11: type_21;
    var phi_2084_: bool;
    var phi_2086_: bool;
    var phi_2087_: bool;
    var phi_2088_: bool;
    var phi_2089_: bool;
    var local_12: type_21;
    var local_13: type_21;
    var local_14: type_21;
    var local_15: type_21;
    var phi_2123_: bool;
    var phi_2125_: bool;
    var phi_2126_: bool;
    var phi_2127_: bool;
    var phi_2128_: bool;
    var local_16: type_21;
    var local_17: type_21;
    var local_18: type_21;
    var local_19: type_21;
    var phi_2162_: bool;
    var phi_2164_: bool;
    var phi_2165_: bool;
    var phi_2166_: bool;
    var phi_2167_: bool;
    var local_20: type_21;
    var local_21: type_21;
    var local_22: type_21;
    var local_23: type_21;
    var phi_2201_: bool;
    var phi_2203_: bool;
    var phi_2204_: bool;
    var phi_2205_: bool;
    var phi_2206_: bool;
    var phi_2211_: bool;
    var phi_2213_: bool;
    var phi_2214_: bool;
    var phi_2215_: bool;
    var phi_2216_: bool;
    var phi_2224_: type_21;
    var phi_3219_: bool;
    var phi_3284_: vec4<f32>;
    var phi_3314_: vec4<f32>;
    var phi_3316_: vec4<f32>;
    var phi_3325_: type_27;
    var phi_3326_: type_27;
    var phi_3331_: type_27;
    var phi_3332_: type_27;
    var phi_3333_: bool;
    var phi_3337_: type_27;
    var phi_2226_: type_27;
    var phi_2228_: type_27;
    var phi_2229_: bool;
    var phi_3431_: bool;
    var phi_2282_: type_27;
    var phi_2283_: type_27;
    var local_24: type_30;
    var local_25: type_30;
    var local_26: type_30;
    var local_27: type_30;
    var local_28: type_30;
    var phi_2370_: vec3<f32>;
    var local_29: type_30;
    var phi_3522_: vec3<f32>;
    var phi_3557_: vec3<f32>;
    var phi_3592_: vec3<f32>;
    var local_30: type_30;
    var phi_3613_: bool;
    var phi_2657_: type_14;
    var phi_2658_: type_14;
    var phi_2681_: type_14;
    var phi_2708_: bool;
    var phi_2714_: type_14;
    var phi_2715_: type_14;
    var phi_2738_: type_14;
    var phi_2761_: bool;
    var phi_2782_: type_23;
    var local_31: type_21;

    switch bitcast<i32>(0u) {
        default: {
            let _e98 = global_2;
            let _e99 = global;
            let _e101 = arrayLength((&global_3.member));
            let _e104 = global_3.member[_e98];
            let _e109 = global_3.member[(_e98 + 1u)];
            let _e113 = global_3.member[(_e98 + 2u)];
            let _e117 = global_3.member[(_e98 + 7u)];
            let _e121 = global_3.member[(_e98 + 8u)];
            let _e125 = global_3.member[(_e98 + 9u)];
            let _e129 = global_3.member[(_e98 + 11u)];
            let _e133 = global_3.member[(_e98 + 12u)];
            let _e137 = global_3.member[(_e98 + 13u)];
            let _e141 = global_3.member[(_e98 + 14u)];
            let _e145 = global_3.member[(_e98 + 15u)];
            let _e149 = global_3.member[(_e98 + 16u)];
            if (_e104 == 1u) {
                global_5 = _e98;
                if (_e117 == 4294967295u) {
                    phi_1165_ = _e99;
                } else {
                    if (_e99 >= _e121) {
                        phi_1157_ = 4294967295u;
                    } else {
                        phi_1157_ = (_e117 + _e99);
                    }
                    let _e154 = phi_1157_;
                    if (_e101 >= 1u) {
                        phi_2882_ = (_e154 <= (_e101 - 1u));
                    } else {
                        phi_2882_ = false;
                    }
                    let _e159 = phi_2882_;
                    if _e159 {
                        let _e162 = global_3.member[_e154];
                        phi_1164_ = _e162;
                    } else {
                        phi_1164_ = 0u;
                    }
                    let _e164 = phi_1164_;
                    phi_1165_ = _e164;
                }
                let _e166 = phi_1165_;
                if (_e166 >= _e113) {
                    phi_1175_ = 4294967295u;
                } else {
                    phi_1175_ = (_e109 + (26u * _e166));
                }
                let _e171 = phi_1175_;
                let _e174 = global_3.member[_e171];
                let _e179 = global_3.member[(_e171 + 1u)];
                let _e184 = global_3.member[(_e171 + 2u)];
                let _e190 = global_3.member[(_e171 + 3u)];
                let _e195 = global_3.member[(_e171 + 4u)];
                let _e200 = global_3.member[(_e171 + 5u)];
                let _e205 = global_3.member[(_e171 + 6u)];
                let _e211 = global_3.member[(_e171 + 7u)];
                let _e216 = global_3.member[(_e171 + 8u)];
                let _e222 = global_3.member[(_e171 + 9u)];
                let _e227 = global_3.member[(_e171 + 10u)];
                let _e233 = global_3.member[(_e171 + 11u)];
                let _e238 = global_3.member[(_e171 + 12u)];
                let _e243 = global_3.member[(_e171 + 13u)];
                let _e249 = global_3.member[(_e171 + 14u)];
                let _e254 = global_3.member[(_e171 + 15u)];
                let _e259 = global_3.member[(_e171 + 16u)];
                let _e264 = global_3.member[(_e171 + 17u)];
                local_5 = array<u32, 4>(0u, 0u, 0u, 0u);
                phi_1257_ = type_14(0u, 4u);
                loop {
                    let _e269 = phi_1257_;
                    if (_e269.member < _e269.member_1) {
                        phi_1258_ = type_14((_e269.member + 1u), _e269.member_1);
                        phi_1281_ = type_14(1u, _e269.member);
                    } else {
                        phi_1258_ = _e269;
                        phi_1281_ = type_14(0u, type_14().member_1);
                    }
                    let _e282 = phi_1258_;
                    let _e284 = phi_1281_;
                    switch bitcast<i32>(_e284.member) {
                        case 0: {
                            phi_1294_ = false;
                            break;
                        }
                        case 1: {
                            let _e291 = global_3.member[((_e171 + 18u) + _e284.member_1)];
                            local_5[_e284.member_1] = _e291;
                            phi_1294_ = true;
                            break;
                        }
                        default: {
                            phi_1294_ = bool();
                            break;
                        }
                    }
                    let _e294 = phi_1294_;
                    continue;
                    continuing {
                        phi_1257_ = _e282;
                        break if !(_e294);
                    }
                }
                let _e296 = local_5;
                local_4 = array<f32, 4>(0f, 0f, 0f, 0f);
                phi_1300_ = type_14(0u, 4u);
                loop {
                    let _e299 = phi_1300_;
                    if (_e299.member < _e299.member_1) {
                        phi_1301_ = type_14((_e299.member + 1u), _e299.member_1);
                        phi_1324_ = type_14(1u, _e299.member);
                    } else {
                        phi_1301_ = _e299;
                        phi_1324_ = type_14(0u, type_14().member_1);
                    }
                    let _e312 = phi_1301_;
                    let _e314 = phi_1324_;
                    switch bitcast<i32>(_e314.member) {
                        case 0: {
                            phi_1338_ = false;
                            break;
                        }
                        case 1: {
                            let _e321 = global_3.member[((_e171 + 22u) + _e314.member_1)];
                            local_4[_e314.member_1] = bitcast<f32>(_e321);
                            phi_1338_ = true;
                            break;
                        }
                        default: {
                            phi_1338_ = bool();
                            break;
                        }
                    }
                    let _e325 = phi_1338_;
                    continue;
                    continuing {
                        phi_1300_ = _e312;
                        break if !(_e325);
                    }
                }
                let _e327 = local_4;
                phi_1344_ = type_14(0u, _e137);
                phi_1347_ = type_30(vec3<f32>(bitcast<f32>(_e174), bitcast<f32>(_e179), bitcast<f32>(_e184)), vec4<f32>(bitcast<f32>(_e190), bitcast<f32>(_e195), bitcast<f32>(_e200), bitcast<f32>(_e205)), vec3<f32>(bitcast<f32>(_e233), bitcast<f32>(_e238), bitcast<f32>(_e243)), vec4<f32>(bitcast<f32>(_e249), bitcast<f32>(_e254), bitcast<f32>(_e259), bitcast<f32>(_e264)), _e296, _e327, vec2<f32>(bitcast<f32>(_e211), bitcast<f32>(_e216)), vec2<f32>(bitcast<f32>(_e222), bitcast<f32>(_e227)));
                loop {
                    let _e331 = phi_1344_;
                    let _e333 = phi_1347_;
                    local_6 = _e333;
                    local_7 = _e333;
                    local_24 = _e333;
                    local_25 = _e333;
                    local_26 = _e333;
                    local_27 = _e333;
                    local_28 = _e333;
                    local_29 = _e333;
                    local_30 = _e333;
                    if (_e331.member < _e331.member_1) {
                        phi_1345_ = type_14((_e331.member + 1u), _e331.member_1);
                        phi_1370_ = type_14(1u, _e331.member);
                    } else {
                        phi_1345_ = _e331;
                        phi_1370_ = type_14(0u, type_14().member_1);
                    }
                    let _e346 = phi_1345_;
                    let _e348 = phi_1370_;
                    switch bitcast<i32>(_e348.member) {
                        case 0: {
                            phi_1348_ = type_30();
                            phi_1526_ = false;
                            break;
                        }
                        case 1: {
                            if (_e348.member_1 >= _e137) {
                                phi_1387_ = 4294967295u;
                            } else {
                                phi_1387_ = (_e133 + (2u * _e348.member_1));
                            }
                            let _e356 = phi_1387_;
                            if (_e101 >= 2u) {
                                phi_2912_ = (_e356 <= (_e101 - 2u));
                            } else {
                                phi_2912_ = false;
                            }
                            let _e361 = phi_2912_;
                            if _e361 {
                                let _e364 = global_3.member[_e356];
                                let _e368 = global_3.member[(_e356 + 1u)];
                                phi_1405_ = type_14(_e364, _e368);
                            } else {
                                phi_1405_ = type_14(4294967295u, 0u);
                            }
                            let _e371 = phi_1405_;
                            if (_e166 >= _e371.member_1) {
                                phi_2938_ = 4294967295u;
                            } else {
                                phi_2938_ = (_e371.member + (9u * _e166));
                            }
                            let _e378 = phi_2938_;
                            if (_e101 >= 9u) {
                                phi_2957_ = (_e378 <= (_e101 - 9u));
                            } else {
                                phi_2957_ = false;
                            }
                            let _e383 = phi_2957_;
                            if _e383 {
                                let _e386 = global_3.member[_e378];
                                let _e391 = global_3.member[(_e378 + 1u)];
                                let _e396 = global_3.member[(_e378 + 2u)];
                                let _e402 = global_3.member[(_e378 + 3u)];
                                let _e407 = global_3.member[(_e378 + 4u)];
                                let _e412 = global_3.member[(_e378 + 5u)];
                                let _e418 = global_3.member[(_e378 + 6u)];
                                let _e423 = global_3.member[(_e378 + 7u)];
                                let _e428 = global_3.member[(_e378 + 8u)];
                                phi_1455_ = type_33(vec3<f32>(bitcast<f32>(_e386), bitcast<f32>(_e391), bitcast<f32>(_e396)), vec3<f32>(bitcast<f32>(_e402), bitcast<f32>(_e407), bitcast<f32>(_e412)), vec3<f32>(bitcast<f32>(_e418), bitcast<f32>(_e423), bitcast<f32>(_e428)));
                            } else {
                                phi_1455_ = type_33(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e433 = phi_1455_;
                            if (_e348.member_1 >= _e145) {
                                phi_1465_ = 4294967295u;
                            } else {
                                phi_1465_ = (_e141 + _e348.member_1);
                            }
                            let _e437 = phi_1465_;
                            if (_e101 >= 1u) {
                                phi_2979_ = (_e437 <= (_e101 - 1u));
                            } else {
                                phi_2979_ = false;
                            }
                            let _e442 = phi_2979_;
                            if _e442 {
                                let _e445 = global_3.member[_e437];
                                phi_1473_ = bitcast<f32>(_e445);
                            } else {
                                phi_1473_ = 0f;
                            }
                            let _e448 = phi_1473_;
                            let _e471 = type_30(vec3<f32>(fma(_e448, _e433.member.x, _e333.member.x), fma(_e448, _e433.member.y, _e333.member.y), fma(_e448, _e433.member.z, _e333.member.z)), _e333.member_1, _e333.member_2, _e333.member_3, _e333.member_4, _e333.member_5, _e333.member_6, _e333.member_7);
                            let _e494 = type_30(_e471.member, _e471.member_1, vec3<f32>(fma(_e448, _e433.member_1.x, _e333.member_2.x), fma(_e448, _e433.member_1.y, _e333.member_2.y), fma(_e448, _e433.member_1.z, _e333.member_2.z)), _e471.member_3, _e471.member_4, _e471.member_5, _e471.member_6, _e471.member_7);
                            phi_1348_ = type_30(_e494.member, _e494.member_1, _e494.member_2, vec4<f32>(fma(_e448, _e433.member_2.x, _e333.member_3.x), fma(_e448, _e433.member_2.y, _e333.member_3.y), fma(_e448, _e433.member_2.z, _e333.member_3.z), _e333.member_3.w), _e494.member_4, _e494.member_5, _e494.member_6, _e494.member_7);
                            phi_1526_ = true;
                            break;
                        }
                        default: {
                            phi_1348_ = type_30();
                            phi_1526_ = bool();
                            break;
                        }
                    }
                    let _e521 = phi_1348_;
                    let _e523 = phi_1526_;
                    continue;
                    continuing {
                        phi_1344_ = _e346;
                        phi_1347_ = _e521;
                        break if !(_e523);
                    }
                }
                let _e528 = global_3.member[(_e149 + 7u)];
                if (_e528 == 1u) {
                    let _e531 = ((_e129 == 4294967295u) != true);
                    if _e531 {
                        if (_e101 >= 4u) {
                            phi_3001_ = (_e129 <= (_e101 - 4u));
                        } else {
                            phi_3001_ = false;
                        }
                        let _e536 = phi_3001_;
                        if _e536 {
                            let _e539 = global_3.member[_e129];
                            let _e543 = global_3.member[(_e129 + 1u)];
                            let _e547 = global_3.member[(_e129 + 2u)];
                            let _e551 = global_3.member[(_e129 + 3u)];
                            phi_1626_ = type_34(type_14(_e539, _e543), type_14(_e547, _e551));
                        } else {
                            phi_1626_ = type_34(type_14(4294967295u, 0u), type_14(4294967295u, 0u));
                        }
                        let _e556 = phi_1626_;
                        let _e558 = local_6;
                        local = _e558.member_5;
                        phi_1629_ = type_14(0u, 4u);
                        phi_1632_ = type_21(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        loop {
                            let _e561 = phi_1629_;
                            let _e563 = phi_1632_;
                            local_8 = _e563;
                            local_9 = _e563;
                            local_10 = _e563;
                            local_11 = _e563;
                            local_12 = _e563;
                            local_13 = _e563;
                            local_14 = _e563;
                            local_15 = _e563;
                            local_16 = _e563;
                            local_17 = _e563;
                            local_18 = _e563;
                            local_19 = _e563;
                            local_20 = _e563;
                            local_21 = _e563;
                            local_22 = _e563;
                            local_23 = _e563;
                            local_31 = _e563;
                            if (_e561.member < _e561.member_1) {
                                phi_1630_ = type_14((_e561.member + 1u), _e561.member_1);
                                phi_1655_ = type_14(1u, _e561.member);
                            } else {
                                phi_1630_ = _e561;
                                phi_1655_ = type_14(0u, type_14().member_1);
                            }
                            let _e576 = phi_1630_;
                            let _e578 = phi_1655_;
                            switch bitcast<i32>(_e578.member) {
                                case 0: {
                                    phi_1633_ = type_21();
                                    phi_2057_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e583 = local_7;
                                    local_1 = _e583.member_4;
                                    let _e585 = (_e578.member_1 < 4u);
                                    if _e585 {
                                    } else {
                                        phi_3908_ = true;
                                        break;
                                    }
                                    let _e587 = local_1[_e578.member_1];
                                    if (_e587 >= _e556.member.member_1) {
                                        phi_1679_ = 4294967295u;
                                    } else {
                                        phi_1679_ = (_e556.member.member + _e587);
                                    }
                                    let _e595 = phi_1679_;
                                    if (_e101 >= 1u) {
                                        phi_3035_ = (_e595 <= (_e101 - 1u));
                                    } else {
                                        phi_3035_ = false;
                                    }
                                    let _e600 = phi_3035_;
                                    if _e600 {
                                        let _e603 = global_3.member[_e595];
                                        phi_1688_ = _e603;
                                    } else {
                                        phi_1688_ = 4294967295u;
                                    }
                                    let _e605 = phi_1688_;
                                    if (_e101 >= 10u) {
                                        phi_3059_ = (_e605 <= (_e101 - 10u));
                                    } else {
                                        phi_3059_ = false;
                                    }
                                    let _e610 = phi_3059_;
                                    if _e610 {
                                        let _e613 = global_3.member[_e605];
                                        let _e618 = global_3.member[(_e605 + 1u)];
                                        let _e623 = global_3.member[(_e605 + 2u)];
                                        let _e629 = global_3.member[(_e605 + 3u)];
                                        let _e634 = global_3.member[(_e605 + 4u)];
                                        let _e639 = global_3.member[(_e605 + 5u)];
                                        let _e644 = global_3.member[(_e605 + 6u)];
                                        let _e650 = global_3.member[(_e605 + 7u)];
                                        let _e655 = global_3.member[(_e605 + 8u)];
                                        let _e660 = global_3.member[(_e605 + 9u)];
                                        phi_1737_ = type_27(vec3<f32>(bitcast<f32>(_e613), bitcast<f32>(_e618), bitcast<f32>(_e623)), vec4<f32>(bitcast<f32>(_e629), bitcast<f32>(_e634), bitcast<f32>(_e639), bitcast<f32>(_e644)), vec3<f32>(bitcast<f32>(_e650), bitcast<f32>(_e655), bitcast<f32>(_e660)));
                                    } else {
                                        phi_1737_ = type_27(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e665 = phi_1737_;
                                    if (_e587 >= _e556.member_1.member_1) {
                                        phi_1747_ = 4294967295u;
                                    } else {
                                        phi_1747_ = (_e556.member_1.member + (16u * _e587));
                                    }
                                    let _e674 = phi_1747_;
                                    if (_e101 >= 16u) {
                                        phi_3084_ = (_e674 <= (_e101 - 16u));
                                    } else {
                                        phi_3084_ = false;
                                    }
                                    let _e679 = phi_3084_;
                                    if _e679 {
                                        let _e682 = global_3.member[_e674];
                                        let _e687 = global_3.member[(_e674 + 1u)];
                                        let _e692 = global_3.member[(_e674 + 2u)];
                                        let _e697 = global_3.member[(_e674 + 3u)];
                                        let _e703 = global_3.member[(_e674 + 4u)];
                                        let _e708 = global_3.member[(_e674 + 5u)];
                                        let _e713 = global_3.member[(_e674 + 6u)];
                                        let _e718 = global_3.member[(_e674 + 7u)];
                                        let _e724 = global_3.member[(_e674 + 8u)];
                                        let _e729 = global_3.member[(_e674 + 9u)];
                                        let _e734 = global_3.member[(_e674 + 10u)];
                                        let _e739 = global_3.member[(_e674 + 11u)];
                                        let _e745 = global_3.member[(_e674 + 12u)];
                                        let _e750 = global_3.member[(_e674 + 13u)];
                                        let _e755 = global_3.member[(_e674 + 14u)];
                                        let _e760 = global_3.member[(_e674 + 15u)];
                                        phi_1820_ = type_21(vec4<f32>(bitcast<f32>(_e682), bitcast<f32>(_e687), bitcast<f32>(_e692), bitcast<f32>(_e697)), vec4<f32>(bitcast<f32>(_e703), bitcast<f32>(_e708), bitcast<f32>(_e713), bitcast<f32>(_e718)), vec4<f32>(bitcast<f32>(_e724), bitcast<f32>(_e729), bitcast<f32>(_e734), bitcast<f32>(_e739)), vec4<f32>(bitcast<f32>(_e745), bitcast<f32>(_e750), bitcast<f32>(_e755), bitcast<f32>(_e760)));
                                    } else {
                                        phi_1820_ = type_21(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                    }
                                    let _e765 = phi_1820_;
                                    let _e773 = (_e665.member_1.x + _e665.member_1.x);
                                    let _e774 = (_e665.member_1.y + _e665.member_1.y);
                                    let _e775 = (_e665.member_1.z + _e665.member_1.z);
                                    let _e777 = (_e665.member_1.z * _e775);
                                    let _e778 = (_e665.member_1.w * _e773);
                                    let _e779 = (_e665.member_1.w * _e774);
                                    let _e780 = (_e665.member_1.w * _e775);
                                    let _e800 = (vec4<f32>((1f - fma(_e665.member_1.y, _e774, _e777)), fma(_e665.member_1.x, _e774, _e780), fma(_e665.member_1.x, _e775, -(_e779)), 0f) * _e665.member_2.x);
                                    let _e802 = (vec4<f32>(fma(_e665.member_1.x, _e774, -(_e780)), (1f - fma(_e665.member_1.x, _e773, _e777)), fma(_e665.member_1.y, _e775, _e778), 0f) * _e665.member_2.y);
                                    let _e804 = (vec4<f32>(fma(_e665.member_1.x, _e775, _e779), fma(_e665.member_1.y, _e775, -(_e778)), (1f - fma(_e665.member_1.x, _e773, (_e665.member_1.y * _e774))), 0f) * _e665.member_2.z);
                                    if _e585 {
                                    } else {
                                        phi_3908_ = true;
                                        break;
                                    }
                                    let _e909 = local[_e578.member_1];
                                    phi_1633_ = type_21((_e563.member + (vec4<f32>(fma(_e665.member.x, _e765.member.w, fma(_e804.x, _e765.member.z, fma(_e800.x, _e765.member.x, (_e802.x * _e765.member.y)))), fma(_e665.member.y, _e765.member.w, fma(_e804.y, _e765.member.z, fma(_e800.y, _e765.member.x, (_e802.y * _e765.member.y)))), fma(_e665.member.z, _e765.member.w, fma(_e804.z, _e765.member.z, fma(_e800.z, _e765.member.x, (_e802.z * _e765.member.y)))), (fma(_e804.w, _e765.member.z, fma(_e800.w, _e765.member.x, (_e802.w * _e765.member.y))) + _e765.member.w)) * _e909)), (_e563.member_1 + (vec4<f32>(fma(_e665.member.x, _e765.member_1.w, fma(_e804.x, _e765.member_1.z, fma(_e800.x, _e765.member_1.x, (_e802.x * _e765.member_1.y)))), fma(_e665.member.y, _e765.member_1.w, fma(_e804.y, _e765.member_1.z, fma(_e800.y, _e765.member_1.x, (_e802.y * _e765.member_1.y)))), fma(_e665.member.z, _e765.member_1.w, fma(_e804.z, _e765.member_1.z, fma(_e800.z, _e765.member_1.x, (_e802.z * _e765.member_1.y)))), (fma(_e804.w, _e765.member_1.z, fma(_e800.w, _e765.member_1.x, (_e802.w * _e765.member_1.y))) + _e765.member_1.w)) * _e909)), (_e563.member_2 + (vec4<f32>(fma(_e665.member.x, _e765.member_2.w, fma(_e804.x, _e765.member_2.z, fma(_e800.x, _e765.member_2.x, (_e802.x * _e765.member_2.y)))), fma(_e665.member.y, _e765.member_2.w, fma(_e804.y, _e765.member_2.z, fma(_e800.y, _e765.member_2.x, (_e802.y * _e765.member_2.y)))), fma(_e665.member.z, _e765.member_2.w, fma(_e804.z, _e765.member_2.z, fma(_e800.z, _e765.member_2.x, (_e802.z * _e765.member_2.y)))), (fma(_e804.w, _e765.member_2.z, fma(_e800.w, _e765.member_2.x, (_e802.w * _e765.member_2.y))) + _e765.member_2.w)) * _e909)), (_e563.member_3 + (vec4<f32>(fma(_e665.member.x, _e765.member_3.w, fma(_e804.x, _e765.member_3.z, fma(_e800.x, _e765.member_3.x, (_e802.x * _e765.member_3.y)))), fma(_e665.member.y, _e765.member_3.w, fma(_e804.y, _e765.member_3.z, fma(_e800.y, _e765.member_3.x, (_e802.y * _e765.member_3.y)))), fma(_e665.member.z, _e765.member_3.w, fma(_e804.z, _e765.member_3.z, fma(_e800.z, _e765.member_3.x, (_e802.z * _e765.member_3.y)))), (fma(_e804.w, _e765.member_3.z, fma(_e800.w, _e765.member_3.x, (_e802.w * _e765.member_3.y))) + _e765.member_3.w)) * _e909)));
                                    phi_2057_ = true;
                                    break;
                                }
                                default: {
                                    phi_1633_ = type_21();
                                    phi_2057_ = bool();
                                    break;
                                }
                            }
                            let _e924 = phi_1633_;
                            let _e926 = phi_2057_;
                            continue;
                            continuing {
                                phi_1629_ = _e576;
                                phi_1632_ = _e924;
                                phi_3908_ = false;
                                break if !(_e926);
                            }
                        }
                        let _e929 = phi_3908_;
                        if _e929 {
                            break;
                        }
                        let _e931 = local_8;
                        let _e936 = global_4.member[0u];
                        if (_e931.member.x == _e936) {
                            let _e939 = local_9;
                            let _e944 = global_4.member[1u];
                            if (_e939.member.y == _e944) {
                                let _e947 = local_10;
                                let _e952 = global_4.member[2u];
                                let _e953 = (_e947.member.z == _e952);
                                if _e953 {
                                    let _e955 = local_11;
                                    let _e960 = global_4.member[3u];
                                    phi_2084_ = (_e955.member.w == _e960);
                                } else {
                                    phi_2084_ = bool();
                                }
                                let _e963 = phi_2084_;
                                phi_2086_ = _e963;
                                phi_2087_ = select(true, false, _e953);
                            } else {
                                phi_2086_ = bool();
                                phi_2087_ = true;
                            }
                            let _e966 = phi_2086_;
                            let _e968 = phi_2087_;
                            phi_2088_ = _e966;
                            phi_2089_ = _e968;
                        } else {
                            phi_2088_ = bool();
                            phi_2089_ = true;
                        }
                        let _e970 = phi_2088_;
                        let _e972 = phi_2089_;
                        if select(_e970, false, _e972) {
                            let _e975 = local_12;
                            let _e980 = global_4.member_1[0u];
                            if (_e975.member_1.x == _e980) {
                                let _e983 = local_13;
                                let _e988 = global_4.member_1[1u];
                                if (_e983.member_1.y == _e988) {
                                    let _e991 = local_14;
                                    let _e996 = global_4.member_1[2u];
                                    let _e997 = (_e991.member_1.z == _e996);
                                    if _e997 {
                                        let _e999 = local_15;
                                        let _e1004 = global_4.member_1[3u];
                                        phi_2123_ = (_e999.member_1.w == _e1004);
                                    } else {
                                        phi_2123_ = bool();
                                    }
                                    let _e1007 = phi_2123_;
                                    phi_2125_ = _e1007;
                                    phi_2126_ = select(true, false, _e997);
                                } else {
                                    phi_2125_ = bool();
                                    phi_2126_ = true;
                                }
                                let _e1010 = phi_2125_;
                                let _e1012 = phi_2126_;
                                phi_2127_ = _e1010;
                                phi_2128_ = _e1012;
                            } else {
                                phi_2127_ = bool();
                                phi_2128_ = true;
                            }
                            let _e1014 = phi_2127_;
                            let _e1016 = phi_2128_;
                            if select(_e1014, false, _e1016) {
                                let _e1019 = local_16;
                                let _e1024 = global_4.member_2[0u];
                                if (_e1019.member_2.x == _e1024) {
                                    let _e1027 = local_17;
                                    let _e1032 = global_4.member_2[1u];
                                    if (_e1027.member_2.y == _e1032) {
                                        let _e1035 = local_18;
                                        let _e1040 = global_4.member_2[2u];
                                        let _e1041 = (_e1035.member_2.z == _e1040);
                                        if _e1041 {
                                            let _e1043 = local_19;
                                            let _e1048 = global_4.member_2[3u];
                                            phi_2162_ = (_e1043.member_2.w == _e1048);
                                        } else {
                                            phi_2162_ = bool();
                                        }
                                        let _e1051 = phi_2162_;
                                        phi_2164_ = _e1051;
                                        phi_2165_ = select(true, false, _e1041);
                                    } else {
                                        phi_2164_ = bool();
                                        phi_2165_ = true;
                                    }
                                    let _e1054 = phi_2164_;
                                    let _e1056 = phi_2165_;
                                    phi_2166_ = _e1054;
                                    phi_2167_ = _e1056;
                                } else {
                                    phi_2166_ = bool();
                                    phi_2167_ = true;
                                }
                                let _e1058 = phi_2166_;
                                let _e1060 = phi_2167_;
                                let _e1061 = select(_e1058, false, _e1060);
                                if _e1061 {
                                    let _e1063 = local_20;
                                    let _e1068 = global_4.member_3[0u];
                                    if (_e1063.member_3.x == _e1068) {
                                        let _e1071 = local_21;
                                        let _e1076 = global_4.member_3[1u];
                                        if (_e1071.member_3.y == _e1076) {
                                            let _e1079 = local_22;
                                            let _e1084 = global_4.member_3[2u];
                                            let _e1085 = (_e1079.member_3.z == _e1084);
                                            if _e1085 {
                                                let _e1087 = local_23;
                                                let _e1092 = global_4.member_3[3u];
                                                phi_2201_ = (_e1087.member_3.w == _e1092);
                                            } else {
                                                phi_2201_ = bool();
                                            }
                                            let _e1095 = phi_2201_;
                                            phi_2203_ = _e1095;
                                            phi_2204_ = select(true, false, _e1085);
                                        } else {
                                            phi_2203_ = bool();
                                            phi_2204_ = true;
                                        }
                                        let _e1098 = phi_2203_;
                                        let _e1100 = phi_2204_;
                                        phi_2205_ = _e1098;
                                        phi_2206_ = _e1100;
                                    } else {
                                        phi_2205_ = bool();
                                        phi_2206_ = true;
                                    }
                                    let _e1102 = phi_2205_;
                                    let _e1104 = phi_2206_;
                                    phi_2211_ = select(_e1102, false, _e1104);
                                } else {
                                    phi_2211_ = bool();
                                }
                                let _e1107 = phi_2211_;
                                phi_2213_ = _e1107;
                                phi_2214_ = select(true, false, _e1061);
                            } else {
                                phi_2213_ = bool();
                                phi_2214_ = true;
                            }
                            let _e1110 = phi_2213_;
                            let _e1112 = phi_2214_;
                            phi_2215_ = _e1110;
                            phi_2216_ = _e1112;
                        } else {
                            phi_2215_ = bool();
                            phi_2216_ = true;
                        }
                        let _e1114 = phi_2215_;
                        let _e1116 = phi_2216_;
                        if select(_e1114, false, _e1116) {
                            phi_2224_ = type_21(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        } else {
                            let _e2071 = local_31;
                            phi_2224_ = _e2071;
                        }
                        let _e1119 = phi_2224_;
                        let _e1142 = fma(_e1119.member_2.z, _e1119.member_3.w, -((_e1119.member_2.w * _e1119.member_3.z)));
                        let _e1145 = fma(_e1119.member_2.y, _e1119.member_3.w, -((_e1119.member_2.w * _e1119.member_3.y)));
                        let _e1148 = fma(_e1119.member_2.y, _e1119.member_3.z, -((_e1119.member_2.z * _e1119.member_3.y)));
                        let _e1151 = fma(_e1119.member_2.x, _e1119.member_3.w, -((_e1119.member_2.w * _e1119.member_3.x)));
                        let _e1154 = fma(_e1119.member_2.x, _e1119.member_3.z, -((_e1119.member_2.z * _e1119.member_3.x)));
                        let _e1157 = fma(_e1119.member_2.x, _e1119.member_3.y, -((_e1119.member_2.y * _e1119.member_3.x)));
                        let _e1179 = fma(-(_e1119.member.w), fma(_e1119.member_1.z, _e1157, fma(_e1119.member_1.x, _e1148, -((_e1119.member_1.y * _e1154)))), fma(_e1119.member.z, fma(_e1119.member_1.w, _e1157, fma(_e1119.member_1.x, _e1145, -((_e1119.member_1.y * _e1151)))), fma(_e1119.member.x, fma(_e1119.member_1.w, _e1148, fma(_e1119.member_1.y, _e1142, -((_e1119.member_1.z * _e1145)))), -((_e1119.member.y * fma(_e1119.member_1.w, _e1154, fma(_e1119.member_1.x, _e1142, -((_e1119.member_1.z * _e1151)))))))));
                        if (_e1179 == 0f) {
                            phi_3331_ = type_27(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            phi_3332_ = type_27();
                            phi_3333_ = true;
                        } else {
                            let _e1188 = (sqrt(fma(_e1119.member.w, _e1119.member.w, fma(_e1119.member.z, _e1119.member.z, fma(_e1119.member.x, _e1119.member.x, (_e1119.member.y * _e1119.member.y))))) * select(-1f, 1f, (_e1179 >= 0f)));
                            let _e1193 = sqrt(fma(_e1119.member_1.w, _e1119.member_1.w, fma(_e1119.member_1.z, _e1119.member_1.z, fma(_e1119.member_1.x, _e1119.member_1.x, (_e1119.member_1.y * _e1119.member_1.y)))));
                            let _e1198 = sqrt(fma(_e1119.member_2.w, _e1119.member_2.w, fma(_e1119.member_2.z, _e1119.member_2.z, fma(_e1119.member_2.x, _e1119.member_2.x, (_e1119.member_2.y * _e1119.member_2.y)))));
                            if (_e1188 != 0f) {
                                phi_3219_ = select(true, false, (_e1193 != 0f));
                            } else {
                                phi_3219_ = true;
                            }
                            let _e1205 = phi_3219_;
                            let _e1206 = select((_e1198 != 0f), false, _e1205);
                            if _e1206 {
                                let _e1207 = (1f / _e1188);
                                let _e1208 = (1f / _e1193);
                                let _e1209 = (1f / _e1198);
                                let _e1210 = (_e1119.member.x * _e1207);
                                let _e1211 = (_e1119.member.z * _e1207);
                                let _e1212 = (_e1119.member_1.x * _e1208);
                                let _e1213 = (_e1119.member_2.x * _e1209);
                                let _e1214 = (_e1119.member_2.y * _e1209);
                                if ((_e1119.member_2.z * _e1209) <= 0f) {
                                    let _e1249 = fma(_e1119.member_1.y, _e1208, -(_e1210));
                                    let _e1251 = fma(-(_e1119.member_2.z), _e1209, 1f);
                                    if (_e1249 <= 0f) {
                                        let _e1265 = (_e1251 - _e1249);
                                        let _e1267 = (0.5f / sqrt(_e1265));
                                        phi_3314_ = vec4<f32>((_e1265 * _e1267), (fma(_e1119.member.y, _e1207, _e1212) * _e1267), (fma(_e1119.member.z, _e1207, _e1213) * _e1267), (fma(_e1119.member_1.z, _e1208, -(_e1214)) * _e1267));
                                    } else {
                                        let _e1253 = (_e1251 + _e1249);
                                        let _e1255 = (0.5f / sqrt(_e1253));
                                        phi_3314_ = vec4<f32>((fma(_e1119.member.y, _e1207, _e1212) * _e1255), (_e1253 * _e1255), (fma(_e1119.member_1.z, _e1208, _e1214) * _e1255), (fma(_e1119.member_2.x, _e1209, -(_e1211)) * _e1255));
                                    }
                                    let _e1278 = phi_3314_;
                                    phi_3316_ = _e1278;
                                } else {
                                    let _e1217 = fma(_e1119.member_1.y, _e1208, _e1210);
                                    let _e1218 = fma(_e1119.member_2.z, _e1209, 1f);
                                    if (_e1217 <= 0f) {
                                        let _e1234 = (_e1218 - _e1217);
                                        let _e1236 = (0.5f / sqrt(_e1234));
                                        phi_3284_ = vec4<f32>((fma(_e1119.member.z, _e1207, _e1213) * _e1236), (fma(_e1119.member_1.z, _e1208, _e1214) * _e1236), (_e1234 * _e1236), (fma(_e1119.member.y, _e1207, -(_e1212)) * _e1236));
                                    } else {
                                        let _e1220 = (_e1218 + _e1217);
                                        let _e1222 = (0.5f / sqrt(_e1220));
                                        phi_3284_ = vec4<f32>((fma(_e1119.member_1.z, _e1208, -(_e1214)) * _e1222), (fma(_e1119.member_2.x, _e1209, -(_e1211)) * _e1222), (fma(_e1119.member.y, _e1207, -(_e1212)) * _e1222), (_e1220 * _e1222));
                                    }
                                    let _e1247 = phi_3284_;
                                    phi_3316_ = _e1247;
                                }
                                let _e1280 = phi_3316_;
                                phi_3325_ = type_27(vec3<f32>(_e1188, _e1193, _e1198), _e1280, vec3<f32>(_e1119.member_3.x, _e1119.member_3.y, _e1119.member_3.z));
                                phi_3326_ = type_27();
                            } else {
                                phi_3325_ = type_27();
                                phi_3326_ = type_27(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e1284 = phi_3325_;
                            let _e1286 = phi_3326_;
                            phi_3331_ = _e1286;
                            phi_3332_ = _e1284;
                            phi_3333_ = select(true, false, _e1206);
                        }
                        let _e1289 = phi_3331_;
                        let _e1291 = phi_3332_;
                        let _e1293 = phi_3333_;
                        if _e1293 {
                            phi_3337_ = _e1289;
                        } else {
                            phi_3337_ = _e1291;
                        }
                        let _e1295 = phi_3337_;
                        phi_2226_ = type_27(_e1295.member_2, _e1295.member_1, _e1295.member);
                    } else {
                        phi_2226_ = type_27();
                    }
                    let _e1301 = phi_2226_;
                    phi_2228_ = _e1301;
                    phi_2229_ = select(true, false, _e531);
                } else {
                    phi_2228_ = type_27();
                    phi_2229_ = true;
                }
                let _e1304 = phi_2228_;
                let _e1306 = phi_2229_;
                if _e1306 {
                    if (_e101 >= 10u) {
                        phi_3431_ = (_e125 <= (_e101 - 10u));
                    } else {
                        phi_3431_ = false;
                    }
                    let _e1311 = phi_3431_;
                    if _e1311 {
                        let _e1314 = global_3.member[_e125];
                        let _e1319 = global_3.member[(_e125 + 1u)];
                        let _e1324 = global_3.member[(_e125 + 2u)];
                        let _e1330 = global_3.member[(_e125 + 3u)];
                        let _e1335 = global_3.member[(_e125 + 4u)];
                        let _e1340 = global_3.member[(_e125 + 5u)];
                        let _e1345 = global_3.member[(_e125 + 6u)];
                        let _e1351 = global_3.member[(_e125 + 7u)];
                        let _e1356 = global_3.member[(_e125 + 8u)];
                        let _e1361 = global_3.member[(_e125 + 9u)];
                        phi_2282_ = type_27(vec3<f32>(bitcast<f32>(_e1314), bitcast<f32>(_e1319), bitcast<f32>(_e1324)), vec4<f32>(bitcast<f32>(_e1330), bitcast<f32>(_e1335), bitcast<f32>(_e1340), bitcast<f32>(_e1345)), vec3<f32>(bitcast<f32>(_e1351), bitcast<f32>(_e1356), bitcast<f32>(_e1361)));
                    } else {
                        phi_2282_ = type_27(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e1366 = phi_2282_;
                    phi_2283_ = _e1366;
                } else {
                    phi_2283_ = _e1304;
                }
                let _e1368 = phi_2283_;
                let _e1376 = (_e1368.member_1.x + _e1368.member_1.x);
                let _e1377 = (_e1368.member_1.y + _e1368.member_1.y);
                let _e1378 = (_e1368.member_1.z + _e1368.member_1.z);
                let _e1380 = (_e1368.member_1.z * _e1378);
                let _e1381 = (_e1368.member_1.w * _e1376);
                let _e1382 = (_e1368.member_1.w * _e1377);
                let _e1383 = (_e1368.member_1.w * _e1378);
                let _e1403 = (vec4<f32>((1f - fma(_e1368.member_1.y, _e1377, _e1380)), fma(_e1368.member_1.x, _e1377, _e1383), fma(_e1368.member_1.x, _e1378, -(_e1382)), 0f) * _e1368.member_2.x);
                let _e1405 = (vec4<f32>(fma(_e1368.member_1.x, _e1377, -(_e1383)), (1f - fma(_e1368.member_1.x, _e1376, _e1380)), fma(_e1368.member_1.y, _e1378, _e1381), 0f) * _e1368.member_2.y);
                let _e1407 = (vec4<f32>(fma(_e1368.member_1.x, _e1378, _e1382), fma(_e1368.member_1.y, _e1378, -(_e1381)), (1f - fma(_e1368.member_1.x, _e1376, (_e1368.member_1.y * _e1377))), 0f) * _e1368.member_2.z);
                let _e1412 = local_24;
                let _e1435 = (_e1368.member.x + fma(_e1407.x, _e1412.member.z, fma(_e1405.x, _e1412.member.y, (_e1403.x * _e1412.member.x))));
                let _e1436 = (_e1368.member.y + fma(_e1407.y, _e1412.member.z, fma(_e1405.y, _e1412.member.y, (_e1403.y * _e1412.member.x))));
                let _e1437 = (_e1368.member.z + fma(_e1407.z, _e1412.member.z, fma(_e1405.z, _e1412.member.y, (_e1403.z * _e1412.member.x))));
                let _e1440 = local_25;
                global_6 = _e1440.member_1;
                let _e1443 = local_26;
                global_7 = _e1443.member_6;
                let _e1446 = local_27;
                global_8 = _e1446.member_7;
                global_9 = vec3<f32>(_e1435, _e1436, _e1437);
                let _e1452 = local_28;
                let _e1460 = sqrt(fma(_e1452.member_2.z, _e1452.member_2.z, fma(_e1452.member_2.x, _e1452.member_2.x, (_e1452.member_2.y * _e1452.member_2.y))));
                if (_e1460 == 0f) {
                    phi_2370_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_2370_ = (_e1452.member_2 * (1f / _e1460));
                }
                let _e1465 = phi_2370_;
                let _e1467 = local_29;
                let _e1476 = sqrt(fma(_e1467.member_3.z, _e1467.member_3.z, fma(_e1467.member_3.x, _e1467.member_3.x, (_e1467.member_3.y * _e1467.member_3.y))));
                if (_e1476 == 0f) {
                    phi_3522_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3522_ = (vec3<f32>(_e1467.member_3.x, _e1467.member_3.y, _e1467.member_3.z) * (1f / _e1476));
                }
                let _e1481 = phi_3522_;
                let _e1483 = (_e1465.x / (_e1368.member_2.x * _e1368.member_2.x));
                let _e1485 = (_e1465.y / (_e1368.member_2.y * _e1368.member_2.y));
                let _e1487 = (_e1465.z / (_e1368.member_2.z * _e1368.member_2.z));
                let _e1494 = fma(_e1407.x, _e1487, fma(_e1403.x, _e1483, (_e1405.x * _e1485)));
                let _e1495 = fma(_e1407.y, _e1487, fma(_e1403.y, _e1483, (_e1405.y * _e1485)));
                let _e1496 = fma(_e1407.z, _e1487, fma(_e1403.z, _e1483, (_e1405.z * _e1485)));
                let _e1501 = sqrt(fma(_e1496, _e1496, fma(_e1494, _e1494, (_e1495 * _e1495))));
                if (_e1501 == 0f) {
                    phi_3557_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3557_ = (vec3<f32>(_e1494, _e1495, _e1496) * (1f / _e1501));
                }
                let _e1506 = phi_3557_;
                global_10 = _e1506;
                let _e1516 = fma(_e1407.x, _e1481.z, fma(_e1403.x, _e1481.x, (_e1405.x * _e1481.y)));
                let _e1517 = fma(_e1407.y, _e1481.z, fma(_e1403.y, _e1481.x, (_e1405.y * _e1481.y)));
                let _e1518 = fma(_e1407.z, _e1481.z, fma(_e1403.z, _e1481.x, (_e1405.z * _e1481.y)));
                let _e1523 = sqrt(fma(_e1518, _e1518, fma(_e1516, _e1516, (_e1517 * _e1517))));
                if (_e1523 == 0f) {
                    phi_3592_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3592_ = (vec3<f32>(_e1516, _e1517, _e1518) * (1f / _e1523));
                }
                let _e1528 = phi_3592_;
                global_11 = _e1528;
                let _e1545 = local_30;
                let _e1549 = select(-1f, 1f, (_e1545.member_3.w >= 0f));
                global_12 = vec3<f32>((fma(_e1506.y, _e1528.z, -((_e1528.y * _e1506.z))) * _e1549), (fma(_e1506.z, _e1528.x, -((_e1528.z * _e1506.x))) * _e1549), (fma(_e1506.x, _e1528.y, -((_e1528.x * _e1506.y))) * _e1549));
                let _e1556 = global_3.member[_e149];
                if (_e101 >= 86u) {
                    phi_3613_ = (_e1556 <= (_e101 - 86u));
                } else {
                    phi_3613_ = false;
                }
                let _e1561 = phi_3613_;
                if _e1561 {
                    let _e1564 = global_3.member[_e1556];
                    let _e1569 = global_3.member[(_e1556 + 1u)];
                    let _e1574 = global_3.member[(_e1556 + 2u)];
                    let _e1579 = global_3.member[(_e1556 + 3u)];
                    let _e1585 = global_3.member[(_e1556 + 4u)];
                    let _e1590 = global_3.member[(_e1556 + 5u)];
                    let _e1595 = global_3.member[(_e1556 + 6u)];
                    let _e1600 = global_3.member[(_e1556 + 7u)];
                    let _e1606 = global_3.member[(_e1556 + 8u)];
                    let _e1611 = global_3.member[(_e1556 + 9u)];
                    let _e1616 = global_3.member[(_e1556 + 10u)];
                    let _e1621 = global_3.member[(_e1556 + 11u)];
                    let _e1627 = global_3.member[(_e1556 + 12u)];
                    let _e1632 = global_3.member[(_e1556 + 13u)];
                    let _e1637 = global_3.member[(_e1556 + 14u)];
                    let _e1642 = global_3.member[(_e1556 + 15u)];
                    let _e1649 = global_3.member[(_e1556 + 16u)];
                    let _e1654 = global_3.member[(_e1556 + 17u)];
                    let _e1659 = global_3.member[(_e1556 + 18u)];
                    let _e1664 = global_3.member[(_e1556 + 19u)];
                    let _e1670 = global_3.member[(_e1556 + 20u)];
                    let _e1675 = global_3.member[(_e1556 + 21u)];
                    let _e1680 = global_3.member[(_e1556 + 22u)];
                    let _e1685 = global_3.member[(_e1556 + 23u)];
                    let _e1691 = global_3.member[(_e1556 + 24u)];
                    let _e1696 = global_3.member[(_e1556 + 25u)];
                    let _e1701 = global_3.member[(_e1556 + 26u)];
                    let _e1706 = global_3.member[(_e1556 + 27u)];
                    let _e1712 = global_3.member[(_e1556 + 28u)];
                    let _e1717 = global_3.member[(_e1556 + 29u)];
                    let _e1722 = global_3.member[(_e1556 + 30u)];
                    let _e1727 = global_3.member[(_e1556 + 31u)];
                    let _e1734 = global_3.member[(_e1556 + 32u)];
                    let _e1739 = global_3.member[(_e1556 + 33u)];
                    let _e1744 = global_3.member[(_e1556 + 34u)];
                    local_3 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    phi_2657_ = type_14(0u, 6u);
                    loop {
                        let _e1749 = phi_2657_;
                        if (_e1749.member < _e1749.member_1) {
                            phi_2658_ = type_14((_e1749.member + 1u), _e1749.member_1);
                            phi_2681_ = type_14(1u, _e1749.member);
                        } else {
                            phi_2658_ = _e1749;
                            phi_2681_ = type_14(0u, type_14().member_1);
                        }
                        let _e1762 = phi_2658_;
                        let _e1764 = phi_2681_;
                        switch bitcast<i32>(_e1764.member) {
                            case 0: {
                                phi_2708_ = false;
                                break;
                            }
                            case 1: {
                                let _e1769 = ((_e1556 + 35u) + (_e1764.member_1 * 4u));
                                let _e1772 = global_3.member[_e1769];
                                let _e1777 = global_3.member[(_e1769 + 1u)];
                                let _e1782 = global_3.member[(_e1769 + 2u)];
                                let _e1787 = global_3.member[(_e1769 + 3u)];
                                local_3[_e1764.member_1] = vec4<f32>(bitcast<f32>(_e1772), bitcast<f32>(_e1777), bitcast<f32>(_e1782), bitcast<f32>(_e1787));
                                phi_2708_ = true;
                                break;
                            }
                            default: {
                                phi_2708_ = bool();
                                break;
                            }
                        }
                        let _e1792 = phi_2708_;
                        continue;
                        continuing {
                            phi_2657_ = _e1762;
                            break if !(_e1792);
                        }
                    }
                    let _e1794 = local_3;
                    local_2 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                    phi_2714_ = type_14(0u, 8u);
                    loop {
                        let _e1797 = phi_2714_;
                        if (_e1797.member < _e1797.member_1) {
                            phi_2715_ = type_14((_e1797.member + 1u), _e1797.member_1);
                            phi_2738_ = type_14(1u, _e1797.member);
                        } else {
                            phi_2715_ = _e1797;
                            phi_2738_ = type_14(0u, type_14().member_1);
                        }
                        let _e1810 = phi_2715_;
                        let _e1812 = phi_2738_;
                        switch bitcast<i32>(_e1812.member) {
                            case 0: {
                                phi_2761_ = false;
                                break;
                            }
                            case 1: {
                                let _e1817 = ((_e1556 + 59u) + (_e1812.member_1 * 3u));
                                let _e1820 = global_3.member[_e1817];
                                let _e1825 = global_3.member[(_e1817 + 1u)];
                                let _e1830 = global_3.member[(_e1817 + 2u)];
                                local_2[_e1812.member_1] = vec3<f32>(bitcast<f32>(_e1820), bitcast<f32>(_e1825), bitcast<f32>(_e1830));
                                phi_2761_ = true;
                                break;
                            }
                            default: {
                                phi_2761_ = bool();
                                break;
                            }
                        }
                        let _e1835 = phi_2761_;
                        continue;
                        continuing {
                            phi_2714_ = _e1810;
                            break if !(_e1835);
                        }
                    }
                    let _e1837 = local_2;
                    let _e1841 = global_3.member[(_e1556 + 83u)];
                    let _e1846 = global_3.member[(_e1556 + 84u)];
                    let _e1851 = global_3.member[(_e1556 + 85u)];
                    phi_2782_ = type_23(type_21(vec4<f32>(bitcast<f32>(_e1564), bitcast<f32>(_e1569), bitcast<f32>(_e1574), bitcast<f32>(_e1579)), vec4<f32>(bitcast<f32>(_e1585), bitcast<f32>(_e1590), bitcast<f32>(_e1595), bitcast<f32>(_e1600)), vec4<f32>(bitcast<f32>(_e1606), bitcast<f32>(_e1611), bitcast<f32>(_e1616), bitcast<f32>(_e1621)), vec4<f32>(bitcast<f32>(_e1627), bitcast<f32>(_e1632), bitcast<f32>(_e1637), bitcast<f32>(_e1642))), type_21(vec4<f32>(bitcast<f32>(_e1649), bitcast<f32>(_e1654), bitcast<f32>(_e1659), bitcast<f32>(_e1664)), vec4<f32>(bitcast<f32>(_e1670), bitcast<f32>(_e1675), bitcast<f32>(_e1680), bitcast<f32>(_e1685)), vec4<f32>(bitcast<f32>(_e1691), bitcast<f32>(_e1696), bitcast<f32>(_e1701), bitcast<f32>(_e1706)), vec4<f32>(bitcast<f32>(_e1712), bitcast<f32>(_e1717), bitcast<f32>(_e1722), bitcast<f32>(_e1727))), vec3<f32>(bitcast<f32>(_e1734), bitcast<f32>(_e1739), bitcast<f32>(_e1744)), type_22(_e1837, _e1794, vec3<f32>(bitcast<f32>(_e1841), bitcast<f32>(_e1846), bitcast<f32>(_e1851))));
                } else {
                    phi_2782_ = type_23(type_21(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_21(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_22(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
                }
                let _e1857 = phi_2782_;
                global_1 = vec4<f32>((fma(fma(_e1857.member.member_3.x, _e1857.member_1.member_2.w, fma(_e1857.member.member_2.x, _e1857.member_1.member_2.z, fma(_e1857.member.member.x, _e1857.member_1.member_2.x, (_e1857.member.member_1.x * _e1857.member_1.member_2.y)))), _e1437, fma(fma(_e1857.member.member_3.x, _e1857.member_1.member.w, fma(_e1857.member.member_2.x, _e1857.member_1.member.z, fma(_e1857.member.member.x, _e1857.member_1.member.x, (_e1857.member.member_1.x * _e1857.member_1.member.y)))), _e1435, (fma(_e1857.member.member_3.x, _e1857.member_1.member_1.w, fma(_e1857.member.member_2.x, _e1857.member_1.member_1.z, fma(_e1857.member.member.x, _e1857.member_1.member_1.x, (_e1857.member.member_1.x * _e1857.member_1.member_1.y)))) * _e1436))) + fma(_e1857.member.member_3.x, _e1857.member_1.member_3.w, fma(_e1857.member.member_2.x, _e1857.member_1.member_3.z, fma(_e1857.member.member.x, _e1857.member_1.member_3.x, (_e1857.member.member_1.x * _e1857.member_1.member_3.y))))), (fma(fma(_e1857.member.member_3.y, _e1857.member_1.member_2.w, fma(_e1857.member.member_2.y, _e1857.member_1.member_2.z, fma(_e1857.member.member.y, _e1857.member_1.member_2.x, (_e1857.member.member_1.y * _e1857.member_1.member_2.y)))), _e1437, fma(fma(_e1857.member.member_3.y, _e1857.member_1.member.w, fma(_e1857.member.member_2.y, _e1857.member_1.member.z, fma(_e1857.member.member.y, _e1857.member_1.member.x, (_e1857.member.member_1.y * _e1857.member_1.member.y)))), _e1435, (fma(_e1857.member.member_3.y, _e1857.member_1.member_1.w, fma(_e1857.member.member_2.y, _e1857.member_1.member_1.z, fma(_e1857.member.member.y, _e1857.member_1.member_1.x, (_e1857.member.member_1.y * _e1857.member_1.member_1.y)))) * _e1436))) + fma(_e1857.member.member_3.y, _e1857.member_1.member_3.w, fma(_e1857.member.member_2.y, _e1857.member_1.member_3.z, fma(_e1857.member.member.y, _e1857.member_1.member_3.x, (_e1857.member.member_1.y * _e1857.member_1.member_3.y))))), (fma(fma(_e1857.member.member_3.z, _e1857.member_1.member_2.w, fma(_e1857.member.member_2.z, _e1857.member_1.member_2.z, fma(_e1857.member.member.z, _e1857.member_1.member_2.x, (_e1857.member.member_1.z * _e1857.member_1.member_2.y)))), _e1437, fma(fma(_e1857.member.member_3.z, _e1857.member_1.member.w, fma(_e1857.member.member_2.z, _e1857.member_1.member.z, fma(_e1857.member.member.z, _e1857.member_1.member.x, (_e1857.member.member_1.z * _e1857.member_1.member.y)))), _e1435, (fma(_e1857.member.member_3.z, _e1857.member_1.member_1.w, fma(_e1857.member.member_2.z, _e1857.member_1.member_1.z, fma(_e1857.member.member.z, _e1857.member_1.member_1.x, (_e1857.member.member_1.z * _e1857.member_1.member_1.y)))) * _e1436))) + fma(_e1857.member.member_3.z, _e1857.member_1.member_3.w, fma(_e1857.member.member_2.z, _e1857.member_1.member_3.z, fma(_e1857.member.member.z, _e1857.member_1.member_3.x, (_e1857.member.member_1.z * _e1857.member_1.member_3.y))))), (fma(fma(_e1857.member.member_3.w, _e1857.member_1.member_2.w, fma(_e1857.member.member_2.w, _e1857.member_1.member_2.z, fma(_e1857.member.member.w, _e1857.member_1.member_2.x, (_e1857.member.member_1.w * _e1857.member_1.member_2.y)))), _e1437, fma(fma(_e1857.member.member_3.w, _e1857.member_1.member.w, fma(_e1857.member.member_2.w, _e1857.member_1.member.z, fma(_e1857.member.member.w, _e1857.member_1.member.x, (_e1857.member.member_1.w * _e1857.member_1.member.y)))), _e1435, (fma(_e1857.member.member_3.w, _e1857.member_1.member_1.w, fma(_e1857.member.member_2.w, _e1857.member_1.member_1.z, fma(_e1857.member.member.w, _e1857.member_1.member_1.x, (_e1857.member.member_1.w * _e1857.member_1.member_1.y)))) * _e1436))) + fma(_e1857.member.member_3.w, _e1857.member_1.member_3.w, fma(_e1857.member.member_2.w, _e1857.member_1.member_3.z, fma(_e1857.member.member.w, _e1857.member_1.member_3.x, (_e1857.member.member_1.w * _e1857.member_1.member_3.y))))));
            } else {
                global_1 = vec4<f32>(10f, 10f, 10f, 1f);
            }
            break;
        }
    }
    return;
}

@vertex 
fn stagerenderlet_vertex(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_2 = param;
    global = param_1;
    function();
    let _e14 = global_1.y;
    global_1.y = -(_e14);
    let _e16 = global_5;
    let _e17 = global_6;
    let _e18 = global_7;
    let _e19 = global_8;
    let _e20 = global_10;
    let _e21 = global_11;
    let _e22 = global_12;
    let _e23 = global_9;
    let _e24 = global_1;
    return VertexOutput(_e16, _e17, _e18, _e19, _e20, _e21, _e22, _e23, _e24);
}
