struct type_10 {
    member: array<u32>,
}

struct type_17 {
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
var<storage> global: type_10;
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
    var phi_2139_: bool;
    var phi_540_: type_17;
    var phi_541_: type_17;
    var phi_556_: type_17;
    var phi_726_: type_17;
    var phi_727_: type_17;
    var phi_742_: type_17;
    var phi_769_: bool;
    var phi_775_: type_17;
    var phi_776_: type_17;
    var phi_791_: type_17;
    var phi_814_: bool;
    var phi_870_: type_26;
    var phi_1013_: type_17;
    var phi_1014_: type_17;
    var phi_1029_: type_17;
    var phi_1073_: bool;
    var phi_1077_: bool;
    var phi_1078_: bool;
    var phi_2127_: bool;
    var phi_1576_: bool;
    var phi_2147_: bool;
    var phi_1085_: type_17;
    var phi_1086_: type_17;
    var phi_1101_: type_17;
    var phi_1111_: type_17;
    var phi_1114_: i32;
    var phi_1112_: type_17;
    var phi_1129_: type_17;
    var phi_1170_: i32;
    var phi_1115_: i32;
    var phi_1171_: bool;
    var phi_2142_: bool;
    var local_8: i32;
    var phi_1178_: type_17;
    var phi_1181_: i32;
    var phi_1179_: type_17;
    var phi_1196_: type_17;
    var phi_1237_: i32;
    var phi_1182_: i32;
    var phi_1238_: bool;
    var phi_2149_: bool;
    var local_9: i32;
    var phi_2156_: bool;
    var phi_1244_: bool;
    var phi_2155_: bool;
    var phi_1246_: bool;
    var phi_2154_: bool;
    var phi_2162_: bool;
    var phi_2161_: bool;
    var phi_2159_: bool;
    var phi_1491_: bool;
    var phi_2158_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e75 = global_3;
            let _e78 = global.member[2u];
            let _e81 = global.member[3u];
            global_1 = vec4<f32>(0f, 0f, 0f, 0f);
            phi_2139_ = false;
            phi_540_ = type_17(0u, arrayLength((&global_2.member)));
            loop {
                let _e84 = phi_2139_;
                let _e86 = phi_540_;
                if (_e86.member < _e86.member_1) {
                    phi_541_ = type_17((_e86.member + 1u), _e86.member_1);
                    phi_556_ = type_17(1u, _e86.member);
                } else {
                    phi_541_ = _e86;
                    phi_556_ = type_17(0u, type_17().member_1);
                }
                let _e99 = phi_541_;
                let _e101 = phi_556_;
                switch bitcast<i32>(_e101.member) {
                    case 0: {
                        phi_2159_ = _e84;
                        phi_1491_ = false;
                        break;
                    }
                    case 1: {
                        let _e108 = global_2.member[_e101.member_1].member_3;
                        let _e112 = global.member[(_e108 + 9u)];
                        let _e115 = global.member[_e112];
                        let _e116 = bitcast<f32>(_e115);
                        let _e120 = global.member[(_e112 + 1u)];
                        let _e121 = bitcast<f32>(_e120);
                        let _e125 = global.member[(_e112 + 2u)];
                        let _e126 = bitcast<f32>(_e125);
                        let _e130 = global.member[(_e112 + 3u)];
                        let _e131 = bitcast<f32>(_e130);
                        let _e135 = global.member[(_e112 + 4u)];
                        let _e136 = bitcast<f32>(_e135);
                        let _e140 = global.member[(_e112 + 5u)];
                        let _e141 = bitcast<f32>(_e140);
                        let _e145 = global.member[(_e112 + 6u)];
                        let _e146 = bitcast<f32>(_e145);
                        let _e150 = global.member[(_e112 + 7u)];
                        let _e151 = bitcast<f32>(_e150);
                        let _e155 = global.member[(_e112 + 8u)];
                        let _e156 = bitcast<f32>(_e155);
                        let _e160 = global.member[(_e112 + 9u)];
                        let _e161 = bitcast<f32>(_e160);
                        let _e165 = global.member[(_e112 + 10u)];
                        let _e166 = bitcast<f32>(_e165);
                        let _e170 = global.member[(_e112 + 11u)];
                        let _e171 = bitcast<f32>(_e170);
                        let _e175 = global.member[(_e112 + 12u)];
                        let _e176 = bitcast<f32>(_e175);
                        let _e180 = global.member[(_e112 + 13u)];
                        let _e181 = bitcast<f32>(_e180);
                        let _e185 = global.member[(_e112 + 14u)];
                        let _e186 = bitcast<f32>(_e185);
                        let _e190 = global.member[(_e112 + 15u)];
                        let _e191 = bitcast<f32>(_e190);
                        let _e195 = global.member[(_e112 + 16u)];
                        let _e196 = bitcast<f32>(_e195);
                        let _e200 = global.member[(_e112 + 17u)];
                        let _e201 = bitcast<f32>(_e200);
                        let _e205 = global.member[(_e112 + 18u)];
                        let _e206 = bitcast<f32>(_e205);
                        let _e210 = global.member[(_e112 + 19u)];
                        let _e211 = bitcast<f32>(_e210);
                        let _e215 = global.member[(_e112 + 20u)];
                        let _e216 = bitcast<f32>(_e215);
                        let _e220 = global.member[(_e112 + 21u)];
                        let _e221 = bitcast<f32>(_e220);
                        let _e225 = global.member[(_e112 + 22u)];
                        let _e226 = bitcast<f32>(_e225);
                        let _e230 = global.member[(_e112 + 23u)];
                        let _e231 = bitcast<f32>(_e230);
                        let _e235 = global.member[(_e112 + 24u)];
                        let _e236 = bitcast<f32>(_e235);
                        let _e240 = global.member[(_e112 + 25u)];
                        let _e241 = bitcast<f32>(_e240);
                        let _e245 = global.member[(_e112 + 26u)];
                        let _e246 = bitcast<f32>(_e245);
                        let _e250 = global.member[(_e112 + 27u)];
                        let _e251 = bitcast<f32>(_e250);
                        let _e255 = global.member[(_e112 + 28u)];
                        let _e256 = bitcast<f32>(_e255);
                        let _e260 = global.member[(_e112 + 29u)];
                        let _e261 = bitcast<f32>(_e260);
                        let _e265 = global.member[(_e112 + 30u)];
                        let _e266 = bitcast<f32>(_e265);
                        let _e270 = global.member[(_e112 + 31u)];
                        let _e271 = bitcast<f32>(_e270);
                        local_7 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        phi_726_ = type_17(0u, 6u);
                        loop {
                            let _e274 = phi_726_;
                            if (_e274.member < _e274.member_1) {
                                phi_727_ = type_17((_e274.member + 1u), _e274.member_1);
                                phi_742_ = type_17(1u, _e274.member);
                            } else {
                                phi_727_ = _e274;
                                phi_742_ = type_17(0u, type_17().member_1);
                            }
                            let _e287 = phi_727_;
                            let _e289 = phi_742_;
                            switch bitcast<i32>(_e289.member) {
                                case 0: {
                                    phi_769_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e294 = ((_e112 + 35u) + (_e289.member_1 * 4u));
                                    let _e297 = global.member[_e294];
                                    let _e302 = global.member[(_e294 + 1u)];
                                    let _e307 = global.member[(_e294 + 2u)];
                                    let _e312 = global.member[(_e294 + 3u)];
                                    local_7[_e289.member_1] = vec4<f32>(bitcast<f32>(_e297), bitcast<f32>(_e302), bitcast<f32>(_e307), bitcast<f32>(_e312));
                                    phi_769_ = true;
                                    break;
                                }
                                default: {
                                    phi_769_ = bool();
                                    break;
                                }
                            }
                            let _e317 = phi_769_;
                            continue;
                            continuing {
                                phi_726_ = _e287;
                                break if !(_e317);
                            }
                        }
                        let _e319 = local_7;
                        local_6 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                        phi_775_ = type_17(0u, 8u);
                        loop {
                            let _e322 = phi_775_;
                            if (_e322.member < _e322.member_1) {
                                phi_776_ = type_17((_e322.member + 1u), _e322.member_1);
                                phi_791_ = type_17(1u, _e322.member);
                            } else {
                                phi_776_ = _e322;
                                phi_791_ = type_17(0u, type_17().member_1);
                            }
                            let _e335 = phi_776_;
                            let _e337 = phi_791_;
                            switch bitcast<i32>(_e337.member) {
                                case 0: {
                                    phi_814_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e342 = ((_e112 + 59u) + (_e337.member_1 * 3u));
                                    let _e345 = global.member[_e342];
                                    let _e350 = global.member[(_e342 + 1u)];
                                    let _e355 = global.member[(_e342 + 2u)];
                                    local_6[_e337.member_1] = vec3<f32>(bitcast<f32>(_e345), bitcast<f32>(_e350), bitcast<f32>(_e355));
                                    phi_814_ = true;
                                    break;
                                }
                                default: {
                                    phi_814_ = bool();
                                    break;
                                }
                            }
                            let _e360 = phi_814_;
                            continue;
                            continuing {
                                phi_775_ = _e335;
                                break if !(_e360);
                            }
                        }
                        let _e362 = local_6;
                        let _e366 = global.member[(_e108 + 10u)];
                        if ((_e366 == 4294967295u) != true) {
                            let _e371 = global.member[_e366];
                            let _e376 = global.member[(_e366 + 1u)];
                            let _e381 = global.member[(_e366 + 2u)];
                            let _e387 = global.member[(_e366 + 3u)];
                            let _e392 = global.member[(_e366 + 4u)];
                            let _e397 = global.member[(_e366 + 5u)];
                            let _e402 = global.member[(_e366 + 6u)];
                            let _e408 = global.member[(_e366 + 7u)];
                            let _e413 = global.member[(_e366 + 8u)];
                            let _e418 = global.member[(_e366 + 9u)];
                            phi_870_ = type_26(vec3<f32>(bitcast<f32>(_e371), bitcast<f32>(_e376), bitcast<f32>(_e381)), vec4<f32>(bitcast<f32>(_e387), bitcast<f32>(_e392), bitcast<f32>(_e397), bitcast<f32>(_e402)), vec3<f32>(bitcast<f32>(_e408), bitcast<f32>(_e413), bitcast<f32>(_e418)));
                        } else {
                            phi_870_ = type_26(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                        }
                        let _e423 = phi_870_;
                        let _e427 = global.member[(_e108 + 3u)];
                        let _e428 = bitcast<f32>(_e427);
                        let _e432 = global.member[(_e108 + 4u)];
                        let _e433 = bitcast<f32>(_e432);
                        let _e437 = global.member[(_e108 + 5u)];
                        let _e438 = bitcast<f32>(_e437);
                        let _e442 = global.member[(_e108 + 6u)];
                        let _e451 = (_e423.member_1.x + _e423.member_1.x);
                        let _e452 = (_e423.member_1.y + _e423.member_1.y);
                        let _e453 = (_e423.member_1.z + _e423.member_1.z);
                        let _e455 = (_e423.member_1.z * _e453);
                        let _e456 = (_e423.member_1.w * _e451);
                        let _e457 = (_e423.member_1.w * _e452);
                        let _e458 = (_e423.member_1.w * _e453);
                        let _e478 = (vec4<f32>((1f - fma(_e423.member_1.y, _e452, _e455)), fma(_e423.member_1.x, _e452, _e458), fma(_e423.member_1.x, _e453, -(_e457)), 0f) * _e423.member_2.x);
                        let _e480 = (vec4<f32>(fma(_e423.member_1.x, _e452, -(_e458)), (1f - fma(_e423.member_1.x, _e451, _e455)), fma(_e423.member_1.y, _e453, _e456), 0f) * _e423.member_2.y);
                        let _e482 = (vec4<f32>(fma(_e423.member_1.x, _e453, _e457), fma(_e423.member_1.y, _e453, -(_e456)), (1f - fma(_e423.member_1.x, _e451, (_e423.member_1.y * _e452))), 0f) * _e423.member_2.z);
                        let _e504 = (_e423.member.x + fma(_e482.x, _e438, fma(_e480.x, _e433, (_e478.x * _e428))));
                        let _e505 = (_e423.member.y + fma(_e482.y, _e438, fma(_e480.y, _e433, (_e478.y * _e428))));
                        let _e506 = (_e423.member.z + fma(_e482.z, _e438, fma(_e480.z, _e433, (_e478.z * _e428))));
                        let _e507 = vec3<f32>(_e504, _e505, _e506);
                        let _e510 = (max(_e423.member_2.x, max(_e423.member_2.y, _e423.member_2.z)) * bitcast<f32>(_e442));
                        let _e512 = sqrt((_e510 * _e510));
                        local_1 = _e362;
                        local = _e319;
                        let _e515 = local[0u][0u];
                        let _e518 = local[0u][1u];
                        let _e523 = local[0u][2u];
                        let _e527 = local[0u][3u];
                        let _e529 = -(_e512);
                        if ((fma(_e523, _e506, fma(_e515, _e504, (_e518 * _e505))) + _e527) < _e529) {
                            phi_2161_ = _e84;
                        } else {
                            phi_1013_ = type_17(0u, 6u);
                            loop {
                                let _e532 = phi_1013_;
                                if (_e532.member < _e532.member_1) {
                                    phi_1014_ = type_17((_e532.member + 1u), _e532.member_1);
                                    phi_1029_ = type_17(1u, _e532.member);
                                } else {
                                    phi_1014_ = _e532;
                                    phi_1029_ = type_17(0u, type_17().member_1);
                                }
                                let _e545 = phi_1014_;
                                let _e547 = phi_1029_;
                                switch bitcast<i32>(_e547.member) {
                                    case 0: {
                                        phi_1077_ = false;
                                        phi_1078_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if (_e547.member_1 != 0u) {
                                            if (_e547.member_1 < 6u) {
                                            } else {
                                                phi_2127_ = true;
                                                phi_1576_ = bool();
                                                break;
                                            }
                                            let _e555 = local[_e547.member_1][0u];
                                            let _e558 = local[_e547.member_1][1u];
                                            let _e563 = local[_e547.member_1][2u];
                                            let _e567 = local[_e547.member_1][3u];
                                            phi_1073_ = select(true, false, ((fma(_e563, _e506, fma(_e555, _e504, (_e558 * _e505))) + _e567) < _e529));
                                        } else {
                                            phi_1073_ = true;
                                        }
                                        let _e572 = phi_1073_;
                                        phi_1077_ = _e572;
                                        phi_1078_ = false;
                                        break;
                                    }
                                    default: {
                                        phi_1077_ = bool();
                                        phi_1078_ = bool();
                                        break;
                                    }
                                }
                                let _e574 = phi_1077_;
                                let _e576 = phi_1078_;
                                continue;
                                continuing {
                                    phi_1013_ = _e545;
                                    phi_2127_ = _e84;
                                    phi_1576_ = _e576;
                                    break if !(_e574);
                                }
                            }
                            let _e579 = phi_2127_;
                            let _e581 = phi_1576_;
                            phi_2158_ = _e579;
                            if _e579 {
                                break;
                            }
                            if _e581 {
                                let _e582 = vec3(_e512);
                                let _e583 = (_e507 - _e582);
                                let _e584 = (_e507 + _e582);
                                phi_2147_ = _e579;
                                phi_1085_ = type_17(0u, 3u);
                                loop {
                                    let _e586 = phi_2147_;
                                    let _e588 = phi_1085_;
                                    if (_e588.member < _e588.member_1) {
                                        phi_1086_ = type_17((_e588.member + 1u), _e588.member_1);
                                        phi_1101_ = type_17(1u, _e588.member);
                                    } else {
                                        phi_1086_ = _e588;
                                        phi_1101_ = type_17(0u, type_17().member_1);
                                    }
                                    let _e601 = phi_1086_;
                                    let _e603 = phi_1101_;
                                    switch bitcast<i32>(_e603.member) {
                                        case 0: {
                                            phi_2155_ = _e586;
                                            phi_1246_ = false;
                                            break;
                                        }
                                        case 1: {
                                            phi_1111_ = type_17(0u, 8u);
                                            phi_1114_ = 0i;
                                            loop {
                                                let _e608 = phi_1111_;
                                                let _e610 = phi_1114_;
                                                local_8 = _e610;
                                                if (_e608.member < _e608.member_1) {
                                                    phi_1112_ = type_17((_e608.member + 1u), _e608.member_1);
                                                    phi_1129_ = type_17(1u, _e608.member);
                                                } else {
                                                    phi_1112_ = _e608;
                                                    phi_1129_ = type_17(0u, type_17().member_1);
                                                }
                                                let _e623 = phi_1112_;
                                                let _e625 = phi_1129_;
                                                switch bitcast<i32>(_e625.member) {
                                                    case 0: {
                                                        phi_1115_ = i32();
                                                        phi_1171_ = false;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if (_e625.member_1 < 8u) {
                                                        } else {
                                                            phi_2142_ = true;
                                                            break;
                                                        }
                                                        let _e632 = local_1[_e625.member_1][0u];
                                                        let _e635 = local_1[_e625.member_1][1u];
                                                        let _e638 = local_1[_e625.member_1][2u];
                                                        local_2 = array<f32, 3>(_e632, _e635, _e638);
                                                        let _e640 = (_e603.member_1 < 3u);
                                                        if _e640 {
                                                        } else {
                                                            phi_2142_ = true;
                                                            break;
                                                        }
                                                        let _e642 = local_2[_e603.member_1];
                                                        local_3 = array<f32, 3>(_e583.x, _e583.y, _e583.z);
                                                        if _e640 {
                                                        } else {
                                                            phi_2142_ = true;
                                                            break;
                                                        }
                                                        let _e648 = local_3[_e603.member_1];
                                                        if (_e642 < _e648) {
                                                            phi_1170_ = (_e610 + 1i);
                                                        } else {
                                                            phi_1170_ = _e610;
                                                        }
                                                        let _e652 = phi_1170_;
                                                        phi_1115_ = _e652;
                                                        phi_1171_ = true;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_1115_ = i32();
                                                        phi_1171_ = bool();
                                                        break;
                                                    }
                                                }
                                                let _e654 = phi_1115_;
                                                let _e656 = phi_1171_;
                                                continue;
                                                continuing {
                                                    phi_1111_ = _e623;
                                                    phi_1114_ = _e654;
                                                    phi_2142_ = _e586;
                                                    break if !(_e656);
                                                }
                                            }
                                            let _e659 = phi_2142_;
                                            phi_2154_ = _e659;
                                            if _e659 {
                                                break;
                                            }
                                            let _e661 = local_8;
                                            if (_e661 == 8i) {
                                                phi_2156_ = _e659;
                                                phi_1244_ = false;
                                            } else {
                                                phi_1178_ = type_17(0u, 8u);
                                                phi_1181_ = 0i;
                                                loop {
                                                    let _e664 = phi_1178_;
                                                    let _e666 = phi_1181_;
                                                    local_9 = _e666;
                                                    if (_e664.member < _e664.member_1) {
                                                        phi_1179_ = type_17((_e664.member + 1u), _e664.member_1);
                                                        phi_1196_ = type_17(1u, _e664.member);
                                                    } else {
                                                        phi_1179_ = _e664;
                                                        phi_1196_ = type_17(0u, type_17().member_1);
                                                    }
                                                    let _e679 = phi_1179_;
                                                    let _e681 = phi_1196_;
                                                    switch bitcast<i32>(_e681.member) {
                                                        case 0: {
                                                            phi_1182_ = i32();
                                                            phi_1238_ = false;
                                                            break;
                                                        }
                                                        case 1: {
                                                            if (_e681.member_1 < 8u) {
                                                            } else {
                                                                phi_2149_ = true;
                                                                break;
                                                            }
                                                            let _e688 = local_1[_e681.member_1][0u];
                                                            let _e691 = local_1[_e681.member_1][1u];
                                                            let _e694 = local_1[_e681.member_1][2u];
                                                            local_4 = array<f32, 3>(_e688, _e691, _e694);
                                                            let _e696 = (_e603.member_1 < 3u);
                                                            if _e696 {
                                                            } else {
                                                                phi_2149_ = true;
                                                                break;
                                                            }
                                                            let _e698 = local_4[_e603.member_1];
                                                            local_5 = array<f32, 3>(_e584.x, _e584.y, _e584.z);
                                                            if _e696 {
                                                            } else {
                                                                phi_2149_ = true;
                                                                break;
                                                            }
                                                            let _e704 = local_5[_e603.member_1];
                                                            if (_e698 > _e704) {
                                                                phi_1237_ = (_e666 + 1i);
                                                            } else {
                                                                phi_1237_ = _e666;
                                                            }
                                                            let _e708 = phi_1237_;
                                                            phi_1182_ = _e708;
                                                            phi_1238_ = true;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_1182_ = i32();
                                                            phi_1238_ = bool();
                                                            break;
                                                        }
                                                    }
                                                    let _e710 = phi_1182_;
                                                    let _e712 = phi_1238_;
                                                    continue;
                                                    continuing {
                                                        phi_1178_ = _e679;
                                                        phi_1181_ = _e710;
                                                        phi_2149_ = _e659;
                                                        break if !(_e712);
                                                    }
                                                }
                                                let _e715 = phi_2149_;
                                                phi_2154_ = _e715;
                                                if _e715 {
                                                    break;
                                                }
                                                let _e717 = local_9;
                                                phi_2156_ = _e715;
                                                phi_1244_ = select(true, false, (_e717 == 8i));
                                            }
                                            let _e721 = phi_2156_;
                                            let _e723 = phi_1244_;
                                            phi_2155_ = _e721;
                                            phi_1246_ = _e723;
                                            break;
                                        }
                                        default: {
                                            phi_2155_ = _e586;
                                            phi_1246_ = bool();
                                            break;
                                        }
                                    }
                                    let _e725 = phi_2155_;
                                    let _e727 = phi_1246_;
                                    continue;
                                    continuing {
                                        phi_2147_ = _e725;
                                        phi_1085_ = _e601;
                                        phi_2154_ = _e725;
                                        break if !(_e727);
                                    }
                                }
                                let _e730 = phi_2154_;
                                phi_2158_ = _e730;
                                if _e730 {
                                    break;
                                }
                                phi_2162_ = _e730;
                            } else {
                                phi_2162_ = _e579;
                            }
                            let _e732 = phi_2162_;
                            phi_2161_ = _e732;
                        }
                        let _e734 = phi_2161_;
                        let _e735 = f32(_e78);
                        let _e752 = fma(_e186, _e211, fma(_e166, _e206, fma(_e126, _e196, (_e146 * _e201))));
                        let _e753 = fma(_e191, _e211, fma(_e171, _e206, fma(_e131, _e196, (_e151 * _e201))));
                        let _e768 = fma(_e186, _e231, fma(_e166, _e226, fma(_e126, _e216, (_e146 * _e221))));
                        let _e769 = fma(_e191, _e231, fma(_e171, _e226, fma(_e131, _e216, (_e151 * _e221))));
                        let _e784 = fma(_e186, _e251, fma(_e166, _e246, fma(_e126, _e236, (_e146 * _e241))));
                        let _e785 = fma(_e191, _e251, fma(_e171, _e246, fma(_e131, _e236, (_e151 * _e241))));
                        let _e800 = fma(_e186, _e271, fma(_e166, _e266, fma(_e126, _e256, (_e146 * _e261))));
                        let _e801 = fma(_e191, _e271, fma(_e171, _e266, fma(_e131, _e256, (_e151 * _e261))));
                        let _e813 = (fma(_e785, _e506, fma(_e753, _e504, (_e769 * _e505))) + _e801);
                        let _e818 = fma(_e512, _e319[5].x, _e504);
                        let _e819 = fma(_e512, _e319[5].y, _e505);
                        let _e820 = fma(_e512, _e319[5].z, _e506);
                        let _e834 = fma(_e512, _e319[0].x, _e504);
                        let _e835 = fma(_e512, _e319[0].y, _e505);
                        let _e836 = fma(_e512, _e319[0].z, _e506);
                        let _e853 = (vec2<f32>(((((fma(fma(_e176, _e251, fma(_e156, _e246, fma(_e116, _e236, (_e136 * _e241)))), _e506, fma(fma(_e176, _e211, fma(_e156, _e206, fma(_e116, _e196, (_e136 * _e201)))), _e504, (fma(_e176, _e231, fma(_e156, _e226, fma(_e116, _e216, (_e136 * _e221)))) * _e505))) + fma(_e176, _e271, fma(_e156, _e266, fma(_e116, _e256, (_e136 * _e261))))) / _e813) + 1f) * 0.5f), fma((-1f - ((fma(fma(_e181, _e251, fma(_e161, _e246, fma(_e121, _e236, (_e141 * _e241)))), _e506, fma(fma(_e181, _e211, fma(_e161, _e206, fma(_e121, _e196, (_e141 * _e201)))), _e504, (fma(_e181, _e231, fma(_e161, _e226, fma(_e121, _e216, (_e141 * _e221)))) * _e505))) + fma(_e181, _e271, fma(_e161, _e266, fma(_e121, _e256, (_e141 * _e261))))) / _e813)), 0.5f, 1f)) * vec2<f32>(_e735, f32(_e81)));
                        let _e854 = (_e512 / _e813);
                        let _e856 = -(_e735);
                        let _e860 = vec3<f32>(fma(_e856, _e854, _e853.x), fma(_e856, _e854, _e853.y), ((_e800 + fma(_e784, _e820, fma(_e768, _e819, (_e752 * _e818)))) / (_e801 + fma(_e785, _e820, fma(_e769, _e819, (_e753 * _e818))))));
                        let _e863 = vec3<f32>(fma(_e735, _e854, _e853.x), fma(_e735, _e854, _e853.y), ((_e800 + fma(_e784, _e836, fma(_e768, _e835, (_e752 * _e834)))) / (_e801 + fma(_e785, _e836, fma(_e769, _e835, (_e753 * _e834))))));
                        let _e864 = min(_e860, _e863);
                        let _e865 = max(_e860, _e863);
                        let _e886 = fma(-((_e865.x - _e864.x)), 0.5f, abs(fma(-((_e864.x + _e865.x)), 0.5f, (_e75.x + 0.5f))));
                        let _e888 = fma(-((_e865.y - _e864.y)), 0.5f, abs(fma(-((_e864.y + _e865.y)), 0.5f, (_e75.y + 0.5f))));
                        let _e889 = max(_e886, 0f);
                        let _e890 = max(_e888, 0f);
                        let _e899 = f32(select(0u, 1u, (_e865.z <= 1f)));
                        let _e900 = abs((min(max(_e886, _e888), 0f) + sqrt(fma(_e889, _e889, (_e890 * _e890)))));
                        if (_e900 < 0.5f) {
                            global_1 = vec4<f32>(0f, 0f, 0f, _e899);
                        } else {
                            if (_e900 <= 2f) {
                                global_1 = vec4<f32>(1f, 1f, 1f, (0.5f * _e899));
                            } else {
                                if (_e900 <= 3f) {
                                    global_1 = vec4<f32>(0.5f, 0.5f, 0.5f, _e899);
                                }
                            }
                        }
                        phi_2159_ = _e734;
                        phi_1491_ = true;
                        break;
                    }
                    default: {
                        phi_2159_ = _e84;
                        phi_1491_ = bool();
                        break;
                    }
                }
                let _e909 = phi_2159_;
                let _e911 = phi_1491_;
                continue;
                continuing {
                    phi_2139_ = _e909;
                    phi_540_ = _e99;
                    phi_2158_ = _e909;
                    break if !(_e911);
                }
            }
            let _e914 = phi_2158_;
            if _e914 {
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
