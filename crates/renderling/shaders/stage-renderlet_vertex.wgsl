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

struct type_28 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_31 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
    member_3: vec4<f32>,
    member_4: array<u32, 4>,
    member_5: array<f32, 4>,
    member_6: vec2<f32>,
    member_7: vec2<f32>,
}

struct type_34 {
    member: type_24,
    member_1: type_24,
}

struct VertexOutput {
    @location(0) @interpolate(flat) member: u32,
    @location(1) @interpolate(flat) member_1: u32,
    @location(2) @interpolate(flat) member_2: u32,
    @location(3) member_3: vec4<f32>,
    @location(4) member_4: vec2<f32>,
    @location(5) member_5: vec2<f32>,
    @location(6) member_6: vec3<f32>,
    @location(7) member_7: vec3<f32>,
    @location(8) member_8: vec3<f32>,
    @location(9) member_9: vec3<f32>,
    @builtin(position) member_10: vec4<f32>,
}

var<private> global: u32;
var<private> global_1: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
@group(0) @binding(0) 
var<storage> global_2: type_12;
var<private> global_3: u32;
var<private> global_4: u32;
var<private> global_5: u32;
var<private> global_6: u32;
var<private> global_7: vec4<f32>;
var<private> global_8: vec2<f32>;
var<private> global_9: vec2<f32>;
var<private> global_10: type_20 = type_20(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
var<private> global_11: vec3<f32>;
var<private> global_12: vec3<f32>;
var<private> global_13: vec3<f32>;
var<private> global_14: vec3<f32>;

fn function() {
    var local: array<f32, 4>;
    var local_1: array<u32, 4>;
    var local_2: array<vec3<f32>, 8>;
    var local_3: array<vec4<f32>, 6>;
    var local_4: array<f32, 4>;
    var local_5: array<u32, 4>;
    var phi_741_: u32;
    var phi_2842_: bool;
    var phi_748_: u32;
    var phi_749_: u32;
    var phi_759_: u32;
    var phi_841_: type_24;
    var phi_842_: type_24;
    var phi_857_: type_24;
    var phi_870_: bool;
    var phi_876_: type_24;
    var phi_877_: type_24;
    var phi_892_: type_24;
    var phi_906_: bool;
    var phi_912_: type_24;
    var phi_915_: type_31;
    var phi_913_: type_24;
    var phi_930_: type_24;
    var phi_947_: u32;
    var phi_2872_: bool;
    var phi_965_: type_24;
    var phi_2898_: u32;
    var phi_2917_: bool;
    var phi_1015_: type_28;
    var phi_1025_: u32;
    var phi_2939_: bool;
    var phi_1033_: f32;
    var phi_916_: type_31;
    var phi_1086_: bool;
    var local_6: type_31;
    var local_7: type_31;
    var local_8: type_31;
    var phi_2962_: bool;
    var phi_1200_: type_34;
    var local_9: type_31;
    var phi_1203_: type_24;
    var phi_1206_: type_20;
    var phi_1204_: type_24;
    var phi_1221_: type_24;
    var local_10: type_31;
    var phi_1245_: u32;
    var phi_2996_: bool;
    var phi_1254_: u32;
    var phi_3020_: bool;
    var phi_1303_: type_27;
    var phi_1313_: u32;
    var phi_3046_: bool;
    var phi_1386_: type_20;
    var phi_1207_: type_20;
    var phi_1623_: bool;
    var phi_3863_: bool;
    var local_11: type_20;
    var local_12: type_20;
    var local_13: type_20;
    var local_14: type_20;
    var phi_1650_: bool;
    var phi_1652_: bool;
    var phi_1653_: bool;
    var phi_1654_: bool;
    var phi_1655_: bool;
    var local_15: type_20;
    var local_16: type_20;
    var local_17: type_20;
    var local_18: type_20;
    var phi_1689_: bool;
    var phi_1691_: bool;
    var phi_1692_: bool;
    var phi_1693_: bool;
    var phi_1694_: bool;
    var local_19: type_20;
    var local_20: type_20;
    var local_21: type_20;
    var local_22: type_20;
    var phi_1728_: bool;
    var phi_1730_: bool;
    var phi_1731_: bool;
    var phi_1732_: bool;
    var phi_1733_: bool;
    var local_23: type_20;
    var local_24: type_20;
    var local_25: type_20;
    var local_26: type_20;
    var phi_1767_: bool;
    var phi_1769_: bool;
    var phi_1770_: bool;
    var phi_1771_: bool;
    var phi_1772_: bool;
    var phi_1777_: bool;
    var phi_1779_: bool;
    var phi_1780_: bool;
    var phi_1781_: bool;
    var phi_1782_: bool;
    var phi_1790_: type_20;
    var phi_3183_: bool;
    var phi_3246_: vec4<f32>;
    var phi_3276_: vec4<f32>;
    var phi_3278_: vec4<f32>;
    var phi_3289_: type_27;
    var phi_3290_: type_27;
    var phi_3293_: type_27;
    var phi_3294_: type_27;
    var phi_3295_: bool;
    var phi_3299_: type_27;
    var phi_1792_: type_27;
    var phi_1794_: type_27;
    var phi_1795_: bool;
    var phi_3393_: bool;
    var phi_1848_: type_27;
    var phi_1849_: type_27;
    var local_27: type_31;
    var phi_1893_: vec3<f32>;
    var local_28: type_31;
    var phi_3438_: vec3<f32>;
    var phi_3520_: vec3<f32>;
    var phi_3555_: vec3<f32>;
    var local_29: type_31;
    var local_30: type_31;
    var phi_3568_: bool;
    var phi_2217_: type_24;
    var phi_2218_: type_24;
    var phi_2233_: type_24;
    var phi_2260_: bool;
    var phi_2266_: type_24;
    var phi_2267_: type_24;
    var phi_2282_: type_24;
    var phi_2305_: bool;
    var phi_2313_: type_22;
    var local_31: type_20;

    switch bitcast<i32>(0u) {
        default: {
            let _e97 = global_3;
            let _e98 = global;
            let _e100 = arrayLength((&global_2.member));
            let _e103 = global_2.member[_e97];
            let _e108 = global_2.member[(_e97 + 1u)];
            let _e112 = global_2.member[(_e97 + 2u)];
            let _e116 = global_2.member[(_e97 + 7u)];
            let _e120 = global_2.member[(_e97 + 8u)];
            let _e124 = global_2.member[(_e97 + 9u)];
            let _e128 = global_2.member[(_e97 + 10u)];
            let _e132 = global_2.member[(_e97 + 11u)];
            let _e136 = global_2.member[(_e97 + 12u)];
            let _e140 = global_2.member[(_e97 + 13u)];
            let _e144 = global_2.member[(_e97 + 14u)];
            let _e148 = global_2.member[(_e97 + 15u)];
            let _e152 = global_2.member[(_e97 + 16u)];
            let _e156 = global_2.member[(_e97 + 17u)];
            if (_e103 == 1u) {
                global_4 = _e124;
                global_5 = _e132;
                global_6 = _e156;
                if (_e116 == 4294967295u) {
                    phi_749_ = _e98;
                } else {
                    if (_e98 >= _e120) {
                        phi_741_ = 4294967295u;
                    } else {
                        phi_741_ = (_e116 + _e98);
                    }
                    let _e161 = phi_741_;
                    if (_e100 >= 1u) {
                        phi_2842_ = (_e161 <= (_e100 - 1u));
                    } else {
                        phi_2842_ = false;
                    }
                    let _e166 = phi_2842_;
                    if _e166 {
                        let _e169 = global_2.member[_e161];
                        phi_748_ = _e169;
                    } else {
                        phi_748_ = 0u;
                    }
                    let _e171 = phi_748_;
                    phi_749_ = _e171;
                }
                let _e173 = phi_749_;
                if (_e173 >= _e112) {
                    phi_759_ = 4294967295u;
                } else {
                    phi_759_ = (_e108 + (26u * _e173));
                }
                let _e178 = phi_759_;
                let _e181 = global_2.member[_e178];
                let _e186 = global_2.member[(_e178 + 1u)];
                let _e191 = global_2.member[(_e178 + 2u)];
                let _e197 = global_2.member[(_e178 + 3u)];
                let _e202 = global_2.member[(_e178 + 4u)];
                let _e207 = global_2.member[(_e178 + 5u)];
                let _e212 = global_2.member[(_e178 + 6u)];
                let _e218 = global_2.member[(_e178 + 7u)];
                let _e223 = global_2.member[(_e178 + 8u)];
                let _e229 = global_2.member[(_e178 + 9u)];
                let _e234 = global_2.member[(_e178 + 10u)];
                let _e240 = global_2.member[(_e178 + 11u)];
                let _e245 = global_2.member[(_e178 + 12u)];
                let _e250 = global_2.member[(_e178 + 13u)];
                let _e256 = global_2.member[(_e178 + 14u)];
                let _e261 = global_2.member[(_e178 + 15u)];
                let _e266 = global_2.member[(_e178 + 16u)];
                let _e271 = global_2.member[(_e178 + 17u)];
                local_5 = array<u32, 4>(0u, 0u, 0u, 0u);
                phi_841_ = type_24(0u, 4u);
                loop {
                    let _e276 = phi_841_;
                    if (_e276.member < _e276.member_1) {
                        phi_842_ = type_24((_e276.member + 1u), _e276.member_1);
                        phi_857_ = type_24(1u, _e276.member);
                    } else {
                        phi_842_ = _e276;
                        phi_857_ = type_24(0u, type_24().member_1);
                    }
                    let _e289 = phi_842_;
                    let _e291 = phi_857_;
                    switch bitcast<i32>(_e291.member) {
                        case 0: {
                            phi_870_ = false;
                            break;
                        }
                        case 1: {
                            let _e298 = global_2.member[((_e178 + 18u) + _e291.member_1)];
                            local_5[_e291.member_1] = _e298;
                            phi_870_ = true;
                            break;
                        }
                        default: {
                            phi_870_ = bool();
                            break;
                        }
                    }
                    let _e301 = phi_870_;
                    continue;
                    continuing {
                        phi_841_ = _e289;
                        break if !(_e301);
                    }
                }
                let _e303 = local_5;
                local_4 = array<f32, 4>(0f, 0f, 0f, 0f);
                phi_876_ = type_24(0u, 4u);
                loop {
                    let _e306 = phi_876_;
                    if (_e306.member < _e306.member_1) {
                        phi_877_ = type_24((_e306.member + 1u), _e306.member_1);
                        phi_892_ = type_24(1u, _e306.member);
                    } else {
                        phi_877_ = _e306;
                        phi_892_ = type_24(0u, type_24().member_1);
                    }
                    let _e319 = phi_877_;
                    let _e321 = phi_892_;
                    switch bitcast<i32>(_e321.member) {
                        case 0: {
                            phi_906_ = false;
                            break;
                        }
                        case 1: {
                            let _e328 = global_2.member[((_e178 + 22u) + _e321.member_1)];
                            local_4[_e321.member_1] = bitcast<f32>(_e328);
                            phi_906_ = true;
                            break;
                        }
                        default: {
                            phi_906_ = bool();
                            break;
                        }
                    }
                    let _e332 = phi_906_;
                    continue;
                    continuing {
                        phi_876_ = _e319;
                        break if !(_e332);
                    }
                }
                let _e334 = local_4;
                phi_912_ = type_24(0u, _e144);
                phi_915_ = type_31(vec3<f32>(bitcast<f32>(_e181), bitcast<f32>(_e186), bitcast<f32>(_e191)), vec4<f32>(bitcast<f32>(_e197), bitcast<f32>(_e202), bitcast<f32>(_e207), bitcast<f32>(_e212)), vec3<f32>(bitcast<f32>(_e240), bitcast<f32>(_e245), bitcast<f32>(_e250)), vec4<f32>(bitcast<f32>(_e256), bitcast<f32>(_e261), bitcast<f32>(_e266), bitcast<f32>(_e271)), _e303, _e334, vec2<f32>(bitcast<f32>(_e218), bitcast<f32>(_e223)), vec2<f32>(bitcast<f32>(_e229), bitcast<f32>(_e234)));
                loop {
                    let _e338 = phi_912_;
                    let _e340 = phi_915_;
                    local_6 = _e340;
                    local_7 = _e340;
                    local_8 = _e340;
                    local_9 = _e340;
                    local_10 = _e340;
                    local_27 = _e340;
                    local_28 = _e340;
                    local_29 = _e340;
                    local_30 = _e340;
                    if (_e338.member < _e338.member_1) {
                        phi_913_ = type_24((_e338.member + 1u), _e338.member_1);
                        phi_930_ = type_24(1u, _e338.member);
                    } else {
                        phi_913_ = _e338;
                        phi_930_ = type_24(0u, type_24().member_1);
                    }
                    let _e353 = phi_913_;
                    let _e355 = phi_930_;
                    switch bitcast<i32>(_e355.member) {
                        case 0: {
                            phi_916_ = type_31();
                            phi_1086_ = false;
                            break;
                        }
                        case 1: {
                            if (_e355.member_1 >= _e144) {
                                phi_947_ = 4294967295u;
                            } else {
                                phi_947_ = (_e140 + (2u * _e355.member_1));
                            }
                            let _e363 = phi_947_;
                            if (_e100 >= 2u) {
                                phi_2872_ = (_e363 <= (_e100 - 2u));
                            } else {
                                phi_2872_ = false;
                            }
                            let _e368 = phi_2872_;
                            if _e368 {
                                let _e371 = global_2.member[_e363];
                                let _e375 = global_2.member[(_e363 + 1u)];
                                phi_965_ = type_24(_e371, _e375);
                            } else {
                                phi_965_ = type_24(4294967295u, 0u);
                            }
                            let _e378 = phi_965_;
                            if (_e173 >= _e378.member_1) {
                                phi_2898_ = 4294967295u;
                            } else {
                                phi_2898_ = (_e378.member + (9u * _e173));
                            }
                            let _e385 = phi_2898_;
                            if (_e100 >= 9u) {
                                phi_2917_ = (_e385 <= (_e100 - 9u));
                            } else {
                                phi_2917_ = false;
                            }
                            let _e390 = phi_2917_;
                            if _e390 {
                                let _e393 = global_2.member[_e385];
                                let _e398 = global_2.member[(_e385 + 1u)];
                                let _e403 = global_2.member[(_e385 + 2u)];
                                let _e409 = global_2.member[(_e385 + 3u)];
                                let _e414 = global_2.member[(_e385 + 4u)];
                                let _e419 = global_2.member[(_e385 + 5u)];
                                let _e425 = global_2.member[(_e385 + 6u)];
                                let _e430 = global_2.member[(_e385 + 7u)];
                                let _e435 = global_2.member[(_e385 + 8u)];
                                phi_1015_ = type_28(vec3<f32>(bitcast<f32>(_e393), bitcast<f32>(_e398), bitcast<f32>(_e403)), vec3<f32>(bitcast<f32>(_e409), bitcast<f32>(_e414), bitcast<f32>(_e419)), vec3<f32>(bitcast<f32>(_e425), bitcast<f32>(_e430), bitcast<f32>(_e435)));
                            } else {
                                phi_1015_ = type_28(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e440 = phi_1015_;
                            if (_e355.member_1 >= _e152) {
                                phi_1025_ = 4294967295u;
                            } else {
                                phi_1025_ = (_e148 + _e355.member_1);
                            }
                            let _e444 = phi_1025_;
                            if (_e100 >= 1u) {
                                phi_2939_ = (_e444 <= (_e100 - 1u));
                            } else {
                                phi_2939_ = false;
                            }
                            let _e449 = phi_2939_;
                            if _e449 {
                                let _e452 = global_2.member[_e444];
                                phi_1033_ = bitcast<f32>(_e452);
                            } else {
                                phi_1033_ = 0f;
                            }
                            let _e455 = phi_1033_;
                            let _e478 = type_31(vec3<f32>(fma(_e455, _e440.member.x, _e340.member.x), fma(_e455, _e440.member.y, _e340.member.y), fma(_e455, _e440.member.z, _e340.member.z)), _e340.member_1, _e340.member_2, _e340.member_3, _e340.member_4, _e340.member_5, _e340.member_6, _e340.member_7);
                            let _e501 = type_31(_e478.member, _e478.member_1, vec3<f32>(fma(_e455, _e440.member_1.x, _e340.member_2.x), fma(_e455, _e440.member_1.y, _e340.member_2.y), fma(_e455, _e440.member_1.z, _e340.member_2.z)), _e478.member_3, _e478.member_4, _e478.member_5, _e478.member_6, _e478.member_7);
                            phi_916_ = type_31(_e501.member, _e501.member_1, _e501.member_2, vec4<f32>(fma(_e455, _e440.member_2.x, _e340.member_3.x), fma(_e455, _e440.member_2.y, _e340.member_3.y), fma(_e455, _e440.member_2.z, _e340.member_3.z), _e340.member_3.w), _e501.member_4, _e501.member_5, _e501.member_6, _e501.member_7);
                            phi_1086_ = true;
                            break;
                        }
                        default: {
                            phi_916_ = type_31();
                            phi_1086_ = bool();
                            break;
                        }
                    }
                    let _e528 = phi_916_;
                    let _e530 = phi_1086_;
                    continue;
                    continuing {
                        phi_912_ = _e353;
                        phi_915_ = _e528;
                        break if !(_e530);
                    }
                }
                let _e533 = local_6;
                global_7 = _e533.member_1;
                let _e536 = local_7;
                global_8 = _e536.member_6;
                let _e539 = local_8;
                global_9 = _e539.member_7;
                let _e544 = global_2.member[(_e156 + 6u)];
                if (_e544 == 1u) {
                    let _e547 = ((_e136 == 4294967295u) != true);
                    if _e547 {
                        if (_e100 >= 4u) {
                            phi_2962_ = (_e136 <= (_e100 - 4u));
                        } else {
                            phi_2962_ = false;
                        }
                        let _e552 = phi_2962_;
                        if _e552 {
                            let _e555 = global_2.member[_e136];
                            let _e559 = global_2.member[(_e136 + 1u)];
                            let _e563 = global_2.member[(_e136 + 2u)];
                            let _e567 = global_2.member[(_e136 + 3u)];
                            phi_1200_ = type_34(type_24(_e555, _e559), type_24(_e563, _e567));
                        } else {
                            phi_1200_ = type_34(type_24(4294967295u, 0u), type_24(4294967295u, 0u));
                        }
                        let _e572 = phi_1200_;
                        let _e574 = local_9;
                        local = _e574.member_5;
                        phi_1203_ = type_24(0u, 4u);
                        phi_1206_ = type_20(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        loop {
                            let _e577 = phi_1203_;
                            let _e579 = phi_1206_;
                            local_11 = _e579;
                            local_12 = _e579;
                            local_13 = _e579;
                            local_14 = _e579;
                            local_15 = _e579;
                            local_16 = _e579;
                            local_17 = _e579;
                            local_18 = _e579;
                            local_19 = _e579;
                            local_20 = _e579;
                            local_21 = _e579;
                            local_22 = _e579;
                            local_23 = _e579;
                            local_24 = _e579;
                            local_25 = _e579;
                            local_26 = _e579;
                            local_31 = _e579;
                            if (_e577.member < _e577.member_1) {
                                phi_1204_ = type_24((_e577.member + 1u), _e577.member_1);
                                phi_1221_ = type_24(1u, _e577.member);
                            } else {
                                phi_1204_ = _e577;
                                phi_1221_ = type_24(0u, type_24().member_1);
                            }
                            let _e592 = phi_1204_;
                            let _e594 = phi_1221_;
                            switch bitcast<i32>(_e594.member) {
                                case 0: {
                                    phi_1207_ = type_20();
                                    phi_1623_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e599 = local_10;
                                    local_1 = _e599.member_4;
                                    let _e601 = (_e594.member_1 < 4u);
                                    if _e601 {
                                    } else {
                                        phi_3863_ = true;
                                        break;
                                    }
                                    let _e603 = local_1[_e594.member_1];
                                    if (_e603 >= _e572.member.member_1) {
                                        phi_1245_ = 4294967295u;
                                    } else {
                                        phi_1245_ = (_e572.member.member + _e603);
                                    }
                                    let _e611 = phi_1245_;
                                    if (_e100 >= 1u) {
                                        phi_2996_ = (_e611 <= (_e100 - 1u));
                                    } else {
                                        phi_2996_ = false;
                                    }
                                    let _e616 = phi_2996_;
                                    if _e616 {
                                        let _e619 = global_2.member[_e611];
                                        phi_1254_ = _e619;
                                    } else {
                                        phi_1254_ = 4294967295u;
                                    }
                                    let _e621 = phi_1254_;
                                    if (_e100 >= 10u) {
                                        phi_3020_ = (_e621 <= (_e100 - 10u));
                                    } else {
                                        phi_3020_ = false;
                                    }
                                    let _e626 = phi_3020_;
                                    if _e626 {
                                        let _e629 = global_2.member[_e621];
                                        let _e634 = global_2.member[(_e621 + 1u)];
                                        let _e639 = global_2.member[(_e621 + 2u)];
                                        let _e645 = global_2.member[(_e621 + 3u)];
                                        let _e650 = global_2.member[(_e621 + 4u)];
                                        let _e655 = global_2.member[(_e621 + 5u)];
                                        let _e660 = global_2.member[(_e621 + 6u)];
                                        let _e666 = global_2.member[(_e621 + 7u)];
                                        let _e671 = global_2.member[(_e621 + 8u)];
                                        let _e676 = global_2.member[(_e621 + 9u)];
                                        phi_1303_ = type_27(vec3<f32>(bitcast<f32>(_e629), bitcast<f32>(_e634), bitcast<f32>(_e639)), vec4<f32>(bitcast<f32>(_e645), bitcast<f32>(_e650), bitcast<f32>(_e655), bitcast<f32>(_e660)), vec3<f32>(bitcast<f32>(_e666), bitcast<f32>(_e671), bitcast<f32>(_e676)));
                                    } else {
                                        phi_1303_ = type_27(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e681 = phi_1303_;
                                    if (_e603 >= _e572.member_1.member_1) {
                                        phi_1313_ = 4294967295u;
                                    } else {
                                        phi_1313_ = (_e572.member_1.member + (16u * _e603));
                                    }
                                    let _e690 = phi_1313_;
                                    if (_e100 >= 16u) {
                                        phi_3046_ = (_e690 <= (_e100 - 16u));
                                    } else {
                                        phi_3046_ = false;
                                    }
                                    let _e695 = phi_3046_;
                                    if _e695 {
                                        let _e698 = global_2.member[_e690];
                                        let _e703 = global_2.member[(_e690 + 1u)];
                                        let _e708 = global_2.member[(_e690 + 2u)];
                                        let _e713 = global_2.member[(_e690 + 3u)];
                                        let _e719 = global_2.member[(_e690 + 4u)];
                                        let _e724 = global_2.member[(_e690 + 5u)];
                                        let _e729 = global_2.member[(_e690 + 6u)];
                                        let _e734 = global_2.member[(_e690 + 7u)];
                                        let _e740 = global_2.member[(_e690 + 8u)];
                                        let _e745 = global_2.member[(_e690 + 9u)];
                                        let _e750 = global_2.member[(_e690 + 10u)];
                                        let _e755 = global_2.member[(_e690 + 11u)];
                                        let _e761 = global_2.member[(_e690 + 12u)];
                                        let _e766 = global_2.member[(_e690 + 13u)];
                                        let _e771 = global_2.member[(_e690 + 14u)];
                                        let _e776 = global_2.member[(_e690 + 15u)];
                                        phi_1386_ = type_20(vec4<f32>(bitcast<f32>(_e698), bitcast<f32>(_e703), bitcast<f32>(_e708), bitcast<f32>(_e713)), vec4<f32>(bitcast<f32>(_e719), bitcast<f32>(_e724), bitcast<f32>(_e729), bitcast<f32>(_e734)), vec4<f32>(bitcast<f32>(_e740), bitcast<f32>(_e745), bitcast<f32>(_e750), bitcast<f32>(_e755)), vec4<f32>(bitcast<f32>(_e761), bitcast<f32>(_e766), bitcast<f32>(_e771), bitcast<f32>(_e776)));
                                    } else {
                                        phi_1386_ = type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                    }
                                    let _e781 = phi_1386_;
                                    let _e789 = (_e681.member_1.x + _e681.member_1.x);
                                    let _e790 = (_e681.member_1.y + _e681.member_1.y);
                                    let _e791 = (_e681.member_1.z + _e681.member_1.z);
                                    let _e793 = (_e681.member_1.z * _e791);
                                    let _e794 = (_e681.member_1.w * _e789);
                                    let _e795 = (_e681.member_1.w * _e790);
                                    let _e796 = (_e681.member_1.w * _e791);
                                    let _e816 = (vec4<f32>((1f - fma(_e681.member_1.y, _e790, _e793)), fma(_e681.member_1.x, _e790, _e796), fma(_e681.member_1.x, _e791, -(_e795)), 0f) * _e681.member_2.x);
                                    let _e818 = (vec4<f32>(fma(_e681.member_1.x, _e790, -(_e796)), (1f - fma(_e681.member_1.x, _e789, _e793)), fma(_e681.member_1.y, _e791, _e794), 0f) * _e681.member_2.y);
                                    let _e820 = (vec4<f32>(fma(_e681.member_1.x, _e791, _e795), fma(_e681.member_1.y, _e791, -(_e794)), (1f - fma(_e681.member_1.x, _e789, (_e681.member_1.y * _e790))), 0f) * _e681.member_2.z);
                                    if _e601 {
                                    } else {
                                        phi_3863_ = true;
                                        break;
                                    }
                                    let _e925 = local[_e594.member_1];
                                    phi_1207_ = type_20((_e579.member + (vec4<f32>(fma(_e681.member.x, _e781.member.w, fma(_e820.x, _e781.member.z, fma(_e816.x, _e781.member.x, (_e818.x * _e781.member.y)))), fma(_e681.member.y, _e781.member.w, fma(_e820.y, _e781.member.z, fma(_e816.y, _e781.member.x, (_e818.y * _e781.member.y)))), fma(_e681.member.z, _e781.member.w, fma(_e820.z, _e781.member.z, fma(_e816.z, _e781.member.x, (_e818.z * _e781.member.y)))), (fma(_e820.w, _e781.member.z, fma(_e816.w, _e781.member.x, (_e818.w * _e781.member.y))) + _e781.member.w)) * _e925)), (_e579.member_1 + (vec4<f32>(fma(_e681.member.x, _e781.member_1.w, fma(_e820.x, _e781.member_1.z, fma(_e816.x, _e781.member_1.x, (_e818.x * _e781.member_1.y)))), fma(_e681.member.y, _e781.member_1.w, fma(_e820.y, _e781.member_1.z, fma(_e816.y, _e781.member_1.x, (_e818.y * _e781.member_1.y)))), fma(_e681.member.z, _e781.member_1.w, fma(_e820.z, _e781.member_1.z, fma(_e816.z, _e781.member_1.x, (_e818.z * _e781.member_1.y)))), (fma(_e820.w, _e781.member_1.z, fma(_e816.w, _e781.member_1.x, (_e818.w * _e781.member_1.y))) + _e781.member_1.w)) * _e925)), (_e579.member_2 + (vec4<f32>(fma(_e681.member.x, _e781.member_2.w, fma(_e820.x, _e781.member_2.z, fma(_e816.x, _e781.member_2.x, (_e818.x * _e781.member_2.y)))), fma(_e681.member.y, _e781.member_2.w, fma(_e820.y, _e781.member_2.z, fma(_e816.y, _e781.member_2.x, (_e818.y * _e781.member_2.y)))), fma(_e681.member.z, _e781.member_2.w, fma(_e820.z, _e781.member_2.z, fma(_e816.z, _e781.member_2.x, (_e818.z * _e781.member_2.y)))), (fma(_e820.w, _e781.member_2.z, fma(_e816.w, _e781.member_2.x, (_e818.w * _e781.member_2.y))) + _e781.member_2.w)) * _e925)), (_e579.member_3 + (vec4<f32>(fma(_e681.member.x, _e781.member_3.w, fma(_e820.x, _e781.member_3.z, fma(_e816.x, _e781.member_3.x, (_e818.x * _e781.member_3.y)))), fma(_e681.member.y, _e781.member_3.w, fma(_e820.y, _e781.member_3.z, fma(_e816.y, _e781.member_3.x, (_e818.y * _e781.member_3.y)))), fma(_e681.member.z, _e781.member_3.w, fma(_e820.z, _e781.member_3.z, fma(_e816.z, _e781.member_3.x, (_e818.z * _e781.member_3.y)))), (fma(_e820.w, _e781.member_3.z, fma(_e816.w, _e781.member_3.x, (_e818.w * _e781.member_3.y))) + _e781.member_3.w)) * _e925)));
                                    phi_1623_ = true;
                                    break;
                                }
                                default: {
                                    phi_1207_ = type_20();
                                    phi_1623_ = bool();
                                    break;
                                }
                            }
                            let _e940 = phi_1207_;
                            let _e942 = phi_1623_;
                            continue;
                            continuing {
                                phi_1203_ = _e592;
                                phi_1206_ = _e940;
                                phi_3863_ = false;
                                break if !(_e942);
                            }
                        }
                        let _e945 = phi_3863_;
                        if _e945 {
                            break;
                        }
                        let _e947 = local_11;
                        let _e952 = global_10.member[0u];
                        if (_e947.member.x == _e952) {
                            let _e955 = local_12;
                            let _e960 = global_10.member[1u];
                            if (_e955.member.y == _e960) {
                                let _e963 = local_13;
                                let _e968 = global_10.member[2u];
                                let _e969 = (_e963.member.z == _e968);
                                if _e969 {
                                    let _e971 = local_14;
                                    let _e976 = global_10.member[3u];
                                    phi_1650_ = (_e971.member.w == _e976);
                                } else {
                                    phi_1650_ = bool();
                                }
                                let _e979 = phi_1650_;
                                phi_1652_ = _e979;
                                phi_1653_ = select(true, false, _e969);
                            } else {
                                phi_1652_ = bool();
                                phi_1653_ = true;
                            }
                            let _e982 = phi_1652_;
                            let _e984 = phi_1653_;
                            phi_1654_ = _e982;
                            phi_1655_ = _e984;
                        } else {
                            phi_1654_ = bool();
                            phi_1655_ = true;
                        }
                        let _e986 = phi_1654_;
                        let _e988 = phi_1655_;
                        if select(_e986, false, _e988) {
                            let _e991 = local_15;
                            let _e996 = global_10.member_1[0u];
                            if (_e991.member_1.x == _e996) {
                                let _e999 = local_16;
                                let _e1004 = global_10.member_1[1u];
                                if (_e999.member_1.y == _e1004) {
                                    let _e1007 = local_17;
                                    let _e1012 = global_10.member_1[2u];
                                    let _e1013 = (_e1007.member_1.z == _e1012);
                                    if _e1013 {
                                        let _e1015 = local_18;
                                        let _e1020 = global_10.member_1[3u];
                                        phi_1689_ = (_e1015.member_1.w == _e1020);
                                    } else {
                                        phi_1689_ = bool();
                                    }
                                    let _e1023 = phi_1689_;
                                    phi_1691_ = _e1023;
                                    phi_1692_ = select(true, false, _e1013);
                                } else {
                                    phi_1691_ = bool();
                                    phi_1692_ = true;
                                }
                                let _e1026 = phi_1691_;
                                let _e1028 = phi_1692_;
                                phi_1693_ = _e1026;
                                phi_1694_ = _e1028;
                            } else {
                                phi_1693_ = bool();
                                phi_1694_ = true;
                            }
                            let _e1030 = phi_1693_;
                            let _e1032 = phi_1694_;
                            if select(_e1030, false, _e1032) {
                                let _e1035 = local_19;
                                let _e1040 = global_10.member_2[0u];
                                if (_e1035.member_2.x == _e1040) {
                                    let _e1043 = local_20;
                                    let _e1048 = global_10.member_2[1u];
                                    if (_e1043.member_2.y == _e1048) {
                                        let _e1051 = local_21;
                                        let _e1056 = global_10.member_2[2u];
                                        let _e1057 = (_e1051.member_2.z == _e1056);
                                        if _e1057 {
                                            let _e1059 = local_22;
                                            let _e1064 = global_10.member_2[3u];
                                            phi_1728_ = (_e1059.member_2.w == _e1064);
                                        } else {
                                            phi_1728_ = bool();
                                        }
                                        let _e1067 = phi_1728_;
                                        phi_1730_ = _e1067;
                                        phi_1731_ = select(true, false, _e1057);
                                    } else {
                                        phi_1730_ = bool();
                                        phi_1731_ = true;
                                    }
                                    let _e1070 = phi_1730_;
                                    let _e1072 = phi_1731_;
                                    phi_1732_ = _e1070;
                                    phi_1733_ = _e1072;
                                } else {
                                    phi_1732_ = bool();
                                    phi_1733_ = true;
                                }
                                let _e1074 = phi_1732_;
                                let _e1076 = phi_1733_;
                                let _e1077 = select(_e1074, false, _e1076);
                                if _e1077 {
                                    let _e1079 = local_23;
                                    let _e1084 = global_10.member_3[0u];
                                    if (_e1079.member_3.x == _e1084) {
                                        let _e1087 = local_24;
                                        let _e1092 = global_10.member_3[1u];
                                        if (_e1087.member_3.y == _e1092) {
                                            let _e1095 = local_25;
                                            let _e1100 = global_10.member_3[2u];
                                            let _e1101 = (_e1095.member_3.z == _e1100);
                                            if _e1101 {
                                                let _e1103 = local_26;
                                                let _e1108 = global_10.member_3[3u];
                                                phi_1767_ = (_e1103.member_3.w == _e1108);
                                            } else {
                                                phi_1767_ = bool();
                                            }
                                            let _e1111 = phi_1767_;
                                            phi_1769_ = _e1111;
                                            phi_1770_ = select(true, false, _e1101);
                                        } else {
                                            phi_1769_ = bool();
                                            phi_1770_ = true;
                                        }
                                        let _e1114 = phi_1769_;
                                        let _e1116 = phi_1770_;
                                        phi_1771_ = _e1114;
                                        phi_1772_ = _e1116;
                                    } else {
                                        phi_1771_ = bool();
                                        phi_1772_ = true;
                                    }
                                    let _e1118 = phi_1771_;
                                    let _e1120 = phi_1772_;
                                    phi_1777_ = select(_e1118, false, _e1120);
                                } else {
                                    phi_1777_ = bool();
                                }
                                let _e1123 = phi_1777_;
                                phi_1779_ = _e1123;
                                phi_1780_ = select(true, false, _e1077);
                            } else {
                                phi_1779_ = bool();
                                phi_1780_ = true;
                            }
                            let _e1126 = phi_1779_;
                            let _e1128 = phi_1780_;
                            phi_1781_ = _e1126;
                            phi_1782_ = _e1128;
                        } else {
                            phi_1781_ = bool();
                            phi_1782_ = true;
                        }
                        let _e1130 = phi_1781_;
                        let _e1132 = phi_1782_;
                        if select(_e1130, false, _e1132) {
                            phi_1790_ = type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                        } else {
                            let _e2062 = local_31;
                            phi_1790_ = _e2062;
                        }
                        let _e1135 = phi_1790_;
                        let _e1158 = fma(_e1135.member_2.z, _e1135.member_3.w, -((_e1135.member_2.w * _e1135.member_3.z)));
                        let _e1161 = fma(_e1135.member_2.y, _e1135.member_3.w, -((_e1135.member_2.w * _e1135.member_3.y)));
                        let _e1164 = fma(_e1135.member_2.y, _e1135.member_3.z, -((_e1135.member_2.z * _e1135.member_3.y)));
                        let _e1167 = fma(_e1135.member_2.x, _e1135.member_3.w, -((_e1135.member_2.w * _e1135.member_3.x)));
                        let _e1170 = fma(_e1135.member_2.x, _e1135.member_3.z, -((_e1135.member_2.z * _e1135.member_3.x)));
                        let _e1173 = fma(_e1135.member_2.x, _e1135.member_3.y, -((_e1135.member_2.y * _e1135.member_3.x)));
                        let _e1195 = fma(-(_e1135.member.w), fma(_e1135.member_1.z, _e1173, fma(_e1135.member_1.x, _e1164, -((_e1135.member_1.y * _e1170)))), fma(_e1135.member.z, fma(_e1135.member_1.w, _e1173, fma(_e1135.member_1.x, _e1161, -((_e1135.member_1.y * _e1167)))), fma(_e1135.member.x, fma(_e1135.member_1.w, _e1164, fma(_e1135.member_1.y, _e1158, -((_e1135.member_1.z * _e1161)))), -((_e1135.member.y * fma(_e1135.member_1.w, _e1170, fma(_e1135.member_1.x, _e1158, -((_e1135.member_1.z * _e1167)))))))));
                        if (_e1195 == 0f) {
                            phi_3293_ = type_27(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            phi_3294_ = type_27();
                            phi_3295_ = true;
                        } else {
                            let _e1204 = (sqrt(fma(_e1135.member.w, _e1135.member.w, fma(_e1135.member.z, _e1135.member.z, fma(_e1135.member.x, _e1135.member.x, (_e1135.member.y * _e1135.member.y))))) * select(-1f, 1f, (_e1195 >= 0f)));
                            let _e1209 = sqrt(fma(_e1135.member_1.w, _e1135.member_1.w, fma(_e1135.member_1.z, _e1135.member_1.z, fma(_e1135.member_1.x, _e1135.member_1.x, (_e1135.member_1.y * _e1135.member_1.y)))));
                            let _e1214 = sqrt(fma(_e1135.member_2.w, _e1135.member_2.w, fma(_e1135.member_2.z, _e1135.member_2.z, fma(_e1135.member_2.x, _e1135.member_2.x, (_e1135.member_2.y * _e1135.member_2.y)))));
                            if (_e1204 != 0f) {
                                phi_3183_ = select(true, false, (_e1209 != 0f));
                            } else {
                                phi_3183_ = true;
                            }
                            let _e1221 = phi_3183_;
                            let _e1222 = select((_e1214 != 0f), false, _e1221);
                            if _e1222 {
                                let _e1223 = (1f / _e1204);
                                let _e1224 = (1f / _e1209);
                                let _e1225 = (1f / _e1214);
                                let _e1226 = (_e1135.member.x * _e1223);
                                let _e1227 = (_e1135.member.z * _e1223);
                                let _e1228 = (_e1135.member_1.x * _e1224);
                                let _e1229 = (_e1135.member_2.x * _e1225);
                                let _e1230 = (_e1135.member_2.y * _e1225);
                                if ((_e1135.member_2.z * _e1225) <= 0f) {
                                    let _e1234 = fma(_e1135.member_1.y, _e1224, -(_e1226));
                                    let _e1236 = fma(-(_e1135.member_2.z), _e1225, 1f);
                                    if (_e1234 <= 0f) {
                                        let _e1238 = (_e1236 - _e1234);
                                        let _e1240 = (0.5f / sqrt(_e1238));
                                        phi_3246_ = vec4<f32>((_e1238 * _e1240), (fma(_e1135.member.y, _e1223, _e1228) * _e1240), (fma(_e1135.member.z, _e1223, _e1229) * _e1240), (fma(_e1135.member_1.z, _e1224, -(_e1230)) * _e1240));
                                    } else {
                                        let _e1250 = (_e1236 + _e1234);
                                        let _e1252 = (0.5f / sqrt(_e1250));
                                        phi_3246_ = vec4<f32>((fma(_e1135.member.y, _e1223, _e1228) * _e1252), (_e1250 * _e1252), (fma(_e1135.member_1.z, _e1224, _e1230) * _e1252), (fma(_e1135.member_2.x, _e1225, -(_e1227)) * _e1252));
                                    }
                                    let _e1263 = phi_3246_;
                                    phi_3278_ = _e1263;
                                } else {
                                    let _e1264 = fma(_e1135.member_1.y, _e1224, _e1226);
                                    let _e1265 = fma(_e1135.member_2.z, _e1225, 1f);
                                    if (_e1264 <= 0f) {
                                        let _e1267 = (_e1265 - _e1264);
                                        let _e1269 = (0.5f / sqrt(_e1267));
                                        phi_3276_ = vec4<f32>((fma(_e1135.member.z, _e1223, _e1229) * _e1269), (fma(_e1135.member_1.z, _e1224, _e1230) * _e1269), (_e1267 * _e1269), (fma(_e1135.member.y, _e1223, -(_e1228)) * _e1269));
                                    } else {
                                        let _e1279 = (_e1265 + _e1264);
                                        let _e1281 = (0.5f / sqrt(_e1279));
                                        phi_3276_ = vec4<f32>((fma(_e1135.member_1.z, _e1224, -(_e1230)) * _e1281), (fma(_e1135.member_2.x, _e1225, -(_e1227)) * _e1281), (fma(_e1135.member.y, _e1223, -(_e1228)) * _e1281), (_e1279 * _e1281));
                                    }
                                    let _e1294 = phi_3276_;
                                    phi_3278_ = _e1294;
                                }
                                let _e1296 = phi_3278_;
                                phi_3289_ = type_27(vec3<f32>(_e1204, _e1209, _e1214), _e1296, vec3<f32>(_e1135.member_3.x, _e1135.member_3.y, _e1135.member_3.z));
                                phi_3290_ = type_27();
                            } else {
                                phi_3289_ = type_27();
                                phi_3290_ = type_27(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                            }
                            let _e1300 = phi_3289_;
                            let _e1302 = phi_3290_;
                            phi_3293_ = _e1302;
                            phi_3294_ = _e1300;
                            phi_3295_ = select(true, false, _e1222);
                        }
                        let _e1305 = phi_3293_;
                        let _e1307 = phi_3294_;
                        let _e1309 = phi_3295_;
                        if _e1309 {
                            phi_3299_ = _e1305;
                        } else {
                            phi_3299_ = _e1307;
                        }
                        let _e1311 = phi_3299_;
                        phi_1792_ = type_27(_e1311.member_2, _e1311.member_1, _e1311.member);
                    } else {
                        phi_1792_ = type_27();
                    }
                    let _e1317 = phi_1792_;
                    phi_1794_ = _e1317;
                    phi_1795_ = select(true, false, _e547);
                } else {
                    phi_1794_ = type_27();
                    phi_1795_ = true;
                }
                let _e1320 = phi_1794_;
                let _e1322 = phi_1795_;
                if _e1322 {
                    if (_e100 >= 10u) {
                        phi_3393_ = (_e128 <= (_e100 - 10u));
                    } else {
                        phi_3393_ = false;
                    }
                    let _e1327 = phi_3393_;
                    if _e1327 {
                        let _e1330 = global_2.member[_e128];
                        let _e1335 = global_2.member[(_e128 + 1u)];
                        let _e1340 = global_2.member[(_e128 + 2u)];
                        let _e1346 = global_2.member[(_e128 + 3u)];
                        let _e1351 = global_2.member[(_e128 + 4u)];
                        let _e1356 = global_2.member[(_e128 + 5u)];
                        let _e1361 = global_2.member[(_e128 + 6u)];
                        let _e1367 = global_2.member[(_e128 + 7u)];
                        let _e1372 = global_2.member[(_e128 + 8u)];
                        let _e1377 = global_2.member[(_e128 + 9u)];
                        phi_1848_ = type_27(vec3<f32>(bitcast<f32>(_e1330), bitcast<f32>(_e1335), bitcast<f32>(_e1340)), vec4<f32>(bitcast<f32>(_e1346), bitcast<f32>(_e1351), bitcast<f32>(_e1356), bitcast<f32>(_e1361)), vec3<f32>(bitcast<f32>(_e1367), bitcast<f32>(_e1372), bitcast<f32>(_e1377)));
                    } else {
                        phi_1848_ = type_27(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                    }
                    let _e1382 = phi_1848_;
                    phi_1849_ = _e1382;
                } else {
                    phi_1849_ = _e1320;
                }
                let _e1384 = phi_1849_;
                let _e1393 = local_27;
                let _e1401 = sqrt(fma(_e1393.member_2.z, _e1393.member_2.z, fma(_e1393.member_2.x, _e1393.member_2.x, (_e1393.member_2.y * _e1393.member_2.y))));
                if (_e1401 == 0f) {
                    phi_1893_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_1893_ = (_e1393.member_2 * (1f / _e1401));
                }
                let _e1406 = phi_1893_;
                let _e1408 = local_28;
                let _e1417 = sqrt(fma(_e1408.member_3.z, _e1408.member_3.z, fma(_e1408.member_3.x, _e1408.member_3.x, (_e1408.member_3.y * _e1408.member_3.y))));
                if (_e1417 == 0f) {
                    phi_3438_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3438_ = (vec3<f32>(_e1408.member_3.x, _e1408.member_3.y, _e1408.member_3.z) * (1f / _e1417));
                }
                let _e1422 = phi_3438_;
                let _e1429 = (_e1384.member_1.x + _e1384.member_1.x);
                let _e1430 = (_e1384.member_1.y + _e1384.member_1.y);
                let _e1431 = (_e1384.member_1.z + _e1384.member_1.z);
                let _e1433 = (_e1384.member_1.z * _e1431);
                let _e1434 = (_e1384.member_1.w * _e1429);
                let _e1435 = (_e1384.member_1.w * _e1430);
                let _e1436 = (_e1384.member_1.w * _e1431);
                let _e1455 = (vec4<f32>((1f - fma(_e1384.member_1.y, _e1430, _e1433)), fma(_e1384.member_1.x, _e1430, _e1436), fma(_e1384.member_1.x, _e1431, -(_e1435)), 0f) * _e1384.member_2.x);
                let _e1456 = (vec4<f32>(fma(_e1384.member_1.x, _e1430, -(_e1436)), (1f - fma(_e1384.member_1.x, _e1429, _e1433)), fma(_e1384.member_1.y, _e1431, _e1434), 0f) * _e1384.member_2.y);
                let _e1457 = (vec4<f32>(fma(_e1384.member_1.x, _e1431, _e1435), fma(_e1384.member_1.y, _e1431, -(_e1434)), (1f - fma(_e1384.member_1.x, _e1429, (_e1384.member_1.y * _e1430))), 0f) * _e1384.member_2.z);
                let _e1462 = (_e1406.x / (_e1384.member_2.x * _e1384.member_2.x));
                let _e1464 = (_e1406.y / (_e1384.member_2.y * _e1384.member_2.y));
                let _e1466 = (_e1406.z / (_e1384.member_2.z * _e1384.member_2.z));
                let _e1482 = fma(_e1457.x, _e1466, fma(_e1455.x, _e1462, (_e1456.x * _e1464)));
                let _e1483 = fma(_e1457.y, _e1466, fma(_e1455.y, _e1462, (_e1456.y * _e1464)));
                let _e1484 = fma(_e1457.z, _e1466, fma(_e1455.z, _e1462, (_e1456.z * _e1464)));
                let _e1489 = sqrt(fma(_e1484, _e1484, fma(_e1482, _e1482, (_e1483 * _e1483))));
                if (_e1489 == 0f) {
                    phi_3520_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3520_ = (vec3<f32>(_e1482, _e1483, _e1484) * (1f / _e1489));
                }
                let _e1494 = phi_3520_;
                global_11 = _e1494;
                let _e1504 = fma(_e1457.x, _e1422.z, fma(_e1455.x, _e1422.x, (_e1456.x * _e1422.y)));
                let _e1505 = fma(_e1457.y, _e1422.z, fma(_e1455.y, _e1422.x, (_e1456.y * _e1422.y)));
                let _e1506 = fma(_e1457.z, _e1422.z, fma(_e1455.z, _e1422.x, (_e1456.z * _e1422.y)));
                let _e1511 = sqrt(fma(_e1506, _e1506, fma(_e1504, _e1504, (_e1505 * _e1505))));
                if (_e1511 == 0f) {
                    phi_3555_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_3555_ = (vec3<f32>(_e1504, _e1505, _e1506) * (1f / _e1511));
                }
                let _e1516 = phi_3555_;
                global_12 = _e1516;
                let _e1533 = local_29;
                let _e1537 = select(-1f, 1f, (_e1533.member_3.w >= 0f));
                global_13 = vec3<f32>((fma(_e1494.y, _e1516.z, -((_e1516.y * _e1494.z))) * _e1537), (fma(_e1494.z, _e1516.x, -((_e1516.z * _e1494.x))) * _e1537), (fma(_e1494.x, _e1516.y, -((_e1516.x * _e1494.y))) * _e1537));
                let _e1543 = local_30;
                let _e1557 = (_e1384.member.x + fma(_e1457.x, _e1543.member.z, fma(_e1456.x, _e1543.member.y, (_e1455.x * _e1543.member.x))));
                let _e1558 = (_e1384.member.y + fma(_e1457.y, _e1543.member.z, fma(_e1456.y, _e1543.member.y, (_e1455.y * _e1543.member.x))));
                let _e1559 = (_e1384.member.z + fma(_e1457.z, _e1543.member.z, fma(_e1456.z, _e1543.member.y, (_e1455.z * _e1543.member.x))));
                global_14 = vec3<f32>(_e1557, _e1558, _e1559);
                if (_e100 >= 83u) {
                    phi_3568_ = (_e124 <= (_e100 - 83u));
                } else {
                    phi_3568_ = false;
                }
                let _e1565 = phi_3568_;
                if _e1565 {
                    let _e1568 = global_2.member[_e124];
                    let _e1573 = global_2.member[(_e124 + 1u)];
                    let _e1578 = global_2.member[(_e124 + 2u)];
                    let _e1583 = global_2.member[(_e124 + 3u)];
                    let _e1589 = global_2.member[(_e124 + 4u)];
                    let _e1594 = global_2.member[(_e124 + 5u)];
                    let _e1599 = global_2.member[(_e124 + 6u)];
                    let _e1604 = global_2.member[(_e124 + 7u)];
                    let _e1610 = global_2.member[(_e124 + 8u)];
                    let _e1615 = global_2.member[(_e124 + 9u)];
                    let _e1620 = global_2.member[(_e124 + 10u)];
                    let _e1625 = global_2.member[(_e124 + 11u)];
                    let _e1631 = global_2.member[(_e124 + 12u)];
                    let _e1636 = global_2.member[(_e124 + 13u)];
                    let _e1641 = global_2.member[(_e124 + 14u)];
                    let _e1646 = global_2.member[(_e124 + 15u)];
                    let _e1653 = global_2.member[(_e124 + 16u)];
                    let _e1658 = global_2.member[(_e124 + 17u)];
                    let _e1663 = global_2.member[(_e124 + 18u)];
                    let _e1668 = global_2.member[(_e124 + 19u)];
                    let _e1674 = global_2.member[(_e124 + 20u)];
                    let _e1679 = global_2.member[(_e124 + 21u)];
                    let _e1684 = global_2.member[(_e124 + 22u)];
                    let _e1689 = global_2.member[(_e124 + 23u)];
                    let _e1695 = global_2.member[(_e124 + 24u)];
                    let _e1700 = global_2.member[(_e124 + 25u)];
                    let _e1705 = global_2.member[(_e124 + 26u)];
                    let _e1710 = global_2.member[(_e124 + 27u)];
                    let _e1716 = global_2.member[(_e124 + 28u)];
                    let _e1721 = global_2.member[(_e124 + 29u)];
                    let _e1726 = global_2.member[(_e124 + 30u)];
                    let _e1731 = global_2.member[(_e124 + 31u)];
                    let _e1738 = global_2.member[(_e124 + 32u)];
                    let _e1743 = global_2.member[(_e124 + 33u)];
                    let _e1748 = global_2.member[(_e124 + 34u)];
                    local_3 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    phi_2217_ = type_24(0u, 6u);
                    loop {
                        let _e1753 = phi_2217_;
                        if (_e1753.member < _e1753.member_1) {
                            phi_2218_ = type_24((_e1753.member + 1u), _e1753.member_1);
                            phi_2233_ = type_24(1u, _e1753.member);
                        } else {
                            phi_2218_ = _e1753;
                            phi_2233_ = type_24(0u, type_24().member_1);
                        }
                        let _e1766 = phi_2218_;
                        let _e1768 = phi_2233_;
                        switch bitcast<i32>(_e1768.member) {
                            case 0: {
                                phi_2260_ = false;
                                break;
                            }
                            case 1: {
                                let _e1773 = ((_e124 + 35u) + (_e1768.member_1 * 4u));
                                let _e1776 = global_2.member[_e1773];
                                let _e1781 = global_2.member[(_e1773 + 1u)];
                                let _e1786 = global_2.member[(_e1773 + 2u)];
                                let _e1791 = global_2.member[(_e1773 + 3u)];
                                local_3[_e1768.member_1] = vec4<f32>(bitcast<f32>(_e1776), bitcast<f32>(_e1781), bitcast<f32>(_e1786), bitcast<f32>(_e1791));
                                phi_2260_ = true;
                                break;
                            }
                            default: {
                                phi_2260_ = bool();
                                break;
                            }
                        }
                        let _e1796 = phi_2260_;
                        continue;
                        continuing {
                            phi_2217_ = _e1766;
                            break if !(_e1796);
                        }
                    }
                    let _e1798 = local_3;
                    local_2 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                    phi_2266_ = type_24(0u, 8u);
                    loop {
                        let _e1801 = phi_2266_;
                        if (_e1801.member < _e1801.member_1) {
                            phi_2267_ = type_24((_e1801.member + 1u), _e1801.member_1);
                            phi_2282_ = type_24(1u, _e1801.member);
                        } else {
                            phi_2267_ = _e1801;
                            phi_2282_ = type_24(0u, type_24().member_1);
                        }
                        let _e1814 = phi_2267_;
                        let _e1816 = phi_2282_;
                        switch bitcast<i32>(_e1816.member) {
                            case 0: {
                                phi_2305_ = false;
                                break;
                            }
                            case 1: {
                                let _e1821 = ((_e124 + 59u) + (_e1816.member_1 * 3u));
                                let _e1824 = global_2.member[_e1821];
                                let _e1829 = global_2.member[(_e1821 + 1u)];
                                let _e1834 = global_2.member[(_e1821 + 2u)];
                                local_2[_e1816.member_1] = vec3<f32>(bitcast<f32>(_e1824), bitcast<f32>(_e1829), bitcast<f32>(_e1834));
                                phi_2305_ = true;
                                break;
                            }
                            default: {
                                phi_2305_ = bool();
                                break;
                            }
                        }
                        let _e1839 = phi_2305_;
                        continue;
                        continuing {
                            phi_2266_ = _e1814;
                            break if !(_e1839);
                        }
                    }
                    let _e1841 = local_2;
                    phi_2313_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1568), bitcast<f32>(_e1573), bitcast<f32>(_e1578), bitcast<f32>(_e1583)), vec4<f32>(bitcast<f32>(_e1589), bitcast<f32>(_e1594), bitcast<f32>(_e1599), bitcast<f32>(_e1604)), vec4<f32>(bitcast<f32>(_e1610), bitcast<f32>(_e1615), bitcast<f32>(_e1620), bitcast<f32>(_e1625)), vec4<f32>(bitcast<f32>(_e1631), bitcast<f32>(_e1636), bitcast<f32>(_e1641), bitcast<f32>(_e1646))), type_20(vec4<f32>(bitcast<f32>(_e1653), bitcast<f32>(_e1658), bitcast<f32>(_e1663), bitcast<f32>(_e1668)), vec4<f32>(bitcast<f32>(_e1674), bitcast<f32>(_e1679), bitcast<f32>(_e1684), bitcast<f32>(_e1689)), vec4<f32>(bitcast<f32>(_e1695), bitcast<f32>(_e1700), bitcast<f32>(_e1705), bitcast<f32>(_e1710)), vec4<f32>(bitcast<f32>(_e1716), bitcast<f32>(_e1721), bitcast<f32>(_e1726), bitcast<f32>(_e1731))), type_21(_e1841, _e1798), vec3<f32>(bitcast<f32>(_e1738), bitcast<f32>(_e1743), bitcast<f32>(_e1748)));
                } else {
                    phi_2313_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
                }
                let _e1845 = phi_2313_;
                global_1 = vec4<f32>((fma(fma(_e1845.member.member_3.x, _e1845.member_1.member_2.w, fma(_e1845.member.member_2.x, _e1845.member_1.member_2.z, fma(_e1845.member.member.x, _e1845.member_1.member_2.x, (_e1845.member.member_1.x * _e1845.member_1.member_2.y)))), _e1559, fma(fma(_e1845.member.member_3.x, _e1845.member_1.member.w, fma(_e1845.member.member_2.x, _e1845.member_1.member.z, fma(_e1845.member.member.x, _e1845.member_1.member.x, (_e1845.member.member_1.x * _e1845.member_1.member.y)))), _e1557, (fma(_e1845.member.member_3.x, _e1845.member_1.member_1.w, fma(_e1845.member.member_2.x, _e1845.member_1.member_1.z, fma(_e1845.member.member.x, _e1845.member_1.member_1.x, (_e1845.member.member_1.x * _e1845.member_1.member_1.y)))) * _e1558))) + fma(_e1845.member.member_3.x, _e1845.member_1.member_3.w, fma(_e1845.member.member_2.x, _e1845.member_1.member_3.z, fma(_e1845.member.member.x, _e1845.member_1.member_3.x, (_e1845.member.member_1.x * _e1845.member_1.member_3.y))))), (fma(fma(_e1845.member.member_3.y, _e1845.member_1.member_2.w, fma(_e1845.member.member_2.y, _e1845.member_1.member_2.z, fma(_e1845.member.member.y, _e1845.member_1.member_2.x, (_e1845.member.member_1.y * _e1845.member_1.member_2.y)))), _e1559, fma(fma(_e1845.member.member_3.y, _e1845.member_1.member.w, fma(_e1845.member.member_2.y, _e1845.member_1.member.z, fma(_e1845.member.member.y, _e1845.member_1.member.x, (_e1845.member.member_1.y * _e1845.member_1.member.y)))), _e1557, (fma(_e1845.member.member_3.y, _e1845.member_1.member_1.w, fma(_e1845.member.member_2.y, _e1845.member_1.member_1.z, fma(_e1845.member.member.y, _e1845.member_1.member_1.x, (_e1845.member.member_1.y * _e1845.member_1.member_1.y)))) * _e1558))) + fma(_e1845.member.member_3.y, _e1845.member_1.member_3.w, fma(_e1845.member.member_2.y, _e1845.member_1.member_3.z, fma(_e1845.member.member.y, _e1845.member_1.member_3.x, (_e1845.member.member_1.y * _e1845.member_1.member_3.y))))), (fma(fma(_e1845.member.member_3.z, _e1845.member_1.member_2.w, fma(_e1845.member.member_2.z, _e1845.member_1.member_2.z, fma(_e1845.member.member.z, _e1845.member_1.member_2.x, (_e1845.member.member_1.z * _e1845.member_1.member_2.y)))), _e1559, fma(fma(_e1845.member.member_3.z, _e1845.member_1.member.w, fma(_e1845.member.member_2.z, _e1845.member_1.member.z, fma(_e1845.member.member.z, _e1845.member_1.member.x, (_e1845.member.member_1.z * _e1845.member_1.member.y)))), _e1557, (fma(_e1845.member.member_3.z, _e1845.member_1.member_1.w, fma(_e1845.member.member_2.z, _e1845.member_1.member_1.z, fma(_e1845.member.member.z, _e1845.member_1.member_1.x, (_e1845.member.member_1.z * _e1845.member_1.member_1.y)))) * _e1558))) + fma(_e1845.member.member_3.z, _e1845.member_1.member_3.w, fma(_e1845.member.member_2.z, _e1845.member_1.member_3.z, fma(_e1845.member.member.z, _e1845.member_1.member_3.x, (_e1845.member.member_1.z * _e1845.member_1.member_3.y))))), (fma(fma(_e1845.member.member_3.w, _e1845.member_1.member_2.w, fma(_e1845.member.member_2.w, _e1845.member_1.member_2.z, fma(_e1845.member.member.w, _e1845.member_1.member_2.x, (_e1845.member.member_1.w * _e1845.member_1.member_2.y)))), _e1559, fma(fma(_e1845.member.member_3.w, _e1845.member_1.member.w, fma(_e1845.member.member_2.w, _e1845.member_1.member.z, fma(_e1845.member.member.w, _e1845.member_1.member.x, (_e1845.member.member_1.w * _e1845.member_1.member.y)))), _e1557, (fma(_e1845.member.member_3.w, _e1845.member_1.member_1.w, fma(_e1845.member.member_2.w, _e1845.member_1.member_1.z, fma(_e1845.member.member.w, _e1845.member_1.member_1.x, (_e1845.member.member_1.w * _e1845.member_1.member_1.y)))) * _e1558))) + fma(_e1845.member.member_3.w, _e1845.member_1.member_3.w, fma(_e1845.member.member_2.w, _e1845.member_1.member_3.z, fma(_e1845.member.member.w, _e1845.member_1.member_3.x, (_e1845.member.member_1.w * _e1845.member_1.member_3.y))))));
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
    let _e16 = global_1.y;
    global_1.y = -(_e16);
    let _e18 = global_4;
    let _e19 = global_5;
    let _e20 = global_6;
    let _e21 = global_7;
    let _e22 = global_8;
    let _e23 = global_9;
    let _e24 = global_11;
    let _e25 = global_12;
    let _e26 = global_13;
    let _e27 = global_14;
    let _e28 = global_1;
    return VertexOutput(_e18, _e19, _e20, _e21, _e22, _e23, _e24, _e25, _e26, _e27, _e28);
}
