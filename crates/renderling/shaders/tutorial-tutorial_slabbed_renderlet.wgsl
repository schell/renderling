struct type_11 {
    member: array<u32>,
}

struct type_18 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_19 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
}

struct type_20 {
    member: type_18,
    member_1: type_18,
    member_2: type_19,
    member_3: vec3<f32>,
}

struct type_22 {
    member: u32,
    member_1: u32,
}

struct type_25 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_28 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
    member_3: vec4<f32>,
    member_4: array<u32, 4>,
    member_5: array<f32, 4>,
    member_6: vec2<f32>,
    member_7: vec2<f32>,
}

struct VertexOutput {
    @location(0) member: vec4<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
@group(0) @binding(0) 
var<storage> global_1: type_11;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: vec4<f32>;
var<private> global_4: u32;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var local_2: array<f32, 4>;
    var local_3: array<u32, 4>;
    var phi_614_: u32;
    var phi_1390_: bool;
    var phi_700_: type_22;
    var phi_716_: type_22;
    var phi_717_: type_22;
    var phi_730_: type_22;
    var phi_746_: type_22;
    var phi_747_: type_22;
    var phi_763_: type_28;
    var phi_764_: bool;
    var phi_731_: type_22;
    var phi_770_: type_28;
    var phi_771_: bool;
    var phi_701_: type_22;
    var phi_773_: type_28;
    var phi_1421_: bool;
    var phi_824_: type_25;
    var phi_1492_: bool;
    var phi_985_: type_22;
    var phi_1001_: type_22;
    var phi_1002_: type_22;
    var phi_1015_: type_22;
    var phi_1031_: type_22;
    var phi_1032_: type_22;
    var phi_1058_: type_20;
    var phi_1059_: bool;
    var phi_1016_: type_22;
    var phi_1079_: type_20;
    var phi_1080_: bool;
    var phi_986_: type_22;
    var phi_1085_: type_20;
    var local_4: type_28;
    var local_5: type_28;
    var local_6: type_20;
    var local_7: type_20;

