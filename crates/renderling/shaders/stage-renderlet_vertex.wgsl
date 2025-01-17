struct type_12 {
    member: array<u32>,
}

struct type_20 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_21 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
}

struct type_22 {
    member: type_20,
    member_1: type_20,
    member_2: type_21,
    member_3: vec3<f32>,
}

struct type_24 {
    member: u32,
    member_1: u32,
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
    member: type_24,
    member_1: type_24,
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
@group(0) @binding(0) 
var<storage> global_2: type_12;
var<private> global_3: u32;
var<private> global_4: u32;
var<private> global_5: vec4<f32>;
var<private> global_6: vec2<f32>;
var<private> global_7: vec2<f32>;
var<private> global_8: type_20 = type_20(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
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
    var phi_743_: u32;
    var phi_2896_: bool;
    var phi_750_: u32;
    var phi_751_: u32;
    var phi_761_: u32;
    var phi_843_: type_24;
    var phi_844_: type_24;
    var phi_867_: type_24;
    var phi_880_: bool;
    var phi_886_: type_24;
    var phi_887_: type_24;
    var phi_910_: type_24;
    var phi_924_: bool;
    var phi_930_: type_24;
    var phi_933_: type_30;
    var phi_931_: type_24;
    var phi_956_: type_24;
    var phi_973_: u32;
    var phi_2926_: bool;
    var phi_991_: type_24;
    var phi_2952_: u32;
    var phi_2971_: bool;
    var phi_1041_: type_33;
    var phi_1051_: u32;
    var phi_2993_: bool;
    var phi_1059_: f32;
    var phi_934_: type_30;
    var phi_1112_: bool;
    var local_6: type_30;
    var local_7: type_30;
    var local_8: type_30;
    var phi_3016_: bool;
    var phi_1236_: type_34;
    var local_9: type_30;
    var phi_1239_: type_24;
    var phi_1242_: type_20;
    var phi_1240_: type_24;
    var phi_1265_: type_24;
    var local_10: type_30;
    var phi_1289_: u32;
    var phi_3050_: bool;
    var phi_1298_: u32;
    var phi_3074_: bool;
    var phi_1347_: type_27;
    var phi_1357_: u32;
    var phi_3099_: bool;
    var phi_1430_: type_20;
    var phi_1243_: type_20;
    var phi_1667_: bool;
    var phi_3915_: bool;
    var local_11: type_20;
    var local_12: type_20;
    var local_13: type_20;
    var local_14: type_20;
    var phi_1694_: bool;
    var phi_1696_: bool;
    var phi_1697_: bool;
    var phi_1698_: bool;
    var phi_1699_: bool;
    var local_15: type_20;
    var local_16: type_20;
    var local_17: type_20;
    var local_18: type_20;
    var phi_1733_: bool;
    var phi_1735_: bool;
    var phi_1736_: bool;
    var phi_1737_: bool;
    var phi_1738_: bool;
    var local_19: type_20;
    var local_20: type_20;
    var local_21: type_20;
    var local_22: type_20;
    var phi_1772_: bool;
    var phi_1774_: bool;
    var phi_1775_: bool;
    var phi_1776_: bool;
    var phi_1777_: bool;
    var local_23: type_20;
    var local_24: type_20;
    var local_25: type_20;
    var local_26: type_20;
    var phi_1811_: bool;
    var phi_1813_: bool;
    var phi_1814_: bool;
    var phi_1815_: bool;
    var phi_1816_: bool;
    var phi_1821_: bool;
    var phi_1823_: bool;
    var phi_1824_: bool;
    var phi_1825_: bool;
    var phi_1826_: bool;
    var phi_1834_: type_20;
    var phi_3234_: bool;
    var phi_3299_: vec4<f32>;
    var phi_3329_: vec4<f32>;
    var phi_3331_: vec4<f32>;
    var phi_3340_: type_27;
    var phi_3341_: type_27;
    var phi_3346_: type_27;
    var phi_3347_: type_27;
    var phi_3348_: bool;
    var phi_3352_: type_27;
    var phi_1836_: type_27;
    var phi_1838_: type_27;
    var phi_1839_: bool;
    var phi_3446_: bool;
    var phi_1892_: type_27;
    var phi_1893_: type_27;
    var local_27: type_30;
    var phi_1937_: vec3<f32>;
    var local_28: type_30;
    var phi_3490_: vec3<f32>;
    var phi_3572_: vec3<f32>;
    var phi_3607_: vec3<f32>;
    var local_29: type_30;
    var local_30: type_30;
    var phi_3620_: bool;
    var phi_2261_: type_24;
    var phi_2262_: type_24;
    var phi_2285_: type_24;
    var phi_2312_: bool;
    var phi_2318_: type_24;
    var phi_2319_: type_24;
    var phi_2342_: type_24;
    var phi_2365_: bool;
    var phi_2373_: type_22;
    var local_31: type_20;

    switch bitcast<i32>(0u) {
        default: {
            let _e95 = global_3;
            let _e96 = global;
            let _e98 = arrayLength((&global_2.member));
            let _e101 = global_2.member[_e95];
            let _e106 = global_2.member[(_e95 + 1u)];
            let _e110 = global_2.member[(_e95 + 2u)];
            let _e114 = global_2.member[(_e95 + 7u)];
            let _e118 = global_2.member[(_e95 + 8u)];
            let _e122 = global_2.member[(_e95 + 9u)];
            let _e126 = global_2.member[(_e95 + 10u)];
            let _e130 = global_2.member[(_e95 + 12u)];
            let _e134 = global_2.member[(_e95 + 13u)];
            let _e138 = global_2.member[(_e95 + 14u)];
            let _e142 = global_2.member[(_e95 + 15u)];
            let _e146 = global_2.member[(_e95 + 16u)];
            let _e150 = global_2.member[(_e95 + 17u)];
            if (_e101 == 1u) {
                global_4 = _e95;
                if (_e114 == 4294967295u) {
                    phi_751_ = _e96;
                } else {
                    if (_e96 >= _e118) {
                        phi_743_ = 4294967295u;
                    } else {
                        phi_743_ = (_e114 + _e96);
                    }
                    let _e155 = phi_743_;
                    if (_e98 >= 1u) {
                        phi_2896_ = (_e155 <= (_e98 - 1u));
                    } else {
                        phi_2896_ = false;
                    }
                    let _e160 = phi_2896_;
                    if _e160 {
                        let _e163 = global_2.member[_e155];
                        phi_750_ = _e163;
                    } else {
                        phi_750_ = 0u;
                    }
                    let _e165 = phi_750_;
                    phi_751_ = _e165;
                }
                let _e167 = phi_751_;
                if (_e167 >= _e110) {
                    phi_761_ = 4294967295u;
                } else {
                    phi_761_ = (_e106 + (26u * _e167));
                }
                let _e172 = phi_761_;
                let _e175 = global_2.member[_e172];
                let _e180 = global_2.member[(_e172 + 1u)];
                let _e185 = global_2.member[(_e172 + 2u)];
                let _e191 = global_2.member[(_e172 + 3u)];
                let _e196 = global_2.member[(_e172 + 4u)];
                let _e201 = global_2.member[(_e172 + 5u)];
                let _e206 = global_2.member[(_e172 + 6u)];
                let _e212 = global_2.member[(_e172 + 7u)];
                let _e217 = global_2.member[(_e172 + 8u)];
                let _e223 = global_2.member[(_e172 + 9u)];
                let _e228 = global_2.member[(_e172 + 10u)];
                let _e234 = global_2.member[(_e172 + 11u)];
                let _e239 = global_2.member[(_e172 + 12u)];
                let _e244 = global_2.member[(_e172 + 13u)];
                let _e250 = global_2.member[(_e172 + 14u)];
                let _e255 = global_2.member[(_e172 + 15u)];
                let _e260 = global_2.member[(_e172 + 16u)];
                let _e265 = global_2.member[(_e172 + 17u)];
                local_5 = array<u32, 4>(0u, 0u, 0u, 0u);
                phi_843_ = type_24(0u, 4u);
                loop {
                    let _e270 = phi_843_;
                    if (_e270.member < _e270.member_1) {
                        phi_844_ = type_24((_e270.member + 1u), _e270.member_1);
                        phi_867_ = type_24(1u, _e270.member);
                    } else {
                        phi_844_ = _e270;
                        phi_867_ = type_24(0u, type_24().member_1);
                    }
                    let _e283 = phi_844_;
                    let _e285 = phi_867_;
                    switch bitcast<i32>(_e285.member) {
                        case 0: {
                            phi_880_ = false;
                            break;
                        }
                        case 1: {
                            let _e292 = global_2.member[((_e172 + 18u) + _e285.member_1)];
                            local_5[_e285.member_1] = _e292;
                            phi_880_ = true;
                            break;
                        }
                        default: {
                            phi_880_ = bool();
                            break;
                        }
                    }
                    let _e295 = phi_880_;
                    continue;
                    continuing {
                        phi_843_ = _e283;
                        break if !(_e295);
                    }
                }
                let _e297 = local_5;
                local_4 = array<f32, 4>(0f, 0f, 0f, 0f);
                phi_886_ = type_24(0u, 4u);
                loop {
                    let _e300 = phi_886_;
                    if (_e300.member < _e300.member_1) {
                        phi_887_ = type_24((_e300.member + 1u), _e300.member_1);
                        phi_910_ = type_24(1u, _e300.member);
                    } else {
                        phi_887_ = _e300;
                        phi_910_ = type_24(0u, type_24().member_1);
                    }
                    let _e313 = phi_887_;
                    let _e315 = phi_910_;
                    switch bitcast<i32>(_e315.member) {
                        case 0: {
                            phi_924_ = false;
                            break;
                        }
                        case 1: {
                            let _e322 = global_2.member[((_e172 + 22u) + _e315.member_1)];
                            local_4[_e315.member_1] = bitcast<f32>(_e322);
                            phi_924_ = true;
                            break;
                        }
                        default: {
                            phi_924_ = bool();
                            break;
                        }
                    }
                    let _e326 = phi_924_;
                    continue;
                    continuing {
                        phi_886_ = _e313;
                        break if !(_e326);
                    }
                }
                let _e328 = local_4;
                phi_930_ = type_24(0u, _e138);
                phi_933_ = type_30(vec3<f32>(bitcast<f32>(_e175), bitcast<f32>(_e180), bitcast<f32>(_e185)), vec4<f32>(bitcast<f32>(_e191), bitcast<f32>(_e196), bitcast<f32>(_e201), bitcast<f32>(_e206)), vec3<f32>(bitcast<f32>(_e234), bitcast<f32>(_e239), bitcast<f32>(_e244)), vec4<f32>(bitcast<f32>(_e250), bitcast<f32>(_e255), bitcast<f32>(_e260), bitcast<f32>(_e265)), _e297, _e328, vec2<f32>(bitcast<f32>(_e212), bitcast<f32>(_e217)), vec2<f32>(bitcast<f32>(_e223), bitcast<f32>(_e228)));
                loop {
                    let _e332 = phi_930_;
                    let _e334 = phi_933_;
                    local_6 = _e334;
                    local_7 = _e334;
                    local_8 = _e334;
                    local_9 = _e334;
                    local_10 = _e334;
                    local_27 = _e334;
                    local_28 = _e334;
                    local_29 = _e334;
                    local_30 = _e334;
                    if (_e332.member < _e332.member_1) {
                        phi_931_ = type_24((_e332.member + 1u), _e332.member_1);
                        phi_956_ = type_24(1u, _e332.member);
                    } else {
                        phi_931_ = _e332;
                        phi_956_ = type_24(0u, type_24().member_1);
                    }
                    let _e347 = phi_931_;
                    let _e349 = phi_956_;
                    switch bitcast<i32>(_e349.member) {
                        case 0: {
                            phi_934_ = type_30();
                            phi_1112_ = false;
                            break;
                        }
                        case 1: {
                            if (_e349.member_1 >= _e138) {
                                phi_973_ = 4294967295u;
                            } else {
                                phi_973_ = (_e134 + (2u * _e349.member_1));
                            }
                            let _e357 = phi_973_;
                            if (_e98 >= 2u) {
                                phi_2926_ = (_e357 <= (_e98 - 2u));
                            } else {
                                phi_2926_ = false;
                            }
                            let _e362 = phi_2926_;
                            if _e362 {
                                let _e365 = global_2.member[_e357];
                                let _e369 = global_2.member[(_e357 + 1u)];
                                phi_991_ = type_24(_e365, _e369);
                            } else {
                                phi_991_ = type_24(4294967295u, 0u);
                            }
                            let _e372 = phi_991_;
                            if (_e167 >= _e372.member_1) {
                                phi_2952_ = 4294967295u;
                            } else {
                                phi_2952_ = (_e372.member + (9u * _e167));
                            }
                            let _e379 = phi_2952_;
                            if (_e98 >= 9u) {
                                phi_2971_ = (_e379 <= (_e98 - 9u));
                            } else {
                                phi_2971_ = false;
                            }
                            let _e384 = phi_2971_;
                            if _e384 {
                                let _e387 = global_2.member[_e379];
                                let _e392 = global_2.member[(_e379 + 1u)];
                                let _e397 = global_2.member[(_e379 + 2u)];
                                let _e403 = global_2.member[(_e379 + 3u)];
                                let _e408 = global_2.member[(_e379 + 4u)];
                                let _e413 = global_2.member[(_e379 + 5u)];
                                let _e419 = global_2.member[(_e379 + 6u)];
                                let _e424 = global_2.member[(_e379 + 7u)];
                                let _e429 = global_2.member[(_e379 + 8u)];
                                phi_1041_ = type_33(vec3<f32>(bitcast<f32>(_e387), bitcast<f32>(_e392), bitcast<f32>(_e397)), vec3<f32>(bitcast<f32>(_e403), bitcast<f32>(_e408), bitcast<f32>(_e413)), vec3<f32>(bitcast<f32>(_e419), bitcast<f32>(_e424), bitcast<f32>(_e429)));
                            } else {
                                phi_1041_ = type_33(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e434 = phi_1041_;
                            if (_e349.member_1 >= _e146) {
                                phi_1051_ = 4294967295u;
                            } else {
                                phi_1051_ = (_e142 + _e349.member_1);
                            }
                            let _e438 = phi_1051_;
                            if (_e98 >= 1u) {
                                phi_2993_ = (_e438 <= (_e98 - 1u));
                            } else {
                                phi_2993_ = false;
                            }
                            let _e443 = phi_2993_;
                            if _e443 {
                                let _e446 = global_2.member[_e438];
                                phi_1059_ = bitcast<f32>(_e446);
                            } else {
                                phi_1059_ = 0f;
                            }
                            let _e449 = phi_1059_;
                            let _e472 = type_30(vec3<f32>(fma(_e449, _e434.member.x, _e334.member.x), fma(_e449, _e434.member.y, _e334.member.y), fma(_e449, _e434.member.z, _e334.member.z)), _e334.member_1, _e334.member_2, _e334.member_3, _e334.member_4, _e334.member_5, _e334.member_6, _e334.member_7);
                            let _e495 = type_30(_e472.member, _e472.member_1, vec3<f32>(fma(_e449, _e434.member_1.x, _e334.member_2.x), fma(_e449, _e434.member_1.y, _e334.member_2.y), fma(_e449, _e434.member_1.z, _e334.member_2.z)), _e472.member_3, _e472.member_4, _e472.member_5, _e472.member_6, _e472.member_7);
                            phi_934_ = type_30(_e495.member, _e495.member_1, _e495.member_2, vec4<f32>(fma(_e449, _e434.member_2.x, _e334.member_3.x), fma(_e449, _e434.member_2.y, _e334.member_3.y), fma(_e449, _e434.member_2.z, _e334.member_3.z), _e334.member_3.w), _e495.member_4, _e495.member_5, _e495.member_6, _e495.member_7);
                            phi_1112_ = true;
                            break;
                        }
                        default: {
                            phi_934_ = type_30();
                            phi_1112_ = bool();
                            break;
                        }
                    }
                    let _e522 = phi_934_;
                    let _e524 = phi_1112_;
                    continue;
                    continuing {
                        phi_930_ = _e347;
                        phi_933_ = _e522;
                        break if !(_e524);
                    }
                }
                let _e527 = local_6;
                global_5 = _e527.member_1;
                let _e530 = local_7;
                global_6 = _e530.member_6;
                let _e533 = local_8;
                global_7 = _e533.member_7;
                let _e538 = global_2.member[(_e150 + 6u)];
                if (_e538 == 1u) {
                    let _e541 = ((_e130 == 4294967295u) != true);
                    if _e541 {
                        if (_e98 >= 4u) {
                            phi_3016_ = (_e130 <= (_e98 - 4u));
                        } else {
                            phi_3016_ = false;
                        }
                        let _e546 = phi_3016_;
                        if _e546 {
                            let _e549 = global_2.member[_e130];
                            let _e553 = global_2.member[(_e130 + 1u)];
                            let _e557 = global_2.member[(_e130 + 2u)];
                            let _e561 = global_2.member[(_e130 + 3u)];
                            phi_1236_ = type_34(type_24(_e549, _e553), type_24(_e557, _e561));
                        } else {
                            phi_1236_ = type_34(type_24(4294967295u, 0u), type_24(4294967295u, 0u));
                        }
                        let _e566 = phi_1236_;
                        let _e568 = local_9;
                        local = _e568.member_5;
                        phi_1239_ = type_24(0u, 4u);
                        phi_1242_ = type_20(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        loop {
                            let _e571 = phi_1239_;
                            let _e573 = phi_1242_;
                            local_11 = _e573;
                            local_12 = _e573;
                            local_13 = _e573;
                            local_14 = _e573;
                            local_15 = _e573;
                            local_16 = _e573;
                            local_17 = _e573;
                            local_18 = _e573;
                            local_19 = _e573;
                            local_20 = _e573;
                            local_21 = _e573;
                            local_22 = _e573;
                            local_23 = _e573;
                            local_24 = _e573;
                            local_25 = _e573;
                            local_26 = _e573;
                            local_31 = _e573;
                            if (_e571.member < _e571.member_1) {
                                phi_1240_ = type_24((_e571.member + 1u), _e571.member_1);
                                phi_1265_ = type_24(1u, _e571.member);
                            } else {
                                phi_1240_ = _e571;
                                phi_1265_ = type_24(0u, type_24().member_1);
                            }
                            let _e586 = phi_1240_;
                            let _e588 = phi_1265_;
                            switch bitcast<i32>(_e588.member) {
                                case 0: {
                                    phi_1243_ = type_20();
                                    phi_1667_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e593 = local_10;
                                    local_1 = _e593.member_4;
                                    let _e595 = (_e588.member_1 < 4u);
                                    if _e595 {
                                    } else {
                                        phi_3915_ = true;
                                        break;
                                    }
                                    let _e597 = local_1[_e588.member_1];
                                    if (_e597 >= _e566.member.member_1) {
                                        phi_1289_ = 4294967295u;
                                    } else {
                                        phi_1289_ = (_e566.member.member + _e597);
                                    }
                                    let _e605 = phi_1289_;
                                    if (_e98 >= 1u) {
                                        phi_3050_ = (_e605 <= (_e98 - 1u));
                                    } else {
                                        phi_3050_ = false;
                                    }
                                    let _e610 = phi_3050_;
                                    if _e610 {
                                        let _e613 = global_2.member[_e605];
                                        phi_1298_ = _e613;
                                    } else {
                                        phi_1298_ = 4294967295u;
                                    }
                                    let _e615 = phi_1298_;
                                    if (_e98 >= 10u) {
                                        phi_3074_ = (_e615 <= (_e98 - 10u));
                                    } else {
                                        phi_3074_ = false;
                                    }
                                    let _e620 = phi_3074_;
                                    if _e620 {
                                        let _e623 = global_2.member[_e615];
                                        let _e628 = global_2.member[(_e615 + 1u)];
                                        let _e633 = global_2.member[(_e615 + 2u)];
                                        let _e639 = global_2.member[(_e615 + 3u)];
                                        let _e644 = global_2.member[(_e615 + 4u)];
                                        let _e649 = global_2.member[(_e615 + 5u)];
                                        let _e654 = global_2.member[(_e615 + 6u)];
                                        let _e660 = global_2.member[(_e615 + 7u)];
                                        let _e665 = global_2.member[(_e615 + 8u)];
                                        let _e670 = global_2.member[(_e615 + 9u)];
                                        phi_1347_ = type_27(vec3<f32>(bitcast<f32>(_e623), bitcast<f32>(_e628), bitcast<f32>(_e633)), vec4<f32>(bitcast<f32>(_e639), bitcast<f32>(_e644), bitcast<f32>(_e649), bitcast<f32>(_e654)), vec3<f32>(bitcast<f32>(_e660), bitcast<f32>(_e665), bitcast<f32>(_e670)));
                                    } else {
                                        phi_1347_ = type_27(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e675 = phi_1347_;
                                    if (_e597 >= _e566.member_1.member_1) {
                                        phi_1357_ = 4294967295u;
                                    } else {
                                        phi_1357_ = (_e566.member_1.member + (16u * _e597));
                                    }
                                    let _e684 = phi_1357_;
                                    if (_e98 >= 16u) {
                                        phi_3099_ = (_e684 <= (_e98 - 16u));
                                    } else {
                                        phi_3099_ = false;
                                    }
                                    let _e689 = phi_3099_;
                                    if _e689 {
                                        let _e692 = global_2.member[_e684];
                                        let _e697 = global_2.member[(_e684 + 1u)];
                                        let _e702 = global_2.member[(_e684 + 2u)];
                                        let _e707 = global_2.member[(_e684 + 3u)];
                                        let _e713 = global_2.member[(_e684 + 4u)];
                                        let _e718 = global_2.member[(_e684 + 5u)];
                                        let _e723 = global_2.member[(_e684 + 6u)];
                                        let _e728 = global_2.member[(_e684 + 7u)];
                                        let _e734 = global_2.member[(_e684 + 8u)];
                                        let _e739 = global_2.member[(_e684 + 9u)];
                                        let _e744 = global_2.member[(_e684 + 10u)];
                                        let _e749 = global_2.member[(_e684 + 11u)];
                                        let _e755 = global_2.member[(_e684 + 12u)];
                                        let _e760 = global_2.member[(_e684 + 13u)];
                                        let _e765 = global_2.member[(_e684 + 14u)];
                                        let _e770 = global_2.member[(_e684 + 15u)];
                                        phi_1430_ = type_20(vec4<f32>(bitcast<f32>(_e692), bitcast<f32>(_e697), bitcast<f32>(_e702), bitcast<f32>(_e707)), vec4<f32>(bitcast<f32>(_e713), bitcast<f32>(_e718), bitcast<f32>(_e723), bitcast<f32>(_e728)), vec4<f32>(bitcast<f32>(_e734), bitcast<f32>(_e739), bitcast<f32>(_e744), bitcast<f32>(_e749)), vec4<f32>(bitcast<f32>(_e755), bitcast<f32>(_e760), bitcast<f32>(_e765), bitcast<f32>(_e770)));
                                    } else {
                                        phi_1430_ = type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                    }
                                    let _e775 = phi_1430_;
                                    let _e783 = (_e675.member_1.x + _e675.member_1.x);
                                    let _e784 = (_e675.member_1.y + _e675.member_1.y);
                                    let _e785 = (_e675.member_1.z + _e675.member_1.z);
                                    let _e787 = (_e675.member_1.z * _e785);
                                    let _e788 = (_e675.member_1.w * _e783);
                                    let _e789 = (_e675.member_1.w * _e784);
                                    let _e790 = (_e675.member_1.w * _e785);
                                    let _e810 = (vec4<f32>((1f - fma(_e675.member_1.y, _e784, _e787)), fma(_e675.member_1.x, _e784, _e790), fma(_e675.member_1.x, _e785, -(_e789)), 0f) * _e675.member_2.x);
                                    let _e812 = (vec4<f32>(fma(_e675.member_1.x, _e784, -(_e790)), (1f - fma(_e675.member_1.x, _e783, _e787)), fma(_e675.member_1.y, _e785, _e788), 0f) * _e675.member_2.y);
                                    let _e814 = (vec4<f32>(fma(_e675.member_1.x, _e785, _e789), fma(_e675.member_1.y, _e785, -(_e788)), (1f - fma(_e675.member_1.x, _e783, (_e675.member_1.y * _e784))), 0f) * _e675.member_2.z);
                                    if _e595 {
                                    } else {
                                        phi_3915_ = true;
                                        break;
                                    }
                                    let _e919 = local[_e588.member_1];
                                    phi_1243_ = type_20((_e573.member + (vec4<f32>(fma(_e675.member.x, _e775.member.w, fma(_e814.x, _e775.member.z, fma(_e810.x, _e775.member.x, (_e812.x * _e775.member.y)))), fma(_e675.member.y, _e775.member.w, fma(_e814.y, _e775.member.z, fma(_e810.y, _e775.member.x, (_e812.y * _e775.member.y)))), fma(_e675.member.z, _e775.member.w, fma(_e814.z, _e775.member.z, fma(_e810.z, _e775.member.x, (_e812.z * _e775.member.y)))), (fma(_e814.w, _e775.member.z, fma(_e810.w, _e775.member.x, (_e812.w * _e775.member.y))) + _e775.member.w)) * _e919)), (_e573.member_1 + (vec4<f32>(fma(_e675.member.x, _e775.member_1.w, fma(_e814.x, _e775.member_1.z, fma(_e810.x, _e775.member_1.x, (_e812.x * _e775.member_1.y)))), fma(_e675.member.y, _e775.member_1.w, fma(_e814.y, _e775.member_1.z, fma(_e810.y, _e775.member_1.x, (_e812.y * _e775.member_1.y)))), fma(_e675.member.z, _e775.member_1.w, fma(_e814.z, _e775.member_1.z, fma(_e810.z, _e775.member_1.x, (_e812.z * _e775.member_1.y)))), (fma(_e814.w, _e775.member_1.z, fma(_e810.w, _e775.member_1.x, (_e812.w * _e775.member_1.y))) + _e775.member_1.w)) * _e919)), (_e573.member_2 + (vec4<f32>(fma(_e675.member.x, _e775.member_2.w, fma(_e814.x, _e775.member_2.z, fma(_e810.x, _e775.member_2.x, (_e812.x * _e775.member_2.y)))), fma(_e675.member.y, _e775.member_2.w, fma(_e814.y, _e775.member_2.z, fma(_e810.y, _e775.member_2.x, (_e812.y * _e775.member_2.y)))), fma(_e675.member.z, _e775.member_2.w, fma(_e814.z, _e775.member_2.z, fma(_e810.z, _e775.member_2.x, (_e812.z * _e775.member_2.y)))), (fma(_e814.w, _e775.member_2.z, fma(_e810.w, _e775.member_2.x, (_e812.w * _e775.member_2.y))) + _e775.member_2.w)) * _e919)), (_e573.member_3 + (vec4<f32>(fma(_e675.member.x, _e775.member_3.w, fma(_e814.x, _e775.member_3.z, fma(_e810.x, _e775.member_3.x, (_e812.x * _e775.member_3.y)))), fma(_e675.member.y, _e775.member_3.w, fma(_e814.y, _e775.member_3.z, fma(_e810.y, _e775.member_3.x, (_e812.y * _e775.member_3.y)))), fma(_e675.member.z, _e775.member_3.w, fma(_e814.z, _e775.member_3.z, fma(_e810.z, _e775.member_3.x, (_e812.z * _e775.member_3.y)))), (fma(_e814.w, _e775.member_3.z, fma(_e810.w, _e775.member_3.x, (_e812.w * _e775.member_3.y))) + _e775.member_3.w)) * _e919)));
                                    phi_1667_ = true;
                                    break;
                                }
                                default: {
                                    phi_1243_ = type_20();
                                    phi_1667_ = bool();
                                    break;
                                }
                            }
                            let _e934 = phi_1243_;
                            let _e936 = phi_1667_;
                            continue;
                            continuing {
                                phi_1239_ = _e586;
                                phi_1242_ = _e934;
                                phi_3915_ = false;
                                break if !(_e936);
                            }
                        }
                        let _e939 = phi_3915_;
                        if _e939 {
                            break;
                        }
                        let _e941 = local_11;
                        let _e946 = global_8.member[0u];
                        if (_e941.member.x == _e946) {
                            let _e949 = local_12;
                            let _e954 = global_8.member[1u];
                            if (_e949.member.y == _e954) {
                                let _e957 = local_13;
                                let _e962 = global_8.member[2u];
                                let _e963 = (_e957.member.z == _e962);
                                if _e963 {
                                    let _e965 = local_14;
                                    let _e970 = global_8.member[3u];
                                    phi_1694_ = (_e965.member.w == _e970);
                                } else {
                                    phi_1694_ = bool();
                                }
                                let _e973 = phi_1694_;
                                phi_1696_ = _e973;
                                phi_1697_ = select(true, false, _e963);
                            } else {
                                phi_1696_ = bool();
                                phi_1697_ = true;
                            }
                            let _e976 = phi_1696_;
                            let _e978 = phi_1697_;
                            phi_1698_ = _e976;
                            phi_1699_ = _e978;
                        } else {
                            phi_1698_ = bool();
                            phi_1699_ = true;
                        }
                        let _e980 = phi_1698_;
                        let _e982 = phi_1699_;
                        if select(_e980, false, _e982) {
                            let _e985 = local_15;
                            let _e990 = global_8.member_1[0u];
                            if (_e985.member_1.x == _e990) {
                                let _e993 = local_16;
                                let _e998 = global_8.member_1[1u];
                                if (_e993.member_1.y == _e998) {
                                    let _e1001 = local_17;
                                    let _e1006 = global_8.member_1[2u];
                                    let _e1007 = (_e1001.member_1.z == _e1006);
                                    if _e1007 {
                                        let _e1009 = local_18;
                                        let _e1014 = global_8.member_1[3u];
                                        phi_1733_ = (_e1009.member_1.w == _e1014);
                                    } else {
                                        phi_1733_ = bool();
                                    }
                                    let _e1017 = phi_1733_;
                                    phi_1735_ = _e1017;
                                    phi_1736_ = select(true, false, _e1007);
                                } else {
                                    phi_1735_ = bool();
                                    phi_1736_ = true;
                                }
                                let _e1020 = phi_1735_;
                                let _e1022 = phi_1736_;
                                phi_1737_ = _e1020;
                                phi_1738_ = _e1022;
                            } else {
                                phi_1737_ = bool();
                                phi_1738_ = true;
                            }
                            let _e1024 = phi_1737_;
                            let _e1026 = phi_1738_;
                            if select(_e1024, false, _e1026) {
                                let _e1029 = local_19;
                                let _e1034 = global_8.member_2[0u];
                                if (_e1029.member_2.x == _e1034) {
                                    let _e1037 = local_20;
                                    let _e1042 = global_8.member_2[1u];
                                    if (_e1037.member_2.y == _e1042) {
                                        let _e1045 = local_21;
                                        let _e1050 = global_8.member_2[2u];
                                        let _e1051 = (_e1045.member_2.z == _e1050);
                                        if _e1051 {
                                            let _e1053 = local_22;
                                            let _e1058 = global_8.member_2[3u];
                                            phi_1772_ = (_e1053.member_2.w == _e1058);
                                        } else {
                                            phi_1772_ = bool();
                                        }
                                        let _e1061 = phi_1772_;
                                        phi_1774_ = _e1061;
                                        phi_1775_ = select(true, false, _e1051);
                                    } else {
                                        phi_1774_ = bool();
                                        phi_1775_ = true;
                                    }
                                    let _e1064 = phi_1774_;
                                    let _e1066 = phi_1775_;
                                    phi_1776_ = _e1064;
                                    phi_1777_ = _e1066;
                                } else {
                                    phi_1776_ = bool();
                                    phi_1777_ = true;
                                }
                                let _e1068 = phi_1776_;
                                let _e1070 = phi_1777_;
                                let _e1071 = select(_e1068, false, _e1070);
                                if _e1071 {
                                    let _e1073 = local_23;
                                    let _e1078 = global_8.member_3[0u];
                                    if (_e1073.member_3.x == _e1078) {
                                        let _e1081 = local_24;
                                        let _e1086 = global_8.member_3[1u];
                                        if (_e1081.member_3.y == _e1086) {
                                            let _e1089 = local_25;
                                            let _e1094 = global_8.member_3[2u];
                                            let _e1095 = (_e1089.member_3.z == _e1094);
                                            if _e1095 {
                                                let _e1097 = local_26;
                                                let _e1102 = global_8.member_3[3u];
                                                phi_1811_ = (_e1097.member_3.w == _e1102);
                                            } else {
                                                phi_1811_ = bool();
                                            }
                                            let _e1105 = phi_1811_;
                                            phi_1813_ = _e1105;
                                            phi_1814_ = select(true, false, _e1095);
                                        } else {
                                            phi_1813_ = bool();
                                            phi_1814_ = true;
                                        }
                                        let _e1108 = phi_1813_;
                                        let _e1110 = phi_1814_;
                                        phi_1815_ = _e1108;
                                        phi_1816_ = _e1110;
                                    } else {
                                        phi_1815_ = bool();
                                        phi_1816_ = true;
                                    }
                                    let _e1112 = phi_1815_;
                                    let _e1114 = phi_1816_;
                                    phi_1821_ = select(_e1112, false, _e1114);
                                } else {
                                    phi_1821_ = bool();
                                }
                                let _e1117 = phi_1821_;
                                phi_1823_ = _e1117;
                                phi_1824_ = select(true, false, _e1071);
                            } else {
                                phi_1823_ = bool();
                                phi_1824_ = true;
                            }
                            let _e1120 = phi_1823_;
                            let _e1122 = phi_1824_;
                            phi_1825_ = _e1120;
                            phi_1826_ = _e1122;
                        } else {
                            phi_1825_ = bool();
                            phi_1826_ = true;
                        }
                        let _e1124 = phi_1825_;
                        let _e1126 = phi_1826_;
                        if select(_e1124, false, _e1126) {
                            phi_1834_ = type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        } else {
                            let _e2056 = local_31;
                            phi_1834_ = _e2056;
                        }
                        let _e1129 = phi_1834_;
                        let _e1152 = fma(_e1129.member_2.z, _e1129.member_3.w, -((_e1129.member_2.w * _e1129.member_3.z)));
                        let _e1155 = fma(_e1129.member_2.y, _e1129.member_3.w, -((_e1129.member_2.w * _e1129.member_3.y)));
                        let _e1158 = fma(_e1129.member_2.y, _e1129.member_3.z, -((_e1129.member_2.z * _e1129.member_3.y)));
                        let _e1161 = fma(_e1129.member_2.x, _e1129.member_3.w, -((_e1129.member_2.w * _e1129.member_3.x)));
                        let _e1164 = fma(_e1129.member_2.x, _e1129.member_3.z, -((_e1129.member_2.z * _e1129.member_3.x)));
                        let _e1167 = fma(_e1129.member_2.x, _e1129.member_3.y, -((_e1129.member_2.y * _e1129.member_3.x)));
                        let _e1189 = fma(-(_e1129.member.w), fma(_e1129.member_1.z, _e1167, fma(_e1129.member_1.x, _e1158, -((_e1129.member_1.y * _e1164)))), fma(_e1129.member.z, fma(_e1129.member_1.w, _e1167, fma(_e1129.member_1.x, _e1155, -((_e1129.member_1.y * _e1161)))), fma(_e1129.member.x, fma(_e1129.member_1.w, _e1158, fma(_e1129.member_1.y, _e1152, -((_e1129.member_1.z * _e1155)))), -((_e1129.member.y * fma(_e1129.member_1.w, _e1164, fma(_e1129.member_1.x, _e1152, -((_e1129.member_1.z * _e1161)))))))));
                        if (_e1189 == 0f) {
                            phi_3346_ = type_27(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            phi_3347_ = type_27();
                            phi_3348_ = true;
                        } else {
                            let _e1198 = (sqrt(fma(_e1129.member.w, _e1129.member.w, fma(_e1129.member.z, _e1129.member.z, fma(_e1129.member.x, _e1129.member.x, (_e1129.member.y * _e1129.member.y))))) * select(-1f, 1f, (_e1189 >= 0f)));
                            let _e1203 = sqrt(fma(_e1129.member_1.w, _e1129.member_1.w, fma(_e1129.member_1.z, _e1129.member_1.z, fma(_e1129.member_1.x, _e1129.member_1.x, (_e1129.member_1.y * _e1129.member_1.y)))));
                            let _e1208 = sqrt(fma(_e1129.member_2.w, _e1129.member_2.w, fma(_e1129.member_2.z, _e1129.member_2.z, fma(_e1129.member_2.x, _e1129.member_2.x, (_e1129.member_2.y * _e1129.member_2.y)))));
                            if (_e1198 != 0f) {
                                phi_3234_ = select(true, false, (_e1203 != 0f));
                            } else {
                                phi_3234_ = true;
                            }
                            let _e1215 = phi_3234_;
                            let _e1216 = select((_e1208 != 0f), false, _e1215);
                            if _e1216 {
                                let _e1217 = (1f / _e1198);
                                let _e1218 = (1f / _e1203);
                                let _e1219 = (1f / _e1208);
                                let _e1220 = (_e1129.member.x * _e1217);
                                let _e1221 = (_e1129.member.z * _e1217);
                                let _e1222 = (_e1129.member_1.x * _e1218);
                                let _e1223 = (_e1129.member_2.x * _e1219);
                                let _e1224 = (_e1129.member_2.y * _e1219);
                                if ((_e1129.member_2.z * _e1219) <= 0f) {
                                    let _e1259 = fma(_e1129.member_1.y, _e1218, -(_e1220));
                                    let _e1261 = fma(-(_e1129.member_2.z), _e1219, 1f);
                                    if (_e1259 <= 0f) {
                                        let _e1275 = (_e1261 - _e1259);
                                        let _e1277 = (0.5f / sqrt(_e1275));
                                        phi_3329_ = vec4<f32>((_e1275 * _e1277), (fma(_e1129.member.y, _e1217, _e1222) * _e1277), (fma(_e1129.member.z, _e1217, _e1223) * _e1277), (fma(_e1129.member_1.z, _e1218, -(_e1224)) * _e1277));
                                    } else {
                                        let _e1263 = (_e1261 + _e1259);
                                        let _e1265 = (0.5f / sqrt(_e1263));
                                        phi_3329_ = vec4<f32>((fma(_e1129.member.y, _e1217, _e1222) * _e1265), (_e1263 * _e1265), (fma(_e1129.member_1.z, _e1218, _e1224) * _e1265), (fma(_e1129.member_2.x, _e1219, -(_e1221)) * _e1265));
                                    }
                                    let _e1288 = phi_3329_;
                                    phi_3331_ = _e1288;
                                } else {
                                    let _e1227 = fma(_e1129.member_1.y, _e1218, _e1220);
                                    let _e1228 = fma(_e1129.member_2.z, _e1219, 1f);
                                    if (_e1227 <= 0f) {
                                        let _e1244 = (_e1228 - _e1227);
                                        let _e1246 = (0.5f / sqrt(_e1244));
                                        phi_3299_ = vec4<f32>((fma(_e1129.member.z, _e1217, _e1223) * _e1246), (fma(_e1129.member_1.z, _e1218, _e1224) * _e1246), (_e1244 * _e1246), (fma(_e1129.member.y, _e1217, -(_e1222)) * _e1246));
                                    } else {
                                        let _e1230 = (_e1228 + _e1227);
                                        let _e1232 = (0.5f / sqrt(_e1230));
                                        phi_3299_ = vec4<f32>((fma(_e1129.member_1.z, _e1218, -(_e1224)) * _e1232), (fma(_e1129.member_2.x, _e1219, -(_e1221)) * _e1232), (fma(_e1129.member.y, _e1217, -(_e1222)) * _e1232), (_e1230 * _e1232));
                                    }
                                    let _e1257 = phi_3299_;
                                    phi_3331_ = _e1257;
                                }
                                let _e1290 = phi_3331_;
                                phi_3340_ = type_27(vec3<f32>(_e1198, _e1203, _e1208), _e1290, vec3<f32>(_e1129.member_3.x, _e1129.member_3.y, _e1129.member_3.z));
                                phi_3341_ = type_27();
                            } else {
                                phi_3340_ = type_27();
                                phi_3341_ = type_27(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e1294 = phi_3340_;
                            let _e1296 = phi_3341_;
                            phi_3346_ = _e1296;
                            phi_3347_ = _e1294;
                            phi_3348_ = select(true, false, _e1216);
                        }
                        let _e1299 = phi_3346_;
                        let _e1301 = phi_3347_;
                        let _e1303 = phi_3348_;
                        if _e1303 {
                            phi_3352_ = _e1299;
                        } else {
                            phi_3352_ = _e1301;
                        }
                        let _e1305 = phi_3352_;
                        phi_1836_ = type_27(_e1305.member_2, _e1305.member_1, _e1305.member);
                    } else {
                        phi_1836_ = type_27();
                    }
                    let _e1311 = phi_1836_;
                    phi_1838_ = _e1311;
                    phi_1839_ = select(true, false, _e541);
                } else {
                    phi_1838_ = type_27();
                    phi_1839_ = true;
                }
                let _e1314 = phi_1838_;
                let _e1316 = phi_1839_;
                if _e1316 {
                    if (_e98 >= 10u) {
                        phi_3446_ = (_e126 <= (_e98 - 10u));
                    } else {
                        phi_3446_ = false;
                    }
                    let _e1321 = phi_3446_;
                    if _e1321 {
                        let _e1324 = global_2.member[_e126];
                        let _e1329 = global_2.member[(_e126 + 1u)];
                        let _e1334 = global_2.member[(_e126 + 2u)];
                        let _e1340 = global_2.member[(_e126 + 3u)];
                        let _e1345 = global_2.member[(_e126 + 4u)];
                        let _e1350 = global_2.member[(_e126 + 5u)];
                        let _e1355 = global_2.member[(_e126 + 6u)];
                        let _e1361 = global_2.member[(_e126 + 7u)];
                        let _e1366 = global_2.member[(_e126 + 8u)];
                        let _e1371 = global_2.member[(_e126 + 9u)];
                        phi_1892_ = type_27(vec3<f32>(bitcast<f32>(_e1324), bitcast<f32>(_e1329), bitcast<f32>(_e1334)), vec4<f32>(bitcast<f32>(_e1340), bitcast<f32>(_e1345), bitcast<f32>(_e1350), bitcast<f32>(_e1355)), vec3<f32>(bitcast<f32>(_e1361), bitcast<f32>(_e1366), bitcast<f32>(_e1371)));
                    } else {
                        phi_1892_ = type_27(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e1376 = phi_1892_;
                    phi_1893_ = _e1376;
                } else {
                    phi_1893_ = _e1314;
                }
                let _e1378 = phi_1893_;
                let _e1387 = local_27;
                let _e1395 = sqrt(fma(_e1387.member_2.z, _e1387.member_2.z, fma(_e1387.member_2.x, _e1387.member_2.x, (_e1387.member_2.y * _e1387.member_2.y))));
                if (_e1395 == 0f) {
                    phi_1937_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_1937_ = (_e1387.member_2 * (1f / _e1395));
                }
                let _e1400 = phi_1937_;
                let _e1402 = local_28;
                let _e1411 = sqrt(fma(_e1402.member_3.z, _e1402.member_3.z, fma(_e1402.member_3.x, _e1402.member_3.x, (_e1402.member_3.y * _e1402.member_3.y))));
                if (_e1411 == 0f) {
                    phi_3490_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3490_ = (vec3<f32>(_e1402.member_3.x, _e1402.member_3.y, _e1402.member_3.z) * (1f / _e1411));
                }
                let _e1416 = phi_3490_;
                let _e1423 = (_e1378.member_1.x + _e1378.member_1.x);
                let _e1424 = (_e1378.member_1.y + _e1378.member_1.y);
                let _e1425 = (_e1378.member_1.z + _e1378.member_1.z);
                let _e1427 = (_e1378.member_1.z * _e1425);
                let _e1428 = (_e1378.member_1.w * _e1423);
                let _e1429 = (_e1378.member_1.w * _e1424);
                let _e1430 = (_e1378.member_1.w * _e1425);
                let _e1449 = (vec4<f32>((1f - fma(_e1378.member_1.y, _e1424, _e1427)), fma(_e1378.member_1.x, _e1424, _e1430), fma(_e1378.member_1.x, _e1425, -(_e1429)), 0f) * _e1378.member_2.x);
                let _e1450 = (vec4<f32>(fma(_e1378.member_1.x, _e1424, -(_e1430)), (1f - fma(_e1378.member_1.x, _e1423, _e1427)), fma(_e1378.member_1.y, _e1425, _e1428), 0f) * _e1378.member_2.y);
                let _e1451 = (vec4<f32>(fma(_e1378.member_1.x, _e1425, _e1429), fma(_e1378.member_1.y, _e1425, -(_e1428)), (1f - fma(_e1378.member_1.x, _e1423, (_e1378.member_1.y * _e1424))), 0f) * _e1378.member_2.z);
                let _e1456 = (_e1400.x / (_e1378.member_2.x * _e1378.member_2.x));
                let _e1458 = (_e1400.y / (_e1378.member_2.y * _e1378.member_2.y));
                let _e1460 = (_e1400.z / (_e1378.member_2.z * _e1378.member_2.z));
                let _e1476 = fma(_e1451.x, _e1460, fma(_e1449.x, _e1456, (_e1450.x * _e1458)));
                let _e1477 = fma(_e1451.y, _e1460, fma(_e1449.y, _e1456, (_e1450.y * _e1458)));
                let _e1478 = fma(_e1451.z, _e1460, fma(_e1449.z, _e1456, (_e1450.z * _e1458)));
                let _e1483 = sqrt(fma(_e1478, _e1478, fma(_e1476, _e1476, (_e1477 * _e1477))));
                if (_e1483 == 0f) {
                    phi_3572_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3572_ = (vec3<f32>(_e1476, _e1477, _e1478) * (1f / _e1483));
                }
                let _e1488 = phi_3572_;
                global_9 = _e1488;
                let _e1498 = fma(_e1451.x, _e1416.z, fma(_e1449.x, _e1416.x, (_e1450.x * _e1416.y)));
                let _e1499 = fma(_e1451.y, _e1416.z, fma(_e1449.y, _e1416.x, (_e1450.y * _e1416.y)));
                let _e1500 = fma(_e1451.z, _e1416.z, fma(_e1449.z, _e1416.x, (_e1450.z * _e1416.y)));
                let _e1505 = sqrt(fma(_e1500, _e1500, fma(_e1498, _e1498, (_e1499 * _e1499))));
                if (_e1505 == 0f) {
                    phi_3607_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3607_ = (vec3<f32>(_e1498, _e1499, _e1500) * (1f / _e1505));
                }
                let _e1510 = phi_3607_;
                global_10 = _e1510;
                let _e1527 = local_29;
                let _e1531 = select(-1f, 1f, (_e1527.member_3.w >= 0f));
                global_11 = vec3<f32>((fma(_e1488.y, _e1510.z, -((_e1510.y * _e1488.z))) * _e1531), (fma(_e1488.z, _e1510.x, -((_e1510.z * _e1488.x))) * _e1531), (fma(_e1488.x, _e1510.y, -((_e1510.x * _e1488.y))) * _e1531));
                let _e1537 = local_30;
                let _e1551 = (_e1378.member.x + fma(_e1451.x, _e1537.member.z, fma(_e1450.x, _e1537.member.y, (_e1449.x * _e1537.member.x))));
                let _e1552 = (_e1378.member.y + fma(_e1451.y, _e1537.member.z, fma(_e1450.y, _e1537.member.y, (_e1449.y * _e1537.member.x))));
                let _e1553 = (_e1378.member.z + fma(_e1451.z, _e1537.member.z, fma(_e1450.z, _e1537.member.y, (_e1449.z * _e1537.member.x))));
                global_12 = vec3<f32>(_e1551, _e1552, _e1553);
                if (_e98 >= 83u) {
                    phi_3620_ = (_e122 <= (_e98 - 83u));
                } else {
                    phi_3620_ = false;
                }
                let _e1559 = phi_3620_;
                if _e1559 {
                    let _e1562 = global_2.member[_e122];
                    let _e1567 = global_2.member[(_e122 + 1u)];
                    let _e1572 = global_2.member[(_e122 + 2u)];
                    let _e1577 = global_2.member[(_e122 + 3u)];
                    let _e1583 = global_2.member[(_e122 + 4u)];
                    let _e1588 = global_2.member[(_e122 + 5u)];
                    let _e1593 = global_2.member[(_e122 + 6u)];
                    let _e1598 = global_2.member[(_e122 + 7u)];
                    let _e1604 = global_2.member[(_e122 + 8u)];
                    let _e1609 = global_2.member[(_e122 + 9u)];
                    let _e1614 = global_2.member[(_e122 + 10u)];
                    let _e1619 = global_2.member[(_e122 + 11u)];
                    let _e1625 = global_2.member[(_e122 + 12u)];
                    let _e1630 = global_2.member[(_e122 + 13u)];
                    let _e1635 = global_2.member[(_e122 + 14u)];
                    let _e1640 = global_2.member[(_e122 + 15u)];
                    let _e1647 = global_2.member[(_e122 + 16u)];
                    let _e1652 = global_2.member[(_e122 + 17u)];
                    let _e1657 = global_2.member[(_e122 + 18u)];
                    let _e1662 = global_2.member[(_e122 + 19u)];
                    let _e1668 = global_2.member[(_e122 + 20u)];
                    let _e1673 = global_2.member[(_e122 + 21u)];
                    let _e1678 = global_2.member[(_e122 + 22u)];
                    let _e1683 = global_2.member[(_e122 + 23u)];
                    let _e1689 = global_2.member[(_e122 + 24u)];
                    let _e1694 = global_2.member[(_e122 + 25u)];
                    let _e1699 = global_2.member[(_e122 + 26u)];
                    let _e1704 = global_2.member[(_e122 + 27u)];
                    let _e1710 = global_2.member[(_e122 + 28u)];
                    let _e1715 = global_2.member[(_e122 + 29u)];
                    let _e1720 = global_2.member[(_e122 + 30u)];
                    let _e1725 = global_2.member[(_e122 + 31u)];
                    let _e1732 = global_2.member[(_e122 + 32u)];
                    let _e1737 = global_2.member[(_e122 + 33u)];
                    let _e1742 = global_2.member[(_e122 + 34u)];
                    local_3 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    phi_2261_ = type_24(0u, 6u);
                    loop {
                        let _e1747 = phi_2261_;
                        if (_e1747.member < _e1747.member_1) {
                            phi_2262_ = type_24((_e1747.member + 1u), _e1747.member_1);
                            phi_2285_ = type_24(1u, _e1747.member);
                        } else {
                            phi_2262_ = _e1747;
                            phi_2285_ = type_24(0u, type_24().member_1);
                        }
                        let _e1760 = phi_2262_;
                        let _e1762 = phi_2285_;
                        switch bitcast<i32>(_e1762.member) {
                            case 0: {
                                phi_2312_ = false;
                                break;
                            }
                            case 1: {
                                let _e1767 = ((_e122 + 35u) + (_e1762.member_1 * 4u));
                                let _e1770 = global_2.member[_e1767];
                                let _e1775 = global_2.member[(_e1767 + 1u)];
                                let _e1780 = global_2.member[(_e1767 + 2u)];
                                let _e1785 = global_2.member[(_e1767 + 3u)];
                                local_3[_e1762.member_1] = vec4<f32>(bitcast<f32>(_e1770), bitcast<f32>(_e1775), bitcast<f32>(_e1780), bitcast<f32>(_e1785));
                                phi_2312_ = true;
                                break;
                            }
                            default: {
                                phi_2312_ = bool();
                                break;
                            }
                        }
                        let _e1790 = phi_2312_;
                        continue;
                        continuing {
                            phi_2261_ = _e1760;
                            break if !(_e1790);
                        }
                    }
                    let _e1792 = local_3;
                    local_2 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                    phi_2318_ = type_24(0u, 8u);
                    loop {
                        let _e1795 = phi_2318_;
                        if (_e1795.member < _e1795.member_1) {
                            phi_2319_ = type_24((_e1795.member + 1u), _e1795.member_1);
                            phi_2342_ = type_24(1u, _e1795.member);
                        } else {
                            phi_2319_ = _e1795;
                            phi_2342_ = type_24(0u, type_24().member_1);
                        }
                        let _e1808 = phi_2319_;
                        let _e1810 = phi_2342_;
                        switch bitcast<i32>(_e1810.member) {
                            case 0: {
                                phi_2365_ = false;
                                break;
                            }
                            case 1: {
                                let _e1815 = ((_e122 + 59u) + (_e1810.member_1 * 3u));
                                let _e1818 = global_2.member[_e1815];
                                let _e1823 = global_2.member[(_e1815 + 1u)];
                                let _e1828 = global_2.member[(_e1815 + 2u)];
                                local_2[_e1810.member_1] = vec3<f32>(bitcast<f32>(_e1818), bitcast<f32>(_e1823), bitcast<f32>(_e1828));
                                phi_2365_ = true;
                                break;
                            }
                            default: {
                                phi_2365_ = bool();
                                break;
                            }
                        }
                        let _e1833 = phi_2365_;
                        continue;
                        continuing {
                            phi_2318_ = _e1808;
                            break if !(_e1833);
                        }
                    }
                    let _e1835 = local_2;
                    phi_2373_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1562), bitcast<f32>(_e1567), bitcast<f32>(_e1572), bitcast<f32>(_e1577)), vec4<f32>(bitcast<f32>(_e1583), bitcast<f32>(_e1588), bitcast<f32>(_e1593), bitcast<f32>(_e1598)), vec4<f32>(bitcast<f32>(_e1604), bitcast<f32>(_e1609), bitcast<f32>(_e1614), bitcast<f32>(_e1619)), vec4<f32>(bitcast<f32>(_e1625), bitcast<f32>(_e1630), bitcast<f32>(_e1635), bitcast<f32>(_e1640))), type_20(vec4<f32>(bitcast<f32>(_e1647), bitcast<f32>(_e1652), bitcast<f32>(_e1657), bitcast<f32>(_e1662)), vec4<f32>(bitcast<f32>(_e1668), bitcast<f32>(_e1673), bitcast<f32>(_e1678), bitcast<f32>(_e1683)), vec4<f32>(bitcast<f32>(_e1689), bitcast<f32>(_e1694), bitcast<f32>(_e1699), bitcast<f32>(_e1704)), vec4<f32>(bitcast<f32>(_e1710), bitcast<f32>(_e1715), bitcast<f32>(_e1720), bitcast<f32>(_e1725))), type_21(_e1835, _e1792), vec3<f32>(bitcast<f32>(_e1732), bitcast<f32>(_e1737), bitcast<f32>(_e1742)));
                } else {
                    phi_2373_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
                }
                let _e1839 = phi_2373_;
                global_1 = vec4<f32>((fma(fma(_e1839.member.member_3.x, _e1839.member_1.member_2.w, fma(_e1839.member.member_2.x, _e1839.member_1.member_2.z, fma(_e1839.member.member.x, _e1839.member_1.member_2.x, (_e1839.member.member_1.x * _e1839.member_1.member_2.y)))), _e1553, fma(fma(_e1839.member.member_3.x, _e1839.member_1.member.w, fma(_e1839.member.member_2.x, _e1839.member_1.member.z, fma(_e1839.member.member.x, _e1839.member_1.member.x, (_e1839.member.member_1.x * _e1839.member_1.member.y)))), _e1551, (fma(_e1839.member.member_3.x, _e1839.member_1.member_1.w, fma(_e1839.member.member_2.x, _e1839.member_1.member_1.z, fma(_e1839.member.member.x, _e1839.member_1.member_1.x, (_e1839.member.member_1.x * _e1839.member_1.member_1.y)))) * _e1552))) + fma(_e1839.member.member_3.x, _e1839.member_1.member_3.w, fma(_e1839.member.member_2.x, _e1839.member_1.member_3.z, fma(_e1839.member.member.x, _e1839.member_1.member_3.x, (_e1839.member.member_1.x * _e1839.member_1.member_3.y))))), (fma(fma(_e1839.member.member_3.y, _e1839.member_1.member_2.w, fma(_e1839.member.member_2.y, _e1839.member_1.member_2.z, fma(_e1839.member.member.y, _e1839.member_1.member_2.x, (_e1839.member.member_1.y * _e1839.member_1.member_2.y)))), _e1553, fma(fma(_e1839.member.member_3.y, _e1839.member_1.member.w, fma(_e1839.member.member_2.y, _e1839.member_1.member.z, fma(_e1839.member.member.y, _e1839.member_1.member.x, (_e1839.member.member_1.y * _e1839.member_1.member.y)))), _e1551, (fma(_e1839.member.member_3.y, _e1839.member_1.member_1.w, fma(_e1839.member.member_2.y, _e1839.member_1.member_1.z, fma(_e1839.member.member.y, _e1839.member_1.member_1.x, (_e1839.member.member_1.y * _e1839.member_1.member_1.y)))) * _e1552))) + fma(_e1839.member.member_3.y, _e1839.member_1.member_3.w, fma(_e1839.member.member_2.y, _e1839.member_1.member_3.z, fma(_e1839.member.member.y, _e1839.member_1.member_3.x, (_e1839.member.member_1.y * _e1839.member_1.member_3.y))))), (fma(fma(_e1839.member.member_3.z, _e1839.member_1.member_2.w, fma(_e1839.member.member_2.z, _e1839.member_1.member_2.z, fma(_e1839.member.member.z, _e1839.member_1.member_2.x, (_e1839.member.member_1.z * _e1839.member_1.member_2.y)))), _e1553, fma(fma(_e1839.member.member_3.z, _e1839.member_1.member.w, fma(_e1839.member.member_2.z, _e1839.member_1.member.z, fma(_e1839.member.member.z, _e1839.member_1.member.x, (_e1839.member.member_1.z * _e1839.member_1.member.y)))), _e1551, (fma(_e1839.member.member_3.z, _e1839.member_1.member_1.w, fma(_e1839.member.member_2.z, _e1839.member_1.member_1.z, fma(_e1839.member.member.z, _e1839.member_1.member_1.x, (_e1839.member.member_1.z * _e1839.member_1.member_1.y)))) * _e1552))) + fma(_e1839.member.member_3.z, _e1839.member_1.member_3.w, fma(_e1839.member.member_2.z, _e1839.member_1.member_3.z, fma(_e1839.member.member.z, _e1839.member_1.member_3.x, (_e1839.member.member_1.z * _e1839.member_1.member_3.y))))), (fma(fma(_e1839.member.member_3.w, _e1839.member_1.member_2.w, fma(_e1839.member.member_2.w, _e1839.member_1.member_2.z, fma(_e1839.member.member.w, _e1839.member_1.member_2.x, (_e1839.member.member_1.w * _e1839.member_1.member_2.y)))), _e1553, fma(fma(_e1839.member.member_3.w, _e1839.member_1.member.w, fma(_e1839.member.member_2.w, _e1839.member_1.member.z, fma(_e1839.member.member.w, _e1839.member_1.member.x, (_e1839.member.member_1.w * _e1839.member_1.member.y)))), _e1551, (fma(_e1839.member.member_3.w, _e1839.member_1.member_1.w, fma(_e1839.member.member_2.w, _e1839.member_1.member_1.z, fma(_e1839.member.member.w, _e1839.member_1.member_1.x, (_e1839.member.member_1.w * _e1839.member_1.member_1.y)))) * _e1552))) + fma(_e1839.member.member_3.w, _e1839.member_1.member_3.w, fma(_e1839.member.member_2.w, _e1839.member_1.member_3.z, fma(_e1839.member.member.w, _e1839.member_1.member_3.x, (_e1839.member.member_1.w * _e1839.member_1.member_3.y))))));
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
    global_3 = param;
    global = param_1;
    function();
    let _e14 = global_1.y;
    global_1.y = -(_e14);
    let _e16 = global_4;
    let _e17 = global_5;
    let _e18 = global_6;
    let _e19 = global_7;
    let _e20 = global_9;
    let _e21 = global_10;
    let _e22 = global_11;
    let _e23 = global_12;
    let _e24 = global_1;
    return VertexOutput(_e16, _e17, _e18, _e19, _e20, _e21, _e22, _e23, _e24);
}
