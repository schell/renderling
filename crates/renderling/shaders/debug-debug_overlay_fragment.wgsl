struct type_2 {
    member: array<u32>,
}

struct type_12 {
    member: u32,
    member_1: u32,
}

struct type_21 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_23 {
    member: array<type_21>,
}

struct type_26 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

@group(0) @binding(0) 
var<storage> global: type_2;
var<private> global_1: vec4<f32>;
@group(0) @binding(1) 
var<storage> global_2: type_23;
var<private> global_3: vec4<f32>;

fn function() {
    var local: array<vec4<f32>, 6>;
    var local_1: array<vec3<f32>, 8>;
    var local_2: array<f32, 3>;
    var local_3: array<f32, 3>;
    var local_4: array<f32, 3>;
    var local_5: array<f32, 3>;
    var local_6: array<vec3<f32>, 8>;
    var local_7: array<vec4<f32>, 6>;
    var phi_688_: type_12;
    var phi_689_: type_12;
    var phi_712_: type_12;
    var phi_739_: bool;
    var phi_745_: type_12;
    var phi_746_: type_12;
    var phi_769_: type_12;
    var phi_792_: bool;
    var phi_2198_: bool;
    var phi_817_: type_12;
    var phi_818_: type_12;
    var phi_841_: type_12;
    var phi_905_: type_26;
    var phi_1048_: type_12;
    var phi_1049_: type_12;
    var phi_1072_: type_12;
    var phi_1116_: bool;
    var phi_1120_: bool;
    var phi_1121_: bool;
    var phi_2191_: bool;
    var phi_1641_: bool;
    var phi_2212_: bool;
    var phi_1128_: type_12;
    var phi_1129_: type_12;
    var phi_1152_: type_12;
    var phi_1162_: type_12;
    var phi_1165_: i32;
    var phi_1163_: type_12;
    var phi_1188_: type_12;
    var phi_1229_: i32;
    var phi_1166_: i32;
    var phi_1230_: bool;
    var phi_2206_: bool;
    var local_8: i32;
    var phi_1237_: type_12;
    var phi_1240_: i32;
    var phi_1238_: type_12;
    var phi_1263_: type_12;
    var phi_1304_: i32;
    var phi_1241_: i32;
    var phi_1305_: bool;
    var phi_2213_: bool;
    var local_9: i32;
    var phi_2220_: bool;
    var phi_1311_: bool;
    var phi_2219_: bool;
    var phi_1313_: bool;
    var phi_2218_: bool;
    var phi_2228_: bool;
    var phi_2227_: bool;
    var phi_2223_: bool;
    var phi_1558_: bool;
    var phi_2222_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e75 = global_3;
            let _e78 = global.member[0u];
            let _e81 = global.member[_e78];
            let _e82 = bitcast<f32>(_e81);
            let _e86 = global.member[(_e78 + 1u)];
            let _e87 = bitcast<f32>(_e86);
            let _e91 = global.member[(_e78 + 2u)];
            let _e92 = bitcast<f32>(_e91);
            let _e96 = global.member[(_e78 + 3u)];
            let _e97 = bitcast<f32>(_e96);
            let _e101 = global.member[(_e78 + 4u)];
            let _e102 = bitcast<f32>(_e101);
            let _e106 = global.member[(_e78 + 5u)];
            let _e107 = bitcast<f32>(_e106);
            let _e111 = global.member[(_e78 + 6u)];
            let _e112 = bitcast<f32>(_e111);
            let _e116 = global.member[(_e78 + 7u)];
            let _e117 = bitcast<f32>(_e116);
            let _e121 = global.member[(_e78 + 8u)];
            let _e122 = bitcast<f32>(_e121);
            let _e126 = global.member[(_e78 + 9u)];
            let _e127 = bitcast<f32>(_e126);
            let _e131 = global.member[(_e78 + 10u)];
            let _e132 = bitcast<f32>(_e131);
            let _e136 = global.member[(_e78 + 11u)];
            let _e137 = bitcast<f32>(_e136);
            let _e141 = global.member[(_e78 + 12u)];
            let _e142 = bitcast<f32>(_e141);
            let _e146 = global.member[(_e78 + 13u)];
            let _e147 = bitcast<f32>(_e146);
            let _e151 = global.member[(_e78 + 14u)];
            let _e152 = bitcast<f32>(_e151);
            let _e156 = global.member[(_e78 + 15u)];
            let _e157 = bitcast<f32>(_e156);
            let _e161 = global.member[(_e78 + 16u)];
            let _e162 = bitcast<f32>(_e161);
            let _e166 = global.member[(_e78 + 17u)];
            let _e167 = bitcast<f32>(_e166);
            let _e171 = global.member[(_e78 + 18u)];
            let _e172 = bitcast<f32>(_e171);
            let _e176 = global.member[(_e78 + 19u)];
            let _e177 = bitcast<f32>(_e176);
            let _e181 = global.member[(_e78 + 20u)];
            let _e182 = bitcast<f32>(_e181);
            let _e186 = global.member[(_e78 + 21u)];
            let _e187 = bitcast<f32>(_e186);
            let _e191 = global.member[(_e78 + 22u)];
            let _e192 = bitcast<f32>(_e191);
            let _e196 = global.member[(_e78 + 23u)];
            let _e197 = bitcast<f32>(_e196);
            let _e201 = global.member[(_e78 + 24u)];
            let _e202 = bitcast<f32>(_e201);
            let _e206 = global.member[(_e78 + 25u)];
            let _e207 = bitcast<f32>(_e206);
            let _e211 = global.member[(_e78 + 26u)];
            let _e212 = bitcast<f32>(_e211);
            let _e216 = global.member[(_e78 + 27u)];
            let _e217 = bitcast<f32>(_e216);
            let _e221 = global.member[(_e78 + 28u)];
            let _e222 = bitcast<f32>(_e221);
            let _e226 = global.member[(_e78 + 29u)];
            let _e227 = bitcast<f32>(_e226);
            let _e231 = global.member[(_e78 + 30u)];
            let _e232 = bitcast<f32>(_e231);
            let _e236 = global.member[(_e78 + 31u)];
            let _e237 = bitcast<f32>(_e236);
            local_7 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
            phi_688_ = type_12(0u, 6u);
            loop {
                let _e240 = phi_688_;
                if (_e240.member < _e240.member_1) {
                    phi_689_ = type_12((_e240.member + 1u), _e240.member_1);
                    phi_712_ = type_12(1u, _e240.member);
                } else {
                    phi_689_ = _e240;
                    phi_712_ = type_12(0u, type_12().member_1);
                }
                let _e253 = phi_689_;
                let _e255 = phi_712_;
                switch bitcast<i32>(_e255.member) {
                    case 0: {
                        phi_739_ = false;
                        break;
                    }
                    case 1: {
                        let _e260 = ((_e78 + 35u) + (_e255.member_1 * 4u));
                        let _e263 = global.member[_e260];
                        let _e268 = global.member[(_e260 + 1u)];
                        let _e273 = global.member[(_e260 + 2u)];
                        let _e278 = global.member[(_e260 + 3u)];
                        local_7[_e255.member_1] = vec4<f32>(bitcast<f32>(_e263), bitcast<f32>(_e268), bitcast<f32>(_e273), bitcast<f32>(_e278));
                        phi_739_ = true;
                        break;
                    }
                    default: {
                        phi_739_ = bool();
                        break;
                    }
                }
                let _e283 = phi_739_;
                continue;
                continuing {
                    phi_688_ = _e253;
                    break if !(_e283);
                }
            }
            let _e285 = local_7;
            local_6 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
            phi_745_ = type_12(0u, 8u);
            loop {
                let _e288 = phi_745_;
                if (_e288.member < _e288.member_1) {
                    phi_746_ = type_12((_e288.member + 1u), _e288.member_1);
                    phi_769_ = type_12(1u, _e288.member);
                } else {
                    phi_746_ = _e288;
                    phi_769_ = type_12(0u, type_12().member_1);
                }
                let _e301 = phi_746_;
                let _e303 = phi_769_;
                switch bitcast<i32>(_e303.member) {
                    case 0: {
                        phi_792_ = false;
                        break;
                    }
                    case 1: {
                        let _e308 = ((_e78 + 59u) + (_e303.member_1 * 3u));
                        let _e311 = global.member[_e308];
                        let _e316 = global.member[(_e308 + 1u)];
                        let _e321 = global.member[(_e308 + 2u)];
                        local_6[_e303.member_1] = vec3<f32>(bitcast<f32>(_e311), bitcast<f32>(_e316), bitcast<f32>(_e321));
                        phi_792_ = true;
                        break;
                    }
                    default: {
                        phi_792_ = bool();
                        break;
                    }
                }
                let _e326 = phi_792_;
                continue;
                continuing {
                    phi_745_ = _e301;
                    break if !(_e326);
                }
            }
            let _e328 = local_6;
            let _e331 = global.member[3u];
            let _e334 = global.member[4u];
            global_1 = vec4<f32>(0f, 0f, 0f, 0f);
            phi_2198_ = false;
            phi_817_ = type_12(0u, arrayLength((&global_2.member)));
            loop {
                let _e337 = phi_2198_;
                let _e339 = phi_817_;
                if (_e339.member < _e339.member_1) {
                    phi_818_ = type_12((_e339.member + 1u), _e339.member_1);
                    phi_841_ = type_12(1u, _e339.member);
                } else {
                    phi_818_ = _e339;
                    phi_841_ = type_12(0u, type_12().member_1);
                }
                let _e352 = phi_818_;
                let _e354 = phi_841_;
                switch bitcast<i32>(_e354.member) {
                    case 0: {
                        phi_2223_ = _e337;
                        phi_1558_ = false;
                        break;
                    }
                    case 1: {
                        let _e361 = global_2.member[_e354.member_1].member_3;
                        let _e365 = global.member[(_e361 + 9u)];
                        if ((_e365 == 4294967295u) != true) {
                            let _e370 = global.member[_e365];
                            let _e375 = global.member[(_e365 + 1u)];
                            let _e380 = global.member[(_e365 + 2u)];
                            let _e386 = global.member[(_e365 + 3u)];
                            let _e391 = global.member[(_e365 + 4u)];
                            let _e396 = global.member[(_e365 + 5u)];
                            let _e401 = global.member[(_e365 + 6u)];
                            let _e407 = global.member[(_e365 + 7u)];
                            let _e412 = global.member[(_e365 + 8u)];
                            let _e417 = global.member[(_e365 + 9u)];
                            phi_905_ = type_26(vec3<f32>(bitcast<f32>(_e370), bitcast<f32>(_e375), bitcast<f32>(_e380)), vec4<f32>(bitcast<f32>(_e386), bitcast<f32>(_e391), bitcast<f32>(_e396), bitcast<f32>(_e401)), vec3<f32>(bitcast<f32>(_e407), bitcast<f32>(_e412), bitcast<f32>(_e417)));
                        } else {
                            phi_905_ = type_26(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                        }
                        let _e422 = phi_905_;
                        let _e426 = global.member[(_e361 + 3u)];
                        let _e427 = bitcast<f32>(_e426);
                        let _e431 = global.member[(_e361 + 4u)];
                        let _e432 = bitcast<f32>(_e431);
                        let _e436 = global.member[(_e361 + 5u)];
                        let _e437 = bitcast<f32>(_e436);
                        let _e441 = global.member[(_e361 + 6u)];
                        let _e450 = (_e422.member_1.x + _e422.member_1.x);
                        let _e451 = (_e422.member_1.y + _e422.member_1.y);
                        let _e452 = (_e422.member_1.z + _e422.member_1.z);
                        let _e454 = (_e422.member_1.z * _e452);
                        let _e455 = (_e422.member_1.w * _e450);
                        let _e456 = (_e422.member_1.w * _e451);
                        let _e457 = (_e422.member_1.w * _e452);
                        let _e477 = (vec4<f32>((1f - fma(_e422.member_1.y, _e451, _e454)), fma(_e422.member_1.x, _e451, _e457), fma(_e422.member_1.x, _e452, -(_e456)), 0f) * _e422.member_2.x);
                        let _e479 = (vec4<f32>(fma(_e422.member_1.x, _e451, -(_e457)), (1f - fma(_e422.member_1.x, _e450, _e454)), fma(_e422.member_1.y, _e452, _e455), 0f) * _e422.member_2.y);
                        let _e481 = (vec4<f32>(fma(_e422.member_1.x, _e452, _e456), fma(_e422.member_1.y, _e452, -(_e455)), (1f - fma(_e422.member_1.x, _e450, (_e422.member_1.y * _e451))), 0f) * _e422.member_2.z);
                        let _e503 = (_e422.member.x + fma(_e481.x, _e437, fma(_e479.x, _e432, (_e477.x * _e427))));
                        let _e504 = (_e422.member.y + fma(_e481.y, _e437, fma(_e479.y, _e432, (_e477.y * _e427))));
                        let _e505 = (_e422.member.z + fma(_e481.z, _e437, fma(_e479.z, _e432, (_e477.z * _e427))));
                        let _e506 = vec3<f32>(_e503, _e504, _e505);
                        let _e509 = (max(_e422.member_2.x, max(_e422.member_2.y, _e422.member_2.z)) * bitcast<f32>(_e441));
                        let _e511 = sqrt((_e509 * _e509));
                        local_1 = _e328;
                        local = _e285;
                        let _e514 = local[0u][0u];
                        let _e517 = local[0u][1u];
                        let _e522 = local[0u][2u];
                        let _e526 = local[0u][3u];
                        let _e528 = -(_e511);
                        if ((fma(_e522, _e505, fma(_e514, _e503, (_e517 * _e504))) + _e526) < _e528) {
                            phi_2227_ = _e337;
                        } else {
                            phi_1048_ = type_12(0u, 6u);
                            loop {
                                let _e531 = phi_1048_;
                                if (_e531.member < _e531.member_1) {
                                    phi_1049_ = type_12((_e531.member + 1u), _e531.member_1);
                                    phi_1072_ = type_12(1u, _e531.member);
                                } else {
                                    phi_1049_ = _e531;
                                    phi_1072_ = type_12(0u, type_12().member_1);
                                }
                                let _e544 = phi_1049_;
                                let _e546 = phi_1072_;
                                switch bitcast<i32>(_e546.member) {
                                    case 0: {
                                        phi_1120_ = false;
                                        phi_1121_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if (_e546.member_1 != 0u) {
                                            if (_e546.member_1 < 6u) {
                                            } else {
                                                phi_2191_ = true;
                                                phi_1641_ = bool();
                                                break;
                                            }
                                            let _e554 = local[_e546.member_1][0u];
                                            let _e557 = local[_e546.member_1][1u];
                                            let _e562 = local[_e546.member_1][2u];
                                            let _e566 = local[_e546.member_1][3u];
                                            phi_1116_ = select(true, false, ((fma(_e562, _e505, fma(_e554, _e503, (_e557 * _e504))) + _e566) < _e528));
                                        } else {
                                            phi_1116_ = true;
                                        }
                                        let _e571 = phi_1116_;
                                        phi_1120_ = _e571;
                                        phi_1121_ = false;
                                        break;
                                    }
                                    default: {
                                        phi_1120_ = bool();
                                        phi_1121_ = bool();
                                        break;
                                    }
                                }
                                let _e573 = phi_1120_;
                                let _e575 = phi_1121_;
                                continue;
                                continuing {
                                    phi_1048_ = _e544;
                                    phi_2191_ = _e337;
                                    phi_1641_ = _e575;
                                    break if !(_e573);
                                }
                            }
                            let _e578 = phi_2191_;
                            let _e580 = phi_1641_;
                            phi_2222_ = _e578;
                            if _e578 {
                                break;
                            }
                            if _e580 {
                                let _e581 = vec3(_e511);
                                let _e582 = (_e506 - _e581);
                                let _e583 = (_e506 + _e581);
                                phi_2212_ = _e578;
                                phi_1128_ = type_12(0u, 3u);
                                loop {
                                    let _e585 = phi_2212_;
                                    let _e587 = phi_1128_;
                                    if (_e587.member < _e587.member_1) {
                                        phi_1129_ = type_12((_e587.member + 1u), _e587.member_1);
                                        phi_1152_ = type_12(1u, _e587.member);
                                    } else {
                                        phi_1129_ = _e587;
                                        phi_1152_ = type_12(0u, type_12().member_1);
                                    }
                                    let _e600 = phi_1129_;
                                    let _e602 = phi_1152_;
                                    switch bitcast<i32>(_e602.member) {
                                        case 0: {
                                            phi_2219_ = _e585;
                                            phi_1313_ = false;
                                            break;
                                        }
                                        case 1: {
                                            phi_1162_ = type_12(0u, 8u);
                                            phi_1165_ = 0i;
                                            loop {
                                                let _e607 = phi_1162_;
                                                let _e609 = phi_1165_;
                                                local_8 = _e609;
                                                if (_e607.member < _e607.member_1) {
                                                    phi_1163_ = type_12((_e607.member + 1u), _e607.member_1);
                                                    phi_1188_ = type_12(1u, _e607.member);
                                                } else {
                                                    phi_1163_ = _e607;
                                                    phi_1188_ = type_12(0u, type_12().member_1);
                                                }
                                                let _e622 = phi_1163_;
                                                let _e624 = phi_1188_;
                                                switch bitcast<i32>(_e624.member) {
                                                    case 0: {
                                                        phi_1166_ = i32();
                                                        phi_1230_ = false;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if (_e624.member_1 < 8u) {
                                                        } else {
                                                            phi_2206_ = true;
                                                            break;
                                                        }
                                                        let _e631 = local_1[_e624.member_1][0u];
                                                        let _e634 = local_1[_e624.member_1][1u];
                                                        let _e637 = local_1[_e624.member_1][2u];
                                                        local_2 = array<f32, 3>(_e631, _e634, _e637);
                                                        let _e639 = (_e602.member_1 < 3u);
                                                        if _e639 {
                                                        } else {
                                                            phi_2206_ = true;
                                                            break;
                                                        }
                                                        let _e641 = local_2[_e602.member_1];
                                                        local_3 = array<f32, 3>(_e582.x, _e582.y, _e582.z);
                                                        if _e639 {
                                                        } else {
                                                            phi_2206_ = true;
                                                            break;
                                                        }
                                                        let _e647 = local_3[_e602.member_1];
                                                        if (_e641 < _e647) {
                                                            phi_1229_ = (_e609 + 1i);
                                                        } else {
                                                            phi_1229_ = _e609;
                                                        }
                                                        let _e651 = phi_1229_;
                                                        phi_1166_ = _e651;
                                                        phi_1230_ = true;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_1166_ = i32();
                                                        phi_1230_ = bool();
                                                        break;
                                                    }
                                                }
                                                let _e653 = phi_1166_;
                                                let _e655 = phi_1230_;
                                                continue;
                                                continuing {
                                                    phi_1162_ = _e622;
                                                    phi_1165_ = _e653;
                                                    phi_2206_ = _e585;
                                                    break if !(_e655);
                                                }
                                            }
                                            let _e658 = phi_2206_;
                                            phi_2218_ = _e658;
                                            if _e658 {
                                                break;
                                            }
                                            let _e660 = local_8;
                                            if (_e660 == 8i) {
                                                phi_2220_ = _e658;
                                                phi_1311_ = false;
                                            } else {
                                                phi_1237_ = type_12(0u, 8u);
                                                phi_1240_ = 0i;
                                                loop {
                                                    let _e663 = phi_1237_;
                                                    let _e665 = phi_1240_;
                                                    local_9 = _e665;
                                                    if (_e663.member < _e663.member_1) {
                                                        phi_1238_ = type_12((_e663.member + 1u), _e663.member_1);
                                                        phi_1263_ = type_12(1u, _e663.member);
                                                    } else {
                                                        phi_1238_ = _e663;
                                                        phi_1263_ = type_12(0u, type_12().member_1);
                                                    }
                                                    let _e678 = phi_1238_;
                                                    let _e680 = phi_1263_;
                                                    switch bitcast<i32>(_e680.member) {
                                                        case 0: {
                                                            phi_1241_ = i32();
                                                            phi_1305_ = false;
                                                            break;
                                                        }
                                                        case 1: {
                                                            if (_e680.member_1 < 8u) {
                                                            } else {
                                                                phi_2213_ = true;
                                                                break;
                                                            }
                                                            let _e687 = local_1[_e680.member_1][0u];
                                                            let _e690 = local_1[_e680.member_1][1u];
                                                            let _e693 = local_1[_e680.member_1][2u];
                                                            local_4 = array<f32, 3>(_e687, _e690, _e693);
                                                            let _e695 = (_e602.member_1 < 3u);
                                                            if _e695 {
                                                            } else {
                                                                phi_2213_ = true;
                                                                break;
                                                            }
                                                            let _e697 = local_4[_e602.member_1];
                                                            local_5 = array<f32, 3>(_e583.x, _e583.y, _e583.z);
                                                            if _e695 {
                                                            } else {
                                                                phi_2213_ = true;
                                                                break;
                                                            }
                                                            let _e703 = local_5[_e602.member_1];
                                                            if (_e697 > _e703) {
                                                                phi_1304_ = (_e665 + 1i);
                                                            } else {
                                                                phi_1304_ = _e665;
                                                            }
                                                            let _e707 = phi_1304_;
                                                            phi_1241_ = _e707;
                                                            phi_1305_ = true;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_1241_ = i32();
                                                            phi_1305_ = bool();
                                                            break;
                                                        }
                                                    }
                                                    let _e709 = phi_1241_;
                                                    let _e711 = phi_1305_;
                                                    continue;
                                                    continuing {
                                                        phi_1237_ = _e678;
                                                        phi_1240_ = _e709;
                                                        phi_2213_ = _e658;
                                                        break if !(_e711);
                                                    }
                                                }
                                                let _e714 = phi_2213_;
                                                phi_2218_ = _e714;
                                                if _e714 {
                                                    break;
                                                }
                                                let _e716 = local_9;
                                                phi_2220_ = _e714;
                                                phi_1311_ = select(true, false, (_e716 == 8i));
                                            }
                                            let _e720 = phi_2220_;
                                            let _e722 = phi_1311_;
                                            phi_2219_ = _e720;
                                            phi_1313_ = _e722;
                                            break;
                                        }
                                        default: {
                                            phi_2219_ = _e585;
                                            phi_1313_ = bool();
                                            break;
                                        }
                                    }
                                    let _e724 = phi_2219_;
                                    let _e726 = phi_1313_;
                                    continue;
                                    continuing {
                                        phi_2212_ = _e724;
                                        phi_1128_ = _e600;
                                        phi_2218_ = _e724;
                                        break if !(_e726);
                                    }
                                }
                                let _e729 = phi_2218_;
                                phi_2222_ = _e729;
                                if _e729 {
                                    break;
                                }
                                phi_2228_ = _e729;
                            } else {
                                phi_2228_ = _e578;
                            }
                            let _e731 = phi_2228_;
                            phi_2227_ = _e731;
                        }
                        let _e733 = phi_2227_;
                        let _e734 = f32(_e331);
                        let _e751 = fma(_e152, _e177, fma(_e132, _e172, fma(_e92, _e162, (_e112 * _e167))));
                        let _e752 = fma(_e157, _e177, fma(_e137, _e172, fma(_e97, _e162, (_e117 * _e167))));
                        let _e767 = fma(_e152, _e197, fma(_e132, _e192, fma(_e92, _e182, (_e112 * _e187))));
                        let _e768 = fma(_e157, _e197, fma(_e137, _e192, fma(_e97, _e182, (_e117 * _e187))));
                        let _e783 = fma(_e152, _e217, fma(_e132, _e212, fma(_e92, _e202, (_e112 * _e207))));
                        let _e784 = fma(_e157, _e217, fma(_e137, _e212, fma(_e97, _e202, (_e117 * _e207))));
                        let _e799 = fma(_e152, _e237, fma(_e132, _e232, fma(_e92, _e222, (_e112 * _e227))));
                        let _e800 = fma(_e157, _e237, fma(_e137, _e232, fma(_e97, _e222, (_e117 * _e227))));
                        let _e812 = (fma(_e784, _e505, fma(_e752, _e503, (_e768 * _e504))) + _e800);
                        let _e817 = fma(_e511, _e285[5].x, _e503);
                        let _e818 = fma(_e511, _e285[5].y, _e504);
                        let _e819 = fma(_e511, _e285[5].z, _e505);
                        let _e833 = fma(_e511, _e285[0].x, _e503);
                        let _e834 = fma(_e511, _e285[0].y, _e504);
                        let _e835 = fma(_e511, _e285[0].z, _e505);
                        let _e852 = (vec2<f32>(((((fma(fma(_e142, _e217, fma(_e122, _e212, fma(_e82, _e202, (_e102 * _e207)))), _e505, fma(fma(_e142, _e177, fma(_e122, _e172, fma(_e82, _e162, (_e102 * _e167)))), _e503, (fma(_e142, _e197, fma(_e122, _e192, fma(_e82, _e182, (_e102 * _e187)))) * _e504))) + fma(_e142, _e237, fma(_e122, _e232, fma(_e82, _e222, (_e102 * _e227))))) / _e812) + 1f) * 0.5f), fma((-1f - ((fma(fma(_e147, _e217, fma(_e127, _e212, fma(_e87, _e202, (_e107 * _e207)))), _e505, fma(fma(_e147, _e177, fma(_e127, _e172, fma(_e87, _e162, (_e107 * _e167)))), _e503, (fma(_e147, _e197, fma(_e127, _e192, fma(_e87, _e182, (_e107 * _e187)))) * _e504))) + fma(_e147, _e237, fma(_e127, _e232, fma(_e87, _e222, (_e107 * _e227))))) / _e812)), 0.5f, 1f)) * vec2<f32>(_e734, f32(_e334)));
                        let _e853 = (_e511 / _e812);
                        let _e855 = -(_e734);
                        let _e859 = vec3<f32>(fma(_e855, _e853, _e852.x), fma(_e855, _e853, _e852.y), ((_e799 + fma(_e783, _e819, fma(_e767, _e818, (_e751 * _e817)))) / (_e800 + fma(_e784, _e819, fma(_e768, _e818, (_e752 * _e817))))));
                        let _e862 = vec3<f32>(fma(_e734, _e853, _e852.x), fma(_e734, _e853, _e852.y), ((_e799 + fma(_e783, _e835, fma(_e767, _e834, (_e751 * _e833)))) / (_e800 + fma(_e784, _e835, fma(_e768, _e834, (_e752 * _e833))))));
                        let _e863 = min(_e859, _e862);
                        let _e864 = max(_e859, _e862);
                        let _e885 = fma(-((_e864.x - _e863.x)), 0.5f, abs(fma(-((_e863.x + _e864.x)), 0.5f, (_e75.x + 0.5f))));
                        let _e887 = fma(-((_e864.y - _e863.y)), 0.5f, abs(fma(-((_e863.y + _e864.y)), 0.5f, (_e75.y + 0.5f))));
                        let _e888 = max(_e885, 0f);
                        let _e889 = max(_e887, 0f);
                        let _e898 = f32(select(0u, 1u, (_e864.z <= 1f)));
                        let _e899 = abs((min(max(_e885, _e887), 0f) + sqrt(fma(_e888, _e888, (_e889 * _e889)))));
                        if (_e899 < 0.5f) {
                            global_1 = vec4<f32>(0f, 0f, 0f, _e898);
                        } else {
                            if (_e899 <= 2f) {
                                global_1 = vec4<f32>(1f, 1f, 1f, (0.5f * _e898));
                            } else {
                                if (_e899 <= 3f) {
                                    global_1 = vec4<f32>(0.5f, 0.5f, 0.5f, _e898);
                                }
                            }
                        }
                        phi_2223_ = _e733;
                        phi_1558_ = true;
                        break;
                    }
                    default: {
                        phi_2223_ = _e337;
                        phi_1558_ = bool();
                        break;
                    }
                }
                let _e908 = phi_2223_;
                let _e910 = phi_1558_;
                continue;
                continuing {
                    phi_2198_ = _e908;
                    phi_817_ = _e352;
                    phi_2222_ = _e908;
                    break if !(_e910);
                }
            }
            let _e913 = phi_2222_;
            if _e913 {
                break;
            }
            break;
        }
    }
    return;
}

@fragment 
fn debugdebug_overlay_fragment(@builtin(position) param: vec4<f32>) -> @location(0) vec4<f32> {
    global_3 = param;
    function();
    let _e3 = global_1;
    return _e3;
}