    let _e77 = global_4;
    let _e78 = global;
    let _e80 = arrayLength((&global_1.member));
    let _e84 = global_1.member[(_e77 + 1u)];
    let _e88 = global_1.member[(_e77 + 2u)];
    let _e92 = global_1.member[(_e77 + 9u)];
    let _e96 = global_1.member[(_e77 + 10u)];
    if (_e78 >= _e88) {
        phi_614_ = 4294967295u;
    } else {
        phi_614_ = (_e84 + (26u * _e78));
    }
    let _e101 = phi_614_;
    if (_e80 >= 26u) {
        phi_1390_ = (_e101 <= (_e80 - 26u));
    } else {
        phi_1390_ = false;
    }
    let _e106 = phi_1390_;
    if _e106 {
        let _e109 = global_1.member[_e101];
        let _e114 = global_1.member[(_e101 + 1u)];
        let _e119 = global_1.member[(_e101 + 2u)];
        let _e125 = global_1.member[(_e101 + 3u)];
        let _e130 = global_1.member[(_e101 + 4u)];
        let _e135 = global_1.member[(_e101 + 5u)];
        let _e140 = global_1.member[(_e101 + 6u)];
        let _e146 = global_1.member[(_e101 + 7u)];
        let _e151 = global_1.member[(_e101 + 8u)];
        let _e157 = global_1.member[(_e101 + 9u)];
        let _e162 = global_1.member[(_e101 + 10u)];
        let _e168 = global_1.member[(_e101 + 11u)];
        let _e173 = global_1.member[(_e101 + 12u)];
        let _e178 = global_1.member[(_e101 + 13u)];
        let _e184 = global_1.member[(_e101 + 14u)];
        let _e189 = global_1.member[(_e101 + 15u)];
        let _e194 = global_1.member[(_e101 + 16u)];
        let _e199 = global_1.member[(_e101 + 17u)];
        local_3 = array<u32, 4>(0u, 0u, 0u, 0u);
        phi_700_ = type_22(0u, 4u);
        loop {
            let _e204 = phi_700_;
            if (_e204.member < _e204.member_1) {
                phi_716_ = type_22((_e204.member + 1u), _e204.member_1);
                phi_717_ = type_22(1u, _e204.member);
            } else {
                phi_716_ = _e204;
                phi_717_ = type_22(0u, type_22().member_1);
            }
            let _e217 = phi_716_;
            let _e219 = phi_717_;
            switch bitcast<i32>(_e219.member) {
                case 0: {
                    let _e223 = local_3;
                    local_2 = array<f32, 4>(0f, 0f, 0f, 0f);
                    phi_730_ = type_22(0u, 4u);
                    loop {
                        let _e226 = phi_730_;
                        if (_e226.member < _e226.member_1) {
                            phi_746_ = type_22((_e226.member + 1u), _e226.member_1);
                            phi_747_ = type_22(1u, _e226.member);
                        } else {
                            phi_746_ = _e226;
                            phi_747_ = type_22(0u, type_22().member_1);
                        }
                        let _e239 = phi_746_;
                        let _e241 = phi_747_;
                        switch bitcast<i32>(_e241.member) {
                            case 0: {
                                let _e245 = local_2;
                                phi_763_ = type_28(vec3<f32>(bitcast<f32>(_e109), bitcast<f32>(_e114), bitcast<f32>(_e119)), vec4<f32>(bitcast<f32>(_e125), bitcast<f32>(_e130), bitcast<f32>(_e135), bitcast<f32>(_e140)), vec3<f32>(bitcast<f32>(_e168), bitcast<f32>(_e173), bitcast<f32>(_e178)), vec4<f32>(bitcast<f32>(_e184), bitcast<f32>(_e189), bitcast<f32>(_e194), bitcast<f32>(_e199)), _e223, _e245, vec2<f32>(bitcast<f32>(_e146), bitcast<f32>(_e151)), vec2<f32>(bitcast<f32>(_e157), bitcast<f32>(_e162)));
                                phi_764_ = false;
                                phi_731_ = type_22();
                                break;
                            }
                            case 1: {
                                let _e250 = global_1.member[((_e101 + 22u) + _e241.member_1)];
                                local_2[_e241.member_1] = bitcast<f32>(_e250);
                                phi_763_ = type_28();
                                phi_764_ = true;
                                phi_731_ = _e239;
                                break;
                            }
                            default: {
                                phi_763_ = type_28();
                                phi_764_ = false;
                                phi_731_ = type_22();
                                break;
                            }
                        }
                        let _e254 = phi_763_;
                        let _e256 = phi_764_;
                        let _e258 = phi_731_;
                        local_4 = _e254;
                        continue;
                        continuing {
                            phi_730_ = _e258;
                            break if !(_e256);
                        }
                    }
                    let _e886 = local_4;
                    phi_770_ = _e886;
                    phi_771_ = false;
                    phi_701_ = type_22();
                    break;
                }
                case 1: {
                    let _e263 = global_1.member[((_e101 + 18u) + _e219.member_1)];
                    local_3[_e219.member_1] = _e263;
                    phi_770_ = type_28();
                    phi_771_ = true;
                    phi_701_ = _e217;
                    break;
                }
                default: {
                    phi_770_ = type_28();
                    phi_771_ = false;
                    phi_701_ = type_22();
                    break;
                }
            }
            let _e266 = phi_770_;
            let _e268 = phi_771_;
            let _e270 = phi_701_;
            local_5 = _e266;
            continue;
            continuing {
                phi_700_ = _e270;
                break if !(_e268);
            }
        }
        let _e891 = local_5;
        phi_773_ = _e891;
    } else {
        phi_773_ = type_28(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), vec3<f32>(0f, 0f, 1f), vec4<f32>(0f, 1f, 0f, 0f), array<u32, 4>(0u, 0u, 0u, 0u), array<f32, 4>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
    }
    let _e273 = phi_773_;
    global_3 = _e273.member_1;
    if (_e80 >= 10u) {
        phi_1421_ = (_e96 <= (_e80 - 10u));
    } else {
        phi_1421_ = false;
    }
    let _e279 = phi_1421_;
    if _e279 {
        let _e282 = global_1.member[_e96];
        let _e287 = global_1.member[(_e96 + 1u)];
        let _e292 = global_1.member[(_e96 + 2u)];
        let _e298 = global_1.member[(_e96 + 3u)];
        let _e303 = global_1.member[(_e96 + 4u)];
        let _e308 = global_1.member[(_e96 + 5u)];
        let _e313 = global_1.member[(_e96 + 6u)];
        let _e319 = global_1.member[(_e96 + 7u)];
        let _e324 = global_1.member[(_e96 + 8u)];
        let _e329 = global_1.member[(_e96 + 9u)];
        phi_824_ = type_25(vec3<f32>(bitcast<f32>(_e282), bitcast<f32>(_e287), bitcast<f32>(_e292)), vec4<f32>(bitcast<f32>(_e298), bitcast<f32>(_e303), bitcast<f32>(_e308), bitcast<f32>(_e313)), vec3<f32>(bitcast<f32>(_e319), bitcast<f32>(_e324), bitcast<f32>(_e329)));
    } else {
        phi_824_ = type_25(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
    }
    let _e334 = phi_824_;
    let _e342 = (_e334.member_1.x + _e334.member_1.x);
    let _e343 = (_e334.member_1.y + _e334.member_1.y);
    let _e344 = (_e334.member_1.z + _e334.member_1.z);
    let _e346 = (_e334.member_1.z * _e344);
    let _e347 = (_e334.member_1.w * _e342);
    let _e348 = (_e334.member_1.w * _e343);
    let _e349 = (_e334.member_1.w * _e344);
    let _e369 = (vec4<f32>((1f - fma(_e334.member_1.y, _e343, _e346)), fma(_e334.member_1.x, _e343, _e349), fma(_e334.member_1.x, _e344, -(_e348)), 0f) * _e334.member_2.x);
    let _e371 = (vec4<f32>(fma(_e334.member_1.x, _e343, -(_e349)), (1f - fma(_e334.member_1.x, _e342, _e346)), fma(_e334.member_1.y, _e344, _e347), 0f) * _e334.member_2.y);
    let _e373 = (vec4<f32>(fma(_e334.member_1.x, _e344, _e348), fma(_e334.member_1.y, _e344, -(_e347)), (1f - fma(_e334.member_1.x, _e342, (_e334.member_1.y * _e343))), 0f) * _e334.member_2.z);
    if (_e80 >= 83u) {
        phi_1492_ = (_e92 <= (_e80 - 83u));
    } else {
        phi_1492_ = false;
    }
    let _e381 = phi_1492_;
    if _e381 {
        let _e384 = global_1.member[_e92];
        let _e389 = global_1.member[(_e92 + 1u)];
        let _e394 = global_1.member[(_e92 + 2u)];
        let _e399 = global_1.member[(_e92 + 3u)];
        let _e405 = global_1.member[(_e92 + 4u)];
        let _e410 = global_1.member[(_e92 + 5u)];
        let _e415 = global_1.member[(_e92 + 6u)];
        let _e420 = global_1.member[(_e92 + 7u)];
        let _e426 = global_1.member[(_e92 + 8u)];
        let _e431 = global_1.member[(_e92 + 9u)];
        let _e436 = global_1.member[(_e92 + 10u)];
        let _e441 = global_1.member[(_e92 + 11u)];
        let _e447 = global_1.member[(_e92 + 12u)];
        let _e452 = global_1.member[(_e92 + 13u)];
        let _e457 = global_1.member[(_e92 + 14u)];
        let _e462 = global_1.member[(_e92 + 15u)];
        let _e469 = global_1.member[(_e92 + 16u)];
        let _e474 = global_1.member[(_e92 + 17u)];
        let _e479 = global_1.member[(_e92 + 18u)];
        let _e484 = global_1.member[(_e92 + 19u)];
        let _e490 = global_1.member[(_e92 + 20u)];
        let _e495 = global_1.member[(_e92 + 21u)];
        let _e500 = global_1.member[(_e92 + 22u)];
        let _e505 = global_1.member[(_e92 + 23u)];
        let _e511 = global_1.member[(_e92 + 24u)];
        let _e516 = global_1.member[(_e92 + 25u)];
        let _e521 = global_1.member[(_e92 + 26u)];
        let _e526 = global_1.member[(_e92 + 27u)];
        let _e532 = global_1.member[(_e92 + 28u)];
        let _e537 = global_1.member[(_e92 + 29u)];
        let _e542 = global_1.member[(_e92 + 30u)];
        let _e547 = global_1.member[(_e92 + 31u)];
        let _e554 = global_1.member[(_e92 + 32u)];
        let _e559 = global_1.member[(_e92 + 33u)];
        let _e564 = global_1.member[(_e92 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_985_ = type_22(0u, 6u);
        loop {
            let _e569 = phi_985_;
            if (_e569.member < _e569.member_1) {
                phi_1001_ = type_22((_e569.member + 1u), _e569.member_1);
                phi_1002_ = type_22(1u, _e569.member);
            } else {
                phi_1001_ = _e569;
                phi_1002_ = type_22(0u, type_22().member_1);
            }
            let _e582 = phi_1001_;
            let _e584 = phi_1002_;
            switch bitcast<i32>(_e584.member) {
                case 0: {
                    let _e588 = local_1;
                    local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                    phi_1015_ = type_22(0u, 8u);
                    loop {
                        let _e591 = phi_1015_;
                        if (_e591.member < _e591.member_1) {
                            phi_1031_ = type_22((_e591.member + 1u), _e591.member_1);
                            phi_1032_ = type_22(1u, _e591.member);
                        } else {
                            phi_1031_ = _e591;
                            phi_1032_ = type_22(0u, type_22().member_1);
                        }
                        let _e604 = phi_1031_;
                        let _e606 = phi_1032_;
                        switch bitcast<i32>(_e606.member) {
                            case 0: {
                                let _e610 = local;
                                phi_1058_ = type_20(type_18(vec4<f32>(bitcast<f32>(_e384), bitcast<f32>(_e389), bitcast<f32>(_e394), bitcast<f32>(_e399)), vec4<f32>(bitcast<f32>(_e405), bitcast<f32>(_e410), bitcast<f32>(_e415), bitcast<f32>(_e420)), vec4<f32>(bitcast<f32>(_e426), bitcast<f32>(_e431), bitcast<f32>(_e436), bitcast<f32>(_e441)), vec4<f32>(bitcast<f32>(_e447), bitcast<f32>(_e452), bitcast<f32>(_e457), bitcast<f32>(_e462))), type_18(vec4<f32>(bitcast<f32>(_e469), bitcast<f32>(_e474), bitcast<f32>(_e479), bitcast<f32>(_e484)), vec4<f32>(bitcast<f32>(_e490), bitcast<f32>(_e495), bitcast<f32>(_e500), bitcast<f32>(_e505)), vec4<f32>(bitcast<f32>(_e511), bitcast<f32>(_e516), bitcast<f32>(_e521), bitcast<f32>(_e526)), vec4<f32>(bitcast<f32>(_e532), bitcast<f32>(_e537), bitcast<f32>(_e542), bitcast<f32>(_e547))), type_19(_e610, _e588), vec3<f32>(bitcast<f32>(_e554), bitcast<f32>(_e559), bitcast<f32>(_e564)));
                                phi_1059_ = false;
                                phi_1016_ = type_22();
                                break;
                            }
                            case 1: {
                                let _e614 = ((_e92 + 59u) + (_e606.member_1 * 3u));
                                let _e617 = global_1.member[_e614];
                                let _e622 = global_1.member[(_e614 + 1u)];
                                let _e627 = global_1.member[(_e614 + 2u)];
                                local[_e606.member_1] = vec3<f32>(bitcast<f32>(_e617), bitcast<f32>(_e622), bitcast<f32>(_e627));
                                phi_1058_ = type_20();
                                phi_1059_ = true;
                                phi_1016_ = _e604;
                                break;
                            }
                            default: {
                                phi_1058_ = type_20();
                                phi_1059_ = false;
                                phi_1016_ = type_22();
                                break;
                            }
                        }
                        let _e632 = phi_1058_;
                        let _e634 = phi_1059_;
                        let _e636 = phi_1016_;
                        local_6 = _e632;
                        continue;
                        continuing {
                            phi_1015_ = _e636;
                            break if !(_e634);
                        }
                    }
                    let _e906 = local_6;
                    phi_1079_ = _e906;
                    phi_1080_ = false;
                    phi_986_ = type_22();
                    break;
                }
                case 1: {
                    let _e639 = ((_e92 + 35u) + (_e584.member_1 * 4u));
                    let _e642 = global_1.member[_e639];
                    let _e647 = global_1.member[(_e639 + 1u)];
                    let _e652 = global_1.member[(_e639 + 2u)];
                    let _e657 = global_1.member[(_e639 + 3u)];
                    local_1[_e584.member_1] = vec4<f32>(bitcast<f32>(_e642), bitcast<f32>(_e647), bitcast<f32>(_e652), bitcast<f32>(_e657));
                    phi_1079_ = type_20();
                    phi_1080_ = true;
                    phi_986_ = _e582;
                    break;
                }
                default: {
                    phi_1079_ = type_20();
                    phi_1080_ = false;
                    phi_986_ = type_22();
                    break;
                }
            }
            let _e662 = phi_1079_;
            let _e664 = phi_1080_;
            let _e666 = phi_986_;
            local_7 = _e662;
            continue;
            continuing {
                phi_985_ = _e666;
                break if !(_e664);
            }
        }
        let _e911 = local_7;
        phi_1085_ = _e911;
    } else {
        phi_1085_ = type_20(type_18(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_18(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_19(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
    }
    let _e669 = phi_1085_;
    let _e709 = fma(_e669.member.member_3.x, _e669.member_1.member.w, fma(_e669.member.member_2.x, _e669.member_1.member.z, fma(_e669.member.member.x, _e669.member_1.member.x, (_e669.member.member_1.x * _e669.member_1.member.y))));
    let _e710 = fma(_e669.member.member_3.y, _e669.member_1.member.w, fma(_e669.member.member_2.y, _e669.member_1.member.z, fma(_e669.member.member.y, _e669.member_1.member.x, (_e669.member.member_1.y * _e669.member_1.member.y))));
    let _e711 = fma(_e669.member.member_3.z, _e669.member_1.member.w, fma(_e669.member.member_2.z, _e669.member_1.member.z, fma(_e669.member.member.z, _e669.member_1.member.x, (_e669.member.member_1.z * _e669.member_1.member.y))));
    let _e712 = fma(_e669.member.member_3.w, _e669.member_1.member.w, fma(_e669.member.member_2.w, _e669.member_1.member.z, fma(_e669.member.member.w, _e669.member_1.member.x, (_e669.member.member_1.w * _e669.member_1.member.y))));
    let _e730 = fma(_e669.member.member_3.x, _e669.member_1.member_1.w, fma(_e669.member.member_2.x, _e669.member_1.member_1.z, fma(_e669.member.member.x, _e669.member_1.member_1.x, (_e669.member.member_1.x * _e669.member_1.member_1.y))));
    let _e731 = fma(_e669.member.member_3.y, _e669.member_1.member_1.w, fma(_e669.member.member_2.y, _e669.member_1.member_1.z, fma(_e669.member.member.y, _e669.member_1.member_1.x, (_e669.member.member_1.y * _e669.member_1.member_1.y))));
    let _e732 = fma(_e669.member.member_3.z, _e669.member_1.member_1.w, fma(_e669.member.member_2.z, _e669.member_1.member_1.z, fma(_e669.member.member.z, _e669.member_1.member_1.x, (_e669.member.member_1.z * _e669.member_1.member_1.y))));
    let _e733 = fma(_e669.member.member_3.w, _e669.member_1.member_1.w, fma(_e669.member.member_2.w, _e669.member_1.member_1.z, fma(_e669.member.member.w, _e669.member_1.member_1.x, (_e669.member.member_1.w * _e669.member_1.member_1.y))));
    let _e751 = fma(_e669.member.member_3.x, _e669.member_1.member_2.w, fma(_e669.member.member_2.x, _e669.member_1.member_2.z, fma(_e669.member.member.x, _e669.member_1.member_2.x, (_e669.member.member_1.x * _e669.member_1.member_2.y))));
    let _e752 = fma(_e669.member.member_3.y, _e669.member_1.member_2.w, fma(_e669.member.member_2.y, _e669.member_1.member_2.z, fma(_e669.member.member.y, _e669.member_1.member_2.x, (_e669.member.member_1.y * _e669.member_1.member_2.y))));
    let _e753 = fma(_e669.member.member_3.z, _e669.member_1.member_2.w, fma(_e669.member.member_2.z, _e669.member_1.member_2.z, fma(_e669.member.member.z, _e669.member_1.member_2.x, (_e669.member.member_1.z * _e669.member_1.member_2.y))));
    let _e754 = fma(_e669.member.member_3.w, _e669.member_1.member_2.w, fma(_e669.member.member_2.w, _e669.member_1.member_2.z, fma(_e669.member.member.w, _e669.member_1.member_2.x, (_e669.member.member_1.w * _e669.member_1.member_2.y))));
    let _e772 = fma(_e669.member.member_3.x, _e669.member_1.member_3.w, fma(_e669.member.member_2.x, _e669.member_1.member_3.z, fma(_e669.member.member.x, _e669.member_1.member_3.x, (_e669.member.member_1.x * _e669.member_1.member_3.y))));
    let _e773 = fma(_e669.member.member_3.y, _e669.member_1.member_3.w, fma(_e669.member.member_2.y, _e669.member_1.member_3.z, fma(_e669.member.member.y, _e669.member_1.member_3.x, (_e669.member.member_1.y * _e669.member_1.member_3.y))));
    let _e774 = fma(_e669.member.member_3.z, _e669.member_1.member_3.w, fma(_e669.member.member_2.z, _e669.member_1.member_3.z, fma(_e669.member.member.z, _e669.member_1.member_3.x, (_e669.member.member_1.z * _e669.member_1.member_3.y))));
    let _e775 = fma(_e669.member.member_3.w, _e669.member_1.member_3.w, fma(_e669.member.member_2.w, _e669.member_1.member_3.z, fma(_e669.member.member.w, _e669.member_1.member_3.x, (_e669.member.member_1.w * _e669.member_1.member_3.y))));
    global_2 = vec4<f32>((fma(fma(_e772, _e373.w, fma(_e751, _e373.z, fma(_e709, _e373.x, (_e730 * _e373.y)))), _e273.member.z, fma(fma(_e772, _e369.w, fma(_e751, _e369.z, fma(_e709, _e369.x, (_e730 * _e369.y)))), _e273.member.x, (fma(_e772, _e371.w, fma(_e751, _e371.z, fma(_e709, _e371.x, (_e730 * _e371.y)))) * _e273.member.y))) + (fma(_e751, _e334.member.z, fma(_e709, _e334.member.x, (_e730 * _e334.member.y))) + _e772)), (fma(fma(_e773, _e373.w, fma(_e752, _e373.z, fma(_e710, _e373.x, (_e731 * _e373.y)))), _e273.member.z, fma(fma(_e773, _e369.w, fma(_e752, _e369.z, fma(_e710, _e369.x, (_e731 * _e369.y)))), _e273.member.x, (fma(_e773, _e371.w, fma(_e752, _e371.z, fma(_e710, _e371.x, (_e731 * _e371.y)))) * _e273.member.y))) + (fma(_e752, _e334.member.z, fma(_e710, _e334.member.x, (_e731 * _e334.member.y))) + _e773)), (fma(fma(_e774, _e373.w, fma(_e753, _e373.z, fma(_e711, _e373.x, (_e732 * _e373.y)))), _e273.member.z, fma(fma(_e774, _e369.w, fma(_e753, _e369.z, fma(_e711, _e369.x, (_e732 * _e369.y)))), _e273.member.x, (fma(_e774, _e371.w, fma(_e753, _e371.z, fma(_e711, _e371.x, (_e732 * _e371.y)))) * _e273.member.y))) + (fma(_e753, _e334.member.z, fma(_e711, _e334.member.x, (_e732 * _e334.member.y))) + _e774)), (fma(fma(_e775, _e373.w, fma(_e754, _e373.z, fma(_e712, _e373.x, (_e733 * _e373.y)))), _e273.member.z, fma(fma(_e775, _e369.w, fma(_e754, _e369.z, fma(_e712, _e369.x, (_e733 * _e369.y)))), _e273.member.x, (fma(_e775, _e371.w, fma(_e754, _e371.z, fma(_e712, _e371.x, (_e733 * _e371.y)))) * _e273.member.y))) + (fma(_e754, _e334.member.z, fma(_e712, _e334.member.x, (_e733 * _e334.member.y))) + _e775)));
    return;
}

@vertex 
fn tutorialtutorial_slabbed_renderlet(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_4 = param;
    global = param_1;
    function();
    let _e7 = global_2.y;
    global_2.y = -(_e7);
    let _e9 = global_3;
    let _e10 = global_2;
    return VertexOutput(_e9, _e10);
}
