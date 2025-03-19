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

struct type_35 {
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

struct type_36 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: f32,
}

struct type_37 {
    member: i32,
    member_1: i32,
    member_2: bool,
}

struct type_38 {
    member: u32,
    member_1: i32,
}

struct type_39 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
    member_3: f32,
    member_4: vec4<f32>,
    member_5: f32,
}

struct type_40 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
    member_4: f32,
    member_5: f32,
    member_6: f32,
    member_7: f32,
    member_8: f32,
    member_9: f32,
    member_10: f32,
    member_11: bool,
    member_12: bool,
}

@group(0) @binding(0) 
var<storage> global: type_3;
@group(0) @binding(1) 
var<storage> global_1: type_3;
@group(0) @binding(10) 
var<storage> global_2: type_3;
var<private> global_3: u32;
var<private> global_4: vec4<f32>;
var<private> global_5: vec2<f32>;
var<private> global_6: vec2<f32>;
var<private> global_7: vec3<f32>;
var<private> global_8: vec3<f32>;
var<private> global_9: vec3<f32>;
var<private> global_10: vec3<f32>;
@group(0) @binding(3) 
var global_11: sampler;
@group(0) @binding(2) 
var global_12: texture_2d_array<f32>;
@group(0) @binding(4) 
var global_13: texture_cube<f32>;
@group(0) @binding(5) 
var global_14: sampler;
@group(0) @binding(6) 
var global_15: texture_cube<f32>;
@group(0) @binding(7) 
var global_16: sampler;
@group(0) @binding(8) 
var global_17: texture_2d<f32>;
@group(0) @binding(9) 
var global_18: sampler;
@group(0) @binding(12) 
var global_19: sampler;
@group(0) @binding(11) 
var global_20: texture_2d_array<f32>;
var<private> global_21: vec4<f32>;

