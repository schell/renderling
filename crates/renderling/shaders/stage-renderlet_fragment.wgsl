struct type_11 {
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

struct type_29 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_31 {
    member: vec3<f32>,
    member_1: f32,
    member_2: vec4<f32>,
    member_3: f32,
    member_4: f32,
    member_5: u32,
    member_6: u32,
    member_7: u32,
    member_8: u32,
    member_9: u32,
    member_10: u32,
    member_11: u32,
    member_12: u32,
    member_13: u32,
    member_14: u32,
    member_15: bool,
    member_16: f32,
}

struct type_32 {
    member: type_24,
    member_1: vec2<u32>,
    member_2: vec2<u32>,
    member_3: u32,
    member_4: u32,
}

struct type_33 {
    member: u32,
    member_1: u32,
    member_2: u32,
}

struct type_34 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: f32,
}

struct type_35 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
    member_3: f32,
    member_4: vec4<f32>,
    member_5: f32,
}

@group(0) @binding(0) 
var<storage> global: type_11;
var<private> global_1: u32;
var<private> global_2: vec4<f32>;
var<private> global_3: vec2<f32>;
var<private> global_4: vec2<f32>;
var<private> global_5: vec3<f32>;
var<private> global_6: vec3<f32>;
var<private> global_7: vec3<f32>;
var<private> global_8: vec3<f32>;
@group(1) @binding(1) 
var global_9: sampler;
@group(1) @binding(0) 
var global_10: texture_2d_array<f32>;
@group(1) @binding(2) 
var global_11: texture_cube<f32>;
@group(1) @binding(3) 
var global_12: sampler;
@group(1) @binding(4) 
var global_13: texture_cube<f32>;
@group(1) @binding(5) 
var global_14: sampler;
@group(1) @binding(6) 
var global_15: texture_2d<f32>;
@group(1) @binding(7) 
var global_16: sampler;
var<private> global_17: vec4<f32>;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var phi_610_: u32;
    var phi_3823_: bool;
    var phi_764_: type_31;
    var phi_768_: type_31;
    var phi_3856_: bool;
    var phi_808_: u32;
    var phi_817_: u32;
    var phi_830_: type_32;
    var phi_3912_: f32;
    var phi_3894_: bool;
    var phi_884_: f32;
    var phi_879_: f32;
    var phi_885_: f32;
    var phi_3877_: bool;
    var phi_850_: f32;
    var phi_887_: f32;
    var phi_3960_: f32;
    var phi_3942_: bool;
    var phi_942_: f32;
    var phi_937_: f32;
    var phi_943_: f32;
    var phi_3925_: bool;
    var phi_908_: f32;
    var phi_945_: f32;
    var phi_3992_: bool;
    var phi_1028_: u32;
    var phi_1037_: u32;
    var phi_1050_: type_32;
    var phi_4047_: f32;
    var phi_4029_: bool;
    var phi_1104_: f32;
    var phi_1099_: f32;
    var phi_1105_: f32;
    var phi_4012_: bool;
    var phi_1070_: f32;
    var phi_1107_: f32;
    var phi_4095_: f32;
    var phi_4077_: bool;
    var phi_1162_: f32;
    var phi_1157_: f32;
    var phi_1163_: f32;
    var phi_4060_: bool;
    var phi_1128_: f32;
    var phi_1165_: f32;
    var phi_4127_: bool;
    var phi_1248_: u32;
    var phi_1257_: u32;
    var phi_1270_: type_32;
    var phi_4182_: f32;
    var phi_4164_: bool;
    var phi_1324_: f32;
    var phi_1319_: f32;
    var phi_1325_: f32;
    var phi_4147_: bool;
    var phi_1290_: f32;
    var phi_1327_: f32;
    var phi_4230_: f32;
    var phi_4212_: bool;
    var phi_1382_: f32;
    var phi_1377_: f32;
    var phi_1383_: f32;
    var phi_4195_: bool;
    var phi_1348_: f32;
    var phi_1385_: f32;
    var phi_4262_: bool;
    var phi_1468_: u32;
    var phi_1477_: u32;
    var phi_1490_: type_32;
    var phi_4317_: f32;
    var phi_4299_: bool;
    var phi_1544_: f32;
    var phi_1539_: f32;
    var phi_1545_: f32;
    var phi_4282_: bool;
    var phi_1510_: f32;
    var phi_1547_: f32;
    var phi_4365_: f32;
    var phi_4347_: bool;
    var phi_1602_: f32;
    var phi_1597_: f32;
    var phi_1603_: f32;
    var phi_4330_: bool;
    var phi_1568_: f32;
    var phi_1605_: f32;
    var phi_4397_: bool;
    var phi_1688_: u32;
    var phi_1697_: u32;
    var phi_1710_: type_32;
    var phi_4452_: f32;
    var phi_4434_: bool;
    var phi_1764_: f32;
    var phi_1759_: f32;
    var phi_1765_: f32;
    var phi_4417_: bool;
    var phi_1730_: f32;
    var phi_1767_: f32;
    var phi_4500_: f32;
    var phi_4482_: bool;
    var phi_1822_: f32;
    var phi_1817_: f32;
    var phi_1823_: f32;
    var phi_4465_: bool;
    var phi_1788_: f32;
    var phi_1825_: f32;
    var phi_4554_: vec3<f32>;
    var phi_4589_: vec3<f32>;
    var phi_4624_: vec3<f32>;
    var phi_4659_: vec3<f32>;
    var phi_4694_: vec3<f32>;
    var phi_1919_: vec3<f32>;
    var phi_1920_: vec3<f32>;
    var phi_4726_: bool;
    var phi_2127_: type_24;
    var phi_2128_: type_24;
    var phi_2143_: type_24;
    var phi_2170_: bool;
    var phi_2176_: type_24;
    var phi_2177_: type_24;
    var phi_2192_: type_24;
    var phi_2215_: bool;
    var phi_2223_: type_22;
    var phi_4798_: vec3<f32>;
    var phi_4857_: vec3<f32>;
    var phi_4931_: vec3<f32>;
    var phi_6075_: vec3<f32>;
    var phi_6026_: vec3<f32>;
    var phi_5977_: vec3<f32>;
    var phi_5928_: vec3<f32>;
    var phi_5879_: vec3<f32>;
    var phi_5830_: vec3<f32>;
    var phi_5781_: vec3<f32>;
    var phi_4981_: vec3<f32>;
    var phi_5016_: vec3<f32>;
    var phi_2263_: type_24;
    var phi_2266_: vec3<f32>;
    var phi_2264_: type_24;
    var phi_2281_: type_24;
    var phi_5033_: u32;
    var phi_5052_: bool;
    var phi_2298_: u32;
    var phi_5084_: bool;
    var phi_2315_: u32;
    var phi_2325_: type_33;
    var phi_5114_: bool;
    var phi_2375_: type_29;
    var phi_5542_: bool;
    var phi_2875_: type_35;
    var phi_5592_: vec3<f32>;
    var phi_5627_: vec3<f32>;
    var phi_5662_: vec3<f32>;
    var phi_3122_: vec3<f32>;
    var phi_5369_: bool;
    var phi_2626_: type_34;
    var phi_5415_: vec3<f32>;
    var phi_5450_: vec3<f32>;
    var phi_2812_: vec3<f32>;
    var phi_5194_: bool;
    var phi_2423_: type_34;
    var phi_5242_: vec3<f32>;
    var phi_5277_: vec3<f32>;
    var phi_3124_: vec3<f32>;
    var phi_3125_: bool;
    var phi_3134_: vec3<f32>;
    var phi_2267_: vec3<f32>;
    var phi_3136_: bool;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var phi_3249_: vec4<f32>;

    let _e110 = arrayLength((&global.member));
    let _e111 = global_1;
    let _e112 = global_2;
    let _e113 = global_3;
    let _e114 = global_4;
    let _e115 = global_5;
    let _e116 = global_6;
    let _e117 = global_7;
    let _e118 = global_8;
    let _e122 = global.member[(_e111 + 9u)];
    let _e126 = global.member[(_e111 + 11u)];
    let _e130 = global.member[(_e111 + 17u)];
    let _e133 = global.member[_e130];
    let _e137 = global.member[(_e130 + 1u)];
    let _e141 = global.member[(_e130 + 4u)];
    switch bitcast<i32>(_e141) {
        case 0: {
            phi_610_ = 0u;
            break;
        }
        case 1: {
            phi_610_ = 1u;
            break;
        }
        case 2: {
            phi_610_ = 2u;
            break;
        }
        case 3: {
            phi_610_ = 3u;
            break;
        }
        case 4: {
            phi_610_ = 4u;
            break;
        }
        case 5: {
            phi_610_ = 5u;
            break;
        }
        case 6: {
            phi_610_ = 6u;
            break;
        }
        case 7: {
            phi_610_ = 7u;
            break;
        }
        case 8: {
            phi_610_ = 8u;
            break;
        }
        case 9: {
            phi_610_ = 9u;
            break;
        }
        case 10: {
            phi_610_ = 10u;
            break;
        }
        case 11: {
            phi_610_ = 11u;
            break;
        }
        case 12: {
            phi_610_ = 12u;
            break;
        }
        case 13: {
            phi_610_ = 13u;
            break;
        }
        case 14: {
            phi_610_ = 14u;
            break;
        }
        case 15: {
            phi_610_ = 15u;
            break;
        }
        case 16: {
            phi_610_ = 16u;
            break;
        }
        case 17: {
            phi_610_ = 17u;
            break;
        }
        case 18: {
            phi_610_ = 18u;
            break;
        }
        case 19: {
            phi_610_ = 19u;
            break;
        }
        default: {
            phi_610_ = 0u;
            break;
        }
    }
    let _e144 = phi_610_;
    let _e148 = global.member[(_e130 + 5u)];
    let _e153 = global.member[(_e130 + 9u)];
    let _e157 = global.member[(_e130 + 10u)];
    if (_e126 == 4294967295u) {
        phi_768_ = type_31(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e110 >= 22u) {
            phi_3823_ = (_e126 <= (_e110 - 22u));
        } else {
            phi_3823_ = false;
        }
        let _e163 = phi_3823_;
        if _e163 {
            let _e166 = global.member[_e126];
            let _e171 = global.member[(_e126 + 1u)];
            let _e176 = global.member[(_e126 + 2u)];
            let _e182 = global.member[(_e126 + 3u)];
            let _e187 = global.member[(_e126 + 4u)];
            let _e192 = global.member[(_e126 + 5u)];
            let _e197 = global.member[(_e126 + 6u)];
            let _e202 = global.member[(_e126 + 7u)];
            let _e208 = global.member[(_e126 + 8u)];
            let _e213 = global.member[(_e126 + 9u)];
            let _e218 = global.member[(_e126 + 10u)];
            let _e222 = global.member[(_e126 + 11u)];
            let _e226 = global.member[(_e126 + 12u)];
            let _e230 = global.member[(_e126 + 13u)];
            let _e234 = global.member[(_e126 + 14u)];
            let _e238 = global.member[(_e126 + 15u)];
            let _e242 = global.member[(_e126 + 16u)];
            let _e246 = global.member[(_e126 + 17u)];
            let _e250 = global.member[(_e126 + 18u)];
            let _e254 = global.member[(_e126 + 19u)];
            let _e258 = global.member[(_e126 + 20u)];
            let _e263 = global.member[(_e126 + 21u)];
            phi_764_ = type_31(vec3<f32>(bitcast<f32>(_e166), bitcast<f32>(_e171), bitcast<f32>(_e176)), bitcast<f32>(_e182), vec4<f32>(bitcast<f32>(_e187), bitcast<f32>(_e192), bitcast<f32>(_e197), bitcast<f32>(_e202)), bitcast<f32>(_e208), bitcast<f32>(_e213), _e218, _e222, _e226, _e230, _e234, _e238, _e242, _e246, _e250, _e254, (_e258 == 1u), bitcast<f32>(_e263));
        } else {
            phi_764_ = type_31(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e267 = phi_764_;
        phi_768_ = type_31(_e267.member, _e267.member_1, _e267.member_2, _e267.member_3, _e267.member_4, _e267.member_5, _e267.member_6, _e267.member_7, _e267.member_8, _e267.member_9, _e267.member_10, _e267.member_11, _e267.member_12, _e267.member_13, _e267.member_14, (_e267.member_15 && (_e148 == 1u)), _e267.member_16);
    }
    let _e289 = phi_768_;
    let _e293 = select(_e114, _e113, vec2((_e289.member_10 == 0u)));
    let _e295 = (_e110 >= 8u);
    if _e295 {
        phi_3856_ = (_e289.member_5 <= (_e110 - 8u));
    } else {
        phi_3856_ = false;
    }
    let _e299 = phi_3856_;
    if _e299 {
        let _e302 = global.member[_e289.member_5];
        let _e306 = global.member[(_e289.member_5 + 1u)];
        let _e311 = global.member[(_e289.member_5 + 2u)];
        let _e315 = global.member[(_e289.member_5 + 3u)];
        let _e320 = global.member[(_e289.member_5 + 4u)];
        let _e324 = global.member[(_e289.member_5 + 5u)];
        let _e328 = global.member[(_e289.member_5 + 6u)];
        switch bitcast<i32>(_e328) {
            case 0: {
                phi_808_ = 0u;
                break;
            }
            case 1: {
                phi_808_ = 1u;
                break;
            }
            case 2: {
                phi_808_ = 2u;
                break;
            }
            default: {
                phi_808_ = 0u;
                break;
            }
        }
        let _e331 = phi_808_;
        let _e335 = global.member[(_e289.member_5 + 7u)];
        switch bitcast<i32>(_e335) {
            case 0: {
                phi_817_ = 0u;
                break;
            }
            case 1: {
                phi_817_ = 1u;
                break;
            }
            case 2: {
                phi_817_ = 2u;
                break;
            }
            default: {
                phi_817_ = 0u;
                break;
            }
        }
        let _e338 = phi_817_;
        phi_830_ = type_32(type_24(_e331, _e338), vec2<u32>(_e302, _e306), vec2<u32>(_e311, _e315), _e320, _e324);
    } else {
        phi_830_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e342 = phi_830_;
    switch bitcast<i32>(_e342.member.member) {
        case 1: {
            let _e380 = abs(_e293.x);
            let _e382 = (_e380 % 1f);
            if (_e380 >= 1f) {
                phi_3877_ = select(true, false, (_e382 == 0f));
            } else {
                phi_3877_ = true;
            }
            let _e386 = phi_3877_;
            let _e387 = select(1f, _e382, _e386);
            if (select(-1f, 1f, (_e293.x >= 0f)) > 0f) {
                phi_850_ = _e387;
            } else {
                phi_850_ = (1f - _e387);
            }
            let _e391 = phi_850_;
            phi_887_ = _e391;
            break;
        }
        case 2: {
            let _e354 = abs(_e293.x);
            let _e361 = ((select(select(u32(_e354), 0u, (_e354 < 0f)), 4294967295u, (_e354 > 4294967000f)) % 2u) == 0u);
            let _e363 = (_e354 % 1f);
            if (_e354 >= 1f) {
                phi_3894_ = select(true, false, (_e363 == 0f));
            } else {
                phi_3894_ = true;
            }
            let _e367 = phi_3894_;
            let _e368 = select(1f, _e363, _e367);
            if (select(-1f, 1f, (_e293.x >= 0f)) > 0f) {
                if _e361 {
                    phi_879_ = _e368;
                } else {
                    phi_879_ = (1f - _e368);
                }
                let _e375 = phi_879_;
                phi_885_ = _e375;
            } else {
                if _e361 {
                    phi_884_ = (1f - _e368);
                } else {
                    phi_884_ = _e368;
                }
                let _e372 = phi_884_;
                phi_885_ = _e372;
            }
            let _e377 = phi_885_;
            phi_887_ = _e377;
            break;
        }
        case 0: {
            if (_e293.x > 1f) {
                phi_3912_ = 0.9999999f;
            } else {
                phi_3912_ = select(_e293.x, 0.00000011920929f, (_e293.x < 0f));
            }
            let _e351 = phi_3912_;
            phi_887_ = _e351;
            break;
        }
        default: {
            phi_887_ = f32();
            break;
        }
    }
    let _e393 = phi_887_;
    switch bitcast<i32>(_e342.member.member_1) {
        case 1: {
            let _e431 = abs(_e293.y);
            let _e433 = (_e431 % 1f);
            if (_e431 >= 1f) {
                phi_3925_ = select(true, false, (_e433 == 0f));
            } else {
                phi_3925_ = true;
            }
            let _e437 = phi_3925_;
            let _e438 = select(1f, _e433, _e437);
            if (select(-1f, 1f, (_e293.y >= 0f)) > 0f) {
                phi_908_ = _e438;
            } else {
                phi_908_ = (1f - _e438);
            }
            let _e442 = phi_908_;
            phi_945_ = _e442;
            break;
        }
        case 2: {
            let _e405 = abs(_e293.y);
            let _e412 = ((select(select(u32(_e405), 0u, (_e405 < 0f)), 4294967295u, (_e405 > 4294967000f)) % 2u) == 0u);
            let _e414 = (_e405 % 1f);
            if (_e405 >= 1f) {
                phi_3942_ = select(true, false, (_e414 == 0f));
            } else {
                phi_3942_ = true;
            }
            let _e418 = phi_3942_;
            let _e419 = select(1f, _e414, _e418);
            if (select(-1f, 1f, (_e293.y >= 0f)) > 0f) {
                if _e412 {
                    phi_937_ = _e419;
                } else {
                    phi_937_ = (1f - _e419);
                }
                let _e426 = phi_937_;
                phi_943_ = _e426;
            } else {
                if _e412 {
                    phi_942_ = (1f - _e419);
                } else {
                    phi_942_ = _e419;
                }
                let _e423 = phi_942_;
                phi_943_ = _e423;
            }
            let _e428 = phi_943_;
            phi_945_ = _e428;
            break;
        }
        case 0: {
            if (_e293.y > 1f) {
                phi_3960_ = 0.9999999f;
            } else {
                phi_3960_ = select(_e293.y, 0.00000011920929f, (_e293.y < 0f));
            }
            let _e402 = phi_3960_;
            phi_945_ = _e402;
            break;
        }
        default: {
            phi_945_ = f32();
            break;
        }
    }
    let _e444 = phi_945_;
    let _e448 = (_e393 * f32(_e342.member_2.x));
    let _e457 = (_e444 * f32(_e342.member_2.y));
    let _e469 = f32(_e133);
    let _e470 = f32(_e137);
    let _e477 = vec3<f32>((f32((select(select(u32(_e448), 0u, (_e448 < 0f)), 4294967295u, (_e448 > 4294967000f)) + _e342.member_1.x)) / _e469), (f32((select(select(u32(_e457), 0u, (_e457 < 0f)), 4294967295u, (_e457 > 4294967000f)) + _e342.member_1.y)) / _e470), f32(_e342.member_3));
    let _e483 = textureSampleLevel(global_10, global_9, vec2<f32>(_e477.x, _e477.y), i32(_e477.z), 0f);
    let _e486 = select(_e483, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e289.member_5 == 4294967295u)));
    let _e490 = select(_e114, _e113, vec2((_e289.member_11 == 0u)));
    if _e295 {
        phi_3992_ = (_e289.member_6 <= (_e110 - 8u));
    } else {
        phi_3992_ = false;
    }
    let _e495 = phi_3992_;
    if _e495 {
        let _e498 = global.member[_e289.member_6];
        let _e502 = global.member[(_e289.member_6 + 1u)];
        let _e507 = global.member[(_e289.member_6 + 2u)];
        let _e511 = global.member[(_e289.member_6 + 3u)];
        let _e516 = global.member[(_e289.member_6 + 4u)];
        let _e520 = global.member[(_e289.member_6 + 5u)];
        let _e524 = global.member[(_e289.member_6 + 6u)];
        switch bitcast<i32>(_e524) {
            case 0: {
                phi_1028_ = 0u;
                break;
            }
            case 1: {
                phi_1028_ = 1u;
                break;
            }
            case 2: {
                phi_1028_ = 2u;
                break;
            }
            default: {
                phi_1028_ = 0u;
                break;
            }
        }
        let _e527 = phi_1028_;
        let _e531 = global.member[(_e289.member_6 + 7u)];
        switch bitcast<i32>(_e531) {
            case 0: {
                phi_1037_ = 0u;
                break;
            }
            case 1: {
                phi_1037_ = 1u;
                break;
            }
            case 2: {
                phi_1037_ = 2u;
                break;
            }
            default: {
                phi_1037_ = 0u;
                break;
            }
        }
        let _e534 = phi_1037_;
        phi_1050_ = type_32(type_24(_e527, _e534), vec2<u32>(_e498, _e502), vec2<u32>(_e507, _e511), _e516, _e520);
    } else {
        phi_1050_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e538 = phi_1050_;
    switch bitcast<i32>(_e538.member.member) {
        case 1: {
            let _e576 = abs(_e490.x);
            let _e578 = (_e576 % 1f);
            if (_e576 >= 1f) {
                phi_4012_ = select(true, false, (_e578 == 0f));
            } else {
                phi_4012_ = true;
            }
            let _e582 = phi_4012_;
            let _e583 = select(1f, _e578, _e582);
            if (select(-1f, 1f, (_e490.x >= 0f)) > 0f) {
                phi_1070_ = _e583;
            } else {
                phi_1070_ = (1f - _e583);
            }
            let _e587 = phi_1070_;
            phi_1107_ = _e587;
            break;
        }
        case 2: {
            let _e550 = abs(_e490.x);
            let _e557 = ((select(select(u32(_e550), 0u, (_e550 < 0f)), 4294967295u, (_e550 > 4294967000f)) % 2u) == 0u);
            let _e559 = (_e550 % 1f);
            if (_e550 >= 1f) {
                phi_4029_ = select(true, false, (_e559 == 0f));
            } else {
                phi_4029_ = true;
            }
            let _e563 = phi_4029_;
            let _e564 = select(1f, _e559, _e563);
            if (select(-1f, 1f, (_e490.x >= 0f)) > 0f) {
                if _e557 {
                    phi_1099_ = _e564;
                } else {
                    phi_1099_ = (1f - _e564);
                }
                let _e571 = phi_1099_;
                phi_1105_ = _e571;
            } else {
                if _e557 {
                    phi_1104_ = (1f - _e564);
                } else {
                    phi_1104_ = _e564;
                }
                let _e568 = phi_1104_;
                phi_1105_ = _e568;
            }
            let _e573 = phi_1105_;
            phi_1107_ = _e573;
            break;
        }
        case 0: {
            if (_e490.x > 1f) {
                phi_4047_ = 0.9999999f;
            } else {
                phi_4047_ = select(_e490.x, 0.00000011920929f, (_e490.x < 0f));
            }
            let _e547 = phi_4047_;
            phi_1107_ = _e547;
            break;
        }
        default: {
            phi_1107_ = f32();
            break;
        }
    }
    let _e589 = phi_1107_;
    switch bitcast<i32>(_e538.member.member_1) {
        case 1: {
            let _e627 = abs(_e490.y);
            let _e629 = (_e627 % 1f);
            if (_e627 >= 1f) {
                phi_4060_ = select(true, false, (_e629 == 0f));
            } else {
                phi_4060_ = true;
            }
            let _e633 = phi_4060_;
            let _e634 = select(1f, _e629, _e633);
            if (select(-1f, 1f, (_e490.y >= 0f)) > 0f) {
                phi_1128_ = _e634;
            } else {
                phi_1128_ = (1f - _e634);
            }
            let _e638 = phi_1128_;
            phi_1165_ = _e638;
            break;
        }
        case 2: {
            let _e601 = abs(_e490.y);
            let _e608 = ((select(select(u32(_e601), 0u, (_e601 < 0f)), 4294967295u, (_e601 > 4294967000f)) % 2u) == 0u);
            let _e610 = (_e601 % 1f);
            if (_e601 >= 1f) {
                phi_4077_ = select(true, false, (_e610 == 0f));
            } else {
                phi_4077_ = true;
            }
            let _e614 = phi_4077_;
            let _e615 = select(1f, _e610, _e614);
            if (select(-1f, 1f, (_e490.y >= 0f)) > 0f) {
                if _e608 {
                    phi_1157_ = _e615;
                } else {
                    phi_1157_ = (1f - _e615);
                }
                let _e622 = phi_1157_;
                phi_1163_ = _e622;
            } else {
                if _e608 {
                    phi_1162_ = (1f - _e615);
                } else {
                    phi_1162_ = _e615;
                }
                let _e619 = phi_1162_;
                phi_1163_ = _e619;
            }
            let _e624 = phi_1163_;
            phi_1165_ = _e624;
            break;
        }
        case 0: {
            if (_e490.y > 1f) {
                phi_4095_ = 0.9999999f;
            } else {
                phi_4095_ = select(_e490.y, 0.00000011920929f, (_e490.y < 0f));
            }
            let _e598 = phi_4095_;
            phi_1165_ = _e598;
            break;
        }
        default: {
            phi_1165_ = f32();
            break;
        }
    }
    let _e640 = phi_1165_;
    let _e644 = (_e589 * f32(_e538.member_2.x));
    let _e653 = (_e640 * f32(_e538.member_2.y));
    let _e671 = vec3<f32>((f32((select(select(u32(_e644), 0u, (_e644 < 0f)), 4294967295u, (_e644 > 4294967000f)) + _e538.member_1.x)) / _e469), (f32((select(select(u32(_e653), 0u, (_e653 < 0f)), 4294967295u, (_e653 > 4294967000f)) + _e538.member_1.y)) / _e470), f32(_e538.member_3));
    let _e677 = textureSampleLevel(global_10, global_9, vec2<f32>(_e671.x, _e671.y), i32(_e671.z), 0f);
    let _e680 = select(_e677, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e289.member_6 == 4294967295u)));
    let _e684 = select(_e114, _e113, vec2((_e289.member_12 == 0u)));
    if _e295 {
        phi_4127_ = (_e289.member_7 <= (_e110 - 8u));
    } else {
        phi_4127_ = false;
    }
    let _e689 = phi_4127_;
    if _e689 {
        let _e692 = global.member[_e289.member_7];
        let _e696 = global.member[(_e289.member_7 + 1u)];
        let _e701 = global.member[(_e289.member_7 + 2u)];
        let _e705 = global.member[(_e289.member_7 + 3u)];
        let _e710 = global.member[(_e289.member_7 + 4u)];
        let _e714 = global.member[(_e289.member_7 + 5u)];
        let _e718 = global.member[(_e289.member_7 + 6u)];
        switch bitcast<i32>(_e718) {
            case 0: {
                phi_1248_ = 0u;
                break;
            }
            case 1: {
                phi_1248_ = 1u;
                break;
            }
            case 2: {
                phi_1248_ = 2u;
                break;
            }
            default: {
                phi_1248_ = 0u;
                break;
            }
        }
        let _e721 = phi_1248_;
        let _e725 = global.member[(_e289.member_7 + 7u)];
        switch bitcast<i32>(_e725) {
            case 0: {
                phi_1257_ = 0u;
                break;
            }
            case 1: {
                phi_1257_ = 1u;
                break;
            }
            case 2: {
                phi_1257_ = 2u;
                break;
            }
            default: {
                phi_1257_ = 0u;
                break;
            }
        }
        let _e728 = phi_1257_;
        phi_1270_ = type_32(type_24(_e721, _e728), vec2<u32>(_e692, _e696), vec2<u32>(_e701, _e705), _e710, _e714);
    } else {
        phi_1270_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e732 = phi_1270_;
    switch bitcast<i32>(_e732.member.member) {
        case 1: {
            let _e770 = abs(_e684.x);
            let _e772 = (_e770 % 1f);
            if (_e770 >= 1f) {
                phi_4147_ = select(true, false, (_e772 == 0f));
            } else {
                phi_4147_ = true;
            }
            let _e776 = phi_4147_;
            let _e777 = select(1f, _e772, _e776);
            if (select(-1f, 1f, (_e684.x >= 0f)) > 0f) {
                phi_1290_ = _e777;
            } else {
                phi_1290_ = (1f - _e777);
            }
            let _e781 = phi_1290_;
            phi_1327_ = _e781;
            break;
        }
        case 2: {
            let _e744 = abs(_e684.x);
            let _e751 = ((select(select(u32(_e744), 0u, (_e744 < 0f)), 4294967295u, (_e744 > 4294967000f)) % 2u) == 0u);
            let _e753 = (_e744 % 1f);
            if (_e744 >= 1f) {
                phi_4164_ = select(true, false, (_e753 == 0f));
            } else {
                phi_4164_ = true;
            }
            let _e757 = phi_4164_;
            let _e758 = select(1f, _e753, _e757);
            if (select(-1f, 1f, (_e684.x >= 0f)) > 0f) {
                if _e751 {
                    phi_1319_ = _e758;
                } else {
                    phi_1319_ = (1f - _e758);
                }
                let _e765 = phi_1319_;
                phi_1325_ = _e765;
            } else {
                if _e751 {
                    phi_1324_ = (1f - _e758);
                } else {
                    phi_1324_ = _e758;
                }
                let _e762 = phi_1324_;
                phi_1325_ = _e762;
            }
            let _e767 = phi_1325_;
            phi_1327_ = _e767;
            break;
        }
        case 0: {
            if (_e684.x > 1f) {
                phi_4182_ = 0.9999999f;
            } else {
                phi_4182_ = select(_e684.x, 0.00000011920929f, (_e684.x < 0f));
            }
            let _e741 = phi_4182_;
            phi_1327_ = _e741;
            break;
        }
        default: {
            phi_1327_ = f32();
            break;
        }
    }
    let _e783 = phi_1327_;
    switch bitcast<i32>(_e732.member.member_1) {
        case 1: {
            let _e821 = abs(_e684.y);
            let _e823 = (_e821 % 1f);
            if (_e821 >= 1f) {
                phi_4195_ = select(true, false, (_e823 == 0f));
            } else {
                phi_4195_ = true;
            }
            let _e827 = phi_4195_;
            let _e828 = select(1f, _e823, _e827);
            if (select(-1f, 1f, (_e684.y >= 0f)) > 0f) {
                phi_1348_ = _e828;
            } else {
                phi_1348_ = (1f - _e828);
            }
            let _e832 = phi_1348_;
            phi_1385_ = _e832;
            break;
        }
        case 2: {
            let _e795 = abs(_e684.y);
            let _e802 = ((select(select(u32(_e795), 0u, (_e795 < 0f)), 4294967295u, (_e795 > 4294967000f)) % 2u) == 0u);
            let _e804 = (_e795 % 1f);
            if (_e795 >= 1f) {
                phi_4212_ = select(true, false, (_e804 == 0f));
            } else {
                phi_4212_ = true;
            }
            let _e808 = phi_4212_;
            let _e809 = select(1f, _e804, _e808);
            if (select(-1f, 1f, (_e684.y >= 0f)) > 0f) {
                if _e802 {
                    phi_1377_ = _e809;
                } else {
                    phi_1377_ = (1f - _e809);
                }
                let _e816 = phi_1377_;
                phi_1383_ = _e816;
            } else {
                if _e802 {
                    phi_1382_ = (1f - _e809);
                } else {
                    phi_1382_ = _e809;
                }
                let _e813 = phi_1382_;
                phi_1383_ = _e813;
            }
            let _e818 = phi_1383_;
            phi_1385_ = _e818;
            break;
        }
        case 0: {
            if (_e684.y > 1f) {
                phi_4230_ = 0.9999999f;
            } else {
                phi_4230_ = select(_e684.y, 0.00000011920929f, (_e684.y < 0f));
            }
            let _e792 = phi_4230_;
            phi_1385_ = _e792;
            break;
        }
        default: {
            phi_1385_ = f32();
            break;
        }
    }
    let _e834 = phi_1385_;
    let _e838 = (_e783 * f32(_e732.member_2.x));
    let _e847 = (_e834 * f32(_e732.member_2.y));
    let _e865 = vec3<f32>((f32((select(select(u32(_e838), 0u, (_e838 < 0f)), 4294967295u, (_e838 > 4294967000f)) + _e732.member_1.x)) / _e469), (f32((select(select(u32(_e847), 0u, (_e847 < 0f)), 4294967295u, (_e847 > 4294967000f)) + _e732.member_1.y)) / _e470), f32(_e732.member_3));
    let _e871 = textureSampleLevel(global_10, global_9, vec2<f32>(_e865.x, _e865.y), i32(_e865.z), 0f);
    let _e872 = (_e289.member_7 == 4294967295u);
    let _e874 = select(_e871, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e872));
    let _e878 = select(_e114, _e113, vec2((_e289.member_13 == 0u)));
    if _e295 {
        phi_4262_ = (_e289.member_8 <= (_e110 - 8u));
    } else {
        phi_4262_ = false;
    }
    let _e883 = phi_4262_;
    if _e883 {
        let _e886 = global.member[_e289.member_8];
        let _e890 = global.member[(_e289.member_8 + 1u)];
        let _e895 = global.member[(_e289.member_8 + 2u)];
        let _e899 = global.member[(_e289.member_8 + 3u)];
        let _e904 = global.member[(_e289.member_8 + 4u)];
        let _e908 = global.member[(_e289.member_8 + 5u)];
        let _e912 = global.member[(_e289.member_8 + 6u)];
        switch bitcast<i32>(_e912) {
            case 0: {
                phi_1468_ = 0u;
                break;
            }
            case 1: {
                phi_1468_ = 1u;
                break;
            }
            case 2: {
                phi_1468_ = 2u;
                break;
            }
            default: {
                phi_1468_ = 0u;
                break;
            }
        }
        let _e915 = phi_1468_;
        let _e919 = global.member[(_e289.member_8 + 7u)];
        switch bitcast<i32>(_e919) {
            case 0: {
                phi_1477_ = 0u;
                break;
            }
            case 1: {
                phi_1477_ = 1u;
                break;
            }
            case 2: {
                phi_1477_ = 2u;
                break;
            }
            default: {
                phi_1477_ = 0u;
                break;
            }
        }
        let _e922 = phi_1477_;
        phi_1490_ = type_32(type_24(_e915, _e922), vec2<u32>(_e886, _e890), vec2<u32>(_e895, _e899), _e904, _e908);
    } else {
        phi_1490_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e926 = phi_1490_;
    switch bitcast<i32>(_e926.member.member) {
        case 1: {
            let _e964 = abs(_e878.x);
            let _e966 = (_e964 % 1f);
            if (_e964 >= 1f) {
                phi_4282_ = select(true, false, (_e966 == 0f));
            } else {
                phi_4282_ = true;
            }
            let _e970 = phi_4282_;
            let _e971 = select(1f, _e966, _e970);
            if (select(-1f, 1f, (_e878.x >= 0f)) > 0f) {
                phi_1510_ = _e971;
            } else {
                phi_1510_ = (1f - _e971);
            }
            let _e975 = phi_1510_;
            phi_1547_ = _e975;
            break;
        }
        case 2: {
            let _e938 = abs(_e878.x);
            let _e945 = ((select(select(u32(_e938), 0u, (_e938 < 0f)), 4294967295u, (_e938 > 4294967000f)) % 2u) == 0u);
            let _e947 = (_e938 % 1f);
            if (_e938 >= 1f) {
                phi_4299_ = select(true, false, (_e947 == 0f));
            } else {
                phi_4299_ = true;
            }
            let _e951 = phi_4299_;
            let _e952 = select(1f, _e947, _e951);
            if (select(-1f, 1f, (_e878.x >= 0f)) > 0f) {
                if _e945 {
                    phi_1539_ = _e952;
                } else {
                    phi_1539_ = (1f - _e952);
                }
                let _e959 = phi_1539_;
                phi_1545_ = _e959;
            } else {
                if _e945 {
                    phi_1544_ = (1f - _e952);
                } else {
                    phi_1544_ = _e952;
                }
                let _e956 = phi_1544_;
                phi_1545_ = _e956;
            }
            let _e961 = phi_1545_;
            phi_1547_ = _e961;
            break;
        }
        case 0: {
            if (_e878.x > 1f) {
                phi_4317_ = 0.9999999f;
            } else {
                phi_4317_ = select(_e878.x, 0.00000011920929f, (_e878.x < 0f));
            }
            let _e935 = phi_4317_;
            phi_1547_ = _e935;
            break;
        }
        default: {
            phi_1547_ = f32();
            break;
        }
    }
    let _e977 = phi_1547_;
    switch bitcast<i32>(_e926.member.member_1) {
        case 1: {
            let _e1015 = abs(_e878.y);
            let _e1017 = (_e1015 % 1f);
            if (_e1015 >= 1f) {
                phi_4330_ = select(true, false, (_e1017 == 0f));
            } else {
                phi_4330_ = true;
            }
            let _e1021 = phi_4330_;
            let _e1022 = select(1f, _e1017, _e1021);
            if (select(-1f, 1f, (_e878.y >= 0f)) > 0f) {
                phi_1568_ = _e1022;
            } else {
                phi_1568_ = (1f - _e1022);
            }
            let _e1026 = phi_1568_;
            phi_1605_ = _e1026;
            break;
        }
        case 2: {
            let _e989 = abs(_e878.y);
            let _e996 = ((select(select(u32(_e989), 0u, (_e989 < 0f)), 4294967295u, (_e989 > 4294967000f)) % 2u) == 0u);
            let _e998 = (_e989 % 1f);
            if (_e989 >= 1f) {
                phi_4347_ = select(true, false, (_e998 == 0f));
            } else {
                phi_4347_ = true;
            }
            let _e1002 = phi_4347_;
            let _e1003 = select(1f, _e998, _e1002);
            if (select(-1f, 1f, (_e878.y >= 0f)) > 0f) {
                if _e996 {
                    phi_1597_ = _e1003;
                } else {
                    phi_1597_ = (1f - _e1003);
                }
                let _e1010 = phi_1597_;
                phi_1603_ = _e1010;
            } else {
                if _e996 {
                    phi_1602_ = (1f - _e1003);
                } else {
                    phi_1602_ = _e1003;
                }
                let _e1007 = phi_1602_;
                phi_1603_ = _e1007;
            }
            let _e1012 = phi_1603_;
            phi_1605_ = _e1012;
            break;
        }
        case 0: {
            if (_e878.y > 1f) {
                phi_4365_ = 0.9999999f;
            } else {
                phi_4365_ = select(_e878.y, 0.00000011920929f, (_e878.y < 0f));
            }
            let _e986 = phi_4365_;
            phi_1605_ = _e986;
            break;
        }
        default: {
            phi_1605_ = f32();
            break;
        }
    }
    let _e1028 = phi_1605_;
    let _e1032 = (_e977 * f32(_e926.member_2.x));
    let _e1041 = (_e1028 * f32(_e926.member_2.y));
    let _e1059 = vec3<f32>((f32((select(select(u32(_e1032), 0u, (_e1032 < 0f)), 4294967295u, (_e1032 > 4294967000f)) + _e926.member_1.x)) / _e469), (f32((select(select(u32(_e1041), 0u, (_e1041 < 0f)), 4294967295u, (_e1041 > 4294967000f)) + _e926.member_1.y)) / _e470), f32(_e926.member_3));
    let _e1065 = textureSampleLevel(global_10, global_9, vec2<f32>(_e1059.x, _e1059.y), i32(_e1059.z), 0f);
    let _e1072 = select(_e114, _e113, vec2((_e289.member_14 == 0u)));
    if _e295 {
        phi_4397_ = (_e289.member_9 <= (_e110 - 8u));
    } else {
        phi_4397_ = false;
    }
    let _e1077 = phi_4397_;
    if _e1077 {
        let _e1080 = global.member[_e289.member_9];
        let _e1084 = global.member[(_e289.member_9 + 1u)];
        let _e1089 = global.member[(_e289.member_9 + 2u)];
        let _e1093 = global.member[(_e289.member_9 + 3u)];
        let _e1098 = global.member[(_e289.member_9 + 4u)];
        let _e1102 = global.member[(_e289.member_9 + 5u)];
        let _e1106 = global.member[(_e289.member_9 + 6u)];
        switch bitcast<i32>(_e1106) {
            case 0: {
                phi_1688_ = 0u;
                break;
            }
            case 1: {
                phi_1688_ = 1u;
                break;
            }
            case 2: {
                phi_1688_ = 2u;
                break;
            }
            default: {
                phi_1688_ = 0u;
                break;
            }
        }
        let _e1109 = phi_1688_;
        let _e1113 = global.member[(_e289.member_9 + 7u)];
        switch bitcast<i32>(_e1113) {
            case 0: {
                phi_1697_ = 0u;
                break;
            }
            case 1: {
                phi_1697_ = 1u;
                break;
            }
            case 2: {
                phi_1697_ = 2u;
                break;
            }
            default: {
                phi_1697_ = 0u;
                break;
            }
        }
        let _e1116 = phi_1697_;
        phi_1710_ = type_32(type_24(_e1109, _e1116), vec2<u32>(_e1080, _e1084), vec2<u32>(_e1089, _e1093), _e1098, _e1102);
    } else {
        phi_1710_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1120 = phi_1710_;
    switch bitcast<i32>(_e1120.member.member) {
        case 1: {
            let _e1158 = abs(_e1072.x);
            let _e1160 = (_e1158 % 1f);
            if (_e1158 >= 1f) {
                phi_4417_ = select(true, false, (_e1160 == 0f));
            } else {
                phi_4417_ = true;
            }
            let _e1164 = phi_4417_;
            let _e1165 = select(1f, _e1160, _e1164);
            if (select(-1f, 1f, (_e1072.x >= 0f)) > 0f) {
                phi_1730_ = _e1165;
            } else {
                phi_1730_ = (1f - _e1165);
            }
            let _e1169 = phi_1730_;
            phi_1767_ = _e1169;
            break;
        }
        case 2: {
            let _e1132 = abs(_e1072.x);
            let _e1139 = ((select(select(u32(_e1132), 0u, (_e1132 < 0f)), 4294967295u, (_e1132 > 4294967000f)) % 2u) == 0u);
            let _e1141 = (_e1132 % 1f);
            if (_e1132 >= 1f) {
                phi_4434_ = select(true, false, (_e1141 == 0f));
            } else {
                phi_4434_ = true;
            }
            let _e1145 = phi_4434_;
            let _e1146 = select(1f, _e1141, _e1145);
            if (select(-1f, 1f, (_e1072.x >= 0f)) > 0f) {
                if _e1139 {
                    phi_1759_ = _e1146;
                } else {
                    phi_1759_ = (1f - _e1146);
                }
                let _e1153 = phi_1759_;
                phi_1765_ = _e1153;
            } else {
                if _e1139 {
                    phi_1764_ = (1f - _e1146);
                } else {
                    phi_1764_ = _e1146;
                }
                let _e1150 = phi_1764_;
                phi_1765_ = _e1150;
            }
            let _e1155 = phi_1765_;
            phi_1767_ = _e1155;
            break;
        }
        case 0: {
            if (_e1072.x > 1f) {
                phi_4452_ = 0.9999999f;
            } else {
                phi_4452_ = select(_e1072.x, 0.00000011920929f, (_e1072.x < 0f));
            }
            let _e1129 = phi_4452_;
            phi_1767_ = _e1129;
            break;
        }
        default: {
            phi_1767_ = f32();
            break;
        }
    }
    let _e1171 = phi_1767_;
    switch bitcast<i32>(_e1120.member.member_1) {
        case 1: {
            let _e1209 = abs(_e1072.y);
            let _e1211 = (_e1209 % 1f);
            if (_e1209 >= 1f) {
                phi_4465_ = select(true, false, (_e1211 == 0f));
            } else {
                phi_4465_ = true;
            }
            let _e1215 = phi_4465_;
            let _e1216 = select(1f, _e1211, _e1215);
            if (select(-1f, 1f, (_e1072.y >= 0f)) > 0f) {
                phi_1788_ = _e1216;
            } else {
                phi_1788_ = (1f - _e1216);
            }
            let _e1220 = phi_1788_;
            phi_1825_ = _e1220;
            break;
        }
        case 2: {
            let _e1183 = abs(_e1072.y);
            let _e1190 = ((select(select(u32(_e1183), 0u, (_e1183 < 0f)), 4294967295u, (_e1183 > 4294967000f)) % 2u) == 0u);
            let _e1192 = (_e1183 % 1f);
            if (_e1183 >= 1f) {
                phi_4482_ = select(true, false, (_e1192 == 0f));
            } else {
                phi_4482_ = true;
            }
            let _e1196 = phi_4482_;
            let _e1197 = select(1f, _e1192, _e1196);
            if (select(-1f, 1f, (_e1072.y >= 0f)) > 0f) {
                if _e1190 {
                    phi_1817_ = _e1197;
                } else {
                    phi_1817_ = (1f - _e1197);
                }
                let _e1204 = phi_1817_;
                phi_1823_ = _e1204;
            } else {
                if _e1190 {
                    phi_1822_ = (1f - _e1197);
                } else {
                    phi_1822_ = _e1197;
                }
                let _e1201 = phi_1822_;
                phi_1823_ = _e1201;
            }
            let _e1206 = phi_1823_;
            phi_1825_ = _e1206;
            break;
        }
        case 0: {
            if (_e1072.y > 1f) {
                phi_4500_ = 0.9999999f;
            } else {
                phi_4500_ = select(_e1072.y, 0.00000011920929f, (_e1072.y < 0f));
            }
            let _e1180 = phi_4500_;
            phi_1825_ = _e1180;
            break;
        }
        default: {
            phi_1825_ = f32();
            break;
        }
    }
    let _e1222 = phi_1825_;
    let _e1226 = (_e1171 * f32(_e1120.member_2.x));
    let _e1235 = (_e1222 * f32(_e1120.member_2.y));
    let _e1253 = vec3<f32>((f32((select(select(u32(_e1226), 0u, (_e1226 < 0f)), 4294967295u, (_e1226 > 4294967000f)) + _e1120.member_1.x)) / _e469), (f32((select(select(u32(_e1235), 0u, (_e1235 < 0f)), 4294967295u, (_e1235 > 4294967000f)) + _e1120.member_1.y)) / _e470), f32(_e1120.member_3));
    let _e1259 = textureSampleLevel(global_10, global_9, vec2<f32>(_e1253.x, _e1253.y), i32(_e1253.z), 0f);
    let _e1262 = select(_e1259, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e289.member_9 == 4294967295u)));
    if _e872 {
        phi_1919_ = vec3<f32>(0f, 0f, 0f);
        phi_1920_ = _e115;
    } else {
        let _e1266 = fma(_e874.x, 2f, -1f);
        let _e1267 = fma(_e874.y, 2f, -1f);
        let _e1268 = fma(_e874.z, 2f, -1f);
        let _e1273 = sqrt(fma(_e1268, _e1268, fma(_e1266, _e1266, (_e1267 * _e1267))));
        if (_e1273 == 0f) {
            phi_4554_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4554_ = (vec3<f32>(_e1266, _e1267, _e1268) * (1f / _e1273));
        }
        let _e1278 = phi_4554_;
        let _e1285 = sqrt(fma(_e116.z, _e116.z, fma(_e116.x, _e116.x, (_e116.y * _e116.y))));
        if (_e1285 == 0f) {
            phi_4589_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4589_ = (_e116 * (1f / _e1285));
        }
        let _e1290 = phi_4589_;
        let _e1297 = sqrt(fma(_e117.z, _e117.z, fma(_e117.x, _e117.x, (_e117.y * _e117.y))));
        if (_e1297 == 0f) {
            phi_4624_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4624_ = (_e117 * (1f / _e1297));
        }
        let _e1302 = phi_4624_;
        let _e1309 = sqrt(fma(_e115.z, _e115.z, fma(_e115.x, _e115.x, (_e115.y * _e115.y))));
        if (_e1309 == 0f) {
            phi_4659_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4659_ = (_e115 * (1f / _e1309));
        }
        let _e1314 = phi_4659_;
        let _e1333 = fma(_e1314.x, _e1278.z, fma(_e1290.x, _e1278.x, (_e1302.x * _e1278.y)));
        let _e1334 = fma(_e1314.y, _e1278.z, fma(_e1290.y, _e1278.x, (_e1302.y * _e1278.y)));
        let _e1335 = fma(_e1314.z, _e1278.z, fma(_e1290.z, _e1278.x, (_e1302.z * _e1278.y)));
        let _e1340 = sqrt(fma(_e1335, _e1335, fma(_e1333, _e1333, (_e1334 * _e1334))));
        if (_e1340 == 0f) {
            phi_4694_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4694_ = (vec3<f32>(_e1333, _e1334, _e1335) * (1f / _e1340));
        }
        let _e1345 = phi_4694_;
        phi_1919_ = _e1278;
        phi_1920_ = _e1345;
    }
    let _e1347 = phi_1919_;
    let _e1349 = phi_1920_;
    let _e1353 = (_e486.x * _e289.member_2.x);
    let _e1356 = (_e486.y * _e289.member_2.y);
    let _e1359 = (_e486.z * _e289.member_2.z);
    let _e1364 = (_e1353 * _e112.x);
    let _e1366 = (_e1356 * _e112.y);
    let _e1368 = (_e1359 * _e112.z);
    let _e1373 = (_e680.y * _e289.member_4);
    let _e1376 = (_e680.z * _e289.member_3);
    let _e1380 = fma(_e289.member_16, (select(_e1065, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e289.member_8 == 4294967295u))).x - 1f), 1f);
    let _e1386 = (_e1262.x * _e289.member.x);
    let _e1388 = (_e1262.y * _e289.member.y);
    let _e1390 = (_e1262.z * _e289.member.z);
    let _e1395 = textureSampleLevel(global_11, global_12, _e1349, 0f);
    if (_e110 >= 83u) {
        phi_4726_ = (_e122 <= (_e110 - 83u));
    } else {
        phi_4726_ = false;
    }
    let _e1403 = phi_4726_;
    if _e1403 {
        let _e1406 = global.member[_e122];
        let _e1411 = global.member[(_e122 + 1u)];
        let _e1416 = global.member[(_e122 + 2u)];
        let _e1421 = global.member[(_e122 + 3u)];
        let _e1427 = global.member[(_e122 + 4u)];
        let _e1432 = global.member[(_e122 + 5u)];
        let _e1437 = global.member[(_e122 + 6u)];
        let _e1442 = global.member[(_e122 + 7u)];
        let _e1448 = global.member[(_e122 + 8u)];
        let _e1453 = global.member[(_e122 + 9u)];
        let _e1458 = global.member[(_e122 + 10u)];
        let _e1463 = global.member[(_e122 + 11u)];
        let _e1469 = global.member[(_e122 + 12u)];
        let _e1474 = global.member[(_e122 + 13u)];
        let _e1479 = global.member[(_e122 + 14u)];
        let _e1484 = global.member[(_e122 + 15u)];
        let _e1491 = global.member[(_e122 + 16u)];
        let _e1496 = global.member[(_e122 + 17u)];
        let _e1501 = global.member[(_e122 + 18u)];
        let _e1506 = global.member[(_e122 + 19u)];
        let _e1512 = global.member[(_e122 + 20u)];
        let _e1517 = global.member[(_e122 + 21u)];
        let _e1522 = global.member[(_e122 + 22u)];
        let _e1527 = global.member[(_e122 + 23u)];
        let _e1533 = global.member[(_e122 + 24u)];
        let _e1538 = global.member[(_e122 + 25u)];
        let _e1543 = global.member[(_e122 + 26u)];
        let _e1548 = global.member[(_e122 + 27u)];
        let _e1554 = global.member[(_e122 + 28u)];
        let _e1559 = global.member[(_e122 + 29u)];
        let _e1564 = global.member[(_e122 + 30u)];
        let _e1569 = global.member[(_e122 + 31u)];
        let _e1576 = global.member[(_e122 + 32u)];
        let _e1581 = global.member[(_e122 + 33u)];
        let _e1586 = global.member[(_e122 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_2127_ = type_24(0u, 6u);
        loop {
            let _e1591 = phi_2127_;
            if (_e1591.member < _e1591.member_1) {
                phi_2128_ = type_24((_e1591.member + 1u), _e1591.member_1);
                phi_2143_ = type_24(1u, _e1591.member);
            } else {
                phi_2128_ = _e1591;
                phi_2143_ = type_24(0u, type_24().member_1);
            }
            let _e1604 = phi_2128_;
            let _e1606 = phi_2143_;
            switch bitcast<i32>(_e1606.member) {
                case 0: {
                    phi_2170_ = false;
                    break;
                }
                case 1: {
                    let _e1611 = ((_e122 + 35u) + (_e1606.member_1 * 4u));
                    let _e1614 = global.member[_e1611];
                    let _e1619 = global.member[(_e1611 + 1u)];
                    let _e1624 = global.member[(_e1611 + 2u)];
                    let _e1629 = global.member[(_e1611 + 3u)];
                    local_1[_e1606.member_1] = vec4<f32>(bitcast<f32>(_e1614), bitcast<f32>(_e1619), bitcast<f32>(_e1624), bitcast<f32>(_e1629));
                    phi_2170_ = true;
                    break;
                }
                default: {
                    phi_2170_ = bool();
                    break;
                }
            }
            let _e1634 = phi_2170_;
            continue;
            continuing {
                phi_2127_ = _e1604;
                break if !(_e1634);
            }
        }
        let _e1636 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2176_ = type_24(0u, 8u);
        loop {
            let _e1639 = phi_2176_;
            if (_e1639.member < _e1639.member_1) {
                phi_2177_ = type_24((_e1639.member + 1u), _e1639.member_1);
                phi_2192_ = type_24(1u, _e1639.member);
            } else {
                phi_2177_ = _e1639;
                phi_2192_ = type_24(0u, type_24().member_1);
            }
            let _e1652 = phi_2177_;
            let _e1654 = phi_2192_;
            switch bitcast<i32>(_e1654.member) {
                case 0: {
                    phi_2215_ = false;
                    break;
                }
                case 1: {
                    let _e1659 = ((_e122 + 59u) + (_e1654.member_1 * 3u));
                    let _e1662 = global.member[_e1659];
                    let _e1667 = global.member[(_e1659 + 1u)];
                    let _e1672 = global.member[(_e1659 + 2u)];
                    local[_e1654.member_1] = vec3<f32>(bitcast<f32>(_e1662), bitcast<f32>(_e1667), bitcast<f32>(_e1672));
                    phi_2215_ = true;
                    break;
                }
                default: {
                    phi_2215_ = bool();
                    break;
                }
            }
            let _e1677 = phi_2215_;
            continue;
            continuing {
                phi_2176_ = _e1652;
                break if !(_e1677);
            }
        }
        let _e1679 = local;
        phi_2223_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1406), bitcast<f32>(_e1411), bitcast<f32>(_e1416), bitcast<f32>(_e1421)), vec4<f32>(bitcast<f32>(_e1427), bitcast<f32>(_e1432), bitcast<f32>(_e1437), bitcast<f32>(_e1442)), vec4<f32>(bitcast<f32>(_e1448), bitcast<f32>(_e1453), bitcast<f32>(_e1458), bitcast<f32>(_e1463)), vec4<f32>(bitcast<f32>(_e1469), bitcast<f32>(_e1474), bitcast<f32>(_e1479), bitcast<f32>(_e1484))), type_20(vec4<f32>(bitcast<f32>(_e1491), bitcast<f32>(_e1496), bitcast<f32>(_e1501), bitcast<f32>(_e1506)), vec4<f32>(bitcast<f32>(_e1512), bitcast<f32>(_e1517), bitcast<f32>(_e1522), bitcast<f32>(_e1527)), vec4<f32>(bitcast<f32>(_e1533), bitcast<f32>(_e1538), bitcast<f32>(_e1543), bitcast<f32>(_e1548)), vec4<f32>(bitcast<f32>(_e1554), bitcast<f32>(_e1559), bitcast<f32>(_e1564), bitcast<f32>(_e1569))), type_21(_e1679, _e1636), vec3<f32>(bitcast<f32>(_e1576), bitcast<f32>(_e1581), bitcast<f32>(_e1586)));
    } else {
        phi_2223_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
    }
    let _e1683 = phi_2223_;
    let _e1685 = (_e1683.member_3 - _e118);
    let _e1692 = sqrt(fma(_e1685.z, _e1685.z, fma(_e1685.x, _e1685.x, (_e1685.y * _e1685.y))));
    let _e1693 = (_e1692 == 0f);
    if _e1693 {
        phi_4798_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4798_ = (_e1685 * (1f / _e1692));
    }
    let _e1697 = phi_4798_;
    let _e1698 = -(_e1697);
    let _e1705 = sqrt(fma(_e1349.z, _e1349.z, fma(_e1349.x, _e1349.x, (_e1349.y * _e1349.y))));
    let _e1706 = (_e1705 == 0f);
    if _e1706 {
        phi_4857_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4857_ = (_e1349 * (1f / _e1705));
    }
    let _e1710 = phi_4857_;
    let _e1720 = (2f * fma(_e1710.z, _e1698.z, fma(_e1710.x, _e1698.x, (_e1710.y * _e1698.y))));
    let _e1727 = textureSampleLevel(global_13, global_14, (_e1698 - vec3<f32>((_e1720 * _e1710.x), (_e1720 * _e1710.y), (_e1720 * _e1710.z))), (_e1373 * 4f));
    if _e1693 {
        phi_4931_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4931_ = (_e1685 * (1f / _e1692));
    }
    let _e1734 = phi_4931_;
    let _e1743 = textureSampleLevel(global_15, global_16, vec2<f32>(max(fma(_e1349.z, _e1734.z, fma(_e1349.x, _e1734.x, (_e1349.y * _e1734.y))), 0f), _e1373), 0f);
    switch bitcast<i32>(_e144) {
        case 0: {
            if _e289.member_15 {
                if _e1706 {
                    phi_4981_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_4981_ = (_e1349 * (1f / _e1705));
                }
                let _e1912 = phi_4981_;
                if _e1693 {
                    phi_5016_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5016_ = (_e1685 * (1f / _e1692));
                }
                let _e1916 = phi_5016_;
                phi_2263_ = type_24(0u, _e157);
                phi_2266_ = vec3<f32>(0f, 0f, 0f);
                loop {
                    let _e1919 = phi_2263_;
                    let _e1921 = phi_2266_;
                    local_2 = _e1921;
                    local_3 = _e1921;
                    local_4 = _e1921;
                    if (_e1919.member < _e1919.member_1) {
                        phi_2264_ = type_24((_e1919.member + 1u), _e1919.member_1);
                        phi_2281_ = type_24(1u, _e1919.member);
                    } else {
                        phi_2264_ = _e1919;
                        phi_2281_ = type_24(0u, type_24().member_1);
                    }
                    let _e1934 = phi_2264_;
                    let _e1936 = phi_2281_;
                    switch bitcast<i32>(_e1936.member) {
                        case 0: {
                            phi_2267_ = vec3<f32>();
                            phi_3136_ = false;
                            break;
                        }
                        case 1: {
                            if (_e1936.member_1 >= _e157) {
                                phi_5033_ = 4294967295u;
                            } else {
                                phi_5033_ = (_e153 + _e1936.member_1);
                            }
                            let _e1943 = phi_5033_;
                            if (_e110 >= 1u) {
                                phi_5052_ = (_e1943 <= (_e110 - 1u));
                            } else {
                                phi_5052_ = false;
                            }
                            let _e1948 = phi_5052_;
                            if _e1948 {
                                let _e1951 = global.member[_e1943];
                                phi_2298_ = _e1951;
                            } else {
                                phi_2298_ = 4294967295u;
                            }
                            let _e1953 = phi_2298_;
                            let _e1954 = (_e1953 == 4294967295u);
                            if _e1954 {
                                phi_3134_ = vec3<f32>();
                            } else {
                                if (_e110 >= 3u) {
                                    phi_5084_ = (_e1953 <= (_e110 - 3u));
                                } else {
                                    phi_5084_ = false;
                                }
                                let _e1959 = phi_5084_;
                                if _e1959 {
                                    let _e1962 = global.member[_e1953];
                                    switch bitcast<i32>(_e1962) {
                                        case 0: {
                                            phi_2315_ = 0u;
                                            break;
                                        }
                                        case 1: {
                                            phi_2315_ = 1u;
                                            break;
                                        }
                                        case 2: {
                                            phi_2315_ = 2u;
                                            break;
                                        }
                                        default: {
                                            phi_2315_ = 0u;
                                            break;
                                        }
                                    }
                                    let _e1965 = phi_2315_;
                                    let _e1969 = global.member[(_e1953 + 1u)];
                                    let _e1973 = global.member[(_e1953 + 2u)];
                                    phi_2325_ = type_33(_e1965, _e1969, _e1973);
                                } else {
                                    phi_2325_ = type_33(0u, 4294967295u, 4294967295u);
                                }
                                let _e1976 = phi_2325_;
                                if (_e110 >= 10u) {
                                    phi_5114_ = (_e1976.member_2 <= (_e110 - 10u));
                                } else {
                                    phi_5114_ = false;
                                }
                                let _e1982 = phi_5114_;
                                if _e1982 {
                                    let _e1985 = global.member[_e1976.member_2];
                                    let _e1990 = global.member[(_e1976.member_2 + 1u)];
                                    let _e1995 = global.member[(_e1976.member_2 + 2u)];
                                    let _e2001 = global.member[(_e1976.member_2 + 3u)];
                                    let _e2006 = global.member[(_e1976.member_2 + 4u)];
                                    let _e2011 = global.member[(_e1976.member_2 + 5u)];
                                    let _e2016 = global.member[(_e1976.member_2 + 6u)];
                                    let _e2022 = global.member[(_e1976.member_2 + 7u)];
                                    let _e2027 = global.member[(_e1976.member_2 + 8u)];
                                    let _e2032 = global.member[(_e1976.member_2 + 9u)];
                                    phi_2375_ = type_29(vec3<f32>(bitcast<f32>(_e1985), bitcast<f32>(_e1990), bitcast<f32>(_e1995)), vec4<f32>(bitcast<f32>(_e2001), bitcast<f32>(_e2006), bitcast<f32>(_e2011), bitcast<f32>(_e2016)), vec3<f32>(bitcast<f32>(_e2022), bitcast<f32>(_e2027), bitcast<f32>(_e2032)));
                                } else {
                                    phi_2375_ = type_29(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e2037 = phi_2375_;
                                let _e2045 = (_e2037.member_1.x + _e2037.member_1.x);
                                let _e2046 = (_e2037.member_1.y + _e2037.member_1.y);
                                let _e2047 = (_e2037.member_1.z + _e2037.member_1.z);
                                let _e2049 = (_e2037.member_1.z * _e2047);
                                let _e2050 = (_e2037.member_1.w * _e2045);
                                let _e2051 = (_e2037.member_1.w * _e2046);
                                let _e2052 = (_e2037.member_1.w * _e2047);
                                let _e2072 = (vec4<f32>((1f - fma(_e2037.member_1.y, _e2046, _e2049)), fma(_e2037.member_1.x, _e2046, _e2052), fma(_e2037.member_1.x, _e2047, -(_e2051)), 0f) * _e2037.member_2.x);
                                let _e2074 = (vec4<f32>(fma(_e2037.member_1.x, _e2046, -(_e2052)), (1f - fma(_e2037.member_1.x, _e2045, _e2049)), fma(_e2037.member_1.y, _e2047, _e2050), 0f) * _e2037.member_2.y);
                                let _e2076 = (vec4<f32>(fma(_e2037.member_1.x, _e2047, _e2051), fma(_e2037.member_1.y, _e2047, -(_e2050)), (1f - fma(_e2037.member_1.x, _e2045, (_e2037.member_1.y * _e2046))), 0f) * _e2037.member_2.z);
                                switch bitcast<i32>(_e1976.member) {
                                    case 0: {
                                        if _e295 {
                                            phi_5194_ = (_e1976.member_1 <= (_e110 - 8u));
                                        } else {
                                            phi_5194_ = false;
                                        }
                                        let _e2572 = phi_5194_;
                                        if _e2572 {
                                            let _e2575 = global.member[_e1976.member_1];
                                            let _e2580 = global.member[(_e1976.member_1 + 1u)];
                                            let _e2585 = global.member[(_e1976.member_1 + 2u)];
                                            let _e2591 = global.member[(_e1976.member_1 + 3u)];
                                            let _e2596 = global.member[(_e1976.member_1 + 4u)];
                                            let _e2601 = global.member[(_e1976.member_1 + 5u)];
                                            let _e2606 = global.member[(_e1976.member_1 + 6u)];
                                            let _e2612 = global.member[(_e1976.member_1 + 7u)];
                                            phi_2423_ = type_34(vec3<f32>(bitcast<f32>(_e2575), bitcast<f32>(_e2580), bitcast<f32>(_e2585)), vec4<f32>(bitcast<f32>(_e2591), bitcast<f32>(_e2596), bitcast<f32>(_e2601), bitcast<f32>(_e2606)), bitcast<f32>(_e2612));
                                        } else {
                                            phi_2423_ = type_34(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2616 = phi_2423_;
                                        let _e2638 = fma(_e2076.x, _e2616.member.z, fma(_e2074.x, _e2616.member.y, (_e2072.x * _e2616.member.x)));
                                        let _e2639 = fma(_e2076.y, _e2616.member.z, fma(_e2074.y, _e2616.member.y, (_e2072.y * _e2616.member.x)));
                                        let _e2640 = fma(_e2076.z, _e2616.member.z, fma(_e2074.z, _e2616.member.y, (_e2072.z * _e2616.member.x)));
                                        let _e2645 = sqrt(fma(_e2640, _e2640, fma(_e2638, _e2638, (_e2639 * _e2639))));
                                        if (_e2645 == 0f) {
                                            phi_5242_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_5242_ = (vec3<f32>(_e2638, _e2639, _e2640) * (1f / _e2645));
                                        }
                                        let _e2650 = phi_5242_;
                                        let _e2652 = -(_e2650.x);
                                        let _e2654 = -(_e2650.y);
                                        let _e2656 = -(_e2650.z);
                                        let _e2657 = -(_e2650);
                                        let _e2659 = fma(-(_e680.z), _e289.member_3, 1f);
                                        let _e2663 = fma(0.4f, _e2659, (_e1364 * _e1376));
                                        let _e2664 = fma(0.4f, _e2659, (_e1366 * _e1376));
                                        let _e2665 = fma(0.4f, _e2659, (_e1368 * _e1376));
                                        let _e2673 = (_e1916 + vec3<f32>(_e2652, _e2654, _e2656));
                                        let _e2680 = sqrt(fma(_e2673.z, _e2673.z, fma(_e2673.x, _e2673.x, (_e2673.y * _e2673.y))));
                                        if (_e2680 == 0f) {
                                            phi_5277_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_5277_ = (_e2673 * (1f / _e2680));
                                        }
                                        let _e2685 = phi_5277_;
                                        let _e2686 = (_e1373 * _e1373);
                                        let _e2697 = max(fma(_e1912.z, _e2685.z, fma(_e1912.x, _e2685.x, (_e1912.y * _e2685.y))), 0f);
                                        let _e2710 = max(fma(_e1912.z, _e1916.z, fma(_e1912.x, _e1916.x, (_e1912.y * _e1916.y))), 0f);
                                        let _e2717 = max(fma(_e1912.z, _e2657.z, fma(_e1912.x, _e2657.x, (_e1912.y * _e2657.y))), 0f);
                                        let _e2718 = fma(_e680.y, _e289.member_4, 1f);
                                        let _e2719 = (_e2718 * _e2718);
                                        let _e2720 = (_e2719 * 0.125f);
                                        let _e2722 = fma(-(_e2719), 0.125f, 1f);
                                        let _e2735 = (1f - max(fma(_e2685.z, _e1916.z, fma(_e2685.x, _e1916.x, (_e2685.y * _e1916.y))), 0f));
                                        let _e2737 = select(_e2735, 0f, (_e2735 < 0f));
                                        let _e2740 = pow(select(_e2737, 1f, (_e2737 > 1f)), 5f);
                                        let _e2741 = fma((1f - _e2663), _e2740, _e2663);
                                        let _e2742 = fma((1f - _e2664), _e2740, _e2664);
                                        let _e2743 = fma((1f - _e2665), _e2740, _e2665);
                                        let _e2750 = (((_e2686 * _e2686) / (pow(fma((_e2697 * _e2697), fma(_e2686, _e2686, -1f), 1f), 2f) * 3.1415927f)) * ((_e2710 / fma(_e2710, _e2722, _e2720)) * (_e2717 / fma(_e2717, _e2722, _e2720))));
                                        let _e2757 = max(fma(_e1912.z, _e2656, fma(_e1912.x, _e2652, (_e1912.y * _e2654))), 0f);
                                        let _e2759 = fma((4f * _e2710), _e2757, 0.0001f);
                                        phi_3124_ = vec3<f32>(fma((fma((((1f - _e2741) * _e2659) * _e1364), 0.31830987f, ((_e2750 * _e2741) / _e2759)) * (_e2616.member_1.x * _e2616.member_2)), _e2757, _e1921.x), fma((fma((((1f - _e2742) * _e2659) * _e1366), 0.31830987f, ((_e2750 * _e2742) / _e2759)) * (_e2616.member_1.y * _e2616.member_2)), _e2757, _e1921.y), fma((fma((((1f - _e2743) * _e2659) * _e1368), 0.31830987f, ((_e2750 * _e2743) / _e2759)) * (_e2616.member_1.z * _e2616.member_2)), _e2757, _e1921.z));
                                        phi_3125_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if _e295 {
                                            phi_5369_ = (_e1976.member_1 <= (_e110 - 8u));
                                        } else {
                                            phi_5369_ = false;
                                        }
                                        let _e2361 = phi_5369_;
                                        if _e2361 {
                                            let _e2364 = global.member[_e1976.member_1];
                                            let _e2369 = global.member[(_e1976.member_1 + 1u)];
                                            let _e2374 = global.member[(_e1976.member_1 + 2u)];
                                            let _e2380 = global.member[(_e1976.member_1 + 3u)];
                                            let _e2385 = global.member[(_e1976.member_1 + 4u)];
                                            let _e2390 = global.member[(_e1976.member_1 + 5u)];
                                            let _e2395 = global.member[(_e1976.member_1 + 6u)];
                                            let _e2401 = global.member[(_e1976.member_1 + 7u)];
                                            phi_2626_ = type_34(vec3<f32>(bitcast<f32>(_e2364), bitcast<f32>(_e2369), bitcast<f32>(_e2374)), vec4<f32>(bitcast<f32>(_e2380), bitcast<f32>(_e2385), bitcast<f32>(_e2390), bitcast<f32>(_e2395)), bitcast<f32>(_e2401));
                                        } else {
                                            phi_2626_ = type_34(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2405 = phi_2626_;
                                        let _e2434 = (vec3<f32>((_e2037.member.x + fma(_e2076.x, _e2405.member.z, fma(_e2074.x, _e2405.member.y, (_e2072.x * _e2405.member.x)))), (_e2037.member.y + fma(_e2076.y, _e2405.member.z, fma(_e2074.y, _e2405.member.y, (_e2072.y * _e2405.member.x)))), (_e2037.member.z + fma(_e2076.z, _e2405.member.z, fma(_e2074.z, _e2405.member.y, (_e2072.z * _e2405.member.x))))) - _e118);
                                        let _e2441 = sqrt(fma(_e2434.z, _e2434.z, fma(_e2434.x, _e2434.x, (_e2434.y * _e2434.y))));
                                        let _e2442 = (_e2441 == 0f);
                                        if _e2442 {
                                            phi_2812_ = vec3<f32>();
                                        } else {
                                            if _e2442 {
                                                phi_5415_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5415_ = (_e2434 * (1f / _e2441));
                                            }
                                            let _e2446 = phi_5415_;
                                            let _e2448 = (_e2405.member_2 / (_e2441 * _e2441));
                                            let _e2450 = fma(-(_e680.z), _e289.member_3, 1f);
                                            let _e2454 = fma(0.4f, _e2450, (_e1364 * _e1376));
                                            let _e2455 = fma(0.4f, _e2450, (_e1366 * _e1376));
                                            let _e2456 = fma(0.4f, _e2450, (_e1368 * _e1376));
                                            let _e2463 = (_e1916 + _e2446);
                                            let _e2470 = sqrt(fma(_e2463.z, _e2463.z, fma(_e2463.x, _e2463.x, (_e2463.y * _e2463.y))));
                                            if (_e2470 == 0f) {
                                                phi_5450_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5450_ = (_e2463 * (1f / _e2470));
                                            }
                                            let _e2475 = phi_5450_;
                                            let _e2476 = (_e1373 * _e1373);
                                            let _e2487 = max(fma(_e1912.z, _e2475.z, fma(_e1912.x, _e2475.x, (_e1912.y * _e2475.y))), 0f);
                                            let _e2500 = max(fma(_e1912.z, _e1916.z, fma(_e1912.x, _e1916.x, (_e1912.y * _e1916.y))), 0f);
                                            let _e2507 = max(fma(_e1912.z, _e2446.z, fma(_e1912.x, _e2446.x, (_e1912.y * _e2446.y))), 0f);
                                            let _e2508 = fma(_e680.y, _e289.member_4, 1f);
                                            let _e2509 = (_e2508 * _e2508);
                                            let _e2510 = (_e2509 * 0.125f);
                                            let _e2512 = fma(-(_e2509), 0.125f, 1f);
                                            let _e2525 = (1f - max(fma(_e2475.z, _e1916.z, fma(_e2475.x, _e1916.x, (_e2475.y * _e1916.y))), 0f));
                                            let _e2527 = select(_e2525, 0f, (_e2525 < 0f));
                                            let _e2530 = pow(select(_e2527, 1f, (_e2527 > 1f)), 5f);
                                            let _e2531 = fma((1f - _e2454), _e2530, _e2454);
                                            let _e2532 = fma((1f - _e2455), _e2530, _e2455);
                                            let _e2533 = fma((1f - _e2456), _e2530, _e2456);
                                            let _e2540 = (((_e2476 * _e2476) / (pow(fma((_e2487 * _e2487), fma(_e2476, _e2476, -1f), 1f), 2f) * 3.1415927f)) * ((_e2500 / fma(_e2500, _e2512, _e2510)) * (_e2507 / fma(_e2507, _e2512, _e2510))));
                                            let _e2545 = fma((4f * _e2500), _e2507, 0.0001f);
                                            phi_2812_ = vec3<f32>(fma((fma((((1f - _e2531) * _e2450) * _e1364), 0.31830987f, ((_e2540 * _e2531) / _e2545)) * (_e2405.member_1.x * _e2448)), _e2507, _e1921.x), fma((fma((((1f - _e2532) * _e2450) * _e1366), 0.31830987f, ((_e2540 * _e2532) / _e2545)) * (_e2405.member_1.y * _e2448)), _e2507, _e1921.y), fma((fma((((1f - _e2533) * _e2450) * _e1368), 0.31830987f, ((_e2540 * _e2533) / _e2545)) * (_e2405.member_1.z * _e2448)), _e2507, _e1921.z));
                                        }
                                        let _e2566 = phi_2812_;
                                        phi_3124_ = _e2566;
                                        phi_3125_ = select(true, false, _e2442);
                                        break;
                                    }
                                    case 2: {
                                        if (_e110 >= 13u) {
                                            phi_5542_ = (_e1976.member_1 <= (_e110 - 13u));
                                        } else {
                                            phi_5542_ = false;
                                        }
                                        let _e2087 = phi_5542_;
                                        if _e2087 {
                                            let _e2090 = global.member[_e1976.member_1];
                                            let _e2095 = global.member[(_e1976.member_1 + 1u)];
                                            let _e2100 = global.member[(_e1976.member_1 + 2u)];
                                            let _e2106 = global.member[(_e1976.member_1 + 3u)];
                                            let _e2111 = global.member[(_e1976.member_1 + 4u)];
                                            let _e2116 = global.member[(_e1976.member_1 + 5u)];
                                            let _e2122 = global.member[(_e1976.member_1 + 6u)];
                                            let _e2127 = global.member[(_e1976.member_1 + 7u)];
                                            let _e2132 = global.member[(_e1976.member_1 + 8u)];
                                            let _e2137 = global.member[(_e1976.member_1 + 9u)];
                                            let _e2142 = global.member[(_e1976.member_1 + 10u)];
                                            let _e2147 = global.member[(_e1976.member_1 + 11u)];
                                            let _e2153 = global.member[(_e1976.member_1 + 12u)];
                                            phi_2875_ = type_35(vec3<f32>(bitcast<f32>(_e2090), bitcast<f32>(_e2095), bitcast<f32>(_e2100)), vec3<f32>(bitcast<f32>(_e2106), bitcast<f32>(_e2111), bitcast<f32>(_e2116)), bitcast<f32>(_e2122), bitcast<f32>(_e2127), vec4<f32>(bitcast<f32>(_e2132), bitcast<f32>(_e2137), bitcast<f32>(_e2142), bitcast<f32>(_e2147)), bitcast<f32>(_e2153));
                                        } else {
                                            phi_2875_ = type_35(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2157 = phi_2875_;
                                        let _e2189 = (vec3<f32>((_e2037.member.x + fma(_e2076.x, _e2157.member.z, fma(_e2074.x, _e2157.member.y, (_e2072.x * _e2157.member.x)))), (_e2037.member.y + fma(_e2076.y, _e2157.member.z, fma(_e2074.y, _e2157.member.y, (_e2072.y * _e2157.member.x)))), (_e2037.member.z + fma(_e2076.z, _e2157.member.z, fma(_e2074.z, _e2157.member.y, (_e2072.z * _e2157.member.x))))) - _e118);
                                        let _e2196 = sqrt(fma(_e2189.z, _e2189.z, fma(_e2189.x, _e2189.x, (_e2189.y * _e2189.y))));
                                        let _e2197 = (_e2196 == 0f);
                                        if _e2197 {
                                            phi_3122_ = vec3<f32>();
                                        } else {
                                            if _e2197 {
                                                phi_5592_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5592_ = (_e2189 * (1f / _e2196));
                                            }
                                            let _e2201 = phi_5592_;
                                            let _e2211 = fma(_e2076.x, _e2157.member_1.z, fma(_e2074.x, _e2157.member_1.y, (_e2072.x * _e2157.member_1.x)));
                                            let _e2212 = fma(_e2076.y, _e2157.member_1.z, fma(_e2074.y, _e2157.member_1.y, (_e2072.y * _e2157.member_1.x)));
                                            let _e2213 = fma(_e2076.z, _e2157.member_1.z, fma(_e2074.z, _e2157.member_1.y, (_e2072.z * _e2157.member_1.x)));
                                            let _e2218 = sqrt(fma(_e2213, _e2213, fma(_e2211, _e2211, (_e2212 * _e2212))));
                                            if (_e2218 == 0f) {
                                                phi_5627_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5627_ = (vec3<f32>(_e2211, _e2212, _e2213) * (1f / _e2218));
                                            }
                                            let _e2223 = phi_5627_;
                                            let _e2235 = ((fma(_e2201.z, _e2223.z, fma(_e2201.x, _e2223.x, (_e2201.y * _e2223.y))) - _e2157.member_3) / (_e2157.member_2 - _e2157.member_3));
                                            let _e2237 = select(_e2235, 0f, (_e2235 < 0f));
                                            let _e2240 = (_e2157.member_5 * select(_e2237, 1f, (_e2237 > 1f)));
                                            let _e2242 = fma(-(_e680.z), _e289.member_3, 1f);
                                            let _e2246 = fma(0.4f, _e2242, (_e1364 * _e1376));
                                            let _e2247 = fma(0.4f, _e2242, (_e1366 * _e1376));
                                            let _e2248 = fma(0.4f, _e2242, (_e1368 * _e1376));
                                            let _e2255 = (_e1916 + _e2201);
                                            let _e2262 = sqrt(fma(_e2255.z, _e2255.z, fma(_e2255.x, _e2255.x, (_e2255.y * _e2255.y))));
                                            if (_e2262 == 0f) {
                                                phi_5662_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5662_ = (_e2255 * (1f / _e2262));
                                            }
                                            let _e2267 = phi_5662_;
                                            let _e2268 = (_e1373 * _e1373);
                                            let _e2279 = max(fma(_e1912.z, _e2267.z, fma(_e1912.x, _e2267.x, (_e1912.y * _e2267.y))), 0f);
                                            let _e2292 = max(fma(_e1912.z, _e1916.z, fma(_e1912.x, _e1916.x, (_e1912.y * _e1916.y))), 0f);
                                            let _e2296 = max(fma(_e1912.z, _e2201.z, fma(_e1912.x, _e2201.x, (_e1912.y * _e2201.y))), 0f);
                                            let _e2297 = fma(_e680.y, _e289.member_4, 1f);
                                            let _e2298 = (_e2297 * _e2297);
                                            let _e2299 = (_e2298 * 0.125f);
                                            let _e2301 = fma(-(_e2298), 0.125f, 1f);
                                            let _e2314 = (1f - max(fma(_e2267.z, _e1916.z, fma(_e2267.x, _e1916.x, (_e2267.y * _e1916.y))), 0f));
                                            let _e2316 = select(_e2314, 0f, (_e2314 < 0f));
                                            let _e2319 = pow(select(_e2316, 1f, (_e2316 > 1f)), 5f);
                                            let _e2320 = fma((1f - _e2246), _e2319, _e2246);
                                            let _e2321 = fma((1f - _e2247), _e2319, _e2247);
                                            let _e2322 = fma((1f - _e2248), _e2319, _e2248);
                                            let _e2329 = (((_e2268 * _e2268) / (pow(fma((_e2279 * _e2279), fma(_e2268, _e2268, -1f), 1f), 2f) * 3.1415927f)) * ((_e2292 / fma(_e2292, _e2301, _e2299)) * (_e2296 / fma(_e2296, _e2301, _e2299))));
                                            let _e2334 = fma((4f * _e2292), _e2296, 0.0001f);
                                            phi_3122_ = vec3<f32>(fma((fma((((1f - _e2320) * _e2242) * _e1364), 0.31830987f, ((_e2329 * _e2320) / _e2334)) * (_e2157.member_4.x * _e2240)), _e2296, _e1921.x), fma((fma((((1f - _e2321) * _e2242) * _e1366), 0.31830987f, ((_e2329 * _e2321) / _e2334)) * (_e2157.member_4.y * _e2240)), _e2296, _e1921.y), fma((fma((((1f - _e2322) * _e2242) * _e1368), 0.31830987f, ((_e2329 * _e2322) / _e2334)) * (_e2157.member_4.z * _e2240)), _e2296, _e1921.z));
                                        }
                                        let _e2355 = phi_3122_;
                                        phi_3124_ = _e2355;
                                        phi_3125_ = select(true, false, _e2197);
                                        break;
                                    }
                                    default: {
                                        phi_3124_ = vec3<f32>();
                                        phi_3125_ = bool();
                                        break;
                                    }
                                }
                                let _e2780 = phi_3124_;
                                let _e2782 = phi_3125_;
                                phi_3134_ = select(_e2780, _e1921, vec3(select(true, false, _e2782)));
                            }
                            let _e2787 = phi_3134_;
                            phi_2267_ = _e2787;
                            phi_3136_ = select(true, false, _e1954);
                            break;
                        }
                        default: {
                            phi_2267_ = vec3<f32>();
                            phi_3136_ = bool();
                            break;
                        }
                    }
                    let _e2790 = phi_2267_;
                    let _e2792 = phi_3136_;
                    continue;
                    continuing {
                        phi_2263_ = _e1934;
                        phi_2266_ = _e2790;
                        break if !(_e2792);
                    }
                }
                let _e2795 = fma(-(_e680.z), _e289.member_3, 1f);
                let _e2799 = fma(0.04f, _e2795, (_e1364 * _e1376));
                let _e2800 = fma(0.04f, _e2795, (_e1366 * _e1376));
                let _e2801 = fma(0.04f, _e2795, (_e1368 * _e1376));
                let _e2813 = fma(-(_e680.y), _e289.member_4, 1f);
                let _e2820 = (1f - max(fma(_e1912.z, _e1916.z, fma(_e1912.x, _e1916.x, (_e1912.y * _e1916.y))), 0f));
                let _e2822 = select(_e2820, 0f, (_e2820 < 0f));
                let _e2825 = pow(select(_e2822, 1f, (_e2822 > 1f)), 5f);
                let _e2826 = fma((max(_e2813, _e2799) - _e2799), _e2825, _e2799);
                let _e2827 = fma((max(_e2813, _e2800) - _e2800), _e2825, _e2800);
                let _e2828 = fma((max(_e2813, _e2801) - _e2801), _e2825, _e2801);
                let _e2848 = local_2;
                let _e2852 = local_3;
                let _e2856 = local_4;
                phi_3249_ = vec4<f32>(fma(_e1386, _e289.member_1, fma(fma(((1f - _e2826) * _e2795), (_e1395.x * _e1364), (_e1727.x * fma(_e2826, _e1743.x, _e1743.y))), _e1380, _e2848.x)), fma(_e1388, _e289.member_1, fma(fma(((1f - _e2827) * _e2795), (_e1395.y * _e1366), (_e1727.y * fma(_e2827, _e1743.x, _e1743.y))), _e1380, _e2852.y)), fma(_e1390, _e289.member_1, fma(fma(((1f - _e2828) * _e2795), (_e1395.z * _e1368), (_e1727.z * fma(_e2828, _e1743.x, _e1743.y))), _e1380, _e2856.z)), 1f);
            } else {
                phi_3249_ = (vec4<f32>((_e112.x * _e486.x), (_e112.y * _e486.y), (_e112.z * _e486.z), (_e112.w * _e486.w)) * _e289.member_2);
            }
            let _e2864 = phi_3249_;
            global_17 = _e2864;
            break;
        }
        case 1: {
            let _e1885 = sqrt(fma(_e113.x, _e113.x, (_e113.y * _e113.y)));
            if (_e1885 == 0f) {
                phi_5781_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5781_ = (vec3<f32>(_e113.x, _e113.y, 0f) * (1f / _e1885));
            }
            let _e1890 = phi_5781_;
            global_17 = vec4<f32>(((_e1890.x + 1f) * 0.5f), ((_e1890.y + 1f) * 0.5f), ((_e1890.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1864 = sqrt(fma(_e114.x, _e114.x, (_e114.y * _e114.y)));
            if (_e1864 == 0f) {
                phi_5830_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5830_ = (vec3<f32>(_e114.x, _e114.y, 0f) * (1f / _e1864));
            }
            let _e1869 = phi_5830_;
            global_17 = vec4<f32>(((_e1869.x + 1f) * 0.5f), ((_e1869.y + 1f) * 0.5f), ((_e1869.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1706 {
                phi_5879_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5879_ = (_e1349 * (1f / _e1705));
            }
            let _e1848 = phi_5879_;
            global_17 = vec4<f32>(((_e1848.x + 1f) * 0.5f), ((_e1848.y + 1f) * 0.5f), ((_e1848.z + 1f) * 0.5f), 1f);
            break;
        }
        case 4: {
            global_17 = _e112;
            break;
        }
        case 5: {
            let _e1829 = sqrt(fma(_e115.z, _e115.z, fma(_e115.x, _e115.x, (_e115.y * _e115.y))));
            if (_e1829 == 0f) {
                phi_5928_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5928_ = (_e115 * (1f / _e1829));
            }
            let _e1834 = phi_5928_;
            global_17 = vec4<f32>(((_e1834.x + 1f) * 0.5f), ((_e1834.y + 1f) * 0.5f), ((_e1834.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1807 = sqrt(fma(_e1347.z, _e1347.z, fma(_e1347.x, _e1347.x, (_e1347.y * _e1347.y))));
            if (_e1807 == 0f) {
                phi_5977_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5977_ = (_e1347 * (1f / _e1807));
            }
            let _e1812 = phi_5977_;
            global_17 = vec4<f32>(((_e1812.x + 1f) * 0.5f), ((_e1812.y + 1f) * 0.5f), ((_e1812.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1785 = sqrt(fma(_e116.z, _e116.z, fma(_e116.x, _e116.x, (_e116.y * _e116.y))));
            if (_e1785 == 0f) {
                phi_6026_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6026_ = (_e116 * (1f / _e1785));
            }
            let _e1790 = phi_6026_;
            global_17 = vec4<f32>(((_e1790.x + 1f) * 0.5f), ((_e1790.y + 1f) * 0.5f), ((_e1790.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1763 = sqrt(fma(_e117.z, _e117.z, fma(_e117.x, _e117.x, (_e117.y * _e117.y))));
            if (_e1763 == 0f) {
                phi_6075_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6075_ = (_e117 * (1f / _e1763));
            }
            let _e1768 = phi_6075_;
            global_17 = vec4<f32>(((_e1768.x + 1f) * 0.5f), ((_e1768.y + 1f) * 0.5f), ((_e1768.z + 1f) * 0.5f), 1f);
            break;
        }
        case 9: {
            global_17 = vec4<f32>(_e1395.x, _e1395.y, _e1395.z, 1f);
            break;
        }
        case 10: {
            global_17 = vec4<f32>(_e1727.x, _e1727.y, _e1727.z, 1f);
            break;
        }
        case 11: {
            global_17 = vec4<f32>(_e1743.x, _e1743.y, 1f, 1f);
            break;
        }
        case 12: {
            global_17 = (vec4<f32>(_e1353, _e1356, _e1359, (_e486.w * _e289.member_2.w)) * _e112);
            break;
        }
        case 13: {
            global_17 = vec4<f32>(_e1373, _e1373, _e1373, 1f);
            break;
        }
        case 14: {
            global_17 = vec4<f32>(_e1376, _e1376, _e1376, 1f);
            break;
        }
        case 15: {
            global_17 = vec4<f32>(_e1380, _e1380, _e1380, 1f);
            break;
        }
        case 16: {
            global_17 = vec4<f32>((_e1386 * _e289.member_1), (_e1388 * _e289.member_1), (_e1390 * _e289.member_1), 1f);
            break;
        }
        case 17: {
            global_17 = vec4<f32>(_e1262.x, _e1262.y, _e1262.z, 1f);
            break;
        }
        case 18: {
            global_17 = vec4<f32>(_e289.member.x, _e289.member.y, _e289.member.z, 1f);
            break;
        }
        case 19: {
            global_17 = vec4<f32>(_e289.member_1, _e289.member_1, _e289.member_1, 1f);
            break;
        }
        default: {
            break;
        }
    }
    return;
}

@fragment 
fn stagerenderlet_fragment(@location(0) @interpolate(flat) param: u32, @location(1) param_1: vec4<f32>, @location(2) param_2: vec2<f32>, @location(3) param_3: vec2<f32>, @location(4) param_4: vec3<f32>, @location(5) param_5: vec3<f32>, @location(6) param_6: vec3<f32>, @location(7) param_7: vec3<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    global_2 = param_1;
    global_3 = param_2;
    global_4 = param_3;
    global_5 = param_4;
    global_6 = param_5;
    global_7 = param_6;
    global_8 = param_7;
    function();
    let _e17 = global_17;
    return _e17;
}
