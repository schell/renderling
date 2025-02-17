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
    var phi_690_: u32;
    var phi_5168_: bool;
    var phi_818_: type_33;
    var phi_822_: type_33;
    var phi_5205_: bool;
    var phi_862_: u32;
    var phi_871_: u32;
    var phi_884_: type_15;
    var phi_5227_: f32;
    var phi_5240_: bool;
    var phi_938_: f32;
    var phi_933_: f32;
    var phi_939_: f32;
    var phi_5257_: bool;
    var phi_904_: f32;
    var phi_941_: f32;
    var phi_5275_: f32;
    var phi_5288_: bool;
    var phi_996_: f32;
    var phi_991_: f32;
    var phi_997_: f32;
    var phi_5305_: bool;
    var phi_962_: f32;
    var phi_999_: f32;
    var phi_5341_: bool;
    var phi_1082_: u32;
    var phi_1091_: u32;
    var phi_1104_: type_15;
    var phi_5362_: f32;
    var phi_5375_: bool;
    var phi_1158_: f32;
    var phi_1153_: f32;
    var phi_1159_: f32;
    var phi_5392_: bool;
    var phi_1124_: f32;
    var phi_1161_: f32;
    var phi_5410_: f32;
    var phi_5423_: bool;
    var phi_1216_: f32;
    var phi_1211_: f32;
    var phi_1217_: f32;
    var phi_5440_: bool;
    var phi_1182_: f32;
    var phi_1219_: f32;
    var phi_5476_: bool;
    var phi_1302_: u32;
    var phi_1311_: u32;
    var phi_1324_: type_15;
    var phi_5497_: f32;
    var phi_5510_: bool;
    var phi_1378_: f32;
    var phi_1373_: f32;
    var phi_1379_: f32;
    var phi_5527_: bool;
    var phi_1344_: f32;
    var phi_1381_: f32;
    var phi_5545_: f32;
    var phi_5558_: bool;
    var phi_1436_: f32;
    var phi_1431_: f32;
    var phi_1437_: f32;
    var phi_5575_: bool;
    var phi_1402_: f32;
    var phi_1439_: f32;
    var phi_5611_: bool;
    var phi_1522_: u32;
    var phi_1531_: u32;
    var phi_1544_: type_15;
    var phi_5632_: f32;
    var phi_5645_: bool;
    var phi_1598_: f32;
    var phi_1593_: f32;
    var phi_1599_: f32;
    var phi_5662_: bool;
    var phi_1564_: f32;
    var phi_1601_: f32;
    var phi_5680_: f32;
    var phi_5693_: bool;
    var phi_1656_: f32;
    var phi_1651_: f32;
    var phi_1657_: f32;
    var phi_5710_: bool;
    var phi_1622_: f32;
    var phi_1659_: f32;
    var phi_5746_: bool;
    var phi_1742_: u32;
    var phi_1751_: u32;
    var phi_1764_: type_15;
    var phi_5767_: f32;
    var phi_5780_: bool;
    var phi_1818_: f32;
    var phi_1813_: f32;
    var phi_1819_: f32;
    var phi_5797_: bool;
    var phi_1784_: f32;
    var phi_1821_: f32;
    var phi_5815_: f32;
    var phi_5828_: bool;
    var phi_1876_: f32;
    var phi_1871_: f32;
    var phi_1877_: f32;
    var phi_5845_: bool;
    var phi_1842_: f32;
    var phi_1879_: f32;
    var phi_5903_: vec3<f32>;
    var phi_5938_: vec3<f32>;
    var phi_5973_: vec3<f32>;
    var phi_6008_: vec3<f32>;
    var phi_6043_: vec3<f32>;
    var phi_1973_: vec3<f32>;
    var phi_1974_: vec3<f32>;
    var phi_6075_: bool;
    var phi_2181_: type_14;
    var phi_2182_: type_14;
    var phi_2205_: type_14;
    var phi_2232_: bool;
    var phi_2238_: type_14;
    var phi_2239_: type_14;
    var phi_2262_: type_14;
    var phi_2285_: bool;
    var phi_2306_: type_25;
    var phi_6147_: vec3<f32>;
    var phi_6206_: vec3<f32>;
    var phi_6280_: vec3<f32>;
    var phi_6340_: vec3<f32>;
    var phi_6389_: vec3<f32>;
    var phi_6438_: vec3<f32>;
    var phi_6487_: vec3<f32>;
    var phi_6536_: vec3<f32>;
    var phi_6585_: vec3<f32>;
    var phi_6634_: vec3<f32>;
    var phi_6673_: vec3<f32>;
    var phi_6708_: vec3<f32>;
    var phi_7968_: bool;
    var phi_2373_: type_14;
    var phi_2376_: vec3<f32>;
    var phi_2374_: type_14;
    var phi_2399_: type_14;
    var phi_6734_: u32;
    var phi_6753_: bool;
    var phi_2416_: u32;
    var phi_6777_: bool;
    var phi_2428_: u32;
    var phi_2442_: type_30;
    var phi_6809_: bool;
    var phi_2492_: type_31;
    var phi_6889_: bool;
    var phi_3599_: type_37;
    var phi_6939_: vec3<f32>;
    var phi_6974_: vec3<f32>;
    var phi_3735_: type_38;
    var phi_7009_: vec3<f32>;
    var phi_3947_: u32;
    var phi_3956_: u32;
    var phi_7118_: bool;
    var phi_7121_: bool;
    var phi_7122_: bool;
    var phi_7941_: bool;
    var phi_4135_: type_35;
    var phi_4138_: f32;
    var phi_4140_: f32;
    var phi_4152_: bool;
    var phi_4180_: type_36;
    var phi_4192_: type_35;
    var phi_4136_: type_35;
    var phi_4195_: type_36;
    var phi_4206_: type_35;
    var phi_4209_: f32;
    var phi_4211_: f32;
    var phi_4223_: bool;
    var phi_4251_: type_36;
    var phi_4263_: type_35;
    var phi_4207_: type_35;
    var phi_4266_: type_36;
    var phi_7140_: f32;
    var phi_7153_: bool;
    var phi_4332_: f32;
    var phi_4327_: f32;
    var phi_4333_: f32;
    var phi_7170_: bool;
    var phi_4298_: f32;
    var phi_4335_: f32;
    var phi_7188_: f32;
    var phi_7201_: bool;
    var phi_4388_: f32;
    var phi_4383_: f32;
    var phi_4389_: f32;
    var phi_7218_: bool;
    var phi_4354_: f32;
    var phi_4391_: f32;
    var phi_4210_: f32;
    var phi_4212_: f32;
    var phi_4453_: bool;
    var phi_7925_: bool;
    var phi_8022_: bool;
    var phi_4139_: f32;
    var phi_4141_: f32;
    var phi_4454_: bool;
    var phi_8021_: bool;
    var local_2: f32;
    var local_3: f32;
    var phi_8093_: bool;
    var phi_4457_: f32;
    var phi_8092_: bool;
    var phi_4458_: f32;
    var phi_8091_: bool;
    var phi_4472_: vec3<f32>;
    var phi_7255_: bool;
    var phi_3346_: type_34;
    var phi_7302_: vec3<f32>;
    var phi_7337_: vec3<f32>;
    var phi_3536_: vec3<f32>;
    var phi_7429_: bool;
    var phi_2540_: type_34;
    var phi_7476_: vec3<f32>;
    var phi_7511_: vec3<f32>;
    var phi_2781_: u32;
    var phi_2790_: u32;
    var phi_7620_: bool;
    var phi_7623_: bool;
    var phi_7624_: bool;
    var phi_8039_: bool;
    var phi_2969_: type_35;
    var phi_2972_: f32;
    var phi_2974_: f32;
    var phi_2986_: bool;
    var phi_3014_: type_36;
    var phi_3026_: type_35;
    var phi_2970_: type_35;
    var phi_3029_: type_36;
    var phi_3040_: type_35;
    var phi_3043_: f32;
    var phi_3045_: f32;
    var phi_3057_: bool;
    var phi_3085_: type_36;
    var phi_3097_: type_35;
    var phi_3041_: type_35;
    var phi_3100_: type_36;
    var phi_7642_: f32;
    var phi_7655_: bool;
    var phi_3166_: f32;
    var phi_3161_: f32;
    var phi_3167_: f32;
    var phi_7672_: bool;
    var phi_3132_: f32;
    var phi_3169_: f32;
    var phi_7690_: f32;
    var phi_7703_: bool;
    var phi_3222_: f32;
    var phi_3217_: f32;
    var phi_3223_: f32;
    var phi_7720_: bool;
    var phi_3188_: f32;
    var phi_3225_: f32;
    var phi_3044_: f32;
    var phi_3046_: f32;
    var phi_3287_: bool;
    var phi_8023_: bool;
    var phi_8085_: bool;
    var phi_2973_: f32;
    var phi_2975_: f32;
    var phi_3288_: bool;
    var phi_8084_: bool;
    var local_4: f32;
    var local_5: f32;
    var phi_8103_: bool;
    var phi_3291_: f32;
    var phi_8102_: bool;
    var phi_3292_: f32;
    var phi_8090_: bool;
    var phi_4474_: vec3<f32>;
    var phi_4475_: bool;
    var phi_8087_: bool;
    var phi_2377_: vec3<f32>;
    var phi_4484_: bool;
    var phi_8086_: bool;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var local_8: vec3<f32>;
    var phi_8112_: bool;
    var phi_4601_: vec4<f32>;
    var phi_8104_: bool;
    var local_9: f32;
    var local_10: f32;
    var local_11: f32;
    var local_12: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e122 = arrayLength((&global.member));
            let _e124 = arrayLength((&global_1.member));
            let _e125 = global_2;
            let _e126 = global_3;
            let _e127 = global_4;
            let _e128 = global_5;
            let _e129 = global_6;
            let _e130 = global_7;
            let _e131 = global_8;
            let _e132 = global_9;
            let _e136 = global.member[(_e125 + 9u)];
            let _e140 = global.member[(_e125 + 11u)];
            let _e144 = global.member[(_e125 + 17u)];
            let _e147 = global.member[_e144];
            let _e151 = global.member[(_e144 + 1u)];
            let _e155 = global.member[(_e144 + 4u)];
            switch bitcast<i32>(_e155) {
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
                case 3: {
                    phi_690_ = 3u;
                    break;
                }
                case 4: {
                    phi_690_ = 4u;
                    break;
                }
                case 5: {
                    phi_690_ = 5u;
                    break;
                }
                case 6: {
                    phi_690_ = 6u;
                    break;
                }
                case 7: {
                    phi_690_ = 7u;
                    break;
                }
                case 8: {
                    phi_690_ = 8u;
                    break;
                }
                case 9: {
                    phi_690_ = 9u;
                    break;
                }
                case 10: {
                    phi_690_ = 10u;
                    break;
                }
                case 11: {
                    phi_690_ = 11u;
                    break;
                }
                case 12: {
                    phi_690_ = 12u;
                    break;
                }
                case 13: {
                    phi_690_ = 13u;
                    break;
                }
                case 14: {
                    phi_690_ = 14u;
                    break;
                }
                case 15: {
                    phi_690_ = 15u;
                    break;
                }
                case 16: {
                    phi_690_ = 16u;
                    break;
                }
                case 17: {
                    phi_690_ = 17u;
                    break;
                }
                case 18: {
                    phi_690_ = 18u;
                    break;
                }
                case 19: {
                    phi_690_ = 19u;
                    break;
                }
                default: {
                    phi_690_ = 0u;
                    break;
                }
            }
            let _e158 = phi_690_;
            let _e162 = global.member[(_e144 + 5u)];
            if (_e140 == 4294967295u) {
                phi_822_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
            } else {
                if (_e122 >= 22u) {
                    phi_5168_ = (_e140 <= (_e122 - 22u));
                } else {
                    phi_5168_ = false;
                }
                let _e169 = phi_5168_;
                if _e169 {
                    let _e172 = global.member[_e140];
                    let _e177 = global.member[(_e140 + 1u)];
                    let _e182 = global.member[(_e140 + 2u)];
                    let _e188 = global.member[(_e140 + 3u)];
                    let _e193 = global.member[(_e140 + 4u)];
                    let _e198 = global.member[(_e140 + 5u)];
                    let _e203 = global.member[(_e140 + 6u)];
                    let _e208 = global.member[(_e140 + 7u)];
                    let _e214 = global.member[(_e140 + 8u)];
                    let _e219 = global.member[(_e140 + 9u)];
                    let _e224 = global.member[(_e140 + 10u)];
                    let _e228 = global.member[(_e140 + 11u)];
                    let _e232 = global.member[(_e140 + 12u)];
                    let _e236 = global.member[(_e140 + 13u)];
                    let _e240 = global.member[(_e140 + 14u)];
                    let _e244 = global.member[(_e140 + 15u)];
                    let _e248 = global.member[(_e140 + 16u)];
                    let _e252 = global.member[(_e140 + 17u)];
                    let _e256 = global.member[(_e140 + 18u)];
                    let _e260 = global.member[(_e140 + 19u)];
                    let _e264 = global.member[(_e140 + 20u)];
                    let _e269 = global.member[(_e140 + 21u)];
                    phi_818_ = type_33(vec3<f32>(bitcast<f32>(_e172), bitcast<f32>(_e177), bitcast<f32>(_e182)), bitcast<f32>(_e188), vec4<f32>(bitcast<f32>(_e193), bitcast<f32>(_e198), bitcast<f32>(_e203), bitcast<f32>(_e208)), bitcast<f32>(_e214), bitcast<f32>(_e219), _e224, _e228, _e232, _e236, _e240, _e244, _e248, _e252, _e256, _e260, (_e264 == 1u), bitcast<f32>(_e269));
                } else {
                    phi_818_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
                }
                let _e273 = phi_818_;
                phi_822_ = type_33(_e273.member, _e273.member_1, _e273.member_2, _e273.member_3, _e273.member_4, _e273.member_5, _e273.member_6, _e273.member_7, _e273.member_8, _e273.member_9, _e273.member_10, _e273.member_11, _e273.member_12, _e273.member_13, _e273.member_14, (_e273.member_15 && (_e162 == 1u)), _e273.member_16);
            }
            let _e295 = phi_822_;
            let _e299 = select(_e128, _e127, vec2((_e295.member_10 == 0u)));
            let _e301 = (_e122 >= 8u);
            if _e301 {
                phi_5205_ = (_e295.member_5 <= (_e122 - 8u));
            } else {
                phi_5205_ = false;
            }
            let _e305 = phi_5205_;
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
                        phi_862_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_862_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_862_ = 2u;
                        break;
                    }
                    default: {
                        phi_862_ = 0u;
                        break;
                    }
                }
                let _e337 = phi_862_;
                let _e341 = global.member[(_e295.member_5 + 7u)];
                switch bitcast<i32>(_e341) {
                    case 0: {
                        phi_871_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_871_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_871_ = 2u;
                        break;
                    }
                    default: {
                        phi_871_ = 0u;
                        break;
                    }
                }
                let _e344 = phi_871_;
                phi_884_ = type_15(type_14(_e337, _e344), vec2<u32>(_e308, _e312), vec2<u32>(_e317, _e321), _e326, _e330);
            } else {
                phi_884_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e348 = phi_884_;
            switch bitcast<i32>(_e348.member.member) {
                case 1: {
                    let _e386 = abs(_e299.x);
                    let _e388 = (_e386 % 1f);
                    if (_e386 >= 1f) {
                        phi_5257_ = select(true, false, (_e388 == 0f));
                    } else {
                        phi_5257_ = true;
                    }
                    let _e392 = phi_5257_;
                    let _e393 = select(1f, _e388, _e392);
                    if (select(-1f, 1f, (_e299.x >= 0f)) > 0f) {
                        phi_904_ = _e393;
                    } else {
                        phi_904_ = (1f - _e393);
                    }
                    let _e397 = phi_904_;
                    phi_941_ = _e397;
                    break;
                }
                case 2: {
                    let _e360 = abs(_e299.x);
                    let _e367 = ((select(select(u32(_e360), 0u, (_e360 < 0f)), 4294967295u, (_e360 > 4294967000f)) % 2u) == 0u);
                    let _e369 = (_e360 % 1f);
                    if (_e360 >= 1f) {
                        phi_5240_ = select(true, false, (_e369 == 0f));
                    } else {
                        phi_5240_ = true;
                    }
                    let _e373 = phi_5240_;
                    let _e374 = select(1f, _e369, _e373);
                    if (select(-1f, 1f, (_e299.x >= 0f)) > 0f) {
                        if _e367 {
                            phi_933_ = _e374;
                        } else {
                            phi_933_ = (1f - _e374);
                        }
                        let _e381 = phi_933_;
                        phi_939_ = _e381;
                    } else {
                        if _e367 {
                            phi_938_ = (1f - _e374);
                        } else {
                            phi_938_ = _e374;
                        }
                        let _e378 = phi_938_;
                        phi_939_ = _e378;
                    }
                    let _e383 = phi_939_;
                    phi_941_ = _e383;
                    break;
                }
                case 0: {
                    if (_e299.x > 1f) {
                        phi_5227_ = 0.9999999f;
                    } else {
                        phi_5227_ = select(_e299.x, 0.00000011920929f, (_e299.x < 0f));
                    }
                    let _e357 = phi_5227_;
                    phi_941_ = _e357;
                    break;
                }
                default: {
                    phi_941_ = f32();
                    break;
                }
            }
            let _e399 = phi_941_;
            switch bitcast<i32>(_e348.member.member_1) {
                case 1: {
                    let _e437 = abs(_e299.y);
                    let _e439 = (_e437 % 1f);
                    if (_e437 >= 1f) {
                        phi_5305_ = select(true, false, (_e439 == 0f));
                    } else {
                        phi_5305_ = true;
                    }
                    let _e443 = phi_5305_;
                    let _e444 = select(1f, _e439, _e443);
                    if (select(-1f, 1f, (_e299.y >= 0f)) > 0f) {
                        phi_962_ = _e444;
                    } else {
                        phi_962_ = (1f - _e444);
                    }
                    let _e448 = phi_962_;
                    phi_999_ = _e448;
                    break;
                }
                case 2: {
                    let _e411 = abs(_e299.y);
                    let _e418 = ((select(select(u32(_e411), 0u, (_e411 < 0f)), 4294967295u, (_e411 > 4294967000f)) % 2u) == 0u);
                    let _e420 = (_e411 % 1f);
                    if (_e411 >= 1f) {
                        phi_5288_ = select(true, false, (_e420 == 0f));
                    } else {
                        phi_5288_ = true;
                    }
                    let _e424 = phi_5288_;
                    let _e425 = select(1f, _e420, _e424);
                    if (select(-1f, 1f, (_e299.y >= 0f)) > 0f) {
                        if _e418 {
                            phi_991_ = _e425;
                        } else {
                            phi_991_ = (1f - _e425);
                        }
                        let _e432 = phi_991_;
                        phi_997_ = _e432;
                    } else {
                        if _e418 {
                            phi_996_ = (1f - _e425);
                        } else {
                            phi_996_ = _e425;
                        }
                        let _e429 = phi_996_;
                        phi_997_ = _e429;
                    }
                    let _e434 = phi_997_;
                    phi_999_ = _e434;
                    break;
                }
                case 0: {
                    if (_e299.y > 1f) {
                        phi_5275_ = 0.9999999f;
                    } else {
                        phi_5275_ = select(_e299.y, 0.00000011920929f, (_e299.y < 0f));
                    }
                    let _e408 = phi_5275_;
                    phi_999_ = _e408;
                    break;
                }
                default: {
                    phi_999_ = f32();
                    break;
                }
            }
            let _e450 = phi_999_;
            let _e454 = (_e399 * f32(_e348.member_2.x));
            let _e463 = (_e450 * f32(_e348.member_2.y));
            let _e475 = f32(_e147);
            let _e476 = f32(_e151);
            let _e483 = vec3<f32>((f32((select(select(u32(_e454), 0u, (_e454 < 0f)), 4294967295u, (_e454 > 4294967000f)) + _e348.member_1.x)) / _e475), (f32((select(select(u32(_e463), 0u, (_e463 < 0f)), 4294967295u, (_e463 > 4294967000f)) + _e348.member_1.y)) / _e476), f32(_e348.member_3));
            let _e489 = textureSampleLevel(global_11, global_10, vec2<f32>(_e483.x, _e483.y), i32(_e483.z), 0f);
            let _e492 = select(_e489, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_5 == 4294967295u)));
            let _e496 = select(_e128, _e127, vec2((_e295.member_11 == 0u)));
            if _e301 {
                phi_5341_ = (_e295.member_6 <= (_e122 - 8u));
            } else {
                phi_5341_ = false;
            }
            let _e501 = phi_5341_;
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
                        phi_1082_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1082_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1082_ = 2u;
                        break;
                    }
                    default: {
                        phi_1082_ = 0u;
                        break;
                    }
                }
                let _e533 = phi_1082_;
                let _e537 = global.member[(_e295.member_6 + 7u)];
                switch bitcast<i32>(_e537) {
                    case 0: {
                        phi_1091_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1091_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1091_ = 2u;
                        break;
                    }
                    default: {
                        phi_1091_ = 0u;
                        break;
                    }
                }
                let _e540 = phi_1091_;
                phi_1104_ = type_15(type_14(_e533, _e540), vec2<u32>(_e504, _e508), vec2<u32>(_e513, _e517), _e522, _e526);
            } else {
                phi_1104_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e544 = phi_1104_;
            switch bitcast<i32>(_e544.member.member) {
                case 1: {
                    let _e582 = abs(_e496.x);
                    let _e584 = (_e582 % 1f);
                    if (_e582 >= 1f) {
                        phi_5392_ = select(true, false, (_e584 == 0f));
                    } else {
                        phi_5392_ = true;
                    }
                    let _e588 = phi_5392_;
                    let _e589 = select(1f, _e584, _e588);
                    if (select(-1f, 1f, (_e496.x >= 0f)) > 0f) {
                        phi_1124_ = _e589;
                    } else {
                        phi_1124_ = (1f - _e589);
                    }
                    let _e593 = phi_1124_;
                    phi_1161_ = _e593;
                    break;
                }
                case 2: {
                    let _e556 = abs(_e496.x);
                    let _e563 = ((select(select(u32(_e556), 0u, (_e556 < 0f)), 4294967295u, (_e556 > 4294967000f)) % 2u) == 0u);
                    let _e565 = (_e556 % 1f);
                    if (_e556 >= 1f) {
                        phi_5375_ = select(true, false, (_e565 == 0f));
                    } else {
                        phi_5375_ = true;
                    }
                    let _e569 = phi_5375_;
                    let _e570 = select(1f, _e565, _e569);
                    if (select(-1f, 1f, (_e496.x >= 0f)) > 0f) {
                        if _e563 {
                            phi_1153_ = _e570;
                        } else {
                            phi_1153_ = (1f - _e570);
                        }
                        let _e577 = phi_1153_;
                        phi_1159_ = _e577;
                    } else {
                        if _e563 {
                            phi_1158_ = (1f - _e570);
                        } else {
                            phi_1158_ = _e570;
                        }
                        let _e574 = phi_1158_;
                        phi_1159_ = _e574;
                    }
                    let _e579 = phi_1159_;
                    phi_1161_ = _e579;
                    break;
                }
                case 0: {
                    if (_e496.x > 1f) {
                        phi_5362_ = 0.9999999f;
                    } else {
                        phi_5362_ = select(_e496.x, 0.00000011920929f, (_e496.x < 0f));
                    }
                    let _e553 = phi_5362_;
                    phi_1161_ = _e553;
                    break;
                }
                default: {
                    phi_1161_ = f32();
                    break;
                }
            }
            let _e595 = phi_1161_;
            switch bitcast<i32>(_e544.member.member_1) {
                case 1: {
                    let _e633 = abs(_e496.y);
                    let _e635 = (_e633 % 1f);
                    if (_e633 >= 1f) {
                        phi_5440_ = select(true, false, (_e635 == 0f));
                    } else {
                        phi_5440_ = true;
                    }
                    let _e639 = phi_5440_;
                    let _e640 = select(1f, _e635, _e639);
                    if (select(-1f, 1f, (_e496.y >= 0f)) > 0f) {
                        phi_1182_ = _e640;
                    } else {
                        phi_1182_ = (1f - _e640);
                    }
                    let _e644 = phi_1182_;
                    phi_1219_ = _e644;
                    break;
                }
                case 2: {
                    let _e607 = abs(_e496.y);
                    let _e614 = ((select(select(u32(_e607), 0u, (_e607 < 0f)), 4294967295u, (_e607 > 4294967000f)) % 2u) == 0u);
                    let _e616 = (_e607 % 1f);
                    if (_e607 >= 1f) {
                        phi_5423_ = select(true, false, (_e616 == 0f));
                    } else {
                        phi_5423_ = true;
                    }
                    let _e620 = phi_5423_;
                    let _e621 = select(1f, _e616, _e620);
                    if (select(-1f, 1f, (_e496.y >= 0f)) > 0f) {
                        if _e614 {
                            phi_1211_ = _e621;
                        } else {
                            phi_1211_ = (1f - _e621);
                        }
                        let _e628 = phi_1211_;
                        phi_1217_ = _e628;
                    } else {
                        if _e614 {
                            phi_1216_ = (1f - _e621);
                        } else {
                            phi_1216_ = _e621;
                        }
                        let _e625 = phi_1216_;
                        phi_1217_ = _e625;
                    }
                    let _e630 = phi_1217_;
                    phi_1219_ = _e630;
                    break;
                }
                case 0: {
                    if (_e496.y > 1f) {
                        phi_5410_ = 0.9999999f;
                    } else {
                        phi_5410_ = select(_e496.y, 0.00000011920929f, (_e496.y < 0f));
                    }
                    let _e604 = phi_5410_;
                    phi_1219_ = _e604;
                    break;
                }
                default: {
                    phi_1219_ = f32();
                    break;
                }
            }
            let _e646 = phi_1219_;
            let _e650 = (_e595 * f32(_e544.member_2.x));
            let _e659 = (_e646 * f32(_e544.member_2.y));
            let _e677 = vec3<f32>((f32((select(select(u32(_e650), 0u, (_e650 < 0f)), 4294967295u, (_e650 > 4294967000f)) + _e544.member_1.x)) / _e475), (f32((select(select(u32(_e659), 0u, (_e659 < 0f)), 4294967295u, (_e659 > 4294967000f)) + _e544.member_1.y)) / _e476), f32(_e544.member_3));
            let _e683 = textureSampleLevel(global_11, global_10, vec2<f32>(_e677.x, _e677.y), i32(_e677.z), 0f);
            let _e686 = select(_e683, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_6 == 4294967295u)));
            let _e690 = select(_e128, _e127, vec2((_e295.member_12 == 0u)));
            if _e301 {
                phi_5476_ = (_e295.member_7 <= (_e122 - 8u));
            } else {
                phi_5476_ = false;
            }
            let _e695 = phi_5476_;
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
                        phi_1302_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1302_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1302_ = 2u;
                        break;
                    }
                    default: {
                        phi_1302_ = 0u;
                        break;
                    }
                }
                let _e727 = phi_1302_;
                let _e731 = global.member[(_e295.member_7 + 7u)];
                switch bitcast<i32>(_e731) {
                    case 0: {
                        phi_1311_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1311_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1311_ = 2u;
                        break;
                    }
                    default: {
                        phi_1311_ = 0u;
                        break;
                    }
                }
                let _e734 = phi_1311_;
                phi_1324_ = type_15(type_14(_e727, _e734), vec2<u32>(_e698, _e702), vec2<u32>(_e707, _e711), _e716, _e720);
            } else {
                phi_1324_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e738 = phi_1324_;
            switch bitcast<i32>(_e738.member.member) {
                case 1: {
                    let _e776 = abs(_e690.x);
                    let _e778 = (_e776 % 1f);
                    if (_e776 >= 1f) {
                        phi_5527_ = select(true, false, (_e778 == 0f));
                    } else {
                        phi_5527_ = true;
                    }
                    let _e782 = phi_5527_;
                    let _e783 = select(1f, _e778, _e782);
                    if (select(-1f, 1f, (_e690.x >= 0f)) > 0f) {
                        phi_1344_ = _e783;
                    } else {
                        phi_1344_ = (1f - _e783);
                    }
                    let _e787 = phi_1344_;
                    phi_1381_ = _e787;
                    break;
                }
                case 2: {
                    let _e750 = abs(_e690.x);
                    let _e757 = ((select(select(u32(_e750), 0u, (_e750 < 0f)), 4294967295u, (_e750 > 4294967000f)) % 2u) == 0u);
                    let _e759 = (_e750 % 1f);
                    if (_e750 >= 1f) {
                        phi_5510_ = select(true, false, (_e759 == 0f));
                    } else {
                        phi_5510_ = true;
                    }
                    let _e763 = phi_5510_;
                    let _e764 = select(1f, _e759, _e763);
                    if (select(-1f, 1f, (_e690.x >= 0f)) > 0f) {
                        if _e757 {
                            phi_1373_ = _e764;
                        } else {
                            phi_1373_ = (1f - _e764);
                        }
                        let _e771 = phi_1373_;
                        phi_1379_ = _e771;
                    } else {
                        if _e757 {
                            phi_1378_ = (1f - _e764);
                        } else {
                            phi_1378_ = _e764;
                        }
                        let _e768 = phi_1378_;
                        phi_1379_ = _e768;
                    }
                    let _e773 = phi_1379_;
                    phi_1381_ = _e773;
                    break;
                }
                case 0: {
                    if (_e690.x > 1f) {
                        phi_5497_ = 0.9999999f;
                    } else {
                        phi_5497_ = select(_e690.x, 0.00000011920929f, (_e690.x < 0f));
                    }
                    let _e747 = phi_5497_;
                    phi_1381_ = _e747;
                    break;
                }
                default: {
                    phi_1381_ = f32();
                    break;
                }
            }
            let _e789 = phi_1381_;
            switch bitcast<i32>(_e738.member.member_1) {
                case 1: {
                    let _e827 = abs(_e690.y);
                    let _e829 = (_e827 % 1f);
                    if (_e827 >= 1f) {
                        phi_5575_ = select(true, false, (_e829 == 0f));
                    } else {
                        phi_5575_ = true;
                    }
                    let _e833 = phi_5575_;
                    let _e834 = select(1f, _e829, _e833);
                    if (select(-1f, 1f, (_e690.y >= 0f)) > 0f) {
                        phi_1402_ = _e834;
                    } else {
                        phi_1402_ = (1f - _e834);
                    }
                    let _e838 = phi_1402_;
                    phi_1439_ = _e838;
                    break;
                }
                case 2: {
                    let _e801 = abs(_e690.y);
                    let _e808 = ((select(select(u32(_e801), 0u, (_e801 < 0f)), 4294967295u, (_e801 > 4294967000f)) % 2u) == 0u);
                    let _e810 = (_e801 % 1f);
                    if (_e801 >= 1f) {
                        phi_5558_ = select(true, false, (_e810 == 0f));
                    } else {
                        phi_5558_ = true;
                    }
                    let _e814 = phi_5558_;
                    let _e815 = select(1f, _e810, _e814);
                    if (select(-1f, 1f, (_e690.y >= 0f)) > 0f) {
                        if _e808 {
                            phi_1431_ = _e815;
                        } else {
                            phi_1431_ = (1f - _e815);
                        }
                        let _e822 = phi_1431_;
                        phi_1437_ = _e822;
                    } else {
                        if _e808 {
                            phi_1436_ = (1f - _e815);
                        } else {
                            phi_1436_ = _e815;
                        }
                        let _e819 = phi_1436_;
                        phi_1437_ = _e819;
                    }
                    let _e824 = phi_1437_;
                    phi_1439_ = _e824;
                    break;
                }
                case 0: {
                    if (_e690.y > 1f) {
                        phi_5545_ = 0.9999999f;
                    } else {
                        phi_5545_ = select(_e690.y, 0.00000011920929f, (_e690.y < 0f));
                    }
                    let _e798 = phi_5545_;
                    phi_1439_ = _e798;
                    break;
                }
                default: {
                    phi_1439_ = f32();
                    break;
                }
            }
            let _e840 = phi_1439_;
            let _e844 = (_e789 * f32(_e738.member_2.x));
            let _e853 = (_e840 * f32(_e738.member_2.y));
            let _e871 = vec3<f32>((f32((select(select(u32(_e844), 0u, (_e844 < 0f)), 4294967295u, (_e844 > 4294967000f)) + _e738.member_1.x)) / _e475), (f32((select(select(u32(_e853), 0u, (_e853 < 0f)), 4294967295u, (_e853 > 4294967000f)) + _e738.member_1.y)) / _e476), f32(_e738.member_3));
            let _e877 = textureSampleLevel(global_11, global_10, vec2<f32>(_e871.x, _e871.y), i32(_e871.z), 0f);
            let _e878 = (_e295.member_7 == 4294967295u);
            let _e880 = select(_e877, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e878));
            let _e884 = select(_e128, _e127, vec2((_e295.member_13 == 0u)));
            if _e301 {
                phi_5611_ = (_e295.member_8 <= (_e122 - 8u));
            } else {
                phi_5611_ = false;
            }
            let _e889 = phi_5611_;
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
                        phi_1522_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1522_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1522_ = 2u;
                        break;
                    }
                    default: {
                        phi_1522_ = 0u;
                        break;
                    }
                }
                let _e921 = phi_1522_;
                let _e925 = global.member[(_e295.member_8 + 7u)];
                switch bitcast<i32>(_e925) {
                    case 0: {
                        phi_1531_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1531_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1531_ = 2u;
                        break;
                    }
                    default: {
                        phi_1531_ = 0u;
                        break;
                    }
                }
                let _e928 = phi_1531_;
                phi_1544_ = type_15(type_14(_e921, _e928), vec2<u32>(_e892, _e896), vec2<u32>(_e901, _e905), _e910, _e914);
            } else {
                phi_1544_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e932 = phi_1544_;
            switch bitcast<i32>(_e932.member.member) {
                case 1: {
                    let _e970 = abs(_e884.x);
                    let _e972 = (_e970 % 1f);
                    if (_e970 >= 1f) {
                        phi_5662_ = select(true, false, (_e972 == 0f));
                    } else {
                        phi_5662_ = true;
                    }
                    let _e976 = phi_5662_;
                    let _e977 = select(1f, _e972, _e976);
                    if (select(-1f, 1f, (_e884.x >= 0f)) > 0f) {
                        phi_1564_ = _e977;
                    } else {
                        phi_1564_ = (1f - _e977);
                    }
                    let _e981 = phi_1564_;
                    phi_1601_ = _e981;
                    break;
                }
                case 2: {
                    let _e944 = abs(_e884.x);
                    let _e951 = ((select(select(u32(_e944), 0u, (_e944 < 0f)), 4294967295u, (_e944 > 4294967000f)) % 2u) == 0u);
                    let _e953 = (_e944 % 1f);
                    if (_e944 >= 1f) {
                        phi_5645_ = select(true, false, (_e953 == 0f));
                    } else {
                        phi_5645_ = true;
                    }
                    let _e957 = phi_5645_;
                    let _e958 = select(1f, _e953, _e957);
                    if (select(-1f, 1f, (_e884.x >= 0f)) > 0f) {
                        if _e951 {
                            phi_1593_ = _e958;
                        } else {
                            phi_1593_ = (1f - _e958);
                        }
                        let _e965 = phi_1593_;
                        phi_1599_ = _e965;
                    } else {
                        if _e951 {
                            phi_1598_ = (1f - _e958);
                        } else {
                            phi_1598_ = _e958;
                        }
                        let _e962 = phi_1598_;
                        phi_1599_ = _e962;
                    }
                    let _e967 = phi_1599_;
                    phi_1601_ = _e967;
                    break;
                }
                case 0: {
                    if (_e884.x > 1f) {
                        phi_5632_ = 0.9999999f;
                    } else {
                        phi_5632_ = select(_e884.x, 0.00000011920929f, (_e884.x < 0f));
                    }
                    let _e941 = phi_5632_;
                    phi_1601_ = _e941;
                    break;
                }
                default: {
                    phi_1601_ = f32();
                    break;
                }
            }
            let _e983 = phi_1601_;
            switch bitcast<i32>(_e932.member.member_1) {
                case 1: {
                    let _e1021 = abs(_e884.y);
                    let _e1023 = (_e1021 % 1f);
                    if (_e1021 >= 1f) {
                        phi_5710_ = select(true, false, (_e1023 == 0f));
                    } else {
                        phi_5710_ = true;
                    }
                    let _e1027 = phi_5710_;
                    let _e1028 = select(1f, _e1023, _e1027);
                    if (select(-1f, 1f, (_e884.y >= 0f)) > 0f) {
                        phi_1622_ = _e1028;
                    } else {
                        phi_1622_ = (1f - _e1028);
                    }
                    let _e1032 = phi_1622_;
                    phi_1659_ = _e1032;
                    break;
                }
                case 2: {
                    let _e995 = abs(_e884.y);
                    let _e1002 = ((select(select(u32(_e995), 0u, (_e995 < 0f)), 4294967295u, (_e995 > 4294967000f)) % 2u) == 0u);
                    let _e1004 = (_e995 % 1f);
                    if (_e995 >= 1f) {
                        phi_5693_ = select(true, false, (_e1004 == 0f));
                    } else {
                        phi_5693_ = true;
                    }
                    let _e1008 = phi_5693_;
                    let _e1009 = select(1f, _e1004, _e1008);
                    if (select(-1f, 1f, (_e884.y >= 0f)) > 0f) {
                        if _e1002 {
                            phi_1651_ = _e1009;
                        } else {
                            phi_1651_ = (1f - _e1009);
                        }
                        let _e1016 = phi_1651_;
                        phi_1657_ = _e1016;
                    } else {
                        if _e1002 {
                            phi_1656_ = (1f - _e1009);
                        } else {
                            phi_1656_ = _e1009;
                        }
                        let _e1013 = phi_1656_;
                        phi_1657_ = _e1013;
                    }
                    let _e1018 = phi_1657_;
                    phi_1659_ = _e1018;
                    break;
                }
                case 0: {
                    if (_e884.y > 1f) {
                        phi_5680_ = 0.9999999f;
                    } else {
                        phi_5680_ = select(_e884.y, 0.00000011920929f, (_e884.y < 0f));
                    }
                    let _e992 = phi_5680_;
                    phi_1659_ = _e992;
                    break;
                }
                default: {
                    phi_1659_ = f32();
                    break;
                }
            }
            let _e1034 = phi_1659_;
            let _e1038 = (_e983 * f32(_e932.member_2.x));
            let _e1047 = (_e1034 * f32(_e932.member_2.y));
            let _e1065 = vec3<f32>((f32((select(select(u32(_e1038), 0u, (_e1038 < 0f)), 4294967295u, (_e1038 > 4294967000f)) + _e932.member_1.x)) / _e475), (f32((select(select(u32(_e1047), 0u, (_e1047 < 0f)), 4294967295u, (_e1047 > 4294967000f)) + _e932.member_1.y)) / _e476), f32(_e932.member_3));
            let _e1071 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1065.x, _e1065.y), i32(_e1065.z), 0f);
            let _e1078 = select(_e128, _e127, vec2((_e295.member_14 == 0u)));
            if _e301 {
                phi_5746_ = (_e295.member_9 <= (_e122 - 8u));
            } else {
                phi_5746_ = false;
            }
            let _e1083 = phi_5746_;
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
                        phi_1742_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1742_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1742_ = 2u;
                        break;
                    }
                    default: {
                        phi_1742_ = 0u;
                        break;
                    }
                }
                let _e1115 = phi_1742_;
                let _e1119 = global.member[(_e295.member_9 + 7u)];
                switch bitcast<i32>(_e1119) {
                    case 0: {
                        phi_1751_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1751_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1751_ = 2u;
                        break;
                    }
                    default: {
                        phi_1751_ = 0u;
                        break;
                    }
                }
                let _e1122 = phi_1751_;
                phi_1764_ = type_15(type_14(_e1115, _e1122), vec2<u32>(_e1086, _e1090), vec2<u32>(_e1095, _e1099), _e1104, _e1108);
            } else {
                phi_1764_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e1126 = phi_1764_;
            switch bitcast<i32>(_e1126.member.member) {
                case 1: {
                    let _e1164 = abs(_e1078.x);
                    let _e1166 = (_e1164 % 1f);
                    if (_e1164 >= 1f) {
                        phi_5797_ = select(true, false, (_e1166 == 0f));
                    } else {
                        phi_5797_ = true;
                    }
                    let _e1170 = phi_5797_;
                    let _e1171 = select(1f, _e1166, _e1170);
                    if (select(-1f, 1f, (_e1078.x >= 0f)) > 0f) {
                        phi_1784_ = _e1171;
                    } else {
                        phi_1784_ = (1f - _e1171);
                    }
                    let _e1175 = phi_1784_;
                    phi_1821_ = _e1175;
                    break;
                }
                case 2: {
                    let _e1138 = abs(_e1078.x);
                    let _e1145 = ((select(select(u32(_e1138), 0u, (_e1138 < 0f)), 4294967295u, (_e1138 > 4294967000f)) % 2u) == 0u);
                    let _e1147 = (_e1138 % 1f);
                    if (_e1138 >= 1f) {
                        phi_5780_ = select(true, false, (_e1147 == 0f));
                    } else {
                        phi_5780_ = true;
                    }
                    let _e1151 = phi_5780_;
                    let _e1152 = select(1f, _e1147, _e1151);
                    if (select(-1f, 1f, (_e1078.x >= 0f)) > 0f) {
                        if _e1145 {
                            phi_1813_ = _e1152;
                        } else {
                            phi_1813_ = (1f - _e1152);
                        }
                        let _e1159 = phi_1813_;
                        phi_1819_ = _e1159;
                    } else {
                        if _e1145 {
                            phi_1818_ = (1f - _e1152);
                        } else {
                            phi_1818_ = _e1152;
                        }
                        let _e1156 = phi_1818_;
                        phi_1819_ = _e1156;
                    }
                    let _e1161 = phi_1819_;
                    phi_1821_ = _e1161;
                    break;
                }
                case 0: {
                    if (_e1078.x > 1f) {
                        phi_5767_ = 0.9999999f;
                    } else {
                        phi_5767_ = select(_e1078.x, 0.00000011920929f, (_e1078.x < 0f));
                    }
                    let _e1135 = phi_5767_;
                    phi_1821_ = _e1135;
                    break;
                }
                default: {
                    phi_1821_ = f32();
                    break;
                }
            }
            let _e1177 = phi_1821_;
            switch bitcast<i32>(_e1126.member.member_1) {
                case 1: {
                    let _e1215 = abs(_e1078.y);
                    let _e1217 = (_e1215 % 1f);
                    if (_e1215 >= 1f) {
                        phi_5845_ = select(true, false, (_e1217 == 0f));
                    } else {
                        phi_5845_ = true;
                    }
                    let _e1221 = phi_5845_;
                    let _e1222 = select(1f, _e1217, _e1221);
                    if (select(-1f, 1f, (_e1078.y >= 0f)) > 0f) {
                        phi_1842_ = _e1222;
                    } else {
                        phi_1842_ = (1f - _e1222);
                    }
                    let _e1226 = phi_1842_;
                    phi_1879_ = _e1226;
                    break;
                }
                case 2: {
                    let _e1189 = abs(_e1078.y);
                    let _e1196 = ((select(select(u32(_e1189), 0u, (_e1189 < 0f)), 4294967295u, (_e1189 > 4294967000f)) % 2u) == 0u);
                    let _e1198 = (_e1189 % 1f);
                    if (_e1189 >= 1f) {
                        phi_5828_ = select(true, false, (_e1198 == 0f));
                    } else {
                        phi_5828_ = true;
                    }
                    let _e1202 = phi_5828_;
                    let _e1203 = select(1f, _e1198, _e1202);
                    if (select(-1f, 1f, (_e1078.y >= 0f)) > 0f) {
                        if _e1196 {
                            phi_1871_ = _e1203;
                        } else {
                            phi_1871_ = (1f - _e1203);
                        }
                        let _e1210 = phi_1871_;
                        phi_1877_ = _e1210;
                    } else {
                        if _e1196 {
                            phi_1876_ = (1f - _e1203);
                        } else {
                            phi_1876_ = _e1203;
                        }
                        let _e1207 = phi_1876_;
                        phi_1877_ = _e1207;
                    }
                    let _e1212 = phi_1877_;
                    phi_1879_ = _e1212;
                    break;
                }
                case 0: {
                    if (_e1078.y > 1f) {
                        phi_5815_ = 0.9999999f;
                    } else {
                        phi_5815_ = select(_e1078.y, 0.00000011920929f, (_e1078.y < 0f));
                    }
                    let _e1186 = phi_5815_;
                    phi_1879_ = _e1186;
                    break;
                }
                default: {
                    phi_1879_ = f32();
                    break;
                }
            }
            let _e1228 = phi_1879_;
            let _e1232 = (_e1177 * f32(_e1126.member_2.x));
            let _e1241 = (_e1228 * f32(_e1126.member_2.y));
            let _e1259 = vec3<f32>((f32((select(select(u32(_e1232), 0u, (_e1232 < 0f)), 4294967295u, (_e1232 > 4294967000f)) + _e1126.member_1.x)) / _e475), (f32((select(select(u32(_e1241), 0u, (_e1241 < 0f)), 4294967295u, (_e1241 > 4294967000f)) + _e1126.member_1.y)) / _e476), f32(_e1126.member_3));
            let _e1265 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1259.x, _e1259.y), i32(_e1259.z), 0f);
            let _e1268 = select(_e1265, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_9 == 4294967295u)));
            if _e878 {
                phi_1973_ = vec3<f32>(0f, 0f, 0f);
                phi_1974_ = _e129;
            } else {
                let _e1272 = fma(_e880.x, 2f, -1f);
                let _e1273 = fma(_e880.y, 2f, -1f);
                let _e1274 = fma(_e880.z, 2f, -1f);
                let _e1279 = sqrt(fma(_e1274, _e1274, fma(_e1272, _e1272, (_e1273 * _e1273))));
                if (_e1279 == 0f) {
                    phi_5903_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5903_ = (vec3<f32>(_e1272, _e1273, _e1274) * (1f / _e1279));
                }
                let _e1284 = phi_5903_;
                let _e1291 = sqrt(fma(_e130.z, _e130.z, fma(_e130.x, _e130.x, (_e130.y * _e130.y))));
                if (_e1291 == 0f) {
                    phi_5938_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5938_ = (_e130 * (1f / _e1291));
                }
                let _e1296 = phi_5938_;
                let _e1303 = sqrt(fma(_e131.z, _e131.z, fma(_e131.x, _e131.x, (_e131.y * _e131.y))));
                if (_e1303 == 0f) {
                    phi_5973_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5973_ = (_e131 * (1f / _e1303));
                }
                let _e1308 = phi_5973_;
                let _e1315 = sqrt(fma(_e129.z, _e129.z, fma(_e129.x, _e129.x, (_e129.y * _e129.y))));
                if (_e1315 == 0f) {
                    phi_6008_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6008_ = (_e129 * (1f / _e1315));
                }
                let _e1320 = phi_6008_;
                let _e1339 = fma(_e1320.x, _e1284.z, fma(_e1296.x, _e1284.x, (_e1308.x * _e1284.y)));
                let _e1340 = fma(_e1320.y, _e1284.z, fma(_e1296.y, _e1284.x, (_e1308.y * _e1284.y)));
                let _e1341 = fma(_e1320.z, _e1284.z, fma(_e1296.z, _e1284.x, (_e1308.z * _e1284.y)));
                let _e1346 = sqrt(fma(_e1341, _e1341, fma(_e1339, _e1339, (_e1340 * _e1340))));
                if (_e1346 == 0f) {
                    phi_6043_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_6043_ = (vec3<f32>(_e1339, _e1340, _e1341) * (1f / _e1346));
                }
                let _e1351 = phi_6043_;
                phi_1973_ = _e1284;
                phi_1974_ = _e1351;
            }
            let _e1353 = phi_1973_;
            let _e1355 = phi_1974_;
            let _e1359 = (_e492.x * _e295.member_2.x);
            let _e1362 = (_e492.y * _e295.member_2.y);
            let _e1365 = (_e492.z * _e295.member_2.z);
            let _e1370 = (_e1359 * _e126.x);
            let _e1372 = (_e1362 * _e126.y);
            let _e1374 = (_e1365 * _e126.z);
            let _e1379 = (_e686.y * _e295.member_4);
            let _e1382 = (_e686.z * _e295.member_3);
            let _e1386 = fma(_e295.member_16, (select(_e1071, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e295.member_8 == 4294967295u))).x - 1f), 1f);
            let _e1392 = (_e1268.x * _e295.member.x);
            let _e1394 = (_e1268.y * _e295.member.y);
            let _e1396 = (_e1268.z * _e295.member.z);
            let _e1401 = textureSampleLevel(global_12, global_13, _e1355, 0f);
            if (_e122 >= 86u) {
                phi_6075_ = (_e136 <= (_e122 - 86u));
            } else {
                phi_6075_ = false;
            }
            let _e1409 = phi_6075_;
            if _e1409 {
                let _e1412 = global.member[_e136];
                let _e1417 = global.member[(_e136 + 1u)];
                let _e1422 = global.member[(_e136 + 2u)];
                let _e1427 = global.member[(_e136 + 3u)];
                let _e1433 = global.member[(_e136 + 4u)];
                let _e1438 = global.member[(_e136 + 5u)];
                let _e1443 = global.member[(_e136 + 6u)];
                let _e1448 = global.member[(_e136 + 7u)];
                let _e1454 = global.member[(_e136 + 8u)];
                let _e1459 = global.member[(_e136 + 9u)];
                let _e1464 = global.member[(_e136 + 10u)];
                let _e1469 = global.member[(_e136 + 11u)];
                let _e1475 = global.member[(_e136 + 12u)];
                let _e1480 = global.member[(_e136 + 13u)];
                let _e1485 = global.member[(_e136 + 14u)];
                let _e1490 = global.member[(_e136 + 15u)];
                let _e1497 = global.member[(_e136 + 16u)];
                let _e1502 = global.member[(_e136 + 17u)];
                let _e1507 = global.member[(_e136 + 18u)];
                let _e1512 = global.member[(_e136 + 19u)];
                let _e1518 = global.member[(_e136 + 20u)];
                let _e1523 = global.member[(_e136 + 21u)];
                let _e1528 = global.member[(_e136 + 22u)];
                let _e1533 = global.member[(_e136 + 23u)];
                let _e1539 = global.member[(_e136 + 24u)];
                let _e1544 = global.member[(_e136 + 25u)];
                let _e1549 = global.member[(_e136 + 26u)];
                let _e1554 = global.member[(_e136 + 27u)];
                let _e1560 = global.member[(_e136 + 28u)];
                let _e1565 = global.member[(_e136 + 29u)];
                let _e1570 = global.member[(_e136 + 30u)];
                let _e1575 = global.member[(_e136 + 31u)];
                let _e1582 = global.member[(_e136 + 32u)];
                let _e1587 = global.member[(_e136 + 33u)];
                let _e1592 = global.member[(_e136 + 34u)];
                local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_2181_ = type_14(0u, 6u);
                loop {
                    let _e1597 = phi_2181_;
                    if (_e1597.member < _e1597.member_1) {
                        phi_2182_ = type_14((_e1597.member + 1u), _e1597.member_1);
                        phi_2205_ = type_14(1u, _e1597.member);
                    } else {
                        phi_2182_ = _e1597;
                        phi_2205_ = type_14(0u, type_14().member_1);
                    }
                    let _e1610 = phi_2182_;
                    let _e1612 = phi_2205_;
                    switch bitcast<i32>(_e1612.member) {
                        case 0: {
                            phi_2232_ = false;
                            break;
                        }
                        case 1: {
                            let _e1617 = ((_e136 + 35u) + (_e1612.member_1 * 4u));
                            let _e1620 = global.member[_e1617];
                            let _e1625 = global.member[(_e1617 + 1u)];
                            let _e1630 = global.member[(_e1617 + 2u)];
                            let _e1635 = global.member[(_e1617 + 3u)];
                            local_1[_e1612.member_1] = vec4<f32>(bitcast<f32>(_e1620), bitcast<f32>(_e1625), bitcast<f32>(_e1630), bitcast<f32>(_e1635));
                            phi_2232_ = true;
                            break;
                        }
                        default: {
                            phi_2232_ = bool();
                            break;
                        }
                    }
                    let _e1640 = phi_2232_;
                    continue;
                    continuing {
                        phi_2181_ = _e1610;
                        break if !(_e1640);
                    }
                }
                let _e1642 = local_1;
                local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                phi_2238_ = type_14(0u, 8u);
                loop {
                    let _e1645 = phi_2238_;
                    if (_e1645.member < _e1645.member_1) {
                        phi_2239_ = type_14((_e1645.member + 1u), _e1645.member_1);
                        phi_2262_ = type_14(1u, _e1645.member);
                    } else {
                        phi_2239_ = _e1645;
                        phi_2262_ = type_14(0u, type_14().member_1);
                    }
                    let _e1658 = phi_2239_;
                    let _e1660 = phi_2262_;
                    switch bitcast<i32>(_e1660.member) {
                        case 0: {
                            phi_2285_ = false;
                            break;
                        }
                        case 1: {
                            let _e1665 = ((_e136 + 59u) + (_e1660.member_1 * 3u));
                            let _e1668 = global.member[_e1665];
                            let _e1673 = global.member[(_e1665 + 1u)];
                            let _e1678 = global.member[(_e1665 + 2u)];
                            local[_e1660.member_1] = vec3<f32>(bitcast<f32>(_e1668), bitcast<f32>(_e1673), bitcast<f32>(_e1678));
                            phi_2285_ = true;
                            break;
                        }
                        default: {
                            phi_2285_ = bool();
                            break;
                        }
                    }
                    let _e1683 = phi_2285_;
                    continue;
                    continuing {
                        phi_2238_ = _e1658;
                        break if !(_e1683);
                    }
                }
                let _e1685 = local;
                let _e1689 = global.member[(_e136 + 83u)];
                let _e1694 = global.member[(_e136 + 84u)];
                let _e1699 = global.member[(_e136 + 85u)];
                phi_2306_ = type_25(type_23(vec4<f32>(bitcast<f32>(_e1412), bitcast<f32>(_e1417), bitcast<f32>(_e1422), bitcast<f32>(_e1427)), vec4<f32>(bitcast<f32>(_e1433), bitcast<f32>(_e1438), bitcast<f32>(_e1443), bitcast<f32>(_e1448)), vec4<f32>(bitcast<f32>(_e1454), bitcast<f32>(_e1459), bitcast<f32>(_e1464), bitcast<f32>(_e1469)), vec4<f32>(bitcast<f32>(_e1475), bitcast<f32>(_e1480), bitcast<f32>(_e1485), bitcast<f32>(_e1490))), type_23(vec4<f32>(bitcast<f32>(_e1497), bitcast<f32>(_e1502), bitcast<f32>(_e1507), bitcast<f32>(_e1512)), vec4<f32>(bitcast<f32>(_e1518), bitcast<f32>(_e1523), bitcast<f32>(_e1528), bitcast<f32>(_e1533)), vec4<f32>(bitcast<f32>(_e1539), bitcast<f32>(_e1544), bitcast<f32>(_e1549), bitcast<f32>(_e1554)), vec4<f32>(bitcast<f32>(_e1560), bitcast<f32>(_e1565), bitcast<f32>(_e1570), bitcast<f32>(_e1575))), vec3<f32>(bitcast<f32>(_e1582), bitcast<f32>(_e1587), bitcast<f32>(_e1592)), type_24(_e1685, _e1642, vec3<f32>(bitcast<f32>(_e1689), bitcast<f32>(_e1694), bitcast<f32>(_e1699))));
            } else {
                phi_2306_ = type_25(type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_24(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
            }
            let _e1705 = phi_2306_;
            let _e1707 = (_e1705.member_2 - _e132);
            let _e1714 = sqrt(fma(_e1707.z, _e1707.z, fma(_e1707.x, _e1707.x, (_e1707.y * _e1707.y))));
            let _e1715 = (_e1714 == 0f);
            if _e1715 {
                phi_6147_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6147_ = (_e1707 * (1f / _e1714));
            }
            let _e1719 = phi_6147_;
            let _e1720 = -(_e1719);
            let _e1727 = sqrt(fma(_e1355.z, _e1355.z, fma(_e1355.x, _e1355.x, (_e1355.y * _e1355.y))));
            let _e1728 = (_e1727 == 0f);
            if _e1728 {
                phi_6206_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6206_ = (_e1355 * (1f / _e1727));
            }
            let _e1732 = phi_6206_;
            let _e1742 = (2f * fma(_e1732.z, _e1720.z, fma(_e1732.x, _e1720.x, (_e1732.y * _e1720.y))));
            let _e1749 = textureSampleLevel(global_14, global_15, (_e1720 - vec3<f32>((_e1742 * _e1732.x), (_e1742 * _e1732.y), (_e1742 * _e1732.z))), (_e1379 * 4f));
            if _e1715 {
                phi_6280_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_6280_ = (_e1707 * (1f / _e1714));
            }
            let _e1756 = phi_6280_;
            let _e1765 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1355.z, _e1756.z, fma(_e1355.x, _e1756.x, (_e1355.y * _e1756.y))), 0f), _e1379), 0f);
            switch bitcast<i32>(_e158) {
                case 0: {
                    if _e295.member_15 {
                        if _e1728 {
                            phi_6673_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_6673_ = (_e1355 * (1f / _e1727));
                        }
                        let _e1934 = phi_6673_;
                        if _e1715 {
                            phi_6708_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_6708_ = (_e1707 * (1f / _e1714));
                        }
                        let _e1938 = phi_6708_;
                        let _e1941 = global_1.member[0u];
                        let _e1944 = global_1.member[1u];
                        let _e1947 = global_1.member[2u];
                        phi_7968_ = false;
                        phi_2373_ = type_14(0u, _e1944);
                        phi_2376_ = vec3<f32>(0f, 0f, 0f);
                        loop {
                            let _e1950 = phi_7968_;
                            let _e1952 = phi_2373_;
                            let _e1954 = phi_2376_;
                            local_6 = _e1954;
                            local_7 = _e1954;
                            local_8 = _e1954;
                            if (_e1952.member < _e1952.member_1) {
                                phi_2374_ = type_14((_e1952.member + 1u), _e1952.member_1);
                                phi_2399_ = type_14(1u, _e1952.member);
                            } else {
                                phi_2374_ = _e1952;
                                phi_2399_ = type_14(0u, type_14().member_1);
                            }
                            let _e1967 = phi_2374_;
                            let _e1969 = phi_2399_;
                            switch bitcast<i32>(_e1969.member) {
                                case 0: {
                                    phi_8087_ = _e1950;
                                    phi_2377_ = vec3<f32>();
                                    phi_4484_ = false;
                                    break;
                                }
                                case 1: {
                                    if (_e1969.member_1 >= _e1944) {
                                        phi_6734_ = 4294967295u;
                                    } else {
                                        phi_6734_ = (_e1941 + _e1969.member_1);
                                    }
                                    let _e1976 = phi_6734_;
                                    if (_e124 >= 1u) {
                                        phi_6753_ = (_e1976 <= (_e124 - 1u));
                                    } else {
                                        phi_6753_ = false;
                                    }
                                    let _e1981 = phi_6753_;
                                    if _e1981 {
                                        let _e1984 = global_1.member[_e1976];
                                        phi_2416_ = _e1984;
                                    } else {
                                        phi_2416_ = 4294967295u;
                                    }
                                    let _e1986 = phi_2416_;
                                    if (_e124 >= 4u) {
                                        phi_6777_ = (_e1986 <= (_e124 - 4u));
                                    } else {
                                        phi_6777_ = false;
                                    }
                                    let _e1991 = phi_6777_;
                                    if _e1991 {
                                        let _e1994 = global_1.member[_e1986];
                                        switch bitcast<i32>(_e1994) {
                                            case 0: {
                                                phi_2428_ = 0u;
                                                break;
                                            }
                                            case 1: {
                                                phi_2428_ = 1u;
                                                break;
                                            }
                                            case 2: {
                                                phi_2428_ = 2u;
                                                break;
                                            }
                                            default: {
                                                phi_2428_ = 0u;
                                                break;
                                            }
                                        }
                                        let _e1997 = phi_2428_;
                                        let _e2001 = global_1.member[(_e1986 + 1u)];
                                        let _e2005 = global_1.member[(_e1986 + 2u)];
                                        let _e2009 = global_1.member[(_e1986 + 3u)];
                                        phi_2442_ = type_30(_e1997, _e2001, _e2005, _e2009);
                                    } else {
                                        phi_2442_ = type_30(0u, 4294967295u, 4294967295u, 4294967295u);
                                    }
                                    let _e2012 = phi_2442_;
                                    if (_e124 >= 10u) {
                                        phi_6809_ = (_e2012.member_2 <= (_e124 - 10u));
                                    } else {
                                        phi_6809_ = false;
                                    }
                                    let _e2018 = phi_6809_;
                                    if _e2018 {
                                        let _e2021 = global_1.member[_e2012.member_2];
                                        let _e2026 = global_1.member[(_e2012.member_2 + 1u)];
                                        let _e2031 = global_1.member[(_e2012.member_2 + 2u)];
                                        let _e2037 = global_1.member[(_e2012.member_2 + 3u)];
                                        let _e2042 = global_1.member[(_e2012.member_2 + 4u)];
                                        let _e2047 = global_1.member[(_e2012.member_2 + 5u)];
                                        let _e2052 = global_1.member[(_e2012.member_2 + 6u)];
                                        let _e2058 = global_1.member[(_e2012.member_2 + 7u)];
                                        let _e2063 = global_1.member[(_e2012.member_2 + 8u)];
                                        let _e2068 = global_1.member[(_e2012.member_2 + 9u)];
                                        phi_2492_ = type_31(vec3<f32>(bitcast<f32>(_e2021), bitcast<f32>(_e2026), bitcast<f32>(_e2031)), vec4<f32>(bitcast<f32>(_e2037), bitcast<f32>(_e2042), bitcast<f32>(_e2047), bitcast<f32>(_e2052)), vec3<f32>(bitcast<f32>(_e2058), bitcast<f32>(_e2063), bitcast<f32>(_e2068)));
                                    } else {
                                        phi_2492_ = type_31(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e2073 = phi_2492_;
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
                                            if (_e124 >= 8u) {
                                                phi_7429_ = (_e2012.member_1 <= (_e124 - 8u));
                                            } else {
                                                phi_7429_ = false;
                                            }
                                            let _e3121 = phi_7429_;
                                            if _e3121 {
                                                let _e3124 = global_1.member[_e2012.member_1];
                                                let _e3129 = global_1.member[(_e2012.member_1 + 1u)];
                                                let _e3134 = global_1.member[(_e2012.member_1 + 2u)];
                                                let _e3140 = global_1.member[(_e2012.member_1 + 3u)];
                                                let _e3145 = global_1.member[(_e2012.member_1 + 4u)];
                                                let _e3150 = global_1.member[(_e2012.member_1 + 5u)];
                                                let _e3155 = global_1.member[(_e2012.member_1 + 6u)];
                                                let _e3161 = global_1.member[(_e2012.member_1 + 7u)];
                                                phi_2540_ = type_34(vec3<f32>(bitcast<f32>(_e3124), bitcast<f32>(_e3129), bitcast<f32>(_e3134)), vec4<f32>(bitcast<f32>(_e3140), bitcast<f32>(_e3145), bitcast<f32>(_e3150), bitcast<f32>(_e3155)), bitcast<f32>(_e3161));
                                            } else {
                                                phi_2540_ = type_34(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e3165 = phi_2540_;
                                            let _e3187 = fma(_e2112.x, _e3165.member.z, fma(_e2110.x, _e3165.member.y, (_e2108.x * _e3165.member.x)));
                                            let _e3188 = fma(_e2112.y, _e3165.member.z, fma(_e2110.y, _e3165.member.y, (_e2108.y * _e3165.member.x)));
                                            let _e3189 = fma(_e2112.z, _e3165.member.z, fma(_e2110.z, _e3165.member.y, (_e2108.z * _e3165.member.x)));
                                            let _e3194 = sqrt(fma(_e3189, _e3189, fma(_e3187, _e3187, (_e3188 * _e3188))));
                                            if (_e3194 == 0f) {
                                                phi_7476_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_7476_ = (vec3<f32>(_e3187, _e3188, _e3189) * (1f / _e3194));
                                            }
                                            let _e3199 = phi_7476_;
                                            let _e3201 = -(_e3199.x);
                                            let _e3203 = -(_e3199.y);
                                            let _e3205 = -(_e3199.z);
                                            let _e3206 = -(_e3199);
                                            let _e3208 = fma(-(_e686.z), _e295.member_3, 1f);
                                            let _e3212 = fma(0.4f, _e3208, (_e1370 * _e1382));
                                            let _e3213 = fma(0.4f, _e3208, (_e1372 * _e1382));
                                            let _e3214 = fma(0.4f, _e3208, (_e1374 * _e1382));
                                            let _e3222 = (_e1938 + vec3<f32>(_e3201, _e3203, _e3205));
                                            let _e3229 = sqrt(fma(_e3222.z, _e3222.z, fma(_e3222.x, _e3222.x, (_e3222.y * _e3222.y))));
                                            if (_e3229 == 0f) {
                                                phi_7511_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_7511_ = (_e3222 * (1f / _e3229));
                                            }
                                            let _e3234 = phi_7511_;
                                            let _e3235 = (_e1379 * _e1379);
                                            let _e3246 = max(fma(_e1934.z, _e3234.z, fma(_e1934.x, _e3234.x, (_e1934.y * _e3234.y))), 0f);
                                            let _e3259 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                            let _e3265 = fma(_e1934.z, _e3206.z, fma(_e1934.x, _e3206.x, (_e1934.y * _e3206.y)));
                                            let _e3266 = max(_e3265, 0f);
                                            let _e3267 = fma(_e686.y, _e295.member_4, 1f);
                                            let _e3268 = (_e3267 * _e3267);
                                            let _e3269 = (_e3268 * 0.125f);
                                            let _e3271 = fma(-(_e3268), 0.125f, 1f);
                                            let _e3284 = (1f - max(fma(_e3234.z, _e1938.z, fma(_e3234.x, _e1938.x, (_e3234.y * _e1938.y))), 0f));
                                            let _e3286 = select(_e3284, 0f, (_e3284 < 0f));
                                            let _e3289 = pow(select(_e3286, 1f, (_e3286 > 1f)), 5f);
                                            let _e3290 = fma((1f - _e3212), _e3289, _e3212);
                                            let _e3291 = fma((1f - _e3213), _e3289, _e3213);
                                            let _e3292 = fma((1f - _e3214), _e3289, _e3214);
                                            let _e3299 = (((_e3235 * _e3235) / (pow(fma((_e3246 * _e3246), fma(_e3235, _e3235, -1f), 1f), 2f) * 3.1415927f)) * ((_e3259 / fma(_e3259, _e3271, _e3269)) * (_e3266 / fma(_e3266, _e3271, _e3269))));
                                            let _e3306 = max(fma(_e1934.z, _e3205, fma(_e1934.x, _e3201, (_e1934.y * _e3203))), 0f);
                                            let _e3308 = fma((4f * _e3259), _e3306, 0.0001f);
                                            if ((_e2012.member_3 == 4294967295u) != true) {
                                                let _e3329 = global_1.member[_e2012.member_3];
                                                let _e3333 = global_1.member[(_e2012.member_3 + 1u)];
                                                let _e3337 = global_1.member[(_e2012.member_3 + 2u)];
                                                let _e3341 = global_1.member[(_e2012.member_3 + 3u)];
                                                let _e3345 = global_1.member[(_e2012.member_3 + 4u)];
                                                let _e3350 = global_1.member[(_e2012.member_3 + 5u)];
                                                let _e3355 = global_1.member[(_e2012.member_3 + 6u)];
                                                let _e3360 = global_1.member[select(_e3337, 4294967295u, (0u >= _e3341))];
                                                let _e3363 = global_1.member[_e3360];
                                                let _e3367 = global_1.member[(_e3360 + 1u)];
                                                let _e3371 = global_1.member[(_e3360 + 2u)];
                                                let _e3375 = global_1.member[(_e3360 + 3u)];
                                                let _e3379 = global_1.member[(_e3360 + 4u)];
                                                let _e3383 = global_1.member[(_e3360 + 6u)];
                                                switch bitcast<i32>(_e3383) {
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
                                                let _e3386 = phi_2781_;
                                                let _e3390 = global_1.member[(_e3360 + 7u)];
                                                switch bitcast<i32>(_e3390) {
                                                    case 0: {
                                                        phi_2790_ = 0u;
                                                        break;
                                                    }
                                                    case 1: {
                                                        phi_2790_ = 1u;
                                                        break;
                                                    }
                                                    case 2: {
                                                        phi_2790_ = 2u;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_2790_ = 0u;
                                                        break;
                                                    }
                                                }
                                                let _e3393 = phi_2790_;
                                                let _e3396 = global_1.member[_e1947];
                                                let _e3400 = global_1.member[(_e1947 + 1u)];
                                                let _e3402 = select(_e3329, 4294967295u, (0u >= _e3333));
                                                let _e3405 = global_1.member[_e3402];
                                                let _e3410 = global_1.member[(_e3402 + 1u)];
                                                let _e3415 = global_1.member[(_e3402 + 2u)];
                                                let _e3420 = global_1.member[(_e3402 + 3u)];
                                                let _e3425 = global_1.member[(_e3402 + 4u)];
                                                let _e3430 = global_1.member[(_e3402 + 5u)];
                                                let _e3435 = global_1.member[(_e3402 + 6u)];
                                                let _e3440 = global_1.member[(_e3402 + 7u)];
                                                let _e3445 = global_1.member[(_e3402 + 8u)];
                                                let _e3450 = global_1.member[(_e3402 + 9u)];
                                                let _e3455 = global_1.member[(_e3402 + 10u)];
                                                let _e3460 = global_1.member[(_e3402 + 11u)];
                                                let _e3465 = global_1.member[(_e3402 + 12u)];
                                                let _e3470 = global_1.member[(_e3402 + 13u)];
                                                let _e3475 = global_1.member[(_e3402 + 14u)];
                                                let _e3480 = global_1.member[(_e3402 + 15u)];
                                                let _e3500 = (bitcast<f32>(_e3480) + fma(bitcast<f32>(_e3460), _e132.z, fma(bitcast<f32>(_e3440), _e132.y, (bitcast<f32>(_e3420) * _e132.x))));
                                                let _e3501 = ((bitcast<f32>(_e3465) + fma(bitcast<f32>(_e3445), _e132.z, fma(bitcast<f32>(_e3425), _e132.y, (bitcast<f32>(_e3405) * _e132.x)))) / _e3500);
                                                let _e3502 = ((bitcast<f32>(_e3470) + fma(bitcast<f32>(_e3450), _e132.z, fma(bitcast<f32>(_e3430), _e132.y, (bitcast<f32>(_e3410) * _e132.x)))) / _e3500);
                                                let _e3503 = ((bitcast<f32>(_e3475) + fma(bitcast<f32>(_e3455), _e132.z, fma(bitcast<f32>(_e3435), _e132.y, (bitcast<f32>(_e3415) * _e132.x)))) / _e3500);
                                                if (abs(_e3501) <= 1f) {
                                                    let _e3507 = (abs(_e3502) <= 1f);
                                                    if _e3507 {
                                                        phi_7620_ = (abs(_e3503) <= 1f);
                                                    } else {
                                                        phi_7620_ = bool();
                                                    }
                                                    let _e3511 = phi_7620_;
                                                    phi_7623_ = _e3511;
                                                    phi_7624_ = select(true, false, _e3507);
                                                } else {
                                                    phi_7623_ = bool();
                                                    phi_7624_ = true;
                                                }
                                                let _e3514 = phi_7623_;
                                                let _e3516 = phi_7624_;
                                                if select(_e3514, false, _e3516) {
                                                    let _e3520 = bitcast<i32>(_e3355);
                                                    let _e3522 = f32(_e3371);
                                                    let _e3523 = f32(_e3375);
                                                    let _e3527 = type_35((_e3520 / -2i), (_e3520 / 2i), false);
                                                    phi_8039_ = _e1950;
                                                    phi_2969_ = _e3527;
                                                    phi_2972_ = 0f;
                                                    phi_2974_ = 0f;
                                                    loop {
                                                        let _e3529 = phi_8039_;
                                                        let _e3531 = phi_2969_;
                                                        let _e3533 = phi_2972_;
                                                        let _e3535 = phi_2974_;
                                                        local_4 = _e3533;
                                                        local_5 = _e3535;
                                                        if _e3531.member_2 {
                                                            phi_2986_ = true;
                                                        } else {
                                                            phi_2986_ = ((_e3531.member <= _e3531.member_1) != true);
                                                        }
                                                        let _e3542 = phi_2986_;
                                                        if _e3542 {
                                                            phi_2970_ = _e3531;
                                                            phi_3029_ = type_36(0u, type_36().member_1);
                                                        } else {
                                                            if (_e3531.member < _e3531.member_1) {
                                                                let _e3550 = (_e3531.member + 1i);
                                                                if select(false, true, ((false == (_e3550 > _e3531.member)) != false)) {
                                                                    phi_3014_ = type_36(0u, type_36().member_1);
                                                                } else {
                                                                    phi_3014_ = type_36(1u, _e3550);
                                                                }
                                                                let _e3560 = phi_3014_;
                                                                switch bitcast<i32>(_e3560.member) {
                                                                    case 0: {
                                                                        phi_8084_ = true;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        break;
                                                                    }
                                                                }
                                                                phi_3026_ = type_35(_e3560.member_1, _e3531.member_1, _e3531.member_2);
                                                            } else {
                                                                phi_3026_ = type_35(_e3531.member, _e3531.member_1, true);
                                                            }
                                                            let _e3569 = phi_3026_;
                                                            phi_2970_ = _e3569;
                                                            phi_3029_ = type_36(1u, _e3531.member);
                                                        }
                                                        let _e3575 = phi_2970_;
                                                        let _e3577 = phi_3029_;
                                                        switch bitcast<i32>(_e3577.member) {
                                                            case 0: {
                                                                phi_8085_ = _e3529;
                                                                phi_2973_ = f32();
                                                                phi_2975_ = f32();
                                                                phi_3288_ = false;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3040_ = _e3527;
                                                                phi_3043_ = _e3533;
                                                                phi_3045_ = _e3535;
                                                                loop {
                                                                    let _e3582 = phi_3040_;
                                                                    let _e3584 = phi_3043_;
                                                                    let _e3586 = phi_3045_;
                                                                    local_11 = _e3584;
                                                                    local_12 = _e3586;
                                                                    if _e3582.member_2 {
                                                                        phi_3057_ = true;
                                                                    } else {
                                                                        phi_3057_ = ((_e3582.member <= _e3582.member_1) != true);
                                                                    }
                                                                    let _e3593 = phi_3057_;
                                                                    if _e3593 {
                                                                        phi_3041_ = _e3582;
                                                                        phi_3100_ = type_36(0u, type_36().member_1);
                                                                    } else {
                                                                        if (_e3582.member < _e3582.member_1) {
                                                                            let _e3601 = (_e3582.member + 1i);
                                                                            if select(false, true, ((false == (_e3601 > _e3582.member)) != false)) {
                                                                                phi_3085_ = type_36(0u, type_36().member_1);
                                                                            } else {
                                                                                phi_3085_ = type_36(1u, _e3601);
                                                                            }
                                                                            let _e3611 = phi_3085_;
                                                                            switch bitcast<i32>(_e3611.member) {
                                                                                case 0: {
                                                                                    phi_8023_ = true;
                                                                                    break;
                                                                                }
                                                                                case 1: {
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            phi_3097_ = type_35(_e3611.member_1, _e3582.member_1, _e3582.member_2);
                                                                        } else {
                                                                            phi_3097_ = type_35(_e3582.member, _e3582.member_1, true);
                                                                        }
                                                                        let _e3620 = phi_3097_;
                                                                        phi_3041_ = _e3620;
                                                                        phi_3100_ = type_36(1u, _e3582.member);
                                                                    }
                                                                    let _e3626 = phi_3041_;
                                                                    let _e3628 = phi_3100_;
                                                                    switch bitcast<i32>(_e3628.member) {
                                                                        case 0: {
                                                                            phi_3044_ = f32();
                                                                            phi_3046_ = f32();
                                                                            phi_3287_ = false;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            let _e3636 = fma((_e3501 + 1f), 0.5f, (f32(_e3577.member_1) * (1f / _e3522)));
                                                                            let _e3637 = fma(fma(_e3502, -1f, 1f), 0.5f, (f32(_e3628.member_1) * (1f / _e3523)));
                                                                            switch bitcast<i32>(_e3386) {
                                                                                case 1: {
                                                                                    let _e3672 = abs(_e3636);
                                                                                    let _e3674 = (_e3672 % 1f);
                                                                                    if (_e3672 >= 1f) {
                                                                                        phi_7672_ = select(true, false, (_e3674 == 0f));
                                                                                    } else {
                                                                                        phi_7672_ = true;
                                                                                    }
                                                                                    let _e3678 = phi_7672_;
                                                                                    let _e3679 = select(1f, _e3674, _e3678);
                                                                                    if (select(-1f, 1f, (_e3636 >= 0f)) > 0f) {
                                                                                        phi_3132_ = _e3679;
                                                                                    } else {
                                                                                        phi_3132_ = (1f - _e3679);
                                                                                    }
                                                                                    let _e3683 = phi_3132_;
                                                                                    phi_3169_ = _e3683;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e3646 = abs(_e3636);
                                                                                    let _e3653 = ((select(select(u32(_e3646), 0u, (_e3646 < 0f)), 4294967295u, (_e3646 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e3655 = (_e3646 % 1f);
                                                                                    if (_e3646 >= 1f) {
                                                                                        phi_7655_ = select(true, false, (_e3655 == 0f));
                                                                                    } else {
                                                                                        phi_7655_ = true;
                                                                                    }
                                                                                    let _e3659 = phi_7655_;
                                                                                    let _e3660 = select(1f, _e3655, _e3659);
                                                                                    if (select(-1f, 1f, (_e3636 >= 0f)) > 0f) {
                                                                                        if _e3653 {
                                                                                            phi_3161_ = _e3660;
                                                                                        } else {
                                                                                            phi_3161_ = (1f - _e3660);
                                                                                        }
                                                                                        let _e3667 = phi_3161_;
                                                                                        phi_3167_ = _e3667;
                                                                                    } else {
                                                                                        if _e3653 {
                                                                                            phi_3166_ = (1f - _e3660);
                                                                                        } else {
                                                                                            phi_3166_ = _e3660;
                                                                                        }
                                                                                        let _e3664 = phi_3166_;
                                                                                        phi_3167_ = _e3664;
                                                                                    }
                                                                                    let _e3669 = phi_3167_;
                                                                                    phi_3169_ = _e3669;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e3636 > 1f) {
                                                                                        phi_7642_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_7642_ = select(_e3636, 0.00000011920929f, (_e3636 < 0f));
                                                                                    }
                                                                                    let _e3643 = phi_7642_;
                                                                                    phi_3169_ = _e3643;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3169_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e3685 = phi_3169_;
                                                                            switch bitcast<i32>(_e3393) {
                                                                                case 1: {
                                                                                    let _e3720 = abs(_e3637);
                                                                                    let _e3722 = (_e3720 % 1f);
                                                                                    if (_e3720 >= 1f) {
                                                                                        phi_7720_ = select(true, false, (_e3722 == 0f));
                                                                                    } else {
                                                                                        phi_7720_ = true;
                                                                                    }
                                                                                    let _e3726 = phi_7720_;
                                                                                    let _e3727 = select(1f, _e3722, _e3726);
                                                                                    if (select(-1f, 1f, (_e3637 >= 0f)) > 0f) {
                                                                                        phi_3188_ = _e3727;
                                                                                    } else {
                                                                                        phi_3188_ = (1f - _e3727);
                                                                                    }
                                                                                    let _e3731 = phi_3188_;
                                                                                    phi_3225_ = _e3731;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e3694 = abs(_e3637);
                                                                                    let _e3701 = ((select(select(u32(_e3694), 0u, (_e3694 < 0f)), 4294967295u, (_e3694 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e3703 = (_e3694 % 1f);
                                                                                    if (_e3694 >= 1f) {
                                                                                        phi_7703_ = select(true, false, (_e3703 == 0f));
                                                                                    } else {
                                                                                        phi_7703_ = true;
                                                                                    }
                                                                                    let _e3707 = phi_7703_;
                                                                                    let _e3708 = select(1f, _e3703, _e3707);
                                                                                    if (select(-1f, 1f, (_e3637 >= 0f)) > 0f) {
                                                                                        if _e3701 {
                                                                                            phi_3217_ = _e3708;
                                                                                        } else {
                                                                                            phi_3217_ = (1f - _e3708);
                                                                                        }
                                                                                        let _e3715 = phi_3217_;
                                                                                        phi_3223_ = _e3715;
                                                                                    } else {
                                                                                        if _e3701 {
                                                                                            phi_3222_ = (1f - _e3708);
                                                                                        } else {
                                                                                            phi_3222_ = _e3708;
                                                                                        }
                                                                                        let _e3712 = phi_3222_;
                                                                                        phi_3223_ = _e3712;
                                                                                    }
                                                                                    let _e3717 = phi_3223_;
                                                                                    phi_3225_ = _e3717;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e3637 > 1f) {
                                                                                        phi_7690_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_7690_ = select(_e3637, 0.00000011920929f, (_e3637 < 0f));
                                                                                    }
                                                                                    let _e3691 = phi_7690_;
                                                                                    phi_3225_ = _e3691;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3225_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e3733 = phi_3225_;
                                                                            let _e3734 = (_e3685 * _e3522);
                                                                            let _e3740 = (_e3733 * _e3523);
                                                                            let _e3755 = vec3<f32>((f32((select(select(u32(_e3734), 0u, (_e3734 < 0f)), 4294967295u, (_e3734 > 4294967000f)) + _e3363)) / f32(_e3396)), (f32((select(select(u32(_e3740), 0u, (_e3740 < 0f)), 4294967295u, (_e3740 > 4294967000f)) + _e3367)) / f32(_e3400)), f32(_e3379));
                                                                            let _e3761 = textureSampleLevel(global_19, global_18, vec2<f32>(_e3755.x, _e3755.y), i32(_e3755.z), 0f);
                                                                            phi_3044_ = (_e3584 + 1f);
                                                                            phi_3046_ = (_e3586 + select(0f, 1f, ((_e3503 - max((bitcast<f32>(_e3350) * (1f - _e3265)), bitcast<f32>(_e3345))) > _e3761.x)));
                                                                            phi_3287_ = true;
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            phi_3044_ = f32();
                                                                            phi_3046_ = f32();
                                                                            phi_3287_ = bool();
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e3772 = phi_3044_;
                                                                    let _e3774 = phi_3046_;
                                                                    let _e3776 = phi_3287_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_3040_ = _e3626;
                                                                        phi_3043_ = _e3772;
                                                                        phi_3045_ = _e3774;
                                                                        phi_8023_ = _e3529;
                                                                        break if !(_e3776);
                                                                    }
                                                                }
                                                                let _e3779 = phi_8023_;
                                                                phi_8084_ = _e3779;
                                                                if _e3779 {
                                                                    break;
                                                                }
                                                                phi_8085_ = _e3779;
                                                                let _e4171 = local_11;
                                                                phi_2973_ = _e4171;
                                                                let _e4174 = local_12;
                                                                phi_2975_ = _e4174;
                                                                phi_3288_ = true;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_8085_ = _e3529;
                                                                phi_2973_ = f32();
                                                                phi_2975_ = f32();
                                                                phi_3288_ = bool();
                                                                break;
                                                            }
                                                        }
                                                        let _e3781 = phi_8085_;
                                                        let _e3783 = phi_2973_;
                                                        let _e3785 = phi_2975_;
                                                        let _e3787 = phi_3288_;
                                                        continue;
                                                        continuing {
                                                            phi_8039_ = _e3781;
                                                            phi_2969_ = _e3575;
                                                            phi_2972_ = _e3783;
                                                            phi_2974_ = _e3785;
                                                            phi_8084_ = _e3781;
                                                            break if !(_e3787);
                                                        }
                                                    }
                                                    let _e3790 = phi_8084_;
                                                    phi_8086_ = _e3790;
                                                    if _e3790 {
                                                        break;
                                                    }
                                                    let _e3792 = local_4;
                                                    let _e3795 = local_5;
                                                    phi_8103_ = _e3790;
                                                    phi_3291_ = (_e3795 / max(_e3792, 1f));
                                                } else {
                                                    phi_8103_ = _e1950;
                                                    phi_3291_ = 0f;
                                                }
                                                let _e3798 = phi_8103_;
                                                let _e3800 = phi_3291_;
                                                phi_8102_ = _e3798;
                                                phi_3292_ = _e3800;
                                            } else {
                                                phi_8102_ = _e1950;
                                                phi_3292_ = 0f;
                                            }
                                            let _e3802 = phi_8102_;
                                            let _e3804 = phi_3292_;
                                            let _e3805 = (1f - _e3804);
                                            phi_8090_ = _e3802;
                                            phi_4474_ = vec3<f32>(fma(((fma((((1f - _e3290) * _e3208) * _e1370), 0.31830987f, ((_e3299 * _e3290) / _e3308)) * (_e3165.member_1.x * _e3165.member_2)) * _e3306), _e3805, _e1954.x), fma(((fma((((1f - _e3291) * _e3208) * _e1372), 0.31830987f, ((_e3299 * _e3291) / _e3308)) * (_e3165.member_1.y * _e3165.member_2)) * _e3306), _e3805, _e1954.y), fma(((fma((((1f - _e3292) * _e3208) * _e1374), 0.31830987f, ((_e3299 * _e3292) / _e3308)) * (_e3165.member_1.z * _e3165.member_2)) * _e3306), _e3805, _e1954.z));
                                            phi_4475_ = true;
                                            break;
                                        }
                                        case 1: {
                                            if (_e124 >= 8u) {
                                                phi_7255_ = (_e2012.member_1 <= (_e124 - 8u));
                                            } else {
                                                phi_7255_ = false;
                                            }
                                            let _e2909 = phi_7255_;
                                            if _e2909 {
                                                let _e2912 = global_1.member[_e2012.member_1];
                                                let _e2917 = global_1.member[(_e2012.member_1 + 1u)];
                                                let _e2922 = global_1.member[(_e2012.member_1 + 2u)];
                                                let _e2928 = global_1.member[(_e2012.member_1 + 3u)];
                                                let _e2933 = global_1.member[(_e2012.member_1 + 4u)];
                                                let _e2938 = global_1.member[(_e2012.member_1 + 5u)];
                                                let _e2943 = global_1.member[(_e2012.member_1 + 6u)];
                                                let _e2949 = global_1.member[(_e2012.member_1 + 7u)];
                                                phi_3346_ = type_34(vec3<f32>(bitcast<f32>(_e2912), bitcast<f32>(_e2917), bitcast<f32>(_e2922)), vec4<f32>(bitcast<f32>(_e2928), bitcast<f32>(_e2933), bitcast<f32>(_e2938), bitcast<f32>(_e2943)), bitcast<f32>(_e2949));
                                            } else {
                                                phi_3346_ = type_34(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2953 = phi_3346_;
                                            let _e2982 = (vec3<f32>((_e2073.member.x + fma(_e2112.x, _e2953.member.z, fma(_e2110.x, _e2953.member.y, (_e2108.x * _e2953.member.x)))), (_e2073.member.y + fma(_e2112.y, _e2953.member.z, fma(_e2110.y, _e2953.member.y, (_e2108.y * _e2953.member.x)))), (_e2073.member.z + fma(_e2112.z, _e2953.member.z, fma(_e2110.z, _e2953.member.y, (_e2108.z * _e2953.member.x))))) - _e132);
                                            let _e2989 = sqrt(fma(_e2982.z, _e2982.z, fma(_e2982.x, _e2982.x, (_e2982.y * _e2982.y))));
                                            let _e2990 = (_e2989 == 0f);
                                            if _e2990 {
                                                phi_3536_ = vec3<f32>();
                                            } else {
                                                if _e2990 {
                                                    phi_7302_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7302_ = (_e2982 * (1f / _e2989));
                                                }
                                                let _e2994 = phi_7302_;
                                                let _e2996 = (_e2953.member_2 / (_e2989 * _e2989));
                                                let _e2998 = fma(-(_e686.z), _e295.member_3, 1f);
                                                let _e3002 = fma(0.4f, _e2998, (_e1370 * _e1382));
                                                let _e3003 = fma(0.4f, _e2998, (_e1372 * _e1382));
                                                let _e3004 = fma(0.4f, _e2998, (_e1374 * _e1382));
                                                let _e3011 = (_e1938 + _e2994);
                                                let _e3018 = sqrt(fma(_e3011.z, _e3011.z, fma(_e3011.x, _e3011.x, (_e3011.y * _e3011.y))));
                                                if (_e3018 == 0f) {
                                                    phi_7337_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7337_ = (_e3011 * (1f / _e3018));
                                                }
                                                let _e3023 = phi_7337_;
                                                let _e3024 = (_e1379 * _e1379);
                                                let _e3035 = max(fma(_e1934.z, _e3023.z, fma(_e1934.x, _e3023.x, (_e1934.y * _e3023.y))), 0f);
                                                let _e3048 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                                let _e3055 = max(fma(_e1934.z, _e2994.z, fma(_e1934.x, _e2994.x, (_e1934.y * _e2994.y))), 0f);
                                                let _e3056 = fma(_e686.y, _e295.member_4, 1f);
                                                let _e3057 = (_e3056 * _e3056);
                                                let _e3058 = (_e3057 * 0.125f);
                                                let _e3060 = fma(-(_e3057), 0.125f, 1f);
                                                let _e3073 = (1f - max(fma(_e3023.z, _e1938.z, fma(_e3023.x, _e1938.x, (_e3023.y * _e1938.y))), 0f));
                                                let _e3075 = select(_e3073, 0f, (_e3073 < 0f));
                                                let _e3078 = pow(select(_e3075, 1f, (_e3075 > 1f)), 5f);
                                                let _e3079 = fma((1f - _e3002), _e3078, _e3002);
                                                let _e3080 = fma((1f - _e3003), _e3078, _e3003);
                                                let _e3081 = fma((1f - _e3004), _e3078, _e3004);
                                                let _e3088 = (((_e3024 * _e3024) / (pow(fma((_e3035 * _e3035), fma(_e3024, _e3024, -1f), 1f), 2f) * 3.1415927f)) * ((_e3048 / fma(_e3048, _e3060, _e3058)) * (_e3055 / fma(_e3055, _e3060, _e3058))));
                                                let _e3093 = fma((4f * _e3048), _e3055, 0.0001f);
                                                phi_3536_ = vec3<f32>(fma((fma((((1f - _e3079) * _e2998) * _e1370), 0.31830987f, ((_e3088 * _e3079) / _e3093)) * (_e2953.member_1.x * _e2996)), _e3055, _e1954.x), fma((fma((((1f - _e3080) * _e2998) * _e1372), 0.31830987f, ((_e3088 * _e3080) / _e3093)) * (_e2953.member_1.y * _e2996)), _e3055, _e1954.y), fma((fma((((1f - _e3081) * _e2998) * _e1374), 0.31830987f, ((_e3088 * _e3081) / _e3093)) * (_e2953.member_1.z * _e2996)), _e3055, _e1954.z));
                                            }
                                            let _e3114 = phi_3536_;
                                            phi_8090_ = _e1950;
                                            phi_4474_ = _e3114;
                                            phi_4475_ = select(true, false, _e2990);
                                            break;
                                        }
                                        case 2: {
                                            if (_e124 >= 13u) {
                                                phi_6889_ = (_e2012.member_1 <= (_e124 - 13u));
                                            } else {
                                                phi_6889_ = false;
                                            }
                                            let _e2123 = phi_6889_;
                                            if _e2123 {
                                                let _e2126 = global_1.member[_e2012.member_1];
                                                let _e2131 = global_1.member[(_e2012.member_1 + 1u)];
                                                let _e2136 = global_1.member[(_e2012.member_1 + 2u)];
                                                let _e2142 = global_1.member[(_e2012.member_1 + 3u)];
                                                let _e2147 = global_1.member[(_e2012.member_1 + 4u)];
                                                let _e2152 = global_1.member[(_e2012.member_1 + 5u)];
                                                let _e2158 = global_1.member[(_e2012.member_1 + 6u)];
                                                let _e2163 = global_1.member[(_e2012.member_1 + 7u)];
                                                let _e2168 = global_1.member[(_e2012.member_1 + 8u)];
                                                let _e2173 = global_1.member[(_e2012.member_1 + 9u)];
                                                let _e2178 = global_1.member[(_e2012.member_1 + 10u)];
                                                let _e2183 = global_1.member[(_e2012.member_1 + 11u)];
                                                let _e2189 = global_1.member[(_e2012.member_1 + 12u)];
                                                phi_3599_ = type_37(vec3<f32>(bitcast<f32>(_e2126), bitcast<f32>(_e2131), bitcast<f32>(_e2136)), vec3<f32>(bitcast<f32>(_e2142), bitcast<f32>(_e2147), bitcast<f32>(_e2152)), bitcast<f32>(_e2158), bitcast<f32>(_e2163), vec4<f32>(bitcast<f32>(_e2168), bitcast<f32>(_e2173), bitcast<f32>(_e2178), bitcast<f32>(_e2183)), bitcast<f32>(_e2189));
                                            } else {
                                                phi_3599_ = type_37(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2193 = phi_3599_;
                                            let _e2219 = vec3<f32>((_e2073.member.x + fma(_e2112.x, _e2193.member.z, fma(_e2110.x, _e2193.member.y, (_e2108.x * _e2193.member.x)))), (_e2073.member.y + fma(_e2112.y, _e2193.member.z, fma(_e2110.y, _e2193.member.y, (_e2108.y * _e2193.member.x)))), (_e2073.member.z + fma(_e2112.z, _e2193.member.z, fma(_e2110.z, _e2193.member.y, (_e2108.z * _e2193.member.x)))));
                                            let _e2220 = (_e2219 - _e132);
                                            let _e2227 = sqrt(fma(_e2220.z, _e2220.z, fma(_e2220.x, _e2220.x, (_e2220.y * _e2220.y))));
                                            let _e2228 = (_e2227 == 0f);
                                            if _e2228 {
                                                phi_3735_ = type_38(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), 0f, 0f, 0f, 0f, 0f, 0f, 0f, false, false);
                                            } else {
                                                if _e2228 {
                                                    phi_6939_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6939_ = (_e2220 * (1f / _e2227));
                                                }
                                                let _e2232 = phi_6939_;
                                                let _e2243 = fma(_e2112.x, _e2193.member_1.z, fma(_e2110.x, _e2193.member_1.y, (_e2108.x * _e2193.member_1.x)));
                                                let _e2244 = fma(_e2112.y, _e2193.member_1.z, fma(_e2110.y, _e2193.member_1.y, (_e2108.y * _e2193.member_1.x)));
                                                let _e2245 = fma(_e2112.z, _e2193.member_1.z, fma(_e2110.z, _e2193.member_1.y, (_e2108.z * _e2193.member_1.x)));
                                                let _e2250 = sqrt(fma(_e2245, _e2245, fma(_e2243, _e2243, (_e2244 * _e2244))));
                                                if (_e2250 == 0f) {
                                                    phi_6974_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6974_ = (vec3<f32>(_e2243, _e2244, _e2245) * (1f / _e2250));
                                                }
                                                let _e2255 = phi_6974_;
                                                let _e2257 = cos(_e2193.member_2);
                                                let _e2259 = cos(_e2193.member_3);
                                                let _e2260 = (_e2257 - _e2259);
                                                let _e2272 = fma(_e2232.z, -(_e2255.z), fma(_e2232.x, -(_e2255.x), (_e2232.y * -(_e2255.y))));
                                                let _e2276 = ((_e2272 - _e2259) / _e2260);
                                                let _e2278 = select(_e2276, 0f, (_e2276 < 0f));
                                                phi_3735_ = type_38(_e2219, _e132, _e2232, _e2255, _e2227, _e2257, _e2259, _e2260, _e2272, _e2276, select(_e2278, 1f, (_e2278 > 1f)), (_e2272 > _e2257), (_e2272 > _e2259));
                                            }
                                            let _e2283 = phi_3735_;
                                            let _e2285 = (_e2283.member_4 == 0f);
                                            if _e2285 {
                                                phi_8091_ = _e1950;
                                                phi_4472_ = vec3<f32>();
                                            } else {
                                                let _e2288 = (_e2193.member_5 * _e2283.member_10);
                                                let _e2292 = fma(-(_e686.z), _e295.member_3, 1f);
                                                let _e2296 = fma(0.4f, _e2292, (_e1370 * _e1382));
                                                let _e2297 = fma(0.4f, _e2292, (_e1372 * _e1382));
                                                let _e2298 = fma(0.4f, _e2292, (_e1374 * _e1382));
                                                let _e2305 = (_e1938 + _e2283.member_2);
                                                let _e2312 = sqrt(fma(_e2305.z, _e2305.z, fma(_e2305.x, _e2305.x, (_e2305.y * _e2305.y))));
                                                if (_e2312 == 0f) {
                                                    phi_7009_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_7009_ = (_e2305 * (1f / _e2312));
                                                }
                                                let _e2317 = phi_7009_;
                                                let _e2318 = (_e1379 * _e1379);
                                                let _e2329 = max(fma(_e1934.z, _e2317.z, fma(_e1934.x, _e2317.x, (_e1934.y * _e2317.y))), 0f);
                                                let _e2342 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                                let _e2349 = max(fma(_e1934.z, _e2283.member_2.z, fma(_e1934.x, _e2283.member_2.x, (_e1934.y * _e2283.member_2.y))), 0f);
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
                                                    let _e2409 = global_1.member[_e2012.member_3];
                                                    let _e2413 = global_1.member[(_e2012.member_3 + 1u)];
                                                    let _e2417 = global_1.member[(_e2012.member_3 + 2u)];
                                                    let _e2421 = global_1.member[(_e2012.member_3 + 3u)];
                                                    let _e2425 = global_1.member[(_e2012.member_3 + 4u)];
                                                    let _e2430 = global_1.member[(_e2012.member_3 + 5u)];
                                                    let _e2435 = global_1.member[(_e2012.member_3 + 6u)];
                                                    let _e2440 = global_1.member[select(_e2417, 4294967295u, (0u >= _e2421))];
                                                    let _e2443 = global_1.member[_e2440];
                                                    let _e2447 = global_1.member[(_e2440 + 1u)];
                                                    let _e2451 = global_1.member[(_e2440 + 2u)];
                                                    let _e2455 = global_1.member[(_e2440 + 3u)];
                                                    let _e2459 = global_1.member[(_e2440 + 4u)];
                                                    let _e2463 = global_1.member[(_e2440 + 6u)];
                                                    switch bitcast<i32>(_e2463) {
                                                        case 0: {
                                                            phi_3947_ = 0u;
                                                            break;
                                                        }
                                                        case 1: {
                                                            phi_3947_ = 1u;
                                                            break;
                                                        }
                                                        case 2: {
                                                            phi_3947_ = 2u;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_3947_ = 0u;
                                                            break;
                                                        }
                                                    }
                                                    let _e2466 = phi_3947_;
                                                    let _e2470 = global_1.member[(_e2440 + 7u)];
                                                    switch bitcast<i32>(_e2470) {
                                                        case 0: {
                                                            phi_3956_ = 0u;
                                                            break;
                                                        }
                                                        case 1: {
                                                            phi_3956_ = 1u;
                                                            break;
                                                        }
                                                        case 2: {
                                                            phi_3956_ = 2u;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_3956_ = 0u;
                                                            break;
                                                        }
                                                    }
                                                    let _e2473 = phi_3956_;
                                                    let _e2476 = global_1.member[_e1947];
                                                    let _e2480 = global_1.member[(_e1947 + 1u)];
                                                    let _e2482 = select(_e2409, 4294967295u, (0u >= _e2413));
                                                    let _e2485 = global_1.member[_e2482];
                                                    let _e2490 = global_1.member[(_e2482 + 1u)];
                                                    let _e2495 = global_1.member[(_e2482 + 2u)];
                                                    let _e2500 = global_1.member[(_e2482 + 3u)];
                                                    let _e2505 = global_1.member[(_e2482 + 4u)];
                                                    let _e2510 = global_1.member[(_e2482 + 5u)];
                                                    let _e2515 = global_1.member[(_e2482 + 6u)];
                                                    let _e2520 = global_1.member[(_e2482 + 7u)];
                                                    let _e2525 = global_1.member[(_e2482 + 8u)];
                                                    let _e2530 = global_1.member[(_e2482 + 9u)];
                                                    let _e2535 = global_1.member[(_e2482 + 10u)];
                                                    let _e2540 = global_1.member[(_e2482 + 11u)];
                                                    let _e2545 = global_1.member[(_e2482 + 12u)];
                                                    let _e2550 = global_1.member[(_e2482 + 13u)];
                                                    let _e2555 = global_1.member[(_e2482 + 14u)];
                                                    let _e2560 = global_1.member[(_e2482 + 15u)];
                                                    let _e2580 = (bitcast<f32>(_e2560) + fma(bitcast<f32>(_e2540), _e132.z, fma(bitcast<f32>(_e2520), _e132.y, (bitcast<f32>(_e2500) * _e132.x))));
                                                    let _e2581 = ((bitcast<f32>(_e2545) + fma(bitcast<f32>(_e2525), _e132.z, fma(bitcast<f32>(_e2505), _e132.y, (bitcast<f32>(_e2485) * _e132.x)))) / _e2580);
                                                    let _e2582 = ((bitcast<f32>(_e2550) + fma(bitcast<f32>(_e2530), _e132.z, fma(bitcast<f32>(_e2510), _e132.y, (bitcast<f32>(_e2490) * _e132.x)))) / _e2580);
                                                    let _e2583 = ((bitcast<f32>(_e2555) + fma(bitcast<f32>(_e2535), _e132.z, fma(bitcast<f32>(_e2515), _e132.y, (bitcast<f32>(_e2495) * _e132.x)))) / _e2580);
                                                    if (abs(_e2581) <= 1f) {
                                                        let _e2587 = (abs(_e2582) <= 1f);
                                                        if _e2587 {
                                                            phi_7118_ = (abs(_e2583) <= 1f);
                                                        } else {
                                                            phi_7118_ = bool();
                                                        }
                                                        let _e2591 = phi_7118_;
                                                        phi_7121_ = _e2591;
                                                        phi_7122_ = select(true, false, _e2587);
                                                    } else {
                                                        phi_7121_ = bool();
                                                        phi_7122_ = true;
                                                    }
                                                    let _e2594 = phi_7121_;
                                                    let _e2596 = phi_7122_;
                                                    if select(_e2594, false, _e2596) {
                                                        let _e2600 = bitcast<i32>(_e2435);
                                                        let _e2602 = f32(_e2451);
                                                        let _e2603 = f32(_e2455);
                                                        let _e2607 = type_35((_e2600 / -2i), (_e2600 / 2i), false);
                                                        phi_7941_ = _e1950;
                                                        phi_4135_ = _e2607;
                                                        phi_4138_ = 0f;
                                                        phi_4140_ = 0f;
                                                        loop {
                                                            let _e2609 = phi_7941_;
                                                            let _e2611 = phi_4135_;
                                                            let _e2613 = phi_4138_;
                                                            let _e2615 = phi_4140_;
                                                            local_2 = _e2613;
                                                            local_3 = _e2615;
                                                            if _e2611.member_2 {
                                                                phi_4152_ = true;
                                                            } else {
                                                                phi_4152_ = ((_e2611.member <= _e2611.member_1) != true);
                                                            }
                                                            let _e2622 = phi_4152_;
                                                            if _e2622 {
                                                                phi_4136_ = _e2611;
                                                                phi_4195_ = type_36(0u, type_36().member_1);
                                                            } else {
                                                                if (_e2611.member < _e2611.member_1) {
                                                                    let _e2630 = (_e2611.member + 1i);
                                                                    if select(false, true, ((false == (_e2630 > _e2611.member)) != false)) {
                                                                        phi_4180_ = type_36(0u, type_36().member_1);
                                                                    } else {
                                                                        phi_4180_ = type_36(1u, _e2630);
                                                                    }
                                                                    let _e2640 = phi_4180_;
                                                                    switch bitcast<i32>(_e2640.member) {
                                                                        case 0: {
                                                                            phi_8021_ = true;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            break;
                                                                        }
                                                                    }
                                                                    phi_4192_ = type_35(_e2640.member_1, _e2611.member_1, _e2611.member_2);
                                                                } else {
                                                                    phi_4192_ = type_35(_e2611.member, _e2611.member_1, true);
                                                                }
                                                                let _e2649 = phi_4192_;
                                                                phi_4136_ = _e2649;
                                                                phi_4195_ = type_36(1u, _e2611.member);
                                                            }
                                                            let _e2655 = phi_4136_;
                                                            let _e2657 = phi_4195_;
                                                            switch bitcast<i32>(_e2657.member) {
                                                                case 0: {
                                                                    phi_8022_ = _e2609;
                                                                    phi_4139_ = f32();
                                                                    phi_4141_ = f32();
                                                                    phi_4454_ = false;
                                                                    break;
                                                                }
                                                                case 1: {
                                                                    phi_4206_ = _e2607;
                                                                    phi_4209_ = _e2613;
                                                                    phi_4211_ = _e2615;
                                                                    loop {
                                                                        let _e2662 = phi_4206_;
                                                                        let _e2664 = phi_4209_;
                                                                        let _e2666 = phi_4211_;
                                                                        local_9 = _e2664;
                                                                        local_10 = _e2666;
                                                                        if _e2662.member_2 {
                                                                            phi_4223_ = true;
                                                                        } else {
                                                                            phi_4223_ = ((_e2662.member <= _e2662.member_1) != true);
                                                                        }
                                                                        let _e2673 = phi_4223_;
                                                                        if _e2673 {
                                                                            phi_4207_ = _e2662;
                                                                            phi_4266_ = type_36(0u, type_36().member_1);
                                                                        } else {
                                                                            if (_e2662.member < _e2662.member_1) {
                                                                                let _e2681 = (_e2662.member + 1i);
                                                                                if select(false, true, ((false == (_e2681 > _e2662.member)) != false)) {
                                                                                    phi_4251_ = type_36(0u, type_36().member_1);
                                                                                } else {
                                                                                    phi_4251_ = type_36(1u, _e2681);
                                                                                }
                                                                                let _e2691 = phi_4251_;
                                                                                switch bitcast<i32>(_e2691.member) {
                                                                                    case 0: {
                                                                                        phi_7925_ = true;
                                                                                        break;
                                                                                    }
                                                                                    case 1: {
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                phi_4263_ = type_35(_e2691.member_1, _e2662.member_1, _e2662.member_2);
                                                                            } else {
                                                                                phi_4263_ = type_35(_e2662.member, _e2662.member_1, true);
                                                                            }
                                                                            let _e2700 = phi_4263_;
                                                                            phi_4207_ = _e2700;
                                                                            phi_4266_ = type_36(1u, _e2662.member);
                                                                        }
                                                                        let _e2706 = phi_4207_;
                                                                        let _e2708 = phi_4266_;
                                                                        switch bitcast<i32>(_e2708.member) {
                                                                            case 0: {
                                                                                phi_4210_ = f32();
                                                                                phi_4212_ = f32();
                                                                                phi_4453_ = false;
                                                                                break;
                                                                            }
                                                                            case 1: {
                                                                                let _e2716 = fma((_e2581 + 1f), 0.5f, (f32(_e2657.member_1) * (1f / _e2602)));
                                                                                let _e2717 = fma(fma(_e2582, -1f, 1f), 0.5f, (f32(_e2708.member_1) * (1f / _e2603)));
                                                                                switch bitcast<i32>(_e2466) {
                                                                                    case 1: {
                                                                                        let _e2752 = abs(_e2716);
                                                                                        let _e2754 = (_e2752 % 1f);
                                                                                        if (_e2752 >= 1f) {
                                                                                            phi_7170_ = select(true, false, (_e2754 == 0f));
                                                                                        } else {
                                                                                            phi_7170_ = true;
                                                                                        }
                                                                                        let _e2758 = phi_7170_;
                                                                                        let _e2759 = select(1f, _e2754, _e2758);
                                                                                        if (select(-1f, 1f, (_e2716 >= 0f)) > 0f) {
                                                                                            phi_4298_ = _e2759;
                                                                                        } else {
                                                                                            phi_4298_ = (1f - _e2759);
                                                                                        }
                                                                                        let _e2763 = phi_4298_;
                                                                                        phi_4335_ = _e2763;
                                                                                        break;
                                                                                    }
                                                                                    case 2: {
                                                                                        let _e2726 = abs(_e2716);
                                                                                        let _e2733 = ((select(select(u32(_e2726), 0u, (_e2726 < 0f)), 4294967295u, (_e2726 > 4294967000f)) % 2u) == 0u);
                                                                                        let _e2735 = (_e2726 % 1f);
                                                                                        if (_e2726 >= 1f) {
                                                                                            phi_7153_ = select(true, false, (_e2735 == 0f));
                                                                                        } else {
                                                                                            phi_7153_ = true;
                                                                                        }
                                                                                        let _e2739 = phi_7153_;
                                                                                        let _e2740 = select(1f, _e2735, _e2739);
                                                                                        if (select(-1f, 1f, (_e2716 >= 0f)) > 0f) {
                                                                                            if _e2733 {
                                                                                                phi_4327_ = _e2740;
                                                                                            } else {
                                                                                                phi_4327_ = (1f - _e2740);
                                                                                            }
                                                                                            let _e2747 = phi_4327_;
                                                                                            phi_4333_ = _e2747;
                                                                                        } else {
                                                                                            if _e2733 {
                                                                                                phi_4332_ = (1f - _e2740);
                                                                                            } else {
                                                                                                phi_4332_ = _e2740;
                                                                                            }
                                                                                            let _e2744 = phi_4332_;
                                                                                            phi_4333_ = _e2744;
                                                                                        }
                                                                                        let _e2749 = phi_4333_;
                                                                                        phi_4335_ = _e2749;
                                                                                        break;
                                                                                    }
                                                                                    case 0: {
                                                                                        if (_e2716 > 1f) {
                                                                                            phi_7140_ = 0.9999999f;
                                                                                        } else {
                                                                                            phi_7140_ = select(_e2716, 0.00000011920929f, (_e2716 < 0f));
                                                                                        }
                                                                                        let _e2723 = phi_7140_;
                                                                                        phi_4335_ = _e2723;
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        phi_4335_ = f32();
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2765 = phi_4335_;
                                                                                switch bitcast<i32>(_e2473) {
                                                                                    case 1: {
                                                                                        let _e2800 = abs(_e2717);
                                                                                        let _e2802 = (_e2800 % 1f);
                                                                                        if (_e2800 >= 1f) {
                                                                                            phi_7218_ = select(true, false, (_e2802 == 0f));
                                                                                        } else {
                                                                                            phi_7218_ = true;
                                                                                        }
                                                                                        let _e2806 = phi_7218_;
                                                                                        let _e2807 = select(1f, _e2802, _e2806);
                                                                                        if (select(-1f, 1f, (_e2717 >= 0f)) > 0f) {
                                                                                            phi_4354_ = _e2807;
                                                                                        } else {
                                                                                            phi_4354_ = (1f - _e2807);
                                                                                        }
                                                                                        let _e2811 = phi_4354_;
                                                                                        phi_4391_ = _e2811;
                                                                                        break;
                                                                                    }
                                                                                    case 2: {
                                                                                        let _e2774 = abs(_e2717);
                                                                                        let _e2781 = ((select(select(u32(_e2774), 0u, (_e2774 < 0f)), 4294967295u, (_e2774 > 4294967000f)) % 2u) == 0u);
                                                                                        let _e2783 = (_e2774 % 1f);
                                                                                        if (_e2774 >= 1f) {
                                                                                            phi_7201_ = select(true, false, (_e2783 == 0f));
                                                                                        } else {
                                                                                            phi_7201_ = true;
                                                                                        }
                                                                                        let _e2787 = phi_7201_;
                                                                                        let _e2788 = select(1f, _e2783, _e2787);
                                                                                        if (select(-1f, 1f, (_e2717 >= 0f)) > 0f) {
                                                                                            if _e2781 {
                                                                                                phi_4383_ = _e2788;
                                                                                            } else {
                                                                                                phi_4383_ = (1f - _e2788);
                                                                                            }
                                                                                            let _e2795 = phi_4383_;
                                                                                            phi_4389_ = _e2795;
                                                                                        } else {
                                                                                            if _e2781 {
                                                                                                phi_4388_ = (1f - _e2788);
                                                                                            } else {
                                                                                                phi_4388_ = _e2788;
                                                                                            }
                                                                                            let _e2792 = phi_4388_;
                                                                                            phi_4389_ = _e2792;
                                                                                        }
                                                                                        let _e2797 = phi_4389_;
                                                                                        phi_4391_ = _e2797;
                                                                                        break;
                                                                                    }
                                                                                    case 0: {
                                                                                        if (_e2717 > 1f) {
                                                                                            phi_7188_ = 0.9999999f;
                                                                                        } else {
                                                                                            phi_7188_ = select(_e2717, 0.00000011920929f, (_e2717 < 0f));
                                                                                        }
                                                                                        let _e2771 = phi_7188_;
                                                                                        phi_4391_ = _e2771;
                                                                                        break;
                                                                                    }
                                                                                    default: {
                                                                                        phi_4391_ = f32();
                                                                                        break;
                                                                                    }
                                                                                }
                                                                                let _e2813 = phi_4391_;
                                                                                let _e2814 = (_e2765 * _e2602);
                                                                                let _e2820 = (_e2813 * _e2603);
                                                                                let _e2835 = vec3<f32>((f32((select(select(u32(_e2814), 0u, (_e2814 < 0f)), 4294967295u, (_e2814 > 4294967000f)) + _e2443)) / f32(_e2476)), (f32((select(select(u32(_e2820), 0u, (_e2820 < 0f)), 4294967295u, (_e2820 > 4294967000f)) + _e2447)) / f32(_e2480)), f32(_e2459));
                                                                                let _e2841 = textureSampleLevel(global_19, global_18, vec2<f32>(_e2835.x, _e2835.y), i32(_e2835.z), 0f);
                                                                                phi_4210_ = (_e2664 + 1f);
                                                                                phi_4212_ = (_e2666 + select(0f, 1f, ((_e2583 - max((bitcast<f32>(_e2430) * (1f - fma(_e1934.z, _e2283.member_3.z, fma(_e1934.x, _e2283.member_3.x, (_e1934.y * _e2283.member_3.y))))), bitcast<f32>(_e2425))) > _e2841.x)));
                                                                                phi_4453_ = true;
                                                                                break;
                                                                            }
                                                                            default: {
                                                                                phi_4210_ = f32();
                                                                                phi_4212_ = f32();
                                                                                phi_4453_ = bool();
                                                                                break;
                                                                            }
                                                                        }
                                                                        let _e2858 = phi_4210_;
                                                                        let _e2860 = phi_4212_;
                                                                        let _e2862 = phi_4453_;
                                                                        continue;
                                                                        continuing {
                                                                            phi_4206_ = _e2706;
                                                                            phi_4209_ = _e2858;
                                                                            phi_4211_ = _e2860;
                                                                            phi_7925_ = _e2609;
                                                                            break if !(_e2862);
                                                                        }
                                                                    }
                                                                    let _e2865 = phi_7925_;
                                                                    phi_8021_ = _e2865;
                                                                    if _e2865 {
                                                                        break;
                                                                    }
                                                                    phi_8022_ = _e2865;
                                                                    let _e4103 = local_9;
                                                                    phi_4139_ = _e4103;
                                                                    let _e4106 = local_10;
                                                                    phi_4141_ = _e4106;
                                                                    phi_4454_ = true;
                                                                    break;
                                                                }
                                                                default: {
                                                                    phi_8022_ = _e2609;
                                                                    phi_4139_ = f32();
                                                                    phi_4141_ = f32();
                                                                    phi_4454_ = bool();
                                                                    break;
                                                                }
                                                            }
                                                            let _e2867 = phi_8022_;
                                                            let _e2869 = phi_4139_;
                                                            let _e2871 = phi_4141_;
                                                            let _e2873 = phi_4454_;
                                                            continue;
                                                            continuing {
                                                                phi_7941_ = _e2867;
                                                                phi_4135_ = _e2655;
                                                                phi_4138_ = _e2869;
                                                                phi_4140_ = _e2871;
                                                                phi_8021_ = _e2867;
                                                                break if !(_e2873);
                                                            }
                                                        }
                                                        let _e2876 = phi_8021_;
                                                        phi_8086_ = _e2876;
                                                        if _e2876 {
                                                            break;
                                                        }
                                                        let _e2878 = local_2;
                                                        let _e2881 = local_3;
                                                        phi_8093_ = _e2876;
                                                        phi_4457_ = (_e2881 / max(_e2878, 1f));
                                                    } else {
                                                        phi_8093_ = _e1950;
                                                        phi_4457_ = 0f;
                                                    }
                                                    let _e2884 = phi_8093_;
                                                    let _e2886 = phi_4457_;
                                                    phi_8092_ = _e2884;
                                                    phi_4458_ = _e2886;
                                                } else {
                                                    phi_8092_ = _e1950;
                                                    phi_4458_ = 0f;
                                                }
                                                let _e2888 = phi_8092_;
                                                let _e2890 = phi_4458_;
                                                let _e2891 = (1f - _e2890);
                                                phi_8091_ = _e2888;
                                                phi_4472_ = vec3<f32>(fma(((fma((((1f - _e2373) * _e2292) * _e1370), 0.31830987f, ((_e2382 * _e2373) / _e2387)) * (_e2193.member_4.x * _e2288)) * _e2349), _e2891, _e1954.x), fma(((fma((((1f - _e2374) * _e2292) * _e1372), 0.31830987f, ((_e2382 * _e2374) / _e2387)) * (_e2193.member_4.y * _e2288)) * _e2349), _e2891, _e1954.y), fma(((fma((((1f - _e2375) * _e2292) * _e1374), 0.31830987f, ((_e2382 * _e2375) / _e2387)) * (_e2193.member_4.z * _e2288)) * _e2349), _e2891, _e1954.z));
                                            }
                                            let _e2900 = phi_8091_;
                                            let _e2902 = phi_4472_;
                                            phi_8090_ = _e2900;
                                            phi_4474_ = _e2902;
                                            phi_4475_ = select(true, false, _e2285);
                                            break;
                                        }
                                        default: {
                                            phi_8090_ = _e1950;
                                            phi_4474_ = vec3<f32>();
                                            phi_4475_ = bool();
                                            break;
                                        }
                                    }
                                    let _e3814 = phi_8090_;
                                    let _e3816 = phi_4474_;
                                    let _e3818 = phi_4475_;
                                    phi_8087_ = _e3814;
                                    phi_2377_ = select(_e3816, _e1954, vec3(select(true, false, _e3818)));
                                    phi_4484_ = true;
                                    break;
                                }
                                default: {
                                    phi_8087_ = _e1950;
                                    phi_2377_ = vec3<f32>();
                                    phi_4484_ = bool();
                                    break;
                                }
                            }
                            let _e3823 = phi_8087_;
                            let _e3825 = phi_2377_;
                            let _e3827 = phi_4484_;
                            continue;
                            continuing {
                                phi_7968_ = _e3823;
                                phi_2373_ = _e1967;
                                phi_2376_ = _e3825;
                                phi_8086_ = _e3823;
                                break if !(_e3827);
                            }
                        }
                        let _e3830 = phi_8086_;
                        phi_8104_ = _e3830;
                        if _e3830 {
                            break;
                        }
                        let _e3832 = fma(-(_e686.z), _e295.member_3, 1f);
                        let _e3836 = fma(0.04f, _e3832, (_e1370 * _e1382));
                        let _e3837 = fma(0.04f, _e3832, (_e1372 * _e1382));
                        let _e3838 = fma(0.04f, _e3832, (_e1374 * _e1382));
                        let _e3850 = fma(-(_e686.y), _e295.member_4, 1f);
                        let _e3857 = (1f - max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f));
                        let _e3859 = select(_e3857, 0f, (_e3857 < 0f));
                        let _e3862 = pow(select(_e3859, 1f, (_e3859 > 1f)), 5f);
                        let _e3863 = fma((max(_e3850, _e3836) - _e3836), _e3862, _e3836);
                        let _e3864 = fma((max(_e3850, _e3837) - _e3837), _e3862, _e3837);
                        let _e3865 = fma((max(_e3850, _e3838) - _e3838), _e3862, _e3838);
                        let _e3885 = local_6;
                        let _e3889 = local_7;
                        let _e3893 = local_8;
                        phi_8112_ = _e3830;
                        phi_4601_ = vec4<f32>(fma(_e1392, _e295.member_1, fma(fma(((1f - _e3863) * _e3832), (_e1401.x * _e1370), (_e1749.x * fma(_e3863, _e1765.x, _e1765.y))), _e1386, _e3885.x)), fma(_e1394, _e295.member_1, fma(fma(((1f - _e3864) * _e3832), (_e1401.y * _e1372), (_e1749.y * fma(_e3864, _e1765.x, _e1765.y))), _e1386, _e3889.y)), fma(_e1396, _e295.member_1, fma(fma(((1f - _e3865) * _e3832), (_e1401.z * _e1374), (_e1749.z * fma(_e3865, _e1765.x, _e1765.y))), _e1386, _e3893.z)), 1f);
                    } else {
                        phi_8112_ = false;
                        phi_4601_ = (vec4<f32>((_e126.x * _e492.x), (_e126.y * _e492.y), (_e126.z * _e492.z), (_e126.w * _e492.w)) * _e295.member_2);
                    }
                    let _e3901 = phi_8112_;
                    let _e3903 = phi_4601_;
                    global_20 = _e3903;
                    phi_8104_ = _e3901;
                    break;
                }
                case 1: {
                    let _e1907 = sqrt(fma(_e127.x, _e127.x, (_e127.y * _e127.y)));
                    if (_e1907 == 0f) {
                        phi_6634_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6634_ = (vec3<f32>(_e127.x, _e127.y, 0f) * (1f / _e1907));
                    }
                    let _e1912 = phi_6634_;
                    global_20 = vec4<f32>(((_e1912.x + 1f) * 0.5f), ((_e1912.y + 1f) * 0.5f), ((_e1912.z + 1f) * 0.5f), 1f);
                    phi_8104_ = false;
                    break;
                }
                case 2: {
                    let _e1886 = sqrt(fma(_e128.x, _e128.x, (_e128.y * _e128.y)));
                    if (_e1886 == 0f) {
                        phi_6585_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6585_ = (vec3<f32>(_e128.x, _e128.y, 0f) * (1f / _e1886));
                    }
                    let _e1891 = phi_6585_;
                    global_20 = vec4<f32>(((_e1891.x + 1f) * 0.5f), ((_e1891.y + 1f) * 0.5f), ((_e1891.z + 1f) * 0.5f), 1f);
                    phi_8104_ = false;
                    break;
                }
                case 3: {
                    if _e1728 {
                        phi_6536_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6536_ = (_e1355 * (1f / _e1727));
                    }
                    let _e1870 = phi_6536_;
                    global_20 = vec4<f32>(((_e1870.x + 1f) * 0.5f), ((_e1870.y + 1f) * 0.5f), ((_e1870.z + 1f) * 0.5f), 1f);
                    phi_8104_ = false;
                    break;
                }
                case 4: {
                    global_20 = _e126;
                    phi_8104_ = false;
                    break;
                }
                case 5: {
                    let _e1851 = sqrt(fma(_e129.z, _e129.z, fma(_e129.x, _e129.x, (_e129.y * _e129.y))));
                    if (_e1851 == 0f) {
                        phi_6487_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6487_ = (_e129 * (1f / _e1851));
                    }
                    let _e1856 = phi_6487_;
                    global_20 = vec4<f32>(((_e1856.x + 1f) * 0.5f), ((_e1856.y + 1f) * 0.5f), ((_e1856.z + 1f) * 0.5f), 1f);
                    phi_8104_ = false;
                    break;
                }
                case 6: {
                    let _e1829 = sqrt(fma(_e1353.z, _e1353.z, fma(_e1353.x, _e1353.x, (_e1353.y * _e1353.y))));
                    if (_e1829 == 0f) {
                        phi_6438_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6438_ = (_e1353 * (1f / _e1829));
                    }
                    let _e1834 = phi_6438_;
                    global_20 = vec4<f32>(((_e1834.x + 1f) * 0.5f), ((_e1834.y + 1f) * 0.5f), ((_e1834.z + 1f) * 0.5f), 1f);
                    phi_8104_ = false;
                    break;
                }
                case 7: {
                    let _e1807 = sqrt(fma(_e130.z, _e130.z, fma(_e130.x, _e130.x, (_e130.y * _e130.y))));
                    if (_e1807 == 0f) {
                        phi_6389_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6389_ = (_e130 * (1f / _e1807));
                    }
                    let _e1812 = phi_6389_;
                    global_20 = vec4<f32>(((_e1812.x + 1f) * 0.5f), ((_e1812.y + 1f) * 0.5f), ((_e1812.z + 1f) * 0.5f), 1f);
                    phi_8104_ = false;
                    break;
                }
                case 8: {
                    let _e1785 = sqrt(fma(_e131.z, _e131.z, fma(_e131.x, _e131.x, (_e131.y * _e131.y))));
                    if (_e1785 == 0f) {
                        phi_6340_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6340_ = (_e131 * (1f / _e1785));
                    }
                    let _e1790 = phi_6340_;
                    global_20 = vec4<f32>(((_e1790.x + 1f) * 0.5f), ((_e1790.y + 1f) * 0.5f), ((_e1790.z + 1f) * 0.5f), 1f);
                    phi_8104_ = false;
                    break;
                }
                case 9: {
                    global_20 = vec4<f32>(_e1401.x, _e1401.y, _e1401.z, 1f);
                    phi_8104_ = false;
                    break;
                }
                case 10: {
                    global_20 = vec4<f32>(_e1749.x, _e1749.y, _e1749.z, 1f);
                    phi_8104_ = false;
                    break;
                }
                case 11: {
                    global_20 = vec4<f32>(_e1765.x, _e1765.y, 1f, 1f);
                    phi_8104_ = false;
                    break;
                }
                case 12: {
                    global_20 = (vec4<f32>(_e1359, _e1362, _e1365, (_e492.w * _e295.member_2.w)) * _e126);
                    phi_8104_ = false;
                    break;
                }
                case 13: {
                    global_20 = vec4<f32>(_e1379, _e1379, _e1379, 1f);
                    phi_8104_ = false;
                    break;
                }
                case 14: {
                    global_20 = vec4<f32>(_e1382, _e1382, _e1382, 1f);
                    phi_8104_ = false;
                    break;
                }
                case 15: {
                    global_20 = vec4<f32>(_e1386, _e1386, _e1386, 1f);
                    phi_8104_ = false;
                    break;
                }
                case 16: {
                    global_20 = vec4<f32>((_e1392 * _e295.member_1), (_e1394 * _e295.member_1), (_e1396 * _e295.member_1), 1f);
                    phi_8104_ = false;
                    break;
                }
                case 17: {
                    global_20 = vec4<f32>(_e1268.x, _e1268.y, _e1268.z, 1f);
                    phi_8104_ = false;
                    break;
                }
                case 18: {
                    global_20 = vec4<f32>(_e295.member.x, _e295.member.y, _e295.member.z, 1f);
                    phi_8104_ = false;
                    break;
                }
                case 19: {
                    global_20 = vec4<f32>(_e295.member_1, _e295.member_1, _e295.member_1, 1f);
                    phi_8104_ = false;
                    break;
                }
                default: {
                    phi_8104_ = false;
                    break;
                }
            }
            let _e3905 = phi_8104_;
            if _e3905 {
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
