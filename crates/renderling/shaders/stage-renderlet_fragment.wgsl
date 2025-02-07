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
    var phi_677_: u32;
    var phi_4330_: bool;
    var phi_831_: type_33;
    var phi_835_: type_33;
    var phi_4367_: bool;
    var phi_875_: u32;
    var phi_884_: u32;
    var phi_897_: type_15;
    var phi_4389_: f32;
    var phi_4402_: bool;
    var phi_951_: f32;
    var phi_946_: f32;
    var phi_952_: f32;
    var phi_4419_: bool;
    var phi_917_: f32;
    var phi_954_: f32;
    var phi_4437_: f32;
    var phi_4450_: bool;
    var phi_1009_: f32;
    var phi_1004_: f32;
    var phi_1010_: f32;
    var phi_4467_: bool;
    var phi_975_: f32;
    var phi_1012_: f32;
    var phi_4503_: bool;
    var phi_1095_: u32;
    var phi_1104_: u32;
    var phi_1117_: type_15;
    var phi_4524_: f32;
    var phi_4537_: bool;
    var phi_1171_: f32;
    var phi_1166_: f32;
    var phi_1172_: f32;
    var phi_4554_: bool;
    var phi_1137_: f32;
    var phi_1174_: f32;
    var phi_4572_: f32;
    var phi_4585_: bool;
    var phi_1229_: f32;
    var phi_1224_: f32;
    var phi_1230_: f32;
    var phi_4602_: bool;
    var phi_1195_: f32;
    var phi_1232_: f32;
    var phi_4638_: bool;
    var phi_1315_: u32;
    var phi_1324_: u32;
    var phi_1337_: type_15;
    var phi_4659_: f32;
    var phi_4672_: bool;
    var phi_1391_: f32;
    var phi_1386_: f32;
    var phi_1392_: f32;
    var phi_4689_: bool;
    var phi_1357_: f32;
    var phi_1394_: f32;
    var phi_4707_: f32;
    var phi_4720_: bool;
    var phi_1449_: f32;
    var phi_1444_: f32;
    var phi_1450_: f32;
    var phi_4737_: bool;
    var phi_1415_: f32;
    var phi_1452_: f32;
    var phi_4773_: bool;
    var phi_1535_: u32;
    var phi_1544_: u32;
    var phi_1557_: type_15;
    var phi_4794_: f32;
    var phi_4807_: bool;
    var phi_1611_: f32;
    var phi_1606_: f32;
    var phi_1612_: f32;
    var phi_4824_: bool;
    var phi_1577_: f32;
    var phi_1614_: f32;
    var phi_4842_: f32;
    var phi_4855_: bool;
    var phi_1669_: f32;
    var phi_1664_: f32;
    var phi_1670_: f32;
    var phi_4872_: bool;
    var phi_1635_: f32;
    var phi_1672_: f32;
    var phi_4908_: bool;
    var phi_1755_: u32;
    var phi_1764_: u32;
    var phi_1777_: type_15;
    var phi_4929_: f32;
    var phi_4942_: bool;
    var phi_1831_: f32;
    var phi_1826_: f32;
    var phi_1832_: f32;
    var phi_4959_: bool;
    var phi_1797_: f32;
    var phi_1834_: f32;
    var phi_4977_: f32;
    var phi_4990_: bool;
    var phi_1889_: f32;
    var phi_1884_: f32;
    var phi_1890_: f32;
    var phi_5007_: bool;
    var phi_1855_: f32;
    var phi_1892_: f32;
    var phi_5065_: vec3<f32>;
    var phi_5100_: vec3<f32>;
    var phi_5135_: vec3<f32>;
    var phi_5170_: vec3<f32>;
    var phi_5205_: vec3<f32>;
    var phi_1986_: vec3<f32>;
    var phi_1987_: vec3<f32>;
    var phi_5237_: bool;
    var phi_2194_: type_14;
    var phi_2195_: type_14;
    var phi_2218_: type_14;
    var phi_2245_: bool;
    var phi_2251_: type_14;
    var phi_2252_: type_14;
    var phi_2275_: type_14;
    var phi_2298_: bool;
    var phi_2319_: type_25;
    var phi_5309_: vec3<f32>;
    var phi_5368_: vec3<f32>;
    var phi_5442_: vec3<f32>;
    var phi_5502_: vec3<f32>;
    var phi_5551_: vec3<f32>;
    var phi_5600_: vec3<f32>;
    var phi_5649_: vec3<f32>;
    var phi_5698_: vec3<f32>;
    var phi_5747_: vec3<f32>;
    var phi_5796_: vec3<f32>;
    var phi_5835_: vec3<f32>;
    var phi_5870_: vec3<f32>;
    var phi_2359_: type_14;
    var phi_2362_: vec3<f32>;
    var phi_2360_: type_14;
    var phi_2385_: type_14;
    var phi_5887_: u32;
    var phi_5906_: bool;
    var phi_2402_: u32;
    var phi_5938_: bool;
    var phi_2419_: u32;
    var phi_2433_: type_30;
    var phi_5970_: bool;
    var phi_2483_: type_31;
    var phi_6050_: bool;
    var phi_3398_: type_35;
    var phi_6100_: vec3<f32>;
    var phi_6135_: vec3<f32>;
    var phi_6170_: vec3<f32>;
    var phi_3653_: vec3<f32>;
    var phi_6262_: bool;
    var phi_3145_: type_34;
    var phi_6309_: vec3<f32>;
    var phi_6344_: vec3<f32>;
    var phi_3335_: vec3<f32>;
    var phi_6436_: bool;
    var phi_2531_: type_34;
    var phi_6483_: vec3<f32>;
    var phi_6518_: vec3<f32>;
    var phi_2768_: u32;
    var phi_2777_: u32;
    var phi_6624_: f32;
    var phi_6637_: bool;
    var phi_2978_: f32;
    var phi_2973_: f32;
    var phi_2979_: f32;
    var phi_6654_: bool;
    var phi_2944_: f32;
    var phi_2981_: f32;
    var phi_6672_: f32;
    var phi_6685_: bool;
    var phi_3034_: f32;
    var phi_3029_: f32;
    var phi_3035_: f32;
    var phi_6702_: bool;
    var phi_3000_: f32;
    var phi_3037_: f32;
    var phi_3091_: f32;
    var phi_3655_: vec3<f32>;
    var phi_3656_: bool;
    var phi_3665_: vec3<f32>;
    var phi_2363_: vec3<f32>;
    var phi_3667_: bool;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var phi_3784_: vec4<f32>;

    let _e116 = arrayLength((&global.member));
    let _e117 = global_2;
    let _e118 = global_3;
    let _e119 = global_4;
    let _e120 = global_5;
    let _e121 = global_6;
    let _e122 = global_7;
    let _e123 = global_8;
    let _e124 = global_9;
    let _e128 = global.member[(_e117 + 9u)];
    let _e132 = global.member[(_e117 + 11u)];
    let _e136 = global.member[(_e117 + 17u)];
    let _e139 = global.member[_e136];
    let _e143 = global.member[(_e136 + 1u)];
    let _e147 = global.member[(_e136 + 4u)];
    switch bitcast<i32>(_e147) {
        case 0: {
            phi_677_ = 0u;
            break;
        }
        case 1: {
            phi_677_ = 1u;
            break;
        }
        case 2: {
            phi_677_ = 2u;
            break;
        }
        case 3: {
            phi_677_ = 3u;
            break;
        }
        case 4: {
            phi_677_ = 4u;
            break;
        }
        case 5: {
            phi_677_ = 5u;
            break;
        }
        case 6: {
            phi_677_ = 6u;
            break;
        }
        case 7: {
            phi_677_ = 7u;
            break;
        }
        case 8: {
            phi_677_ = 8u;
            break;
        }
        case 9: {
            phi_677_ = 9u;
            break;
        }
        case 10: {
            phi_677_ = 10u;
            break;
        }
        case 11: {
            phi_677_ = 11u;
            break;
        }
        case 12: {
            phi_677_ = 12u;
            break;
        }
        case 13: {
            phi_677_ = 13u;
            break;
        }
        case 14: {
            phi_677_ = 14u;
            break;
        }
        case 15: {
            phi_677_ = 15u;
            break;
        }
        case 16: {
            phi_677_ = 16u;
            break;
        }
        case 17: {
            phi_677_ = 17u;
            break;
        }
        case 18: {
            phi_677_ = 18u;
            break;
        }
        case 19: {
            phi_677_ = 19u;
            break;
        }
        default: {
            phi_677_ = 0u;
            break;
        }
    }
    let _e150 = phi_677_;
    let _e154 = global.member[(_e136 + 5u)];
    let _e159 = global.member[(_e136 + 9u)];
    let _e163 = global.member[(_e136 + 10u)];
    if (_e132 == 4294967295u) {
        phi_835_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e116 >= 22u) {
            phi_4330_ = (_e132 <= (_e116 - 22u));
        } else {
            phi_4330_ = false;
        }
        let _e169 = phi_4330_;
        if _e169 {
            let _e172 = global.member[_e132];
            let _e177 = global.member[(_e132 + 1u)];
            let _e182 = global.member[(_e132 + 2u)];
            let _e188 = global.member[(_e132 + 3u)];
            let _e193 = global.member[(_e132 + 4u)];
            let _e198 = global.member[(_e132 + 5u)];
            let _e203 = global.member[(_e132 + 6u)];
            let _e208 = global.member[(_e132 + 7u)];
            let _e214 = global.member[(_e132 + 8u)];
            let _e219 = global.member[(_e132 + 9u)];
            let _e224 = global.member[(_e132 + 10u)];
            let _e228 = global.member[(_e132 + 11u)];
            let _e232 = global.member[(_e132 + 12u)];
            let _e236 = global.member[(_e132 + 13u)];
            let _e240 = global.member[(_e132 + 14u)];
            let _e244 = global.member[(_e132 + 15u)];
            let _e248 = global.member[(_e132 + 16u)];
            let _e252 = global.member[(_e132 + 17u)];
            let _e256 = global.member[(_e132 + 18u)];
            let _e260 = global.member[(_e132 + 19u)];
            let _e264 = global.member[(_e132 + 20u)];
            let _e269 = global.member[(_e132 + 21u)];
            phi_831_ = type_33(vec3<f32>(bitcast<f32>(_e172), bitcast<f32>(_e177), bitcast<f32>(_e182)), bitcast<f32>(_e188), vec4<f32>(bitcast<f32>(_e193), bitcast<f32>(_e198), bitcast<f32>(_e203), bitcast<f32>(_e208)), bitcast<f32>(_e214), bitcast<f32>(_e219), _e224, _e228, _e232, _e236, _e240, _e244, _e248, _e252, _e256, _e260, (_e264 == 1u), bitcast<f32>(_e269));
        } else {
            phi_831_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e273 = phi_831_;
        phi_835_ = type_33(_e273.member, _e273.member_1, _e273.member_2, _e273.member_3, _e273.member_4, _e273.member_5, _e273.member_6, _e273.member_7, _e273.member_8, _e273.member_9, _e273.member_10, _e273.member_11, _e273.member_12, _e273.member_13, _e273.member_14, (_e273.member_15 && (_e154 == 1u)), _e273.member_16);
    }
    let _e295 = phi_835_;
    let _e299 = select(_e120, _e119, vec2((_e295.member_10 == 0u)));
    let _e301 = (_e116 >= 8u);
    if _e301 {
        phi_4367_ = (_e295.member_5 <= (_e116 - 8u));
    } else {
        phi_4367_ = false;
    }
    let _e305 = phi_4367_;
    if _e305 {
        let _e308 = global.member[_e295.member_5];
        let _e312 = global.member[(_e295.member_5 + 1u)];
        let _e317 = global.member[(_e295.member_5 + 2u)];
        let _e321 = global.member[(_e295.member_5 + 3u)];
        let _e326 = global.member[(_e295.member_5 + 4u)];
        let _e330 = global.member[(_e295.member_5 + 5u)];
        let _e334 = global.member[(_e295.member_5 + 6u)];
        switch bitcast<i32>(_e334) {
            case 0: {
                phi_875_ = 0u;
                break;
            }
            case 1: {
                phi_875_ = 1u;
                break;
            }
            case 2: {
                phi_875_ = 2u;
                break;
            }
            default: {
                phi_875_ = 0u;
                break;
            }
        }
        let _e337 = phi_875_;
        let _e341 = global.member[(_e295.member_5 + 7u)];
        switch bitcast<i32>(_e341) {
            case 0: {
                phi_884_ = 0u;
                break;
            }
            case 1: {
                phi_884_ = 1u;
                break;
            }
            case 2: {
                phi_884_ = 2u;
                break;
            }
            default: {
                phi_884_ = 0u;
                break;
            }
        }
        let _e344 = phi_884_;
        phi_897_ = type_15(type_14(_e337, _e344), vec2<u32>(_e308, _e312), vec2<u32>(_e317, _e321), _e326, _e330);
    } else {
        phi_897_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e348 = phi_897_;
    switch bitcast<i32>(_e348.member.member) {
        case 1: {
            let _e386 = abs(_e299.x);
            let _e388 = (_e386 % 1f);
            if (_e386 >= 1f) {
                phi_4419_ = select(true, false, (_e388 == 0f));
            } else {
                phi_4419_ = true;
            }
            let _e392 = phi_4419_;
            let _e393 = select(1f, _e388, _e392);
            if (select(-1f, 1f, (_e299.x >= 0f)) > 0f) {
                phi_917_ = _e393;
            } else {
                phi_917_ = (1f - _e393);
            }
            let _e397 = phi_917_;
            phi_954_ = _e397;
            break;
        }
        case 2: {
            let _e360 = abs(_e299.x);
            let _e367 = ((select(select(u32(_e360), 0u, (_e360 < 0f)), 4294967295u, (_e360 > 4294967000f)) % 2u) == 0u);
            let _e369 = (_e360 % 1f);
            if (_e360 >= 1f) {
                phi_4402_ = select(true, false, (_e369 == 0f));
            } else {
                phi_4402_ = true;
            }
            let _e373 = phi_4402_;
            let _e374 = select(1f, _e369, _e373);
            if (select(-1f, 1f, (_e299.x >= 0f)) > 0f) {
                if _e367 {
                    phi_946_ = _e374;
                } else {
                    phi_946_ = (1f - _e374);
                }
                let _e381 = phi_946_;
                phi_952_ = _e381;
            } else {
                if _e367 {
                    phi_951_ = (1f - _e374);
                } else {
                    phi_951_ = _e374;
                }
                let _e378 = phi_951_;
                phi_952_ = _e378;
            }
            let _e383 = phi_952_;
            phi_954_ = _e383;
            break;
        }
        case 0: {
            if (_e299.x > 1f) {
                phi_4389_ = 0.9999999f;
            } else {
                phi_4389_ = select(_e299.x, 0.00000011920929f, (_e299.x < 0f));
            }
            let _e357 = phi_4389_;
            phi_954_ = _e357;
            break;
        }
        default: {
            phi_954_ = f32();
            break;
        }
    }
    let _e399 = phi_954_;
    switch bitcast<i32>(_e348.member.member_1) {
        case 1: {
            let _e437 = abs(_e299.y);
            let _e439 = (_e437 % 1f);
            if (_e437 >= 1f) {
                phi_4467_ = select(true, false, (_e439 == 0f));
            } else {
                phi_4467_ = true;
            }
            let _e443 = phi_4467_;
            let _e444 = select(1f, _e439, _e443);
            if (select(-1f, 1f, (_e299.y >= 0f)) > 0f) {
                phi_975_ = _e444;
            } else {
                phi_975_ = (1f - _e444);
            }
            let _e448 = phi_975_;
            phi_1012_ = _e448;
            break;
        }
        case 2: {
            let _e411 = abs(_e299.y);
            let _e418 = ((select(select(u32(_e411), 0u, (_e411 < 0f)), 4294967295u, (_e411 > 4294967000f)) % 2u) == 0u);
            let _e420 = (_e411 % 1f);
            if (_e411 >= 1f) {
                phi_4450_ = select(true, false, (_e420 == 0f));
            } else {
                phi_4450_ = true;
            }
            let _e424 = phi_4450_;
            let _e425 = select(1f, _e420, _e424);
            if (select(-1f, 1f, (_e299.y >= 0f)) > 0f) {
                if _e418 {
                    phi_1004_ = _e425;
                } else {
                    phi_1004_ = (1f - _e425);
                }
                let _e432 = phi_1004_;
                phi_1010_ = _e432;
            } else {
                if _e418 {
                    phi_1009_ = (1f - _e425);
                } else {
                    phi_1009_ = _e425;
                }
                let _e429 = phi_1009_;
                phi_1010_ = _e429;
            }
            let _e434 = phi_1010_;
            phi_1012_ = _e434;
            break;
        }
        case 0: {
            if (_e299.y > 1f) {
                phi_4437_ = 0.9999999f;
            } else {
                phi_4437_ = select(_e299.y, 0.00000011920929f, (_e299.y < 0f));
            }
            let _e408 = phi_4437_;
            phi_1012_ = _e408;
            break;
        }
        default: {
            phi_1012_ = f32();
            break;
        }
    }
    let _e450 = phi_1012_;
    let _e454 = (_e399 * f32(_e348.member_2.x));
    let _e463 = (_e450 * f32(_e348.member_2.y));
    let _e475 = f32(_e139);
    let _e476 = f32(_e143);
    let _e483 = vec3<f32>((f32((select(select(u32(_e454), 0u, (_e454 < 0f)), 4294967295u, (_e454 > 4294967000f)) + _e348.member_1.x)) / _e475), (f32((select(select(u32(_e463), 0u, (_e463 < 0f)), 4294967295u, (_e463 > 4294967000f)) + _e348.member_1.y)) / _e476), f32(_e348.member_3));
    let _e489 = textureSampleLevel(global_11, global_10, vec2<f32>(_e483.x, _e483.y), i32(_e483.z), 0f);
    let _e492 = select(_e489, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_5 == 4294967295u)));
    let _e496 = select(_e120, _e119, vec2((_e295.member_11 == 0u)));
    if _e301 {
        phi_4503_ = (_e295.member_6 <= (_e116 - 8u));
    } else {
        phi_4503_ = false;
    }
    let _e501 = phi_4503_;
    if _e501 {
        let _e504 = global.member[_e295.member_6];
        let _e508 = global.member[(_e295.member_6 + 1u)];
        let _e513 = global.member[(_e295.member_6 + 2u)];
        let _e517 = global.member[(_e295.member_6 + 3u)];
        let _e522 = global.member[(_e295.member_6 + 4u)];
        let _e526 = global.member[(_e295.member_6 + 5u)];
        let _e530 = global.member[(_e295.member_6 + 6u)];
        switch bitcast<i32>(_e530) {
            case 0: {
                phi_1095_ = 0u;
                break;
            }
            case 1: {
                phi_1095_ = 1u;
                break;
            }
            case 2: {
                phi_1095_ = 2u;
                break;
            }
            default: {
                phi_1095_ = 0u;
                break;
            }
        }
        let _e533 = phi_1095_;
        let _e537 = global.member[(_e295.member_6 + 7u)];
        switch bitcast<i32>(_e537) {
            case 0: {
                phi_1104_ = 0u;
                break;
            }
            case 1: {
                phi_1104_ = 1u;
                break;
            }
            case 2: {
                phi_1104_ = 2u;
                break;
            }
            default: {
                phi_1104_ = 0u;
                break;
            }
        }
        let _e540 = phi_1104_;
        phi_1117_ = type_15(type_14(_e533, _e540), vec2<u32>(_e504, _e508), vec2<u32>(_e513, _e517), _e522, _e526);
    } else {
        phi_1117_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e544 = phi_1117_;
    switch bitcast<i32>(_e544.member.member) {
        case 1: {
            let _e582 = abs(_e496.x);
            let _e584 = (_e582 % 1f);
            if (_e582 >= 1f) {
                phi_4554_ = select(true, false, (_e584 == 0f));
            } else {
                phi_4554_ = true;
            }
            let _e588 = phi_4554_;
            let _e589 = select(1f, _e584, _e588);
            if (select(-1f, 1f, (_e496.x >= 0f)) > 0f) {
                phi_1137_ = _e589;
            } else {
                phi_1137_ = (1f - _e589);
            }
            let _e593 = phi_1137_;
            phi_1174_ = _e593;
            break;
        }
        case 2: {
            let _e556 = abs(_e496.x);
            let _e563 = ((select(select(u32(_e556), 0u, (_e556 < 0f)), 4294967295u, (_e556 > 4294967000f)) % 2u) == 0u);
            let _e565 = (_e556 % 1f);
            if (_e556 >= 1f) {
                phi_4537_ = select(true, false, (_e565 == 0f));
            } else {
                phi_4537_ = true;
            }
            let _e569 = phi_4537_;
            let _e570 = select(1f, _e565, _e569);
            if (select(-1f, 1f, (_e496.x >= 0f)) > 0f) {
                if _e563 {
                    phi_1166_ = _e570;
                } else {
                    phi_1166_ = (1f - _e570);
                }
                let _e577 = phi_1166_;
                phi_1172_ = _e577;
            } else {
                if _e563 {
                    phi_1171_ = (1f - _e570);
                } else {
                    phi_1171_ = _e570;
                }
                let _e574 = phi_1171_;
                phi_1172_ = _e574;
            }
            let _e579 = phi_1172_;
            phi_1174_ = _e579;
            break;
        }
        case 0: {
            if (_e496.x > 1f) {
                phi_4524_ = 0.9999999f;
            } else {
                phi_4524_ = select(_e496.x, 0.00000011920929f, (_e496.x < 0f));
            }
            let _e553 = phi_4524_;
            phi_1174_ = _e553;
            break;
        }
        default: {
            phi_1174_ = f32();
            break;
        }
    }
    let _e595 = phi_1174_;
    switch bitcast<i32>(_e544.member.member_1) {
        case 1: {
            let _e633 = abs(_e496.y);
            let _e635 = (_e633 % 1f);
            if (_e633 >= 1f) {
                phi_4602_ = select(true, false, (_e635 == 0f));
            } else {
                phi_4602_ = true;
            }
            let _e639 = phi_4602_;
            let _e640 = select(1f, _e635, _e639);
            if (select(-1f, 1f, (_e496.y >= 0f)) > 0f) {
                phi_1195_ = _e640;
            } else {
                phi_1195_ = (1f - _e640);
            }
            let _e644 = phi_1195_;
            phi_1232_ = _e644;
            break;
        }
        case 2: {
            let _e607 = abs(_e496.y);
            let _e614 = ((select(select(u32(_e607), 0u, (_e607 < 0f)), 4294967295u, (_e607 > 4294967000f)) % 2u) == 0u);
            let _e616 = (_e607 % 1f);
            if (_e607 >= 1f) {
                phi_4585_ = select(true, false, (_e616 == 0f));
            } else {
                phi_4585_ = true;
            }
            let _e620 = phi_4585_;
            let _e621 = select(1f, _e616, _e620);
            if (select(-1f, 1f, (_e496.y >= 0f)) > 0f) {
                if _e614 {
                    phi_1224_ = _e621;
                } else {
                    phi_1224_ = (1f - _e621);
                }
                let _e628 = phi_1224_;
                phi_1230_ = _e628;
            } else {
                if _e614 {
                    phi_1229_ = (1f - _e621);
                } else {
                    phi_1229_ = _e621;
                }
                let _e625 = phi_1229_;
                phi_1230_ = _e625;
            }
            let _e630 = phi_1230_;
            phi_1232_ = _e630;
            break;
        }
        case 0: {
            if (_e496.y > 1f) {
                phi_4572_ = 0.9999999f;
            } else {
                phi_4572_ = select(_e496.y, 0.00000011920929f, (_e496.y < 0f));
            }
            let _e604 = phi_4572_;
            phi_1232_ = _e604;
            break;
        }
        default: {
            phi_1232_ = f32();
            break;
        }
    }
    let _e646 = phi_1232_;
    let _e650 = (_e595 * f32(_e544.member_2.x));
    let _e659 = (_e646 * f32(_e544.member_2.y));
    let _e677 = vec3<f32>((f32((select(select(u32(_e650), 0u, (_e650 < 0f)), 4294967295u, (_e650 > 4294967000f)) + _e544.member_1.x)) / _e475), (f32((select(select(u32(_e659), 0u, (_e659 < 0f)), 4294967295u, (_e659 > 4294967000f)) + _e544.member_1.y)) / _e476), f32(_e544.member_3));
    let _e683 = textureSampleLevel(global_11, global_10, vec2<f32>(_e677.x, _e677.y), i32(_e677.z), 0f);
    let _e686 = select(_e683, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_6 == 4294967295u)));
    let _e690 = select(_e120, _e119, vec2((_e295.member_12 == 0u)));
    if _e301 {
        phi_4638_ = (_e295.member_7 <= (_e116 - 8u));
    } else {
        phi_4638_ = false;
    }
    let _e695 = phi_4638_;
    if _e695 {
        let _e698 = global.member[_e295.member_7];
        let _e702 = global.member[(_e295.member_7 + 1u)];
        let _e707 = global.member[(_e295.member_7 + 2u)];
        let _e711 = global.member[(_e295.member_7 + 3u)];
        let _e716 = global.member[(_e295.member_7 + 4u)];
        let _e720 = global.member[(_e295.member_7 + 5u)];
        let _e724 = global.member[(_e295.member_7 + 6u)];
        switch bitcast<i32>(_e724) {
            case 0: {
                phi_1315_ = 0u;
                break;
            }
            case 1: {
                phi_1315_ = 1u;
                break;
            }
            case 2: {
                phi_1315_ = 2u;
                break;
            }
            default: {
                phi_1315_ = 0u;
                break;
            }
        }
        let _e727 = phi_1315_;
        let _e731 = global.member[(_e295.member_7 + 7u)];
        switch bitcast<i32>(_e731) {
            case 0: {
                phi_1324_ = 0u;
                break;
            }
            case 1: {
                phi_1324_ = 1u;
                break;
            }
            case 2: {
                phi_1324_ = 2u;
                break;
            }
            default: {
                phi_1324_ = 0u;
                break;
            }
        }
        let _e734 = phi_1324_;
        phi_1337_ = type_15(type_14(_e727, _e734), vec2<u32>(_e698, _e702), vec2<u32>(_e707, _e711), _e716, _e720);
    } else {
        phi_1337_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e738 = phi_1337_;
    switch bitcast<i32>(_e738.member.member) {
        case 1: {
            let _e776 = abs(_e690.x);
            let _e778 = (_e776 % 1f);
            if (_e776 >= 1f) {
                phi_4689_ = select(true, false, (_e778 == 0f));
            } else {
                phi_4689_ = true;
            }
            let _e782 = phi_4689_;
            let _e783 = select(1f, _e778, _e782);
            if (select(-1f, 1f, (_e690.x >= 0f)) > 0f) {
                phi_1357_ = _e783;
            } else {
                phi_1357_ = (1f - _e783);
            }
            let _e787 = phi_1357_;
            phi_1394_ = _e787;
            break;
        }
        case 2: {
            let _e750 = abs(_e690.x);
            let _e757 = ((select(select(u32(_e750), 0u, (_e750 < 0f)), 4294967295u, (_e750 > 4294967000f)) % 2u) == 0u);
            let _e759 = (_e750 % 1f);
            if (_e750 >= 1f) {
                phi_4672_ = select(true, false, (_e759 == 0f));
            } else {
                phi_4672_ = true;
            }
            let _e763 = phi_4672_;
            let _e764 = select(1f, _e759, _e763);
            if (select(-1f, 1f, (_e690.x >= 0f)) > 0f) {
                if _e757 {
                    phi_1386_ = _e764;
                } else {
                    phi_1386_ = (1f - _e764);
                }
                let _e771 = phi_1386_;
                phi_1392_ = _e771;
            } else {
                if _e757 {
                    phi_1391_ = (1f - _e764);
                } else {
                    phi_1391_ = _e764;
                }
                let _e768 = phi_1391_;
                phi_1392_ = _e768;
            }
            let _e773 = phi_1392_;
            phi_1394_ = _e773;
            break;
        }
        case 0: {
            if (_e690.x > 1f) {
                phi_4659_ = 0.9999999f;
            } else {
                phi_4659_ = select(_e690.x, 0.00000011920929f, (_e690.x < 0f));
            }
            let _e747 = phi_4659_;
            phi_1394_ = _e747;
            break;
        }
        default: {
            phi_1394_ = f32();
            break;
        }
    }
    let _e789 = phi_1394_;
    switch bitcast<i32>(_e738.member.member_1) {
        case 1: {
            let _e827 = abs(_e690.y);
            let _e829 = (_e827 % 1f);
            if (_e827 >= 1f) {
                phi_4737_ = select(true, false, (_e829 == 0f));
            } else {
                phi_4737_ = true;
            }
            let _e833 = phi_4737_;
            let _e834 = select(1f, _e829, _e833);
            if (select(-1f, 1f, (_e690.y >= 0f)) > 0f) {
                phi_1415_ = _e834;
            } else {
                phi_1415_ = (1f - _e834);
            }
            let _e838 = phi_1415_;
            phi_1452_ = _e838;
            break;
        }
        case 2: {
            let _e801 = abs(_e690.y);
            let _e808 = ((select(select(u32(_e801), 0u, (_e801 < 0f)), 4294967295u, (_e801 > 4294967000f)) % 2u) == 0u);
            let _e810 = (_e801 % 1f);
            if (_e801 >= 1f) {
                phi_4720_ = select(true, false, (_e810 == 0f));
            } else {
                phi_4720_ = true;
            }
            let _e814 = phi_4720_;
            let _e815 = select(1f, _e810, _e814);
            if (select(-1f, 1f, (_e690.y >= 0f)) > 0f) {
                if _e808 {
                    phi_1444_ = _e815;
                } else {
                    phi_1444_ = (1f - _e815);
                }
                let _e822 = phi_1444_;
                phi_1450_ = _e822;
            } else {
                if _e808 {
                    phi_1449_ = (1f - _e815);
                } else {
                    phi_1449_ = _e815;
                }
                let _e819 = phi_1449_;
                phi_1450_ = _e819;
            }
            let _e824 = phi_1450_;
            phi_1452_ = _e824;
            break;
        }
        case 0: {
            if (_e690.y > 1f) {
                phi_4707_ = 0.9999999f;
            } else {
                phi_4707_ = select(_e690.y, 0.00000011920929f, (_e690.y < 0f));
            }
            let _e798 = phi_4707_;
            phi_1452_ = _e798;
            break;
        }
        default: {
            phi_1452_ = f32();
            break;
        }
    }
    let _e840 = phi_1452_;
    let _e844 = (_e789 * f32(_e738.member_2.x));
    let _e853 = (_e840 * f32(_e738.member_2.y));
    let _e871 = vec3<f32>((f32((select(select(u32(_e844), 0u, (_e844 < 0f)), 4294967295u, (_e844 > 4294967000f)) + _e738.member_1.x)) / _e475), (f32((select(select(u32(_e853), 0u, (_e853 < 0f)), 4294967295u, (_e853 > 4294967000f)) + _e738.member_1.y)) / _e476), f32(_e738.member_3));
    let _e877 = textureSampleLevel(global_11, global_10, vec2<f32>(_e871.x, _e871.y), i32(_e871.z), 0f);
    let _e878 = (_e295.member_7 == 4294967295u);
    let _e880 = select(_e877, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e878));
    let _e884 = select(_e120, _e119, vec2((_e295.member_13 == 0u)));
    if _e301 {
        phi_4773_ = (_e295.member_8 <= (_e116 - 8u));
    } else {
        phi_4773_ = false;
    }
    let _e889 = phi_4773_;
    if _e889 {
        let _e892 = global.member[_e295.member_8];
        let _e896 = global.member[(_e295.member_8 + 1u)];
        let _e901 = global.member[(_e295.member_8 + 2u)];
        let _e905 = global.member[(_e295.member_8 + 3u)];
        let _e910 = global.member[(_e295.member_8 + 4u)];
        let _e914 = global.member[(_e295.member_8 + 5u)];
        let _e918 = global.member[(_e295.member_8 + 6u)];
        switch bitcast<i32>(_e918) {
            case 0: {
                phi_1535_ = 0u;
                break;
            }
            case 1: {
                phi_1535_ = 1u;
                break;
            }
            case 2: {
                phi_1535_ = 2u;
                break;
            }
            default: {
                phi_1535_ = 0u;
                break;
            }
        }
        let _e921 = phi_1535_;
        let _e925 = global.member[(_e295.member_8 + 7u)];
        switch bitcast<i32>(_e925) {
            case 0: {
                phi_1544_ = 0u;
                break;
            }
            case 1: {
                phi_1544_ = 1u;
                break;
            }
            case 2: {
                phi_1544_ = 2u;
                break;
            }
            default: {
                phi_1544_ = 0u;
                break;
            }
        }
        let _e928 = phi_1544_;
        phi_1557_ = type_15(type_14(_e921, _e928), vec2<u32>(_e892, _e896), vec2<u32>(_e901, _e905), _e910, _e914);
    } else {
        phi_1557_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e932 = phi_1557_;
    switch bitcast<i32>(_e932.member.member) {
        case 1: {
            let _e970 = abs(_e884.x);
            let _e972 = (_e970 % 1f);
            if (_e970 >= 1f) {
                phi_4824_ = select(true, false, (_e972 == 0f));
            } else {
                phi_4824_ = true;
            }
            let _e976 = phi_4824_;
            let _e977 = select(1f, _e972, _e976);
            if (select(-1f, 1f, (_e884.x >= 0f)) > 0f) {
                phi_1577_ = _e977;
            } else {
                phi_1577_ = (1f - _e977);
            }
            let _e981 = phi_1577_;
            phi_1614_ = _e981;
            break;
        }
        case 2: {
            let _e944 = abs(_e884.x);
            let _e951 = ((select(select(u32(_e944), 0u, (_e944 < 0f)), 4294967295u, (_e944 > 4294967000f)) % 2u) == 0u);
            let _e953 = (_e944 % 1f);
            if (_e944 >= 1f) {
                phi_4807_ = select(true, false, (_e953 == 0f));
            } else {
                phi_4807_ = true;
            }
            let _e957 = phi_4807_;
            let _e958 = select(1f, _e953, _e957);
            if (select(-1f, 1f, (_e884.x >= 0f)) > 0f) {
                if _e951 {
                    phi_1606_ = _e958;
                } else {
                    phi_1606_ = (1f - _e958);
                }
                let _e965 = phi_1606_;
                phi_1612_ = _e965;
            } else {
                if _e951 {
                    phi_1611_ = (1f - _e958);
                } else {
                    phi_1611_ = _e958;
                }
                let _e962 = phi_1611_;
                phi_1612_ = _e962;
            }
            let _e967 = phi_1612_;
            phi_1614_ = _e967;
            break;
        }
        case 0: {
            if (_e884.x > 1f) {
                phi_4794_ = 0.9999999f;
            } else {
                phi_4794_ = select(_e884.x, 0.00000011920929f, (_e884.x < 0f));
            }
            let _e941 = phi_4794_;
            phi_1614_ = _e941;
            break;
        }
        default: {
            phi_1614_ = f32();
            break;
        }
    }
    let _e983 = phi_1614_;
    switch bitcast<i32>(_e932.member.member_1) {
        case 1: {
            let _e1021 = abs(_e884.y);
            let _e1023 = (_e1021 % 1f);
            if (_e1021 >= 1f) {
                phi_4872_ = select(true, false, (_e1023 == 0f));
            } else {
                phi_4872_ = true;
            }
            let _e1027 = phi_4872_;
            let _e1028 = select(1f, _e1023, _e1027);
            if (select(-1f, 1f, (_e884.y >= 0f)) > 0f) {
                phi_1635_ = _e1028;
            } else {
                phi_1635_ = (1f - _e1028);
            }
            let _e1032 = phi_1635_;
            phi_1672_ = _e1032;
            break;
        }
        case 2: {
            let _e995 = abs(_e884.y);
            let _e1002 = ((select(select(u32(_e995), 0u, (_e995 < 0f)), 4294967295u, (_e995 > 4294967000f)) % 2u) == 0u);
            let _e1004 = (_e995 % 1f);
            if (_e995 >= 1f) {
                phi_4855_ = select(true, false, (_e1004 == 0f));
            } else {
                phi_4855_ = true;
            }
            let _e1008 = phi_4855_;
            let _e1009 = select(1f, _e1004, _e1008);
            if (select(-1f, 1f, (_e884.y >= 0f)) > 0f) {
                if _e1002 {
                    phi_1664_ = _e1009;
                } else {
                    phi_1664_ = (1f - _e1009);
                }
                let _e1016 = phi_1664_;
                phi_1670_ = _e1016;
            } else {
                if _e1002 {
                    phi_1669_ = (1f - _e1009);
                } else {
                    phi_1669_ = _e1009;
                }
                let _e1013 = phi_1669_;
                phi_1670_ = _e1013;
            }
            let _e1018 = phi_1670_;
            phi_1672_ = _e1018;
            break;
        }
        case 0: {
            if (_e884.y > 1f) {
                phi_4842_ = 0.9999999f;
            } else {
                phi_4842_ = select(_e884.y, 0.00000011920929f, (_e884.y < 0f));
            }
            let _e992 = phi_4842_;
            phi_1672_ = _e992;
            break;
        }
        default: {
            phi_1672_ = f32();
            break;
        }
    }
    let _e1034 = phi_1672_;
    let _e1038 = (_e983 * f32(_e932.member_2.x));
    let _e1047 = (_e1034 * f32(_e932.member_2.y));
    let _e1065 = vec3<f32>((f32((select(select(u32(_e1038), 0u, (_e1038 < 0f)), 4294967295u, (_e1038 > 4294967000f)) + _e932.member_1.x)) / _e475), (f32((select(select(u32(_e1047), 0u, (_e1047 < 0f)), 4294967295u, (_e1047 > 4294967000f)) + _e932.member_1.y)) / _e476), f32(_e932.member_3));
    let _e1071 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1065.x, _e1065.y), i32(_e1065.z), 0f);
    let _e1078 = select(_e120, _e119, vec2((_e295.member_14 == 0u)));
    if _e301 {
        phi_4908_ = (_e295.member_9 <= (_e116 - 8u));
    } else {
        phi_4908_ = false;
    }
    let _e1083 = phi_4908_;
    if _e1083 {
        let _e1086 = global.member[_e295.member_9];
        let _e1090 = global.member[(_e295.member_9 + 1u)];
        let _e1095 = global.member[(_e295.member_9 + 2u)];
        let _e1099 = global.member[(_e295.member_9 + 3u)];
        let _e1104 = global.member[(_e295.member_9 + 4u)];
        let _e1108 = global.member[(_e295.member_9 + 5u)];
        let _e1112 = global.member[(_e295.member_9 + 6u)];
        switch bitcast<i32>(_e1112) {
            case 0: {
                phi_1755_ = 0u;
                break;
            }
            case 1: {
                phi_1755_ = 1u;
                break;
            }
            case 2: {
                phi_1755_ = 2u;
                break;
            }
            default: {
                phi_1755_ = 0u;
                break;
            }
        }
        let _e1115 = phi_1755_;
        let _e1119 = global.member[(_e295.member_9 + 7u)];
        switch bitcast<i32>(_e1119) {
            case 0: {
                phi_1764_ = 0u;
                break;
            }
            case 1: {
                phi_1764_ = 1u;
                break;
            }
            case 2: {
                phi_1764_ = 2u;
                break;
            }
            default: {
                phi_1764_ = 0u;
                break;
            }
        }
        let _e1122 = phi_1764_;
        phi_1777_ = type_15(type_14(_e1115, _e1122), vec2<u32>(_e1086, _e1090), vec2<u32>(_e1095, _e1099), _e1104, _e1108);
    } else {
        phi_1777_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1126 = phi_1777_;
    switch bitcast<i32>(_e1126.member.member) {
        case 1: {
            let _e1164 = abs(_e1078.x);
            let _e1166 = (_e1164 % 1f);
            if (_e1164 >= 1f) {
                phi_4959_ = select(true, false, (_e1166 == 0f));
            } else {
                phi_4959_ = true;
            }
            let _e1170 = phi_4959_;
            let _e1171 = select(1f, _e1166, _e1170);
            if (select(-1f, 1f, (_e1078.x >= 0f)) > 0f) {
                phi_1797_ = _e1171;
            } else {
                phi_1797_ = (1f - _e1171);
            }
            let _e1175 = phi_1797_;
            phi_1834_ = _e1175;
            break;
        }
        case 2: {
            let _e1138 = abs(_e1078.x);
            let _e1145 = ((select(select(u32(_e1138), 0u, (_e1138 < 0f)), 4294967295u, (_e1138 > 4294967000f)) % 2u) == 0u);
            let _e1147 = (_e1138 % 1f);
            if (_e1138 >= 1f) {
                phi_4942_ = select(true, false, (_e1147 == 0f));
            } else {
                phi_4942_ = true;
            }
            let _e1151 = phi_4942_;
            let _e1152 = select(1f, _e1147, _e1151);
            if (select(-1f, 1f, (_e1078.x >= 0f)) > 0f) {
                if _e1145 {
                    phi_1826_ = _e1152;
                } else {
                    phi_1826_ = (1f - _e1152);
                }
                let _e1159 = phi_1826_;
                phi_1832_ = _e1159;
            } else {
                if _e1145 {
                    phi_1831_ = (1f - _e1152);
                } else {
                    phi_1831_ = _e1152;
                }
                let _e1156 = phi_1831_;
                phi_1832_ = _e1156;
            }
            let _e1161 = phi_1832_;
            phi_1834_ = _e1161;
            break;
        }
        case 0: {
            if (_e1078.x > 1f) {
                phi_4929_ = 0.9999999f;
            } else {
                phi_4929_ = select(_e1078.x, 0.00000011920929f, (_e1078.x < 0f));
            }
            let _e1135 = phi_4929_;
            phi_1834_ = _e1135;
            break;
        }
        default: {
            phi_1834_ = f32();
            break;
        }
    }
    let _e1177 = phi_1834_;
    switch bitcast<i32>(_e1126.member.member_1) {
        case 1: {
            let _e1215 = abs(_e1078.y);
            let _e1217 = (_e1215 % 1f);
            if (_e1215 >= 1f) {
                phi_5007_ = select(true, false, (_e1217 == 0f));
            } else {
                phi_5007_ = true;
            }
            let _e1221 = phi_5007_;
            let _e1222 = select(1f, _e1217, _e1221);
            if (select(-1f, 1f, (_e1078.y >= 0f)) > 0f) {
                phi_1855_ = _e1222;
            } else {
                phi_1855_ = (1f - _e1222);
            }
            let _e1226 = phi_1855_;
            phi_1892_ = _e1226;
            break;
        }
        case 2: {
            let _e1189 = abs(_e1078.y);
            let _e1196 = ((select(select(u32(_e1189), 0u, (_e1189 < 0f)), 4294967295u, (_e1189 > 4294967000f)) % 2u) == 0u);
            let _e1198 = (_e1189 % 1f);
            if (_e1189 >= 1f) {
                phi_4990_ = select(true, false, (_e1198 == 0f));
            } else {
                phi_4990_ = true;
            }
            let _e1202 = phi_4990_;
            let _e1203 = select(1f, _e1198, _e1202);
            if (select(-1f, 1f, (_e1078.y >= 0f)) > 0f) {
                if _e1196 {
                    phi_1884_ = _e1203;
                } else {
                    phi_1884_ = (1f - _e1203);
                }
                let _e1210 = phi_1884_;
                phi_1890_ = _e1210;
            } else {
                if _e1196 {
                    phi_1889_ = (1f - _e1203);
                } else {
                    phi_1889_ = _e1203;
                }
                let _e1207 = phi_1889_;
                phi_1890_ = _e1207;
            }
            let _e1212 = phi_1890_;
            phi_1892_ = _e1212;
            break;
        }
        case 0: {
            if (_e1078.y > 1f) {
                phi_4977_ = 0.9999999f;
            } else {
                phi_4977_ = select(_e1078.y, 0.00000011920929f, (_e1078.y < 0f));
            }
            let _e1186 = phi_4977_;
            phi_1892_ = _e1186;
            break;
        }
        default: {
            phi_1892_ = f32();
            break;
        }
    }
    let _e1228 = phi_1892_;
    let _e1232 = (_e1177 * f32(_e1126.member_2.x));
    let _e1241 = (_e1228 * f32(_e1126.member_2.y));
    let _e1259 = vec3<f32>((f32((select(select(u32(_e1232), 0u, (_e1232 < 0f)), 4294967295u, (_e1232 > 4294967000f)) + _e1126.member_1.x)) / _e475), (f32((select(select(u32(_e1241), 0u, (_e1241 < 0f)), 4294967295u, (_e1241 > 4294967000f)) + _e1126.member_1.y)) / _e476), f32(_e1126.member_3));
    let _e1265 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1259.x, _e1259.y), i32(_e1259.z), 0f);
    let _e1268 = select(_e1265, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_9 == 4294967295u)));
    if _e878 {
        phi_1986_ = vec3<f32>(0f, 0f, 0f);
        phi_1987_ = _e121;
    } else {
        let _e1272 = fma(_e880.x, 2f, -1f);
        let _e1273 = fma(_e880.y, 2f, -1f);
        let _e1274 = fma(_e880.z, 2f, -1f);
        let _e1279 = sqrt(fma(_e1274, _e1274, fma(_e1272, _e1272, (_e1273 * _e1273))));
        if (_e1279 == 0f) {
            phi_5065_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_5065_ = (vec3<f32>(_e1272, _e1273, _e1274) * (1f / _e1279));
        }
        let _e1284 = phi_5065_;
        let _e1291 = sqrt(fma(_e122.z, _e122.z, fma(_e122.x, _e122.x, (_e122.y * _e122.y))));
        if (_e1291 == 0f) {
            phi_5100_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_5100_ = (_e122 * (1f / _e1291));
        }
        let _e1296 = phi_5100_;
        let _e1303 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
        if (_e1303 == 0f) {
            phi_5135_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_5135_ = (_e123 * (1f / _e1303));
        }
        let _e1308 = phi_5135_;
        let _e1315 = sqrt(fma(_e121.z, _e121.z, fma(_e121.x, _e121.x, (_e121.y * _e121.y))));
        if (_e1315 == 0f) {
            phi_5170_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_5170_ = (_e121 * (1f / _e1315));
        }
        let _e1320 = phi_5170_;
        let _e1339 = fma(_e1320.x, _e1284.z, fma(_e1296.x, _e1284.x, (_e1308.x * _e1284.y)));
        let _e1340 = fma(_e1320.y, _e1284.z, fma(_e1296.y, _e1284.x, (_e1308.y * _e1284.y)));
        let _e1341 = fma(_e1320.z, _e1284.z, fma(_e1296.z, _e1284.x, (_e1308.z * _e1284.y)));
        let _e1346 = sqrt(fma(_e1341, _e1341, fma(_e1339, _e1339, (_e1340 * _e1340))));
        if (_e1346 == 0f) {
            phi_5205_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_5205_ = (vec3<f32>(_e1339, _e1340, _e1341) * (1f / _e1346));
        }
        let _e1351 = phi_5205_;
        phi_1986_ = _e1284;
        phi_1987_ = _e1351;
    }
    let _e1353 = phi_1986_;
    let _e1355 = phi_1987_;
    let _e1359 = (_e492.x * _e295.member_2.x);
    let _e1362 = (_e492.y * _e295.member_2.y);
    let _e1365 = (_e492.z * _e295.member_2.z);
    let _e1370 = (_e1359 * _e118.x);
    let _e1372 = (_e1362 * _e118.y);
    let _e1374 = (_e1365 * _e118.z);
    let _e1379 = (_e686.y * _e295.member_4);
    let _e1382 = (_e686.z * _e295.member_3);
    let _e1386 = fma(_e295.member_16, (select(_e1071, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_8 == 4294967295u))).x - 1f), 1f);
    let _e1392 = (_e1268.x * _e295.member.x);
    let _e1394 = (_e1268.y * _e295.member.y);
    let _e1396 = (_e1268.z * _e295.member.z);
    let _e1401 = textureSampleLevel(global_12, global_13, _e1355, 0f);
    if (_e116 >= 86u) {
        phi_5237_ = (_e128 <= (_e116 - 86u));
    } else {
        phi_5237_ = false;
    }
    let _e1409 = phi_5237_;
    if _e1409 {
        let _e1412 = global.member[_e128];
        let _e1417 = global.member[(_e128 + 1u)];
        let _e1422 = global.member[(_e128 + 2u)];
        let _e1427 = global.member[(_e128 + 3u)];
        let _e1433 = global.member[(_e128 + 4u)];
        let _e1438 = global.member[(_e128 + 5u)];
        let _e1443 = global.member[(_e128 + 6u)];
        let _e1448 = global.member[(_e128 + 7u)];
        let _e1454 = global.member[(_e128 + 8u)];
        let _e1459 = global.member[(_e128 + 9u)];
        let _e1464 = global.member[(_e128 + 10u)];
        let _e1469 = global.member[(_e128 + 11u)];
        let _e1475 = global.member[(_e128 + 12u)];
        let _e1480 = global.member[(_e128 + 13u)];
        let _e1485 = global.member[(_e128 + 14u)];
        let _e1490 = global.member[(_e128 + 15u)];
        let _e1497 = global.member[(_e128 + 16u)];
        let _e1502 = global.member[(_e128 + 17u)];
        let _e1507 = global.member[(_e128 + 18u)];
        let _e1512 = global.member[(_e128 + 19u)];
        let _e1518 = global.member[(_e128 + 20u)];
        let _e1523 = global.member[(_e128 + 21u)];
        let _e1528 = global.member[(_e128 + 22u)];
        let _e1533 = global.member[(_e128 + 23u)];
        let _e1539 = global.member[(_e128 + 24u)];
        let _e1544 = global.member[(_e128 + 25u)];
        let _e1549 = global.member[(_e128 + 26u)];
        let _e1554 = global.member[(_e128 + 27u)];
        let _e1560 = global.member[(_e128 + 28u)];
        let _e1565 = global.member[(_e128 + 29u)];
        let _e1570 = global.member[(_e128 + 30u)];
        let _e1575 = global.member[(_e128 + 31u)];
        let _e1582 = global.member[(_e128 + 32u)];
        let _e1587 = global.member[(_e128 + 33u)];
        let _e1592 = global.member[(_e128 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_2194_ = type_14(0u, 6u);
        loop {
            let _e1597 = phi_2194_;
            if (_e1597.member < _e1597.member_1) {
                phi_2195_ = type_14((_e1597.member + 1u), _e1597.member_1);
                phi_2218_ = type_14(1u, _e1597.member);
            } else {
                phi_2195_ = _e1597;
                phi_2218_ = type_14(0u, type_14().member_1);
            }
            let _e1610 = phi_2195_;
            let _e1612 = phi_2218_;
            switch bitcast<i32>(_e1612.member) {
                case 0: {
                    phi_2245_ = false;
                    break;
                }
                case 1: {
                    let _e1617 = ((_e128 + 35u) + (_e1612.member_1 * 4u));
                    let _e1620 = global.member[_e1617];
                    let _e1625 = global.member[(_e1617 + 1u)];
                    let _e1630 = global.member[(_e1617 + 2u)];
                    let _e1635 = global.member[(_e1617 + 3u)];
                    local_1[_e1612.member_1] = vec4<f32>(bitcast<f32>(_e1620), bitcast<f32>(_e1625), bitcast<f32>(_e1630), bitcast<f32>(_e1635));
                    phi_2245_ = true;
                    break;
                }
                default: {
                    phi_2245_ = bool();
                    break;
                }
            }
            let _e1640 = phi_2245_;
            continue;
            continuing {
                phi_2194_ = _e1610;
                break if !(_e1640);
            }
        }
        let _e1642 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2251_ = type_14(0u, 8u);
        loop {
            let _e1645 = phi_2251_;
            if (_e1645.member < _e1645.member_1) {
                phi_2252_ = type_14((_e1645.member + 1u), _e1645.member_1);
                phi_2275_ = type_14(1u, _e1645.member);
            } else {
                phi_2252_ = _e1645;
                phi_2275_ = type_14(0u, type_14().member_1);
            }
            let _e1658 = phi_2252_;
            let _e1660 = phi_2275_;
            switch bitcast<i32>(_e1660.member) {
                case 0: {
                    phi_2298_ = false;
                    break;
                }
                case 1: {
                    let _e1665 = ((_e128 + 59u) + (_e1660.member_1 * 3u));
                    let _e1668 = global.member[_e1665];
                    let _e1673 = global.member[(_e1665 + 1u)];
                    let _e1678 = global.member[(_e1665 + 2u)];
                    local[_e1660.member_1] = vec3<f32>(bitcast<f32>(_e1668), bitcast<f32>(_e1673), bitcast<f32>(_e1678));
                    phi_2298_ = true;
                    break;
                }
                default: {
                    phi_2298_ = bool();
                    break;
                }
            }
            let _e1683 = phi_2298_;
            continue;
            continuing {
                phi_2251_ = _e1658;
                break if !(_e1683);
            }
        }
        let _e1685 = local;
        let _e1689 = global.member[(_e128 + 83u)];
        let _e1694 = global.member[(_e128 + 84u)];
        let _e1699 = global.member[(_e128 + 85u)];
        phi_2319_ = type_25(type_23(vec4<f32>(bitcast<f32>(_e1412), bitcast<f32>(_e1417), bitcast<f32>(_e1422), bitcast<f32>(_e1427)), vec4<f32>(bitcast<f32>(_e1433), bitcast<f32>(_e1438), bitcast<f32>(_e1443), bitcast<f32>(_e1448)), vec4<f32>(bitcast<f32>(_e1454), bitcast<f32>(_e1459), bitcast<f32>(_e1464), bitcast<f32>(_e1469)), vec4<f32>(bitcast<f32>(_e1475), bitcast<f32>(_e1480), bitcast<f32>(_e1485), bitcast<f32>(_e1490))), type_23(vec4<f32>(bitcast<f32>(_e1497), bitcast<f32>(_e1502), bitcast<f32>(_e1507), bitcast<f32>(_e1512)), vec4<f32>(bitcast<f32>(_e1518), bitcast<f32>(_e1523), bitcast<f32>(_e1528), bitcast<f32>(_e1533)), vec4<f32>(bitcast<f32>(_e1539), bitcast<f32>(_e1544), bitcast<f32>(_e1549), bitcast<f32>(_e1554)), vec4<f32>(bitcast<f32>(_e1560), bitcast<f32>(_e1565), bitcast<f32>(_e1570), bitcast<f32>(_e1575))), vec3<f32>(bitcast<f32>(_e1582), bitcast<f32>(_e1587), bitcast<f32>(_e1592)), type_24(_e1685, _e1642, vec3<f32>(bitcast<f32>(_e1689), bitcast<f32>(_e1694), bitcast<f32>(_e1699))));
    } else {
        phi_2319_ = type_25(type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_24(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e1705 = phi_2319_;
    let _e1707 = (_e1705.member_2 - _e124);
    let _e1714 = sqrt(fma(_e1707.z, _e1707.z, fma(_e1707.x, _e1707.x, (_e1707.y * _e1707.y))));
    let _e1715 = (_e1714 == 0f);
    if _e1715 {
        phi_5309_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5309_ = (_e1707 * (1f / _e1714));
    }
    let _e1719 = phi_5309_;
    let _e1720 = -(_e1719);
    let _e1727 = sqrt(fma(_e1355.z, _e1355.z, fma(_e1355.x, _e1355.x, (_e1355.y * _e1355.y))));
    let _e1728 = (_e1727 == 0f);
    if _e1728 {
        phi_5368_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5368_ = (_e1355 * (1f / _e1727));
    }
    let _e1732 = phi_5368_;
    let _e1742 = (2f * fma(_e1732.z, _e1720.z, fma(_e1732.x, _e1720.x, (_e1732.y * _e1720.y))));
    let _e1749 = textureSampleLevel(global_14, global_15, (_e1720 - vec3<f32>((_e1742 * _e1732.x), (_e1742 * _e1732.y), (_e1742 * _e1732.z))), (_e1379 * 4f));
    if _e1715 {
        phi_5442_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5442_ = (_e1707 * (1f / _e1714));
    }
    let _e1756 = phi_5442_;
    let _e1765 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1355.z, _e1756.z, fma(_e1355.x, _e1756.x, (_e1355.y * _e1756.y))), 0f), _e1379), 0f);
    switch bitcast<i32>(_e150) {
        case 0: {
            if _e295.member_15 {
                if _e1728 {
                    phi_5835_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5835_ = (_e1355 * (1f / _e1727));
                }
                let _e1934 = phi_5835_;
                if _e1715 {
                    phi_5870_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5870_ = (_e1707 * (1f / _e1714));
                }
                let _e1938 = phi_5870_;
                phi_2359_ = type_14(0u, _e163);
                phi_2362_ = vec3<f32>(0f, 0f, 0f);
                loop {
                    let _e1941 = phi_2359_;
                    let _e1943 = phi_2362_;
                    local_2 = _e1943;
                    local_3 = _e1943;
                    local_4 = _e1943;
                    if (_e1941.member < _e1941.member_1) {
                        phi_2360_ = type_14((_e1941.member + 1u), _e1941.member_1);
                        phi_2385_ = type_14(1u, _e1941.member);
                    } else {
                        phi_2360_ = _e1941;
                        phi_2385_ = type_14(0u, type_14().member_1);
                    }
                    let _e1956 = phi_2360_;
                    let _e1958 = phi_2385_;
                    switch bitcast<i32>(_e1958.member) {
                        case 0: {
                            phi_2363_ = vec3<f32>();
                            phi_3667_ = false;
                            break;
                        }
                        case 1: {
                            if (_e1958.member_1 >= _e163) {
                                phi_5887_ = 4294967295u;
                            } else {
                                phi_5887_ = (_e159 + _e1958.member_1);
                            }
                            let _e1965 = phi_5887_;
                            if (_e116 >= 1u) {
                                phi_5906_ = (_e1965 <= (_e116 - 1u));
                            } else {
                                phi_5906_ = false;
                            }
                            let _e1970 = phi_5906_;
                            if _e1970 {
                                let _e1973 = global.member[_e1965];
                                phi_2402_ = _e1973;
                            } else {
                                phi_2402_ = 4294967295u;
                            }
                            let _e1975 = phi_2402_;
                            let _e1976 = (_e1975 == 4294967295u);
                            if _e1976 {
                                phi_3665_ = vec3<f32>();
                            } else {
                                if (_e116 >= 4u) {
                                    phi_5938_ = (_e1975 <= (_e116 - 4u));
                                } else {
                                    phi_5938_ = false;
                                }
                                let _e1981 = phi_5938_;
                                if _e1981 {
                                    let _e1984 = global.member[_e1975];
                                    switch bitcast<i32>(_e1984) {
                                        case 0: {
                                            phi_2419_ = 0u;
                                            break;
                                        }
                                        case 1: {
                                            phi_2419_ = 1u;
                                            break;
                                        }
                                        case 2: {
                                            phi_2419_ = 2u;
                                            break;
                                        }
                                        default: {
                                            phi_2419_ = 0u;
                                            break;
                                        }
                                    }
                                    let _e1987 = phi_2419_;
                                    let _e1991 = global.member[(_e1975 + 1u)];
                                    let _e1995 = global.member[(_e1975 + 2u)];
                                    let _e1999 = global.member[(_e1975 + 3u)];
                                    phi_2433_ = type_30(_e1987, _e1991, _e1995, _e1999);
                                } else {
                                    phi_2433_ = type_30(0u, 4294967295u, 4294967295u, 4294967295u);
                                }
                                let _e2002 = phi_2433_;
                                if (_e116 >= 10u) {
                                    phi_5970_ = (_e2002.member_2 <= (_e116 - 10u));
                                } else {
                                    phi_5970_ = false;
                                }
                                let _e2008 = phi_5970_;
                                if _e2008 {
                                    let _e2011 = global.member[_e2002.member_2];
                                    let _e2016 = global.member[(_e2002.member_2 + 1u)];
                                    let _e2021 = global.member[(_e2002.member_2 + 2u)];
                                    let _e2027 = global.member[(_e2002.member_2 + 3u)];
                                    let _e2032 = global.member[(_e2002.member_2 + 4u)];
                                    let _e2037 = global.member[(_e2002.member_2 + 5u)];
                                    let _e2042 = global.member[(_e2002.member_2 + 6u)];
                                    let _e2048 = global.member[(_e2002.member_2 + 7u)];
                                    let _e2053 = global.member[(_e2002.member_2 + 8u)];
                                    let _e2058 = global.member[(_e2002.member_2 + 9u)];
                                    phi_2483_ = type_31(vec3<f32>(bitcast<f32>(_e2011), bitcast<f32>(_e2016), bitcast<f32>(_e2021)), vec4<f32>(bitcast<f32>(_e2027), bitcast<f32>(_e2032), bitcast<f32>(_e2037), bitcast<f32>(_e2042)), vec3<f32>(bitcast<f32>(_e2048), bitcast<f32>(_e2053), bitcast<f32>(_e2058)));
                                } else {
                                    phi_2483_ = type_31(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e2063 = phi_2483_;
                                let _e2071 = (_e2063.member_1.x + _e2063.member_1.x);
                                let _e2072 = (_e2063.member_1.y + _e2063.member_1.y);
                                let _e2073 = (_e2063.member_1.z + _e2063.member_1.z);
                                let _e2075 = (_e2063.member_1.z * _e2073);
                                let _e2076 = (_e2063.member_1.w * _e2071);
                                let _e2077 = (_e2063.member_1.w * _e2072);
                                let _e2078 = (_e2063.member_1.w * _e2073);
                                let _e2098 = (vec4<f32>((1f - fma(_e2063.member_1.y, _e2072, _e2075)), fma(_e2063.member_1.x, _e2072, _e2078), fma(_e2063.member_1.x, _e2073, -(_e2077)), 0f) * _e2063.member_2.x);
                                let _e2100 = (vec4<f32>(fma(_e2063.member_1.x, _e2072, -(_e2078)), (1f - fma(_e2063.member_1.x, _e2071, _e2075)), fma(_e2063.member_1.y, _e2073, _e2076), 0f) * _e2063.member_2.y);
                                let _e2102 = (vec4<f32>(fma(_e2063.member_1.x, _e2073, _e2077), fma(_e2063.member_1.y, _e2073, -(_e2076)), (1f - fma(_e2063.member_1.x, _e2071, (_e2063.member_1.y * _e2072))), 0f) * _e2063.member_2.z);
                                switch bitcast<i32>(_e2002.member) {
                                    case 0: {
                                        if _e301 {
                                            phi_6436_ = (_e2002.member_1 <= (_e116 - 8u));
                                        } else {
                                            phi_6436_ = false;
                                        }
                                        let _e2598 = phi_6436_;
                                        if _e2598 {
                                            let _e2601 = global.member[_e2002.member_1];
                                            let _e2606 = global.member[(_e2002.member_1 + 1u)];
                                            let _e2611 = global.member[(_e2002.member_1 + 2u)];
                                            let _e2617 = global.member[(_e2002.member_1 + 3u)];
                                            let _e2622 = global.member[(_e2002.member_1 + 4u)];
                                            let _e2627 = global.member[(_e2002.member_1 + 5u)];
                                            let _e2632 = global.member[(_e2002.member_1 + 6u)];
                                            let _e2638 = global.member[(_e2002.member_1 + 7u)];
                                            phi_2531_ = type_34(vec3<f32>(bitcast<f32>(_e2601), bitcast<f32>(_e2606), bitcast<f32>(_e2611)), vec4<f32>(bitcast<f32>(_e2617), bitcast<f32>(_e2622), bitcast<f32>(_e2627), bitcast<f32>(_e2632)), bitcast<f32>(_e2638));
                                        } else {
                                            phi_2531_ = type_34(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2642 = phi_2531_;
                                        let _e2664 = fma(_e2102.x, _e2642.member.z, fma(_e2100.x, _e2642.member.y, (_e2098.x * _e2642.member.x)));
                                        let _e2665 = fma(_e2102.y, _e2642.member.z, fma(_e2100.y, _e2642.member.y, (_e2098.y * _e2642.member.x)));
                                        let _e2666 = fma(_e2102.z, _e2642.member.z, fma(_e2100.z, _e2642.member.y, (_e2098.z * _e2642.member.x)));
                                        let _e2671 = sqrt(fma(_e2666, _e2666, fma(_e2664, _e2664, (_e2665 * _e2665))));
                                        if (_e2671 == 0f) {
                                            phi_6483_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6483_ = (vec3<f32>(_e2664, _e2665, _e2666) * (1f / _e2671));
                                        }
                                        let _e2676 = phi_6483_;
                                        let _e2678 = -(_e2676.x);
                                        let _e2680 = -(_e2676.y);
                                        let _e2682 = -(_e2676.z);
                                        let _e2683 = -(_e2676);
                                        let _e2685 = fma(-(_e686.z), _e295.member_3, 1f);
                                        let _e2689 = fma(0.4f, _e2685, (_e1370 * _e1382));
                                        let _e2690 = fma(0.4f, _e2685, (_e1372 * _e1382));
                                        let _e2691 = fma(0.4f, _e2685, (_e1374 * _e1382));
                                        let _e2699 = (_e1938 + vec3<f32>(_e2678, _e2680, _e2682));
                                        let _e2706 = sqrt(fma(_e2699.z, _e2699.z, fma(_e2699.x, _e2699.x, (_e2699.y * _e2699.y))));
                                        if (_e2706 == 0f) {
                                            phi_6518_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6518_ = (_e2699 * (1f / _e2706));
                                        }
                                        let _e2711 = phi_6518_;
                                        let _e2712 = (_e1379 * _e1379);
                                        let _e2723 = max(fma(_e1934.z, _e2711.z, fma(_e1934.x, _e2711.x, (_e1934.y * _e2711.y))), 0f);
                                        let _e2736 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                        let _e2743 = max(fma(_e1934.z, _e2683.z, fma(_e1934.x, _e2683.x, (_e1934.y * _e2683.y))), 0f);
                                        let _e2744 = fma(_e686.y, _e295.member_4, 1f);
                                        let _e2745 = (_e2744 * _e2744);
                                        let _e2746 = (_e2745 * 0.125f);
                                        let _e2748 = fma(-(_e2745), 0.125f, 1f);
                                        let _e2761 = (1f - max(fma(_e2711.z, _e1938.z, fma(_e2711.x, _e1938.x, (_e2711.y * _e1938.y))), 0f));
                                        let _e2763 = select(_e2761, 0f, (_e2761 < 0f));
                                        let _e2766 = pow(select(_e2763, 1f, (_e2763 > 1f)), 5f);
                                        let _e2767 = fma((1f - _e2689), _e2766, _e2689);
                                        let _e2768 = fma((1f - _e2690), _e2766, _e2690);
                                        let _e2769 = fma((1f - _e2691), _e2766, _e2691);
                                        let _e2776 = (((_e2712 * _e2712) / (pow(fma((_e2723 * _e2723), fma(_e2712, _e2712, -1f), 1f), 2f) * 3.1415927f)) * ((_e2736 / fma(_e2736, _e2748, _e2746)) * (_e2743 / fma(_e2743, _e2748, _e2746))));
                                        let _e2782 = fma(_e1934.z, _e2682, fma(_e1934.x, _e2678, (_e1934.y * _e2680)));
                                        let _e2783 = max(_e2782, 0f);
                                        let _e2785 = fma((4f * _e2736), _e2783, 0.0001f);
                                        if ((_e2002.member_3 == 4294967295u) != true) {
                                            let _e2806 = global_1.member[_e2002.member_3];
                                            let _e2810 = global_1.member[(_e2002.member_3 + 1u)];
                                            let _e2814 = global_1.member[(_e2002.member_3 + 2u)];
                                            let _e2818 = global_1.member[(_e2002.member_3 + 3u)];
                                            let _e2822 = global_1.member[(_e2002.member_3 + 4u)];
                                            let _e2827 = global_1.member[(_e2002.member_3 + 5u)];
                                            let _e2833 = global_1.member[select(_e2814, 4294967295u, (0u >= _e2818))];
                                            let _e2836 = global_1.member[_e2833];
                                            let _e2840 = global_1.member[(_e2833 + 1u)];
                                            let _e2844 = global_1.member[(_e2833 + 2u)];
                                            let _e2848 = global_1.member[(_e2833 + 3u)];
                                            let _e2852 = global_1.member[(_e2833 + 4u)];
                                            let _e2856 = global_1.member[(_e2833 + 6u)];
                                            switch bitcast<i32>(_e2856) {
                                                case 0: {
                                                    phi_2768_ = 0u;
                                                    break;
                                                }
                                                case 1: {
                                                    phi_2768_ = 1u;
                                                    break;
                                                }
                                                case 2: {
                                                    phi_2768_ = 2u;
                                                    break;
                                                }
                                                default: {
                                                    phi_2768_ = 0u;
                                                    break;
                                                }
                                            }
                                            let _e2859 = phi_2768_;
                                            let _e2863 = global_1.member[(_e2833 + 7u)];
                                            switch bitcast<i32>(_e2863) {
                                                case 0: {
                                                    phi_2777_ = 0u;
                                                    break;
                                                }
                                                case 1: {
                                                    phi_2777_ = 1u;
                                                    break;
                                                }
                                                case 2: {
                                                    phi_2777_ = 2u;
                                                    break;
                                                }
                                                default: {
                                                    phi_2777_ = 0u;
                                                    break;
                                                }
                                            }
                                            let _e2866 = phi_2777_;
                                            let _e2869 = global_1.member[2u];
                                            let _e2872 = global_1.member[_e2869];
                                            let _e2876 = global_1.member[(_e2869 + 1u)];
                                            let _e2878 = select(_e2806, 4294967295u, (0u >= _e2810));
                                            let _e2881 = global_1.member[_e2878];
                                            let _e2886 = global_1.member[(_e2878 + 1u)];
                                            let _e2891 = global_1.member[(_e2878 + 2u)];
                                            let _e2896 = global_1.member[(_e2878 + 3u)];
                                            let _e2901 = global_1.member[(_e2878 + 4u)];
                                            let _e2906 = global_1.member[(_e2878 + 5u)];
                                            let _e2911 = global_1.member[(_e2878 + 6u)];
                                            let _e2916 = global_1.member[(_e2878 + 7u)];
                                            let _e2921 = global_1.member[(_e2878 + 8u)];
                                            let _e2926 = global_1.member[(_e2878 + 9u)];
                                            let _e2931 = global_1.member[(_e2878 + 10u)];
                                            let _e2936 = global_1.member[(_e2878 + 11u)];
                                            let _e2941 = global_1.member[(_e2878 + 12u)];
                                            let _e2946 = global_1.member[(_e2878 + 13u)];
                                            let _e2951 = global_1.member[(_e2878 + 14u)];
                                            let _e2956 = global_1.member[(_e2878 + 15u)];
                                            let _e2976 = (bitcast<f32>(_e2956) + fma(bitcast<f32>(_e2936), _e124.z, fma(bitcast<f32>(_e2916), _e124.y, (bitcast<f32>(_e2896) * _e124.x))));
                                            let _e2982 = ((((bitcast<f32>(_e2941) + fma(bitcast<f32>(_e2921), _e124.z, fma(bitcast<f32>(_e2901), _e124.y, (bitcast<f32>(_e2881) * _e124.x)))) / _e2976) + 1f) * 0.5f);
                                            let _e2983 = (fma(((bitcast<f32>(_e2946) + fma(bitcast<f32>(_e2926), _e124.z, fma(bitcast<f32>(_e2906), _e124.y, (bitcast<f32>(_e2886) * _e124.x)))) / _e2976), -1f, 1f) * 0.5f);
                                            switch bitcast<i32>(_e2859) {
                                                case 1: {
                                                    let _e3018 = abs(_e2982);
                                                    let _e3020 = (_e3018 % 1f);
                                                    if (_e3018 >= 1f) {
                                                        phi_6654_ = select(true, false, (_e3020 == 0f));
                                                    } else {
                                                        phi_6654_ = true;
                                                    }
                                                    let _e3024 = phi_6654_;
                                                    let _e3025 = select(1f, _e3020, _e3024);
                                                    if (select(-1f, 1f, (_e2982 >= 0f)) > 0f) {
                                                        phi_2944_ = _e3025;
                                                    } else {
                                                        phi_2944_ = (1f - _e3025);
                                                    }
                                                    let _e3029 = phi_2944_;
                                                    phi_2981_ = _e3029;
                                                    break;
                                                }
                                                case 2: {
                                                    let _e2992 = abs(_e2982);
                                                    let _e2999 = ((select(select(u32(_e2992), 0u, (_e2992 < 0f)), 4294967295u, (_e2992 > 4294967000f)) % 2u) == 0u);
                                                    let _e3001 = (_e2992 % 1f);
                                                    if (_e2992 >= 1f) {
                                                        phi_6637_ = select(true, false, (_e3001 == 0f));
                                                    } else {
                                                        phi_6637_ = true;
                                                    }
                                                    let _e3005 = phi_6637_;
                                                    let _e3006 = select(1f, _e3001, _e3005);
                                                    if (select(-1f, 1f, (_e2982 >= 0f)) > 0f) {
                                                        if _e2999 {
                                                            phi_2973_ = _e3006;
                                                        } else {
                                                            phi_2973_ = (1f - _e3006);
                                                        }
                                                        let _e3013 = phi_2973_;
                                                        phi_2979_ = _e3013;
                                                    } else {
                                                        if _e2999 {
                                                            phi_2978_ = (1f - _e3006);
                                                        } else {
                                                            phi_2978_ = _e3006;
                                                        }
                                                        let _e3010 = phi_2978_;
                                                        phi_2979_ = _e3010;
                                                    }
                                                    let _e3015 = phi_2979_;
                                                    phi_2981_ = _e3015;
                                                    break;
                                                }
                                                case 0: {
                                                    if (_e2982 > 1f) {
                                                        phi_6624_ = 0.9999999f;
                                                    } else {
                                                        phi_6624_ = select(_e2982, 0.00000011920929f, (_e2982 < 0f));
                                                    }
                                                    let _e2989 = phi_6624_;
                                                    phi_2981_ = _e2989;
                                                    break;
                                                }
                                                default: {
                                                    phi_2981_ = f32();
                                                    break;
                                                }
                                            }
                                            let _e3031 = phi_2981_;
                                            switch bitcast<i32>(_e2866) {
                                                case 1: {
                                                    let _e3066 = abs(_e2983);
                                                    let _e3068 = (_e3066 % 1f);
                                                    if (_e3066 >= 1f) {
                                                        phi_6702_ = select(true, false, (_e3068 == 0f));
                                                    } else {
                                                        phi_6702_ = true;
                                                    }
                                                    let _e3072 = phi_6702_;
                                                    let _e3073 = select(1f, _e3068, _e3072);
                                                    if (select(-1f, 1f, (_e2983 >= 0f)) > 0f) {
                                                        phi_3000_ = _e3073;
                                                    } else {
                                                        phi_3000_ = (1f - _e3073);
                                                    }
                                                    let _e3077 = phi_3000_;
                                                    phi_3037_ = _e3077;
                                                    break;
                                                }
                                                case 2: {
                                                    let _e3040 = abs(_e2983);
                                                    let _e3047 = ((select(select(u32(_e3040), 0u, (_e3040 < 0f)), 4294967295u, (_e3040 > 4294967000f)) % 2u) == 0u);
                                                    let _e3049 = (_e3040 % 1f);
                                                    if (_e3040 >= 1f) {
                                                        phi_6685_ = select(true, false, (_e3049 == 0f));
                                                    } else {
                                                        phi_6685_ = true;
                                                    }
                                                    let _e3053 = phi_6685_;
                                                    let _e3054 = select(1f, _e3049, _e3053);
                                                    if (select(-1f, 1f, (_e2983 >= 0f)) > 0f) {
                                                        if _e3047 {
                                                            phi_3029_ = _e3054;
                                                        } else {
                                                            phi_3029_ = (1f - _e3054);
                                                        }
                                                        let _e3061 = phi_3029_;
                                                        phi_3035_ = _e3061;
                                                    } else {
                                                        if _e3047 {
                                                            phi_3034_ = (1f - _e3054);
                                                        } else {
                                                            phi_3034_ = _e3054;
                                                        }
                                                        let _e3058 = phi_3034_;
                                                        phi_3035_ = _e3058;
                                                    }
                                                    let _e3063 = phi_3035_;
                                                    phi_3037_ = _e3063;
                                                    break;
                                                }
                                                case 0: {
                                                    if (_e2983 > 1f) {
                                                        phi_6672_ = 0.9999999f;
                                                    } else {
                                                        phi_6672_ = select(_e2983, 0.00000011920929f, (_e2983 < 0f));
                                                    }
                                                    let _e3037 = phi_6672_;
                                                    phi_3037_ = _e3037;
                                                    break;
                                                }
                                                default: {
                                                    phi_3037_ = f32();
                                                    break;
                                                }
                                            }
                                            let _e3079 = phi_3037_;
                                            let _e3081 = (_e3031 * f32(_e2844));
                                            let _e3088 = (_e3079 * f32(_e2848));
                                            let _e3103 = vec3<f32>((f32((select(select(u32(_e3081), 0u, (_e3081 < 0f)), 4294967295u, (_e3081 > 4294967000f)) + _e2836)) / f32(_e2872)), (f32((select(select(u32(_e3088), 0u, (_e3088 < 0f)), 4294967295u, (_e3088 > 4294967000f)) + _e2840)) / f32(_e2876)), f32(_e2852));
                                            let _e3109 = textureSampleLevel(global_19, global_18, vec2<f32>(_e3103.x, _e3103.y), i32(_e3103.z), 0f);
                                            phi_3091_ = select(0f, 1f, ((((bitcast<f32>(_e2951) + fma(bitcast<f32>(_e2931), _e124.z, fma(bitcast<f32>(_e2911), _e124.y, (bitcast<f32>(_e2891) * _e124.x)))) / _e2976) - max((bitcast<f32>(_e2827) * (1f - _e2782)), bitcast<f32>(_e2822))) > _e3109.x));
                                        } else {
                                            phi_3091_ = 0f;
                                        }
                                        let _e3118 = phi_3091_;
                                        let _e3119 = (1f - _e3118);
                                        phi_3655_ = vec3<f32>(fma(((fma((((1f - _e2767) * _e2685) * _e1370), 0.31830987f, ((_e2776 * _e2767) / _e2785)) * (_e2642.member_1.x * _e2642.member_2)) * _e2783), _e3119, _e1943.x), fma(((fma((((1f - _e2768) * _e2685) * _e1372), 0.31830987f, ((_e2776 * _e2768) / _e2785)) * (_e2642.member_1.y * _e2642.member_2)) * _e2783), _e3119, _e1943.y), fma(((fma((((1f - _e2769) * _e2685) * _e1374), 0.31830987f, ((_e2776 * _e2769) / _e2785)) * (_e2642.member_1.z * _e2642.member_2)) * _e2783), _e3119, _e1943.z));
                                        phi_3656_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if _e301 {
                                            phi_6262_ = (_e2002.member_1 <= (_e116 - 8u));
                                        } else {
                                            phi_6262_ = false;
                                        }
                                        let _e2387 = phi_6262_;
                                        if _e2387 {
                                            let _e2390 = global.member[_e2002.member_1];
                                            let _e2395 = global.member[(_e2002.member_1 + 1u)];
                                            let _e2400 = global.member[(_e2002.member_1 + 2u)];
                                            let _e2406 = global.member[(_e2002.member_1 + 3u)];
                                            let _e2411 = global.member[(_e2002.member_1 + 4u)];
                                            let _e2416 = global.member[(_e2002.member_1 + 5u)];
                                            let _e2421 = global.member[(_e2002.member_1 + 6u)];
                                            let _e2427 = global.member[(_e2002.member_1 + 7u)];
                                            phi_3145_ = type_34(vec3<f32>(bitcast<f32>(_e2390), bitcast<f32>(_e2395), bitcast<f32>(_e2400)), vec4<f32>(bitcast<f32>(_e2406), bitcast<f32>(_e2411), bitcast<f32>(_e2416), bitcast<f32>(_e2421)), bitcast<f32>(_e2427));
                                        } else {
                                            phi_3145_ = type_34(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2431 = phi_3145_;
                                        let _e2460 = (vec3<f32>((_e2063.member.x + fma(_e2102.x, _e2431.member.z, fma(_e2100.x, _e2431.member.y, (_e2098.x * _e2431.member.x)))), (_e2063.member.y + fma(_e2102.y, _e2431.member.z, fma(_e2100.y, _e2431.member.y, (_e2098.y * _e2431.member.x)))), (_e2063.member.z + fma(_e2102.z, _e2431.member.z, fma(_e2100.z, _e2431.member.y, (_e2098.z * _e2431.member.x))))) - _e124);
                                        let _e2467 = sqrt(fma(_e2460.z, _e2460.z, fma(_e2460.x, _e2460.x, (_e2460.y * _e2460.y))));
                                        let _e2468 = (_e2467 == 0f);
                                        if _e2468 {
                                            phi_3335_ = vec3<f32>();
                                        } else {
                                            if _e2468 {
                                                phi_6309_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6309_ = (_e2460 * (1f / _e2467));
                                            }
                                            let _e2472 = phi_6309_;
                                            let _e2474 = (_e2431.member_2 / (_e2467 * _e2467));
                                            let _e2476 = fma(-(_e686.z), _e295.member_3, 1f);
                                            let _e2480 = fma(0.4f, _e2476, (_e1370 * _e1382));
                                            let _e2481 = fma(0.4f, _e2476, (_e1372 * _e1382));
                                            let _e2482 = fma(0.4f, _e2476, (_e1374 * _e1382));
                                            let _e2489 = (_e1938 + _e2472);
                                            let _e2496 = sqrt(fma(_e2489.z, _e2489.z, fma(_e2489.x, _e2489.x, (_e2489.y * _e2489.y))));
                                            if (_e2496 == 0f) {
                                                phi_6344_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6344_ = (_e2489 * (1f / _e2496));
                                            }
                                            let _e2501 = phi_6344_;
                                            let _e2502 = (_e1379 * _e1379);
                                            let _e2513 = max(fma(_e1934.z, _e2501.z, fma(_e1934.x, _e2501.x, (_e1934.y * _e2501.y))), 0f);
                                            let _e2526 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                            let _e2533 = max(fma(_e1934.z, _e2472.z, fma(_e1934.x, _e2472.x, (_e1934.y * _e2472.y))), 0f);
                                            let _e2534 = fma(_e686.y, _e295.member_4, 1f);
                                            let _e2535 = (_e2534 * _e2534);
                                            let _e2536 = (_e2535 * 0.125f);
                                            let _e2538 = fma(-(_e2535), 0.125f, 1f);
                                            let _e2551 = (1f - max(fma(_e2501.z, _e1938.z, fma(_e2501.x, _e1938.x, (_e2501.y * _e1938.y))), 0f));
                                            let _e2553 = select(_e2551, 0f, (_e2551 < 0f));
                                            let _e2556 = pow(select(_e2553, 1f, (_e2553 > 1f)), 5f);
                                            let _e2557 = fma((1f - _e2480), _e2556, _e2480);
                                            let _e2558 = fma((1f - _e2481), _e2556, _e2481);
                                            let _e2559 = fma((1f - _e2482), _e2556, _e2482);
                                            let _e2566 = (((_e2502 * _e2502) / (pow(fma((_e2513 * _e2513), fma(_e2502, _e2502, -1f), 1f), 2f) * 3.1415927f)) * ((_e2526 / fma(_e2526, _e2538, _e2536)) * (_e2533 / fma(_e2533, _e2538, _e2536))));
                                            let _e2571 = fma((4f * _e2526), _e2533, 0.0001f);
                                            phi_3335_ = vec3<f32>(fma((fma((((1f - _e2557) * _e2476) * _e1370), 0.31830987f, ((_e2566 * _e2557) / _e2571)) * (_e2431.member_1.x * _e2474)), _e2533, _e1943.x), fma((fma((((1f - _e2558) * _e2476) * _e1372), 0.31830987f, ((_e2566 * _e2558) / _e2571)) * (_e2431.member_1.y * _e2474)), _e2533, _e1943.y), fma((fma((((1f - _e2559) * _e2476) * _e1374), 0.31830987f, ((_e2566 * _e2559) / _e2571)) * (_e2431.member_1.z * _e2474)), _e2533, _e1943.z));
                                        }
                                        let _e2592 = phi_3335_;
                                        phi_3655_ = _e2592;
                                        phi_3656_ = select(true, false, _e2468);
                                        break;
                                    }
                                    case 2: {
                                        if (_e116 >= 13u) {
                                            phi_6050_ = (_e2002.member_1 <= (_e116 - 13u));
                                        } else {
                                            phi_6050_ = false;
                                        }
                                        let _e2113 = phi_6050_;
                                        if _e2113 {
                                            let _e2116 = global.member[_e2002.member_1];
                                            let _e2121 = global.member[(_e2002.member_1 + 1u)];
                                            let _e2126 = global.member[(_e2002.member_1 + 2u)];
                                            let _e2132 = global.member[(_e2002.member_1 + 3u)];
                                            let _e2137 = global.member[(_e2002.member_1 + 4u)];
                                            let _e2142 = global.member[(_e2002.member_1 + 5u)];
                                            let _e2148 = global.member[(_e2002.member_1 + 6u)];
                                            let _e2153 = global.member[(_e2002.member_1 + 7u)];
                                            let _e2158 = global.member[(_e2002.member_1 + 8u)];
                                            let _e2163 = global.member[(_e2002.member_1 + 9u)];
                                            let _e2168 = global.member[(_e2002.member_1 + 10u)];
                                            let _e2173 = global.member[(_e2002.member_1 + 11u)];
                                            let _e2179 = global.member[(_e2002.member_1 + 12u)];
                                            phi_3398_ = type_35(vec3<f32>(bitcast<f32>(_e2116), bitcast<f32>(_e2121), bitcast<f32>(_e2126)), vec3<f32>(bitcast<f32>(_e2132), bitcast<f32>(_e2137), bitcast<f32>(_e2142)), bitcast<f32>(_e2148), bitcast<f32>(_e2153), vec4<f32>(bitcast<f32>(_e2158), bitcast<f32>(_e2163), bitcast<f32>(_e2168), bitcast<f32>(_e2173)), bitcast<f32>(_e2179));
                                        } else {
                                            phi_3398_ = type_35(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2183 = phi_3398_;
                                        let _e2215 = (vec3<f32>((_e2063.member.x + fma(_e2102.x, _e2183.member.z, fma(_e2100.x, _e2183.member.y, (_e2098.x * _e2183.member.x)))), (_e2063.member.y + fma(_e2102.y, _e2183.member.z, fma(_e2100.y, _e2183.member.y, (_e2098.y * _e2183.member.x)))), (_e2063.member.z + fma(_e2102.z, _e2183.member.z, fma(_e2100.z, _e2183.member.y, (_e2098.z * _e2183.member.x))))) - _e124);
                                        let _e2222 = sqrt(fma(_e2215.z, _e2215.z, fma(_e2215.x, _e2215.x, (_e2215.y * _e2215.y))));
                                        let _e2223 = (_e2222 == 0f);
                                        if _e2223 {
                                            phi_3653_ = vec3<f32>();
                                        } else {
                                            if _e2223 {
                                                phi_6100_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6100_ = (_e2215 * (1f / _e2222));
                                            }
                                            let _e2227 = phi_6100_;
                                            let _e2237 = fma(_e2102.x, _e2183.member_1.z, fma(_e2100.x, _e2183.member_1.y, (_e2098.x * _e2183.member_1.x)));
                                            let _e2238 = fma(_e2102.y, _e2183.member_1.z, fma(_e2100.y, _e2183.member_1.y, (_e2098.y * _e2183.member_1.x)));
                                            let _e2239 = fma(_e2102.z, _e2183.member_1.z, fma(_e2100.z, _e2183.member_1.y, (_e2098.z * _e2183.member_1.x)));
                                            let _e2244 = sqrt(fma(_e2239, _e2239, fma(_e2237, _e2237, (_e2238 * _e2238))));
                                            if (_e2244 == 0f) {
                                                phi_6135_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6135_ = (vec3<f32>(_e2237, _e2238, _e2239) * (1f / _e2244));
                                            }
                                            let _e2249 = phi_6135_;
                                            let _e2261 = ((fma(_e2227.z, _e2249.z, fma(_e2227.x, _e2249.x, (_e2227.y * _e2249.y))) - _e2183.member_3) / (_e2183.member_2 - _e2183.member_3));
                                            let _e2263 = select(_e2261, 0f, (_e2261 < 0f));
                                            let _e2266 = (_e2183.member_5 * select(_e2263, 1f, (_e2263 > 1f)));
                                            let _e2268 = fma(-(_e686.z), _e295.member_3, 1f);
                                            let _e2272 = fma(0.4f, _e2268, (_e1370 * _e1382));
                                            let _e2273 = fma(0.4f, _e2268, (_e1372 * _e1382));
                                            let _e2274 = fma(0.4f, _e2268, (_e1374 * _e1382));
                                            let _e2281 = (_e1938 + _e2227);
                                            let _e2288 = sqrt(fma(_e2281.z, _e2281.z, fma(_e2281.x, _e2281.x, (_e2281.y * _e2281.y))));
                                            if (_e2288 == 0f) {
                                                phi_6170_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6170_ = (_e2281 * (1f / _e2288));
                                            }
                                            let _e2293 = phi_6170_;
                                            let _e2294 = (_e1379 * _e1379);
                                            let _e2305 = max(fma(_e1934.z, _e2293.z, fma(_e1934.x, _e2293.x, (_e1934.y * _e2293.y))), 0f);
                                            let _e2318 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                            let _e2322 = max(fma(_e1934.z, _e2227.z, fma(_e1934.x, _e2227.x, (_e1934.y * _e2227.y))), 0f);
                                            let _e2323 = fma(_e686.y, _e295.member_4, 1f);
                                            let _e2324 = (_e2323 * _e2323);
                                            let _e2325 = (_e2324 * 0.125f);
                                            let _e2327 = fma(-(_e2324), 0.125f, 1f);
                                            let _e2340 = (1f - max(fma(_e2293.z, _e1938.z, fma(_e2293.x, _e1938.x, (_e2293.y * _e1938.y))), 0f));
                                            let _e2342 = select(_e2340, 0f, (_e2340 < 0f));
                                            let _e2345 = pow(select(_e2342, 1f, (_e2342 > 1f)), 5f);
                                            let _e2346 = fma((1f - _e2272), _e2345, _e2272);
                                            let _e2347 = fma((1f - _e2273), _e2345, _e2273);
                                            let _e2348 = fma((1f - _e2274), _e2345, _e2274);
                                            let _e2355 = (((_e2294 * _e2294) / (pow(fma((_e2305 * _e2305), fma(_e2294, _e2294, -1f), 1f), 2f) * 3.1415927f)) * ((_e2318 / fma(_e2318, _e2327, _e2325)) * (_e2322 / fma(_e2322, _e2327, _e2325))));
                                            let _e2360 = fma((4f * _e2318), _e2322, 0.0001f);
                                            phi_3653_ = vec3<f32>(fma((fma((((1f - _e2346) * _e2268) * _e1370), 0.31830987f, ((_e2355 * _e2346) / _e2360)) * (_e2183.member_4.x * _e2266)), _e2322, _e1943.x), fma((fma((((1f - _e2347) * _e2268) * _e1372), 0.31830987f, ((_e2355 * _e2347) / _e2360)) * (_e2183.member_4.y * _e2266)), _e2322, _e1943.y), fma((fma((((1f - _e2348) * _e2268) * _e1374), 0.31830987f, ((_e2355 * _e2348) / _e2360)) * (_e2183.member_4.z * _e2266)), _e2322, _e1943.z));
                                        }
                                        let _e2381 = phi_3653_;
                                        phi_3655_ = _e2381;
                                        phi_3656_ = select(true, false, _e2223);
                                        break;
                                    }
                                    default: {
                                        phi_3655_ = vec3<f32>();
                                        phi_3656_ = bool();
                                        break;
                                    }
                                }
                                let _e3128 = phi_3655_;
                                let _e3130 = phi_3656_;
                                phi_3665_ = select(_e3128, _e1943, vec3(select(true, false, _e3130)));
                            }
                            let _e3135 = phi_3665_;
                            phi_2363_ = _e3135;
                            phi_3667_ = select(true, false, _e1976);
                            break;
                        }
                        default: {
                            phi_2363_ = vec3<f32>();
                            phi_3667_ = bool();
                            break;
                        }
                    }
                    let _e3138 = phi_2363_;
                    let _e3140 = phi_3667_;
                    continue;
                    continuing {
                        phi_2359_ = _e1956;
                        phi_2362_ = _e3138;
                        break if !(_e3140);
                    }
                }
                let _e3143 = fma(-(_e686.z), _e295.member_3, 1f);
                let _e3147 = fma(0.04f, _e3143, (_e1370 * _e1382));
                let _e3148 = fma(0.04f, _e3143, (_e1372 * _e1382));
                let _e3149 = fma(0.04f, _e3143, (_e1374 * _e1382));
                let _e3161 = fma(-(_e686.y), _e295.member_4, 1f);
                let _e3168 = (1f - max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f));
                let _e3170 = select(_e3168, 0f, (_e3168 < 0f));
                let _e3173 = pow(select(_e3170, 1f, (_e3170 > 1f)), 5f);
                let _e3174 = fma((max(_e3161, _e3147) - _e3147), _e3173, _e3147);
                let _e3175 = fma((max(_e3161, _e3148) - _e3148), _e3173, _e3148);
                let _e3176 = fma((max(_e3161, _e3149) - _e3149), _e3173, _e3149);
                let _e3196 = local_2;
                let _e3200 = local_3;
                let _e3204 = local_4;
                phi_3784_ = vec4<f32>(fma(_e1392, _e295.member_1, fma(fma(((1f - _e3174) * _e3143), (_e1401.x * _e1370), (_e1749.x * fma(_e3174, _e1765.x, _e1765.y))), _e1386, _e3196.x)), fma(_e1394, _e295.member_1, fma(fma(((1f - _e3175) * _e3143), (_e1401.y * _e1372), (_e1749.y * fma(_e3175, _e1765.x, _e1765.y))), _e1386, _e3200.y)), fma(_e1396, _e295.member_1, fma(fma(((1f - _e3176) * _e3143), (_e1401.z * _e1374), (_e1749.z * fma(_e3176, _e1765.x, _e1765.y))), _e1386, _e3204.z)), 1f);
            } else {
                phi_3784_ = (vec4<f32>((_e118.x * _e492.x), (_e118.y * _e492.y), (_e118.z * _e492.z), (_e118.w * _e492.w)) * _e295.member_2);
            }
            let _e3212 = phi_3784_;
            global_20 = _e3212;
            break;
        }
        case 1: {
            let _e1907 = sqrt(fma(_e119.x, _e119.x, (_e119.y * _e119.y)));
            if (_e1907 == 0f) {
                phi_5796_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5796_ = (vec3<f32>(_e119.x, _e119.y, 0f) * (1f / _e1907));
            }
            let _e1912 = phi_5796_;
            global_20 = vec4<f32>(((_e1912.x + 1f) * 0.5f), ((_e1912.y + 1f) * 0.5f), ((_e1912.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1886 = sqrt(fma(_e120.x, _e120.x, (_e120.y * _e120.y)));
            if (_e1886 == 0f) {
                phi_5747_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5747_ = (vec3<f32>(_e120.x, _e120.y, 0f) * (1f / _e1886));
            }
            let _e1891 = phi_5747_;
            global_20 = vec4<f32>(((_e1891.x + 1f) * 0.5f), ((_e1891.y + 1f) * 0.5f), ((_e1891.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1728 {
                phi_5698_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5698_ = (_e1355 * (1f / _e1727));
            }
            let _e1870 = phi_5698_;
            global_20 = vec4<f32>(((_e1870.x + 1f) * 0.5f), ((_e1870.y + 1f) * 0.5f), ((_e1870.z + 1f) * 0.5f), 1f);
            break;
        }
        case 4: {
            global_20 = _e118;
            break;
        }
        case 5: {
            let _e1851 = sqrt(fma(_e121.z, _e121.z, fma(_e121.x, _e121.x, (_e121.y * _e121.y))));
            if (_e1851 == 0f) {
                phi_5649_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5649_ = (_e121 * (1f / _e1851));
            }
            let _e1856 = phi_5649_;
            global_20 = vec4<f32>(((_e1856.x + 1f) * 0.5f), ((_e1856.y + 1f) * 0.5f), ((_e1856.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1829 = sqrt(fma(_e1353.z, _e1353.z, fma(_e1353.x, _e1353.x, (_e1353.y * _e1353.y))));
            if (_e1829 == 0f) {
                phi_5600_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5600_ = (_e1353 * (1f / _e1829));
            }
            let _e1834 = phi_5600_;
            global_20 = vec4<f32>(((_e1834.x + 1f) * 0.5f), ((_e1834.y + 1f) * 0.5f), ((_e1834.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1807 = sqrt(fma(_e122.z, _e122.z, fma(_e122.x, _e122.x, (_e122.y * _e122.y))));
            if (_e1807 == 0f) {
                phi_5551_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5551_ = (_e122 * (1f / _e1807));
            }
            let _e1812 = phi_5551_;
            global_20 = vec4<f32>(((_e1812.x + 1f) * 0.5f), ((_e1812.y + 1f) * 0.5f), ((_e1812.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1785 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
            if (_e1785 == 0f) {
                phi_5502_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5502_ = (_e123 * (1f / _e1785));
            }
            let _e1790 = phi_5502_;
            global_20 = vec4<f32>(((_e1790.x + 1f) * 0.5f), ((_e1790.y + 1f) * 0.5f), ((_e1790.z + 1f) * 0.5f), 1f);
            break;
        }
        case 9: {
            global_20 = vec4<f32>(_e1401.x, _e1401.y, _e1401.z, 1f);
            break;
        }
        case 10: {
            global_20 = vec4<f32>(_e1749.x, _e1749.y, _e1749.z, 1f);
            break;
        }
        case 11: {
            global_20 = vec4<f32>(_e1765.x, _e1765.y, 1f, 1f);
            break;
        }
        case 12: {
            global_20 = (vec4<f32>(_e1359, _e1362, _e1365, (_e492.w * _e295.member_2.w)) * _e118);
            break;
        }
        case 13: {
            global_20 = vec4<f32>(_e1379, _e1379, _e1379, 1f);
            break;
        }
        case 14: {
            global_20 = vec4<f32>(_e1382, _e1382, _e1382, 1f);
            break;
        }
        case 15: {
            global_20 = vec4<f32>(_e1386, _e1386, _e1386, 1f);
            break;
        }
        case 16: {
            global_20 = vec4<f32>((_e1392 * _e295.member_1), (_e1394 * _e295.member_1), (_e1396 * _e295.member_1), 1f);
            break;
        }
        case 17: {
            global_20 = vec4<f32>(_e1268.x, _e1268.y, _e1268.z, 1f);
            break;
        }
        case 18: {
            global_20 = vec4<f32>(_e295.member.x, _e295.member.y, _e295.member.z, 1f);
            break;
        }
        case 19: {
            global_20 = vec4<f32>(_e295.member_1, _e295.member_1, _e295.member_1, 1f);
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
