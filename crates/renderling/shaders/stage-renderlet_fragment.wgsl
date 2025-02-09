struct type_3 {
    member: array<u32>,
}

struct type_14 {
    member: u32,
    member_1: u32,
}

struct type_15 {
    member: type_14,
    member_1: vec2<u32>,
    member_2: vec2<u32>,
    member_3: u32,
    member_4: u32,
}

struct type_23 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_24 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
    member_2: vec3<f32>,
}

struct type_25 {
    member: type_23,
    member_1: type_23,
    member_2: vec3<f32>,
    member_3: type_24,
}

struct type_30 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_31 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_33 {
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
var<storage> global: type_3;
@group(2) @binding(0) 
var<storage> global_1: type_3;
var<private> global_2: u32;
var<private> global_3: vec4<f32>;
var<private> global_4: vec2<f32>;
var<private> global_5: vec2<f32>;
var<private> global_6: vec3<f32>;
var<private> global_7: vec3<f32>;
var<private> global_8: vec3<f32>;
var<private> global_9: vec3<f32>;
@group(1) @binding(1) 
var global_10: sampler;
@group(1) @binding(0) 
var global_11: texture_2d_array<f32>;
@group(1) @binding(2) 
var global_12: texture_cube<f32>;
@group(1) @binding(3) 
var global_13: sampler;
@group(1) @binding(4) 
var global_14: texture_cube<f32>;
@group(1) @binding(5) 
var global_15: sampler;
@group(1) @binding(6) 
var global_16: texture_2d<f32>;
@group(1) @binding(7) 
var global_17: sampler;
@group(2) @binding(2) 
var global_18: sampler;
@group(2) @binding(1) 
var global_19: texture_2d_array<f32>;
var<private> global_20: vec4<f32>;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var phi_680_: u32;
    var phi_4342_: bool;
    var phi_808_: type_33;
    var phi_812_: type_33;
    var phi_4379_: bool;
    var phi_852_: u32;
    var phi_861_: u32;
    var phi_874_: type_15;
    var phi_4401_: f32;
    var phi_4414_: bool;
    var phi_928_: f32;
    var phi_923_: f32;
    var phi_929_: f32;
    var phi_4431_: bool;
    var phi_894_: f32;
    var phi_931_: f32;
    var phi_4449_: f32;
    var phi_4462_: bool;
    var phi_986_: f32;
    var phi_981_: f32;
    var phi_987_: f32;
    var phi_4479_: bool;
    var phi_952_: f32;
    var phi_989_: f32;
    var phi_4515_: bool;
    var phi_1072_: u32;
    var phi_1081_: u32;
    var phi_1094_: type_15;
    var phi_4536_: f32;
    var phi_4549_: bool;
    var phi_1148_: f32;
    var phi_1143_: f32;
    var phi_1149_: f32;
    var phi_4566_: bool;
    var phi_1114_: f32;
    var phi_1151_: f32;
    var phi_4584_: f32;
    var phi_4597_: bool;
    var phi_1206_: f32;
    var phi_1201_: f32;
    var phi_1207_: f32;
    var phi_4614_: bool;
    var phi_1172_: f32;
    var phi_1209_: f32;
    var phi_4650_: bool;
    var phi_1292_: u32;
    var phi_1301_: u32;
    var phi_1314_: type_15;
    var phi_4671_: f32;
    var phi_4684_: bool;
    var phi_1368_: f32;
    var phi_1363_: f32;
    var phi_1369_: f32;
    var phi_4701_: bool;
    var phi_1334_: f32;
    var phi_1371_: f32;
    var phi_4719_: f32;
    var phi_4732_: bool;
    var phi_1426_: f32;
    var phi_1421_: f32;
    var phi_1427_: f32;
    var phi_4749_: bool;
    var phi_1392_: f32;
    var phi_1429_: f32;
    var phi_4785_: bool;
    var phi_1512_: u32;
    var phi_1521_: u32;
    var phi_1534_: type_15;
    var phi_4806_: f32;
    var phi_4819_: bool;
    var phi_1588_: f32;
    var phi_1583_: f32;
    var phi_1589_: f32;
    var phi_4836_: bool;
    var phi_1554_: f32;
    var phi_1591_: f32;
    var phi_4854_: f32;
    var phi_4867_: bool;
    var phi_1646_: f32;
    var phi_1641_: f32;
    var phi_1647_: f32;
    var phi_4884_: bool;
    var phi_1612_: f32;
    var phi_1649_: f32;
    var phi_4920_: bool;
    var phi_1732_: u32;
    var phi_1741_: u32;
    var phi_1754_: type_15;
    var phi_4941_: f32;
    var phi_4954_: bool;
    var phi_1808_: f32;
    var phi_1803_: f32;
    var phi_1809_: f32;
    var phi_4971_: bool;
    var phi_1774_: f32;
    var phi_1811_: f32;
    var phi_4989_: f32;
    var phi_5002_: bool;
    var phi_1866_: f32;
    var phi_1861_: f32;
    var phi_1867_: f32;
    var phi_5019_: bool;
    var phi_1832_: f32;
    var phi_1869_: f32;
    var phi_5077_: vec3<f32>;
    var phi_5112_: vec3<f32>;
    var phi_5147_: vec3<f32>;
    var phi_5182_: vec3<f32>;
    var phi_5217_: vec3<f32>;
    var phi_1963_: vec3<f32>;
    var phi_1964_: vec3<f32>;
    var phi_5249_: bool;
    var phi_2171_: type_14;
    var phi_2172_: type_14;
    var phi_2195_: type_14;
    var phi_2222_: bool;
    var phi_2228_: type_14;
    var phi_2229_: type_14;
    var phi_2252_: type_14;
    var phi_2275_: bool;
    var phi_2296_: type_25;
    var phi_5321_: vec3<f32>;
    var phi_5380_: vec3<f32>;
    var phi_5454_: vec3<f32>;
    var phi_5514_: vec3<f32>;
    var phi_5563_: vec3<f32>;
    var phi_5612_: vec3<f32>;
    var phi_5661_: vec3<f32>;
    var phi_5710_: vec3<f32>;
    var phi_5759_: vec3<f32>;
    var phi_5808_: vec3<f32>;
    var phi_5847_: vec3<f32>;
    var phi_5882_: vec3<f32>;
    var phi_2363_: type_14;
    var phi_2366_: vec3<f32>;
    var phi_2364_: type_14;
    var phi_2389_: type_14;
    var phi_5908_: u32;
    var phi_5927_: bool;
    var phi_2406_: u32;
    var phi_5959_: bool;
    var phi_2423_: u32;
    var phi_2437_: type_30;
    var phi_5991_: bool;
    var phi_2487_: type_31;
    var phi_6071_: bool;
    var phi_3413_: type_35;
    var phi_6121_: vec3<f32>;
    var phi_6156_: vec3<f32>;
    var phi_6191_: vec3<f32>;
    var phi_3668_: vec3<f32>;
    var phi_6283_: bool;
    var phi_3160_: type_34;
    var phi_6330_: vec3<f32>;
    var phi_6365_: vec3<f32>;
    var phi_3350_: vec3<f32>;
    var phi_6457_: bool;
    var phi_2535_: type_34;
    var phi_6504_: vec3<f32>;
    var phi_6539_: vec3<f32>;
    var phi_2772_: u32;
    var phi_2781_: u32;
    var phi_6645_: f32;
    var phi_6658_: bool;
    var phi_2987_: f32;
    var phi_2982_: f32;
    var phi_2988_: f32;
    var phi_6675_: bool;
    var phi_2953_: f32;
    var phi_2990_: f32;
    var phi_6693_: f32;
    var phi_6706_: bool;
    var phi_3043_: f32;
    var phi_3038_: f32;
    var phi_3044_: f32;
    var phi_6723_: bool;
    var phi_3009_: f32;
    var phi_3046_: f32;
    var phi_3106_: f32;
    var phi_3670_: vec3<f32>;
    var phi_3671_: bool;
    var phi_3680_: vec3<f32>;
    var phi_2367_: vec3<f32>;
    var phi_3682_: bool;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var phi_3799_: vec4<f32>;

