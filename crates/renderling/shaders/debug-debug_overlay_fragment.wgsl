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
    var phi_2208_: bool;
    var phi_539_: type_17;
    var phi_540_: type_17;
    var phi_563_: type_17;
    var phi_733_: type_17;
    var phi_734_: type_17;
    var phi_757_: type_17;
    var phi_784_: bool;
    var phi_790_: type_17;
    var phi_791_: type_17;
    var phi_814_: type_17;
    var phi_837_: bool;
    var phi_906_: type_26;
    var phi_1049_: type_17;
    var phi_1050_: type_17;
    var phi_1073_: type_17;
    var phi_1117_: bool;
    var phi_1121_: bool;
    var phi_1122_: bool;
    var phi_2195_: bool;
    var phi_1644_: bool;
    var phi_2216_: bool;
    var phi_1129_: type_17;
    var phi_1130_: type_17;
    var phi_1153_: type_17;
    var phi_1163_: type_17;
    var phi_1166_: i32;
    var phi_1164_: type_17;
    var phi_1189_: type_17;
    var phi_1230_: i32;
    var phi_1167_: i32;
    var phi_1231_: bool;
    var phi_2210_: bool;
    var local_8: i32;
    var phi_1238_: type_17;
    var phi_1241_: i32;
    var phi_1239_: type_17;
    var phi_1264_: type_17;
    var phi_1305_: i32;
    var phi_1242_: i32;
    var phi_1306_: bool;
    var phi_2217_: bool;
    var local_9: i32;
    var phi_2224_: bool;
    var phi_1312_: bool;
    var phi_2223_: bool;
    var phi_1314_: bool;
    var phi_2222_: bool;
    var phi_2232_: bool;
    var phi_2231_: bool;
    var phi_2227_: bool;
    var phi_1559_: bool;
    var phi_2226_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e75 = global_3;
            let _e78 = global.member[2u];
            let _e81 = global.member[3u];
            global_1 = vec4<f32>(0f, 0f, 0f, 0f);
            phi_2208_ = false;
            phi_539_ = type_17(0u, arrayLength((&global_2.member)));
            loop {
                let _e84 = phi_2208_;
                let _e86 = phi_539_;
                if (_e86.member < _e86.member_1) {
                    phi_540_ = type_17((_e86.member + 1u), _e86.member_1);
                    phi_563_ = type_17(1u, _e86.member);
                } else {
                    phi_540_ = _e86;
                    phi_563_ = type_17(0u, type_17().member_1);
                }
                let _e99 = phi_540_;
                let _e101 = phi_563_;
                switch bitcast<i32>(_e101.member) {
                    case 0: {
                        phi_2227_ = _e84;
                        phi_1559_ = false;
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
                        phi_733_ = type_17(0u, 6u);
                        loop {
                            let _e274 = phi_733_;
                            if (_e274.member < _e274.member_1) {
                                phi_734_ = type_17((_e274.member + 1u), _e274.member_1);
                                phi_757_ = type_17(1u, _e274.member);
                            } else {
                                phi_734_ = _e274;
                                phi_757_ = type_17(0u, type_17().member_1);
                            }
                            let _e287 = phi_734_;
                            let _e289 = phi_757_;
                            switch bitcast<i32>(_e289.member) {
                                case 0: {
                                    phi_784_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e294 = ((_e112 + 35u) + (_e289.member_1 * 4u));
                                    let _e297 = global.member[_e294];
                                    let _e302 = global.member[(_e294 + 1u)];
                                    let _e307 = global.member[(_e294 + 2u)];
                                    let _e312 = global.member[(_e294 + 3u)];
                                    local_7[_e289.member_1] = vec4<f32>(bitcast<f32>(_e297), bitcast<f32>(_e302), bitcast<f32>(_e307), bitcast<f32>(_e312));
                                    phi_784_ = true;
                                    break;
                                }
                                default: {
                                    phi_784_ = bool();
                                    break;
                                }
                            }
                            let _e317 = phi_784_;
                            continue;
                            continuing {
                                phi_733_ = _e287;
                                break if !(_e317);
                            }
                        }
                        let _e319 = local_7;
                        local_6 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                        phi_790_ = type_17(0u, 8u);
                        loop {
                            let _e322 = phi_790_;
                            if (_e322.member < _e322.member_1) {
                                phi_791_ = type_17((_e322.member + 1u), _e322.member_1);
                                phi_814_ = type_17(1u, _e322.member);
                            } else {
                                phi_791_ = _e322;
                                phi_814_ = type_17(0u, type_17().member_1);
                            }
                            let _e335 = phi_791_;
                            let _e337 = phi_814_;
                            switch bitcast<i32>(_e337.member) {
                                case 0: {
                                    phi_837_ = false;
                                    break;
                                }
                                case 1: {
                                    let _e342 = ((_e112 + 59u) + (_e337.member_1 * 3u));
                                    let _e345 = global.member[_e342];
                                    let _e350 = global.member[(_e342 + 1u)];
                                    let _e355 = global.member[(_e342 + 2u)];
                                    local_6[_e337.member_1] = vec3<f32>(bitcast<f32>(_e345), bitcast<f32>(_e350), bitcast<f32>(_e355));
                                    phi_837_ = true;
                                    break;
                                }
                                default: {
                                    phi_837_ = bool();
                                    break;
                                }
                            }
                            let _e360 = phi_837_;
                            continue;
                            continuing {
                                phi_790_ = _e335;
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
                            phi_906_ = type_26(vec3<f32>(bitcast<f32>(_e371), bitcast<f32>(_e376), bitcast<f32>(_e381)), vec4<f32>(bitcast<f32>(_e387), bitcast<f32>(_e392), bitcast<f32>(_e397), bitcast<f32>(_e402)), vec3<f32>(bitcast<f32>(_e408), bitcast<f32>(_e413), bitcast<f32>(_e418)));
                        } else {
                            phi_906_ = type_26(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                        }
                        let _e423 = phi_906_;
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
                            phi_2231_ = _e84;
                        } else {
                            phi_1049_ = type_17(0u, 6u);
                            loop {
                                let _e532 = phi_1049_;
                                if (_e532.member < _e532.member_1) {
                                    phi_1050_ = type_17((_e532.member + 1u), _e532.member_1);
                                    phi_1073_ = type_17(1u, _e532.member);
                                } else {
                                    phi_1050_ = _e532;
                                    phi_1073_ = type_17(0u, type_17().member_1);
                                }
                                let _e545 = phi_1050_;
                                let _e547 = phi_1073_;
                                switch bitcast<i32>(_e547.member) {
                                    case 0: {
                                        phi_1121_ = false;
                                        phi_1122_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if (_e547.member_1 != 0u) {
                                            if (_e547.member_1 < 6u) {
                                            } else {
                                                phi_2195_ = true;
                                                phi_1644_ = bool();
                                                break;
                                            }
                                            let _e555 = local[_e547.member_1][0u];
                                            let _e558 = local[_e547.member_1][1u];
                                            let _e563 = local[_e547.member_1][2u];
                                            let _e567 = local[_e547.member_1][3u];
                                            phi_1117_ = select(true, false, ((fma(_e563, _e506, fma(_e555, _e504, (_e558 * _e505))) + _e567) < _e529));
                                        } else {
                                            phi_1117_ = true;
                                        }
                                        let _e572 = phi_1117_;
                                        phi_1121_ = _e572;
                                        phi_1122_ = false;
                                        break;
                                    }
                                    default: {
                                        phi_1121_ = bool();
                                        phi_1122_ = bool();
                                        break;
                                    }
                                }
                                let _e574 = phi_1121_;
                                let _e576 = phi_1122_;
                                continue;
                                continuing {
                                    phi_1049_ = _e545;
                                    phi_2195_ = _e84;
                                    phi_1644_ = _e576;
                                    break if !(_e574);
                                }
                            }
                            let _e579 = phi_2195_;
                            let _e581 = phi_1644_;
                            phi_2226_ = _e579;
                            if _e579 {
                                break;
                            }
                            if _e581 {
                                let _e582 = vec3(_e512);
                                let _e583 = (_e507 - _e582);
                                let _e584 = (_e507 + _e582);
                                phi_2216_ = _e579;
                                phi_1129_ = type_17(0u, 3u);
                                loop {
                                    let _e586 = phi_2216_;
                                    let _e588 = phi_1129_;
                                    if (_e588.member < _e588.member_1) {
                                        phi_1130_ = type_17((_e588.member + 1u), _e588.member_1);
                                        phi_1153_ = type_17(1u, _e588.member);
                                    } else {
                                        phi_1130_ = _e588;
                                        phi_1153_ = type_17(0u, type_17().member_1);
                                    }
                                    let _e601 = phi_1130_;
                                    let _e603 = phi_1153_;
                                    switch bitcast<i32>(_e603.member) {
                                        case 0: {
                                            phi_2223_ = _e586;
                                            phi_1314_ = false;
                                            break;
                                        }
                                        case 1: {
                                            phi_1163_ = type_17(0u, 8u);
                                            phi_1166_ = 0i;
                                            loop {
                                                let _e608 = phi_1163_;
                                                let _e610 = phi_1166_;
                                                local_8 = _e610;
                                                if (_e608.member < _e608.member_1) {
                                                    phi_1164_ = type_17((_e608.member + 1u), _e608.member_1);
                                                    phi_1189_ = type_17(1u, _e608.member);
                                                } else {
                                                    phi_1164_ = _e608;
                                                    phi_1189_ = type_17(0u, type_17().member_1);
                                                }
                                                let _e623 = phi_1164_;
                                                let _e625 = phi_1189_;
                                                switch bitcast<i32>(_e625.member) {
                                                    case 0: {
                                                        phi_1167_ = i32();
                                                        phi_1231_ = false;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if (_e625.member_1 < 8u) {
                                                        } else {
                                                            phi_2210_ = true;
                                                            break;
                                                        }
                                                        let _e632 = local_1[_e625.member_1][0u];
                                                        let _e635 = local_1[_e625.member_1][1u];
                                                        let _e638 = local_1[_e625.member_1][2u];
                                                        local_2 = array<f32, 3>(_e632, _e635, _e638);
                                                        let _e640 = (_e603.member_1 < 3u);
                                                        if _e640 {
                                                        } else {
                                                            phi_2210_ = true;
                                                            break;
                                                        }
                                                        let _e642 = local_2[_e603.member_1];
                                                        local_3 = array<f32, 3>(_e583.x, _e583.y, _e583.z);
                                                        if _e640 {
                                                        } else {
                                                            phi_2210_ = true;
                                                            break;
                                                        }
                                                        let _e648 = local_3[_e603.member_1];
                                                        if (_e642 < _e648) {
                                                            phi_1230_ = (_e610 + 1i);
                                                        } else {
                                                            phi_1230_ = _e610;
                                                        }
                                                        let _e652 = phi_1230_;
                                                        phi_1167_ = _e652;
                                                        phi_1231_ = true;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_1167_ = i32();
                                                        phi_1231_ = bool();
                                                        break;
                                                    }
                                                }
                                                let _e654 = phi_1167_;
                                                let _e656 = phi_1231_;
                                                continue;
                                                continuing {
                                                    phi_1163_ = _e623;
                                                    phi_1166_ = _e654;
                                                    phi_2210_ = _e586;
                                                    break if !(_e656);
                                                }
                                            }
                                            let _e659 = phi_2210_;
                                            phi_2222_ = _e659;
                                            if _e659 {
                                                break;
                                            }
                                            let _e661 = local_8;
                                            if (_e661 == 8i) {
                                                phi_2224_ = _e659;
                                                phi_1312_ = false;
                                            } else {
                                                phi_1238_ = type_17(0u, 8u);
                                                phi_1241_ = 0i;
                                                loop {
                                                    let _e664 = phi_1238_;
                                                    let _e666 = phi_1241_;
                                                    local_9 = _e666;
                                                    if (_e664.member < _e664.member_1) {
                                                        phi_1239_ = type_17((_e664.member + 1u), _e664.member_1);
                                                        phi_1264_ = type_17(1u, _e664.member);
                                                    } else {
                                                        phi_1239_ = _e664;
                                                        phi_1264_ = type_17(0u, type_17().member_1);
                                                    }
                                                    let _e679 = phi_1239_;
                                                    let _e681 = phi_1264_;
                                                    switch bitcast<i32>(_e681.member) {
                                                        case 0: {
                                                            phi_1242_ = i32();
                                                            phi_1306_ = false;
                                                            break;
                                                        }
                                                        case 1: {
                                                            if (_e681.member_1 < 8u) {
                                                            } else {
                                                                phi_2217_ = true;
                                                                break;
                                                            }
                                                            let _e688 = local_1[_e681.member_1][0u];
                                                            let _e691 = local_1[_e681.member_1][1u];
                                                            let _e694 = local_1[_e681.member_1][2u];
                                                            local_4 = array<f32, 3>(_e688, _e691, _e694);
                                                            let _e696 = (_e603.member_1 < 3u);
                                                            if _e696 {
                                                            } else {
                                                                phi_2217_ = true;
                                                                break;
                                                            }
                                                            let _e698 = local_4[_e603.member_1];
                                                            local_5 = array<f32, 3>(_e584.x, _e584.y, _e584.z);
                                                            if _e696 {
                                                            } else {
                                                                phi_2217_ = true;
                                                                break;
                                                            }
                                                            let _e704 = local_5[_e603.member_1];
                                                            if (_e698 > _e704) {
                                                                phi_1305_ = (_e666 + 1i);
                                                            } else {
                                                                phi_1305_ = _e666;
                                                            }
                                                            let _e708 = phi_1305_;
                                                            phi_1242_ = _e708;
                                                            phi_1306_ = true;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_1242_ = i32();
                                                            phi_1306_ = bool();
                                                            break;
                                                        }
                                                    }
                                                    let _e710 = phi_1242_;
                                                    let _e712 = phi_1306_;
                                                    continue;
                                                    continuing {
                                                        phi_1238_ = _e679;
                                                        phi_1241_ = _e710;
                                                        phi_2217_ = _e659;
                                                        break if !(_e712);
                                                    }
                                                }
                                                let _e715 = phi_2217_;
                                                phi_2222_ = _e715;
                                                if _e715 {
                                                    break;
                                                }
                                                let _e717 = local_9;
                                                phi_2224_ = _e715;
                                                phi_1312_ = select(true, false, (_e717 == 8i));
                                            }
                                            let _e721 = phi_2224_;
                                            let _e723 = phi_1312_;
                                            phi_2223_ = _e721;
                                            phi_1314_ = _e723;
                                            break;
                                        }
                                        default: {
                                            phi_2223_ = _e586;
                                            phi_1314_ = bool();
                                            break;
                                        }
                                    }
                                    let _e725 = phi_2223_;
                                    let _e727 = phi_1314_;
                                    continue;
                                    continuing {
                                        phi_2216_ = _e725;
                                        phi_1129_ = _e601;
                                        phi_2222_ = _e725;
                                        break if !(_e727);
                                    }
                                }
                                let _e730 = phi_2222_;
                                phi_2226_ = _e730;
                                if _e730 {
                                    break;
                                }
                                phi_2232_ = _e730;
                            } else {
                                phi_2232_ = _e579;
                            }
                            let _e732 = phi_2232_;
                            phi_2231_ = _e732;
                        }
                        let _e734 = phi_2231_;
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
                        phi_2227_ = _e734;
                        phi_1559_ = true;
                        break;
                    }
                    default: {
                        phi_2227_ = _e84;
                        phi_1559_ = bool();
                        break;
                    }
                }
                let _e909 = phi_2227_;
                let _e911 = phi_1559_;
                continue;
                continuing {
                    phi_2208_ = _e909;
                    phi_539_ = _e99;
                    phi_2226_ = _e909;
                    break if !(_e911);
                }
            }
            let _e914 = phi_2226_;
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
