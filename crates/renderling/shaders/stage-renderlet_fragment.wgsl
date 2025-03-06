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
    member: u32,
    member_1: vec2<f32>,
}

struct type_38 {
    member: bool,
    member_1: vec3<f32>,
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
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var phi_704_: u32;
    var phi_5724_: bool;
    var phi_832_: type_33;
    var phi_836_: type_33;
    var phi_5761_: bool;
    var phi_876_: u32;
    var phi_885_: u32;
    var phi_898_: type_15;
    var phi_5783_: f32;
    var phi_5796_: bool;
    var phi_952_: f32;
    var phi_947_: f32;
    var phi_953_: f32;
    var phi_5813_: bool;
    var phi_918_: f32;
    var phi_955_: f32;
    var phi_5831_: f32;
    var phi_5844_: bool;
    var phi_1010_: f32;
    var phi_1005_: f32;
    var phi_1011_: f32;
    var phi_5861_: bool;
    var phi_976_: f32;
    var phi_1013_: f32;
    var phi_5897_: bool;
    var phi_1096_: u32;
    var phi_1105_: u32;
    var phi_1118_: type_15;
    var phi_5918_: f32;
    var phi_5931_: bool;
    var phi_1172_: f32;
    var phi_1167_: f32;
    var phi_1173_: f32;
    var phi_5948_: bool;
    var phi_1138_: f32;
    var phi_1175_: f32;
    var phi_5966_: f32;
    var phi_5979_: bool;
    var phi_1230_: f32;
    var phi_1225_: f32;
    var phi_1231_: f32;
    var phi_5996_: bool;
    var phi_1196_: f32;
    var phi_1233_: f32;
    var phi_6032_: bool;
    var phi_1316_: u32;
    var phi_1325_: u32;
    var phi_1338_: type_15;
    var phi_6053_: f32;
    var phi_6066_: bool;
    var phi_1392_: f32;
    var phi_1387_: f32;
    var phi_1393_: f32;
    var phi_6083_: bool;
    var phi_1358_: f32;
    var phi_1395_: f32;
    var phi_6101_: f32;
    var phi_6114_: bool;
    var phi_1450_: f32;
    var phi_1445_: f32;
    var phi_1451_: f32;
    var phi_6131_: bool;
    var phi_1416_: f32;
    var phi_1453_: f32;
    var phi_6167_: bool;
    var phi_1536_: u32;
    var phi_1545_: u32;
    var phi_1558_: type_15;
    var phi_6188_: f32;
    var phi_6201_: bool;
    var phi_1612_: f32;
    var phi_1607_: f32;
    var phi_1613_: f32;
    var phi_6218_: bool;
    var phi_1578_: f32;
    var phi_1615_: f32;
    var phi_6236_: f32;
    var phi_6249_: bool;
    var phi_1670_: f32;
    var phi_1665_: f32;
    var phi_1671_: f32;
    var phi_6266_: bool;
    var phi_1636_: f32;
    var phi_1673_: f32;
    var phi_6302_: bool;
    var phi_1756_: u32;
    var phi_1765_: u32;
    var phi_1778_: type_15;
    var phi_6323_: f32;
    var phi_6336_: bool;
    var phi_1832_: f32;
    var phi_1827_: f32;
    var phi_1833_: f32;
    var phi_6353_: bool;
    var phi_1798_: f32;
    var phi_1835_: f32;
    var phi_6371_: f32;
    var phi_6384_: bool;
    var phi_1890_: f32;
    var phi_1885_: f32;
    var phi_1891_: f32;
    var phi_6401_: bool;
    var phi_1856_: f32;
    var phi_1893_: f32;
    var phi_6459_: vec3<f32>;
    var phi_6494_: vec3<f32>;
    var phi_6529_: vec3<f32>;
    var phi_6564_: vec3<f32>;
    var phi_6599_: vec3<f32>;
    var phi_1987_: vec3<f32>;
    var phi_1988_: vec3<f32>;
    var phi_6631_: bool;
    var phi_2195_: type_14;
    var phi_2196_: type_14;
    var phi_2219_: type_14;
    var phi_2246_: bool;
    var phi_2252_: type_14;
    var phi_2253_: type_14;
    var phi_2276_: type_14;
    var phi_2299_: bool;
    var phi_2320_: type_25;
    var phi_6703_: vec3<f32>;
    var phi_6762_: vec3<f32>;
    var phi_6836_: vec3<f32>;
    var phi_6896_: vec3<f32>;
    var phi_6945_: vec3<f32>;
    var phi_6994_: vec3<f32>;
    var phi_7043_: vec3<f32>;
    var phi_7092_: vec3<f32>;
    var phi_7141_: vec3<f32>;
    var phi_7190_: vec3<f32>;
    var phi_7229_: vec3<f32>;
    var phi_7264_: vec3<f32>;
    var phi_9032_: bool;
    var phi_2387_: type_14;
    var phi_2390_: vec3<f32>;
    var phi_2388_: type_14;
    var phi_2413_: type_14;
    var phi_7290_: u32;
    var phi_7309_: bool;
    var phi_2430_: u32;
    var phi_7333_: bool;
    var phi_2442_: u32;
    var phi_2456_: type_30;
    var phi_7365_: bool;
    var phi_2506_: type_31;
    var phi_7445_: bool;
    var phi_4013_: type_39;
    var phi_7495_: vec3<f32>;
    var phi_7530_: vec3<f32>;
    var phi_4149_: type_40;
    var phi_7565_: vec3<f32>;
    var phi_7670_: bool;
    var phi_7673_: bool;
    var phi_7674_: bool;
    var phi_4511_: u32;
    var phi_4520_: u32;
    var phi_9005_: bool;
    var phi_4548_: type_35;
    var phi_4551_: f32;
    var phi_4553_: f32;
    var phi_4565_: bool;
    var phi_4593_: type_36;
    var phi_4605_: type_35;
    var phi_4549_: type_35;
    var phi_4608_: type_36;
    var phi_4619_: type_35;
    var phi_4622_: f32;
    var phi_4624_: f32;
    var phi_4636_: bool;
    var phi_4664_: type_36;
    var phi_4676_: type_35;
    var phi_4620_: type_35;
    var phi_4679_: type_36;
    var phi_7696_: f32;
    var phi_7709_: bool;
    var phi_4745_: f32;
    var phi_4740_: f32;
    var phi_4746_: f32;
    var phi_7726_: bool;
    var phi_4711_: f32;
    var phi_4748_: f32;
    var phi_7744_: f32;
    var phi_7757_: bool;
    var phi_4801_: f32;
    var phi_4796_: f32;
    var phi_4802_: f32;
    var phi_7774_: bool;
    var phi_4767_: f32;
    var phi_4804_: f32;
    var phi_4623_: f32;
    var phi_4625_: f32;
    var phi_4865_: bool;
    var phi_8989_: bool;
    var phi_9086_: bool;
    var phi_4552_: f32;
    var phi_4554_: f32;
    var phi_4866_: bool;
    var phi_9085_: bool;
    var local_2: f32;
    var local_3: f32;
    var phi_9157_: bool;
    var phi_4869_: f32;
    var phi_9156_: bool;
    var phi_4870_: f32;
    var phi_9155_: bool;
    var phi_4871_: f32;
    var phi_4872_: vec3<f32>;
    var phi_7811_: bool;
    var phi_3345_: type_34;
    var phi_7858_: vec3<f32>;
    var phi_7893_: vec3<f32>;
    var phi_8014_: vec3<f32>;
    var phi_8144_: type_38;
    var phi_8147_: type_38;
    var phi_8148_: bool;
    var phi_8153_: type_38;
    var phi_8180_: type_38;
    var phi_8183_: type_38;
    var phi_8184_: bool;
    var phi_8189_: type_38;
    var phi_8216_: type_38;
    var phi_8219_: type_38;
    var phi_8220_: bool;
    var phi_8225_: type_38;
    var phi_8252_: type_38;
    var phi_8255_: type_38;
    var phi_8256_: bool;
    var phi_8261_: type_38;
    var phi_8288_: type_38;
    var phi_8291_: type_38;
    var phi_8292_: bool;
    var phi_8297_: type_38;
    var phi_8324_: type_38;
    var phi_8327_: type_38;
    var phi_8328_: bool;
    var phi_8333_: type_38;
    var phi_8056_: type_37;
    var phi_8068_: type_37;
    var phi_8080_: type_37;
    var phi_8091_: type_37;
    var phi_8103_: type_37;
    var phi_8116_: type_37;
    var phi_3599_: u32;
    var phi_8352_: bool;
    var phi_8355_: bool;
    var phi_8356_: bool;
    var phi_3727_: u32;
    var phi_3758_: u32;
    var phi_3767_: u32;
    var phi_8378_: f32;
    var phi_8391_: bool;
    var phi_3827_: f32;
    var phi_3822_: f32;
    var phi_3828_: f32;
    var phi_8408_: bool;
    var phi_3793_: f32;
    var phi_3830_: f32;
    var phi_8426_: f32;
    var phi_8439_: bool;
    var phi_3885_: f32;
    var phi_3880_: f32;
    var phi_3886_: f32;
    var phi_8456_: bool;
    var phi_3851_: f32;
    var phi_3888_: f32;
    var phi_3947_: f32;
    var phi_3948_: f32;
    var phi_3949_: f32;
    var phi_3950_: vec3<f32>;
    var phi_8493_: bool;
    var phi_2554_: type_34;
    var phi_8540_: vec3<f32>;
    var phi_8575_: vec3<f32>;
    var phi_8680_: bool;
    var phi_8683_: bool;
    var phi_8684_: bool;
    var phi_2945_: u32;
    var phi_2954_: u32;
    var phi_9103_: bool;
    var phi_2982_: type_35;
    var phi_2985_: f32;
    var phi_2987_: f32;
    var phi_2999_: bool;
    var phi_3027_: type_36;
    var phi_3039_: type_35;
    var phi_2983_: type_35;
    var phi_3042_: type_36;
    var phi_3053_: type_35;
    var phi_3056_: f32;
    var phi_3058_: f32;
    var phi_3070_: bool;
    var phi_3098_: type_36;
    var phi_3110_: type_35;
    var phi_3054_: type_35;
    var phi_3113_: type_36;
    var phi_8706_: f32;
    var phi_8719_: bool;
    var phi_3179_: f32;
    var phi_3174_: f32;
    var phi_3180_: f32;
    var phi_8736_: bool;
    var phi_3145_: f32;
    var phi_3182_: f32;
    var phi_8754_: f32;
    var phi_8767_: bool;
    var phi_3235_: f32;
    var phi_3230_: f32;
    var phi_3236_: f32;
    var phi_8784_: bool;
    var phi_3201_: f32;
    var phi_3238_: f32;
    var phi_3057_: f32;
    var phi_3059_: f32;
    var phi_3299_: bool;
    var phi_9087_: bool;
    var phi_9149_: bool;
    var phi_2986_: f32;
    var phi_2988_: f32;
    var phi_3300_: bool;
    var phi_9148_: bool;
    var local_4: f32;
    var local_5: f32;
    var phi_9232_: bool;
    var phi_3303_: f32;
    var phi_9231_: bool;
    var phi_3304_: f32;
    var phi_9154_: bool;
    var phi_4874_: f32;
    var phi_4875_: vec3<f32>;
    var phi_4876_: bool;
    var phi_4896_: vec3<f32>;
    var phi_9151_: bool;
    var phi_2391_: vec3<f32>;
    var phi_4902_: bool;
    var phi_9150_: bool;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var local_8: vec3<f32>;
    var phi_9241_: bool;
    var phi_5019_: vec4<f32>;
    var phi_9233_: bool;
    var local_9: f32;
    var local_10: f32;
    var local_11: f32;
    var local_12: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e126 = arrayLength((&global.member));
            let _e128 = arrayLength((&global_1.member));
            let _e129 = global_2;
            let _e130 = global_3;
            let _e131 = global_4;
            let _e132 = global_5;
            let _e133 = global_6;
            let _e134 = global_7;
            let _e135 = global_8;
            let _e136 = global_9;
            let _e140 = global.member[(_e129 + 9u)];
            let _e144 = global.member[(_e129 + 11u)];
            let _e148 = global.member[(_e129 + 17u)];
            let _e151 = global.member[_e148];
            let _e155 = global.member[(_e148 + 1u)];
            let _e159 = global.member[(_e148 + 4u)];
            switch bitcast<i32>(_e159) {
                case 0: {
                    phi_704_ = 0u;
                    break;
                }
                case 1: {
                    phi_704_ = 1u;
                    break;
                }
                case 2: {
                    phi_704_ = 2u;
                    break;
                }
                case 3: {
                    phi_704_ = 3u;
                    break;
                }
                case 4: {
                    phi_704_ = 4u;
                    break;
                }
                case 5: {
                    phi_704_ = 5u;
                    break;
                }
                case 6: {
                    phi_704_ = 6u;
                    break;
                }
                case 7: {
                    phi_704_ = 7u;
                    break;
                }
                case 8: {
                    phi_704_ = 8u;
                    break;
                }
                case 9: {
                    phi_704_ = 9u;
                    break;
                }
                case 10: {
                    phi_704_ = 10u;
                    break;
                }
                case 11: {
                    phi_704_ = 11u;
                    break;
                }
                case 12: {
                    phi_704_ = 12u;
                    break;
                }
                case 13: {
                    phi_704_ = 13u;
                    break;
                }
                case 14: {
                    phi_704_ = 14u;
                    break;
                }
                case 15: {
                    phi_704_ = 15u;
                    break;
                }
                case 16: {
                    phi_704_ = 16u;
                    break;
                }
                case 17: {
                    phi_704_ = 17u;
                    break;
                }
                case 18: {
                    phi_704_ = 18u;
                    break;
                }
                case 19: {
                    phi_704_ = 19u;
                    break;
                }
                default: {
                    phi_704_ = 0u;
                    break;
                }
            }
            let _e162 = phi_704_;
            let _e166 = global.member[(_e148 + 5u)];
            if (_e144 == 4294967295u) {
                phi_836_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
            } else {
                if (_e126 >= 22u) {
                    phi_5724_ = (_e144 <= (_e126 - 22u));
                } else {
                    phi_5724_ = false;
                }
                let _e173 = phi_5724_;
                if _e173 {
                    let _e176 = global.member[_e144];
                    let _e181 = global.member[(_e144 + 1u)];
                    let _e186 = global.member[(_e144 + 2u)];
                    let _e192 = global.member[(_e144 + 3u)];
                    let _e197 = global.member[(_e144 + 4u)];
                    let _e202 = global.member[(_e144 + 5u)];
                    let _e207 = global.member[(_e144 + 6u)];
                    let _e212 = global.member[(_e144 + 7u)];
                    let _e218 = global.member[(_e144 + 8u)];
                    let _e223 = global.member[(_e144 + 9u)];
                    let _e228 = global.member[(_e144 + 10u)];
                    let _e232 = global.member[(_e144 + 11u)];
                    let _e236 = global.member[(_e144 + 12u)];
                    let _e240 = global.member[(_e144 + 13u)];
                    let _e244 = global.member[(_e144 + 14u)];
                    let _e248 = global.member[(_e144 + 15u)];
                    let _e252 = global.member[(_e144 + 16u)];
                    let _e256 = global.member[(_e144 + 17u)];
                    let _e260 = global.member[(_e144 + 18u)];
                    let _e264 = global.member[(_e144 + 19u)];
                    let _e268 = global.member[(_e144 + 20u)];
                    let _e273 = global.member[(_e144 + 21u)];
                    phi_832_ = type_33(vec3<f32>(bitcast<f32>(_e176), bitcast<f32>(_e181), bitcast<f32>(_e186)), bitcast<f32>(_e192), vec4<f32>(bitcast<f32>(_e197), bitcast<f32>(_e202), bitcast<f32>(_e207), bitcast<f32>(_e212)), bitcast<f32>(_e218), bitcast<f32>(_e223), _e228, _e232, _e236, _e240, _e244, _e248, _e252, _e256, _e260, _e264, (_e268 == 1u), bitcast<f32>(_e273));
                } else {
                    phi_832_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
                }
                let _e277 = phi_832_;
                phi_836_ = type_33(_e277.member, _e277.member_1, _e277.member_2, _e277.member_3, _e277.member_4, _e277.member_5, _e277.member_6, _e277.member_7, _e277.member_8, _e277.member_9, _e277.member_10, _e277.member_11, _e277.member_12, _e277.member_13, _e277.member_14, (_e277.member_15 && (_e166 == 1u)), _e277.member_16);
            }
            let _e299 = phi_836_;
            let _e303 = select(_e132, _e131, vec2((_e299.member_10 == 0u)));
            let _e305 = (_e126 >= 8u);
            if _e305 {
                phi_5761_ = (_e299.member_5 <= (_e126 - 8u));
            } else {
                phi_5761_ = false;
            }
            let _e309 = phi_5761_;
            if _e309 {
                let _e312 = global.member[_e299.member_5];
                let _e316 = global.member[(_e299.member_5 + 1u)];
                let _e321 = global.member[(_e299.member_5 + 2u)];
                let _e325 = global.member[(_e299.member_5 + 3u)];
                let _e330 = global.member[(_e299.member_5 + 4u)];
                let _e334 = global.member[(_e299.member_5 + 5u)];
                let _e338 = global.member[(_e299.member_5 + 6u)];
                switch bitcast<i32>(_e338) {
                    case 0: {
                        phi_876_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_876_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_876_ = 2u;
                        break;
                    }
                    default: {
                        phi_876_ = 0u;
                        break;
                    }
                }
                let _e341 = phi_876_;
                let _e345 = global.member[(_e299.member_5 + 7u)];
                switch bitcast<i32>(_e345) {
                    case 0: {
                        phi_885_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_885_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_885_ = 2u;
                        break;
                    }
                    default: {
                        phi_885_ = 0u;
                        break;
                    }
                }
                let _e348 = phi_885_;
                phi_898_ = type_15(type_14(_e341, _e348), vec2<u32>(_e312, _e316), vec2<u32>(_e321, _e325), _e330, _e334);
            } else {
                phi_898_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e352 = phi_898_;
            switch bitcast<i32>(_e352.member.member) {
                case 1: {
                    let _e390 = abs(_e303.x);
                    let _e392 = (_e390 % 1f);
                    if (_e390 >= 1f) {
                        phi_5813_ = select(true, false, (_e392 == 0f));
                    } else {
                        phi_5813_ = true;
                    }
                    let _e396 = phi_5813_;
                    let _e397 = select(1f, _e392, _e396);
                    if (select(-1f, 1f, (_e303.x >= 0f)) > 0f) {
                        phi_918_ = _e397;
                    } else {
                        phi_918_ = (1f - _e397);
                    }
                    let _e401 = phi_918_;
                    phi_955_ = _e401;
                    break;
                }
                case 2: {
                    let _e364 = abs(_e303.x);
                    let _e371 = ((select(select(u32(_e364), 0u, (_e364 < 0f)), 4294967295u, (_e364 > 4294967000f)) % 2u) == 0u);
                    let _e373 = (_e364 % 1f);
                    if (_e364 >= 1f) {
                        phi_5796_ = select(true, false, (_e373 == 0f));
                    } else {
                        phi_5796_ = true;
                    }
                    let _e377 = phi_5796_;
                    let _e378 = select(1f, _e373, _e377);
                    if (select(-1f, 1f, (_e303.x >= 0f)) > 0f) {
                        if _e371 {
                            phi_947_ = _e378;
                        } else {
                            phi_947_ = (1f - _e378);
                        }
                        let _e385 = phi_947_;
                        phi_953_ = _e385;
                    } else {
                        if _e371 {
                            phi_952_ = (1f - _e378);
                        } else {
                            phi_952_ = _e378;
                        }
                        let _e382 = phi_952_;
                        phi_953_ = _e382;
                    }
                    let _e387 = phi_953_;
                    phi_955_ = _e387;
                    break;
                }
                case 0: {
                    if (_e303.x > 1f) {
                        phi_5783_ = 0.9999999f;
                    } else {
                        phi_5783_ = select(_e303.x, 0.00000011920929f, (_e303.x < 0f));
                    }
                    let _e361 = phi_5783_;
                    phi_955_ = _e361;
                    break;
                }
                default: {
                    phi_955_ = f32();
                    break;
                }
            }
            let _e403 = phi_955_;
            switch bitcast<i32>(_e352.member.member_1) {
                case 1: {
                    let _e441 = abs(_e303.y);
                    let _e443 = (_e441 % 1f);
                    if (_e441 >= 1f) {
                        phi_5861_ = select(true, false, (_e443 == 0f));
                    } else {
                        phi_5861_ = true;
                    }
                    let _e447 = phi_5861_;
                    let _e448 = select(1f, _e443, _e447);
                    if (select(-1f, 1f, (_e303.y >= 0f)) > 0f) {
                        phi_976_ = _e448;
                    } else {
                        phi_976_ = (1f - _e448);
                    }
                    let _e452 = phi_976_;
                    phi_1013_ = _e452;
                    break;
                }
                case 2: {
                    let _e415 = abs(_e303.y);
                    let _e422 = ((select(select(u32(_e415), 0u, (_e415 < 0f)), 4294967295u, (_e415 > 4294967000f)) % 2u) == 0u);
                    let _e424 = (_e415 % 1f);
                    if (_e415 >= 1f) {
                        phi_5844_ = select(true, false, (_e424 == 0f));
                    } else {
                        phi_5844_ = true;
                    }
                    let _e428 = phi_5844_;
                    let _e429 = select(1f, _e424, _e428);
                    if (select(-1f, 1f, (_e303.y >= 0f)) > 0f) {
                        if _e422 {
                            phi_1005_ = _e429;
                        } else {
                            phi_1005_ = (1f - _e429);
                        }
                        let _e436 = phi_1005_;
                        phi_1011_ = _e436;
                    } else {
                        if _e422 {
                            phi_1010_ = (1f - _e429);
                        } else {
                            phi_1010_ = _e429;
                        }
                        let _e433 = phi_1010_;
                        phi_1011_ = _e433;
                    }
                    let _e438 = phi_1011_;
                    phi_1013_ = _e438;
                    break;
                }
                case 0: {
                    if (_e303.y > 1f) {
                        phi_5831_ = 0.9999999f;
                    } else {
                        phi_5831_ = select(_e303.y, 0.00000011920929f, (_e303.y < 0f));
                    }
                    let _e412 = phi_5831_;
                    phi_1013_ = _e412;
                    break;
                }
                default: {
                    phi_1013_ = f32();
                    break;
                }
            }
            let _e454 = phi_1013_;
            let _e458 = (_e403 * f32(_e352.member_2.x));
            let _e467 = (_e454 * f32(_e352.member_2.y));
            let _e479 = f32(_e151);
            let _e480 = f32(_e155);
            let _e487 = vec3<f32>((f32((select(select(u32(_e458), 0u, (_e458 < 0f)), 4294967295u, (_e458 > 4294967000f)) + _e352.member_1.x)) / _e479), (f32((select(select(u32(_e467), 0u, (_e467 < 0f)), 4294967295u, (_e467 > 4294967000f)) + _e352.member_1.y)) / _e480), f32(_e352.member_3));
            let _e493 = textureSampleLevel(global_11, global_10, vec2<f32>(_e487.x, _e487.y), i32(_e487.z), 0f);
            let _e496 = select(_e493, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e299.member_5 == 4294967295u)));
            let _e500 = select(_e132, _e131, vec2((_e299.member_11 == 0u)));
            if _e305 {
                phi_5897_ = (_e299.member_6 <= (_e126 - 8u));
            } else {
                phi_5897_ = false;
            }
            let _e505 = phi_5897_;
            if _e505 {
                let _e508 = global.member[_e299.member_6];
                let _e512 = global.member[(_e299.member_6 + 1u)];
                let _e517 = global.member[(_e299.member_6 + 2u)];
                let _e521 = global.member[(_e299.member_6 + 3u)];
                let _e526 = global.member[(_e299.member_6 + 4u)];
                let _e530 = global.member[(_e299.member_6 + 5u)];
                let _e534 = global.member[(_e299.member_6 + 6u)];
                switch bitcast<i32>(_e534) {
                    case 0: {
                        phi_1096_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1096_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1096_ = 2u;
                        break;
                    }
                    default: {
                        phi_1096_ = 0u;
                        break;
                    }
                }
                let _e537 = phi_1096_;
                let _e541 = global.member[(_e299.member_6 + 7u)];
                switch bitcast<i32>(_e541) {
                    case 0: {
                        phi_1105_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1105_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1105_ = 2u;
                        break;
                    }
                    default: {
                        phi_1105_ = 0u;
                        break;
                    }
                }
                let _e544 = phi_1105_;
                phi_1118_ = type_15(type_14(_e537, _e544), vec2<u32>(_e508, _e512), vec2<u32>(_e517, _e521), _e526, _e530);
            } else {
                phi_1118_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e548 = phi_1118_;
            switch bitcast<i32>(_e548.member.member) {
                case 1: {
                    let _e586 = abs(_e500.x);
                    let _e588 = (_e586 % 1f);
                    if (_e586 >= 1f) {
                        phi_5948_ = select(true, false, (_e588 == 0f));
                    } else {
                        phi_5948_ = true;
                    }
                    let _e592 = phi_5948_;
                    let _e593 = select(1f, _e588, _e592);
                    if (select(-1f, 1f, (_e500.x >= 0f)) > 0f) {
                        phi_1138_ = _e593;
                    } else {
                        phi_1138_ = (1f - _e593);
                    }
                    let _e597 = phi_1138_;
                    phi_1175_ = _e597;
                    break;
                }
                case 2: {
                    let _e560 = abs(_e500.x);
                    let _e567 = ((select(select(u32(_e560), 0u, (_e560 < 0f)), 4294967295u, (_e560 > 4294967000f)) % 2u) == 0u);
                    let _e569 = (_e560 % 1f);
                    if (_e560 >= 1f) {
                        phi_5931_ = select(true, false, (_e569 == 0f));
                    } else {
                        phi_5931_ = true;
                    }
                    let _e573 = phi_5931_;
                    let _e574 = select(1f, _e569, _e573);
                    if (select(-1f, 1f, (_e500.x >= 0f)) > 0f) {
                        if _e567 {
                            phi_1167_ = _e574;
                        } else {
                            phi_1167_ = (1f - _e574);
                        }
                        let _e581 = phi_1167_;
                        phi_1173_ = _e581;
                    } else {
                        if _e567 {
                            phi_1172_ = (1f - _e574);
                        } else {
                            phi_1172_ = _e574;
                        }
                        let _e578 = phi_1172_;
                        phi_1173_ = _e578;
                    }
                    let _e583 = phi_1173_;
                    phi_1175_ = _e583;
                    break;
                }
                case 0: {
                    if (_e500.x > 1f) {
                        phi_5918_ = 0.9999999f;
                    } else {
                        phi_5918_ = select(_e500.x, 0.00000011920929f, (_e500.x < 0f));
                    }
                    let _e557 = phi_5918_;
                    phi_1175_ = _e557;
                    break;
                }
                default: {
                    phi_1175_ = f32();
                    break;
                }
            }
            let _e599 = phi_1175_;
            switch bitcast<i32>(_e548.member.member_1) {
                case 1: {
                    let _e637 = abs(_e500.y);
                    let _e639 = (_e637 % 1f);
                    if (_e637 >= 1f) {
                        phi_5996_ = select(true, false, (_e639 == 0f));
                    } else {
                        phi_5996_ = true;
                    }
                    let _e643 = phi_5996_;
                    let _e644 = select(1f, _e639, _e643);
                    if (select(-1f, 1f, (_e500.y >= 0f)) > 0f) {
                        phi_1196_ = _e644;
                    } else {
                        phi_1196_ = (1f - _e644);
                    }
                    let _e648 = phi_1196_;
                    phi_1233_ = _e648;
                    break;
                }
                case 2: {
                    let _e611 = abs(_e500.y);
                    let _e618 = ((select(select(u32(_e611), 0u, (_e611 < 0f)), 4294967295u, (_e611 > 4294967000f)) % 2u) == 0u);
                    let _e620 = (_e611 % 1f);
                    if (_e611 >= 1f) {
                        phi_5979_ = select(true, false, (_e620 == 0f));
                    } else {
                        phi_5979_ = true;
                    }
                    let _e624 = phi_5979_;
                    let _e625 = select(1f, _e620, _e624);
                    if (select(-1f, 1f, (_e500.y >= 0f)) > 0f) {
                        if _e618 {
                            phi_1225_ = _e625;
                        } else {
                            phi_1225_ = (1f - _e625);
                        }
                        let _e632 = phi_1225_;
                        phi_1231_ = _e632;
                    } else {
                        if _e618 {
                            phi_1230_ = (1f - _e625);
                        } else {
                            phi_1230_ = _e625;
                        }
                        let _e629 = phi_1230_;
                        phi_1231_ = _e629;
                    }
                    let _e634 = phi_1231_;
                    phi_1233_ = _e634;
                    break;
                }
                case 0: {
                    if (_e500.y > 1f) {
                        phi_5966_ = 0.9999999f;
                    } else {
                        phi_5966_ = select(_e500.y, 0.00000011920929f, (_e500.y < 0f));
                    }
                    let _e608 = phi_5966_;
                    phi_1233_ = _e608;
                    break;
                }
                default: {
                    phi_1233_ = f32();
                    break;
                }
            }
            let _e650 = phi_1233_;
            let _e654 = (_e599 * f32(_e548.member_2.x));
            let _e663 = (_e650 * f32(_e548.member_2.y));
            let _e681 = vec3<f32>((f32((select(select(u32(_e654), 0u, (_e654 < 0f)), 4294967295u, (_e654 > 4294967000f)) + _e548.member_1.x)) / _e479), (f32((select(select(u32(_e663), 0u, (_e663 < 0f)), 4294967295u, (_e663 > 4294967000f)) + _e548.member_1.y)) / _e480), f32(_e548.member_3));
            let _e687 = textureSampleLevel(global_11, global_10, vec2<f32>(_e681.x, _e681.y), i32(_e681.z), 0f);
            let _e690 = select(_e687, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e299.member_6 == 4294967295u)));
            let _e694 = select(_e132, _e131, vec2((_e299.member_12 == 0u)));
            if _e305 {
                phi_6032_ = (_e299.member_7 <= (_e126 - 8u));
            } else {
                phi_6032_ = false;
            }
            let _e699 = phi_6032_;
            if _e699 {
                let _e702 = global.member[_e299.member_7];
                let _e706 = global.member[(_e299.member_7 + 1u)];
                let _e711 = global.member[(_e299.member_7 + 2u)];
                let _e715 = global.member[(_e299.member_7 + 3u)];
                let _e720 = global.member[(_e299.member_7 + 4u)];
                let _e724 = global.member[(_e299.member_7 + 5u)];
                let _e728 = global.member[(_e299.member_7 + 6u)];
                switch bitcast<i32>(_e728) {
                    case 0: {
                        phi_1316_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1316_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1316_ = 2u;
                        break;
                    }
                    default: {
                        phi_1316_ = 0u;
                        break;
                    }
                }
                let _e731 = phi_1316_;
                let _e735 = global.member[(_e299.member_7 + 7u)];
                switch bitcast<i32>(_e735) {
                    case 0: {
                        phi_1325_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1325_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1325_ = 2u;
                        break;
                    }
                    default: {
                        phi_1325_ = 0u;
                        break;
                    }
                }
                let _e738 = phi_1325_;
                phi_1338_ = type_15(type_14(_e731, _e738), vec2<u32>(_e702, _e706), vec2<u32>(_e711, _e715), _e720, _e724);
            } else {
                phi_1338_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e742 = phi_1338_;
            switch bitcast<i32>(_e742.member.member) {
                case 1: {
                    let _e780 = abs(_e694.x);
                    let _e782 = (_e780 % 1f);
                    if (_e780 >= 1f) {
                        phi_6083_ = select(true, false, (_e782 == 0f));
                    } else {
                        phi_6083_ = true;
                    }
                    let _e786 = phi_6083_;
                    let _e787 = select(1f, _e782, _e786);
                    if (select(-1f, 1f, (_e694.x >= 0f)) > 0f) {
                        phi_1358_ = _e787;
                    } else {
                        phi_1358_ = (1f - _e787);
                    }
                    let _e791 = phi_1358_;
                    phi_1395_ = _e791;
                    break;
                }
                case 2: {
                    let _e754 = abs(_e694.x);
                    let _e761 = ((select(select(u32(_e754), 0u, (_e754 < 0f)), 4294967295u, (_e754 > 4294967000f)) % 2u) == 0u);
                    let _e763 = (_e754 % 1f);
                    if (_e754 >= 1f) {
                        phi_6066_ = select(true, false, (_e763 == 0f));
                    } else {
                        phi_6066_ = true;
                    }
                    let _e767 = phi_6066_;
                    let _e768 = select(1f, _e763, _e767);
                    if (select(-1f, 1f, (_e694.x >= 0f)) > 0f) {
                        if _e761 {
                            phi_1387_ = _e768;
                        } else {
                            phi_1387_ = (1f - _e768);
                        }
                        let _e775 = phi_1387_;
                        phi_1393_ = _e775;
                    } else {
                        if _e761 {
                            phi_1392_ = (1f - _e768);
                        } else {
                            phi_1392_ = _e768;
                        }
                        let _e772 = phi_1392_;
                        phi_1393_ = _e772;
                    }
                    let _e777 = phi_1393_;
                    phi_1395_ = _e777;
                    break;
                }
                case 0: {
                    if (_e694.x > 1f) {
                        phi_6053_ = 0.9999999f;
                    } else {
                        phi_6053_ = select(_e694.x, 0.00000011920929f, (_e694.x < 0f));
                    }
                    let _e751 = phi_6053_;
                    phi_1395_ = _e751;
                    break;
                }
                default: {
                    phi_1395_ = f32();
                    break;
                }
            }
            let _e793 = phi_1395_;
            switch bitcast<i32>(_e742.member.member_1) {
                case 1: {
                    let _e831 = abs(_e694.y);
                    let _e833 = (_e831 % 1f);
                    if (_e831 >= 1f) {
                        phi_6131_ = select(true, false, (_e833 == 0f));
                    } else {
                        phi_6131_ = true;
                    }
                    let _e837 = phi_6131_;
                    let _e838 = select(1f, _e833, _e837);
                    if (select(-1f, 1f, (_e694.y >= 0f)) > 0f) {
                        phi_1416_ = _e838;
                    } else {
                        phi_1416_ = (1f - _e838);
                    }
                    let _e842 = phi_1416_;
                    phi_1453_ = _e842;
                    break;
                }
                case 2: {
                    let _e805 = abs(_e694.y);
                    let _e812 = ((select(select(u32(_e805), 0u, (_e805 < 0f)), 4294967295u, (_e805 > 4294967000f)) % 2u) == 0u);
                    let _e814 = (_e805 % 1f);
                    if (_e805 >= 1f) {
                        phi_6114_ = select(true, false, (_e814 == 0f));
                    } else {
                        phi_6114_ = true;
                    }
                    let _e818 = phi_6114_;
                    let _e819 = select(1f, _e814, _e818);
                    if (select(-1f, 1f, (_e694.y >= 0f)) > 0f) {
                        if _e812 {
                            phi_1445_ = _e819;
                        } else {
                            phi_1445_ = (1f - _e819);
                        }
                        let _e826 = phi_1445_;
                        phi_1451_ = _e826;
                    } else {
                        if _e812 {
                            phi_1450_ = (1f - _e819);
                        } else {
                            phi_1450_ = _e819;
                        }
                        let _e823 = phi_1450_;
                        phi_1451_ = _e823;
                    }
                    let _e828 = phi_1451_;
                    phi_1453_ = _e828;
                    break;
                }
                case 0: {
                    if (_e694.y > 1f) {
                        phi_6101_ = 0.9999999f;
                    } else {
                        phi_6101_ = select(_e694.y, 0.00000011920929f, (_e694.y < 0f));
                    }
                    let _e802 = phi_6101_;
                    phi_1453_ = _e802;
                    break;
                }
                default: {
                    phi_1453_ = f32();
                    break;
                }
            }
            let _e844 = phi_1453_;
            let _e848 = (_e793 * f32(_e742.member_2.x));
            let _e857 = (_e844 * f32(_e742.member_2.y));
            let _e875 = vec3<f32>((f32((select(select(u32(_e848), 0u, (_e848 < 0f)), 4294967295u, (_e848 > 4294967000f)) + _e742.member_1.x)) / _e479), (f32((select(select(u32(_e857), 0u, (_e857 < 0f)), 4294967295u, (_e857 > 4294967000f)) + _e742.member_1.y)) / _e480), f32(_e742.member_3));
            let _e881 = textureSampleLevel(global_11, global_10, vec2<f32>(_e875.x, _e875.y), i32(_e875.z), 0f);
            let _e882 = (_e299.member_7 == 4294967295u);
            let _e884 = select(_e881, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e882));
            let _e888 = select(_e132, _e131, vec2((_e299.member_13 == 0u)));
            if _e305 {
                phi_6167_ = (_e299.member_8 <= (_e126 - 8u));
            } else {
                phi_6167_ = false;
            }
            let _e893 = phi_6167_;
            if _e893 {
                let _e896 = global.member[_e299.member_8];
                let _e900 = global.member[(_e299.member_8 + 1u)];
                let _e905 = global.member[(_e299.member_8 + 2u)];
                let _e909 = global.member[(_e299.member_8 + 3u)];
                let _e914 = global.member[(_e299.member_8 + 4u)];
                let _e918 = global.member[(_e299.member_8 + 5u)];
                let _e922 = global.member[(_e299.member_8 + 6u)];
                switch bitcast<i32>(_e922) {
                    case 0: {
                        phi_1536_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1536_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1536_ = 2u;
                        break;
                    }
                    default: {
                        phi_1536_ = 0u;
                        break;
                    }
                }
                let _e925 = phi_1536_;
                let _e929 = global.member[(_e299.member_8 + 7u)];
                switch bitcast<i32>(_e929) {
                    case 0: {
                        phi_1545_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1545_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1545_ = 2u;
                        break;
                    }
                    default: {
                        phi_1545_ = 0u;
                        break;
                    }
                }
                let _e932 = phi_1545_;
                phi_1558_ = type_15(type_14(_e925, _e932), vec2<u32>(_e896, _e900), vec2<u32>(_e905, _e909), _e914, _e918);
            } else {
                phi_1558_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e936 = phi_1558_;
            switch bitcast<i32>(_e936.member.member) {
                case 1: {
                    let _e974 = abs(_e888.x);
                    let _e976 = (_e974 % 1f);
                    if (_e974 >= 1f) {
                        phi_6218_ = select(true, false, (_e976 == 0f));
                    } else {
                        phi_6218_ = true;
                    }
                    let _e980 = phi_6218_;
                    let _e981 = select(1f, _e976, _e980);
                    if (select(-1f, 1f, (_e888.x >= 0f)) > 0f) {
                        phi_1578_ = _e981;
                    } else {
                        phi_1578_ = (1f - _e981);
                    }
                    let _e985 = phi_1578_;
                    phi_1615_ = _e985;
                    break;
                }
                case 2: {
                    let _e948 = abs(_e888.x);
                    let _e955 = ((select(select(u32(_e948), 0u, (_e948 < 0f)), 4294967295u, (_e948 > 4294967000f)) % 2u) == 0u);
                    let _e957 = (_e948 % 1f);
                    if (_e948 >= 1f) {
                        phi_6201_ = select(true, false, (_e957 == 0f));
                    } else {
                        phi_6201_ = true;
                    }
                    let _e961 = phi_6201_;
                    let _e962 = select(1f, _e957, _e961);
                    if (select(-1f, 1f, (_e888.x >= 0f)) > 0f) {
                        if _e955 {
                            phi_1607_ = _e962;
                        } else {
                            phi_1607_ = (1f - _e962);
                        }
                        let _e969 = phi_1607_;
                        phi_1613_ = _e969;
                    } else {
                        if _e955 {
                            phi_1612_ = (1f - _e962);
                        } else {
                            phi_1612_ = _e962;
                        }
                        let _e966 = phi_1612_;
                        phi_1613_ = _e966;
                    }
                    let _e971 = phi_1613_;
                    phi_1615_ = _e971;
                    break;
                }
                case 0: {
                    if (_e888.x > 1f) {
                        phi_6188_ = 0.9999999f;
                    } else {
                        phi_6188_ = select(_e888.x, 0.00000011920929f, (_e888.x < 0f));
                    }
                    let _e945 = phi_6188_;
                    phi_1615_ = _e945;
                    break;
                }
                default: {
                    phi_1615_ = f32();
                    break;
                }
            }
            let _e987 = phi_1615_;
            switch bitcast<i32>(_e936.member.member_1) {
                case 1: {
                    let _e1025 = abs(_e888.y);
                    let _e1027 = (_e1025 % 1f);
                    if (_e1025 >= 1f) {
                        phi_6266_ = select(true, false, (_e1027 == 0f));
                    } else {
                        phi_6266_ = true;
                    }
                    let _e1031 = phi_6266_;
                    let _e1032 = select(1f, _e1027, _e1031);
                    if (select(-1f, 1f, (_e888.y >= 0f)) > 0f) {
                        phi_1636_ = _e1032;
                    } else {
                        phi_1636_ = (1f - _e1032);
                    }
                    let _e1036 = phi_1636_;
                    phi_1673_ = _e1036;
                    break;
                }
                case 2: {
                    let _e999 = abs(_e888.y);
                    let _e1006 = ((select(select(u32(_e999), 0u, (_e999 < 0f)), 4294967295u, (_e999 > 4294967000f)) % 2u) == 0u);
                    let _e1008 = (_e999 % 1f);
                    if (_e999 >= 1f) {
                        phi_6249_ = select(true, false, (_e1008 == 0f));
                    } else {
                        phi_6249_ = true;
                    }
                    let _e1012 = phi_6249_;
                    let _e1013 = select(1f, _e1008, _e1012);
                    if (select(-1f, 1f, (_e888.y >= 0f)) > 0f) {
                        if _e1006 {
                            phi_1665_ = _e1013;
                        } else {
                            phi_1665_ = (1f - _e1013);
                        }
                        let _e1020 = phi_1665_;
                        phi_1671_ = _e1020;
                    } else {
                        if _e1006 {
                            phi_1670_ = (1f - _e1013);
                        } else {
                            phi_1670_ = _e1013;
                        }
                        let _e1017 = phi_1670_;
                        phi_1671_ = _e1017;
                    }
                    let _e1022 = phi_1671_;
                    phi_1673_ = _e1022;
                    break;
                }
                case 0: {
                    if (_e888.y > 1f) {
                        phi_6236_ = 0.9999999f;
                    } else {
                        phi_6236_ = select(_e888.y, 0.00000011920929f, (_e888.y < 0f));
                    }
                    let _e996 = phi_6236_;
                    phi_1673_ = _e996;
                    break;
                }
                default: {
                    phi_1673_ = f32();
                    break;
                }
            }
            let _e1038 = phi_1673_;
            let _e1042 = (_e987 * f32(_e936.member_2.x));
            let _e1051 = (_e1038 * f32(_e936.member_2.y));
            let _e1069 = vec3<f32>((f32((select(select(u32(_e1042), 0u, (_e1042 < 0f)), 4294967295u, (_e1042 > 4294967000f)) + _e936.member_1.x)) / _e479), (f32((select(select(u32(_e1051), 0u, (_e1051 < 0f)), 4294967295u, (_e1051 > 4294967000f)) + _e936.member_1.y)) / _e480), f32(_e936.member_3));
            let _e1075 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1069.x, _e1069.y), i32(_e1069.z), 0f);
            let _e1082 = select(_e132, _e131, vec2((_e299.member_14 == 0u)));
            if _e305 {
                phi_6302_ = (_e299.member_9 <= (_e126 - 8u));
            } else {
                phi_6302_ = false;
            }
            let _e1087 = phi_6302_;
            if _e1087 {
                let _e1090 = global.member[_e299.member_9];
                let _e1094 = global.member[(_e299.member_9 + 1u)];
                let _e1099 = global.member[(_e299.member_9 + 2u)];
                let _e1103 = global.member[(_e299.member_9 + 3u)];
                let _e1108 = global.member[(_e299.member_9 + 4u)];
                let _e1112 = global.member[(_e299.member_9 + 5u)];
                let _e1116 = global.member[(_e299.member_9 + 6u)];
                switch bitcast<i32>(_e1116) {
                    case 0: {
                        phi_1756_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1756_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1756_ = 2u;
                        break;
                    }
                    default: {
                        phi_1756_ = 0u;
                        break;
                    }
                }
                let _e1119 = phi_1756_;
                let _e1123 = global.member[(_e299.member_9 + 7u)];
                switch bitcast<i32>(_e1123) {
                    case 0: {
                        phi_1765_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1765_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1765_ = 2u;
                        break;
                    }
                    default: {
                        phi_1765_ = 0u;
                        break;
                    }
                }
                let _e1126 = phi_1765_;
                phi_1778_ = type_15(type_14(_e1119, _e1126), vec2<u32>(_e1090, _e1094), vec2<u32>(_e1099, _e1103), _e1108, _e1112);
            } else {
                phi_1778_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e1130 = phi_1778_;
            switch bitcast<i32>(_e1130.member.member) {
                case 1: {
                    let _e1168 = abs(_e1082.x);
                    let _e1170 = (_e1168 % 1f);
                    if (_e1168 >= 1f) {
                        phi_6353_ = select(true, false, (_e1170 == 0f));
                    } else {
                        phi_6353_ = true;
                    }
                    let _e1174 = phi_6353_;
                    let _e1175 = select(1f, _e1170, _e1174);
                    if (select(-1f, 1f, (_e1082.x >= 0f)) > 0f) {
                        phi_1798_ = _e1175;
                    } else {
                        phi_1798_ = (1f - _e1175);
                    }
                    let _e1179 = phi_1798_;
                    phi_1835_ = _e1179;
                    break;
                }
                case 2: {
                    let _e1142 = abs(_e1082.x);
                    let _e1149 = ((select(select(u32(_e1142), 0u, (_e1142 < 0f)), 4294967295u, (_e1142 > 4294967000f)) % 2u) == 0u);
                    let _e1151 = (_e1142 % 1f);
                    if (_e1142 >= 1f) {
                        phi_6336_ = select(true, false, (_e1151 == 0f));
                    } else {
                        phi_6336_ = true;
                    }
                    let _e1155 = phi_6336_;
                    let _e1156 = select(1f, _e1151, _e1155);
                    if (select(-1f, 1f, (_e1082.x >= 0f)) > 0f) {
                        if _e1149 {
                            phi_1827_ = _e1156;
                        } else {
                            phi_1827_ = (1f - _e1156);
                        }
                        let _e1163 = phi_1827_;
                        phi_1833_ = _e1163;
                    } else {
                        if _e1149 {
                            phi_1832_ = (1f - _e1156);
                        } else {
                            phi_1832_ = _e1156;
                        }
                        let _e1160 = phi_1832_;
                        phi_1833_ = _e1160;
                    }
                    let _e1165 = phi_1833_;
                    phi_1835_ = _e1165;
                    break;
                }
                case 0: {
                    if (_e1082.x > 1f) {
                        phi_6323_ = 0.9999999f;
                    } else {
                        phi_6323_ = select(_e1082.x, 0.00000011920929f, (_e1082.x < 0f));
                    }
                    let _e1139 = phi_6323_;
                    phi_1835_ = _e1139;
                    break;
                }
                default: {
                    phi_1835_ = f32();
                    break;
                }
            }
            let _e1181 = phi_1835_;
            switch bitcast<i32>(_e1130.member.member_1) {
                case 1: {
                    let _e1219 = abs(_e1082.y);
                    let _e1221 = (_e1219 % 1f);
                    if (_e1219 >= 1f) {
                        phi_6401_ = select(true, false, (_e1221 == 0f));
                    } else {
                        phi_6401_ = true;
                    }
                    let _e1225 = phi_6401_;
                    let _e1226 = select(1f, _e1221, _e1225);
                    if (select(-1f, 1f, (_e1082.y >= 0f)) > 0f) {
                        phi_1856_ = _e1226;
                    } else {
                        phi_1856_ = (1f - _e1226);
                    }
                    let _e1230 = phi_1856_;
                    phi_1893_ = _e1230;
                    break;
                }
                case 2: {
                    let _e1193 = abs(_e1082.y);
                    let _e1200 = ((select(select(u32(_e1193), 0u, (_e1193 < 0f)), 4294967295u, (_e1193 > 4294967000f)) % 2u) == 0u);
                    let _e1202 = (_e1193 % 1f);
                    if (_e1193 >= 1f) {
                        phi_6384_ = select(true, false, (_e1202 == 0f));
                    } else {
                        phi_6384_ = true;
                    }
                    let _e1206 = phi_6384_;
                    let _e1207 = select(1f, _e1202, _e1206);
                    if (select(-1f, 1f, (_e1082.y >= 0f)) > 0f) {
                        if _e1200 {
                            phi_1885_ = _e1207;
                        } else {
                            phi_1885_ = (1f - _e1207);
                        }
                        let _e1214 = phi_1885_;
                        phi_1891_ = _e1214;
                    } else {
                        if _e1200 {
                            phi_1890_ = (1f - _e1207);
                        } else {
                            phi_1890_ = _e1207;
                        }
                        let _e1211 = phi_1890_;
                        phi_1891_ = _e1211;
                    }
                    let _e1216 = phi_1891_;
                    phi_1893_ = _e1216;
                    break;
                }
                case 0: {
                    if (_e1082.y > 1f) {
                        phi_6371_ = 0.9999999f;
                    } else {
                        phi_6371_ = select(_e1082.y, 0.00000011920929f, (_e1082.y < 0f));
                    }
                    let _e1190 = phi_6371_;
                    phi_1893_ = _e1190;
                    break;
                }
                default: {
                    phi_1893_ = f32();
                    break;
                }
            }
            let _e1232 = phi_1893_;
            let _e1236 = (_e1181 * f32(_e1130.member_2.x));
            let _e1245 = (_e1232 * f32(_e1130.member_2.y));
            let _e1263 = vec3<f32>((f32((select(select(u32(_e1236), 0u, (_e1236 < 0f)), 4294967295u, (_e1236 > 4294967000f)) + _e1130.member_1.x)) / _e479), (f32((select(select(u32(_e1245), 0u, (_e1245 < 0f)), 4294967295u, (_e1245 > 4294967000f)) + _e1130.member_1.y)) / _e480), f32(_e1130.member_3));
            let _e1269 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1263.x, _e1263.y), i32(_e1263.z), 0f);
            let _e1272 = select(_e1269, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e299.member_9 == 4294967295u)));
            if _e882 {
                phi_1987_ = vec3<f32>(0f, 0f, 0f);
                phi_1988_ = _e133;
            } else {
                let _e1276 = fma(_e884.x, 2f, -1f);
                let _e1277 = fma(_e884.y, 2f, -1f);
                let _e1278 = fma(_e884.z, 2f, -1f);
                let _e1283 = sqrt(fma(_e1278, _e1278, fma(_e1276, _e1276, (_e1277 * _e1277))));
                if (_e1283 == 0f) {
                    phi_6459_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6459_ = (vec3<f32>(_e1276, _e1277, _e1278) * (1f / _e1283));
                }
                let _e1288 = phi_6459_;
                let _e1295 = sqrt(fma(_e134.z, _e134.z, fma(_e134.x, _e134.x, (_e134.y * _e134.y))));
                if (_e1295 == 0f) {
                    phi_6494_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6494_ = (_e134 * (1f / _e1295));
                }
                let _e1300 = phi_6494_;
                let _e1307 = sqrt(fma(_e135.z, _e135.z, fma(_e135.x, _e135.x, (_e135.y * _e135.y))));
                if (_e1307 == 0f) {
                    phi_6529_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6529_ = (_e135 * (1f / _e1307));
                }
                let _e1312 = phi_6529_;
                let _e1319 = sqrt(fma(_e133.z, _e133.z, fma(_e133.x, _e133.x, (_e133.y * _e133.y))));
                if (_e1319 == 0f) {
                    phi_6564_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6564_ = (_e133 * (1f / _e1319));
                }
                let _e1324 = phi_6564_;
                let _e1343 = fma(_e1324.x, _e1288.z, fma(_e1300.x, _e1288.x, (_e1312.x * _e1288.y)));
                let _e1344 = fma(_e1324.y, _e1288.z, fma(_e1300.y, _e1288.x, (_e1312.y * _e1288.y)));
                let _e1345 = fma(_e1324.z, _e1288.z, fma(_e1300.z, _e1288.x, (_e1312.z * _e1288.y)));
                let _e1350 = sqrt(fma(_e1345, _e1345, fma(_e1343, _e1343, (_e1344 * _e1344))));
                if (_e1350 == 0f) {
                    phi_6599_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6599_ = (vec3<f32>(_e1343, _e1344, _e1345) * (1f / _e1350));
                }
                let _e1355 = phi_6599_;
                phi_1987_ = _e1288;
                phi_1988_ = _e1355;
            }
            let _e1357 = phi_1987_;
            let _e1359 = phi_1988_;
            let _e1363 = (_e496.x * _e299.member_2.x);
            let _e1366 = (_e496.y * _e299.member_2.y);
            let _e1369 = (_e496.z * _e299.member_2.z);
            let _e1374 = (_e1363 * _e130.x);
            let _e1376 = (_e1366 * _e130.y);
            let _e1378 = (_e1369 * _e130.z);
            let _e1383 = (_e690.y * _e299.member_4);
            let _e1386 = (_e690.z * _e299.member_3);
            let _e1390 = fma(_e299.member_16, (select(_e1075, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e299.member_8 == 4294967295u))).x - 1f), 1f);
            let _e1396 = (_e1272.x * _e299.member.x);
            let _e1398 = (_e1272.y * _e299.member.y);
            let _e1400 = (_e1272.z * _e299.member.z);
            let _e1405 = textureSampleLevel(global_12, global_13, _e1359, 0f);
            if (_e126 >= 86u) {
                phi_6631_ = (_e140 <= (_e126 - 86u));
            } else {
                phi_6631_ = false;
            }
            let _e1413 = phi_6631_;
            if _e1413 {
                let _e1416 = global.member[_e140];
                let _e1421 = global.member[(_e140 + 1u)];
                let _e1426 = global.member[(_e140 + 2u)];
                let _e1431 = global.member[(_e140 + 3u)];
                let _e1437 = global.member[(_e140 + 4u)];
                let _e1442 = global.member[(_e140 + 5u)];
                let _e1447 = global.member[(_e140 + 6u)];
                let _e1452 = global.member[(_e140 + 7u)];
                let _e1458 = global.member[(_e140 + 8u)];
                let _e1463 = global.member[(_e140 + 9u)];
                let _e1468 = global.member[(_e140 + 10u)];
                let _e1473 = global.member[(_e140 + 11u)];
                let _e1479 = global.member[(_e140 + 12u)];
                let _e1484 = global.member[(_e140 + 13u)];
                let _e1489 = global.member[(_e140 + 14u)];
                let _e1494 = global.member[(_e140 + 15u)];
                let _e1501 = global.member[(_e140 + 16u)];
                let _e1506 = global.member[(_e140 + 17u)];
                let _e1511 = global.member[(_e140 + 18u)];
                let _e1516 = global.member[(_e140 + 19u)];
                let _e1522 = global.member[(_e140 + 20u)];
                let _e1527 = global.member[(_e140 + 21u)];
                let _e1532 = global.member[(_e140 + 22u)];
                let _e1537 = global.member[(_e140 + 23u)];
                let _e1543 = global.member[(_e140 + 24u)];
                let _e1548 = global.member[(_e140 + 25u)];
                let _e1553 = global.member[(_e140 + 26u)];
                let _e1558 = global.member[(_e140 + 27u)];
                let _e1564 = global.member[(_e140 + 28u)];
                let _e1569 = global.member[(_e140 + 29u)];
                let _e1574 = global.member[(_e140 + 30u)];
                let _e1579 = global.member[(_e140 + 31u)];
                let _e1586 = global.member[(_e140 + 32u)];
                let _e1591 = global.member[(_e140 + 33u)];
                let _e1596 = global.member[(_e140 + 34u)];
                local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_2195_ = type_14(0u, 6u);
                loop {
                    let _e1601 = phi_2195_;
                    if (_e1601.member < _e1601.member_1) {
                        phi_2196_ = type_14((_e1601.member + 1u), _e1601.member_1);
                        phi_2219_ = type_14(1u, _e1601.member);
                    } else {
                        phi_2196_ = _e1601;
                        phi_2219_ = type_14(0u, type_14().member_1);
                    }
                    let _e1614 = phi_2196_;
                    let _e1616 = phi_2219_;
                    switch bitcast<i32>(_e1616.member) {
                        case 0: {
                            phi_2246_ = false;
                            break;
                        }
                        case 1: {
                            let _e1621 = ((_e140 + 35u) + (_e1616.member_1 * 4u));
                            let _e1624 = global.member[_e1621];
                            let _e1629 = global.member[(_e1621 + 1u)];
                            let _e1634 = global.member[(_e1621 + 2u)];
                            let _e1639 = global.member[(_e1621 + 3u)];
                            local_1[_e1616.member_1] = vec4<f32>(bitcast<f32>(_e1624), bitcast<f32>(_e1629), bitcast<f32>(_e1634), bitcast<f32>(_e1639));
                            phi_2246_ = true;
                            break;
                        }
                        default: {
                            phi_2246_ = bool();
                            break;
                        }
                    }
                    let _e1644 = phi_2246_;
                    continue;
                    continuing {
                        phi_2195_ = _e1614;
                        break if !(_e1644);
                    }
                }
                let _e1646 = local_1;
                local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                phi_2252_ = type_14(0u, 8u);
                loop {
                    let _e1649 = phi_2252_;
                    if (_e1649.member < _e1649.member_1) {
                        phi_2253_ = type_14((_e1649.member + 1u), _e1649.member_1);
                        phi_2276_ = type_14(1u, _e1649.member);
                    } else {
                        phi_2253_ = _e1649;
                        phi_2276_ = type_14(0u, type_14().member_1);
                    }
                    let _e1662 = phi_2253_;
                    let _e1664 = phi_2276_;
                    switch bitcast<i32>(_e1664.member) {
                        case 0: {
                            phi_2299_ = false;
                            break;
                        }
                        case 1: {
                            let _e1669 = ((_e140 + 59u) + (_e1664.member_1 * 3u));
                            let _e1672 = global.member[_e1669];
                            let _e1677 = global.member[(_e1669 + 1u)];
                            let _e1682 = global.member[(_e1669 + 2u)];
                            local[_e1664.member_1] = vec3<f32>(bitcast<f32>(_e1672), bitcast<f32>(_e1677), bitcast<f32>(_e1682));
                            phi_2299_ = true;
                            break;
                        }
                        default: {
                            phi_2299_ = bool();
                            break;
                        }
                    }
                    let _e1687 = phi_2299_;
                    continue;
                    continuing {
                        phi_2252_ = _e1662;
                        break if !(_e1687);
                    }
                }
                let _e1689 = local;
                let _e1693 = global.member[(_e140 + 83u)];
                let _e1698 = global.member[(_e140 + 84u)];
                let _e1703 = global.member[(_e140 + 85u)];
                phi_2320_ = type_25(type_23(vec4<f32>(bitcast<f32>(_e1416), bitcast<f32>(_e1421), bitcast<f32>(_e1426), bitcast<f32>(_e1431)), vec4<f32>(bitcast<f32>(_e1437), bitcast<f32>(_e1442), bitcast<f32>(_e1447), bitcast<f32>(_e1452)), vec4<f32>(bitcast<f32>(_e1458), bitcast<f32>(_e1463), bitcast<f32>(_e1468), bitcast<f32>(_e1473)), vec4<f32>(bitcast<f32>(_e1479), bitcast<f32>(_e1484), bitcast<f32>(_e1489), bitcast<f32>(_e1494))), type_23(vec4<f32>(bitcast<f32>(_e1501), bitcast<f32>(_e1506), bitcast<f32>(_e1511), bitcast<f32>(_e1516)), vec4<f32>(bitcast<f32>(_e1522), bitcast<f32>(_e1527), bitcast<f32>(_e1532), bitcast<f32>(_e1537)), vec4<f32>(bitcast<f32>(_e1543), bitcast<f32>(_e1548), bitcast<f32>(_e1553), bitcast<f32>(_e1558)), vec4<f32>(bitcast<f32>(_e1564), bitcast<f32>(_e1569), bitcast<f32>(_e1574), bitcast<f32>(_e1579))), vec3<f32>(bitcast<f32>(_e1586), bitcast<f32>(_e1591), bitcast<f32>(_e1596)), type_24(_e1689, _e1646, vec3<f32>(bitcast<f32>(_e1693), bitcast<f32>(_e1698), bitcast<f32>(_e1703))));
            } else {
                phi_2320_ = type_25(type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_24(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
            }
            let _e1709 = phi_2320_;
            let _e1711 = (_e1709.member_2 - _e136);
            let _e1718 = sqrt(fma(_e1711.z, _e1711.z, fma(_e1711.x, _e1711.x, (_e1711.y * _e1711.y))));
            let _e1719 = (_e1718 == 0f);
            if _e1719 {
                phi_6703_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6703_ = (_e1711 * (1f / _e1718));
            }
            let _e1723 = phi_6703_;
            let _e1724 = -(_e1723);
            let _e1731 = sqrt(fma(_e1359.z, _e1359.z, fma(_e1359.x, _e1359.x, (_e1359.y * _e1359.y))));
            let _e1732 = (_e1731 == 0f);
            if _e1732 {
                phi_6762_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6762_ = (_e1359 * (1f / _e1731));
            }
            let _e1736 = phi_6762_;
            let _e1746 = (2f * fma(_e1736.z, _e1724.z, fma(_e1736.x, _e1724.x, (_e1736.y * _e1724.y))));
            let _e1753 = textureSampleLevel(global_14, global_15, (_e1724 - vec3<f32>((_e1746 * _e1736.x), (_e1746 * _e1736.y), (_e1746 * _e1736.z))), (_e1383 * 4f));
            if _e1719 {
                phi_6836_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6836_ = (_e1711 * (1f / _e1718));
            }
            let _e1760 = phi_6836_;
            let _e1769 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1359.z, _e1760.z, fma(_e1359.x, _e1760.x, (_e1359.y * _e1760.y))), 0f), _e1383), 0f);
            switch bitcast<i32>(_e162) {
                case 0: {
                    if _e299.member_15 {
                        if _e1732 {
                            phi_7229_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_7229_ = (_e1359 * (1f / _e1731));
                        }
                        let _e1938 = phi_7229_;
                        if _e1719 {
                            phi_7264_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_7264_ = (_e1711 * (1f / _e1718));
                        }
                        let _e1942 = phi_7264_;
                        let _e1945 = global_1.member[0u];
                        let _e1948 = global_1.member[1u];
                        let _e1951 = global_1.member[2u];
                        phi_9032_ = false;
                        phi_2387_ = type_14(0u, _e1948);
                        phi_2390_ = vec3<f32>(0f, 0f, 0f);
                        loop {
                            let _e1954 = phi_9032_;
                            let _e1956 = phi_2387_;
                            let _e1958 = phi_2390_;
                            local_6 = _e1958;
                            local_7 = _e1958;
                            local_8 = _e1958;
                            if (_e1956.member < _e1956.member_1) {
                                phi_2388_ = type_14((_e1956.member + 1u), _e1956.member_1);
                                phi_2413_ = type_14(1u, _e1956.member);
                            } else {
                                phi_2388_ = _e1956;
                                phi_2413_ = type_14(0u, type_14().member_1);
                            }
                            let _e1971 = phi_2388_;
                            let _e1973 = phi_2413_;
                            switch bitcast<i32>(_e1973.member) {
                                case 0: {
                                    phi_9151_ = _e1954;
                                    phi_2391_ = vec3<f32>();
                                    phi_4902_ = false;
                                    break;
                                }
                                case 1: {
                                    if (_e1973.member_1 >= _e1948) {
                                        phi_7290_ = 4294967295u;
                                    } else {
                                        phi_7290_ = (_e1945 + _e1973.member_1);
                                    }
                                    let _e1980 = phi_7290_;
                                    if (_e128 >= 1u) {
                                        phi_7309_ = (_e1980 <= (_e128 - 1u));
                                    } else {
                                        phi_7309_ = false;
                                    }
                                    let _e1985 = phi_7309_;
                                    if _e1985 {
                                        let _e1988 = global_1.member[_e1980];
                                        phi_2430_ = _e1988;
                                    } else {
                                        phi_2430_ = 4294967295u;
                                    }
                                    let _e1990 = phi_2430_;
                                    if (_e128 >= 4u) {
                                        phi_7333_ = (_e1990 <= (_e128 - 4u));
                                    } else {
                                        phi_7333_ = false;
                                    }
                                    let _e1995 = phi_7333_;
                                    if _e1995 {
                                        let _e1998 = global_1.member[_e1990];
                                        switch bitcast<i32>(_e1998) {
                                            case 0: {
                                                phi_2442_ = 0u;
                                                break;
                                            }
                                            case 1: {
                                                phi_2442_ = 1u;
                                                break;
                                            }
                                            case 2: {
                                                phi_2442_ = 2u;
                                                break;
                                            }
                                            default: {
                                                phi_2442_ = 0u;
                                                break;
                                            }
                                        }
                                        let _e2001 = phi_2442_;
                                        let _e2005 = global_1.member[(_e1990 + 1u)];
                                        let _e2009 = global_1.member[(_e1990 + 2u)];
                                        let _e2013 = global_1.member[(_e1990 + 3u)];
                                        phi_2456_ = type_30(_e2001, _e2005, _e2009, _e2013);
                                    } else {
                                        phi_2456_ = type_30(0u, 4294967295u, 4294967295u, 4294967295u);
                                    }
                                    let _e2016 = phi_2456_;
                                    if (_e128 >= 10u) {
                                        phi_7365_ = (_e2016.member_2 <= (_e128 - 10u));
                                    } else {
                                        phi_7365_ = false;
                                    }
                                    let _e2022 = phi_7365_;
                                    if _e2022 {
                                        let _e2025 = global_1.member[_e2016.member_2];
                                        let _e2030 = global_1.member[(_e2016.member_2 + 1u)];
                                        let _e2035 = global_1.member[(_e2016.member_2 + 2u)];
                                        let _e2041 = global_1.member[(_e2016.member_2 + 3u)];
                                        let _e2046 = global_1.member[(_e2016.member_2 + 4u)];
                                        let _e2051 = global_1.member[(_e2016.member_2 + 5u)];
                                        let _e2056 = global_1.member[(_e2016.member_2 + 6u)];
                                        let _e2062 = global_1.member[(_e2016.member_2 + 7u)];
                                        let _e2067 = global_1.member[(_e2016.member_2 + 8u)];
                                        let _e2072 = global_1.member[(_e2016.member_2 + 9u)];
                                        phi_2506_ = type_31(vec3<f32>(bitcast<f32>(_e2025), bitcast<f32>(_e2030), bitcast<f32>(_e2035)), vec4<f32>(bitcast<f32>(_e2041), bitcast<f32>(_e2046), bitcast<f32>(_e2051), bitcast<f32>(_e2056)), vec3<f32>(bitcast<f32>(_e2062), bitcast<f32>(_e2067), bitcast<f32>(_e2072)));
                                    } else {
                                        phi_2506_ = type_31(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e2077 = phi_2506_;
                                    let _e2085 = (_e2077.member_1.x + _e2077.member_1.x);
                                    let _e2086 = (_e2077.member_1.y + _e2077.member_1.y);
                                    let _e2087 = (_e2077.member_1.z + _e2077.member_1.z);
                                    let _e2089 = (_e2077.member_1.z * _e2087);
                                    let _e2090 = (_e2077.member_1.w * _e2085);
                                    let _e2091 = (_e2077.member_1.w * _e2086);
                                    let _e2092 = (_e2077.member_1.w * _e2087);
                                    let _e2112 = (vec4<f32>((1f - fma(_e2077.member_1.y, _e2086, _e2089)), fma(_e2077.member_1.x, _e2086, _e2092), fma(_e2077.member_1.x, _e2087, -(_e2091)), 0f) * _e2077.member_2.x);
                                    let _e2114 = (vec4<f32>(fma(_e2077.member_1.x, _e2086, -(_e2092)), (1f - fma(_e2077.member_1.x, _e2085, _e2089)), fma(_e2077.member_1.y, _e2087, _e2090), 0f) * _e2077.member_2.y);
                                    let _e2116 = (vec4<f32>(fma(_e2077.member_1.x, _e2087, _e2091), fma(_e2077.member_1.y, _e2087, -(_e2090)), (1f - fma(_e2077.member_1.x, _e2085, (_e2077.member_1.y * _e2086))), 0f) * _e2077.member_2.z);
                                    switch bitcast<i32>(_e2016.member) {
                                        case 0: {
                                            if (_e128 >= 8u) {
                                                phi_8493_ = (_e2016.member_1 <= (_e128 - 8u));
                                            } else {
                                                phi_8493_ = false;
                                            }
                                            let _e3629 = phi_8493_;
                                            if _e3629 {
                                                let _e3632 = global_1.member[_e2016.member_1];
                                                let _e3637 = global_1.member[(_e2016.member_1 + 1u)];
                                                let _e3642 = global_1.member[(_e2016.member_1 + 2u)];
                                                let _e3648 = global_1.member[(_e2016.member_1 + 3u)];
                                                let _e3653 = global_1.member[(_e2016.member_1 + 4u)];
                                                let _e3658 = global_1.member[(_e2016.member_1 + 5u)];
                                                let _e3663 = global_1.member[(_e2016.member_1 + 6u)];
                                                let _e3669 = global_1.member[(_e2016.member_1 + 7u)];
                                                phi_2554_ = type_34(vec3<f32>(bitcast<f32>(_e3632), bitcast<f32>(_e3637), bitcast<f32>(_e3642)), vec4<f32>(bitcast<f32>(_e3648), bitcast<f32>(_e3653), bitcast<f32>(_e3658), bitcast<f32>(_e3663)), bitcast<f32>(_e3669));
                                            } else {
                                                phi_2554_ = type_34(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e3673 = phi_2554_;
                                            let _e3695 = fma(_e2116.x, _e3673.member.z, fma(_e2114.x, _e3673.member.y, (_e2112.x * _e3673.member.x)));
                                            let _e3696 = fma(_e2116.y, _e3673.member.z, fma(_e2114.y, _e3673.member.y, (_e2112.y * _e3673.member.x)));
                                            let _e3697 = fma(_e2116.z, _e3673.member.z, fma(_e2114.z, _e3673.member.y, (_e2112.z * _e3673.member.x)));
                                            let _e3702 = sqrt(fma(_e3697, _e3697, fma(_e3695, _e3695, (_e3696 * _e3696))));
                                            if (_e3702 == 0f) {
                                                phi_8540_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_8540_ = (vec3<f32>(_e3695, _e3696, _e3697) * (1f / _e3702));
                                            }
                                            let _e3707 = phi_8540_;
                                            let _e3709 = -(_e3707.x);
                                            let _e3711 = -(_e3707.y);
                                            let _e3713 = -(_e3707.z);
                                            let _e3714 = -(_e3707);
                                            let _e3716 = fma(-(_e690.z), _e299.member_3, 1f);
                                            let _e3720 = fma(0.4f, _e3716, (_e1374 * _e1386));
                                            let _e3721 = fma(0.4f, _e3716, (_e1376 * _e1386));
                                            let _e3722 = fma(0.4f, _e3716, (_e1378 * _e1386));
                                            let _e3730 = (_e1942 + vec3<f32>(_e3709, _e3711, _e3713));
                                            let _e3737 = sqrt(fma(_e3730.z, _e3730.z, fma(_e3730.x, _e3730.x, (_e3730.y * _e3730.y))));
                                            if (_e3737 == 0f) {
                                                phi_8575_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_8575_ = (_e3730 * (1f / _e3737));
                                            }
                                            let _e3742 = phi_8575_;
                                            let _e3743 = (_e1383 * _e1383);
                                            let _e3754 = max(fma(_e1938.z, _e3742.z, fma(_e1938.x, _e3742.x, (_e1938.y * _e3742.y))), 0f);
                                            let _e3767 = max(fma(_e1938.z, _e1942.z, fma(_e1938.x, _e1942.x, (_e1938.y * _e1942.y))), 0f);
                                            let _e3773 = fma(_e1938.z, _e3714.z, fma(_e1938.x, _e3714.x, (_e1938.y * _e3714.y)));
                                            let _e3774 = max(_e3773, 0f);
                                            let _e3775 = fma(_e690.y, _e299.member_4, 1f);
                                            let _e3776 = (_e3775 * _e3775);
                                            let _e3777 = (_e3776 * 0.125f);
                                            let _e3779 = fma(-(_e3776), 0.125f, 1f);
                                            let _e3792 = (1f - max(fma(_e3742.z, _e1942.z, fma(_e3742.x, _e1942.x, (_e3742.y * _e1942.y))), 0f));
                                            let _e3794 = select(_e3792, 0f, (_e3792 < 0f));
                                            let _e3797 = pow(select(_e3794, 1f, (_e3794 > 1f)), 5f);
                                            let _e3798 = fma((1f - _e3720), _e3797, _e3720);
                                            let _e3799 = fma((1f - _e3721), _e3797, _e3721);
                                            let _e3800 = fma((1f - _e3722), _e3797, _e3722);
                                            let _e3807 = (((_e3743 * _e3743) / (pow(fma((_e3754 * _e3754), fma(_e3743, _e3743, -1f), 1f), 2f) * 3.1415927f)) * ((_e3767 / fma(_e3767, _e3779, _e3777)) * (_e3774 / fma(_e3774, _e3779, _e3777))));
                                            let _e3814 = max(fma(_e1938.z, _e3713, fma(_e1938.x, _e3709, (_e1938.y * _e3711))), 0f);
                                            let _e3816 = fma((4f * _e3767), _e3814, 0.0001f);
                                            if ((_e2016.member_3 == 4294967295u) != true) {
                                                let _e3838 = global_1.member[_e2016.member_3];
                                                let _e3842 = global_1.member[(_e2016.member_3 + 1u)];
                                                let _e3846 = global_1.member[(_e2016.member_3 + 2u)];
                                                let _e3850 = global_1.member[(_e2016.member_3 + 3u)];
                                                let _e3854 = global_1.member[(_e2016.member_3 + 4u)];
                                                let _e3859 = global_1.member[(_e2016.member_3 + 5u)];
                                                let _e3864 = global_1.member[(_e2016.member_3 + 6u)];
                                                let _e3867 = global_1.member[_e1951];
                                                let _e3871 = global_1.member[(_e1951 + 1u)];
                                                let _e3873 = select(_e3838, 4294967295u, (0u >= _e3842));
                                                let _e3876 = global_1.member[_e3873];
                                                let _e3881 = global_1.member[(_e3873 + 1u)];
                                                let _e3886 = global_1.member[(_e3873 + 2u)];
                                                let _e3891 = global_1.member[(_e3873 + 3u)];
                                                let _e3896 = global_1.member[(_e3873 + 4u)];
                                                let _e3901 = global_1.member[(_e3873 + 5u)];
                                                let _e3906 = global_1.member[(_e3873 + 6u)];
                                                let _e3911 = global_1.member[(_e3873 + 7u)];
                                                let _e3916 = global_1.member[(_e3873 + 8u)];
                                                let _e3921 = global_1.member[(_e3873 + 9u)];
                                                let _e3926 = global_1.member[(_e3873 + 10u)];
                                                let _e3931 = global_1.member[(_e3873 + 11u)];
                                                let _e3936 = global_1.member[(_e3873 + 12u)];
                                                let _e3941 = global_1.member[(_e3873 + 13u)];
                                                let _e3946 = global_1.member[(_e3873 + 14u)];
                                                let _e3951 = global_1.member[(_e3873 + 15u)];
                                                let _e3971 = (bitcast<f32>(_e3951) + fma(bitcast<f32>(_e3931), _e136.z, fma(bitcast<f32>(_e3911), _e136.y, (bitcast<f32>(_e3891) * _e136.x))));
                                                let _e3972 = ((bitcast<f32>(_e3936) + fma(bitcast<f32>(_e3916), _e136.z, fma(bitcast<f32>(_e3896), _e136.y, (bitcast<f32>(_e3876) * _e136.x)))) / _e3971);
                                                let _e3973 = ((bitcast<f32>(_e3941) + fma(bitcast<f32>(_e3921), _e136.z, fma(bitcast<f32>(_e3901), _e136.y, (bitcast<f32>(_e3881) * _e136.x)))) / _e3971);
                                                let _e3974 = ((bitcast<f32>(_e3946) + fma(bitcast<f32>(_e3926), _e136.z, fma(bitcast<f32>(_e3906), _e136.y, (bitcast<f32>(_e3886) * _e136.x)))) / _e3971);
                                                if (abs(_e3972) <= 1f) {
                                                    let _e3978 = (abs(_e3973) <= 1f);
                                                    if _e3978 {
                                                        phi_8680_ = (abs(_e3974) <= 1f);
                                                    } else {
                                                        phi_8680_ = bool();
                                                    }
                                                    let _e3982 = phi_8680_;
                                                    phi_8683_ = _e3982;
                                                    phi_8684_ = select(true, false, _e3978);
                                                } else {
                                                    phi_8683_ = bool();
                                                    phi_8684_ = true;
                                                }
                                                let _e3985 = phi_8683_;
                                                let _e3987 = phi_8684_;
                                                if select(_e3985, false, _e3987) {
                                                    let _e3995 = global_1.member[select(_e3846, 4294967295u, (0u >= _e3850))];
                                                    let _e3998 = global_1.member[_e3995];
                                                    let _e4002 = global_1.member[(_e3995 + 1u)];
                                                    let _e4006 = global_1.member[(_e3995 + 2u)];
                                                    let _e4010 = global_1.member[(_e3995 + 3u)];
                                                    let _e4014 = global_1.member[(_e3995 + 4u)];
                                                    let _e4018 = global_1.member[(_e3995 + 6u)];
                                                    switch bitcast<i32>(_e4018) {
                                                        case 0: {
                                                            phi_2945_ = 0u;
                                                            break;
                                                        }
                                                        case 1: {
                                                            phi_2945_ = 1u;
                                                            break;
                                                        }
                                                        case 2: {
                                                            phi_2945_ = 2u;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_2945_ = 0u;
                                                            break;
                                                        }
                                                    }
                                                    let _e4021 = phi_2945_;
                                                    let _e4025 = global_1.member[(_e3995 + 7u)];
                                                    switch bitcast<i32>(_e4025) {
                                                        case 0: {
                                                            phi_2954_ = 0u;
                                                            break;
                                                        }
                                                        case 1: {
                                                            phi_2954_ = 1u;
                                                            break;
                                                        }
                                                        case 2: {
                                                            phi_2954_ = 2u;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_2954_ = 0u;
                                                            break;
                                                        }
                                                    }
                                                    let _e4028 = phi_2954_;
                                                    let _e4029 = bitcast<i32>(_e3864);
                                                    let _e4031 = f32(_e4006);
                                                    let _e4032 = f32(_e4010);
                                                    let _e4036 = type_35((_e4029 / -2i), (_e4029 / 2i), false);
                                                    phi_9103_ = _e1954;
                                                    phi_2982_ = _e4036;
                                                    phi_2985_ = 0f;
                                                    phi_2987_ = 0f;
                                                    loop {
                                                        let _e4038 = phi_9103_;
                                                        let _e4040 = phi_2982_;
                                                        let _e4042 = phi_2985_;
                                                        let _e4044 = phi_2987_;
                                                        local_4 = _e4042;
                                                        local_5 = _e4044;
                                                        if _e4040.member_2 {
                                                            phi_2999_ = true;
                                                        } else {
                                                            phi_2999_ = ((_e4040.member <= _e4040.member_1) != true);
                                                        }
                                                        let _e4051 = phi_2999_;
                                                        if _e4051 {
                                                            phi_2983_ = _e4040;
                                                            phi_3042_ = type_36(0u, type_36().member_1);
                                                        } else {
                                                            if (_e4040.member < _e4040.member_1) {
                                                                let _e4059 = (_e4040.member + 1i);
                                                                if select(false, true, ((false == (_e4059 > _e4040.member)) != false)) {
                                                                    phi_3027_ = type_36(0u, type_36().member_1);
                                                                } else {
                                                                    phi_3027_ = type_36(1u, _e4059);
                                                                }
                                                                let _e4069 = phi_3027_;
                                                                switch bitcast<i32>(_e4069.member) {
                                                                    case 0: {
                                                                        phi_9148_ = true;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        break;
                                                                    }
                                                                }
                                                                phi_3039_ = type_35(_e4069.member_1, _e4040.member_1, _e4040.member_2);
                                                            } else {
                                                                phi_3039_ = type_35(_e4040.member, _e4040.member_1, true);
                                                            }
                                                            let _e4078 = phi_3039_;
                                                            phi_2983_ = _e4078;
                                                            phi_3042_ = type_36(1u, _e4040.member);
                                                        }
                                                        let _e4084 = phi_2983_;
                                                        let _e4086 = phi_3042_;
                                                        switch bitcast<i32>(_e4086.member) {
                                                            case 0: {
                                                                phi_9149_ = _e4038;
                                                                phi_2986_ = f32();
                                                                phi_2988_ = f32();
                                                                phi_3300_ = false;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3053_ = _e4036;
                                                                phi_3056_ = _e4042;
                                                                phi_3058_ = _e4044;
                                                                loop {
                                                                    let _e4091 = phi_3053_;
                                                                    let _e4093 = phi_3056_;
                                                                    let _e4095 = phi_3058_;
                                                                    local_11 = _e4093;
                                                                    local_12 = _e4095;
                                                                    if _e4091.member_2 {
                                                                        phi_3070_ = true;
                                                                    } else {
                                                                        phi_3070_ = ((_e4091.member <= _e4091.member_1) != true);
                                                                    }
                                                                    let _e4102 = phi_3070_;
                                                                    if _e4102 {
                                                                        phi_3054_ = _e4091;
                                                                        phi_3113_ = type_36(0u, type_36().member_1);
                                                                    } else {
                                                                        if (_e4091.member < _e4091.member_1) {
                                                                            let _e4110 = (_e4091.member + 1i);
                                                                            if select(false, true, ((false == (_e4110 > _e4091.member)) != false)) {
                                                                                phi_3098_ = type_36(0u, type_36().member_1);
                                                                            } else {
                                                                                phi_3098_ = type_36(1u, _e4110);
                                                                            }
                                                                            let _e4120 = phi_3098_;
                                                                            switch bitcast<i32>(_e4120.member) {
                                                                                case 0: {
                                                                                    phi_9087_ = true;
                                                                                    break;
                                                                                }
                                                                                case 1: {
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            phi_3110_ = type_35(_e4120.member_1, _e4091.member_1, _e4091.member_2);
                                                                        } else {
                                                                            phi_3110_ = type_35(_e4091.member, _e4091.member_1, true);
                                                                        }
                                                                        let _e4129 = phi_3110_;
                                                                        phi_3054_ = _e4129;
                                                                        phi_3113_ = type_36(1u, _e4091.member);
                                                                    }
                                                                    let _e4135 = phi_3054_;
                                                                    let _e4137 = phi_3113_;
                                                                    switch bitcast<i32>(_e4137.member) {
                                                                        case 0: {
                                                                            phi_3057_ = f32();
                                                                            phi_3059_ = f32();
                                                                            phi_3299_ = false;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            let _e4145 = fma((_e3972 + 1f), 0.5f, (f32(_e4086.member_1) * (1f / _e4031)));
                                                                            let _e4146 = fma(fma(_e3973, -1f, 1f), 0.5f, (f32(_e4137.member_1) * (1f / _e4032)));
                                                                            switch bitcast<i32>(_e4021) {
                                                                                case 1: {
                                                                                    let _e4181 = abs(_e4145);
                                                                                    let _e4183 = (_e4181 % 1f);
                                                                                    if (_e4181 >= 1f) {
                                                                                        phi_8736_ = select(true, false, (_e4183 == 0f));
                                                                                    } else {
                                                                                        phi_8736_ = true;
                                                                                    }
                                                                                    let _e4187 = phi_8736_;
                                                                                    let _e4188 = select(1f, _e4183, _e4187);
                                                                                    if (select(-1f, 1f, (_e4145 >= 0f)) > 0f) {
                                                                                        phi_3145_ = _e4188;
                                                                                    } else {
                                                                                        phi_3145_ = (1f - _e4188);
                                                                                    }
                                                                                    let _e4192 = phi_3145_;
                                                                                    phi_3182_ = _e4192;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e4155 = abs(_e4145);
                                                                                    let _e4162 = ((select(select(u32(_e4155), 0u, (_e4155 < 0f)), 4294967295u, (_e4155 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e4164 = (_e4155 % 1f);
                                                                                    if (_e4155 >= 1f) {
                                                                                        phi_8719_ = select(true, false, (_e4164 == 0f));
                                                                                    } else {
                                                                                        phi_8719_ = true;
                                                                                    }
                                                                                    let _e4168 = phi_8719_;
                                                                                    let _e4169 = select(1f, _e4164, _e4168);
                                                                                    if (select(-1f, 1f, (_e4145 >= 0f)) > 0f) {
                                                                                        if _e4162 {
                                                                                            phi_3174_ = _e4169;
                                                                                        } else {
                                                                                            phi_3174_ = (1f - _e4169);
                                                                                        }
                                                                                        let _e4176 = phi_3174_;
                                                                                        phi_3180_ = _e4176;
                                                                                    } else {
                                                                                        if _e4162 {
                                                                                            phi_3179_ = (1f - _e4169);
                                                                                        } else {
                                                                                            phi_3179_ = _e4169;
                                                                                        }
                                                                                        let _e4173 = phi_3179_;
                                                                                        phi_3180_ = _e4173;
                                                                                    }
                                                                                    let _e4178 = phi_3180_;
                                                                                    phi_3182_ = _e4178;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e4145 > 1f) {
                                                                                        phi_8706_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_8706_ = select(_e4145, 0.00000011920929f, (_e4145 < 0f));
                                                                                    }
                                                                                    let _e4152 = phi_8706_;
                                                                                    phi_3182_ = _e4152;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3182_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e4194 = phi_3182_;
                                                                            switch bitcast<i32>(_e4028) {
                                                                                case 1: {
                                                                                    let _e4229 = abs(_e4146);
                                                                                    let _e4231 = (_e4229 % 1f);
                                                                                    if (_e4229 >= 1f) {
                                                                                        phi_8784_ = select(true, false, (_e4231 == 0f));
                                                                                    } else {
                                                                                        phi_8784_ = true;
                                                                                    }
                                                                                    let _e4235 = phi_8784_;
                                                                                    let _e4236 = select(1f, _e4231, _e4235);
                                                                                    if (select(-1f, 1f, (_e4146 >= 0f)) > 0f) {
                                                                                        phi_3201_ = _e4236;
                                                                                    } else {
                                                                                        phi_3201_ = (1f - _e4236);
                                                                                    }
                                                                                    let _e4240 = phi_3201_;
                                                                                    phi_3238_ = _e4240;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e4203 = abs(_e4146);
                                                                                    let _e4210 = ((select(select(u32(_e4203), 0u, (_e4203 < 0f)), 4294967295u, (_e4203 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e4212 = (_e4203 % 1f);
                                                                                    if (_e4203 >= 1f) {
                                                                                        phi_8767_ = select(true, false, (_e4212 == 0f));
                                                                                    } else {
                                                                                        phi_8767_ = true;
                                                                                    }
                                                                                    let _e4216 = phi_8767_;
                                                                                    let _e4217 = select(1f, _e4212, _e4216);
                                                                                    if (select(-1f, 1f, (_e4146 >= 0f)) > 0f) {
                                                                                        if _e4210 {
                                                                                            phi_3230_ = _e4217;
                                                                                        } else {
                                                                                            phi_3230_ = (1f - _e4217);
                                                                                        }
                                                                                        let _e4224 = phi_3230_;
                                                                                        phi_3236_ = _e4224;
                                                                                    } else {
                                                                                        if _e4210 {
                                                                                            phi_3235_ = (1f - _e4217);
                                                                                        } else {
                                                                                            phi_3235_ = _e4217;
                                                                                        }
                                                                                        let _e4221 = phi_3235_;
                                                                                        phi_3236_ = _e4221;
                                                                                    }
                                                                                    let _e4226 = phi_3236_;
                                                                                    phi_3238_ = _e4226;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e4146 > 1f) {
                                                                                        phi_8754_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_8754_ = select(_e4146, 0.00000011920929f, (_e4146 < 0f));
                                                                                    }
                                                                                    let _e4200 = phi_8754_;
                                                                                    phi_3238_ = _e4200;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3238_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e4242 = phi_3238_;
                                                                            let _e4243 = (_e4194 * _e4031);
                                                                            let _e4249 = (_e4242 * _e4032);
                                                                            let _e4264 = vec3<f32>((f32((select(select(u32(_e4243), 0u, (_e4243 < 0f)), 4294967295u, (_e4243 > 4294967000f)) + _e3998)) / f32(_e3867)), (f32((select(select(u32(_e4249), 0u, (_e4249 < 0f)), 4294967295u, (_e4249 > 4294967000f)) + _e4002)) / f32(_e3871)), f32(_e4014));
                                                                            let _e4270 = textureSampleLevel(global_19, global_18, vec2<f32>(_e4264.x, _e4264.y), i32(_e4264.z), 0f);
                                                                            phi_3057_ = (_e4093 + 1f);
                                                                            phi_3059_ = (_e4095 + select(0f, 1f, ((_e3974 - max((bitcast<f32>(_e3859) * (1f - _e3773)), bitcast<f32>(_e3854))) > _e4270.x)));
                                                                            phi_3299_ = true;
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            phi_3057_ = f32();
                                                                            phi_3059_ = f32();
                                                                            phi_3299_ = bool();
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e4281 = phi_3057_;
                                                                    let _e4283 = phi_3059_;
                                                                    let _e4285 = phi_3299_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_3053_ = _e4135;
                                                                        phi_3056_ = _e4281;
                                                                        phi_3058_ = _e4283;
                                                                        phi_9087_ = _e4038;
                                                                        break if !(_e4285);
                                                                    }
                                                                }
                                                                let _e4288 = phi_9087_;
                                                                phi_9148_ = _e4288;
                                                                if _e4288 {
                                                                    break;
                                                                }
                                                                phi_9149_ = _e4288;
                                                                let _e4745 = local_11;
                                                                phi_2986_ = _e4745;
                                                                let _e4748 = local_12;
                                                                phi_2988_ = _e4748;
                                                                phi_3300_ = true;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_9149_ = _e4038;
                                                                phi_2986_ = f32();
                                                                phi_2988_ = f32();
                                                                phi_3300_ = bool();
                                                                break;
                                                            }
                                                        }
                                                        let _e4290 = phi_9149_;
                                                        let _e4292 = phi_2986_;
                                                        let _e4294 = phi_2988_;
                                                        let _e4296 = phi_3300_;
                                                        continue;
                                                        continuing {
                                                            phi_9103_ = _e4290;
                                                            phi_2982_ = _e4084;
                                                            phi_2985_ = _e4292;
                                                            phi_2987_ = _e4294;
                                                            phi_9148_ = _e4290;
                                                            break if !(_e4296);
                                                        }
                                                    }
                                                    let _e4299 = phi_9148_;
                                                    phi_9150_ = _e4299;
                                                    if _e4299 {
                                                        break;
                                                    }
                                                    let _e4301 = local_4;
                                                    let _e4304 = local_5;
                                                    phi_9232_ = _e4299;
                                                    phi_3303_ = (_e4304 / max(_e4301, 1f));
                                                } else {
                                                    phi_9232_ = _e1954;
                                                    phi_3303_ = 0f;
                                                }
                                                let _e4307 = phi_9232_;
                                                let _e4309 = phi_3303_;
                                                phi_9231_ = _e4307;
                                                phi_3304_ = _e4309;
                                            } else {
                                                phi_9231_ = _e1954;
                                                phi_3304_ = 0f;
                                            }
                                            let _e4311 = phi_9231_;
                                            let _e4313 = phi_3304_;
                                            phi_9154_ = _e4311;
                                            phi_4874_ = _e4313;
                                            phi_4875_ = vec3<f32>(((fma((((1f - _e3798) * _e3716) * _e1374), 0.31830987f, ((_e3807 * _e3798) / _e3816)) * (_e3673.member_1.x * _e3673.member_2)) * _e3814), ((fma((((1f - _e3799) * _e3716) * _e1376), 0.31830987f, ((_e3807 * _e3799) / _e3816)) * (_e3673.member_1.y * _e3673.member_2)) * _e3814), ((fma((((1f - _e3800) * _e3716) * _e1378), 0.31830987f, ((_e3807 * _e3800) / _e3816)) * (_e3673.member_1.z * _e3673.member_2)) * _e3814));
                                            phi_4876_ = true;
                                            break;
                                        }
                                        case 1: {
                                            if (_e128 >= 8u) {
                                                phi_7811_ = (_e2016.member_1 <= (_e128 - 8u));
                                            } else {
                                                phi_7811_ = false;
                                            }
                                            let _e2901 = phi_7811_;
                                            if _e2901 {
                                                let _e2904 = global_1.member[_e2016.member_1];
                                                let _e2909 = global_1.member[(_e2016.member_1 + 1u)];
                                                let _e2914 = global_1.member[(_e2016.member_1 + 2u)];
                                                let _e2920 = global_1.member[(_e2016.member_1 + 3u)];
                                                let _e2925 = global_1.member[(_e2016.member_1 + 4u)];
                                                let _e2930 = global_1.member[(_e2016.member_1 + 5u)];
                                                let _e2935 = global_1.member[(_e2016.member_1 + 6u)];
                                                let _e2941 = global_1.member[(_e2016.member_1 + 7u)];
                                                phi_3345_ = type_34(vec3<f32>(bitcast<f32>(_e2904), bitcast<f32>(_e2909), bitcast<f32>(_e2914)), vec4<f32>(bitcast<f32>(_e2920), bitcast<f32>(_e2925), bitcast<f32>(_e2930), bitcast<f32>(_e2935)), bitcast<f32>(_e2941));
                                            } else {
                                                phi_3345_ = type_34(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2945 = phi_3345_;
                                            let _e2974 = (vec3<f32>((_e2077.member.x + fma(_e2116.x, _e2945.member.z, fma(_e2114.x, _e2945.member.y, (_e2112.x * _e2945.member.x)))), (_e2077.member.y + fma(_e2116.y, _e2945.member.z, fma(_e2114.y, _e2945.member.y, (_e2112.y * _e2945.member.x)))), (_e2077.member.z + fma(_e2116.z, _e2945.member.z, fma(_e2114.z, _e2945.member.y, (_e2112.z * _e2945.member.x))))) - _e136);
                                            let _e2981 = sqrt(fma(_e2974.z, _e2974.z, fma(_e2974.x, _e2974.x, (_e2974.y * _e2974.y))));
                                            let _e2982 = (_e2981 == 0f);
                                            if _e2982 {
                                                phi_3949_ = f32();
                                                phi_3950_ = vec3<f32>();
                                            } else {
                                                if _e2982 {
                                                    phi_7858_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7858_ = (_e2974 * (1f / _e2981));
                                                }
                                                let _e2986 = phi_7858_;
                                                let _e2988 = (_e2945.member_2 / (_e2981 * _e2981));
                                                let _e2990 = fma(-(_e690.z), _e299.member_3, 1f);
                                                let _e2994 = fma(0.4f, _e2990, (_e1374 * _e1386));
                                                let _e2995 = fma(0.4f, _e2990, (_e1376 * _e1386));
                                                let _e2996 = fma(0.4f, _e2990, (_e1378 * _e1386));
                                                let _e3003 = (_e1942 + _e2986);
                                                let _e3010 = sqrt(fma(_e3003.z, _e3003.z, fma(_e3003.x, _e3003.x, (_e3003.y * _e3003.y))));
                                                if (_e3010 == 0f) {
                                                    phi_7893_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7893_ = (_e3003 * (1f / _e3010));
                                                }
                                                let _e3015 = phi_7893_;
                                                let _e3016 = (_e1383 * _e1383);
                                                let _e3027 = max(fma(_e1938.z, _e3015.z, fma(_e1938.x, _e3015.x, (_e1938.y * _e3015.y))), 0f);
                                                let _e3040 = max(fma(_e1938.z, _e1942.z, fma(_e1938.x, _e1942.x, (_e1938.y * _e1942.y))), 0f);
                                                let _e3046 = fma(_e1938.z, _e2986.z, fma(_e1938.x, _e2986.x, (_e1938.y * _e2986.y)));
                                                let _e3047 = max(_e3046, 0f);
                                                let _e3048 = fma(_e690.y, _e299.member_4, 1f);
                                                let _e3049 = (_e3048 * _e3048);
                                                let _e3050 = (_e3049 * 0.125f);
                                                let _e3052 = fma(-(_e3049), 0.125f, 1f);
                                                let _e3065 = (1f - max(fma(_e3015.z, _e1942.z, fma(_e3015.x, _e1942.x, (_e3015.y * _e1942.y))), 0f));
                                                let _e3067 = select(_e3065, 0f, (_e3065 < 0f));
                                                let _e3070 = pow(select(_e3067, 1f, (_e3067 > 1f)), 5f);
                                                let _e3071 = fma((1f - _e2994), _e3070, _e2994);
                                                let _e3072 = fma((1f - _e2995), _e3070, _e2995);
                                                let _e3073 = fma((1f - _e2996), _e3070, _e2996);
                                                let _e3080 = (((_e3016 * _e3016) / (pow(fma((_e3027 * _e3027), fma(_e3016, _e3016, -1f), 1f), 2f) * 3.1415927f)) * ((_e3040 / fma(_e3040, _e3052, _e3050)) * (_e3047 / fma(_e3047, _e3052, _e3050))));
                                                let _e3085 = fma((4f * _e3040), _e3047, 0.0001f);
                                                if ((_e2016.member_3 == 4294967295u) != true) {
                                                    let _e3107 = global_1.member[_e2016.member_3];
                                                    let _e3111 = global_1.member[(_e2016.member_3 + 1u)];
                                                    let _e3115 = global_1.member[(_e2016.member_3 + 2u)];
                                                    let _e3119 = global_1.member[(_e2016.member_3 + 3u)];
                                                    let _e3123 = global_1.member[(_e2016.member_3 + 4u)];
                                                    let _e3128 = global_1.member[(_e2016.member_3 + 5u)];
                                                    let _e3132 = global_1.member[_e1951];
                                                    let _e3136 = global_1.member[(_e1951 + 1u)];
                                                    let _e3137 = -(_e2986);
                                                    let _e3144 = sqrt(fma(_e3137.z, _e3137.z, fma(_e3137.x, _e3137.x, (_e3137.y * _e3137.y))));
                                                    if (_e3144 == 0f) {
                                                        phi_8014_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_8014_ = (_e3137 * (1f / _e3144));
                                                    }
                                                    let _e3149 = phi_8014_;
                                                    if (abs((-1f * _e3149.x)) > 0.00000011920929f) {
                                                        let _e3156 = (1f / _e3149.x);
                                                        let _e3157 = (_e3156 >= 0f);
                                                        if _e3157 {
                                                            phi_8144_ = type_38(true, (_e3149 * _e3156));
                                                        } else {
                                                            phi_8144_ = type_38();
                                                        }
                                                        let _e3161 = phi_8144_;
                                                        phi_8147_ = _e3161;
                                                        phi_8148_ = select(true, false, _e3157);
                                                    } else {
                                                        phi_8147_ = type_38();
                                                        phi_8148_ = true;
                                                    }
                                                    let _e3164 = phi_8147_;
                                                    let _e3166 = phi_8148_;
                                                    if _e3166 {
                                                        phi_8153_ = type_38(false, vec3<f32>(0f, 0f, 0f));
                                                    } else {
                                                        phi_8153_ = _e3164;
                                                    }
                                                    let _e3168 = phi_8153_;
                                                    if _e3168.member {
                                                        phi_8116_ = type_37(0u, vec2<f32>(((1f - _e3168.member_1.z) * 0.5f), ((1f - _e3168.member_1.y) * 0.5f)));
                                                    } else {
                                                        if (abs(_e3149.x) > 0.00000011920929f) {
                                                            let _e3173 = (-1f / _e3149.x);
                                                            let _e3174 = (_e3173 >= 0f);
                                                            if _e3174 {
                                                                phi_8180_ = type_38(true, (_e3149 * _e3173));
                                                            } else {
                                                                phi_8180_ = type_38();
                                                            }
                                                            let _e3178 = phi_8180_;
                                                            phi_8183_ = _e3178;
                                                            phi_8184_ = select(true, false, _e3174);
                                                        } else {
                                                            phi_8183_ = type_38();
                                                            phi_8184_ = true;
                                                        }
                                                        let _e3181 = phi_8183_;
                                                        let _e3183 = phi_8184_;
                                                        if _e3183 {
                                                            phi_8189_ = type_38(false, vec3<f32>(0f, 0f, 0f));
                                                        } else {
                                                            phi_8189_ = _e3181;
                                                        }
                                                        let _e3185 = phi_8189_;
                                                        if _e3185.member {
                                                            phi_8103_ = type_37(1u, vec2<f32>(((_e3185.member_1.z + 1f) * 0.5f), ((1f - _e3185.member_1.y) * 0.5f)));
                                                        } else {
                                                            if (abs((-1f * _e3149.y)) > 0.00000011920929f) {
                                                                let _e3191 = (1f / _e3149.y);
                                                                let _e3192 = (_e3191 >= 0f);
                                                                if _e3192 {
                                                                    phi_8216_ = type_38(true, (_e3149 * _e3191));
                                                                } else {
                                                                    phi_8216_ = type_38();
                                                                }
                                                                let _e3196 = phi_8216_;
                                                                phi_8219_ = _e3196;
                                                                phi_8220_ = select(true, false, _e3192);
                                                            } else {
                                                                phi_8219_ = type_38();
                                                                phi_8220_ = true;
                                                            }
                                                            let _e3199 = phi_8219_;
                                                            let _e3201 = phi_8220_;
                                                            if _e3201 {
                                                                phi_8225_ = type_38(false, vec3<f32>(0f, 0f, 0f));
                                                            } else {
                                                                phi_8225_ = _e3199;
                                                            }
                                                            let _e3203 = phi_8225_;
                                                            if _e3203.member {
                                                                phi_8091_ = type_37(2u, vec2<f32>(((_e3203.member_1.x + 1f) * 0.5f), ((_e3203.member_1.z + 1f) * 0.5f)));
                                                            } else {
                                                                if (abs(_e3149.y) > 0.00000011920929f) {
                                                                    let _e3208 = (-1f / _e3149.y);
                                                                    let _e3209 = (_e3208 >= 0f);
                                                                    if _e3209 {
                                                                        phi_8252_ = type_38(true, (_e3149 * _e3208));
                                                                    } else {
                                                                        phi_8252_ = type_38();
                                                                    }
                                                                    let _e3213 = phi_8252_;
                                                                    phi_8255_ = _e3213;
                                                                    phi_8256_ = select(true, false, _e3209);
                                                                } else {
                                                                    phi_8255_ = type_38();
                                                                    phi_8256_ = true;
                                                                }
                                                                let _e3216 = phi_8255_;
                                                                let _e3218 = phi_8256_;
                                                                if _e3218 {
                                                                    phi_8261_ = type_38(false, vec3<f32>(0f, 0f, 0f));
                                                                } else {
                                                                    phi_8261_ = _e3216;
                                                                }
                                                                let _e3220 = phi_8261_;
                                                                if _e3220.member {
                                                                    phi_8080_ = type_37(3u, vec2<f32>(((_e3220.member_1.x + 1f) * 0.5f), ((1f - _e3220.member_1.z) * 0.5f)));
                                                                } else {
                                                                    if (abs((-1f * _e3149.z)) > 0.00000011920929f) {
                                                                        let _e3226 = (1f / _e3149.z);
                                                                        let _e3227 = (_e3226 >= 0f);
                                                                        if _e3227 {
                                                                            phi_8288_ = type_38(true, (_e3149 * _e3226));
                                                                        } else {
                                                                            phi_8288_ = type_38();
                                                                        }
                                                                        let _e3231 = phi_8288_;
                                                                        phi_8291_ = _e3231;
                                                                        phi_8292_ = select(true, false, _e3227);
                                                                    } else {
                                                                        phi_8291_ = type_38();
                                                                        phi_8292_ = true;
                                                                    }
                                                                    let _e3234 = phi_8291_;
                                                                    let _e3236 = phi_8292_;
                                                                    if _e3236 {
                                                                        phi_8297_ = type_38(false, vec3<f32>(0f, 0f, 0f));
                                                                    } else {
                                                                        phi_8297_ = _e3234;
                                                                    }
                                                                    let _e3238 = phi_8297_;
                                                                    if _e3238.member {
                                                                        phi_8068_ = type_37(4u, vec2<f32>(((_e3238.member_1.x + 1f) * 0.5f), ((1f - _e3238.member_1.y) * 0.5f)));
                                                                    } else {
                                                                        if (abs(_e3149.z) > 0.00000011920929f) {
                                                                            let _e3243 = (-1f / _e3149.z);
                                                                            let _e3244 = (_e3243 >= 0f);
                                                                            if _e3244 {
                                                                                phi_8324_ = type_38(true, (_e3149 * _e3243));
                                                                            } else {
                                                                                phi_8324_ = type_38();
                                                                            }
                                                                            let _e3248 = phi_8324_;
                                                                            phi_8327_ = _e3248;
                                                                            phi_8328_ = select(true, false, _e3244);
                                                                        } else {
                                                                            phi_8327_ = type_38();
                                                                            phi_8328_ = true;
                                                                        }
                                                                        let _e3251 = phi_8327_;
                                                                        let _e3253 = phi_8328_;
                                                                        if _e3253 {
                                                                            phi_8333_ = type_38(false, vec3<f32>(0f, 0f, 0f));
                                                                        } else {
                                                                            phi_8333_ = _e3251;
                                                                        }
                                                                        let _e3255 = phi_8333_;
                                                                        if _e3255.member {
                                                                            phi_8056_ = type_37(5u, vec2<f32>(((1f - _e3255.member_1.x) * 0.5f), ((1f - _e3255.member_1.y) * 0.5f)));
                                                                        } else {
                                                                            phi_8056_ = type_37(0u, vec2<f32>(0.5f, 0.5f));
                                                                        }
                                                                        let _e3267 = phi_8056_;
                                                                        phi_8068_ = _e3267;
                                                                    }
                                                                    let _e3277 = phi_8068_;
                                                                    phi_8080_ = _e3277;
                                                                }
                                                                let _e3287 = phi_8080_;
                                                                phi_8091_ = _e3287;
                                                            }
                                                            let _e3297 = phi_8091_;
                                                            phi_8103_ = _e3297;
                                                        }
                                                        let _e3307 = phi_8103_;
                                                        phi_8116_ = _e3307;
                                                    }
                                                    let _e3317 = phi_8116_;
                                                    if (_e3317.member >= _e3111) {
                                                        phi_3599_ = 4294967295u;
                                                    } else {
                                                        phi_3599_ = (_e3107 + (16u * _e3317.member));
                                                    }
                                                    let _e3324 = phi_3599_;
                                                    let _e3327 = global_1.member[_e3324];
                                                    let _e3332 = global_1.member[(_e3324 + 1u)];
                                                    let _e3337 = global_1.member[(_e3324 + 2u)];
                                                    let _e3342 = global_1.member[(_e3324 + 3u)];
                                                    let _e3347 = global_1.member[(_e3324 + 4u)];
                                                    let _e3352 = global_1.member[(_e3324 + 5u)];
                                                    let _e3357 = global_1.member[(_e3324 + 6u)];
                                                    let _e3362 = global_1.member[(_e3324 + 7u)];
                                                    let _e3367 = global_1.member[(_e3324 + 8u)];
                                                    let _e3372 = global_1.member[(_e3324 + 9u)];
                                                    let _e3377 = global_1.member[(_e3324 + 10u)];
                                                    let _e3382 = global_1.member[(_e3324 + 11u)];
                                                    let _e3387 = global_1.member[(_e3324 + 12u)];
                                                    let _e3392 = global_1.member[(_e3324 + 13u)];
                                                    let _e3397 = global_1.member[(_e3324 + 14u)];
                                                    let _e3402 = global_1.member[(_e3324 + 15u)];
                                                    let _e3422 = (bitcast<f32>(_e3402) + fma(bitcast<f32>(_e3382), _e136.z, fma(bitcast<f32>(_e3362), _e136.y, (bitcast<f32>(_e3342) * _e136.x))));
                                                    let _e3425 = ((bitcast<f32>(_e3397) + fma(bitcast<f32>(_e3377), _e136.z, fma(bitcast<f32>(_e3357), _e136.y, (bitcast<f32>(_e3337) * _e136.x)))) / _e3422);
                                                    if (abs(((bitcast<f32>(_e3387) + fma(bitcast<f32>(_e3367), _e136.z, fma(bitcast<f32>(_e3347), _e136.y, (bitcast<f32>(_e3327) * _e136.x)))) / _e3422)) <= 1f) {
                                                        let _e3429 = (abs(((bitcast<f32>(_e3392) + fma(bitcast<f32>(_e3372), _e136.z, fma(bitcast<f32>(_e3352), _e136.y, (bitcast<f32>(_e3332) * _e136.x)))) / _e3422)) <= 1f);
                                                        if _e3429 {
                                                            phi_8352_ = (abs(_e3425) <= 1f);
                                                        } else {
                                                            phi_8352_ = bool();
                                                        }
                                                        let _e3433 = phi_8352_;
                                                        phi_8355_ = _e3433;
                                                        phi_8356_ = select(true, false, _e3429);
                                                    } else {
                                                        phi_8355_ = bool();
                                                        phi_8356_ = true;
                                                    }
                                                    let _e3436 = phi_8355_;
                                                    let _e3438 = phi_8356_;
                                                    if select(_e3436, false, _e3438) {
                                                        if (_e3317.member >= _e3119) {
                                                            phi_3727_ = 4294967295u;
                                                        } else {
                                                            phi_3727_ = (_e3115 + _e3317.member);
                                                        }
                                                        let _e3443 = phi_3727_;
                                                        let _e3446 = global_1.member[_e3443];
                                                        let _e3449 = global_1.member[_e3446];
                                                        let _e3453 = global_1.member[(_e3446 + 1u)];
                                                        let _e3457 = global_1.member[(_e3446 + 2u)];
                                                        let _e3461 = global_1.member[(_e3446 + 3u)];
                                                        let _e3465 = global_1.member[(_e3446 + 4u)];
                                                        let _e3469 = global_1.member[(_e3446 + 6u)];
                                                        switch bitcast<i32>(_e3469) {
                                                            case 0: {
                                                                phi_3758_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3758_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_3758_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_3758_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e3472 = phi_3758_;
                                                        let _e3476 = global_1.member[(_e3446 + 7u)];
                                                        switch bitcast<i32>(_e3476) {
                                                            case 0: {
                                                                phi_3767_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3767_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_3767_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_3767_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e3479 = phi_3767_;
                                                        switch bitcast<i32>(_e3472) {
                                                            case 1: {
                                                                let _e3515 = abs(_e3317.member_1.x);
                                                                let _e3517 = (_e3515 % 1f);
                                                                if (_e3515 >= 1f) {
                                                                    phi_8408_ = select(true, false, (_e3517 == 0f));
                                                                } else {
                                                                    phi_8408_ = true;
                                                                }
                                                                let _e3521 = phi_8408_;
                                                                let _e3522 = select(1f, _e3517, _e3521);
                                                                if (select(-1f, 1f, (_e3317.member_1.x >= 0f)) > 0f) {
                                                                    phi_3793_ = _e3522;
                                                                } else {
                                                                    phi_3793_ = (1f - _e3522);
                                                                }
                                                                let _e3526 = phi_3793_;
                                                                phi_3830_ = _e3526;
                                                                break;
                                                            }
                                                            case 2: {
                                                                let _e3489 = abs(_e3317.member_1.x);
                                                                let _e3496 = ((select(select(u32(_e3489), 0u, (_e3489 < 0f)), 4294967295u, (_e3489 > 4294967000f)) % 2u) == 0u);
                                                                let _e3498 = (_e3489 % 1f);
                                                                if (_e3489 >= 1f) {
                                                                    phi_8391_ = select(true, false, (_e3498 == 0f));
                                                                } else {
                                                                    phi_8391_ = true;
                                                                }
                                                                let _e3502 = phi_8391_;
                                                                let _e3503 = select(1f, _e3498, _e3502);
                                                                if (select(-1f, 1f, (_e3317.member_1.x >= 0f)) > 0f) {
                                                                    if _e3496 {
                                                                        phi_3822_ = _e3503;
                                                                    } else {
                                                                        phi_3822_ = (1f - _e3503);
                                                                    }
                                                                    let _e3510 = phi_3822_;
                                                                    phi_3828_ = _e3510;
                                                                } else {
                                                                    if _e3496 {
                                                                        phi_3827_ = (1f - _e3503);
                                                                    } else {
                                                                        phi_3827_ = _e3503;
                                                                    }
                                                                    let _e3507 = phi_3827_;
                                                                    phi_3828_ = _e3507;
                                                                }
                                                                let _e3512 = phi_3828_;
                                                                phi_3830_ = _e3512;
                                                                break;
                                                            }
                                                            case 0: {
                                                                if (_e3317.member_1.x > 1f) {
                                                                    phi_8378_ = 0.9999999f;
                                                                } else {
                                                                    phi_8378_ = select(_e3317.member_1.x, 0.00000011920929f, (_e3317.member_1.x < 0f));
                                                                }
                                                                let _e3486 = phi_8378_;
                                                                phi_3830_ = _e3486;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_3830_ = f32();
                                                                break;
                                                            }
                                                        }
                                                        let _e3528 = phi_3830_;
                                                        switch bitcast<i32>(_e3479) {
                                                            case 1: {
                                                                let _e3564 = abs(_e3317.member_1.y);
                                                                let _e3566 = (_e3564 % 1f);
                                                                if (_e3564 >= 1f) {
                                                                    phi_8456_ = select(true, false, (_e3566 == 0f));
                                                                } else {
                                                                    phi_8456_ = true;
                                                                }
                                                                let _e3570 = phi_8456_;
                                                                let _e3571 = select(1f, _e3566, _e3570);
                                                                if (select(-1f, 1f, (_e3317.member_1.y >= 0f)) > 0f) {
                                                                    phi_3851_ = _e3571;
                                                                } else {
                                                                    phi_3851_ = (1f - _e3571);
                                                                }
                                                                let _e3575 = phi_3851_;
                                                                phi_3888_ = _e3575;
                                                                break;
                                                            }
                                                            case 2: {
                                                                let _e3538 = abs(_e3317.member_1.y);
                                                                let _e3545 = ((select(select(u32(_e3538), 0u, (_e3538 < 0f)), 4294967295u, (_e3538 > 4294967000f)) % 2u) == 0u);
                                                                let _e3547 = (_e3538 % 1f);
                                                                if (_e3538 >= 1f) {
                                                                    phi_8439_ = select(true, false, (_e3547 == 0f));
                                                                } else {
                                                                    phi_8439_ = true;
                                                                }
                                                                let _e3551 = phi_8439_;
                                                                let _e3552 = select(1f, _e3547, _e3551);
                                                                if (select(-1f, 1f, (_e3317.member_1.y >= 0f)) > 0f) {
                                                                    if _e3545 {
                                                                        phi_3880_ = _e3552;
                                                                    } else {
                                                                        phi_3880_ = (1f - _e3552);
                                                                    }
                                                                    let _e3559 = phi_3880_;
                                                                    phi_3886_ = _e3559;
                                                                } else {
                                                                    if _e3545 {
                                                                        phi_3885_ = (1f - _e3552);
                                                                    } else {
                                                                        phi_3885_ = _e3552;
                                                                    }
                                                                    let _e3556 = phi_3885_;
                                                                    phi_3886_ = _e3556;
                                                                }
                                                                let _e3561 = phi_3886_;
                                                                phi_3888_ = _e3561;
                                                                break;
                                                            }
                                                            case 0: {
                                                                if (_e3317.member_1.y > 1f) {
                                                                    phi_8426_ = 0.9999999f;
                                                                } else {
                                                                    phi_8426_ = select(_e3317.member_1.y, 0.00000011920929f, (_e3317.member_1.y < 0f));
                                                                }
                                                                let _e3535 = phi_8426_;
                                                                phi_3888_ = _e3535;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_3888_ = f32();
                                                                break;
                                                            }
                                                        }
                                                        let _e3577 = phi_3888_;
                                                        let _e3579 = (_e3528 * f32(_e3457));
                                                        let _e3586 = (_e3577 * f32(_e3461));
                                                        let _e3601 = vec3<f32>((f32((select(select(u32(_e3579), 0u, (_e3579 < 0f)), 4294967295u, (_e3579 > 4294967000f)) + _e3449)) / f32(_e3132)), (f32((select(select(u32(_e3586), 0u, (_e3586 < 0f)), 4294967295u, (_e3586 > 4294967000f)) + _e3453)) / f32(_e3136)), f32(_e3465));
                                                        let _e3607 = textureSampleLevel(global_19, global_18, vec2<f32>(_e3601.x, _e3601.y), i32(_e3601.z), 0f);
                                                        phi_3947_ = select(0f, 1f, ((_e3425 - max((bitcast<f32>(_e3128) * (1f - _e3046)), bitcast<f32>(_e3123))) > _e3607.x));
                                                    } else {
                                                        phi_3947_ = 0f;
                                                    }
                                                    let _e3616 = phi_3947_;
                                                    phi_3948_ = _e3616;
                                                } else {
                                                    phi_3948_ = 0f;
                                                }
                                                let _e3618 = phi_3948_;
                                                phi_3949_ = _e3618;
                                                phi_3950_ = vec3<f32>(((fma((((1f - _e3071) * _e2990) * _e1374), 0.31830987f, ((_e3080 * _e3071) / _e3085)) * (_e2945.member_1.x * _e2988)) * _e3047), ((fma((((1f - _e3072) * _e2990) * _e1376), 0.31830987f, ((_e3080 * _e3072) / _e3085)) * (_e2945.member_1.y * _e2988)) * _e3047), ((fma((((1f - _e3073) * _e2990) * _e1378), 0.31830987f, ((_e3080 * _e3073) / _e3085)) * (_e2945.member_1.z * _e2988)) * _e3047));
                                            }
                                            let _e3620 = phi_3949_;
                                            let _e3622 = phi_3950_;
                                            phi_9154_ = _e1954;
                                            phi_4874_ = _e3620;
                                            phi_4875_ = _e3622;
                                            phi_4876_ = select(true, false, _e2982);
                                            break;
                                        }
                                        case 2: {
                                            if (_e128 >= 13u) {
                                                phi_7445_ = (_e2016.member_1 <= (_e128 - 13u));
                                            } else {
                                                phi_7445_ = false;
                                            }
                                            let _e2127 = phi_7445_;
                                            if _e2127 {
                                                let _e2130 = global_1.member[_e2016.member_1];
                                                let _e2135 = global_1.member[(_e2016.member_1 + 1u)];
                                                let _e2140 = global_1.member[(_e2016.member_1 + 2u)];
                                                let _e2146 = global_1.member[(_e2016.member_1 + 3u)];
                                                let _e2151 = global_1.member[(_e2016.member_1 + 4u)];
                                                let _e2156 = global_1.member[(_e2016.member_1 + 5u)];
                                                let _e2162 = global_1.member[(_e2016.member_1 + 6u)];
                                                let _e2167 = global_1.member[(_e2016.member_1 + 7u)];
                                                let _e2172 = global_1.member[(_e2016.member_1 + 8u)];
                                                let _e2177 = global_1.member[(_e2016.member_1 + 9u)];
                                                let _e2182 = global_1.member[(_e2016.member_1 + 10u)];
                                                let _e2187 = global_1.member[(_e2016.member_1 + 11u)];
                                                let _e2193 = global_1.member[(_e2016.member_1 + 12u)];
                                                phi_4013_ = type_39(vec3<f32>(bitcast<f32>(_e2130), bitcast<f32>(_e2135), bitcast<f32>(_e2140)), vec3<f32>(bitcast<f32>(_e2146), bitcast<f32>(_e2151), bitcast<f32>(_e2156)), bitcast<f32>(_e2162), bitcast<f32>(_e2167), vec4<f32>(bitcast<f32>(_e2172), bitcast<f32>(_e2177), bitcast<f32>(_e2182), bitcast<f32>(_e2187)), bitcast<f32>(_e2193));
                                            } else {
                                                phi_4013_ = type_39(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2197 = phi_4013_;
                                            let _e2223 = vec3<f32>((_e2077.member.x + fma(_e2116.x, _e2197.member.z, fma(_e2114.x, _e2197.member.y, (_e2112.x * _e2197.member.x)))), (_e2077.member.y + fma(_e2116.y, _e2197.member.z, fma(_e2114.y, _e2197.member.y, (_e2112.y * _e2197.member.x)))), (_e2077.member.z + fma(_e2116.z, _e2197.member.z, fma(_e2114.z, _e2197.member.y, (_e2112.z * _e2197.member.x)))));
                                            let _e2224 = (_e2223 - _e136);
                                            let _e2231 = sqrt(fma(_e2224.z, _e2224.z, fma(_e2224.x, _e2224.x, (_e2224.y * _e2224.y))));
                                            let _e2232 = (_e2231 == 0f);
                                            if _e2232 {
                                                phi_4149_ = type_40(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), 0f, 0f, 0f, 0f, 0f, 0f, 0f, false, false);
                                            } else {
                                                if _e2232 {
                                                    phi_7495_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7495_ = (_e2224 * (1f / _e2231));
                                                }
                                                let _e2236 = phi_7495_;
                                                let _e2247 = fma(_e2116.x, _e2197.member_1.z, fma(_e2114.x, _e2197.member_1.y, (_e2112.x * _e2197.member_1.x)));
                                                let _e2248 = fma(_e2116.y, _e2197.member_1.z, fma(_e2114.y, _e2197.member_1.y, (_e2112.y * _e2197.member_1.x)));
                                                let _e2249 = fma(_e2116.z, _e2197.member_1.z, fma(_e2114.z, _e2197.member_1.y, (_e2112.z * _e2197.member_1.x)));
                                                let _e2254 = sqrt(fma(_e2249, _e2249, fma(_e2247, _e2247, (_e2248 * _e2248))));
                                                if (_e2254 == 0f) {
                                                    phi_7530_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7530_ = (vec3<f32>(_e2247, _e2248, _e2249) * (1f / _e2254));
                                                }
                                                let _e2259 = phi_7530_;
                                                let _e2261 = cos(_e2197.member_2);
                                                let _e2263 = cos(_e2197.member_3);
                                                let _e2264 = (_e2261 - _e2263);
                                                let _e2276 = fma(_e2236.z, -(_e2259.z), fma(_e2236.x, -(_e2259.x), (_e2236.y * -(_e2259.y))));
                                                let _e2280 = ((_e2276 - _e2263) / _e2264);
                                                let _e2282 = select(_e2280, 0f, (_e2280 < 0f));
                                                phi_4149_ = type_40(_e2223, _e136, _e2236, _e2259, _e2231, _e2261, _e2263, _e2264, _e2276, _e2280, select(_e2282, 1f, (_e2282 > 1f)), (_e2276 > _e2261), (_e2276 > _e2263));
                                            }
                                            let _e2287 = phi_4149_;
                                            let _e2289 = (_e2287.member_4 == 0f);
                                            if _e2289 {
                                                phi_9155_ = _e1954;
                                                phi_4871_ = f32();
                                                phi_4872_ = vec3<f32>();
                                            } else {
                                                let _e2292 = (_e2197.member_5 * _e2287.member_10);
                                                let _e2296 = fma(-(_e690.z), _e299.member_3, 1f);
                                                let _e2300 = fma(0.4f, _e2296, (_e1374 * _e1386));
                                                let _e2301 = fma(0.4f, _e2296, (_e1376 * _e1386));
                                                let _e2302 = fma(0.4f, _e2296, (_e1378 * _e1386));
                                                let _e2309 = (_e1942 + _e2287.member_2);
                                                let _e2316 = sqrt(fma(_e2309.z, _e2309.z, fma(_e2309.x, _e2309.x, (_e2309.y * _e2309.y))));
                                                if (_e2316 == 0f) {
                                                    phi_7565_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7565_ = (_e2309 * (1f / _e2316));
                                                }
                                                let _e2321 = phi_7565_;
                                                let _e2322 = (_e1383 * _e1383);
                                                let _e2333 = max(fma(_e1938.z, _e2321.z, fma(_e1938.x, _e2321.x, (_e1938.y * _e2321.y))), 0f);
                                                let _e2346 = max(fma(_e1938.z, _e1942.z, fma(_e1938.x, _e1942.x, (_e1938.y * _e1942.y))), 0f);
                                                let _e2352 = fma(_e1938.z, _e2287.member_2.z, fma(_e1938.x, _e2287.member_2.x, (_e1938.y * _e2287.member_2.y)));
                                                let _e2353 = max(_e2352, 0f);
                                                let _e2354 = fma(_e690.y, _e299.member_4, 1f);
                                                let _e2355 = (_e2354 * _e2354);
                                                let _e2356 = (_e2355 * 0.125f);
                                                let _e2358 = fma(-(_e2355), 0.125f, 1f);
                                                let _e2371 = (1f - max(fma(_e2321.z, _e1942.z, fma(_e2321.x, _e1942.x, (_e2321.y * _e1942.y))), 0f));
                                                let _e2373 = select(_e2371, 0f, (_e2371 < 0f));
                                                let _e2376 = pow(select(_e2373, 1f, (_e2373 > 1f)), 5f);
                                                let _e2377 = fma((1f - _e2300), _e2376, _e2300);
                                                let _e2378 = fma((1f - _e2301), _e2376, _e2301);
                                                let _e2379 = fma((1f - _e2302), _e2376, _e2302);
                                                let _e2386 = (((_e2322 * _e2322) / (pow(fma((_e2333 * _e2333), fma(_e2322, _e2322, -1f), 1f), 2f) * 3.1415927f)) * ((_e2346 / fma(_e2346, _e2358, _e2356)) * (_e2353 / fma(_e2353, _e2358, _e2356))));
                                                let _e2391 = fma((4f * _e2346), _e2353, 0.0001f);
                                                if ((_e2016.member_3 == 4294967295u) != true) {
                                                    let _e2413 = global_1.member[_e2016.member_3];
                                                    let _e2417 = global_1.member[(_e2016.member_3 + 1u)];
                                                    let _e2421 = global_1.member[(_e2016.member_3 + 2u)];
                                                    let _e2425 = global_1.member[(_e2016.member_3 + 3u)];
                                                    let _e2429 = global_1.member[(_e2016.member_3 + 4u)];
                                                    let _e2434 = global_1.member[(_e2016.member_3 + 5u)];
                                                    let _e2439 = global_1.member[(_e2016.member_3 + 6u)];
                                                    let _e2442 = global_1.member[_e1951];
                                                    let _e2446 = global_1.member[(_e1951 + 1u)];
                                                    let _e2448 = select(_e2413, 4294967295u, (0u >= _e2417));
                                                    let _e2451 = global_1.member[_e2448];
                                                    let _e2456 = global_1.member[(_e2448 + 1u)];
                                                    let _e2461 = global_1.member[(_e2448 + 2u)];
                                                    let _e2466 = global_1.member[(_e2448 + 3u)];
                                                    let _e2471 = global_1.member[(_e2448 + 4u)];
                                                    let _e2476 = global_1.member[(_e2448 + 5u)];
                                                    let _e2481 = global_1.member[(_e2448 + 6u)];
                                                    let _e2486 = global_1.member[(_e2448 + 7u)];
                                                    let _e2491 = global_1.member[(_e2448 + 8u)];
                                                    let _e2496 = global_1.member[(_e2448 + 9u)];
                                                    let _e2501 = global_1.member[(_e2448 + 10u)];
                                                    let _e2506 = global_1.member[(_e2448 + 11u)];
                                                    let _e2511 = global_1.member[(_e2448 + 12u)];
                                                    let _e2516 = global_1.member[(_e2448 + 13u)];
                                                    let _e2521 = global_1.member[(_e2448 + 14u)];
                                                    let _e2526 = global_1.member[(_e2448 + 15u)];
                                                    let _e2546 = (bitcast<f32>(_e2526) + fma(bitcast<f32>(_e2506), _e136.z, fma(bitcast<f32>(_e2486), _e136.y, (bitcast<f32>(_e2466) * _e136.x))));
                                                    let _e2547 = ((bitcast<f32>(_e2511) + fma(bitcast<f32>(_e2491), _e136.z, fma(bitcast<f32>(_e2471), _e136.y, (bitcast<f32>(_e2451) * _e136.x)))) / _e2546);
                                                    let _e2548 = ((bitcast<f32>(_e2516) + fma(bitcast<f32>(_e2496), _e136.z, fma(bitcast<f32>(_e2476), _e136.y, (bitcast<f32>(_e2456) * _e136.x)))) / _e2546);
                                                    let _e2549 = ((bitcast<f32>(_e2521) + fma(bitcast<f32>(_e2501), _e136.z, fma(bitcast<f32>(_e2481), _e136.y, (bitcast<f32>(_e2461) * _e136.x)))) / _e2546);
                                                    if (abs(_e2547) <= 1f) {
                                                        let _e2553 = (abs(_e2548) <= 1f);
                                                        if _e2553 {
                                                            phi_7670_ = (abs(_e2549) <= 1f);
                                                        } else {
                                                            phi_7670_ = bool();
                                                        }
                                                        let _e2557 = phi_7670_;
                                                        phi_7673_ = _e2557;
                                                        phi_7674_ = select(true, false, _e2553);
                                                    } else {
                                                        phi_7673_ = bool();
                                                        phi_7674_ = true;
                                                    }
                                                    let _e2560 = phi_7673_;
                                                    let _e2562 = phi_7674_;
                                                    if select(_e2560, false, _e2562) {
                                                        let _e2570 = global_1.member[select(_e2421, 4294967295u, (0u >= _e2425))];
                                                        let _e2573 = global_1.member[_e2570];
                                                        let _e2577 = global_1.member[(_e2570 + 1u)];
                                                        let _e2581 = global_1.member[(_e2570 + 2u)];
                                                        let _e2585 = global_1.member[(_e2570 + 3u)];
                                                        let _e2589 = global_1.member[(_e2570 + 4u)];
                                                        let _e2593 = global_1.member[(_e2570 + 6u)];
                                                        switch bitcast<i32>(_e2593) {
                                                            case 0: {
                                                                phi_4511_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_4511_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_4511_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_4511_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e2596 = phi_4511_;
                                                        let _e2600 = global_1.member[(_e2570 + 7u)];
                                                        switch bitcast<i32>(_e2600) {
                                                            case 0: {
                                                                phi_4520_ = 0u;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_4520_ = 1u;
                                                                break;
                                                            }
                                                            case 2: {
                                                                phi_4520_ = 2u;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_4520_ = 0u;
                                                                break;
                                                            }
                                                        }
                                                        let _e2603 = phi_4520_;
                                                        let _e2604 = bitcast<i32>(_e2439);
                                                        let _e2606 = f32(_e2581);
                                                        let _e2607 = f32(_e2585);
                                                        let _e2611 = type_35((_e2604 / -2i), (_e2604 / 2i), false);
                                                        phi_9005_ = _e1954;
                                                        phi_4548_ = _e2611;
                                                        phi_4551_ = 0f;
                                                        phi_4553_ = 0f;
                                                        loop {
                                                            let _e2613 = phi_9005_;
                                                            let _e2615 = phi_4548_;
                                                            let _e2617 = phi_4551_;
                                                            let _e2619 = phi_4553_;
                                                            local_2 = _e2617;
                                                            local_3 = _e2619;
                                                            if _e2615.member_2 {
                                                                phi_4565_ = true;
                                                            } else {
                                                                phi_4565_ = ((_e2615.member <= _e2615.member_1) != true);
                                                            }
                                                            let _e2626 = phi_4565_;
                                                            if _e2626 {
                                                                phi_4549_ = _e2615;
                                                                phi_4608_ = type_36(0u, type_36().member_1);
                                                            } else {
                                                                if (_e2615.member < _e2615.member_1) {
                                                                    let _e2634 = (_e2615.member + 1i);
                                                                    if select(false, true, ((false == (_e2634 > _e2615.member)) != false)) {
                                                                        phi_4593_ = type_36(0u, type_36().member_1);
                                                                    } else {
                                                                        phi_4593_ = type_36(1u, _e2634);
                                                                    }
                                                                    let _e2644 = phi_4593_;
                                                                    switch bitcast<i32>(_e2644.member) {
                                                                        case 0: {
                                                                            phi_9085_ = true;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            break;
                                                                        }
                                                                    }
                                                                    phi_4605_ = type_35(_e2644.member_1, _e2615.member_1, _e2615.member_2);
                                                                } else {
                                                                    phi_4605_ = type_35(_e2615.member, _e2615.member_1, true);
                                                                }
                                                                let _e2653 = phi_4605_;
                                                                phi_4549_ = _e2653;
                                                                phi_4608_ = type_36(1u, _e2615.member);
                                                            }
                                                            let _e2659 = phi_4549_;
                                                            let _e2661 = phi_4608_;
                                                            switch bitcast<i32>(_e2661.member) {
                                                                case 0: {
                                                                    phi_9086_ = _e2613;
                                                                    phi_4552_ = f32();
                                                                    phi_4554_ = f32();
                                                                    phi_4866_ = false;
                                                                    break;
                                                                }
                                                                case 1: {
                                                                    phi_4619_ = _e2611;
                                                                    phi_4622_ = _e2617;
                                                                    phi_4624_ = _e2619;
                                                                    loop {
                                                                        let _e2666 = phi_4619_;
                                                                        let _e2668 = phi_4622_;
                                                                        let _e2670 = phi_4624_;
                                                                        local_9 = _e2668;
                                                                        local_10 = _e2670;
                                                                        if _e2666.member_2 {
                                                                            phi_4636_ = true;
                                                                        } else {
                                                                            phi_4636_ = ((_e2666.member <= _e2666.member_1) != true);
                                                                        }
                                                                        let _e2677 = phi_4636_;
                                                                        if _e2677 {
                                                                            phi_4620_ = _e2666;
                                                                            phi_4679_ = type_36(0u, type_36().member_1);
                                                                        } else {
                                                                            if (_e2666.member < _e2666.member_1) {
                                                                                let _e2685 = (_e2666.member + 1i);
                                                                                if select(false, true, ((false == (_e2685 > _e2666.member)) != false)) {
                                                                                    phi_4664_ = type_36(0u, type_36().member_1);
                                                                                } else {
                                                                                    phi_4664_ = type_36(1u, _e2685);
                                                                                }
                                                                                let _e2695 = phi_4664_;
                                                                                switch bitcast<i32>(_e2695.member) {
                                                                                    case 0: {
                                                                                        phi_8989_ = true;
                                                                                        break;
                                                                                    }
                                                                                    case 1: {
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                phi_4676_ = type_35(_e2695.member_1, _e2666.member_1, _e2666.member_2);
                                                                            } else {
                                                                                phi_4676_ = type_35(_e2666.member, _e2666.member_1, true);
                                                                            }
                                                                            let _e2704 = phi_4676_;
                                                                            phi_4620_ = _e2704;
                                                                            phi_4679_ = type_36(1u, _e2666.member);
                                                                        }
                                                                        let _e2710 = phi_4620_;
                                                                        let _e2712 = phi_4679_;
                                                                        switch bitcast<i32>(_e2712.member) {
                                                                            case 0: {
                                                                                phi_4623_ = f32();
                                                                                phi_4625_ = f32();
                                                                                phi_4865_ = false;
                                                                                break;
                                                                            }
                                                                            case 1: {
                                                                                let _e2720 = fma((_e2547 + 1f), 0.5f, (f32(_e2661.member_1) * (1f / _e2606)));
                                                                                let _e2721 = fma(fma(_e2548, -1f, 1f), 0.5f, (f32(_e2712.member_1) * (1f / _e2607)));
                                                                                switch bitcast<i32>(_e2596) {
                                                                                    case 1: {
                                                                                        let _e2756 = abs(_e2720);
                                                                                        let _e2758 = (_e2756 % 1f);
                                                                                        if (_e2756 >= 1f) {
                                                                                            phi_7726_ = select(true, false, (_e2758 == 0f));
                                                                                        } else {
                                                                                            phi_7726_ = true;
                                                                                        }
                                                                                        let _e2762 = phi_7726_;
                                                                                        let _e2763 = select(1f, _e2758, _e2762);
                                                                                        if (select(-1f, 1f, (_e2720 >= 0f)) > 0f) {
                                                                                            phi_4711_ = _e2763;
                                                                                        } else {
                                                                                            phi_4711_ = (1f - _e2763);
                                                                                        }
                                                                                        let _e2767 = phi_4711_;
                                                                                        phi_4748_ = _e2767;
                                                                                        break;
                                                                                    }
                                                                                    case 2: {
                                                                                        let _e2730 = abs(_e2720);
                                                                                        let _e2737 = ((select(select(u32(_e2730), 0u, (_e2730 < 0f)), 4294967295u, (_e2730 > 4294967000f)) % 2u) == 0u);
                                                                                        let _e2739 = (_e2730 % 1f);
                                                                                        if (_e2730 >= 1f) {
                                                                                            phi_7709_ = select(true, false, (_e2739 == 0f));
                                                                                        } else {
                                                                                            phi_7709_ = true;
                                                                                        }
                                                                                        let _e2743 = phi_7709_;
                                                                                        let _e2744 = select(1f, _e2739, _e2743);
                                                                                        if (select(-1f, 1f, (_e2720 >= 0f)) > 0f) {
                                                                                            if _e2737 {
                                                                                                phi_4740_ = _e2744;
                                                                                            } else {
                                                                                                phi_4740_ = (1f - _e2744);
                                                                                            }
                                                                                            let _e2751 = phi_4740_;
                                                                                            phi_4746_ = _e2751;
                                                                                        } else {
                                                                                            if _e2737 {
                                                                                                phi_4745_ = (1f - _e2744);
                                                                                            } else {
                                                                                                phi_4745_ = _e2744;
                                                                                            }
                                                                                            let _e2748 = phi_4745_;
                                                                                            phi_4746_ = _e2748;
                                                                                        }
                                                                                        let _e2753 = phi_4746_;
                                                                                        phi_4748_ = _e2753;
                                                                                        break;
                                                                                    }
                                                                                    case 0: {
                                                                                        if (_e2720 > 1f) {
                                                                                            phi_7696_ = 0.9999999f;
                                                                                        } else {
                                                                                            phi_7696_ = select(_e2720, 0.00000011920929f, (_e2720 < 0f));
                                                                                        }
                                                                                        let _e2727 = phi_7696_;
                                                                                        phi_4748_ = _e2727;
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        phi_4748_ = f32();
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2769 = phi_4748_;
                                                                                switch bitcast<i32>(_e2603) {
                                                                                    case 1: {
                                                                                        let _e2804 = abs(_e2721);
                                                                                        let _e2806 = (_e2804 % 1f);
                                                                                        if (_e2804 >= 1f) {
                                                                                            phi_7774_ = select(true, false, (_e2806 == 0f));
                                                                                        } else {
                                                                                            phi_7774_ = true;
                                                                                        }
                                                                                        let _e2810 = phi_7774_;
                                                                                        let _e2811 = select(1f, _e2806, _e2810);
                                                                                        if (select(-1f, 1f, (_e2721 >= 0f)) > 0f) {
                                                                                            phi_4767_ = _e2811;
                                                                                        } else {
                                                                                            phi_4767_ = (1f - _e2811);
                                                                                        }
                                                                                        let _e2815 = phi_4767_;
                                                                                        phi_4804_ = _e2815;
                                                                                        break;
                                                                                    }
                                                                                    case 2: {
                                                                                        let _e2778 = abs(_e2721);
                                                                                        let _e2785 = ((select(select(u32(_e2778), 0u, (_e2778 < 0f)), 4294967295u, (_e2778 > 4294967000f)) % 2u) == 0u);
                                                                                        let _e2787 = (_e2778 % 1f);
                                                                                        if (_e2778 >= 1f) {
                                                                                            phi_7757_ = select(true, false, (_e2787 == 0f));
                                                                                        } else {
                                                                                            phi_7757_ = true;
                                                                                        }
                                                                                        let _e2791 = phi_7757_;
                                                                                        let _e2792 = select(1f, _e2787, _e2791);
                                                                                        if (select(-1f, 1f, (_e2721 >= 0f)) > 0f) {
                                                                                            if _e2785 {
                                                                                                phi_4796_ = _e2792;
                                                                                            } else {
                                                                                                phi_4796_ = (1f - _e2792);
                                                                                            }
                                                                                            let _e2799 = phi_4796_;
                                                                                            phi_4802_ = _e2799;
                                                                                        } else {
                                                                                            if _e2785 {
                                                                                                phi_4801_ = (1f - _e2792);
                                                                                            } else {
                                                                                                phi_4801_ = _e2792;
                                                                                            }
                                                                                            let _e2796 = phi_4801_;
                                                                                            phi_4802_ = _e2796;
                                                                                        }
                                                                                        let _e2801 = phi_4802_;
                                                                                        phi_4804_ = _e2801;
                                                                                        break;
                                                                                    }
                                                                                    case 0: {
                                                                                        if (_e2721 > 1f) {
                                                                                            phi_7744_ = 0.9999999f;
                                                                                        } else {
                                                                                            phi_7744_ = select(_e2721, 0.00000011920929f, (_e2721 < 0f));
                                                                                        }
                                                                                        let _e2775 = phi_7744_;
                                                                                        phi_4804_ = _e2775;
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        phi_4804_ = f32();
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2817 = phi_4804_;
                                                                                let _e2818 = (_e2769 * _e2606);
                                                                                let _e2824 = (_e2817 * _e2607);
                                                                                let _e2839 = vec3<f32>((f32((select(select(u32(_e2818), 0u, (_e2818 < 0f)), 4294967295u, (_e2818 > 4294967000f)) + _e2573)) / f32(_e2442)), (f32((select(select(u32(_e2824), 0u, (_e2824 < 0f)), 4294967295u, (_e2824 > 4294967000f)) + _e2577)) / f32(_e2446)), f32(_e2589));
                                                                                let _e2845 = textureSampleLevel(global_19, global_18, vec2<f32>(_e2839.x, _e2839.y), i32(_e2839.z), 0f);
                                                                                phi_4623_ = (_e2668 + 1f);
                                                                                phi_4625_ = (_e2670 + select(0f, 1f, ((_e2549 - max((bitcast<f32>(_e2434) * (1f - _e2352)), bitcast<f32>(_e2429))) > _e2845.x)));
                                                                                phi_4865_ = true;
                                                                                break;
                                                                            }
                                                                            default: {
                                                                                phi_4623_ = f32();
                                                                                phi_4625_ = f32();
                                                                                phi_4865_ = bool();
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e2856 = phi_4623_;
                                                                        let _e2858 = phi_4625_;
                                                                        let _e2860 = phi_4865_;
                                                                        continue;
                                                                        continuing {
                                                                            phi_4619_ = _e2710;
                                                                            phi_4622_ = _e2856;
                                                                            phi_4624_ = _e2858;
                                                                            phi_8989_ = _e2613;
                                                                            break if !(_e2860);
                                                                        }
                                                                    }
                                                                    let _e2863 = phi_8989_;
                                                                    phi_9085_ = _e2863;
                                                                    if _e2863 {
                                                                        break;
                                                                    }
                                                                    phi_9086_ = _e2863;
                                                                    let _e4619 = local_9;
                                                                    phi_4552_ = _e4619;
                                                                    let _e4622 = local_10;
                                                                    phi_4554_ = _e4622;
                                                                    phi_4866_ = true;
                                                                    break;
                                                                }
                                                                default: {
                                                                    phi_9086_ = _e2613;
                                                                    phi_4552_ = f32();
                                                                    phi_4554_ = f32();
                                                                    phi_4866_ = bool();
                                                                    break;
                                                                }
                                                            }
                                                            let _e2865 = phi_9086_;
                                                            let _e2867 = phi_4552_;
                                                            let _e2869 = phi_4554_;
                                                            let _e2871 = phi_4866_;
                                                            continue;
                                                            continuing {
                                                                phi_9005_ = _e2865;
                                                                phi_4548_ = _e2659;
                                                                phi_4551_ = _e2867;
                                                                phi_4553_ = _e2869;
                                                                phi_9085_ = _e2865;
                                                                break if !(_e2871);
                                                            }
                                                        }
                                                        let _e2874 = phi_9085_;
                                                        phi_9150_ = _e2874;
                                                        if _e2874 {
                                                            break;
                                                        }
                                                        let _e2876 = local_2;
                                                        let _e2879 = local_3;
                                                        phi_9157_ = _e2874;
                                                        phi_4869_ = (_e2879 / max(_e2876, 1f));
                                                    } else {
                                                        phi_9157_ = _e1954;
                                                        phi_4869_ = 0f;
                                                    }
                                                    let _e2882 = phi_9157_;
                                                    let _e2884 = phi_4869_;
                                                    phi_9156_ = _e2882;
                                                    phi_4870_ = _e2884;
                                                } else {
                                                    phi_9156_ = _e1954;
                                                    phi_4870_ = 0f;
                                                }
                                                let _e2886 = phi_9156_;
                                                let _e2888 = phi_4870_;
                                                phi_9155_ = _e2886;
                                                phi_4871_ = _e2888;
                                                phi_4872_ = vec3<f32>(((fma((((1f - _e2377) * _e2296) * _e1374), 0.31830987f, ((_e2386 * _e2377) / _e2391)) * (_e2197.member_4.x * _e2292)) * _e2353), ((fma((((1f - _e2378) * _e2296) * _e1376), 0.31830987f, ((_e2386 * _e2378) / _e2391)) * (_e2197.member_4.y * _e2292)) * _e2353), ((fma((((1f - _e2379) * _e2296) * _e1378), 0.31830987f, ((_e2386 * _e2379) / _e2391)) * (_e2197.member_4.z * _e2292)) * _e2353));
                                            }
                                            let _e2890 = phi_9155_;
                                            let _e2892 = phi_4871_;
                                            let _e2894 = phi_4872_;
                                            phi_9154_ = _e2890;
                                            phi_4874_ = _e2892;
                                            phi_4875_ = _e2894;
                                            phi_4876_ = select(true, false, _e2289);
                                            break;
                                        }
                                        default: {
                                            phi_9154_ = _e1954;
                                            phi_4874_ = f32();
                                            phi_4875_ = vec3<f32>();
                                            phi_4876_ = bool();
                                            break;
                                        }
                                    }
                                    let _e4315 = phi_9154_;
                                    let _e4317 = phi_4874_;
                                    let _e4319 = phi_4875_;
                                    let _e4321 = phi_4876_;
                                    if _e4321 {
                                        let _e4322 = (1f - _e4317);
                                        phi_4896_ = vec3<f32>(fma(_e4319.x, _e4322, _e1958.x), fma(_e4319.y, _e4322, _e1958.y), fma(_e4319.z, _e4322, _e1958.z));
                                    } else {
                                        phi_4896_ = vec3<f32>();
                                    }
                                    let _e4334 = phi_4896_;
                                    phi_9151_ = _e4315;
                                    phi_2391_ = select(_e4334, _e1958, vec3(select(true, false, _e4321)));
                                    phi_4902_ = true;
                                    break;
                                }
                                default: {
                                    phi_9151_ = _e1954;
                                    phi_2391_ = vec3<f32>();
                                    phi_4902_ = bool();
                                    break;
                                }
                            }
                            let _e4339 = phi_9151_;
                            let _e4341 = phi_2391_;
                            let _e4343 = phi_4902_;
                            continue;
                            continuing {
                                phi_9032_ = _e4339;
                                phi_2387_ = _e1971;
                                phi_2390_ = _e4341;
                                phi_9150_ = _e4339;
                                break if !(_e4343);
                            }
                        }
                        let _e4346 = phi_9150_;
                        phi_9233_ = _e4346;
                        if _e4346 {
                            break;
                        }
                        let _e4348 = fma(-(_e690.z), _e299.member_3, 1f);
                        let _e4352 = fma(0.04f, _e4348, (_e1374 * _e1386));
                        let _e4353 = fma(0.04f, _e4348, (_e1376 * _e1386));
                        let _e4354 = fma(0.04f, _e4348, (_e1378 * _e1386));
                        let _e4366 = fma(-(_e690.y), _e299.member_4, 1f);
                        let _e4373 = (1f - max(fma(_e1938.z, _e1942.z, fma(_e1938.x, _e1942.x, (_e1938.y * _e1942.y))), 0f));
                        let _e4375 = select(_e4373, 0f, (_e4373 < 0f));
                        let _e4378 = pow(select(_e4375, 1f, (_e4375 > 1f)), 5f);
                        let _e4379 = fma((max(_e4366, _e4352) - _e4352), _e4378, _e4352);
                        let _e4380 = fma((max(_e4366, _e4353) - _e4353), _e4378, _e4353);
                        let _e4381 = fma((max(_e4366, _e4354) - _e4354), _e4378, _e4354);
                        let _e4401 = local_6;
                        let _e4405 = local_7;
                        let _e4409 = local_8;
                        phi_9241_ = _e4346;
                        phi_5019_ = vec4<f32>(fma(_e1396, _e299.member_1, fma(fma(((1f - _e4379) * _e4348), (_e1405.x * _e1374), (_e1753.x * fma(_e4379, _e1769.x, _e1769.y))), _e1390, _e4401.x)), fma(_e1398, _e299.member_1, fma(fma(((1f - _e4380) * _e4348), (_e1405.y * _e1376), (_e1753.y * fma(_e4380, _e1769.x, _e1769.y))), _e1390, _e4405.y)), fma(_e1400, _e299.member_1, fma(fma(((1f - _e4381) * _e4348), (_e1405.z * _e1378), (_e1753.z * fma(_e4381, _e1769.x, _e1769.y))), _e1390, _e4409.z)), 1f);
                    } else {
                        phi_9241_ = false;
                        phi_5019_ = (vec4<f32>((_e130.x * _e496.x), (_e130.y * _e496.y), (_e130.z * _e496.z), (_e130.w * _e496.w)) * _e299.member_2);
                    }
                    let _e4417 = phi_9241_;
                    let _e4419 = phi_5019_;
                    global_20 = _e4419;
                    phi_9233_ = _e4417;
                    break;
                }
                case 1: {
                    let _e1911 = sqrt(fma(_e131.x, _e131.x, (_e131.y * _e131.y)));
                    if (_e1911 == 0f) {
                        phi_7190_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7190_ = (vec3<f32>(_e131.x, _e131.y, 0f) * (1f / _e1911));
                    }
                    let _e1916 = phi_7190_;
                    global_20 = vec4<f32>(((_e1916.x + 1f) * 0.5f), ((_e1916.y + 1f) * 0.5f), ((_e1916.z + 1f) * 0.5f), 1f);
                    phi_9233_ = false;
                    break;
                }
                case 2: {
                    let _e1890 = sqrt(fma(_e132.x, _e132.x, (_e132.y * _e132.y)));
                    if (_e1890 == 0f) {
                        phi_7141_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7141_ = (vec3<f32>(_e132.x, _e132.y, 0f) * (1f / _e1890));
                    }
                    let _e1895 = phi_7141_;
                    global_20 = vec4<f32>(((_e1895.x + 1f) * 0.5f), ((_e1895.y + 1f) * 0.5f), ((_e1895.z + 1f) * 0.5f), 1f);
                    phi_9233_ = false;
                    break;
                }
                case 3: {
                    if _e1732 {
                        phi_7092_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7092_ = (_e1359 * (1f / _e1731));
                    }
                    let _e1874 = phi_7092_;
                    global_20 = vec4<f32>(((_e1874.x + 1f) * 0.5f), ((_e1874.y + 1f) * 0.5f), ((_e1874.z + 1f) * 0.5f), 1f);
                    phi_9233_ = false;
                    break;
                }
                case 4: {
                    global_20 = _e130;
                    phi_9233_ = false;
                    break;
                }
                case 5: {
                    let _e1855 = sqrt(fma(_e133.z, _e133.z, fma(_e133.x, _e133.x, (_e133.y * _e133.y))));
                    if (_e1855 == 0f) {
                        phi_7043_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_7043_ = (_e133 * (1f / _e1855));
                    }
                    let _e1860 = phi_7043_;
                    global_20 = vec4<f32>(((_e1860.x + 1f) * 0.5f), ((_e1860.y + 1f) * 0.5f), ((_e1860.z + 1f) * 0.5f), 1f);
                    phi_9233_ = false;
                    break;
                }
                case 6: {
                    let _e1833 = sqrt(fma(_e1357.z, _e1357.z, fma(_e1357.x, _e1357.x, (_e1357.y * _e1357.y))));
                    if (_e1833 == 0f) {
                        phi_6994_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6994_ = (_e1357 * (1f / _e1833));
                    }
                    let _e1838 = phi_6994_;
                    global_20 = vec4<f32>(((_e1838.x + 1f) * 0.5f), ((_e1838.y + 1f) * 0.5f), ((_e1838.z + 1f) * 0.5f), 1f);
                    phi_9233_ = false;
                    break;
                }
                case 7: {
                    let _e1811 = sqrt(fma(_e134.z, _e134.z, fma(_e134.x, _e134.x, (_e134.y * _e134.y))));
                    if (_e1811 == 0f) {
                        phi_6945_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6945_ = (_e134 * (1f / _e1811));
                    }
                    let _e1816 = phi_6945_;
                    global_20 = vec4<f32>(((_e1816.x + 1f) * 0.5f), ((_e1816.y + 1f) * 0.5f), ((_e1816.z + 1f) * 0.5f), 1f);
                    phi_9233_ = false;
                    break;
                }
                case 8: {
                    let _e1789 = sqrt(fma(_e135.z, _e135.z, fma(_e135.x, _e135.x, (_e135.y * _e135.y))));
                    if (_e1789 == 0f) {
                        phi_6896_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6896_ = (_e135 * (1f / _e1789));
                    }
                    let _e1794 = phi_6896_;
                    global_20 = vec4<f32>(((_e1794.x + 1f) * 0.5f), ((_e1794.y + 1f) * 0.5f), ((_e1794.z + 1f) * 0.5f), 1f);
                    phi_9233_ = false;
                    break;
                }
                case 9: {
                    global_20 = vec4<f32>(_e1405.x, _e1405.y, _e1405.z, 1f);
                    phi_9233_ = false;
                    break;
                }
                case 10: {
                    global_20 = vec4<f32>(_e1753.x, _e1753.y, _e1753.z, 1f);
                    phi_9233_ = false;
                    break;
                }
                case 11: {
                    global_20 = vec4<f32>(_e1769.x, _e1769.y, 1f, 1f);
                    phi_9233_ = false;
                    break;
                }
                case 12: {
                    global_20 = (vec4<f32>(_e1363, _e1366, _e1369, (_e496.w * _e299.member_2.w)) * _e130);
                    phi_9233_ = false;
                    break;
                }
                case 13: {
                    global_20 = vec4<f32>(_e1383, _e1383, _e1383, 1f);
                    phi_9233_ = false;
                    break;
                }
                case 14: {
                    global_20 = vec4<f32>(_e1386, _e1386, _e1386, 1f);
                    phi_9233_ = false;
                    break;
                }
                case 15: {
                    global_20 = vec4<f32>(_e1390, _e1390, _e1390, 1f);
                    phi_9233_ = false;
                    break;
                }
                case 16: {
                    global_20 = vec4<f32>((_e1396 * _e299.member_1), (_e1398 * _e299.member_1), (_e1400 * _e299.member_1), 1f);
                    phi_9233_ = false;
                    break;
                }
                case 17: {
                    global_20 = vec4<f32>(_e1272.x, _e1272.y, _e1272.z, 1f);
                    phi_9233_ = false;
                    break;
                }
                case 18: {
                    global_20 = vec4<f32>(_e299.member.x, _e299.member.y, _e299.member.z, 1f);
                    phi_9233_ = false;
                    break;
                }
                case 19: {
                    global_20 = vec4<f32>(_e299.member_1, _e299.member_1, _e299.member_1, 1f);
                    phi_9233_ = false;
                    break;
                }
                default: {
                    phi_9233_ = false;
                    break;
                }
            }
            let _e4421 = phi_9233_;
            if _e4421 {
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
