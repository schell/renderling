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
    member: i32,
    member_1: i32,
    member_2: bool,
}

struct type_36 {
    member: u32,
    member_1: i32,
}

struct type_37 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
    member_3: f32,
    member_4: vec4<f32>,
    member_5: f32,
}

struct type_38 {
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
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var phi_696_: u32;
    var phi_5692_: bool;
    var phi_824_: type_33;
    var phi_828_: type_33;
    var phi_5729_: bool;
    var phi_868_: u32;
    var phi_877_: u32;
    var phi_890_: type_15;
    var phi_5751_: f32;
    var phi_5764_: bool;
    var phi_944_: f32;
    var phi_939_: f32;
    var phi_945_: f32;
    var phi_5781_: bool;
    var phi_910_: f32;
    var phi_947_: f32;
    var phi_5799_: f32;
    var phi_5812_: bool;
    var phi_1002_: f32;
    var phi_997_: f32;
    var phi_1003_: f32;
    var phi_5829_: bool;
    var phi_968_: f32;
    var phi_1005_: f32;
    var phi_5865_: bool;
    var phi_1088_: u32;
    var phi_1097_: u32;
    var phi_1110_: type_15;
    var phi_5886_: f32;
    var phi_5899_: bool;
    var phi_1164_: f32;
    var phi_1159_: f32;
    var phi_1165_: f32;
    var phi_5916_: bool;
    var phi_1130_: f32;
    var phi_1167_: f32;
    var phi_5934_: f32;
    var phi_5947_: bool;
    var phi_1222_: f32;
    var phi_1217_: f32;
    var phi_1223_: f32;
    var phi_5964_: bool;
    var phi_1188_: f32;
    var phi_1225_: f32;
    var phi_6000_: bool;
    var phi_1308_: u32;
    var phi_1317_: u32;
    var phi_1330_: type_15;
    var phi_6021_: f32;
    var phi_6034_: bool;
    var phi_1384_: f32;
    var phi_1379_: f32;
    var phi_1385_: f32;
    var phi_6051_: bool;
    var phi_1350_: f32;
    var phi_1387_: f32;
    var phi_6069_: f32;
    var phi_6082_: bool;
    var phi_1442_: f32;
    var phi_1437_: f32;
    var phi_1443_: f32;
    var phi_6099_: bool;
    var phi_1408_: f32;
    var phi_1445_: f32;
    var phi_6135_: bool;
    var phi_1528_: u32;
    var phi_1537_: u32;
    var phi_1550_: type_15;
    var phi_6156_: f32;
    var phi_6169_: bool;
    var phi_1604_: f32;
    var phi_1599_: f32;
    var phi_1605_: f32;
    var phi_6186_: bool;
    var phi_1570_: f32;
    var phi_1607_: f32;
    var phi_6204_: f32;
    var phi_6217_: bool;
    var phi_1662_: f32;
    var phi_1657_: f32;
    var phi_1663_: f32;
    var phi_6234_: bool;
    var phi_1628_: f32;
    var phi_1665_: f32;
    var phi_6270_: bool;
    var phi_1748_: u32;
    var phi_1757_: u32;
    var phi_1770_: type_15;
    var phi_6291_: f32;
    var phi_6304_: bool;
    var phi_1824_: f32;
    var phi_1819_: f32;
    var phi_1825_: f32;
    var phi_6321_: bool;
    var phi_1790_: f32;
    var phi_1827_: f32;
    var phi_6339_: f32;
    var phi_6352_: bool;
    var phi_1882_: f32;
    var phi_1877_: f32;
    var phi_1883_: f32;
    var phi_6369_: bool;
    var phi_1848_: f32;
    var phi_1885_: f32;
    var phi_6427_: vec3<f32>;
    var phi_6462_: vec3<f32>;
    var phi_6497_: vec3<f32>;
    var phi_6532_: vec3<f32>;
    var phi_6567_: vec3<f32>;
    var phi_1979_: vec3<f32>;
    var phi_1980_: vec3<f32>;
    var phi_6599_: bool;
    var phi_2187_: type_14;
    var phi_2188_: type_14;
    var phi_2211_: type_14;
    var phi_2238_: bool;
    var phi_2244_: type_14;
    var phi_2245_: type_14;
    var phi_2268_: type_14;
    var phi_2291_: bool;
    var phi_2312_: type_25;
    var phi_6671_: vec3<f32>;
    var phi_6730_: vec3<f32>;
    var phi_6804_: vec3<f32>;
    var phi_6864_: vec3<f32>;
    var phi_6913_: vec3<f32>;
    var phi_6962_: vec3<f32>;
    var phi_7011_: vec3<f32>;
    var phi_7060_: vec3<f32>;
    var phi_7109_: vec3<f32>;
    var phi_7158_: vec3<f32>;
    var phi_7197_: vec3<f32>;
    var phi_7232_: vec3<f32>;
    var phi_8761_: bool;
    var phi_2379_: type_14;
    var phi_2382_: vec3<f32>;
    var phi_2380_: type_14;
    var phi_2405_: type_14;
    var phi_7258_: u32;
    var phi_7277_: bool;
    var phi_2422_: u32;
    var phi_7301_: bool;
    var phi_2434_: u32;
    var phi_2448_: type_30;
    var phi_7333_: bool;
    var phi_2498_: type_31;
    var phi_7413_: bool;
    var phi_4005_: type_37;
    var phi_7463_: vec3<f32>;
    var phi_7498_: vec3<f32>;
    var phi_4141_: type_38;
    var phi_7533_: vec3<f32>;
    var phi_7638_: bool;
    var phi_7641_: bool;
    var phi_7642_: bool;
    var phi_4503_: u32;
    var phi_4512_: u32;
    var phi_8734_: bool;
    var phi_4540_: type_35;
    var phi_4543_: f32;
    var phi_4545_: f32;
    var phi_4557_: bool;
    var phi_4585_: type_36;
    var phi_4597_: type_35;
    var phi_4541_: type_35;
    var phi_4600_: type_36;
    var phi_4611_: type_35;
    var phi_4614_: f32;
    var phi_4616_: f32;
    var phi_4628_: bool;
    var phi_4656_: type_36;
    var phi_4668_: type_35;
    var phi_4612_: type_35;
    var phi_4671_: type_36;
    var phi_7664_: f32;
    var phi_7677_: bool;
    var phi_4737_: f32;
    var phi_4732_: f32;
    var phi_4738_: f32;
    var phi_7694_: bool;
    var phi_4703_: f32;
    var phi_4740_: f32;
    var phi_7712_: f32;
    var phi_7725_: bool;
    var phi_4793_: f32;
    var phi_4788_: f32;
    var phi_4794_: f32;
    var phi_7742_: bool;
    var phi_4759_: f32;
    var phi_4796_: f32;
    var phi_4615_: f32;
    var phi_4617_: f32;
    var phi_4857_: bool;
    var phi_8718_: bool;
    var phi_8815_: bool;
    var phi_4544_: f32;
    var phi_4546_: f32;
    var phi_4858_: bool;
    var phi_8814_: bool;
    var local_2: f32;
    var local_3: f32;
    var phi_8886_: bool;
    var phi_4861_: f32;
    var phi_8885_: bool;
    var phi_4862_: f32;
    var phi_8884_: bool;
    var phi_4863_: f32;
    var phi_4864_: vec3<f32>;
    var phi_7779_: bool;
    var phi_3337_: type_34;
    var phi_7826_: vec3<f32>;
    var phi_7861_: vec3<f32>;
    var phi_7981_: vec2<f32>;
    var phi_7984_: vec2<f32>;
    var phi_7985_: u32;
    var phi_7988_: vec2<f32>;
    var phi_7989_: u32;
    var phi_7990_: bool;
    var phi_8015_: vec2<f32>;
    var phi_8018_: vec2<f32>;
    var phi_8019_: u32;
    var phi_8022_: vec2<f32>;
    var phi_8023_: u32;
    var phi_8024_: bool;
    var phi_8045_: vec2<f32>;
    var phi_8048_: vec2<f32>;
    var phi_8049_: u32;
    var phi_8051_: vec2<f32>;
    var phi_8052_: u32;
    var phi_3591_: u32;
    var phi_8081_: bool;
    var phi_8084_: bool;
    var phi_8085_: bool;
    var phi_3719_: u32;
    var phi_3750_: u32;
    var phi_3759_: u32;
    var phi_8107_: f32;
    var phi_8120_: bool;
    var phi_3819_: f32;
    var phi_3814_: f32;
    var phi_3820_: f32;
    var phi_8137_: bool;
    var phi_3785_: f32;
    var phi_3822_: f32;
    var phi_8155_: f32;
    var phi_8168_: bool;
    var phi_3877_: f32;
    var phi_3872_: f32;
    var phi_3878_: f32;
    var phi_8185_: bool;
    var phi_3843_: f32;
    var phi_3880_: f32;
    var phi_3939_: f32;
    var phi_3940_: f32;
    var phi_3941_: f32;
    var phi_3942_: vec3<f32>;
    var phi_8222_: bool;
    var phi_2546_: type_34;
    var phi_8269_: vec3<f32>;
    var phi_8304_: vec3<f32>;
    var phi_8409_: bool;
    var phi_8412_: bool;
    var phi_8413_: bool;
    var phi_2937_: u32;
    var phi_2946_: u32;
    var phi_8832_: bool;
    var phi_2974_: type_35;
    var phi_2977_: f32;
    var phi_2979_: f32;
    var phi_2991_: bool;
    var phi_3019_: type_36;
    var phi_3031_: type_35;
    var phi_2975_: type_35;
    var phi_3034_: type_36;
    var phi_3045_: type_35;
    var phi_3048_: f32;
    var phi_3050_: f32;
    var phi_3062_: bool;
    var phi_3090_: type_36;
    var phi_3102_: type_35;
    var phi_3046_: type_35;
    var phi_3105_: type_36;
    var phi_8435_: f32;
    var phi_8448_: bool;
    var phi_3171_: f32;
    var phi_3166_: f32;
    var phi_3172_: f32;
    var phi_8465_: bool;
    var phi_3137_: f32;
    var phi_3174_: f32;
    var phi_8483_: f32;
    var phi_8496_: bool;
    var phi_3227_: f32;
    var phi_3222_: f32;
    var phi_3228_: f32;
    var phi_8513_: bool;
    var phi_3193_: f32;
    var phi_3230_: f32;
    var phi_3049_: f32;
    var phi_3051_: f32;
    var phi_3291_: bool;
    var phi_8816_: bool;
    var phi_8878_: bool;
    var phi_2978_: f32;
    var phi_2980_: f32;
    var phi_3292_: bool;
    var phi_8877_: bool;
    var local_4: f32;
    var local_5: f32;
    var phi_8945_: bool;
    var phi_3295_: f32;
    var phi_8944_: bool;
    var phi_3296_: f32;
    var phi_8883_: bool;
    var phi_4866_: f32;
    var phi_4867_: vec3<f32>;
    var phi_4868_: bool;
    var phi_4888_: vec3<f32>;
    var phi_8880_: bool;
    var phi_2383_: vec3<f32>;
    var phi_4894_: bool;
    var phi_8879_: bool;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var local_8: vec3<f32>;
    var phi_8954_: bool;
    var phi_5011_: vec4<f32>;
    var phi_8946_: bool;
    var local_9: f32;
    var local_10: f32;
    var local_11: f32;
    var local_12: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e124 = arrayLength((&global.member));
            let _e126 = arrayLength((&global_1.member));
            let _e127 = global_2;
            let _e128 = global_3;
            let _e129 = global_4;
            let _e130 = global_5;
            let _e131 = global_6;
            let _e132 = global_7;
            let _e133 = global_8;
            let _e134 = global_9;
            let _e138 = global.member[(_e127 + 9u)];
            let _e142 = global.member[(_e127 + 11u)];
            let _e146 = global.member[(_e127 + 17u)];
            let _e149 = global.member[_e146];
            let _e153 = global.member[(_e146 + 1u)];
            let _e157 = global.member[(_e146 + 4u)];
            switch bitcast<i32>(_e157) {
                case 0: {
                    phi_696_ = 0u;
                    break;
                }
                case 1: {
                    phi_696_ = 1u;
                    break;
                }
                case 2: {
                    phi_696_ = 2u;
                    break;
                }
                case 3: {
                    phi_696_ = 3u;
                    break;
                }
                case 4: {
                    phi_696_ = 4u;
                    break;
                }
                case 5: {
                    phi_696_ = 5u;
                    break;
                }
                case 6: {
                    phi_696_ = 6u;
                    break;
                }
                case 7: {
                    phi_696_ = 7u;
                    break;
                }
                case 8: {
                    phi_696_ = 8u;
                    break;
                }
                case 9: {
                    phi_696_ = 9u;
                    break;
                }
                case 10: {
                    phi_696_ = 10u;
                    break;
                }
                case 11: {
                    phi_696_ = 11u;
                    break;
                }
                case 12: {
                    phi_696_ = 12u;
                    break;
                }
                case 13: {
                    phi_696_ = 13u;
                    break;
                }
                case 14: {
                    phi_696_ = 14u;
                    break;
                }
                case 15: {
                    phi_696_ = 15u;
                    break;
                }
                case 16: {
                    phi_696_ = 16u;
                    break;
                }
                case 17: {
                    phi_696_ = 17u;
                    break;
                }
                case 18: {
                    phi_696_ = 18u;
                    break;
                }
                case 19: {
                    phi_696_ = 19u;
                    break;
                }
                default: {
                    phi_696_ = 0u;
                    break;
                }
            }
            let _e160 = phi_696_;
            let _e164 = global.member[(_e146 + 5u)];
            if (_e142 == 4294967295u) {
                phi_828_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
            } else {
                if (_e124 >= 22u) {
                    phi_5692_ = (_e142 <= (_e124 - 22u));
                } else {
                    phi_5692_ = false;
                }
                let _e171 = phi_5692_;
                if _e171 {
                    let _e174 = global.member[_e142];
                    let _e179 = global.member[(_e142 + 1u)];
                    let _e184 = global.member[(_e142 + 2u)];
                    let _e190 = global.member[(_e142 + 3u)];
                    let _e195 = global.member[(_e142 + 4u)];
                    let _e200 = global.member[(_e142 + 5u)];
                    let _e205 = global.member[(_e142 + 6u)];
                    let _e210 = global.member[(_e142 + 7u)];
                    let _e216 = global.member[(_e142 + 8u)];
                    let _e221 = global.member[(_e142 + 9u)];
                    let _e226 = global.member[(_e142 + 10u)];
                    let _e230 = global.member[(_e142 + 11u)];
                    let _e234 = global.member[(_e142 + 12u)];
                    let _e238 = global.member[(_e142 + 13u)];
                    let _e242 = global.member[(_e142 + 14u)];
                    let _e246 = global.member[(_e142 + 15u)];
                    let _e250 = global.member[(_e142 + 16u)];
                    let _e254 = global.member[(_e142 + 17u)];
                    let _e258 = global.member[(_e142 + 18u)];
                    let _e262 = global.member[(_e142 + 19u)];
                    let _e266 = global.member[(_e142 + 20u)];
                    let _e271 = global.member[(_e142 + 21u)];
                    phi_824_ = type_33(vec3<f32>(bitcast<f32>(_e174), bitcast<f32>(_e179), bitcast<f32>(_e184)), bitcast<f32>(_e190), vec4<f32>(bitcast<f32>(_e195), bitcast<f32>(_e200), bitcast<f32>(_e205), bitcast<f32>(_e210)), bitcast<f32>(_e216), bitcast<f32>(_e221), _e226, _e230, _e234, _e238, _e242, _e246, _e250, _e254, _e258, _e262, (_e266 == 1u), bitcast<f32>(_e271));
                } else {
                    phi_824_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
                }
                let _e275 = phi_824_;
                phi_828_ = type_33(_e275.member, _e275.member_1, _e275.member_2, _e275.member_3, _e275.member_4, _e275.member_5, _e275.member_6, _e275.member_7, _e275.member_8, _e275.member_9, _e275.member_10, _e275.member_11, _e275.member_12, _e275.member_13, _e275.member_14, (_e275.member_15 && (_e164 == 1u)), _e275.member_16);
            }
            let _e297 = phi_828_;
            let _e301 = select(_e130, _e129, vec2((_e297.member_10 == 0u)));
            let _e303 = (_e124 >= 8u);
            if _e303 {
                phi_5729_ = (_e297.member_5 <= (_e124 - 8u));
            } else {
                phi_5729_ = false;
            }
            let _e307 = phi_5729_;
            if _e307 {
                let _e310 = global.member[_e297.member_5];
                let _e314 = global.member[(_e297.member_5 + 1u)];
                let _e319 = global.member[(_e297.member_5 + 2u)];
                let _e323 = global.member[(_e297.member_5 + 3u)];
                let _e328 = global.member[(_e297.member_5 + 4u)];
                let _e332 = global.member[(_e297.member_5 + 5u)];
                let _e336 = global.member[(_e297.member_5 + 6u)];
                switch bitcast<i32>(_e336) {
                    case 0: {
                        phi_868_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_868_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_868_ = 2u;
                        break;
                    }
                    default: {
                        phi_868_ = 0u;
                        break;
                    }
                }
                let _e339 = phi_868_;
                let _e343 = global.member[(_e297.member_5 + 7u)];
                switch bitcast<i32>(_e343) {
                    case 0: {
                        phi_877_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_877_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_877_ = 2u;
                        break;
                    }
                    default: {
                        phi_877_ = 0u;
                        break;
                    }
                }
                let _e346 = phi_877_;
                phi_890_ = type_15(type_14(_e339, _e346), vec2<u32>(_e310, _e314), vec2<u32>(_e319, _e323), _e328, _e332);
            } else {
                phi_890_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e350 = phi_890_;
            switch bitcast<i32>(_e350.member.member) {
                case 1: {
                    let _e388 = abs(_e301.x);
                    let _e390 = (_e388 % 1f);
                    if (_e388 >= 1f) {
                        phi_5781_ = select(true, false, (_e390 == 0f));
                    } else {
                        phi_5781_ = true;
                    }
                    let _e394 = phi_5781_;
                    let _e395 = select(1f, _e390, _e394);
                    if (select(-1f, 1f, (_e301.x >= 0f)) > 0f) {
                        phi_910_ = _e395;
                    } else {
                        phi_910_ = (1f - _e395);
                    }
                    let _e399 = phi_910_;
                    phi_947_ = _e399;
                    break;
                }
                case 2: {
                    let _e362 = abs(_e301.x);
                    let _e369 = ((select(select(u32(_e362), 0u, (_e362 < 0f)), 4294967295u, (_e362 > 4294967000f)) % 2u) == 0u);
                    let _e371 = (_e362 % 1f);
                    if (_e362 >= 1f) {
                        phi_5764_ = select(true, false, (_e371 == 0f));
                    } else {
                        phi_5764_ = true;
                    }
                    let _e375 = phi_5764_;
                    let _e376 = select(1f, _e371, _e375);
                    if (select(-1f, 1f, (_e301.x >= 0f)) > 0f) {
                        if _e369 {
                            phi_939_ = _e376;
                        } else {
                            phi_939_ = (1f - _e376);
                        }
                        let _e383 = phi_939_;
                        phi_945_ = _e383;
                    } else {
                        if _e369 {
                            phi_944_ = (1f - _e376);
                        } else {
                            phi_944_ = _e376;
                        }
                        let _e380 = phi_944_;
                        phi_945_ = _e380;
                    }
                    let _e385 = phi_945_;
                    phi_947_ = _e385;
                    break;
                }
                case 0: {
                    if (_e301.x > 1f) {
                        phi_5751_ = 0.9999999f;
                    } else {
                        phi_5751_ = select(_e301.x, 0.00000011920929f, (_e301.x < 0f));
                    }
                    let _e359 = phi_5751_;
                    phi_947_ = _e359;
                    break;
                }
                default: {
                    phi_947_ = f32();
                    break;
                }
            }
            let _e401 = phi_947_;
            switch bitcast<i32>(_e350.member.member_1) {
                case 1: {
                    let _e439 = abs(_e301.y);
                    let _e441 = (_e439 % 1f);
                    if (_e439 >= 1f) {
                        phi_5829_ = select(true, false, (_e441 == 0f));
                    } else {
                        phi_5829_ = true;
                    }
                    let _e445 = phi_5829_;
                    let _e446 = select(1f, _e441, _e445);
                    if (select(-1f, 1f, (_e301.y >= 0f)) > 0f) {
                        phi_968_ = _e446;
                    } else {
                        phi_968_ = (1f - _e446);
                    }
                    let _e450 = phi_968_;
                    phi_1005_ = _e450;
                    break;
                }
                case 2: {
                    let _e413 = abs(_e301.y);
                    let _e420 = ((select(select(u32(_e413), 0u, (_e413 < 0f)), 4294967295u, (_e413 > 4294967000f)) % 2u) == 0u);
                    let _e422 = (_e413 % 1f);
                    if (_e413 >= 1f) {
                        phi_5812_ = select(true, false, (_e422 == 0f));
                    } else {
                        phi_5812_ = true;
                    }
                    let _e426 = phi_5812_;
                    let _e427 = select(1f, _e422, _e426);
                    if (select(-1f, 1f, (_e301.y >= 0f)) > 0f) {
                        if _e420 {
                            phi_997_ = _e427;
                        } else {
                            phi_997_ = (1f - _e427);
                        }
                        let _e434 = phi_997_;
                        phi_1003_ = _e434;
                    } else {
                        if _e420 {
                            phi_1002_ = (1f - _e427);
                        } else {
                            phi_1002_ = _e427;
                        }
                        let _e431 = phi_1002_;
                        phi_1003_ = _e431;
                    }
                    let _e436 = phi_1003_;
                    phi_1005_ = _e436;
                    break;
                }
                case 0: {
                    if (_e301.y > 1f) {
                        phi_5799_ = 0.9999999f;
                    } else {
                        phi_5799_ = select(_e301.y, 0.00000011920929f, (_e301.y < 0f));
                    }
                    let _e410 = phi_5799_;
                    phi_1005_ = _e410;
                    break;
                }
                default: {
                    phi_1005_ = f32();
                    break;
                }
            }
            let _e452 = phi_1005_;
            let _e456 = (_e401 * f32(_e350.member_2.x));
            let _e465 = (_e452 * f32(_e350.member_2.y));
            let _e477 = f32(_e149);
            let _e478 = f32(_e153);
            let _e485 = vec3<f32>((f32((select(select(u32(_e456), 0u, (_e456 < 0f)), 4294967295u, (_e456 > 4294967000f)) + _e350.member_1.x)) / _e477), (f32((select(select(u32(_e465), 0u, (_e465 < 0f)), 4294967295u, (_e465 > 4294967000f)) + _e350.member_1.y)) / _e478), f32(_e350.member_3));
            let _e491 = textureSampleLevel(global_11, global_10, vec2<f32>(_e485.x, _e485.y), i32(_e485.z), 0f);
            let _e494 = select(_e491, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e297.member_5 == 4294967295u)));
            let _e498 = select(_e130, _e129, vec2((_e297.member_11 == 0u)));
            if _e303 {
                phi_5865_ = (_e297.member_6 <= (_e124 - 8u));
            } else {
                phi_5865_ = false;
            }
            let _e503 = phi_5865_;
            if _e503 {
                let _e506 = global.member[_e297.member_6];
                let _e510 = global.member[(_e297.member_6 + 1u)];
                let _e515 = global.member[(_e297.member_6 + 2u)];
                let _e519 = global.member[(_e297.member_6 + 3u)];
                let _e524 = global.member[(_e297.member_6 + 4u)];
                let _e528 = global.member[(_e297.member_6 + 5u)];
                let _e532 = global.member[(_e297.member_6 + 6u)];
                switch bitcast<i32>(_e532) {
                    case 0: {
                        phi_1088_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1088_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1088_ = 2u;
                        break;
                    }
                    default: {
                        phi_1088_ = 0u;
                        break;
                    }
                }
                let _e535 = phi_1088_;
                let _e539 = global.member[(_e297.member_6 + 7u)];
                switch bitcast<i32>(_e539) {
                    case 0: {
                        phi_1097_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1097_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1097_ = 2u;
                        break;
                    }
                    default: {
                        phi_1097_ = 0u;
                        break;
                    }
                }
                let _e542 = phi_1097_;
                phi_1110_ = type_15(type_14(_e535, _e542), vec2<u32>(_e506, _e510), vec2<u32>(_e515, _e519), _e524, _e528);
            } else {
                phi_1110_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e546 = phi_1110_;
            switch bitcast<i32>(_e546.member.member) {
                case 1: {
                    let _e584 = abs(_e498.x);
                    let _e586 = (_e584 % 1f);
                    if (_e584 >= 1f) {
                        phi_5916_ = select(true, false, (_e586 == 0f));
                    } else {
                        phi_5916_ = true;
                    }
                    let _e590 = phi_5916_;
                    let _e591 = select(1f, _e586, _e590);
                    if (select(-1f, 1f, (_e498.x >= 0f)) > 0f) {
                        phi_1130_ = _e591;
                    } else {
                        phi_1130_ = (1f - _e591);
                    }
                    let _e595 = phi_1130_;
                    phi_1167_ = _e595;
                    break;
                }
                case 2: {
                    let _e558 = abs(_e498.x);
                    let _e565 = ((select(select(u32(_e558), 0u, (_e558 < 0f)), 4294967295u, (_e558 > 4294967000f)) % 2u) == 0u);
                    let _e567 = (_e558 % 1f);
                    if (_e558 >= 1f) {
                        phi_5899_ = select(true, false, (_e567 == 0f));
                    } else {
                        phi_5899_ = true;
                    }
                    let _e571 = phi_5899_;
                    let _e572 = select(1f, _e567, _e571);
                    if (select(-1f, 1f, (_e498.x >= 0f)) > 0f) {
                        if _e565 {
                            phi_1159_ = _e572;
                        } else {
                            phi_1159_ = (1f - _e572);
                        }
                        let _e579 = phi_1159_;
                        phi_1165_ = _e579;
                    } else {
                        if _e565 {
                            phi_1164_ = (1f - _e572);
                        } else {
                            phi_1164_ = _e572;
                        }
                        let _e576 = phi_1164_;
                        phi_1165_ = _e576;
                    }
                    let _e581 = phi_1165_;
                    phi_1167_ = _e581;
                    break;
                }
                case 0: {
                    if (_e498.x > 1f) {
                        phi_5886_ = 0.9999999f;
                    } else {
                        phi_5886_ = select(_e498.x, 0.00000011920929f, (_e498.x < 0f));
                    }
                    let _e555 = phi_5886_;
                    phi_1167_ = _e555;
                    break;
                }
                default: {
                    phi_1167_ = f32();
                    break;
                }
            }
            let _e597 = phi_1167_;
            switch bitcast<i32>(_e546.member.member_1) {
                case 1: {
                    let _e635 = abs(_e498.y);
                    let _e637 = (_e635 % 1f);
                    if (_e635 >= 1f) {
                        phi_5964_ = select(true, false, (_e637 == 0f));
                    } else {
                        phi_5964_ = true;
                    }
                    let _e641 = phi_5964_;
                    let _e642 = select(1f, _e637, _e641);
                    if (select(-1f, 1f, (_e498.y >= 0f)) > 0f) {
                        phi_1188_ = _e642;
                    } else {
                        phi_1188_ = (1f - _e642);
                    }
                    let _e646 = phi_1188_;
                    phi_1225_ = _e646;
                    break;
                }
                case 2: {
                    let _e609 = abs(_e498.y);
                    let _e616 = ((select(select(u32(_e609), 0u, (_e609 < 0f)), 4294967295u, (_e609 > 4294967000f)) % 2u) == 0u);
                    let _e618 = (_e609 % 1f);
                    if (_e609 >= 1f) {
                        phi_5947_ = select(true, false, (_e618 == 0f));
                    } else {
                        phi_5947_ = true;
                    }
                    let _e622 = phi_5947_;
                    let _e623 = select(1f, _e618, _e622);
                    if (select(-1f, 1f, (_e498.y >= 0f)) > 0f) {
                        if _e616 {
                            phi_1217_ = _e623;
                        } else {
                            phi_1217_ = (1f - _e623);
                        }
                        let _e630 = phi_1217_;
                        phi_1223_ = _e630;
                    } else {
                        if _e616 {
                            phi_1222_ = (1f - _e623);
                        } else {
                            phi_1222_ = _e623;
                        }
                        let _e627 = phi_1222_;
                        phi_1223_ = _e627;
                    }
                    let _e632 = phi_1223_;
                    phi_1225_ = _e632;
                    break;
                }
                case 0: {
                    if (_e498.y > 1f) {
                        phi_5934_ = 0.9999999f;
                    } else {
                        phi_5934_ = select(_e498.y, 0.00000011920929f, (_e498.y < 0f));
                    }
                    let _e606 = phi_5934_;
                    phi_1225_ = _e606;
                    break;
                }
                default: {
                    phi_1225_ = f32();
                    break;
                }
            }
            let _e648 = phi_1225_;
            let _e652 = (_e597 * f32(_e546.member_2.x));
            let _e661 = (_e648 * f32(_e546.member_2.y));
            let _e679 = vec3<f32>((f32((select(select(u32(_e652), 0u, (_e652 < 0f)), 4294967295u, (_e652 > 4294967000f)) + _e546.member_1.x)) / _e477), (f32((select(select(u32(_e661), 0u, (_e661 < 0f)), 4294967295u, (_e661 > 4294967000f)) + _e546.member_1.y)) / _e478), f32(_e546.member_3));
            let _e685 = textureSampleLevel(global_11, global_10, vec2<f32>(_e679.x, _e679.y), i32(_e679.z), 0f);
            let _e688 = select(_e685, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e297.member_6 == 4294967295u)));
            let _e692 = select(_e130, _e129, vec2((_e297.member_12 == 0u)));
            if _e303 {
                phi_6000_ = (_e297.member_7 <= (_e124 - 8u));
            } else {
                phi_6000_ = false;
            }
            let _e697 = phi_6000_;
            if _e697 {
                let _e700 = global.member[_e297.member_7];
                let _e704 = global.member[(_e297.member_7 + 1u)];
                let _e709 = global.member[(_e297.member_7 + 2u)];
                let _e713 = global.member[(_e297.member_7 + 3u)];
                let _e718 = global.member[(_e297.member_7 + 4u)];
                let _e722 = global.member[(_e297.member_7 + 5u)];
                let _e726 = global.member[(_e297.member_7 + 6u)];
                switch bitcast<i32>(_e726) {
                    case 0: {
                        phi_1308_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1308_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1308_ = 2u;
                        break;
                    }
                    default: {
                        phi_1308_ = 0u;
                        break;
                    }
                }
                let _e729 = phi_1308_;
                let _e733 = global.member[(_e297.member_7 + 7u)];
                switch bitcast<i32>(_e733) {
                    case 0: {
                        phi_1317_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1317_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1317_ = 2u;
                        break;
                    }
                    default: {
                        phi_1317_ = 0u;
                        break;
                    }
                }
                let _e736 = phi_1317_;
                phi_1330_ = type_15(type_14(_e729, _e736), vec2<u32>(_e700, _e704), vec2<u32>(_e709, _e713), _e718, _e722);
            } else {
                phi_1330_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e740 = phi_1330_;
            switch bitcast<i32>(_e740.member.member) {
                case 1: {
                    let _e778 = abs(_e692.x);
                    let _e780 = (_e778 % 1f);
                    if (_e778 >= 1f) {
                        phi_6051_ = select(true, false, (_e780 == 0f));
                    } else {
                        phi_6051_ = true;
                    }
                    let _e784 = phi_6051_;
                    let _e785 = select(1f, _e780, _e784);
                    if (select(-1f, 1f, (_e692.x >= 0f)) > 0f) {
                        phi_1350_ = _e785;
                    } else {
                        phi_1350_ = (1f - _e785);
                    }
                    let _e789 = phi_1350_;
                    phi_1387_ = _e789;
                    break;
                }
                case 2: {
                    let _e752 = abs(_e692.x);
                    let _e759 = ((select(select(u32(_e752), 0u, (_e752 < 0f)), 4294967295u, (_e752 > 4294967000f)) % 2u) == 0u);
                    let _e761 = (_e752 % 1f);
                    if (_e752 >= 1f) {
                        phi_6034_ = select(true, false, (_e761 == 0f));
                    } else {
                        phi_6034_ = true;
                    }
                    let _e765 = phi_6034_;
                    let _e766 = select(1f, _e761, _e765);
                    if (select(-1f, 1f, (_e692.x >= 0f)) > 0f) {
                        if _e759 {
                            phi_1379_ = _e766;
                        } else {
                            phi_1379_ = (1f - _e766);
                        }
                        let _e773 = phi_1379_;
                        phi_1385_ = _e773;
                    } else {
                        if _e759 {
                            phi_1384_ = (1f - _e766);
                        } else {
                            phi_1384_ = _e766;
                        }
                        let _e770 = phi_1384_;
                        phi_1385_ = _e770;
                    }
                    let _e775 = phi_1385_;
                    phi_1387_ = _e775;
                    break;
                }
                case 0: {
                    if (_e692.x > 1f) {
                        phi_6021_ = 0.9999999f;
                    } else {
                        phi_6021_ = select(_e692.x, 0.00000011920929f, (_e692.x < 0f));
                    }
                    let _e749 = phi_6021_;
                    phi_1387_ = _e749;
                    break;
                }
                default: {
                    phi_1387_ = f32();
                    break;
                }
            }
            let _e791 = phi_1387_;
            switch bitcast<i32>(_e740.member.member_1) {
                case 1: {
                    let _e829 = abs(_e692.y);
                    let _e831 = (_e829 % 1f);
                    if (_e829 >= 1f) {
                        phi_6099_ = select(true, false, (_e831 == 0f));
                    } else {
                        phi_6099_ = true;
                    }
                    let _e835 = phi_6099_;
                    let _e836 = select(1f, _e831, _e835);
                    if (select(-1f, 1f, (_e692.y >= 0f)) > 0f) {
                        phi_1408_ = _e836;
                    } else {
                        phi_1408_ = (1f - _e836);
                    }
                    let _e840 = phi_1408_;
                    phi_1445_ = _e840;
                    break;
                }
                case 2: {
                    let _e803 = abs(_e692.y);
                    let _e810 = ((select(select(u32(_e803), 0u, (_e803 < 0f)), 4294967295u, (_e803 > 4294967000f)) % 2u) == 0u);
                    let _e812 = (_e803 % 1f);
                    if (_e803 >= 1f) {
                        phi_6082_ = select(true, false, (_e812 == 0f));
                    } else {
                        phi_6082_ = true;
                    }
                    let _e816 = phi_6082_;
                    let _e817 = select(1f, _e812, _e816);
                    if (select(-1f, 1f, (_e692.y >= 0f)) > 0f) {
                        if _e810 {
                            phi_1437_ = _e817;
                        } else {
                            phi_1437_ = (1f - _e817);
                        }
                        let _e824 = phi_1437_;
                        phi_1443_ = _e824;
                    } else {
                        if _e810 {
                            phi_1442_ = (1f - _e817);
                        } else {
                            phi_1442_ = _e817;
                        }
                        let _e821 = phi_1442_;
                        phi_1443_ = _e821;
                    }
                    let _e826 = phi_1443_;
                    phi_1445_ = _e826;
                    break;
                }
                case 0: {
                    if (_e692.y > 1f) {
                        phi_6069_ = 0.9999999f;
                    } else {
                        phi_6069_ = select(_e692.y, 0.00000011920929f, (_e692.y < 0f));
                    }
                    let _e800 = phi_6069_;
                    phi_1445_ = _e800;
                    break;
                }
                default: {
                    phi_1445_ = f32();
                    break;
                }
            }
            let _e842 = phi_1445_;
            let _e846 = (_e791 * f32(_e740.member_2.x));
            let _e855 = (_e842 * f32(_e740.member_2.y));
            let _e873 = vec3<f32>((f32((select(select(u32(_e846), 0u, (_e846 < 0f)), 4294967295u, (_e846 > 4294967000f)) + _e740.member_1.x)) / _e477), (f32((select(select(u32(_e855), 0u, (_e855 < 0f)), 4294967295u, (_e855 > 4294967000f)) + _e740.member_1.y)) / _e478), f32(_e740.member_3));
            let _e879 = textureSampleLevel(global_11, global_10, vec2<f32>(_e873.x, _e873.y), i32(_e873.z), 0f);
            let _e880 = (_e297.member_7 == 4294967295u);
            let _e882 = select(_e879, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e880));
            let _e886 = select(_e130, _e129, vec2((_e297.member_13 == 0u)));
            if _e303 {
                phi_6135_ = (_e297.member_8 <= (_e124 - 8u));
            } else {
                phi_6135_ = false;
            }
            let _e891 = phi_6135_;
            if _e891 {
                let _e894 = global.member[_e297.member_8];
                let _e898 = global.member[(_e297.member_8 + 1u)];
                let _e903 = global.member[(_e297.member_8 + 2u)];
                let _e907 = global.member[(_e297.member_8 + 3u)];
                let _e912 = global.member[(_e297.member_8 + 4u)];
                let _e916 = global.member[(_e297.member_8 + 5u)];
                let _e920 = global.member[(_e297.member_8 + 6u)];
                switch bitcast<i32>(_e920) {
                    case 0: {
                        phi_1528_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1528_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1528_ = 2u;
                        break;
                    }
                    default: {
                        phi_1528_ = 0u;
                        break;
                    }
                }
                let _e923 = phi_1528_;
                let _e927 = global.member[(_e297.member_8 + 7u)];
                switch bitcast<i32>(_e927) {
                    case 0: {
                        phi_1537_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1537_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1537_ = 2u;
                        break;
                    }
                    default: {
                        phi_1537_ = 0u;
                        break;
                    }
                }
                let _e930 = phi_1537_;
                phi_1550_ = type_15(type_14(_e923, _e930), vec2<u32>(_e894, _e898), vec2<u32>(_e903, _e907), _e912, _e916);
            } else {
                phi_1550_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e934 = phi_1550_;
            switch bitcast<i32>(_e934.member.member) {
                case 1: {
                    let _e972 = abs(_e886.x);
                    let _e974 = (_e972 % 1f);
                    if (_e972 >= 1f) {
                        phi_6186_ = select(true, false, (_e974 == 0f));
                    } else {
                        phi_6186_ = true;
                    }
                    let _e978 = phi_6186_;
                    let _e979 = select(1f, _e974, _e978);
                    if (select(-1f, 1f, (_e886.x >= 0f)) > 0f) {
                        phi_1570_ = _e979;
                    } else {
                        phi_1570_ = (1f - _e979);
                    }
                    let _e983 = phi_1570_;
                    phi_1607_ = _e983;
                    break;
                }
                case 2: {
                    let _e946 = abs(_e886.x);
                    let _e953 = ((select(select(u32(_e946), 0u, (_e946 < 0f)), 4294967295u, (_e946 > 4294967000f)) % 2u) == 0u);
                    let _e955 = (_e946 % 1f);
                    if (_e946 >= 1f) {
                        phi_6169_ = select(true, false, (_e955 == 0f));
                    } else {
                        phi_6169_ = true;
                    }
                    let _e959 = phi_6169_;
                    let _e960 = select(1f, _e955, _e959);
                    if (select(-1f, 1f, (_e886.x >= 0f)) > 0f) {
                        if _e953 {
                            phi_1599_ = _e960;
                        } else {
                            phi_1599_ = (1f - _e960);
                        }
                        let _e967 = phi_1599_;
                        phi_1605_ = _e967;
                    } else {
                        if _e953 {
                            phi_1604_ = (1f - _e960);
                        } else {
                            phi_1604_ = _e960;
                        }
                        let _e964 = phi_1604_;
                        phi_1605_ = _e964;
                    }
                    let _e969 = phi_1605_;
                    phi_1607_ = _e969;
                    break;
                }
                case 0: {
                    if (_e886.x > 1f) {
                        phi_6156_ = 0.9999999f;
                    } else {
                        phi_6156_ = select(_e886.x, 0.00000011920929f, (_e886.x < 0f));
                    }
                    let _e943 = phi_6156_;
                    phi_1607_ = _e943;
                    break;
                }
                default: {
                    phi_1607_ = f32();
                    break;
                }
            }
            let _e985 = phi_1607_;
            switch bitcast<i32>(_e934.member.member_1) {
                case 1: {
                    let _e1023 = abs(_e886.y);
                    let _e1025 = (_e1023 % 1f);
                    if (_e1023 >= 1f) {
                        phi_6234_ = select(true, false, (_e1025 == 0f));
                    } else {
                        phi_6234_ = true;
                    }
                    let _e1029 = phi_6234_;
                    let _e1030 = select(1f, _e1025, _e1029);
                    if (select(-1f, 1f, (_e886.y >= 0f)) > 0f) {
                        phi_1628_ = _e1030;
                    } else {
                        phi_1628_ = (1f - _e1030);
                    }
                    let _e1034 = phi_1628_;
                    phi_1665_ = _e1034;
                    break;
                }
                case 2: {
                    let _e997 = abs(_e886.y);
                    let _e1004 = ((select(select(u32(_e997), 0u, (_e997 < 0f)), 4294967295u, (_e997 > 4294967000f)) % 2u) == 0u);
                    let _e1006 = (_e997 % 1f);
                    if (_e997 >= 1f) {
                        phi_6217_ = select(true, false, (_e1006 == 0f));
                    } else {
                        phi_6217_ = true;
                    }
                    let _e1010 = phi_6217_;
                    let _e1011 = select(1f, _e1006, _e1010);
                    if (select(-1f, 1f, (_e886.y >= 0f)) > 0f) {
                        if _e1004 {
                            phi_1657_ = _e1011;
                        } else {
                            phi_1657_ = (1f - _e1011);
                        }
                        let _e1018 = phi_1657_;
                        phi_1663_ = _e1018;
                    } else {
                        if _e1004 {
                            phi_1662_ = (1f - _e1011);
                        } else {
                            phi_1662_ = _e1011;
                        }
                        let _e1015 = phi_1662_;
                        phi_1663_ = _e1015;
                    }
                    let _e1020 = phi_1663_;
                    phi_1665_ = _e1020;
                    break;
                }
                case 0: {
                    if (_e886.y > 1f) {
                        phi_6204_ = 0.9999999f;
                    } else {
                        phi_6204_ = select(_e886.y, 0.00000011920929f, (_e886.y < 0f));
                    }
                    let _e994 = phi_6204_;
                    phi_1665_ = _e994;
                    break;
                }
                default: {
                    phi_1665_ = f32();
                    break;
                }
            }
            let _e1036 = phi_1665_;
            let _e1040 = (_e985 * f32(_e934.member_2.x));
            let _e1049 = (_e1036 * f32(_e934.member_2.y));
            let _e1067 = vec3<f32>((f32((select(select(u32(_e1040), 0u, (_e1040 < 0f)), 4294967295u, (_e1040 > 4294967000f)) + _e934.member_1.x)) / _e477), (f32((select(select(u32(_e1049), 0u, (_e1049 < 0f)), 4294967295u, (_e1049 > 4294967000f)) + _e934.member_1.y)) / _e478), f32(_e934.member_3));
            let _e1073 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1067.x, _e1067.y), i32(_e1067.z), 0f);
            let _e1080 = select(_e130, _e129, vec2((_e297.member_14 == 0u)));
            if _e303 {
                phi_6270_ = (_e297.member_9 <= (_e124 - 8u));
            } else {
                phi_6270_ = false;
            }
            let _e1085 = phi_6270_;
            if _e1085 {
                let _e1088 = global.member[_e297.member_9];
                let _e1092 = global.member[(_e297.member_9 + 1u)];
                let _e1097 = global.member[(_e297.member_9 + 2u)];
                let _e1101 = global.member[(_e297.member_9 + 3u)];
                let _e1106 = global.member[(_e297.member_9 + 4u)];
                let _e1110 = global.member[(_e297.member_9 + 5u)];
                let _e1114 = global.member[(_e297.member_9 + 6u)];
                switch bitcast<i32>(_e1114) {
                    case 0: {
                        phi_1748_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1748_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1748_ = 2u;
                        break;
                    }
                    default: {
                        phi_1748_ = 0u;
                        break;
                    }
                }
                let _e1117 = phi_1748_;
                let _e1121 = global.member[(_e297.member_9 + 7u)];
                switch bitcast<i32>(_e1121) {
                    case 0: {
                        phi_1757_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1757_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1757_ = 2u;
                        break;
                    }
                    default: {
                        phi_1757_ = 0u;
                        break;
                    }
                }
                let _e1124 = phi_1757_;
                phi_1770_ = type_15(type_14(_e1117, _e1124), vec2<u32>(_e1088, _e1092), vec2<u32>(_e1097, _e1101), _e1106, _e1110);
            } else {
                phi_1770_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e1128 = phi_1770_;
            switch bitcast<i32>(_e1128.member.member) {
                case 1: {
                    let _e1166 = abs(_e1080.x);
                    let _e1168 = (_e1166 % 1f);
                    if (_e1166 >= 1f) {
                        phi_6321_ = select(true, false, (_e1168 == 0f));
                    } else {
                        phi_6321_ = true;
                    }
                    let _e1172 = phi_6321_;
                    let _e1173 = select(1f, _e1168, _e1172);
                    if (select(-1f, 1f, (_e1080.x >= 0f)) > 0f) {
                        phi_1790_ = _e1173;
                    } else {
                        phi_1790_ = (1f - _e1173);
                    }
                    let _e1177 = phi_1790_;
                    phi_1827_ = _e1177;
                    break;
                }
                case 2: {
                    let _e1140 = abs(_e1080.x);
                    let _e1147 = ((select(select(u32(_e1140), 0u, (_e1140 < 0f)), 4294967295u, (_e1140 > 4294967000f)) % 2u) == 0u);
                    let _e1149 = (_e1140 % 1f);
                    if (_e1140 >= 1f) {
                        phi_6304_ = select(true, false, (_e1149 == 0f));
                    } else {
                        phi_6304_ = true;
                    }
                    let _e1153 = phi_6304_;
                    let _e1154 = select(1f, _e1149, _e1153);
                    if (select(-1f, 1f, (_e1080.x >= 0f)) > 0f) {
                        if _e1147 {
                            phi_1819_ = _e1154;
                        } else {
                            phi_1819_ = (1f - _e1154);
                        }
                        let _e1161 = phi_1819_;
                        phi_1825_ = _e1161;
                    } else {
                        if _e1147 {
                            phi_1824_ = (1f - _e1154);
                        } else {
                            phi_1824_ = _e1154;
                        }
                        let _e1158 = phi_1824_;
                        phi_1825_ = _e1158;
                    }
                    let _e1163 = phi_1825_;
                    phi_1827_ = _e1163;
                    break;
                }
                case 0: {
                    if (_e1080.x > 1f) {
                        phi_6291_ = 0.9999999f;
                    } else {
                        phi_6291_ = select(_e1080.x, 0.00000011920929f, (_e1080.x < 0f));
                    }
                    let _e1137 = phi_6291_;
                    phi_1827_ = _e1137;
                    break;
                }
                default: {
                    phi_1827_ = f32();
                    break;
                }
            }
            let _e1179 = phi_1827_;
            switch bitcast<i32>(_e1128.member.member_1) {
                case 1: {
                    let _e1217 = abs(_e1080.y);
                    let _e1219 = (_e1217 % 1f);
                    if (_e1217 >= 1f) {
                        phi_6369_ = select(true, false, (_e1219 == 0f));
                    } else {
                        phi_6369_ = true;
                    }
                    let _e1223 = phi_6369_;
                    let _e1224 = select(1f, _e1219, _e1223);
                    if (select(-1f, 1f, (_e1080.y >= 0f)) > 0f) {
                        phi_1848_ = _e1224;
                    } else {
                        phi_1848_ = (1f - _e1224);
                    }
                    let _e1228 = phi_1848_;
                    phi_1885_ = _e1228;
                    break;
                }
                case 2: {
                    let _e1191 = abs(_e1080.y);
                    let _e1198 = ((select(select(u32(_e1191), 0u, (_e1191 < 0f)), 4294967295u, (_e1191 > 4294967000f)) % 2u) == 0u);
                    let _e1200 = (_e1191 % 1f);
                    if (_e1191 >= 1f) {
                        phi_6352_ = select(true, false, (_e1200 == 0f));
                    } else {
                        phi_6352_ = true;
                    }
                    let _e1204 = phi_6352_;
                    let _e1205 = select(1f, _e1200, _e1204);
                    if (select(-1f, 1f, (_e1080.y >= 0f)) > 0f) {
                        if _e1198 {
                            phi_1877_ = _e1205;
                        } else {
                            phi_1877_ = (1f - _e1205);
                        }
                        let _e1212 = phi_1877_;
                        phi_1883_ = _e1212;
                    } else {
                        if _e1198 {
                            phi_1882_ = (1f - _e1205);
                        } else {
                            phi_1882_ = _e1205;
                        }
                        let _e1209 = phi_1882_;
                        phi_1883_ = _e1209;
                    }
                    let _e1214 = phi_1883_;
                    phi_1885_ = _e1214;
                    break;
                }
                case 0: {
                    if (_e1080.y > 1f) {
                        phi_6339_ = 0.9999999f;
                    } else {
                        phi_6339_ = select(_e1080.y, 0.00000011920929f, (_e1080.y < 0f));
                    }
                    let _e1188 = phi_6339_;
                    phi_1885_ = _e1188;
                    break;
                }
                default: {
                    phi_1885_ = f32();
                    break;
                }
            }
            let _e1230 = phi_1885_;
            let _e1234 = (_e1179 * f32(_e1128.member_2.x));
            let _e1243 = (_e1230 * f32(_e1128.member_2.y));
            let _e1261 = vec3<f32>((f32((select(select(u32(_e1234), 0u, (_e1234 < 0f)), 4294967295u, (_e1234 > 4294967000f)) + _e1128.member_1.x)) / _e477), (f32((select(select(u32(_e1243), 0u, (_e1243 < 0f)), 4294967295u, (_e1243 > 4294967000f)) + _e1128.member_1.y)) / _e478), f32(_e1128.member_3));
            let _e1267 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1261.x, _e1261.y), i32(_e1261.z), 0f);
            let _e1270 = select(_e1267, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e297.member_9 == 4294967295u)));
            if _e880 {
                phi_1979_ = vec3<f32>(0f, 0f, 0f);
                phi_1980_ = _e131;
            } else {
                let _e1274 = fma(_e882.x, 2f, -1f);
                let _e1275 = fma(_e882.y, 2f, -1f);
                let _e1276 = fma(_e882.z, 2f, -1f);
                let _e1281 = sqrt(fma(_e1276, _e1276, fma(_e1274, _e1274, (_e1275 * _e1275))));
                if (_e1281 == 0f) {
                    phi_6427_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6427_ = (vec3<f32>(_e1274, _e1275, _e1276) * (1f / _e1281));
                }
                let _e1286 = phi_6427_;
                let _e1293 = sqrt(fma(_e132.z, _e132.z, fma(_e132.x, _e132.x, (_e132.y * _e132.y))));
                if (_e1293 == 0f) {
                    phi_6462_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6462_ = (_e132 * (1f / _e1293));
                }
                let _e1298 = phi_6462_;
                let _e1305 = sqrt(fma(_e133.z, _e133.z, fma(_e133.x, _e133.x, (_e133.y * _e133.y))));
                if (_e1305 == 0f) {
                    phi_6497_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6497_ = (_e133 * (1f / _e1305));
                }
                let _e1310 = phi_6497_;
                let _e1317 = sqrt(fma(_e131.z, _e131.z, fma(_e131.x, _e131.x, (_e131.y * _e131.y))));
                if (_e1317 == 0f) {
                    phi_6532_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6532_ = (_e131 * (1f / _e1317));
                }
                let _e1322 = phi_6532_;
                let _e1341 = fma(_e1322.x, _e1286.z, fma(_e1298.x, _e1286.x, (_e1310.x * _e1286.y)));
                let _e1342 = fma(_e1322.y, _e1286.z, fma(_e1298.y, _e1286.x, (_e1310.y * _e1286.y)));
                let _e1343 = fma(_e1322.z, _e1286.z, fma(_e1298.z, _e1286.x, (_e1310.z * _e1286.y)));
                let _e1348 = sqrt(fma(_e1343, _e1343, fma(_e1341, _e1341, (_e1342 * _e1342))));
                if (_e1348 == 0f) {
                    phi_6567_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6567_ = (vec3<f32>(_e1341, _e1342, _e1343) * (1f / _e1348));
                }
                let _e1353 = phi_6567_;
                phi_1979_ = _e1286;
                phi_1980_ = _e1353;
            }
            let _e1355 = phi_1979_;
            let _e1357 = phi_1980_;
            let _e1361 = (_e494.x * _e297.member_2.x);
            let _e1364 = (_e494.y * _e297.member_2.y);
            let _e1367 = (_e494.z * _e297.member_2.z);
            let _e1372 = (_e1361 * _e128.x);
            let _e1374 = (_e1364 * _e128.y);
            let _e1376 = (_e1367 * _e128.z);
            let _e1381 = (_e688.y * _e297.member_4);
            let _e1384 = (_e688.z * _e297.member_3);
            let _e1388 = fma(_e297.member_16, (select(_e1073, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e297.member_8 == 4294967295u))).x - 1f), 1f);
            let _e1394 = (_e1270.x * _e297.member.x);
            let _e1396 = (_e1270.y * _e297.member.y);
            let _e1398 = (_e1270.z * _e297.member.z);
            let _e1403 = textureSampleLevel(global_12, global_13, _e1357, 0f);
            if (_e124 >= 86u) {
                phi_6599_ = (_e138 <= (_e124 - 86u));
            } else {
                phi_6599_ = false;
            }
            let _e1411 = phi_6599_;
            if _e1411 {
                let _e1414 = global.member[_e138];
                let _e1419 = global.member[(_e138 + 1u)];
                let _e1424 = global.member[(_e138 + 2u)];
                let _e1429 = global.member[(_e138 + 3u)];
                let _e1435 = global.member[(_e138 + 4u)];
                let _e1440 = global.member[(_e138 + 5u)];
                let _e1445 = global.member[(_e138 + 6u)];
                let _e1450 = global.member[(_e138 + 7u)];
                let _e1456 = global.member[(_e138 + 8u)];
                let _e1461 = global.member[(_e138 + 9u)];
                let _e1466 = global.member[(_e138 + 10u)];
                let _e1471 = global.member[(_e138 + 11u)];
                let _e1477 = global.member[(_e138 + 12u)];
                let _e1482 = global.member[(_e138 + 13u)];
                let _e1487 = global.member[(_e138 + 14u)];
                let _e1492 = global.member[(_e138 + 15u)];
                let _e1499 = global.member[(_e138 + 16u)];
                let _e1504 = global.member[(_e138 + 17u)];
                let _e1509 = global.member[(_e138 + 18u)];
                let _e1514 = global.member[(_e138 + 19u)];
                let _e1520 = global.member[(_e138 + 20u)];
                let _e1525 = global.member[(_e138 + 21u)];
                let _e1530 = global.member[(_e138 + 22u)];
                let _e1535 = global.member[(_e138 + 23u)];
                let _e1541 = global.member[(_e138 + 24u)];
                let _e1546 = global.member[(_e138 + 25u)];
                let _e1551 = global.member[(_e138 + 26u)];
                let _e1556 = global.member[(_e138 + 27u)];
                let _e1562 = global.member[(_e138 + 28u)];
                let _e1567 = global.member[(_e138 + 29u)];
                let _e1572 = global.member[(_e138 + 30u)];
                let _e1577 = global.member[(_e138 + 31u)];
                let _e1584 = global.member[(_e138 + 32u)];
                let _e1589 = global.member[(_e138 + 33u)];
                let _e1594 = global.member[(_e138 + 34u)];
                local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_2187_ = type_14(0u, 6u);
                loop {
                    let _e1599 = phi_2187_;
                    if (_e1599.member < _e1599.member_1) {
                        phi_2188_ = type_14((_e1599.member + 1u), _e1599.member_1);
                        phi_2211_ = type_14(1u, _e1599.member);
                    } else {
                        phi_2188_ = _e1599;
                        phi_2211_ = type_14(0u, type_14().member_1);
                    }
                    let _e1612 = phi_2188_;
                    let _e1614 = phi_2211_;
                    switch bitcast<i32>(_e1614.member) {
                        case 0: {
                            phi_2238_ = false;
                            break;
                        }
                        case 1: {
                            let _e1619 = ((_e138 + 35u) + (_e1614.member_1 * 4u));
                            let _e1622 = global.member[_e1619];
                            let _e1627 = global.member[(_e1619 + 1u)];
                            let _e1632 = global.member[(_e1619 + 2u)];
                            let _e1637 = global.member[(_e1619 + 3u)];
                            local_1[_e1614.member_1] = vec4<f32>(bitcast<f32>(_e1622), bitcast<f32>(_e1627), bitcast<f32>(_e1632), bitcast<f32>(_e1637));
                            phi_2238_ = true;
                            break;
                        }
                        default: {
                            phi_2238_ = bool();
                            break;
                        }
                    }
                    let _e1642 = phi_2238_;
                    continue;
                    continuing {
                        phi_2187_ = _e1612;
                        break if !(_e1642);
                    }
                }
                let _e1644 = local_1;
                local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                phi_2244_ = type_14(0u, 8u);
                loop {
                    let _e1647 = phi_2244_;
                    if (_e1647.member < _e1647.member_1) {
                        phi_2245_ = type_14((_e1647.member + 1u), _e1647.member_1);
                        phi_2268_ = type_14(1u, _e1647.member);
                    } else {
                        phi_2245_ = _e1647;
                        phi_2268_ = type_14(0u, type_14().member_1);
                    }
                    let _e1660 = phi_2245_;
                    let _e1662 = phi_2268_;
                    switch bitcast<i32>(_e1662.member) {
                        case 0: {
                            phi_2291_ = false;
                            break;
                        }
                        case 1: {
                            let _e1667 = ((_e138 + 59u) + (_e1662.member_1 * 3u));
                            let _e1670 = global.member[_e1667];
                            let _e1675 = global.member[(_e1667 + 1u)];
                            let _e1680 = global.member[(_e1667 + 2u)];
                            local[_e1662.member_1] = vec3<f32>(bitcast<f32>(_e1670), bitcast<f32>(_e1675), bitcast<f32>(_e1680));
                            phi_2291_ = true;
                            break;
                        }
                        default: {
                            phi_2291_ = bool();
                            break;
                        }
                    }
                    let _e1685 = phi_2291_;
                    continue;
                    continuing {
                        phi_2244_ = _e1660;
                        break if !(_e1685);
                    }
                }
                let _e1687 = local;
                let _e1691 = global.member[(_e138 + 83u)];
                let _e1696 = global.member[(_e138 + 84u)];
                let _e1701 = global.member[(_e138 + 85u)];
                phi_2312_ = type_25(type_23(vec4<f32>(bitcast<f32>(_e1414), bitcast<f32>(_e1419), bitcast<f32>(_e1424), bitcast<f32>(_e1429)), vec4<f32>(bitcast<f32>(_e1435), bitcast<f32>(_e1440), bitcast<f32>(_e1445), bitcast<f32>(_e1450)), vec4<f32>(bitcast<f32>(_e1456), bitcast<f32>(_e1461), bitcast<f32>(_e1466), bitcast<f32>(_e1471)), vec4<f32>(bitcast<f32>(_e1477), bitcast<f32>(_e1482), bitcast<f32>(_e1487), bitcast<f32>(_e1492))), type_23(vec4<f32>(bitcast<f32>(_e1499), bitcast<f32>(_e1504), bitcast<f32>(_e1509), bitcast<f32>(_e1514)), vec4<f32>(bitcast<f32>(_e1520), bitcast<f32>(_e1525), bitcast<f32>(_e1530), bitcast<f32>(_e1535)), vec4<f32>(bitcast<f32>(_e1541), bitcast<f32>(_e1546), bitcast<f32>(_e1551), bitcast<f32>(_e1556)), vec4<f32>(bitcast<f32>(_e1562), bitcast<f32>(_e1567), bitcast<f32>(_e1572), bitcast<f32>(_e1577))), vec3<f32>(bitcast<f32>(_e1584), bitcast<f32>(_e1589), bitcast<f32>(_e1594)), type_24(_e1687, _e1644, vec3<f32>(bitcast<f32>(_e1691), bitcast<f32>(_e1696), bitcast<f32>(_e1701))));
            } else {
                phi_2312_ = type_25(type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_24(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
            }
            let _e1707 = phi_2312_;
            let _e1709 = (_e1707.member_2 - _e134);
            let _e1716 = sqrt(fma(_e1709.z, _e1709.z, fma(_e1709.x, _e1709.x, (_e1709.y * _e1709.y))));
            let _e1717 = (_e1716 == 0f);
            if _e1717 {
                phi_6671_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6671_ = (_e1709 * (1f / _e1716));
            }
            let _e1721 = phi_6671_;
            let _e1722 = -(_e1721);
            let _e1729 = sqrt(fma(_e1357.z, _e1357.z, fma(_e1357.x, _e1357.x, (_e1357.y * _e1357.y))));
            let _e1730 = (_e1729 == 0f);
            if _e1730 {
                phi_6730_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6730_ = (_e1357 * (1f / _e1729));
            }
            let _e1734 = phi_6730_;
            let _e1744 = (2f * fma(_e1734.z, _e1722.z, fma(_e1734.x, _e1722.x, (_e1734.y * _e1722.y))));
            let _e1751 = textureSampleLevel(global_14, global_15, (_e1722 - vec3<f32>((_e1744 * _e1734.x), (_e1744 * _e1734.y), (_e1744 * _e1734.z))), (_e1381 * 4f));
            if _e1717 {
                phi_6804_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6804_ = (_e1709 * (1f / _e1716));
            }
            let _e1758 = phi_6804_;
            let _e1767 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1357.z, _e1758.z, fma(_e1357.x, _e1758.x, (_e1357.y * _e1758.y))), 0f), _e1381), 0f);
            switch bitcast<i32>(_e160) {
                case 0: {
                    if _e297.member_15 {
                        if _e1730 {
                            phi_7197_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_7197_ = (_e1357 * (1f / _e1729));
                        }
                        let _e1936 = phi_7197_;
                        if _e1717 {
                            phi_7232_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_7232_ = (_e1709 * (1f / _e1716));
                        }
                        let _e1940 = phi_7232_;
                        let _e1943 = global_1.member[0u];
                        let _e1946 = global_1.member[1u];
                        let _e1949 = global_1.member[2u];
                        phi_8761_ = false;
                        phi_2379_ = type_14(0u, _e1946);
                        phi_2382_ = vec3<f32>(0f, 0f, 0f);
                        loop {
                            let _e1952 = phi_8761_;
                            let _e1954 = phi_2379_;
                            let _e1956 = phi_2382_;
                            local_6 = _e1956;
                            local_7 = _e1956;
                            local_8 = _e1956;
                            if (_e1954.member < _e1954.member_1) {
                                phi_2380_ = type_14((_e1954.member + 1u), _e1954.member_1);
                                phi_2405_ = type_14(1u, _e1954.member);
                            } else {
                                phi_2380_ = _e1954;
                                phi_2405_ = type_14(0u, type_14().member_1);
                            }
                            let _e1969 = phi_2380_;
                            let _e1971 = phi_2405_;
                            switch bitcast<i32>(_e1971.member) {
                                case 0: {
                                    phi_8880_ = _e1952;
                                    phi_2383_ = vec3<f32>();
                                    phi_4894_ = false;
                                    break;
                                }
                                case 1: {
                                    if (_e1971.member_1 >= _e1946) {
                                        phi_7258_ = 4294967295u;
                                    } else {
                                        phi_7258_ = (_e1943 + _e1971.member_1);
                                    }
                                    let _e1978 = phi_7258_;
                                    if (_e126 >= 1u) {
                                        phi_7277_ = (_e1978 <= (_e126 - 1u));
                                    } else {
                                        phi_7277_ = false;
                                    }
                                    let _e1983 = phi_7277_;
                                    if _e1983 {
                                        let _e1986 = global_1.member[_e1978];
                                        phi_2422_ = _e1986;
                                    } else {
                                        phi_2422_ = 4294967295u;
                                    }
                                    let _e1988 = phi_2422_;
                                    if (_e126 >= 4u) {
                                        phi_7301_ = (_e1988 <= (_e126 - 4u));
                                    } else {
                                        phi_7301_ = false;
                                    }
                                    let _e1993 = phi_7301_;
                                    if _e1993 {
                                        let _e1996 = global_1.member[_e1988];
                                        switch bitcast<i32>(_e1996) {
                                            case 0: {
                                                phi_2434_ = 0u;
                                                break;
                                            }
                                            case 1: {
                                                phi_2434_ = 1u;
                                                break;
                                            }
                                            case 2: {
                                                phi_2434_ = 2u;
                                                break;
                                            }
                                            default: {
                                                phi_2434_ = 0u;
                                                break;
                                            }
                                        }
                                        let _e1999 = phi_2434_;
                                        let _e2003 = global_1.member[(_e1988 + 1u)];
                                        let _e2007 = global_1.member[(_e1988 + 2u)];
                                        let _e2011 = global_1.member[(_e1988 + 3u)];
                                        phi_2448_ = type_30(_e1999, _e2003, _e2007, _e2011);
                                    } else {
                                        phi_2448_ = type_30(0u, 4294967295u, 4294967295u, 4294967295u);
                                    }
                                    let _e2014 = phi_2448_;
                                    if (_e126 >= 10u) {
                                        phi_7333_ = (_e2014.member_2 <= (_e126 - 10u));
                                    } else {
                                        phi_7333_ = false;
                                    }
                                    let _e2020 = phi_7333_;
                                    if _e2020 {
                                        let _e2023 = global_1.member[_e2014.member_2];
                                        let _e2028 = global_1.member[(_e2014.member_2 + 1u)];
                                        let _e2033 = global_1.member[(_e2014.member_2 + 2u)];
                                        let _e2039 = global_1.member[(_e2014.member_2 + 3u)];
                                        let _e2044 = global_1.member[(_e2014.member_2 + 4u)];
                                        let _e2049 = global_1.member[(_e2014.member_2 + 5u)];
                                        let _e2054 = global_1.member[(_e2014.member_2 + 6u)];
                                        let _e2060 = global_1.member[(_e2014.member_2 + 7u)];
                                        let _e2065 = global_1.member[(_e2014.member_2 + 8u)];
                                        let _e2070 = global_1.member[(_e2014.member_2 + 9u)];
                                        phi_2498_ = type_31(vec3<f32>(bitcast<f32>(_e2023), bitcast<f32>(_e2028), bitcast<f32>(_e2033)), vec4<f32>(bitcast<f32>(_e2039), bitcast<f32>(_e2044), bitcast<f32>(_e2049), bitcast<f32>(_e2054)), vec3<f32>(bitcast<f32>(_e2060), bitcast<f32>(_e2065), bitcast<f32>(_e2070)));
                                    } else {
                                        phi_2498_ = type_31(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e2075 = phi_2498_;
                                    let _e2083 = (_e2075.member_1.x + _e2075.member_1.x);
                                    let _e2084 = (_e2075.member_1.y + _e2075.member_1.y);
                                    let _e2085 = (_e2075.member_1.z + _e2075.member_1.z);
                                    let _e2087 = (_e2075.member_1.z * _e2085);
                                    let _e2088 = (_e2075.member_1.w * _e2083);
                                    let _e2089 = (_e2075.member_1.w * _e2084);
                                    let _e2090 = (_e2075.member_1.w * _e2085);
                                    let _e2110 = (vec4<f32>((1f - fma(_e2075.member_1.y, _e2084, _e2087)), fma(_e2075.member_1.x, _e2084, _e2090), fma(_e2075.member_1.x, _e2085, -(_e2089)), 0f) * _e2075.member_2.x);
                                    let _e2112 = (vec4<f32>(fma(_e2075.member_1.x, _e2084, -(_e2090)), (1f - fma(_e2075.member_1.x, _e2083, _e2087)), fma(_e2075.member_1.y, _e2085, _e2088), 0f) * _e2075.member_2.y);
                                    let _e2114 = (vec4<f32>(fma(_e2075.member_1.x, _e2085, _e2089), fma(_e2075.member_1.y, _e2085, -(_e2088)), (1f - fma(_e2075.member_1.x, _e2083, (_e2075.member_1.y * _e2084))), 0f) * _e2075.member_2.z);
                                    switch bitcast<i32>(_e2014.member) {
                                        case 0: {
                                            if (_e126 >= 8u) {
                                                phi_8222_ = (_e2014.member_1 <= (_e126 - 8u));
                                            } else {
                                                phi_8222_ = false;
                                            }
                                            let _e3526 = phi_8222_;
                                            if _e3526 {
                                                let _e3529 = global_1.member[_e2014.member_1];
                                                let _e3534 = global_1.member[(_e2014.member_1 + 1u)];
                                                let _e3539 = global_1.member[(_e2014.member_1 + 2u)];
                                                let _e3545 = global_1.member[(_e2014.member_1 + 3u)];
                                                let _e3550 = global_1.member[(_e2014.member_1 + 4u)];
                                                let _e3555 = global_1.member[(_e2014.member_1 + 5u)];
                                                let _e3560 = global_1.member[(_e2014.member_1 + 6u)];
                                                let _e3566 = global_1.member[(_e2014.member_1 + 7u)];
                                                phi_2546_ = type_34(vec3<f32>(bitcast<f32>(_e3529), bitcast<f32>(_e3534), bitcast<f32>(_e3539)), vec4<f32>(bitcast<f32>(_e3545), bitcast<f32>(_e3550), bitcast<f32>(_e3555), bitcast<f32>(_e3560)), bitcast<f32>(_e3566));
                                            } else {
                                                phi_2546_ = type_34(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e3570 = phi_2546_;
                                            let _e3592 = fma(_e2114.x, _e3570.member.z, fma(_e2112.x, _e3570.member.y, (_e2110.x * _e3570.member.x)));
                                            let _e3593 = fma(_e2114.y, _e3570.member.z, fma(_e2112.y, _e3570.member.y, (_e2110.y * _e3570.member.x)));
                                            let _e3594 = fma(_e2114.z, _e3570.member.z, fma(_e2112.z, _e3570.member.y, (_e2110.z * _e3570.member.x)));
                                            let _e3599 = sqrt(fma(_e3594, _e3594, fma(_e3592, _e3592, (_e3593 * _e3593))));
                                            if (_e3599 == 0f) {
                                                phi_8269_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_8269_ = (vec3<f32>(_e3592, _e3593, _e3594) * (1f / _e3599));
                                            }
                                            let _e3604 = phi_8269_;
                                            let _e3606 = -(_e3604.x);
                                            let _e3608 = -(_e3604.y);
                                            let _e3610 = -(_e3604.z);
                                            let _e3611 = -(_e3604);
                                            let _e3613 = fma(-(_e688.z), _e297.member_3, 1f);
                                            let _e3617 = fma(0.4f, _e3613, (_e1372 * _e1384));
                                            let _e3618 = fma(0.4f, _e3613, (_e1374 * _e1384));
                                            let _e3619 = fma(0.4f, _e3613, (_e1376 * _e1384));
                                            let _e3627 = (_e1940 + vec3<f32>(_e3606, _e3608, _e3610));
                                            let _e3634 = sqrt(fma(_e3627.z, _e3627.z, fma(_e3627.x, _e3627.x, (_e3627.y * _e3627.y))));
                                            if (_e3634 == 0f) {
                                                phi_8304_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_8304_ = (_e3627 * (1f / _e3634));
                                            }
                                            let _e3639 = phi_8304_;
                                            let _e3640 = (_e1381 * _e1381);
                                            let _e3651 = max(fma(_e1936.z, _e3639.z, fma(_e1936.x, _e3639.x, (_e1936.y * _e3639.y))), 0f);
                                            let _e3664 = max(fma(_e1936.z, _e1940.z, fma(_e1936.x, _e1940.x, (_e1936.y * _e1940.y))), 0f);
                                            let _e3670 = fma(_e1936.z, _e3611.z, fma(_e1936.x, _e3611.x, (_e1936.y * _e3611.y)));
                                            let _e3671 = max(_e3670, 0f);
                                            let _e3672 = fma(_e688.y, _e297.member_4, 1f);
                                            let _e3673 = (_e3672 * _e3672);
                                            let _e3674 = (_e3673 * 0.125f);
                                            let _e3676 = fma(-(_e3673), 0.125f, 1f);
                                            let _e3689 = (1f - max(fma(_e3639.z, _e1940.z, fma(_e3639.x, _e1940.x, (_e3639.y * _e1940.y))), 0f));
                                            let _e3691 = select(_e3689, 0f, (_e3689 < 0f));
                                            let _e3694 = pow(select(_e3691, 1f, (_e3691 > 1f)), 5f);
                                            let _e3695 = fma((1f - _e3617), _e3694, _e3617);
                                            let _e3696 = fma((1f - _e3618), _e3694, _e3618);
                                            let _e3697 = fma((1f - _e3619), _e3694, _e3619);
                                            let _e3704 = (((_e3640 * _e3640) / (pow(fma((_e3651 * _e3651), fma(_e3640, _e3640, -1f), 1f), 2f) * 3.1415927f)) * ((_e3664 / fma(_e3664, _e3676, _e3674)) * (_e3671 / fma(_e3671, _e3676, _e3674))));
                                            let _e3711 = max(fma(_e1936.z, _e3610, fma(_e1936.x, _e3606, (_e1936.y * _e3608))), 0f);
                                            let _e3713 = fma((4f * _e3664), _e3711, 0.0001f);
                                            if ((_e2014.member_3 == 4294967295u) != true) {
                                                let _e3735 = global_1.member[_e2014.member_3];
                                                let _e3739 = global_1.member[(_e2014.member_3 + 1u)];
                                                let _e3743 = global_1.member[(_e2014.member_3 + 2u)];
                                                let _e3747 = global_1.member[(_e2014.member_3 + 3u)];
                                                let _e3751 = global_1.member[(_e2014.member_3 + 4u)];
                                                let _e3756 = global_1.member[(_e2014.member_3 + 5u)];
                                                let _e3761 = global_1.member[(_e2014.member_3 + 6u)];
                                                let _e3764 = global_1.member[_e1949];
                                                let _e3768 = global_1.member[(_e1949 + 1u)];
                                                let _e3770 = select(_e3735, 4294967295u, (0u >= _e3739));
                                                let _e3773 = global_1.member[_e3770];
                                                let _e3778 = global_1.member[(_e3770 + 1u)];
                                                let _e3783 = global_1.member[(_e3770 + 2u)];
                                                let _e3788 = global_1.member[(_e3770 + 3u)];
                                                let _e3793 = global_1.member[(_e3770 + 4u)];
                                                let _e3798 = global_1.member[(_e3770 + 5u)];
                                                let _e3803 = global_1.member[(_e3770 + 6u)];
                                                let _e3808 = global_1.member[(_e3770 + 7u)];
                                                let _e3813 = global_1.member[(_e3770 + 8u)];
                                                let _e3818 = global_1.member[(_e3770 + 9u)];
                                                let _e3823 = global_1.member[(_e3770 + 10u)];
                                                let _e3828 = global_1.member[(_e3770 + 11u)];
                                                let _e3833 = global_1.member[(_e3770 + 12u)];
                                                let _e3838 = global_1.member[(_e3770 + 13u)];
                                                let _e3843 = global_1.member[(_e3770 + 14u)];
                                                let _e3848 = global_1.member[(_e3770 + 15u)];
                                                let _e3868 = (bitcast<f32>(_e3848) + fma(bitcast<f32>(_e3828), _e134.z, fma(bitcast<f32>(_e3808), _e134.y, (bitcast<f32>(_e3788) * _e134.x))));
                                                let _e3869 = ((bitcast<f32>(_e3833) + fma(bitcast<f32>(_e3813), _e134.z, fma(bitcast<f32>(_e3793), _e134.y, (bitcast<f32>(_e3773) * _e134.x)))) / _e3868);
                                                let _e3870 = ((bitcast<f32>(_e3838) + fma(bitcast<f32>(_e3818), _e134.z, fma(bitcast<f32>(_e3798), _e134.y, (bitcast<f32>(_e3778) * _e134.x)))) / _e3868);
                                                let _e3871 = ((bitcast<f32>(_e3843) + fma(bitcast<f32>(_e3823), _e134.z, fma(bitcast<f32>(_e3803), _e134.y, (bitcast<f32>(_e3783) * _e134.x)))) / _e3868);
                                                if (abs(_e3869) <= 1f) {
                                                    let _e3875 = (abs(_e3870) <= 1f);
                                                    if _e3875 {
                                                        phi_8409_ = (abs(_e3871) <= 1f);
                                                    } else {
                                                        phi_8409_ = bool();
                                                    }
                                                    let _e3879 = phi_8409_;
                                                    phi_8412_ = _e3879;
                                                    phi_8413_ = select(true, false, _e3875);
                                                } else {
                                                    phi_8412_ = bool();
                                                    phi_8413_ = true;
                                                }
                                                let _e3882 = phi_8412_;
                                                let _e3884 = phi_8413_;
                                                if select(_e3882, false, _e3884) {
                                                    let _e3892 = global_1.member[select(_e3743, 4294967295u, (0u >= _e3747))];
                                                    let _e3895 = global_1.member[_e3892];
                                                    let _e3899 = global_1.member[(_e3892 + 1u)];
                                                    let _e3903 = global_1.member[(_e3892 + 2u)];
                                                    let _e3907 = global_1.member[(_e3892 + 3u)];
                                                    let _e3911 = global_1.member[(_e3892 + 4u)];
                                                    let _e3915 = global_1.member[(_e3892 + 6u)];
                                                    switch bitcast<i32>(_e3915) {
                                                        case 0: {
                                                            phi_2937_ = 0u;
                                                            break;
                                                        }
                                                        case 1: {
                                                            phi_2937_ = 1u;
                                                            break;
                                                        }
                                                        case 2: {
                                                            phi_2937_ = 2u;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_2937_ = 0u;
                                                            break;
                                                        }
                                                    }
                                                    let _e3918 = phi_2937_;
                                                    let _e3922 = global_1.member[(_e3892 + 7u)];
                                                    switch bitcast<i32>(_e3922) {
                                                        case 0: {
                                                            phi_2946_ = 0u;
                                                            break;
                                                        }
                                                        case 1: {
                                                            phi_2946_ = 1u;
                                                            break;
                                                        }
                                                        case 2: {
                                                            phi_2946_ = 2u;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_2946_ = 0u;
                                                            break;
                                                        }
                                                    }
                                                    let _e3925 = phi_2946_;
                                                    let _e3926 = bitcast<i32>(_e3761);
                                                    let _e3928 = f32(_e3903);
                                                    let _e3929 = f32(_e3907);
                                                    let _e3933 = type_35((_e3926 / -2i), (_e3926 / 2i), false);
                                                    phi_8832_ = _e1952;
                                                    phi_2974_ = _e3933;
                                                    phi_2977_ = 0f;
                                                    phi_2979_ = 0f;
                                                    loop {
                                                        let _e3935 = phi_8832_;
                                                        let _e3937 = phi_2974_;
                                                        let _e3939 = phi_2977_;
                                                        let _e3941 = phi_2979_;
                                                        local_4 = _e3939;
                                                        local_5 = _e3941;
                                                        if _e3937.member_2 {
                                                            phi_2991_ = true;
                                                        } else {
                                                            phi_2991_ = ((_e3937.member <= _e3937.member_1) != true);
                                                        }
                                                        let _e3948 = phi_2991_;
                                                        if _e3948 {
                                                            phi_2975_ = _e3937;
                                                            phi_3034_ = type_36(0u, type_36().member_1);
                                                        } else {
                                                            if (_e3937.member < _e3937.member_1) {
                                                                let _e3956 = (_e3937.member + 1i);
                                                                if select(false, true, ((false == (_e3956 > _e3937.member)) != false)) {
                                                                    phi_3019_ = type_36(0u, type_36().member_1);
                                                                } else {
                                                                    phi_3019_ = type_36(1u, _e3956);
                                                                }
                                                                let _e3966 = phi_3019_;
                                                                switch bitcast<i32>(_e3966.member) {
                                                                    case 0: {
                                                                        phi_8877_ = true;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        break;
                                                                    }
                                                                }
                                                                phi_3031_ = type_35(_e3966.member_1, _e3937.member_1, _e3937.member_2);
                                                            } else {
                                                                phi_3031_ = type_35(_e3937.member, _e3937.member_1, true);
                                                            }
                                                            let _e3975 = phi_3031_;
                                                            phi_2975_ = _e3975;
                                                            phi_3034_ = type_36(1u, _e3937.member);
                                                        }
                                                        let _e3981 = phi_2975_;
                                                        let _e3983 = phi_3034_;
                                                        switch bitcast<i32>(_e3983.member) {
                                                            case 0: {
                                                                phi_8878_ = _e3935;
                                                                phi_2978_ = f32();
                                                                phi_2980_ = f32();
                                                                phi_3292_ = false;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3045_ = _e3933;
                                                                phi_3048_ = _e3939;
                                                                phi_3050_ = _e3941;
                                                                loop {
                                                                    let _e3988 = phi_3045_;
                                                                    let _e3990 = phi_3048_;
                                                                    let _e3992 = phi_3050_;
                                                                    local_11 = _e3990;
                                                                    local_12 = _e3992;
                                                                    if _e3988.member_2 {
                                                                        phi_3062_ = true;
                                                                    } else {
                                                                        phi_3062_ = ((_e3988.member <= _e3988.member_1) != true);
                                                                    }
                                                                    let _e3999 = phi_3062_;
                                                                    if _e3999 {
                                                                        phi_3046_ = _e3988;
                                                                        phi_3105_ = type_36(0u, type_36().member_1);
                                                                    } else {
                                                                        if (_e3988.member < _e3988.member_1) {
                                                                            let _e4007 = (_e3988.member + 1i);
                                                                            if select(false, true, ((false == (_e4007 > _e3988.member)) != false)) {
                                                                                phi_3090_ = type_36(0u, type_36().member_1);
                                                                            } else {
                                                                                phi_3090_ = type_36(1u, _e4007);
                                                                            }
                                                                            let _e4017 = phi_3090_;
                                                                            switch bitcast<i32>(_e4017.member) {
                                                                                case 0: {
                                                                                    phi_8816_ = true;
                                                                                    break;
                                                                                }
                                                                                case 1: {
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            phi_3102_ = type_35(_e4017.member_1, _e3988.member_1, _e3988.member_2);
                                                                        } else {
                                                                            phi_3102_ = type_35(_e3988.member, _e3988.member_1, true);
                                                                        }
                                                                        let _e4026 = phi_3102_;
                                                                        phi_3046_ = _e4026;
                                                                        phi_3105_ = type_36(1u, _e3988.member);
                                                                    }
                                                                    let _e4032 = phi_3046_;
                                                                    let _e4034 = phi_3105_;
                                                                    switch bitcast<i32>(_e4034.member) {
                                                                        case 0: {
                                                                            phi_3049_ = f32();
                                                                            phi_3051_ = f32();
                                                                            phi_3291_ = false;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            let _e4042 = fma((_e3869 + 1f), 0.5f, (f32(_e3983.member_1) * (1f / _e3928)));
                                                                            let _e4043 = fma(fma(_e3870, -1f, 1f), 0.5f, (f32(_e4034.member_1) * (1f / _e3929)));
                                                                            switch bitcast<i32>(_e3918) {
                                                                                case 1: {
                                                                                    let _e4078 = abs(_e4042);
                                                                                    let _e4080 = (_e4078 % 1f);
                                                                                    if (_e4078 >= 1f) {
                                                                                        phi_8465_ = select(true, false, (_e4080 == 0f));
                                                                                    } else {
                                                                                        phi_8465_ = true;
                                                                                    }
                                                                                    let _e4084 = phi_8465_;
                                                                                    let _e4085 = select(1f, _e4080, _e4084);
                                                                                    if (select(-1f, 1f, (_e4042 >= 0f)) > 0f) {
                                                                                        phi_3137_ = _e4085;
                                                                                    } else {
                                                                                        phi_3137_ = (1f - _e4085);
                                                                                    }
                                                                                    let _e4089 = phi_3137_;
                                                                                    phi_3174_ = _e4089;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e4052 = abs(_e4042);
                                                                                    let _e4059 = ((select(select(u32(_e4052), 0u, (_e4052 < 0f)), 4294967295u, (_e4052 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e4061 = (_e4052 % 1f);
                                                                                    if (_e4052 >= 1f) {
                                                                                        phi_8448_ = select(true, false, (_e4061 == 0f));
                                                                                    } else {
                                                                                        phi_8448_ = true;
                                                                                    }
                                                                                    let _e4065 = phi_8448_;
                                                                                    let _e4066 = select(1f, _e4061, _e4065);
                                                                                    if (select(-1f, 1f, (_e4042 >= 0f)) > 0f) {
                                                                                        if _e4059 {
                                                                                            phi_3166_ = _e4066;
                                                                                        } else {
                                                                                            phi_3166_ = (1f - _e4066);
                                                                                        }
                                                                                        let _e4073 = phi_3166_;
                                                                                        phi_3172_ = _e4073;
                                                                                    } else {
                                                                                        if _e4059 {
                                                                                            phi_3171_ = (1f - _e4066);
                                                                                        } else {
                                                                                            phi_3171_ = _e4066;
                                                                                        }
                                                                                        let _e4070 = phi_3171_;
                                                                                        phi_3172_ = _e4070;
                                                                                    }
                                                                                    let _e4075 = phi_3172_;
                                                                                    phi_3174_ = _e4075;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e4042 > 1f) {
                                                                                        phi_8435_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_8435_ = select(_e4042, 0.00000011920929f, (_e4042 < 0f));
                                                                                    }
                                                                                    let _e4049 = phi_8435_;
                                                                                    phi_3174_ = _e4049;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3174_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e4091 = phi_3174_;
                                                                            switch bitcast<i32>(_e3925) {
                                                                                case 1: {
                                                                                    let _e4126 = abs(_e4043);
                                                                                    let _e4128 = (_e4126 % 1f);
                                                                                    if (_e4126 >= 1f) {
                                                                                        phi_8513_ = select(true, false, (_e4128 == 0f));
                                                                                    } else {
                                                                                        phi_8513_ = true;
                                                                                    }
                                                                                    let _e4132 = phi_8513_;
                                                                                    let _e4133 = select(1f, _e4128, _e4132);
                                                                                    if (select(-1f, 1f, (_e4043 >= 0f)) > 0f) {
                                                                                        phi_3193_ = _e4133;
                                                                                    } else {
                                                                                        phi_3193_ = (1f - _e4133);
                                                                                    }
                                                                                    let _e4137 = phi_3193_;
                                                                                    phi_3230_ = _e4137;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e4100 = abs(_e4043);
                                                                                    let _e4107 = ((select(select(u32(_e4100), 0u, (_e4100 < 0f)), 4294967295u, (_e4100 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e4109 = (_e4100 % 1f);
                                                                                    if (_e4100 >= 1f) {
                                                                                        phi_8496_ = select(true, false, (_e4109 == 0f));
                                                                                    } else {
                                                                                        phi_8496_ = true;
                                                                                    }
                                                                                    let _e4113 = phi_8496_;
                                                                                    let _e4114 = select(1f, _e4109, _e4113);
                                                                                    if (select(-1f, 1f, (_e4043 >= 0f)) > 0f) {
                                                                                        if _e4107 {
                                                                                            phi_3222_ = _e4114;
                                                                                        } else {
                                                                                            phi_3222_ = (1f - _e4114);
                                                                                        }
                                                                                        let _e4121 = phi_3222_;
                                                                                        phi_3228_ = _e4121;
                                                                                    } else {
                                                                                        if _e4107 {
                                                                                            phi_3227_ = (1f - _e4114);
                                                                                        } else {
                                                                                            phi_3227_ = _e4114;
                                                                                        }
                                                                                        let _e4118 = phi_3227_;
                                                                                        phi_3228_ = _e4118;
                                                                                    }
                                                                                    let _e4123 = phi_3228_;
                                                                                    phi_3230_ = _e4123;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e4043 > 1f) {
                                                                                        phi_8483_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_8483_ = select(_e4043, 0.00000011920929f, (_e4043 < 0f));
                                                                                    }
                                                                                    let _e4097 = phi_8483_;
                                                                                    phi_3230_ = _e4097;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3230_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e4139 = phi_3230_;
                                                                            let _e4140 = (_e4091 * _e3928);
                                                                            let _e4146 = (_e4139 * _e3929);
                                                                            let _e4161 = vec3<f32>((f32((select(select(u32(_e4140), 0u, (_e4140 < 0f)), 4294967295u, (_e4140 > 4294967000f)) + _e3895)) / f32(_e3764)), (f32((select(select(u32(_e4146), 0u, (_e4146 < 0f)), 4294967295u, (_e4146 > 4294967000f)) + _e3899)) / f32(_e3768)), f32(_e3911));
                                                                            let _e4167 = textureSampleLevel(global_19, global_18, vec2<f32>(_e4161.x, _e4161.y), i32(_e4161.z), 0f);
                                                                            phi_3049_ = (_e3990 + 1f);
                                                                            phi_3051_ = (_e3992 + select(0f, 1f, ((_e3871 - max((bitcast<f32>(_e3756) * (1f - _e3670)), bitcast<f32>(_e3751))) > _e4167.x)));
                                                                            phi_3291_ = true;
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            phi_3049_ = f32();
                                                                            phi_3051_ = f32();
                                                                            phi_3291_ = bool();
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e4178 = phi_3049_;
                                                                    let _e4180 = phi_3051_;
                                                                    let _e4182 = phi_3291_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_3045_ = _e4032;
                                                                        phi_3048_ = _e4178;
                                                                        phi_3050_ = _e4180;
                                                                        phi_8816_ = _e3935;
                                                                        break if !(_e4182);
                                                                    }
                                                                }
                                                                let _e4185 = phi_8816_;
                                                                phi_8877_ = _e4185;
                                                                if _e4185 {
                                                                    break;
                                                                }
                                                                phi_8878_ = _e4185;
                                                                let _e4628 = local_11;
                                                                phi_2978_ = _e4628;
                                                                let _e4631 = local_12;
                                                                phi_2980_ = _e4631;
                                                                phi_3292_ = true;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_8878_ = _e3935;
                                                                phi_2978_ = f32();
                                                                phi_2980_ = f32();
                                                                phi_3292_ = bool();
                                                                break;
                                                            }
                                                        }
                                                        let _e4187 = phi_8878_;
                                                        let _e4189 = phi_2978_;
                                                        let _e4191 = phi_2980_;
                                                        let _e4193 = phi_3292_;
                                                        continue;
                                                        continuing {
                                                            phi_8832_ = _e4187;
                                                            phi_2974_ = _e3981;
                                                            phi_2977_ = _e4189;
                                                            phi_2979_ = _e4191;
                                                            phi_8877_ = _e4187;
                                                            break if !(_e4193);
                                                        }
                                                    }
                                                    let _e4196 = phi_8877_;
                                                    phi_8879_ = _e4196;
                                                    if _e4196 {
                                                        break;
                                                    }
                                                    let _e4198 = local_4;
                                                    let _e4201 = local_5;
                                                    phi_8945_ = _e4196;
                                                    phi_3295_ = (_e4201 / max(_e4198, 1f));
                                                } else {
                                                    phi_8945_ = _e1952;
                                                    phi_3295_ = 0f;
                                                }
                                                let _e4204 = phi_8945_;
                                                let _e4206 = phi_3295_;
                                                phi_8944_ = _e4204;
                                                phi_3296_ = _e4206;
                                            } else {
                                                phi_8944_ = _e1952;
                                                phi_3296_ = 0f;
                                            }
                                            let _e4208 = phi_8944_;
                                            let _e4210 = phi_3296_;
                                            phi_8883_ = _e4208;
                                            phi_4866_ = _e4210;
                                            phi_4867_ = vec3<f32>(((fma((((1f - _e3695) * _e3613) * _e1372), 0.31830987f, ((_e3704 * _e3695) / _e3713)) * (_e3570.member_1.x * _e3570.member_2)) * _e3711), ((fma((((1f - _e3696) * _e3613) * _e1374), 0.31830987f, ((_e3704 * _e3696) / _e3713)) * (_e3570.member_1.y * _e3570.member_2)) * _e3711), ((fma((((1f - _e3697) * _e3613) * _e1376), 0.31830987f, ((_e3704 * _e3697) / _e3713)) * (_e3570.member_1.z * _e3570.member_2)) * _e3711));
                                            phi_4868_ = true;
                                            break;
                                        }
                                        case 1: {
                                            if (_e126 >= 8u) {
                                                phi_7779_ = (_e2014.member_1 <= (_e126 - 8u));
                                            } else {
                                                phi_7779_ = false;
                                            }
                                            let _e2899 = phi_7779_;
                                            if _e2899 {
                                                let _e2902 = global_1.member[_e2014.member_1];
                                                let _e2907 = global_1.member[(_e2014.member_1 + 1u)];
                                                let _e2912 = global_1.member[(_e2014.member_1 + 2u)];
                                                let _e2918 = global_1.member[(_e2014.member_1 + 3u)];
                                                let _e2923 = global_1.member[(_e2014.member_1 + 4u)];
                                                let _e2928 = global_1.member[(_e2014.member_1 + 5u)];
                                                let _e2933 = global_1.member[(_e2014.member_1 + 6u)];
                                                let _e2939 = global_1.member[(_e2014.member_1 + 7u)];
                                                phi_3337_ = type_34(vec3<f32>(bitcast<f32>(_e2902), bitcast<f32>(_e2907), bitcast<f32>(_e2912)), vec4<f32>(bitcast<f32>(_e2918), bitcast<f32>(_e2923), bitcast<f32>(_e2928), bitcast<f32>(_e2933)), bitcast<f32>(_e2939));
                                            } else {
                                                phi_3337_ = type_34(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2943 = phi_3337_;
                                            let _e2971 = vec3<f32>((_e2075.member.x + fma(_e2114.x, _e2943.member.z, fma(_e2112.x, _e2943.member.y, (_e2110.x * _e2943.member.x)))), (_e2075.member.y + fma(_e2114.y, _e2943.member.z, fma(_e2112.y, _e2943.member.y, (_e2110.y * _e2943.member.x)))), (_e2075.member.z + fma(_e2114.z, _e2943.member.z, fma(_e2112.z, _e2943.member.y, (_e2110.z * _e2943.member.x)))));
                                            let _e2972 = (_e2971 - _e134);
                                            let _e2979 = sqrt(fma(_e2972.z, _e2972.z, fma(_e2972.x, _e2972.x, (_e2972.y * _e2972.y))));
                                            let _e2980 = (_e2979 == 0f);
                                            if _e2980 {
                                                phi_3941_ = f32();
                                                phi_3942_ = vec3<f32>();
                                            } else {
                                                if _e2980 {
                                                    phi_7826_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7826_ = (_e2972 * (1f / _e2979));
                                                }
                                                let _e2984 = phi_7826_;
                                                let _e2986 = (_e2943.member_2 / (_e2979 * _e2979));
                                                let _e2988 = fma(-(_e688.z), _e297.member_3, 1f);
                                                let _e2992 = fma(0.4f, _e2988, (_e1372 * _e1384));
                                                let _e2993 = fma(0.4f, _e2988, (_e1374 * _e1384));
                                                let _e2994 = fma(0.4f, _e2988, (_e1376 * _e1384));
                                                let _e3001 = (_e1940 + _e2984);
                                                let _e3008 = sqrt(fma(_e3001.z, _e3001.z, fma(_e3001.x, _e3001.x, (_e3001.y * _e3001.y))));
                                                if (_e3008 == 0f) {
                                                    phi_7861_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7861_ = (_e3001 * (1f / _e3008));
                                                }
                                                let _e3013 = phi_7861_;
                                                let _e3014 = (_e1381 * _e1381);
                                                let _e3025 = max(fma(_e1936.z, _e3013.z, fma(_e1936.x, _e3013.x, (_e1936.y * _e3013.y))), 0f);
                                                let _e3038 = max(fma(_e1936.z, _e1940.z, fma(_e1936.x, _e1940.x, (_e1936.y * _e1940.y))), 0f);
                                                let _e3044 = fma(_e1936.z, _e2984.z, fma(_e1936.x, _e2984.x, (_e1936.y * _e2984.y)));
                                                let _e3045 = max(_e3044, 0f);
                                                let _e3046 = fma(_e688.y, _e297.member_4, 1f);
                                                let _e3047 = (_e3046 * _e3046);
                                                let _e3048 = (_e3047 * 0.125f);
                                                let _e3050 = fma(-(_e3047), 0.125f, 1f);
                                                let _e3063 = (1f - max(fma(_e3013.z, _e1940.z, fma(_e3013.x, _e1940.x, (_e3013.y * _e1940.y))), 0f));
                                                let _e3065 = select(_e3063, 0f, (_e3063 < 0f));
                                                let _e3068 = pow(select(_e3065, 1f, (_e3065 > 1f)), 5f);
                                                let _e3069 = fma((1f - _e2992), _e3068, _e2992);
                                                let _e3070 = fma((1f - _e2993), _e3068, _e2993);
                                                let _e3071 = fma((1f - _e2994), _e3068, _e2994);
                                                let _e3078 = (((_e3014 * _e3014) / (pow(fma((_e3025 * _e3025), fma(_e3014, _e3014, -1f), 1f), 2f) * 3.1415927f)) * ((_e3038 / fma(_e3038, _e3050, _e3048)) * (_e3045 / fma(_e3045, _e3050, _e3048))));
                                                let _e3083 = fma((4f * _e3038), _e3045, 0.0001f);
                                                if ((_e2014.member_3 == 4294967295u) != true) {
                                                    let _e3105 = global_1.member[_e2014.member_3];
                                                    let _e3109 = global_1.member[(_e2014.member_3 + 1u)];
                                                    let _e3113 = global_1.member[(_e2014.member_3 + 2u)];
                                                    let _e3117 = global_1.member[(_e2014.member_3 + 3u)];
                                                    let _e3121 = global_1.member[(_e2014.member_3 + 4u)];
                                                    let _e3126 = global_1.member[(_e2014.member_3 + 5u)];
                                                    let _e3130 = global_1.member[_e1949];
                                                    let _e3134 = global_1.member[(_e1949 + 1u)];
                                                    let _e3135 = (_e134 - _e2971);
                                                    let _e3137 = abs(_e3135.x);
                                                    let _e3139 = abs(_e3135.y);
                                                    let _e3141 = abs(_e3135.z);
                                                    if (_e3137 >= _e3139) {
                                                        let _e3143 = (_e3137 >= _e3141);
                                                        if _e3143 {
                                                            let _e3144 = (_e3135.x > 0f);
                                                            if _e3144 {
                                                                phi_7981_ = vec2<f32>((-(_e3135.z) / _e3137), (-(_e3135.y) / _e3137));
                                                            } else {
                                                                phi_7981_ = vec2<f32>((_e3135.z / _e3137), (-(_e3135.y) / _e3137));
                                                            }
                                                            let _e3155 = phi_7981_;
                                                            phi_7984_ = _e3155;
                                                            phi_7985_ = select(1u, 0u, _e3144);
                                                        } else {
                                                            phi_7984_ = vec2<f32>();
                                                            phi_7985_ = u32();
                                                        }
                                                        let _e3158 = phi_7984_;
                                                        let _e3160 = phi_7985_;
                                                        phi_7988_ = _e3158;
                                                        phi_7989_ = _e3160;
                                                        phi_7990_ = select(true, false, _e3143);
                                                    } else {
                                                        phi_7988_ = vec2<f32>();
                                                        phi_7989_ = u32();
                                                        phi_7990_ = true;
                                                    }
                                                    let _e3163 = phi_7988_;
                                                    let _e3165 = phi_7989_;
                                                    let _e3167 = phi_7990_;
                                                    if _e3167 {
                                                        if (_e3139 >= _e3137) {
                                                            let _e3169 = (_e3139 >= _e3141);
                                                            if _e3169 {
                                                                let _e3170 = (_e3135.y > 0f);
                                                                if _e3170 {
                                                                    phi_8015_ = vec2<f32>((_e3135.x / _e3139), (_e3135.z / _e3139));
                                                                } else {
                                                                    phi_8015_ = vec2<f32>((_e3135.x / _e3139), (-(_e3135.z) / _e3139));
                                                                }
                                                                let _e3179 = phi_8015_;
                                                                phi_8018_ = _e3179;
                                                                phi_8019_ = select(3u, 2u, _e3170);
                                                            } else {
                                                                phi_8018_ = vec2<f32>();
                                                                phi_8019_ = u32();
                                                            }
                                                            let _e3182 = phi_8018_;
                                                            let _e3184 = phi_8019_;
                                                            phi_8022_ = _e3182;
                                                            phi_8023_ = _e3184;
                                                            phi_8024_ = select(true, false, _e3169);
                                                        } else {
                                                            phi_8022_ = vec2<f32>();
                                                            phi_8023_ = u32();
                                                            phi_8024_ = true;
                                                        }
                                                        let _e3187 = phi_8022_;
                                                        let _e3189 = phi_8023_;
                                                        let _e3191 = phi_8024_;
                                                        if _e3191 {
                                                            let _e3192 = (_e3135.z > 0f);
                                                            if _e3192 {
                                                                phi_8045_ = vec2<f32>((_e3135.x / _e3141), (-(_e3135.y) / _e3141));
                                                            } else {
                                                                phi_8045_ = vec2<f32>((-(_e3135.x) / _e3141), (-(_e3135.y) / _e3141));
                                                            }
                                                            let _e3203 = phi_8045_;
                                                            phi_8048_ = _e3203;
                                                            phi_8049_ = select(5u, 4u, _e3192);
                                                        } else {
                                                            phi_8048_ = _e3187;
                                                            phi_8049_ = _e3189;
                                                        }
                                                        let _e3206 = phi_8048_;
                                                        let _e3208 = phi_8049_;
                                                        phi_8051_ = _e3206;
                                                        phi_8052_ = _e3208;
                                                    } else {
                                                        phi_8051_ = _e3163;
                                                        phi_8052_ = _e3165;
                                                    }
                                                    let _e3210 = phi_8051_;
                                                    let _e3212 = phi_8052_;
                                                    let _e3217 = ((_e3210.x + 1f) * 0.5f);
                                                    let _e3218 = ((_e3210.y + 1f) * 0.5f);
                                                    if (_e3212 >= _e3109) {
                                                        phi_3591_ = 4294967295u;
                                                    } else {
                                                        phi_3591_ = (_e3105 + (16u * _e3212));
                                                    }
                                                    let _e3223 = phi_3591_;
                                                    let _e3226 = global_1.member[_e3223];
                                                    let _e3231 = global_1.member[(_e3223 + 1u)];
                                                    let _e3236 = global_1.member[(_e3223 + 2u)];
                                                    let _e3241 = global_1.member[(_e3223 + 3u)];
                                                    let _e3246 = global_1.member[(_e3223 + 4u)];
                                                    let _e3251 = global_1.member[(_e3223 + 5u)];
                                                    let _e3256 = global_1.member[(_e3223 + 6u)];
                                                    let _e3261 = global_1.member[(_e3223 + 7u)];
                                                    let _e3266 = global_1.member[(_e3223 + 8u)];
                                                    let _e3271 = global_1.member[(_e3223 + 9u)];
                                                    let _e3276 = global_1.member[(_e3223 + 10u)];
                                                    let _e3281 = global_1.member[(_e3223 + 11u)];
                                                    let _e3286 = global_1.member[(_e3223 + 12u)];
                                                    let _e3291 = global_1.member[(_e3223 + 13u)];
                                                    let _e3296 = global_1.member[(_e3223 + 14u)];
                                                    let _e3301 = global_1.member[(_e3223 + 15u)];
                                                    let _e3321 = (bitcast<f32>(_e3301) + fma(bitcast<f32>(_e3281), _e134.z, fma(bitcast<f32>(_e3261), _e134.y, (bitcast<f32>(_e3241) * _e134.x))));
                                                    let _e3324 = ((bitcast<f32>(_e3296) + fma(bitcast<f32>(_e3276), _e134.z, fma(bitcast<f32>(_e3256), _e134.y, (bitcast<f32>(_e3236) * _e134.x)))) / _e3321);
                                                    if (abs(((bitcast<f32>(_e3286) + fma(bitcast<f32>(_e3266), _e134.z, fma(bitcast<f32>(_e3246), _e134.y, (bitcast<f32>(_e3226) * _e134.x)))) / _e3321)) <= 1f) {
                                                        let _e3328 = (abs(((bitcast<f32>(_e3291) + fma(bitcast<f32>(_e3271), _e134.z, fma(bitcast<f32>(_e3251), _e134.y, (bitcast<f32>(_e3231) * _e134.x)))) / _e3321)) <= 1f);
                                                        if _e3328 {
                                                            phi_8081_ = (abs(_e3324) <= 1f);
                                                        } else {
                                                            phi_8081_ = bool();
                                                        }
                                                        let _e3332 = phi_8081_;
                                                        phi_8084_ = _e3332;
                                                        phi_8085_ = select(true, false, _e3328);
                                                    } else {
                                                        phi_8084_ = bool();
                                                        phi_8085_ = true;
                                                    }
                                                    let _e3335 = phi_8084_;
                                                    let _e3337 = phi_8085_;
                                                    if select(_e3335, false, _e3337) {
                                                        if (_e3212 >= _e3117) {
                                                            phi_3719_ = 4294967295u;
                                                        } else {
                                                            phi_3719_ = (_e3113 + _e3212);
                                                        }
                                                        let _e3342 = phi_3719_;
                                                        let _e3345 = global_1.member[_e3342];
                                                        let _e3348 = global_1.member[_e3345];
                                                        let _e3352 = global_1.member[(_e3345 + 1u)];
                                                        let _e3356 = global_1.member[(_e3345 + 2u)];
                                                        let _e3360 = global_1.member[(_e3345 + 3u)];
                                                        let _e3364 = global_1.member[(_e3345 + 4u)];
                                                        let _e3368 = global_1.member[(_e3345 + 6u)];
                                                        switch bitcast<i32>(_e3368) {
                                                            case 0: {
                                                                phi_3750_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3750_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_3750_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_3750_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e3371 = phi_3750_;
                                                        let _e3375 = global_1.member[(_e3345 + 7u)];
                                                        switch bitcast<i32>(_e3375) {
                                                            case 0: {
                                                                phi_3759_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3759_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_3759_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_3759_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e3378 = phi_3759_;
                                                        switch bitcast<i32>(_e3371) {
                                                            case 1: {
                                                                let _e3413 = abs(_e3217);
                                                                let _e3415 = (_e3413 % 1f);
                                                                if (_e3413 >= 1f) {
                                                                    phi_8137_ = select(true, false, (_e3415 == 0f));
                                                                } else {
                                                                    phi_8137_ = true;
                                                                }
                                                                let _e3419 = phi_8137_;
                                                                let _e3420 = select(1f, _e3415, _e3419);
                                                                if (select(-1f, 1f, (_e3217 >= 0f)) > 0f) {
                                                                    phi_3785_ = _e3420;
                                                                } else {
                                                                    phi_3785_ = (1f - _e3420);
                                                                }
                                                                let _e3424 = phi_3785_;
                                                                phi_3822_ = _e3424;
                                                                break;
                                                            }
                                                            case 2: {
                                                                let _e3387 = abs(_e3217);
                                                                let _e3394 = ((select(select(u32(_e3387), 0u, (_e3387 < 0f)), 4294967295u, (_e3387 > 4294967000f)) % 2u) == 0u);
                                                                let _e3396 = (_e3387 % 1f);
                                                                if (_e3387 >= 1f) {
                                                                    phi_8120_ = select(true, false, (_e3396 == 0f));
                                                                } else {
                                                                    phi_8120_ = true;
                                                                }
                                                                let _e3400 = phi_8120_;
                                                                let _e3401 = select(1f, _e3396, _e3400);
                                                                if (select(-1f, 1f, (_e3217 >= 0f)) > 0f) {
                                                                    if _e3394 {
                                                                        phi_3814_ = _e3401;
                                                                    } else {
                                                                        phi_3814_ = (1f - _e3401);
                                                                    }
                                                                    let _e3408 = phi_3814_;
                                                                    phi_3820_ = _e3408;
                                                                } else {
                                                                    if _e3394 {
                                                                        phi_3819_ = (1f - _e3401);
                                                                    } else {
                                                                        phi_3819_ = _e3401;
                                                                    }
                                                                    let _e3405 = phi_3819_;
                                                                    phi_3820_ = _e3405;
                                                                }
                                                                let _e3410 = phi_3820_;
                                                                phi_3822_ = _e3410;
                                                                break;
                                                            }
                                                            case 0: {
                                                                if (_e3217 > 1f) {
                                                                    phi_8107_ = 0.9999999f;
                                                                } else {
                                                                    phi_8107_ = select(_e3217, 0.00000011920929f, (_e3217 < 0f));
                                                                }
                                                                let _e3384 = phi_8107_;
                                                                phi_3822_ = _e3384;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_3822_ = f32();
                                                                break;
                                                            }
                                                        }
                                                        let _e3426 = phi_3822_;
                                                        switch bitcast<i32>(_e3378) {
                                                            case 1: {
                                                                let _e3461 = abs(_e3218);
                                                                let _e3463 = (_e3461 % 1f);
                                                                if (_e3461 >= 1f) {
                                                                    phi_8185_ = select(true, false, (_e3463 == 0f));
                                                                } else {
                                                                    phi_8185_ = true;
                                                                }
                                                                let _e3467 = phi_8185_;
                                                                let _e3468 = select(1f, _e3463, _e3467);
                                                                if (select(-1f, 1f, (_e3218 >= 0f)) > 0f) {
                                                                    phi_3843_ = _e3468;
                                                                } else {
                                                                    phi_3843_ = (1f - _e3468);
                                                                }
                                                                let _e3472 = phi_3843_;
                                                                phi_3880_ = _e3472;
                                                                break;
                                                            }
                                                            case 2: {
                                                                let _e3435 = abs(_e3218);
                                                                let _e3442 = ((select(select(u32(_e3435), 0u, (_e3435 < 0f)), 4294967295u, (_e3435 > 4294967000f)) % 2u) == 0u);
                                                                let _e3444 = (_e3435 % 1f);
                                                                if (_e3435 >= 1f) {
                                                                    phi_8168_ = select(true, false, (_e3444 == 0f));
                                                                } else {
                                                                    phi_8168_ = true;
                                                                }
                                                                let _e3448 = phi_8168_;
                                                                let _e3449 = select(1f, _e3444, _e3448);
                                                                if (select(-1f, 1f, (_e3218 >= 0f)) > 0f) {
                                                                    if _e3442 {
                                                                        phi_3872_ = _e3449;
                                                                    } else {
                                                                        phi_3872_ = (1f - _e3449);
                                                                    }
                                                                    let _e3456 = phi_3872_;
                                                                    phi_3878_ = _e3456;
                                                                } else {
                                                                    if _e3442 {
                                                                        phi_3877_ = (1f - _e3449);
                                                                    } else {
                                                                        phi_3877_ = _e3449;
                                                                    }
                                                                    let _e3453 = phi_3877_;
                                                                    phi_3878_ = _e3453;
                                                                }
                                                                let _e3458 = phi_3878_;
                                                                phi_3880_ = _e3458;
                                                                break;
                                                            }
                                                            case 0: {
                                                                if (_e3218 > 1f) {
                                                                    phi_8155_ = 0.9999999f;
                                                                } else {
                                                                    phi_8155_ = select(_e3218, 0.00000011920929f, (_e3218 < 0f));
                                                                }
                                                                let _e3432 = phi_8155_;
                                                                phi_3880_ = _e3432;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_3880_ = f32();
                                                                break;
                                                            }
                                                        }
                                                        let _e3474 = phi_3880_;
                                                        let _e3476 = (_e3426 * f32(_e3356));
                                                        let _e3483 = (_e3474 * f32(_e3360));
                                                        let _e3498 = vec3<f32>((f32((select(select(u32(_e3476), 0u, (_e3476 < 0f)), 4294967295u, (_e3476 > 4294967000f)) + _e3348)) / f32(_e3130)), (f32((select(select(u32(_e3483), 0u, (_e3483 < 0f)), 4294967295u, (_e3483 > 4294967000f)) + _e3352)) / f32(_e3134)), f32(_e3364));
                                                        let _e3504 = textureSampleLevel(global_19, global_18, vec2<f32>(_e3498.x, _e3498.y), i32(_e3498.z), 0f);
                                                        phi_3939_ = select(0f, 1f, ((_e3324 - max((bitcast<f32>(_e3126) * (1f - _e3044)), bitcast<f32>(_e3121))) > _e3504.x));
                                                    } else {
                                                        phi_3939_ = 0f;
                                                    }
                                                    let _e3513 = phi_3939_;
                                                    phi_3940_ = _e3513;
                                                } else {
                                                    phi_3940_ = 0f;
                                                }
                                                let _e3515 = phi_3940_;
                                                phi_3941_ = _e3515;
                                                phi_3942_ = vec3<f32>(((fma((((1f - _e3069) * _e2988) * _e1372), 0.31830987f, ((_e3078 * _e3069) / _e3083)) * (_e2943.member_1.x * _e2986)) * _e3045), ((fma((((1f - _e3070) * _e2988) * _e1374), 0.31830987f, ((_e3078 * _e3070) / _e3083)) * (_e2943.member_1.y * _e2986)) * _e3045), ((fma((((1f - _e3071) * _e2988) * _e1376), 0.31830987f, ((_e3078 * _e3071) / _e3083)) * (_e2943.member_1.z * _e2986)) * _e3045));
                                            }
                                            let _e3517 = phi_3941_;
                                            let _e3519 = phi_3942_;
                                            phi_8883_ = _e1952;
                                            phi_4866_ = _e3517;
                                            phi_4867_ = _e3519;
                                            phi_4868_ = select(true, false, _e2980);
                                            break;
                                        }
                                        case 2: {
                                            if (_e126 >= 13u) {
                                                phi_7413_ = (_e2014.member_1 <= (_e126 - 13u));
                                            } else {
                                                phi_7413_ = false;
                                            }
                                            let _e2125 = phi_7413_;
                                            if _e2125 {
                                                let _e2128 = global_1.member[_e2014.member_1];
                                                let _e2133 = global_1.member[(_e2014.member_1 + 1u)];
                                                let _e2138 = global_1.member[(_e2014.member_1 + 2u)];
                                                let _e2144 = global_1.member[(_e2014.member_1 + 3u)];
                                                let _e2149 = global_1.member[(_e2014.member_1 + 4u)];
                                                let _e2154 = global_1.member[(_e2014.member_1 + 5u)];
                                                let _e2160 = global_1.member[(_e2014.member_1 + 6u)];
                                                let _e2165 = global_1.member[(_e2014.member_1 + 7u)];
                                                let _e2170 = global_1.member[(_e2014.member_1 + 8u)];
                                                let _e2175 = global_1.member[(_e2014.member_1 + 9u)];
                                                let _e2180 = global_1.member[(_e2014.member_1 + 10u)];
                                                let _e2185 = global_1.member[(_e2014.member_1 + 11u)];
                                                let _e2191 = global_1.member[(_e2014.member_1 + 12u)];
                                                phi_4005_ = type_37(vec3<f32>(bitcast<f32>(_e2128), bitcast<f32>(_e2133), bitcast<f32>(_e2138)), vec3<f32>(bitcast<f32>(_e2144), bitcast<f32>(_e2149), bitcast<f32>(_e2154)), bitcast<f32>(_e2160), bitcast<f32>(_e2165), vec4<f32>(bitcast<f32>(_e2170), bitcast<f32>(_e2175), bitcast<f32>(_e2180), bitcast<f32>(_e2185)), bitcast<f32>(_e2191));
                                            } else {
                                                phi_4005_ = type_37(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2195 = phi_4005_;
                                            let _e2221 = vec3<f32>((_e2075.member.x + fma(_e2114.x, _e2195.member.z, fma(_e2112.x, _e2195.member.y, (_e2110.x * _e2195.member.x)))), (_e2075.member.y + fma(_e2114.y, _e2195.member.z, fma(_e2112.y, _e2195.member.y, (_e2110.y * _e2195.member.x)))), (_e2075.member.z + fma(_e2114.z, _e2195.member.z, fma(_e2112.z, _e2195.member.y, (_e2110.z * _e2195.member.x)))));
                                            let _e2222 = (_e2221 - _e134);
                                            let _e2229 = sqrt(fma(_e2222.z, _e2222.z, fma(_e2222.x, _e2222.x, (_e2222.y * _e2222.y))));
                                            let _e2230 = (_e2229 == 0f);
                                            if _e2230 {
                                                phi_4141_ = type_38(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), 0f, 0f, 0f, 0f, 0f, 0f, 0f, false, false);
                                            } else {
                                                if _e2230 {
                                                    phi_7463_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7463_ = (_e2222 * (1f / _e2229));
                                                }
                                                let _e2234 = phi_7463_;
                                                let _e2245 = fma(_e2114.x, _e2195.member_1.z, fma(_e2112.x, _e2195.member_1.y, (_e2110.x * _e2195.member_1.x)));
                                                let _e2246 = fma(_e2114.y, _e2195.member_1.z, fma(_e2112.y, _e2195.member_1.y, (_e2110.y * _e2195.member_1.x)));
                                                let _e2247 = fma(_e2114.z, _e2195.member_1.z, fma(_e2112.z, _e2195.member_1.y, (_e2110.z * _e2195.member_1.x)));
                                                let _e2252 = sqrt(fma(_e2247, _e2247, fma(_e2245, _e2245, (_e2246 * _e2246))));
                                                if (_e2252 == 0f) {
                                                    phi_7498_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7498_ = (vec3<f32>(_e2245, _e2246, _e2247) * (1f / _e2252));
                                                }
                                                let _e2257 = phi_7498_;
                                                let _e2259 = cos(_e2195.member_2);
                                                let _e2261 = cos(_e2195.member_3);
                                                let _e2262 = (_e2259 - _e2261);
                                                let _e2274 = fma(_e2234.z, -(_e2257.z), fma(_e2234.x, -(_e2257.x), (_e2234.y * -(_e2257.y))));
                                                let _e2278 = ((_e2274 - _e2261) / _e2262);
                                                let _e2280 = select(_e2278, 0f, (_e2278 < 0f));
                                                phi_4141_ = type_38(_e2221, _e134, _e2234, _e2257, _e2229, _e2259, _e2261, _e2262, _e2274, _e2278, select(_e2280, 1f, (_e2280 > 1f)), (_e2274 > _e2259), (_e2274 > _e2261));
                                            }
                                            let _e2285 = phi_4141_;
                                            let _e2287 = (_e2285.member_4 == 0f);
                                            if _e2287 {
                                                phi_8884_ = _e1952;
                                                phi_4863_ = f32();
                                                phi_4864_ = vec3<f32>();
                                            } else {
                                                let _e2290 = (_e2195.member_5 * _e2285.member_10);
                                                let _e2294 = fma(-(_e688.z), _e297.member_3, 1f);
                                                let _e2298 = fma(0.4f, _e2294, (_e1372 * _e1384));
                                                let _e2299 = fma(0.4f, _e2294, (_e1374 * _e1384));
                                                let _e2300 = fma(0.4f, _e2294, (_e1376 * _e1384));
                                                let _e2307 = (_e1940 + _e2285.member_2);
                                                let _e2314 = sqrt(fma(_e2307.z, _e2307.z, fma(_e2307.x, _e2307.x, (_e2307.y * _e2307.y))));
                                                if (_e2314 == 0f) {
                                                    phi_7533_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7533_ = (_e2307 * (1f / _e2314));
                                                }
                                                let _e2319 = phi_7533_;
                                                let _e2320 = (_e1381 * _e1381);
                                                let _e2331 = max(fma(_e1936.z, _e2319.z, fma(_e1936.x, _e2319.x, (_e1936.y * _e2319.y))), 0f);
                                                let _e2344 = max(fma(_e1936.z, _e1940.z, fma(_e1936.x, _e1940.x, (_e1936.y * _e1940.y))), 0f);
                                                let _e2350 = fma(_e1936.z, _e2285.member_2.z, fma(_e1936.x, _e2285.member_2.x, (_e1936.y * _e2285.member_2.y)));
                                                let _e2351 = max(_e2350, 0f);
                                                let _e2352 = fma(_e688.y, _e297.member_4, 1f);
                                                let _e2353 = (_e2352 * _e2352);
                                                let _e2354 = (_e2353 * 0.125f);
                                                let _e2356 = fma(-(_e2353), 0.125f, 1f);
                                                let _e2369 = (1f - max(fma(_e2319.z, _e1940.z, fma(_e2319.x, _e1940.x, (_e2319.y * _e1940.y))), 0f));
                                                let _e2371 = select(_e2369, 0f, (_e2369 < 0f));
                                                let _e2374 = pow(select(_e2371, 1f, (_e2371 > 1f)), 5f);
                                                let _e2375 = fma((1f - _e2298), _e2374, _e2298);
                                                let _e2376 = fma((1f - _e2299), _e2374, _e2299);
                                                let _e2377 = fma((1f - _e2300), _e2374, _e2300);
                                                let _e2384 = (((_e2320 * _e2320) / (pow(fma((_e2331 * _e2331), fma(_e2320, _e2320, -1f), 1f), 2f) * 3.1415927f)) * ((_e2344 / fma(_e2344, _e2356, _e2354)) * (_e2351 / fma(_e2351, _e2356, _e2354))));
                                                let _e2389 = fma((4f * _e2344), _e2351, 0.0001f);
                                                if ((_e2014.member_3 == 4294967295u) != true) {
                                                    let _e2411 = global_1.member[_e2014.member_3];
                                                    let _e2415 = global_1.member[(_e2014.member_3 + 1u)];
                                                    let _e2419 = global_1.member[(_e2014.member_3 + 2u)];
                                                    let _e2423 = global_1.member[(_e2014.member_3 + 3u)];
                                                    let _e2427 = global_1.member[(_e2014.member_3 + 4u)];
                                                    let _e2432 = global_1.member[(_e2014.member_3 + 5u)];
                                                    let _e2437 = global_1.member[(_e2014.member_3 + 6u)];
                                                    let _e2440 = global_1.member[_e1949];
                                                    let _e2444 = global_1.member[(_e1949 + 1u)];
                                                    let _e2446 = select(_e2411, 4294967295u, (0u >= _e2415));
                                                    let _e2449 = global_1.member[_e2446];
                                                    let _e2454 = global_1.member[(_e2446 + 1u)];
                                                    let _e2459 = global_1.member[(_e2446 + 2u)];
                                                    let _e2464 = global_1.member[(_e2446 + 3u)];
                                                    let _e2469 = global_1.member[(_e2446 + 4u)];
                                                    let _e2474 = global_1.member[(_e2446 + 5u)];
                                                    let _e2479 = global_1.member[(_e2446 + 6u)];
                                                    let _e2484 = global_1.member[(_e2446 + 7u)];
                                                    let _e2489 = global_1.member[(_e2446 + 8u)];
                                                    let _e2494 = global_1.member[(_e2446 + 9u)];
                                                    let _e2499 = global_1.member[(_e2446 + 10u)];
                                                    let _e2504 = global_1.member[(_e2446 + 11u)];
                                                    let _e2509 = global_1.member[(_e2446 + 12u)];
                                                    let _e2514 = global_1.member[(_e2446 + 13u)];
                                                    let _e2519 = global_1.member[(_e2446 + 14u)];
                                                    let _e2524 = global_1.member[(_e2446 + 15u)];
                                                    let _e2544 = (bitcast<f32>(_e2524) + fma(bitcast<f32>(_e2504), _e134.z, fma(bitcast<f32>(_e2484), _e134.y, (bitcast<f32>(_e2464) * _e134.x))));
                                                    let _e2545 = ((bitcast<f32>(_e2509) + fma(bitcast<f32>(_e2489), _e134.z, fma(bitcast<f32>(_e2469), _e134.y, (bitcast<f32>(_e2449) * _e134.x)))) / _e2544);
                                                    let _e2546 = ((bitcast<f32>(_e2514) + fma(bitcast<f32>(_e2494), _e134.z, fma(bitcast<f32>(_e2474), _e134.y, (bitcast<f32>(_e2454) * _e134.x)))) / _e2544);
                                                    let _e2547 = ((bitcast<f32>(_e2519) + fma(bitcast<f32>(_e2499), _e134.z, fma(bitcast<f32>(_e2479), _e134.y, (bitcast<f32>(_e2459) * _e134.x)))) / _e2544);
                                                    if (abs(_e2545) <= 1f) {
                                                        let _e2551 = (abs(_e2546) <= 1f);
                                                        if _e2551 {
                                                            phi_7638_ = (abs(_e2547) <= 1f);
                                                        } else {
                                                            phi_7638_ = bool();
                                                        }
                                                        let _e2555 = phi_7638_;
                                                        phi_7641_ = _e2555;
                                                        phi_7642_ = select(true, false, _e2551);
                                                    } else {
                                                        phi_7641_ = bool();
                                                        phi_7642_ = true;
                                                    }
                                                    let _e2558 = phi_7641_;
                                                    let _e2560 = phi_7642_;
                                                    if select(_e2558, false, _e2560) {
                                                        let _e2568 = global_1.member[select(_e2419, 4294967295u, (0u >= _e2423))];
                                                        let _e2571 = global_1.member[_e2568];
                                                        let _e2575 = global_1.member[(_e2568 + 1u)];
                                                        let _e2579 = global_1.member[(_e2568 + 2u)];
                                                        let _e2583 = global_1.member[(_e2568 + 3u)];
                                                        let _e2587 = global_1.member[(_e2568 + 4u)];
                                                        let _e2591 = global_1.member[(_e2568 + 6u)];
                                                        switch bitcast<i32>(_e2591) {
                                                            case 0: {
                                                                phi_4503_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_4503_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_4503_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_4503_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e2594 = phi_4503_;
                                                        let _e2598 = global_1.member[(_e2568 + 7u)];
                                                        switch bitcast<i32>(_e2598) {
                                                            case 0: {
                                                                phi_4512_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_4512_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_4512_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_4512_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e2601 = phi_4512_;
                                                        let _e2602 = bitcast<i32>(_e2437);
                                                        let _e2604 = f32(_e2579);
                                                        let _e2605 = f32(_e2583);
                                                        let _e2609 = type_35((_e2602 / -2i), (_e2602 / 2i), false);
                                                        phi_8734_ = _e1952;
                                                        phi_4540_ = _e2609;
                                                        phi_4543_ = 0f;
                                                        phi_4545_ = 0f;
                                                        loop {
                                                            let _e2611 = phi_8734_;
                                                            let _e2613 = phi_4540_;
                                                            let _e2615 = phi_4543_;
                                                            let _e2617 = phi_4545_;
                                                            local_2 = _e2615;
                                                            local_3 = _e2617;
                                                            if _e2613.member_2 {
                                                                phi_4557_ = true;
                                                            } else {
                                                                phi_4557_ = ((_e2613.member <= _e2613.member_1) != true);
                                                            }
                                                            let _e2624 = phi_4557_;
                                                            if _e2624 {
                                                                phi_4541_ = _e2613;
                                                                phi_4600_ = type_36(0u, type_36().member_1);
                                                            } else {
                                                                if (_e2613.member < _e2613.member_1) {
                                                                    let _e2632 = (_e2613.member + 1i);
                                                                    if select(false, true, ((false == (_e2632 > _e2613.member)) != false)) {
                                                                        phi_4585_ = type_36(0u, type_36().member_1);
                                                                    } else {
                                                                        phi_4585_ = type_36(1u, _e2632);
                                                                    }
                                                                    let _e2642 = phi_4585_;
                                                                    switch bitcast<i32>(_e2642.member) {
                                                                        case 0: {
                                                                            phi_8814_ = true;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            break;
                                                                        }
                                                                    }
                                                                    phi_4597_ = type_35(_e2642.member_1, _e2613.member_1, _e2613.member_2);
                                                                } else {
                                                                    phi_4597_ = type_35(_e2613.member, _e2613.member_1, true);
                                                                }
                                                                let _e2651 = phi_4597_;
                                                                phi_4541_ = _e2651;
                                                                phi_4600_ = type_36(1u, _e2613.member);
                                                            }
                                                            let _e2657 = phi_4541_;
                                                            let _e2659 = phi_4600_;
                                                            switch bitcast<i32>(_e2659.member) {
                                                                case 0: {
                                                                    phi_8815_ = _e2611;
                                                                    phi_4544_ = f32();
                                                                    phi_4546_ = f32();
                                                                    phi_4858_ = false;
                                                                    break;
                                                                }
                                                                case 1: {
                                                                    phi_4611_ = _e2609;
                                                                    phi_4614_ = _e2615;
                                                                    phi_4616_ = _e2617;
                                                                    loop {
                                                                        let _e2664 = phi_4611_;
                                                                        let _e2666 = phi_4614_;
                                                                        let _e2668 = phi_4616_;
                                                                        local_9 = _e2666;
                                                                        local_10 = _e2668;
                                                                        if _e2664.member_2 {
                                                                            phi_4628_ = true;
                                                                        } else {
                                                                            phi_4628_ = ((_e2664.member <= _e2664.member_1) != true);
                                                                        }
                                                                        let _e2675 = phi_4628_;
                                                                        if _e2675 {
                                                                            phi_4612_ = _e2664;
                                                                            phi_4671_ = type_36(0u, type_36().member_1);
                                                                        } else {
                                                                            if (_e2664.member < _e2664.member_1) {
                                                                                let _e2683 = (_e2664.member + 1i);
                                                                                if select(false, true, ((false == (_e2683 > _e2664.member)) != false)) {
                                                                                    phi_4656_ = type_36(0u, type_36().member_1);
                                                                                } else {
                                                                                    phi_4656_ = type_36(1u, _e2683);
                                                                                }
                                                                                let _e2693 = phi_4656_;
                                                                                switch bitcast<i32>(_e2693.member) {
                                                                                    case 0: {
                                                                                        phi_8718_ = true;
                                                                                        break;
                                                                                    }
                                                                                    case 1: {
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                phi_4668_ = type_35(_e2693.member_1, _e2664.member_1, _e2664.member_2);
                                                                            } else {
                                                                                phi_4668_ = type_35(_e2664.member, _e2664.member_1, true);
                                                                            }
                                                                            let _e2702 = phi_4668_;
                                                                            phi_4612_ = _e2702;
                                                                            phi_4671_ = type_36(1u, _e2664.member);
                                                                        }
                                                                        let _e2708 = phi_4612_;
                                                                        let _e2710 = phi_4671_;
                                                                        switch bitcast<i32>(_e2710.member) {
                                                                            case 0: {
                                                                                phi_4615_ = f32();
                                                                                phi_4617_ = f32();
                                                                                phi_4857_ = false;
                                                                                break;
                                                                            }
                                                                            case 1: {
                                                                                let _e2718 = fma((_e2545 + 1f), 0.5f, (f32(_e2659.member_1) * (1f / _e2604)));
                                                                                let _e2719 = fma(fma(_e2546, -1f, 1f), 0.5f, (f32(_e2710.member_1) * (1f / _e2605)));
                                                                                switch bitcast<i32>(_e2594) {
                                                                                    case 1: {
                                                                                        let _e2754 = abs(_e2718);
                                                                                        let _e2756 = (_e2754 % 1f);
                                                                                        if (_e2754 >= 1f) {
                                                                                            phi_7694_ = select(true, false, (_e2756 == 0f));
                                                                                        } else {
                                                                                            phi_7694_ = true;
                                                                                        }
                                                                                        let _e2760 = phi_7694_;
                                                                                        let _e2761 = select(1f, _e2756, _e2760);
                                                                                        if (select(-1f, 1f, (_e2718 >= 0f)) > 0f) {
                                                                                            phi_4703_ = _e2761;
                                                                                        } else {
                                                                                            phi_4703_ = (1f - _e2761);
                                                                                        }
                                                                                        let _e2765 = phi_4703_;
                                                                                        phi_4740_ = _e2765;
                                                                                        break;
                                                                                    }
                                                                                    case 2: {
                                                                                        let _e2728 = abs(_e2718);
                                                                                        let _e2735 = ((select(select(u32(_e2728), 0u, (_e2728 < 0f)), 4294967295u, (_e2728 > 4294967000f)) % 2u) == 0u);
                                                                                        let _e2737 = (_e2728 % 1f);
                                                                                        if (_e2728 >= 1f) {
                                                                                            phi_7677_ = select(true, false, (_e2737 == 0f));
                                                                                        } else {
                                                                                            phi_7677_ = true;
                                                                                        }
                                                                                        let _e2741 = phi_7677_;
                                                                                        let _e2742 = select(1f, _e2737, _e2741);
                                                                                        if (select(-1f, 1f, (_e2718 >= 0f)) > 0f) {
                                                                                            if _e2735 {
                                                                                                phi_4732_ = _e2742;
                                                                                            } else {
                                                                                                phi_4732_ = (1f - _e2742);
                                                                                            }
                                                                                            let _e2749 = phi_4732_;
                                                                                            phi_4738_ = _e2749;
                                                                                        } else {
                                                                                            if _e2735 {
                                                                                                phi_4737_ = (1f - _e2742);
                                                                                            } else {
                                                                                                phi_4737_ = _e2742;
                                                                                            }
                                                                                            let _e2746 = phi_4737_;
                                                                                            phi_4738_ = _e2746;
                                                                                        }
                                                                                        let _e2751 = phi_4738_;
                                                                                        phi_4740_ = _e2751;
                                                                                        break;
                                                                                    }
                                                                                    case 0: {
                                                                                        if (_e2718 > 1f) {
                                                                                            phi_7664_ = 0.9999999f;
                                                                                        } else {
                                                                                            phi_7664_ = select(_e2718, 0.00000011920929f, (_e2718 < 0f));
                                                                                        }
                                                                                        let _e2725 = phi_7664_;
                                                                                        phi_4740_ = _e2725;
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        phi_4740_ = f32();
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2767 = phi_4740_;
                                                                                switch bitcast<i32>(_e2601) {
                                                                                    case 1: {
                                                                                        let _e2802 = abs(_e2719);
                                                                                        let _e2804 = (_e2802 % 1f);
                                                                                        if (_e2802 >= 1f) {
                                                                                            phi_7742_ = select(true, false, (_e2804 == 0f));
                                                                                        } else {
                                                                                            phi_7742_ = true;
                                                                                        }
                                                                                        let _e2808 = phi_7742_;
                                                                                        let _e2809 = select(1f, _e2804, _e2808);
                                                                                        if (select(-1f, 1f, (_e2719 >= 0f)) > 0f) {
                                                                                            phi_4759_ = _e2809;
                                                                                        } else {
                                                                                            phi_4759_ = (1f - _e2809);
                                                                                        }
                                                                                        let _e2813 = phi_4759_;
                                                                                        phi_4796_ = _e2813;
                                                                                        break;
                                                                                    }
                                                                                    case 2: {
                                                                                        let _e2776 = abs(_e2719);
                                                                                        let _e2783 = ((select(select(u32(_e2776), 0u, (_e2776 < 0f)), 4294967295u, (_e2776 > 4294967000f)) % 2u) == 0u);
                                                                                        let _e2785 = (_e2776 % 1f);
                                                                                        if (_e2776 >= 1f) {
                                                                                            phi_7725_ = select(true, false, (_e2785 == 0f));
                                                                                        } else {
                                                                                            phi_7725_ = true;
                                                                                        }
                                                                                        let _e2789 = phi_7725_;
                                                                                        let _e2790 = select(1f, _e2785, _e2789);
                                                                                        if (select(-1f, 1f, (_e2719 >= 0f)) > 0f) {
                                                                                            if _e2783 {
                                                                                                phi_4788_ = _e2790;
                                                                                            } else {
                                                                                                phi_4788_ = (1f - _e2790);
                                                                                            }
                                                                                            let _e2797 = phi_4788_;
                                                                                            phi_4794_ = _e2797;
                                                                                        } else {
                                                                                            if _e2783 {
                                                                                                phi_4793_ = (1f - _e2790);
                                                                                            } else {
                                                                                                phi_4793_ = _e2790;
                                                                                            }
                                                                                            let _e2794 = phi_4793_;
                                                                                            phi_4794_ = _e2794;
                                                                                        }
                                                                                        let _e2799 = phi_4794_;
                                                                                        phi_4796_ = _e2799;
                                                                                        break;
                                                                                    }
                                                                                    case 0: {
                                                                                        if (_e2719 > 1f) {
                                                                                            phi_7712_ = 0.9999999f;
                                                                                        } else {
                                                                                            phi_7712_ = select(_e2719, 0.00000011920929f, (_e2719 < 0f));
                                                                                        }
                                                                                        let _e2773 = phi_7712_;
                                                                                        phi_4796_ = _e2773;
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        phi_4796_ = f32();
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2815 = phi_4796_;
                                                                                let _e2816 = (_e2767 * _e2604);
                                                                                let _e2822 = (_e2815 * _e2605);
                                                                                let _e2837 = vec3<f32>((f32((select(select(u32(_e2816), 0u, (_e2816 < 0f)), 4294967295u, (_e2816 > 4294967000f)) + _e2571)) / f32(_e2440)), (f32((select(select(u32(_e2822), 0u, (_e2822 < 0f)), 4294967295u, (_e2822 > 4294967000f)) + _e2575)) / f32(_e2444)), f32(_e2587));
                                                                                let _e2843 = textureSampleLevel(global_19, global_18, vec2<f32>(_e2837.x, _e2837.y), i32(_e2837.z), 0f);
                                                                                phi_4615_ = (_e2666 + 1f);
                                                                                phi_4617_ = (_e2668 + select(0f, 1f, ((_e2547 - max((bitcast<f32>(_e2432) * (1f - _e2350)), bitcast<f32>(_e2427))) > _e2843.x)));
                                                                                phi_4857_ = true;
                                                                                break;
                                                                            }
                                                                            default: {
                                                                                phi_4615_ = f32();
                                                                                phi_4617_ = f32();
                                                                                phi_4857_ = bool();
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e2854 = phi_4615_;
                                                                        let _e2856 = phi_4617_;
                                                                        let _e2858 = phi_4857_;
                                                                        continue;
                                                                        continuing {
                                                                            phi_4611_ = _e2708;
                                                                            phi_4614_ = _e2854;
                                                                            phi_4616_ = _e2856;
                                                                            phi_8718_ = _e2611;
                                                                            break if !(_e2858);
                                                                        }
                                                                    }
                                                                    let _e2861 = phi_8718_;
                                                                    phi_8814_ = _e2861;
                                                                    if _e2861 {
                                                                        break;
                                                                    }
                                                                    phi_8815_ = _e2861;
                                                                    let _e4516 = local_9;
                                                                    phi_4544_ = _e4516;
                                                                    let _e4519 = local_10;
                                                                    phi_4546_ = _e4519;
                                                                    phi_4858_ = true;
                                                                    break;
                                                                }
                                                                default: {
                                                                    phi_8815_ = _e2611;
                                                                    phi_4544_ = f32();
                                                                    phi_4546_ = f32();
                                                                    phi_4858_ = bool();
                                                                    break;
                                                                }
                                                            }
                                                            let _e2863 = phi_8815_;
                                                            let _e2865 = phi_4544_;
                                                            let _e2867 = phi_4546_;
                                                            let _e2869 = phi_4858_;
                                                            continue;
                                                            continuing {
                                                                phi_8734_ = _e2863;
                                                                phi_4540_ = _e2657;
                                                                phi_4543_ = _e2865;
                                                                phi_4545_ = _e2867;
                                                                phi_8814_ = _e2863;
                                                                break if !(_e2869);
                                                            }
                                                        }
                                                        let _e2872 = phi_8814_;
                                                        phi_8879_ = _e2872;
                                                        if _e2872 {
                                                            break;
                                                        }
                                                        let _e2874 = local_2;
                                                        let _e2877 = local_3;
                                                        phi_8886_ = _e2872;
                                                        phi_4861_ = (_e2877 / max(_e2874, 1f));
                                                    } else {
                                                        phi_8886_ = _e1952;
                                                        phi_4861_ = 0f;
                                                    }
                                                    let _e2880 = phi_8886_;
                                                    let _e2882 = phi_4861_;
                                                    phi_8885_ = _e2880;
                                                    phi_4862_ = _e2882;
                                                } else {
                                                    phi_8885_ = _e1952;
                                                    phi_4862_ = 0f;
                                                }
                                                let _e2884 = phi_8885_;
                                                let _e2886 = phi_4862_;
                                                phi_8884_ = _e2884;
                                                phi_4863_ = _e2886;
                                                phi_4864_ = vec3<f32>(((fma((((1f - _e2375) * _e2294) * _e1372), 0.31830987f, ((_e2384 * _e2375) / _e2389)) * (_e2195.member_4.x * _e2290)) * _e2351), ((fma((((1f - _e2376) * _e2294) * _e1374), 0.31830987f, ((_e2384 * _e2376) / _e2389)) * (_e2195.member_4.y * _e2290)) * _e2351), ((fma((((1f - _e2377) * _e2294) * _e1376), 0.31830987f, ((_e2384 * _e2377) / _e2389)) * (_e2195.member_4.z * _e2290)) * _e2351));
                                            }
                                            let _e2888 = phi_8884_;
                                            let _e2890 = phi_4863_;
                                            let _e2892 = phi_4864_;
                                            phi_8883_ = _e2888;
                                            phi_4866_ = _e2890;
                                            phi_4867_ = _e2892;
                                            phi_4868_ = select(true, false, _e2287);
                                            break;
                                        }
                                        default: {
                                            phi_8883_ = _e1952;
                                            phi_4866_ = f32();
                                            phi_4867_ = vec3<f32>();
                                            phi_4868_ = bool();
                                            break;
                                        }
                                    }
                                    let _e4212 = phi_8883_;
                                    let _e4214 = phi_4866_;
                                    let _e4216 = phi_4867_;
                                    let _e4218 = phi_4868_;
                                    if _e4218 {
                                        let _e4219 = (1f - _e4214);
                                        phi_4888_ = vec3<f32>(fma(_e4216.x, _e4219, _e1956.x), fma(_e4216.y, _e4219, _e1956.y), fma(_e4216.z, _e4219, _e1956.z));
                                    } else {
                                        phi_4888_ = vec3<f32>();
                                    }
                                    let _e4231 = phi_4888_;
                                    phi_8880_ = _e4212;
                                    phi_2383_ = select(_e4231, _e1956, vec3(select(true, false, _e4218)));
                                    phi_4894_ = true;
                                    break;
                                }
                                default: {
                                    phi_8880_ = _e1952;
                                    phi_2383_ = vec3<f32>();
                                    phi_4894_ = bool();
                                    break;
                                }
                            }
                            let _e4236 = phi_8880_;
                            let _e4238 = phi_2383_;
                            let _e4240 = phi_4894_;
                            continue;
                            continuing {
                                phi_8761_ = _e4236;
                                phi_2379_ = _e1969;
                                phi_2382_ = _e4238;
                                phi_8879_ = _e4236;
                                break if !(_e4240);
                            }
                        }
                        let _e4243 = phi_8879_;
                        phi_8946_ = _e4243;
                        if _e4243 {
                            break;
                        }
                        let _e4245 = fma(-(_e688.z), _e297.member_3, 1f);
                        let _e4249 = fma(0.04f, _e4245, (_e1372 * _e1384));
                        let _e4250 = fma(0.04f, _e4245, (_e1374 * _e1384));
                        let _e4251 = fma(0.04f, _e4245, (_e1376 * _e1384));
                        let _e4263 = fma(-(_e688.y), _e297.member_4, 1f);
                        let _e4270 = (1f - max(fma(_e1936.z, _e1940.z, fma(_e1936.x, _e1940.x, (_e1936.y * _e1940.y))), 0f));
                        let _e4272 = select(_e4270, 0f, (_e4270 < 0f));
                        let _e4275 = pow(select(_e4272, 1f, (_e4272 > 1f)), 5f);
                        let _e4276 = fma((max(_e4263, _e4249) - _e4249), _e4275, _e4249);
                        let _e4277 = fma((max(_e4263, _e4250) - _e4250), _e4275, _e4250);
                        let _e4278 = fma((max(_e4263, _e4251) - _e4251), _e4275, _e4251);
                        let _e4298 = local_6;
                        let _e4302 = local_7;
                        let _e4306 = local_8;
                        phi_8954_ = _e4243;
                        phi_5011_ = vec4<f32>(fma(_e1394, _e297.member_1, fma(fma(((1f - _e4276) * _e4245), (_e1403.x * _e1372), (_e1751.x * fma(_e4276, _e1767.x, _e1767.y))), _e1388, _e4298.x)), fma(_e1396, _e297.member_1, fma(fma(((1f - _e4277) * _e4245), (_e1403.y * _e1374), (_e1751.y * fma(_e4277, _e1767.x, _e1767.y))), _e1388, _e4302.y)), fma(_e1398, _e297.member_1, fma(fma(((1f - _e4278) * _e4245), (_e1403.z * _e1376), (_e1751.z * fma(_e4278, _e1767.x, _e1767.y))), _e1388, _e4306.z)), 1f);
                    } else {
                        phi_8954_ = false;
                        phi_5011_ = (vec4<f32>((_e128.x * _e494.x), (_e128.y * _e494.y), (_e128.z * _e494.z), (_e128.w * _e494.w)) * _e297.member_2);
                    }
                    let _e4314 = phi_8954_;
                    let _e4316 = phi_5011_;
                    global_20 = _e4316;
                    phi_8946_ = _e4314;
                    break;
                }
                case 1: {
                    let _e1909 = sqrt(fma(_e129.x, _e129.x, (_e129.y * _e129.y)));
                    if (_e1909 == 0f) {
                        phi_7158_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7158_ = (vec3<f32>(_e129.x, _e129.y, 0f) * (1f / _e1909));
                    }
                    let _e1914 = phi_7158_;
                    global_20 = vec4<f32>(((_e1914.x + 1f) * 0.5f), ((_e1914.y + 1f) * 0.5f), ((_e1914.z + 1f) * 0.5f), 1f);
                    phi_8946_ = false;
                    break;
                }
                case 2: {
                    let _e1888 = sqrt(fma(_e130.x, _e130.x, (_e130.y * _e130.y)));
                    if (_e1888 == 0f) {
                        phi_7109_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7109_ = (vec3<f32>(_e130.x, _e130.y, 0f) * (1f / _e1888));
                    }
                    let _e1893 = phi_7109_;
                    global_20 = vec4<f32>(((_e1893.x + 1f) * 0.5f), ((_e1893.y + 1f) * 0.5f), ((_e1893.z + 1f) * 0.5f), 1f);
                    phi_8946_ = false;
                    break;
                }
                case 3: {
                    if _e1730 {
                        phi_7060_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7060_ = (_e1357 * (1f / _e1729));
                    }
                    let _e1872 = phi_7060_;
                    global_20 = vec4<f32>(((_e1872.x + 1f) * 0.5f), ((_e1872.y + 1f) * 0.5f), ((_e1872.z + 1f) * 0.5f), 1f);
                    phi_8946_ = false;
                    break;
                }
                case 4: {
                    global_20 = _e128;
                    phi_8946_ = false;
                    break;
                }
                case 5: {
                    let _e1853 = sqrt(fma(_e131.z, _e131.z, fma(_e131.x, _e131.x, (_e131.y * _e131.y))));
                    if (_e1853 == 0f) {
                        phi_7011_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7011_ = (_e131 * (1f / _e1853));
                    }
                    let _e1858 = phi_7011_;
                    global_20 = vec4<f32>(((_e1858.x + 1f) * 0.5f), ((_e1858.y + 1f) * 0.5f), ((_e1858.z + 1f) * 0.5f), 1f);
                    phi_8946_ = false;
                    break;
                }
                case 6: {
                    let _e1831 = sqrt(fma(_e1355.z, _e1355.z, fma(_e1355.x, _e1355.x, (_e1355.y * _e1355.y))));
                    if (_e1831 == 0f) {
                        phi_6962_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6962_ = (_e1355 * (1f / _e1831));
                    }
                    let _e1836 = phi_6962_;
                    global_20 = vec4<f32>(((_e1836.x + 1f) * 0.5f), ((_e1836.y + 1f) * 0.5f), ((_e1836.z + 1f) * 0.5f), 1f);
                    phi_8946_ = false;
                    break;
                }
                case 7: {
                    let _e1809 = sqrt(fma(_e132.z, _e132.z, fma(_e132.x, _e132.x, (_e132.y * _e132.y))));
                    if (_e1809 == 0f) {
                        phi_6913_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6913_ = (_e132 * (1f / _e1809));
                    }
                    let _e1814 = phi_6913_;
                    global_20 = vec4<f32>(((_e1814.x + 1f) * 0.5f), ((_e1814.y + 1f) * 0.5f), ((_e1814.z + 1f) * 0.5f), 1f);
                    phi_8946_ = false;
                    break;
                }
                case 8: {
                    let _e1787 = sqrt(fma(_e133.z, _e133.z, fma(_e133.x, _e133.x, (_e133.y * _e133.y))));
                    if (_e1787 == 0f) {
                        phi_6864_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6864_ = (_e133 * (1f / _e1787));
                    }
                    let _e1792 = phi_6864_;
                    global_20 = vec4<f32>(((_e1792.x + 1f) * 0.5f), ((_e1792.y + 1f) * 0.5f), ((_e1792.z + 1f) * 0.5f), 1f);
                    phi_8946_ = false;
                    break;
                }
                case 9: {
                    global_20 = vec4<f32>(_e1403.x, _e1403.y, _e1403.z, 1f);
                    phi_8946_ = false;
                    break;
                }
                case 10: {
                    global_20 = vec4<f32>(_e1751.x, _e1751.y, _e1751.z, 1f);
                    phi_8946_ = false;
                    break;
                }
                case 11: {
                    global_20 = vec4<f32>(_e1767.x, _e1767.y, 1f, 1f);
                    phi_8946_ = false;
                    break;
                }
                case 12: {
                    global_20 = (vec4<f32>(_e1361, _e1364, _e1367, (_e494.w * _e297.member_2.w)) * _e128);
                    phi_8946_ = false;
                    break;
                }
                case 13: {
                    global_20 = vec4<f32>(_e1381, _e1381, _e1381, 1f);
                    phi_8946_ = false;
                    break;
                }
                case 14: {
                    global_20 = vec4<f32>(_e1384, _e1384, _e1384, 1f);
                    phi_8946_ = false;
                    break;
                }
                case 15: {
                    global_20 = vec4<f32>(_e1388, _e1388, _e1388, 1f);
                    phi_8946_ = false;
                    break;
                }
                case 16: {
                    global_20 = vec4<f32>((_e1394 * _e297.member_1), (_e1396 * _e297.member_1), (_e1398 * _e297.member_1), 1f);
                    phi_8946_ = false;
                    break;
                }
                case 17: {
                    global_20 = vec4<f32>(_e1270.x, _e1270.y, _e1270.z, 1f);
                    phi_8946_ = false;
                    break;
                }
                case 18: {
                    global_20 = vec4<f32>(_e297.member.x, _e297.member.y, _e297.member.z, 1f);
                    phi_8946_ = false;
                    break;
                }
                case 19: {
                    global_20 = vec4<f32>(_e297.member_1, _e297.member_1, _e297.member_1, 1f);
                    phi_8946_ = false;
                    break;
                }
                default: {
                    phi_8946_ = false;
                    break;
                }
            }
            let _e4318 = phi_8946_;
            if _e4318 {
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
