struct type_3 {
    member: array<u32>,
}

struct type_13 {
    member: u32,
    member_1: u32,
}

struct type_15 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: vec4<f32>;
var<private> global_2: u32;
var<private> global_3: vec3<f32>;

fn function() {
    var phi_354_: f32;
    var phi_390_: f32;
    var phi_391_: f32;
    var phi_392_: f32;
    var phi_393_: f32;
    var phi_420_: f32;
    var phi_421_: f32;
    var phi_422_: f32;
    var phi_1984_: f32;
    var phi_484_: f32;
    var phi_485_: f32;
    var phi_486_: f32;
    var phi_487_: f32;
    var phi_488_: f32;
    var phi_489_: f32;
    var phi_490_: f32;
    var phi_2141_: f32;
    var phi_2231_: f32;
    var phi_578_: f32;
    var phi_579_: f32;
    var phi_580_: f32;
    var phi_581_: f32;
    var phi_582_: f32;
    var phi_583_: f32;
    var phi_584_: f32;
    var phi_585_: f32;
    var phi_586_: f32;
    var phi_587_: f32;
    var phi_3219_: u32;
    var phi_640_: f32;
    var phi_643_: f32;
    var phi_2327_: u32;
    var phi_3227_: u32;
    var phi_2334_: type_13;
    var phi_684_: type_13;
    var phi_695_: type_13;
    var phi_696_: type_13;
    var phi_2444_: f32;
    var phi_2511_: bool;
    var phi_771_: f32;
    var phi_772_: f32;
    var phi_773_: f32;
    var phi_774_: f32;
    var phi_775_: f32;
    var phi_776_: f32;
    var phi_777_: f32;
    var phi_778_: f32;
    var phi_779_: f32;
    var phi_780_: f32;
    var phi_781_: f32;
    var phi_782_: f32;
    var phi_783_: f32;
    var phi_784_: f32;
    var phi_785_: f32;
    var phi_2649_: f32;
    var phi_2739_: f32;
    var phi_878_: type_15;
    var phi_881_: f32;
    var phi_2816_: bool;
    var phi_2896_: bool;
    var phi_894_: f32;
    var phi_2990_: bool;
    var phi_3070_: bool;
    var phi_912_: type_15;
    var phi_913_: f32;
    var phi_915_: f32;
    var phi_916_: bool;
    var phi_879_: type_15;
    var phi_882_: f32;
    var phi_917_: f32;
    var phi_918_: f32;
    var phi_919_: f32;
    var phi_920_: f32;
    var phi_921_: f32;
    var phi_922_: f32;
    var phi_923_: f32;
    var phi_924_: f32;
    var phi_925_: f32;
    var phi_926_: f32;
    var phi_927_: f32;
    var phi_928_: f32;
    var phi_929_: f32;
    var phi_930_: f32;
    var phi_931_: f32;
    var phi_932_: f32;
    var phi_933_: f32;
    var phi_934_: f32;
    var phi_935_: f32;
    var phi_936_: f32;
    var phi_937_: f32;
    var phi_938_: bool;
    var phi_939_: bool;
    var phi_940_: f32;
    var phi_941_: f32;
    var phi_942_: bool;
    var phi_943_: f32;
    var phi_944_: f32;
    var phi_951_: f32;
    var phi_953_: bool;
    var phi_954_: f32;
    var phi_955_: f32;
    var phi_956_: bool;
    var phi_957_: bool;
    var phi_958_: f32;
    var phi_959_: bool;
    var phi_960_: f32;
    var phi_961_: f32;
    var phi_967_: f32;
    var phi_968_: f32;
    var phi_969_: f32;
    var phi_970_: f32;
    var phi_971_: f32;
    var phi_3141_: f32;
    var phi_3223_: u32;
    var phi_3181_: u32;
    var phi_3284_: u32;
    var phi_3188_: type_13;
    var phi_1066_: bool;
    var phi_1067_: bool;
    var phi_1068_: bool;
    var phi_1069_: bool;
    var phi_1070_: bool;
    var phi_1071_: bool;
    var phi_1072_: bool;
    var phi_1073_: bool;
    var local: bool;
    var local_1: type_15;
    var local_2: f32;
    var local_3: f32;
    var local_4: f32;

    let _e40 = arrayLength((&global.member));
    let _e41 = global_2;
    let _e42 = global_3;
    if (_e41 < _e40) {
        let _e46 = global.member[_e41];
        let _e47 = bitcast<f32>(_e46);
        let _e48 = (_e41 + 1u);
        if (_e48 < _e40) {
            let _e52 = global.member[_e48];
            let _e53 = bitcast<f32>(_e52);
            let _e54 = (_e41 + 2u);
            if (_e54 < _e40) {
                let _e58 = global.member[_e54];
                let _e59 = bitcast<f32>(_e58);
                let _e60 = (_e41 + 3u);
                if (_e60 < _e40) {
                    let _e64 = global.member[_e60];
                    let _e65 = bitcast<f32>(_e64);
                    let _e66 = (_e41 + 4u);
                    if (_e66 < _e40) {
                        let _e70 = global.member[_e66];
                        let _e71 = bitcast<f32>(_e70);
                        if ((_e41 + 5u) < _e40) {
                            let _e74 = (_e41 + 6u);
                            if (_e74 < _e40) {
                                let _e78 = global.member[_e74];
                                let _e80 = (_e41 + 7u);
                                if (_e80 < _e40) {
                                    let _e84 = global.member[_e80];
                                    let _e86 = (_e41 + 8u);
                                    if (_e86 < _e40) {
                                        let _e90 = global.member[_e86];
                                        if ((_e41 + 9u) < _e40) {
                                            let _e94 = (_e41 + 10u);
                                            if (_e94 < _e40) {
                                                let _e98 = global.member[_e94];
                                                let _e99 = (_e41 + 11u);
                                                if (_e99 < _e40) {
                                                    let _e103 = global.member[_e99];
                                                    let _e104 = (_e41 + 12u);
                                                    if (_e104 < _e40) {
                                                        let _e108 = global.member[_e104];
                                                        if (_e98 < _e40) {
                                                            let _e112 = global.member[_e98];
                                                            let _e113 = (_e98 + 1u);
                                                            if (_e113 < _e40) {
                                                                let _e117 = global.member[_e113];
                                                                switch bitcast<i32>(_e112) {
                                                                    case 0: {
                                                                        if (_e117 < _e40) {
                                                                            let _e122 = global.member[_e117];
                                                                            phi_354_ = (sqrt(fma(_e42.z, _e42.z, fma(_e42.x, _e42.x, (_e42.y * _e42.y)))) - bitcast<f32>(_e122));
                                                                        } else {
                                                                            phi_354_ = f32();
                                                                        }
                                                                        let _e133 = phi_354_;
                                                                        phi_971_ = _e133;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        if (_e117 < _e40) {
                                                                            let _e137 = global.member[_e117];
                                                                            let _e139 = (_e117 + 1u);
                                                                            if (_e139 < _e40) {
                                                                                let _e143 = global.member[_e139];
                                                                                let _e145 = (_e117 + 2u);
                                                                                if (_e145 < _e40) {
                                                                                    let _e149 = global.member[_e145];
                                                                                    let _e151 = (_e117 + 3u);
                                                                                    if (_e151 < _e40) {
                                                                                        let _e155 = global.member[_e151];
                                                                                        phi_390_ = (fma(_e42.z, bitcast<f32>(_e149), fma(_e42.x, bitcast<f32>(_e137), (_e42.y * bitcast<f32>(_e143)))) + bitcast<f32>(_e155));
                                                                                    } else {
                                                                                        phi_390_ = f32();
                                                                                    }
                                                                                    let _e165 = phi_390_;
                                                                                    phi_391_ = _e165;
                                                                                } else {
                                                                                    phi_391_ = f32();
                                                                                }
                                                                                let _e167 = phi_391_;
                                                                                phi_392_ = _e167;
                                                                            } else {
                                                                                phi_392_ = f32();
                                                                            }
                                                                            let _e169 = phi_392_;
                                                                            phi_393_ = _e169;
                                                                        } else {
                                                                            phi_393_ = f32();
                                                                        }
                                                                        let _e171 = phi_393_;
                                                                        phi_971_ = _e171;
                                                                        break;
                                                                    }
                                                                    case 2: {
                                                                        if (_e117 < _e40) {
                                                                            let _e175 = global.member[_e117];
                                                                            let _e177 = (_e117 + 1u);
                                                                            if (_e177 < _e40) {
                                                                                let _e181 = global.member[_e177];
                                                                                let _e183 = (_e117 + 2u);
                                                                                if (_e183 < _e40) {
                                                                                    let _e187 = global.member[_e183];
                                                                                    let _e196 = fma(-(bitcast<f32>(_e175)), 0.5f, abs(_e42.x));
                                                                                    let _e198 = fma(-(bitcast<f32>(_e181)), 0.5f, abs(_e42.y));
                                                                                    let _e200 = fma(-(bitcast<f32>(_e187)), 0.5f, abs(_e42.z));
                                                                                    let _e201 = max(_e196, 0f);
                                                                                    let _e202 = max(_e198, 0f);
                                                                                    let _e203 = max(_e200, 0f);
                                                                                    phi_420_ = (sqrt(fma(_e203, _e203, fma(_e201, _e201, (_e202 * _e202)))) + min(max(_e196, max(_e198, _e200)), 0f));
                                                                                } else {
                                                                                    phi_420_ = f32();
                                                                                }
                                                                                let _e213 = phi_420_;
                                                                                phi_421_ = _e213;
                                                                            } else {
                                                                                phi_421_ = f32();
                                                                            }
                                                                            let _e215 = phi_421_;
                                                                            phi_422_ = _e215;
                                                                        } else {
                                                                            phi_422_ = f32();
                                                                        }
                                                                        let _e217 = phi_422_;
                                                                        phi_971_ = _e217;
                                                                        break;
                                                                    }
                                                                    case 4: {
                                                                        if (_e117 < _e40) {
                                                                            let _e221 = global.member[_e117];
                                                                            let _e222 = bitcast<f32>(_e221);
                                                                            let _e223 = (_e117 + 1u);
                                                                            if (_e223 < _e40) {
                                                                                let _e227 = global.member[_e223];
                                                                                let _e228 = bitcast<f32>(_e227);
                                                                                let _e229 = (_e117 + 2u);
                                                                                if (_e229 < _e40) {
                                                                                    let _e233 = global.member[_e229];
                                                                                    let _e234 = bitcast<f32>(_e233);
                                                                                    let _e235 = (_e117 + 3u);
                                                                                    if (_e235 < _e40) {
                                                                                        let _e239 = global.member[_e235];
                                                                                        let _e241 = (_e117 + 4u);
                                                                                        if (_e241 < _e40) {
                                                                                            let _e245 = global.member[_e241];
                                                                                            let _e247 = (_e117 + 5u);
                                                                                            if (_e247 < _e40) {
                                                                                                let _e251 = global.member[_e247];
                                                                                                let _e253 = (_e117 + 6u);
                                                                                                if (_e253 < _e40) {
                                                                                                    let _e257 = global.member[_e253];
                                                                                                    let _e260 = (_e42.x - _e222);
                                                                                                    let _e262 = (_e42.y - _e228);
                                                                                                    let _e264 = (_e42.z - _e234);
                                                                                                    let _e265 = (bitcast<f32>(_e239) - _e222);
                                                                                                    let _e266 = (bitcast<f32>(_e245) - _e228);
                                                                                                    let _e267 = (bitcast<f32>(_e251) - _e234);
                                                                                                    let _e274 = (fma(_e264, _e267, fma(_e260, _e265, (_e262 * _e266))) / fma(_e267, _e267, fma(_e265, _e265, (_e266 * _e266))));
                                                                                                    if (_e274 < 0f) {
                                                                                                        phi_1984_ = 0f;
                                                                                                    } else {
                                                                                                        phi_1984_ = select(_e274, 1f, (_e274 > 1f));
                                                                                                    }
                                                                                                    let _e279 = phi_1984_;
                                                                                                    let _e281 = fma(-(_e265), _e279, _e260);
                                                                                                    let _e283 = fma(-(_e266), _e279, _e262);
                                                                                                    let _e285 = fma(-(_e267), _e279, _e264);
                                                                                                    phi_484_ = fma(-(bitcast<f32>(_e257)), 0.5f, sqrt(fma(_e285, _e285, fma(_e281, _e281, (_e283 * _e283)))));
                                                                                                } else {
                                                                                                    phi_484_ = f32();
                                                                                                }
                                                                                                let _e293 = phi_484_;
                                                                                                phi_485_ = _e293;
                                                                                            } else {
                                                                                                phi_485_ = f32();
                                                                                            }
                                                                                            let _e295 = phi_485_;
                                                                                            phi_486_ = _e295;
                                                                                        } else {
                                                                                            phi_486_ = f32();
                                                                                        }
                                                                                        let _e297 = phi_486_;
                                                                                        phi_487_ = _e297;
                                                                                    } else {
                                                                                        phi_487_ = f32();
                                                                                    }
                                                                                    let _e299 = phi_487_;
                                                                                    phi_488_ = _e299;
                                                                                } else {
                                                                                    phi_488_ = f32();
                                                                                }
                                                                                let _e301 = phi_488_;
                                                                                phi_489_ = _e301;
                                                                            } else {
                                                                                phi_489_ = f32();
                                                                            }
                                                                            let _e303 = phi_489_;
                                                                            phi_490_ = _e303;
                                                                        } else {
                                                                            phi_490_ = f32();
                                                                        }
                                                                        let _e305 = phi_490_;
                                                                        phi_971_ = _e305;
                                                                        break;
                                                                    }
                                                                    case 3: {
                                                                        if (_e117 < _e40) {
                                                                            let _e309 = global.member[_e117];
                                                                            let _e310 = bitcast<f32>(_e309);
                                                                            let _e311 = (_e117 + 1u);
                                                                            if (_e311 < _e40) {
                                                                                let _e315 = global.member[_e311];
                                                                                let _e316 = bitcast<f32>(_e315);
                                                                                let _e317 = (_e117 + 2u);
                                                                                if (_e317 < _e40) {
                                                                                    let _e321 = global.member[_e317];
                                                                                    let _e322 = bitcast<f32>(_e321);
                                                                                    let _e324 = (_e117 + 3u);
                                                                                    if (_e324 < _e40) {
                                                                                        let _e328 = global.member[_e324];
                                                                                        let _e329 = bitcast<f32>(_e328);
                                                                                        let _e330 = (_e117 + 4u);
                                                                                        if (_e330 < _e40) {
                                                                                            let _e334 = global.member[_e330];
                                                                                            let _e335 = bitcast<f32>(_e334);
                                                                                            let _e336 = (_e117 + 5u);
                                                                                            if (_e336 < _e40) {
                                                                                                let _e340 = global.member[_e336];
                                                                                                let _e341 = bitcast<f32>(_e340);
                                                                                                let _e343 = (_e117 + 6u);
                                                                                                if (_e343 < _e40) {
                                                                                                    let _e347 = global.member[_e343];
                                                                                                    let _e349 = (_e117 + 7u);
                                                                                                    if (_e349 < _e40) {
                                                                                                        let _e353 = global.member[_e349];
                                                                                                        let _e355 = (_e117 + 8u);
                                                                                                        if (_e355 < _e40) {
                                                                                                            let _e359 = global.member[_e355];
                                                                                                            let _e361 = (_e117 + 9u);
                                                                                                            if (_e361 < _e40) {
                                                                                                                let _e365 = global.member[_e361];
                                                                                                                let _e367 = (vec3<f32>(_e329, _e335, _e341) - vec3<f32>(_e310, _e316, _e322));
                                                                                                                let _e371 = (fma(-2f, _e329, _e310) + bitcast<f32>(_e347));
                                                                                                                let _e372 = (fma(-2f, _e335, _e316) + bitcast<f32>(_e353));
                                                                                                                let _e373 = (fma(-2f, _e341, _e322) + bitcast<f32>(_e359));
                                                                                                                let _e378 = (_e310 - _e42.x);
                                                                                                                let _e380 = (_e316 - _e42.y);
                                                                                                                let _e382 = (_e322 - _e42.z);
                                                                                                                let _e385 = fma(_e373, _e373, fma(_e371, _e371, (_e372 * _e372)));
                                                                                                                let _e386 = (1f / _e385);
                                                                                                                let _e389 = fma(_e367.z, _e373, fma(_e367.x, _e371, (_e367.y * _e372)));
                                                                                                                let _e390 = (_e386 * _e389);
                                                                                                                let _e398 = (_e386 * fma(2f, fma(_e367.z, _e367.z, fma(_e367.x, _e367.x, (_e367.y * _e367.y))), fma(_e382, _e373, fma(_e378, _e371, (_e380 * _e372)))));
                                                                                                                let _e405 = fma(_e398, 0.33333334f, -((_e390 * _e390)));
                                                                                                                let _e411 = fma(_e390, fma((2f * _e390), _e390, -(_e398)), (_e386 * fma(_e382, _e367.z, fma(_e378, _e367.x, (_e380 * _e367.y)))));
                                                                                                                let _e413 = fma(_e411, _e411, (4f * ((_e405 * _e405) * _e405)));
                                                                                                                if (_e413 >= 0f) {
                                                                                                                    let _e415 = sqrt(_e413);
                                                                                                                    let _e419 = ((_e415 - _e411) * 0.5f);
                                                                                                                    let _e420 = ((-(_e415) - _e411) * 0.5f);
                                                                                                                    let _e442 = fma((-1f / _e385), _e389, fma((f32(select(0u, 1u, (_e419 >= 0f))) - f32(select(0u, 1u, (_e419 < 0f)))), pow(abs(_e419), 0.33333334f), ((f32(select(0u, 1u, (_e420 >= 0f))) - f32(select(0u, 1u, (_e420 < 0f)))) * pow(abs(_e420), 0.33333334f))));
                                                                                                                    if (_e442 < 0f) {
                                                                                                                        phi_2141_ = 0f;
                                                                                                                    } else {
                                                                                                                        phi_2141_ = select(_e442, 1f, (_e442 > 1f));
                                                                                                                    }
                                                                                                                    let _e447 = phi_2141_;
                                                                                                                    let _e454 = fma(fma(_e367.x, 2f, (_e371 * _e447)), _e447, _e378);
                                                                                                                    let _e455 = fma(fma(_e367.y, 2f, (_e372 * _e447)), _e447, _e380);
                                                                                                                    let _e456 = fma(fma(_e367.z, 2f, (_e373 * _e447)), _e447, _e382);
                                                                                                                    phi_2231_ = fma(_e456, _e456, fma(_e454, _e454, (_e455 * _e455)));
                                                                                                                } else {
                                                                                                                    let _e461 = sqrt(-(_e405));
                                                                                                                    let _e466 = (acos((_e411 / ((_e405 * _e461) * 2f))) * 0.33333334f);
                                                                                                                    let _e467 = cos(_e466);
                                                                                                                    let _e472 = -(_e390);
                                                                                                                    let _e477 = min(max(fma((_e467 + _e467), _e461, _e472), 0f), 1f);
                                                                                                                    let _e478 = min(max(fma(fma(sin(_e466), -1.7320508f, -(_e467)), _e461, _e472), 0f), 1f);
                                                                                                                    let _e485 = fma(fma(_e367.x, 2f, (_e371 * _e477)), _e477, _e378);
                                                                                                                    let _e486 = fma(fma(_e367.y, 2f, (_e372 * _e477)), _e477, _e380);
                                                                                                                    let _e487 = fma(fma(_e367.z, 2f, (_e373 * _e477)), _e477, _e382);
                                                                                                                    let _e497 = fma(fma(_e367.x, 2f, (_e371 * _e478)), _e478, _e378);
                                                                                                                    let _e498 = fma(fma(_e367.y, 2f, (_e372 * _e478)), _e478, _e380);
                                                                                                                    let _e499 = fma(fma(_e367.z, 2f, (_e373 * _e478)), _e478, _e382);
                                                                                                                    phi_2231_ = min(fma(_e487, _e487, fma(_e485, _e485, (_e486 * _e486))), fma(_e499, _e499, fma(_e497, _e497, (_e498 * _e498))));
                                                                                                                }
                                                                                                                let _e505 = phi_2231_;
                                                                                                                phi_578_ = fma(-(bitcast<f32>(_e365)), 0.5f, sqrt(_e505));
                                                                                                            } else {
                                                                                                                phi_578_ = f32();
                                                                                                            }
                                                                                                            let _e510 = phi_578_;
                                                                                                            phi_579_ = _e510;
                                                                                                        } else {
                                                                                                            phi_579_ = f32();
                                                                                                        }
                                                                                                        let _e512 = phi_579_;
                                                                                                        phi_580_ = _e512;
                                                                                                    } else {
                                                                                                        phi_580_ = f32();
                                                                                                    }
                                                                                                    let _e514 = phi_580_;
                                                                                                    phi_581_ = _e514;
                                                                                                } else {
                                                                                                    phi_581_ = f32();
                                                                                                }
                                                                                                let _e516 = phi_581_;
                                                                                                phi_582_ = _e516;
                                                                                            } else {
                                                                                                phi_582_ = f32();
                                                                                            }
                                                                                            let _e518 = phi_582_;
                                                                                            phi_583_ = _e518;
                                                                                        } else {
                                                                                            phi_583_ = f32();
                                                                                        }
                                                                                        let _e520 = phi_583_;
                                                                                        phi_584_ = _e520;
                                                                                    } else {
                                                                                        phi_584_ = f32();
                                                                                    }
                                                                                    let _e522 = phi_584_;
                                                                                    phi_585_ = _e522;
                                                                                } else {
                                                                                    phi_585_ = f32();
                                                                                }
                                                                                let _e524 = phi_585_;
                                                                                phi_586_ = _e524;
                                                                            } else {
                                                                                phi_586_ = f32();
                                                                            }
                                                                            let _e526 = phi_586_;
                                                                            phi_587_ = _e526;
                                                                        } else {
                                                                            phi_587_ = f32();
                                                                        }
                                                                        let _e528 = phi_587_;
                                                                        phi_971_ = _e528;
                                                                        break;
                                                                    }
                                                                    case 5: {
                                                                        if (_e117 < _e40) {
                                                                            let _e532 = global.member[_e117];
                                                                            let _e533 = (_e117 + 1u);
                                                                            if (_e533 < _e40) {
                                                                                let _e537 = global.member[_e533];
                                                                                let _e538 = (_e117 + 2u);
                                                                                if (_e538 < _e40) {
                                                                                    let _e542 = global.member[_e538];
                                                                                    let _e544 = (_e117 + 3u);
                                                                                    if (_e544 < _e40) {
                                                                                        let _e548 = global.member[_e544];
                                                                                        let _e549 = (_e548 == 1u);
                                                                                        let _e551 = dpdx(_e42.x);
                                                                                        let _e554 = dpdy(_e42.y);
                                                                                        phi_3219_ = 0u;
                                                                                        phi_640_ = 1f;
                                                                                        phi_643_ = 340282350000000000000000000000000000000f;
                                                                                        loop {
                                                                                            let _e557 = phi_3219_;
                                                                                            let _e559 = phi_640_;
                                                                                            let _e561 = phi_643_;
                                                                                            let _e562 = (_e557 >= _e537);
                                                                                            if _e562 {
                                                                                                phi_3227_ = _e557;
                                                                                                phi_2334_ = type_13(0u, type_13().member_1);
                                                                                            } else {
                                                                                                if _e562 {
                                                                                                    phi_2327_ = 4294967295u;
                                                                                                } else {
                                                                                                    phi_2327_ = (_e532 + (2u * _e557));
                                                                                                }
                                                                                                let _e569 = phi_2327_;
                                                                                                phi_3227_ = (_e557 + 1u);
                                                                                                phi_2334_ = type_13(1u, _e569);
                                                                                            }
                                                                                            let _e573 = phi_3227_;
                                                                                            let _e575 = phi_2334_;
                                                                                            switch bitcast<i32>(_e575.member) {
                                                                                                case 0: {
                                                                                                    phi_957_ = false;
                                                                                                    phi_958_ = fma(_e559, _e561, (bitcast<f32>(_e542) * -0.5f));
                                                                                                    phi_959_ = false;
                                                                                                    phi_960_ = f32();
                                                                                                    phi_961_ = f32();
                                                                                                    break;
                                                                                                }
                                                                                                case 1: {
                                                                                                    if (_e575.member_1 < _e40) {
                                                                                                        let _e584 = global.member[_e575.member_1];
                                                                                                        let _e585 = (_e575.member_1 + 1u);
                                                                                                        switch bitcast<i32>(_e584) {
                                                                                                            case 0: {
                                                                                                                phi_696_ = type_13(0u, type_13().member_1);
                                                                                                                break;
                                                                                                            }
                                                                                                            case 1: {
                                                                                                                if (_e585 < _e40) {
                                                                                                                    let _e595 = global.member[_e585];
                                                                                                                    phi_684_ = type_13(1u, _e595);
                                                                                                                } else {
                                                                                                                    phi_684_ = type_13();
                                                                                                                }
                                                                                                                let _e598 = phi_684_;
                                                                                                                phi_696_ = _e598;
                                                                                                                break;
                                                                                                            }
                                                                                                            case 2: {
                                                                                                                if (_e585 < _e40) {
                                                                                                                    let _e602 = global.member[_e585];
                                                                                                                    phi_695_ = type_13(2u, _e602);
                                                                                                                } else {
                                                                                                                    phi_695_ = type_13();
                                                                                                                }
                                                                                                                let _e605 = phi_695_;
                                                                                                                phi_696_ = _e605;
                                                                                                                break;
                                                                                                            }
                                                                                                            default: {
                                                                                                                phi_696_ = type_13(0u, type_13().member_1);
                                                                                                                break;
                                                                                                            }
                                                                                                        }
                                                                                                        let _e607 = phi_696_;
                                                                                                        switch bitcast<i32>(_e607.member) {
                                                                                                            case 0: {
                                                                                                                phi_938_ = false;
                                                                                                                phi_939_ = true;
                                                                                                                phi_940_ = _e559;
                                                                                                                phi_941_ = _e561;
                                                                                                                phi_942_ = false;
                                                                                                                phi_943_ = f32();
                                                                                                                phi_944_ = f32();
                                                                                                                break;
                                                                                                            }
                                                                                                            case 1: {
                                                                                                                if (_e607.member_1 < _e40) {
                                                                                                                    let _e614 = global.member[_e607.member_1];
                                                                                                                    let _e615 = bitcast<f32>(_e614);
                                                                                                                    let _e616 = (_e607.member_1 + 1u);
                                                                                                                    if (_e616 < _e40) {
                                                                                                                        let _e620 = global.member[_e616];
                                                                                                                        let _e621 = bitcast<f32>(_e620);
                                                                                                                        let _e622 = (_e607.member_1 + 2u);
                                                                                                                        if (_e622 < _e40) {
                                                                                                                            let _e626 = global.member[_e622];
                                                                                                                            let _e627 = bitcast<f32>(_e626);
                                                                                                                            let _e628 = (_e607.member_1 + 3u);
                                                                                                                            if (_e628 < _e40) {
                                                                                                                                let _e632 = global.member[_e628];
                                                                                                                                let _e634 = (_e607.member_1 + 4u);
                                                                                                                                if (_e634 < _e40) {
                                                                                                                                    let _e638 = global.member[_e634];
                                                                                                                                    let _e639 = bitcast<f32>(_e638);
                                                                                                                                    let _e640 = (_e607.member_1 + 5u);
                                                                                                                                    if (_e640 < _e40) {
                                                                                                                                        let _e644 = global.member[_e640];
                                                                                                                                        let _e646 = (_e607.member_1 + 6u);
                                                                                                                                        if (_e646 < _e40) {
                                                                                                                                            let _e650 = global.member[_e646];
                                                                                                                                            let _e652 = (_e42.x - _e615);
                                                                                                                                            let _e653 = (_e42.y - _e621);
                                                                                                                                            let _e655 = (_e42.z - _e627);
                                                                                                                                            let _e656 = (bitcast<f32>(_e632) - _e615);
                                                                                                                                            let _e657 = (_e639 - _e621);
                                                                                                                                            let _e658 = (bitcast<f32>(_e644) - _e627);
                                                                                                                                            let _e665 = (fma(_e655, _e658, fma(_e652, _e656, (_e653 * _e657))) / fma(_e658, _e658, fma(_e656, _e656, (_e657 * _e657))));
                                                                                                                                            if (_e665 < 0f) {
                                                                                                                                                phi_2444_ = 0f;
                                                                                                                                            } else {
                                                                                                                                                phi_2444_ = select(_e665, 1f, (_e665 > 1f));
                                                                                                                                            }
                                                                                                                                            let _e670 = phi_2444_;
                                                                                                                                            let _e672 = fma(-(_e656), _e670, _e652);
                                                                                                                                            let _e674 = fma(-(_e657), _e670, _e653);
                                                                                                                                            let _e676 = fma(-(_e658), _e670, _e655);
                                                                                                                                            if _e549 {
                                                                                                                                                let _e683 = (_e42.y >= _e621);
                                                                                                                                                let _e684 = (_e42.y < _e639);
                                                                                                                                                let _e687 = ((_e656 * _e653) > (_e657 * _e652));
                                                                                                                                                if select(false, _e687, select(false, _e684, _e683)) {
                                                                                                                                                    phi_2511_ = true;
                                                                                                                                                } else {
                                                                                                                                                    phi_2511_ = (select(_e687, true, select(_e684, true, _e683)) != true);
                                                                                                                                                }
                                                                                                                                                let _e694 = phi_2511_;
                                                                                                                                                phi_771_ = (_e559 * select(1f, -1f, _e694));
                                                                                                                                            } else {
                                                                                                                                                phi_771_ = _e559;
                                                                                                                                            }
                                                                                                                                            let _e698 = phi_771_;
                                                                                                                                            phi_772_ = fma(-(bitcast<f32>(_e650)), 0.5f, sqrt(fma(_e676, _e676, fma(_e672, _e672, (_e674 * _e674)))));
                                                                                                                                            phi_773_ = _e698;
                                                                                                                                        } else {
                                                                                                                                            phi_772_ = f32();
                                                                                                                                            phi_773_ = f32();
                                                                                                                                        }
                                                                                                                                        let _e700 = phi_772_;
                                                                                                                                        let _e702 = phi_773_;
                                                                                                                                        phi_774_ = _e700;
                                                                                                                                        phi_775_ = _e702;
                                                                                                                                    } else {
                                                                                                                                        phi_774_ = f32();
                                                                                                                                        phi_775_ = f32();
                                                                                                                                    }
                                                                                                                                    let _e704 = phi_774_;
                                                                                                                                    let _e706 = phi_775_;
                                                                                                                                    phi_776_ = _e704;
                                                                                                                                    phi_777_ = _e706;
                                                                                                                                } else {
                                                                                                                                    phi_776_ = f32();
                                                                                                                                    phi_777_ = f32();
                                                                                                                                }
                                                                                                                                let _e708 = phi_776_;
                                                                                                                                let _e710 = phi_777_;
                                                                                                                                phi_778_ = _e708;
                                                                                                                                phi_779_ = _e710;
                                                                                                                            } else {
                                                                                                                                phi_778_ = f32();
                                                                                                                                phi_779_ = f32();
                                                                                                                            }
                                                                                                                            let _e712 = phi_778_;
                                                                                                                            let _e714 = phi_779_;
                                                                                                                            phi_780_ = _e712;
                                                                                                                            phi_781_ = _e714;
                                                                                                                        } else {
                                                                                                                            phi_780_ = f32();
                                                                                                                            phi_781_ = f32();
                                                                                                                        }
                                                                                                                        let _e716 = phi_780_;
                                                                                                                        let _e718 = phi_781_;
                                                                                                                        phi_782_ = _e716;
                                                                                                                        phi_783_ = _e718;
                                                                                                                    } else {
                                                                                                                        phi_782_ = f32();
                                                                                                                        phi_783_ = f32();
                                                                                                                    }
                                                                                                                    let _e720 = phi_782_;
                                                                                                                    let _e722 = phi_783_;
                                                                                                                    phi_784_ = _e720;
                                                                                                                    phi_785_ = _e722;
                                                                                                                } else {
                                                                                                                    phi_784_ = f32();
                                                                                                                    phi_785_ = f32();
                                                                                                                }
                                                                                                                let _e724 = phi_784_;
                                                                                                                let _e726 = phi_785_;
                                                                                                                phi_938_ = false;
                                                                                                                phi_939_ = false;
                                                                                                                phi_940_ = f32();
                                                                                                                phi_941_ = f32();
                                                                                                                phi_942_ = true;
                                                                                                                phi_943_ = _e724;
                                                                                                                phi_944_ = _e726;
                                                                                                                break;
                                                                                                            }
                                                                                                            case 2: {
                                                                                                                if (_e607.member_1 < _e40) {
                                                                                                                    let _e730 = global.member[_e607.member_1];
                                                                                                                    let _e731 = bitcast<f32>(_e730);
                                                                                                                    let _e732 = (_e607.member_1 + 1u);
                                                                                                                    if (_e732 < _e40) {
                                                                                                                        let _e736 = global.member[_e732];
                                                                                                                        let _e737 = bitcast<f32>(_e736);
                                                                                                                        let _e738 = (_e607.member_1 + 2u);
                                                                                                                        if (_e738 < _e40) {
                                                                                                                            let _e742 = global.member[_e738];
                                                                                                                            let _e743 = bitcast<f32>(_e742);
                                                                                                                            let _e744 = vec3<f32>(_e731, _e737, _e743);
                                                                                                                            let _e745 = (_e607.member_1 + 3u);
                                                                                                                            if (_e745 < _e40) {
                                                                                                                                let _e749 = global.member[_e745];
                                                                                                                                let _e750 = bitcast<f32>(_e749);
                                                                                                                                let _e751 = (_e607.member_1 + 4u);
                                                                                                                                if (_e751 < _e40) {
                                                                                                                                    let _e755 = global.member[_e751];
                                                                                                                                    let _e756 = bitcast<f32>(_e755);
                                                                                                                                    let _e757 = (_e607.member_1 + 5u);
                                                                                                                                    if (_e757 < _e40) {
                                                                                                                                        let _e761 = global.member[_e757];
                                                                                                                                        let _e762 = bitcast<f32>(_e761);
                                                                                                                                        let _e763 = vec3<f32>(_e750, _e756, _e762);
                                                                                                                                        let _e764 = (_e607.member_1 + 6u);
                                                                                                                                        if (_e764 < _e40) {
                                                                                                                                            let _e768 = global.member[_e764];
                                                                                                                                            let _e769 = bitcast<f32>(_e768);
                                                                                                                                            let _e770 = (_e607.member_1 + 7u);
                                                                                                                                            if (_e770 < _e40) {
                                                                                                                                                let _e774 = global.member[_e770];
                                                                                                                                                let _e775 = bitcast<f32>(_e774);
                                                                                                                                                let _e776 = (_e607.member_1 + 8u);
                                                                                                                                                if (_e776 < _e40) {
                                                                                                                                                    let _e780 = global.member[_e776];
                                                                                                                                                    let _e781 = bitcast<f32>(_e780);
                                                                                                                                                    let _e783 = (_e607.member_1 + 9u);
                                                                                                                                                    if (_e783 < _e40) {
                                                                                                                                                        let _e787 = global.member[_e783];
                                                                                                                                                        let _e789 = (_e763 - _e744);
                                                                                                                                                        let _e793 = (fma(-2f, _e750, _e731) + _e769);
                                                                                                                                                        let _e794 = (fma(-2f, _e756, _e737) + _e775);
                                                                                                                                                        let _e795 = (fma(-2f, _e762, _e743) + _e781);
                                                                                                                                                        let _e799 = (_e731 - _e42.x);
                                                                                                                                                        let _e800 = (_e737 - _e42.y);
                                                                                                                                                        let _e802 = (_e743 - _e42.z);
                                                                                                                                                        let _e805 = fma(_e795, _e795, fma(_e793, _e793, (_e794 * _e794)));
                                                                                                                                                        let _e806 = (1f / _e805);
                                                                                                                                                        let _e809 = fma(_e789.z, _e795, fma(_e789.x, _e793, (_e789.y * _e794)));
                                                                                                                                                        let _e810 = (_e806 * _e809);
                                                                                                                                                        let _e818 = (_e806 * fma(2f, fma(_e789.z, _e789.z, fma(_e789.x, _e789.x, (_e789.y * _e789.y))), fma(_e802, _e795, fma(_e799, _e793, (_e800 * _e794)))));
                                                                                                                                                        let _e825 = fma(_e818, 0.33333334f, -((_e810 * _e810)));
                                                                                                                                                        let _e831 = fma(_e810, fma((2f * _e810), _e810, -(_e818)), (_e806 * fma(_e802, _e789.z, fma(_e799, _e789.x, (_e800 * _e789.y)))));
                                                                                                                                                        let _e833 = fma(_e831, _e831, (4f * ((_e825 * _e825) * _e825)));
                                                                                                                                                        if (_e833 >= 0f) {
                                                                                                                                                            let _e835 = sqrt(_e833);
                                                                                                                                                            let _e839 = ((_e835 - _e831) * 0.5f);
                                                                                                                                                            let _e840 = ((-(_e835) - _e831) * 0.5f);
                                                                                                                                                            let _e862 = fma((-1f / _e805), _e809, fma((f32(select(0u, 1u, (_e839 >= 0f))) - f32(select(0u, 1u, (_e839 < 0f)))), pow(abs(_e839), 0.33333334f), ((f32(select(0u, 1u, (_e840 >= 0f))) - f32(select(0u, 1u, (_e840 < 0f)))) * pow(abs(_e840), 0.33333334f))));
                                                                                                                                                            if (_e862 < 0f) {
                                                                                                                                                                phi_2649_ = 0f;
                                                                                                                                                            } else {
                                                                                                                                                                phi_2649_ = select(_e862, 1f, (_e862 > 1f));
                                                                                                                                                            }
                                                                                                                                                            let _e867 = phi_2649_;
                                                                                                                                                            let _e874 = fma(fma(_e789.x, 2f, (_e793 * _e867)), _e867, _e799);
                                                                                                                                                            let _e875 = fma(fma(_e789.y, 2f, (_e794 * _e867)), _e867, _e800);
                                                                                                                                                            let _e876 = fma(fma(_e789.z, 2f, (_e795 * _e867)), _e867, _e802);
                                                                                                                                                            phi_2739_ = fma(_e876, _e876, fma(_e874, _e874, (_e875 * _e875)));
                                                                                                                                                        } else {
                                                                                                                                                            let _e881 = sqrt(-(_e825));
                                                                                                                                                            let _e886 = (acos((_e831 / ((_e825 * _e881) * 2f))) * 0.33333334f);
                                                                                                                                                            let _e887 = cos(_e886);
                                                                                                                                                            let _e892 = -(_e810);
                                                                                                                                                            let _e897 = min(max(fma((_e887 + _e887), _e881, _e892), 0f), 1f);
                                                                                                                                                            let _e898 = min(max(fma(fma(sin(_e886), -1.7320508f, -(_e887)), _e881, _e892), 0f), 1f);
                                                                                                                                                            let _e905 = fma(fma(_e789.x, 2f, (_e793 * _e897)), _e897, _e799);
                                                                                                                                                            let _e906 = fma(fma(_e789.y, 2f, (_e794 * _e897)), _e897, _e800);
                                                                                                                                                            let _e907 = fma(fma(_e789.z, 2f, (_e795 * _e897)), _e897, _e802);
                                                                                                                                                            let _e917 = fma(fma(_e789.x, 2f, (_e793 * _e898)), _e898, _e799);
                                                                                                                                                            let _e918 = fma(fma(_e789.y, 2f, (_e794 * _e898)), _e898, _e800);
                                                                                                                                                            let _e919 = fma(fma(_e789.z, 2f, (_e795 * _e898)), _e898, _e802);
                                                                                                                                                            phi_2739_ = min(fma(_e907, _e907, fma(_e905, _e905, (_e906 * _e906))), fma(_e919, _e919, fma(_e917, _e917, (_e918 * _e918))));
                                                                                                                                                        }
                                                                                                                                                        let _e925 = phi_2739_;
                                                                                                                                                        if _e549 {
                                                                                                                                                            phi_878_ = type_15(_e744, _e763, vec3<f32>(_e769, _e775, _e781));
                                                                                                                                                            phi_881_ = _e559;
                                                                                                                                                            loop {
                                                                                                                                                                let _e931 = phi_878_;
                                                                                                                                                                let _e933 = phi_881_;
                                                                                                                                                                let _e939 = (_e931.member_1.x - _e931.member.x);
                                                                                                                                                                let _e942 = (_e931.member_1.y - _e931.member.y);
                                                                                                                                                                let _e945 = (_e931.member_1.z - _e931.member.z);
                                                                                                                                                                let _e947 = (_e931.member_2.x - _e931.member.x);
                                                                                                                                                                let _e949 = (_e931.member_2.y - _e931.member.y);
                                                                                                                                                                let _e951 = (_e931.member_2.z - _e931.member.z);
                                                                                                                                                                let _e954 = fma(_e942, _e951, -((_e949 * _e945)));
                                                                                                                                                                let _e957 = fma(_e945, _e947, -((_e951 * _e939)));
                                                                                                                                                                let _e960 = fma(_e939, _e949, -((_e947 * _e942)));
                                                                                                                                                                let _e965 = (sqrt(fma(_e960, _e960, fma(_e954, _e954, (_e957 * _e957)))) * 0.5f);
                                                                                                                                                                let _e966 = (abs(_e551) * abs(_e554));
                                                                                                                                                                let _e967 = (_e931.member.x - _e931.member_1.x);
                                                                                                                                                                let _e968 = (_e931.member.y - _e931.member_1.y);
                                                                                                                                                                let _e969 = (_e931.member.z - _e931.member_1.z);
                                                                                                                                                                if (_e965 <= (sqrt(fma(_e969, _e969, fma(_e967, _e967, (_e968 * _e968)))) * _e966)) {
                                                                                                                                                                    phi_2816_ = true;
                                                                                                                                                                } else {
                                                                                                                                                                    phi_2816_ = (_e965 <= 0.00000011920929f);
                                                                                                                                                                }
                                                                                                                                                                let _e978 = phi_2816_;
                                                                                                                                                                if _e978 {
                                                                                                                                                                    let _e981 = (_e42.y >= _e931.member.y);
                                                                                                                                                                    let _e982 = (_e42.y < _e931.member_2.y);
                                                                                                                                                                    let _e985 = ((_e947 * (_e42.y - _e931.member.y)) > (_e949 * (_e42.x - _e931.member.x)));
                                                                                                                                                                    if select(false, _e985, select(false, _e982, _e981)) {
                                                                                                                                                                        phi_2896_ = true;
                                                                                                                                                                    } else {
                                                                                                                                                                        phi_2896_ = (select(_e985, true, select(_e982, true, _e981)) != true);
                                                                                                                                                                    }
                                                                                                                                                                    let _e992 = phi_2896_;
                                                                                                                                                                    phi_915_ = (_e933 * select(1f, -1f, _e992));
                                                                                                                                                                    phi_916_ = false;
                                                                                                                                                                    phi_879_ = type_15();
                                                                                                                                                                    phi_882_ = f32();
                                                                                                                                                                } else {
                                                                                                                                                                    phi_894_ = 1f;
                                                                                                                                                                    loop {
                                                                                                                                                                        let _e996 = phi_894_;
                                                                                                                                                                        let _e997 = (_e996 * 0.5f);
                                                                                                                                                                        let _e1001 = fma(_e939, _e997, _e931.member.x);
                                                                                                                                                                        let _e1002 = fma(_e942, _e997, _e931.member.y);
                                                                                                                                                                        let _e1003 = fma(_e945, _e997, _e931.member.z);
                                                                                                                                                                        let _e1005 = (_e931.member + vec3<f32>((_e939 * _e997), (_e942 * _e997), (_e945 * _e997)));
                                                                                                                                                                        let _e1006 = (_e931.member_2.x - _e931.member_1.x);
                                                                                                                                                                        let _e1007 = (_e931.member_2.y - _e931.member_1.y);
                                                                                                                                                                        let _e1008 = (_e931.member_2.z - _e931.member_1.z);
                                                                                                                                                                        let _e1020 = fma((fma(_e1006, _e997, _e931.member_1.x) - _e1001), _e997, _e1001);
                                                                                                                                                                        let _e1021 = fma((fma(_e1007, _e997, _e931.member_1.y) - _e1002), _e997, _e1002);
                                                                                                                                                                        let _e1022 = fma((fma(_e1008, _e997, _e931.member_1.z) - _e1003), _e997, _e1003);
                                                                                                                                                                        let _e1026 = (_e1005.x - _e931.member.x);
                                                                                                                                                                        let _e1028 = (_e1005.y - _e931.member.y);
                                                                                                                                                                        let _e1030 = (_e1005.z - _e931.member.z);
                                                                                                                                                                        let _e1031 = (_e1020 - _e931.member.x);
                                                                                                                                                                        let _e1032 = (_e1021 - _e931.member.y);
                                                                                                                                                                        let _e1033 = (_e1022 - _e931.member.z);
                                                                                                                                                                        let _e1036 = fma(_e1028, _e1033, -((_e1032 * _e1030)));
                                                                                                                                                                        let _e1039 = fma(_e1030, _e1031, -((_e1033 * _e1026)));
                                                                                                                                                                        let _e1042 = fma(_e1026, _e1032, -((_e1031 * _e1028)));
                                                                                                                                                                        let _e1047 = (sqrt(fma(_e1042, _e1042, fma(_e1036, _e1036, (_e1039 * _e1039)))) * 0.5f);
                                                                                                                                                                        let _e1048 = (_e931.member.x - _e1005.x);
                                                                                                                                                                        let _e1049 = (_e931.member.y - _e1005.y);
                                                                                                                                                                        let _e1050 = (_e931.member.z - _e1005.z);
                                                                                                                                                                        if (_e1047 <= (sqrt(fma(_e1050, _e1050, fma(_e1048, _e1048, (_e1049 * _e1049)))) * _e966)) {
                                                                                                                                                                            phi_2990_ = true;
                                                                                                                                                                        } else {
                                                                                                                                                                            phi_2990_ = (_e1047 <= 0.00000011920929f);
                                                                                                                                                                        }
                                                                                                                                                                        let _e1059 = phi_2990_;
                                                                                                                                                                        if _e1059 {
                                                                                                                                                                            let _e1062 = (_e42.y >= _e931.member.y);
                                                                                                                                                                            let _e1063 = (_e42.y < _e1021);
                                                                                                                                                                            let _e1066 = ((_e1031 * (_e42.y - _e931.member.y)) > (_e1032 * (_e42.x - _e931.member.x)));
                                                                                                                                                                            if select(false, _e1066, select(false, _e1063, _e1062)) {
                                                                                                                                                                                phi_3070_ = true;
                                                                                                                                                                            } else {
                                                                                                                                                                                phi_3070_ = (select(_e1066, true, select(_e1063, true, _e1062)) != true);
                                                                                                                                                                            }
                                                                                                                                                                            let _e1073 = phi_3070_;
                                                                                                                                                                            phi_912_ = type_15(vec3<f32>(_e1020, _e1021, _e1022), (_e931.member_1 + vec3<f32>((_e1006 * _e997), (_e1007 * _e997), (_e1008 * _e997))), _e931.member_2);
                                                                                                                                                                            phi_913_ = (_e933 * select(1f, -1f, _e1073));
                                                                                                                                                                        } else {
                                                                                                                                                                            phi_912_ = type_15();
                                                                                                                                                                            phi_913_ = f32();
                                                                                                                                                                        }
                                                                                                                                                                        let _e1077 = phi_912_;
                                                                                                                                                                        let _e1079 = phi_913_;
                                                                                                                                                                        local = select(false, true, _e1059);
                                                                                                                                                                        local_1 = _e1077;
                                                                                                                                                                        local_2 = _e1079;
                                                                                                                                                                        continue;
                                                                                                                                                                        continuing {
                                                                                                                                                                            phi_894_ = select(_e997, f32(), _e1059);
                                                                                                                                                                            break if !(select(true, false, _e1059));
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                    phi_915_ = f32();
                                                                                                                                                                    let _e1383 = local;
                                                                                                                                                                    phi_916_ = _e1383;
                                                                                                                                                                    let _e1386 = local_1;
                                                                                                                                                                    phi_879_ = _e1386;
                                                                                                                                                                    let _e1389 = local_2;
                                                                                                                                                                    phi_882_ = _e1389;
                                                                                                                                                                }
                                                                                                                                                                let _e1085 = phi_915_;
                                                                                                                                                                let _e1087 = phi_916_;
                                                                                                                                                                let _e1089 = phi_879_;
                                                                                                                                                                let _e1091 = phi_882_;
                                                                                                                                                                local_3 = _e1085;
                                                                                                                                                                continue;
                                                                                                                                                                continuing {
                                                                                                                                                                    phi_878_ = _e1089;
                                                                                                                                                                    phi_881_ = _e1091;
                                                                                                                                                                    break if !(_e1087);
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                            let _e1392 = local_3;
                                                                                                                                                            phi_917_ = _e1392;
                                                                                                                                                        } else {
                                                                                                                                                            phi_917_ = _e559;
                                                                                                                                                        }
                                                                                                                                                        let _e1094 = phi_917_;
                                                                                                                                                        phi_918_ = fma(-(bitcast<f32>(_e787)), 0.5f, sqrt(_e925));
                                                                                                                                                        phi_919_ = _e1094;
                                                                                                                                                    } else {
                                                                                                                                                        phi_918_ = f32();
                                                                                                                                                        phi_919_ = f32();
                                                                                                                                                    }
                                                                                                                                                    let _e1096 = phi_918_;
                                                                                                                                                    let _e1098 = phi_919_;
                                                                                                                                                    phi_920_ = _e1096;
                                                                                                                                                    phi_921_ = _e1098;
                                                                                                                                                } else {
                                                                                                                                                    phi_920_ = f32();
                                                                                                                                                    phi_921_ = f32();
                                                                                                                                                }
                                                                                                                                                let _e1100 = phi_920_;
                                                                                                                                                let _e1102 = phi_921_;
                                                                                                                                                phi_922_ = _e1100;
                                                                                                                                                phi_923_ = _e1102;
                                                                                                                                            } else {
                                                                                                                                                phi_922_ = f32();
                                                                                                                                                phi_923_ = f32();
                                                                                                                                            }
                                                                                                                                            let _e1104 = phi_922_;
                                                                                                                                            let _e1106 = phi_923_;
                                                                                                                                            phi_924_ = _e1104;
                                                                                                                                            phi_925_ = _e1106;
                                                                                                                                        } else {
                                                                                                                                            phi_924_ = f32();
                                                                                                                                            phi_925_ = f32();
                                                                                                                                        }
                                                                                                                                        let _e1108 = phi_924_;
                                                                                                                                        let _e1110 = phi_925_;
                                                                                                                                        phi_926_ = _e1108;
                                                                                                                                        phi_927_ = _e1110;
                                                                                                                                    } else {
                                                                                                                                        phi_926_ = f32();
                                                                                                                                        phi_927_ = f32();
                                                                                                                                    }
                                                                                                                                    let _e1112 = phi_926_;
                                                                                                                                    let _e1114 = phi_927_;
                                                                                                                                    phi_928_ = _e1112;
                                                                                                                                    phi_929_ = _e1114;
                                                                                                                                } else {
                                                                                                                                    phi_928_ = f32();
                                                                                                                                    phi_929_ = f32();
                                                                                                                                }
                                                                                                                                let _e1116 = phi_928_;
                                                                                                                                let _e1118 = phi_929_;
                                                                                                                                phi_930_ = _e1116;
                                                                                                                                phi_931_ = _e1118;
                                                                                                                            } else {
                                                                                                                                phi_930_ = f32();
                                                                                                                                phi_931_ = f32();
                                                                                                                            }
                                                                                                                            let _e1120 = phi_930_;
                                                                                                                            let _e1122 = phi_931_;
                                                                                                                            phi_932_ = _e1120;
                                                                                                                            phi_933_ = _e1122;
                                                                                                                        } else {
                                                                                                                            phi_932_ = f32();
                                                                                                                            phi_933_ = f32();
                                                                                                                        }
                                                                                                                        let _e1124 = phi_932_;
                                                                                                                        let _e1126 = phi_933_;
                                                                                                                        phi_934_ = _e1124;
                                                                                                                        phi_935_ = _e1126;
                                                                                                                    } else {
                                                                                                                        phi_934_ = f32();
                                                                                                                        phi_935_ = f32();
                                                                                                                    }
                                                                                                                    let _e1128 = phi_934_;
                                                                                                                    let _e1130 = phi_935_;
                                                                                                                    phi_936_ = _e1128;
                                                                                                                    phi_937_ = _e1130;
                                                                                                                } else {
                                                                                                                    phi_936_ = f32();
                                                                                                                    phi_937_ = f32();
                                                                                                                }
                                                                                                                let _e1132 = phi_936_;
                                                                                                                let _e1134 = phi_937_;
                                                                                                                phi_938_ = false;
                                                                                                                phi_939_ = false;
                                                                                                                phi_940_ = f32();
                                                                                                                phi_941_ = f32();
                                                                                                                phi_942_ = true;
                                                                                                                phi_943_ = _e1132;
                                                                                                                phi_944_ = _e1134;
                                                                                                                break;
                                                                                                            }
                                                                                                            default: {
                                                                                                                phi_938_ = true;
                                                                                                                phi_939_ = false;
                                                                                                                phi_940_ = f32();
                                                                                                                phi_941_ = f32();
                                                                                                                phi_942_ = false;
                                                                                                                phi_943_ = f32();
                                                                                                                phi_944_ = f32();
                                                                                                                break;
                                                                                                            }
                                                                                                        }
                                                                                                        let _e1136 = phi_938_;
                                                                                                        let _e1138 = phi_939_;
                                                                                                        let _e1140 = phi_940_;
                                                                                                        let _e1142 = phi_941_;
                                                                                                        let _e1144 = phi_942_;
                                                                                                        let _e1146 = phi_943_;
                                                                                                        let _e1148 = phi_944_;
                                                                                                        if _e1144 {
                                                                                                            phi_951_ = min(_e561, _e1146);
                                                                                                        } else {
                                                                                                            phi_951_ = _e1142;
                                                                                                        }
                                                                                                        let _e1151 = phi_951_;
                                                                                                        phi_953_ = select(_e1138, true, _e1144);
                                                                                                        phi_954_ = select(_e1140, _e1148, _e1144);
                                                                                                        phi_955_ = _e1151;
                                                                                                        phi_956_ = select(_e1136, false, _e1144);
                                                                                                    } else {
                                                                                                        phi_953_ = false;
                                                                                                        phi_954_ = f32();
                                                                                                        phi_955_ = f32();
                                                                                                        phi_956_ = false;
                                                                                                    }
                                                                                                    let _e1156 = phi_953_;
                                                                                                    let _e1158 = phi_954_;
                                                                                                    let _e1160 = phi_955_;
                                                                                                    let _e1162 = phi_956_;
                                                                                                    phi_957_ = _e1162;
                                                                                                    phi_958_ = f32();
                                                                                                    phi_959_ = _e1156;
                                                                                                    phi_960_ = _e1158;
                                                                                                    phi_961_ = _e1160;
                                                                                                    break;
                                                                                                }
                                                                                                default: {
                                                                                                    phi_957_ = true;
                                                                                                    phi_958_ = f32();
                                                                                                    phi_959_ = false;
                                                                                                    phi_960_ = f32();
                                                                                                    phi_961_ = f32();
                                                                                                    break;
                                                                                                }
                                                                                            }
                                                                                            let _e1164 = phi_957_;
                                                                                            let _e1166 = phi_958_;
                                                                                            let _e1168 = phi_959_;
                                                                                            let _e1170 = phi_960_;
                                                                                            let _e1172 = phi_961_;
                                                                                            local_4 = select(_e1166, f32(), _e1164);
                                                                                            continue;
                                                                                            continuing {
                                                                                                phi_3219_ = _e573;
                                                                                                phi_640_ = select(_e1170, f32(), _e1164);
                                                                                                phi_643_ = select(_e1172, f32(), _e1164);
                                                                                                break if !(select(_e1168, false, _e1164));
                                                                                            }
                                                                                        }
                                                                                        let _e1432 = local_4;
                                                                                        phi_967_ = _e1432;
                                                                                    } else {
                                                                                        phi_967_ = f32();
                                                                                    }
                                                                                    let _e1179 = phi_967_;
                                                                                    phi_968_ = _e1179;
                                                                                } else {
                                                                                    phi_968_ = f32();
                                                                                }
                                                                                let _e1181 = phi_968_;
                                                                                phi_969_ = _e1181;
                                                                            } else {
                                                                                phi_969_ = f32();
                                                                            }
                                                                            let _e1183 = phi_969_;
                                                                            phi_970_ = _e1183;
                                                                        } else {
                                                                            phi_970_ = f32();
                                                                        }
                                                                        let _e1185 = phi_970_;
                                                                        phi_971_ = _e1185;
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        phi_971_ = 0f;
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1187 = phi_971_;
                                                                let _e1190 = f32(select(0u, 1u, (_e1187 >= 0f)));
                                                                let _e1197 = abs(_e1187);
                                                                let _e1199 = ((_e1197 / _e47) + 0.5f);
                                                                let _e1204 = fwidth(_e1187);
                                                                let _e1206 = fma(-(_e1204), 0.5f, _e53);
                                                                let _e1211 = (fma(abs(((_e1199 - trunc(_e1199)) - 0.5f)), _e47, -(_e1206)) / (fma(_e1204, 0.5f, _e53) - _e1206));
                                                                if (_e1211 < 0f) {
                                                                    phi_3141_ = 0f;
                                                                } else {
                                                                    phi_3141_ = select(_e1211, 1f, (_e1211 > 1f));
                                                                }
                                                                let _e1216 = phi_3141_;
                                                                let _e1219 = ((_e1216 * _e1216) * fma(-2f, _e1216, 3f));
                                                                global_1 = vec4<f32>((fma((bitcast<f32>(_e78) - _e59), _e1190, _e59) * _e1219), (fma((bitcast<f32>(_e84) - _e65), _e1190, _e65) * _e1219), (fma((bitcast<f32>(_e90) - _e71), _e1190, _e71) * _e1219), 1f);
                                                                phi_3223_ = 0u;
                                                                loop {
                                                                    let _e1225 = phi_3223_;
                                                                    let _e1226 = (_e1225 >= _e108);
                                                                    if _e1226 {
                                                                        phi_3284_ = _e1225;
                                                                        phi_3188_ = type_13(0u, type_13().member_1);
                                                                    } else {
                                                                        if _e1226 {
                                                                            phi_3181_ = 4294967295u;
                                                                        } else {
                                                                            phi_3181_ = (_e103 + (3u * _e1225));
                                                                        }
                                                                        let _e1233 = phi_3181_;
                                                                        phi_3284_ = (_e1225 + 1u);
                                                                        phi_3188_ = type_13(1u, _e1233);
                                                                    }
                                                                    let _e1237 = phi_3284_;
                                                                    let _e1239 = phi_3188_;
                                                                    switch bitcast<i32>(_e1239.member) {
                                                                        case 0: {
                                                                            phi_1072_ = true;
                                                                            phi_1073_ = false;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            if (_e1239.member_1 < _e40) {
                                                                                let _e1246 = global.member[_e1239.member_1];
                                                                                let _e1248 = (_e1239.member_1 + 1u);
                                                                                if (_e1248 < _e40) {
                                                                                    let _e1252 = global.member[_e1248];
                                                                                    let _e1254 = (_e1239.member_1 + 2u);
                                                                                    if (_e1254 < _e40) {
                                                                                        let _e1258 = global.member[_e1254];
                                                                                        if (_e1197 <= 0.1f) {
                                                                                            let _e1261 = (_e1197 * 10f);
                                                                                            let _e1263 = global_1[0u];
                                                                                            global_1[0u] = (_e1263 * _e1261);
                                                                                            let _e1266 = global_1[1u];
                                                                                            global_1[1u] = (_e1266 * _e1261);
                                                                                            let _e1269 = global_1[2u];
                                                                                            global_1[2u] = (_e1269 * _e1261);
                                                                                            let _e1272 = global_1[3u];
                                                                                            global_1[3u] = _e1272;
                                                                                        }
                                                                                        let _e1274 = (_e42.x - bitcast<f32>(_e1246));
                                                                                        let _e1276 = (_e42.y - bitcast<f32>(_e1252));
                                                                                        let _e1278 = (_e42.z - bitcast<f32>(_e1258));
                                                                                        let _e1284 = ((sqrt(fma(_e1278, _e1278, fma(_e1274, _e1274, (_e1276 * _e1276)))) - 0.05f) <= 0f);
                                                                                        if _e1284 {
                                                                                            let _e1286 = global_1[0u];
                                                                                            global_1[0u] = _e1286;
                                                                                            let _e1288 = global_1[1u];
                                                                                            global_1[1u] = (_e1288 * 0.5f);
                                                                                            let _e1291 = global_1[2u];
                                                                                            global_1[2u] = (_e1291 * 0.5f);
                                                                                            let _e1294 = global_1[3u];
                                                                                            global_1[3u] = _e1294;
                                                                                        }
                                                                                        phi_1066_ = select(false, true, _e1284);
                                                                                        phi_1067_ = select(true, false, _e1284);
                                                                                    } else {
                                                                                        phi_1066_ = false;
                                                                                        phi_1067_ = false;
                                                                                    }
                                                                                    let _e1298 = phi_1066_;
                                                                                    let _e1300 = phi_1067_;
                                                                                    phi_1068_ = _e1298;
                                                                                    phi_1069_ = _e1300;
                                                                                } else {
                                                                                    phi_1068_ = false;
                                                                                    phi_1069_ = false;
                                                                                }
                                                                                let _e1302 = phi_1068_;
                                                                                let _e1304 = phi_1069_;
                                                                                phi_1070_ = _e1302;
                                                                                phi_1071_ = _e1304;
                                                                            } else {
                                                                                phi_1070_ = false;
                                                                                phi_1071_ = false;
                                                                            }
                                                                            let _e1306 = phi_1070_;
                                                                            let _e1308 = phi_1071_;
                                                                            phi_1072_ = _e1306;
                                                                            phi_1073_ = _e1308;
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            phi_1072_ = false;
                                                                            phi_1073_ = false;
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e1310 = phi_1072_;
                                                                    let _e1312 = phi_1073_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_3223_ = _e1237;
                                                                        break if !(select(_e1312, false, _e1310));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}

@fragment 
fn sdfsdf_shape_fragment(@location(0) @interpolate(flat) param: u32, @location(1) param_1: vec3<f32>) -> @location(0) vec4<f32> {
    global_2 = param;
    global_3 = param_1;
    function();
    let _e5 = global_1;
    return _e5;
}
