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

struct type_28 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_30 {
    member: vec2<u32>,
    member_1: vec2<u32>,
    member_2: type_24,
    member_3: u32,
    member_4: bool,
    member_5: bool,
}

struct type_32 {
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

struct type_33 {
    member: type_24,
    member_1: vec2<u32>,
    member_2: vec2<u32>,
    member_3: u32,
    member_4: u32,
}

struct type_34 {
    member: u32,
    member_1: u32,
    member_2: u32,
}

struct type_35 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: f32,
}

struct type_36 {
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
var<private> global_2: u32;
var<private> global_3: u32;
var<private> global_4: vec4<f32>;
var<private> global_5: vec2<f32>;
var<private> global_6: vec2<f32>;
var<private> global_7: vec3<f32>;
var<private> global_8: vec3<f32>;
var<private> global_9: vec3<f32>;
var<private> global_10: vec3<f32>;
@group(1) @binding(1) 
var global_11: sampler;
@group(1) @binding(0) 
var global_12: texture_2d_array<f32>;
@group(1) @binding(2) 
var global_13: texture_cube<f32>;
@group(1) @binding(3) 
var global_14: sampler;
@group(1) @binding(4) 
var global_15: texture_cube<f32>;
@group(1) @binding(5) 
var global_16: sampler;
@group(1) @binding(6) 
var global_17: texture_2d<f32>;
@group(1) @binding(7) 
var global_18: sampler;
var<private> global_19: vec4<f32>;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var phi_3787_: bool;
    var phi_501_: u32;
    var phi_527_: type_30;
    var phi_3838_: bool;
    var phi_646_: type_32;
    var phi_650_: type_32;
    var phi_3871_: bool;
    var phi_690_: u32;
    var phi_699_: u32;
    var phi_712_: type_33;
    var phi_3931_: f32;
    var phi_3912_: f32;
    var phi_3913_: bool;
    var phi_766_: f32;
    var phi_761_: f32;
    var phi_767_: f32;
    var phi_3893_: f32;
    var phi_3894_: bool;
    var phi_732_: f32;
    var phi_771_: bool;
    var phi_772_: f32;
    var phi_3983_: f32;
    var phi_3964_: f32;
    var phi_3965_: bool;
    var phi_830_: f32;
    var phi_825_: f32;
    var phi_831_: f32;
    var phi_3945_: f32;
    var phi_3946_: bool;
    var phi_796_: f32;
    var phi_835_: bool;
    var phi_836_: f32;
    var phi_4015_: bool;
    var phi_922_: u32;
    var phi_931_: u32;
    var phi_944_: type_33;
    var phi_4074_: f32;
    var phi_4055_: f32;
    var phi_4056_: bool;
    var phi_998_: f32;
    var phi_993_: f32;
    var phi_999_: f32;
    var phi_4036_: f32;
    var phi_4037_: bool;
    var phi_964_: f32;
    var phi_1003_: bool;
    var phi_1004_: f32;
    var phi_4126_: f32;
    var phi_4107_: f32;
    var phi_4108_: bool;
    var phi_1062_: f32;
    var phi_1057_: f32;
    var phi_1063_: f32;
    var phi_4088_: f32;
    var phi_4089_: bool;
    var phi_1028_: f32;
    var phi_1067_: bool;
    var phi_1068_: f32;
    var phi_4158_: bool;
    var phi_1154_: u32;
    var phi_1163_: u32;
    var phi_1176_: type_33;
    var phi_4217_: f32;
    var phi_4198_: f32;
    var phi_4199_: bool;
    var phi_1230_: f32;
    var phi_1225_: f32;
    var phi_1231_: f32;
    var phi_4179_: f32;
    var phi_4180_: bool;
    var phi_1196_: f32;
    var phi_1235_: bool;
    var phi_1236_: f32;
    var phi_4269_: f32;
    var phi_4250_: f32;
    var phi_4251_: bool;
    var phi_1294_: f32;
    var phi_1289_: f32;
    var phi_1295_: f32;
    var phi_4231_: f32;
    var phi_4232_: bool;
    var phi_1260_: f32;
    var phi_1299_: bool;
    var phi_1300_: f32;
    var phi_4301_: bool;
    var phi_1386_: u32;
    var phi_1395_: u32;
    var phi_1408_: type_33;
    var phi_4360_: f32;
    var phi_4341_: f32;
    var phi_4342_: bool;
    var phi_1462_: f32;
    var phi_1457_: f32;
    var phi_1463_: f32;
    var phi_4322_: f32;
    var phi_4323_: bool;
    var phi_1428_: f32;
    var phi_1467_: bool;
    var phi_1468_: f32;
    var phi_4412_: f32;
    var phi_4393_: f32;
    var phi_4394_: bool;
    var phi_1526_: f32;
    var phi_1521_: f32;
    var phi_1527_: f32;
    var phi_4374_: f32;
    var phi_4375_: bool;
    var phi_1492_: f32;
    var phi_1531_: bool;
    var phi_1532_: f32;
    var phi_4444_: bool;
    var phi_1618_: u32;
    var phi_1627_: u32;
    var phi_1640_: type_33;
    var phi_4503_: f32;
    var phi_4484_: f32;
    var phi_4485_: bool;
    var phi_1694_: f32;
    var phi_1689_: f32;
    var phi_1695_: f32;
    var phi_4465_: f32;
    var phi_4466_: bool;
    var phi_1660_: f32;
    var phi_1699_: bool;
    var phi_1700_: f32;
    var phi_4555_: f32;
    var phi_4536_: f32;
    var phi_4537_: bool;
    var phi_1758_: f32;
    var phi_1753_: f32;
    var phi_1759_: f32;
    var phi_4517_: f32;
    var phi_4518_: bool;
    var phi_1724_: f32;
    var phi_1763_: bool;
    var phi_1764_: f32;
    var phi_4609_: vec3<f32>;
    var phi_4644_: vec3<f32>;
    var phi_4679_: vec3<f32>;
    var phi_4714_: vec3<f32>;
    var phi_4749_: vec3<f32>;
    var phi_1861_: vec3<f32>;
    var phi_1862_: vec3<f32>;
    var phi_4781_: bool;
    var phi_2068_: type_24;
    var phi_2084_: type_24;
    var phi_2085_: type_24;
    var phi_2098_: type_24;
    var phi_2114_: type_24;
    var phi_2115_: type_24;
    var phi_2141_: type_22;
    var phi_2142_: bool;
    var phi_2099_: type_24;
    var phi_2162_: type_22;
    var phi_2163_: bool;
    var phi_2069_: type_24;
    var phi_2168_: type_22;
    var phi_4853_: vec3<f32>;
    var phi_4912_: vec3<f32>;
    var phi_4986_: vec3<f32>;
    var phi_6131_: vec3<f32>;
    var phi_6082_: vec3<f32>;
    var phi_6033_: vec3<f32>;
    var phi_5984_: vec3<f32>;
    var phi_5935_: vec3<f32>;
    var phi_5886_: vec3<f32>;
    var phi_5837_: vec3<f32>;
    var phi_5036_: vec3<f32>;
    var phi_5071_: vec3<f32>;
    var phi_2208_: type_24;
    var phi_2211_: vec3<f32>;
    var phi_2226_: type_24;
    var phi_2227_: type_24;
    var phi_5088_: u32;
    var phi_5107_: bool;
    var phi_2244_: u32;
    var phi_5139_: bool;
    var phi_2261_: u32;
    var phi_2271_: type_34;
    var phi_5169_: bool;
    var phi_2321_: type_28;
    var phi_5598_: bool;
    var phi_2824_: type_36;
    var phi_5648_: vec3<f32>;
    var phi_5683_: vec3<f32>;
    var phi_5718_: vec3<f32>;
    var phi_3076_: vec3<f32>;
    var phi_5425_: bool;
    var phi_2573_: type_35;
    var phi_5471_: vec3<f32>;
    var phi_5506_: vec3<f32>;
    var phi_2762_: vec3<f32>;
    var phi_5250_: bool;
    var phi_2369_: type_35;
    var phi_5298_: vec3<f32>;
    var phi_5333_: vec3<f32>;
    var phi_3077_: bool;
    var phi_3078_: bool;
    var phi_3079_: vec3<f32>;
    var phi_3080_: bool;
    var phi_3085_: type_24;
    var phi_3093_: type_24;
    var phi_3097_: bool;
    var phi_3098_: type_24;
    var phi_3099_: vec3<f32>;
    var phi_3100_: bool;
    var phi_3101_: bool;
    var phi_3102_: bool;
    var phi_3103_: bool;
    var phi_3104_: type_24;
    var phi_3105_: vec3<f32>;
    var phi_3111_: type_24;
    var phi_3215_: vec4<f32>;
    var phi_2209_: type_24;
    var phi_3232_: bool;
    var phi_3233_: vec4<f32>;
    var local_2: type_22;
    var local_3: type_22;
    var local_4: bool;
    var local_5: vec4<f32>;