fn function() {
    var local: array<vec3<f32>, 21>;
    var local_1: array<vec3<f32>, 8>;
    var local_2: array<vec4<f32>, 6>;
    var phi_722_: u32;
    var phi_846_: type_35;
    var phi_5791_: bool;
    var phi_886_: u32;
    var phi_895_: u32;
    var phi_908_: type_15;
    var phi_5813_: f32;
    var phi_5826_: bool;
    var phi_962_: f32;
    var phi_957_: f32;
    var phi_963_: f32;
    var phi_5843_: bool;
    var phi_928_: f32;
    var phi_965_: f32;
    var phi_5861_: f32;
    var phi_5874_: bool;
    var phi_1020_: f32;
    var phi_1015_: f32;
    var phi_1021_: f32;
    var phi_5891_: bool;
    var phi_986_: f32;
    var phi_1023_: f32;
    var phi_5927_: bool;
    var phi_1106_: u32;
    var phi_1115_: u32;
    var phi_1128_: type_15;
    var phi_5948_: f32;
    var phi_5961_: bool;
    var phi_1182_: f32;
    var phi_1177_: f32;
    var phi_1183_: f32;
    var phi_5978_: bool;
    var phi_1148_: f32;
    var phi_1185_: f32;
    var phi_5996_: f32;
    var phi_6009_: bool;
    var phi_1240_: f32;
    var phi_1235_: f32;
    var phi_1241_: f32;
    var phi_6026_: bool;
    var phi_1206_: f32;
    var phi_1243_: f32;
    var phi_6062_: bool;
    var phi_1326_: u32;
    var phi_1335_: u32;
    var phi_1348_: type_15;
    var phi_6083_: f32;
    var phi_6096_: bool;
    var phi_1402_: f32;
    var phi_1397_: f32;
    var phi_1403_: f32;
    var phi_6113_: bool;
    var phi_1368_: f32;
    var phi_1405_: f32;
    var phi_6131_: f32;
    var phi_6144_: bool;
    var phi_1460_: f32;
    var phi_1455_: f32;
    var phi_1461_: f32;
    var phi_6161_: bool;
    var phi_1426_: f32;
    var phi_1463_: f32;
    var phi_6197_: bool;
    var phi_1546_: u32;
    var phi_1555_: u32;
    var phi_1568_: type_15;
    var phi_6218_: f32;
    var phi_6231_: bool;
    var phi_1622_: f32;
    var phi_1617_: f32;
    var phi_1623_: f32;
    var phi_6248_: bool;
    var phi_1588_: f32;
    var phi_1625_: f32;
    var phi_6266_: f32;
    var phi_6279_: bool;
    var phi_1680_: f32;
    var phi_1675_: f32;
    var phi_1681_: f32;
    var phi_6296_: bool;
    var phi_1646_: f32;
    var phi_1683_: f32;
    var phi_6332_: bool;
    var phi_1766_: u32;
    var phi_1775_: u32;
    var phi_1788_: type_15;
    var phi_6353_: f32;
    var phi_6366_: bool;
    var phi_1842_: f32;
    var phi_1837_: f32;
    var phi_1843_: f32;
    var phi_6383_: bool;
    var phi_1808_: f32;
    var phi_1845_: f32;
    var phi_6401_: f32;
    var phi_6414_: bool;
    var phi_1900_: f32;
    var phi_1895_: f32;
    var phi_1901_: f32;
    var phi_6431_: bool;
    var phi_1866_: f32;
    var phi_1903_: f32;
    var phi_6489_: vec3<f32>;
    var phi_6524_: vec3<f32>;
    var phi_6559_: vec3<f32>;
    var phi_6594_: vec3<f32>;
    var phi_6629_: vec3<f32>;
    var phi_1997_: vec3<f32>;
    var phi_1998_: vec3<f32>;
    var phi_6661_: bool;
    var phi_2204_: type_14;
    var phi_2205_: type_14;
    var phi_2228_: type_14;
    var phi_2255_: bool;
    var phi_2261_: type_14;
    var phi_2262_: type_14;
    var phi_2285_: type_14;
    var phi_2308_: bool;
    var phi_2329_: type_25;
    var phi_6733_: vec3<f32>;
    var phi_6792_: vec3<f32>;
    var phi_6866_: vec3<f32>;
    var phi_6926_: vec3<f32>;
    var phi_6975_: vec3<f32>;
    var phi_7024_: vec3<f32>;
    var phi_7073_: vec3<f32>;
    var phi_7122_: vec3<f32>;
    var phi_7171_: vec3<f32>;
    var phi_7220_: vec3<f32>;
    var phi_7259_: vec3<f32>;
    var phi_7294_: vec3<f32>;
    var phi_8831_: bool;
    var phi_2396_: type_14;
    var phi_2399_: vec3<f32>;
    var phi_2397_: type_14;
    var phi_2422_: type_14;
    var phi_7320_: u32;
    var phi_7339_: bool;
    var phi_2439_: u32;
    var phi_7363_: bool;
    var phi_2451_: u32;
    var phi_2465_: type_30;
    var phi_7395_: bool;
    var phi_2515_: type_31;
    var phi_7475_: bool;
    var phi_4095_: type_39;
    var phi_7525_: vec3<f32>;
    var phi_7560_: vec3<f32>;
    var phi_4231_: type_40;
    var phi_7595_: vec3<f32>;
    var phi_7700_: bool;
    var phi_7703_: bool;
    var phi_7704_: bool;
    var phi_4603_: u32;
    var phi_4612_: u32;
    var phi_8804_: bool;
    var phi_4640_: type_37;
    var phi_4643_: f32;
    var phi_4645_: f32;
    var phi_4657_: bool;
    var phi_4685_: type_38;
    var phi_4697_: type_37;
    var phi_4641_: type_37;
    var phi_4700_: type_38;
    var phi_4711_: type_37;
    var phi_4714_: f32;
    var phi_4716_: f32;
    var phi_4728_: bool;
    var phi_4756_: type_38;
    var phi_4768_: type_37;
    var phi_4712_: type_37;
    var phi_4771_: type_38;
    var phi_7726_: f32;
    var phi_7739_: bool;
    var phi_4837_: f32;
    var phi_4832_: f32;
    var phi_4838_: f32;
    var phi_7756_: bool;
    var phi_4803_: f32;
    var phi_4840_: f32;
    var phi_7774_: f32;
    var phi_7787_: bool;
    var phi_4893_: f32;
    var phi_4888_: f32;
    var phi_4894_: f32;
    var phi_7804_: bool;
    var phi_4859_: f32;
    var phi_4896_: f32;
    var phi_4955_: f32;
    var phi_4715_: f32;
    var phi_4717_: f32;
    var phi_4957_: bool;
    var phi_8788_: bool;
    var phi_8885_: bool;
    var phi_4644_: f32;
    var phi_4646_: f32;
    var phi_4958_: bool;
    var phi_8884_: bool;
    var local_3: f32;
    var local_4: f32;
    var phi_9012_: bool;
    var phi_4961_: f32;
    var phi_9011_: bool;
    var phi_4962_: f32;
    var phi_9010_: bool;
    var phi_4963_: f32;
    var phi_4964_: vec3<f32>;
    var phi_7841_: bool;
    var phi_3364_: type_36;
    var phi_7888_: vec3<f32>;
    var phi_7923_: vec3<f32>;
    var phi_3643_: type_14;
    var phi_3646_: f32;
    var phi_3644_: type_14;
    var phi_3669_: type_14;
    var phi_8044_: vec3<f32>;
    var phi_8078_: vec2<f32>;
    var phi_8081_: vec2<f32>;
    var phi_8082_: u32;
    var phi_8085_: vec2<f32>;
    var phi_8086_: u32;
    var phi_8087_: bool;
    var phi_8112_: vec2<f32>;
    var phi_8115_: vec2<f32>;
    var phi_8116_: u32;
    var phi_8119_: vec2<f32>;
    var phi_8120_: u32;
    var phi_8121_: bool;
    var phi_8142_: vec2<f32>;
    var phi_8145_: vec2<f32>;
    var phi_8146_: u32;
    var phi_8148_: vec2<f32>;
    var phi_8149_: u32;
    var phi_3706_: u32;
    var phi_3807_: u32;
    var phi_3838_: u32;
    var phi_3847_: u32;
    var phi_8179_: f32;
    var phi_8192_: bool;
    var phi_3907_: f32;
    var phi_3902_: f32;
    var phi_3908_: f32;
    var phi_8209_: bool;
    var phi_3873_: f32;
    var phi_3910_: f32;
    var phi_8227_: f32;
    var phi_8240_: bool;
    var phi_3965_: f32;
    var phi_3960_: f32;
    var phi_3966_: f32;
    var phi_8257_: bool;
    var phi_3931_: f32;
    var phi_3968_: f32;
    var phi_4027_: f32;
    var phi_3647_: f32;
    var phi_4028_: bool;
    var phi_8886_: bool;
    var local_5: f32;
    var phi_9014_: bool;
    var phi_4030_: f32;
    var phi_9013_: bool;
    var phi_4031_: f32;
    var phi_4032_: vec3<f32>;
    var phi_8294_: bool;
    var phi_2563_: type_36;
    var phi_8341_: vec3<f32>;
    var phi_8376_: vec3<f32>;
    var phi_8481_: bool;
    var phi_8484_: bool;
    var phi_8485_: bool;
    var phi_2964_: u32;
    var phi_2973_: u32;
    var phi_8958_: bool;
    var phi_3001_: type_37;
    var phi_3004_: f32;
    var phi_3006_: f32;
    var phi_3018_: bool;
    var phi_3046_: type_38;
    var phi_3058_: type_37;
    var phi_3002_: type_37;
    var phi_3061_: type_38;
    var phi_3072_: type_37;
    var phi_3075_: f32;
    var phi_3077_: f32;
    var phi_3089_: bool;
    var phi_3117_: type_38;
    var phi_3129_: type_37;
    var phi_3073_: type_37;
    var phi_3132_: type_38;
    var phi_8507_: f32;
    var phi_8520_: bool;
    var phi_3198_: f32;
    var phi_3193_: f32;
    var phi_3199_: f32;
    var phi_8537_: bool;
    var phi_3164_: f32;
    var phi_3201_: f32;
    var phi_8555_: f32;
    var phi_8568_: bool;
    var phi_3254_: f32;
    var phi_3249_: f32;
    var phi_3255_: f32;
    var phi_8585_: bool;
    var phi_3220_: f32;
    var phi_3257_: f32;
    var phi_3316_: f32;
    var phi_3076_: f32;
    var phi_3078_: f32;
    var phi_3318_: bool;
    var phi_8942_: bool;
    var phi_9004_: bool;
    var phi_3005_: f32;
    var phi_3007_: f32;
    var phi_3319_: bool;
    var phi_9003_: bool;
    var local_6: f32;
    var local_7: f32;
    var phi_9016_: bool;
    var phi_3322_: f32;
    var phi_9015_: bool;
    var phi_3323_: f32;
    var phi_9009_: bool;
    var phi_4966_: f32;
    var phi_4967_: vec3<f32>;
    var phi_4968_: bool;
    var phi_4988_: vec3<f32>;
    var phi_9006_: bool;
    var phi_2400_: vec3<f32>;
    var phi_4994_: bool;
    var phi_9005_: bool;
    var local_8: vec3<f32>;
    var local_9: vec3<f32>;
    var local_10: vec3<f32>;
    var phi_9025_: bool;
    var phi_5111_: vec4<f32>;
    var phi_9017_: bool;
    var local_11: f32;
    var local_12: f32;
    var local_13: f32;
    var local_14: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e146 = arrayLength((&global.member));
            let _e148 = arrayLength((&global_1.member));
            let _e150 = arrayLength((&global_2.member));
            let _e151 = global_3;
            let _e152 = global_4;
            let _e153 = global_5;
            let _e154 = global_6;
            let _e155 = global_7;
            let _e156 = global_8;
            let _e157 = global_9;
            let _e158 = global_10;
            let _e162 = global.member[(_e151 + 10u)];
            let _e166 = global.member[(_e151 + 16u)];
            let _e169 = global.member[_e166];
            let _e173 = global.member[(_e166 + 1u)];
            let _e177 = global.member[(_e166 + 2u)];
            let _e181 = global.member[(_e166 + 5u)];
            switch bitcast<i32>(_e181) {
                case 0: {
                    phi_722_ = 0u;
                    break;
                }
                case 1: {
                    phi_722_ = 1u;
                    break;
                }
                case 2: {
                    phi_722_ = 2u;
                    break;
                }
                case 3: {
                    phi_722_ = 3u;
                    break;
                }
                case 4: {
                    phi_722_ = 4u;
                    break;
                }
                case 5: {
                    phi_722_ = 5u;
                    break;
                }
                case 6: {
                    phi_722_ = 6u;
                    break;
                }
                case 7: {
                    phi_722_ = 7u;
                    break;
                }
                case 8: {
                    phi_722_ = 8u;
                    break;
                }
                case 9: {
                    phi_722_ = 9u;
                    break;
                }
                case 10: {
                    phi_722_ = 10u;
                    break;
                }
                case 11: {
                    phi_722_ = 11u;
                    break;
                }
                case 12: {
                    phi_722_ = 12u;
                    break;
                }
                case 13: {
                    phi_722_ = 13u;
                    break;
                }
                case 14: {
                    phi_722_ = 14u;
                    break;
                }
                case 15: {
                    phi_722_ = 15u;
                    break;
                }
                case 16: {
                    phi_722_ = 16u;
                    break;
                }
                case 17: {
                    phi_722_ = 17u;
                    break;
                }
                case 18: {
                    phi_722_ = 18u;
                    break;
                }
                case 19: {
                    phi_722_ = 19u;
                    break;
                }
                default: {
                    phi_722_ = 0u;
                    break;
                }
            }
            let _e184 = phi_722_;
            let _e188 = global.member[(_e166 + 6u)];
            if (_e162 == 4294967295u) {
                phi_846_ = type_35(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
            } else {
                let _e193 = global_1.member[_e162];
                let _e198 = global_1.member[(_e162 + 1u)];
                let _e203 = global_1.member[(_e162 + 2u)];
                let _e209 = global_1.member[(_e162 + 3u)];
                let _e214 = global_1.member[(_e162 + 4u)];
                let _e219 = global_1.member[(_e162 + 5u)];
                let _e224 = global_1.member[(_e162 + 6u)];
                let _e229 = global_1.member[(_e162 + 7u)];
                let _e235 = global_1.member[(_e162 + 8u)];
                let _e240 = global_1.member[(_e162 + 9u)];
                let _e245 = global_1.member[(_e162 + 10u)];
                let _e249 = global_1.member[(_e162 + 11u)];
                let _e253 = global_1.member[(_e162 + 12u)];
                let _e257 = global_1.member[(_e162 + 13u)];
                let _e261 = global_1.member[(_e162 + 14u)];
                let _e265 = global_1.member[(_e162 + 15u)];
                let _e269 = global_1.member[(_e162 + 16u)];
                let _e273 = global_1.member[(_e162 + 17u)];
                let _e277 = global_1.member[(_e162 + 18u)];
                let _e281 = global_1.member[(_e162 + 19u)];
                let _e285 = global_1.member[(_e162 + 20u)];
                let _e290 = global_1.member[(_e162 + 21u)];
                phi_846_ = type_35(vec3<f32>(bitcast<f32>(_e193), bitcast<f32>(_e198), bitcast<f32>(_e203)), bitcast<f32>(_e209), vec4<f32>(bitcast<f32>(_e214), bitcast<f32>(_e219), bitcast<f32>(_e224), bitcast<f32>(_e229)), bitcast<f32>(_e235), bitcast<f32>(_e240), _e245, _e249, _e253, _e257, _e261, _e265, _e269, _e273, _e277, _e281, ((_e285 == 1u) && (_e188 == 1u)), bitcast<f32>(_e290));
            }
            let _e295 = phi_846_;
            let _e299 = select(_e154, _e153, vec2((_e295.member_10 == 0u)));
            let _e301 = (_e148 >= 8u);
            if _e301 {
                phi_5791_ = (_e295.member_5 <= (_e148 - 8u));
            } else {
                phi_5791_ = false;
            }
            let _e305 = phi_5791_;
            if _e305 {
                let _e308 = global_1.member[_e295.member_5];
                let _e312 = global_1.member[(_e295.member_5 + 1u)];
                let _e317 = global_1.member[(_e295.member_5 + 2u)];
                let _e321 = global_1.member[(_e295.member_5 + 3u)];
                let _e326 = global_1.member[(_e295.member_5 + 4u)];
                let _e330 = global_1.member[(_e295.member_5 + 5u)];
                let _e334 = global_1.member[(_e295.member_5 + 6u)];
                switch bitcast<i32>(_e334) {
                    case 0: {
                        phi_886_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_886_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_886_ = 2u;
                        break;
                    }
                    default: {
                        phi_886_ = 0u;
                        break;
                    }
                }
                let _e337 = phi_886_;
                let _e341 = global_1.member[(_e295.member_5 + 7u)];
                switch bitcast<i32>(_e341) {
                    case 0: {
                        phi_895_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_895_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_895_ = 2u;
                        break;
                    }
                    default: {
                        phi_895_ = 0u;
                        break;
                    }
                }
                let _e344 = phi_895_;
                phi_908_ = type_15(type_14(_e337, _e344), vec2<u32>(_e308, _e312), vec2<u32>(_e317, _e321), _e326, _e330);
            } else {
                phi_908_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e348 = phi_908_;
            switch bitcast<i32>(_e348.member.member) {
                case 1: {
                    let _e386 = abs(_e299.x);
                    let _e388 = (_e386 % 1f);
                    if (_e386 >= 1f) {
                        phi_5843_ = select(true, false, (_e388 == 0f));
                    } else {
                        phi_5843_ = true;
                    }
                    let _e392 = phi_5843_;
                    let _e393 = select(1f, _e388, _e392);
                    if (select(-1f, 1f, (_e299.x >= 0f)) > 0f) {
                        phi_928_ = _e393;
                    } else {
                        phi_928_ = (1f - _e393);
                    }
                    let _e397 = phi_928_;
                    phi_965_ = _e397;
                    break;
                }
                case 2: {
                    let _e360 = abs(_e299.x);
                    let _e367 = ((select(select(u32(_e360), 0u, (_e360 < 0f)), 4294967295u, (_e360 > 4294967000f)) % 2u) == 0u);
                    let _e369 = (_e360 % 1f);
                    if (_e360 >= 1f) {
                        phi_5826_ = select(true, false, (_e369 == 0f));
                    } else {
                        phi_5826_ = true;
                    }
                    let _e373 = phi_5826_;
                    let _e374 = select(1f, _e369, _e373);
                    if (select(-1f, 1f, (_e299.x >= 0f)) > 0f) {
                        if _e367 {
                            phi_957_ = _e374;
                        } else {
                            phi_957_ = (1f - _e374);
                        }
                        let _e381 = phi_957_;
                        phi_963_ = _e381;
                    } else {
                        if _e367 {
                            phi_962_ = (1f - _e374);
                        } else {
                            phi_962_ = _e374;
                        }
                        let _e378 = phi_962_;
                        phi_963_ = _e378;
                    }
                    let _e383 = phi_963_;
                    phi_965_ = _e383;
                    break;
                }
                case 0: {
                    if (_e299.x > 1f) {
                        phi_5813_ = 0.9999999f;
                    } else {
                        phi_5813_ = select(_e299.x, 0.00000011920929f, (_e299.x < 0f));
                    }
                    let _e357 = phi_5813_;
                    phi_965_ = _e357;
                    break;
                }
                default: {
                    phi_965_ = f32();
                    break;
                }
            }
            let _e399 = phi_965_;
            switch bitcast<i32>(_e348.member.member_1) {
                case 1: {
                    let _e437 = abs(_e299.y);
                    let _e439 = (_e437 % 1f);
                    if (_e437 >= 1f) {
                        phi_5891_ = select(true, false, (_e439 == 0f));
                    } else {
                        phi_5891_ = true;
                    }
                    let _e443 = phi_5891_;
                    let _e444 = select(1f, _e439, _e443);
                    if (select(-1f, 1f, (_e299.y >= 0f)) > 0f) {
                        phi_986_ = _e444;
                    } else {
                        phi_986_ = (1f - _e444);
                    }
                    let _e448 = phi_986_;
                    phi_1023_ = _e448;
                    break;
                }
                case 2: {
                    let _e411 = abs(_e299.y);
                    let _e418 = ((select(select(u32(_e411), 0u, (_e411 < 0f)), 4294967295u, (_e411 > 4294967000f)) % 2u) == 0u);
                    let _e420 = (_e411 % 1f);
                    if (_e411 >= 1f) {
                        phi_5874_ = select(true, false, (_e420 == 0f));
                    } else {
                        phi_5874_ = true;
                    }
                    let _e424 = phi_5874_;
                    let _e425 = select(1f, _e420, _e424);
                    if (select(-1f, 1f, (_e299.y >= 0f)) > 0f) {
                        if _e418 {
                            phi_1015_ = _e425;
                        } else {
                            phi_1015_ = (1f - _e425);
                        }
                        let _e432 = phi_1015_;
                        phi_1021_ = _e432;
                    } else {
                        if _e418 {
                            phi_1020_ = (1f - _e425);
                        } else {
                            phi_1020_ = _e425;
                        }
                        let _e429 = phi_1020_;
                        phi_1021_ = _e429;
                    }
                    let _e434 = phi_1021_;
                    phi_1023_ = _e434;
                    break;
                }
                case 0: {
                    if (_e299.y > 1f) {
                        phi_5861_ = 0.9999999f;
                    } else {
                        phi_5861_ = select(_e299.y, 0.00000011920929f, (_e299.y < 0f));
                    }
                    let _e408 = phi_5861_;
                    phi_1023_ = _e408;
                    break;
                }
                default: {
                    phi_1023_ = f32();
                    break;
                }
            }
            let _e450 = phi_1023_;
            let _e454 = (_e399 * f32(_e348.member_2.x));
            let _e463 = (_e450 * f32(_e348.member_2.y));
            let _e475 = f32(_e173);
            let _e476 = f32(_e177);
            let _e483 = vec3<f32>((f32((select(select(u32(_e454), 0u, (_e454 < 0f)), 4294967295u, (_e454 > 4294967000f)) + _e348.member_1.x)) / _e475), (f32((select(select(u32(_e463), 0u, (_e463 < 0f)), 4294967295u, (_e463 > 4294967000f)) + _e348.member_1.y)) / _e476), f32(_e348.member_3));
            let _e489 = textureSampleLevel(global_12, global_11, vec2<f32>(_e483.x, _e483.y), i32(_e483.z), 0f);
            let _e492 = select(_e489, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_5 == 4294967295u)));
            let _e496 = select(_e154, _e153, vec2((_e295.member_11 == 0u)));
            if _e301 {
                phi_5927_ = (_e295.member_6 <= (_e148 - 8u));
            } else {
                phi_5927_ = false;
            }
            let _e501 = phi_5927_;
            if _e501 {
                let _e504 = global_1.member[_e295.member_6];
                let _e508 = global_1.member[(_e295.member_6 + 1u)];
                let _e513 = global_1.member[(_e295.member_6 + 2u)];
                let _e517 = global_1.member[(_e295.member_6 + 3u)];
                let _e522 = global_1.member[(_e295.member_6 + 4u)];
                let _e526 = global_1.member[(_e295.member_6 + 5u)];
                let _e530 = global_1.member[(_e295.member_6 + 6u)];
                switch bitcast<i32>(_e530) {
                    case 0: {
                        phi_1106_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1106_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1106_ = 2u;
                        break;
                    }
                    default: {
                        phi_1106_ = 0u;
                        break;
                    }
                }
                let _e533 = phi_1106_;
                let _e537 = global_1.member[(_e295.member_6 + 7u)];
                switch bitcast<i32>(_e537) {
                    case 0: {
                        phi_1115_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1115_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1115_ = 2u;
                        break;
                    }
                    default: {
                        phi_1115_ = 0u;
                        break;
                    }
                }
                let _e540 = phi_1115_;
                phi_1128_ = type_15(type_14(_e533, _e540), vec2<u32>(_e504, _e508), vec2<u32>(_e513, _e517), _e522, _e526);
            } else {
                phi_1128_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e544 = phi_1128_;
            switch bitcast<i32>(_e544.member.member) {
                case 1: {
                    let _e582 = abs(_e496.x);
                    let _e584 = (_e582 % 1f);
                    if (_e582 >= 1f) {
                        phi_5978_ = select(true, false, (_e584 == 0f));
                    } else {
                        phi_5978_ = true;
                    }
                    let _e588 = phi_5978_;
                    let _e589 = select(1f, _e584, _e588);
                    if (select(-1f, 1f, (_e496.x >= 0f)) > 0f) {
                        phi_1148_ = _e589;
                    } else {
                        phi_1148_ = (1f - _e589);
                    }
                    let _e593 = phi_1148_;
                    phi_1185_ = _e593;
                    break;
                }
                case 2: {
                    let _e556 = abs(_e496.x);
                    let _e563 = ((select(select(u32(_e556), 0u, (_e556 < 0f)), 4294967295u, (_e556 > 4294967000f)) % 2u) == 0u);
                    let _e565 = (_e556 % 1f);
                    if (_e556 >= 1f) {
                        phi_5961_ = select(true, false, (_e565 == 0f));
                    } else {
                        phi_5961_ = true;
                    }
                    let _e569 = phi_5961_;
                    let _e570 = select(1f, _e565, _e569);
                    if (select(-1f, 1f, (_e496.x >= 0f)) > 0f) {
                        if _e563 {
                            phi_1177_ = _e570;
                        } else {
                            phi_1177_ = (1f - _e570);
                        }
                        let _e577 = phi_1177_;
                        phi_1183_ = _e577;
                    } else {
                        if _e563 {
                            phi_1182_ = (1f - _e570);
                        } else {
                            phi_1182_ = _e570;
                        }
                        let _e574 = phi_1182_;
                        phi_1183_ = _e574;
                    }
                    let _e579 = phi_1183_;
                    phi_1185_ = _e579;
                    break;
                }
                case 0: {
                    if (_e496.x > 1f) {
                        phi_5948_ = 0.9999999f;
                    } else {
                        phi_5948_ = select(_e496.x, 0.00000011920929f, (_e496.x < 0f));
                    }
                    let _e553 = phi_5948_;
                    phi_1185_ = _e553;
                    break;
                }
                default: {
                    phi_1185_ = f32();
                    break;
                }
            }
            let _e595 = phi_1185_;
            switch bitcast<i32>(_e544.member.member_1) {
                case 1: {
                    let _e633 = abs(_e496.y);
                    let _e635 = (_e633 % 1f);
                    if (_e633 >= 1f) {
                        phi_6026_ = select(true, false, (_e635 == 0f));
                    } else {
                        phi_6026_ = true;
                    }
                    let _e639 = phi_6026_;
                    let _e640 = select(1f, _e635, _e639);
                    if (select(-1f, 1f, (_e496.y >= 0f)) > 0f) {
                        phi_1206_ = _e640;
                    } else {
                        phi_1206_ = (1f - _e640);
                    }
                    let _e644 = phi_1206_;
                    phi_1243_ = _e644;
                    break;
                }
                case 2: {
                    let _e607 = abs(_e496.y);
                    let _e614 = ((select(select(u32(_e607), 0u, (_e607 < 0f)), 4294967295u, (_e607 > 4294967000f)) % 2u) == 0u);
                    let _e616 = (_e607 % 1f);
                    if (_e607 >= 1f) {
                        phi_6009_ = select(true, false, (_e616 == 0f));
                    } else {
                        phi_6009_ = true;
                    }
                    let _e620 = phi_6009_;
                    let _e621 = select(1f, _e616, _e620);
                    if (select(-1f, 1f, (_e496.y >= 0f)) > 0f) {
                        if _e614 {
                            phi_1235_ = _e621;
                        } else {
                            phi_1235_ = (1f - _e621);
                        }
                        let _e628 = phi_1235_;
                        phi_1241_ = _e628;
                    } else {
                        if _e614 {
                            phi_1240_ = (1f - _e621);
                        } else {
                            phi_1240_ = _e621;
                        }
                        let _e625 = phi_1240_;
                        phi_1241_ = _e625;
                    }
                    let _e630 = phi_1241_;
                    phi_1243_ = _e630;
                    break;
                }
                case 0: {
                    if (_e496.y > 1f) {
                        phi_5996_ = 0.9999999f;
                    } else {
                        phi_5996_ = select(_e496.y, 0.00000011920929f, (_e496.y < 0f));
                    }
                    let _e604 = phi_5996_;
                    phi_1243_ = _e604;
                    break;
                }
                default: {
                    phi_1243_ = f32();
                    break;
                }
            }
            let _e646 = phi_1243_;
            let _e650 = (_e595 * f32(_e544.member_2.x));
            let _e659 = (_e646 * f32(_e544.member_2.y));
            let _e677 = vec3<f32>((f32((select(select(u32(_e650), 0u, (_e650 < 0f)), 4294967295u, (_e650 > 4294967000f)) + _e544.member_1.x)) / _e475), (f32((select(select(u32(_e659), 0u, (_e659 < 0f)), 4294967295u, (_e659 > 4294967000f)) + _e544.member_1.y)) / _e476), f32(_e544.member_3));
            let _e683 = textureSampleLevel(global_12, global_11, vec2<f32>(_e677.x, _e677.y), i32(_e677.z), 0f);
            let _e686 = select(_e683, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_6 == 4294967295u)));
            let _e690 = select(_e154, _e153, vec2((_e295.member_12 == 0u)));
            if _e301 {
                phi_6062_ = (_e295.member_7 <= (_e148 - 8u));
            } else {
                phi_6062_ = false;
            }
            let _e695 = phi_6062_;
            if _e695 {
                let _e698 = global_1.member[_e295.member_7];
                let _e702 = global_1.member[(_e295.member_7 + 1u)];
                let _e707 = global_1.member[(_e295.member_7 + 2u)];
                let _e711 = global_1.member[(_e295.member_7 + 3u)];
                let _e716 = global_1.member[(_e295.member_7 + 4u)];
                let _e720 = global_1.member[(_e295.member_7 + 5u)];
                let _e724 = global_1.member[(_e295.member_7 + 6u)];
                switch bitcast<i32>(_e724) {
                    case 0: {
                        phi_1326_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1326_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1326_ = 2u;
                        break;
                    }
                    default: {
                        phi_1326_ = 0u;
                        break;
                    }
                }
                let _e727 = phi_1326_;
                let _e731 = global_1.member[(_e295.member_7 + 7u)];
                switch bitcast<i32>(_e731) {
                    case 0: {
                        phi_1335_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1335_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1335_ = 2u;
                        break;
                    }
                    default: {
                        phi_1335_ = 0u;
                        break;
                    }
                }
                let _e734 = phi_1335_;
                phi_1348_ = type_15(type_14(_e727, _e734), vec2<u32>(_e698, _e702), vec2<u32>(_e707, _e711), _e716, _e720);
            } else {
                phi_1348_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e738 = phi_1348_;
            switch bitcast<i32>(_e738.member.member) {
                case 1: {
                    let _e776 = abs(_e690.x);
                    let _e778 = (_e776 % 1f);
                    if (_e776 >= 1f) {
                        phi_6113_ = select(true, false, (_e778 == 0f));
                    } else {
                        phi_6113_ = true;
                    }
                    let _e782 = phi_6113_;
                    let _e783 = select(1f, _e778, _e782);
                    if (select(-1f, 1f, (_e690.x >= 0f)) > 0f) {
                        phi_1368_ = _e783;
                    } else {
                        phi_1368_ = (1f - _e783);
                    }
                    let _e787 = phi_1368_;
                    phi_1405_ = _e787;
                    break;
                }
                case 2: {
                    let _e750 = abs(_e690.x);
                    let _e757 = ((select(select(u32(_e750), 0u, (_e750 < 0f)), 4294967295u, (_e750 > 4294967000f)) % 2u) == 0u);
                    let _e759 = (_e750 % 1f);
                    if (_e750 >= 1f) {
                        phi_6096_ = select(true, false, (_e759 == 0f));
                    } else {
                        phi_6096_ = true;
                    }
                    let _e763 = phi_6096_;
                    let _e764 = select(1f, _e759, _e763);
                    if (select(-1f, 1f, (_e690.x >= 0f)) > 0f) {
                        if _e757 {
                            phi_1397_ = _e764;
                        } else {
                            phi_1397_ = (1f - _e764);
                        }
                        let _e771 = phi_1397_;
                        phi_1403_ = _e771;
                    } else {
                        if _e757 {
                            phi_1402_ = (1f - _e764);
                        } else {
                            phi_1402_ = _e764;
                        }
                        let _e768 = phi_1402_;
                        phi_1403_ = _e768;
                    }
                    let _e773 = phi_1403_;
                    phi_1405_ = _e773;
                    break;
                }
                case 0: {
                    if (_e690.x > 1f) {
                        phi_6083_ = 0.9999999f;
                    } else {
                        phi_6083_ = select(_e690.x, 0.00000011920929f, (_e690.x < 0f));
                    }
                    let _e747 = phi_6083_;
                    phi_1405_ = _e747;
                    break;
                }
                default: {
                    phi_1405_ = f32();
                    break;
                }
            }
            let _e789 = phi_1405_;
            switch bitcast<i32>(_e738.member.member_1) {
                case 1: {
                    let _e827 = abs(_e690.y);
                    let _e829 = (_e827 % 1f);
                    if (_e827 >= 1f) {
                        phi_6161_ = select(true, false, (_e829 == 0f));
                    } else {
                        phi_6161_ = true;
                    }
                    let _e833 = phi_6161_;
                    let _e834 = select(1f, _e829, _e833);
                    if (select(-1f, 1f, (_e690.y >= 0f)) > 0f) {
                        phi_1426_ = _e834;
                    } else {
                        phi_1426_ = (1f - _e834);
                    }
                    let _e838 = phi_1426_;
                    phi_1463_ = _e838;
                    break;
                }
                case 2: {
                    let _e801 = abs(_e690.y);
                    let _e808 = ((select(select(u32(_e801), 0u, (_e801 < 0f)), 4294967295u, (_e801 > 4294967000f)) % 2u) == 0u);
                    let _e810 = (_e801 % 1f);
                    if (_e801 >= 1f) {
                        phi_6144_ = select(true, false, (_e810 == 0f));
                    } else {
                        phi_6144_ = true;
                    }
                    let _e814 = phi_6144_;
                    let _e815 = select(1f, _e810, _e814);
                    if (select(-1f, 1f, (_e690.y >= 0f)) > 0f) {
                        if _e808 {
                            phi_1455_ = _e815;
                        } else {
                            phi_1455_ = (1f - _e815);
                        }
                        let _e822 = phi_1455_;
                        phi_1461_ = _e822;
                    } else {
                        if _e808 {
                            phi_1460_ = (1f - _e815);
                        } else {
                            phi_1460_ = _e815;
                        }
                        let _e819 = phi_1460_;
                        phi_1461_ = _e819;
                    }
                    let _e824 = phi_1461_;
                    phi_1463_ = _e824;
                    break;
                }
                case 0: {
                    if (_e690.y > 1f) {
                        phi_6131_ = 0.9999999f;
                    } else {
                        phi_6131_ = select(_e690.y, 0.00000011920929f, (_e690.y < 0f));
                    }
                    let _e798 = phi_6131_;
                    phi_1463_ = _e798;
                    break;
                }
                default: {
                    phi_1463_ = f32();
                    break;
                }
            }
            let _e840 = phi_1463_;
            let _e844 = (_e789 * f32(_e738.member_2.x));
            let _e853 = (_e840 * f32(_e738.member_2.y));
            let _e871 = vec3<f32>((f32((select(select(u32(_e844), 0u, (_e844 < 0f)), 4294967295u, (_e844 > 4294967000f)) + _e738.member_1.x)) / _e475), (f32((select(select(u32(_e853), 0u, (_e853 < 0f)), 4294967295u, (_e853 > 4294967000f)) + _e738.member_1.y)) / _e476), f32(_e738.member_3));
            let _e877 = textureSampleLevel(global_12, global_11, vec2<f32>(_e871.x, _e871.y), i32(_e871.z), 0f);
            let _e878 = (_e295.member_7 == 4294967295u);
            let _e880 = select(_e877, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e878));
            let _e884 = select(_e154, _e153, vec2((_e295.member_13 == 0u)));
            if _e301 {
                phi_6197_ = (_e295.member_8 <= (_e148 - 8u));
            } else {
                phi_6197_ = false;
            }
            let _e889 = phi_6197_;
            if _e889 {
                let _e892 = global_1.member[_e295.member_8];
                let _e896 = global_1.member[(_e295.member_8 + 1u)];
                let _e901 = global_1.member[(_e295.member_8 + 2u)];
                let _e905 = global_1.member[(_e295.member_8 + 3u)];
                let _e910 = global_1.member[(_e295.member_8 + 4u)];
                let _e914 = global_1.member[(_e295.member_8 + 5u)];
                let _e918 = global_1.member[(_e295.member_8 + 6u)];
                switch bitcast<i32>(_e918) {
                    case 0: {
                        phi_1546_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1546_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1546_ = 2u;
                        break;
                    }
                    default: {
                        phi_1546_ = 0u;
                        break;
                    }
                }
                let _e921 = phi_1546_;
                let _e925 = global_1.member[(_e295.member_8 + 7u)];
                switch bitcast<i32>(_e925) {
                    case 0: {
                        phi_1555_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1555_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1555_ = 2u;
                        break;
                    }
                    default: {
                        phi_1555_ = 0u;
                        break;
                    }
                }
                let _e928 = phi_1555_;
                phi_1568_ = type_15(type_14(_e921, _e928), vec2<u32>(_e892, _e896), vec2<u32>(_e901, _e905), _e910, _e914);
            } else {
                phi_1568_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e932 = phi_1568_;
            switch bitcast<i32>(_e932.member.member) {
                case 1: {
                    let _e970 = abs(_e884.x);
                    let _e972 = (_e970 % 1f);
                    if (_e970 >= 1f) {
                        phi_6248_ = select(true, false, (_e972 == 0f));
                    } else {
                        phi_6248_ = true;
                    }
                    let _e976 = phi_6248_;
                    let _e977 = select(1f, _e972, _e976);
                    if (select(-1f, 1f, (_e884.x >= 0f)) > 0f) {
                        phi_1588_ = _e977;
                    } else {
                        phi_1588_ = (1f - _e977);
                    }
                    let _e981 = phi_1588_;
                    phi_1625_ = _e981;
                    break;
                }
                case 2: {
                    let _e944 = abs(_e884.x);
                    let _e951 = ((select(select(u32(_e944), 0u, (_e944 < 0f)), 4294967295u, (_e944 > 4294967000f)) % 2u) == 0u);
                    let _e953 = (_e944 % 1f);
                    if (_e944 >= 1f) {
                        phi_6231_ = select(true, false, (_e953 == 0f));
                    } else {
                        phi_6231_ = true;
                    }
                    let _e957 = phi_6231_;
                    let _e958 = select(1f, _e953, _e957);
                    if (select(-1f, 1f, (_e884.x >= 0f)) > 0f) {
                        if _e951 {
                            phi_1617_ = _e958;
                        } else {
                            phi_1617_ = (1f - _e958);
                        }
                        let _e965 = phi_1617_;
                        phi_1623_ = _e965;
                    } else {
                        if _e951 {
                            phi_1622_ = (1f - _e958);
                        } else {
                            phi_1622_ = _e958;
                        }
                        let _e962 = phi_1622_;
                        phi_1623_ = _e962;
                    }
                    let _e967 = phi_1623_;
                    phi_1625_ = _e967;
                    break;
                }
                case 0: {
                    if (_e884.x > 1f) {
                        phi_6218_ = 0.9999999f;
                    } else {
                        phi_6218_ = select(_e884.x, 0.00000011920929f, (_e884.x < 0f));
                    }
                    let _e941 = phi_6218_;
                    phi_1625_ = _e941;
                    break;
                }
                default: {
                    phi_1625_ = f32();
                    break;
                }
            }
            let _e983 = phi_1625_;
            switch bitcast<i32>(_e932.member.member_1) {
                case 1: {
                    let _e1021 = abs(_e884.y);
                    let _e1023 = (_e1021 % 1f);
                    if (_e1021 >= 1f) {
                        phi_6296_ = select(true, false, (_e1023 == 0f));
                    } else {
                        phi_6296_ = true;
                    }
                    let _e1027 = phi_6296_;
                    let _e1028 = select(1f, _e1023, _e1027);
                    if (select(-1f, 1f, (_e884.y >= 0f)) > 0f) {
                        phi_1646_ = _e1028;
                    } else {
                        phi_1646_ = (1f - _e1028);
                    }
                    let _e1032 = phi_1646_;
                    phi_1683_ = _e1032;
                    break;
                }
                case 2: {
                    let _e995 = abs(_e884.y);
                    let _e1002 = ((select(select(u32(_e995), 0u, (_e995 < 0f)), 4294967295u, (_e995 > 4294967000f)) % 2u) == 0u);
                    let _e1004 = (_e995 % 1f);
                    if (_e995 >= 1f) {
                        phi_6279_ = select(true, false, (_e1004 == 0f));
                    } else {
                        phi_6279_ = true;
                    }
                    let _e1008 = phi_6279_;
                    let _e1009 = select(1f, _e1004, _e1008);
                    if (select(-1f, 1f, (_e884.y >= 0f)) > 0f) {
                        if _e1002 {
                            phi_1675_ = _e1009;
                        } else {
                            phi_1675_ = (1f - _e1009);
                        }
                        let _e1016 = phi_1675_;
                        phi_1681_ = _e1016;
                    } else {
                        if _e1002 {
                            phi_1680_ = (1f - _e1009);
                        } else {
                            phi_1680_ = _e1009;
                        }
                        let _e1013 = phi_1680_;
                        phi_1681_ = _e1013;
                    }
                    let _e1018 = phi_1681_;
                    phi_1683_ = _e1018;
                    break;
                }
                case 0: {
                    if (_e884.y > 1f) {
                        phi_6266_ = 0.9999999f;
                    } else {
                        phi_6266_ = select(_e884.y, 0.00000011920929f, (_e884.y < 0f));
                    }
                    let _e992 = phi_6266_;
                    phi_1683_ = _e992;
                    break;
                }
                default: {
                    phi_1683_ = f32();
                    break;
                }
            }
            let _e1034 = phi_1683_;
            let _e1038 = (_e983 * f32(_e932.member_2.x));
            let _e1047 = (_e1034 * f32(_e932.member_2.y));
            let _e1065 = vec3<f32>((f32((select(select(u32(_e1038), 0u, (_e1038 < 0f)), 4294967295u, (_e1038 > 4294967000f)) + _e932.member_1.x)) / _e475), (f32((select(select(u32(_e1047), 0u, (_e1047 < 0f)), 4294967295u, (_e1047 > 4294967000f)) + _e932.member_1.y)) / _e476), f32(_e932.member_3));
            let _e1071 = textureSampleLevel(global_12, global_11, vec2<f32>(_e1065.x, _e1065.y), i32(_e1065.z), 0f);
            let _e1078 = select(_e154, _e153, vec2((_e295.member_14 == 0u)));
            if _e301 {
                phi_6332_ = (_e295.member_9 <= (_e148 - 8u));
            } else {
                phi_6332_ = false;
            }
            let _e1083 = phi_6332_;
            if _e1083 {
                let _e1086 = global_1.member[_e295.member_9];
                let _e1090 = global_1.member[(_e295.member_9 + 1u)];
                let _e1095 = global_1.member[(_e295.member_9 + 2u)];
                let _e1099 = global_1.member[(_e295.member_9 + 3u)];
                let _e1104 = global_1.member[(_e295.member_9 + 4u)];
                let _e1108 = global_1.member[(_e295.member_9 + 5u)];
                let _e1112 = global_1.member[(_e295.member_9 + 6u)];
                switch bitcast<i32>(_e1112) {
                    case 0: {
                        phi_1766_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1766_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1766_ = 2u;
                        break;
                    }
                    default: {
                        phi_1766_ = 0u;
                        break;
                    }
                }
                let _e1115 = phi_1766_;
                let _e1119 = global_1.member[(_e295.member_9 + 7u)];
                switch bitcast<i32>(_e1119) {
                    case 0: {
                        phi_1775_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1775_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1775_ = 2u;
                        break;
                    }
                    default: {
                        phi_1775_ = 0u;
                        break;
                    }
                }
                let _e1122 = phi_1775_;
                phi_1788_ = type_15(type_14(_e1115, _e1122), vec2<u32>(_e1086, _e1090), vec2<u32>(_e1095, _e1099), _e1104, _e1108);
            } else {
                phi_1788_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e1126 = phi_1788_;
            switch bitcast<i32>(_e1126.member.member) {
                case 1: {
                    let _e1164 = abs(_e1078.x);
                    let _e1166 = (_e1164 % 1f);
                    if (_e1164 >= 1f) {
                        phi_6383_ = select(true, false, (_e1166 == 0f));
                    } else {
                        phi_6383_ = true;
                    }
                    let _e1170 = phi_6383_;
                    let _e1171 = select(1f, _e1166, _e1170);
                    if (select(-1f, 1f, (_e1078.x >= 0f)) > 0f) {
                        phi_1808_ = _e1171;
                    } else {
                        phi_1808_ = (1f - _e1171);
                    }
                    let _e1175 = phi_1808_;
                    phi_1845_ = _e1175;
                    break;
                }
                case 2: {
                    let _e1138 = abs(_e1078.x);
                    let _e1145 = ((select(select(u32(_e1138), 0u, (_e1138 < 0f)), 4294967295u, (_e1138 > 4294967000f)) % 2u) == 0u);
                    let _e1147 = (_e1138 % 1f);
                    if (_e1138 >= 1f) {
                        phi_6366_ = select(true, false, (_e1147 == 0f));
                    } else {
                        phi_6366_ = true;
                    }
                    let _e1151 = phi_6366_;
                    let _e1152 = select(1f, _e1147, _e1151);
                    if (select(-1f, 1f, (_e1078.x >= 0f)) > 0f) {
                        if _e1145 {
                            phi_1837_ = _e1152;
                        } else {
                            phi_1837_ = (1f - _e1152);
                        }
                        let _e1159 = phi_1837_;
                        phi_1843_ = _e1159;
                    } else {
                        if _e1145 {
                            phi_1842_ = (1f - _e1152);
                        } else {
                            phi_1842_ = _e1152;
                        }
                        let _e1156 = phi_1842_;
                        phi_1843_ = _e1156;
                    }
                    let _e1161 = phi_1843_;
                    phi_1845_ = _e1161;
                    break;
                }
                case 0: {
                    if (_e1078.x > 1f) {
                        phi_6353_ = 0.9999999f;
                    } else {
                        phi_6353_ = select(_e1078.x, 0.00000011920929f, (_e1078.x < 0f));
                    }
                    let _e1135 = phi_6353_;
                    phi_1845_ = _e1135;
                    break;
                }
                default: {
                    phi_1845_ = f32();
                    break;
                }
            }
            let _e1177 = phi_1845_;
            switch bitcast<i32>(_e1126.member.member_1) {
                case 1: {
                    let _e1215 = abs(_e1078.y);
                    let _e1217 = (_e1215 % 1f);
                    if (_e1215 >= 1f) {
                        phi_6431_ = select(true, false, (_e1217 == 0f));
                    } else {
                        phi_6431_ = true;
                    }
                    let _e1221 = phi_6431_;
                    let _e1222 = select(1f, _e1217, _e1221);
                    if (select(-1f, 1f, (_e1078.y >= 0f)) > 0f) {
                        phi_1866_ = _e1222;
                    } else {
                        phi_1866_ = (1f - _e1222);
                    }
                    let _e1226 = phi_1866_;
                    phi_1903_ = _e1226;
                    break;
                }
                case 2: {
                    let _e1189 = abs(_e1078.y);
                    let _e1196 = ((select(select(u32(_e1189), 0u, (_e1189 < 0f)), 4294967295u, (_e1189 > 4294967000f)) % 2u) == 0u);
                    let _e1198 = (_e1189 % 1f);
                    if (_e1189 >= 1f) {
                        phi_6414_ = select(true, false, (_e1198 == 0f));
                    } else {
                        phi_6414_ = true;
                    }
                    let _e1202 = phi_6414_;
                    let _e1203 = select(1f, _e1198, _e1202);
                    if (select(-1f, 1f, (_e1078.y >= 0f)) > 0f) {
                        if _e1196 {
                            phi_1895_ = _e1203;
                        } else {
                            phi_1895_ = (1f - _e1203);
                        }
                        let _e1210 = phi_1895_;
                        phi_1901_ = _e1210;
                    } else {
                        if _e1196 {
                            phi_1900_ = (1f - _e1203);
                        } else {
                            phi_1900_ = _e1203;
                        }
                        let _e1207 = phi_1900_;
                        phi_1901_ = _e1207;
                    }
                    let _e1212 = phi_1901_;
                    phi_1903_ = _e1212;
                    break;
                }
                case 0: {
                    if (_e1078.y > 1f) {
                        phi_6401_ = 0.9999999f;
                    } else {
                        phi_6401_ = select(_e1078.y, 0.00000011920929f, (_e1078.y < 0f));
                    }
                    let _e1186 = phi_6401_;
                    phi_1903_ = _e1186;
                    break;
                }
                default: {
                    phi_1903_ = f32();
                    break;
                }
            }
            let _e1228 = phi_1903_;
            let _e1232 = (_e1177 * f32(_e1126.member_2.x));
            let _e1241 = (_e1228 * f32(_e1126.member_2.y));
            let _e1259 = vec3<f32>((f32((select(select(u32(_e1232), 0u, (_e1232 < 0f)), 4294967295u, (_e1232 > 4294967000f)) + _e1126.member_1.x)) / _e475), (f32((select(select(u32(_e1241), 0u, (_e1241 < 0f)), 4294967295u, (_e1241 > 4294967000f)) + _e1126.member_1.y)) / _e476), f32(_e1126.member_3));
            let _e1265 = textureSampleLevel(global_12, global_11, vec2<f32>(_e1259.x, _e1259.y), i32(_e1259.z), 0f);
            let _e1268 = select(_e1265, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_9 == 4294967295u)));
            if _e878 {
                phi_1997_ = vec3<f32>(0f, 0f, 0f);
                phi_1998_ = _e155;
            } else {
                let _e1272 = fma(_e880.x, 2f, -1f);
                let _e1273 = fma(_e880.y, 2f, -1f);
                let _e1274 = fma(_e880.z, 2f, -1f);
                let _e1279 = sqrt(fma(_e1274, _e1274, fma(_e1272, _e1272, (_e1273 * _e1273))));
                if (_e1279 == 0f) {
                    phi_6489_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6489_ = (vec3<f32>(_e1272, _e1273, _e1274) * (1f / _e1279));
                }
                let _e1284 = phi_6489_;
                let _e1291 = sqrt(fma(_e156.z, _e156.z, fma(_e156.x, _e156.x, (_e156.y * _e156.y))));
                if (_e1291 == 0f) {
                    phi_6524_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6524_ = (_e156 * (1f / _e1291));
                }
                let _e1296 = phi_6524_;
                let _e1303 = sqrt(fma(_e157.z, _e157.z, fma(_e157.x, _e157.x, (_e157.y * _e157.y))));
                if (_e1303 == 0f) {
                    phi_6559_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6559_ = (_e157 * (1f / _e1303));
                }
                let _e1308 = phi_6559_;
                let _e1315 = sqrt(fma(_e155.z, _e155.z, fma(_e155.x, _e155.x, (_e155.y * _e155.y))));
                if (_e1315 == 0f) {
                    phi_6594_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6594_ = (_e155 * (1f / _e1315));
                }
                let _e1320 = phi_6594_;
                let _e1339 = fma(_e1320.x, _e1284.z, fma(_e1296.x, _e1284.x, (_e1308.x * _e1284.y)));
                let _e1340 = fma(_e1320.y, _e1284.z, fma(_e1296.y, _e1284.x, (_e1308.y * _e1284.y)));
                let _e1341 = fma(_e1320.z, _e1284.z, fma(_e1296.z, _e1284.x, (_e1308.z * _e1284.y)));
                let _e1346 = sqrt(fma(_e1341, _e1341, fma(_e1339, _e1339, (_e1340 * _e1340))));
                if (_e1346 == 0f) {
                    phi_6629_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6629_ = (vec3<f32>(_e1339, _e1340, _e1341) * (1f / _e1346));
                }
                let _e1351 = phi_6629_;
                phi_1997_ = _e1284;
                phi_1998_ = _e1351;
            }
            let _e1353 = phi_1997_;
            let _e1355 = phi_1998_;
            let _e1359 = (_e492.x * _e295.member_2.x);
            let _e1362 = (_e492.y * _e295.member_2.y);
            let _e1365 = (_e492.z * _e295.member_2.z);
            let _e1370 = (_e1359 * _e152.x);
            let _e1372 = (_e1362 * _e152.y);
            let _e1374 = (_e1365 * _e152.z);
            let _e1379 = (_e686.y * _e295.member_4);
            let _e1382 = (_e686.z * _e295.member_3);
            let _e1386 = fma(_e295.member_16, (select(_e1071, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_8 == 4294967295u))).x - 1f), 1f);
            let _e1392 = (_e1268.x * _e295.member.x);
            let _e1394 = (_e1268.y * _e295.member.y);
            let _e1396 = (_e1268.z * _e295.member.z);
            let _e1401 = textureSampleLevel(global_13, global_14, _e1355, 0f);
            if (_e146 >= 86u) {
                phi_6661_ = (_e169 <= (_e146 - 86u));
            } else {
                phi_6661_ = false;
            }
            let _e1409 = phi_6661_;
            if _e1409 {
                let _e1412 = global.member[_e169];
                let _e1417 = global.member[(_e169 + 1u)];
                let _e1422 = global.member[(_e169 + 2u)];
                let _e1427 = global.member[(_e169 + 3u)];
                let _e1433 = global.member[(_e169 + 4u)];
                let _e1438 = global.member[(_e169 + 5u)];
                let _e1443 = global.member[(_e169 + 6u)];
                let _e1448 = global.member[(_e169 + 7u)];
                let _e1454 = global.member[(_e169 + 8u)];
                let _e1459 = global.member[(_e169 + 9u)];
                let _e1464 = global.member[(_e169 + 10u)];
                let _e1469 = global.member[(_e169 + 11u)];
                let _e1475 = global.member[(_e169 + 12u)];
                let _e1480 = global.member[(_e169 + 13u)];
                let _e1485 = global.member[(_e169 + 14u)];
                let _e1490 = global.member[(_e169 + 15u)];
                let _e1497 = global.member[(_e169 + 16u)];
                let _e1502 = global.member[(_e169 + 17u)];
                let _e1507 = global.member[(_e169 + 18u)];
                let _e1512 = global.member[(_e169 + 19u)];
                let _e1518 = global.member[(_e169 + 20u)];
                let _e1523 = global.member[(_e169 + 21u)];
                let _e1528 = global.member[(_e169 + 22u)];
                let _e1533 = global.member[(_e169 + 23u)];
                let _e1539 = global.member[(_e169 + 24u)];
                let _e1544 = global.member[(_e169 + 25u)];
                let _e1549 = global.member[(_e169 + 26u)];
                let _e1554 = global.member[(_e169 + 27u)];
                let _e1560 = global.member[(_e169 + 28u)];
                let _e1565 = global.member[(_e169 + 29u)];
                let _e1570 = global.member[(_e169 + 30u)];
                let _e1575 = global.member[(_e169 + 31u)];
                let _e1582 = global.member[(_e169 + 32u)];
                let _e1587 = global.member[(_e169 + 33u)];
                let _e1592 = global.member[(_e169 + 34u)];
                local_2 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_2204_ = type_14(0u, 6u);
                loop {
                    let _e1597 = phi_2204_;
                    if (_e1597.member < _e1597.member_1) {
                        phi_2205_ = type_14((_e1597.member + 1u), _e1597.member_1);
                        phi_2228_ = type_14(1u, _e1597.member);
                    } else {
                        phi_2205_ = _e1597;
                        phi_2228_ = type_14(0u, type_14().member_1);
                    }
                    let _e1610 = phi_2205_;
                    let _e1612 = phi_2228_;
                    switch bitcast<i32>(_e1612.member) {
                        case 0: {
                            phi_2255_ = false;
                            break;
                        }
                        case 1: {
                            let _e1617 = ((_e169 + 35u) + (_e1612.member_1 * 4u));
                            let _e1620 = global.member[_e1617];
                            let _e1625 = global.member[(_e1617 + 1u)];
                            let _e1630 = global.member[(_e1617 + 2u)];
                            let _e1635 = global.member[(_e1617 + 3u)];
                            local_2[_e1612.member_1] = vec4<f32>(bitcast<f32>(_e1620), bitcast<f32>(_e1625), bitcast<f32>(_e1630), bitcast<f32>(_e1635));
                            phi_2255_ = true;
                            break;
                        }
                        default: {
                            phi_2255_ = bool();
                            break;
                        }
                    }
                    let _e1640 = phi_2255_;
                    continue;
                    continuing {
                        phi_2204_ = _e1610;
                        break if !(_e1640);
                    }
                }
                let _e1642 = local_2;
                local_1 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                phi_2261_ = type_14(0u, 8u);
                loop {
                    let _e1645 = phi_2261_;
                    if (_e1645.member < _e1645.member_1) {
                        phi_2262_ = type_14((_e1645.member + 1u), _e1645.member_1);
                        phi_2285_ = type_14(1u, _e1645.member);
                    } else {
                        phi_2262_ = _e1645;
                        phi_2285_ = type_14(0u, type_14().member_1);
                    }
                    let _e1658 = phi_2262_;
                    let _e1660 = phi_2285_;
                    switch bitcast<i32>(_e1660.member) {
                        case 0: {
                            phi_2308_ = false;
                            break;
                        }
                        case 1: {
                            let _e1665 = ((_e169 + 59u) + (_e1660.member_1 * 3u));
                            let _e1668 = global.member[_e1665];
                            let _e1673 = global.member[(_e1665 + 1u)];
                            let _e1678 = global.member[(_e1665 + 2u)];
                            local_1[_e1660.member_1] = vec3<f32>(bitcast<f32>(_e1668), bitcast<f32>(_e1673), bitcast<f32>(_e1678));
                            phi_2308_ = true;
                            break;
                        }
                        default: {
                            phi_2308_ = bool();
                            break;
                        }
                    }
                    let _e1683 = phi_2308_;
                    continue;
                    continuing {
                        phi_2261_ = _e1658;
                        break if !(_e1683);
                    }
                }
                let _e1685 = local_1;
                let _e1689 = global.member[(_e169 + 83u)];
                let _e1694 = global.member[(_e169 + 84u)];
                let _e1699 = global.member[(_e169 + 85u)];
                phi_2329_ = type_25(type_23(vec4<f32>(bitcast<f32>(_e1412), bitcast<f32>(_e1417), bitcast<f32>(_e1422), bitcast<f32>(_e1427)), vec4<f32>(bitcast<f32>(_e1433), bitcast<f32>(_e1438), bitcast<f32>(_e1443), bitcast<f32>(_e1448)), vec4<f32>(bitcast<f32>(_e1454), bitcast<f32>(_e1459), bitcast<f32>(_e1464), bitcast<f32>(_e1469)), vec4<f32>(bitcast<f32>(_e1475), bitcast<f32>(_e1480), bitcast<f32>(_e1485), bitcast<f32>(_e1490))), type_23(vec4<f32>(bitcast<f32>(_e1497), bitcast<f32>(_e1502), bitcast<f32>(_e1507), bitcast<f32>(_e1512)), vec4<f32>(bitcast<f32>(_e1518), bitcast<f32>(_e1523), bitcast<f32>(_e1528), bitcast<f32>(_e1533)), vec4<f32>(bitcast<f32>(_e1539), bitcast<f32>(_e1544), bitcast<f32>(_e1549), bitcast<f32>(_e1554)), vec4<f32>(bitcast<f32>(_e1560), bitcast<f32>(_e1565), bitcast<f32>(_e1570), bitcast<f32>(_e1575))), vec3<f32>(bitcast<f32>(_e1582), bitcast<f32>(_e1587), bitcast<f32>(_e1592)), type_24(_e1685, _e1642, vec3<f32>(bitcast<f32>(_e1689), bitcast<f32>(_e1694), bitcast<f32>(_e1699))));
            } else {
                phi_2329_ = type_25(type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_24(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
            }
            let _e1705 = phi_2329_;
            let _e1707 = (_e1705.member_2 - _e158);
            let _e1714 = sqrt(fma(_e1707.z, _e1707.z, fma(_e1707.x, _e1707.x, (_e1707.y * _e1707.y))));
            let _e1715 = (_e1714 == 0f);
            if _e1715 {
                phi_6733_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6733_ = (_e1707 * (1f / _e1714));
            }
            let _e1719 = phi_6733_;
            let _e1720 = -(_e1719);
            let _e1727 = sqrt(fma(_e1355.z, _e1355.z, fma(_e1355.x, _e1355.x, (_e1355.y * _e1355.y))));
            let _e1728 = (_e1727 == 0f);
            if _e1728 {
                phi_6792_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6792_ = (_e1355 * (1f / _e1727));
            }
            let _e1732 = phi_6792_;
            let _e1742 = (2f * fma(_e1732.z, _e1720.z, fma(_e1732.x, _e1720.x, (_e1732.y * _e1720.y))));
            let _e1749 = textureSampleLevel(global_15, global_16, (_e1720 - vec3<f32>((_e1742 * _e1732.x), (_e1742 * _e1732.y), (_e1742 * _e1732.z))), (_e1379 * 4f));
            if _e1715 {
                phi_6866_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6866_ = (_e1707 * (1f / _e1714));
            }
            let _e1756 = phi_6866_;
            let _e1765 = textureSampleLevel(global_17, global_18, vec2<f32>(max(fma(_e1355.z, _e1756.z, fma(_e1355.x, _e1756.x, (_e1355.y * _e1756.y))), 0f), _e1379), 0f);
            switch bitcast<i32>(_e184) {
                case 0: {
                    if _e295.member_15 {
                        if _e1728 {
                            phi_7259_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_7259_ = (_e1355 * (1f / _e1727));
                        }
                        let _e1934 = phi_7259_;
                        if _e1715 {
                            phi_7294_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_7294_ = (_e1707 * (1f / _e1714));
                        }
                        let _e1938 = phi_7294_;
                        let _e1941 = global_2.member[0u];
                        let _e1944 = global_2.member[1u];
                        let _e1947 = global_2.member[2u];
                        phi_8831_ = false;
                        phi_2396_ = type_14(0u, _e1944);
                        phi_2399_ = vec3<f32>(0f, 0f, 0f);
                        loop {
                            let _e1950 = phi_8831_;
                            let _e1952 = phi_2396_;
                            let _e1954 = phi_2399_;
                            local_8 = _e1954;
                            local_9 = _e1954;
                            local_10 = _e1954;
                            if (_e1952.member < _e1952.member_1) {
                                phi_2397_ = type_14((_e1952.member + 1u), _e1952.member_1);
                                phi_2422_ = type_14(1u, _e1952.member);
                            } else {
                                phi_2397_ = _e1952;
                                phi_2422_ = type_14(0u, type_14().member_1);
                            }
                            let _e1967 = phi_2397_;
                            let _e1969 = phi_2422_;
                            switch bitcast<i32>(_e1969.member) {
                                case 0: {
                                    phi_9006_ = _e1950;
                                    phi_2400_ = vec3<f32>();
                                    phi_4994_ = false;
                                    break;
                                }
                                case 1: {
                                    if (_e1969.member_1 >= _e1944) {
                                        phi_7320_ = 4294967295u;
                                    } else {
                                        phi_7320_ = (_e1941 + _e1969.member_1);
                                    }
                                    let _e1976 = phi_7320_;
                                    if (_e150 >= 1u) {
                                        phi_7339_ = (_e1976 <= (_e150 - 1u));
                                    } else {
                                        phi_7339_ = false;
                                    }
                                    let _e1981 = phi_7339_;
                                    if _e1981 {
                                        let _e1984 = global_2.member[_e1976];
                                        phi_2439_ = _e1984;
                                    } else {
                                        phi_2439_ = 4294967295u;
                                    }
                                    let _e1986 = phi_2439_;
                                    if (_e150 >= 4u) {
                                        phi_7363_ = (_e1986 <= (_e150 - 4u));
                                    } else {
                                        phi_7363_ = false;
                                    }
                                    let _e1991 = phi_7363_;
                                    if _e1991 {
                                        let _e1994 = global_2.member[_e1986];
                                        switch bitcast<i32>(_e1994) {
                                            case 0: {
                                                phi_2451_ = 0u;
                                                break;
                                            }
                                            case 1: {
                                                phi_2451_ = 1u;
                                                break;
                                            }
                                            case 2: {
                                                phi_2451_ = 2u;
                                                break;
                                            }
                                            default: {
                                                phi_2451_ = 0u;
                                                break;
                                            }
                                        }
                                        let _e1997 = phi_2451_;
                                        let _e2001 = global_2.member[(_e1986 + 1u)];
                                        let _e2005 = global_2.member[(_e1986 + 2u)];
                                        let _e2009 = global_2.member[(_e1986 + 3u)];
                                        phi_2465_ = type_30(_e1997, _e2001, _e2005, _e2009);
                                    } else {
                                        phi_2465_ = type_30(0u, 4294967295u, 4294967295u, 4294967295u);
                                    }
                                    let _e2012 = phi_2465_;
                                    if (_e150 >= 10u) {
                                        phi_7395_ = (_e2012.member_2 <= (_e150 - 10u));
                                    } else {
                                        phi_7395_ = false;
                                    }
                                    let _e2018 = phi_7395_;
                                    if _e2018 {
                                        let _e2021 = global_2.member[_e2012.member_2];
                                        let _e2026 = global_2.member[(_e2012.member_2 + 1u)];
                                        let _e2031 = global_2.member[(_e2012.member_2 + 2u)];
                                        let _e2037 = global_2.member[(_e2012.member_2 + 3u)];
                                        let _e2042 = global_2.member[(_e2012.member_2 + 4u)];
                                        let _e2047 = global_2.member[(_e2012.member_2 + 5u)];
                                        let _e2052 = global_2.member[(_e2012.member_2 + 6u)];
                                        let _e2058 = global_2.member[(_e2012.member_2 + 7u)];
                                        let _e2063 = global_2.member[(_e2012.member_2 + 8u)];
                                        let _e2068 = global_2.member[(_e2012.member_2 + 9u)];
                                        phi_2515_ = type_31(vec3<f32>(bitcast<f32>(_e2021), bitcast<f32>(_e2026), bitcast<f32>(_e2031)), vec4<f32>(bitcast<f32>(_e2037), bitcast<f32>(_e2042), bitcast<f32>(_e2047), bitcast<f32>(_e2052)), vec3<f32>(bitcast<f32>(_e2058), bitcast<f32>(_e2063), bitcast<f32>(_e2068)));
                                    } else {
                                        phi_2515_ = type_31(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e2073 = phi_2515_;
                                    let _e2081 = (_e2073.member_1.x + _e2073.member_1.x);
                                    let _e2082 = (_e2073.member_1.y + _e2073.member_1.y);
                                    let _e2083 = (_e2073.member_1.z + _e2073.member_1.z);
                                    let _e2085 = (_e2073.member_1.z * _e2083);
                                    let _e2086 = (_e2073.member_1.w * _e2081);
                                    let _e2087 = (_e2073.member_1.w * _e2082);
                                    let _e2088 = (_e2073.member_1.w * _e2083);
                                    let _e2108 = (vec4<f32>((1f - fma(_e2073.member_1.y, _e2082, _e2085)), fma(_e2073.member_1.x, _e2082, _e2088), fma(_e2073.member_1.x, _e2083, -(_e2087)), 0f) * _e2073.member_2.x);
                                    let _e2110 = (vec4<f32>(fma(_e2073.member_1.x, _e2082, -(_e2088)), (1f - fma(_e2073.member_1.x, _e2081, _e2085)), fma(_e2073.member_1.y, _e2083, _e2086), 0f) * _e2073.member_2.y);
                                    let _e2112 = (vec4<f32>(fma(_e2073.member_1.x, _e2083, _e2087), fma(_e2073.member_1.y, _e2083, -(_e2086)), (1f - fma(_e2073.member_1.x, _e2081, (_e2073.member_1.y * _e2082))), 0f) * _e2073.member_2.z);
                                    switch bitcast<i32>(_e2012.member) {
                                        case 0: {
                                            if (_e150 >= 8u) {
                                                phi_8294_ = (_e2012.member_1 <= (_e150 - 8u));
                                            } else {
                                                phi_8294_ = false;
                                            }
                                            let _e3544 = phi_8294_;
                                            if _e3544 {
                                                let _e3547 = global_2.member[_e2012.member_1];
                                                let _e3552 = global_2.member[(_e2012.member_1 + 1u)];
                                                let _e3557 = global_2.member[(_e2012.member_1 + 2u)];
                                                let _e3563 = global_2.member[(_e2012.member_1 + 3u)];
                                                let _e3568 = global_2.member[(_e2012.member_1 + 4u)];
                                                let _e3573 = global_2.member[(_e2012.member_1 + 5u)];
                                                let _e3578 = global_2.member[(_e2012.member_1 + 6u)];
                                                let _e3584 = global_2.member[(_e2012.member_1 + 7u)];
                                                phi_2563_ = type_36(vec3<f32>(bitcast<f32>(_e3547), bitcast<f32>(_e3552), bitcast<f32>(_e3557)), vec4<f32>(bitcast<f32>(_e3563), bitcast<f32>(_e3568), bitcast<f32>(_e3573), bitcast<f32>(_e3578)), bitcast<f32>(_e3584));
                                            } else {
                                                phi_2563_ = type_36(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e3588 = phi_2563_;
                                            let _e3610 = fma(_e2112.x, _e3588.member.z, fma(_e2110.x, _e3588.member.y, (_e2108.x * _e3588.member.x)));
                                            let _e3611 = fma(_e2112.y, _e3588.member.z, fma(_e2110.y, _e3588.member.y, (_e2108.y * _e3588.member.x)));
                                            let _e3612 = fma(_e2112.z, _e3588.member.z, fma(_e2110.z, _e3588.member.y, (_e2108.z * _e3588.member.x)));
                                            let _e3617 = sqrt(fma(_e3612, _e3612, fma(_e3610, _e3610, (_e3611 * _e3611))));
                                            if (_e3617 == 0f) {
                                                phi_8341_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_8341_ = (vec3<f32>(_e3610, _e3611, _e3612) * (1f / _e3617));
                                            }
                                            let _e3622 = phi_8341_;
                                            let _e3624 = -(_e3622.x);
                                            let _e3626 = -(_e3622.y);
                                            let _e3628 = -(_e3622.z);
                                            let _e3629 = -(_e3622);
                                            let _e3631 = fma(-(_e686.z), _e295.member_3, 1f);
                                            let _e3635 = fma(0.4f, _e3631, (_e1370 * _e1382));
                                            let _e3636 = fma(0.4f, _e3631, (_e1372 * _e1382));
                                            let _e3637 = fma(0.4f, _e3631, (_e1374 * _e1382));
                                            let _e3645 = (_e1938 + vec3<f32>(_e3624, _e3626, _e3628));
                                            let _e3652 = sqrt(fma(_e3645.z, _e3645.z, fma(_e3645.x, _e3645.x, (_e3645.y * _e3645.y))));
                                            if (_e3652 == 0f) {
                                                phi_8376_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_8376_ = (_e3645 * (1f / _e3652));
                                            }
                                            let _e3657 = phi_8376_;
                                            let _e3658 = (_e1379 * _e1379);
                                            let _e3669 = max(fma(_e1934.z, _e3657.z, fma(_e1934.x, _e3657.x, (_e1934.y * _e3657.y))), 0f);
                                            let _e3682 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                            let _e3688 = fma(_e1934.z, _e3629.z, fma(_e1934.x, _e3629.x, (_e1934.y * _e3629.y)));
                                            let _e3689 = max(_e3688, 0f);
                                            let _e3690 = fma(_e686.y, _e295.member_4, 1f);
                                            let _e3691 = (_e3690 * _e3690);
                                            let _e3692 = (_e3691 * 0.125f);
                                            let _e3694 = fma(-(_e3691), 0.125f, 1f);
                                            let _e3707 = (1f - max(fma(_e3657.z, _e1938.z, fma(_e3657.x, _e1938.x, (_e3657.y * _e1938.y))), 0f));
                                            let _e3709 = select(_e3707, 0f, (_e3707 < 0f));
                                            let _e3712 = pow(select(_e3709, 1f, (_e3709 > 1f)), 5f);
                                            let _e3713 = fma((1f - _e3635), _e3712, _e3635);
                                            let _e3714 = fma((1f - _e3636), _e3712, _e3636);
                                            let _e3715 = fma((1f - _e3637), _e3712, _e3637);
                                            let _e3722 = (((_e3658 * _e3658) / (pow(fma((_e3669 * _e3669), fma(_e3658, _e3658, -1f), 1f), 2f) * 3.1415927f)) * ((_e3682 / fma(_e3682, _e3694, _e3692)) * (_e3689 / fma(_e3689, _e3694, _e3692))));
                                            let _e3729 = max(fma(_e1934.z, _e3628, fma(_e1934.x, _e3624, (_e1934.y * _e3626))), 0f);
                                            let _e3731 = fma((4f * _e3682), _e3729, 0.0001f);
                                            if ((_e2012.member_3 == 4294967295u) != true) {
                                                let _e3753 = global_2.member[_e2012.member_3];
                                                let _e3757 = global_2.member[(_e2012.member_3 + 1u)];
                                                let _e3761 = global_2.member[(_e2012.member_3 + 4u)];
                                                let _e3765 = global_2.member[(_e2012.member_3 + 5u)];
                                                let _e3769 = global_2.member[(_e2012.member_3 + 6u)];
                                                let _e3774 = global_2.member[(_e2012.member_3 + 7u)];
                                                let _e3779 = global_2.member[(_e2012.member_3 + 8u)];
                                                let _e3782 = global_2.member[_e1947];
                                                let _e3786 = global_2.member[(_e1947 + 1u)];
                                                let _e3788 = select(_e3753, 4294967295u, (0u >= _e3757));
                                                let _e3791 = global_2.member[_e3788];
                                                let _e3796 = global_2.member[(_e3788 + 1u)];
                                                let _e3801 = global_2.member[(_e3788 + 2u)];
                                                let _e3806 = global_2.member[(_e3788 + 3u)];
                                                let _e3811 = global_2.member[(_e3788 + 4u)];
                                                let _e3816 = global_2.member[(_e3788 + 5u)];
                                                let _e3821 = global_2.member[(_e3788 + 6u)];
                                                let _e3826 = global_2.member[(_e3788 + 7u)];
                                                let _e3831 = global_2.member[(_e3788 + 8u)];
                                                let _e3836 = global_2.member[(_e3788 + 9u)];
                                                let _e3841 = global_2.member[(_e3788 + 10u)];
                                                let _e3846 = global_2.member[(_e3788 + 11u)];
                                                let _e3851 = global_2.member[(_e3788 + 12u)];
                                                let _e3856 = global_2.member[(_e3788 + 13u)];
                                                let _e3861 = global_2.member[(_e3788 + 14u)];
                                                let _e3866 = global_2.member[(_e3788 + 15u)];
                                                let _e3886 = (bitcast<f32>(_e3866) + fma(bitcast<f32>(_e3846), _e158.z, fma(bitcast<f32>(_e3826), _e158.y, (bitcast<f32>(_e3806) * _e158.x))));
                                                let _e3887 = ((bitcast<f32>(_e3851) + fma(bitcast<f32>(_e3831), _e158.z, fma(bitcast<f32>(_e3811), _e158.y, (bitcast<f32>(_e3791) * _e158.x)))) / _e3886);
                                                let _e3888 = ((bitcast<f32>(_e3856) + fma(bitcast<f32>(_e3836), _e158.z, fma(bitcast<f32>(_e3816), _e158.y, (bitcast<f32>(_e3796) * _e158.x)))) / _e3886);
                                                let _e3889 = ((bitcast<f32>(_e3861) + fma(bitcast<f32>(_e3841), _e158.z, fma(bitcast<f32>(_e3821), _e158.y, (bitcast<f32>(_e3801) * _e158.x)))) / _e3886);
                                                if (abs(_e3887) <= 1f) {
                                                    let _e3893 = (abs(_e3888) <= 1f);
                                                    if _e3893 {
                                                        phi_8481_ = (abs(_e3889) <= 1f);
                                                    } else {
                                                        phi_8481_ = bool();
                                                    }
                                                    let _e3897 = phi_8481_;
                                                    phi_8484_ = _e3897;
                                                    phi_8485_ = select(true, false, _e3893);
                                                } else {
                                                    phi_8484_ = bool();
                                                    phi_8485_ = true;
                                                }
                                                let _e3900 = phi_8484_;
                                                let _e3902 = phi_8485_;
                                                if select(_e3900, false, _e3902) {
                                                    let _e3910 = global_2.member[select(_e3761, 4294967295u, (0u >= _e3765))];
                                                    let _e3913 = global_2.member[_e3910];
                                                    let _e3917 = global_2.member[(_e3910 + 1u)];
                                                    let _e3921 = global_2.member[(_e3910 + 2u)];
                                                    let _e3925 = global_2.member[(_e3910 + 3u)];
                                                    let _e3929 = global_2.member[(_e3910 + 4u)];
                                                    let _e3933 = global_2.member[(_e3910 + 6u)];
                                                    switch bitcast<i32>(_e3933) {
                                                        case 0: {
                                                            phi_2964_ = 0u;
                                                            break;
                                                        }
                                                        case 1: {
                                                            phi_2964_ = 1u;
                                                            break;
                                                        }
                                                        case 2: {
                                                            phi_2964_ = 2u;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_2964_ = 0u;
                                                            break;
                                                        }
                                                    }
                                                    let _e3936 = phi_2964_;
                                                    let _e3940 = global_2.member[(_e3910 + 7u)];
                                                    switch bitcast<i32>(_e3940) {
                                                        case 0: {
                                                            phi_2973_ = 0u;
                                                            break;
                                                        }
                                                        case 1: {
                                                            phi_2973_ = 1u;
                                                            break;
                                                        }
                                                        case 2: {
                                                            phi_2973_ = 2u;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_2973_ = 0u;
                                                            break;
                                                        }
                                                    }
                                                    let _e3943 = phi_2973_;
                                                    let _e3944 = bitcast<i32>(_e3779);
                                                    let _e3946 = f32(_e3921);
                                                    let _e3947 = f32(_e3925);
                                                    let _e3951 = type_37((_e3944 / -2i), (_e3944 / 2i), false);
                                                    phi_8958_ = _e1950;
                                                    phi_3001_ = _e3951;
                                                    phi_3004_ = 0f;
                                                    phi_3006_ = 0f;
                                                    loop {
                                                        let _e3953 = phi_8958_;
                                                        let _e3955 = phi_3001_;
                                                        let _e3957 = phi_3004_;
                                                        let _e3959 = phi_3006_;
                                                        local_6 = _e3957;
                                                        local_7 = _e3959;
                                                        if _e3955.member_2 {
                                                            phi_3018_ = true;
                                                        } else {
                                                            phi_3018_ = ((_e3955.member <= _e3955.member_1) != true);
                                                        }
                                                        let _e3966 = phi_3018_;
                                                        if _e3966 {
                                                            phi_3002_ = _e3955;
                                                            phi_3061_ = type_38(0u, type_38().member_1);
                                                        } else {
                                                            if (_e3955.member < _e3955.member_1) {
                                                                let _e3974 = (_e3955.member + 1i);
                                                                if select(false, true, ((false == (_e3974 > _e3955.member)) != false)) {
                                                                    phi_3046_ = type_38(0u, type_38().member_1);
                                                                } else {
                                                                    phi_3046_ = type_38(1u, _e3974);
                                                                }
                                                                let _e3984 = phi_3046_;
                                                                switch bitcast<i32>(_e3984.member) {
                                                                    case 0: {
                                                                        phi_9003_ = true;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        break;
                                                                    }
                                                                }
                                                                phi_3058_ = type_37(_e3984.member_1, _e3955.member_1, _e3955.member_2);
                                                            } else {
                                                                phi_3058_ = type_37(_e3955.member, _e3955.member_1, true);
                                                            }
                                                            let _e3993 = phi_3058_;
                                                            phi_3002_ = _e3993;
                                                            phi_3061_ = type_38(1u, _e3955.member);
                                                        }
                                                        let _e3999 = phi_3002_;
                                                        let _e4001 = phi_3061_;
                                                        switch bitcast<i32>(_e4001.member) {
                                                            case 0: {
                                                                phi_9004_ = _e3953;
                                                                phi_3005_ = f32();
                                                                phi_3007_ = f32();
                                                                phi_3319_ = false;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3072_ = _e3951;
                                                                phi_3075_ = _e3957;
                                                                phi_3077_ = _e3959;
                                                                loop {
                                                                    let _e4006 = phi_3072_;
                                                                    let _e4008 = phi_3075_;
                                                                    let _e4010 = phi_3077_;
                                                                    local_13 = _e4008;
                                                                    local_14 = _e4010;
                                                                    if _e4006.member_2 {
                                                                        phi_3089_ = true;
                                                                    } else {
                                                                        phi_3089_ = ((_e4006.member <= _e4006.member_1) != true);
                                                                    }
                                                                    let _e4017 = phi_3089_;
                                                                    if _e4017 {
                                                                        phi_3073_ = _e4006;
                                                                        phi_3132_ = type_38(0u, type_38().member_1);
                                                                    } else {
                                                                        if (_e4006.member < _e4006.member_1) {
                                                                            let _e4025 = (_e4006.member + 1i);
                                                                            if select(false, true, ((false == (_e4025 > _e4006.member)) != false)) {
                                                                                phi_3117_ = type_38(0u, type_38().member_1);
                                                                            } else {
                                                                                phi_3117_ = type_38(1u, _e4025);
                                                                            }
                                                                            let _e4035 = phi_3117_;
                                                                            switch bitcast<i32>(_e4035.member) {
                                                                                case 0: {
                                                                                    phi_8942_ = true;
                                                                                    break;
                                                                                }
                                                                                case 1: {
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            phi_3129_ = type_37(_e4035.member_1, _e4006.member_1, _e4006.member_2);
                                                                        } else {
                                                                            phi_3129_ = type_37(_e4006.member, _e4006.member_1, true);
                                                                        }
                                                                        let _e4044 = phi_3129_;
                                                                        phi_3073_ = _e4044;
                                                                        phi_3132_ = type_38(1u, _e4006.member);
                                                                    }
                                                                    let _e4050 = phi_3073_;
                                                                    let _e4052 = phi_3132_;
                                                                    switch bitcast<i32>(_e4052.member) {
                                                                        case 0: {
                                                                            phi_3076_ = f32();
                                                                            phi_3078_ = f32();
                                                                            phi_3318_ = false;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            let _e4060 = fma((_e3887 + 1f), 0.5f, (f32(_e4001.member_1) * (1f / _e3946)));
                                                                            let _e4061 = fma(fma(_e3888, -1f, 1f), 0.5f, (f32(_e4052.member_1) * (1f / _e3947)));
                                                                            switch bitcast<i32>(_e3936) {
                                                                                case 1: {
                                                                                    let _e4096 = abs(_e4060);
                                                                                    let _e4098 = (_e4096 % 1f);
                                                                                    if (_e4096 >= 1f) {
                                                                                        phi_8537_ = select(true, false, (_e4098 == 0f));
                                                                                    } else {
                                                                                        phi_8537_ = true;
                                                                                    }
                                                                                    let _e4102 = phi_8537_;
                                                                                    let _e4103 = select(1f, _e4098, _e4102);
                                                                                    if (select(-1f, 1f, (_e4060 >= 0f)) > 0f) {
                                                                                        phi_3164_ = _e4103;
                                                                                    } else {
                                                                                        phi_3164_ = (1f - _e4103);
                                                                                    }
                                                                                    let _e4107 = phi_3164_;
                                                                                    phi_3201_ = _e4107;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e4070 = abs(_e4060);
                                                                                    let _e4077 = ((select(select(u32(_e4070), 0u, (_e4070 < 0f)), 4294967295u, (_e4070 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e4079 = (_e4070 % 1f);
                                                                                    if (_e4070 >= 1f) {
                                                                                        phi_8520_ = select(true, false, (_e4079 == 0f));
                                                                                    } else {
                                                                                        phi_8520_ = true;
                                                                                    }
                                                                                    let _e4083 = phi_8520_;
                                                                                    let _e4084 = select(1f, _e4079, _e4083);
                                                                                    if (select(-1f, 1f, (_e4060 >= 0f)) > 0f) {
                                                                                        if _e4077 {
                                                                                            phi_3193_ = _e4084;
                                                                                        } else {
                                                                                            phi_3193_ = (1f - _e4084);
                                                                                        }
                                                                                        let _e4091 = phi_3193_;
                                                                                        phi_3199_ = _e4091;
                                                                                    } else {
                                                                                        if _e4077 {
                                                                                            phi_3198_ = (1f - _e4084);
                                                                                        } else {
                                                                                            phi_3198_ = _e4084;
                                                                                        }
                                                                                        let _e4088 = phi_3198_;
                                                                                        phi_3199_ = _e4088;
                                                                                    }
                                                                                    let _e4093 = phi_3199_;
                                                                                    phi_3201_ = _e4093;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e4060 > 1f) {
                                                                                        phi_8507_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_8507_ = select(_e4060, 0.00000011920929f, (_e4060 < 0f));
                                                                                    }
                                                                                    let _e4067 = phi_8507_;
                                                                                    phi_3201_ = _e4067;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3201_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e4109 = phi_3201_;
                                                                            switch bitcast<i32>(_e3943) {
                                                                                case 1: {
                                                                                    let _e4144 = abs(_e4061);
                                                                                    let _e4146 = (_e4144 % 1f);
                                                                                    if (_e4144 >= 1f) {
                                                                                        phi_8585_ = select(true, false, (_e4146 == 0f));
                                                                                    } else {
                                                                                        phi_8585_ = true;
                                                                                    }
                                                                                    let _e4150 = phi_8585_;
                                                                                    let _e4151 = select(1f, _e4146, _e4150);
                                                                                    if (select(-1f, 1f, (_e4061 >= 0f)) > 0f) {
                                                                                        phi_3220_ = _e4151;
                                                                                    } else {
                                                                                        phi_3220_ = (1f - _e4151);
                                                                                    }
                                                                                    let _e4155 = phi_3220_;
                                                                                    phi_3257_ = _e4155;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e4118 = abs(_e4061);
                                                                                    let _e4125 = ((select(select(u32(_e4118), 0u, (_e4118 < 0f)), 4294967295u, (_e4118 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e4127 = (_e4118 % 1f);
                                                                                    if (_e4118 >= 1f) {
                                                                                        phi_8568_ = select(true, false, (_e4127 == 0f));
                                                                                    } else {
                                                                                        phi_8568_ = true;
                                                                                    }
                                                                                    let _e4131 = phi_8568_;
                                                                                    let _e4132 = select(1f, _e4127, _e4131);
                                                                                    if (select(-1f, 1f, (_e4061 >= 0f)) > 0f) {
                                                                                        if _e4125 {
                                                                                            phi_3249_ = _e4132;
                                                                                        } else {
                                                                                            phi_3249_ = (1f - _e4132);
                                                                                        }
                                                                                        let _e4139 = phi_3249_;
                                                                                        phi_3255_ = _e4139;
                                                                                    } else {
                                                                                        if _e4125 {
                                                                                            phi_3254_ = (1f - _e4132);
                                                                                        } else {
                                                                                            phi_3254_ = _e4132;
                                                                                        }
                                                                                        let _e4136 = phi_3254_;
                                                                                        phi_3255_ = _e4136;
                                                                                    }
                                                                                    let _e4141 = phi_3255_;
                                                                                    phi_3257_ = _e4141;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e4061 > 1f) {
                                                                                        phi_8555_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_8555_ = select(_e4061, 0.00000011920929f, (_e4061 < 0f));
                                                                                    }
                                                                                    let _e4115 = phi_8555_;
                                                                                    phi_3257_ = _e4115;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3257_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e4157 = phi_3257_;
                                                                            let _e4158 = (_e4109 * _e3946);
                                                                            let _e4164 = (_e4157 * _e3947);
                                                                            let _e4179 = vec3<f32>((f32((select(select(u32(_e4158), 0u, (_e4158 < 0f)), 4294967295u, (_e4158 > 4294967000f)) + _e3913)) / f32(_e3782)), (f32((select(select(u32(_e4164), 0u, (_e4164 < 0f)), 4294967295u, (_e4164 > 4294967000f)) + _e3917)) / f32(_e3786)), f32(_e3929));
                                                                            let _e4185 = textureSampleLevel(global_20, global_19, vec2<f32>(_e4179.x, _e4179.y), i32(_e4179.z), 0f);
                                                                            if ((_e3889 - max((bitcast<f32>(_e3774) * (1f - _e3688)), bitcast<f32>(_e3769))) > _e4185.x) {
                                                                                phi_3316_ = (_e4010 + 1f);
                                                                            } else {
                                                                                phi_3316_ = _e4010;
                                                                            }
                                                                            let _e4194 = phi_3316_;
                                                                            phi_3076_ = (_e4008 + 1f);
                                                                            phi_3078_ = _e4194;
                                                                            phi_3318_ = true;
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            phi_3076_ = f32();
                                                                            phi_3078_ = f32();
                                                                            phi_3318_ = bool();
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e4197 = phi_3076_;
                                                                    let _e4199 = phi_3078_;
                                                                    let _e4201 = phi_3318_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_3072_ = _e4050;
                                                                        phi_3075_ = _e4197;
                                                                        phi_3077_ = _e4199;
                                                                        phi_8942_ = _e3953;
                                                                        break if !(_e4201);
                                                                    }
                                                                }
                                                                let _e4204 = phi_8942_;
                                                                phi_9003_ = _e4204;
                                                                if _e4204 {
                                                                    break;
                                                                }
                                                                phi_9004_ = _e4204;
                                                                let _e4655 = local_13;
                                                                phi_3005_ = _e4655;
                                                                let _e4658 = local_14;
                                                                phi_3007_ = _e4658;
                                                                phi_3319_ = true;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_9004_ = _e3953;
                                                                phi_3005_ = f32();
                                                                phi_3007_ = f32();
                                                                phi_3319_ = bool();
                                                                break;
                                                            }
                                                        }
                                                        let _e4206 = phi_9004_;
                                                        let _e4208 = phi_3005_;
                                                        let _e4210 = phi_3007_;
                                                        let _e4212 = phi_3319_;
                                                        continue;
                                                        continuing {
                                                            phi_8958_ = _e4206;
                                                            phi_3001_ = _e3999;
                                                            phi_3004_ = _e4208;
                                                            phi_3006_ = _e4210;
                                                            phi_9003_ = _e4206;
                                                            break if !(_e4212);
                                                        }
                                                    }
                                                    let _e4215 = phi_9003_;
                                                    phi_9005_ = _e4215;
                                                    if _e4215 {
                                                        break;
                                                    }
                                                    let _e4217 = local_6;
                                                    let _e4220 = local_7;
                                                    phi_9016_ = _e4215;
                                                    phi_3322_ = (_e4220 / max(_e4217, 1f));
                                                } else {
                                                    phi_9016_ = _e1950;
                                                    phi_3322_ = 0f;
                                                }
                                                let _e4223 = phi_9016_;
                                                let _e4225 = phi_3322_;
                                                phi_9015_ = _e4223;
                                                phi_3323_ = _e4225;
                                            } else {
                                                phi_9015_ = _e1950;
                                                phi_3323_ = 0f;
                                            }
                                            let _e4227 = phi_9015_;
                                            let _e4229 = phi_3323_;
                                            phi_9009_ = _e4227;
                                            phi_4966_ = _e4229;
                                            phi_4967_ = vec3<f32>(((fma((((1f - _e3713) * _e3631) * _e1370), 0.31830987f, ((_e3722 * _e3713) / _e3731)) * (_e3588.member_1.x * _e3588.member_2)) * _e3729), ((fma((((1f - _e3714) * _e3631) * _e1372), 0.31830987f, ((_e3722 * _e3714) / _e3731)) * (_e3588.member_1.y * _e3588.member_2)) * _e3729), ((fma((((1f - _e3715) * _e3631) * _e1374), 0.31830987f, ((_e3722 * _e3715) / _e3731)) * (_e3588.member_1.z * _e3588.member_2)) * _e3729));
                                            phi_4968_ = true;
                                            break;
                                        }
                                        case 1: {
                                            if (_e150 >= 8u) {
                                                phi_7841_ = (_e2012.member_1 <= (_e150 - 8u));
                                            } else {
                                                phi_7841_ = false;
                                            }
                                            let _e2898 = phi_7841_;
                                            if _e2898 {
                                                let _e2901 = global_2.member[_e2012.member_1];
                                                let _e2906 = global_2.member[(_e2012.member_1 + 1u)];
                                                let _e2911 = global_2.member[(_e2012.member_1 + 2u)];
                                                let _e2917 = global_2.member[(_e2012.member_1 + 3u)];
                                                let _e2922 = global_2.member[(_e2012.member_1 + 4u)];
                                                let _e2927 = global_2.member[(_e2012.member_1 + 5u)];
                                                let _e2932 = global_2.member[(_e2012.member_1 + 6u)];
                                                let _e2938 = global_2.member[(_e2012.member_1 + 7u)];
                                                phi_3364_ = type_36(vec3<f32>(bitcast<f32>(_e2901), bitcast<f32>(_e2906), bitcast<f32>(_e2911)), vec4<f32>(bitcast<f32>(_e2917), bitcast<f32>(_e2922), bitcast<f32>(_e2927), bitcast<f32>(_e2932)), bitcast<f32>(_e2938));
                                            } else {
                                                phi_3364_ = type_36(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2942 = phi_3364_;
                                            let _e2967 = (_e2073.member.x + fma(_e2112.x, _e2942.member.z, fma(_e2110.x, _e2942.member.y, (_e2108.x * _e2942.member.x))));
                                            let _e2968 = (_e2073.member.y + fma(_e2112.y, _e2942.member.z, fma(_e2110.y, _e2942.member.y, (_e2108.y * _e2942.member.x))));
                                            let _e2969 = (_e2073.member.z + fma(_e2112.z, _e2942.member.z, fma(_e2110.z, _e2942.member.y, (_e2108.z * _e2942.member.x))));
                                            let _e2971 = (vec3<f32>(_e2967, _e2968, _e2969) - _e158);
                                            let _e2978 = sqrt(fma(_e2971.z, _e2971.z, fma(_e2971.x, _e2971.x, (_e2971.y * _e2971.y))));
                                            let _e2979 = (_e2978 == 0f);
                                            if _e2979 {
                                                phi_9013_ = _e1950;
                                                phi_4031_ = f32();
                                                phi_4032_ = vec3<f32>();
                                            } else {
                                                if _e2979 {
                                                    phi_7888_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7888_ = (_e2971 * (1f / _e2978));
                                                }
                                                let _e2983 = phi_7888_;
                                                let _e2985 = (_e2942.member_2 / (_e2978 * _e2978));
                                                let _e2987 = fma(-(_e686.z), _e295.member_3, 1f);
                                                let _e2991 = fma(0.4f, _e2987, (_e1370 * _e1382));
                                                let _e2992 = fma(0.4f, _e2987, (_e1372 * _e1382));
                                                let _e2993 = fma(0.4f, _e2987, (_e1374 * _e1382));
                                                let _e3000 = (_e1938 + _e2983);
                                                let _e3007 = sqrt(fma(_e3000.z, _e3000.z, fma(_e3000.x, _e3000.x, (_e3000.y * _e3000.y))));
                                                if (_e3007 == 0f) {
                                                    phi_7923_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7923_ = (_e3000 * (1f / _e3007));
                                                }
                                                let _e3012 = phi_7923_;
                                                let _e3013 = (_e1379 * _e1379);
                                                let _e3024 = max(fma(_e1934.z, _e3012.z, fma(_e1934.x, _e3012.x, (_e1934.y * _e3012.y))), 0f);
                                                let _e3037 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                                let _e3043 = fma(_e1934.z, _e2983.z, fma(_e1934.x, _e2983.x, (_e1934.y * _e2983.y)));
                                                let _e3044 = max(_e3043, 0f);
                                                let _e3045 = fma(_e686.y, _e295.member_4, 1f);
                                                let _e3046 = (_e3045 * _e3045);
                                                let _e3047 = (_e3046 * 0.125f);
                                                let _e3049 = fma(-(_e3046), 0.125f, 1f);
                                                let _e3062 = (1f - max(fma(_e3012.z, _e1938.z, fma(_e3012.x, _e1938.x, (_e3012.y * _e1938.y))), 0f));
                                                let _e3064 = select(_e3062, 0f, (_e3062 < 0f));
                                                let _e3067 = pow(select(_e3064, 1f, (_e3064 > 1f)), 5f);
                                                let _e3068 = fma((1f - _e2991), _e3067, _e2991);
                                                let _e3069 = fma((1f - _e2992), _e3067, _e2992);
                                                let _e3070 = fma((1f - _e2993), _e3067, _e2993);
                                                let _e3077 = (((_e3013 * _e3013) / (pow(fma((_e3024 * _e3024), fma(_e3013, _e3013, -1f), 1f), 2f) * 3.1415927f)) * ((_e3037 / fma(_e3037, _e3049, _e3047)) * (_e3044 / fma(_e3044, _e3049, _e3047))));
                                                let _e3082 = fma((4f * _e3037), _e3044, 0.0001f);
                                                if ((_e2012.member_3 == 4294967295u) != true) {
                                                    let _e3104 = global_2.member[_e2012.member_3];
                                                    let _e3108 = global_2.member[(_e2012.member_3 + 1u)];
                                                    let _e3112 = global_2.member[(_e2012.member_3 + 3u)];
                                                    let _e3117 = global_2.member[(_e2012.member_3 + 4u)];
                                                    let _e3121 = global_2.member[(_e2012.member_3 + 5u)];
                                                    let _e3125 = global_2.member[(_e2012.member_3 + 6u)];
                                                    let _e3130 = global_2.member[(_e2012.member_3 + 7u)];
                                                    let _e3135 = global_2.member[(_e2012.member_3 + 8u)];
                                                    let _e3138 = global_2.member[_e1947];
                                                    let _e3142 = global_2.member[(_e1947 + 1u)];
                                                    let _e3144 = (_e158.x - _e2967);
                                                    let _e3146 = (_e158.y - _e2968);
                                                    let _e3148 = (_e158.z - _e2969);
                                                    let _e3151 = min(max(f32(_e3135), 1f), 21f);
                                                    let _e3163 = ((1f + (sqrt(fma(_e3148, _e3148, fma(_e3144, _e3144, (_e3146 * _e3146)))) / bitcast<f32>(_e3112))) * 0.04f);
                                                    phi_3643_ = type_14(0u, select(select(u32(_e3151), 0u, (_e3151 < 0f)), 4294967295u, (_e3151 > 4294967000f)));
                                                    phi_3646_ = 0f;
                                                    loop {
                                                        let _e3166 = phi_3643_;
                                                        let _e3168 = phi_3646_;
                                                        local_5 = _e3168;
                                                        if (_e3166.member < _e3166.member_1) {
                                                            phi_3644_ = type_14((_e3166.member + 1u), _e3166.member_1);
                                                            phi_3669_ = type_14(1u, _e3166.member);
                                                        } else {
                                                            phi_3644_ = _e3166;
                                                            phi_3669_ = type_14(0u, type_14().member_1);
                                                        }
                                                        let _e3181 = phi_3644_;
                                                        let _e3183 = phi_3669_;
                                                        switch bitcast<i32>(_e3183.member) {
                                                            case 0: {
                                                                phi_3647_ = f32();
                                                                phi_4028_ = false;
                                                                break;
                                                            }
                                                            case 1: {
                                                                local = array<vec3<f32>, 21>(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3<f32>(1f, -1f, 1f), vec3<f32>(-1f, -1f, 1f), vec3<f32>(-1f, 1f, 1f), vec3<f32>(1f, 1f, -1f), vec3<f32>(1f, -1f, -1f), vec3<f32>(-1f, -1f, -1f), vec3<f32>(-1f, 1f, -1f), vec3<f32>(1f, 1f, 0f), vec3<f32>(1f, -1f, 0f), vec3<f32>(-1f, -1f, 0f), vec3<f32>(-1f, 1f, 0f), vec3<f32>(1f, 0f, 1f), vec3<f32>(-1f, 0f, 1f), vec3<f32>(1f, 0f, -1f), vec3<f32>(-1f, 0f, -1f), vec3<f32>(0f, 1f, 1f), vec3<f32>(0f, -1f, 1f), vec3<f32>(0f, -1f, -1f), vec3<f32>(0f, 1f, -1f));
                                                                if (_e3183.member_1 < 21u) {
                                                                } else {
                                                                    phi_8886_ = true;
                                                                    break;
                                                                }
                                                                let _e3189 = local[_e3183.member_1];
                                                                let _e3193 = fma(_e3189.x, _e3163, _e3144);
                                                                let _e3194 = fma(_e3189.y, _e3163, _e3146);
                                                                let _e3195 = fma(_e3189.z, _e3163, _e3148);
                                                                let _e3200 = sqrt(fma(_e3195, _e3195, fma(_e3193, _e3193, (_e3194 * _e3194))));
                                                                if (_e3200 == 0f) {
                                                                    phi_8044_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    phi_8044_ = (vec3<f32>(_e3193, _e3194, _e3195) * (1f / _e3200));
                                                                }
                                                                let _e3205 = phi_8044_;
                                                                let _e3207 = abs(_e3205.x);
                                                                let _e3209 = abs(_e3205.y);
                                                                let _e3211 = abs(_e3205.z);
                                                                if (_e3207 >= _e3209) {
                                                                    let _e3213 = (_e3207 >= _e3211);
                                                                    if _e3213 {
                                                                        let _e3214 = (_e3205.x > 0f);
                                                                        if _e3214 {
                                                                            phi_8078_ = vec2<f32>((-(_e3205.z) / _e3207), (-(_e3205.y) / _e3207));
                                                                        } else {
                                                                            phi_8078_ = vec2<f32>((_e3205.z / _e3207), (-(_e3205.y) / _e3207));
                                                                        }
                                                                        let _e3225 = phi_8078_;
                                                                        phi_8081_ = _e3225;
                                                                        phi_8082_ = select(1u, 0u, _e3214);
                                                                    } else {
                                                                        phi_8081_ = vec2<f32>();
                                                                        phi_8082_ = u32();
                                                                    }
                                                                    let _e3228 = phi_8081_;
                                                                    let _e3230 = phi_8082_;
                                                                    phi_8085_ = _e3228;
                                                                    phi_8086_ = _e3230;
                                                                    phi_8087_ = select(true, false, _e3213);
                                                                } else {
                                                                    phi_8085_ = vec2<f32>();
                                                                    phi_8086_ = u32();
                                                                    phi_8087_ = true;
                                                                }
                                                                let _e3233 = phi_8085_;
                                                                let _e3235 = phi_8086_;
                                                                let _e3237 = phi_8087_;
                                                                if _e3237 {
                                                                    if (_e3209 >= _e3207) {
                                                                        let _e3239 = (_e3209 >= _e3211);
                                                                        if _e3239 {
                                                                            let _e3240 = (_e3205.y > 0f);
                                                                            if _e3240 {
                                                                                phi_8112_ = vec2<f32>((_e3205.x / _e3209), (_e3205.z / _e3209));
                                                                            } else {
                                                                                phi_8112_ = vec2<f32>((_e3205.x / _e3209), (-(_e3205.z) / _e3209));
                                                                            }
                                                                            let _e3249 = phi_8112_;
                                                                            phi_8115_ = _e3249;
                                                                            phi_8116_ = select(3u, 2u, _e3240);
                                                                        } else {
                                                                            phi_8115_ = vec2<f32>();
                                                                            phi_8116_ = u32();
                                                                        }
                                                                        let _e3252 = phi_8115_;
                                                                        let _e3254 = phi_8116_;
                                                                        phi_8119_ = _e3252;
                                                                        phi_8120_ = _e3254;
                                                                        phi_8121_ = select(true, false, _e3239);
                                                                    } else {
                                                                        phi_8119_ = vec2<f32>();
                                                                        phi_8120_ = u32();
                                                                        phi_8121_ = true;
                                                                    }
                                                                    let _e3257 = phi_8119_;
                                                                    let _e3259 = phi_8120_;
                                                                    let _e3261 = phi_8121_;
                                                                    if _e3261 {
                                                                        let _e3262 = (_e3205.z > 0f);
                                                                        if _e3262 {
                                                                            phi_8142_ = vec2<f32>((_e3205.x / _e3211), (-(_e3205.y) / _e3211));
                                                                        } else {
                                                                            phi_8142_ = vec2<f32>((-(_e3205.x) / _e3211), (-(_e3205.y) / _e3211));
                                                                        }
                                                                        let _e3273 = phi_8142_;
                                                                        phi_8145_ = _e3273;
                                                                        phi_8146_ = select(5u, 4u, _e3262);
                                                                    } else {
                                                                        phi_8145_ = _e3257;
                                                                        phi_8146_ = _e3259;
                                                                    }
                                                                    let _e3276 = phi_8145_;
                                                                    let _e3278 = phi_8146_;
                                                                    phi_8148_ = _e3276;
                                                                    phi_8149_ = _e3278;
                                                                } else {
                                                                    phi_8148_ = _e3233;
                                                                    phi_8149_ = _e3235;
                                                                }
                                                                let _e3280 = phi_8148_;
                                                                let _e3282 = phi_8149_;
                                                                let _e3287 = ((_e3280.x + 1f) * 0.5f);
                                                                let _e3288 = ((_e3280.y + 1f) * 0.5f);
                                                                if (_e3282 >= _e3108) {
                                                                    phi_3706_ = 4294967295u;
                                                                } else {
                                                                    phi_3706_ = (_e3104 + (16u * _e3282));
                                                                }
                                                                let _e3293 = phi_3706_;
                                                                let _e3297 = global_2.member[(_e3293 + 2u)];
                                                                let _e3302 = global_2.member[(_e3293 + 3u)];
                                                                let _e3307 = global_2.member[(_e3293 + 6u)];
                                                                let _e3312 = global_2.member[(_e3293 + 7u)];
                                                                let _e3317 = global_2.member[(_e3293 + 10u)];
                                                                let _e3322 = global_2.member[(_e3293 + 11u)];
                                                                let _e3327 = global_2.member[(_e3293 + 14u)];
                                                                let _e3332 = global_2.member[(_e3293 + 15u)];
                                                                if (_e3282 >= _e3121) {
                                                                    phi_3807_ = 4294967295u;
                                                                } else {
                                                                    phi_3807_ = (_e3117 + _e3282);
                                                                }
                                                                let _e3346 = phi_3807_;
                                                                let _e3349 = global_2.member[_e3346];
                                                                let _e3352 = global_2.member[_e3349];
                                                                let _e3356 = global_2.member[(_e3349 + 1u)];
                                                                let _e3360 = global_2.member[(_e3349 + 2u)];
                                                                let _e3364 = global_2.member[(_e3349 + 3u)];
                                                                let _e3368 = global_2.member[(_e3349 + 4u)];
                                                                let _e3372 = global_2.member[(_e3349 + 6u)];
                                                                switch bitcast<i32>(_e3372) {
                                                                    case 0: {
                                                                        phi_3838_ = 0u;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        phi_3838_ = 1u;
                                                                        break;
                                                                    }
                                                                    case 2: {
                                                                        phi_3838_ = 2u;
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        phi_3838_ = 0u;
                                                                        break;
                                                                    }
                                                                }
                                                                let _e3375 = phi_3838_;
                                                                let _e3379 = global_2.member[(_e3349 + 7u)];
                                                                switch bitcast<i32>(_e3379) {
                                                                    case 0: {
                                                                        phi_3847_ = 0u;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        phi_3847_ = 1u;
                                                                        break;
                                                                    }
                                                                    case 2: {
                                                                        phi_3847_ = 2u;
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        phi_3847_ = 0u;
                                                                        break;
                                                                    }
                                                                }
                                                                let _e3382 = phi_3847_;
                                                                switch bitcast<i32>(_e3375) {
                                                                    case 1: {
                                                                        let _e3417 = abs(_e3287);
                                                                        let _e3419 = (_e3417 % 1f);
                                                                        if (_e3417 >= 1f) {
                                                                            phi_8209_ = select(true, false, (_e3419 == 0f));
                                                                        } else {
                                                                            phi_8209_ = true;
                                                                        }
                                                                        let _e3423 = phi_8209_;
                                                                        let _e3424 = select(1f, _e3419, _e3423);
                                                                        if (select(-1f, 1f, (_e3287 >= 0f)) > 0f) {
                                                                            phi_3873_ = _e3424;
                                                                        } else {
                                                                            phi_3873_ = (1f - _e3424);
                                                                        }
                                                                        let _e3428 = phi_3873_;
                                                                        phi_3910_ = _e3428;
                                                                        break;
                                                                    }
                                                                    case 2: {
                                                                        let _e3391 = abs(_e3287);
                                                                        let _e3398 = ((select(select(u32(_e3391), 0u, (_e3391 < 0f)), 4294967295u, (_e3391 > 4294967000f)) % 2u) == 0u);
                                                                        let _e3400 = (_e3391 % 1f);
                                                                        if (_e3391 >= 1f) {
                                                                            phi_8192_ = select(true, false, (_e3400 == 0f));
                                                                        } else {
                                                                            phi_8192_ = true;
                                                                        }
                                                                        let _e3404 = phi_8192_;
                                                                        let _e3405 = select(1f, _e3400, _e3404);
                                                                        if (select(-1f, 1f, (_e3287 >= 0f)) > 0f) {
                                                                            if _e3398 {
                                                                                phi_3902_ = _e3405;
                                                                            } else {
                                                                                phi_3902_ = (1f - _e3405);
                                                                            }
                                                                            let _e3412 = phi_3902_;
                                                                            phi_3908_ = _e3412;
                                                                        } else {
                                                                            if _e3398 {
                                                                                phi_3907_ = (1f - _e3405);
                                                                            } else {
                                                                                phi_3907_ = _e3405;
                                                                            }
                                                                            let _e3409 = phi_3907_;
                                                                            phi_3908_ = _e3409;
                                                                        }
                                                                        let _e3414 = phi_3908_;
                                                                        phi_3910_ = _e3414;
                                                                        break;
                                                                    }
                                                                    case 0: {
                                                                        if (_e3287 > 1f) {
                                                                            phi_8179_ = 0.9999999f;
                                                                        } else {
                                                                            phi_8179_ = select(_e3287, 0.00000011920929f, (_e3287 < 0f));
                                                                        }
                                                                        let _e3388 = phi_8179_;
                                                                        phi_3910_ = _e3388;
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        phi_3910_ = f32();
                                                                        break;
                                                                    }
                                                                }
                                                                let _e3430 = phi_3910_;
                                                                switch bitcast<i32>(_e3382) {
                                                                    case 1: {
                                                                        let _e3465 = abs(_e3288);
                                                                        let _e3467 = (_e3465 % 1f);
                                                                        if (_e3465 >= 1f) {
                                                                            phi_8257_ = select(true, false, (_e3467 == 0f));
                                                                        } else {
                                                                            phi_8257_ = true;
                                                                        }
                                                                        let _e3471 = phi_8257_;
                                                                        let _e3472 = select(1f, _e3467, _e3471);
                                                                        if (select(-1f, 1f, (_e3288 >= 0f)) > 0f) {
                                                                            phi_3931_ = _e3472;
                                                                        } else {
                                                                            phi_3931_ = (1f - _e3472);
                                                                        }
                                                                        let _e3476 = phi_3931_;
                                                                        phi_3968_ = _e3476;
                                                                        break;
                                                                    }
                                                                    case 2: {
                                                                        let _e3439 = abs(_e3288);
                                                                        let _e3446 = ((select(select(u32(_e3439), 0u, (_e3439 < 0f)), 4294967295u, (_e3439 > 4294967000f)) % 2u) == 0u);
                                                                        let _e3448 = (_e3439 % 1f);
                                                                        if (_e3439 >= 1f) {
                                                                            phi_8240_ = select(true, false, (_e3448 == 0f));
                                                                        } else {
                                                                            phi_8240_ = true;
                                                                        }
                                                                        let _e3452 = phi_8240_;
                                                                        let _e3453 = select(1f, _e3448, _e3452);
                                                                        if (select(-1f, 1f, (_e3288 >= 0f)) > 0f) {
                                                                            if _e3446 {
                                                                                phi_3960_ = _e3453;
                                                                            } else {
                                                                                phi_3960_ = (1f - _e3453);
                                                                            }
                                                                            let _e3460 = phi_3960_;
                                                                            phi_3966_ = _e3460;
                                                                        } else {
                                                                            if _e3446 {
                                                                                phi_3965_ = (1f - _e3453);
                                                                            } else {
                                                                                phi_3965_ = _e3453;
                                                                            }
                                                                            let _e3457 = phi_3965_;
                                                                            phi_3966_ = _e3457;
                                                                        }
                                                                        let _e3462 = phi_3966_;
                                                                        phi_3968_ = _e3462;
                                                                        break;
                                                                    }
                                                                    case 0: {
                                                                        if (_e3288 > 1f) {
                                                                            phi_8227_ = 0.9999999f;
                                                                        } else {
                                                                            phi_8227_ = select(_e3288, 0.00000011920929f, (_e3288 < 0f));
                                                                        }
                                                                        let _e3436 = phi_8227_;
                                                                        phi_3968_ = _e3436;
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        phi_3968_ = f32();
                                                                        break;
                                                                    }
                                                                }
                                                                let _e3478 = phi_3968_;
                                                                let _e3480 = (_e3430 * f32(_e3360));
                                                                let _e3487 = (_e3478 * f32(_e3364));
                                                                let _e3502 = vec3<f32>((f32((select(select(u32(_e3480), 0u, (_e3480 < 0f)), 4294967295u, (_e3480 > 4294967000f)) + _e3352)) / f32(_e3138)), (f32((select(select(u32(_e3487), 0u, (_e3487 < 0f)), 4294967295u, (_e3487 > 4294967000f)) + _e3356)) / f32(_e3142)), f32(_e3368));
                                                                let _e3508 = textureSampleLevel(global_20, global_19, vec2<f32>(_e3502.x, _e3502.y), i32(_e3502.z), 0f);
                                                                if ((((bitcast<f32>(_e3327) + fma(bitcast<f32>(_e3317), _e158.z, fma(bitcast<f32>(_e3307), _e158.y, (bitcast<f32>(_e3297) * _e158.x)))) / (bitcast<f32>(_e3332) + fma(bitcast<f32>(_e3322), _e158.z, fma(bitcast<f32>(_e3312), _e158.y, (bitcast<f32>(_e3302) * _e158.x))))) - max((bitcast<f32>(_e3130) * (1f - _e3043)), bitcast<f32>(_e3125))) > _e3508.x) {
                                                                    phi_4027_ = (_e3168 + 1f);
                                                                } else {
                                                                    phi_4027_ = _e3168;
                                                                }
                                                                let _e3517 = phi_4027_;
                                                                phi_3647_ = _e3517;
                                                                phi_4028_ = true;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_3647_ = f32();
                                                                phi_4028_ = bool();
                                                                break;
                                                            }
                                                        }
                                                        let _e3519 = phi_3647_;
                                                        let _e3521 = phi_4028_;
                                                        continue;
                                                        continuing {
                                                            phi_3643_ = _e3181;
                                                            phi_3646_ = _e3519;
                                                            phi_8886_ = _e1950;
                                                            break if !(_e3521);
                                                        }
                                                    }
                                                    let _e3524 = phi_8886_;
                                                    phi_9005_ = _e3524;
                                                    if _e3524 {
                                                        break;
                                                    }
                                                    let _e3526 = local_5;
                                                    phi_9014_ = _e3524;
                                                    phi_4030_ = (_e3526 / _e3151);
                                                } else {
                                                    phi_9014_ = _e1950;
                                                    phi_4030_ = 0f;
                                                }
                                                let _e3529 = phi_9014_;
                                                let _e3531 = phi_4030_;
                                                phi_9013_ = _e3529;
                                                phi_4031_ = _e3531;
                                                phi_4032_ = vec3<f32>(((fma((((1f - _e3068) * _e2987) * _e1370), 0.31830987f, ((_e3077 * _e3068) / _e3082)) * (_e2942.member_1.x * _e2985)) * _e3044), ((fma((((1f - _e3069) * _e2987) * _e1372), 0.31830987f, ((_e3077 * _e3069) / _e3082)) * (_e2942.member_1.y * _e2985)) * _e3044), ((fma((((1f - _e3070) * _e2987) * _e1374), 0.31830987f, ((_e3077 * _e3070) / _e3082)) * (_e2942.member_1.z * _e2985)) * _e3044));
                                            }
                                            let _e3533 = phi_9013_;
                                            let _e3535 = phi_4031_;
                                            let _e3537 = phi_4032_;
                                            phi_9009_ = _e3533;
                                            phi_4966_ = _e3535;
                                            phi_4967_ = _e3537;
                                            phi_4968_ = select(true, false, _e2979);
                                            break;
                                        }
                                        case 2: {
                                            if (_e150 >= 13u) {
                                                phi_7475_ = (_e2012.member_1 <= (_e150 - 13u));
                                            } else {
                                                phi_7475_ = false;
                                            }
                                            let _e2123 = phi_7475_;
                                            if _e2123 {
                                                let _e2126 = global_2.member[_e2012.member_1];
                                                let _e2131 = global_2.member[(_e2012.member_1 + 1u)];
                                                let _e2136 = global_2.member[(_e2012.member_1 + 2u)];
                                                let _e2142 = global_2.member[(_e2012.member_1 + 3u)];
                                                let _e2147 = global_2.member[(_e2012.member_1 + 4u)];
                                                let _e2152 = global_2.member[(_e2012.member_1 + 5u)];
                                                let _e2158 = global_2.member[(_e2012.member_1 + 6u)];
                                                let _e2163 = global_2.member[(_e2012.member_1 + 7u)];
                                                let _e2168 = global_2.member[(_e2012.member_1 + 8u)];
                                                let _e2173 = global_2.member[(_e2012.member_1 + 9u)];
                                                let _e2178 = global_2.member[(_e2012.member_1 + 10u)];
                                                let _e2183 = global_2.member[(_e2012.member_1 + 11u)];
                                                let _e2189 = global_2.member[(_e2012.member_1 + 12u)];
                                                phi_4095_ = type_39(vec3<f32>(bitcast<f32>(_e2126), bitcast<f32>(_e2131), bitcast<f32>(_e2136)), vec3<f32>(bitcast<f32>(_e2142), bitcast<f32>(_e2147), bitcast<f32>(_e2152)), bitcast<f32>(_e2158), bitcast<f32>(_e2163), vec4<f32>(bitcast<f32>(_e2168), bitcast<f32>(_e2173), bitcast<f32>(_e2178), bitcast<f32>(_e2183)), bitcast<f32>(_e2189));
                                            } else {
                                                phi_4095_ = type_39(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2193 = phi_4095_;
                                            let _e2219 = vec3<f32>((_e2073.member.x + fma(_e2112.x, _e2193.member.z, fma(_e2110.x, _e2193.member.y, (_e2108.x * _e2193.member.x)))), (_e2073.member.y + fma(_e2112.y, _e2193.member.z, fma(_e2110.y, _e2193.member.y, (_e2108.y * _e2193.member.x)))), (_e2073.member.z + fma(_e2112.z, _e2193.member.z, fma(_e2110.z, _e2193.member.y, (_e2108.z * _e2193.member.x)))));
                                            let _e2220 = (_e2219 - _e158);
                                            let _e2227 = sqrt(fma(_e2220.z, _e2220.z, fma(_e2220.x, _e2220.x, (_e2220.y * _e2220.y))));
                                            let _e2228 = (_e2227 == 0f);
                                            if _e2228 {
                                                phi_4231_ = type_40(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), 0f, 0f, 0f, 0f, 0f, 0f, 0f, false, false);
                                            } else {
                                                if _e2228 {
                                                    phi_7525_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7525_ = (_e2220 * (1f / _e2227));
                                                }
                                                let _e2232 = phi_7525_;
                                                let _e2243 = fma(_e2112.x, _e2193.member_1.z, fma(_e2110.x, _e2193.member_1.y, (_e2108.x * _e2193.member_1.x)));
                                                let _e2244 = fma(_e2112.y, _e2193.member_1.z, fma(_e2110.y, _e2193.member_1.y, (_e2108.y * _e2193.member_1.x)));
                                                let _e2245 = fma(_e2112.z, _e2193.member_1.z, fma(_e2110.z, _e2193.member_1.y, (_e2108.z * _e2193.member_1.x)));
                                                let _e2250 = sqrt(fma(_e2245, _e2245, fma(_e2243, _e2243, (_e2244 * _e2244))));
                                                if (_e2250 == 0f) {
                                                    phi_7560_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7560_ = (vec3<f32>(_e2243, _e2244, _e2245) * (1f / _e2250));
                                                }
                                                let _e2255 = phi_7560_;
                                                let _e2257 = cos(_e2193.member_2);
                                                let _e2259 = cos(_e2193.member_3);
                                                let _e2260 = (_e2257 - _e2259);
                                                let _e2272 = fma(_e2232.z, -(_e2255.z), fma(_e2232.x, -(_e2255.x), (_e2232.y * -(_e2255.y))));
                                                let _e2276 = ((_e2272 - _e2259) / _e2260);
                                                let _e2278 = select(_e2276, 0f, (_e2276 < 0f));
                                                phi_4231_ = type_40(_e2219, _e158, _e2232, _e2255, _e2227, _e2257, _e2259, _e2260, _e2272, _e2276, select(_e2278, 1f, (_e2278 > 1f)), (_e2272 > _e2257), (_e2272 > _e2259));
                                            }
                                            let _e2283 = phi_4231_;
                                            let _e2285 = (_e2283.member_4 == 0f);
                                            if _e2285 {
                                                phi_9010_ = _e1950;
                                                phi_4963_ = f32();
                                                phi_4964_ = vec3<f32>();
                                            } else {
                                                let _e2288 = (_e2193.member_5 * _e2283.member_10);
                                                let _e2292 = fma(-(_e686.z), _e295.member_3, 1f);
                                                let _e2296 = fma(0.4f, _e2292, (_e1370 * _e1382));
                                                let _e2297 = fma(0.4f, _e2292, (_e1372 * _e1382));
                                                let _e2298 = fma(0.4f, _e2292, (_e1374 * _e1382));
                                                let _e2305 = (_e1938 + _e2283.member_2);
                                                let _e2312 = sqrt(fma(_e2305.z, _e2305.z, fma(_e2305.x, _e2305.x, (_e2305.y * _e2305.y))));
                                                if (_e2312 == 0f) {
                                                    phi_7595_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7595_ = (_e2305 * (1f / _e2312));
                                                }
                                                let _e2317 = phi_7595_;
                                                let _e2318 = (_e1379 * _e1379);
                                                let _e2329 = max(fma(_e1934.z, _e2317.z, fma(_e1934.x, _e2317.x, (_e1934.y * _e2317.y))), 0f);
                                                let _e2342 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                                let _e2348 = fma(_e1934.z, _e2283.member_2.z, fma(_e1934.x, _e2283.member_2.x, (_e1934.y * _e2283.member_2.y)));
                                                let _e2349 = max(_e2348, 0f);
                                                let _e2350 = fma(_e686.y, _e295.member_4, 1f);
                                                let _e2351 = (_e2350 * _e2350);
                                                let _e2352 = (_e2351 * 0.125f);
                                                let _e2354 = fma(-(_e2351), 0.125f, 1f);
                                                let _e2367 = (1f - max(fma(_e2317.z, _e1938.z, fma(_e2317.x, _e1938.x, (_e2317.y * _e1938.y))), 0f));
                                                let _e2369 = select(_e2367, 0f, (_e2367 < 0f));
                                                let _e2372 = pow(select(_e2369, 1f, (_e2369 > 1f)), 5f);
                                                let _e2373 = fma((1f - _e2296), _e2372, _e2296);
                                                let _e2374 = fma((1f - _e2297), _e2372, _e2297);
                                                let _e2375 = fma((1f - _e2298), _e2372, _e2298);
                                                let _e2382 = (((_e2318 * _e2318) / (pow(fma((_e2329 * _e2329), fma(_e2318, _e2318, -1f), 1f), 2f) * 3.1415927f)) * ((_e2342 / fma(_e2342, _e2354, _e2352)) * (_e2349 / fma(_e2349, _e2354, _e2352))));
                                                let _e2387 = fma((4f * _e2342), _e2349, 0.0001f);
                                                if ((_e2012.member_3 == 4294967295u) != true) {
                                                    let _e2409 = global_2.member[_e2012.member_3];
                                                    let _e2413 = global_2.member[(_e2012.member_3 + 1u)];
                                                    let _e2417 = global_2.member[(_e2012.member_3 + 4u)];
                                                    let _e2421 = global_2.member[(_e2012.member_3 + 5u)];
                                                    let _e2425 = global_2.member[(_e2012.member_3 + 6u)];
                                                    let _e2430 = global_2.member[(_e2012.member_3 + 7u)];
                                                    let _e2435 = global_2.member[(_e2012.member_3 + 8u)];
                                                    let _e2438 = global_2.member[_e1947];
                                                    let _e2442 = global_2.member[(_e1947 + 1u)];
                                                    let _e2444 = select(_e2409, 4294967295u, (0u >= _e2413));
                                                    let _e2447 = global_2.member[_e2444];
                                                    let _e2452 = global_2.member[(_e2444 + 1u)];
                                                    let _e2457 = global_2.member[(_e2444 + 2u)];
                                                    let _e2462 = global_2.member[(_e2444 + 3u)];
                                                    let _e2467 = global_2.member[(_e2444 + 4u)];
                                                    let _e2472 = global_2.member[(_e2444 + 5u)];
                                                    let _e2477 = global_2.member[(_e2444 + 6u)];
                                                    let _e2482 = global_2.member[(_e2444 + 7u)];
                                                    let _e2487 = global_2.member[(_e2444 + 8u)];
                                                    let _e2492 = global_2.member[(_e2444 + 9u)];
                                                    let _e2497 = global_2.member[(_e2444 + 10u)];
                                                    let _e2502 = global_2.member[(_e2444 + 11u)];
                                                    let _e2507 = global_2.member[(_e2444 + 12u)];
                                                    let _e2512 = global_2.member[(_e2444 + 13u)];
                                                    let _e2517 = global_2.member[(_e2444 + 14u)];
                                                    let _e2522 = global_2.member[(_e2444 + 15u)];
                                                    let _e2542 = (bitcast<f32>(_e2522) + fma(bitcast<f32>(_e2502), _e158.z, fma(bitcast<f32>(_e2482), _e158.y, (bitcast<f32>(_e2462) * _e158.x))));
                                                    let _e2543 = ((bitcast<f32>(_e2507) + fma(bitcast<f32>(_e2487), _e158.z, fma(bitcast<f32>(_e2467), _e158.y, (bitcast<f32>(_e2447) * _e158.x)))) / _e2542);
                                                    let _e2544 = ((bitcast<f32>(_e2512) + fma(bitcast<f32>(_e2492), _e158.z, fma(bitcast<f32>(_e2472), _e158.y, (bitcast<f32>(_e2452) * _e158.x)))) / _e2542);
                                                    let _e2545 = ((bitcast<f32>(_e2517) + fma(bitcast<f32>(_e2497), _e158.z, fma(bitcast<f32>(_e2477), _e158.y, (bitcast<f32>(_e2457) * _e158.x)))) / _e2542);
                                                    if (abs(_e2543) <= 1f) {
                                                        let _e2549 = (abs(_e2544) <= 1f);
                                                        if _e2549 {
                                                            phi_7700_ = (abs(_e2545) <= 1f);
                                                        } else {
                                                            phi_7700_ = bool();
                                                        }
                                                        let _e2553 = phi_7700_;
                                                        phi_7703_ = _e2553;
                                                        phi_7704_ = select(true, false, _e2549);
                                                    } else {
                                                        phi_7703_ = bool();
                                                        phi_7704_ = true;
                                                    }
                                                    let _e2556 = phi_7703_;
                                                    let _e2558 = phi_7704_;
                                                    if select(_e2556, false, _e2558) {
                                                        let _e2566 = global_2.member[select(_e2417, 4294967295u, (0u >= _e2421))];
                                                        let _e2569 = global_2.member[_e2566];
                                                        let _e2573 = global_2.member[(_e2566 + 1u)];
                                                        let _e2577 = global_2.member[(_e2566 + 2u)];
                                                        let _e2581 = global_2.member[(_e2566 + 3u)];
                                                        let _e2585 = global_2.member[(_e2566 + 4u)];
                                                        let _e2589 = global_2.member[(_e2566 + 6u)];
                                                        switch bitcast<i32>(_e2589) {
                                                            case 0: {
                                                                phi_4603_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_4603_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_4603_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_4603_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e2592 = phi_4603_;
                                                        let _e2596 = global_2.member[(_e2566 + 7u)];
                                                        switch bitcast<i32>(_e2596) {
                                                            case 0: {
                                                                phi_4612_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_4612_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_4612_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_4612_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e2599 = phi_4612_;
                                                        let _e2600 = bitcast<i32>(_e2435);
                                                        let _e2602 = f32(_e2577);
                                                        let _e2603 = f32(_e2581);
                                                        let _e2607 = type_37((_e2600 / -2i), (_e2600 / 2i), false);
                                                        phi_8804_ = _e1950;
                                                        phi_4640_ = _e2607;
                                                        phi_4643_ = 0f;
                                                        phi_4645_ = 0f;
                                                        loop {
                                                            let _e2609 = phi_8804_;
                                                            let _e2611 = phi_4640_;
                                                            let _e2613 = phi_4643_;
                                                            let _e2615 = phi_4645_;
                                                            local_3 = _e2613;
                                                            local_4 = _e2615;
                                                            if _e2611.member_2 {
                                                                phi_4657_ = true;
                                                            } else {
                                                                phi_4657_ = ((_e2611.member <= _e2611.member_1) != true);
                                                            }
                                                            let _e2622 = phi_4657_;
                                                            if _e2622 {
                                                                phi_4641_ = _e2611;
                                                                phi_4700_ = type_38(0u, type_38().member_1);
                                                            } else {
                                                                if (_e2611.member < _e2611.member_1) {
                                                                    let _e2630 = (_e2611.member + 1i);
                                                                    if select(false, true, ((false == (_e2630 > _e2611.member)) != false)) {
                                                                        phi_4685_ = type_38(0u, type_38().member_1);
                                                                    } else {
                                                                        phi_4685_ = type_38(1u, _e2630);
                                                                    }
                                                                    let _e2640 = phi_4685_;
                                                                    switch bitcast<i32>(_e2640.member) {
                                                                        case 0: {
                                                                            phi_8884_ = true;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            break;
                                                                        }
                                                                    }
                                                                    phi_4697_ = type_37(_e2640.member_1, _e2611.member_1, _e2611.member_2);
                                                                } else {
                                                                    phi_4697_ = type_37(_e2611.member, _e2611.member_1, true);
                                                                }
                                                                let _e2649 = phi_4697_;
                                                                phi_4641_ = _e2649;
                                                                phi_4700_ = type_38(1u, _e2611.member);
                                                            }
                                                            let _e2655 = phi_4641_;
                                                            let _e2657 = phi_4700_;
                                                            switch bitcast<i32>(_e2657.member) {
                                                                case 0: {
                                                                    phi_8885_ = _e2609;
                                                                    phi_4644_ = f32();
                                                                    phi_4646_ = f32();
                                                                    phi_4958_ = false;
                                                                    break;
                                                                }
                                                                case 1: {
                                                                    phi_4711_ = _e2607;
                                                                    phi_4714_ = _e2613;
                                                                    phi_4716_ = _e2615;
                                                                    loop {
                                                                        let _e2662 = phi_4711_;
                                                                        let _e2664 = phi_4714_;
                                                                        let _e2666 = phi_4716_;
                                                                        local_11 = _e2664;
                                                                        local_12 = _e2666;
                                                                        if _e2662.member_2 {
                                                                            phi_4728_ = true;
                                                                        } else {
                                                                            phi_4728_ = ((_e2662.member <= _e2662.member_1) != true);
                                                                        }
                                                                        let _e2673 = phi_4728_;
                                                                        if _e2673 {
                                                                            phi_4712_ = _e2662;
                                                                            phi_4771_ = type_38(0u, type_38().member_1);
                                                                        } else {
                                                                            if (_e2662.member < _e2662.member_1) {
                                                                                let _e2681 = (_e2662.member + 1i);
                                                                                if select(false, true, ((false == (_e2681 > _e2662.member)) != false)) {
                                                                                    phi_4756_ = type_38(0u, type_38().member_1);
                                                                                } else {
                                                                                    phi_4756_ = type_38(1u, _e2681);
                                                                                }
                                                                                let _e2691 = phi_4756_;
                                                                                switch bitcast<i32>(_e2691.member) {
                                                                                    case 0: {
                                                                                        phi_8788_ = true;
                                                                                        break;
                                                                                    }
                                                                                    case 1: {
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                phi_4768_ = type_37(_e2691.member_1, _e2662.member_1, _e2662.member_2);
                                                                            } else {
                                                                                phi_4768_ = type_37(_e2662.member, _e2662.member_1, true);
                                                                            }
                                                                            let _e2700 = phi_4768_;
                                                                            phi_4712_ = _e2700;
                                                                            phi_4771_ = type_38(1u, _e2662.member);
                                                                        }
                                                                        let _e2706 = phi_4712_;
                                                                        let _e2708 = phi_4771_;
                                                                        switch bitcast<i32>(_e2708.member) {
                                                                            case 0: {
                                                                                phi_4715_ = f32();
                                                                                phi_4717_ = f32();
                                                                                phi_4957_ = false;
                                                                                break;
                                                                            }
                                                                            case 1: {
                                                                                let _e2716 = fma((_e2543 + 1f), 0.5f, (f32(_e2657.member_1) * (1f / _e2602)));
                                                                                let _e2717 = fma(fma(_e2544, -1f, 1f), 0.5f, (f32(_e2708.member_1) * (1f / _e2603)));
                                                                                switch bitcast<i32>(_e2592) {
                                                                                    case 1: {
                                                                                        let _e2752 = abs(_e2716);
                                                                                        let _e2754 = (_e2752 % 1f);
                                                                                        if (_e2752 >= 1f) {
                                                                                            phi_7756_ = select(true, false, (_e2754 == 0f));
                                                                                        } else {
                                                                                            phi_7756_ = true;
                                                                                        }
                                                                                        let _e2758 = phi_7756_;
                                                                                        let _e2759 = select(1f, _e2754, _e2758);
                                                                                        if (select(-1f, 1f, (_e2716 >= 0f)) > 0f) {
                                                                                            phi_4803_ = _e2759;
                                                                                        } else {
                                                                                            phi_4803_ = (1f - _e2759);
                                                                                        }
                                                                                        let _e2763 = phi_4803_;
                                                                                        phi_4840_ = _e2763;
                                                                                        break;
                                                                                    }
                                                                                    case 2: {
                                                                                        let _e2726 = abs(_e2716);
                                                                                        let _e2733 = ((select(select(u32(_e2726), 0u, (_e2726 < 0f)), 4294967295u, (_e2726 > 4294967000f)) % 2u) == 0u);
                                                                                        let _e2735 = (_e2726 % 1f);
                                                                                        if (_e2726 >= 1f) {
                                                                                            phi_7739_ = select(true, false, (_e2735 == 0f));
                                                                                        } else {
                                                                                            phi_7739_ = true;
                                                                                        }
                                                                                        let _e2739 = phi_7739_;
                                                                                        let _e2740 = select(1f, _e2735, _e2739);
                                                                                        if (select(-1f, 1f, (_e2716 >= 0f)) > 0f) {
                                                                                            if _e2733 {
                                                                                                phi_4832_ = _e2740;
                                                                                            } else {
                                                                                                phi_4832_ = (1f - _e2740);
                                                                                            }
                                                                                            let _e2747 = phi_4832_;
                                                                                            phi_4838_ = _e2747;
                                                                                        } else {
                                                                                            if _e2733 {
                                                                                                phi_4837_ = (1f - _e2740);
                                                                                            } else {
                                                                                                phi_4837_ = _e2740;
                                                                                            }
                                                                                            let _e2744 = phi_4837_;
                                                                                            phi_4838_ = _e2744;
                                                                                        }
                                                                                        let _e2749 = phi_4838_;
                                                                                        phi_4840_ = _e2749;
                                                                                        break;
                                                                                    }
                                                                                    case 0: {
                                                                                        if (_e2716 > 1f) {
                                                                                            phi_7726_ = 0.9999999f;
                                                                                        } else {
                                                                                            phi_7726_ = select(_e2716, 0.00000011920929f, (_e2716 < 0f));
                                                                                        }
                                                                                        let _e2723 = phi_7726_;
                                                                                        phi_4840_ = _e2723;
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        phi_4840_ = f32();
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2765 = phi_4840_;
                                                                                switch bitcast<i32>(_e2599) {
                                                                                    case 1: {
                                                                                        let _e2800 = abs(_e2717);
                                                                                        let _e2802 = (_e2800 % 1f);
                                                                                        if (_e2800 >= 1f) {
                                                                                            phi_7804_ = select(true, false, (_e2802 == 0f));
                                                                                        } else {
                                                                                            phi_7804_ = true;
                                                                                        }
                                                                                        let _e2806 = phi_7804_;
                                                                                        let _e2807 = select(1f, _e2802, _e2806);
                                                                                        if (select(-1f, 1f, (_e2717 >= 0f)) > 0f) {
                                                                                            phi_4859_ = _e2807;
                                                                                        } else {
                                                                                            phi_4859_ = (1f - _e2807);
                                                                                        }
                                                                                        let _e2811 = phi_4859_;
                                                                                        phi_4896_ = _e2811;
                                                                                        break;
                                                                                    }
                                                                                    case 2: {
                                                                                        let _e2774 = abs(_e2717);
                                                                                        let _e2781 = ((select(select(u32(_e2774), 0u, (_e2774 < 0f)), 4294967295u, (_e2774 > 4294967000f)) % 2u) == 0u);
                                                                                        let _e2783 = (_e2774 % 1f);
                                                                                        if (_e2774 >= 1f) {
                                                                                            phi_7787_ = select(true, false, (_e2783 == 0f));
                                                                                        } else {
                                                                                            phi_7787_ = true;
                                                                                        }
                                                                                        let _e2787 = phi_7787_;
                                                                                        let _e2788 = select(1f, _e2783, _e2787);
                                                                                        if (select(-1f, 1f, (_e2717 >= 0f)) > 0f) {
                                                                                            if _e2781 {
                                                                                                phi_4888_ = _e2788;
                                                                                            } else {
                                                                                                phi_4888_ = (1f - _e2788);
                                                                                            }
                                                                                            let _e2795 = phi_4888_;
                                                                                            phi_4894_ = _e2795;
                                                                                        } else {
                                                                                            if _e2781 {
                                                                                                phi_4893_ = (1f - _e2788);
                                                                                            } else {
                                                                                                phi_4893_ = _e2788;
                                                                                            }
                                                                                            let _e2792 = phi_4893_;
                                                                                            phi_4894_ = _e2792;
                                                                                        }
                                                                                        let _e2797 = phi_4894_;
                                                                                        phi_4896_ = _e2797;
                                                                                        break;
                                                                                    }
                                                                                    case 0: {
                                                                                        if (_e2717 > 1f) {
                                                                                            phi_7774_ = 0.9999999f;
                                                                                        } else {
                                                                                            phi_7774_ = select(_e2717, 0.00000011920929f, (_e2717 < 0f));
                                                                                        }
                                                                                        let _e2771 = phi_7774_;
                                                                                        phi_4896_ = _e2771;
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        phi_4896_ = f32();
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2813 = phi_4896_;
                                                                                let _e2814 = (_e2765 * _e2602);
                                                                                let _e2820 = (_e2813 * _e2603);
                                                                                let _e2835 = vec3<f32>((f32((select(select(u32(_e2814), 0u, (_e2814 < 0f)), 4294967295u, (_e2814 > 4294967000f)) + _e2569)) / f32(_e2438)), (f32((select(select(u32(_e2820), 0u, (_e2820 < 0f)), 4294967295u, (_e2820 > 4294967000f)) + _e2573)) / f32(_e2442)), f32(_e2585));
                                                                                let _e2841 = textureSampleLevel(global_20, global_19, vec2<f32>(_e2835.x, _e2835.y), i32(_e2835.z), 0f);
                                                                                if ((_e2545 - max((bitcast<f32>(_e2430) * (1f - _e2348)), bitcast<f32>(_e2425))) > _e2841.x) {
                                                                                    phi_4955_ = (_e2666 + 1f);
                                                                                } else {
                                                                                    phi_4955_ = _e2666;
                                                                                }
                                                                                let _e2850 = phi_4955_;
                                                                                phi_4715_ = (_e2664 + 1f);
                                                                                phi_4717_ = _e2850;
                                                                                phi_4957_ = true;
                                                                                break;
                                                                            }
                                                                            default: {
                                                                                phi_4715_ = f32();
                                                                                phi_4717_ = f32();
                                                                                phi_4957_ = bool();
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e2853 = phi_4715_;
                                                                        let _e2855 = phi_4717_;
                                                                        let _e2857 = phi_4957_;
                                                                        continue;
                                                                        continuing {
                                                                            phi_4711_ = _e2706;
                                                                            phi_4714_ = _e2853;
                                                                            phi_4716_ = _e2855;
                                                                            phi_8788_ = _e2609;
                                                                            break if !(_e2857);
                                                                        }
                                                                    }
                                                                    let _e2860 = phi_8788_;
                                                                    phi_8884_ = _e2860;
                                                                    if _e2860 {
                                                                        break;
                                                                    }
                                                                    phi_8885_ = _e2860;
                                                                    let _e4534 = local_11;
                                                                    phi_4644_ = _e4534;
                                                                    let _e4537 = local_12;
                                                                    phi_4646_ = _e4537;
                                                                    phi_4958_ = true;
                                                                    break;
                                                                }
                                                                default: {
                                                                    phi_8885_ = _e2609;
                                                                    phi_4644_ = f32();
                                                                    phi_4646_ = f32();
                                                                    phi_4958_ = bool();
                                                                    break;
                                                                }
                                                            }
                                                            let _e2862 = phi_8885_;
                                                            let _e2864 = phi_4644_;
                                                            let _e2866 = phi_4646_;
                                                            let _e2868 = phi_4958_;
                                                            continue;
                                                            continuing {
                                                                phi_8804_ = _e2862;
                                                                phi_4640_ = _e2655;
                                                                phi_4643_ = _e2864;
                                                                phi_4645_ = _e2866;
                                                                phi_8884_ = _e2862;
                                                                break if !(_e2868);
                                                            }
                                                        }
                                                        let _e2871 = phi_8884_;
                                                        phi_9005_ = _e2871;
                                                        if _e2871 {
                                                            break;
                                                        }
                                                        let _e2873 = local_3;
                                                        let _e2876 = local_4;
                                                        phi_9012_ = _e2871;
                                                        phi_4961_ = (_e2876 / max(_e2873, 1f));
                                                    } else {
                                                        phi_9012_ = _e1950;
                                                        phi_4961_ = 0f;
                                                    }
                                                    let _e2879 = phi_9012_;
                                                    let _e2881 = phi_4961_;
                                                    phi_9011_ = _e2879;
                                                    phi_4962_ = _e2881;
                                                } else {
                                                    phi_9011_ = _e1950;
                                                    phi_4962_ = 0f;
                                                }
                                                let _e2883 = phi_9011_;
                                                let _e2885 = phi_4962_;
                                                phi_9010_ = _e2883;
                                                phi_4963_ = _e2885;
                                                phi_4964_ = vec3<f32>(((fma((((1f - _e2373) * _e2292) * _e1370), 0.31830987f, ((_e2382 * _e2373) / _e2387)) * (_e2193.member_4.x * _e2288)) * _e2349), ((fma((((1f - _e2374) * _e2292) * _e1372), 0.31830987f, ((_e2382 * _e2374) / _e2387)) * (_e2193.member_4.y * _e2288)) * _e2349), ((fma((((1f - _e2375) * _e2292) * _e1374), 0.31830987f, ((_e2382 * _e2375) / _e2387)) * (_e2193.member_4.z * _e2288)) * _e2349));
                                            }
                                            let _e2887 = phi_9010_;
                                            let _e2889 = phi_4963_;
                                            let _e2891 = phi_4964_;
                                            phi_9009_ = _e2887;
                                            phi_4966_ = _e2889;
                                            phi_4967_ = _e2891;
                                            phi_4968_ = select(true, false, _e2285);
                                            break;
                                        }
                                        default: {
                                            phi_9009_ = _e1950;
                                            phi_4966_ = f32();
                                            phi_4967_ = vec3<f32>();
                                            phi_4968_ = bool();
                                            break;
                                        }
                                    }
                                    let _e4231 = phi_9009_;
                                    let _e4233 = phi_4966_;
                                    let _e4235 = phi_4967_;
                                    let _e4237 = phi_4968_;
                                    if _e4237 {
                                        let _e4238 = (1f - _e4233);
                                        phi_4988_ = vec3<f32>(fma(_e4235.x, _e4238, _e1954.x), fma(_e4235.y, _e4238, _e1954.y), fma(_e4235.z, _e4238, _e1954.z));
                                    } else {
                                        phi_4988_ = vec3<f32>();
                                    }
                                    let _e4250 = phi_4988_;
                                    phi_9006_ = _e4231;
                                    phi_2400_ = select(_e4250, _e1954, vec3(select(true, false, _e4237)));
                                    phi_4994_ = true;
                                    break;
                                }
                                default: {
                                    phi_9006_ = _e1950;
                                    phi_2400_ = vec3<f32>();
                                    phi_4994_ = bool();
                                    break;
                                }
                            }
                            let _e4255 = phi_9006_;
                            let _e4257 = phi_2400_;
                            let _e4259 = phi_4994_;
                            continue;
                            continuing {
                                phi_8831_ = _e4255;
                                phi_2396_ = _e1967;
                                phi_2399_ = _e4257;
                                phi_9005_ = _e4255;
                                break if !(_e4259);
                            }
                        }
                        let _e4262 = phi_9005_;
                        phi_9017_ = _e4262;
                        if _e4262 {
                            break;
                        }
                        let _e4264 = fma(-(_e686.z), _e295.member_3, 1f);
                        let _e4268 = fma(0.04f, _e4264, (_e1370 * _e1382));
                        let _e4269 = fma(0.04f, _e4264, (_e1372 * _e1382));
                        let _e4270 = fma(0.04f, _e4264, (_e1374 * _e1382));
                        let _e4282 = fma(-(_e686.y), _e295.member_4, 1f);
                        let _e4289 = (1f - max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f));
                        let _e4291 = select(_e4289, 0f, (_e4289 < 0f));
                        let _e4294 = pow(select(_e4291, 1f, (_e4291 > 1f)), 5f);
                        let _e4295 = fma((max(_e4282, _e4268) - _e4268), _e4294, _e4268);
                        let _e4296 = fma((max(_e4282, _e4269) - _e4269), _e4294, _e4269);
                        let _e4297 = fma((max(_e4282, _e4270) - _e4270), _e4294, _e4270);
                        let _e4317 = local_8;
                        let _e4321 = local_9;
                        let _e4325 = local_10;
                        phi_9025_ = _e4262;
                        phi_5111_ = vec4<f32>(fma(_e1392, _e295.member_1, fma(fma(((1f - _e4295) * _e4264), (_e1401.x * _e1370), (_e1749.x * fma(_e4295, _e1765.x, _e1765.y))), _e1386, _e4317.x)), fma(_e1394, _e295.member_1, fma(fma(((1f - _e4296) * _e4264), (_e1401.y * _e1372), (_e1749.y * fma(_e4296, _e1765.x, _e1765.y))), _e1386, _e4321.y)), fma(_e1396, _e295.member_1, fma(fma(((1f - _e4297) * _e4264), (_e1401.z * _e1374), (_e1749.z * fma(_e4297, _e1765.x, _e1765.y))), _e1386, _e4325.z)), 1f);
                    } else {
                        phi_9025_ = false;
                        phi_5111_ = (vec4<f32>((_e152.x * _e492.x), (_e152.y * _e492.y), (_e152.z * _e492.z), (_e152.w * _e492.w)) * _e295.member_2);
                    }
                    let _e4333 = phi_9025_;
                    let _e4335 = phi_5111_;
                    global_21 = _e4335;
                    phi_9017_ = _e4333;
                    break;
                }
                case 1: {
                    let _e1907 = sqrt(fma(_e153.x, _e153.x, (_e153.y * _e153.y)));
                    if (_e1907 == 0f) {
                        phi_7220_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7220_ = (vec3<f32>(_e153.x, _e153.y, 0f) * (1f / _e1907));
                    }
                    let _e1912 = phi_7220_;
                    global_21 = vec4<f32>(((_e1912.x + 1f) * 0.5f), ((_e1912.y + 1f) * 0.5f), ((_e1912.z + 1f) * 0.5f), 1f);
                    phi_9017_ = false;
                    break;
                }
                case 2: {
                    let _e1886 = sqrt(fma(_e154.x, _e154.x, (_e154.y * _e154.y)));
                    if (_e1886 == 0f) {
                        phi_7171_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7171_ = (vec3<f32>(_e154.x, _e154.y, 0f) * (1f / _e1886));
                    }
                    let _e1891 = phi_7171_;
                    global_21 = vec4<f32>(((_e1891.x + 1f) * 0.5f), ((_e1891.y + 1f) * 0.5f), ((_e1891.z + 1f) * 0.5f), 1f);
                    phi_9017_ = false;
                    break;
                }
                case 3: {
                    if _e1728 {
                        phi_7122_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7122_ = (_e1355 * (1f / _e1727));
                    }
                    let _e1870 = phi_7122_;
                    global_21 = vec4<f32>(((_e1870.x + 1f) * 0.5f), ((_e1870.y + 1f) * 0.5f), ((_e1870.z + 1f) * 0.5f), 1f);
                    phi_9017_ = false;
                    break;
                }
                case 4: {
                    global_21 = _e152;
                    phi_9017_ = false;
                    break;
                }
                case 5: {
                    let _e1851 = sqrt(fma(_e155.z, _e155.z, fma(_e155.x, _e155.x, (_e155.y * _e155.y))));
                    if (_e1851 == 0f) {
                        phi_7073_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7073_ = (_e155 * (1f / _e1851));
                    }
                    let _e1856 = phi_7073_;
                    global_21 = vec4<f32>(((_e1856.x + 1f) * 0.5f), ((_e1856.y + 1f) * 0.5f), ((_e1856.z + 1f) * 0.5f), 1f);
                    phi_9017_ = false;
                    break;
                }
                case 6: {
                    let _e1829 = sqrt(fma(_e1353.z, _e1353.z, fma(_e1353.x, _e1353.x, (_e1353.y * _e1353.y))));
                    if (_e1829 == 0f) {
                        phi_7024_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7024_ = (_e1353 * (1f / _e1829));
                    }
                    let _e1834 = phi_7024_;
                    global_21 = vec4<f32>(((_e1834.x + 1f) * 0.5f), ((_e1834.y + 1f) * 0.5f), ((_e1834.z + 1f) * 0.5f), 1f);
                    phi_9017_ = false;
                    break;
                }
                case 7: {
                    let _e1807 = sqrt(fma(_e156.z, _e156.z, fma(_e156.x, _e156.x, (_e156.y * _e156.y))));
                    if (_e1807 == 0f) {
                        phi_6975_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6975_ = (_e156 * (1f / _e1807));
                    }
                    let _e1812 = phi_6975_;
                    global_21 = vec4<f32>(((_e1812.x + 1f) * 0.5f), ((_e1812.y + 1f) * 0.5f), ((_e1812.z + 1f) * 0.5f), 1f);
                    phi_9017_ = false;
                    break;
                }
                case 8: {
                    let _e1785 = sqrt(fma(_e157.z, _e157.z, fma(_e157.x, _e157.x, (_e157.y * _e157.y))));
                    if (_e1785 == 0f) {
                        phi_6926_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6926_ = (_e157 * (1f / _e1785));
                    }
                    let _e1790 = phi_6926_;
                    global_21 = vec4<f32>(((_e1790.x + 1f) * 0.5f), ((_e1790.y + 1f) * 0.5f), ((_e1790.z + 1f) * 0.5f), 1f);
                    phi_9017_ = false;
                    break;
                }
                case 9: {
                    global_21 = vec4<f32>(_e1401.x, _e1401.y, _e1401.z, 1f);
                    phi_9017_ = false;
                    break;
                }
                case 10: {
                    global_21 = vec4<f32>(_e1749.x, _e1749.y, _e1749.z, 1f);
                    phi_9017_ = false;
                    break;
                }
                case 11: {
                    global_21 = vec4<f32>(_e1765.x, _e1765.y, 1f, 1f);
                    phi_9017_ = false;
                    break;
                }
                case 12: {
                    global_21 = (vec4<f32>(_e1359, _e1362, _e1365, (_e492.w * _e295.member_2.w)) * _e152);
                    phi_9017_ = false;
                    break;
                }
                case 13: {
                    global_21 = vec4<f32>(_e1379, _e1379, _e1379, 1f);
                    phi_9017_ = false;
                    break;
                }
                case 14: {
                    global_21 = vec4<f32>(_e1382, _e1382, _e1382, 1f);
                    phi_9017_ = false;
                    break;
                }
                case 15: {
                    global_21 = vec4<f32>(_e1386, _e1386, _e1386, 1f);
                    phi_9017_ = false;
                    break;
                }
                case 16: {
                    global_21 = vec4<f32>((_e1392 * _e295.member_1), (_e1394 * _e295.member_1), (_e1396 * _e295.member_1), 1f);
                    phi_9017_ = false;
                    break;
                }
                case 17: {
                    global_21 = vec4<f32>(_e1268.x, _e1268.y, _e1268.z, 1f);
                    phi_9017_ = false;
                    break;
                }
                case 18: {
                    global_21 = vec4<f32>(_e295.member.x, _e295.member.y, _e295.member.z, 1f);
                    phi_9017_ = false;
                    break;
                }
                case 19: {
                    global_21 = vec4<f32>(_e295.member_1, _e295.member_1, _e295.member_1, 1f);
                    phi_9017_ = false;
                    break;
                }
                default: {
                    phi_9017_ = false;
                    break;
                }
            }
            let _e4337 = phi_9017_;
            if _e4337 {
                break;
            }
            break;
        }
    }
    return;
}

@fragment 
fn stagerenderlet_fragment(@location(0) @interpolate(flat) param: u32, @location(1) param_1: vec4<f32>, @location(2) param_2: vec2<f32>, @location(3) param_3: vec2<f32>, @location(4) param_4: vec3<f32>, @location(5) param_5: vec3<f32>, @location(6) param_6: vec3<f32>, @location(7) param_7: vec3<f32>) -> @location(0) vec4<f32> {
    global_3 = param;
    global_4 = param_1;
    global_5 = param_2;
    global_6 = param_3;
    global_7 = param_4;
    global_8 = param_5;
    global_9 = param_6;
    global_10 = param_7;
    function();
    let _e17 = global_21;
    return _e17;
}