    let _e116 = arrayLength((&global.member));
    let _e118 = arrayLength((&global_1.member));
    let _e119 = global_2;
    let _e120 = global_3;
    let _e121 = global_4;
    let _e122 = global_5;
    let _e123 = global_6;
    let _e124 = global_7;
    let _e125 = global_8;
    let _e126 = global_9;
    let _e130 = global.member[(_e119 + 9u)];
    let _e134 = global.member[(_e119 + 11u)];
    let _e138 = global.member[(_e119 + 17u)];
    let _e141 = global.member[_e138];
    let _e145 = global.member[(_e138 + 1u)];
    let _e149 = global.member[(_e138 + 4u)];
    switch bitcast<i32>(_e149) {
        case 0: {
            phi_680_ = 0u;
            break;
        }
        case 1: {
            phi_680_ = 1u;
            break;
        }
        case 2: {
            phi_680_ = 2u;
            break;
        }
        case 3: {
            phi_680_ = 3u;
            break;
        }
        case 4: {
            phi_680_ = 4u;
            break;
        }
        case 5: {
            phi_680_ = 5u;
            break;
        }
        case 6: {
            phi_680_ = 6u;
            break;
        }
        case 7: {
            phi_680_ = 7u;
            break;
        }
        case 8: {
            phi_680_ = 8u;
            break;
        }
        case 9: {
            phi_680_ = 9u;
            break;
        }
        case 10: {
            phi_680_ = 10u;
            break;
        }
        case 11: {
            phi_680_ = 11u;
            break;
        }
        case 12: {
            phi_680_ = 12u;
            break;
        }
        case 13: {
            phi_680_ = 13u;
            break;
        }
        case 14: {
            phi_680_ = 14u;
            break;
        }
        case 15: {
            phi_680_ = 15u;
            break;
        }
        case 16: {
            phi_680_ = 16u;
            break;
        }
        case 17: {
            phi_680_ = 17u;
            break;
        }
        case 18: {
            phi_680_ = 18u;
            break;
        }
        case 19: {
            phi_680_ = 19u;
            break;
        }
        default: {
            phi_680_ = 0u;
            break;
        }
    }
    let _e152 = phi_680_;
    let _e156 = global.member[(_e138 + 5u)];
    if (_e134 == 4294967295u) {
        phi_812_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e116 >= 22u) {
            phi_4342_ = (_e134 <= (_e116 - 22u));
        } else {
            phi_4342_ = false;
        }
        let _e163 = phi_4342_;
        if _e163 {
            let _e166 = global.member[_e134];
            let _e171 = global.member[(_e134 + 1u)];
            let _e176 = global.member[(_e134 + 2u)];
            let _e182 = global.member[(_e134 + 3u)];
            let _e187 = global.member[(_e134 + 4u)];
            let _e192 = global.member[(_e134 + 5u)];
            let _e197 = global.member[(_e134 + 6u)];
            let _e202 = global.member[(_e134 + 7u)];
            let _e208 = global.member[(_e134 + 8u)];
            let _e213 = global.member[(_e134 + 9u)];
            let _e218 = global.member[(_e134 + 10u)];
            let _e222 = global.member[(_e134 + 11u)];
            let _e226 = global.member[(_e134 + 12u)];
            let _e230 = global.member[(_e134 + 13u)];
            let _e234 = global.member[(_e134 + 14u)];
            let _e238 = global.member[(_e134 + 15u)];
            let _e242 = global.member[(_e134 + 16u)];
            let _e246 = global.member[(_e134 + 17u)];
            let _e250 = global.member[(_e134 + 18u)];
            let _e254 = global.member[(_e134 + 19u)];
            let _e258 = global.member[(_e134 + 20u)];
            let _e263 = global.member[(_e134 + 21u)];
            phi_808_ = type_33(vec3<f32>(bitcast<f32>(_e166), bitcast<f32>(_e171), bitcast<f32>(_e176)), bitcast<f32>(_e182), vec4<f32>(bitcast<f32>(_e187), bitcast<f32>(_e192), bitcast<f32>(_e197), bitcast<f32>(_e202)), bitcast<f32>(_e208), bitcast<f32>(_e213), _e218, _e222, _e226, _e230, _e234, _e238, _e242, _e246, _e250, _e254, (_e258 == 1u), bitcast<f32>(_e263));
        } else {
            phi_808_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e267 = phi_808_;
        phi_812_ = type_33(_e267.member, _e267.member_1, _e267.member_2, _e267.member_3, _e267.member_4, _e267.member_5, _e267.member_6, _e267.member_7, _e267.member_8, _e267.member_9, _e267.member_10, _e267.member_11, _e267.member_12, _e267.member_13, _e267.member_14, (_e267.member_15 && (_e156 == 1u)), _e267.member_16);
    }
    let _e289 = phi_812_;
    let _e293 = select(_e122, _e121, vec2((_e289.member_10 == 0u)));
    let _e295 = (_e116 >= 8u);
    if _e295 {
        phi_4379_ = (_e289.member_5 <= (_e116 - 8u));
    } else {
        phi_4379_ = false;
    }
    let _e299 = phi_4379_;
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
                phi_852_ = 0u;
                break;
            }
            case 1: {
                phi_852_ = 1u;
                break;
            }
            case 2: {
                phi_852_ = 2u;
                break;
            }
            default: {
                phi_852_ = 0u;
                break;
            }
        }
        let _e331 = phi_852_;
        let _e335 = global.member[(_e289.member_5 + 7u)];
        switch bitcast<i32>(_e335) {
            case 0: {
                phi_861_ = 0u;
                break;
            }
            case 1: {
                phi_861_ = 1u;
                break;
            }
            case 2: {
                phi_861_ = 2u;
                break;
            }
            default: {
                phi_861_ = 0u;
                break;
            }
        }
        let _e338 = phi_861_;
        phi_874_ = type_15(type_14(_e331, _e338), vec2<u32>(_e302, _e306), vec2<u32>(_e311, _e315), _e320, _e324);
    } else {
        phi_874_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e342 = phi_874_;
    switch bitcast<i32>(_e342.member.member) {
        case 1: {
            let _e380 = abs(_e293.x);
            let _e382 = (_e380 % 1f);
            if (_e380 >= 1f) {
                phi_4431_ = select(true, false, (_e382 == 0f));
            } else {
                phi_4431_ = true;
            }
            let _e386 = phi_4431_;
            let _e387 = select(1f, _e382, _e386);
            if (select(-1f, 1f, (_e293.x >= 0f)) > 0f) {
                phi_894_ = _e387;
            } else {
                phi_894_ = (1f - _e387);
            }
            let _e391 = phi_894_;
            phi_931_ = _e391;
            break;
        }
        case 2: {
            let _e354 = abs(_e293.x);
            let _e361 = ((select(select(u32(_e354), 0u, (_e354 < 0f)), 4294967295u, (_e354 > 4294967000f)) % 2u) == 0u);
            let _e363 = (_e354 % 1f);
            if (_e354 >= 1f) {
                phi_4414_ = select(true, false, (_e363 == 0f));
            } else {
                phi_4414_ = true;
            }
            let _e367 = phi_4414_;
            let _e368 = select(1f, _e363, _e367);
            if (select(-1f, 1f, (_e293.x >= 0f)) > 0f) {
                if _e361 {
                    phi_923_ = _e368;
                } else {
                    phi_923_ = (1f - _e368);
                }
                let _e375 = phi_923_;
                phi_929_ = _e375;
            } else {
                if _e361 {
                    phi_928_ = (1f - _e368);
                } else {
                    phi_928_ = _e368;
                }
                let _e372 = phi_928_;
                phi_929_ = _e372;
            }
            let _e377 = phi_929_;
            phi_931_ = _e377;
            break;
        }
        case 0: {
            if (_e293.x > 1f) {
                phi_4401_ = 0.9999999f;
            } else {
                phi_4401_ = select(_e293.x, 0.00000011920929f, (_e293.x < 0f));
            }
            let _e351 = phi_4401_;
            phi_931_ = _e351;
            break;
        }
        default: {
            phi_931_ = f32();
            break;
        }
    }
    let _e393 = phi_931_;
    switch bitcast<i32>(_e342.member.member_1) {
        case 1: {
            let _e431 = abs(_e293.y);
            let _e433 = (_e431 % 1f);
            if (_e431 >= 1f) {
                phi_4479_ = select(true, false, (_e433 == 0f));
            } else {
                phi_4479_ = true;
            }
            let _e437 = phi_4479_;
            let _e438 = select(1f, _e433, _e437);
            if (select(-1f, 1f, (_e293.y >= 0f)) > 0f) {
                phi_952_ = _e438;
            } else {
                phi_952_ = (1f - _e438);
            }
            let _e442 = phi_952_;
            phi_989_ = _e442;
            break;
        }
        case 2: {
            let _e405 = abs(_e293.y);
            let _e412 = ((select(select(u32(_e405), 0u, (_e405 < 0f)), 4294967295u, (_e405 > 4294967000f)) % 2u) == 0u);
            let _e414 = (_e405 % 1f);
            if (_e405 >= 1f) {
                phi_4462_ = select(true, false, (_e414 == 0f));
            } else {
                phi_4462_ = true;
            }
            let _e418 = phi_4462_;
            let _e419 = select(1f, _e414, _e418);
            if (select(-1f, 1f, (_e293.y >= 0f)) > 0f) {
                if _e412 {
                    phi_981_ = _e419;
                } else {
                    phi_981_ = (1f - _e419);
                }
                let _e426 = phi_981_;
                phi_987_ = _e426;
            } else {
                if _e412 {
                    phi_986_ = (1f - _e419);
                } else {
                    phi_986_ = _e419;
                }
                let _e423 = phi_986_;
                phi_987_ = _e423;
            }
            let _e428 = phi_987_;
            phi_989_ = _e428;
            break;
        }
        case 0: {
            if (_e293.y > 1f) {
                phi_4449_ = 0.9999999f;
            } else {
                phi_4449_ = select(_e293.y, 0.00000011920929f, (_e293.y < 0f));
            }
            let _e402 = phi_4449_;
            phi_989_ = _e402;
            break;
        }
        default: {
            phi_989_ = f32();
            break;
        }
    }
    let _e444 = phi_989_;
    let _e448 = (_e393 * f32(_e342.member_2.x));
    let _e457 = (_e444 * f32(_e342.member_2.y));
    let _e469 = f32(_e141);
    let _e470 = f32(_e145);
    let _e477 = vec3<f32>((f32((select(select(u32(_e448), 0u, (_e448 < 0f)), 4294967295u, (_e448 > 4294967000f)) + _e342.member_1.x)) / _e469), (f32((select(select(u32(_e457), 0u, (_e457 < 0f)), 4294967295u, (_e457 > 4294967000f)) + _e342.member_1.y)) / _e470), f32(_e342.member_3));
    let _e483 = textureSampleLevel(global_11, global_10, vec2<f32>(_e477.x, _e477.y), i32(_e477.z), 0f);
    let _e486 = select(_e483, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e289.member_5 == 4294967295u)));
    let _e490 = select(_e122, _e121, vec2((_e289.member_11 == 0u)));
    if _e295 {
        phi_4515_ = (_e289.member_6 <= (_e116 - 8u));
    } else {
        phi_4515_ = false;
    }
    let _e495 = phi_4515_;
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
                phi_1072_ = 0u;
                break;
            }
            case 1: {
                phi_1072_ = 1u;
                break;
            }
            case 2: {
                phi_1072_ = 2u;
                break;
            }
            default: {
                phi_1072_ = 0u;
                break;
            }
        }
        let _e527 = phi_1072_;
        let _e531 = global.member[(_e289.member_6 + 7u)];
        switch bitcast<i32>(_e531) {
            case 0: {
                phi_1081_ = 0u;
                break;
            }
            case 1: {
                phi_1081_ = 1u;
                break;
            }
            case 2: {
                phi_1081_ = 2u;
                break;
            }
            default: {
                phi_1081_ = 0u;
                break;
            }
        }
        let _e534 = phi_1081_;
        phi_1094_ = type_15(type_14(_e527, _e534), vec2<u32>(_e498, _e502), vec2<u32>(_e507, _e511), _e516, _e520);
    } else {
        phi_1094_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e538 = phi_1094_;
    switch bitcast<i32>(_e538.member.member) {
        case 1: {
            let _e576 = abs(_e490.x);
            let _e578 = (_e576 % 1f);
            if (_e576 >= 1f) {
                phi_4566_ = select(true, false, (_e578 == 0f));
            } else {
                phi_4566_ = true;
            }
            let _e582 = phi_4566_;
            let _e583 = select(1f, _e578, _e582);
            if (select(-1f, 1f, (_e490.x >= 0f)) > 0f) {
                phi_1114_ = _e583;
            } else {
                phi_1114_ = (1f - _e583);
            }
            let _e587 = phi_1114_;
            phi_1151_ = _e587;
            break;
        }
        case 2: {
            let _e550 = abs(_e490.x);
            let _e557 = ((select(select(u32(_e550), 0u, (_e550 < 0f)), 4294967295u, (_e550 > 4294967000f)) % 2u) == 0u);
            let _e559 = (_e550 % 1f);
            if (_e550 >= 1f) {
                phi_4549_ = select(true, false, (_e559 == 0f));
            } else {
                phi_4549_ = true;
            }
            let _e563 = phi_4549_;
            let _e564 = select(1f, _e559, _e563);
            if (select(-1f, 1f, (_e490.x >= 0f)) > 0f) {
                if _e557 {
                    phi_1143_ = _e564;
                } else {
                    phi_1143_ = (1f - _e564);
                }
                let _e571 = phi_1143_;
                phi_1149_ = _e571;
            } else {
                if _e557 {
                    phi_1148_ = (1f - _e564);
                } else {
                    phi_1148_ = _e564;
                }
                let _e568 = phi_1148_;
                phi_1149_ = _e568;
            }
            let _e573 = phi_1149_;
            phi_1151_ = _e573;
            break;
        }
        case 0: {
            if (_e490.x > 1f) {
                phi_4536_ = 0.9999999f;
            } else {
                phi_4536_ = select(_e490.x, 0.00000011920929f, (_e490.x < 0f));
            }
            let _e547 = phi_4536_;
            phi_1151_ = _e547;
            break;
        }
        default: {
            phi_1151_ = f32();
            break;
        }
    }
    let _e589 = phi_1151_;
    switch bitcast<i32>(_e538.member.member_1) {
        case 1: {
            let _e627 = abs(_e490.y);
            let _e629 = (_e627 % 1f);
            if (_e627 >= 1f) {
                phi_4614_ = select(true, false, (_e629 == 0f));
            } else {
                phi_4614_ = true;
            }
            let _e633 = phi_4614_;
            let _e634 = select(1f, _e629, _e633);
            if (select(-1f, 1f, (_e490.y >= 0f)) > 0f) {
                phi_1172_ = _e634;
            } else {
                phi_1172_ = (1f - _e634);
            }
            let _e638 = phi_1172_;
            phi_1209_ = _e638;
            break;
        }
        case 2: {
            let _e601 = abs(_e490.y);
            let _e608 = ((select(select(u32(_e601), 0u, (_e601 < 0f)), 4294967295u, (_e601 > 4294967000f)) % 2u) == 0u);
            let _e610 = (_e601 % 1f);
            if (_e601 >= 1f) {
                phi_4597_ = select(true, false, (_e610 == 0f));
            } else {
                phi_4597_ = true;
            }
            let _e614 = phi_4597_;
            let _e615 = select(1f, _e610, _e614);
            if (select(-1f, 1f, (_e490.y >= 0f)) > 0f) {
                if _e608 {
                    phi_1201_ = _e615;
                } else {
                    phi_1201_ = (1f - _e615);
                }
                let _e622 = phi_1201_;
                phi_1207_ = _e622;
            } else {
                if _e608 {
                    phi_1206_ = (1f - _e615);
                } else {
                    phi_1206_ = _e615;
                }
                let _e619 = phi_1206_;
                phi_1207_ = _e619;
            }
            let _e624 = phi_1207_;
            phi_1209_ = _e624;
            break;
        }
        case 0: {
            if (_e490.y > 1f) {
                phi_4584_ = 0.9999999f;
            } else {
                phi_4584_ = select(_e490.y, 0.00000011920929f, (_e490.y < 0f));
            }
            let _e598 = phi_4584_;
            phi_1209_ = _e598;
            break;
        }
        default: {
            phi_1209_ = f32();
            break;
        }
    }
    let _e640 = phi_1209_;
    let _e644 = (_e589 * f32(_e538.member_2.x));
    let _e653 = (_e640 * f32(_e538.member_2.y));
    let _e671 = vec3<f32>((f32((select(select(u32(_e644), 0u, (_e644 < 0f)), 4294967295u, (_e644 > 4294967000f)) + _e538.member_1.x)) / _e469), (f32((select(select(u32(_e653), 0u, (_e653 < 0f)), 4294967295u, (_e653 > 4294967000f)) + _e538.member_1.y)) / _e470), f32(_e538.member_3));
    let _e677 = textureSampleLevel(global_11, global_10, vec2<f32>(_e671.x, _e671.y), i32(_e671.z), 0f);
    let _e680 = select(_e677, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e289.member_6 == 4294967295u)));
    let _e684 = select(_e122, _e121, vec2((_e289.member_12 == 0u)));
    if _e295 {
        phi_4650_ = (_e289.member_7 <= (_e116 - 8u));
    } else {
        phi_4650_ = false;
    }
    let _e689 = phi_4650_;
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
                phi_1292_ = 0u;
                break;
            }
            case 1: {
                phi_1292_ = 1u;
                break;
            }
            case 2: {
                phi_1292_ = 2u;
                break;
            }
            default: {
                phi_1292_ = 0u;
                break;
            }
        }
        let _e721 = phi_1292_;
        let _e725 = global.member[(_e289.member_7 + 7u)];
        switch bitcast<i32>(_e725) {
            case 0: {
                phi_1301_ = 0u;
                break;
            }
            case 1: {
                phi_1301_ = 1u;
                break;
            }
            case 2: {
                phi_1301_ = 2u;
                break;
            }
            default: {
                phi_1301_ = 0u;
                break;
            }
        }
        let _e728 = phi_1301_;
        phi_1314_ = type_15(type_14(_e721, _e728), vec2<u32>(_e692, _e696), vec2<u32>(_e701, _e705), _e710, _e714);
    } else {
        phi_1314_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e732 = phi_1314_;
    switch bitcast<i32>(_e732.member.member) {
        case 1: {
            let _e770 = abs(_e684.x);
            let _e772 = (_e770 % 1f);
            if (_e770 >= 1f) {
                phi_4701_ = select(true, false, (_e772 == 0f));
            } else {
                phi_4701_ = true;
            }
            let _e776 = phi_4701_;
            let _e777 = select(1f, _e772, _e776);
            if (select(-1f, 1f, (_e684.x >= 0f)) > 0f) {
                phi_1334_ = _e777;
            } else {
                phi_1334_ = (1f - _e777);
            }
            let _e781 = phi_1334_;
            phi_1371_ = _e781;
            break;
        }
        case 2: {
            let _e744 = abs(_e684.x);
            let _e751 = ((select(select(u32(_e744), 0u, (_e744 < 0f)), 4294967295u, (_e744 > 4294967000f)) % 2u) == 0u);
            let _e753 = (_e744 % 1f);
            if (_e744 >= 1f) {
                phi_4684_ = select(true, false, (_e753 == 0f));
            } else {
                phi_4684_ = true;
            }
            let _e757 = phi_4684_;
            let _e758 = select(1f, _e753, _e757);
            if (select(-1f, 1f, (_e684.x >= 0f)) > 0f) {
                if _e751 {
                    phi_1363_ = _e758;
                } else {
                    phi_1363_ = (1f - _e758);
                }
                let _e765 = phi_1363_;
                phi_1369_ = _e765;
            } else {
                if _e751 {
                    phi_1368_ = (1f - _e758);
                } else {
                    phi_1368_ = _e758;
                }
                let _e762 = phi_1368_;
                phi_1369_ = _e762;
            }
            let _e767 = phi_1369_;
            phi_1371_ = _e767;
            break;
        }
        case 0: {
            if (_e684.x > 1f) {
                phi_4671_ = 0.9999999f;
            } else {
                phi_4671_ = select(_e684.x, 0.00000011920929f, (_e684.x < 0f));
            }
            let _e741 = phi_4671_;
            phi_1371_ = _e741;
            break;
        }
        default: {
            phi_1371_ = f32();
            break;
        }
    }
    let _e783 = phi_1371_;
    switch bitcast<i32>(_e732.member.member_1) {
        case 1: {
            let _e821 = abs(_e684.y);
            let _e823 = (_e821 % 1f);
            if (_e821 >= 1f) {
                phi_4749_ = select(true, false, (_e823 == 0f));
            } else {
                phi_4749_ = true;
            }
            let _e827 = phi_4749_;
            let _e828 = select(1f, _e823, _e827);
            if (select(-1f, 1f, (_e684.y >= 0f)) > 0f) {
                phi_1392_ = _e828;
            } else {
                phi_1392_ = (1f - _e828);
            }
            let _e832 = phi_1392_;
            phi_1429_ = _e832;
            break;
        }
        case 2: {
            let _e795 = abs(_e684.y);
            let _e802 = ((select(select(u32(_e795), 0u, (_e795 < 0f)), 4294967295u, (_e795 > 4294967000f)) % 2u) == 0u);
            let _e804 = (_e795 % 1f);
            if (_e795 >= 1f) {
                phi_4732_ = select(true, false, (_e804 == 0f));
            } else {
                phi_4732_ = true;
            }
            let _e808 = phi_4732_;
            let _e809 = select(1f, _e804, _e808);
            if (select(-1f, 1f, (_e684.y >= 0f)) > 0f) {
                if _e802 {
                    phi_1421_ = _e809;
                } else {
                    phi_1421_ = (1f - _e809);
                }
                let _e816 = phi_1421_;
                phi_1427_ = _e816;
            } else {
                if _e802 {
                    phi_1426_ = (1f - _e809);
                } else {
                    phi_1426_ = _e809;
                }
                let _e813 = phi_1426_;
                phi_1427_ = _e813;
            }
            let _e818 = phi_1427_;
            phi_1429_ = _e818;
            break;
        }
        case 0: {
            if (_e684.y > 1f) {
                phi_4719_ = 0.9999999f;
            } else {
                phi_4719_ = select(_e684.y, 0.00000011920929f, (_e684.y < 0f));
            }
            let _e792 = phi_4719_;
            phi_1429_ = _e792;
            break;
        }
        default: {
            phi_1429_ = f32();
            break;
        }
    }
    let _e834 = phi_1429_;
    let _e838 = (_e783 * f32(_e732.member_2.x));
    let _e847 = (_e834 * f32(_e732.member_2.y));
    let _e865 = vec3<f32>((f32((select(select(u32(_e838), 0u, (_e838 < 0f)), 4294967295u, (_e838 > 4294967000f)) + _e732.member_1.x)) / _e469), (f32((select(select(u32(_e847), 0u, (_e847 < 0f)), 4294967295u, (_e847 > 4294967000f)) + _e732.member_1.y)) / _e470), f32(_e732.member_3));
    let _e871 = textureSampleLevel(global_11, global_10, vec2<f32>(_e865.x, _e865.y), i32(_e865.z), 0f);
    let _e872 = (_e289.member_7 == 4294967295u);
    let _e874 = select(_e871, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e872));
    let _e878 = select(_e122, _e121, vec2((_e289.member_13 == 0u)));
    if _e295 {
        phi_4785_ = (_e289.member_8 <= (_e116 - 8u));
    } else {
        phi_4785_ = false;
    }
    let _e883 = phi_4785_;
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
                phi_1512_ = 0u;
                break;
            }
            case 1: {
                phi_1512_ = 1u;
                break;
            }
            case 2: {
                phi_1512_ = 2u;
                break;
            }
            default: {
                phi_1512_ = 0u;
                break;
            }
        }
        let _e915 = phi_1512_;
        let _e919 = global.member[(_e289.member_8 + 7u)];
        switch bitcast<i32>(_e919) {
            case 0: {
                phi_1521_ = 0u;
                break;
            }
            case 1: {
                phi_1521_ = 1u;
                break;
            }
            case 2: {
                phi_1521_ = 2u;
                break;
            }
            default: {
                phi_1521_ = 0u;
                break;
            }
        }
        let _e922 = phi_1521_;
        phi_1534_ = type_15(type_14(_e915, _e922), vec2<u32>(_e886, _e890), vec2<u32>(_e895, _e899), _e904, _e908);
    } else {
        phi_1534_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e926 = phi_1534_;
    switch bitcast<i32>(_e926.member.member) {
        case 1: {
            let _e964 = abs(_e878.x);
            let _e966 = (_e964 % 1f);
            if (_e964 >= 1f) {
                phi_4836_ = select(true, false, (_e966 == 0f));
            } else {
                phi_4836_ = true;
            }
            let _e970 = phi_4836_;
            let _e971 = select(1f, _e966, _e970);
            if (select(-1f, 1f, (_e878.x >= 0f)) > 0f) {
                phi_1554_ = _e971;
            } else {
                phi_1554_ = (1f - _e971);
            }
            let _e975 = phi_1554_;
            phi_1591_ = _e975;
            break;
        }
        case 2: {
            let _e938 = abs(_e878.x);
            let _e945 = ((select(select(u32(_e938), 0u, (_e938 < 0f)), 4294967295u, (_e938 > 4294967000f)) % 2u) == 0u);
            let _e947 = (_e938 % 1f);
            if (_e938 >= 1f) {
                phi_4819_ = select(true, false, (_e947 == 0f));
            } else {
                phi_4819_ = true;
            }
            let _e951 = phi_4819_;
            let _e952 = select(1f, _e947, _e951);
            if (select(-1f, 1f, (_e878.x >= 0f)) > 0f) {
                if _e945 {
                    phi_1583_ = _e952;
                } else {
                    phi_1583_ = (1f - _e952);
                }
                let _e959 = phi_1583_;
                phi_1589_ = _e959;
            } else {
                if _e945 {
                    phi_1588_ = (1f - _e952);
                } else {
                    phi_1588_ = _e952;
                }
                let _e956 = phi_1588_;
                phi_1589_ = _e956;
            }
            let _e961 = phi_1589_;
            phi_1591_ = _e961;
            break;
        }
        case 0: {
            if (_e878.x > 1f) {
                phi_4806_ = 0.9999999f;
            } else {
                phi_4806_ = select(_e878.x, 0.00000011920929f, (_e878.x < 0f));
            }
            let _e935 = phi_4806_;
            phi_1591_ = _e935;
            break;
        }
        default: {
            phi_1591_ = f32();
            break;
        }
    }
    let _e977 = phi_1591_;
    switch bitcast<i32>(_e926.member.member_1) {
        case 1: {
            let _e1015 = abs(_e878.y);
            let _e1017 = (_e1015 % 1f);
            if (_e1015 >= 1f) {
                phi_4884_ = select(true, false, (_e1017 == 0f));
            } else {
                phi_4884_ = true;
            }
            let _e1021 = phi_4884_;
            let _e1022 = select(1f, _e1017, _e1021);
            if (select(-1f, 1f, (_e878.y >= 0f)) > 0f) {
                phi_1612_ = _e1022;
            } else {
                phi_1612_ = (1f - _e1022);
            }
            let _e1026 = phi_1612_;
            phi_1649_ = _e1026;
            break;
        }
        case 2: {
            let _e989 = abs(_e878.y);
            let _e996 = ((select(select(u32(_e989), 0u, (_e989 < 0f)), 4294967295u, (_e989 > 4294967000f)) % 2u) == 0u);
            let _e998 = (_e989 % 1f);
            if (_e989 >= 1f) {
                phi_4867_ = select(true, false, (_e998 == 0f));
            } else {
                phi_4867_ = true;
            }
            let _e1002 = phi_4867_;
            let _e1003 = select(1f, _e998, _e1002);
            if (select(-1f, 1f, (_e878.y >= 0f)) > 0f) {
                if _e996 {
                    phi_1641_ = _e1003;
                } else {
                    phi_1641_ = (1f - _e1003);
                }
                let _e1010 = phi_1641_;
                phi_1647_ = _e1010;
            } else {
                if _e996 {
                    phi_1646_ = (1f - _e1003);
                } else {
                    phi_1646_ = _e1003;
                }
                let _e1007 = phi_1646_;
                phi_1647_ = _e1007;
            }
            let _e1012 = phi_1647_;
            phi_1649_ = _e1012;
            break;
        }
        case 0: {
            if (_e878.y > 1f) {
                phi_4854_ = 0.9999999f;
            } else {
                phi_4854_ = select(_e878.y, 0.00000011920929f, (_e878.y < 0f));
            }
            let _e986 = phi_4854_;
            phi_1649_ = _e986;
            break;
        }
        default: {
            phi_1649_ = f32();
            break;
        }
    }
    let _e1028 = phi_1649_;
    let _e1032 = (_e977 * f32(_e926.member_2.x));
    let _e1041 = (_e1028 * f32(_e926.member_2.y));
    let _e1059 = vec3<f32>((f32((select(select(u32(_e1032), 0u, (_e1032 < 0f)), 4294967295u, (_e1032 > 4294967000f)) + _e926.member_1.x)) / _e469), (f32((select(select(u32(_e1041), 0u, (_e1041 < 0f)), 4294967295u, (_e1041 > 4294967000f)) + _e926.member_1.y)) / _e470), f32(_e926.member_3));
    let _e1065 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1059.x, _e1059.y), i32(_e1059.z), 0f);
    let _e1072 = select(_e122, _e121, vec2((_e289.member_14 == 0u)));
    if _e295 {
        phi_4920_ = (_e289.member_9 <= (_e116 - 8u));
    } else {
        phi_4920_ = false;
    }
    let _e1077 = phi_4920_;
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
                phi_1732_ = 0u;
                break;
            }
            case 1: {
                phi_1732_ = 1u;
                break;
            }
            case 2: {
                phi_1732_ = 2u;
                break;
            }
            default: {
                phi_1732_ = 0u;
                break;
            }
        }
        let _e1109 = phi_1732_;
        let _e1113 = global.member[(_e289.member_9 + 7u)];
        switch bitcast<i32>(_e1113) {
            case 0: {
                phi_1741_ = 0u;
                break;
            }
            case 1: {
                phi_1741_ = 1u;
                break;
            }
            case 2: {
                phi_1741_ = 2u;
                break;
            }
            default: {
                phi_1741_ = 0u;
                break;
            }
        }
        let _e1116 = phi_1741_;
        phi_1754_ = type_15(type_14(_e1109, _e1116), vec2<u32>(_e1080, _e1084), vec2<u32>(_e1089, _e1093), _e1098, _e1102);
    } else {
        phi_1754_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1120 = phi_1754_;
    switch bitcast<i32>(_e1120.member.member) {
        case 1: {
            let _e1158 = abs(_e1072.x);
            let _e1160 = (_e1158 % 1f);
            if (_e1158 >= 1f) {
                phi_4971_ = select(true, false, (_e1160 == 0f));
            } else {
                phi_4971_ = true;
            }
            let _e1164 = phi_4971_;
            let _e1165 = select(1f, _e1160, _e1164);
            if (select(-1f, 1f, (_e1072.x >= 0f)) > 0f) {
                phi_1774_ = _e1165;
            } else {
                phi_1774_ = (1f - _e1165);
            }
            let _e1169 = phi_1774_;
            phi_1811_ = _e1169;
            break;
        }
        case 2: {
            let _e1132 = abs(_e1072.x);
            let _e1139 = ((select(select(u32(_e1132), 0u, (_e1132 < 0f)), 4294967295u, (_e1132 > 4294967000f)) % 2u) == 0u);
            let _e1141 = (_e1132 % 1f);
            if (_e1132 >= 1f) {
                phi_4954_ = select(true, false, (_e1141 == 0f));
            } else {
                phi_4954_ = true;
            }
            let _e1145 = phi_4954_;
            let _e1146 = select(1f, _e1141, _e1145);
            if (select(-1f, 1f, (_e1072.x >= 0f)) > 0f) {
                if _e1139 {
                    phi_1803_ = _e1146;
                } else {
                    phi_1803_ = (1f - _e1146);
                }
                let _e1153 = phi_1803_;
                phi_1809_ = _e1153;
            } else {
                if _e1139 {
                    phi_1808_ = (1f - _e1146);
                } else {
                    phi_1808_ = _e1146;
                }
                let _e1150 = phi_1808_;
                phi_1809_ = _e1150;
            }
            let _e1155 = phi_1809_;
            phi_1811_ = _e1155;
            break;
        }
        case 0: {
            if (_e1072.x > 1f) {
                phi_4941_ = 0.9999999f;
            } else {
                phi_4941_ = select(_e1072.x, 0.00000011920929f, (_e1072.x < 0f));
            }
            let _e1129 = phi_4941_;
            phi_1811_ = _e1129;
            break;
        }
        default: {
            phi_1811_ = f32();
            break;
        }
    }
    let _e1171 = phi_1811_;
    switch bitcast<i32>(_e1120.member.member_1) {
        case 1: {
            let _e1209 = abs(_e1072.y);
            let _e1211 = (_e1209 % 1f);
            if (_e1209 >= 1f) {
                phi_5019_ = select(true, false, (_e1211 == 0f));
            } else {
                phi_5019_ = true;
            }
            let _e1215 = phi_5019_;
            let _e1216 = select(1f, _e1211, _e1215);
            if (select(-1f, 1f, (_e1072.y >= 0f)) > 0f) {
                phi_1832_ = _e1216;
            } else {
                phi_1832_ = (1f - _e1216);
            }
            let _e1220 = phi_1832_;
            phi_1869_ = _e1220;
            break;
        }
        case 2: {
            let _e1183 = abs(_e1072.y);
            let _e1190 = ((select(select(u32(_e1183), 0u, (_e1183 < 0f)), 4294967295u, (_e1183 > 4294967000f)) % 2u) == 0u);
            let _e1192 = (_e1183 % 1f);
            if (_e1183 >= 1f) {
                phi_5002_ = select(true, false, (_e1192 == 0f));
            } else {
                phi_5002_ = true;
            }
            let _e1196 = phi_5002_;
            let _e1197 = select(1f, _e1192, _e1196);
            if (select(-1f, 1f, (_e1072.y >= 0f)) > 0f) {
                if _e1190 {
                    phi_1861_ = _e1197;
                } else {
                    phi_1861_ = (1f - _e1197);
                }
                let _e1204 = phi_1861_;
                phi_1867_ = _e1204;
            } else {
                if _e1190 {
                    phi_1866_ = (1f - _e1197);
                } else {
                    phi_1866_ = _e1197;
                }
                let _e1201 = phi_1866_;
                phi_1867_ = _e1201;
            }
            let _e1206 = phi_1867_;
            phi_1869_ = _e1206;
            break;
        }
        case 0: {
            if (_e1072.y > 1f) {
                phi_4989_ = 0.9999999f;
            } else {
                phi_4989_ = select(_e1072.y, 0.00000011920929f, (_e1072.y < 0f));
            }
            let _e1180 = phi_4989_;
            phi_1869_ = _e1180;
            break;
        }
        default: {
            phi_1869_ = f32();
            break;
        }
    }
    let _e1222 = phi_1869_;
    let _e1226 = (_e1171 * f32(_e1120.member_2.x));
    let _e1235 = (_e1222 * f32(_e1120.member_2.y));
    let _e1253 = vec3<f32>((f32((select(select(u32(_e1226), 0u, (_e1226 < 0f)), 4294967295u, (_e1226 > 4294967000f)) + _e1120.member_1.x)) / _e469), (f32((select(select(u32(_e1235), 0u, (_e1235 < 0f)), 4294967295u, (_e1235 > 4294967000f)) + _e1120.member_1.y)) / _e470), f32(_e1120.member_3));
    let _e1259 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1253.x, _e1253.y), i32(_e1253.z), 0f);
    let _e1262 = select(_e1259, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e289.member_9 == 4294967295u)));
    if _e872 {
        phi_1963_ = vec3<f32>(0f, 0f, 0f);
        phi_1964_ = _e123;
    } else {
        let _e1266 = fma(_e874.x, 2f, -1f);
        let _e1267 = fma(_e874.y, 2f, -1f);
        let _e1268 = fma(_e874.z, 2f, -1f);
        let _e1273 = sqrt(fma(_e1268, _e1268, fma(_e1266, _e1266, (_e1267 * _e1267))));
        if (_e1273 == 0f) {
            phi_5077_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_5077_ = (vec3<f32>(_e1266, _e1267, _e1268) * (1f / _e1273));
        }
        let _e1278 = phi_5077_;
        let _e1285 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
        if (_e1285 == 0f) {
            phi_5112_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_5112_ = (_e124 * (1f / _e1285));
        }
        let _e1290 = phi_5112_;
        let _e1297 = sqrt(fma(_e125.z, _e125.z, fma(_e125.x, _e125.x, (_e125.y * _e125.y))));
        if (_e1297 == 0f) {
            phi_5147_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_5147_ = (_e125 * (1f / _e1297));
        }
        let _e1302 = phi_5147_;
        let _e1309 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
        if (_e1309 == 0f) {
            phi_5182_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_5182_ = (_e123 * (1f / _e1309));
        }
        let _e1314 = phi_5182_;
        let _e1333 = fma(_e1314.x, _e1278.z, fma(_e1290.x, _e1278.x, (_e1302.x * _e1278.y)));
        let _e1334 = fma(_e1314.y, _e1278.z, fma(_e1290.y, _e1278.x, (_e1302.y * _e1278.y)));
        let _e1335 = fma(_e1314.z, _e1278.z, fma(_e1290.z, _e1278.x, (_e1302.z * _e1278.y)));
        let _e1340 = sqrt(fma(_e1335, _e1335, fma(_e1333, _e1333, (_e1334 * _e1334))));
        if (_e1340 == 0f) {
            phi_5217_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_5217_ = (vec3<f32>(_e1333, _e1334, _e1335) * (1f / _e1340));
        }
        let _e1345 = phi_5217_;
        phi_1963_ = _e1278;
        phi_1964_ = _e1345;
    }
    let _e1347 = phi_1963_;
    let _e1349 = phi_1964_;
    let _e1353 = (_e486.x * _e289.member_2.x);
    let _e1356 = (_e486.y * _e289.member_2.y);
    let _e1359 = (_e486.z * _e289.member_2.z);
    let _e1364 = (_e1353 * _e120.x);
    let _e1366 = (_e1356 * _e120.y);
    let _e1368 = (_e1359 * _e120.z);
    let _e1373 = (_e680.y * _e289.member_4);
    let _e1376 = (_e680.z * _e289.member_3);
    let _e1380 = fma(_e289.member_16, (select(_e1065, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e289.member_8 == 4294967295u))).x - 1f), 1f);
    let _e1386 = (_e1262.x * _e289.member.x);
    let _e1388 = (_e1262.y * _e289.member.y);
    let _e1390 = (_e1262.z * _e289.member.z);
    let _e1395 = textureSampleLevel(global_12, global_13, _e1349, 0f);
    if (_e116 >= 86u) {
        phi_5249_ = (_e130 <= (_e116 - 86u));
    } else {
        phi_5249_ = false;
    }
    let _e1403 = phi_5249_;
    if _e1403 {
        let _e1406 = global.member[_e130];
        let _e1411 = global.member[(_e130 + 1u)];
        let _e1416 = global.member[(_e130 + 2u)];
        let _e1421 = global.member[(_e130 + 3u)];
        let _e1427 = global.member[(_e130 + 4u)];
        let _e1432 = global.member[(_e130 + 5u)];
        let _e1437 = global.member[(_e130 + 6u)];
        let _e1442 = global.member[(_e130 + 7u)];
        let _e1448 = global.member[(_e130 + 8u)];
        let _e1453 = global.member[(_e130 + 9u)];
        let _e1458 = global.member[(_e130 + 10u)];
        let _e1463 = global.member[(_e130 + 11u)];
        let _e1469 = global.member[(_e130 + 12u)];
        let _e1474 = global.member[(_e130 + 13u)];
        let _e1479 = global.member[(_e130 + 14u)];
        let _e1484 = global.member[(_e130 + 15u)];
        let _e1491 = global.member[(_e130 + 16u)];
        let _e1496 = global.member[(_e130 + 17u)];
        let _e1501 = global.member[(_e130 + 18u)];
        let _e1506 = global.member[(_e130 + 19u)];
        let _e1512 = global.member[(_e130 + 20u)];
        let _e1517 = global.member[(_e130 + 21u)];
        let _e1522 = global.member[(_e130 + 22u)];
        let _e1527 = global.member[(_e130 + 23u)];
        let _e1533 = global.member[(_e130 + 24u)];
        let _e1538 = global.member[(_e130 + 25u)];
        let _e1543 = global.member[(_e130 + 26u)];
        let _e1548 = global.member[(_e130 + 27u)];
        let _e1554 = global.member[(_e130 + 28u)];
        let _e1559 = global.member[(_e130 + 29u)];
        let _e1564 = global.member[(_e130 + 30u)];
        let _e1569 = global.member[(_e130 + 31u)];
        let _e1576 = global.member[(_e130 + 32u)];
        let _e1581 = global.member[(_e130 + 33u)];
        let _e1586 = global.member[(_e130 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_2171_ = type_14(0u, 6u);
        loop {
            let _e1591 = phi_2171_;
            if (_e1591.member < _e1591.member_1) {
                phi_2172_ = type_14((_e1591.member + 1u), _e1591.member_1);
                phi_2195_ = type_14(1u, _e1591.member);
            } else {
                phi_2172_ = _e1591;
                phi_2195_ = type_14(0u, type_14().member_1);
            }
            let _e1604 = phi_2172_;
            let _e1606 = phi_2195_;
            switch bitcast<i32>(_e1606.member) {
                case 0: {
                    phi_2222_ = false;
                    break;
                }
                case 1: {
                    let _e1611 = ((_e130 + 35u) + (_e1606.member_1 * 4u));
                    let _e1614 = global.member[_e1611];
                    let _e1619 = global.member[(_e1611 + 1u)];
                    let _e1624 = global.member[(_e1611 + 2u)];
                    let _e1629 = global.member[(_e1611 + 3u)];
                    local_1[_e1606.member_1] = vec4<f32>(bitcast<f32>(_e1614), bitcast<f32>(_e1619), bitcast<f32>(_e1624), bitcast<f32>(_e1629));
                    phi_2222_ = true;
                    break;
                }
                default: {
                    phi_2222_ = bool();
                    break;
                }
            }
            let _e1634 = phi_2222_;
            continue;
            continuing {
                phi_2171_ = _e1604;
                break if !(_e1634);
            }
        }
        let _e1636 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2228_ = type_14(0u, 8u);
        loop {
            let _e1639 = phi_2228_;
            if (_e1639.member < _e1639.member_1) {
                phi_2229_ = type_14((_e1639.member + 1u), _e1639.member_1);
                phi_2252_ = type_14(1u, _e1639.member);
            } else {
                phi_2229_ = _e1639;
                phi_2252_ = type_14(0u, type_14().member_1);
            }
            let _e1652 = phi_2229_;
            let _e1654 = phi_2252_;
            switch bitcast<i32>(_e1654.member) {
                case 0: {
                    phi_2275_ = false;
                    break;
                }
                case 1: {
                    let _e1659 = ((_e130 + 59u) + (_e1654.member_1 * 3u));
                    let _e1662 = global.member[_e1659];
                    let _e1667 = global.member[(_e1659 + 1u)];
                    let _e1672 = global.member[(_e1659 + 2u)];
                    local[_e1654.member_1] = vec3<f32>(bitcast<f32>(_e1662), bitcast<f32>(_e1667), bitcast<f32>(_e1672));
                    phi_2275_ = true;
                    break;
                }
                default: {
                    phi_2275_ = bool();
                    break;
                }
            }
            let _e1677 = phi_2275_;
            continue;
            continuing {
                phi_2228_ = _e1652;
                break if !(_e1677);
            }
        }
        let _e1679 = local;
        let _e1683 = global.member[(_e130 + 83u)];
        let _e1688 = global.member[(_e130 + 84u)];
        let _e1693 = global.member[(_e130 + 85u)];
        phi_2296_ = type_25(type_23(vec4<f32>(bitcast<f32>(_e1406), bitcast<f32>(_e1411), bitcast<f32>(_e1416), bitcast<f32>(_e1421)), vec4<f32>(bitcast<f32>(_e1427), bitcast<f32>(_e1432), bitcast<f32>(_e1437), bitcast<f32>(_e1442)), vec4<f32>(bitcast<f32>(_e1448), bitcast<f32>(_e1453), bitcast<f32>(_e1458), bitcast<f32>(_e1463)), vec4<f32>(bitcast<f32>(_e1469), bitcast<f32>(_e1474), bitcast<f32>(_e1479), bitcast<f32>(_e1484))), type_23(vec4<f32>(bitcast<f32>(_e1491), bitcast<f32>(_e1496), bitcast<f32>(_e1501), bitcast<f32>(_e1506)), vec4<f32>(bitcast<f32>(_e1512), bitcast<f32>(_e1517), bitcast<f32>(_e1522), bitcast<f32>(_e1527)), vec4<f32>(bitcast<f32>(_e1533), bitcast<f32>(_e1538), bitcast<f32>(_e1543), bitcast<f32>(_e1548)), vec4<f32>(bitcast<f32>(_e1554), bitcast<f32>(_e1559), bitcast<f32>(_e1564), bitcast<f32>(_e1569))), vec3<f32>(bitcast<f32>(_e1576), bitcast<f32>(_e1581), bitcast<f32>(_e1586)), type_24(_e1679, _e1636, vec3<f32>(bitcast<f32>(_e1683), bitcast<f32>(_e1688), bitcast<f32>(_e1693))));
    } else {
        phi_2296_ = type_25(type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_24(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e1699 = phi_2296_;
    let _e1701 = (_e1699.member_2 - _e126);
    let _e1708 = sqrt(fma(_e1701.z, _e1701.z, fma(_e1701.x, _e1701.x, (_e1701.y * _e1701.y))));
    let _e1709 = (_e1708 == 0f);
    if _e1709 {
        phi_5321_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5321_ = (_e1701 * (1f / _e1708));
    }
    let _e1713 = phi_5321_;
    let _e1714 = -(_e1713);
    let _e1721 = sqrt(fma(_e1349.z, _e1349.z, fma(_e1349.x, _e1349.x, (_e1349.y * _e1349.y))));
    let _e1722 = (_e1721 == 0f);
    if _e1722 {
        phi_5380_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5380_ = (_e1349 * (1f / _e1721));
    }
    let _e1726 = phi_5380_;
    let _e1736 = (2f * fma(_e1726.z, _e1714.z, fma(_e1726.x, _e1714.x, (_e1726.y * _e1714.y))));
    let _e1743 = textureSampleLevel(global_14, global_15, (_e1714 - vec3<f32>((_e1736 * _e1726.x), (_e1736 * _e1726.y), (_e1736 * _e1726.z))), (_e1373 * 4f));
    if _e1709 {
        phi_5454_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5454_ = (_e1701 * (1f / _e1708));
    }
    let _e1750 = phi_5454_;
    let _e1759 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1349.z, _e1750.z, fma(_e1349.x, _e1750.x, (_e1349.y * _e1750.y))), 0f), _e1373), 0f);
    switch bitcast<i32>(_e152) {
        case 0: {
            if _e289.member_15 {
                if _e1722 {
                    phi_5847_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5847_ = (_e1349 * (1f / _e1721));
                }
                let _e1928 = phi_5847_;
                if _e1709 {
                    phi_5882_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5882_ = (_e1701 * (1f / _e1708));
                }
                let _e1932 = phi_5882_;
                let _e1935 = global_1.member[0u];
                let _e1938 = global_1.member[1u];
                let _e1941 = global_1.member[2u];
                phi_2363_ = type_14(0u, _e1938);
                phi_2366_ = vec3<f32>(0f, 0f, 0f);
                loop {
                    let _e1944 = phi_2363_;
                    let _e1946 = phi_2366_;
                    local_2 = _e1946;
                    local_3 = _e1946;
                    local_4 = _e1946;
                    if (_e1944.member < _e1944.member_1) {
                        phi_2364_ = type_14((_e1944.member + 1u), _e1944.member_1);
                        phi_2389_ = type_14(1u, _e1944.member);
                    } else {
                        phi_2364_ = _e1944;
                        phi_2389_ = type_14(0u, type_14().member_1);
                    }
                    let _e1959 = phi_2364_;
                    let _e1961 = phi_2389_;
                    switch bitcast<i32>(_e1961.member) {
                        case 0: {
                            phi_2367_ = vec3<f32>();
                            phi_3682_ = false;
                            break;
                        }
                        case 1: {
                            if (_e1961.member_1 >= _e1938) {
                                phi_5908_ = 4294967295u;
                            } else {
                                phi_5908_ = (_e1935 + _e1961.member_1);
                            }
                            let _e1968 = phi_5908_;
                            if (_e118 >= 1u) {
                                phi_5927_ = (_e1968 <= (_e118 - 1u));
                            } else {
                                phi_5927_ = false;
                            }
                            let _e1973 = phi_5927_;
                            if _e1973 {
                                let _e1976 = global_1.member[_e1968];
                                phi_2406_ = _e1976;
                            } else {
                                phi_2406_ = 4294967295u;
                            }
                            let _e1978 = phi_2406_;
                            let _e1979 = (_e1978 == 4294967295u);
                            if _e1979 {
                                phi_3680_ = vec3<f32>();
                            } else {
                                if (_e118 >= 4u) {
                                    phi_5959_ = (_e1978 <= (_e118 - 4u));
                                } else {
                                    phi_5959_ = false;
                                }
                                let _e1984 = phi_5959_;
                                if _e1984 {
                                    let _e1987 = global_1.member[_e1978];
                                    switch bitcast<i32>(_e1987) {
                                        case 0: {
                                            phi_2423_ = 0u;
                                            break;
                                        }
                                        case 1: {
                                            phi_2423_ = 1u;
                                            break;
                                        }
                                        case 2: {
                                            phi_2423_ = 2u;
                                            break;
                                        }
                                        default: {
                                            phi_2423_ = 0u;
                                            break;
                                        }
                                    }
                                    let _e1990 = phi_2423_;
                                    let _e1994 = global_1.member[(_e1978 + 1u)];
                                    let _e1998 = global_1.member[(_e1978 + 2u)];
                                    let _e2002 = global_1.member[(_e1978 + 3u)];
                                    phi_2437_ = type_30(_e1990, _e1994, _e1998, _e2002);
                                } else {
                                    phi_2437_ = type_30(0u, 4294967295u, 4294967295u, 4294967295u);
                                }
                                let _e2005 = phi_2437_;
                                if (_e118 >= 10u) {
                                    phi_5991_ = (_e2005.member_2 <= (_e118 - 10u));
                                } else {
                                    phi_5991_ = false;
                                }
                                let _e2011 = phi_5991_;
                                if _e2011 {
                                    let _e2014 = global_1.member[_e2005.member_2];
                                    let _e2019 = global_1.member[(_e2005.member_2 + 1u)];
                                    let _e2024 = global_1.member[(_e2005.member_2 + 2u)];
                                    let _e2030 = global_1.member[(_e2005.member_2 + 3u)];
                                    let _e2035 = global_1.member[(_e2005.member_2 + 4u)];
                                    let _e2040 = global_1.member[(_e2005.member_2 + 5u)];
                                    let _e2045 = global_1.member[(_e2005.member_2 + 6u)];
                                    let _e2051 = global_1.member[(_e2005.member_2 + 7u)];
                                    let _e2056 = global_1.member[(_e2005.member_2 + 8u)];
                                    let _e2061 = global_1.member[(_e2005.member_2 + 9u)];
                                    phi_2487_ = type_31(vec3<f32>(bitcast<f32>(_e2014), bitcast<f32>(_e2019), bitcast<f32>(_e2024)), vec4<f32>(bitcast<f32>(_e2030), bitcast<f32>(_e2035), bitcast<f32>(_e2040), bitcast<f32>(_e2045)), vec3<f32>(bitcast<f32>(_e2051), bitcast<f32>(_e2056), bitcast<f32>(_e2061)));
                                } else {
                                    phi_2487_ = type_31(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e2066 = phi_2487_;
                                let _e2074 = (_e2066.member_1.x + _e2066.member_1.x);
                                let _e2075 = (_e2066.member_1.y + _e2066.member_1.y);
                                let _e2076 = (_e2066.member_1.z + _e2066.member_1.z);
                                let _e2078 = (_e2066.member_1.z * _e2076);
                                let _e2079 = (_e2066.member_1.w * _e2074);
                                let _e2080 = (_e2066.member_1.w * _e2075);
                                let _e2081 = (_e2066.member_1.w * _e2076);
                                let _e2101 = (vec4<f32>((1f - fma(_e2066.member_1.y, _e2075, _e2078)), fma(_e2066.member_1.x, _e2075, _e2081), fma(_e2066.member_1.x, _e2076, -(_e2080)), 0f) * _e2066.member_2.x);
                                let _e2103 = (vec4<f32>(fma(_e2066.member_1.x, _e2075, -(_e2081)), (1f - fma(_e2066.member_1.x, _e2074, _e2078)), fma(_e2066.member_1.y, _e2076, _e2079), 0f) * _e2066.member_2.y);
                                let _e2105 = (vec4<f32>(fma(_e2066.member_1.x, _e2076, _e2080), fma(_e2066.member_1.y, _e2076, -(_e2079)), (1f - fma(_e2066.member_1.x, _e2074, (_e2066.member_1.y * _e2075))), 0f) * _e2066.member_2.z);
                                switch bitcast<i32>(_e2005.member) {
                                    case 0: {
                                        if (_e118 >= 8u) {
                                            phi_6457_ = (_e2005.member_1 <= (_e118 - 8u));
                                        } else {
                                            phi_6457_ = false;
                                        }
                                        let _e2603 = phi_6457_;
                                        if _e2603 {
                                            let _e2606 = global_1.member[_e2005.member_1];
                                            let _e2611 = global_1.member[(_e2005.member_1 + 1u)];
                                            let _e2616 = global_1.member[(_e2005.member_1 + 2u)];
                                            let _e2622 = global_1.member[(_e2005.member_1 + 3u)];
                                            let _e2627 = global_1.member[(_e2005.member_1 + 4u)];
                                            let _e2632 = global_1.member[(_e2005.member_1 + 5u)];
                                            let _e2637 = global_1.member[(_e2005.member_1 + 6u)];
                                            let _e2643 = global_1.member[(_e2005.member_1 + 7u)];
                                            phi_2535_ = type_34(vec3<f32>(bitcast<f32>(_e2606), bitcast<f32>(_e2611), bitcast<f32>(_e2616)), vec4<f32>(bitcast<f32>(_e2622), bitcast<f32>(_e2627), bitcast<f32>(_e2632), bitcast<f32>(_e2637)), bitcast<f32>(_e2643));
                                        } else {
                                            phi_2535_ = type_34(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2647 = phi_2535_;
                                        let _e2669 = fma(_e2105.x, _e2647.member.z, fma(_e2103.x, _e2647.member.y, (_e2101.x * _e2647.member.x)));
                                        let _e2670 = fma(_e2105.y, _e2647.member.z, fma(_e2103.y, _e2647.member.y, (_e2101.y * _e2647.member.x)));
                                        let _e2671 = fma(_e2105.z, _e2647.member.z, fma(_e2103.z, _e2647.member.y, (_e2101.z * _e2647.member.x)));
                                        let _e2676 = sqrt(fma(_e2671, _e2671, fma(_e2669, _e2669, (_e2670 * _e2670))));
                                        if (_e2676 == 0f) {
                                            phi_6504_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6504_ = (vec3<f32>(_e2669, _e2670, _e2671) * (1f / _e2676));
                                        }
                                        let _e2681 = phi_6504_;
                                        let _e2683 = -(_e2681.x);
                                        let _e2685 = -(_e2681.y);
                                        let _e2687 = -(_e2681.z);
                                        let _e2688 = -(_e2681);
                                        let _e2690 = fma(-(_e680.z), _e289.member_3, 1f);
                                        let _e2694 = fma(0.4f, _e2690, (_e1364 * _e1376));
                                        let _e2695 = fma(0.4f, _e2690, (_e1366 * _e1376));
                                        let _e2696 = fma(0.4f, _e2690, (_e1368 * _e1376));
                                        let _e2704 = (_e1932 + vec3<f32>(_e2683, _e2685, _e2687));
                                        let _e2711 = sqrt(fma(_e2704.z, _e2704.z, fma(_e2704.x, _e2704.x, (_e2704.y * _e2704.y))));
                                        if (_e2711 == 0f) {
                                            phi_6539_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6539_ = (_e2704 * (1f / _e2711));
                                        }
                                        let _e2716 = phi_6539_;
                                        let _e2717 = (_e1373 * _e1373);
                                        let _e2728 = max(fma(_e1928.z, _e2716.z, fma(_e1928.x, _e2716.x, (_e1928.y * _e2716.y))), 0f);
                                        let _e2741 = max(fma(_e1928.z, _e1932.z, fma(_e1928.x, _e1932.x, (_e1928.y * _e1932.y))), 0f);
                                        let _e2747 = fma(_e1928.z, _e2688.z, fma(_e1928.x, _e2688.x, (_e1928.y * _e2688.y)));
                                        let _e2748 = max(_e2747, 0f);
                                        let _e2749 = fma(_e680.y, _e289.member_4, 1f);
                                        let _e2750 = (_e2749 * _e2749);
                                        let _e2751 = (_e2750 * 0.125f);
                                        let _e2753 = fma(-(_e2750), 0.125f, 1f);
                                        let _e2766 = (1f - max(fma(_e2716.z, _e1932.z, fma(_e2716.x, _e1932.x, (_e2716.y * _e1932.y))), 0f));
                                        let _e2768 = select(_e2766, 0f, (_e2766 < 0f));
                                        let _e2771 = pow(select(_e2768, 1f, (_e2768 > 1f)), 5f);
                                        let _e2772 = fma((1f - _e2694), _e2771, _e2694);
                                        let _e2773 = fma((1f - _e2695), _e2771, _e2695);
                                        let _e2774 = fma((1f - _e2696), _e2771, _e2696);
                                        let _e2781 = (((_e2717 * _e2717) / (pow(fma((_e2728 * _e2728), fma(_e2717, _e2717, -1f), 1f), 2f) * 3.1415927f)) * ((_e2741 / fma(_e2741, _e2753, _e2751)) * (_e2748 / fma(_e2748, _e2753, _e2751))));
                                        let _e2788 = max(fma(_e1928.z, _e2687, fma(_e1928.x, _e2683, (_e1928.y * _e2685))), 0f);
                                        let _e2790 = fma((4f * _e2741), _e2788, 0.0001f);
                                        if ((_e2005.member_3 == 4294967295u) != true) {
                                            let _e2811 = global_1.member[_e2005.member_3];
                                            let _e2815 = global_1.member[(_e2005.member_3 + 1u)];
                                            let _e2819 = global_1.member[(_e2005.member_3 + 2u)];
                                            let _e2823 = global_1.member[(_e2005.member_3 + 3u)];
                                            let _e2827 = global_1.member[(_e2005.member_3 + 4u)];
                                            let _e2832 = global_1.member[(_e2005.member_3 + 5u)];
                                            let _e2838 = global_1.member[select(_e2819, 4294967295u, (0u >= _e2823))];
                                            let _e2841 = global_1.member[_e2838];
                                            let _e2845 = global_1.member[(_e2838 + 1u)];
                                            let _e2849 = global_1.member[(_e2838 + 2u)];
                                            let _e2853 = global_1.member[(_e2838 + 3u)];
                                            let _e2857 = global_1.member[(_e2838 + 4u)];
                                            let _e2861 = global_1.member[(_e2838 + 6u)];
                                            switch bitcast<i32>(_e2861) {
                                                case 0: {
                                                    phi_2772_ = 0u;
                                                    break;
                                                }
                                                case 1: {
                                                    phi_2772_ = 1u;
                                                    break;
                                                }
                                                case 2: {
                                                    phi_2772_ = 2u;
                                                    break;
                                                }
                                                default: {
                                                    phi_2772_ = 0u;
                                                    break;
                                                }
                                            }
                                            let _e2864 = phi_2772_;
                                            let _e2868 = global_1.member[(_e2838 + 7u)];
                                            switch bitcast<i32>(_e2868) {
                                                case 0: {
                                                    phi_2781_ = 0u;
                                                    break;
                                                }
                                                case 1: {
                                                    phi_2781_ = 1u;
                                                    break;
                                                }
                                                case 2: {
                                                    phi_2781_ = 2u;
                                                    break;
                                                }
                                                default: {
                                                    phi_2781_ = 0u;
                                                    break;
                                                }
                                            }
                                            let _e2871 = phi_2781_;
                                            let _e2874 = global_1.member[_e1941];
                                            let _e2878 = global_1.member[(_e1941 + 1u)];
                                            let _e2880 = select(_e2811, 4294967295u, (0u >= _e2815));
                                            let _e2883 = global_1.member[_e2880];
                                            let _e2888 = global_1.member[(_e2880 + 1u)];
                                            let _e2893 = global_1.member[(_e2880 + 2u)];
                                            let _e2898 = global_1.member[(_e2880 + 3u)];
                                            let _e2903 = global_1.member[(_e2880 + 4u)];
                                            let _e2908 = global_1.member[(_e2880 + 5u)];
                                            let _e2913 = global_1.member[(_e2880 + 6u)];
                                            let _e2918 = global_1.member[(_e2880 + 7u)];
                                            let _e2923 = global_1.member[(_e2880 + 8u)];
                                            let _e2928 = global_1.member[(_e2880 + 9u)];
                                            let _e2933 = global_1.member[(_e2880 + 10u)];
                                            let _e2938 = global_1.member[(_e2880 + 11u)];
                                            let _e2943 = global_1.member[(_e2880 + 12u)];
                                            let _e2948 = global_1.member[(_e2880 + 13u)];
                                            let _e2953 = global_1.member[(_e2880 + 14u)];
                                            let _e2958 = global_1.member[(_e2880 + 15u)];
                                            let _e2978 = (bitcast<f32>(_e2958) + fma(bitcast<f32>(_e2938), _e126.z, fma(bitcast<f32>(_e2918), _e126.y, (bitcast<f32>(_e2898) * _e126.x))));
                                            let _e2984 = ((((bitcast<f32>(_e2943) + fma(bitcast<f32>(_e2923), _e126.z, fma(bitcast<f32>(_e2903), _e126.y, (bitcast<f32>(_e2883) * _e126.x)))) / _e2978) + 1f) * 0.5f);
                                            let _e2985 = (fma(((bitcast<f32>(_e2948) + fma(bitcast<f32>(_e2928), _e126.z, fma(bitcast<f32>(_e2908), _e126.y, (bitcast<f32>(_e2888) * _e126.x)))) / _e2978), -1f, 1f) * 0.5f);
                                            switch bitcast<i32>(_e2864) {
                                                case 1: {
                                                    let _e3020 = abs(_e2984);
                                                    let _e3022 = (_e3020 % 1f);
                                                    if (_e3020 >= 1f) {
                                                        phi_6675_ = select(true, false, (_e3022 == 0f));
                                                    } else {
                                                        phi_6675_ = true;
                                                    }
                                                    let _e3026 = phi_6675_;
                                                    let _e3027 = select(1f, _e3022, _e3026);
                                                    if (select(-1f, 1f, (_e2984 >= 0f)) > 0f) {
                                                        phi_2953_ = _e3027;
                                                    } else {
                                                        phi_2953_ = (1f - _e3027);
                                                    }
                                                    let _e3031 = phi_2953_;
                                                    phi_2990_ = _e3031;
                                                    break;
                                                }
                                                case 2: {
                                                    let _e2994 = abs(_e2984);
                                                    let _e3001 = ((select(select(u32(_e2994), 0u, (_e2994 < 0f)), 4294967295u, (_e2994 > 4294967000f)) % 2u) == 0u);
                                                    let _e3003 = (_e2994 % 1f);
                                                    if (_e2994 >= 1f) {
                                                        phi_6658_ = select(true, false, (_e3003 == 0f));
                                                    } else {
                                                        phi_6658_ = true;
                                                    }
                                                    let _e3007 = phi_6658_;
                                                    let _e3008 = select(1f, _e3003, _e3007);
                                                    if (select(-1f, 1f, (_e2984 >= 0f)) > 0f) {
                                                        if _e3001 {
                                                            phi_2982_ = _e3008;
                                                        } else {
                                                            phi_2982_ = (1f - _e3008);
                                                        }
                                                        let _e3015 = phi_2982_;
                                                        phi_2988_ = _e3015;
                                                    } else {
                                                        if _e3001 {
                                                            phi_2987_ = (1f - _e3008);
                                                        } else {
                                                            phi_2987_ = _e3008;
                                                        }
                                                        let _e3012 = phi_2987_;
                                                        phi_2988_ = _e3012;
                                                    }
                                                    let _e3017 = phi_2988_;
                                                    phi_2990_ = _e3017;
                                                    break;
                                                }
                                                case 0: {
                                                    if (_e2984 > 1f) {
                                                        phi_6645_ = 0.9999999f;
                                                    } else {
                                                        phi_6645_ = select(_e2984, 0.00000011920929f, (_e2984 < 0f));
                                                    }
                                                    let _e2991 = phi_6645_;
                                                    phi_2990_ = _e2991;
                                                    break;
                                                }
                                                default: {
                                                    phi_2990_ = f32();
                                                    break;
                                                }
                                            }
                                            let _e3033 = phi_2990_;
                                            switch bitcast<i32>(_e2871) {
                                                case 1: {
                                                    let _e3068 = abs(_e2985);
                                                    let _e3070 = (_e3068 % 1f);
                                                    if (_e3068 >= 1f) {
                                                        phi_6723_ = select(true, false, (_e3070 == 0f));
                                                    } else {
                                                        phi_6723_ = true;
                                                    }
                                                    let _e3074 = phi_6723_;
                                                    let _e3075 = select(1f, _e3070, _e3074);
                                                    if (select(-1f, 1f, (_e2985 >= 0f)) > 0f) {
                                                        phi_3009_ = _e3075;
                                                    } else {
                                                        phi_3009_ = (1f - _e3075);
                                                    }
                                                    let _e3079 = phi_3009_;
                                                    phi_3046_ = _e3079;
                                                    break;
                                                }
                                                case 2: {
                                                    let _e3042 = abs(_e2985);
                                                    let _e3049 = ((select(select(u32(_e3042), 0u, (_e3042 < 0f)), 4294967295u, (_e3042 > 4294967000f)) % 2u) == 0u);
                                                    let _e3051 = (_e3042 % 1f);
                                                    if (_e3042 >= 1f) {
                                                        phi_6706_ = select(true, false, (_e3051 == 0f));
                                                    } else {
                                                        phi_6706_ = true;
                                                    }
                                                    let _e3055 = phi_6706_;
                                                    let _e3056 = select(1f, _e3051, _e3055);
                                                    if (select(-1f, 1f, (_e2985 >= 0f)) > 0f) {
                                                        if _e3049 {
                                                            phi_3038_ = _e3056;
                                                        } else {
                                                            phi_3038_ = (1f - _e3056);
                                                        }
                                                        let _e3063 = phi_3038_;
                                                        phi_3044_ = _e3063;
                                                    } else {
                                                        if _e3049 {
                                                            phi_3043_ = (1f - _e3056);
                                                        } else {
                                                            phi_3043_ = _e3056;
                                                        }
                                                        let _e3060 = phi_3043_;
                                                        phi_3044_ = _e3060;
                                                    }
                                                    let _e3065 = phi_3044_;
                                                    phi_3046_ = _e3065;
                                                    break;
                                                }
                                                case 0: {
                                                    if (_e2985 > 1f) {
                                                        phi_6693_ = 0.9999999f;
                                                    } else {
                                                        phi_6693_ = select(_e2985, 0.00000011920929f, (_e2985 < 0f));
                                                    }
                                                    let _e3039 = phi_6693_;
                                                    phi_3046_ = _e3039;
                                                    break;
                                                }
                                                default: {
                                                    phi_3046_ = f32();
                                                    break;
                                                }
                                            }
                                            let _e3081 = phi_3046_;
                                            let _e3083 = (_e3033 * f32(_e2849));
                                            let _e3090 = (_e3081 * f32(_e2853));
                                            let _e3105 = vec3<f32>((f32((select(select(u32(_e3083), 0u, (_e3083 < 0f)), 4294967295u, (_e3083 > 4294967000f)) + _e2841)) / f32(_e2874)), (f32((select(select(u32(_e3090), 0u, (_e3090 < 0f)), 4294967295u, (_e3090 > 4294967000f)) + _e2845)) / f32(_e2878)), f32(_e2857));
                                            let _e3111 = textureSampleLevel(global_19, global_18, vec2<f32>(_e3105.x, _e3105.y), i32(_e3105.z), 0f);
                                            phi_3106_ = select(0f, 1f, ((((bitcast<f32>(_e2953) + fma(bitcast<f32>(_e2933), _e126.z, fma(bitcast<f32>(_e2913), _e126.y, (bitcast<f32>(_e2893) * _e126.x)))) / _e2978) - max((bitcast<f32>(_e2832) * (1f - _e2747)), bitcast<f32>(_e2827))) > _e3111.x));
                                        } else {
                                            phi_3106_ = 0f;
                                        }
                                        let _e3120 = phi_3106_;
                                        let _e3121 = (1f - _e3120);
                                        phi_3670_ = vec3<f32>(fma(((fma((((1f - _e2772) * _e2690) * _e1364), 0.31830987f, ((_e2781 * _e2772) / _e2790)) * (_e2647.member_1.x * _e2647.member_2)) * _e2788), _e3121, _e1946.x), fma(((fma((((1f - _e2773) * _e2690) * _e1366), 0.31830987f, ((_e2781 * _e2773) / _e2790)) * (_e2647.member_1.y * _e2647.member_2)) * _e2788), _e3121, _e1946.y), fma(((fma((((1f - _e2774) * _e2690) * _e1368), 0.31830987f, ((_e2781 * _e2774) / _e2790)) * (_e2647.member_1.z * _e2647.member_2)) * _e2788), _e3121, _e1946.z));
                                        phi_3671_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if (_e118 >= 8u) {
                                            phi_6283_ = (_e2005.member_1 <= (_e118 - 8u));
                                        } else {
                                            phi_6283_ = false;
                                        }
                                        let _e2391 = phi_6283_;
                                        if _e2391 {
                                            let _e2394 = global_1.member[_e2005.member_1];
                                            let _e2399 = global_1.member[(_e2005.member_1 + 1u)];
                                            let _e2404 = global_1.member[(_e2005.member_1 + 2u)];
                                            let _e2410 = global_1.member[(_e2005.member_1 + 3u)];
                                            let _e2415 = global_1.member[(_e2005.member_1 + 4u)];
                                            let _e2420 = global_1.member[(_e2005.member_1 + 5u)];
                                            let _e2425 = global_1.member[(_e2005.member_1 + 6u)];
                                            let _e2431 = global_1.member[(_e2005.member_1 + 7u)];
                                            phi_3160_ = type_34(vec3<f32>(bitcast<f32>(_e2394), bitcast<f32>(_e2399), bitcast<f32>(_e2404)), vec4<f32>(bitcast<f32>(_e2410), bitcast<f32>(_e2415), bitcast<f32>(_e2420), bitcast<f32>(_e2425)), bitcast<f32>(_e2431));
                                        } else {
                                            phi_3160_ = type_34(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2435 = phi_3160_;
                                        let _e2464 = (vec3<f32>((_e2066.member.x + fma(_e2105.x, _e2435.member.z, fma(_e2103.x, _e2435.member.y, (_e2101.x * _e2435.member.x)))), (_e2066.member.y + fma(_e2105.y, _e2435.member.z, fma(_e2103.y, _e2435.member.y, (_e2101.y * _e2435.member.x)))), (_e2066.member.z + fma(_e2105.z, _e2435.member.z, fma(_e2103.z, _e2435.member.y, (_e2101.z * _e2435.member.x))))) - _e126);
                                        let _e2471 = sqrt(fma(_e2464.z, _e2464.z, fma(_e2464.x, _e2464.x, (_e2464.y * _e2464.y))));
                                        let _e2472 = (_e2471 == 0f);
                                        if _e2472 {
                                            phi_3350_ = vec3<f32>();
                                        } else {
                                            if _e2472 {
                                                phi_6330_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6330_ = (_e2464 * (1f / _e2471));
                                            }
                                            let _e2476 = phi_6330_;
                                            let _e2478 = (_e2435.member_2 / (_e2471 * _e2471));
                                            let _e2480 = fma(-(_e680.z), _e289.member_3, 1f);
                                            let _e2484 = fma(0.4f, _e2480, (_e1364 * _e1376));
                                            let _e2485 = fma(0.4f, _e2480, (_e1366 * _e1376));
                                            let _e2486 = fma(0.4f, _e2480, (_e1368 * _e1376));
                                            let _e2493 = (_e1932 + _e2476);
                                            let _e2500 = sqrt(fma(_e2493.z, _e2493.z, fma(_e2493.x, _e2493.x, (_e2493.y * _e2493.y))));
                                            if (_e2500 == 0f) {
                                                phi_6365_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6365_ = (_e2493 * (1f / _e2500));
                                            }
                                            let _e2505 = phi_6365_;
                                            let _e2506 = (_e1373 * _e1373);
                                            let _e2517 = max(fma(_e1928.z, _e2505.z, fma(_e1928.x, _e2505.x, (_e1928.y * _e2505.y))), 0f);
                                            let _e2530 = max(fma(_e1928.z, _e1932.z, fma(_e1928.x, _e1932.x, (_e1928.y * _e1932.y))), 0f);
                                            let _e2537 = max(fma(_e1928.z, _e2476.z, fma(_e1928.x, _e2476.x, (_e1928.y * _e2476.y))), 0f);
                                            let _e2538 = fma(_e680.y, _e289.member_4, 1f);
                                            let _e2539 = (_e2538 * _e2538);
                                            let _e2540 = (_e2539 * 0.125f);
                                            let _e2542 = fma(-(_e2539), 0.125f, 1f);
                                            let _e2555 = (1f - max(fma(_e2505.z, _e1932.z, fma(_e2505.x, _e1932.x, (_e2505.y * _e1932.y))), 0f));
                                            let _e2557 = select(_e2555, 0f, (_e2555 < 0f));
                                            let _e2560 = pow(select(_e2557, 1f, (_e2557 > 1f)), 5f);
                                            let _e2561 = fma((1f - _e2484), _e2560, _e2484);
                                            let _e2562 = fma((1f - _e2485), _e2560, _e2485);
                                            let _e2563 = fma((1f - _e2486), _e2560, _e2486);
                                            let _e2570 = (((_e2506 * _e2506) / (pow(fma((_e2517 * _e2517), fma(_e2506, _e2506, -1f), 1f), 2f) * 3.1415927f)) * ((_e2530 / fma(_e2530, _e2542, _e2540)) * (_e2537 / fma(_e2537, _e2542, _e2540))));
                                            let _e2575 = fma((4f * _e2530), _e2537, 0.0001f);
                                            phi_3350_ = vec3<f32>(fma((fma((((1f - _e2561) * _e2480) * _e1364), 0.31830987f, ((_e2570 * _e2561) / _e2575)) * (_e2435.member_1.x * _e2478)), _e2537, _e1946.x), fma((fma((((1f - _e2562) * _e2480) * _e1366), 0.31830987f, ((_e2570 * _e2562) / _e2575)) * (_e2435.member_1.y * _e2478)), _e2537, _e1946.y), fma((fma((((1f - _e2563) * _e2480) * _e1368), 0.31830987f, ((_e2570 * _e2563) / _e2575)) * (_e2435.member_1.z * _e2478)), _e2537, _e1946.z));
                                        }
                                        let _e2596 = phi_3350_;
                                        phi_3670_ = _e2596;
                                        phi_3671_ = select(true, false, _e2472);
                                        break;
                                    }
                                    case 2: {
                                        if (_e118 >= 13u) {
                                            phi_6071_ = (_e2005.member_1 <= (_e118 - 13u));
                                        } else {
                                            phi_6071_ = false;
                                        }
                                        let _e2116 = phi_6071_;
                                        if _e2116 {
                                            let _e2119 = global_1.member[_e2005.member_1];
                                            let _e2124 = global_1.member[(_e2005.member_1 + 1u)];
                                            let _e2129 = global_1.member[(_e2005.member_1 + 2u)];
                                            let _e2135 = global_1.member[(_e2005.member_1 + 3u)];
                                            let _e2140 = global_1.member[(_e2005.member_1 + 4u)];
                                            let _e2145 = global_1.member[(_e2005.member_1 + 5u)];
                                            let _e2151 = global_1.member[(_e2005.member_1 + 6u)];
                                            let _e2156 = global_1.member[(_e2005.member_1 + 7u)];
                                            let _e2161 = global_1.member[(_e2005.member_1 + 8u)];
                                            let _e2166 = global_1.member[(_e2005.member_1 + 9u)];
                                            let _e2171 = global_1.member[(_e2005.member_1 + 10u)];
                                            let _e2176 = global_1.member[(_e2005.member_1 + 11u)];
                                            let _e2182 = global_1.member[(_e2005.member_1 + 12u)];
                                            phi_3413_ = type_35(vec3<f32>(bitcast<f32>(_e2119), bitcast<f32>(_e2124), bitcast<f32>(_e2129)), vec3<f32>(bitcast<f32>(_e2135), bitcast<f32>(_e2140), bitcast<f32>(_e2145)), bitcast<f32>(_e2151), bitcast<f32>(_e2156), vec4<f32>(bitcast<f32>(_e2161), bitcast<f32>(_e2166), bitcast<f32>(_e2171), bitcast<f32>(_e2176)), bitcast<f32>(_e2182));
                                        } else {
                                            phi_3413_ = type_35(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2186 = phi_3413_;
                                        let _e2218 = (vec3<f32>((_e2066.member.x + fma(_e2105.x, _e2186.member.z, fma(_e2103.x, _e2186.member.y, (_e2101.x * _e2186.member.x)))), (_e2066.member.y + fma(_e2105.y, _e2186.member.z, fma(_e2103.y, _e2186.member.y, (_e2101.y * _e2186.member.x)))), (_e2066.member.z + fma(_e2105.z, _e2186.member.z, fma(_e2103.z, _e2186.member.y, (_e2101.z * _e2186.member.x))))) - _e126);
                                        let _e2225 = sqrt(fma(_e2218.z, _e2218.z, fma(_e2218.x, _e2218.x, (_e2218.y * _e2218.y))));
                                        let _e2226 = (_e2225 == 0f);
                                        if _e2226 {
                                            phi_3668_ = vec3<f32>();
                                        } else {
                                            if _e2226 {
                                                phi_6121_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6121_ = (_e2218 * (1f / _e2225));
                                            }
                                            let _e2230 = phi_6121_;
                                            let _e2240 = fma(_e2105.x, _e2186.member_1.z, fma(_e2103.x, _e2186.member_1.y, (_e2101.x * _e2186.member_1.x)));
                                            let _e2241 = fma(_e2105.y, _e2186.member_1.z, fma(_e2103.y, _e2186.member_1.y, (_e2101.y * _e2186.member_1.x)));
                                            let _e2242 = fma(_e2105.z, _e2186.member_1.z, fma(_e2103.z, _e2186.member_1.y, (_e2101.z * _e2186.member_1.x)));
                                            let _e2247 = sqrt(fma(_e2242, _e2242, fma(_e2240, _e2240, (_e2241 * _e2241))));
                                            if (_e2247 == 0f) {
                                                phi_6156_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6156_ = (vec3<f32>(_e2240, _e2241, _e2242) * (1f / _e2247));
                                            }
                                            let _e2252 = phi_6156_;
                                            let _e2264 = ((fma(_e2230.z, _e2252.z, fma(_e2230.x, _e2252.x, (_e2230.y * _e2252.y))) - _e2186.member_3) / (_e2186.member_2 - _e2186.member_3));
                                            let _e2266 = select(_e2264, 0f, (_e2264 < 0f));
                                            let _e2269 = (_e2186.member_5 * select(_e2266, 1f, (_e2266 > 1f)));
                                            let _e2271 = fma(-(_e680.z), _e289.member_3, 1f);
                                            let _e2275 = fma(0.4f, _e2271, (_e1364 * _e1376));
                                            let _e2276 = fma(0.4f, _e2271, (_e1366 * _e1376));
                                            let _e2277 = fma(0.4f, _e2271, (_e1368 * _e1376));
                                            let _e2284 = (_e1932 + _e2230);
                                            let _e2291 = sqrt(fma(_e2284.z, _e2284.z, fma(_e2284.x, _e2284.x, (_e2284.y * _e2284.y))));
                                            if (_e2291 == 0f) {
                                                phi_6191_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6191_ = (_e2284 * (1f / _e2291));
                                            }
                                            let _e2296 = phi_6191_;
                                            let _e2297 = (_e1373 * _e1373);
                                            let _e2308 = max(fma(_e1928.z, _e2296.z, fma(_e1928.x, _e2296.x, (_e1928.y * _e2296.y))), 0f);
                                            let _e2321 = max(fma(_e1928.z, _e1932.z, fma(_e1928.x, _e1932.x, (_e1928.y * _e1932.y))), 0f);
                                            let _e2325 = max(fma(_e1928.z, _e2230.z, fma(_e1928.x, _e2230.x, (_e1928.y * _e2230.y))), 0f);
                                            let _e2326 = fma(_e680.y, _e289.member_4, 1f);
                                            let _e2327 = (_e2326 * _e2326);
                                            let _e2328 = (_e2327 * 0.125f);
                                            let _e2330 = fma(-(_e2327), 0.125f, 1f);
                                            let _e2343 = (1f - max(fma(_e2296.z, _e1932.z, fma(_e2296.x, _e1932.x, (_e2296.y * _e1932.y))), 0f));
                                            let _e2345 = select(_e2343, 0f, (_e2343 < 0f));
                                            let _e2348 = pow(select(_e2345, 1f, (_e2345 > 1f)), 5f);
                                            let _e2349 = fma((1f - _e2275), _e2348, _e2275);
                                            let _e2350 = fma((1f - _e2276), _e2348, _e2276);
                                            let _e2351 = fma((1f - _e2277), _e2348, _e2277);
                                            let _e2358 = (((_e2297 * _e2297) / (pow(fma((_e2308 * _e2308), fma(_e2297, _e2297, -1f), 1f), 2f) * 3.1415927f)) * ((_e2321 / fma(_e2321, _e2330, _e2328)) * (_e2325 / fma(_e2325, _e2330, _e2328))));
                                            let _e2363 = fma((4f * _e2321), _e2325, 0.0001f);
                                            phi_3668_ = vec3<f32>(fma((fma((((1f - _e2349) * _e2271) * _e1364), 0.31830987f, ((_e2358 * _e2349) / _e2363)) * (_e2186.member_4.x * _e2269)), _e2325, _e1946.x), fma((fma((((1f - _e2350) * _e2271) * _e1366), 0.31830987f, ((_e2358 * _e2350) / _e2363)) * (_e2186.member_4.y * _e2269)), _e2325, _e1946.y), fma((fma((((1f - _e2351) * _e2271) * _e1368), 0.31830987f, ((_e2358 * _e2351) / _e2363)) * (_e2186.member_4.z * _e2269)), _e2325, _e1946.z));
                                        }
                                        let _e2384 = phi_3668_;
                                        phi_3670_ = _e2384;
                                        phi_3671_ = select(true, false, _e2226);
                                        break;
                                    }
                                    default: {
                                        phi_3670_ = vec3<f32>();
                                        phi_3671_ = bool();
                                        break;
                                    }
                                }
                                let _e3130 = phi_3670_;
                                let _e3132 = phi_3671_;
                                phi_3680_ = select(_e3130, _e1946, vec3(select(true, false, _e3132)));
                            }
                            let _e3137 = phi_3680_;
                            phi_2367_ = _e3137;
                            phi_3682_ = select(true, false, _e1979);
                            break;
                        }
                        default: {
                            phi_2367_ = vec3<f32>();
                            phi_3682_ = bool();
                            break;
                        }
                    }
                    let _e3140 = phi_2367_;
                    let _e3142 = phi_3682_;
                    continue;
                    continuing {
                        phi_2363_ = _e1959;
                        phi_2366_ = _e3140;
                        break if !(_e3142);
                    }
                }
                let _e3145 = fma(-(_e680.z), _e289.member_3, 1f);
                let _e3149 = fma(0.04f, _e3145, (_e1364 * _e1376));
                let _e3150 = fma(0.04f, _e3145, (_e1366 * _e1376));
                let _e3151 = fma(0.04f, _e3145, (_e1368 * _e1376));
                let _e3163 = fma(-(_e680.y), _e289.member_4, 1f);
                let _e3170 = (1f - max(fma(_e1928.z, _e1932.z, fma(_e1928.x, _e1932.x, (_e1928.y * _e1932.y))), 0f));
                let _e3172 = select(_e3170, 0f, (_e3170 < 0f));
                let _e3175 = pow(select(_e3172, 1f, (_e3172 > 1f)), 5f);
                let _e3176 = fma((max(_e3163, _e3149) - _e3149), _e3175, _e3149);
                let _e3177 = fma((max(_e3163, _e3150) - _e3150), _e3175, _e3150);
                let _e3178 = fma((max(_e3163, _e3151) - _e3151), _e3175, _e3151);
                let _e3198 = local_2;
                let _e3202 = local_3;
                let _e3206 = local_4;
                phi_3799_ = vec4<f32>(fma(_e1386, _e289.member_1, fma(fma(((1f - _e3176) * _e3145), (_e1395.x * _e1364), (_e1743.x * fma(_e3176, _e1759.x, _e1759.y))), _e1380, _e3198.x)), fma(_e1388, _e289.member_1, fma(fma(((1f - _e3177) * _e3145), (_e1395.y * _e1366), (_e1743.y * fma(_e3177, _e1759.x, _e1759.y))), _e1380, _e3202.y)), fma(_e1390, _e289.member_1, fma(fma(((1f - _e3178) * _e3145), (_e1395.z * _e1368), (_e1743.z * fma(_e3178, _e1759.x, _e1759.y))), _e1380, _e3206.z)), 1f);
            } else {
                phi_3799_ = (vec4<f32>((_e120.x * _e486.x), (_e120.y * _e486.y), (_e120.z * _e486.z), (_e120.w * _e486.w)) * _e289.member_2);
            }
            let _e3214 = phi_3799_;
            global_20 = _e3214;
            break;
        }
        case 1: {
            let _e1901 = sqrt(fma(_e121.x, _e121.x, (_e121.y * _e121.y)));
            if (_e1901 == 0f) {
                phi_5808_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5808_ = (vec3<f32>(_e121.x, _e121.y, 0f) * (1f / _e1901));
            }
            let _e1906 = phi_5808_;
            global_20 = vec4<f32>(((_e1906.x + 1f) * 0.5f), ((_e1906.y + 1f) * 0.5f), ((_e1906.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1880 = sqrt(fma(_e122.x, _e122.x, (_e122.y * _e122.y)));
            if (_e1880 == 0f) {
                phi_5759_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5759_ = (vec3<f32>(_e122.x, _e122.y, 0f) * (1f / _e1880));
            }
            let _e1885 = phi_5759_;
            global_20 = vec4<f32>(((_e1885.x + 1f) * 0.5f), ((_e1885.y + 1f) * 0.5f), ((_e1885.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1722 {
                phi_5710_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5710_ = (_e1349 * (1f / _e1721));
            }
            let _e1864 = phi_5710_;
            global_20 = vec4<f32>(((_e1864.x + 1f) * 0.5f), ((_e1864.y + 1f) * 0.5f), ((_e1864.z + 1f) * 0.5f), 1f);
            break;
        }
        case 4: {
            global_20 = _e120;
            break;
        }
        case 5: {
            let _e1845 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
            if (_e1845 == 0f) {
                phi_5661_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5661_ = (_e123 * (1f / _e1845));
            }
            let _e1850 = phi_5661_;
            global_20 = vec4<f32>(((_e1850.x + 1f) * 0.5f), ((_e1850.y + 1f) * 0.5f), ((_e1850.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1823 = sqrt(fma(_e1347.z, _e1347.z, fma(_e1347.x, _e1347.x, (_e1347.y * _e1347.y))));
            if (_e1823 == 0f) {
                phi_5612_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5612_ = (_e1347 * (1f / _e1823));
            }
            let _e1828 = phi_5612_;
            global_20 = vec4<f32>(((_e1828.x + 1f) * 0.5f), ((_e1828.y + 1f) * 0.5f), ((_e1828.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1801 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
            if (_e1801 == 0f) {
                phi_5563_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5563_ = (_e124 * (1f / _e1801));
            }
            let _e1806 = phi_5563_;
            global_20 = vec4<f32>(((_e1806.x + 1f) * 0.5f), ((_e1806.y + 1f) * 0.5f), ((_e1806.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1779 = sqrt(fma(_e125.z, _e125.z, fma(_e125.x, _e125.x, (_e125.y * _e125.y))));
            if (_e1779 == 0f) {
                phi_5514_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5514_ = (_e125 * (1f / _e1779));
            }
            let _e1784 = phi_5514_;
            global_20 = vec4<f32>(((_e1784.x + 1f) * 0.5f), ((_e1784.y + 1f) * 0.5f), ((_e1784.z + 1f) * 0.5f), 1f);
            break;
        }
        case 9: {
            global_20 = vec4<f32>(_e1395.x, _e1395.y, _e1395.z, 1f);
            break;
        }
        case 10: {
            global_20 = vec4<f32>(_e1743.x, _e1743.y, _e1743.z, 1f);
            break;
        }
        case 11: {
            global_20 = vec4<f32>(_e1759.x, _e1759.y, 1f, 1f);
            break;
        }
        case 12: {
            global_20 = (vec4<f32>(_e1353, _e1356, _e1359, (_e486.w * _e289.member_2.w)) * _e120);
            break;
        }
        case 13: {
            global_20 = vec4<f32>(_e1373, _e1373, _e1373, 1f);
            break;
        }
        case 14: {
            global_20 = vec4<f32>(_e1376, _e1376, _e1376, 1f);
            break;
        }
        case 15: {
            global_20 = vec4<f32>(_e1380, _e1380, _e1380, 1f);
            break;
        }
        case 16: {
            global_20 = vec4<f32>((_e1386 * _e289.member_1), (_e1388 * _e289.member_1), (_e1390 * _e289.member_1), 1f);
            break;
        }
        case 17: {
            global_20 = vec4<f32>(_e1262.x, _e1262.y, _e1262.z, 1f);
            break;
        }
        case 18: {
            global_20 = vec4<f32>(_e289.member.x, _e289.member.y, _e289.member.z, 1f);
            break;
        }
        case 19: {
            global_20 = vec4<f32>(_e289.member_1, _e289.member_1, _e289.member_1, 1f);
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
    global_2 = param;
    global_3 = param_1;
    global_4 = param_2;
    global_5 = param_3;
    global_6 = param_4;
    global_7 = param_5;
    global_8 = param_6;
    global_9 = param_7;
    function();
    let _e17 = global_20;
    return _e17;
}
