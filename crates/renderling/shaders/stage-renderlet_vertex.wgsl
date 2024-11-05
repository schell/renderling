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
    var phi_742_: u32;
    var phi_2870_: bool;
    var phi_749_: u32;
    var phi_750_: u32;
    var phi_760_: u32;
    var phi_842_: type_24;
    var phi_858_: type_24;
    var phi_859_: type_24;
    var phi_872_: type_24;
    var phi_888_: type_24;
    var phi_889_: type_24;
    var phi_902_: type_24;
    var phi_905_: type_31;
    var phi_920_: type_24;
    var phi_921_: type_24;
    var phi_2901_: bool;
    var phi_1042_: type_34;
    var phi_1045_: type_24;
    var phi_1048_: type_20;
    var phi_1063_: type_24;
    var phi_1064_: type_24;
    var phi_1098_: bool;
    var phi_1100_: bool;
    var phi_1101_: bool;
    var phi_1102_: bool;
    var phi_1103_: bool;
    var phi_1137_: bool;
    var phi_1139_: bool;
    var phi_1140_: bool;
    var phi_1141_: bool;
    var phi_1142_: bool;
    var phi_1176_: bool;
    var phi_1178_: bool;
    var phi_1179_: bool;
    var phi_1180_: bool;
    var phi_1181_: bool;
    var phi_1215_: bool;
    var phi_1217_: bool;
    var phi_1218_: bool;
    var phi_1219_: bool;
    var phi_1220_: bool;
    var phi_1225_: bool;
    var phi_1227_: bool;
    var phi_1228_: bool;
    var phi_1229_: bool;
    var phi_1230_: bool;
    var phi_1238_: type_20;
    var phi_3005_: bool;
    var phi_3006_: bool;
    var phi_3069_: vec4<f32>;
    var phi_3099_: vec4<f32>;
    var phi_3101_: vec4<f32>;
    var phi_3112_: type_27;
    var phi_3114_: type_27;
    var phi_3116_: bool;
    var phi_3117_: type_27;
    var phi_3118_: type_27;
    var phi_3122_: type_27;
    var phi_1256_: u32;
    var phi_3218_: bool;
    var phi_1265_: u32;
    var phi_3242_: bool;
    var phi_1314_: type_27;
    var phi_1324_: u32;
    var phi_3267_: bool;
    var phi_1397_: type_20;
    var phi_1635_: type_24;
    var phi_1636_: type_20;
    var phi_1637_: bool;
    var phi_1638_: type_24;
    var phi_1639_: type_20;
    var phi_1640_: bool;
    var phi_1641_: type_27;
    var phi_1642_: bool;
    var phi_1046_: type_24;
    var phi_1049_: type_20;
    var phi_1643_: bool;
    var phi_1644_: type_27;
    var phi_1646_: bool;
    var phi_1647_: type_27;
    var phi_1648_: bool;
    var phi_3334_: bool;
    var phi_1701_: type_27;
    var phi_1703_: type_27;
    var phi_1750_: vec3<f32>;
    var phi_3379_: vec3<f32>;
    var phi_3461_: vec3<f32>;
    var phi_3496_: vec3<f32>;
    var phi_3509_: bool;
    var phi_2074_: type_24;
    var phi_2090_: type_24;
    var phi_2091_: type_24;
    var phi_2104_: type_24;
    var phi_2120_: type_24;
    var phi_2121_: type_24;
    var phi_2147_: type_22;
    var phi_2148_: bool;
    var phi_2105_: type_24;
    var phi_2168_: type_22;
    var phi_2169_: bool;
    var phi_2075_: type_24;
    var phi_2174_: type_22;
    var phi_2235_: u32;
    var phi_3766_: bool;
    var phi_2253_: type_24;
    var phi_3792_: u32;
    var phi_3811_: bool;
    var phi_2303_: type_28;
    var phi_2313_: u32;
    var phi_3833_: bool;
    var phi_2321_: f32;
    var phi_2375_: bool;
    var phi_903_: type_24;
    var phi_906_: type_31;
    var phi_2383_: bool;
    var phi_873_: type_24;
    var phi_2390_: bool;
    var phi_843_: type_24;
    var local_6: bool;
    var local_7: type_27;
    var local_8: type_22;
    var local_9: type_22;

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
            phi_750_ = _e98;
        } else {
            if (_e98 >= _e120) {
                phi_742_ = 4294967295u;
            } else {
                phi_742_ = (_e116 + _e98);
            }
            let _e161 = phi_742_;
            if (_e100 >= 1u) {
                phi_2870_ = (_e161 <= (_e100 - 1u));
            } else {
                phi_2870_ = false;
            }
            let _e166 = phi_2870_;
            if _e166 {
                let _e169 = global_2.member[_e161];
                phi_749_ = _e169;
            } else {
                phi_749_ = 0u;
            }
            let _e171 = phi_749_;
            phi_750_ = _e171;
        }
        let _e173 = phi_750_;
        if (_e173 >= _e112) {
            phi_760_ = 4294967295u;
        } else {
            phi_760_ = (_e108 + (26u * _e173));
        }
        let _e178 = phi_760_;
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
        phi_842_ = type_24(0u, 4u);
        loop {
            let _e276 = phi_842_;
            if (_e276.member < _e276.member_1) {
                phi_858_ = type_24((_e276.member + 1u), _e276.member_1);
                phi_859_ = type_24(1u, _e276.member);
            } else {
                phi_858_ = _e276;
                phi_859_ = type_24(0u, type_24().member_1);
            }
            let _e289 = phi_858_;
            let _e291 = phi_859_;
            switch bitcast<i32>(_e291.member) {
                case 0: {
                    let _e295 = local_5;
                    local_4 = array<f32, 4>(0f, 0f, 0f, 0f);
                    phi_872_ = type_24(0u, 4u);
                    loop {
                        let _e298 = phi_872_;
                        if (_e298.member < _e298.member_1) {
                            phi_888_ = type_24((_e298.member + 1u), _e298.member_1);
                            phi_889_ = type_24(1u, _e298.member);
                        } else {
                            phi_888_ = _e298;
                            phi_889_ = type_24(0u, type_24().member_1);
                        }
                        let _e311 = phi_888_;
                        let _e313 = phi_889_;
                        switch bitcast<i32>(_e313.member) {
                            case 0: {
                                let _e317 = local_4;
                                phi_902_ = type_24(0u, _e144);
                                phi_905_ = type_31(vec3<f32>(bitcast<f32>(_e181), bitcast<f32>(_e186), bitcast<f32>(_e191)), vec4<f32>(bitcast<f32>(_e197), bitcast<f32>(_e202), bitcast<f32>(_e207), bitcast<f32>(_e212)), vec3<f32>(bitcast<f32>(_e240), bitcast<f32>(_e245), bitcast<f32>(_e250)), vec4<f32>(bitcast<f32>(_e256), bitcast<f32>(_e261), bitcast<f32>(_e266), bitcast<f32>(_e271)), _e295, _e317, vec2<f32>(bitcast<f32>(_e218), bitcast<f32>(_e223)), vec2<f32>(bitcast<f32>(_e229), bitcast<f32>(_e234)));
                                loop {
                                    let _e321 = phi_902_;
                                    let _e323 = phi_905_;
                                    if (_e321.member < _e321.member_1) {
                                        phi_920_ = type_24((_e321.member + 1u), _e321.member_1);
                                        phi_921_ = type_24(1u, _e321.member);
                                    } else {
                                        phi_920_ = _e321;
                                        phi_921_ = type_24(0u, type_24().member_1);
                                    }
                                    let _e336 = phi_920_;
                                    let _e338 = phi_921_;
                                    switch bitcast<i32>(_e338.member) {
                                        case 0: {
                                            global_7 = _e323.member_1;
                                            global_8 = _e323.member_6;
                                            global_9 = _e323.member_7;
                                            let _e348 = global_2.member[(_e156 + 6u)];
                                            if (_e348 == 1u) {
                                                let _e351 = ((_e136 == 4294967295u) != true);
                                                if _e351 {
                                                    if (_e100 >= 4u) {
                                                        phi_2901_ = (_e136 <= (_e100 - 4u));
                                                    } else {
                                                        phi_2901_ = false;
                                                    }
                                                    let _e356 = phi_2901_;
                                                    if _e356 {
                                                        let _e359 = global_2.member[_e136];
                                                        let _e363 = global_2.member[(_e136 + 1u)];
                                                        let _e367 = global_2.member[(_e136 + 2u)];
                                                        let _e371 = global_2.member[(_e136 + 3u)];
                                                        phi_1042_ = type_34(type_24(_e359, _e363), type_24(_e367, _e371));
                                                    } else {
                                                        phi_1042_ = type_34(type_24(4294967295u, 0u), type_24(4294967295u, 0u));
                                                    }
                                                    let _e376 = phi_1042_;
                                                    local = _e323.member_5;
                                                    phi_1045_ = type_24(0u, 4u);
                                                    phi_1048_ = type_20(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                                    loop {
                                                        let _e379 = phi_1045_;
                                                        let _e381 = phi_1048_;
                                                        if (_e379.member < _e379.member_1) {
                                                            phi_1063_ = type_24((_e379.member + 1u), _e379.member_1);
                                                            phi_1064_ = type_24(1u, _e379.member);
                                                        } else {
                                                            phi_1063_ = _e379;
                                                            phi_1064_ = type_24(0u, type_24().member_1);
                                                        }
                                                        let _e394 = phi_1063_;
                                                        let _e396 = phi_1064_;
                                                        switch bitcast<i32>(_e396.member) {
                                                            case 0: {
                                                                let _e404 = global_10.member[0u];
                                                                if (_e381.member.x == _e404) {
                                                                    let _e410 = global_10.member[1u];
                                                                    if (_e381.member.y == _e410) {
                                                                        let _e416 = global_10.member[2u];
                                                                        let _e417 = (_e381.member.z == _e416);
                                                                        if _e417 {
                                                                            let _e422 = global_10.member[3u];
                                                                            phi_1098_ = (_e381.member.w == _e422);
                                                                        } else {
                                                                            phi_1098_ = bool();
                                                                        }
                                                                        let _e425 = phi_1098_;
                                                                        phi_1100_ = _e425;
                                                                        phi_1101_ = select(true, false, _e417);
                                                                    } else {
                                                                        phi_1100_ = bool();
                                                                        phi_1101_ = true;
                                                                    }
                                                                    let _e428 = phi_1100_;
                                                                    let _e430 = phi_1101_;
                                                                    phi_1102_ = _e428;
                                                                    phi_1103_ = _e430;
                                                                } else {
                                                                    phi_1102_ = bool();
                                                                    phi_1103_ = true;
                                                                }
                                                                let _e432 = phi_1102_;
                                                                let _e434 = phi_1103_;
                                                                if select(_e432, false, _e434) {
                                                                    let _e440 = global_10.member_1[0u];
                                                                    if (_e381.member_1.x == _e440) {
                                                                        let _e446 = global_10.member_1[1u];
                                                                        if (_e381.member_1.y == _e446) {
                                                                            let _e452 = global_10.member_1[2u];
                                                                            let _e453 = (_e381.member_1.z == _e452);
                                                                            if _e453 {
                                                                                let _e458 = global_10.member_1[3u];
                                                                                phi_1137_ = (_e381.member_1.w == _e458);
                                                                            } else {
                                                                                phi_1137_ = bool();
                                                                            }
                                                                            let _e461 = phi_1137_;
                                                                            phi_1139_ = _e461;
                                                                            phi_1140_ = select(true, false, _e453);
                                                                        } else {
                                                                            phi_1139_ = bool();
                                                                            phi_1140_ = true;
                                                                        }
                                                                        let _e464 = phi_1139_;
                                                                        let _e466 = phi_1140_;
                                                                        phi_1141_ = _e464;
                                                                        phi_1142_ = _e466;
                                                                    } else {
                                                                        phi_1141_ = bool();
                                                                        phi_1142_ = true;
                                                                    }
                                                                    let _e468 = phi_1141_;
                                                                    let _e470 = phi_1142_;
                                                                    if select(_e468, false, _e470) {
                                                                        let _e476 = global_10.member_2[0u];
                                                                        if (_e381.member_2.x == _e476) {
                                                                            let _e482 = global_10.member_2[1u];
                                                                            if (_e381.member_2.y == _e482) {
                                                                                let _e488 = global_10.member_2[2u];
                                                                                let _e489 = (_e381.member_2.z == _e488);
                                                                                if _e489 {
                                                                                    let _e494 = global_10.member_2[3u];
                                                                                    phi_1176_ = (_e381.member_2.w == _e494);
                                                                                } else {
                                                                                    phi_1176_ = bool();
                                                                                }
                                                                                let _e497 = phi_1176_;
                                                                                phi_1178_ = _e497;
                                                                                phi_1179_ = select(true, false, _e489);
                                                                            } else {
                                                                                phi_1178_ = bool();
                                                                                phi_1179_ = true;
                                                                            }
                                                                            let _e500 = phi_1178_;
                                                                            let _e502 = phi_1179_;
                                                                            phi_1180_ = _e500;
                                                                            phi_1181_ = _e502;
                                                                        } else {
                                                                            phi_1180_ = bool();
                                                                            phi_1181_ = true;
                                                                        }
                                                                        let _e504 = phi_1180_;
                                                                        let _e506 = phi_1181_;
                                                                        let _e507 = select(_e504, false, _e506);
                                                                        if _e507 {
                                                                            let _e512 = global_10.member_3[0u];
                                                                            if (_e381.member_3.x == _e512) {
                                                                                let _e518 = global_10.member_3[1u];
                                                                                if (_e381.member_3.y == _e518) {
                                                                                    let _e524 = global_10.member_3[2u];
                                                                                    let _e525 = (_e381.member_3.z == _e524);
                                                                                    if _e525 {
                                                                                        let _e530 = global_10.member_3[3u];
                                                                                        phi_1215_ = (_e381.member_3.w == _e530);
                                                                                    } else {
                                                                                        phi_1215_ = bool();
                                                                                    }
                                                                                    let _e533 = phi_1215_;
                                                                                    phi_1217_ = _e533;
                                                                                    phi_1218_ = select(true, false, _e525);
                                                                                } else {
                                                                                    phi_1217_ = bool();
                                                                                    phi_1218_ = true;
                                                                                }
                                                                                let _e536 = phi_1217_;
                                                                                let _e538 = phi_1218_;
                                                                                phi_1219_ = _e536;
                                                                                phi_1220_ = _e538;
                                                                            } else {
                                                                                phi_1219_ = bool();
                                                                                phi_1220_ = true;
                                                                            }
                                                                            let _e540 = phi_1219_;
                                                                            let _e542 = phi_1220_;
                                                                            phi_1225_ = select(_e540, false, _e542);
                                                                        } else {
                                                                            phi_1225_ = bool();
                                                                        }
                                                                        let _e545 = phi_1225_;
                                                                        phi_1227_ = _e545;
                                                                        phi_1228_ = select(true, false, _e507);
                                                                    } else {
                                                                        phi_1227_ = bool();
                                                                        phi_1228_ = true;
                                                                    }
                                                                    let _e548 = phi_1227_;
                                                                    let _e550 = phi_1228_;
                                                                    phi_1229_ = _e548;
                                                                    phi_1230_ = _e550;
                                                                } else {
                                                                    phi_1229_ = bool();
                                                                    phi_1230_ = true;
                                                                }
                                                                let _e552 = phi_1229_;
                                                                let _e554 = phi_1230_;
                                                                if select(_e552, false, _e554) {
                                                                    phi_1238_ = type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                                                } else {
                                                                    phi_1238_ = _e381;
                                                                }
                                                                let _e557 = phi_1238_;
                                                                let _e580 = fma(_e557.member_2.z, _e557.member_3.w, -((_e557.member_2.w * _e557.member_3.z)));
                                                                let _e583 = fma(_e557.member_2.y, _e557.member_3.w, -((_e557.member_2.w * _e557.member_3.y)));
                                                                let _e586 = fma(_e557.member_2.y, _e557.member_3.z, -((_e557.member_2.z * _e557.member_3.y)));
                                                                let _e589 = fma(_e557.member_2.x, _e557.member_3.w, -((_e557.member_2.w * _e557.member_3.x)));
                                                                let _e592 = fma(_e557.member_2.x, _e557.member_3.z, -((_e557.member_2.z * _e557.member_3.x)));
                                                                let _e595 = fma(_e557.member_2.x, _e557.member_3.y, -((_e557.member_2.y * _e557.member_3.x)));
                                                                let _e617 = fma(-(_e557.member.w), fma(_e557.member_1.z, _e595, fma(_e557.member_1.x, _e586, -((_e557.member_1.y * _e592)))), fma(_e557.member.z, fma(_e557.member_1.w, _e595, fma(_e557.member_1.x, _e583, -((_e557.member_1.y * _e589)))), fma(_e557.member.x, fma(_e557.member_1.w, _e586, fma(_e557.member_1.y, _e580, -((_e557.member_1.z * _e583)))), -((_e557.member.y * fma(_e557.member_1.w, _e592, fma(_e557.member_1.x, _e580, -((_e557.member_1.z * _e589)))))))));
                                                                if (_e617 == 0f) {
                                                                    phi_3116_ = true;
                                                                    phi_3117_ = type_27(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                                                                    phi_3118_ = type_27();
                                                                } else {
                                                                    let _e626 = (sqrt(fma(_e557.member.w, _e557.member.w, fma(_e557.member.z, _e557.member.z, fma(_e557.member.x, _e557.member.x, (_e557.member.y * _e557.member.y))))) * select(-1f, 1f, (_e617 >= 0f)));
                                                                    let _e631 = sqrt(fma(_e557.member_1.w, _e557.member_1.w, fma(_e557.member_1.z, _e557.member_1.z, fma(_e557.member_1.x, _e557.member_1.x, (_e557.member_1.y * _e557.member_1.y)))));
                                                                    let _e636 = sqrt(fma(_e557.member_2.w, _e557.member_2.w, fma(_e557.member_2.z, _e557.member_2.z, fma(_e557.member_2.x, _e557.member_2.x, (_e557.member_2.y * _e557.member_2.y)))));
                                                                    let _e639 = (_e631 != 0f);
                                                                    if (_e626 != 0f) {
                                                                        phi_3005_ = select(bool(), (_e636 != 0f), _e639);
                                                                        phi_3006_ = select(true, false, _e639);
                                                                    } else {
                                                                        phi_3005_ = bool();
                                                                        phi_3006_ = true;
                                                                    }
                                                                    let _e644 = phi_3005_;
                                                                    let _e646 = phi_3006_;
                                                                    let _e647 = select(_e644, false, _e646);
                                                                    if _e647 {
                                                                        let _e648 = (1f / _e626);
                                                                        let _e649 = (1f / _e631);
                                                                        let _e650 = (1f / _e636);
                                                                        let _e651 = (_e557.member.x * _e648);
                                                                        let _e652 = (_e557.member.z * _e648);
                                                                        let _e653 = (_e557.member_1.x * _e649);
                                                                        let _e654 = (_e557.member_2.x * _e650);
                                                                        let _e655 = (_e557.member_2.y * _e650);
                                                                        if ((_e557.member_2.z * _e650) <= 0f) {
                                                                            let _e659 = fma(_e557.member_1.y, _e649, -(_e651));
                                                                            let _e661 = fma(-(_e557.member_2.z), _e650, 1f);
                                                                            if (_e659 <= 0f) {
                                                                                let _e663 = (_e661 - _e659);
                                                                                let _e665 = (0.5f / sqrt(_e663));
                                                                                phi_3069_ = vec4<f32>((_e663 * _e665), (fma(_e557.member.y, _e648, _e653) * _e665), (fma(_e557.member.z, _e648, _e654) * _e665), (fma(_e557.member_1.z, _e649, -(_e655)) * _e665));
                                                                            } else {
                                                                                let _e675 = (_e661 + _e659);
                                                                                let _e677 = (0.5f / sqrt(_e675));
                                                                                phi_3069_ = vec4<f32>((fma(_e557.member.y, _e648, _e653) * _e677), (_e675 * _e677), (fma(_e557.member_1.z, _e649, _e655) * _e677), (fma(_e557.member_2.x, _e650, -(_e652)) * _e677));
                                                                            }
                                                                            let _e688 = phi_3069_;
                                                                            phi_3101_ = _e688;
                                                                        } else {
                                                                            let _e689 = fma(_e557.member_1.y, _e649, _e651);
                                                                            let _e690 = fma(_e557.member_2.z, _e650, 1f);
                                                                            if (_e689 <= 0f) {
                                                                                let _e692 = (_e690 - _e689);
                                                                                let _e694 = (0.5f / sqrt(_e692));
                                                                                phi_3099_ = vec4<f32>((fma(_e557.member.z, _e648, _e654) * _e694), (fma(_e557.member_1.z, _e649, _e655) * _e694), (_e692 * _e694), (fma(_e557.member.y, _e648, -(_e653)) * _e694));
                                                                            } else {
                                                                                let _e704 = (_e690 + _e689);
                                                                                let _e706 = (0.5f / sqrt(_e704));
                                                                                phi_3099_ = vec4<f32>((fma(_e557.member_1.z, _e649, -(_e655)) * _e706), (fma(_e557.member_2.x, _e650, -(_e652)) * _e706), (fma(_e557.member.y, _e648, -(_e653)) * _e706), (_e704 * _e706));
                                                                            }
                                                                            let _e719 = phi_3099_;
                                                                            phi_3101_ = _e719;
                                                                        }
                                                                        let _e721 = phi_3101_;
                                                                        phi_3112_ = type_27(vec3<f32>(_e626, _e631, _e636), _e721, vec3<f32>(_e557.member_3.x, _e557.member_3.y, _e557.member_3.z));
                                                                        phi_3114_ = type_27();
                                                                    } else {
                                                                        phi_3112_ = type_27();
                                                                        phi_3114_ = type_27(vec3<f32>(1f, 1f, 1f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(0f, 0f, 0f));
                                                                    }
                                                                    let _e725 = phi_3112_;
                                                                    let _e727 = phi_3114_;
                                                                    phi_3116_ = select(true, false, _e647);
                                                                    phi_3117_ = _e727;
                                                                    phi_3118_ = _e725;
                                                                }
                                                                let _e730 = phi_3116_;
                                                                let _e732 = phi_3117_;
                                                                let _e734 = phi_3118_;
                                                                if _e730 {
                                                                    phi_3122_ = _e732;
                                                                } else {
                                                                    phi_3122_ = _e734;
                                                                }
                                                                let _e736 = phi_3122_;
                                                                phi_1640_ = true;
                                                                phi_1641_ = type_27(_e736.member_2, _e736.member_1, _e736.member);
                                                                phi_1642_ = false;
                                                                phi_1046_ = type_24();
                                                                phi_1049_ = type_20();
                                                                break;
                                                            }
                                                            case 1: {
                                                                local_1 = _e323.member_4;
                                                                let _e742 = (_e396.member_1 < 4u);
                                                                if _e742 {
                                                                    let _e744 = local_1[_e396.member_1];
                                                                    if (_e744 >= _e376.member.member_1) {
                                                                        phi_1256_ = 4294967295u;
                                                                    } else {
                                                                        phi_1256_ = (_e376.member.member + _e744);
                                                                    }
                                                                    let _e752 = phi_1256_;
                                                                    if (_e100 >= 1u) {
                                                                        phi_3218_ = (_e752 <= (_e100 - 1u));
                                                                    } else {
                                                                        phi_3218_ = false;
                                                                    }
                                                                    let _e757 = phi_3218_;
                                                                    if _e757 {
                                                                        let _e760 = global_2.member[_e752];
                                                                        phi_1265_ = _e760;
                                                                    } else {
                                                                        phi_1265_ = 4294967295u;
                                                                    }
                                                                    let _e762 = phi_1265_;
                                                                    if (_e100 >= 10u) {
                                                                        phi_3242_ = (_e762 <= (_e100 - 10u));
                                                                    } else {
                                                                        phi_3242_ = false;
                                                                    }
                                                                    let _e767 = phi_3242_;
                                                                    if _e767 {
                                                                        let _e770 = global_2.member[_e762];
                                                                        let _e775 = global_2.member[(_e762 + 1u)];
                                                                        let _e780 = global_2.member[(_e762 + 2u)];
                                                                        let _e786 = global_2.member[(_e762 + 3u)];
                                                                        let _e791 = global_2.member[(_e762 + 4u)];
                                                                        let _e796 = global_2.member[(_e762 + 5u)];
                                                                        let _e801 = global_2.member[(_e762 + 6u)];
                                                                        let _e807 = global_2.member[(_e762 + 7u)];
                                                                        let _e812 = global_2.member[(_e762 + 8u)];
                                                                        let _e817 = global_2.member[(_e762 + 9u)];
                                                                        phi_1314_ = type_27(vec3<f32>(bitcast<f32>(_e770), bitcast<f32>(_e775), bitcast<f32>(_e780)), vec4<f32>(bitcast<f32>(_e786), bitcast<f32>(_e791), bitcast<f32>(_e796), bitcast<f32>(_e801)), vec3<f32>(bitcast<f32>(_e807), bitcast<f32>(_e812), bitcast<f32>(_e817)));
                                                                    } else {
                                                                        phi_1314_ = type_27(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                                                    }
                                                                    let _e822 = phi_1314_;
                                                                    if (_e744 >= _e376.member_1.member_1) {
                                                                        phi_1324_ = 4294967295u;
                                                                    } else {
                                                                        phi_1324_ = (_e376.member_1.member + (16u * _e744));
                                                                    }
                                                                    let _e831 = phi_1324_;
                                                                    if (_e100 >= 16u) {
                                                                        phi_3267_ = (_e831 <= (_e100 - 16u));
                                                                    } else {
                                                                        phi_3267_ = false;
                                                                    }
                                                                    let _e836 = phi_3267_;
                                                                    if _e836 {
                                                                        let _e839 = global_2.member[_e831];
                                                                        let _e844 = global_2.member[(_e831 + 1u)];
                                                                        let _e849 = global_2.member[(_e831 + 2u)];
                                                                        let _e854 = global_2.member[(_e831 + 3u)];
                                                                        let _e860 = global_2.member[(_e831 + 4u)];
                                                                        let _e865 = global_2.member[(_e831 + 5u)];
                                                                        let _e870 = global_2.member[(_e831 + 6u)];
                                                                        let _e875 = global_2.member[(_e831 + 7u)];
                                                                        let _e881 = global_2.member[(_e831 + 8u)];
                                                                        let _e886 = global_2.member[(_e831 + 9u)];
                                                                        let _e891 = global_2.member[(_e831 + 10u)];
                                                                        let _e896 = global_2.member[(_e831 + 11u)];
                                                                        let _e902 = global_2.member[(_e831 + 12u)];
                                                                        let _e907 = global_2.member[(_e831 + 13u)];
                                                                        let _e912 = global_2.member[(_e831 + 14u)];
                                                                        let _e917 = global_2.member[(_e831 + 15u)];
                                                                        phi_1397_ = type_20(vec4<f32>(bitcast<f32>(_e839), bitcast<f32>(_e844), bitcast<f32>(_e849), bitcast<f32>(_e854)), vec4<f32>(bitcast<f32>(_e860), bitcast<f32>(_e865), bitcast<f32>(_e870), bitcast<f32>(_e875)), vec4<f32>(bitcast<f32>(_e881), bitcast<f32>(_e886), bitcast<f32>(_e891), bitcast<f32>(_e896)), vec4<f32>(bitcast<f32>(_e902), bitcast<f32>(_e907), bitcast<f32>(_e912), bitcast<f32>(_e917)));
                                                                    } else {
                                                                        phi_1397_ = type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f));
                                                                    }
                                                                    let _e922 = phi_1397_;
                                                                    let _e930 = (_e822.member_1.x + _e822.member_1.x);
                                                                    let _e931 = (_e822.member_1.y + _e822.member_1.y);
                                                                    let _e932 = (_e822.member_1.z + _e822.member_1.z);
                                                                    let _e934 = (_e822.member_1.z * _e932);
                                                                    let _e935 = (_e822.member_1.w * _e930);
                                                                    let _e936 = (_e822.member_1.w * _e931);
                                                                    let _e937 = (_e822.member_1.w * _e932);
                                                                    let _e957 = (vec4<f32>((1f - fma(_e822.member_1.y, _e931, _e934)), fma(_e822.member_1.x, _e931, _e937), fma(_e822.member_1.x, _e932, -(_e936)), 0f) * _e822.member_2.x);
                                                                    let _e959 = (vec4<f32>(fma(_e822.member_1.x, _e931, -(_e937)), (1f - fma(_e822.member_1.x, _e930, _e934)), fma(_e822.member_1.y, _e932, _e935), 0f) * _e822.member_2.y);
                                                                    let _e961 = (vec4<f32>(fma(_e822.member_1.x, _e932, _e936), fma(_e822.member_1.y, _e932, -(_e935)), (1f - fma(_e822.member_1.x, _e930, (_e822.member_1.y * _e931))), 0f) * _e822.member_2.z);
                                                                    if _e742 {
                                                                        let _e1066 = local[_e396.member_1];
                                                                        phi_1635_ = _e394;
                                                                        phi_1636_ = type_20((_e381.member + (vec4<f32>(fma(_e822.member.x, _e922.member.w, fma(_e961.x, _e922.member.z, fma(_e957.x, _e922.member.x, (_e959.x * _e922.member.y)))), fma(_e822.member.y, _e922.member.w, fma(_e961.y, _e922.member.z, fma(_e957.y, _e922.member.x, (_e959.y * _e922.member.y)))), fma(_e822.member.z, _e922.member.w, fma(_e961.z, _e922.member.z, fma(_e957.z, _e922.member.x, (_e959.z * _e922.member.y)))), (fma(_e961.w, _e922.member.z, fma(_e957.w, _e922.member.x, (_e959.w * _e922.member.y))) + _e922.member.w)) * _e1066)), (_e381.member_1 + (vec4<f32>(fma(_e822.member.x, _e922.member_1.w, fma(_e961.x, _e922.member_1.z, fma(_e957.x, _e922.member_1.x, (_e959.x * _e922.member_1.y)))), fma(_e822.member.y, _e922.member_1.w, fma(_e961.y, _e922.member_1.z, fma(_e957.y, _e922.member_1.x, (_e959.y * _e922.member_1.y)))), fma(_e822.member.z, _e922.member_1.w, fma(_e961.z, _e922.member_1.z, fma(_e957.z, _e922.member_1.x, (_e959.z * _e922.member_1.y)))), (fma(_e961.w, _e922.member_1.z, fma(_e957.w, _e922.member_1.x, (_e959.w * _e922.member_1.y))) + _e922.member_1.w)) * _e1066)), (_e381.member_2 + (vec4<f32>(fma(_e822.member.x, _e922.member_2.w, fma(_e961.x, _e922.member_2.z, fma(_e957.x, _e922.member_2.x, (_e959.x * _e922.member_2.y)))), fma(_e822.member.y, _e922.member_2.w, fma(_e961.y, _e922.member_2.z, fma(_e957.y, _e922.member_2.x, (_e959.y * _e922.member_2.y)))), fma(_e822.member.z, _e922.member_2.w, fma(_e961.z, _e922.member_2.z, fma(_e957.z, _e922.member_2.x, (_e959.z * _e922.member_2.y)))), (fma(_e961.w, _e922.member_2.z, fma(_e957.w, _e922.member_2.x, (_e959.w * _e922.member_2.y))) + _e922.member_2.w)) * _e1066)), (_e381.member_3 + (vec4<f32>(fma(_e822.member.x, _e922.member_3.w, fma(_e961.x, _e922.member_3.z, fma(_e957.x, _e922.member_3.x, (_e959.x * _e922.member_3.y)))), fma(_e822.member.y, _e922.member_3.w, fma(_e961.y, _e922.member_3.z, fma(_e957.y, _e922.member_3.x, (_e959.y * _e922.member_3.y)))), fma(_e822.member.z, _e922.member_3.w, fma(_e961.z, _e922.member_3.z, fma(_e957.z, _e922.member_3.x, (_e959.z * _e922.member_3.y)))), (fma(_e961.w, _e922.member_3.z, fma(_e957.w, _e922.member_3.x, (_e959.w * _e922.member_3.y))) + _e922.member_3.w)) * _e1066)));
                                                                    } else {
                                                                        phi_1635_ = type_24();
                                                                        phi_1636_ = type_20();
                                                                    }
                                                                    let _e1081 = phi_1635_;
                                                                    let _e1083 = phi_1636_;
                                                                    phi_1637_ = select(false, true, _e742);
                                                                    phi_1638_ = _e1081;
                                                                    phi_1639_ = _e1083;
                                                                } else {
                                                                    phi_1637_ = false;
                                                                    phi_1638_ = type_24();
                                                                    phi_1639_ = type_20();
                                                                }
                                                                let _e1086 = phi_1637_;
                                                                let _e1088 = phi_1638_;
                                                                let _e1090 = phi_1639_;
                                                                phi_1640_ = false;
                                                                phi_1641_ = type_27();
                                                                phi_1642_ = _e1086;
                                                                phi_1046_ = _e1088;
                                                                phi_1049_ = _e1090;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_1640_ = false;
                                                                phi_1641_ = type_27();
                                                                phi_1642_ = false;
                                                                phi_1046_ = type_24();
                                                                phi_1049_ = type_20();
                                                                break;
                                                            }
                                                        }
                                                        let _e1092 = phi_1640_;
                                                        let _e1094 = phi_1641_;
                                                        let _e1096 = phi_1642_;
                                                        let _e1098 = phi_1046_;
                                                        let _e1100 = phi_1049_;
                                                        local_6 = _e1092;
                                                        local_7 = _e1094;
                                                        continue;
                                                        continuing {
                                                            phi_1045_ = _e1098;
                                                            phi_1048_ = _e1100;
                                                            break if !(_e1096);
                                                        }
                                                    }
                                                    let _e2034 = local_6;
                                                    phi_1643_ = _e2034;
                                                    let _e2037 = local_7;
                                                    phi_1644_ = _e2037;
                                                } else {
                                                    phi_1643_ = false;
                                                    phi_1644_ = type_27();
                                                }
                                                let _e1103 = phi_1643_;
                                                let _e1105 = phi_1644_;
                                                phi_1646_ = _e1103;
                                                phi_1647_ = _e1105;
                                                phi_1648_ = select(true, false, _e351);
                                            } else {
                                                phi_1646_ = false;
                                                phi_1647_ = type_27();
                                                phi_1648_ = true;
                                            }
                                            let _e1108 = phi_1646_;
                                            let _e1110 = phi_1647_;
                                            let _e1112 = phi_1648_;
                                            if _e1112 {
                                                if (_e100 >= 10u) {
                                                    phi_3334_ = (_e128 <= (_e100 - 10u));
                                                } else {
                                                    phi_3334_ = false;
                                                }
                                                let _e1117 = phi_3334_;
                                                if _e1117 {
                                                    let _e1120 = global_2.member[_e128];
                                                    let _e1125 = global_2.member[(_e128 + 1u)];
                                                    let _e1130 = global_2.member[(_e128 + 2u)];
                                                    let _e1136 = global_2.member[(_e128 + 3u)];
                                                    let _e1141 = global_2.member[(_e128 + 4u)];
                                                    let _e1146 = global_2.member[(_e128 + 5u)];
                                                    let _e1151 = global_2.member[(_e128 + 6u)];
                                                    let _e1157 = global_2.member[(_e128 + 7u)];
                                                    let _e1162 = global_2.member[(_e128 + 8u)];
                                                    let _e1167 = global_2.member[(_e128 + 9u)];
                                                    phi_1701_ = type_27(vec3<f32>(bitcast<f32>(_e1120), bitcast<f32>(_e1125), bitcast<f32>(_e1130)), vec4<f32>(bitcast<f32>(_e1136), bitcast<f32>(_e1141), bitcast<f32>(_e1146), bitcast<f32>(_e1151)), vec3<f32>(bitcast<f32>(_e1157), bitcast<f32>(_e1162), bitcast<f32>(_e1167)));
                                                } else {
                                                    phi_1701_ = type_27(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                                }
                                                let _e1172 = phi_1701_;
                                                phi_1703_ = _e1172;
                                            } else {
                                                phi_1703_ = _e1110;
                                            }
                                            let _e1174 = phi_1703_;
                                            if select(_e1108, true, _e1112) {
                                                let _e1190 = sqrt(fma(_e323.member_2.z, _e323.member_2.z, fma(_e323.member_2.x, _e323.member_2.x, (_e323.member_2.y * _e323.member_2.y))));
                                                if (_e1190 == 0f) {
                                                    phi_1750_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_1750_ = (_e323.member_2 * (1f / _e1190));
                                                }
                                                let _e1195 = phi_1750_;
                                                let _e1204 = sqrt(fma(_e323.member_3.z, _e323.member_3.z, fma(_e323.member_3.x, _e323.member_3.x, (_e323.member_3.y * _e323.member_3.y))));
                                                if (_e1204 == 0f) {
                                                    phi_3379_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_3379_ = (vec3<f32>(_e323.member_3.x, _e323.member_3.y, _e323.member_3.z) * (1f / _e1204));
                                                }
                                                let _e1209 = phi_3379_;
                                                let _e1216 = (_e1174.member_1.x + _e1174.member_1.x);
                                                let _e1217 = (_e1174.member_1.y + _e1174.member_1.y);
                                                let _e1218 = (_e1174.member_1.z + _e1174.member_1.z);
                                                let _e1220 = (_e1174.member_1.z * _e1218);
                                                let _e1221 = (_e1174.member_1.w * _e1216);
                                                let _e1222 = (_e1174.member_1.w * _e1217);
                                                let _e1223 = (_e1174.member_1.w * _e1218);
                                                let _e1242 = (vec4<f32>((1f - fma(_e1174.member_1.y, _e1217, _e1220)), fma(_e1174.member_1.x, _e1217, _e1223), fma(_e1174.member_1.x, _e1218, -(_e1222)), 0f) * _e1174.member_2.x);
                                                let _e1243 = (vec4<f32>(fma(_e1174.member_1.x, _e1217, -(_e1223)), (1f - fma(_e1174.member_1.x, _e1216, _e1220)), fma(_e1174.member_1.y, _e1218, _e1221), 0f) * _e1174.member_2.y);
                                                let _e1244 = (vec4<f32>(fma(_e1174.member_1.x, _e1218, _e1222), fma(_e1174.member_1.y, _e1218, -(_e1221)), (1f - fma(_e1174.member_1.x, _e1216, (_e1174.member_1.y * _e1217))), 0f) * _e1174.member_2.z);
                                                let _e1249 = (_e1195.x / (_e1174.member_2.x * _e1174.member_2.x));
                                                let _e1251 = (_e1195.y / (_e1174.member_2.y * _e1174.member_2.y));
                                                let _e1253 = (_e1195.z / (_e1174.member_2.z * _e1174.member_2.z));
                                                let _e1269 = fma(_e1244.x, _e1253, fma(_e1242.x, _e1249, (_e1243.x * _e1251)));
                                                let _e1270 = fma(_e1244.y, _e1253, fma(_e1242.y, _e1249, (_e1243.y * _e1251)));
                                                let _e1271 = fma(_e1244.z, _e1253, fma(_e1242.z, _e1249, (_e1243.z * _e1251)));
                                                let _e1276 = sqrt(fma(_e1271, _e1271, fma(_e1269, _e1269, (_e1270 * _e1270))));
                                                if (_e1276 == 0f) {
                                                    phi_3461_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_3461_ = (vec3<f32>(_e1269, _e1270, _e1271) * (1f / _e1276));
                                                }
                                                let _e1281 = phi_3461_;
                                                global_11 = _e1281;
                                                let _e1291 = fma(_e1244.x, _e1209.z, fma(_e1242.x, _e1209.x, (_e1243.x * _e1209.y)));
                                                let _e1292 = fma(_e1244.y, _e1209.z, fma(_e1242.y, _e1209.x, (_e1243.y * _e1209.y)));
                                                let _e1293 = fma(_e1244.z, _e1209.z, fma(_e1242.z, _e1209.x, (_e1243.z * _e1209.y)));
                                                let _e1298 = sqrt(fma(_e1293, _e1293, fma(_e1291, _e1291, (_e1292 * _e1292))));
                                                if (_e1298 == 0f) {
                                                    phi_3496_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_3496_ = (vec3<f32>(_e1291, _e1292, _e1293) * (1f / _e1298));
                                                }
                                                let _e1303 = phi_3496_;
                                                global_12 = _e1303;
                                                let _e1322 = select(-1f, 1f, (_e323.member_3.w >= 0f));
                                                global_13 = vec3<f32>((fma(_e1281.y, _e1303.z, -((_e1303.y * _e1281.z))) * _e1322), (fma(_e1281.z, _e1303.x, -((_e1303.z * _e1281.x))) * _e1322), (fma(_e1281.x, _e1303.y, -((_e1303.x * _e1281.y))) * _e1322));
                                                let _e1340 = (_e1174.member.x + fma(_e1244.x, _e323.member.z, fma(_e1243.x, _e323.member.y, (_e1242.x * _e323.member.x))));
                                                let _e1341 = (_e1174.member.y + fma(_e1244.y, _e323.member.z, fma(_e1243.y, _e323.member.y, (_e1242.y * _e323.member.x))));
                                                let _e1342 = (_e1174.member.z + fma(_e1244.z, _e323.member.z, fma(_e1243.z, _e323.member.y, (_e1242.z * _e323.member.x))));
                                                global_14 = vec3<f32>(_e1340, _e1341, _e1342);
                                                if (_e100 >= 83u) {
                                                    phi_3509_ = (_e124 <= (_e100 - 83u));
                                                } else {
                                                    phi_3509_ = false;
                                                }
                                                let _e1348 = phi_3509_;
                                                if _e1348 {
                                                    let _e1351 = global_2.member[_e124];
                                                    let _e1356 = global_2.member[(_e124 + 1u)];
                                                    let _e1361 = global_2.member[(_e124 + 2u)];
                                                    let _e1366 = global_2.member[(_e124 + 3u)];
                                                    let _e1372 = global_2.member[(_e124 + 4u)];
                                                    let _e1377 = global_2.member[(_e124 + 5u)];
                                                    let _e1382 = global_2.member[(_e124 + 6u)];
                                                    let _e1387 = global_2.member[(_e124 + 7u)];
                                                    let _e1393 = global_2.member[(_e124 + 8u)];
                                                    let _e1398 = global_2.member[(_e124 + 9u)];
                                                    let _e1403 = global_2.member[(_e124 + 10u)];
                                                    let _e1408 = global_2.member[(_e124 + 11u)];
                                                    let _e1414 = global_2.member[(_e124 + 12u)];
                                                    let _e1419 = global_2.member[(_e124 + 13u)];
                                                    let _e1424 = global_2.member[(_e124 + 14u)];
                                                    let _e1429 = global_2.member[(_e124 + 15u)];
                                                    let _e1436 = global_2.member[(_e124 + 16u)];
                                                    let _e1441 = global_2.member[(_e124 + 17u)];
                                                    let _e1446 = global_2.member[(_e124 + 18u)];
                                                    let _e1451 = global_2.member[(_e124 + 19u)];
                                                    let _e1457 = global_2.member[(_e124 + 20u)];
                                                    let _e1462 = global_2.member[(_e124 + 21u)];
                                                    let _e1467 = global_2.member[(_e124 + 22u)];
                                                    let _e1472 = global_2.member[(_e124 + 23u)];
                                                    let _e1478 = global_2.member[(_e124 + 24u)];
                                                    let _e1483 = global_2.member[(_e124 + 25u)];
                                                    let _e1488 = global_2.member[(_e124 + 26u)];
                                                    let _e1493 = global_2.member[(_e124 + 27u)];
                                                    let _e1499 = global_2.member[(_e124 + 28u)];
                                                    let _e1504 = global_2.member[(_e124 + 29u)];
                                                    let _e1509 = global_2.member[(_e124 + 30u)];
                                                    let _e1514 = global_2.member[(_e124 + 31u)];
                                                    let _e1521 = global_2.member[(_e124 + 32u)];
                                                    let _e1526 = global_2.member[(_e124 + 33u)];
                                                    let _e1531 = global_2.member[(_e124 + 34u)];
                                                    local_3 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                                    phi_2074_ = type_24(0u, 6u);
                                                    loop {
                                                        let _e1536 = phi_2074_;
                                                        if (_e1536.member < _e1536.member_1) {
                                                            phi_2090_ = type_24((_e1536.member + 1u), _e1536.member_1);
                                                            phi_2091_ = type_24(1u, _e1536.member);
                                                        } else {
                                                            phi_2090_ = _e1536;
                                                            phi_2091_ = type_24(0u, type_24().member_1);
                                                        }
                                                        let _e1549 = phi_2090_;
                                                        let _e1551 = phi_2091_;
                                                        switch bitcast<i32>(_e1551.member) {
                                                            case 0: {
                                                                let _e1555 = local_3;
                                                                local_2 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                                phi_2104_ = type_24(0u, 8u);
                                                                loop {
                                                                    let _e1558 = phi_2104_;
                                                                    if (_e1558.member < _e1558.member_1) {
                                                                        phi_2120_ = type_24((_e1558.member + 1u), _e1558.member_1);
                                                                        phi_2121_ = type_24(1u, _e1558.member);
                                                                    } else {
                                                                        phi_2120_ = _e1558;
                                                                        phi_2121_ = type_24(0u, type_24().member_1);
                                                                    }
                                                                    let _e1571 = phi_2120_;
                                                                    let _e1573 = phi_2121_;
                                                                    switch bitcast<i32>(_e1573.member) {
                                                                        case 0: {
                                                                            let _e1577 = local_2;
                                                                            phi_2147_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1351), bitcast<f32>(_e1356), bitcast<f32>(_e1361), bitcast<f32>(_e1366)), vec4<f32>(bitcast<f32>(_e1372), bitcast<f32>(_e1377), bitcast<f32>(_e1382), bitcast<f32>(_e1387)), vec4<f32>(bitcast<f32>(_e1393), bitcast<f32>(_e1398), bitcast<f32>(_e1403), bitcast<f32>(_e1408)), vec4<f32>(bitcast<f32>(_e1414), bitcast<f32>(_e1419), bitcast<f32>(_e1424), bitcast<f32>(_e1429))), type_20(vec4<f32>(bitcast<f32>(_e1436), bitcast<f32>(_e1441), bitcast<f32>(_e1446), bitcast<f32>(_e1451)), vec4<f32>(bitcast<f32>(_e1457), bitcast<f32>(_e1462), bitcast<f32>(_e1467), bitcast<f32>(_e1472)), vec4<f32>(bitcast<f32>(_e1478), bitcast<f32>(_e1483), bitcast<f32>(_e1488), bitcast<f32>(_e1493)), vec4<f32>(bitcast<f32>(_e1499), bitcast<f32>(_e1504), bitcast<f32>(_e1509), bitcast<f32>(_e1514))), type_21(_e1577, _e1555), vec3<f32>(bitcast<f32>(_e1521), bitcast<f32>(_e1526), bitcast<f32>(_e1531)));
                                                                            phi_2148_ = false;
                                                                            phi_2105_ = type_24();
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            let _e1581 = ((_e124 + 59u) + (_e1573.member_1 * 3u));
                                                                            let _e1584 = global_2.member[_e1581];
                                                                            let _e1589 = global_2.member[(_e1581 + 1u)];
                                                                            let _e1594 = global_2.member[(_e1581 + 2u)];
                                                                            local_2[_e1573.member_1] = vec3<f32>(bitcast<f32>(_e1584), bitcast<f32>(_e1589), bitcast<f32>(_e1594));
                                                                            phi_2147_ = type_22();
                                                                            phi_2148_ = true;
                                                                            phi_2105_ = _e1571;
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            phi_2147_ = type_22();
                                                                            phi_2148_ = false;
                                                                            phi_2105_ = type_24();
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e1599 = phi_2147_;
                                                                    let _e1601 = phi_2148_;
                                                                    let _e1603 = phi_2105_;
                                                                    local_8 = _e1599;
                                                                    continue;
                                                                    continuing {
                                                                        phi_2104_ = _e1603;
                                                                        break if !(_e1601);
                                                                    }
                                                                }
                                                                let _e2060 = local_8;
                                                                phi_2168_ = _e2060;
                                                                phi_2169_ = false;
                                                                phi_2075_ = type_24();
                                                                break;
                                                            }
                                                            case 1: {
                                                                let _e1606 = ((_e124 + 35u) + (_e1551.member_1 * 4u));
                                                                let _e1609 = global_2.member[_e1606];
                                                                let _e1614 = global_2.member[(_e1606 + 1u)];
                                                                let _e1619 = global_2.member[(_e1606 + 2u)];
                                                                let _e1624 = global_2.member[(_e1606 + 3u)];
                                                                local_3[_e1551.member_1] = vec4<f32>(bitcast<f32>(_e1609), bitcast<f32>(_e1614), bitcast<f32>(_e1619), bitcast<f32>(_e1624));
                                                                phi_2168_ = type_22();
                                                                phi_2169_ = true;
                                                                phi_2075_ = _e1549;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_2168_ = type_22();
                                                                phi_2169_ = false;
                                                                phi_2075_ = type_24();
                                                                break;
                                                            }
                                                        }
                                                        let _e1629 = phi_2168_;
                                                        let _e1631 = phi_2169_;
                                                        let _e1633 = phi_2075_;
                                                        local_9 = _e1629;
                                                        continue;
                                                        continuing {
                                                            phi_2074_ = _e1633;
                                                            break if !(_e1631);
                                                        }
                                                    }
                                                    let _e2065 = local_9;
                                                    phi_2174_ = _e2065;
                                                } else {
                                                    phi_2174_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
                                                }
                                                let _e1636 = phi_2174_;
                                                global_1 = vec4<f32>((fma(fma(_e1636.member.member_3.x, _e1636.member_1.member_2.w, fma(_e1636.member.member_2.x, _e1636.member_1.member_2.z, fma(_e1636.member.member.x, _e1636.member_1.member_2.x, (_e1636.member.member_1.x * _e1636.member_1.member_2.y)))), _e1342, fma(fma(_e1636.member.member_3.x, _e1636.member_1.member.w, fma(_e1636.member.member_2.x, _e1636.member_1.member.z, fma(_e1636.member.member.x, _e1636.member_1.member.x, (_e1636.member.member_1.x * _e1636.member_1.member.y)))), _e1340, (fma(_e1636.member.member_3.x, _e1636.member_1.member_1.w, fma(_e1636.member.member_2.x, _e1636.member_1.member_1.z, fma(_e1636.member.member.x, _e1636.member_1.member_1.x, (_e1636.member.member_1.x * _e1636.member_1.member_1.y)))) * _e1341))) + fma(_e1636.member.member_3.x, _e1636.member_1.member_3.w, fma(_e1636.member.member_2.x, _e1636.member_1.member_3.z, fma(_e1636.member.member.x, _e1636.member_1.member_3.x, (_e1636.member.member_1.x * _e1636.member_1.member_3.y))))), (fma(fma(_e1636.member.member_3.y, _e1636.member_1.member_2.w, fma(_e1636.member.member_2.y, _e1636.member_1.member_2.z, fma(_e1636.member.member.y, _e1636.member_1.member_2.x, (_e1636.member.member_1.y * _e1636.member_1.member_2.y)))), _e1342, fma(fma(_e1636.member.member_3.y, _e1636.member_1.member.w, fma(_e1636.member.member_2.y, _e1636.member_1.member.z, fma(_e1636.member.member.y, _e1636.member_1.member.x, (_e1636.member.member_1.y * _e1636.member_1.member.y)))), _e1340, (fma(_e1636.member.member_3.y, _e1636.member_1.member_1.w, fma(_e1636.member.member_2.y, _e1636.member_1.member_1.z, fma(_e1636.member.member.y, _e1636.member_1.member_1.x, (_e1636.member.member_1.y * _e1636.member_1.member_1.y)))) * _e1341))) + fma(_e1636.member.member_3.y, _e1636.member_1.member_3.w, fma(_e1636.member.member_2.y, _e1636.member_1.member_3.z, fma(_e1636.member.member.y, _e1636.member_1.member_3.x, (_e1636.member.member_1.y * _e1636.member_1.member_3.y))))), (fma(fma(_e1636.member.member_3.z, _e1636.member_1.member_2.w, fma(_e1636.member.member_2.z, _e1636.member_1.member_2.z, fma(_e1636.member.member.z, _e1636.member_1.member_2.x, (_e1636.member.member_1.z * _e1636.member_1.member_2.y)))), _e1342, fma(fma(_e1636.member.member_3.z, _e1636.member_1.member.w, fma(_e1636.member.member_2.z, _e1636.member_1.member.z, fma(_e1636.member.member.z, _e1636.member_1.member.x, (_e1636.member.member_1.z * _e1636.member_1.member.y)))), _e1340, (fma(_e1636.member.member_3.z, _e1636.member_1.member_1.w, fma(_e1636.member.member_2.z, _e1636.member_1.member_1.z, fma(_e1636.member.member.z, _e1636.member_1.member_1.x, (_e1636.member.member_1.z * _e1636.member_1.member_1.y)))) * _e1341))) + fma(_e1636.member.member_3.z, _e1636.member_1.member_3.w, fma(_e1636.member.member_2.z, _e1636.member_1.member_3.z, fma(_e1636.member.member.z, _e1636.member_1.member_3.x, (_e1636.member.member_1.z * _e1636.member_1.member_3.y))))), (fma(fma(_e1636.member.member_3.w, _e1636.member_1.member_2.w, fma(_e1636.member.member_2.w, _e1636.member_1.member_2.z, fma(_e1636.member.member.w, _e1636.member_1.member_2.x, (_e1636.member.member_1.w * _e1636.member_1.member_2.y)))), _e1342, fma(fma(_e1636.member.member_3.w, _e1636.member_1.member.w, fma(_e1636.member.member_2.w, _e1636.member_1.member.z, fma(_e1636.member.member.w, _e1636.member_1.member.x, (_e1636.member.member_1.w * _e1636.member_1.member.y)))), _e1340, (fma(_e1636.member.member_3.w, _e1636.member_1.member_1.w, fma(_e1636.member.member_2.w, _e1636.member_1.member_1.z, fma(_e1636.member.member.w, _e1636.member_1.member_1.x, (_e1636.member.member_1.w * _e1636.member_1.member_1.y)))) * _e1341))) + fma(_e1636.member.member_3.w, _e1636.member_1.member_3.w, fma(_e1636.member.member_2.w, _e1636.member_1.member_3.z, fma(_e1636.member.member.w, _e1636.member_1.member_3.x, (_e1636.member.member_1.w * _e1636.member_1.member_3.y))))));
                                            }
                                            phi_2375_ = false;
                                            phi_903_ = type_24();
                                            phi_906_ = type_31();
                                            break;
                                        }
                                        case 1: {
                                            if (_e338.member_1 >= _e144) {
                                                phi_2235_ = 4294967295u;
                                            } else {
                                                phi_2235_ = (_e140 + (2u * _e338.member_1));
                                            }
                                            let _e1764 = phi_2235_;
                                            if (_e100 >= 2u) {
                                                phi_3766_ = (_e1764 <= (_e100 - 2u));
                                            } else {
                                                phi_3766_ = false;
                                            }
                                            let _e1769 = phi_3766_;
                                            if _e1769 {
                                                let _e1772 = global_2.member[_e1764];
                                                let _e1776 = global_2.member[(_e1764 + 1u)];
                                                phi_2253_ = type_24(_e1772, _e1776);
                                            } else {
                                                phi_2253_ = type_24(4294967295u, 0u);
                                            }
                                            let _e1779 = phi_2253_;
                                            if (_e173 >= _e1779.member_1) {
                                                phi_3792_ = 4294967295u;
                                            } else {
                                                phi_3792_ = (_e1779.member + (9u * _e173));
                                            }
                                            let _e1786 = phi_3792_;
                                            if (_e100 >= 9u) {
                                                phi_3811_ = (_e1786 <= (_e100 - 9u));
                                            } else {
                                                phi_3811_ = false;
                                            }
                                            let _e1791 = phi_3811_;
                                            if _e1791 {
                                                let _e1794 = global_2.member[_e1786];
                                                let _e1799 = global_2.member[(_e1786 + 1u)];
                                                let _e1804 = global_2.member[(_e1786 + 2u)];
                                                let _e1810 = global_2.member[(_e1786 + 3u)];
                                                let _e1815 = global_2.member[(_e1786 + 4u)];
                                                let _e1820 = global_2.member[(_e1786 + 5u)];
                                                let _e1826 = global_2.member[(_e1786 + 6u)];
                                                let _e1831 = global_2.member[(_e1786 + 7u)];
                                                let _e1836 = global_2.member[(_e1786 + 8u)];
                                                phi_2303_ = type_28(vec3<f32>(bitcast<f32>(_e1794), bitcast<f32>(_e1799), bitcast<f32>(_e1804)), vec3<f32>(bitcast<f32>(_e1810), bitcast<f32>(_e1815), bitcast<f32>(_e1820)), vec3<f32>(bitcast<f32>(_e1826), bitcast<f32>(_e1831), bitcast<f32>(_e1836)));
                                            } else {
                                                phi_2303_ = type_28(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                            }
                                            let _e1841 = phi_2303_;
                                            if (_e338.member_1 >= _e152) {
                                                phi_2313_ = 4294967295u;
                                            } else {
                                                phi_2313_ = (_e148 + _e338.member_1);
                                            }
                                            let _e1845 = phi_2313_;
                                            if (_e100 >= 1u) {
                                                phi_3833_ = (_e1845 <= (_e100 - 1u));
                                            } else {
                                                phi_3833_ = false;
                                            }
                                            let _e1850 = phi_3833_;
                                            if _e1850 {
                                                let _e1853 = global_2.member[_e1845];
                                                phi_2321_ = bitcast<f32>(_e1853);
                                            } else {
                                                phi_2321_ = 0f;
                                            }
                                            let _e1856 = phi_2321_;
                                            let _e1879 = type_31(vec3<f32>(fma(_e1856, _e1841.member.x, _e323.member.x), fma(_e1856, _e1841.member.y, _e323.member.y), fma(_e1856, _e1841.member.z, _e323.member.z)), _e323.member_1, _e323.member_2, _e323.member_3, _e323.member_4, _e323.member_5, _e323.member_6, _e323.member_7);
                                            let _e1902 = type_31(_e1879.member, _e1879.member_1, vec3<f32>(fma(_e1856, _e1841.member_1.x, _e323.member_2.x), fma(_e1856, _e1841.member_1.y, _e323.member_2.y), fma(_e1856, _e1841.member_1.z, _e323.member_2.z)), _e1879.member_3, _e1879.member_4, _e1879.member_5, _e1879.member_6, _e1879.member_7);
                                            phi_2375_ = true;
                                            phi_903_ = _e336;
                                            phi_906_ = type_31(_e1902.member, _e1902.member_1, _e1902.member_2, vec4<f32>(fma(_e1856, _e1841.member_2.x, _e323.member_3.x), fma(_e1856, _e1841.member_2.y, _e323.member_3.y), fma(_e1856, _e1841.member_2.z, _e323.member_3.z), _e323.member_3.w), _e1902.member_4, _e1902.member_5, _e1902.member_6, _e1902.member_7);
                                            break;
                                        }
                                        default: {
                                            phi_2375_ = false;
                                            phi_903_ = type_24();
                                            phi_906_ = type_31();
                                            break;
                                        }
                                    }
                                    let _e1929 = phi_2375_;
                                    let _e1931 = phi_903_;
                                    let _e1933 = phi_906_;
                                    continue;
                                    continuing {
                                        phi_902_ = _e1931;
                                        phi_905_ = _e1933;
                                        break if !(_e1929);
                                    }
                                }
                                phi_2383_ = false;
                                phi_873_ = type_24();
                                break;
                            }
                            case 1: {
                                let _e1938 = global_2.member[((_e178 + 22u) + _e313.member_1)];
                                local_4[_e313.member_1] = bitcast<f32>(_e1938);
                                phi_2383_ = true;
                                phi_873_ = _e311;
                                break;
                            }
                            default: {
                                phi_2383_ = false;
                                phi_873_ = type_24();
                                break;
                            }
                        }
                        let _e1942 = phi_2383_;
                        let _e1944 = phi_873_;
                        continue;
                        continuing {
                            phi_872_ = _e1944;
                            break if !(_e1942);
                        }
                    }
                    phi_2390_ = false;
                    phi_843_ = type_24();
                    break;
                }
                case 1: {
                    let _e1949 = global_2.member[((_e178 + 18u) + _e291.member_1)];
                    local_5[_e291.member_1] = _e1949;
                    phi_2390_ = true;
                    phi_843_ = _e289;
                    break;
                }
                default: {
                    phi_2390_ = false;
                    phi_843_ = type_24();
                    break;
                }
            }
            let _e1952 = phi_2390_;
            let _e1954 = phi_843_;
            continue;
            continuing {
                phi_842_ = _e1954;
                break if !(_e1952);
            }
        }
    } else {
        global_1 = vec4<f32>(10f, 10f, 10f, 1f);
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