    let _e118 = arrayLength((&global.member));
    let _e119 = global_1;
    let _e120 = global_2;
    let _e121 = global_3;
    let _e122 = global_4;
    let _e123 = global_5;
    let _e124 = global_6;
    let _e125 = global_7;
    let _e126 = global_8;
    let _e127 = global_9;
    let _e128 = global_10;
    if (_e118 >= 9u) {
        phi_3787_ = (_e121 <= (_e118 - 9u));
    } else {
        phi_3787_ = false;
    }
    let _e133 = phi_3787_;
    if _e133 {
        let _e136 = global.member[_e121];
        let _e140 = global.member[(_e121 + 1u)];
        let _e145 = global.member[(_e121 + 2u)];
        let _e149 = global.member[(_e121 + 3u)];
        let _e154 = global.member[(_e121 + 4u)];
        switch bitcast<i32>(_e154) {
            case 0: {
                phi_501_ = 0u;
                break;
            }
            case 1: {
                phi_501_ = 1u;
                break;
            }
            case 2: {
                phi_501_ = 2u;
                break;
            }
            case 3: {
                phi_501_ = 3u;
                break;
            }
            case 4: {
                phi_501_ = 4u;
                break;
            }
            case 5: {
                phi_501_ = 5u;
                break;
            }
            case 6: {
                phi_501_ = 6u;
                break;
            }
            case 7: {
                phi_501_ = 7u;
                break;
            }
            case 8: {
                phi_501_ = 8u;
                break;
            }
            case 9: {
                phi_501_ = 9u;
                break;
            }
            case 10: {
                phi_501_ = 10u;
                break;
            }
            case 11: {
                phi_501_ = 11u;
                break;
            }
            case 12: {
                phi_501_ = 12u;
                break;
            }
            case 13: {
                phi_501_ = 13u;
                break;
            }
            case 14: {
                phi_501_ = 14u;
                break;
            }
            case 15: {
                phi_501_ = 15u;
                break;
            }
            case 16: {
                phi_501_ = 16u;
                break;
            }
            case 17: {
                phi_501_ = 17u;
                break;
            }
            case 18: {
                phi_501_ = 18u;
                break;
            }
            case 19: {
                phi_501_ = 19u;
                break;
            }
            default: {
                phi_501_ = 0u;
                break;
            }
        }
        let _e157 = phi_501_;
        let _e161 = global.member[(_e121 + 5u)];
        let _e166 = global.member[(_e121 + 6u)];
        let _e171 = global.member[(_e121 + 7u)];
        let _e175 = global.member[(_e121 + 8u)];
        phi_527_ = type_30(vec2<u32>(_e136, _e140), vec2<u32>(_e145, _e149), type_24(_e171, _e175), _e157, (_e161 == 1u), (_e166 == 1u));
    } else {
        phi_527_ = type_30(vec2<u32>(0u, 0u), vec2<u32>(1u, 1u), type_24(4294967295u, 0u), 0u, true, true);
    }
    let _e179 = phi_527_;
    if (_e120 == 4294967295u) {
        phi_650_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e118 >= 22u) {
            phi_3838_ = (_e120 <= (_e118 - 22u));
        } else {
            phi_3838_ = false;
        }
        let _e192 = phi_3838_;
        if _e192 {
            let _e195 = global.member[_e120];
            let _e200 = global.member[(_e120 + 1u)];
            let _e205 = global.member[(_e120 + 2u)];
            let _e211 = global.member[(_e120 + 3u)];
            let _e216 = global.member[(_e120 + 4u)];
            let _e221 = global.member[(_e120 + 5u)];
            let _e226 = global.member[(_e120 + 6u)];
            let _e231 = global.member[(_e120 + 7u)];
            let _e237 = global.member[(_e120 + 8u)];
            let _e242 = global.member[(_e120 + 9u)];
            let _e247 = global.member[(_e120 + 10u)];
            let _e251 = global.member[(_e120 + 11u)];
            let _e255 = global.member[(_e120 + 12u)];
            let _e259 = global.member[(_e120 + 13u)];
            let _e263 = global.member[(_e120 + 14u)];
            let _e267 = global.member[(_e120 + 15u)];
            let _e271 = global.member[(_e120 + 16u)];
            let _e275 = global.member[(_e120 + 17u)];
            let _e279 = global.member[(_e120 + 18u)];
            let _e283 = global.member[(_e120 + 19u)];
            let _e287 = global.member[(_e120 + 20u)];
            let _e292 = global.member[(_e120 + 21u)];
            phi_646_ = type_32(vec3<f32>(bitcast<f32>(_e195), bitcast<f32>(_e200), bitcast<f32>(_e205)), bitcast<f32>(_e211), vec4<f32>(bitcast<f32>(_e216), bitcast<f32>(_e221), bitcast<f32>(_e226), bitcast<f32>(_e231)), bitcast<f32>(_e237), bitcast<f32>(_e242), _e247, _e251, _e255, _e259, _e263, _e267, _e271, _e275, _e279, _e283, (_e287 == 1u), bitcast<f32>(_e292));
        } else {
            phi_646_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e296 = phi_646_;
        phi_650_ = type_32(_e296.member, _e296.member_1, _e296.member_2, _e296.member_3, _e296.member_4, _e296.member_5, _e296.member_6, _e296.member_7, _e296.member_8, _e296.member_9, _e296.member_10, _e296.member_11, _e296.member_12, _e296.member_13, _e296.member_14, (_e296.member_15 && _e179.member_4), _e296.member_16);
    }
    let _e318 = phi_650_;
    let _e322 = select(_e124, _e123, vec2((_e318.member_10 == 0u)));
    let _e324 = (_e118 >= 8u);
    if _e324 {
        phi_3871_ = (_e318.member_5 <= (_e118 - 8u));
    } else {
        phi_3871_ = false;
    }
    let _e328 = phi_3871_;
    if _e328 {
        let _e331 = global.member[_e318.member_5];
        let _e335 = global.member[(_e318.member_5 + 1u)];
        let _e340 = global.member[(_e318.member_5 + 2u)];
        let _e344 = global.member[(_e318.member_5 + 3u)];
        let _e349 = global.member[(_e318.member_5 + 4u)];
        let _e353 = global.member[(_e318.member_5 + 5u)];
        let _e357 = global.member[(_e318.member_5 + 6u)];
        switch bitcast<i32>(_e357) {
            case 0: {
                phi_690_ = 0u;
                break;
            }
            case 1: {
                phi_690_ = 1u;
                break;
            }
            case 2: {
                phi_690_ = 2u;
                break;
            }
            default: {
                phi_690_ = 0u;
                break;
            }
        }
        let _e360 = phi_690_;
        let _e364 = global.member[(_e318.member_5 + 7u)];
        switch bitcast<i32>(_e364) {
            case 0: {
                phi_699_ = 0u;
                break;
            }
            case 1: {
                phi_699_ = 1u;
                break;
            }
            case 2: {
                phi_699_ = 2u;
                break;
            }
            default: {
                phi_699_ = 0u;
                break;
            }
        }
        let _e367 = phi_699_;
        phi_712_ = type_33(type_24(_e360, _e367), vec2<u32>(_e331, _e335), vec2<u32>(_e340, _e344), _e349, _e353);
    } else {
        phi_712_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e371 = phi_712_;
    switch bitcast<i32>(_e371.member.member) {
        case 1: {
            let _e412 = abs(_e322.x);
            let _e414 = (_e412 % 1f);
            if (_e412 >= 1f) {
                let _e415 = (_e414 == 0f);
                phi_3893_ = select(f32(), 1f, _e415);
                phi_3894_ = select(true, false, _e415);
            } else {
                phi_3893_ = f32();
                phi_3894_ = true;
            }
            let _e419 = phi_3893_;
            let _e421 = phi_3894_;
            let _e422 = select(_e419, _e414, _e421);
            if (select(-1f, 1f, (_e322.x >= 0f)) > 0f) {
                phi_732_ = _e422;
            } else {
                phi_732_ = (1f - _e422);
            }
            let _e426 = phi_732_;
            phi_771_ = true;
            phi_772_ = _e426;
            break;
        }
        case 2: {
            let _e383 = abs(_e322.x);
            let _e390 = ((select(select(u32(_e383), 0u, (_e383 < 0f)), 4294967295u, (_e383 > 4294967000f)) % 2u) == 0u);
            let _e392 = (_e383 % 1f);
            if (_e383 >= 1f) {
                let _e393 = (_e392 == 0f);
                phi_3912_ = select(f32(), 1f, _e393);
                phi_3913_ = select(true, false, _e393);
            } else {
                phi_3912_ = f32();
                phi_3913_ = true;
            }
            let _e397 = phi_3912_;
            let _e399 = phi_3913_;
            let _e400 = select(_e397, _e392, _e399);
            if (select(-1f, 1f, (_e322.x >= 0f)) > 0f) {
                if _e390 {
                    phi_761_ = _e400;
                } else {
                    phi_761_ = (1f - _e400);
                }
                let _e407 = phi_761_;
                phi_767_ = _e407;
            } else {
                if _e390 {
                    phi_766_ = (1f - _e400);
                } else {
                    phi_766_ = _e400;
                }
                let _e404 = phi_766_;
                phi_767_ = _e404;
            }
            let _e409 = phi_767_;
            phi_771_ = true;
            phi_772_ = _e409;
            break;
        }
        case 0: {
            if (_e322.x > 1f) {
                phi_3931_ = 0.9999999f;
            } else {
                phi_3931_ = select(_e322.x, 0.00000011920929f, (_e322.x < 0f));
            }
            let _e380 = phi_3931_;
            phi_771_ = true;
            phi_772_ = _e380;
            break;
        }
        default: {
            phi_771_ = false;
            phi_772_ = f32();
            break;
        }
    }
    let _e428 = phi_771_;
    let _e430 = phi_772_;
    if _e428 {
        switch bitcast<i32>(_e371.member.member_1) {
            case 1: {
                let _e471 = abs(_e322.y);
                let _e473 = (_e471 % 1f);
                if (_e471 >= 1f) {
                    let _e474 = (_e473 == 0f);
                    phi_3945_ = select(f32(), 1f, _e474);
                    phi_3946_ = select(true, false, _e474);
                } else {
                    phi_3945_ = f32();
                    phi_3946_ = true;
                }
                let _e478 = phi_3945_;
                let _e480 = phi_3946_;
                let _e481 = select(_e478, _e473, _e480);
                if (select(-1f, 1f, (_e322.y >= 0f)) > 0f) {
                    phi_796_ = _e481;
                } else {
                    phi_796_ = (1f - _e481);
                }
                let _e485 = phi_796_;
                phi_835_ = true;
                phi_836_ = _e485;
                break;
            }
            case 2: {
                let _e442 = abs(_e322.y);
                let _e449 = ((select(select(u32(_e442), 0u, (_e442 < 0f)), 4294967295u, (_e442 > 4294967000f)) % 2u) == 0u);
                let _e451 = (_e442 % 1f);
                if (_e442 >= 1f) {
                    let _e452 = (_e451 == 0f);
                    phi_3964_ = select(f32(), 1f, _e452);
                    phi_3965_ = select(true, false, _e452);
                } else {
                    phi_3964_ = f32();
                    phi_3965_ = true;
                }
                let _e456 = phi_3964_;
                let _e458 = phi_3965_;
                let _e459 = select(_e456, _e451, _e458);
                if (select(-1f, 1f, (_e322.y >= 0f)) > 0f) {
                    if _e449 {
                        phi_825_ = _e459;
                    } else {
                        phi_825_ = (1f - _e459);
                    }
                    let _e466 = phi_825_;
                    phi_831_ = _e466;
                } else {
                    if _e449 {
                        phi_830_ = (1f - _e459);
                    } else {
                        phi_830_ = _e459;
                    }
                    let _e463 = phi_830_;
                    phi_831_ = _e463;
                }
                let _e468 = phi_831_;
                phi_835_ = true;
                phi_836_ = _e468;
                break;
            }
            case 0: {
                if (_e322.y > 1f) {
                    phi_3983_ = 0.9999999f;
                } else {
                    phi_3983_ = select(_e322.y, 0.00000011920929f, (_e322.y < 0f));
                }
                let _e439 = phi_3983_;
                phi_835_ = true;
                phi_836_ = _e439;
                break;
            }
            default: {
                phi_835_ = false;
                phi_836_ = f32();
                break;
            }
        }
        let _e487 = phi_835_;
        let _e489 = phi_836_;
        if _e487 {
            let _e493 = (_e430 * f32(_e371.member_2.x));
            let _e502 = (_e489 * f32(_e371.member_2.y));
            let _e515 = f32(_e179.member.x);
            let _e517 = f32(_e179.member.y);
            let _e524 = vec3<f32>((f32((select(select(u32(_e493), 0u, (_e493 < 0f)), 4294967295u, (_e493 > 4294967000f)) + _e371.member_1.x)) / _e515), (f32((select(select(u32(_e502), 0u, (_e502 < 0f)), 4294967295u, (_e502 > 4294967000f)) + _e371.member_1.y)) / _e517), f32(_e371.member_3));
            let _e530 = textureSampleLevel(global_12, global_11, vec2<f32>(_e524.x, _e524.y), i32(_e524.z), 0f);
            let _e533 = select(_e530, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e318.member_5 == 4294967295u)));
            let _e537 = select(_e124, _e123, vec2((_e318.member_11 == 0u)));
            if _e324 {
                phi_4015_ = (_e318.member_6 <= (_e118 - 8u));
            } else {
                phi_4015_ = false;
            }
            let _e542 = phi_4015_;
            if _e542 {
                let _e545 = global.member[_e318.member_6];
                let _e549 = global.member[(_e318.member_6 + 1u)];
                let _e554 = global.member[(_e318.member_6 + 2u)];
                let _e558 = global.member[(_e318.member_6 + 3u)];
                let _e563 = global.member[(_e318.member_6 + 4u)];
                let _e567 = global.member[(_e318.member_6 + 5u)];
                let _e571 = global.member[(_e318.member_6 + 6u)];
                switch bitcast<i32>(_e571) {
                    case 0: {
                        phi_922_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_922_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_922_ = 2u;
                        break;
                    }
                    default: {
                        phi_922_ = 0u;
                        break;
                    }
                }
                let _e574 = phi_922_;
                let _e578 = global.member[(_e318.member_6 + 7u)];
                switch bitcast<i32>(_e578) {
                    case 0: {
                        phi_931_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_931_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_931_ = 2u;
                        break;
                    }
                    default: {
                        phi_931_ = 0u;
                        break;
                    }
                }
                let _e581 = phi_931_;
                phi_944_ = type_33(type_24(_e574, _e581), vec2<u32>(_e545, _e549), vec2<u32>(_e554, _e558), _e563, _e567);
            } else {
                phi_944_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e585 = phi_944_;
            switch bitcast<i32>(_e585.member.member) {
                case 1: {
                    let _e626 = abs(_e537.x);
                    let _e628 = (_e626 % 1f);
                    if (_e626 >= 1f) {
                        let _e629 = (_e628 == 0f);
                        phi_4036_ = select(f32(), 1f, _e629);
                        phi_4037_ = select(true, false, _e629);
                    } else {
                        phi_4036_ = f32();
                        phi_4037_ = true;
                    }
                    let _e633 = phi_4036_;
                    let _e635 = phi_4037_;
                    let _e636 = select(_e633, _e628, _e635);
                    if (select(-1f, 1f, (_e537.x >= 0f)) > 0f) {
                        phi_964_ = _e636;
                    } else {
                        phi_964_ = (1f - _e636);
                    }
                    let _e640 = phi_964_;
                    phi_1003_ = true;
                    phi_1004_ = _e640;
                    break;
                }
                case 2: {
                    let _e597 = abs(_e537.x);
                    let _e604 = ((select(select(u32(_e597), 0u, (_e597 < 0f)), 4294967295u, (_e597 > 4294967000f)) % 2u) == 0u);
                    let _e606 = (_e597 % 1f);
                    if (_e597 >= 1f) {
                        let _e607 = (_e606 == 0f);
                        phi_4055_ = select(f32(), 1f, _e607);
                        phi_4056_ = select(true, false, _e607);
                    } else {
                        phi_4055_ = f32();
                        phi_4056_ = true;
                    }
                    let _e611 = phi_4055_;
                    let _e613 = phi_4056_;
                    let _e614 = select(_e611, _e606, _e613);
                    if (select(-1f, 1f, (_e537.x >= 0f)) > 0f) {
                        if _e604 {
                            phi_993_ = _e614;
                        } else {
                            phi_993_ = (1f - _e614);
                        }
                        let _e621 = phi_993_;
                        phi_999_ = _e621;
                    } else {
                        if _e604 {
                            phi_998_ = (1f - _e614);
                        } else {
                            phi_998_ = _e614;
                        }
                        let _e618 = phi_998_;
                        phi_999_ = _e618;
                    }
                    let _e623 = phi_999_;
                    phi_1003_ = true;
                    phi_1004_ = _e623;
                    break;
                }
                case 0: {
                    if (_e537.x > 1f) {
                        phi_4074_ = 0.9999999f;
                    } else {
                        phi_4074_ = select(_e537.x, 0.00000011920929f, (_e537.x < 0f));
                    }
                    let _e594 = phi_4074_;
                    phi_1003_ = true;
                    phi_1004_ = _e594;
                    break;
                }
                default: {
                    phi_1003_ = false;
                    phi_1004_ = f32();
                    break;
                }
            }
            let _e642 = phi_1003_;
            let _e644 = phi_1004_;
            if _e642 {
                switch bitcast<i32>(_e585.member.member_1) {
                    case 1: {
                        let _e685 = abs(_e537.y);
                        let _e687 = (_e685 % 1f);
                        if (_e685 >= 1f) {
                            let _e688 = (_e687 == 0f);
                            phi_4088_ = select(f32(), 1f, _e688);
                            phi_4089_ = select(true, false, _e688);
                        } else {
                            phi_4088_ = f32();
                            phi_4089_ = true;
                        }
                        let _e692 = phi_4088_;
                        let _e694 = phi_4089_;
                        let _e695 = select(_e692, _e687, _e694);
                        if (select(-1f, 1f, (_e537.y >= 0f)) > 0f) {
                            phi_1028_ = _e695;
                        } else {
                            phi_1028_ = (1f - _e695);
                        }
                        let _e699 = phi_1028_;
                        phi_1067_ = true;
                        phi_1068_ = _e699;
                        break;
                    }
                    case 2: {
                        let _e656 = abs(_e537.y);
                        let _e663 = ((select(select(u32(_e656), 0u, (_e656 < 0f)), 4294967295u, (_e656 > 4294967000f)) % 2u) == 0u);
                        let _e665 = (_e656 % 1f);
                        if (_e656 >= 1f) {
                            let _e666 = (_e665 == 0f);
                            phi_4107_ = select(f32(), 1f, _e666);
                            phi_4108_ = select(true, false, _e666);
                        } else {
                            phi_4107_ = f32();
                            phi_4108_ = true;
                        }
                        let _e670 = phi_4107_;
                        let _e672 = phi_4108_;
                        let _e673 = select(_e670, _e665, _e672);
                        if (select(-1f, 1f, (_e537.y >= 0f)) > 0f) {
                            if _e663 {
                                phi_1057_ = _e673;
                            } else {
                                phi_1057_ = (1f - _e673);
                            }
                            let _e680 = phi_1057_;
                            phi_1063_ = _e680;
                        } else {
                            if _e663 {
                                phi_1062_ = (1f - _e673);
                            } else {
                                phi_1062_ = _e673;
                            }
                            let _e677 = phi_1062_;
                            phi_1063_ = _e677;
                        }
                        let _e682 = phi_1063_;
                        phi_1067_ = true;
                        phi_1068_ = _e682;
                        break;
                    }
                    case 0: {
                        if (_e537.y > 1f) {
                            phi_4126_ = 0.9999999f;
                        } else {
                            phi_4126_ = select(_e537.y, 0.00000011920929f, (_e537.y < 0f));
                        }
                        let _e653 = phi_4126_;
                        phi_1067_ = true;
                        phi_1068_ = _e653;
                        break;
                    }
                    default: {
                        phi_1067_ = false;
                        phi_1068_ = f32();
                        break;
                    }
                }
                let _e701 = phi_1067_;
                let _e703 = phi_1068_;
                if _e701 {
                    let _e707 = (_e644 * f32(_e585.member_2.x));
                    let _e716 = (_e703 * f32(_e585.member_2.y));
                    let _e734 = vec3<f32>((f32((select(select(u32(_e707), 0u, (_e707 < 0f)), 4294967295u, (_e707 > 4294967000f)) + _e585.member_1.x)) / _e515), (f32((select(select(u32(_e716), 0u, (_e716 < 0f)), 4294967295u, (_e716 > 4294967000f)) + _e585.member_1.y)) / _e517), f32(_e585.member_3));
                    let _e740 = textureSampleLevel(global_12, global_11, vec2<f32>(_e734.x, _e734.y), i32(_e734.z), 0f);
                    let _e743 = select(_e740, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e318.member_6 == 4294967295u)));
                    let _e747 = select(_e124, _e123, vec2((_e318.member_12 == 0u)));
                    if _e324 {
                        phi_4158_ = (_e318.member_7 <= (_e118 - 8u));
                    } else {
                        phi_4158_ = false;
                    }
                    let _e752 = phi_4158_;
                    if _e752 {
                        let _e755 = global.member[_e318.member_7];
                        let _e759 = global.member[(_e318.member_7 + 1u)];
                        let _e764 = global.member[(_e318.member_7 + 2u)];
                        let _e768 = global.member[(_e318.member_7 + 3u)];
                        let _e773 = global.member[(_e318.member_7 + 4u)];
                        let _e777 = global.member[(_e318.member_7 + 5u)];
                        let _e781 = global.member[(_e318.member_7 + 6u)];
                        switch bitcast<i32>(_e781) {
                            case 0: {
                                phi_1154_ = 0u;
                                break;
                            }
                            case 1: {
                                phi_1154_ = 1u;
                                break;
                            }
                            case 2: {
                                phi_1154_ = 2u;
                                break;
                            }
                            default: {
                                phi_1154_ = 0u;
                                break;
                            }
                        }
                        let _e784 = phi_1154_;
                        let _e788 = global.member[(_e318.member_7 + 7u)];
                        switch bitcast<i32>(_e788) {
                            case 0: {
                                phi_1163_ = 0u;
                                break;
                            }
                            case 1: {
                                phi_1163_ = 1u;
                                break;
                            }
                            case 2: {
                                phi_1163_ = 2u;
                                break;
                            }
                            default: {
                                phi_1163_ = 0u;
                                break;
                            }
                        }
                        let _e791 = phi_1163_;
                        phi_1176_ = type_33(type_24(_e784, _e791), vec2<u32>(_e755, _e759), vec2<u32>(_e764, _e768), _e773, _e777);
                    } else {
                        phi_1176_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
                    }
                    let _e795 = phi_1176_;
                    switch bitcast<i32>(_e795.member.member) {
                        case 1: {
                            let _e836 = abs(_e747.x);
                            let _e838 = (_e836 % 1f);
                            if (_e836 >= 1f) {
                                let _e839 = (_e838 == 0f);
                                phi_4179_ = select(f32(), 1f, _e839);
                                phi_4180_ = select(true, false, _e839);
                            } else {
                                phi_4179_ = f32();
                                phi_4180_ = true;
                            }
                            let _e843 = phi_4179_;
                            let _e845 = phi_4180_;
                            let _e846 = select(_e843, _e838, _e845);
                            if (select(-1f, 1f, (_e747.x >= 0f)) > 0f) {
                                phi_1196_ = _e846;
                            } else {
                                phi_1196_ = (1f - _e846);
                            }
                            let _e850 = phi_1196_;
                            phi_1235_ = true;
                            phi_1236_ = _e850;
                            break;
                        }
                        case 2: {
                            let _e807 = abs(_e747.x);
                            let _e814 = ((select(select(u32(_e807), 0u, (_e807 < 0f)), 4294967295u, (_e807 > 4294967000f)) % 2u) == 0u);
                            let _e816 = (_e807 % 1f);
                            if (_e807 >= 1f) {
                                let _e817 = (_e816 == 0f);
                                phi_4198_ = select(f32(), 1f, _e817);
                                phi_4199_ = select(true, false, _e817);
                            } else {
                                phi_4198_ = f32();
                                phi_4199_ = true;
                            }
                            let _e821 = phi_4198_;
                            let _e823 = phi_4199_;
                            let _e824 = select(_e821, _e816, _e823);
                            if (select(-1f, 1f, (_e747.x >= 0f)) > 0f) {
                                if _e814 {
                                    phi_1225_ = _e824;
                                } else {
                                    phi_1225_ = (1f - _e824);
                                }
                                let _e831 = phi_1225_;
                                phi_1231_ = _e831;
                            } else {
                                if _e814 {
                                    phi_1230_ = (1f - _e824);
                                } else {
                                    phi_1230_ = _e824;
                                }
                                let _e828 = phi_1230_;
                                phi_1231_ = _e828;
                            }
                            let _e833 = phi_1231_;
                            phi_1235_ = true;
                            phi_1236_ = _e833;
                            break;
                        }
                        case 0: {
                            if (_e747.x > 1f) {
                                phi_4217_ = 0.9999999f;
                            } else {
                                phi_4217_ = select(_e747.x, 0.00000011920929f, (_e747.x < 0f));
                            }
                            let _e804 = phi_4217_;
                            phi_1235_ = true;
                            phi_1236_ = _e804;
                            break;
                        }
                        default: {
                            phi_1235_ = false;
                            phi_1236_ = f32();
                            break;
                        }
                    }
                    let _e852 = phi_1235_;
                    let _e854 = phi_1236_;
                    if _e852 {
                        switch bitcast<i32>(_e795.member.member_1) {
                            case 1: {
                                let _e895 = abs(_e747.y);
                                let _e897 = (_e895 % 1f);
                                if (_e895 >= 1f) {
                                    let _e898 = (_e897 == 0f);
                                    phi_4231_ = select(f32(), 1f, _e898);
                                    phi_4232_ = select(true, false, _e898);
                                } else {
                                    phi_4231_ = f32();
                                    phi_4232_ = true;
                                }
                                let _e902 = phi_4231_;
                                let _e904 = phi_4232_;
                                let _e905 = select(_e902, _e897, _e904);
                                if (select(-1f, 1f, (_e747.y >= 0f)) > 0f) {
                                    phi_1260_ = _e905;
                                } else {
                                    phi_1260_ = (1f - _e905);
                                }
                                let _e909 = phi_1260_;
                                phi_1299_ = true;
                                phi_1300_ = _e909;
                                break;
                            }
                            case 2: {
                                let _e866 = abs(_e747.y);
                                let _e873 = ((select(select(u32(_e866), 0u, (_e866 < 0f)), 4294967295u, (_e866 > 4294967000f)) % 2u) == 0u);
                                let _e875 = (_e866 % 1f);
                                if (_e866 >= 1f) {
                                    let _e876 = (_e875 == 0f);
                                    phi_4250_ = select(f32(), 1f, _e876);
                                    phi_4251_ = select(true, false, _e876);
                                } else {
                                    phi_4250_ = f32();
                                    phi_4251_ = true;
                                }
                                let _e880 = phi_4250_;
                                let _e882 = phi_4251_;
                                let _e883 = select(_e880, _e875, _e882);
                                if (select(-1f, 1f, (_e747.y >= 0f)) > 0f) {
                                    if _e873 {
                                        phi_1289_ = _e883;
                                    } else {
                                        phi_1289_ = (1f - _e883);
                                    }
                                    let _e890 = phi_1289_;
                                    phi_1295_ = _e890;
                                } else {
                                    if _e873 {
                                        phi_1294_ = (1f - _e883);
                                    } else {
                                        phi_1294_ = _e883;
                                    }
                                    let _e887 = phi_1294_;
                                    phi_1295_ = _e887;
                                }
                                let _e892 = phi_1295_;
                                phi_1299_ = true;
                                phi_1300_ = _e892;
                                break;
                            }
                            case 0: {
                                if (_e747.y > 1f) {
                                    phi_4269_ = 0.9999999f;
                                } else {
                                    phi_4269_ = select(_e747.y, 0.00000011920929f, (_e747.y < 0f));
                                }
                                let _e863 = phi_4269_;
                                phi_1299_ = true;
                                phi_1300_ = _e863;
                                break;
                            }
                            default: {
                                phi_1299_ = false;
                                phi_1300_ = f32();
                                break;
                            }
                        }
                        let _e911 = phi_1299_;
                        let _e913 = phi_1300_;
                        if _e911 {
                            let _e917 = (_e854 * f32(_e795.member_2.x));
                            let _e926 = (_e913 * f32(_e795.member_2.y));
                            let _e944 = vec3<f32>((f32((select(select(u32(_e917), 0u, (_e917 < 0f)), 4294967295u, (_e917 > 4294967000f)) + _e795.member_1.x)) / _e515), (f32((select(select(u32(_e926), 0u, (_e926 < 0f)), 4294967295u, (_e926 > 4294967000f)) + _e795.member_1.y)) / _e517), f32(_e795.member_3));
                            let _e950 = textureSampleLevel(global_12, global_11, vec2<f32>(_e944.x, _e944.y), i32(_e944.z), 0f);
                            let _e951 = (_e318.member_7 == 4294967295u);
                            let _e953 = select(_e950, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e951));
                            let _e957 = select(_e124, _e123, vec2((_e318.member_13 == 0u)));
                            if _e324 {
                                phi_4301_ = (_e318.member_8 <= (_e118 - 8u));
                            } else {
                                phi_4301_ = false;
                            }
                            let _e962 = phi_4301_;
                            if _e962 {
                                let _e965 = global.member[_e318.member_8];
                                let _e969 = global.member[(_e318.member_8 + 1u)];
                                let _e974 = global.member[(_e318.member_8 + 2u)];
                                let _e978 = global.member[(_e318.member_8 + 3u)];
                                let _e983 = global.member[(_e318.member_8 + 4u)];
                                let _e987 = global.member[(_e318.member_8 + 5u)];
                                let _e991 = global.member[(_e318.member_8 + 6u)];
                                switch bitcast<i32>(_e991) {
                                    case 0: {
                                        phi_1386_ = 0u;
                                        break;
                                    }
                                    case 1: {
                                        phi_1386_ = 1u;
                                        break;
                                    }
                                    case 2: {
                                        phi_1386_ = 2u;
                                        break;
                                    }
                                    default: {
                                        phi_1386_ = 0u;
                                        break;
                                    }
                                }
                                let _e994 = phi_1386_;
                                let _e998 = global.member[(_e318.member_8 + 7u)];
                                switch bitcast<i32>(_e998) {
                                    case 0: {
                                        phi_1395_ = 0u;
                                        break;
                                    }
                                    case 1: {
                                        phi_1395_ = 1u;
                                        break;
                                    }
                                    case 2: {
                                        phi_1395_ = 2u;
                                        break;
                                    }
                                    default: {
                                        phi_1395_ = 0u;
                                        break;
                                    }
                                }
                                let _e1001 = phi_1395_;
                                phi_1408_ = type_33(type_24(_e994, _e1001), vec2<u32>(_e965, _e969), vec2<u32>(_e974, _e978), _e983, _e987);
                            } else {
                                phi_1408_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
                            }
                            let _e1005 = phi_1408_;
                            switch bitcast<i32>(_e1005.member.member) {
                                case 1: {
                                    let _e1046 = abs(_e957.x);
                                    let _e1048 = (_e1046 % 1f);
                                    if (_e1046 >= 1f) {
                                        let _e1049 = (_e1048 == 0f);
                                        phi_4322_ = select(f32(), 1f, _e1049);
                                        phi_4323_ = select(true, false, _e1049);
                                    } else {
                                        phi_4322_ = f32();
                                        phi_4323_ = true;
                                    }
                                    let _e1053 = phi_4322_;
                                    let _e1055 = phi_4323_;
                                    let _e1056 = select(_e1053, _e1048, _e1055);
                                    if (select(-1f, 1f, (_e957.x >= 0f)) > 0f) {
                                        phi_1428_ = _e1056;
                                    } else {
                                        phi_1428_ = (1f - _e1056);
                                    }
                                    let _e1060 = phi_1428_;
                                    phi_1467_ = true;
                                    phi_1468_ = _e1060;
                                    break;
                                }
                                case 2: {
                                    let _e1017 = abs(_e957.x);
                                    let _e1024 = ((select(select(u32(_e1017), 0u, (_e1017 < 0f)), 4294967295u, (_e1017 > 4294967000f)) % 2u) == 0u);
                                    let _e1026 = (_e1017 % 1f);
                                    if (_e1017 >= 1f) {
                                        let _e1027 = (_e1026 == 0f);
                                        phi_4341_ = select(f32(), 1f, _e1027);
                                        phi_4342_ = select(true, false, _e1027);
                                    } else {
                                        phi_4341_ = f32();
                                        phi_4342_ = true;
                                    }
                                    let _e1031 = phi_4341_;
                                    let _e1033 = phi_4342_;
                                    let _e1034 = select(_e1031, _e1026, _e1033);
                                    if (select(-1f, 1f, (_e957.x >= 0f)) > 0f) {
                                        if _e1024 {
                                            phi_1457_ = _e1034;
                                        } else {
                                            phi_1457_ = (1f - _e1034);
                                        }
                                        let _e1041 = phi_1457_;
                                        phi_1463_ = _e1041;
                                    } else {
                                        if _e1024 {
                                            phi_1462_ = (1f - _e1034);
                                        } else {
                                            phi_1462_ = _e1034;
                                        }
                                        let _e1038 = phi_1462_;
                                        phi_1463_ = _e1038;
                                    }
                                    let _e1043 = phi_1463_;
                                    phi_1467_ = true;
                                    phi_1468_ = _e1043;
                                    break;
                                }
                                case 0: {
                                    if (_e957.x > 1f) {
                                        phi_4360_ = 0.9999999f;
                                    } else {
                                        phi_4360_ = select(_e957.x, 0.00000011920929f, (_e957.x < 0f));
                                    }
                                    let _e1014 = phi_4360_;
                                    phi_1467_ = true;
                                    phi_1468_ = _e1014;
                                    break;
                                }
                                default: {
                                    phi_1467_ = false;
                                    phi_1468_ = f32();
                                    break;
                                }
                            }
                            let _e1062 = phi_1467_;
                            let _e1064 = phi_1468_;
                            if _e1062 {
                                switch bitcast<i32>(_e1005.member.member_1) {
                                    case 1: {
                                        let _e1105 = abs(_e957.y);
                                        let _e1107 = (_e1105 % 1f);
                                        if (_e1105 >= 1f) {
                                            let _e1108 = (_e1107 == 0f);
                                            phi_4374_ = select(f32(), 1f, _e1108);
                                            phi_4375_ = select(true, false, _e1108);
                                        } else {
                                            phi_4374_ = f32();
                                            phi_4375_ = true;
                                        }
                                        let _e1112 = phi_4374_;
                                        let _e1114 = phi_4375_;
                                        let _e1115 = select(_e1112, _e1107, _e1114);
                                        if (select(-1f, 1f, (_e957.y >= 0f)) > 0f) {
                                            phi_1492_ = _e1115;
                                        } else {
                                            phi_1492_ = (1f - _e1115);
                                        }
                                        let _e1119 = phi_1492_;
                                        phi_1531_ = true;
                                        phi_1532_ = _e1119;
                                        break;
                                    }
                                    case 2: {
                                        let _e1076 = abs(_e957.y);
                                        let _e1083 = ((select(select(u32(_e1076), 0u, (_e1076 < 0f)), 4294967295u, (_e1076 > 4294967000f)) % 2u) == 0u);
                                        let _e1085 = (_e1076 % 1f);
                                        if (_e1076 >= 1f) {
                                            let _e1086 = (_e1085 == 0f);
                                            phi_4393_ = select(f32(), 1f, _e1086);
                                            phi_4394_ = select(true, false, _e1086);
                                        } else {
                                            phi_4393_ = f32();
                                            phi_4394_ = true;
                                        }
                                        let _e1090 = phi_4393_;
                                        let _e1092 = phi_4394_;
                                        let _e1093 = select(_e1090, _e1085, _e1092);
                                        if (select(-1f, 1f, (_e957.y >= 0f)) > 0f) {
                                            if _e1083 {
                                                phi_1521_ = _e1093;
                                            } else {
                                                phi_1521_ = (1f - _e1093);
                                            }
                                            let _e1100 = phi_1521_;
                                            phi_1527_ = _e1100;
                                        } else {
                                            if _e1083 {
                                                phi_1526_ = (1f - _e1093);
                                            } else {
                                                phi_1526_ = _e1093;
                                            }
                                            let _e1097 = phi_1526_;
                                            phi_1527_ = _e1097;
                                        }
                                        let _e1102 = phi_1527_;
                                        phi_1531_ = true;
                                        phi_1532_ = _e1102;
                                        break;
                                    }
                                    case 0: {
                                        if (_e957.y > 1f) {
                                            phi_4412_ = 0.9999999f;
                                        } else {
                                            phi_4412_ = select(_e957.y, 0.00000011920929f, (_e957.y < 0f));
                                        }
                                        let _e1073 = phi_4412_;
                                        phi_1531_ = true;
                                        phi_1532_ = _e1073;
                                        break;
                                    }
                                    default: {
                                        phi_1531_ = false;
                                        phi_1532_ = f32();
                                        break;
                                    }
                                }
                                let _e1121 = phi_1531_;
                                let _e1123 = phi_1532_;
                                if _e1121 {
                                    let _e1127 = (_e1064 * f32(_e1005.member_2.x));
                                    let _e1136 = (_e1123 * f32(_e1005.member_2.y));
                                    let _e1154 = vec3<f32>((f32((select(select(u32(_e1127), 0u, (_e1127 < 0f)), 4294967295u, (_e1127 > 4294967000f)) + _e1005.member_1.x)) / _e515), (f32((select(select(u32(_e1136), 0u, (_e1136 < 0f)), 4294967295u, (_e1136 > 4294967000f)) + _e1005.member_1.y)) / _e517), f32(_e1005.member_3));
                                    let _e1160 = textureSampleLevel(global_12, global_11, vec2<f32>(_e1154.x, _e1154.y), i32(_e1154.z), 0f);
                                    let _e1167 = select(_e124, _e123, vec2((_e318.member_14 == 0u)));
                                    if _e324 {
                                        phi_4444_ = (_e318.member_9 <= (_e118 - 8u));
                                    } else {
                                        phi_4444_ = false;
                                    }
                                    let _e1172 = phi_4444_;
                                    if _e1172 {
                                        let _e1175 = global.member[_e318.member_9];
                                        let _e1179 = global.member[(_e318.member_9 + 1u)];
                                        let _e1184 = global.member[(_e318.member_9 + 2u)];
                                        let _e1188 = global.member[(_e318.member_9 + 3u)];
                                        let _e1193 = global.member[(_e318.member_9 + 4u)];
                                        let _e1197 = global.member[(_e318.member_9 + 5u)];
                                        let _e1201 = global.member[(_e318.member_9 + 6u)];
                                        switch bitcast<i32>(_e1201) {
                                            case 0: {
                                                phi_1618_ = 0u;
                                                break;
                                            }
                                            case 1: {
                                                phi_1618_ = 1u;
                                                break;
                                            }
                                            case 2: {
                                                phi_1618_ = 2u;
                                                break;
                                            }
                                            default: {
                                                phi_1618_ = 0u;
                                                break;
                                            }
                                        }
                                        let _e1204 = phi_1618_;
                                        let _e1208 = global.member[(_e318.member_9 + 7u)];
                                        switch bitcast<i32>(_e1208) {
                                            case 0: {
                                                phi_1627_ = 0u;
                                                break;
                                            }
                                            case 1: {
                                                phi_1627_ = 1u;
                                                break;
                                            }
                                            case 2: {
                                                phi_1627_ = 2u;
                                                break;
                                            }
                                            default: {
                                                phi_1627_ = 0u;
                                                break;
                                            }
                                        }
                                        let _e1211 = phi_1627_;
                                        phi_1640_ = type_33(type_24(_e1204, _e1211), vec2<u32>(_e1175, _e1179), vec2<u32>(_e1184, _e1188), _e1193, _e1197);
                                    } else {
                                        phi_1640_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
                                    }
                                    let _e1215 = phi_1640_;
                                    switch bitcast<i32>(_e1215.member.member) {
                                        case 1: {
                                            let _e1256 = abs(_e1167.x);
                                            let _e1258 = (_e1256 % 1f);
                                            if (_e1256 >= 1f) {
                                                let _e1259 = (_e1258 == 0f);
                                                phi_4465_ = select(f32(), 1f, _e1259);
                                                phi_4466_ = select(true, false, _e1259);
                                            } else {
                                                phi_4465_ = f32();
                                                phi_4466_ = true;
                                            }
                                            let _e1263 = phi_4465_;
                                            let _e1265 = phi_4466_;
                                            let _e1266 = select(_e1263, _e1258, _e1265);
                                            if (select(-1f, 1f, (_e1167.x >= 0f)) > 0f) {
                                                phi_1660_ = _e1266;
                                            } else {
                                                phi_1660_ = (1f - _e1266);
                                            }
                                            let _e1270 = phi_1660_;
                                            phi_1699_ = true;
                                            phi_1700_ = _e1270;
                                            break;
                                        }
                                        case 2: {
                                            let _e1227 = abs(_e1167.x);
                                            let _e1234 = ((select(select(u32(_e1227), 0u, (_e1227 < 0f)), 4294967295u, (_e1227 > 4294967000f)) % 2u) == 0u);
                                            let _e1236 = (_e1227 % 1f);
                                            if (_e1227 >= 1f) {
                                                let _e1237 = (_e1236 == 0f);
                                                phi_4484_ = select(f32(), 1f, _e1237);
                                                phi_4485_ = select(true, false, _e1237);
                                            } else {
                                                phi_4484_ = f32();
                                                phi_4485_ = true;
                                            }
                                            let _e1241 = phi_4484_;
                                            let _e1243 = phi_4485_;
                                            let _e1244 = select(_e1241, _e1236, _e1243);
                                            if (select(-1f, 1f, (_e1167.x >= 0f)) > 0f) {
                                                if _e1234 {
                                                    phi_1689_ = _e1244;
                                                } else {
                                                    phi_1689_ = (1f - _e1244);
                                                }
                                                let _e1251 = phi_1689_;
                                                phi_1695_ = _e1251;
                                            } else {
                                                if _e1234 {
                                                    phi_1694_ = (1f - _e1244);
                                                } else {
                                                    phi_1694_ = _e1244;
                                                }
                                                let _e1248 = phi_1694_;
                                                phi_1695_ = _e1248;
                                            }
                                            let _e1253 = phi_1695_;
                                            phi_1699_ = true;
                                            phi_1700_ = _e1253;
                                            break;
                                        }
                                        case 0: {
                                            if (_e1167.x > 1f) {
                                                phi_4503_ = 0.9999999f;
                                            } else {
                                                phi_4503_ = select(_e1167.x, 0.00000011920929f, (_e1167.x < 0f));
                                            }
                                            let _e1224 = phi_4503_;
                                            phi_1699_ = true;
                                            phi_1700_ = _e1224;
                                            break;
                                        }
                                        default: {
                                            phi_1699_ = false;
                                            phi_1700_ = f32();
                                            break;
                                        }
                                    }
                                    let _e1272 = phi_1699_;
                                    let _e1274 = phi_1700_;
                                    if _e1272 {
                                        switch bitcast<i32>(_e1215.member.member_1) {
                                            case 1: {
                                                let _e1315 = abs(_e1167.y);
                                                let _e1317 = (_e1315 % 1f);
                                                if (_e1315 >= 1f) {
                                                    let _e1318 = (_e1317 == 0f);
                                                    phi_4517_ = select(f32(), 1f, _e1318);
                                                    phi_4518_ = select(true, false, _e1318);
                                                } else {
                                                    phi_4517_ = f32();
                                                    phi_4518_ = true;
                                                }
                                                let _e1322 = phi_4517_;
                                                let _e1324 = phi_4518_;
                                                let _e1325 = select(_e1322, _e1317, _e1324);
                                                if (select(-1f, 1f, (_e1167.y >= 0f)) > 0f) {
                                                    phi_1724_ = _e1325;
                                                } else {
                                                    phi_1724_ = (1f - _e1325);
                                                }
                                                let _e1329 = phi_1724_;
                                                phi_1763_ = true;
                                                phi_1764_ = _e1329;
                                                break;
                                            }
                                            case 2: {
                                                let _e1286 = abs(_e1167.y);
                                                let _e1293 = ((select(select(u32(_e1286), 0u, (_e1286 < 0f)), 4294967295u, (_e1286 > 4294967000f)) % 2u) == 0u);
                                                let _e1295 = (_e1286 % 1f);
                                                if (_e1286 >= 1f) {
                                                    let _e1296 = (_e1295 == 0f);
                                                    phi_4536_ = select(f32(), 1f, _e1296);
                                                    phi_4537_ = select(true, false, _e1296);
                                                } else {
                                                    phi_4536_ = f32();
                                                    phi_4537_ = true;
                                                }
                                                let _e1300 = phi_4536_;
                                                let _e1302 = phi_4537_;
                                                let _e1303 = select(_e1300, _e1295, _e1302);
                                                if (select(-1f, 1f, (_e1167.y >= 0f)) > 0f) {
                                                    if _e1293 {
                                                        phi_1753_ = _e1303;
                                                    } else {
                                                        phi_1753_ = (1f - _e1303);
                                                    }
                                                    let _e1310 = phi_1753_;
                                                    phi_1759_ = _e1310;
                                                } else {
                                                    if _e1293 {
                                                        phi_1758_ = (1f - _e1303);
                                                    } else {
                                                        phi_1758_ = _e1303;
                                                    }
                                                    let _e1307 = phi_1758_;
                                                    phi_1759_ = _e1307;
                                                }
                                                let _e1312 = phi_1759_;
                                                phi_1763_ = true;
                                                phi_1764_ = _e1312;
                                                break;
                                            }
                                            case 0: {
                                                if (_e1167.y > 1f) {
                                                    phi_4555_ = 0.9999999f;
                                                } else {
                                                    phi_4555_ = select(_e1167.y, 0.00000011920929f, (_e1167.y < 0f));
                                                }
                                                let _e1283 = phi_4555_;
                                                phi_1763_ = true;
                                                phi_1764_ = _e1283;
                                                break;
                                            }
                                            default: {
                                                phi_1763_ = false;
                                                phi_1764_ = f32();
                                                break;
                                            }
                                        }
                                        let _e1331 = phi_1763_;
                                        let _e1333 = phi_1764_;
                                        if _e1331 {
                                            let _e1337 = (_e1274 * f32(_e1215.member_2.x));
                                            let _e1346 = (_e1333 * f32(_e1215.member_2.y));
                                            let _e1364 = vec3<f32>((f32((select(select(u32(_e1337), 0u, (_e1337 < 0f)), 4294967295u, (_e1337 > 4294967000f)) + _e1215.member_1.x)) / _e515), (f32((select(select(u32(_e1346), 0u, (_e1346 < 0f)), 4294967295u, (_e1346 > 4294967000f)) + _e1215.member_1.y)) / _e517), f32(_e1215.member_3));
                                            let _e1370 = textureSampleLevel(global_12, global_11, vec2<f32>(_e1364.x, _e1364.y), i32(_e1364.z), 0f);
                                            let _e1373 = select(_e1370, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e318.member_9 == 4294967295u)));
                                            if _e951 {
                                                phi_1861_ = vec3<f32>(0f, 0f, 0f);
                                                phi_1862_ = _e125;
                                            } else {
                                                let _e1377 = fma(_e953.x, 2f, -1f);
                                                let _e1378 = fma(_e953.y, 2f, -1f);
                                                let _e1379 = fma(_e953.z, 2f, -1f);
                                                let _e1384 = sqrt(fma(_e1379, _e1379, fma(_e1377, _e1377, (_e1378 * _e1378))));
                                                if (_e1384 == 0f) {
                                                    phi_4609_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_4609_ = (vec3<f32>(_e1377, _e1378, _e1379) * (1f / _e1384));
                                                }
                                                let _e1389 = phi_4609_;
                                                let _e1396 = sqrt(fma(_e126.z, _e126.z, fma(_e126.x, _e126.x, (_e126.y * _e126.y))));
                                                if (_e1396 == 0f) {
                                                    phi_4644_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_4644_ = (_e126 * (1f / _e1396));
                                                }
                                                let _e1401 = phi_4644_;
                                                let _e1408 = sqrt(fma(_e127.z, _e127.z, fma(_e127.x, _e127.x, (_e127.y * _e127.y))));
                                                if (_e1408 == 0f) {
                                                    phi_4679_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_4679_ = (_e127 * (1f / _e1408));
                                                }
                                                let _e1413 = phi_4679_;
                                                let _e1420 = sqrt(fma(_e125.z, _e125.z, fma(_e125.x, _e125.x, (_e125.y * _e125.y))));
                                                if (_e1420 == 0f) {
                                                    phi_4714_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_4714_ = (_e125 * (1f / _e1420));
                                                }
                                                let _e1425 = phi_4714_;
                                                let _e1444 = fma(_e1425.x, _e1389.z, fma(_e1401.x, _e1389.x, (_e1413.x * _e1389.y)));
                                                let _e1445 = fma(_e1425.y, _e1389.z, fma(_e1401.y, _e1389.x, (_e1413.y * _e1389.y)));
                                                let _e1446 = fma(_e1425.z, _e1389.z, fma(_e1401.z, _e1389.x, (_e1413.z * _e1389.y)));
                                                let _e1451 = sqrt(fma(_e1446, _e1446, fma(_e1444, _e1444, (_e1445 * _e1445))));
                                                if (_e1451 == 0f) {
                                                    phi_4749_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_4749_ = (vec3<f32>(_e1444, _e1445, _e1446) * (1f / _e1451));
                                                }
                                                let _e1456 = phi_4749_;
                                                phi_1861_ = _e1389;
                                                phi_1862_ = _e1456;
                                            }
                                            let _e1458 = phi_1861_;
                                            let _e1460 = phi_1862_;
                                            let _e1464 = (_e533.x * _e318.member_2.x);
                                            let _e1467 = (_e533.y * _e318.member_2.y);
                                            let _e1470 = (_e533.z * _e318.member_2.z);
                                            let _e1475 = (_e1464 * _e122.x);
                                            let _e1477 = (_e1467 * _e122.y);
                                            let _e1479 = (_e1470 * _e122.z);
                                            let _e1484 = (_e743.y * _e318.member_4);
                                            let _e1487 = (_e743.z * _e318.member_3);
                                            let _e1491 = fma(_e318.member_16, (select(_e1160, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e318.member_8 == 4294967295u))).x - 1f), 1f);
                                            let _e1497 = (_e1373.x * _e318.member.x);
                                            let _e1499 = (_e1373.y * _e318.member.y);
                                            let _e1501 = (_e1373.z * _e318.member.z);
                                            let _e1506 = textureSampleLevel(global_13, global_14, _e1460, 0f);
                                            if (_e118 >= 83u) {
                                                phi_4781_ = (_e119 <= (_e118 - 83u));
                                            } else {
                                                phi_4781_ = false;
                                            }
                                            let _e1514 = phi_4781_;
                                            if _e1514 {
                                                let _e1517 = global.member[_e119];
                                                let _e1522 = global.member[(_e119 + 1u)];
                                                let _e1527 = global.member[(_e119 + 2u)];
                                                let _e1532 = global.member[(_e119 + 3u)];
                                                let _e1538 = global.member[(_e119 + 4u)];
                                                let _e1543 = global.member[(_e119 + 5u)];
                                                let _e1548 = global.member[(_e119 + 6u)];
                                                let _e1553 = global.member[(_e119 + 7u)];
                                                let _e1559 = global.member[(_e119 + 8u)];
                                                let _e1564 = global.member[(_e119 + 9u)];
                                                let _e1569 = global.member[(_e119 + 10u)];
                                                let _e1574 = global.member[(_e119 + 11u)];
                                                let _e1580 = global.member[(_e119 + 12u)];
                                                let _e1585 = global.member[(_e119 + 13u)];
                                                let _e1590 = global.member[(_e119 + 14u)];
                                                let _e1595 = global.member[(_e119 + 15u)];
                                                let _e1602 = global.member[(_e119 + 16u)];
                                                let _e1607 = global.member[(_e119 + 17u)];
                                                let _e1612 = global.member[(_e119 + 18u)];
                                                let _e1617 = global.member[(_e119 + 19u)];
                                                let _e1623 = global.member[(_e119 + 20u)];
                                                let _e1628 = global.member[(_e119 + 21u)];
                                                let _e1633 = global.member[(_e119 + 22u)];
                                                let _e1638 = global.member[(_e119 + 23u)];
                                                let _e1644 = global.member[(_e119 + 24u)];
                                                let _e1649 = global.member[(_e119 + 25u)];
                                                let _e1654 = global.member[(_e119 + 26u)];
                                                let _e1659 = global.member[(_e119 + 27u)];
                                                let _e1665 = global.member[(_e119 + 28u)];
                                                let _e1670 = global.member[(_e119 + 29u)];
                                                let _e1675 = global.member[(_e119 + 30u)];
                                                let _e1680 = global.member[(_e119 + 31u)];
                                                let _e1687 = global.member[(_e119 + 32u)];
                                                let _e1692 = global.member[(_e119 + 33u)];
                                                let _e1697 = global.member[(_e119 + 34u)];
                                                local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                                phi_2068_ = type_24(0u, 6u);
                                                loop {
                                                    let _e1702 = phi_2068_;
                                                    if (_e1702.member < _e1702.member_1) {
                                                        phi_2084_ = type_24((_e1702.member + 1u), _e1702.member_1);
                                                        phi_2085_ = type_24(1u, _e1702.member);
                                                    } else {
                                                        phi_2084_ = _e1702;
                                                        phi_2085_ = type_24(0u, type_24().member_1);
                                                    }
                                                    let _e1715 = phi_2084_;
                                                    let _e1717 = phi_2085_;
                                                    switch bitcast<i32>(_e1717.member) {
                                                        case 0: {
                                                            let _e1744 = local_1;
                                                            local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                            phi_2098_ = type_24(0u, 8u);
                                                            loop {
                                                                let _e1747 = phi_2098_;
                                                                if (_e1747.member < _e1747.member_1) {
                                                                    phi_2114_ = type_24((_e1747.member + 1u), _e1747.member_1);
                                                                    phi_2115_ = type_24(1u, _e1747.member);
                                                                } else {
                                                                    phi_2114_ = _e1747;
                                                                    phi_2115_ = type_24(0u, type_24().member_1);
                                                                }
                                                                let _e1760 = phi_2114_;
                                                                let _e1762 = phi_2115_;
                                                                switch bitcast<i32>(_e1762.member) {
                                                                    case 0: {
                                                                        let _e1784 = local;
                                                                        phi_2141_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1517), bitcast<f32>(_e1522), bitcast<f32>(_e1527), bitcast<f32>(_e1532)), vec4<f32>(bitcast<f32>(_e1538), bitcast<f32>(_e1543), bitcast<f32>(_e1548), bitcast<f32>(_e1553)), vec4<f32>(bitcast<f32>(_e1559), bitcast<f32>(_e1564), bitcast<f32>(_e1569), bitcast<f32>(_e1574)), vec4<f32>(bitcast<f32>(_e1580), bitcast<f32>(_e1585), bitcast<f32>(_e1590), bitcast<f32>(_e1595))), type_20(vec4<f32>(bitcast<f32>(_e1602), bitcast<f32>(_e1607), bitcast<f32>(_e1612), bitcast<f32>(_e1617)), vec4<f32>(bitcast<f32>(_e1623), bitcast<f32>(_e1628), bitcast<f32>(_e1633), bitcast<f32>(_e1638)), vec4<f32>(bitcast<f32>(_e1644), bitcast<f32>(_e1649), bitcast<f32>(_e1654), bitcast<f32>(_e1659)), vec4<f32>(bitcast<f32>(_e1665), bitcast<f32>(_e1670), bitcast<f32>(_e1675), bitcast<f32>(_e1680))), type_21(_e1784, _e1744), vec3<f32>(bitcast<f32>(_e1687), bitcast<f32>(_e1692), bitcast<f32>(_e1697)));
                                                                        phi_2142_ = false;
                                                                        phi_2099_ = type_24();
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        let _e1767 = ((_e119 + 59u) + (_e1762.member_1 * 3u));
                                                                        let _e1770 = global.member[_e1767];
                                                                        let _e1775 = global.member[(_e1767 + 1u)];
                                                                        let _e1780 = global.member[(_e1767 + 2u)];
                                                                        local[_e1762.member_1] = vec3<f32>(bitcast<f32>(_e1770), bitcast<f32>(_e1775), bitcast<f32>(_e1780));
                                                                        phi_2141_ = type_22();
                                                                        phi_2142_ = true;
                                                                        phi_2099_ = _e1760;
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        phi_2141_ = type_22();
                                                                        phi_2142_ = false;
                                                                        phi_2099_ = type_24();
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1788 = phi_2141_;
                                                                let _e1790 = phi_2142_;
                                                                let _e1792 = phi_2099_;
                                                                local_2 = _e1788;
                                                                continue;
                                                                continuing {
                                                                    phi_2098_ = _e1792;
                                                                    break if !(_e1790);
                                                                }
                                                            }
                                                            let _e3177 = local_2;
                                                            phi_2162_ = _e3177;
                                                            phi_2163_ = false;
                                                            phi_2069_ = type_24();
                                                            break;
                                                        }
                                                        case 1: {
                                                            let _e1722 = ((_e119 + 35u) + (_e1717.member_1 * 4u));
                                                            let _e1725 = global.member[_e1722];
                                                            let _e1730 = global.member[(_e1722 + 1u)];
                                                            let _e1735 = global.member[(_e1722 + 2u)];
                                                            let _e1740 = global.member[(_e1722 + 3u)];
                                                            local_1[_e1717.member_1] = vec4<f32>(bitcast<f32>(_e1725), bitcast<f32>(_e1730), bitcast<f32>(_e1735), bitcast<f32>(_e1740));
                                                            phi_2162_ = type_22();
                                                            phi_2163_ = true;
                                                            phi_2069_ = _e1715;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_2162_ = type_22();
                                                            phi_2163_ = false;
                                                            phi_2069_ = type_24();
                                                            break;
                                                        }
                                                    }
                                                    let _e1795 = phi_2162_;
                                                    let _e1797 = phi_2163_;
                                                    let _e1799 = phi_2069_;
                                                    local_3 = _e1795;
                                                    continue;
                                                    continuing {
                                                        phi_2068_ = _e1799;
                                                        break if !(_e1797);
                                                    }
                                                }
                                                let _e3182 = local_3;
                                                phi_2168_ = _e3182;
                                            } else {
                                                phi_2168_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
                                            }
                                            let _e1802 = phi_2168_;
                                            let _e1804 = (_e1802.member_3 - _e128);
                                            let _e1811 = sqrt(fma(_e1804.z, _e1804.z, fma(_e1804.x, _e1804.x, (_e1804.y * _e1804.y))));
                                            let _e1812 = (_e1811 == 0f);
                                            if _e1812 {
                                                phi_4853_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_4853_ = (_e1804 * (1f / _e1811));
                                            }
                                            let _e1816 = phi_4853_;
                                            let _e1817 = -(_e1816);
                                            let _e1824 = sqrt(fma(_e1460.z, _e1460.z, fma(_e1460.x, _e1460.x, (_e1460.y * _e1460.y))));
                                            let _e1825 = (_e1824 == 0f);
                                            if _e1825 {
                                                phi_4912_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_4912_ = (_e1460 * (1f / _e1824));
                                            }
                                            let _e1829 = phi_4912_;
                                            let _e1839 = (2f * fma(_e1829.z, _e1817.z, fma(_e1829.x, _e1817.x, (_e1829.y * _e1817.y))));
                                            let _e1846 = textureSampleLevel(global_15, global_16, (_e1817 - vec3<f32>((_e1839 * _e1829.x), (_e1839 * _e1829.y), (_e1839 * _e1829.z))), (_e1484 * 4f));
                                            if _e1812 {
                                                phi_4986_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_4986_ = (_e1804 * (1f / _e1811));
                                            }
                                            let _e1853 = phi_4986_;
                                            let _e1862 = textureSampleLevel(global_17, global_18, vec2<f32>(max(fma(_e1460.z, _e1853.z, fma(_e1460.x, _e1853.x, (_e1460.y * _e1853.y))), 0f), _e1484), 0f);
                                            switch bitcast<i32>(_e179.member_3) {
                                                case 0: {
                                                    if _e318.member_15 {
                                                        if _e1825 {
                                                            phi_5036_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_5036_ = (_e1460 * (1f / _e1824));
                                                        }
                                                        let _e2031 = phi_5036_;
                                                        if _e1812 {
                                                            phi_5071_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_5071_ = (_e1804 * (1f / _e1811));
                                                        }
                                                        let _e2035 = phi_5071_;
                                                        phi_2208_ = type_24(0u, _e179.member_2.member_1);
                                                        phi_2211_ = vec3<f32>(0f, 0f, 0f);
                                                        loop {
                                                            let _e2038 = phi_2208_;
                                                            let _e2040 = phi_2211_;
                                                            if (_e2038.member < _e2038.member_1) {
                                                                phi_2226_ = type_24((_e2038.member + 1u), _e2038.member_1);
                                                                phi_2227_ = type_24(1u, _e2038.member);
                                                            } else {
                                                                phi_2226_ = _e2038;
                                                                phi_2227_ = type_24(0u, type_24().member_1);
                                                            }
                                                            let _e2053 = phi_2226_;
                                                            let _e2055 = phi_2227_;
                                                            switch bitcast<i32>(_e2055.member) {
                                                                case 0: {
                                                                    phi_3101_ = false;
                                                                    phi_3102_ = true;
                                                                    phi_3103_ = false;
                                                                    phi_3104_ = type_24();
                                                                    phi_3105_ = vec3<f32>();
                                                                    break;
                                                                }
                                                                case 1: {
                                                                    if (_e2055.member_1 >= _e179.member_2.member_1) {
                                                                        phi_5088_ = 4294967295u;
                                                                    } else {
                                                                        phi_5088_ = (_e179.member_2.member + _e2055.member_1);
                                                                    }
                                                                    let _e2062 = phi_5088_;
                                                                    if (_e118 >= 1u) {
                                                                        phi_5107_ = (_e2062 <= (_e118 - 1u));
                                                                    } else {
                                                                        phi_5107_ = false;
                                                                    }
                                                                    let _e2067 = phi_5107_;
                                                                    if _e2067 {
                                                                        let _e2070 = global.member[_e2062];
                                                                        phi_2244_ = _e2070;
                                                                    } else {
                                                                        phi_2244_ = 4294967295u;
                                                                    }
                                                                    let _e2072 = phi_2244_;
                                                                    let _e2073 = (_e2072 == 4294967295u);
                                                                    if _e2073 {
                                                                        phi_3097_ = false;
                                                                        phi_3098_ = type_24();
                                                                        phi_3099_ = vec3<f32>();
                                                                        phi_3100_ = false;
                                                                    } else {
                                                                        if (_e118 >= 3u) {
                                                                            phi_5139_ = (_e2072 <= (_e118 - 3u));
                                                                        } else {
                                                                            phi_5139_ = false;
                                                                        }
                                                                        let _e2078 = phi_5139_;
                                                                        if _e2078 {
                                                                            let _e2081 = global.member[_e2072];
                                                                            switch bitcast<i32>(_e2081) {
                                                                                case 0: {
                                                                                    phi_2261_ = 0u;
                                                                                    break;
                                                                                }
                                                                                case 1: {
                                                                                    phi_2261_ = 1u;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    phi_2261_ = 2u;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_2261_ = 0u;
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2084 = phi_2261_;
                                                                            let _e2088 = global.member[(_e2072 + 1u)];
                                                                            let _e2092 = global.member[(_e2072 + 2u)];
                                                                            phi_2271_ = type_34(_e2084, _e2088, _e2092);
                                                                        } else {
                                                                            phi_2271_ = type_34(0u, 4294967295u, 4294967295u);
                                                                        }
                                                                        let _e2095 = phi_2271_;
                                                                        if (_e118 >= 10u) {
                                                                            phi_5169_ = (_e2095.member_2 <= (_e118 - 10u));
                                                                        } else {
                                                                            phi_5169_ = false;
                                                                        }
                                                                        let _e2101 = phi_5169_;
                                                                        if _e2101 {
                                                                            let _e2104 = global.member[_e2095.member_2];
                                                                            let _e2109 = global.member[(_e2095.member_2 + 1u)];
                                                                            let _e2114 = global.member[(_e2095.member_2 + 2u)];
                                                                            let _e2120 = global.member[(_e2095.member_2 + 3u)];
                                                                            let _e2125 = global.member[(_e2095.member_2 + 4u)];
                                                                            let _e2130 = global.member[(_e2095.member_2 + 5u)];
                                                                            let _e2135 = global.member[(_e2095.member_2 + 6u)];
                                                                            let _e2141 = global.member[(_e2095.member_2 + 7u)];
                                                                            let _e2146 = global.member[(_e2095.member_2 + 8u)];
                                                                            let _e2151 = global.member[(_e2095.member_2 + 9u)];
                                                                            phi_2321_ = type_28(vec3<f32>(bitcast<f32>(_e2104), bitcast<f32>(_e2109), bitcast<f32>(_e2114)), vec4<f32>(bitcast<f32>(_e2120), bitcast<f32>(_e2125), bitcast<f32>(_e2130), bitcast<f32>(_e2135)), vec3<f32>(bitcast<f32>(_e2141), bitcast<f32>(_e2146), bitcast<f32>(_e2151)));
                                                                        } else {
                                                                            phi_2321_ = type_28(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                                                        }
                                                                        let _e2156 = phi_2321_;
                                                                        let _e2164 = (_e2156.member_1.x + _e2156.member_1.x);
                                                                        let _e2165 = (_e2156.member_1.y + _e2156.member_1.y);
                                                                        let _e2166 = (_e2156.member_1.z + _e2156.member_1.z);
                                                                        let _e2168 = (_e2156.member_1.z * _e2166);
                                                                        let _e2169 = (_e2156.member_1.w * _e2164);
                                                                        let _e2170 = (_e2156.member_1.w * _e2165);
                                                                        let _e2171 = (_e2156.member_1.w * _e2166);
                                                                        let _e2191 = (vec4<f32>((1f - fma(_e2156.member_1.y, _e2165, _e2168)), fma(_e2156.member_1.x, _e2165, _e2171), fma(_e2156.member_1.x, _e2166, -(_e2170)), 0f) * _e2156.member_2.x);
                                                                        let _e2193 = (vec4<f32>(fma(_e2156.member_1.x, _e2165, -(_e2171)), (1f - fma(_e2156.member_1.x, _e2164, _e2168)), fma(_e2156.member_1.y, _e2166, _e2169), 0f) * _e2156.member_2.y);
                                                                        let _e2195 = (vec4<f32>(fma(_e2156.member_1.x, _e2166, _e2170), fma(_e2156.member_1.y, _e2166, -(_e2169)), (1f - fma(_e2156.member_1.x, _e2164, (_e2156.member_1.y * _e2165))), 0f) * _e2156.member_2.z);
                                                                        switch bitcast<i32>(_e2095.member) {
                                                                            case 0: {
                                                                                if _e324 {
                                                                                    phi_5250_ = (_e2095.member_1 <= (_e118 - 8u));
                                                                                } else {
                                                                                    phi_5250_ = false;
                                                                                }
                                                                                let _e2693 = phi_5250_;
                                                                                if _e2693 {
                                                                                    let _e2696 = global.member[_e2095.member_1];
                                                                                    let _e2701 = global.member[(_e2095.member_1 + 1u)];
                                                                                    let _e2706 = global.member[(_e2095.member_1 + 2u)];
                                                                                    let _e2712 = global.member[(_e2095.member_1 + 3u)];
                                                                                    let _e2717 = global.member[(_e2095.member_1 + 4u)];
                                                                                    let _e2722 = global.member[(_e2095.member_1 + 5u)];
                                                                                    let _e2727 = global.member[(_e2095.member_1 + 6u)];
                                                                                    let _e2733 = global.member[(_e2095.member_1 + 7u)];
                                                                                    phi_2369_ = type_35(vec3<f32>(bitcast<f32>(_e2696), bitcast<f32>(_e2701), bitcast<f32>(_e2706)), vec4<f32>(bitcast<f32>(_e2712), bitcast<f32>(_e2717), bitcast<f32>(_e2722), bitcast<f32>(_e2727)), bitcast<f32>(_e2733));
                                                                                } else {
                                                                                    phi_2369_ = type_35(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                                                                }
                                                                                let _e2737 = phi_2369_;
                                                                                let _e2759 = fma(_e2195.x, _e2737.member.z, fma(_e2193.x, _e2737.member.y, (_e2191.x * _e2737.member.x)));
                                                                                let _e2760 = fma(_e2195.y, _e2737.member.z, fma(_e2193.y, _e2737.member.y, (_e2191.y * _e2737.member.x)));
                                                                                let _e2761 = fma(_e2195.z, _e2737.member.z, fma(_e2193.z, _e2737.member.y, (_e2191.z * _e2737.member.x)));
                                                                                let _e2766 = sqrt(fma(_e2761, _e2761, fma(_e2759, _e2759, (_e2760 * _e2760))));
                                                                                if (_e2766 == 0f) {
                                                                                    phi_5298_ = vec3<f32>(0f, 0f, 0f);
                                                                                } else {
                                                                                    phi_5298_ = (vec3<f32>(_e2759, _e2760, _e2761) * (1f / _e2766));
                                                                                }
                                                                                let _e2771 = phi_5298_;
                                                                                let _e2773 = -(_e2771.x);
                                                                                let _e2775 = -(_e2771.y);
                                                                                let _e2777 = -(_e2771.z);
                                                                                let _e2778 = -(_e2771);
                                                                                let _e2782 = fma(fma(_e1464, _e122.x, -0.4f), _e1487, 0.4f);
                                                                                let _e2783 = fma(fma(_e1467, _e122.y, -0.4f), _e1487, 0.4f);
                                                                                let _e2784 = fma(fma(_e1470, _e122.z, -0.4f), _e1487, 0.4f);
                                                                                let _e2792 = (_e2035 + vec3<f32>(_e2773, _e2775, _e2777));
                                                                                let _e2799 = sqrt(fma(_e2792.z, _e2792.z, fma(_e2792.x, _e2792.x, (_e2792.y * _e2792.y))));
                                                                                if (_e2799 == 0f) {
                                                                                    phi_5333_ = vec3<f32>(0f, 0f, 0f);
                                                                                } else {
                                                                                    phi_5333_ = (_e2792 * (1f / _e2799));
                                                                                }
                                                                                let _e2804 = phi_5333_;
                                                                                let _e2805 = (_e1484 * _e1484);
                                                                                let _e2816 = max(fma(_e2031.z, _e2804.z, fma(_e2031.x, _e2804.x, (_e2031.y * _e2804.y))), 0f);
                                                                                let _e2829 = max(fma(_e2031.z, _e2035.z, fma(_e2031.x, _e2035.x, (_e2031.y * _e2035.y))), 0f);
                                                                                let _e2836 = max(fma(_e2031.z, _e2778.z, fma(_e2031.x, _e2778.x, (_e2031.y * _e2778.y))), 0f);
                                                                                let _e2837 = fma(_e743.y, _e318.member_4, 1f);
                                                                                let _e2838 = (_e2837 * _e2837);
                                                                                let _e2839 = (_e2838 * 0.125f);
                                                                                let _e2841 = fma(-(_e2838), 0.125f, 1f);
                                                                                let _e2854 = (1f - max(fma(_e2804.z, _e2035.z, fma(_e2804.x, _e2035.x, (_e2804.y * _e2035.y))), 0f));
                                                                                let _e2856 = select(_e2854, 0f, (_e2854 < 0f));
                                                                                let _e2859 = pow(select(_e2856, 1f, (_e2856 > 1f)), 5f);
                                                                                let _e2860 = fma((1f - _e2782), _e2859, _e2782);
                                                                                let _e2861 = fma((1f - _e2783), _e2859, _e2783);
                                                                                let _e2862 = fma((1f - _e2784), _e2859, _e2784);
                                                                                let _e2867 = fma(-(_e743.z), _e318.member_3, 1f);
                                                                                let _e2871 = (((_e2805 * _e2805) / (pow(fma((_e2816 * _e2816), fma(_e2805, _e2805, -1f), 1f), 2f) * 3.1415927f)) * ((_e2829 / fma(_e2829, _e2841, _e2839)) * (_e2836 / fma(_e2836, _e2841, _e2839))));
                                                                                let _e2878 = max(fma(_e2031.z, _e2777, fma(_e2031.x, _e2773, (_e2031.y * _e2775))), 0f);
                                                                                let _e2880 = fma((4f * _e2829), _e2878, 0.0001f);
                                                                                phi_3077_ = false;
                                                                                phi_3078_ = true;
                                                                                phi_3079_ = vec3<f32>(fma((fma((((1f - _e2860) * _e2867) * _e1475), 0.31830987f, ((_e2871 * _e2860) / _e2880)) * (_e2737.member_1.x * _e2737.member_2)), _e2878, _e2040.x), fma((fma((((1f - _e2861) * _e2867) * _e1477), 0.31830987f, ((_e2871 * _e2861) / _e2880)) * (_e2737.member_1.y * _e2737.member_2)), _e2878, _e2040.y), fma((fma((((1f - _e2862) * _e2867) * _e1479), 0.31830987f, ((_e2871 * _e2862) / _e2880)) * (_e2737.member_1.z * _e2737.member_2)), _e2878, _e2040.z));
                                                                                phi_3080_ = false;
                                                                                break;
                                                                            }
                                                                            case 1: {
                                                                                if _e324 {
                                                                                    phi_5425_ = (_e2095.member_1 <= (_e118 - 8u));
                                                                                } else {
                                                                                    phi_5425_ = false;
                                                                                }
                                                                                let _e2481 = phi_5425_;
                                                                                if _e2481 {
                                                                                    let _e2484 = global.member[_e2095.member_1];
                                                                                    let _e2489 = global.member[(_e2095.member_1 + 1u)];
                                                                                    let _e2494 = global.member[(_e2095.member_1 + 2u)];
                                                                                    let _e2500 = global.member[(_e2095.member_1 + 3u)];
                                                                                    let _e2505 = global.member[(_e2095.member_1 + 4u)];
                                                                                    let _e2510 = global.member[(_e2095.member_1 + 5u)];
                                                                                    let _e2515 = global.member[(_e2095.member_1 + 6u)];
                                                                                    let _e2521 = global.member[(_e2095.member_1 + 7u)];
                                                                                    phi_2573_ = type_35(vec3<f32>(bitcast<f32>(_e2484), bitcast<f32>(_e2489), bitcast<f32>(_e2494)), vec4<f32>(bitcast<f32>(_e2500), bitcast<f32>(_e2505), bitcast<f32>(_e2510), bitcast<f32>(_e2515)), bitcast<f32>(_e2521));
                                                                                } else {
                                                                                    phi_2573_ = type_35(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                                                                }
                                                                                let _e2525 = phi_2573_;
                                                                                let _e2554 = (vec3<f32>((_e2156.member.x + fma(_e2195.x, _e2525.member.z, fma(_e2193.x, _e2525.member.y, (_e2191.x * _e2525.member.x)))), (_e2156.member.y + fma(_e2195.y, _e2525.member.z, fma(_e2193.y, _e2525.member.y, (_e2191.y * _e2525.member.x)))), (_e2156.member.z + fma(_e2195.z, _e2525.member.z, fma(_e2193.z, _e2525.member.y, (_e2191.z * _e2525.member.x))))) - _e128);
                                                                                let _e2561 = sqrt(fma(_e2554.z, _e2554.z, fma(_e2554.x, _e2554.x, (_e2554.y * _e2554.y))));
                                                                                let _e2562 = (_e2561 == 0f);
                                                                                if _e2562 {
                                                                                    phi_2762_ = vec3<f32>();
                                                                                } else {
                                                                                    if _e2562 {
                                                                                        phi_5471_ = vec3<f32>(0f, 0f, 0f);
                                                                                    } else {
                                                                                        phi_5471_ = (_e2554 * (1f / _e2561));
                                                                                    }
                                                                                    let _e2566 = phi_5471_;
                                                                                    let _e2568 = (_e2525.member_2 / (_e2561 * _e2561));
                                                                                    let _e2572 = fma(fma(_e1464, _e122.x, -0.4f), _e1487, 0.4f);
                                                                                    let _e2573 = fma(fma(_e1467, _e122.y, -0.4f), _e1487, 0.4f);
                                                                                    let _e2574 = fma(fma(_e1470, _e122.z, -0.4f), _e1487, 0.4f);
                                                                                    let _e2581 = (_e2035 + _e2566);
                                                                                    let _e2588 = sqrt(fma(_e2581.z, _e2581.z, fma(_e2581.x, _e2581.x, (_e2581.y * _e2581.y))));
                                                                                    if (_e2588 == 0f) {
                                                                                        phi_5506_ = vec3<f32>(0f, 0f, 0f);
                                                                                    } else {
                                                                                        phi_5506_ = (_e2581 * (1f / _e2588));
                                                                                    }
                                                                                    let _e2593 = phi_5506_;
                                                                                    let _e2594 = (_e1484 * _e1484);
                                                                                    let _e2605 = max(fma(_e2031.z, _e2593.z, fma(_e2031.x, _e2593.x, (_e2031.y * _e2593.y))), 0f);
                                                                                    let _e2618 = max(fma(_e2031.z, _e2035.z, fma(_e2031.x, _e2035.x, (_e2031.y * _e2035.y))), 0f);
                                                                                    let _e2625 = max(fma(_e2031.z, _e2566.z, fma(_e2031.x, _e2566.x, (_e2031.y * _e2566.y))), 0f);
                                                                                    let _e2626 = fma(_e743.y, _e318.member_4, 1f);
                                                                                    let _e2627 = (_e2626 * _e2626);
                                                                                    let _e2628 = (_e2627 * 0.125f);
                                                                                    let _e2630 = fma(-(_e2627), 0.125f, 1f);
                                                                                    let _e2643 = (1f - max(fma(_e2593.z, _e2035.z, fma(_e2593.x, _e2035.x, (_e2593.y * _e2035.y))), 0f));
                                                                                    let _e2645 = select(_e2643, 0f, (_e2643 < 0f));
                                                                                    let _e2648 = pow(select(_e2645, 1f, (_e2645 > 1f)), 5f);
                                                                                    let _e2649 = fma((1f - _e2572), _e2648, _e2572);
                                                                                    let _e2650 = fma((1f - _e2573), _e2648, _e2573);
                                                                                    let _e2651 = fma((1f - _e2574), _e2648, _e2574);
                                                                                    let _e2656 = fma(-(_e743.z), _e318.member_3, 1f);
                                                                                    let _e2660 = (((_e2594 * _e2594) / (pow(fma((_e2605 * _e2605), fma(_e2594, _e2594, -1f), 1f), 2f) * 3.1415927f)) * ((_e2618 / fma(_e2618, _e2630, _e2628)) * (_e2625 / fma(_e2625, _e2630, _e2628))));
                                                                                    let _e2665 = fma((4f * _e2618), _e2625, 0.0001f);
                                                                                    phi_2762_ = vec3<f32>(fma((fma((((1f - _e2649) * _e2656) * _e1475), 0.31830987f, ((_e2660 * _e2649) / _e2665)) * (_e2525.member_1.x * _e2568)), _e2625, _e2040.x), fma((fma((((1f - _e2650) * _e2656) * _e1477), 0.31830987f, ((_e2660 * _e2650) / _e2665)) * (_e2525.member_1.y * _e2568)), _e2625, _e2040.y), fma((fma((((1f - _e2651) * _e2656) * _e1479), 0.31830987f, ((_e2660 * _e2651) / _e2665)) * (_e2525.member_1.z * _e2568)), _e2625, _e2040.z));
                                                                                }
                                                                                let _e2686 = phi_2762_;
                                                                                phi_3077_ = false;
                                                                                phi_3078_ = select(true, false, _e2562);
                                                                                phi_3079_ = _e2686;
                                                                                phi_3080_ = select(false, true, _e2562);
                                                                                break;
                                                                            }
                                                                            case 2: {
                                                                                if (_e118 >= 13u) {
                                                                                    phi_5598_ = (_e2095.member_1 <= (_e118 - 13u));
                                                                                } else {
                                                                                    phi_5598_ = false;
                                                                                }
                                                                                let _e2206 = phi_5598_;
                                                                                if _e2206 {
                                                                                    let _e2209 = global.member[_e2095.member_1];
                                                                                    let _e2214 = global.member[(_e2095.member_1 + 1u)];
                                                                                    let _e2219 = global.member[(_e2095.member_1 + 2u)];
                                                                                    let _e2225 = global.member[(_e2095.member_1 + 3u)];
                                                                                    let _e2230 = global.member[(_e2095.member_1 + 4u)];
                                                                                    let _e2235 = global.member[(_e2095.member_1 + 5u)];
                                                                                    let _e2241 = global.member[(_e2095.member_1 + 6u)];
                                                                                    let _e2246 = global.member[(_e2095.member_1 + 7u)];
                                                                                    let _e2251 = global.member[(_e2095.member_1 + 8u)];
                                                                                    let _e2256 = global.member[(_e2095.member_1 + 9u)];
                                                                                    let _e2261 = global.member[(_e2095.member_1 + 10u)];
                                                                                    let _e2266 = global.member[(_e2095.member_1 + 11u)];
                                                                                    let _e2272 = global.member[(_e2095.member_1 + 12u)];
                                                                                    phi_2824_ = type_36(vec3<f32>(bitcast<f32>(_e2209), bitcast<f32>(_e2214), bitcast<f32>(_e2219)), vec3<f32>(bitcast<f32>(_e2225), bitcast<f32>(_e2230), bitcast<f32>(_e2235)), bitcast<f32>(_e2241), bitcast<f32>(_e2246), vec4<f32>(bitcast<f32>(_e2251), bitcast<f32>(_e2256), bitcast<f32>(_e2261), bitcast<f32>(_e2266)), bitcast<f32>(_e2272));
                                                                                } else {
                                                                                    phi_2824_ = type_36(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                                                                }
                                                                                let _e2276 = phi_2824_;
                                                                                let _e2308 = (vec3<f32>((_e2156.member.x + fma(_e2195.x, _e2276.member.z, fma(_e2193.x, _e2276.member.y, (_e2191.x * _e2276.member.x)))), (_e2156.member.y + fma(_e2195.y, _e2276.member.z, fma(_e2193.y, _e2276.member.y, (_e2191.y * _e2276.member.x)))), (_e2156.member.z + fma(_e2195.z, _e2276.member.z, fma(_e2193.z, _e2276.member.y, (_e2191.z * _e2276.member.x))))) - _e128);
                                                                                let _e2315 = sqrt(fma(_e2308.z, _e2308.z, fma(_e2308.x, _e2308.x, (_e2308.y * _e2308.y))));
                                                                                let _e2316 = (_e2315 == 0f);
                                                                                if _e2316 {
                                                                                    phi_3076_ = vec3<f32>();
                                                                                } else {
                                                                                    if _e2316 {
                                                                                        phi_5648_ = vec3<f32>(0f, 0f, 0f);
                                                                                    } else {
                                                                                        phi_5648_ = (_e2308 * (1f / _e2315));
                                                                                    }
                                                                                    let _e2320 = phi_5648_;
                                                                                    let _e2330 = fma(_e2195.x, _e2276.member_1.z, fma(_e2193.x, _e2276.member_1.y, (_e2191.x * _e2276.member_1.x)));
                                                                                    let _e2331 = fma(_e2195.y, _e2276.member_1.z, fma(_e2193.y, _e2276.member_1.y, (_e2191.y * _e2276.member_1.x)));
                                                                                    let _e2332 = fma(_e2195.z, _e2276.member_1.z, fma(_e2193.z, _e2276.member_1.y, (_e2191.z * _e2276.member_1.x)));
                                                                                    let _e2337 = sqrt(fma(_e2332, _e2332, fma(_e2330, _e2330, (_e2331 * _e2331))));
                                                                                    if (_e2337 == 0f) {
                                                                                        phi_5683_ = vec3<f32>(0f, 0f, 0f);
                                                                                    } else {
                                                                                        phi_5683_ = (vec3<f32>(_e2330, _e2331, _e2332) * (1f / _e2337));
                                                                                    }
                                                                                    let _e2342 = phi_5683_;
                                                                                    let _e2354 = ((fma(_e2320.z, _e2342.z, fma(_e2320.x, _e2342.x, (_e2320.y * _e2342.y))) - _e2276.member_3) / (_e2276.member_2 - _e2276.member_3));
                                                                                    let _e2356 = select(_e2354, 0f, (_e2354 < 0f));
                                                                                    let _e2359 = (_e2276.member_5 * select(_e2356, 1f, (_e2356 > 1f)));
                                                                                    let _e2363 = fma(fma(_e1464, _e122.x, -0.4f), _e1487, 0.4f);
                                                                                    let _e2364 = fma(fma(_e1467, _e122.y, -0.4f), _e1487, 0.4f);
                                                                                    let _e2365 = fma(fma(_e1470, _e122.z, -0.4f), _e1487, 0.4f);
                                                                                    let _e2372 = (_e2035 + _e2320);
                                                                                    let _e2379 = sqrt(fma(_e2372.z, _e2372.z, fma(_e2372.x, _e2372.x, (_e2372.y * _e2372.y))));
                                                                                    if (_e2379 == 0f) {
                                                                                        phi_5718_ = vec3<f32>(0f, 0f, 0f);
                                                                                    } else {
                                                                                        phi_5718_ = (_e2372 * (1f / _e2379));
                                                                                    }
                                                                                    let _e2384 = phi_5718_;
                                                                                    let _e2385 = (_e1484 * _e1484);
                                                                                    let _e2396 = max(fma(_e2031.z, _e2384.z, fma(_e2031.x, _e2384.x, (_e2031.y * _e2384.y))), 0f);
                                                                                    let _e2409 = max(fma(_e2031.z, _e2035.z, fma(_e2031.x, _e2035.x, (_e2031.y * _e2035.y))), 0f);
                                                                                    let _e2413 = max(fma(_e2031.z, _e2320.z, fma(_e2031.x, _e2320.x, (_e2031.y * _e2320.y))), 0f);
                                                                                    let _e2414 = fma(_e743.y, _e318.member_4, 1f);
                                                                                    let _e2415 = (_e2414 * _e2414);
                                                                                    let _e2416 = (_e2415 * 0.125f);
                                                                                    let _e2418 = fma(-(_e2415), 0.125f, 1f);
                                                                                    let _e2431 = (1f - max(fma(_e2384.z, _e2035.z, fma(_e2384.x, _e2035.x, (_e2384.y * _e2035.y))), 0f));
                                                                                    let _e2433 = select(_e2431, 0f, (_e2431 < 0f));
                                                                                    let _e2436 = pow(select(_e2433, 1f, (_e2433 > 1f)), 5f);
                                                                                    let _e2437 = fma((1f - _e2363), _e2436, _e2363);
                                                                                    let _e2438 = fma((1f - _e2364), _e2436, _e2364);
                                                                                    let _e2439 = fma((1f - _e2365), _e2436, _e2365);
                                                                                    let _e2444 = fma(-(_e743.z), _e318.member_3, 1f);
                                                                                    let _e2448 = (((_e2385 * _e2385) / (pow(fma((_e2396 * _e2396), fma(_e2385, _e2385, -1f), 1f), 2f) * 3.1415927f)) * ((_e2409 / fma(_e2409, _e2418, _e2416)) * (_e2413 / fma(_e2413, _e2418, _e2416))));
                                                                                    let _e2453 = fma((4f * _e2409), _e2413, 0.0001f);
                                                                                    phi_3076_ = vec3<f32>(fma((fma((((1f - _e2437) * _e2444) * _e1475), 0.31830987f, ((_e2448 * _e2437) / _e2453)) * (_e2276.member_4.x * _e2359)), _e2413, _e2040.x), fma((fma((((1f - _e2438) * _e2444) * _e1477), 0.31830987f, ((_e2448 * _e2438) / _e2453)) * (_e2276.member_4.y * _e2359)), _e2413, _e2040.y), fma((fma((((1f - _e2439) * _e2444) * _e1479), 0.31830987f, ((_e2448 * _e2439) / _e2453)) * (_e2276.member_4.z * _e2359)), _e2413, _e2040.z));
                                                                                }
                                                                                let _e2474 = phi_3076_;
                                                                                phi_3077_ = false;
                                                                                phi_3078_ = select(true, false, _e2316);
                                                                                phi_3079_ = _e2474;
                                                                                phi_3080_ = select(false, true, _e2316);
                                                                                break;
                                                                            }
                                                                            default: {
                                                                                phi_3077_ = true;
                                                                                phi_3078_ = false;
                                                                                phi_3079_ = vec3<f32>();
                                                                                phi_3080_ = false;
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e2901 = phi_3077_;
                                                                        let _e2903 = phi_3078_;
                                                                        let _e2905 = phi_3079_;
                                                                        let _e2907 = phi_3080_;
                                                                        if _e2903 {
                                                                            phi_3085_ = _e2053;
                                                                        } else {
                                                                            phi_3085_ = type_24();
                                                                        }
                                                                        let _e2909 = phi_3085_;
                                                                        let _e2914 = select(_e2907, false, _e2903);
                                                                        if _e2914 {
                                                                            phi_3093_ = _e2053;
                                                                        } else {
                                                                            phi_3093_ = _e2909;
                                                                        }
                                                                        let _e2916 = phi_3093_;
                                                                        phi_3097_ = select(select(false, true, _e2903), true, _e2914);
                                                                        phi_3098_ = _e2916;
                                                                        phi_3099_ = select(select(vec3<f32>(), _e2905, vec3(_e2903)), _e2040, vec3(_e2914));
                                                                        phi_3100_ = select(select(_e2901, false, _e2903), false, _e2914);
                                                                    }
                                                                    let _e2922 = phi_3097_;
                                                                    let _e2924 = phi_3098_;
                                                                    let _e2926 = phi_3099_;
                                                                    let _e2928 = phi_3100_;
                                                                    phi_3101_ = _e2928;
                                                                    phi_3102_ = select(false, true, _e2073);
                                                                    phi_3103_ = _e2922;
                                                                    phi_3104_ = _e2924;
                                                                    phi_3105_ = _e2926;
                                                                    break;
                                                                }
                                                                default: {
                                                                    phi_3101_ = true;
                                                                    phi_3102_ = false;
                                                                    phi_3103_ = false;
                                                                    phi_3104_ = type_24();
                                                                    phi_3105_ = vec3<f32>();
                                                                    break;
                                                                }
                                                            }
                                                            let _e2931 = phi_3101_;
                                                            let _e2933 = phi_3102_;
                                                            let _e2935 = phi_3103_;
                                                            let _e2937 = phi_3104_;
                                                            let _e2939 = phi_3105_;
                                                            if _e2931 {
                                                                phi_3111_ = type_24();
                                                            } else {
                                                                phi_3111_ = _e2937;
                                                            }
                                                            let _e2941 = phi_3111_;
                                                            let _e2942 = select(_e2933, false, _e2931);
                                                            if _e2942 {
                                                                let _e2949 = fma(fma(_e1464, _e122.x, -0.04f), _e1487, 0.04f);
                                                                let _e2950 = fma(fma(_e1467, _e122.y, -0.04f), _e1487, 0.04f);
                                                                let _e2951 = fma(fma(_e1470, _e122.z, -0.04f), _e1487, 0.04f);
                                                                let _e2963 = fma(-(_e743.y), _e318.member_4, 1f);
                                                                let _e2970 = (1f - max(fma(_e2031.z, _e2035.z, fma(_e2031.x, _e2035.x, (_e2031.y * _e2035.y))), 0f));
                                                                let _e2972 = select(_e2970, 0f, (_e2970 < 0f));
                                                                let _e2975 = pow(select(_e2972, 1f, (_e2972 > 1f)), 5f);
                                                                let _e2976 = fma((max(_e2963, _e2949) - _e2949), _e2975, _e2949);
                                                                let _e2977 = fma((max(_e2963, _e2950) - _e2950), _e2975, _e2950);
                                                                let _e2978 = fma((max(_e2963, _e2951) - _e2951), _e2975, _e2951);
                                                                let _e2983 = fma(-(_e743.z), _e318.member_3, 1f);
                                                                phi_3215_ = vec4<f32>(fma(_e1497, _e318.member_1, fma(fma(((1f - _e2976) * _e2983), (_e1506.x * _e1475), (_e1846.x * fma(_e2976, _e1862.x, _e1862.y))), _e1491, _e2040.x)), fma(_e1499, _e318.member_1, fma(fma(((1f - _e2977) * _e2983), (_e1506.y * _e1477), (_e1846.y * fma(_e2977, _e1862.x, _e1862.y))), _e1491, _e2040.y)), fma(_e1501, _e318.member_1, fma(fma(((1f - _e2978) * _e2983), (_e1506.z * _e1479), (_e1846.z * fma(_e2978, _e1862.x, _e1862.y))), _e1491, _e2040.z)), 1f);
                                                                phi_2209_ = type_24();
                                                            } else {
                                                                phi_3215_ = vec4<f32>();
                                                                phi_2209_ = _e2941;
                                                            }
                                                            let _e3010 = phi_3215_;
                                                            let _e3012 = phi_2209_;
                                                            local_4 = select(false, true, _e2942);
                                                            local_5 = _e3010;
                                                            continue;
                                                            continuing {
                                                                phi_2208_ = _e3012;
                                                                phi_2211_ = select(select(_e2939, vec3<f32>(), vec3(_e2931)), vec3<f32>(), vec3(_e2942));
                                                                break if !(select(select(_e2935, false, _e2931), false, _e2942));
                                                            }
                                                        }
                                                        let _e3242 = local_4;
                                                        phi_3232_ = _e3242;
                                                        let _e3245 = local_5;
                                                        phi_3233_ = _e3245;
                                                    } else {
                                                        phi_3232_ = true;
                                                        phi_3233_ = (vec4<f32>((_e122.x * _e533.x), (_e122.y * _e533.y), (_e122.z * _e533.z), (_e122.w * _e533.w)) * _e318.member_2);
                                                    }
                                                    let _e3019 = phi_3232_;
                                                    let _e3021 = phi_3233_;
                                                    if _e3019 {
                                                        global_19 = _e3021;
                                                    }
                                                    break;
                                                }
                                                case 1: {
                                                    let _e2004 = sqrt(fma(_e123.x, _e123.x, (_e123.y * _e123.y)));
                                                    if (_e2004 == 0f) {
                                                        phi_5837_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_5837_ = (vec3<f32>(_e123.x, _e123.y, 0f) * (1f / _e2004));
                                                    }
                                                    let _e2009 = phi_5837_;
                                                    global_19 = vec4<f32>(((_e2009.x + 1f) * 0.5f), ((_e2009.y + 1f) * 0.5f), ((_e2009.z + 1f) * 0.5f), 1f);
                                                    break;
                                                }
                                                case 2: {
                                                    let _e1983 = sqrt(fma(_e124.x, _e124.x, (_e124.y * _e124.y)));
                                                    if (_e1983 == 0f) {
                                                        phi_5886_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_5886_ = (vec3<f32>(_e124.x, _e124.y, 0f) * (1f / _e1983));
                                                    }
                                                    let _e1988 = phi_5886_;
                                                    global_19 = vec4<f32>(((_e1988.x + 1f) * 0.5f), ((_e1988.y + 1f) * 0.5f), ((_e1988.z + 1f) * 0.5f), 1f);
                                                    break;
                                                }
                                                case 3: {
                                                    if _e1825 {
                                                        phi_5935_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_5935_ = (_e1460 * (1f / _e1824));
                                                    }
                                                    let _e1967 = phi_5935_;
                                                    global_19 = vec4<f32>(((_e1967.x + 1f) * 0.5f), ((_e1967.y + 1f) * 0.5f), ((_e1967.z + 1f) * 0.5f), 1f);
                                                    break;
                                                }
                                                case 4: {
                                                    global_19 = _e122;
                                                    break;
                                                }
                                                case 5: {
                                                    let _e1948 = sqrt(fma(_e125.z, _e125.z, fma(_e125.x, _e125.x, (_e125.y * _e125.y))));
                                                    if (_e1948 == 0f) {
                                                        phi_5984_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_5984_ = (_e125 * (1f / _e1948));
                                                    }
                                                    let _e1953 = phi_5984_;
                                                    global_19 = vec4<f32>(((_e1953.x + 1f) * 0.5f), ((_e1953.y + 1f) * 0.5f), ((_e1953.z + 1f) * 0.5f), 1f);
                                                    break;
                                                }
                                                case 6: {
                                                    let _e1926 = sqrt(fma(_e1458.z, _e1458.z, fma(_e1458.x, _e1458.x, (_e1458.y * _e1458.y))));
                                                    if (_e1926 == 0f) {
                                                        phi_6033_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_6033_ = (_e1458 * (1f / _e1926));
                                                    }
                                                    let _e1931 = phi_6033_;
                                                    global_19 = vec4<f32>(((_e1931.x + 1f) * 0.5f), ((_e1931.y + 1f) * 0.5f), ((_e1931.z + 1f) * 0.5f), 1f);
                                                    break;
                                                }
                                                case 7: {
                                                    let _e1904 = sqrt(fma(_e126.z, _e126.z, fma(_e126.x, _e126.x, (_e126.y * _e126.y))));
                                                    if (_e1904 == 0f) {
                                                        phi_6082_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_6082_ = (_e126 * (1f / _e1904));
                                                    }
                                                    let _e1909 = phi_6082_;
                                                    global_19 = vec4<f32>(((_e1909.x + 1f) * 0.5f), ((_e1909.y + 1f) * 0.5f), ((_e1909.z + 1f) * 0.5f), 1f);
                                                    break;
                                                }
                                                case 8: {
                                                    let _e1882 = sqrt(fma(_e127.z, _e127.z, fma(_e127.x, _e127.x, (_e127.y * _e127.y))));
                                                    if (_e1882 == 0f) {
                                                        phi_6131_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_6131_ = (_e127 * (1f / _e1882));
                                                    }
                                                    let _e1887 = phi_6131_;
                                                    global_19 = vec4<f32>(((_e1887.x + 1f) * 0.5f), ((_e1887.y + 1f) * 0.5f), ((_e1887.z + 1f) * 0.5f), 1f);
                                                    break;
                                                }
                                                case 9: {
                                                    global_19 = vec4<f32>(_e1506.x, _e1506.y, _e1506.z, 1f);
                                                    break;
                                                }
                                                case 10: {
                                                    global_19 = vec4<f32>(_e1846.x, _e1846.y, _e1846.z, 1f);
                                                    break;
                                                }
                                                case 11: {
                                                    global_19 = vec4<f32>(_e1862.x, _e1862.y, 1f, 1f);
                                                    break;
                                                }
                                                case 12: {
                                                    global_19 = (vec4<f32>(_e1464, _e1467, _e1470, (_e533.w * _e318.member_2.w)) * _e122);
                                                    break;
                                                }
                                                case 13: {
                                                    global_19 = vec4<f32>(_e1484, _e1484, _e1484, 1f);
                                                    break;
                                                }
                                                case 14: {
                                                    global_19 = vec4<f32>(_e1487, _e1487, _e1487, 1f);
                                                    break;
                                                }
                                                case 15: {
                                                    global_19 = vec4<f32>(_e1491, _e1491, _e1491, 1f);
                                                    break;
                                                }
                                                case 16: {
                                                    global_19 = vec4<f32>((_e1497 * _e318.member_1), (_e1499 * _e318.member_1), (_e1501 * _e318.member_1), 1f);
                                                    break;
                                                }
                                                case 17: {
                                                    global_19 = vec4<f32>(_e1373.x, _e1373.y, _e1373.z, 1f);
                                                    break;
                                                }
                                                case 18: {
                                                    global_19 = vec4<f32>(_e318.member.x, _e318.member.y, _e318.member.z, 1f);
                                                    break;
                                                }
                                                case 19: {
                                                    global_19 = vec4<f32>(_e318.member_1, _e318.member_1, _e318.member_1, 1f);
                                                    break;
                                                }
                                                default: {
                                                    break;
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
fn stagerenderlet_fragment(@location(0) @interpolate(flat) param: u32, @location(1) @interpolate(flat) param_1: u32, @location(2) @interpolate(flat) param_2: u32, @location(3) param_3: vec4<f32>, @location(4) param_4: vec2<f32>, @location(5) param_5: vec2<f32>, @location(6) param_6: vec3<f32>, @location(7) param_7: vec3<f32>, @location(8) param_8: vec3<f32>, @location(9) param_9: vec3<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    global_2 = param_1;
    global_3 = param_2;
    global_4 = param_3;
    global_5 = param_4;
    global_6 = param_5;
    global_7 = param_6;
    global_8 = param_7;
    global_9 = param_8;
    global_10 = param_9;
    function();
    let _e21 = global_19;
    return _e21;
}
