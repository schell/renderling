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
    var local: array<vec3<f32>, 21>;
    var local_1: array<vec3<f32>, 8>;
    var local_2: array<vec4<f32>, 6>;
    var phi_722_: u32;
    var phi_5788_: bool;
    var phi_850_: type_35;
    var phi_854_: type_35;
    var phi_5825_: bool;
    var phi_894_: u32;
    var phi_903_: u32;
    var phi_916_: type_15;
    var phi_5847_: f32;
    var phi_5860_: bool;
    var phi_970_: f32;
    var phi_965_: f32;
    var phi_971_: f32;
    var phi_5877_: bool;
    var phi_936_: f32;
    var phi_973_: f32;
    var phi_5895_: f32;
    var phi_5908_: bool;
    var phi_1028_: f32;
    var phi_1023_: f32;
    var phi_1029_: f32;
    var phi_5925_: bool;
    var phi_994_: f32;
    var phi_1031_: f32;
    var phi_5961_: bool;
    var phi_1114_: u32;
    var phi_1123_: u32;
    var phi_1136_: type_15;
    var phi_5982_: f32;
    var phi_5995_: bool;
    var phi_1190_: f32;
    var phi_1185_: f32;
    var phi_1191_: f32;
    var phi_6012_: bool;
    var phi_1156_: f32;
    var phi_1193_: f32;
    var phi_6030_: f32;
    var phi_6043_: bool;
    var phi_1248_: f32;
    var phi_1243_: f32;
    var phi_1249_: f32;
    var phi_6060_: bool;
    var phi_1214_: f32;
    var phi_1251_: f32;
    var phi_6096_: bool;
    var phi_1334_: u32;
    var phi_1343_: u32;
    var phi_1356_: type_15;
    var phi_6117_: f32;
    var phi_6130_: bool;
    var phi_1410_: f32;
    var phi_1405_: f32;
    var phi_1411_: f32;
    var phi_6147_: bool;
    var phi_1376_: f32;
    var phi_1413_: f32;
    var phi_6165_: f32;
    var phi_6178_: bool;
    var phi_1468_: f32;
    var phi_1463_: f32;
    var phi_1469_: f32;
    var phi_6195_: bool;
    var phi_1434_: f32;
    var phi_1471_: f32;
    var phi_6231_: bool;
    var phi_1554_: u32;
    var phi_1563_: u32;
    var phi_1576_: type_15;
    var phi_6252_: f32;
    var phi_6265_: bool;
    var phi_1630_: f32;
    var phi_1625_: f32;
    var phi_1631_: f32;
    var phi_6282_: bool;
    var phi_1596_: f32;
    var phi_1633_: f32;
    var phi_6300_: f32;
    var phi_6313_: bool;
    var phi_1688_: f32;
    var phi_1683_: f32;
    var phi_1689_: f32;
    var phi_6330_: bool;
    var phi_1654_: f32;
    var phi_1691_: f32;
    var phi_6366_: bool;
    var phi_1774_: u32;
    var phi_1783_: u32;
    var phi_1796_: type_15;
    var phi_6387_: f32;
    var phi_6400_: bool;
    var phi_1850_: f32;
    var phi_1845_: f32;
    var phi_1851_: f32;
    var phi_6417_: bool;
    var phi_1816_: f32;
    var phi_1853_: f32;
    var phi_6435_: f32;
    var phi_6448_: bool;
    var phi_1908_: f32;
    var phi_1903_: f32;
    var phi_1909_: f32;
    var phi_6465_: bool;
    var phi_1874_: f32;
    var phi_1911_: f32;
    var phi_6523_: vec3<f32>;
    var phi_6558_: vec3<f32>;
    var phi_6593_: vec3<f32>;
    var phi_6628_: vec3<f32>;
    var phi_6663_: vec3<f32>;
    var phi_2005_: vec3<f32>;
    var phi_2006_: vec3<f32>;
    var phi_6695_: bool;
    var phi_2213_: type_14;
    var phi_2214_: type_14;
    var phi_2237_: type_14;
    var phi_2264_: bool;
    var phi_2270_: type_14;
    var phi_2271_: type_14;
    var phi_2294_: type_14;
    var phi_2317_: bool;
    var phi_2338_: type_25;
    var phi_6767_: vec3<f32>;
    var phi_6826_: vec3<f32>;
    var phi_6900_: vec3<f32>;
    var phi_6960_: vec3<f32>;
    var phi_7009_: vec3<f32>;
    var phi_7058_: vec3<f32>;
    var phi_7107_: vec3<f32>;
    var phi_7156_: vec3<f32>;
    var phi_7205_: vec3<f32>;
    var phi_7254_: vec3<f32>;
    var phi_7293_: vec3<f32>;
    var phi_7328_: vec3<f32>;
    var phi_8867_: bool;
    var phi_2405_: type_14;
    var phi_2408_: vec3<f32>;
    var phi_2406_: type_14;
    var phi_2431_: type_14;
    var phi_7354_: u32;
    var phi_7373_: bool;
    var phi_2448_: u32;
    var phi_7397_: bool;
    var phi_2460_: u32;
    var phi_2474_: type_30;
    var phi_7429_: bool;
    var phi_2524_: type_31;
    var phi_7509_: bool;
    var phi_4094_: type_39;
    var phi_7559_: vec3<f32>;
    var phi_7594_: vec3<f32>;
    var phi_4230_: type_40;
    var phi_7629_: vec3<f32>;
    var phi_7734_: bool;
    var phi_7737_: bool;
    var phi_7738_: bool;
    var phi_4597_: u32;
    var phi_4606_: u32;
    var phi_8840_: bool;
    var phi_4634_: type_37;
    var phi_4637_: f32;
    var phi_4639_: f32;
    var phi_4651_: bool;
    var phi_4679_: type_38;
    var phi_4691_: type_37;
    var phi_4635_: type_37;
    var phi_4694_: type_38;
    var phi_4705_: type_37;
    var phi_4708_: f32;
    var phi_4710_: f32;
    var phi_4722_: bool;
    var phi_4750_: type_38;
    var phi_4762_: type_37;
    var phi_4706_: type_37;
    var phi_4765_: type_38;
    var phi_7760_: f32;
    var phi_7773_: bool;
    var phi_4831_: f32;
    var phi_4826_: f32;
    var phi_4832_: f32;
    var phi_7790_: bool;
    var phi_4797_: f32;
    var phi_4834_: f32;
    var phi_7808_: f32;
    var phi_7821_: bool;
    var phi_4887_: f32;
    var phi_4882_: f32;
    var phi_4888_: f32;
    var phi_7838_: bool;
    var phi_4853_: f32;
    var phi_4890_: f32;
    var phi_4949_: f32;
    var phi_4709_: f32;
    var phi_4711_: f32;
    var phi_4951_: bool;
    var phi_8824_: bool;
    var phi_8921_: bool;
    var phi_4638_: f32;
    var phi_4640_: f32;
    var phi_4952_: bool;
    var phi_8920_: bool;
    var local_3: f32;
    var local_4: f32;
    var phi_9048_: bool;
    var phi_4955_: f32;
    var phi_9047_: bool;
    var phi_4956_: f32;
    var phi_9046_: bool;
    var phi_4957_: f32;
    var phi_4958_: vec3<f32>;
    var phi_7875_: bool;
    var phi_3368_: type_36;
    var phi_7922_: vec3<f32>;
    var phi_7957_: vec3<f32>;
    var phi_3642_: type_14;
    var phi_3645_: f32;
    var phi_3643_: type_14;
    var phi_3668_: type_14;
    var phi_8078_: vec3<f32>;
    var phi_8112_: vec2<f32>;
    var phi_8115_: vec2<f32>;
    var phi_8116_: u32;
    var phi_8119_: vec2<f32>;
    var phi_8120_: u32;
    var phi_8121_: bool;
    var phi_8146_: vec2<f32>;
    var phi_8149_: vec2<f32>;
    var phi_8150_: u32;
    var phi_8153_: vec2<f32>;
    var phi_8154_: u32;
    var phi_8155_: bool;
    var phi_8176_: vec2<f32>;
    var phi_8179_: vec2<f32>;
    var phi_8180_: u32;
    var phi_8182_: vec2<f32>;
    var phi_8183_: u32;
    var phi_3705_: u32;
    var phi_3806_: u32;
    var phi_3837_: u32;
    var phi_3846_: u32;
    var phi_8213_: f32;
    var phi_8226_: bool;
    var phi_3906_: f32;
    var phi_3901_: f32;
    var phi_3907_: f32;
    var phi_8243_: bool;
    var phi_3872_: f32;
    var phi_3909_: f32;
    var phi_8261_: f32;
    var phi_8274_: bool;
    var phi_3964_: f32;
    var phi_3959_: f32;
    var phi_3965_: f32;
    var phi_8291_: bool;
    var phi_3930_: f32;
    var phi_3967_: f32;
    var phi_4026_: f32;
    var phi_3646_: f32;
    var phi_4027_: bool;
    var phi_8922_: bool;
    var local_5: f32;
    var phi_9050_: bool;
    var phi_4029_: f32;
    var phi_9049_: bool;
    var phi_4030_: f32;
    var phi_4031_: vec3<f32>;
    var phi_8328_: bool;
    var phi_2572_: type_36;
    var phi_8375_: vec3<f32>;
    var phi_8410_: vec3<f32>;
    var phi_8515_: bool;
    var phi_8518_: bool;
    var phi_8519_: bool;
    var phi_2968_: u32;
    var phi_2977_: u32;
    var phi_8994_: bool;
    var phi_3005_: type_37;
    var phi_3008_: f32;
    var phi_3010_: f32;
    var phi_3022_: bool;
    var phi_3050_: type_38;
    var phi_3062_: type_37;
    var phi_3006_: type_37;
    var phi_3065_: type_38;
    var phi_3076_: type_37;
    var phi_3079_: f32;
    var phi_3081_: f32;
    var phi_3093_: bool;
    var phi_3121_: type_38;
    var phi_3133_: type_37;
    var phi_3077_: type_37;
    var phi_3136_: type_38;
    var phi_8541_: f32;
    var phi_8554_: bool;
    var phi_3202_: f32;
    var phi_3197_: f32;
    var phi_3203_: f32;
    var phi_8571_: bool;
    var phi_3168_: f32;
    var phi_3205_: f32;
    var phi_8589_: f32;
    var phi_8602_: bool;
    var phi_3258_: f32;
    var phi_3253_: f32;
    var phi_3259_: f32;
    var phi_8619_: bool;
    var phi_3224_: f32;
    var phi_3261_: f32;
    var phi_3320_: f32;
    var phi_3080_: f32;
    var phi_3082_: f32;
    var phi_3322_: bool;
    var phi_8978_: bool;
    var phi_9040_: bool;
    var phi_3009_: f32;
    var phi_3011_: f32;
    var phi_3323_: bool;
    var phi_9039_: bool;
    var local_6: f32;
    var local_7: f32;
    var phi_9052_: bool;
    var phi_3326_: f32;
    var phi_9051_: bool;
    var phi_3327_: f32;
    var phi_9045_: bool;
    var phi_4960_: f32;
    var phi_4961_: vec3<f32>;
    var phi_4962_: bool;
    var phi_4982_: vec3<f32>;
    var phi_9042_: bool;
    var phi_2409_: vec3<f32>;
    var phi_4988_: bool;
    var phi_9041_: bool;
    var local_8: vec3<f32>;
    var local_9: vec3<f32>;
    var local_10: vec3<f32>;
    var phi_9061_: bool;
    var phi_5105_: vec4<f32>;
    var phi_9053_: bool;
    var local_11: f32;
    var local_12: f32;
    var local_13: f32;
    var local_14: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e146 = arrayLength((&global.member));
            let _e148 = arrayLength((&global_1.member));
            let _e149 = global_2;
            let _e150 = global_3;
            let _e151 = global_4;
            let _e152 = global_5;
            let _e153 = global_6;
            let _e154 = global_7;
            let _e155 = global_8;
            let _e156 = global_9;
            let _e160 = global.member[(_e149 + 9u)];
            let _e164 = global.member[(_e149 + 11u)];
            let _e168 = global.member[(_e149 + 17u)];
            let _e171 = global.member[_e168];
            let _e175 = global.member[(_e168 + 1u)];
            let _e179 = global.member[(_e168 + 4u)];
            switch bitcast<i32>(_e179) {
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
            let _e182 = phi_722_;
            let _e186 = global.member[(_e168 + 5u)];
            if (_e164 == 4294967295u) {
                phi_854_ = type_35(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
            } else {
                if (_e146 >= 22u) {
                    phi_5788_ = (_e164 <= (_e146 - 22u));
                } else {
                    phi_5788_ = false;
                }
                let _e193 = phi_5788_;
                if _e193 {
                    let _e196 = global.member[_e164];
                    let _e201 = global.member[(_e164 + 1u)];
                    let _e206 = global.member[(_e164 + 2u)];
                    let _e212 = global.member[(_e164 + 3u)];
                    let _e217 = global.member[(_e164 + 4u)];
                    let _e222 = global.member[(_e164 + 5u)];
                    let _e227 = global.member[(_e164 + 6u)];
                    let _e232 = global.member[(_e164 + 7u)];
                    let _e238 = global.member[(_e164 + 8u)];
                    let _e243 = global.member[(_e164 + 9u)];
                    let _e248 = global.member[(_e164 + 10u)];
                    let _e252 = global.member[(_e164 + 11u)];
                    let _e256 = global.member[(_e164 + 12u)];
                    let _e260 = global.member[(_e164 + 13u)];
                    let _e264 = global.member[(_e164 + 14u)];
                    let _e268 = global.member[(_e164 + 15u)];
                    let _e272 = global.member[(_e164 + 16u)];
                    let _e276 = global.member[(_e164 + 17u)];
                    let _e280 = global.member[(_e164 + 18u)];
                    let _e284 = global.member[(_e164 + 19u)];
                    let _e288 = global.member[(_e164 + 20u)];
                    let _e293 = global.member[(_e164 + 21u)];
                    phi_850_ = type_35(vec3<f32>(bitcast<f32>(_e196), bitcast<f32>(_e201), bitcast<f32>(_e206)), bitcast<f32>(_e212), vec4<f32>(bitcast<f32>(_e217), bitcast<f32>(_e222), bitcast<f32>(_e227), bitcast<f32>(_e232)), bitcast<f32>(_e238), bitcast<f32>(_e243), _e248, _e252, _e256, _e260, _e264, _e268, _e272, _e276, _e280, _e284, (_e288 == 1u), bitcast<f32>(_e293));
                } else {
                    phi_850_ = type_35(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
                }
                let _e297 = phi_850_;
                phi_854_ = type_35(_e297.member, _e297.member_1, _e297.member_2, _e297.member_3, _e297.member_4, _e297.member_5, _e297.member_6, _e297.member_7, _e297.member_8, _e297.member_9, _e297.member_10, _e297.member_11, _e297.member_12, _e297.member_13, _e297.member_14, (_e297.member_15 && (_e186 == 1u)), _e297.member_16);
            }
            let _e319 = phi_854_;
            let _e323 = select(_e152, _e151, vec2((_e319.member_10 == 0u)));
            let _e325 = (_e146 >= 8u);
            if _e325 {
                phi_5825_ = (_e319.member_5 <= (_e146 - 8u));
            } else {
                phi_5825_ = false;
            }
            let _e329 = phi_5825_;
            if _e329 {
                let _e332 = global.member[_e319.member_5];
                let _e336 = global.member[(_e319.member_5 + 1u)];
                let _e341 = global.member[(_e319.member_5 + 2u)];
                let _e345 = global.member[(_e319.member_5 + 3u)];
                let _e350 = global.member[(_e319.member_5 + 4u)];
                let _e354 = global.member[(_e319.member_5 + 5u)];
                let _e358 = global.member[(_e319.member_5 + 6u)];
                switch bitcast<i32>(_e358) {
                    case 0: {
                        phi_894_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_894_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_894_ = 2u;
                        break;
                    }
                    default: {
                        phi_894_ = 0u;
                        break;
                    }
                }
                let _e361 = phi_894_;
                let _e365 = global.member[(_e319.member_5 + 7u)];
                switch bitcast<i32>(_e365) {
                    case 0: {
                        phi_903_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_903_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_903_ = 2u;
                        break;
                    }
                    default: {
                        phi_903_ = 0u;
                        break;
                    }
                }
                let _e368 = phi_903_;
                phi_916_ = type_15(type_14(_e361, _e368), vec2<u32>(_e332, _e336), vec2<u32>(_e341, _e345), _e350, _e354);
            } else {
                phi_916_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e372 = phi_916_;
            switch bitcast<i32>(_e372.member.member) {
                case 1: {
                    let _e410 = abs(_e323.x);
                    let _e412 = (_e410 % 1f);
                    if (_e410 >= 1f) {
                        phi_5877_ = select(true, false, (_e412 == 0f));
                    } else {
                        phi_5877_ = true;
                    }
                    let _e416 = phi_5877_;
                    let _e417 = select(1f, _e412, _e416);
                    if (select(-1f, 1f, (_e323.x >= 0f)) > 0f) {
                        phi_936_ = _e417;
                    } else {
                        phi_936_ = (1f - _e417);
                    }
                    let _e421 = phi_936_;
                    phi_973_ = _e421;
                    break;
                }
                case 2: {
                    let _e384 = abs(_e323.x);
                    let _e391 = ((select(select(u32(_e384), 0u, (_e384 < 0f)), 4294967295u, (_e384 > 4294967000f)) % 2u) == 0u);
                    let _e393 = (_e384 % 1f);
                    if (_e384 >= 1f) {
                        phi_5860_ = select(true, false, (_e393 == 0f));
                    } else {
                        phi_5860_ = true;
                    }
                    let _e397 = phi_5860_;
                    let _e398 = select(1f, _e393, _e397);
                    if (select(-1f, 1f, (_e323.x >= 0f)) > 0f) {
                        if _e391 {
                            phi_965_ = _e398;
                        } else {
                            phi_965_ = (1f - _e398);
                        }
                        let _e405 = phi_965_;
                        phi_971_ = _e405;
                    } else {
                        if _e391 {
                            phi_970_ = (1f - _e398);
                        } else {
                            phi_970_ = _e398;
                        }
                        let _e402 = phi_970_;
                        phi_971_ = _e402;
                    }
                    let _e407 = phi_971_;
                    phi_973_ = _e407;
                    break;
                }
                case 0: {
                    if (_e323.x > 1f) {
                        phi_5847_ = 0.9999999f;
                    } else {
                        phi_5847_ = select(_e323.x, 0.00000011920929f, (_e323.x < 0f));
                    }
                    let _e381 = phi_5847_;
                    phi_973_ = _e381;
                    break;
                }
                default: {
                    phi_973_ = f32();
                    break;
                }
            }
            let _e423 = phi_973_;
            switch bitcast<i32>(_e372.member.member_1) {
                case 1: {
                    let _e461 = abs(_e323.y);
                    let _e463 = (_e461 % 1f);
                    if (_e461 >= 1f) {
                        phi_5925_ = select(true, false, (_e463 == 0f));
                    } else {
                        phi_5925_ = true;
                    }
                    let _e467 = phi_5925_;
                    let _e468 = select(1f, _e463, _e467);
                    if (select(-1f, 1f, (_e323.y >= 0f)) > 0f) {
                        phi_994_ = _e468;
                    } else {
                        phi_994_ = (1f - _e468);
                    }
                    let _e472 = phi_994_;
                    phi_1031_ = _e472;
                    break;
                }
                case 2: {
                    let _e435 = abs(_e323.y);
                    let _e442 = ((select(select(u32(_e435), 0u, (_e435 < 0f)), 4294967295u, (_e435 > 4294967000f)) % 2u) == 0u);
                    let _e444 = (_e435 % 1f);
                    if (_e435 >= 1f) {
                        phi_5908_ = select(true, false, (_e444 == 0f));
                    } else {
                        phi_5908_ = true;
                    }
                    let _e448 = phi_5908_;
                    let _e449 = select(1f, _e444, _e448);
                    if (select(-1f, 1f, (_e323.y >= 0f)) > 0f) {
                        if _e442 {
                            phi_1023_ = _e449;
                        } else {
                            phi_1023_ = (1f - _e449);
                        }
                        let _e456 = phi_1023_;
                        phi_1029_ = _e456;
                    } else {
                        if _e442 {
                            phi_1028_ = (1f - _e449);
                        } else {
                            phi_1028_ = _e449;
                        }
                        let _e453 = phi_1028_;
                        phi_1029_ = _e453;
                    }
                    let _e458 = phi_1029_;
                    phi_1031_ = _e458;
                    break;
                }
                case 0: {
                    if (_e323.y > 1f) {
                        phi_5895_ = 0.9999999f;
                    } else {
                        phi_5895_ = select(_e323.y, 0.00000011920929f, (_e323.y < 0f));
                    }
                    let _e432 = phi_5895_;
                    phi_1031_ = _e432;
                    break;
                }
                default: {
                    phi_1031_ = f32();
                    break;
                }
            }
            let _e474 = phi_1031_;
            let _e478 = (_e423 * f32(_e372.member_2.x));
            let _e487 = (_e474 * f32(_e372.member_2.y));
            let _e499 = f32(_e171);
            let _e500 = f32(_e175);
            let _e507 = vec3<f32>((f32((select(select(u32(_e478), 0u, (_e478 < 0f)), 4294967295u, (_e478 > 4294967000f)) + _e372.member_1.x)) / _e499), (f32((select(select(u32(_e487), 0u, (_e487 < 0f)), 4294967295u, (_e487 > 4294967000f)) + _e372.member_1.y)) / _e500), f32(_e372.member_3));
            let _e513 = textureSampleLevel(global_11, global_10, vec2<f32>(_e507.x, _e507.y), i32(_e507.z), 0f);
            let _e516 = select(_e513, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e319.member_5 == 4294967295u)));
            let _e520 = select(_e152, _e151, vec2((_e319.member_11 == 0u)));
            if _e325 {
                phi_5961_ = (_e319.member_6 <= (_e146 - 8u));
            } else {
                phi_5961_ = false;
            }
            let _e525 = phi_5961_;
            if _e525 {
                let _e528 = global.member[_e319.member_6];
                let _e532 = global.member[(_e319.member_6 + 1u)];
                let _e537 = global.member[(_e319.member_6 + 2u)];
                let _e541 = global.member[(_e319.member_6 + 3u)];
                let _e546 = global.member[(_e319.member_6 + 4u)];
                let _e550 = global.member[(_e319.member_6 + 5u)];
                let _e554 = global.member[(_e319.member_6 + 6u)];
                switch bitcast<i32>(_e554) {
                    case 0: {
                        phi_1114_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1114_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1114_ = 2u;
                        break;
                    }
                    default: {
                        phi_1114_ = 0u;
                        break;
                    }
                }
                let _e557 = phi_1114_;
                let _e561 = global.member[(_e319.member_6 + 7u)];
                switch bitcast<i32>(_e561) {
                    case 0: {
                        phi_1123_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1123_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1123_ = 2u;
                        break;
                    }
                    default: {
                        phi_1123_ = 0u;
                        break;
                    }
                }
                let _e564 = phi_1123_;
                phi_1136_ = type_15(type_14(_e557, _e564), vec2<u32>(_e528, _e532), vec2<u32>(_e537, _e541), _e546, _e550);
            } else {
                phi_1136_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e568 = phi_1136_;
            switch bitcast<i32>(_e568.member.member) {
                case 1: {
                    let _e606 = abs(_e520.x);
                    let _e608 = (_e606 % 1f);
                    if (_e606 >= 1f) {
                        phi_6012_ = select(true, false, (_e608 == 0f));
                    } else {
                        phi_6012_ = true;
                    }
                    let _e612 = phi_6012_;
                    let _e613 = select(1f, _e608, _e612);
                    if (select(-1f, 1f, (_e520.x >= 0f)) > 0f) {
                        phi_1156_ = _e613;
                    } else {
                        phi_1156_ = (1f - _e613);
                    }
                    let _e617 = phi_1156_;
                    phi_1193_ = _e617;
                    break;
                }
                case 2: {
                    let _e580 = abs(_e520.x);
                    let _e587 = ((select(select(u32(_e580), 0u, (_e580 < 0f)), 4294967295u, (_e580 > 4294967000f)) % 2u) == 0u);
                    let _e589 = (_e580 % 1f);
                    if (_e580 >= 1f) {
                        phi_5995_ = select(true, false, (_e589 == 0f));
                    } else {
                        phi_5995_ = true;
                    }
                    let _e593 = phi_5995_;
                    let _e594 = select(1f, _e589, _e593);
                    if (select(-1f, 1f, (_e520.x >= 0f)) > 0f) {
                        if _e587 {
                            phi_1185_ = _e594;
                        } else {
                            phi_1185_ = (1f - _e594);
                        }
                        let _e601 = phi_1185_;
                        phi_1191_ = _e601;
                    } else {
                        if _e587 {
                            phi_1190_ = (1f - _e594);
                        } else {
                            phi_1190_ = _e594;
                        }
                        let _e598 = phi_1190_;
                        phi_1191_ = _e598;
                    }
                    let _e603 = phi_1191_;
                    phi_1193_ = _e603;
                    break;
                }
                case 0: {
                    if (_e520.x > 1f) {
                        phi_5982_ = 0.9999999f;
                    } else {
                        phi_5982_ = select(_e520.x, 0.00000011920929f, (_e520.x < 0f));
                    }
                    let _e577 = phi_5982_;
                    phi_1193_ = _e577;
                    break;
                }
                default: {
                    phi_1193_ = f32();
                    break;
                }
            }
            let _e619 = phi_1193_;
            switch bitcast<i32>(_e568.member.member_1) {
                case 1: {
                    let _e657 = abs(_e520.y);
                    let _e659 = (_e657 % 1f);
                    if (_e657 >= 1f) {
                        phi_6060_ = select(true, false, (_e659 == 0f));
                    } else {
                        phi_6060_ = true;
                    }
                    let _e663 = phi_6060_;
                    let _e664 = select(1f, _e659, _e663);
                    if (select(-1f, 1f, (_e520.y >= 0f)) > 0f) {
                        phi_1214_ = _e664;
                    } else {
                        phi_1214_ = (1f - _e664);
                    }
                    let _e668 = phi_1214_;
                    phi_1251_ = _e668;
                    break;
                }
                case 2: {
                    let _e631 = abs(_e520.y);
                    let _e638 = ((select(select(u32(_e631), 0u, (_e631 < 0f)), 4294967295u, (_e631 > 4294967000f)) % 2u) == 0u);
                    let _e640 = (_e631 % 1f);
                    if (_e631 >= 1f) {
                        phi_6043_ = select(true, false, (_e640 == 0f));
                    } else {
                        phi_6043_ = true;
                    }
                    let _e644 = phi_6043_;
                    let _e645 = select(1f, _e640, _e644);
                    if (select(-1f, 1f, (_e520.y >= 0f)) > 0f) {
                        if _e638 {
                            phi_1243_ = _e645;
                        } else {
                            phi_1243_ = (1f - _e645);
                        }
                        let _e652 = phi_1243_;
                        phi_1249_ = _e652;
                    } else {
                        if _e638 {
                            phi_1248_ = (1f - _e645);
                        } else {
                            phi_1248_ = _e645;
                        }
                        let _e649 = phi_1248_;
                        phi_1249_ = _e649;
                    }
                    let _e654 = phi_1249_;
                    phi_1251_ = _e654;
                    break;
                }
                case 0: {
                    if (_e520.y > 1f) {
                        phi_6030_ = 0.9999999f;
                    } else {
                        phi_6030_ = select(_e520.y, 0.00000011920929f, (_e520.y < 0f));
                    }
                    let _e628 = phi_6030_;
                    phi_1251_ = _e628;
                    break;
                }
                default: {
                    phi_1251_ = f32();
                    break;
                }
            }
            let _e670 = phi_1251_;
            let _e674 = (_e619 * f32(_e568.member_2.x));
            let _e683 = (_e670 * f32(_e568.member_2.y));
            let _e701 = vec3<f32>((f32((select(select(u32(_e674), 0u, (_e674 < 0f)), 4294967295u, (_e674 > 4294967000f)) + _e568.member_1.x)) / _e499), (f32((select(select(u32(_e683), 0u, (_e683 < 0f)), 4294967295u, (_e683 > 4294967000f)) + _e568.member_1.y)) / _e500), f32(_e568.member_3));
            let _e707 = textureSampleLevel(global_11, global_10, vec2<f32>(_e701.x, _e701.y), i32(_e701.z), 0f);
            let _e710 = select(_e707, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e319.member_6 == 4294967295u)));
            let _e714 = select(_e152, _e151, vec2((_e319.member_12 == 0u)));
            if _e325 {
                phi_6096_ = (_e319.member_7 <= (_e146 - 8u));
            } else {
                phi_6096_ = false;
            }
            let _e719 = phi_6096_;
            if _e719 {
                let _e722 = global.member[_e319.member_7];
                let _e726 = global.member[(_e319.member_7 + 1u)];
                let _e731 = global.member[(_e319.member_7 + 2u)];
                let _e735 = global.member[(_e319.member_7 + 3u)];
                let _e740 = global.member[(_e319.member_7 + 4u)];
                let _e744 = global.member[(_e319.member_7 + 5u)];
                let _e748 = global.member[(_e319.member_7 + 6u)];
                switch bitcast<i32>(_e748) {
                    case 0: {
                        phi_1334_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1334_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1334_ = 2u;
                        break;
                    }
                    default: {
                        phi_1334_ = 0u;
                        break;
                    }
                }
                let _e751 = phi_1334_;
                let _e755 = global.member[(_e319.member_7 + 7u)];
                switch bitcast<i32>(_e755) {
                    case 0: {
                        phi_1343_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1343_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1343_ = 2u;
                        break;
                    }
                    default: {
                        phi_1343_ = 0u;
                        break;
                    }
                }
                let _e758 = phi_1343_;
                phi_1356_ = type_15(type_14(_e751, _e758), vec2<u32>(_e722, _e726), vec2<u32>(_e731, _e735), _e740, _e744);
            } else {
                phi_1356_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e762 = phi_1356_;
            switch bitcast<i32>(_e762.member.member) {
                case 1: {
                    let _e800 = abs(_e714.x);
                    let _e802 = (_e800 % 1f);
                    if (_e800 >= 1f) {
                        phi_6147_ = select(true, false, (_e802 == 0f));
                    } else {
                        phi_6147_ = true;
                    }
                    let _e806 = phi_6147_;
                    let _e807 = select(1f, _e802, _e806);
                    if (select(-1f, 1f, (_e714.x >= 0f)) > 0f) {
                        phi_1376_ = _e807;
                    } else {
                        phi_1376_ = (1f - _e807);
                    }
                    let _e811 = phi_1376_;
                    phi_1413_ = _e811;
                    break;
                }
                case 2: {
                    let _e774 = abs(_e714.x);
                    let _e781 = ((select(select(u32(_e774), 0u, (_e774 < 0f)), 4294967295u, (_e774 > 4294967000f)) % 2u) == 0u);
                    let _e783 = (_e774 % 1f);
                    if (_e774 >= 1f) {
                        phi_6130_ = select(true, false, (_e783 == 0f));
                    } else {
                        phi_6130_ = true;
                    }
                    let _e787 = phi_6130_;
                    let _e788 = select(1f, _e783, _e787);
                    if (select(-1f, 1f, (_e714.x >= 0f)) > 0f) {
                        if _e781 {
                            phi_1405_ = _e788;
                        } else {
                            phi_1405_ = (1f - _e788);
                        }
                        let _e795 = phi_1405_;
                        phi_1411_ = _e795;
                    } else {
                        if _e781 {
                            phi_1410_ = (1f - _e788);
                        } else {
                            phi_1410_ = _e788;
                        }
                        let _e792 = phi_1410_;
                        phi_1411_ = _e792;
                    }
                    let _e797 = phi_1411_;
                    phi_1413_ = _e797;
                    break;
                }
                case 0: {
                    if (_e714.x > 1f) {
                        phi_6117_ = 0.9999999f;
                    } else {
                        phi_6117_ = select(_e714.x, 0.00000011920929f, (_e714.x < 0f));
                    }
                    let _e771 = phi_6117_;
                    phi_1413_ = _e771;
                    break;
                }
                default: {
                    phi_1413_ = f32();
                    break;
                }
            }
            let _e813 = phi_1413_;
            switch bitcast<i32>(_e762.member.member_1) {
                case 1: {
                    let _e851 = abs(_e714.y);
                    let _e853 = (_e851 % 1f);
                    if (_e851 >= 1f) {
                        phi_6195_ = select(true, false, (_e853 == 0f));
                    } else {
                        phi_6195_ = true;
                    }
                    let _e857 = phi_6195_;
                    let _e858 = select(1f, _e853, _e857);
                    if (select(-1f, 1f, (_e714.y >= 0f)) > 0f) {
                        phi_1434_ = _e858;
                    } else {
                        phi_1434_ = (1f - _e858);
                    }
                    let _e862 = phi_1434_;
                    phi_1471_ = _e862;
                    break;
                }
                case 2: {
                    let _e825 = abs(_e714.y);
                    let _e832 = ((select(select(u32(_e825), 0u, (_e825 < 0f)), 4294967295u, (_e825 > 4294967000f)) % 2u) == 0u);
                    let _e834 = (_e825 % 1f);
                    if (_e825 >= 1f) {
                        phi_6178_ = select(true, false, (_e834 == 0f));
                    } else {
                        phi_6178_ = true;
                    }
                    let _e838 = phi_6178_;
                    let _e839 = select(1f, _e834, _e838);
                    if (select(-1f, 1f, (_e714.y >= 0f)) > 0f) {
                        if _e832 {
                            phi_1463_ = _e839;
                        } else {
                            phi_1463_ = (1f - _e839);
                        }
                        let _e846 = phi_1463_;
                        phi_1469_ = _e846;
                    } else {
                        if _e832 {
                            phi_1468_ = (1f - _e839);
                        } else {
                            phi_1468_ = _e839;
                        }
                        let _e843 = phi_1468_;
                        phi_1469_ = _e843;
                    }
                    let _e848 = phi_1469_;
                    phi_1471_ = _e848;
                    break;
                }
                case 0: {
                    if (_e714.y > 1f) {
                        phi_6165_ = 0.9999999f;
                    } else {
                        phi_6165_ = select(_e714.y, 0.00000011920929f, (_e714.y < 0f));
                    }
                    let _e822 = phi_6165_;
                    phi_1471_ = _e822;
                    break;
                }
                default: {
                    phi_1471_ = f32();
                    break;
                }
            }
            let _e864 = phi_1471_;
            let _e868 = (_e813 * f32(_e762.member_2.x));
            let _e877 = (_e864 * f32(_e762.member_2.y));
            let _e895 = vec3<f32>((f32((select(select(u32(_e868), 0u, (_e868 < 0f)), 4294967295u, (_e868 > 4294967000f)) + _e762.member_1.x)) / _e499), (f32((select(select(u32(_e877), 0u, (_e877 < 0f)), 4294967295u, (_e877 > 4294967000f)) + _e762.member_1.y)) / _e500), f32(_e762.member_3));
            let _e901 = textureSampleLevel(global_11, global_10, vec2<f32>(_e895.x, _e895.y), i32(_e895.z), 0f);
            let _e902 = (_e319.member_7 == 4294967295u);
            let _e904 = select(_e901, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e902));
            let _e908 = select(_e152, _e151, vec2((_e319.member_13 == 0u)));
            if _e325 {
                phi_6231_ = (_e319.member_8 <= (_e146 - 8u));
            } else {
                phi_6231_ = false;
            }
            let _e913 = phi_6231_;
            if _e913 {
                let _e916 = global.member[_e319.member_8];
                let _e920 = global.member[(_e319.member_8 + 1u)];
                let _e925 = global.member[(_e319.member_8 + 2u)];
                let _e929 = global.member[(_e319.member_8 + 3u)];
                let _e934 = global.member[(_e319.member_8 + 4u)];
                let _e938 = global.member[(_e319.member_8 + 5u)];
                let _e942 = global.member[(_e319.member_8 + 6u)];
                switch bitcast<i32>(_e942) {
                    case 0: {
                        phi_1554_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1554_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1554_ = 2u;
                        break;
                    }
                    default: {
                        phi_1554_ = 0u;
                        break;
                    }
                }
                let _e945 = phi_1554_;
                let _e949 = global.member[(_e319.member_8 + 7u)];
                switch bitcast<i32>(_e949) {
                    case 0: {
                        phi_1563_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1563_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1563_ = 2u;
                        break;
                    }
                    default: {
                        phi_1563_ = 0u;
                        break;
                    }
                }
                let _e952 = phi_1563_;
                phi_1576_ = type_15(type_14(_e945, _e952), vec2<u32>(_e916, _e920), vec2<u32>(_e925, _e929), _e934, _e938);
            } else {
                phi_1576_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e956 = phi_1576_;
            switch bitcast<i32>(_e956.member.member) {
                case 1: {
                    let _e994 = abs(_e908.x);
                    let _e996 = (_e994 % 1f);
                    if (_e994 >= 1f) {
                        phi_6282_ = select(true, false, (_e996 == 0f));
                    } else {
                        phi_6282_ = true;
                    }
                    let _e1000 = phi_6282_;
                    let _e1001 = select(1f, _e996, _e1000);
                    if (select(-1f, 1f, (_e908.x >= 0f)) > 0f) {
                        phi_1596_ = _e1001;
                    } else {
                        phi_1596_ = (1f - _e1001);
                    }
                    let _e1005 = phi_1596_;
                    phi_1633_ = _e1005;
                    break;
                }
                case 2: {
                    let _e968 = abs(_e908.x);
                    let _e975 = ((select(select(u32(_e968), 0u, (_e968 < 0f)), 4294967295u, (_e968 > 4294967000f)) % 2u) == 0u);
                    let _e977 = (_e968 % 1f);
                    if (_e968 >= 1f) {
                        phi_6265_ = select(true, false, (_e977 == 0f));
                    } else {
                        phi_6265_ = true;
                    }
                    let _e981 = phi_6265_;
                    let _e982 = select(1f, _e977, _e981);
                    if (select(-1f, 1f, (_e908.x >= 0f)) > 0f) {
                        if _e975 {
                            phi_1625_ = _e982;
                        } else {
                            phi_1625_ = (1f - _e982);
                        }
                        let _e989 = phi_1625_;
                        phi_1631_ = _e989;
                    } else {
                        if _e975 {
                            phi_1630_ = (1f - _e982);
                        } else {
                            phi_1630_ = _e982;
                        }
                        let _e986 = phi_1630_;
                        phi_1631_ = _e986;
                    }
                    let _e991 = phi_1631_;
                    phi_1633_ = _e991;
                    break;
                }
                case 0: {
                    if (_e908.x > 1f) {
                        phi_6252_ = 0.9999999f;
                    } else {
                        phi_6252_ = select(_e908.x, 0.00000011920929f, (_e908.x < 0f));
                    }
                    let _e965 = phi_6252_;
                    phi_1633_ = _e965;
                    break;
                }
                default: {
                    phi_1633_ = f32();
                    break;
                }
            }
            let _e1007 = phi_1633_;
            switch bitcast<i32>(_e956.member.member_1) {
                case 1: {
                    let _e1045 = abs(_e908.y);
                    let _e1047 = (_e1045 % 1f);
                    if (_e1045 >= 1f) {
                        phi_6330_ = select(true, false, (_e1047 == 0f));
                    } else {
                        phi_6330_ = true;
                    }
                    let _e1051 = phi_6330_;
                    let _e1052 = select(1f, _e1047, _e1051);
                    if (select(-1f, 1f, (_e908.y >= 0f)) > 0f) {
                        phi_1654_ = _e1052;
                    } else {
                        phi_1654_ = (1f - _e1052);
                    }
                    let _e1056 = phi_1654_;
                    phi_1691_ = _e1056;
                    break;
                }
                case 2: {
                    let _e1019 = abs(_e908.y);
                    let _e1026 = ((select(select(u32(_e1019), 0u, (_e1019 < 0f)), 4294967295u, (_e1019 > 4294967000f)) % 2u) == 0u);
                    let _e1028 = (_e1019 % 1f);
                    if (_e1019 >= 1f) {
                        phi_6313_ = select(true, false, (_e1028 == 0f));
                    } else {
                        phi_6313_ = true;
                    }
                    let _e1032 = phi_6313_;
                    let _e1033 = select(1f, _e1028, _e1032);
                    if (select(-1f, 1f, (_e908.y >= 0f)) > 0f) {
                        if _e1026 {
                            phi_1683_ = _e1033;
                        } else {
                            phi_1683_ = (1f - _e1033);
                        }
                        let _e1040 = phi_1683_;
                        phi_1689_ = _e1040;
                    } else {
                        if _e1026 {
                            phi_1688_ = (1f - _e1033);
                        } else {
                            phi_1688_ = _e1033;
                        }
                        let _e1037 = phi_1688_;
                        phi_1689_ = _e1037;
                    }
                    let _e1042 = phi_1689_;
                    phi_1691_ = _e1042;
                    break;
                }
                case 0: {
                    if (_e908.y > 1f) {
                        phi_6300_ = 0.9999999f;
                    } else {
                        phi_6300_ = select(_e908.y, 0.00000011920929f, (_e908.y < 0f));
                    }
                    let _e1016 = phi_6300_;
                    phi_1691_ = _e1016;
                    break;
                }
                default: {
                    phi_1691_ = f32();
                    break;
                }
            }
            let _e1058 = phi_1691_;
            let _e1062 = (_e1007 * f32(_e956.member_2.x));
            let _e1071 = (_e1058 * f32(_e956.member_2.y));
            let _e1089 = vec3<f32>((f32((select(select(u32(_e1062), 0u, (_e1062 < 0f)), 4294967295u, (_e1062 > 4294967000f)) + _e956.member_1.x)) / _e499), (f32((select(select(u32(_e1071), 0u, (_e1071 < 0f)), 4294967295u, (_e1071 > 4294967000f)) + _e956.member_1.y)) / _e500), f32(_e956.member_3));
            let _e1095 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1089.x, _e1089.y), i32(_e1089.z), 0f);
            let _e1102 = select(_e152, _e151, vec2((_e319.member_14 == 0u)));
            if _e325 {
                phi_6366_ = (_e319.member_9 <= (_e146 - 8u));
            } else {
                phi_6366_ = false;
            }
            let _e1107 = phi_6366_;
            if _e1107 {
                let _e1110 = global.member[_e319.member_9];
                let _e1114 = global.member[(_e319.member_9 + 1u)];
                let _e1119 = global.member[(_e319.member_9 + 2u)];
                let _e1123 = global.member[(_e319.member_9 + 3u)];
                let _e1128 = global.member[(_e319.member_9 + 4u)];
                let _e1132 = global.member[(_e319.member_9 + 5u)];
                let _e1136 = global.member[(_e319.member_9 + 6u)];
                switch bitcast<i32>(_e1136) {
                    case 0: {
                        phi_1774_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1774_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1774_ = 2u;
                        break;
                    }
                    default: {
                        phi_1774_ = 0u;
                        break;
                    }
                }
                let _e1139 = phi_1774_;
                let _e1143 = global.member[(_e319.member_9 + 7u)];
                switch bitcast<i32>(_e1143) {
                    case 0: {
                        phi_1783_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1783_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1783_ = 2u;
                        break;
                    }
                    default: {
                        phi_1783_ = 0u;
                        break;
                    }
                }
                let _e1146 = phi_1783_;
                phi_1796_ = type_15(type_14(_e1139, _e1146), vec2<u32>(_e1110, _e1114), vec2<u32>(_e1119, _e1123), _e1128, _e1132);
            } else {
                phi_1796_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e1150 = phi_1796_;
            switch bitcast<i32>(_e1150.member.member) {
                case 1: {
                    let _e1188 = abs(_e1102.x);
                    let _e1190 = (_e1188 % 1f);
                    if (_e1188 >= 1f) {
                        phi_6417_ = select(true, false, (_e1190 == 0f));
                    } else {
                        phi_6417_ = true;
                    }
                    let _e1194 = phi_6417_;
                    let _e1195 = select(1f, _e1190, _e1194);
                    if (select(-1f, 1f, (_e1102.x >= 0f)) > 0f) {
                        phi_1816_ = _e1195;
                    } else {
                        phi_1816_ = (1f - _e1195);
                    }
                    let _e1199 = phi_1816_;
                    phi_1853_ = _e1199;
                    break;
                }
                case 2: {
                    let _e1162 = abs(_e1102.x);
                    let _e1169 = ((select(select(u32(_e1162), 0u, (_e1162 < 0f)), 4294967295u, (_e1162 > 4294967000f)) % 2u) == 0u);
                    let _e1171 = (_e1162 % 1f);
                    if (_e1162 >= 1f) {
                        phi_6400_ = select(true, false, (_e1171 == 0f));
                    } else {
                        phi_6400_ = true;
                    }
                    let _e1175 = phi_6400_;
                    let _e1176 = select(1f, _e1171, _e1175);
                    if (select(-1f, 1f, (_e1102.x >= 0f)) > 0f) {
                        if _e1169 {
                            phi_1845_ = _e1176;
                        } else {
                            phi_1845_ = (1f - _e1176);
                        }
                        let _e1183 = phi_1845_;
                        phi_1851_ = _e1183;
                    } else {
                        if _e1169 {
                            phi_1850_ = (1f - _e1176);
                        } else {
                            phi_1850_ = _e1176;
                        }
                        let _e1180 = phi_1850_;
                        phi_1851_ = _e1180;
                    }
                    let _e1185 = phi_1851_;
                    phi_1853_ = _e1185;
                    break;
                }
                case 0: {
                    if (_e1102.x > 1f) {
                        phi_6387_ = 0.9999999f;
                    } else {
                        phi_6387_ = select(_e1102.x, 0.00000011920929f, (_e1102.x < 0f));
                    }
                    let _e1159 = phi_6387_;
                    phi_1853_ = _e1159;
                    break;
                }
                default: {
                    phi_1853_ = f32();
                    break;
                }
            }
            let _e1201 = phi_1853_;
            switch bitcast<i32>(_e1150.member.member_1) {
                case 1: {
                    let _e1239 = abs(_e1102.y);
                    let _e1241 = (_e1239 % 1f);
                    if (_e1239 >= 1f) {
                        phi_6465_ = select(true, false, (_e1241 == 0f));
                    } else {
                        phi_6465_ = true;
                    }
                    let _e1245 = phi_6465_;
                    let _e1246 = select(1f, _e1241, _e1245);
                    if (select(-1f, 1f, (_e1102.y >= 0f)) > 0f) {
                        phi_1874_ = _e1246;
                    } else {
                        phi_1874_ = (1f - _e1246);
                    }
                    let _e1250 = phi_1874_;
                    phi_1911_ = _e1250;
                    break;
                }
                case 2: {
                    let _e1213 = abs(_e1102.y);
                    let _e1220 = ((select(select(u32(_e1213), 0u, (_e1213 < 0f)), 4294967295u, (_e1213 > 4294967000f)) % 2u) == 0u);
                    let _e1222 = (_e1213 % 1f);
                    if (_e1213 >= 1f) {
                        phi_6448_ = select(true, false, (_e1222 == 0f));
                    } else {
                        phi_6448_ = true;
                    }
                    let _e1226 = phi_6448_;
                    let _e1227 = select(1f, _e1222, _e1226);
                    if (select(-1f, 1f, (_e1102.y >= 0f)) > 0f) {
                        if _e1220 {
                            phi_1903_ = _e1227;
                        } else {
                            phi_1903_ = (1f - _e1227);
                        }
                        let _e1234 = phi_1903_;
                        phi_1909_ = _e1234;
                    } else {
                        if _e1220 {
                            phi_1908_ = (1f - _e1227);
                        } else {
                            phi_1908_ = _e1227;
                        }
                        let _e1231 = phi_1908_;
                        phi_1909_ = _e1231;
                    }
                    let _e1236 = phi_1909_;
                    phi_1911_ = _e1236;
                    break;
                }
                case 0: {
                    if (_e1102.y > 1f) {
                        phi_6435_ = 0.9999999f;
                    } else {
                        phi_6435_ = select(_e1102.y, 0.00000011920929f, (_e1102.y < 0f));
                    }
                    let _e1210 = phi_6435_;
                    phi_1911_ = _e1210;
                    break;
                }
                default: {
                    phi_1911_ = f32();
                    break;
                }
            }
            let _e1252 = phi_1911_;
            let _e1256 = (_e1201 * f32(_e1150.member_2.x));
            let _e1265 = (_e1252 * f32(_e1150.member_2.y));
            let _e1283 = vec3<f32>((f32((select(select(u32(_e1256), 0u, (_e1256 < 0f)), 4294967295u, (_e1256 > 4294967000f)) + _e1150.member_1.x)) / _e499), (f32((select(select(u32(_e1265), 0u, (_e1265 < 0f)), 4294967295u, (_e1265 > 4294967000f)) + _e1150.member_1.y)) / _e500), f32(_e1150.member_3));
            let _e1289 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1283.x, _e1283.y), i32(_e1283.z), 0f);
            let _e1292 = select(_e1289, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e319.member_9 == 4294967295u)));
            if _e902 {
                phi_2005_ = vec3<f32>(0f, 0f, 0f);
                phi_2006_ = _e153;
            } else {
                let _e1296 = fma(_e904.x, 2f, -1f);
                let _e1297 = fma(_e904.y, 2f, -1f);
                let _e1298 = fma(_e904.z, 2f, -1f);
                let _e1303 = sqrt(fma(_e1298, _e1298, fma(_e1296, _e1296, (_e1297 * _e1297))));
                if (_e1303 == 0f) {
                    phi_6523_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6523_ = (vec3<f32>(_e1296, _e1297, _e1298) * (1f / _e1303));
                }
                let _e1308 = phi_6523_;
                let _e1315 = sqrt(fma(_e154.z, _e154.z, fma(_e154.x, _e154.x, (_e154.y * _e154.y))));
                if (_e1315 == 0f) {
                    phi_6558_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6558_ = (_e154 * (1f / _e1315));
                }
                let _e1320 = phi_6558_;
                let _e1327 = sqrt(fma(_e155.z, _e155.z, fma(_e155.x, _e155.x, (_e155.y * _e155.y))));
                if (_e1327 == 0f) {
                    phi_6593_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6593_ = (_e155 * (1f / _e1327));
                }
                let _e1332 = phi_6593_;
                let _e1339 = sqrt(fma(_e153.z, _e153.z, fma(_e153.x, _e153.x, (_e153.y * _e153.y))));
                if (_e1339 == 0f) {
                    phi_6628_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6628_ = (_e153 * (1f / _e1339));
                }
                let _e1344 = phi_6628_;
                let _e1363 = fma(_e1344.x, _e1308.z, fma(_e1320.x, _e1308.x, (_e1332.x * _e1308.y)));
                let _e1364 = fma(_e1344.y, _e1308.z, fma(_e1320.y, _e1308.x, (_e1332.y * _e1308.y)));
                let _e1365 = fma(_e1344.z, _e1308.z, fma(_e1320.z, _e1308.x, (_e1332.z * _e1308.y)));
                let _e1370 = sqrt(fma(_e1365, _e1365, fma(_e1363, _e1363, (_e1364 * _e1364))));
                if (_e1370 == 0f) {
                    phi_6663_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6663_ = (vec3<f32>(_e1363, _e1364, _e1365) * (1f / _e1370));
                }
                let _e1375 = phi_6663_;
                phi_2005_ = _e1308;
                phi_2006_ = _e1375;
            }
            let _e1377 = phi_2005_;
            let _e1379 = phi_2006_;
            let _e1383 = (_e516.x * _e319.member_2.x);
            let _e1386 = (_e516.y * _e319.member_2.y);
            let _e1389 = (_e516.z * _e319.member_2.z);
            let _e1394 = (_e1383 * _e150.x);
            let _e1396 = (_e1386 * _e150.y);
            let _e1398 = (_e1389 * _e150.z);
            let _e1403 = (_e710.y * _e319.member_4);
            let _e1406 = (_e710.z * _e319.member_3);
            let _e1410 = fma(_e319.member_16, (select(_e1095, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e319.member_8 == 4294967295u))).x - 1f), 1f);
            let _e1416 = (_e1292.x * _e319.member.x);
            let _e1418 = (_e1292.y * _e319.member.y);
            let _e1420 = (_e1292.z * _e319.member.z);
            let _e1425 = textureSampleLevel(global_12, global_13, _e1379, 0f);
            if (_e146 >= 86u) {
                phi_6695_ = (_e160 <= (_e146 - 86u));
            } else {
                phi_6695_ = false;
            }
            let _e1433 = phi_6695_;
            if _e1433 {
                let _e1436 = global.member[_e160];
                let _e1441 = global.member[(_e160 + 1u)];
                let _e1446 = global.member[(_e160 + 2u)];
                let _e1451 = global.member[(_e160 + 3u)];
                let _e1457 = global.member[(_e160 + 4u)];
                let _e1462 = global.member[(_e160 + 5u)];
                let _e1467 = global.member[(_e160 + 6u)];
                let _e1472 = global.member[(_e160 + 7u)];
                let _e1478 = global.member[(_e160 + 8u)];
                let _e1483 = global.member[(_e160 + 9u)];
                let _e1488 = global.member[(_e160 + 10u)];
                let _e1493 = global.member[(_e160 + 11u)];
                let _e1499 = global.member[(_e160 + 12u)];
                let _e1504 = global.member[(_e160 + 13u)];
                let _e1509 = global.member[(_e160 + 14u)];
                let _e1514 = global.member[(_e160 + 15u)];
                let _e1521 = global.member[(_e160 + 16u)];
                let _e1526 = global.member[(_e160 + 17u)];
                let _e1531 = global.member[(_e160 + 18u)];
                let _e1536 = global.member[(_e160 + 19u)];
                let _e1542 = global.member[(_e160 + 20u)];
                let _e1547 = global.member[(_e160 + 21u)];
                let _e1552 = global.member[(_e160 + 22u)];
                let _e1557 = global.member[(_e160 + 23u)];
                let _e1563 = global.member[(_e160 + 24u)];
                let _e1568 = global.member[(_e160 + 25u)];
                let _e1573 = global.member[(_e160 + 26u)];
                let _e1578 = global.member[(_e160 + 27u)];
                let _e1584 = global.member[(_e160 + 28u)];
                let _e1589 = global.member[(_e160 + 29u)];
                let _e1594 = global.member[(_e160 + 30u)];
                let _e1599 = global.member[(_e160 + 31u)];
                let _e1606 = global.member[(_e160 + 32u)];
                let _e1611 = global.member[(_e160 + 33u)];
                let _e1616 = global.member[(_e160 + 34u)];
                local_2 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_2213_ = type_14(0u, 6u);
                loop {
                    let _e1621 = phi_2213_;
                    if (_e1621.member < _e1621.member_1) {
                        phi_2214_ = type_14((_e1621.member + 1u), _e1621.member_1);
                        phi_2237_ = type_14(1u, _e1621.member);
                    } else {
                        phi_2214_ = _e1621;
                        phi_2237_ = type_14(0u, type_14().member_1);
                    }
                    let _e1634 = phi_2214_;
                    let _e1636 = phi_2237_;
                    switch bitcast<i32>(_e1636.member) {
                        case 0: {
                            phi_2264_ = false;
                            break;
                        }
                        case 1: {
                            let _e1641 = ((_e160 + 35u) + (_e1636.member_1 * 4u));
                            let _e1644 = global.member[_e1641];
                            let _e1649 = global.member[(_e1641 + 1u)];
                            let _e1654 = global.member[(_e1641 + 2u)];
                            let _e1659 = global.member[(_e1641 + 3u)];
                            local_2[_e1636.member_1] = vec4<f32>(bitcast<f32>(_e1644), bitcast<f32>(_e1649), bitcast<f32>(_e1654), bitcast<f32>(_e1659));
                            phi_2264_ = true;
                            break;
                        }
                        default: {
                            phi_2264_ = bool();
                            break;
                        }
                    }
                    let _e1664 = phi_2264_;
                    continue;
                    continuing {
                        phi_2213_ = _e1634;
                        break if !(_e1664);
                    }
                }
                let _e1666 = local_2;
                local_1 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                phi_2270_ = type_14(0u, 8u);
                loop {
                    let _e1669 = phi_2270_;
                    if (_e1669.member < _e1669.member_1) {
                        phi_2271_ = type_14((_e1669.member + 1u), _e1669.member_1);
                        phi_2294_ = type_14(1u, _e1669.member);
                    } else {
                        phi_2271_ = _e1669;
                        phi_2294_ = type_14(0u, type_14().member_1);
                    }
                    let _e1682 = phi_2271_;
                    let _e1684 = phi_2294_;
                    switch bitcast<i32>(_e1684.member) {
                        case 0: {
                            phi_2317_ = false;
                            break;
                        }
                        case 1: {
                            let _e1689 = ((_e160 + 59u) + (_e1684.member_1 * 3u));
                            let _e1692 = global.member[_e1689];
                            let _e1697 = global.member[(_e1689 + 1u)];
                            let _e1702 = global.member[(_e1689 + 2u)];
                            local_1[_e1684.member_1] = vec3<f32>(bitcast<f32>(_e1692), bitcast<f32>(_e1697), bitcast<f32>(_e1702));
                            phi_2317_ = true;
                            break;
                        }
                        default: {
                            phi_2317_ = bool();
                            break;
                        }
                    }
                    let _e1707 = phi_2317_;
                    continue;
                    continuing {
                        phi_2270_ = _e1682;
                        break if !(_e1707);
                    }
                }
                let _e1709 = local_1;
                let _e1713 = global.member[(_e160 + 83u)];
                let _e1718 = global.member[(_e160 + 84u)];
                let _e1723 = global.member[(_e160 + 85u)];
                phi_2338_ = type_25(type_23(vec4<f32>(bitcast<f32>(_e1436), bitcast<f32>(_e1441), bitcast<f32>(_e1446), bitcast<f32>(_e1451)), vec4<f32>(bitcast<f32>(_e1457), bitcast<f32>(_e1462), bitcast<f32>(_e1467), bitcast<f32>(_e1472)), vec4<f32>(bitcast<f32>(_e1478), bitcast<f32>(_e1483), bitcast<f32>(_e1488), bitcast<f32>(_e1493)), vec4<f32>(bitcast<f32>(_e1499), bitcast<f32>(_e1504), bitcast<f32>(_e1509), bitcast<f32>(_e1514))), type_23(vec4<f32>(bitcast<f32>(_e1521), bitcast<f32>(_e1526), bitcast<f32>(_e1531), bitcast<f32>(_e1536)), vec4<f32>(bitcast<f32>(_e1542), bitcast<f32>(_e1547), bitcast<f32>(_e1552), bitcast<f32>(_e1557)), vec4<f32>(bitcast<f32>(_e1563), bitcast<f32>(_e1568), bitcast<f32>(_e1573), bitcast<f32>(_e1578)), vec4<f32>(bitcast<f32>(_e1584), bitcast<f32>(_e1589), bitcast<f32>(_e1594), bitcast<f32>(_e1599))), vec3<f32>(bitcast<f32>(_e1606), bitcast<f32>(_e1611), bitcast<f32>(_e1616)), type_24(_e1709, _e1666, vec3<f32>(bitcast<f32>(_e1713), bitcast<f32>(_e1718), bitcast<f32>(_e1723))));
            } else {
                phi_2338_ = type_25(type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_24(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
            }
            let _e1729 = phi_2338_;
            let _e1731 = (_e1729.member_2 - _e156);
            let _e1738 = sqrt(fma(_e1731.z, _e1731.z, fma(_e1731.x, _e1731.x, (_e1731.y * _e1731.y))));
            let _e1739 = (_e1738 == 0f);
            if _e1739 {
                phi_6767_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6767_ = (_e1731 * (1f / _e1738));
            }
            let _e1743 = phi_6767_;
            let _e1744 = -(_e1743);
            let _e1751 = sqrt(fma(_e1379.z, _e1379.z, fma(_e1379.x, _e1379.x, (_e1379.y * _e1379.y))));
            let _e1752 = (_e1751 == 0f);
            if _e1752 {
                phi_6826_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6826_ = (_e1379 * (1f / _e1751));
            }
            let _e1756 = phi_6826_;
            let _e1766 = (2f * fma(_e1756.z, _e1744.z, fma(_e1756.x, _e1744.x, (_e1756.y * _e1744.y))));
            let _e1773 = textureSampleLevel(global_14, global_15, (_e1744 - vec3<f32>((_e1766 * _e1756.x), (_e1766 * _e1756.y), (_e1766 * _e1756.z))), (_e1403 * 4f));
            if _e1739 {
                phi_6900_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6900_ = (_e1731 * (1f / _e1738));
            }
            let _e1780 = phi_6900_;
            let _e1789 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1379.z, _e1780.z, fma(_e1379.x, _e1780.x, (_e1379.y * _e1780.y))), 0f), _e1403), 0f);
            switch bitcast<i32>(_e182) {
                case 0: {
                    if _e319.member_15 {
                        if _e1752 {
                            phi_7293_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_7293_ = (_e1379 * (1f / _e1751));
                        }
                        let _e1958 = phi_7293_;
                        if _e1739 {
                            phi_7328_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_7328_ = (_e1731 * (1f / _e1738));
                        }
                        let _e1962 = phi_7328_;
                        let _e1965 = global_1.member[0u];
                        let _e1968 = global_1.member[1u];
                        let _e1971 = global_1.member[2u];
                        phi_8867_ = false;
                        phi_2405_ = type_14(0u, _e1968);
                        phi_2408_ = vec3<f32>(0f, 0f, 0f);
                        loop {
                            let _e1974 = phi_8867_;
                            let _e1976 = phi_2405_;
                            let _e1978 = phi_2408_;
                            local_8 = _e1978;
                            local_9 = _e1978;
                            local_10 = _e1978;
                            if (_e1976.member < _e1976.member_1) {
                                phi_2406_ = type_14((_e1976.member + 1u), _e1976.member_1);
                                phi_2431_ = type_14(1u, _e1976.member);
                            } else {
                                phi_2406_ = _e1976;
                                phi_2431_ = type_14(0u, type_14().member_1);
                            }
                            let _e1991 = phi_2406_;
                            let _e1993 = phi_2431_;
                            switch bitcast<i32>(_e1993.member) {
                                case 0: {
                                    phi_9042_ = _e1974;
                                    phi_2409_ = vec3<f32>();
                                    phi_4988_ = false;
                                    break;
                                }
                                case 1: {
                                    if (_e1993.member_1 >= _e1968) {
                                        phi_7354_ = 4294967295u;
                                    } else {
                                        phi_7354_ = (_e1965 + _e1993.member_1);
                                    }
                                    let _e2000 = phi_7354_;
                                    if (_e148 >= 1u) {
                                        phi_7373_ = (_e2000 <= (_e148 - 1u));
                                    } else {
                                        phi_7373_ = false;
                                    }
                                    let _e2005 = phi_7373_;
                                    if _e2005 {
                                        let _e2008 = global_1.member[_e2000];
                                        phi_2448_ = _e2008;
                                    } else {
                                        phi_2448_ = 4294967295u;
                                    }
                                    let _e2010 = phi_2448_;
                                    if (_e148 >= 4u) {
                                        phi_7397_ = (_e2010 <= (_e148 - 4u));
                                    } else {
                                        phi_7397_ = false;
                                    }
                                    let _e2015 = phi_7397_;
                                    if _e2015 {
                                        let _e2018 = global_1.member[_e2010];
                                        switch bitcast<i32>(_e2018) {
                                            case 0: {
                                                phi_2460_ = 0u;
                                                break;
                                            }
                                            case 1: {
                                                phi_2460_ = 1u;
                                                break;
                                            }
                                            case 2: {
                                                phi_2460_ = 2u;
                                                break;
                                            }
                                            default: {
                                                phi_2460_ = 0u;
                                                break;
                                            }
                                        }
                                        let _e2021 = phi_2460_;
                                        let _e2025 = global_1.member[(_e2010 + 1u)];
                                        let _e2029 = global_1.member[(_e2010 + 2u)];
                                        let _e2033 = global_1.member[(_e2010 + 3u)];
                                        phi_2474_ = type_30(_e2021, _e2025, _e2029, _e2033);
                                    } else {
                                        phi_2474_ = type_30(0u, 4294967295u, 4294967295u, 4294967295u);
                                    }
                                    let _e2036 = phi_2474_;
                                    if (_e148 >= 10u) {
                                        phi_7429_ = (_e2036.member_2 <= (_e148 - 10u));
                                    } else {
                                        phi_7429_ = false;
                                    }
                                    let _e2042 = phi_7429_;
                                    if _e2042 {
                                        let _e2045 = global_1.member[_e2036.member_2];
                                        let _e2050 = global_1.member[(_e2036.member_2 + 1u)];
                                        let _e2055 = global_1.member[(_e2036.member_2 + 2u)];
                                        let _e2061 = global_1.member[(_e2036.member_2 + 3u)];
                                        let _e2066 = global_1.member[(_e2036.member_2 + 4u)];
                                        let _e2071 = global_1.member[(_e2036.member_2 + 5u)];
                                        let _e2076 = global_1.member[(_e2036.member_2 + 6u)];
                                        let _e2082 = global_1.member[(_e2036.member_2 + 7u)];
                                        let _e2087 = global_1.member[(_e2036.member_2 + 8u)];
                                        let _e2092 = global_1.member[(_e2036.member_2 + 9u)];
                                        phi_2524_ = type_31(vec3<f32>(bitcast<f32>(_e2045), bitcast<f32>(_e2050), bitcast<f32>(_e2055)), vec4<f32>(bitcast<f32>(_e2061), bitcast<f32>(_e2066), bitcast<f32>(_e2071), bitcast<f32>(_e2076)), vec3<f32>(bitcast<f32>(_e2082), bitcast<f32>(_e2087), bitcast<f32>(_e2092)));
                                    } else {
                                        phi_2524_ = type_31(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e2097 = phi_2524_;
                                    let _e2105 = (_e2097.member_1.x + _e2097.member_1.x);
                                    let _e2106 = (_e2097.member_1.y + _e2097.member_1.y);
                                    let _e2107 = (_e2097.member_1.z + _e2097.member_1.z);
                                    let _e2109 = (_e2097.member_1.z * _e2107);
                                    let _e2110 = (_e2097.member_1.w * _e2105);
                                    let _e2111 = (_e2097.member_1.w * _e2106);
                                    let _e2112 = (_e2097.member_1.w * _e2107);
                                    let _e2132 = (vec4<f32>((1f - fma(_e2097.member_1.y, _e2106, _e2109)), fma(_e2097.member_1.x, _e2106, _e2112), fma(_e2097.member_1.x, _e2107, -(_e2111)), 0f) * _e2097.member_2.x);
                                    let _e2134 = (vec4<f32>(fma(_e2097.member_1.x, _e2106, -(_e2112)), (1f - fma(_e2097.member_1.x, _e2105, _e2109)), fma(_e2097.member_1.y, _e2107, _e2110), 0f) * _e2097.member_2.y);
                                    let _e2136 = (vec4<f32>(fma(_e2097.member_1.x, _e2107, _e2111), fma(_e2097.member_1.y, _e2107, -(_e2110)), (1f - fma(_e2097.member_1.x, _e2105, (_e2097.member_1.y * _e2106))), 0f) * _e2097.member_2.z);
                                    switch bitcast<i32>(_e2036.member) {
                                        case 0: {
                                            if (_e148 >= 8u) {
                                                phi_8328_ = (_e2036.member_1 <= (_e148 - 8u));
                                            } else {
                                                phi_8328_ = false;
                                            }
                                            let _e3568 = phi_8328_;
                                            if _e3568 {
                                                let _e3571 = global_1.member[_e2036.member_1];
                                                let _e3576 = global_1.member[(_e2036.member_1 + 1u)];
                                                let _e3581 = global_1.member[(_e2036.member_1 + 2u)];
                                                let _e3587 = global_1.member[(_e2036.member_1 + 3u)];
                                                let _e3592 = global_1.member[(_e2036.member_1 + 4u)];
                                                let _e3597 = global_1.member[(_e2036.member_1 + 5u)];
                                                let _e3602 = global_1.member[(_e2036.member_1 + 6u)];
                                                let _e3608 = global_1.member[(_e2036.member_1 + 7u)];
                                                phi_2572_ = type_36(vec3<f32>(bitcast<f32>(_e3571), bitcast<f32>(_e3576), bitcast<f32>(_e3581)), vec4<f32>(bitcast<f32>(_e3587), bitcast<f32>(_e3592), bitcast<f32>(_e3597), bitcast<f32>(_e3602)), bitcast<f32>(_e3608));
                                            } else {
                                                phi_2572_ = type_36(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e3612 = phi_2572_;
                                            let _e3634 = fma(_e2136.x, _e3612.member.z, fma(_e2134.x, _e3612.member.y, (_e2132.x * _e3612.member.x)));
                                            let _e3635 = fma(_e2136.y, _e3612.member.z, fma(_e2134.y, _e3612.member.y, (_e2132.y * _e3612.member.x)));
                                            let _e3636 = fma(_e2136.z, _e3612.member.z, fma(_e2134.z, _e3612.member.y, (_e2132.z * _e3612.member.x)));
                                            let _e3641 = sqrt(fma(_e3636, _e3636, fma(_e3634, _e3634, (_e3635 * _e3635))));
                                            if (_e3641 == 0f) {
                                                phi_8375_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_8375_ = (vec3<f32>(_e3634, _e3635, _e3636) * (1f / _e3641));
                                            }
                                            let _e3646 = phi_8375_;
                                            let _e3648 = -(_e3646.x);
                                            let _e3650 = -(_e3646.y);
                                            let _e3652 = -(_e3646.z);
                                            let _e3653 = -(_e3646);
                                            let _e3655 = fma(-(_e710.z), _e319.member_3, 1f);
                                            let _e3659 = fma(0.4f, _e3655, (_e1394 * _e1406));
                                            let _e3660 = fma(0.4f, _e3655, (_e1396 * _e1406));
                                            let _e3661 = fma(0.4f, _e3655, (_e1398 * _e1406));
                                            let _e3669 = (_e1962 + vec3<f32>(_e3648, _e3650, _e3652));
                                            let _e3676 = sqrt(fma(_e3669.z, _e3669.z, fma(_e3669.x, _e3669.x, (_e3669.y * _e3669.y))));
                                            if (_e3676 == 0f) {
                                                phi_8410_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_8410_ = (_e3669 * (1f / _e3676));
                                            }
                                            let _e3681 = phi_8410_;
                                            let _e3682 = (_e1403 * _e1403);
                                            let _e3693 = max(fma(_e1958.z, _e3681.z, fma(_e1958.x, _e3681.x, (_e1958.y * _e3681.y))), 0f);
                                            let _e3706 = max(fma(_e1958.z, _e1962.z, fma(_e1958.x, _e1962.x, (_e1958.y * _e1962.y))), 0f);
                                            let _e3712 = fma(_e1958.z, _e3653.z, fma(_e1958.x, _e3653.x, (_e1958.y * _e3653.y)));
                                            let _e3713 = max(_e3712, 0f);
                                            let _e3714 = fma(_e710.y, _e319.member_4, 1f);
                                            let _e3715 = (_e3714 * _e3714);
                                            let _e3716 = (_e3715 * 0.125f);
                                            let _e3718 = fma(-(_e3715), 0.125f, 1f);
                                            let _e3731 = (1f - max(fma(_e3681.z, _e1962.z, fma(_e3681.x, _e1962.x, (_e3681.y * _e1962.y))), 0f));
                                            let _e3733 = select(_e3731, 0f, (_e3731 < 0f));
                                            let _e3736 = pow(select(_e3733, 1f, (_e3733 > 1f)), 5f);
                                            let _e3737 = fma((1f - _e3659), _e3736, _e3659);
                                            let _e3738 = fma((1f - _e3660), _e3736, _e3660);
                                            let _e3739 = fma((1f - _e3661), _e3736, _e3661);
                                            let _e3746 = (((_e3682 * _e3682) / (pow(fma((_e3693 * _e3693), fma(_e3682, _e3682, -1f), 1f), 2f) * 3.1415927f)) * ((_e3706 / fma(_e3706, _e3718, _e3716)) * (_e3713 / fma(_e3713, _e3718, _e3716))));
                                            let _e3753 = max(fma(_e1958.z, _e3652, fma(_e1958.x, _e3648, (_e1958.y * _e3650))), 0f);
                                            let _e3755 = fma((4f * _e3706), _e3753, 0.0001f);
                                            if ((_e2036.member_3 == 4294967295u) != true) {
                                                let _e3777 = global_1.member[_e2036.member_3];
                                                let _e3781 = global_1.member[(_e2036.member_3 + 1u)];
                                                let _e3785 = global_1.member[(_e2036.member_3 + 3u)];
                                                let _e3789 = global_1.member[(_e2036.member_3 + 4u)];
                                                let _e3793 = global_1.member[(_e2036.member_3 + 5u)];
                                                let _e3798 = global_1.member[(_e2036.member_3 + 6u)];
                                                let _e3803 = global_1.member[(_e2036.member_3 + 7u)];
                                                let _e3806 = global_1.member[_e1971];
                                                let _e3810 = global_1.member[(_e1971 + 1u)];
                                                let _e3812 = select(_e3777, 4294967295u, (0u >= _e3781));
                                                let _e3815 = global_1.member[_e3812];
                                                let _e3820 = global_1.member[(_e3812 + 1u)];
                                                let _e3825 = global_1.member[(_e3812 + 2u)];
                                                let _e3830 = global_1.member[(_e3812 + 3u)];
                                                let _e3835 = global_1.member[(_e3812 + 4u)];
                                                let _e3840 = global_1.member[(_e3812 + 5u)];
                                                let _e3845 = global_1.member[(_e3812 + 6u)];
                                                let _e3850 = global_1.member[(_e3812 + 7u)];
                                                let _e3855 = global_1.member[(_e3812 + 8u)];
                                                let _e3860 = global_1.member[(_e3812 + 9u)];
                                                let _e3865 = global_1.member[(_e3812 + 10u)];
                                                let _e3870 = global_1.member[(_e3812 + 11u)];
                                                let _e3875 = global_1.member[(_e3812 + 12u)];
                                                let _e3880 = global_1.member[(_e3812 + 13u)];
                                                let _e3885 = global_1.member[(_e3812 + 14u)];
                                                let _e3890 = global_1.member[(_e3812 + 15u)];
                                                let _e3910 = (bitcast<f32>(_e3890) + fma(bitcast<f32>(_e3870), _e156.z, fma(bitcast<f32>(_e3850), _e156.y, (bitcast<f32>(_e3830) * _e156.x))));
                                                let _e3911 = ((bitcast<f32>(_e3875) + fma(bitcast<f32>(_e3855), _e156.z, fma(bitcast<f32>(_e3835), _e156.y, (bitcast<f32>(_e3815) * _e156.x)))) / _e3910);
                                                let _e3912 = ((bitcast<f32>(_e3880) + fma(bitcast<f32>(_e3860), _e156.z, fma(bitcast<f32>(_e3840), _e156.y, (bitcast<f32>(_e3820) * _e156.x)))) / _e3910);
                                                let _e3913 = ((bitcast<f32>(_e3885) + fma(bitcast<f32>(_e3865), _e156.z, fma(bitcast<f32>(_e3845), _e156.y, (bitcast<f32>(_e3825) * _e156.x)))) / _e3910);
                                                if (abs(_e3911) <= 1f) {
                                                    let _e3917 = (abs(_e3912) <= 1f);
                                                    if _e3917 {
                                                        phi_8515_ = (abs(_e3913) <= 1f);
                                                    } else {
                                                        phi_8515_ = bool();
                                                    }
                                                    let _e3921 = phi_8515_;
                                                    phi_8518_ = _e3921;
                                                    phi_8519_ = select(true, false, _e3917);
                                                } else {
                                                    phi_8518_ = bool();
                                                    phi_8519_ = true;
                                                }
                                                let _e3924 = phi_8518_;
                                                let _e3926 = phi_8519_;
                                                if select(_e3924, false, _e3926) {
                                                    let _e3934 = global_1.member[select(_e3785, 4294967295u, (0u >= _e3789))];
                                                    let _e3937 = global_1.member[_e3934];
                                                    let _e3941 = global_1.member[(_e3934 + 1u)];
                                                    let _e3945 = global_1.member[(_e3934 + 2u)];
                                                    let _e3949 = global_1.member[(_e3934 + 3u)];
                                                    let _e3953 = global_1.member[(_e3934 + 4u)];
                                                    let _e3957 = global_1.member[(_e3934 + 6u)];
                                                    switch bitcast<i32>(_e3957) {
                                                        case 0: {
                                                            phi_2968_ = 0u;
                                                            break;
                                                        }
                                                        case 1: {
                                                            phi_2968_ = 1u;
                                                            break;
                                                        }
                                                        case 2: {
                                                            phi_2968_ = 2u;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_2968_ = 0u;
                                                            break;
                                                        }
                                                    }
                                                    let _e3960 = phi_2968_;
                                                    let _e3964 = global_1.member[(_e3934 + 7u)];
                                                    switch bitcast<i32>(_e3964) {
                                                        case 0: {
                                                            phi_2977_ = 0u;
                                                            break;
                                                        }
                                                        case 1: {
                                                            phi_2977_ = 1u;
                                                            break;
                                                        }
                                                        case 2: {
                                                            phi_2977_ = 2u;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_2977_ = 0u;
                                                            break;
                                                        }
                                                    }
                                                    let _e3967 = phi_2977_;
                                                    let _e3968 = bitcast<i32>(_e3803);
                                                    let _e3970 = f32(_e3945);
                                                    let _e3971 = f32(_e3949);
                                                    let _e3975 = type_37((_e3968 / -2i), (_e3968 / 2i), false);
                                                    phi_8994_ = _e1974;
                                                    phi_3005_ = _e3975;
                                                    phi_3008_ = 0f;
                                                    phi_3010_ = 0f;
                                                    loop {
                                                        let _e3977 = phi_8994_;
                                                        let _e3979 = phi_3005_;
                                                        let _e3981 = phi_3008_;
                                                        let _e3983 = phi_3010_;
                                                        local_6 = _e3981;
                                                        local_7 = _e3983;
                                                        if _e3979.member_2 {
                                                            phi_3022_ = true;
                                                        } else {
                                                            phi_3022_ = ((_e3979.member <= _e3979.member_1) != true);
                                                        }
                                                        let _e3990 = phi_3022_;
                                                        if _e3990 {
                                                            phi_3006_ = _e3979;
                                                            phi_3065_ = type_38(0u, type_38().member_1);
                                                        } else {
                                                            if (_e3979.member < _e3979.member_1) {
                                                                let _e3998 = (_e3979.member + 1i);
                                                                if select(false, true, ((false == (_e3998 > _e3979.member)) != false)) {
                                                                    phi_3050_ = type_38(0u, type_38().member_1);
                                                                } else {
                                                                    phi_3050_ = type_38(1u, _e3998);
                                                                }
                                                                let _e4008 = phi_3050_;
                                                                switch bitcast<i32>(_e4008.member) {
                                                                    case 0: {
                                                                        phi_9039_ = true;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        break;
                                                                    }
                                                                }
                                                                phi_3062_ = type_37(_e4008.member_1, _e3979.member_1, _e3979.member_2);
                                                            } else {
                                                                phi_3062_ = type_37(_e3979.member, _e3979.member_1, true);
                                                            }
                                                            let _e4017 = phi_3062_;
                                                            phi_3006_ = _e4017;
                                                            phi_3065_ = type_38(1u, _e3979.member);
                                                        }
                                                        let _e4023 = phi_3006_;
                                                        let _e4025 = phi_3065_;
                                                        switch bitcast<i32>(_e4025.member) {
                                                            case 0: {
                                                                phi_9040_ = _e3977;
                                                                phi_3009_ = f32();
                                                                phi_3011_ = f32();
                                                                phi_3323_ = false;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3076_ = _e3975;
                                                                phi_3079_ = _e3981;
                                                                phi_3081_ = _e3983;
                                                                loop {
                                                                    let _e4030 = phi_3076_;
                                                                    let _e4032 = phi_3079_;
                                                                    let _e4034 = phi_3081_;
                                                                    local_13 = _e4032;
                                                                    local_14 = _e4034;
                                                                    if _e4030.member_2 {
                                                                        phi_3093_ = true;
                                                                    } else {
                                                                        phi_3093_ = ((_e4030.member <= _e4030.member_1) != true);
                                                                    }
                                                                    let _e4041 = phi_3093_;
                                                                    if _e4041 {
                                                                        phi_3077_ = _e4030;
                                                                        phi_3136_ = type_38(0u, type_38().member_1);
                                                                    } else {
                                                                        if (_e4030.member < _e4030.member_1) {
                                                                            let _e4049 = (_e4030.member + 1i);
                                                                            if select(false, true, ((false == (_e4049 > _e4030.member)) != false)) {
                                                                                phi_3121_ = type_38(0u, type_38().member_1);
                                                                            } else {
                                                                                phi_3121_ = type_38(1u, _e4049);
                                                                            }
                                                                            let _e4059 = phi_3121_;
                                                                            switch bitcast<i32>(_e4059.member) {
                                                                                case 0: {
                                                                                    phi_8978_ = true;
                                                                                    break;
                                                                                }
                                                                                case 1: {
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            phi_3133_ = type_37(_e4059.member_1, _e4030.member_1, _e4030.member_2);
                                                                        } else {
                                                                            phi_3133_ = type_37(_e4030.member, _e4030.member_1, true);
                                                                        }
                                                                        let _e4068 = phi_3133_;
                                                                        phi_3077_ = _e4068;
                                                                        phi_3136_ = type_38(1u, _e4030.member);
                                                                    }
                                                                    let _e4074 = phi_3077_;
                                                                    let _e4076 = phi_3136_;
                                                                    switch bitcast<i32>(_e4076.member) {
                                                                        case 0: {
                                                                            phi_3080_ = f32();
                                                                            phi_3082_ = f32();
                                                                            phi_3322_ = false;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            let _e4084 = fma((_e3911 + 1f), 0.5f, (f32(_e4025.member_1) * (1f / _e3970)));
                                                                            let _e4085 = fma(fma(_e3912, -1f, 1f), 0.5f, (f32(_e4076.member_1) * (1f / _e3971)));
                                                                            switch bitcast<i32>(_e3960) {
                                                                                case 1: {
                                                                                    let _e4120 = abs(_e4084);
                                                                                    let _e4122 = (_e4120 % 1f);
                                                                                    if (_e4120 >= 1f) {
                                                                                        phi_8571_ = select(true, false, (_e4122 == 0f));
                                                                                    } else {
                                                                                        phi_8571_ = true;
                                                                                    }
                                                                                    let _e4126 = phi_8571_;
                                                                                    let _e4127 = select(1f, _e4122, _e4126);
                                                                                    if (select(-1f, 1f, (_e4084 >= 0f)) > 0f) {
                                                                                        phi_3168_ = _e4127;
                                                                                    } else {
                                                                                        phi_3168_ = (1f - _e4127);
                                                                                    }
                                                                                    let _e4131 = phi_3168_;
                                                                                    phi_3205_ = _e4131;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e4094 = abs(_e4084);
                                                                                    let _e4101 = ((select(select(u32(_e4094), 0u, (_e4094 < 0f)), 4294967295u, (_e4094 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e4103 = (_e4094 % 1f);
                                                                                    if (_e4094 >= 1f) {
                                                                                        phi_8554_ = select(true, false, (_e4103 == 0f));
                                                                                    } else {
                                                                                        phi_8554_ = true;
                                                                                    }
                                                                                    let _e4107 = phi_8554_;
                                                                                    let _e4108 = select(1f, _e4103, _e4107);
                                                                                    if (select(-1f, 1f, (_e4084 >= 0f)) > 0f) {
                                                                                        if _e4101 {
                                                                                            phi_3197_ = _e4108;
                                                                                        } else {
                                                                                            phi_3197_ = (1f - _e4108);
                                                                                        }
                                                                                        let _e4115 = phi_3197_;
                                                                                        phi_3203_ = _e4115;
                                                                                    } else {
                                                                                        if _e4101 {
                                                                                            phi_3202_ = (1f - _e4108);
                                                                                        } else {
                                                                                            phi_3202_ = _e4108;
                                                                                        }
                                                                                        let _e4112 = phi_3202_;
                                                                                        phi_3203_ = _e4112;
                                                                                    }
                                                                                    let _e4117 = phi_3203_;
                                                                                    phi_3205_ = _e4117;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e4084 > 1f) {
                                                                                        phi_8541_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_8541_ = select(_e4084, 0.00000011920929f, (_e4084 < 0f));
                                                                                    }
                                                                                    let _e4091 = phi_8541_;
                                                                                    phi_3205_ = _e4091;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3205_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e4133 = phi_3205_;
                                                                            switch bitcast<i32>(_e3967) {
                                                                                case 1: {
                                                                                    let _e4168 = abs(_e4085);
                                                                                    let _e4170 = (_e4168 % 1f);
                                                                                    if (_e4168 >= 1f) {
                                                                                        phi_8619_ = select(true, false, (_e4170 == 0f));
                                                                                    } else {
                                                                                        phi_8619_ = true;
                                                                                    }
                                                                                    let _e4174 = phi_8619_;
                                                                                    let _e4175 = select(1f, _e4170, _e4174);
                                                                                    if (select(-1f, 1f, (_e4085 >= 0f)) > 0f) {
                                                                                        phi_3224_ = _e4175;
                                                                                    } else {
                                                                                        phi_3224_ = (1f - _e4175);
                                                                                    }
                                                                                    let _e4179 = phi_3224_;
                                                                                    phi_3261_ = _e4179;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e4142 = abs(_e4085);
                                                                                    let _e4149 = ((select(select(u32(_e4142), 0u, (_e4142 < 0f)), 4294967295u, (_e4142 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e4151 = (_e4142 % 1f);
                                                                                    if (_e4142 >= 1f) {
                                                                                        phi_8602_ = select(true, false, (_e4151 == 0f));
                                                                                    } else {
                                                                                        phi_8602_ = true;
                                                                                    }
                                                                                    let _e4155 = phi_8602_;
                                                                                    let _e4156 = select(1f, _e4151, _e4155);
                                                                                    if (select(-1f, 1f, (_e4085 >= 0f)) > 0f) {
                                                                                        if _e4149 {
                                                                                            phi_3253_ = _e4156;
                                                                                        } else {
                                                                                            phi_3253_ = (1f - _e4156);
                                                                                        }
                                                                                        let _e4163 = phi_3253_;
                                                                                        phi_3259_ = _e4163;
                                                                                    } else {
                                                                                        if _e4149 {
                                                                                            phi_3258_ = (1f - _e4156);
                                                                                        } else {
                                                                                            phi_3258_ = _e4156;
                                                                                        }
                                                                                        let _e4160 = phi_3258_;
                                                                                        phi_3259_ = _e4160;
                                                                                    }
                                                                                    let _e4165 = phi_3259_;
                                                                                    phi_3261_ = _e4165;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e4085 > 1f) {
                                                                                        phi_8589_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_8589_ = select(_e4085, 0.00000011920929f, (_e4085 < 0f));
                                                                                    }
                                                                                    let _e4139 = phi_8589_;
                                                                                    phi_3261_ = _e4139;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3261_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e4181 = phi_3261_;
                                                                            let _e4182 = (_e4133 * _e3970);
                                                                            let _e4188 = (_e4181 * _e3971);
                                                                            let _e4203 = vec3<f32>((f32((select(select(u32(_e4182), 0u, (_e4182 < 0f)), 4294967295u, (_e4182 > 4294967000f)) + _e3937)) / f32(_e3806)), (f32((select(select(u32(_e4188), 0u, (_e4188 < 0f)), 4294967295u, (_e4188 > 4294967000f)) + _e3941)) / f32(_e3810)), f32(_e3953));
                                                                            let _e4209 = textureSampleLevel(global_19, global_18, vec2<f32>(_e4203.x, _e4203.y), i32(_e4203.z), 0f);
                                                                            if ((_e3913 - max((bitcast<f32>(_e3798) * (1f - _e3712)), bitcast<f32>(_e3793))) > _e4209.x) {
                                                                                phi_3320_ = (_e4034 + 1f);
                                                                            } else {
                                                                                phi_3320_ = _e4034;
                                                                            }
                                                                            let _e4218 = phi_3320_;
                                                                            phi_3080_ = (_e4032 + 1f);
                                                                            phi_3082_ = _e4218;
                                                                            phi_3322_ = true;
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            phi_3080_ = f32();
                                                                            phi_3082_ = f32();
                                                                            phi_3322_ = bool();
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e4221 = phi_3080_;
                                                                    let _e4223 = phi_3082_;
                                                                    let _e4225 = phi_3322_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_3076_ = _e4074;
                                                                        phi_3079_ = _e4221;
                                                                        phi_3081_ = _e4223;
                                                                        phi_8978_ = _e3977;
                                                                        break if !(_e4225);
                                                                    }
                                                                }
                                                                let _e4228 = phi_8978_;
                                                                phi_9039_ = _e4228;
                                                                if _e4228 {
                                                                    break;
                                                                }
                                                                phi_9040_ = _e4228;
                                                                let _e4681 = local_13;
                                                                phi_3009_ = _e4681;
                                                                let _e4684 = local_14;
                                                                phi_3011_ = _e4684;
                                                                phi_3323_ = true;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_9040_ = _e3977;
                                                                phi_3009_ = f32();
                                                                phi_3011_ = f32();
                                                                phi_3323_ = bool();
                                                                break;
                                                            }
                                                        }
                                                        let _e4230 = phi_9040_;
                                                        let _e4232 = phi_3009_;
                                                        let _e4234 = phi_3011_;
                                                        let _e4236 = phi_3323_;
                                                        continue;
                                                        continuing {
                                                            phi_8994_ = _e4230;
                                                            phi_3005_ = _e4023;
                                                            phi_3008_ = _e4232;
                                                            phi_3010_ = _e4234;
                                                            phi_9039_ = _e4230;
                                                            break if !(_e4236);
                                                        }
                                                    }
                                                    let _e4239 = phi_9039_;
                                                    phi_9041_ = _e4239;
                                                    if _e4239 {
                                                        break;
                                                    }
                                                    let _e4241 = local_6;
                                                    let _e4244 = local_7;
                                                    phi_9052_ = _e4239;
                                                    phi_3326_ = (_e4244 / max(_e4241, 1f));
                                                } else {
                                                    phi_9052_ = _e1974;
                                                    phi_3326_ = 0f;
                                                }
                                                let _e4247 = phi_9052_;
                                                let _e4249 = phi_3326_;
                                                phi_9051_ = _e4247;
                                                phi_3327_ = _e4249;
                                            } else {
                                                phi_9051_ = _e1974;
                                                phi_3327_ = 0f;
                                            }
                                            let _e4251 = phi_9051_;
                                            let _e4253 = phi_3327_;
                                            phi_9045_ = _e4251;
                                            phi_4960_ = _e4253;
                                            phi_4961_ = vec3<f32>(((fma((((1f - _e3737) * _e3655) * _e1394), 0.31830987f, ((_e3746 * _e3737) / _e3755)) * (_e3612.member_1.x * _e3612.member_2)) * _e3753), ((fma((((1f - _e3738) * _e3655) * _e1396), 0.31830987f, ((_e3746 * _e3738) / _e3755)) * (_e3612.member_1.y * _e3612.member_2)) * _e3753), ((fma((((1f - _e3739) * _e3655) * _e1398), 0.31830987f, ((_e3746 * _e3739) / _e3755)) * (_e3612.member_1.z * _e3612.member_2)) * _e3753));
                                            phi_4962_ = true;
                                            break;
                                        }
                                        case 1: {
                                            if (_e148 >= 8u) {
                                                phi_7875_ = (_e2036.member_1 <= (_e148 - 8u));
                                            } else {
                                                phi_7875_ = false;
                                            }
                                            let _e2922 = phi_7875_;
                                            if _e2922 {
                                                let _e2925 = global_1.member[_e2036.member_1];
                                                let _e2930 = global_1.member[(_e2036.member_1 + 1u)];
                                                let _e2935 = global_1.member[(_e2036.member_1 + 2u)];
                                                let _e2941 = global_1.member[(_e2036.member_1 + 3u)];
                                                let _e2946 = global_1.member[(_e2036.member_1 + 4u)];
                                                let _e2951 = global_1.member[(_e2036.member_1 + 5u)];
                                                let _e2956 = global_1.member[(_e2036.member_1 + 6u)];
                                                let _e2962 = global_1.member[(_e2036.member_1 + 7u)];
                                                phi_3368_ = type_36(vec3<f32>(bitcast<f32>(_e2925), bitcast<f32>(_e2930), bitcast<f32>(_e2935)), vec4<f32>(bitcast<f32>(_e2941), bitcast<f32>(_e2946), bitcast<f32>(_e2951), bitcast<f32>(_e2956)), bitcast<f32>(_e2962));
                                            } else {
                                                phi_3368_ = type_36(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2966 = phi_3368_;
                                            let _e2991 = (_e2097.member.x + fma(_e2136.x, _e2966.member.z, fma(_e2134.x, _e2966.member.y, (_e2132.x * _e2966.member.x))));
                                            let _e2992 = (_e2097.member.y + fma(_e2136.y, _e2966.member.z, fma(_e2134.y, _e2966.member.y, (_e2132.y * _e2966.member.x))));
                                            let _e2993 = (_e2097.member.z + fma(_e2136.z, _e2966.member.z, fma(_e2134.z, _e2966.member.y, (_e2132.z * _e2966.member.x))));
                                            let _e2995 = (vec3<f32>(_e2991, _e2992, _e2993) - _e156);
                                            let _e3002 = sqrt(fma(_e2995.z, _e2995.z, fma(_e2995.x, _e2995.x, (_e2995.y * _e2995.y))));
                                            let _e3003 = (_e3002 == 0f);
                                            if _e3003 {
                                                phi_9049_ = _e1974;
                                                phi_4030_ = f32();
                                                phi_4031_ = vec3<f32>();
                                            } else {
                                                if _e3003 {
                                                    phi_7922_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7922_ = (_e2995 * (1f / _e3002));
                                                }
                                                let _e3007 = phi_7922_;
                                                let _e3009 = (_e2966.member_2 / (_e3002 * _e3002));
                                                let _e3011 = fma(-(_e710.z), _e319.member_3, 1f);
                                                let _e3015 = fma(0.4f, _e3011, (_e1394 * _e1406));
                                                let _e3016 = fma(0.4f, _e3011, (_e1396 * _e1406));
                                                let _e3017 = fma(0.4f, _e3011, (_e1398 * _e1406));
                                                let _e3024 = (_e1962 + _e3007);
                                                let _e3031 = sqrt(fma(_e3024.z, _e3024.z, fma(_e3024.x, _e3024.x, (_e3024.y * _e3024.y))));
                                                if (_e3031 == 0f) {
                                                    phi_7957_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7957_ = (_e3024 * (1f / _e3031));
                                                }
                                                let _e3036 = phi_7957_;
                                                let _e3037 = (_e1403 * _e1403);
                                                let _e3048 = max(fma(_e1958.z, _e3036.z, fma(_e1958.x, _e3036.x, (_e1958.y * _e3036.y))), 0f);
                                                let _e3061 = max(fma(_e1958.z, _e1962.z, fma(_e1958.x, _e1962.x, (_e1958.y * _e1962.y))), 0f);
                                                let _e3067 = fma(_e1958.z, _e3007.z, fma(_e1958.x, _e3007.x, (_e1958.y * _e3007.y)));
                                                let _e3068 = max(_e3067, 0f);
                                                let _e3069 = fma(_e710.y, _e319.member_4, 1f);
                                                let _e3070 = (_e3069 * _e3069);
                                                let _e3071 = (_e3070 * 0.125f);
                                                let _e3073 = fma(-(_e3070), 0.125f, 1f);
                                                let _e3086 = (1f - max(fma(_e3036.z, _e1962.z, fma(_e3036.x, _e1962.x, (_e3036.y * _e1962.y))), 0f));
                                                let _e3088 = select(_e3086, 0f, (_e3086 < 0f));
                                                let _e3091 = pow(select(_e3088, 1f, (_e3088 > 1f)), 5f);
                                                let _e3092 = fma((1f - _e3015), _e3091, _e3015);
                                                let _e3093 = fma((1f - _e3016), _e3091, _e3016);
                                                let _e3094 = fma((1f - _e3017), _e3091, _e3017);
                                                let _e3101 = (((_e3037 * _e3037) / (pow(fma((_e3048 * _e3048), fma(_e3037, _e3037, -1f), 1f), 2f) * 3.1415927f)) * ((_e3061 / fma(_e3061, _e3073, _e3071)) * (_e3068 / fma(_e3068, _e3073, _e3071))));
                                                let _e3106 = fma((4f * _e3061), _e3068, 0.0001f);
                                                if ((_e2036.member_3 == 4294967295u) != true) {
                                                    let _e3128 = global_1.member[_e2036.member_3];
                                                    let _e3132 = global_1.member[(_e2036.member_3 + 1u)];
                                                    let _e3136 = global_1.member[(_e2036.member_3 + 2u)];
                                                    let _e3141 = global_1.member[(_e2036.member_3 + 3u)];
                                                    let _e3145 = global_1.member[(_e2036.member_3 + 4u)];
                                                    let _e3149 = global_1.member[(_e2036.member_3 + 5u)];
                                                    let _e3154 = global_1.member[(_e2036.member_3 + 6u)];
                                                    let _e3159 = global_1.member[(_e2036.member_3 + 7u)];
                                                    let _e3162 = global_1.member[_e1971];
                                                    let _e3166 = global_1.member[(_e1971 + 1u)];
                                                    let _e3168 = (_e156.x - _e2991);
                                                    let _e3170 = (_e156.y - _e2992);
                                                    let _e3172 = (_e156.z - _e2993);
                                                    let _e3175 = min(max(f32(_e3159), 1f), 21f);
                                                    let _e3187 = ((1f + (sqrt(fma(_e3172, _e3172, fma(_e3168, _e3168, (_e3170 * _e3170)))) / bitcast<f32>(_e3136))) * 0.04f);
                                                    phi_3642_ = type_14(0u, select(select(u32(_e3175), 0u, (_e3175 < 0f)), 4294967295u, (_e3175 > 4294967000f)));
                                                    phi_3645_ = 0f;
                                                    loop {
                                                        let _e3190 = phi_3642_;
                                                        let _e3192 = phi_3645_;
                                                        local_5 = _e3192;
                                                        if (_e3190.member < _e3190.member_1) {
                                                            phi_3643_ = type_14((_e3190.member + 1u), _e3190.member_1);
                                                            phi_3668_ = type_14(1u, _e3190.member);
                                                        } else {
                                                            phi_3643_ = _e3190;
                                                            phi_3668_ = type_14(0u, type_14().member_1);
                                                        }
                                                        let _e3205 = phi_3643_;
                                                        let _e3207 = phi_3668_;
                                                        switch bitcast<i32>(_e3207.member) {
                                                            case 0: {
                                                                phi_3646_ = f32();
                                                                phi_4027_ = false;
                                                                break;
                                                            }
                                                            case 1: {
                                                                local = array<vec3<f32>, 21>(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3<f32>(1f, -1f, 1f), vec3<f32>(-1f, -1f, 1f), vec3<f32>(-1f, 1f, 1f), vec3<f32>(1f, 1f, -1f), vec3<f32>(1f, -1f, -1f), vec3<f32>(-1f, -1f, -1f), vec3<f32>(-1f, 1f, -1f), vec3<f32>(1f, 1f, 0f), vec3<f32>(1f, -1f, 0f), vec3<f32>(-1f, -1f, 0f), vec3<f32>(-1f, 1f, 0f), vec3<f32>(1f, 0f, 1f), vec3<f32>(-1f, 0f, 1f), vec3<f32>(1f, 0f, -1f), vec3<f32>(-1f, 0f, -1f), vec3<f32>(0f, 1f, 1f), vec3<f32>(0f, -1f, 1f), vec3<f32>(0f, -1f, -1f), vec3<f32>(0f, 1f, -1f));
                                                                if (_e3207.member_1 < 21u) {
                                                                } else {
                                                                    phi_8922_ = true;
                                                                    break;
                                                                }
                                                                let _e3213 = local[_e3207.member_1];
                                                                let _e3217 = fma(_e3213.x, _e3187, _e3168);
                                                                let _e3218 = fma(_e3213.y, _e3187, _e3170);
                                                                let _e3219 = fma(_e3213.z, _e3187, _e3172);
                                                                let _e3224 = sqrt(fma(_e3219, _e3219, fma(_e3217, _e3217, (_e3218 * _e3218))));
                                                                if (_e3224 == 0f) {
                                                                    phi_8078_ = vec3<f32>(0f, 0f, 0f);
                                                                } else {
                                                                    phi_8078_ = (vec3<f32>(_e3217, _e3218, _e3219) * (1f / _e3224));
                                                                }
                                                                let _e3229 = phi_8078_;
                                                                let _e3231 = abs(_e3229.x);
                                                                let _e3233 = abs(_e3229.y);
                                                                let _e3235 = abs(_e3229.z);
                                                                if (_e3231 >= _e3233) {
                                                                    let _e3237 = (_e3231 >= _e3235);
                                                                    if _e3237 {
                                                                        let _e3238 = (_e3229.x > 0f);
                                                                        if _e3238 {
                                                                            phi_8112_ = vec2<f32>((-(_e3229.z) / _e3231), (-(_e3229.y) / _e3231));
                                                                        } else {
                                                                            phi_8112_ = vec2<f32>((_e3229.z / _e3231), (-(_e3229.y) / _e3231));
                                                                        }
                                                                        let _e3249 = phi_8112_;
                                                                        phi_8115_ = _e3249;
                                                                        phi_8116_ = select(1u, 0u, _e3238);
                                                                    } else {
                                                                        phi_8115_ = vec2<f32>();
                                                                        phi_8116_ = u32();
                                                                    }
                                                                    let _e3252 = phi_8115_;
                                                                    let _e3254 = phi_8116_;
                                                                    phi_8119_ = _e3252;
                                                                    phi_8120_ = _e3254;
                                                                    phi_8121_ = select(true, false, _e3237);
                                                                } else {
                                                                    phi_8119_ = vec2<f32>();
                                                                    phi_8120_ = u32();
                                                                    phi_8121_ = true;
                                                                }
                                                                let _e3257 = phi_8119_;
                                                                let _e3259 = phi_8120_;
                                                                let _e3261 = phi_8121_;
                                                                if _e3261 {
                                                                    if (_e3233 >= _e3231) {
                                                                        let _e3263 = (_e3233 >= _e3235);
                                                                        if _e3263 {
                                                                            let _e3264 = (_e3229.y > 0f);
                                                                            if _e3264 {
                                                                                phi_8146_ = vec2<f32>((_e3229.x / _e3233), (_e3229.z / _e3233));
                                                                            } else {
                                                                                phi_8146_ = vec2<f32>((_e3229.x / _e3233), (-(_e3229.z) / _e3233));
                                                                            }
                                                                            let _e3273 = phi_8146_;
                                                                            phi_8149_ = _e3273;
                                                                            phi_8150_ = select(3u, 2u, _e3264);
                                                                        } else {
                                                                            phi_8149_ = vec2<f32>();
                                                                            phi_8150_ = u32();
                                                                        }
                                                                        let _e3276 = phi_8149_;
                                                                        let _e3278 = phi_8150_;
                                                                        phi_8153_ = _e3276;
                                                                        phi_8154_ = _e3278;
                                                                        phi_8155_ = select(true, false, _e3263);
                                                                    } else {
                                                                        phi_8153_ = vec2<f32>();
                                                                        phi_8154_ = u32();
                                                                        phi_8155_ = true;
                                                                    }
                                                                    let _e3281 = phi_8153_;
                                                                    let _e3283 = phi_8154_;
                                                                    let _e3285 = phi_8155_;
                                                                    if _e3285 {
                                                                        let _e3286 = (_e3229.z > 0f);
                                                                        if _e3286 {
                                                                            phi_8176_ = vec2<f32>((_e3229.x / _e3235), (-(_e3229.y) / _e3235));
                                                                        } else {
                                                                            phi_8176_ = vec2<f32>((-(_e3229.x) / _e3235), (-(_e3229.y) / _e3235));
                                                                        }
                                                                        let _e3297 = phi_8176_;
                                                                        phi_8179_ = _e3297;
                                                                        phi_8180_ = select(5u, 4u, _e3286);
                                                                    } else {
                                                                        phi_8179_ = _e3281;
                                                                        phi_8180_ = _e3283;
                                                                    }
                                                                    let _e3300 = phi_8179_;
                                                                    let _e3302 = phi_8180_;
                                                                    phi_8182_ = _e3300;
                                                                    phi_8183_ = _e3302;
                                                                } else {
                                                                    phi_8182_ = _e3257;
                                                                    phi_8183_ = _e3259;
                                                                }
                                                                let _e3304 = phi_8182_;
                                                                let _e3306 = phi_8183_;
                                                                let _e3311 = ((_e3304.x + 1f) * 0.5f);
                                                                let _e3312 = ((_e3304.y + 1f) * 0.5f);
                                                                if (_e3306 >= _e3132) {
                                                                    phi_3705_ = 4294967295u;
                                                                } else {
                                                                    phi_3705_ = (_e3128 + (16u * _e3306));
                                                                }
                                                                let _e3317 = phi_3705_;
                                                                let _e3321 = global_1.member[(_e3317 + 2u)];
                                                                let _e3326 = global_1.member[(_e3317 + 3u)];
                                                                let _e3331 = global_1.member[(_e3317 + 6u)];
                                                                let _e3336 = global_1.member[(_e3317 + 7u)];
                                                                let _e3341 = global_1.member[(_e3317 + 10u)];
                                                                let _e3346 = global_1.member[(_e3317 + 11u)];
                                                                let _e3351 = global_1.member[(_e3317 + 14u)];
                                                                let _e3356 = global_1.member[(_e3317 + 15u)];
                                                                if (_e3306 >= _e3145) {
                                                                    phi_3806_ = 4294967295u;
                                                                } else {
                                                                    phi_3806_ = (_e3141 + _e3306);
                                                                }
                                                                let _e3370 = phi_3806_;
                                                                let _e3373 = global_1.member[_e3370];
                                                                let _e3376 = global_1.member[_e3373];
                                                                let _e3380 = global_1.member[(_e3373 + 1u)];
                                                                let _e3384 = global_1.member[(_e3373 + 2u)];
                                                                let _e3388 = global_1.member[(_e3373 + 3u)];
                                                                let _e3392 = global_1.member[(_e3373 + 4u)];
                                                                let _e3396 = global_1.member[(_e3373 + 6u)];
                                                                switch bitcast<i32>(_e3396) {
                                                                    case 0: {
                                                                        phi_3837_ = 0u;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        phi_3837_ = 1u;
                                                                        break;
                                                                    }
                                                                    case 2: {
                                                                        phi_3837_ = 2u;
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        phi_3837_ = 0u;
                                                                        break;
                                                                    }
                                                                }
                                                                let _e3399 = phi_3837_;
                                                                let _e3403 = global_1.member[(_e3373 + 7u)];
                                                                switch bitcast<i32>(_e3403) {
                                                                    case 0: {
                                                                        phi_3846_ = 0u;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        phi_3846_ = 1u;
                                                                        break;
                                                                    }
                                                                    case 2: {
                                                                        phi_3846_ = 2u;
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        phi_3846_ = 0u;
                                                                        break;
                                                                    }
                                                                }
                                                                let _e3406 = phi_3846_;
                                                                switch bitcast<i32>(_e3399) {
                                                                    case 1: {
                                                                        let _e3441 = abs(_e3311);
                                                                        let _e3443 = (_e3441 % 1f);
                                                                        if (_e3441 >= 1f) {
                                                                            phi_8243_ = select(true, false, (_e3443 == 0f));
                                                                        } else {
                                                                            phi_8243_ = true;
                                                                        }
                                                                        let _e3447 = phi_8243_;
                                                                        let _e3448 = select(1f, _e3443, _e3447);
                                                                        if (select(-1f, 1f, (_e3311 >= 0f)) > 0f) {
                                                                            phi_3872_ = _e3448;
                                                                        } else {
                                                                            phi_3872_ = (1f - _e3448);
                                                                        }
                                                                        let _e3452 = phi_3872_;
                                                                        phi_3909_ = _e3452;
                                                                        break;
                                                                    }
                                                                    case 2: {
                                                                        let _e3415 = abs(_e3311);
                                                                        let _e3422 = ((select(select(u32(_e3415), 0u, (_e3415 < 0f)), 4294967295u, (_e3415 > 4294967000f)) % 2u) == 0u);
                                                                        let _e3424 = (_e3415 % 1f);
                                                                        if (_e3415 >= 1f) {
                                                                            phi_8226_ = select(true, false, (_e3424 == 0f));
                                                                        } else {
                                                                            phi_8226_ = true;
                                                                        }
                                                                        let _e3428 = phi_8226_;
                                                                        let _e3429 = select(1f, _e3424, _e3428);
                                                                        if (select(-1f, 1f, (_e3311 >= 0f)) > 0f) {
                                                                            if _e3422 {
                                                                                phi_3901_ = _e3429;
                                                                            } else {
                                                                                phi_3901_ = (1f - _e3429);
                                                                            }
                                                                            let _e3436 = phi_3901_;
                                                                            phi_3907_ = _e3436;
                                                                        } else {
                                                                            if _e3422 {
                                                                                phi_3906_ = (1f - _e3429);
                                                                            } else {
                                                                                phi_3906_ = _e3429;
                                                                            }
                                                                            let _e3433 = phi_3906_;
                                                                            phi_3907_ = _e3433;
                                                                        }
                                                                        let _e3438 = phi_3907_;
                                                                        phi_3909_ = _e3438;
                                                                        break;
                                                                    }
                                                                    case 0: {
                                                                        if (_e3311 > 1f) {
                                                                            phi_8213_ = 0.9999999f;
                                                                        } else {
                                                                            phi_8213_ = select(_e3311, 0.00000011920929f, (_e3311 < 0f));
                                                                        }
                                                                        let _e3412 = phi_8213_;
                                                                        phi_3909_ = _e3412;
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        phi_3909_ = f32();
                                                                        break;
                                                                    }
                                                                }
                                                                let _e3454 = phi_3909_;
                                                                switch bitcast<i32>(_e3406) {
                                                                    case 1: {
                                                                        let _e3489 = abs(_e3312);
                                                                        let _e3491 = (_e3489 % 1f);
                                                                        if (_e3489 >= 1f) {
                                                                            phi_8291_ = select(true, false, (_e3491 == 0f));
                                                                        } else {
                                                                            phi_8291_ = true;
                                                                        }
                                                                        let _e3495 = phi_8291_;
                                                                        let _e3496 = select(1f, _e3491, _e3495);
                                                                        if (select(-1f, 1f, (_e3312 >= 0f)) > 0f) {
                                                                            phi_3930_ = _e3496;
                                                                        } else {
                                                                            phi_3930_ = (1f - _e3496);
                                                                        }
                                                                        let _e3500 = phi_3930_;
                                                                        phi_3967_ = _e3500;
                                                                        break;
                                                                    }
                                                                    case 2: {
                                                                        let _e3463 = abs(_e3312);
                                                                        let _e3470 = ((select(select(u32(_e3463), 0u, (_e3463 < 0f)), 4294967295u, (_e3463 > 4294967000f)) % 2u) == 0u);
                                                                        let _e3472 = (_e3463 % 1f);
                                                                        if (_e3463 >= 1f) {
                                                                            phi_8274_ = select(true, false, (_e3472 == 0f));
                                                                        } else {
                                                                            phi_8274_ = true;
                                                                        }
                                                                        let _e3476 = phi_8274_;
                                                                        let _e3477 = select(1f, _e3472, _e3476);
                                                                        if (select(-1f, 1f, (_e3312 >= 0f)) > 0f) {
                                                                            if _e3470 {
                                                                                phi_3959_ = _e3477;
                                                                            } else {
                                                                                phi_3959_ = (1f - _e3477);
                                                                            }
                                                                            let _e3484 = phi_3959_;
                                                                            phi_3965_ = _e3484;
                                                                        } else {
                                                                            if _e3470 {
                                                                                phi_3964_ = (1f - _e3477);
                                                                            } else {
                                                                                phi_3964_ = _e3477;
                                                                            }
                                                                            let _e3481 = phi_3964_;
                                                                            phi_3965_ = _e3481;
                                                                        }
                                                                        let _e3486 = phi_3965_;
                                                                        phi_3967_ = _e3486;
                                                                        break;
                                                                    }
                                                                    case 0: {
                                                                        if (_e3312 > 1f) {
                                                                            phi_8261_ = 0.9999999f;
                                                                        } else {
                                                                            phi_8261_ = select(_e3312, 0.00000011920929f, (_e3312 < 0f));
                                                                        }
                                                                        let _e3460 = phi_8261_;
                                                                        phi_3967_ = _e3460;
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        phi_3967_ = f32();
                                                                        break;
                                                                    }
                                                                }
                                                                let _e3502 = phi_3967_;
                                                                let _e3504 = (_e3454 * f32(_e3384));
                                                                let _e3511 = (_e3502 * f32(_e3388));
                                                                let _e3526 = vec3<f32>((f32((select(select(u32(_e3504), 0u, (_e3504 < 0f)), 4294967295u, (_e3504 > 4294967000f)) + _e3376)) / f32(_e3162)), (f32((select(select(u32(_e3511), 0u, (_e3511 < 0f)), 4294967295u, (_e3511 > 4294967000f)) + _e3380)) / f32(_e3166)), f32(_e3392));
                                                                let _e3532 = textureSampleLevel(global_19, global_18, vec2<f32>(_e3526.x, _e3526.y), i32(_e3526.z), 0f);
                                                                if ((((bitcast<f32>(_e3351) + fma(bitcast<f32>(_e3341), _e156.z, fma(bitcast<f32>(_e3331), _e156.y, (bitcast<f32>(_e3321) * _e156.x)))) / (bitcast<f32>(_e3356) + fma(bitcast<f32>(_e3346), _e156.z, fma(bitcast<f32>(_e3336), _e156.y, (bitcast<f32>(_e3326) * _e156.x))))) - max((bitcast<f32>(_e3154) * (1f - _e3067)), bitcast<f32>(_e3149))) > _e3532.x) {
                                                                    phi_4026_ = (_e3192 + 1f);
                                                                } else {
                                                                    phi_4026_ = _e3192;
                                                                }
                                                                let _e3541 = phi_4026_;
                                                                phi_3646_ = _e3541;
                                                                phi_4027_ = true;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_3646_ = f32();
                                                                phi_4027_ = bool();
                                                                break;
                                                            }
                                                        }
                                                        let _e3543 = phi_3646_;
                                                        let _e3545 = phi_4027_;
                                                        continue;
                                                        continuing {
                                                            phi_3642_ = _e3205;
                                                            phi_3645_ = _e3543;
                                                            phi_8922_ = _e1974;
                                                            break if !(_e3545);
                                                        }
                                                    }
                                                    let _e3548 = phi_8922_;
                                                    phi_9041_ = _e3548;
                                                    if _e3548 {
                                                        break;
                                                    }
                                                    let _e3550 = local_5;
                                                    phi_9050_ = _e3548;
                                                    phi_4029_ = (_e3550 / _e3175);
                                                } else {
                                                    phi_9050_ = _e1974;
                                                    phi_4029_ = 0f;
                                                }
                                                let _e3553 = phi_9050_;
                                                let _e3555 = phi_4029_;
                                                phi_9049_ = _e3553;
                                                phi_4030_ = _e3555;
                                                phi_4031_ = vec3<f32>(((fma((((1f - _e3092) * _e3011) * _e1394), 0.31830987f, ((_e3101 * _e3092) / _e3106)) * (_e2966.member_1.x * _e3009)) * _e3068), ((fma((((1f - _e3093) * _e3011) * _e1396), 0.31830987f, ((_e3101 * _e3093) / _e3106)) * (_e2966.member_1.y * _e3009)) * _e3068), ((fma((((1f - _e3094) * _e3011) * _e1398), 0.31830987f, ((_e3101 * _e3094) / _e3106)) * (_e2966.member_1.z * _e3009)) * _e3068));
                                            }
                                            let _e3557 = phi_9049_;
                                            let _e3559 = phi_4030_;
                                            let _e3561 = phi_4031_;
                                            phi_9045_ = _e3557;
                                            phi_4960_ = _e3559;
                                            phi_4961_ = _e3561;
                                            phi_4962_ = select(true, false, _e3003);
                                            break;
                                        }
                                        case 2: {
                                            if (_e148 >= 13u) {
                                                phi_7509_ = (_e2036.member_1 <= (_e148 - 13u));
                                            } else {
                                                phi_7509_ = false;
                                            }
                                            let _e2147 = phi_7509_;
                                            if _e2147 {
                                                let _e2150 = global_1.member[_e2036.member_1];
                                                let _e2155 = global_1.member[(_e2036.member_1 + 1u)];
                                                let _e2160 = global_1.member[(_e2036.member_1 + 2u)];
                                                let _e2166 = global_1.member[(_e2036.member_1 + 3u)];
                                                let _e2171 = global_1.member[(_e2036.member_1 + 4u)];
                                                let _e2176 = global_1.member[(_e2036.member_1 + 5u)];
                                                let _e2182 = global_1.member[(_e2036.member_1 + 6u)];
                                                let _e2187 = global_1.member[(_e2036.member_1 + 7u)];
                                                let _e2192 = global_1.member[(_e2036.member_1 + 8u)];
                                                let _e2197 = global_1.member[(_e2036.member_1 + 9u)];
                                                let _e2202 = global_1.member[(_e2036.member_1 + 10u)];
                                                let _e2207 = global_1.member[(_e2036.member_1 + 11u)];
                                                let _e2213 = global_1.member[(_e2036.member_1 + 12u)];
                                                phi_4094_ = type_39(vec3<f32>(bitcast<f32>(_e2150), bitcast<f32>(_e2155), bitcast<f32>(_e2160)), vec3<f32>(bitcast<f32>(_e2166), bitcast<f32>(_e2171), bitcast<f32>(_e2176)), bitcast<f32>(_e2182), bitcast<f32>(_e2187), vec4<f32>(bitcast<f32>(_e2192), bitcast<f32>(_e2197), bitcast<f32>(_e2202), bitcast<f32>(_e2207)), bitcast<f32>(_e2213));
                                            } else {
                                                phi_4094_ = type_39(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2217 = phi_4094_;
                                            let _e2243 = vec3<f32>((_e2097.member.x + fma(_e2136.x, _e2217.member.z, fma(_e2134.x, _e2217.member.y, (_e2132.x * _e2217.member.x)))), (_e2097.member.y + fma(_e2136.y, _e2217.member.z, fma(_e2134.y, _e2217.member.y, (_e2132.y * _e2217.member.x)))), (_e2097.member.z + fma(_e2136.z, _e2217.member.z, fma(_e2134.z, _e2217.member.y, (_e2132.z * _e2217.member.x)))));
                                            let _e2244 = (_e2243 - _e156);
                                            let _e2251 = sqrt(fma(_e2244.z, _e2244.z, fma(_e2244.x, _e2244.x, (_e2244.y * _e2244.y))));
                                            let _e2252 = (_e2251 == 0f);
                                            if _e2252 {
                                                phi_4230_ = type_40(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), 0f, 0f, 0f, 0f, 0f, 0f, 0f, false, false);
                                            } else {
                                                if _e2252 {
                                                    phi_7559_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7559_ = (_e2244 * (1f / _e2251));
                                                }
                                                let _e2256 = phi_7559_;
                                                let _e2267 = fma(_e2136.x, _e2217.member_1.z, fma(_e2134.x, _e2217.member_1.y, (_e2132.x * _e2217.member_1.x)));
                                                let _e2268 = fma(_e2136.y, _e2217.member_1.z, fma(_e2134.y, _e2217.member_1.y, (_e2132.y * _e2217.member_1.x)));
                                                let _e2269 = fma(_e2136.z, _e2217.member_1.z, fma(_e2134.z, _e2217.member_1.y, (_e2132.z * _e2217.member_1.x)));
                                                let _e2274 = sqrt(fma(_e2269, _e2269, fma(_e2267, _e2267, (_e2268 * _e2268))));
                                                if (_e2274 == 0f) {
                                                    phi_7594_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7594_ = (vec3<f32>(_e2267, _e2268, _e2269) * (1f / _e2274));
                                                }
                                                let _e2279 = phi_7594_;
                                                let _e2281 = cos(_e2217.member_2);
                                                let _e2283 = cos(_e2217.member_3);
                                                let _e2284 = (_e2281 - _e2283);
                                                let _e2296 = fma(_e2256.z, -(_e2279.z), fma(_e2256.x, -(_e2279.x), (_e2256.y * -(_e2279.y))));
                                                let _e2300 = ((_e2296 - _e2283) / _e2284);
                                                let _e2302 = select(_e2300, 0f, (_e2300 < 0f));
                                                phi_4230_ = type_40(_e2243, _e156, _e2256, _e2279, _e2251, _e2281, _e2283, _e2284, _e2296, _e2300, select(_e2302, 1f, (_e2302 > 1f)), (_e2296 > _e2281), (_e2296 > _e2283));
                                            }
                                            let _e2307 = phi_4230_;
                                            let _e2309 = (_e2307.member_4 == 0f);
                                            if _e2309 {
                                                phi_9046_ = _e1974;
                                                phi_4957_ = f32();
                                                phi_4958_ = vec3<f32>();
                                            } else {
                                                let _e2312 = (_e2217.member_5 * _e2307.member_10);
                                                let _e2316 = fma(-(_e710.z), _e319.member_3, 1f);
                                                let _e2320 = fma(0.4f, _e2316, (_e1394 * _e1406));
                                                let _e2321 = fma(0.4f, _e2316, (_e1396 * _e1406));
                                                let _e2322 = fma(0.4f, _e2316, (_e1398 * _e1406));
                                                let _e2329 = (_e1962 + _e2307.member_2);
                                                let _e2336 = sqrt(fma(_e2329.z, _e2329.z, fma(_e2329.x, _e2329.x, (_e2329.y * _e2329.y))));
                                                if (_e2336 == 0f) {
                                                    phi_7629_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7629_ = (_e2329 * (1f / _e2336));
                                                }
                                                let _e2341 = phi_7629_;
                                                let _e2342 = (_e1403 * _e1403);
                                                let _e2353 = max(fma(_e1958.z, _e2341.z, fma(_e1958.x, _e2341.x, (_e1958.y * _e2341.y))), 0f);
                                                let _e2366 = max(fma(_e1958.z, _e1962.z, fma(_e1958.x, _e1962.x, (_e1958.y * _e1962.y))), 0f);
                                                let _e2372 = fma(_e1958.z, _e2307.member_2.z, fma(_e1958.x, _e2307.member_2.x, (_e1958.y * _e2307.member_2.y)));
                                                let _e2373 = max(_e2372, 0f);
                                                let _e2374 = fma(_e710.y, _e319.member_4, 1f);
                                                let _e2375 = (_e2374 * _e2374);
                                                let _e2376 = (_e2375 * 0.125f);
                                                let _e2378 = fma(-(_e2375), 0.125f, 1f);
                                                let _e2391 = (1f - max(fma(_e2341.z, _e1962.z, fma(_e2341.x, _e1962.x, (_e2341.y * _e1962.y))), 0f));
                                                let _e2393 = select(_e2391, 0f, (_e2391 < 0f));
                                                let _e2396 = pow(select(_e2393, 1f, (_e2393 > 1f)), 5f);
                                                let _e2397 = fma((1f - _e2320), _e2396, _e2320);
                                                let _e2398 = fma((1f - _e2321), _e2396, _e2321);
                                                let _e2399 = fma((1f - _e2322), _e2396, _e2322);
                                                let _e2406 = (((_e2342 * _e2342) / (pow(fma((_e2353 * _e2353), fma(_e2342, _e2342, -1f), 1f), 2f) * 3.1415927f)) * ((_e2366 / fma(_e2366, _e2378, _e2376)) * (_e2373 / fma(_e2373, _e2378, _e2376))));
                                                let _e2411 = fma((4f * _e2366), _e2373, 0.0001f);
                                                if ((_e2036.member_3 == 4294967295u) != true) {
                                                    let _e2433 = global_1.member[_e2036.member_3];
                                                    let _e2437 = global_1.member[(_e2036.member_3 + 1u)];
                                                    let _e2441 = global_1.member[(_e2036.member_3 + 3u)];
                                                    let _e2445 = global_1.member[(_e2036.member_3 + 4u)];
                                                    let _e2449 = global_1.member[(_e2036.member_3 + 5u)];
                                                    let _e2454 = global_1.member[(_e2036.member_3 + 6u)];
                                                    let _e2459 = global_1.member[(_e2036.member_3 + 7u)];
                                                    let _e2462 = global_1.member[_e1971];
                                                    let _e2466 = global_1.member[(_e1971 + 1u)];
                                                    let _e2468 = select(_e2433, 4294967295u, (0u >= _e2437));
                                                    let _e2471 = global_1.member[_e2468];
                                                    let _e2476 = global_1.member[(_e2468 + 1u)];
                                                    let _e2481 = global_1.member[(_e2468 + 2u)];
                                                    let _e2486 = global_1.member[(_e2468 + 3u)];
                                                    let _e2491 = global_1.member[(_e2468 + 4u)];
                                                    let _e2496 = global_1.member[(_e2468 + 5u)];
                                                    let _e2501 = global_1.member[(_e2468 + 6u)];
                                                    let _e2506 = global_1.member[(_e2468 + 7u)];
                                                    let _e2511 = global_1.member[(_e2468 + 8u)];
                                                    let _e2516 = global_1.member[(_e2468 + 9u)];
                                                    let _e2521 = global_1.member[(_e2468 + 10u)];
                                                    let _e2526 = global_1.member[(_e2468 + 11u)];
                                                    let _e2531 = global_1.member[(_e2468 + 12u)];
                                                    let _e2536 = global_1.member[(_e2468 + 13u)];
                                                    let _e2541 = global_1.member[(_e2468 + 14u)];
                                                    let _e2546 = global_1.member[(_e2468 + 15u)];
                                                    let _e2566 = (bitcast<f32>(_e2546) + fma(bitcast<f32>(_e2526), _e156.z, fma(bitcast<f32>(_e2506), _e156.y, (bitcast<f32>(_e2486) * _e156.x))));
                                                    let _e2567 = ((bitcast<f32>(_e2531) + fma(bitcast<f32>(_e2511), _e156.z, fma(bitcast<f32>(_e2491), _e156.y, (bitcast<f32>(_e2471) * _e156.x)))) / _e2566);
                                                    let _e2568 = ((bitcast<f32>(_e2536) + fma(bitcast<f32>(_e2516), _e156.z, fma(bitcast<f32>(_e2496), _e156.y, (bitcast<f32>(_e2476) * _e156.x)))) / _e2566);
                                                    let _e2569 = ((bitcast<f32>(_e2541) + fma(bitcast<f32>(_e2521), _e156.z, fma(bitcast<f32>(_e2501), _e156.y, (bitcast<f32>(_e2481) * _e156.x)))) / _e2566);
                                                    if (abs(_e2567) <= 1f) {
                                                        let _e2573 = (abs(_e2568) <= 1f);
                                                        if _e2573 {
                                                            phi_7734_ = (abs(_e2569) <= 1f);
                                                        } else {
                                                            phi_7734_ = bool();
                                                        }
                                                        let _e2577 = phi_7734_;
                                                        phi_7737_ = _e2577;
                                                        phi_7738_ = select(true, false, _e2573);
                                                    } else {
                                                        phi_7737_ = bool();
                                                        phi_7738_ = true;
                                                    }
                                                    let _e2580 = phi_7737_;
                                                    let _e2582 = phi_7738_;
                                                    if select(_e2580, false, _e2582) {
                                                        let _e2590 = global_1.member[select(_e2441, 4294967295u, (0u >= _e2445))];
                                                        let _e2593 = global_1.member[_e2590];
                                                        let _e2597 = global_1.member[(_e2590 + 1u)];
                                                        let _e2601 = global_1.member[(_e2590 + 2u)];
                                                        let _e2605 = global_1.member[(_e2590 + 3u)];
                                                        let _e2609 = global_1.member[(_e2590 + 4u)];
                                                        let _e2613 = global_1.member[(_e2590 + 6u)];
                                                        switch bitcast<i32>(_e2613) {
                                                            case 0: {
                                                                phi_4597_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_4597_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_4597_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_4597_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e2616 = phi_4597_;
                                                        let _e2620 = global_1.member[(_e2590 + 7u)];
                                                        switch bitcast<i32>(_e2620) {
                                                            case 0: {
                                                                phi_4606_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_4606_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_4606_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_4606_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e2623 = phi_4606_;
                                                        let _e2624 = bitcast<i32>(_e2459);
                                                        let _e2626 = f32(_e2601);
                                                        let _e2627 = f32(_e2605);
                                                        let _e2631 = type_37((_e2624 / -2i), (_e2624 / 2i), false);
                                                        phi_8840_ = _e1974;
                                                        phi_4634_ = _e2631;
                                                        phi_4637_ = 0f;
                                                        phi_4639_ = 0f;
                                                        loop {
                                                            let _e2633 = phi_8840_;
                                                            let _e2635 = phi_4634_;
                                                            let _e2637 = phi_4637_;
                                                            let _e2639 = phi_4639_;
                                                            local_3 = _e2637;
                                                            local_4 = _e2639;
                                                            if _e2635.member_2 {
                                                                phi_4651_ = true;
                                                            } else {
                                                                phi_4651_ = ((_e2635.member <= _e2635.member_1) != true);
                                                            }
                                                            let _e2646 = phi_4651_;
                                                            if _e2646 {
                                                                phi_4635_ = _e2635;
                                                                phi_4694_ = type_38(0u, type_38().member_1);
                                                            } else {
                                                                if (_e2635.member < _e2635.member_1) {
                                                                    let _e2654 = (_e2635.member + 1i);
                                                                    if select(false, true, ((false == (_e2654 > _e2635.member)) != false)) {
                                                                        phi_4679_ = type_38(0u, type_38().member_1);
                                                                    } else {
                                                                        phi_4679_ = type_38(1u, _e2654);
                                                                    }
                                                                    let _e2664 = phi_4679_;
                                                                    switch bitcast<i32>(_e2664.member) {
                                                                        case 0: {
                                                                            phi_8920_ = true;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            break;
                                                                        }
                                                                    }
                                                                    phi_4691_ = type_37(_e2664.member_1, _e2635.member_1, _e2635.member_2);
                                                                } else {
                                                                    phi_4691_ = type_37(_e2635.member, _e2635.member_1, true);
                                                                }
                                                                let _e2673 = phi_4691_;
                                                                phi_4635_ = _e2673;
                                                                phi_4694_ = type_38(1u, _e2635.member);
                                                            }
                                                            let _e2679 = phi_4635_;
                                                            let _e2681 = phi_4694_;
                                                            switch bitcast<i32>(_e2681.member) {
                                                                case 0: {
                                                                    phi_8921_ = _e2633;
                                                                    phi_4638_ = f32();
                                                                    phi_4640_ = f32();
                                                                    phi_4952_ = false;
                                                                    break;
                                                                }
                                                                case 1: {
                                                                    phi_4705_ = _e2631;
                                                                    phi_4708_ = _e2637;
                                                                    phi_4710_ = _e2639;
                                                                    loop {
                                                                        let _e2686 = phi_4705_;
                                                                        let _e2688 = phi_4708_;
                                                                        let _e2690 = phi_4710_;
                                                                        local_11 = _e2688;
                                                                        local_12 = _e2690;
                                                                        if _e2686.member_2 {
                                                                            phi_4722_ = true;
                                                                        } else {
                                                                            phi_4722_ = ((_e2686.member <= _e2686.member_1) != true);
                                                                        }
                                                                        let _e2697 = phi_4722_;
                                                                        if _e2697 {
                                                                            phi_4706_ = _e2686;
                                                                            phi_4765_ = type_38(0u, type_38().member_1);
                                                                        } else {
                                                                            if (_e2686.member < _e2686.member_1) {
                                                                                let _e2705 = (_e2686.member + 1i);
                                                                                if select(false, true, ((false == (_e2705 > _e2686.member)) != false)) {
                                                                                    phi_4750_ = type_38(0u, type_38().member_1);
                                                                                } else {
                                                                                    phi_4750_ = type_38(1u, _e2705);
                                                                                }
                                                                                let _e2715 = phi_4750_;
                                                                                switch bitcast<i32>(_e2715.member) {
                                                                                    case 0: {
                                                                                        phi_8824_ = true;
                                                                                        break;
                                                                                    }
                                                                                    case 1: {
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                phi_4762_ = type_37(_e2715.member_1, _e2686.member_1, _e2686.member_2);
                                                                            } else {
                                                                                phi_4762_ = type_37(_e2686.member, _e2686.member_1, true);
                                                                            }
                                                                            let _e2724 = phi_4762_;
                                                                            phi_4706_ = _e2724;
                                                                            phi_4765_ = type_38(1u, _e2686.member);
                                                                        }
                                                                        let _e2730 = phi_4706_;
                                                                        let _e2732 = phi_4765_;
                                                                        switch bitcast<i32>(_e2732.member) {
                                                                            case 0: {
                                                                                phi_4709_ = f32();
                                                                                phi_4711_ = f32();
                                                                                phi_4951_ = false;
                                                                                break;
                                                                            }
                                                                            case 1: {
                                                                                let _e2740 = fma((_e2567 + 1f), 0.5f, (f32(_e2681.member_1) * (1f / _e2626)));
                                                                                let _e2741 = fma(fma(_e2568, -1f, 1f), 0.5f, (f32(_e2732.member_1) * (1f / _e2627)));
                                                                                switch bitcast<i32>(_e2616) {
                                                                                    case 1: {
                                                                                        let _e2776 = abs(_e2740);
                                                                                        let _e2778 = (_e2776 % 1f);
                                                                                        if (_e2776 >= 1f) {
                                                                                            phi_7790_ = select(true, false, (_e2778 == 0f));
                                                                                        } else {
                                                                                            phi_7790_ = true;
                                                                                        }
                                                                                        let _e2782 = phi_7790_;
                                                                                        let _e2783 = select(1f, _e2778, _e2782);
                                                                                        if (select(-1f, 1f, (_e2740 >= 0f)) > 0f) {
                                                                                            phi_4797_ = _e2783;
                                                                                        } else {
                                                                                            phi_4797_ = (1f - _e2783);
                                                                                        }
                                                                                        let _e2787 = phi_4797_;
                                                                                        phi_4834_ = _e2787;
                                                                                        break;
                                                                                    }
                                                                                    case 2: {
                                                                                        let _e2750 = abs(_e2740);
                                                                                        let _e2757 = ((select(select(u32(_e2750), 0u, (_e2750 < 0f)), 4294967295u, (_e2750 > 4294967000f)) % 2u) == 0u);
                                                                                        let _e2759 = (_e2750 % 1f);
                                                                                        if (_e2750 >= 1f) {
                                                                                            phi_7773_ = select(true, false, (_e2759 == 0f));
                                                                                        } else {
                                                                                            phi_7773_ = true;
                                                                                        }
                                                                                        let _e2763 = phi_7773_;
                                                                                        let _e2764 = select(1f, _e2759, _e2763);
                                                                                        if (select(-1f, 1f, (_e2740 >= 0f)) > 0f) {
                                                                                            if _e2757 {
                                                                                                phi_4826_ = _e2764;
                                                                                            } else {
                                                                                                phi_4826_ = (1f - _e2764);
                                                                                            }
                                                                                            let _e2771 = phi_4826_;
                                                                                            phi_4832_ = _e2771;
                                                                                        } else {
                                                                                            if _e2757 {
                                                                                                phi_4831_ = (1f - _e2764);
                                                                                            } else {
                                                                                                phi_4831_ = _e2764;
                                                                                            }
                                                                                            let _e2768 = phi_4831_;
                                                                                            phi_4832_ = _e2768;
                                                                                        }
                                                                                        let _e2773 = phi_4832_;
                                                                                        phi_4834_ = _e2773;
                                                                                        break;
                                                                                    }
                                                                                    case 0: {
                                                                                        if (_e2740 > 1f) {
                                                                                            phi_7760_ = 0.9999999f;
                                                                                        } else {
                                                                                            phi_7760_ = select(_e2740, 0.00000011920929f, (_e2740 < 0f));
                                                                                        }
                                                                                        let _e2747 = phi_7760_;
                                                                                        phi_4834_ = _e2747;
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        phi_4834_ = f32();
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2789 = phi_4834_;
                                                                                switch bitcast<i32>(_e2623) {
                                                                                    case 1: {
                                                                                        let _e2824 = abs(_e2741);
                                                                                        let _e2826 = (_e2824 % 1f);
                                                                                        if (_e2824 >= 1f) {
                                                                                            phi_7838_ = select(true, false, (_e2826 == 0f));
                                                                                        } else {
                                                                                            phi_7838_ = true;
                                                                                        }
                                                                                        let _e2830 = phi_7838_;
                                                                                        let _e2831 = select(1f, _e2826, _e2830);
                                                                                        if (select(-1f, 1f, (_e2741 >= 0f)) > 0f) {
                                                                                            phi_4853_ = _e2831;
                                                                                        } else {
                                                                                            phi_4853_ = (1f - _e2831);
                                                                                        }
                                                                                        let _e2835 = phi_4853_;
                                                                                        phi_4890_ = _e2835;
                                                                                        break;
                                                                                    }
                                                                                    case 2: {
                                                                                        let _e2798 = abs(_e2741);
                                                                                        let _e2805 = ((select(select(u32(_e2798), 0u, (_e2798 < 0f)), 4294967295u, (_e2798 > 4294967000f)) % 2u) == 0u);
                                                                                        let _e2807 = (_e2798 % 1f);
                                                                                        if (_e2798 >= 1f) {
                                                                                            phi_7821_ = select(true, false, (_e2807 == 0f));
                                                                                        } else {
                                                                                            phi_7821_ = true;
                                                                                        }
                                                                                        let _e2811 = phi_7821_;
                                                                                        let _e2812 = select(1f, _e2807, _e2811);
                                                                                        if (select(-1f, 1f, (_e2741 >= 0f)) > 0f) {
                                                                                            if _e2805 {
                                                                                                phi_4882_ = _e2812;
                                                                                            } else {
                                                                                                phi_4882_ = (1f - _e2812);
                                                                                            }
                                                                                            let _e2819 = phi_4882_;
                                                                                            phi_4888_ = _e2819;
                                                                                        } else {
                                                                                            if _e2805 {
                                                                                                phi_4887_ = (1f - _e2812);
                                                                                            } else {
                                                                                                phi_4887_ = _e2812;
                                                                                            }
                                                                                            let _e2816 = phi_4887_;
                                                                                            phi_4888_ = _e2816;
                                                                                        }
                                                                                        let _e2821 = phi_4888_;
                                                                                        phi_4890_ = _e2821;
                                                                                        break;
                                                                                    }
                                                                                    case 0: {
                                                                                        if (_e2741 > 1f) {
                                                                                            phi_7808_ = 0.9999999f;
                                                                                        } else {
                                                                                            phi_7808_ = select(_e2741, 0.00000011920929f, (_e2741 < 0f));
                                                                                        }
                                                                                        let _e2795 = phi_7808_;
                                                                                        phi_4890_ = _e2795;
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        phi_4890_ = f32();
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2837 = phi_4890_;
                                                                                let _e2838 = (_e2789 * _e2626);
                                                                                let _e2844 = (_e2837 * _e2627);
                                                                                let _e2859 = vec3<f32>((f32((select(select(u32(_e2838), 0u, (_e2838 < 0f)), 4294967295u, (_e2838 > 4294967000f)) + _e2593)) / f32(_e2462)), (f32((select(select(u32(_e2844), 0u, (_e2844 < 0f)), 4294967295u, (_e2844 > 4294967000f)) + _e2597)) / f32(_e2466)), f32(_e2609));
                                                                                let _e2865 = textureSampleLevel(global_19, global_18, vec2<f32>(_e2859.x, _e2859.y), i32(_e2859.z), 0f);
                                                                                if ((_e2569 - max((bitcast<f32>(_e2454) * (1f - _e2372)), bitcast<f32>(_e2449))) > _e2865.x) {
                                                                                    phi_4949_ = (_e2690 + 1f);
                                                                                } else {
                                                                                    phi_4949_ = _e2690;
                                                                                }
                                                                                let _e2874 = phi_4949_;
                                                                                phi_4709_ = (_e2688 + 1f);
                                                                                phi_4711_ = _e2874;
                                                                                phi_4951_ = true;
                                                                                break;
                                                                            }
                                                                            default: {
                                                                                phi_4709_ = f32();
                                                                                phi_4711_ = f32();
                                                                                phi_4951_ = bool();
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e2877 = phi_4709_;
                                                                        let _e2879 = phi_4711_;
                                                                        let _e2881 = phi_4951_;
                                                                        continue;
                                                                        continuing {
                                                                            phi_4705_ = _e2730;
                                                                            phi_4708_ = _e2877;
                                                                            phi_4710_ = _e2879;
                                                                            phi_8824_ = _e2633;
                                                                            break if !(_e2881);
                                                                        }
                                                                    }
                                                                    let _e2884 = phi_8824_;
                                                                    phi_8920_ = _e2884;
                                                                    if _e2884 {
                                                                        break;
                                                                    }
                                                                    phi_8921_ = _e2884;
                                                                    let _e4560 = local_11;
                                                                    phi_4638_ = _e4560;
                                                                    let _e4563 = local_12;
                                                                    phi_4640_ = _e4563;
                                                                    phi_4952_ = true;
                                                                    break;
                                                                }
                                                                default: {
                                                                    phi_8921_ = _e2633;
                                                                    phi_4638_ = f32();
                                                                    phi_4640_ = f32();
                                                                    phi_4952_ = bool();
                                                                    break;
                                                                }
                                                            }
                                                            let _e2886 = phi_8921_;
                                                            let _e2888 = phi_4638_;
                                                            let _e2890 = phi_4640_;
                                                            let _e2892 = phi_4952_;
                                                            continue;
                                                            continuing {
                                                                phi_8840_ = _e2886;
                                                                phi_4634_ = _e2679;
                                                                phi_4637_ = _e2888;
                                                                phi_4639_ = _e2890;
                                                                phi_8920_ = _e2886;
                                                                break if !(_e2892);
                                                            }
                                                        }
                                                        let _e2895 = phi_8920_;
                                                        phi_9041_ = _e2895;
                                                        if _e2895 {
                                                            break;
                                                        }
                                                        let _e2897 = local_3;
                                                        let _e2900 = local_4;
                                                        phi_9048_ = _e2895;
                                                        phi_4955_ = (_e2900 / max(_e2897, 1f));
                                                    } else {
                                                        phi_9048_ = _e1974;
                                                        phi_4955_ = 0f;
                                                    }
                                                    let _e2903 = phi_9048_;
                                                    let _e2905 = phi_4955_;
                                                    phi_9047_ = _e2903;
                                                    phi_4956_ = _e2905;
                                                } else {
                                                    phi_9047_ = _e1974;
                                                    phi_4956_ = 0f;
                                                }
                                                let _e2907 = phi_9047_;
                                                let _e2909 = phi_4956_;
                                                phi_9046_ = _e2907;
                                                phi_4957_ = _e2909;
                                                phi_4958_ = vec3<f32>(((fma((((1f - _e2397) * _e2316) * _e1394), 0.31830987f, ((_e2406 * _e2397) / _e2411)) * (_e2217.member_4.x * _e2312)) * _e2373), ((fma((((1f - _e2398) * _e2316) * _e1396), 0.31830987f, ((_e2406 * _e2398) / _e2411)) * (_e2217.member_4.y * _e2312)) * _e2373), ((fma((((1f - _e2399) * _e2316) * _e1398), 0.31830987f, ((_e2406 * _e2399) / _e2411)) * (_e2217.member_4.z * _e2312)) * _e2373));
                                            }
                                            let _e2911 = phi_9046_;
                                            let _e2913 = phi_4957_;
                                            let _e2915 = phi_4958_;
                                            phi_9045_ = _e2911;
                                            phi_4960_ = _e2913;
                                            phi_4961_ = _e2915;
                                            phi_4962_ = select(true, false, _e2309);
                                            break;
                                        }
                                        default: {
                                            phi_9045_ = _e1974;
                                            phi_4960_ = f32();
                                            phi_4961_ = vec3<f32>();
                                            phi_4962_ = bool();
                                            break;
                                        }
                                    }
                                    let _e4255 = phi_9045_;
                                    let _e4257 = phi_4960_;
                                    let _e4259 = phi_4961_;
                                    let _e4261 = phi_4962_;
                                    if _e4261 {
                                        let _e4262 = (1f - _e4257);
                                        phi_4982_ = vec3<f32>(fma(_e4259.x, _e4262, _e1978.x), fma(_e4259.y, _e4262, _e1978.y), fma(_e4259.z, _e4262, _e1978.z));
                                    } else {
                                        phi_4982_ = vec3<f32>();
                                    }
                                    let _e4274 = phi_4982_;
                                    phi_9042_ = _e4255;
                                    phi_2409_ = select(_e4274, _e1978, vec3(select(true, false, _e4261)));
                                    phi_4988_ = true;
                                    break;
                                }
                                default: {
                                    phi_9042_ = _e1974;
                                    phi_2409_ = vec3<f32>();
                                    phi_4988_ = bool();
                                    break;
                                }
                            }
                            let _e4279 = phi_9042_;
                            let _e4281 = phi_2409_;
                            let _e4283 = phi_4988_;
                            continue;
                            continuing {
                                phi_8867_ = _e4279;
                                phi_2405_ = _e1991;
                                phi_2408_ = _e4281;
                                phi_9041_ = _e4279;
                                break if !(_e4283);
                            }
                        }
                        let _e4286 = phi_9041_;
                        phi_9053_ = _e4286;
                        if _e4286 {
                            break;
                        }
                        let _e4288 = fma(-(_e710.z), _e319.member_3, 1f);
                        let _e4292 = fma(0.04f, _e4288, (_e1394 * _e1406));
                        let _e4293 = fma(0.04f, _e4288, (_e1396 * _e1406));
                        let _e4294 = fma(0.04f, _e4288, (_e1398 * _e1406));
                        let _e4306 = fma(-(_e710.y), _e319.member_4, 1f);
                        let _e4313 = (1f - max(fma(_e1958.z, _e1962.z, fma(_e1958.x, _e1962.x, (_e1958.y * _e1962.y))), 0f));
                        let _e4315 = select(_e4313, 0f, (_e4313 < 0f));
                        let _e4318 = pow(select(_e4315, 1f, (_e4315 > 1f)), 5f);
                        let _e4319 = fma((max(_e4306, _e4292) - _e4292), _e4318, _e4292);
                        let _e4320 = fma((max(_e4306, _e4293) - _e4293), _e4318, _e4293);
                        let _e4321 = fma((max(_e4306, _e4294) - _e4294), _e4318, _e4294);
                        let _e4341 = local_8;
                        let _e4345 = local_9;
                        let _e4349 = local_10;
                        phi_9061_ = _e4286;
                        phi_5105_ = vec4<f32>(fma(_e1416, _e319.member_1, fma(fma(((1f - _e4319) * _e4288), (_e1425.x * _e1394), (_e1773.x * fma(_e4319, _e1789.x, _e1789.y))), _e1410, _e4341.x)), fma(_e1418, _e319.member_1, fma(fma(((1f - _e4320) * _e4288), (_e1425.y * _e1396), (_e1773.y * fma(_e4320, _e1789.x, _e1789.y))), _e1410, _e4345.y)), fma(_e1420, _e319.member_1, fma(fma(((1f - _e4321) * _e4288), (_e1425.z * _e1398), (_e1773.z * fma(_e4321, _e1789.x, _e1789.y))), _e1410, _e4349.z)), 1f);
                    } else {
                        phi_9061_ = false;
                        phi_5105_ = (vec4<f32>((_e150.x * _e516.x), (_e150.y * _e516.y), (_e150.z * _e516.z), (_e150.w * _e516.w)) * _e319.member_2);
                    }
                    let _e4357 = phi_9061_;
                    let _e4359 = phi_5105_;
                    global_20 = _e4359;
                    phi_9053_ = _e4357;
                    break;
                }
                case 1: {
                    let _e1931 = sqrt(fma(_e151.x, _e151.x, (_e151.y * _e151.y)));
                    if (_e1931 == 0f) {
                        phi_7254_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7254_ = (vec3<f32>(_e151.x, _e151.y, 0f) * (1f / _e1931));
                    }
                    let _e1936 = phi_7254_;
                    global_20 = vec4<f32>(((_e1936.x + 1f) * 0.5f), ((_e1936.y + 1f) * 0.5f), ((_e1936.z + 1f) * 0.5f), 1f);
                    phi_9053_ = false;
                    break;
                }
                case 2: {
                    let _e1910 = sqrt(fma(_e152.x, _e152.x, (_e152.y * _e152.y)));
                    if (_e1910 == 0f) {
                        phi_7205_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7205_ = (vec3<f32>(_e152.x, _e152.y, 0f) * (1f / _e1910));
                    }
                    let _e1915 = phi_7205_;
                    global_20 = vec4<f32>(((_e1915.x + 1f) * 0.5f), ((_e1915.y + 1f) * 0.5f), ((_e1915.z + 1f) * 0.5f), 1f);
                    phi_9053_ = false;
                    break;
                }
                case 3: {
                    if _e1752 {
                        phi_7156_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7156_ = (_e1379 * (1f / _e1751));
                    }
                    let _e1894 = phi_7156_;
                    global_20 = vec4<f32>(((_e1894.x + 1f) * 0.5f), ((_e1894.y + 1f) * 0.5f), ((_e1894.z + 1f) * 0.5f), 1f);
                    phi_9053_ = false;
                    break;
                }
                case 4: {
                    global_20 = _e150;
                    phi_9053_ = false;
                    break;
                }
                case 5: {
                    let _e1875 = sqrt(fma(_e153.z, _e153.z, fma(_e153.x, _e153.x, (_e153.y * _e153.y))));
                    if (_e1875 == 0f) {
                        phi_7107_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7107_ = (_e153 * (1f / _e1875));
                    }
                    let _e1880 = phi_7107_;
                    global_20 = vec4<f32>(((_e1880.x + 1f) * 0.5f), ((_e1880.y + 1f) * 0.5f), ((_e1880.z + 1f) * 0.5f), 1f);
                    phi_9053_ = false;
                    break;
                }
                case 6: {
                    let _e1853 = sqrt(fma(_e1377.z, _e1377.z, fma(_e1377.x, _e1377.x, (_e1377.y * _e1377.y))));
                    if (_e1853 == 0f) {
                        phi_7058_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7058_ = (_e1377 * (1f / _e1853));
                    }
                    let _e1858 = phi_7058_;
                    global_20 = vec4<f32>(((_e1858.x + 1f) * 0.5f), ((_e1858.y + 1f) * 0.5f), ((_e1858.z + 1f) * 0.5f), 1f);
                    phi_9053_ = false;
                    break;
                }
                case 7: {
                    let _e1831 = sqrt(fma(_e154.z, _e154.z, fma(_e154.x, _e154.x, (_e154.y * _e154.y))));
                    if (_e1831 == 0f) {
                        phi_7009_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7009_ = (_e154 * (1f / _e1831));
                    }
                    let _e1836 = phi_7009_;
                    global_20 = vec4<f32>(((_e1836.x + 1f) * 0.5f), ((_e1836.y + 1f) * 0.5f), ((_e1836.z + 1f) * 0.5f), 1f);
                    phi_9053_ = false;
                    break;
                }
                case 8: {
                    let _e1809 = sqrt(fma(_e155.z, _e155.z, fma(_e155.x, _e155.x, (_e155.y * _e155.y))));
                    if (_e1809 == 0f) {
                        phi_6960_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6960_ = (_e155 * (1f / _e1809));
                    }
                    let _e1814 = phi_6960_;
                    global_20 = vec4<f32>(((_e1814.x + 1f) * 0.5f), ((_e1814.y + 1f) * 0.5f), ((_e1814.z + 1f) * 0.5f), 1f);
                    phi_9053_ = false;
                    break;
                }
                case 9: {
                    global_20 = vec4<f32>(_e1425.x, _e1425.y, _e1425.z, 1f);
                    phi_9053_ = false;
                    break;
                }
                case 10: {
                    global_20 = vec4<f32>(_e1773.x, _e1773.y, _e1773.z, 1f);
                    phi_9053_ = false;
                    break;
                }
                case 11: {
                    global_20 = vec4<f32>(_e1789.x, _e1789.y, 1f, 1f);
                    phi_9053_ = false;
                    break;
                }
                case 12: {
                    global_20 = (vec4<f32>(_e1383, _e1386, _e1389, (_e516.w * _e319.member_2.w)) * _e150);
                    phi_9053_ = false;
                    break;
                }
                case 13: {
                    global_20 = vec4<f32>(_e1403, _e1403, _e1403, 1f);
                    phi_9053_ = false;
                    break;
                }
                case 14: {
                    global_20 = vec4<f32>(_e1406, _e1406, _e1406, 1f);
                    phi_9053_ = false;
                    break;
                }
                case 15: {
                    global_20 = vec4<f32>(_e1410, _e1410, _e1410, 1f);
                    phi_9053_ = false;
                    break;
                }
                case 16: {
                    global_20 = vec4<f32>((_e1416 * _e319.member_1), (_e1418 * _e319.member_1), (_e1420 * _e319.member_1), 1f);
                    phi_9053_ = false;
                    break;
                }
                case 17: {
                    global_20 = vec4<f32>(_e1292.x, _e1292.y, _e1292.z, 1f);
                    phi_9053_ = false;
                    break;
                }
                case 18: {
                    global_20 = vec4<f32>(_e319.member.x, _e319.member.y, _e319.member.z, 1f);
                    phi_9053_ = false;
                    break;
                }
                case 19: {
                    global_20 = vec4<f32>(_e319.member_1, _e319.member_1, _e319.member_1, 1f);
                    phi_9053_ = false;
                    break;
                }
                default: {
                    phi_9053_ = false;
                    break;
                }
            }
            let _e4361 = phi_9053_;
            if _e4361 {
                break;
            }
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
