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
    var phi_4564_: bool;
    var phi_818_: type_33;
    var phi_822_: type_33;
    var phi_4601_: bool;
    var phi_862_: u32;
    var phi_871_: u32;
    var phi_884_: type_15;
    var phi_4623_: f32;
    var phi_4636_: bool;
    var phi_938_: f32;
    var phi_933_: f32;
    var phi_939_: f32;
    var phi_4653_: bool;
    var phi_904_: f32;
    var phi_941_: f32;
    var phi_4671_: f32;
    var phi_4684_: bool;
    var phi_996_: f32;
    var phi_991_: f32;
    var phi_997_: f32;
    var phi_4701_: bool;
    var phi_962_: f32;
    var phi_999_: f32;
    var phi_4737_: bool;
    var phi_1082_: u32;
    var phi_1091_: u32;
    var phi_1104_: type_15;
    var phi_4758_: f32;
    var phi_4771_: bool;
    var phi_1158_: f32;
    var phi_1153_: f32;
    var phi_1159_: f32;
    var phi_4788_: bool;
    var phi_1124_: f32;
    var phi_1161_: f32;
    var phi_4806_: f32;
    var phi_4819_: bool;
    var phi_1216_: f32;
    var phi_1211_: f32;
    var phi_1217_: f32;
    var phi_4836_: bool;
    var phi_1182_: f32;
    var phi_1219_: f32;
    var phi_4872_: bool;
    var phi_1302_: u32;
    var phi_1311_: u32;
    var phi_1324_: type_15;
    var phi_4893_: f32;
    var phi_4906_: bool;
    var phi_1378_: f32;
    var phi_1373_: f32;
    var phi_1379_: f32;
    var phi_4923_: bool;
    var phi_1344_: f32;
    var phi_1381_: f32;
    var phi_4941_: f32;
    var phi_4954_: bool;
    var phi_1436_: f32;
    var phi_1431_: f32;
    var phi_1437_: f32;
    var phi_4971_: bool;
    var phi_1402_: f32;
    var phi_1439_: f32;
    var phi_5007_: bool;
    var phi_1522_: u32;
    var phi_1531_: u32;
    var phi_1544_: type_15;
    var phi_5028_: f32;
    var phi_5041_: bool;
    var phi_1598_: f32;
    var phi_1593_: f32;
    var phi_1599_: f32;
    var phi_5058_: bool;
    var phi_1564_: f32;
    var phi_1601_: f32;
    var phi_5076_: f32;
    var phi_5089_: bool;
    var phi_1656_: f32;
    var phi_1651_: f32;
    var phi_1657_: f32;
    var phi_5106_: bool;
    var phi_1622_: f32;
    var phi_1659_: f32;
    var phi_5142_: bool;
    var phi_1742_: u32;
    var phi_1751_: u32;
    var phi_1764_: type_15;
    var phi_5163_: f32;
    var phi_5176_: bool;
    var phi_1818_: f32;
    var phi_1813_: f32;
    var phi_1819_: f32;
    var phi_5193_: bool;
    var phi_1784_: f32;
    var phi_1821_: f32;
    var phi_5211_: f32;
    var phi_5224_: bool;
    var phi_1876_: f32;
    var phi_1871_: f32;
    var phi_1877_: f32;
    var phi_5241_: bool;
    var phi_1842_: f32;
    var phi_1879_: f32;
    var phi_5299_: vec3<f32>;
    var phi_5334_: vec3<f32>;
    var phi_5369_: vec3<f32>;
    var phi_5404_: vec3<f32>;
    var phi_5439_: vec3<f32>;
    var phi_1973_: vec3<f32>;
    var phi_1974_: vec3<f32>;
    var phi_5471_: bool;
    var phi_2181_: type_14;
    var phi_2182_: type_14;
    var phi_2205_: type_14;
    var phi_2232_: bool;
    var phi_2238_: type_14;
    var phi_2239_: type_14;
    var phi_2262_: type_14;
    var phi_2285_: bool;
    var phi_2306_: type_25;
    var phi_5543_: vec3<f32>;
    var phi_5602_: vec3<f32>;
    var phi_5676_: vec3<f32>;
    var phi_5736_: vec3<f32>;
    var phi_5785_: vec3<f32>;
    var phi_5834_: vec3<f32>;
    var phi_5883_: vec3<f32>;
    var phi_5932_: vec3<f32>;
    var phi_5981_: vec3<f32>;
    var phi_6030_: vec3<f32>;
    var phi_6069_: vec3<f32>;
    var phi_6104_: vec3<f32>;
    var phi_7205_: bool;
    var phi_2373_: type_14;
    var phi_2376_: vec3<f32>;
    var phi_2374_: type_14;
    var phi_2399_: type_14;
    var phi_6130_: u32;
    var phi_6149_: bool;
    var phi_2416_: u32;
    var phi_6173_: bool;
    var phi_2428_: u32;
    var phi_2442_: type_30;
    var phi_6205_: bool;
    var phi_2492_: type_31;
    var phi_6285_: bool;
    var phi_3599_: type_37;
    var phi_6335_: vec3<f32>;
    var phi_6370_: vec3<f32>;
    var phi_3735_: type_38;
    var phi_6405_: vec3<f32>;
    var phi_3872_: vec3<f32>;
    var phi_6497_: bool;
    var phi_3346_: type_34;
    var phi_6544_: vec3<f32>;
    var phi_6579_: vec3<f32>;
    var phi_3536_: vec3<f32>;
    var phi_6671_: bool;
    var phi_2540_: type_34;
    var phi_6718_: vec3<f32>;
    var phi_6753_: vec3<f32>;
    var phi_2781_: u32;
    var phi_2790_: u32;
    var phi_6862_: bool;
    var phi_6865_: bool;
    var phi_6866_: bool;
    var phi_7183_: bool;
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
    var phi_6884_: f32;
    var phi_6897_: bool;
    var phi_3166_: f32;
    var phi_3161_: f32;
    var phi_3167_: f32;
    var phi_6914_: bool;
    var phi_3132_: f32;
    var phi_3169_: f32;
    var phi_6932_: f32;
    var phi_6945_: bool;
    var phi_3222_: f32;
    var phi_3217_: f32;
    var phi_3223_: f32;
    var phi_6962_: bool;
    var phi_3188_: f32;
    var phi_3225_: f32;
    var phi_3044_: f32;
    var phi_3046_: f32;
    var phi_3287_: bool;
    var phi_7167_: bool;
    var phi_7259_: bool;
    var phi_2973_: f32;
    var phi_2975_: f32;
    var phi_3288_: bool;
    var phi_7258_: bool;
    var local_2: f32;
    var local_3: f32;
    var phi_7287_: bool;
    var phi_3291_: f32;
    var phi_7286_: bool;
    var phi_3292_: f32;
    var phi_7264_: bool;
    var phi_3874_: vec3<f32>;
    var phi_3875_: bool;
    var phi_7261_: bool;
    var phi_2377_: vec3<f32>;
    var phi_3884_: bool;
    var phi_7260_: bool;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var phi_7296_: bool;
    var phi_4001_: vec4<f32>;
    var phi_7288_: bool;
    var local_7: f32;
    var local_8: f32;

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
                    phi_4564_ = (_e140 <= (_e122 - 22u));
                } else {
                    phi_4564_ = false;
                }
                let _e169 = phi_4564_;
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
                phi_4601_ = (_e295.member_5 <= (_e122 - 8u));
            } else {
                phi_4601_ = false;
            }
            let _e305 = phi_4601_;
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
                        phi_4653_ = select(true, false, (_e388 == 0f));
                    } else {
                        phi_4653_ = true;
                    }
                    let _e392 = phi_4653_;
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
                        phi_4636_ = select(true, false, (_e369 == 0f));
                    } else {
                        phi_4636_ = true;
                    }
                    let _e373 = phi_4636_;
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
                        phi_4623_ = 0.9999999f;
                    } else {
                        phi_4623_ = select(_e299.x, 0.00000011920929f, (_e299.x < 0f));
                    }
                    let _e357 = phi_4623_;
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
                        phi_4701_ = select(true, false, (_e439 == 0f));
                    } else {
                        phi_4701_ = true;
                    }
                    let _e443 = phi_4701_;
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
                        phi_4684_ = select(true, false, (_e420 == 0f));
                    } else {
                        phi_4684_ = true;
                    }
                    let _e424 = phi_4684_;
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
                        phi_4671_ = 0.9999999f;
                    } else {
                        phi_4671_ = select(_e299.y, 0.00000011920929f, (_e299.y < 0f));
                    }
                    let _e408 = phi_4671_;
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
                phi_4737_ = (_e295.member_6 <= (_e122 - 8u));
            } else {
                phi_4737_ = false;
            }
            let _e501 = phi_4737_;
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
                        phi_4788_ = select(true, false, (_e584 == 0f));
                    } else {
                        phi_4788_ = true;
                    }
                    let _e588 = phi_4788_;
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
                        phi_4771_ = select(true, false, (_e565 == 0f));
                    } else {
                        phi_4771_ = true;
                    }
                    let _e569 = phi_4771_;
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
                        phi_4758_ = 0.9999999f;
                    } else {
                        phi_4758_ = select(_e496.x, 0.00000011920929f, (_e496.x < 0f));
                    }
                    let _e553 = phi_4758_;
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
                        phi_4836_ = select(true, false, (_e635 == 0f));
                    } else {
                        phi_4836_ = true;
                    }
                    let _e639 = phi_4836_;
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
                        phi_4819_ = select(true, false, (_e616 == 0f));
                    } else {
                        phi_4819_ = true;
                    }
                    let _e620 = phi_4819_;
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
                        phi_4806_ = 0.9999999f;
                    } else {
                        phi_4806_ = select(_e496.y, 0.00000011920929f, (_e496.y < 0f));
                    }
                    let _e604 = phi_4806_;
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
                phi_4872_ = (_e295.member_7 <= (_e122 - 8u));
            } else {
                phi_4872_ = false;
            }
            let _e695 = phi_4872_;
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
                        phi_4923_ = select(true, false, (_e778 == 0f));
                    } else {
                        phi_4923_ = true;
                    }
                    let _e782 = phi_4923_;
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
                        phi_4906_ = select(true, false, (_e759 == 0f));
                    } else {
                        phi_4906_ = true;
                    }
                    let _e763 = phi_4906_;
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
                        phi_4893_ = 0.9999999f;
                    } else {
                        phi_4893_ = select(_e690.x, 0.00000011920929f, (_e690.x < 0f));
                    }
                    let _e747 = phi_4893_;
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
                        phi_4971_ = select(true, false, (_e829 == 0f));
                    } else {
                        phi_4971_ = true;
                    }
                    let _e833 = phi_4971_;
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
                        phi_4954_ = select(true, false, (_e810 == 0f));
                    } else {
                        phi_4954_ = true;
                    }
                    let _e814 = phi_4954_;
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
                        phi_4941_ = 0.9999999f;
                    } else {
                        phi_4941_ = select(_e690.y, 0.00000011920929f, (_e690.y < 0f));
                    }
                    let _e798 = phi_4941_;
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
                phi_5007_ = (_e295.member_8 <= (_e122 - 8u));
            } else {
                phi_5007_ = false;
            }
            let _e889 = phi_5007_;
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
                        phi_5058_ = select(true, false, (_e972 == 0f));
                    } else {
                        phi_5058_ = true;
                    }
                    let _e976 = phi_5058_;
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
                        phi_5041_ = select(true, false, (_e953 == 0f));
                    } else {
                        phi_5041_ = true;
                    }
                    let _e957 = phi_5041_;
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
                        phi_5028_ = 0.9999999f;
                    } else {
                        phi_5028_ = select(_e884.x, 0.00000011920929f, (_e884.x < 0f));
                    }
                    let _e941 = phi_5028_;
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
                        phi_5106_ = select(true, false, (_e1023 == 0f));
                    } else {
                        phi_5106_ = true;
                    }
                    let _e1027 = phi_5106_;
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
                        phi_5089_ = select(true, false, (_e1004 == 0f));
                    } else {
                        phi_5089_ = true;
                    }
                    let _e1008 = phi_5089_;
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
                        phi_5076_ = 0.9999999f;
                    } else {
                        phi_5076_ = select(_e884.y, 0.00000011920929f, (_e884.y < 0f));
                    }
                    let _e992 = phi_5076_;
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
                phi_5142_ = (_e295.member_9 <= (_e122 - 8u));
            } else {
                phi_5142_ = false;
            }
            let _e1083 = phi_5142_;
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
                        phi_5193_ = select(true, false, (_e1166 == 0f));
                    } else {
                        phi_5193_ = true;
                    }
                    let _e1170 = phi_5193_;
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
                        phi_5176_ = select(true, false, (_e1147 == 0f));
                    } else {
                        phi_5176_ = true;
                    }
                    let _e1151 = phi_5176_;
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
                        phi_5163_ = 0.9999999f;
                    } else {
                        phi_5163_ = select(_e1078.x, 0.00000011920929f, (_e1078.x < 0f));
                    }
                    let _e1135 = phi_5163_;
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
                        phi_5241_ = select(true, false, (_e1217 == 0f));
                    } else {
                        phi_5241_ = true;
                    }
                    let _e1221 = phi_5241_;
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
                        phi_5224_ = select(true, false, (_e1198 == 0f));
                    } else {
                        phi_5224_ = true;
                    }
                    let _e1202 = phi_5224_;
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
                        phi_5211_ = 0.9999999f;
                    } else {
                        phi_5211_ = select(_e1078.y, 0.00000011920929f, (_e1078.y < 0f));
                    }
                    let _e1186 = phi_5211_;
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
                    phi_5299_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5299_ = (vec3<f32>(_e1272, _e1273, _e1274) * (1f / _e1279));
                }
                let _e1284 = phi_5299_;
                let _e1291 = sqrt(fma(_e130.z, _e130.z, fma(_e130.x, _e130.x, (_e130.y * _e130.y))));
                if (_e1291 == 0f) {
                    phi_5334_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5334_ = (_e130 * (1f / _e1291));
                }
                let _e1296 = phi_5334_;
                let _e1303 = sqrt(fma(_e131.z, _e131.z, fma(_e131.x, _e131.x, (_e131.y * _e131.y))));
                if (_e1303 == 0f) {
                    phi_5369_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5369_ = (_e131 * (1f / _e1303));
                }
                let _e1308 = phi_5369_;
                let _e1315 = sqrt(fma(_e129.z, _e129.z, fma(_e129.x, _e129.x, (_e129.y * _e129.y))));
                if (_e1315 == 0f) {
                    phi_5404_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5404_ = (_e129 * (1f / _e1315));
                }
                let _e1320 = phi_5404_;
                let _e1339 = fma(_e1320.x, _e1284.z, fma(_e1296.x, _e1284.x, (_e1308.x * _e1284.y)));
                let _e1340 = fma(_e1320.y, _e1284.z, fma(_e1296.y, _e1284.x, (_e1308.y * _e1284.y)));
                let _e1341 = fma(_e1320.z, _e1284.z, fma(_e1296.z, _e1284.x, (_e1308.z * _e1284.y)));
                let _e1346 = sqrt(fma(_e1341, _e1341, fma(_e1339, _e1339, (_e1340 * _e1340))));
                if (_e1346 == 0f) {
                    phi_5439_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5439_ = (vec3<f32>(_e1339, _e1340, _e1341) * (1f / _e1346));
                }
                let _e1351 = phi_5439_;
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
                phi_5471_ = (_e136 <= (_e122 - 86u));
            } else {
                phi_5471_ = false;
            }
            let _e1409 = phi_5471_;
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
                phi_5543_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5543_ = (_e1707 * (1f / _e1714));
            }
            let _e1719 = phi_5543_;
            let _e1720 = -(_e1719);
            let _e1727 = sqrt(fma(_e1355.z, _e1355.z, fma(_e1355.x, _e1355.x, (_e1355.y * _e1355.y))));
            let _e1728 = (_e1727 == 0f);
            if _e1728 {
                phi_5602_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5602_ = (_e1355 * (1f / _e1727));
            }
            let _e1732 = phi_5602_;
            let _e1742 = (2f * fma(_e1732.z, _e1720.z, fma(_e1732.x, _e1720.x, (_e1732.y * _e1720.y))));
            let _e1749 = textureSampleLevel(global_14, global_15, (_e1720 - vec3<f32>((_e1742 * _e1732.x), (_e1742 * _e1732.y), (_e1742 * _e1732.z))), (_e1379 * 4f));
            if _e1715 {
                phi_5676_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5676_ = (_e1707 * (1f / _e1714));
            }
            let _e1756 = phi_5676_;
            let _e1765 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1355.z, _e1756.z, fma(_e1355.x, _e1756.x, (_e1355.y * _e1756.y))), 0f), _e1379), 0f);
            switch bitcast<i32>(_e158) {
                case 0: {
                    if _e295.member_15 {
                        if _e1728 {
                            phi_6069_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_6069_ = (_e1355 * (1f / _e1727));
                        }
                        let _e1934 = phi_6069_;
                        if _e1715 {
                            phi_6104_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_6104_ = (_e1707 * (1f / _e1714));
                        }
                        let _e1938 = phi_6104_;
                        let _e1941 = global_1.member[0u];
                        let _e1944 = global_1.member[1u];
                        let _e1947 = global_1.member[2u];
                        phi_7205_ = false;
                        phi_2373_ = type_14(0u, _e1944);
                        phi_2376_ = vec3<f32>(0f, 0f, 0f);
                        loop {
                            let _e1950 = phi_7205_;
                            let _e1952 = phi_2373_;
                            let _e1954 = phi_2376_;
                            local_4 = _e1954;
                            local_5 = _e1954;
                            local_6 = _e1954;
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
                                    phi_7261_ = _e1950;
                                    phi_2377_ = vec3<f32>();
                                    phi_3884_ = false;
                                    break;
                                }
                                case 1: {
                                    if (_e1969.member_1 >= _e1944) {
                                        phi_6130_ = 4294967295u;
                                    } else {
                                        phi_6130_ = (_e1941 + _e1969.member_1);
                                    }
                                    let _e1976 = phi_6130_;
                                    if (_e124 >= 1u) {
                                        phi_6149_ = (_e1976 <= (_e124 - 1u));
                                    } else {
                                        phi_6149_ = false;
                                    }
                                    let _e1981 = phi_6149_;
                                    if _e1981 {
                                        let _e1984 = global_1.member[_e1976];
                                        phi_2416_ = _e1984;
                                    } else {
                                        phi_2416_ = 4294967295u;
                                    }
                                    let _e1986 = phi_2416_;
                                    if (_e124 >= 4u) {
                                        phi_6173_ = (_e1986 <= (_e124 - 4u));
                                    } else {
                                        phi_6173_ = false;
                                    }
                                    let _e1991 = phi_6173_;
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
                                        phi_6205_ = (_e2012.member_2 <= (_e124 - 10u));
                                    } else {
                                        phi_6205_ = false;
                                    }
                                    let _e2018 = phi_6205_;
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
                                                phi_6671_ = (_e2012.member_1 <= (_e124 - 8u));
                                            } else {
                                                phi_6671_ = false;
                                            }
                                            let _e2627 = phi_6671_;
                                            if _e2627 {
                                                let _e2630 = global_1.member[_e2012.member_1];
                                                let _e2635 = global_1.member[(_e2012.member_1 + 1u)];
                                                let _e2640 = global_1.member[(_e2012.member_1 + 2u)];
                                                let _e2646 = global_1.member[(_e2012.member_1 + 3u)];
                                                let _e2651 = global_1.member[(_e2012.member_1 + 4u)];
                                                let _e2656 = global_1.member[(_e2012.member_1 + 5u)];
                                                let _e2661 = global_1.member[(_e2012.member_1 + 6u)];
                                                let _e2667 = global_1.member[(_e2012.member_1 + 7u)];
                                                phi_2540_ = type_34(vec3<f32>(bitcast<f32>(_e2630), bitcast<f32>(_e2635), bitcast<f32>(_e2640)), vec4<f32>(bitcast<f32>(_e2646), bitcast<f32>(_e2651), bitcast<f32>(_e2656), bitcast<f32>(_e2661)), bitcast<f32>(_e2667));
                                            } else {
                                                phi_2540_ = type_34(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2671 = phi_2540_;
                                            let _e2693 = fma(_e2112.x, _e2671.member.z, fma(_e2110.x, _e2671.member.y, (_e2108.x * _e2671.member.x)));
                                            let _e2694 = fma(_e2112.y, _e2671.member.z, fma(_e2110.y, _e2671.member.y, (_e2108.y * _e2671.member.x)));
                                            let _e2695 = fma(_e2112.z, _e2671.member.z, fma(_e2110.z, _e2671.member.y, (_e2108.z * _e2671.member.x)));
                                            let _e2700 = sqrt(fma(_e2695, _e2695, fma(_e2693, _e2693, (_e2694 * _e2694))));
                                            if (_e2700 == 0f) {
                                                phi_6718_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6718_ = (vec3<f32>(_e2693, _e2694, _e2695) * (1f / _e2700));
                                            }
                                            let _e2705 = phi_6718_;
                                            let _e2707 = -(_e2705.x);
                                            let _e2709 = -(_e2705.y);
                                            let _e2711 = -(_e2705.z);
                                            let _e2712 = -(_e2705);
                                            let _e2714 = fma(-(_e686.z), _e295.member_3, 1f);
                                            let _e2718 = fma(0.4f, _e2714, (_e1370 * _e1382));
                                            let _e2719 = fma(0.4f, _e2714, (_e1372 * _e1382));
                                            let _e2720 = fma(0.4f, _e2714, (_e1374 * _e1382));
                                            let _e2728 = (_e1938 + vec3<f32>(_e2707, _e2709, _e2711));
                                            let _e2735 = sqrt(fma(_e2728.z, _e2728.z, fma(_e2728.x, _e2728.x, (_e2728.y * _e2728.y))));
                                            if (_e2735 == 0f) {
                                                phi_6753_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6753_ = (_e2728 * (1f / _e2735));
                                            }
                                            let _e2740 = phi_6753_;
                                            let _e2741 = (_e1379 * _e1379);
                                            let _e2752 = max(fma(_e1934.z, _e2740.z, fma(_e1934.x, _e2740.x, (_e1934.y * _e2740.y))), 0f);
                                            let _e2765 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                            let _e2771 = fma(_e1934.z, _e2712.z, fma(_e1934.x, _e2712.x, (_e1934.y * _e2712.y)));
                                            let _e2772 = max(_e2771, 0f);
                                            let _e2773 = fma(_e686.y, _e295.member_4, 1f);
                                            let _e2774 = (_e2773 * _e2773);
                                            let _e2775 = (_e2774 * 0.125f);
                                            let _e2777 = fma(-(_e2774), 0.125f, 1f);
                                            let _e2790 = (1f - max(fma(_e2740.z, _e1938.z, fma(_e2740.x, _e1938.x, (_e2740.y * _e1938.y))), 0f));
                                            let _e2792 = select(_e2790, 0f, (_e2790 < 0f));
                                            let _e2795 = pow(select(_e2792, 1f, (_e2792 > 1f)), 5f);
                                            let _e2796 = fma((1f - _e2718), _e2795, _e2718);
                                            let _e2797 = fma((1f - _e2719), _e2795, _e2719);
                                            let _e2798 = fma((1f - _e2720), _e2795, _e2720);
                                            let _e2805 = (((_e2741 * _e2741) / (pow(fma((_e2752 * _e2752), fma(_e2741, _e2741, -1f), 1f), 2f) * 3.1415927f)) * ((_e2765 / fma(_e2765, _e2777, _e2775)) * (_e2772 / fma(_e2772, _e2777, _e2775))));
                                            let _e2812 = max(fma(_e1934.z, _e2711, fma(_e1934.x, _e2707, (_e1934.y * _e2709))), 0f);
                                            let _e2814 = fma((4f * _e2765), _e2812, 0.0001f);
                                            if ((_e2012.member_3 == 4294967295u) != true) {
                                                let _e2835 = global_1.member[_e2012.member_3];
                                                let _e2839 = global_1.member[(_e2012.member_3 + 1u)];
                                                let _e2843 = global_1.member[(_e2012.member_3 + 2u)];
                                                let _e2847 = global_1.member[(_e2012.member_3 + 3u)];
                                                let _e2851 = global_1.member[(_e2012.member_3 + 4u)];
                                                let _e2856 = global_1.member[(_e2012.member_3 + 5u)];
                                                let _e2861 = global_1.member[(_e2012.member_3 + 6u)];
                                                let _e2866 = global_1.member[select(_e2843, 4294967295u, (0u >= _e2847))];
                                                let _e2869 = global_1.member[_e2866];
                                                let _e2873 = global_1.member[(_e2866 + 1u)];
                                                let _e2877 = global_1.member[(_e2866 + 2u)];
                                                let _e2881 = global_1.member[(_e2866 + 3u)];
                                                let _e2885 = global_1.member[(_e2866 + 4u)];
                                                let _e2889 = global_1.member[(_e2866 + 6u)];
                                                switch bitcast<i32>(_e2889) {
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
                                                let _e2892 = phi_2781_;
                                                let _e2896 = global_1.member[(_e2866 + 7u)];
                                                switch bitcast<i32>(_e2896) {
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
                                                let _e2899 = phi_2790_;
                                                let _e2902 = global_1.member[_e1947];
                                                let _e2906 = global_1.member[(_e1947 + 1u)];
                                                let _e2908 = select(_e2835, 4294967295u, (0u >= _e2839));
                                                let _e2911 = global_1.member[_e2908];
                                                let _e2916 = global_1.member[(_e2908 + 1u)];
                                                let _e2921 = global_1.member[(_e2908 + 2u)];
                                                let _e2926 = global_1.member[(_e2908 + 3u)];
                                                let _e2931 = global_1.member[(_e2908 + 4u)];
                                                let _e2936 = global_1.member[(_e2908 + 5u)];
                                                let _e2941 = global_1.member[(_e2908 + 6u)];
                                                let _e2946 = global_1.member[(_e2908 + 7u)];
                                                let _e2951 = global_1.member[(_e2908 + 8u)];
                                                let _e2956 = global_1.member[(_e2908 + 9u)];
                                                let _e2961 = global_1.member[(_e2908 + 10u)];
                                                let _e2966 = global_1.member[(_e2908 + 11u)];
                                                let _e2971 = global_1.member[(_e2908 + 12u)];
                                                let _e2976 = global_1.member[(_e2908 + 13u)];
                                                let _e2981 = global_1.member[(_e2908 + 14u)];
                                                let _e2986 = global_1.member[(_e2908 + 15u)];
                                                let _e3006 = (bitcast<f32>(_e2986) + fma(bitcast<f32>(_e2966), _e132.z, fma(bitcast<f32>(_e2946), _e132.y, (bitcast<f32>(_e2926) * _e132.x))));
                                                let _e3007 = ((bitcast<f32>(_e2971) + fma(bitcast<f32>(_e2951), _e132.z, fma(bitcast<f32>(_e2931), _e132.y, (bitcast<f32>(_e2911) * _e132.x)))) / _e3006);
                                                let _e3008 = ((bitcast<f32>(_e2976) + fma(bitcast<f32>(_e2956), _e132.z, fma(bitcast<f32>(_e2936), _e132.y, (bitcast<f32>(_e2916) * _e132.x)))) / _e3006);
                                                let _e3009 = ((bitcast<f32>(_e2981) + fma(bitcast<f32>(_e2961), _e132.z, fma(bitcast<f32>(_e2941), _e132.y, (bitcast<f32>(_e2921) * _e132.x)))) / _e3006);
                                                if (abs(_e3007) <= 1f) {
                                                    let _e3013 = (abs(_e3008) <= 1f);
                                                    if _e3013 {
                                                        phi_6862_ = (abs(_e3009) <= 1f);
                                                    } else {
                                                        phi_6862_ = bool();
                                                    }
                                                    let _e3017 = phi_6862_;
                                                    phi_6865_ = _e3017;
                                                    phi_6866_ = select(true, false, _e3013);
                                                } else {
                                                    phi_6865_ = bool();
                                                    phi_6866_ = true;
                                                }
                                                let _e3020 = phi_6865_;
                                                let _e3022 = phi_6866_;
                                                if select(_e3020, false, _e3022) {
                                                    let _e3026 = bitcast<i32>(_e2861);
                                                    let _e3028 = f32(_e2877);
                                                    let _e3029 = f32(_e2881);
                                                    let _e3033 = type_35((_e3026 / -2i), (_e3026 / 2i), false);
                                                    phi_7183_ = _e1950;
                                                    phi_2969_ = _e3033;
                                                    phi_2972_ = 0f;
                                                    phi_2974_ = 0f;
                                                    loop {
                                                        let _e3035 = phi_7183_;
                                                        let _e3037 = phi_2969_;
                                                        let _e3039 = phi_2972_;
                                                        let _e3041 = phi_2974_;
                                                        local_2 = _e3039;
                                                        local_3 = _e3041;
                                                        if _e3037.member_2 {
                                                            phi_2986_ = true;
                                                        } else {
                                                            phi_2986_ = ((_e3037.member <= _e3037.member_1) != true);
                                                        }
                                                        let _e3048 = phi_2986_;
                                                        if _e3048 {
                                                            phi_2970_ = _e3037;
                                                            phi_3029_ = type_36(0u, type_36().member_1);
                                                        } else {
                                                            if (_e3037.member < _e3037.member_1) {
                                                                let _e3056 = (_e3037.member + 1i);
                                                                if select(false, true, ((false == (_e3056 > _e3037.member)) != false)) {
                                                                    phi_3014_ = type_36(0u, type_36().member_1);
                                                                } else {
                                                                    phi_3014_ = type_36(1u, _e3056);
                                                                }
                                                                let _e3066 = phi_3014_;
                                                                switch bitcast<i32>(_e3066.member) {
                                                                    case 0: {
                                                                        phi_7258_ = true;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        break;
                                                                    }
                                                                }
                                                                phi_3026_ = type_35(_e3066.member_1, _e3037.member_1, _e3037.member_2);
                                                            } else {
                                                                phi_3026_ = type_35(_e3037.member, _e3037.member_1, true);
                                                            }
                                                            let _e3075 = phi_3026_;
                                                            phi_2970_ = _e3075;
                                                            phi_3029_ = type_36(1u, _e3037.member);
                                                        }
                                                        let _e3081 = phi_2970_;
                                                        let _e3083 = phi_3029_;
                                                        switch bitcast<i32>(_e3083.member) {
                                                            case 0: {
                                                                phi_7259_ = _e3035;
                                                                phi_2973_ = f32();
                                                                phi_2975_ = f32();
                                                                phi_3288_ = false;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3040_ = _e3033;
                                                                phi_3043_ = _e3039;
                                                                phi_3045_ = _e3041;
                                                                loop {
                                                                    let _e3088 = phi_3040_;
                                                                    let _e3090 = phi_3043_;
                                                                    let _e3092 = phi_3045_;
                                                                    local_7 = _e3090;
                                                                    local_8 = _e3092;
                                                                    if _e3088.member_2 {
                                                                        phi_3057_ = true;
                                                                    } else {
                                                                        phi_3057_ = ((_e3088.member <= _e3088.member_1) != true);
                                                                    }
                                                                    let _e3099 = phi_3057_;
                                                                    if _e3099 {
                                                                        phi_3041_ = _e3088;
                                                                        phi_3100_ = type_36(0u, type_36().member_1);
                                                                    } else {
                                                                        if (_e3088.member < _e3088.member_1) {
                                                                            let _e3107 = (_e3088.member + 1i);
                                                                            if select(false, true, ((false == (_e3107 > _e3088.member)) != false)) {
                                                                                phi_3085_ = type_36(0u, type_36().member_1);
                                                                            } else {
                                                                                phi_3085_ = type_36(1u, _e3107);
                                                                            }
                                                                            let _e3117 = phi_3085_;
                                                                            switch bitcast<i32>(_e3117.member) {
                                                                                case 0: {
                                                                                    phi_7167_ = true;
                                                                                    break;
                                                                                }
                                                                                case 1: {
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            phi_3097_ = type_35(_e3117.member_1, _e3088.member_1, _e3088.member_2);
                                                                        } else {
                                                                            phi_3097_ = type_35(_e3088.member, _e3088.member_1, true);
                                                                        }
                                                                        let _e3126 = phi_3097_;
                                                                        phi_3041_ = _e3126;
                                                                        phi_3100_ = type_36(1u, _e3088.member);
                                                                    }
                                                                    let _e3132 = phi_3041_;
                                                                    let _e3134 = phi_3100_;
                                                                    switch bitcast<i32>(_e3134.member) {
                                                                        case 0: {
                                                                            phi_3044_ = f32();
                                                                            phi_3046_ = f32();
                                                                            phi_3287_ = false;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            let _e3142 = fma((_e3007 + 1f), 0.5f, (f32(_e3083.member_1) * (1f / _e3028)));
                                                                            let _e3143 = fma(fma(_e3008, -1f, 1f), 0.5f, (f32(_e3134.member_1) * (1f / _e3029)));
                                                                            switch bitcast<i32>(_e2892) {
                                                                                case 1: {
                                                                                    let _e3178 = abs(_e3142);
                                                                                    let _e3180 = (_e3178 % 1f);
                                                                                    if (_e3178 >= 1f) {
                                                                                        phi_6914_ = select(true, false, (_e3180 == 0f));
                                                                                    } else {
                                                                                        phi_6914_ = true;
                                                                                    }
                                                                                    let _e3184 = phi_6914_;
                                                                                    let _e3185 = select(1f, _e3180, _e3184);
                                                                                    if (select(-1f, 1f, (_e3142 >= 0f)) > 0f) {
                                                                                        phi_3132_ = _e3185;
                                                                                    } else {
                                                                                        phi_3132_ = (1f - _e3185);
                                                                                    }
                                                                                    let _e3189 = phi_3132_;
                                                                                    phi_3169_ = _e3189;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e3152 = abs(_e3142);
                                                                                    let _e3159 = ((select(select(u32(_e3152), 0u, (_e3152 < 0f)), 4294967295u, (_e3152 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e3161 = (_e3152 % 1f);
                                                                                    if (_e3152 >= 1f) {
                                                                                        phi_6897_ = select(true, false, (_e3161 == 0f));
                                                                                    } else {
                                                                                        phi_6897_ = true;
                                                                                    }
                                                                                    let _e3165 = phi_6897_;
                                                                                    let _e3166 = select(1f, _e3161, _e3165);
                                                                                    if (select(-1f, 1f, (_e3142 >= 0f)) > 0f) {
                                                                                        if _e3159 {
                                                                                            phi_3161_ = _e3166;
                                                                                        } else {
                                                                                            phi_3161_ = (1f - _e3166);
                                                                                        }
                                                                                        let _e3173 = phi_3161_;
                                                                                        phi_3167_ = _e3173;
                                                                                    } else {
                                                                                        if _e3159 {
                                                                                            phi_3166_ = (1f - _e3166);
                                                                                        } else {
                                                                                            phi_3166_ = _e3166;
                                                                                        }
                                                                                        let _e3170 = phi_3166_;
                                                                                        phi_3167_ = _e3170;
                                                                                    }
                                                                                    let _e3175 = phi_3167_;
                                                                                    phi_3169_ = _e3175;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e3142 > 1f) {
                                                                                        phi_6884_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_6884_ = select(_e3142, 0.00000011920929f, (_e3142 < 0f));
                                                                                    }
                                                                                    let _e3149 = phi_6884_;
                                                                                    phi_3169_ = _e3149;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3169_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e3191 = phi_3169_;
                                                                            switch bitcast<i32>(_e2899) {
                                                                                case 1: {
                                                                                    let _e3226 = abs(_e3143);
                                                                                    let _e3228 = (_e3226 % 1f);
                                                                                    if (_e3226 >= 1f) {
                                                                                        phi_6962_ = select(true, false, (_e3228 == 0f));
                                                                                    } else {
                                                                                        phi_6962_ = true;
                                                                                    }
                                                                                    let _e3232 = phi_6962_;
                                                                                    let _e3233 = select(1f, _e3228, _e3232);
                                                                                    if (select(-1f, 1f, (_e3143 >= 0f)) > 0f) {
                                                                                        phi_3188_ = _e3233;
                                                                                    } else {
                                                                                        phi_3188_ = (1f - _e3233);
                                                                                    }
                                                                                    let _e3237 = phi_3188_;
                                                                                    phi_3225_ = _e3237;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e3200 = abs(_e3143);
                                                                                    let _e3207 = ((select(select(u32(_e3200), 0u, (_e3200 < 0f)), 4294967295u, (_e3200 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e3209 = (_e3200 % 1f);
                                                                                    if (_e3200 >= 1f) {
                                                                                        phi_6945_ = select(true, false, (_e3209 == 0f));
                                                                                    } else {
                                                                                        phi_6945_ = true;
                                                                                    }
                                                                                    let _e3213 = phi_6945_;
                                                                                    let _e3214 = select(1f, _e3209, _e3213);
                                                                                    if (select(-1f, 1f, (_e3143 >= 0f)) > 0f) {
                                                                                        if _e3207 {
                                                                                            phi_3217_ = _e3214;
                                                                                        } else {
                                                                                            phi_3217_ = (1f - _e3214);
                                                                                        }
                                                                                        let _e3221 = phi_3217_;
                                                                                        phi_3223_ = _e3221;
                                                                                    } else {
                                                                                        if _e3207 {
                                                                                            phi_3222_ = (1f - _e3214);
                                                                                        } else {
                                                                                            phi_3222_ = _e3214;
                                                                                        }
                                                                                        let _e3218 = phi_3222_;
                                                                                        phi_3223_ = _e3218;
                                                                                    }
                                                                                    let _e3223 = phi_3223_;
                                                                                    phi_3225_ = _e3223;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e3143 > 1f) {
                                                                                        phi_6932_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_6932_ = select(_e3143, 0.00000011920929f, (_e3143 < 0f));
                                                                                    }
                                                                                    let _e3197 = phi_6932_;
                                                                                    phi_3225_ = _e3197;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3225_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e3239 = phi_3225_;
                                                                            let _e3240 = (_e3191 * _e3028);
                                                                            let _e3246 = (_e3239 * _e3029);
                                                                            let _e3261 = vec3<f32>((f32((select(select(u32(_e3240), 0u, (_e3240 < 0f)), 4294967295u, (_e3240 > 4294967000f)) + _e2869)) / f32(_e2902)), (f32((select(select(u32(_e3246), 0u, (_e3246 < 0f)), 4294967295u, (_e3246 > 4294967000f)) + _e2873)) / f32(_e2906)), f32(_e2885));
                                                                            let _e3267 = textureSampleLevel(global_19, global_18, vec2<f32>(_e3261.x, _e3261.y), i32(_e3261.z), 0f);
                                                                            phi_3044_ = (_e3090 + 1f);
                                                                            phi_3046_ = (_e3092 + select(0f, 1f, ((_e3009 - max((bitcast<f32>(_e2856) * (1f - _e2771)), bitcast<f32>(_e2851))) > _e3267.x)));
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
                                                                    let _e3278 = phi_3044_;
                                                                    let _e3280 = phi_3046_;
                                                                    let _e3282 = phi_3287_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_3040_ = _e3132;
                                                                        phi_3043_ = _e3278;
                                                                        phi_3045_ = _e3280;
                                                                        phi_7167_ = _e3035;
                                                                        break if !(_e3282);
                                                                    }
                                                                }
                                                                let _e3285 = phi_7167_;
                                                                phi_7258_ = _e3285;
                                                                if _e3285 {
                                                                    break;
                                                                }
                                                                phi_7259_ = _e3285;
                                                                let _e3619 = local_7;
                                                                phi_2973_ = _e3619;
                                                                let _e3622 = local_8;
                                                                phi_2975_ = _e3622;
                                                                phi_3288_ = true;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_7259_ = _e3035;
                                                                phi_2973_ = f32();
                                                                phi_2975_ = f32();
                                                                phi_3288_ = bool();
                                                                break;
                                                            }
                                                        }
                                                        let _e3287 = phi_7259_;
                                                        let _e3289 = phi_2973_;
                                                        let _e3291 = phi_2975_;
                                                        let _e3293 = phi_3288_;
                                                        continue;
                                                        continuing {
                                                            phi_7183_ = _e3287;
                                                            phi_2969_ = _e3081;
                                                            phi_2972_ = _e3289;
                                                            phi_2974_ = _e3291;
                                                            phi_7258_ = _e3287;
                                                            break if !(_e3293);
                                                        }
                                                    }
                                                    let _e3296 = phi_7258_;
                                                    phi_7260_ = _e3296;
                                                    if _e3296 {
                                                        break;
                                                    }
                                                    let _e3298 = local_2;
                                                    let _e3301 = local_3;
                                                    phi_7287_ = _e3296;
                                                    phi_3291_ = (_e3301 / max(_e3298, 1f));
                                                } else {
                                                    phi_7287_ = _e1950;
                                                    phi_3291_ = 0f;
                                                }
                                                let _e3304 = phi_7287_;
                                                let _e3306 = phi_3291_;
                                                phi_7286_ = _e3304;
                                                phi_3292_ = _e3306;
                                            } else {
                                                phi_7286_ = _e1950;
                                                phi_3292_ = 0f;
                                            }
                                            let _e3308 = phi_7286_;
                                            let _e3310 = phi_3292_;
                                            let _e3311 = (1f - _e3310);
                                            phi_7264_ = _e3308;
                                            phi_3874_ = vec3<f32>(fma(((fma((((1f - _e2796) * _e2714) * _e1370), 0.31830987f, ((_e2805 * _e2796) / _e2814)) * (_e2671.member_1.x * _e2671.member_2)) * _e2812), _e3311, _e1954.x), fma(((fma((((1f - _e2797) * _e2714) * _e1372), 0.31830987f, ((_e2805 * _e2797) / _e2814)) * (_e2671.member_1.y * _e2671.member_2)) * _e2812), _e3311, _e1954.y), fma(((fma((((1f - _e2798) * _e2714) * _e1374), 0.31830987f, ((_e2805 * _e2798) / _e2814)) * (_e2671.member_1.z * _e2671.member_2)) * _e2812), _e3311, _e1954.z));
                                            phi_3875_ = true;
                                            break;
                                        }
                                        case 1: {
                                            if (_e124 >= 8u) {
                                                phi_6497_ = (_e2012.member_1 <= (_e124 - 8u));
                                            } else {
                                                phi_6497_ = false;
                                            }
                                            let _e2415 = phi_6497_;
                                            if _e2415 {
                                                let _e2418 = global_1.member[_e2012.member_1];
                                                let _e2423 = global_1.member[(_e2012.member_1 + 1u)];
                                                let _e2428 = global_1.member[(_e2012.member_1 + 2u)];
                                                let _e2434 = global_1.member[(_e2012.member_1 + 3u)];
                                                let _e2439 = global_1.member[(_e2012.member_1 + 4u)];
                                                let _e2444 = global_1.member[(_e2012.member_1 + 5u)];
                                                let _e2449 = global_1.member[(_e2012.member_1 + 6u)];
                                                let _e2455 = global_1.member[(_e2012.member_1 + 7u)];
                                                phi_3346_ = type_34(vec3<f32>(bitcast<f32>(_e2418), bitcast<f32>(_e2423), bitcast<f32>(_e2428)), vec4<f32>(bitcast<f32>(_e2434), bitcast<f32>(_e2439), bitcast<f32>(_e2444), bitcast<f32>(_e2449)), bitcast<f32>(_e2455));
                                            } else {
                                                phi_3346_ = type_34(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2459 = phi_3346_;
                                            let _e2488 = (vec3<f32>((_e2073.member.x + fma(_e2112.x, _e2459.member.z, fma(_e2110.x, _e2459.member.y, (_e2108.x * _e2459.member.x)))), (_e2073.member.y + fma(_e2112.y, _e2459.member.z, fma(_e2110.y, _e2459.member.y, (_e2108.y * _e2459.member.x)))), (_e2073.member.z + fma(_e2112.z, _e2459.member.z, fma(_e2110.z, _e2459.member.y, (_e2108.z * _e2459.member.x))))) - _e132);
                                            let _e2495 = sqrt(fma(_e2488.z, _e2488.z, fma(_e2488.x, _e2488.x, (_e2488.y * _e2488.y))));
                                            let _e2496 = (_e2495 == 0f);
                                            if _e2496 {
                                                phi_3536_ = vec3<f32>();
                                            } else {
                                                if _e2496 {
                                                    phi_6544_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6544_ = (_e2488 * (1f / _e2495));
                                                }
                                                let _e2500 = phi_6544_;
                                                let _e2502 = (_e2459.member_2 / (_e2495 * _e2495));
                                                let _e2504 = fma(-(_e686.z), _e295.member_3, 1f);
                                                let _e2508 = fma(0.4f, _e2504, (_e1370 * _e1382));
                                                let _e2509 = fma(0.4f, _e2504, (_e1372 * _e1382));
                                                let _e2510 = fma(0.4f, _e2504, (_e1374 * _e1382));
                                                let _e2517 = (_e1938 + _e2500);
                                                let _e2524 = sqrt(fma(_e2517.z, _e2517.z, fma(_e2517.x, _e2517.x, (_e2517.y * _e2517.y))));
                                                if (_e2524 == 0f) {
                                                    phi_6579_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6579_ = (_e2517 * (1f / _e2524));
                                                }
                                                let _e2529 = phi_6579_;
                                                let _e2530 = (_e1379 * _e1379);
                                                let _e2541 = max(fma(_e1934.z, _e2529.z, fma(_e1934.x, _e2529.x, (_e1934.y * _e2529.y))), 0f);
                                                let _e2554 = max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f);
                                                let _e2561 = max(fma(_e1934.z, _e2500.z, fma(_e1934.x, _e2500.x, (_e1934.y * _e2500.y))), 0f);
                                                let _e2562 = fma(_e686.y, _e295.member_4, 1f);
                                                let _e2563 = (_e2562 * _e2562);
                                                let _e2564 = (_e2563 * 0.125f);
                                                let _e2566 = fma(-(_e2563), 0.125f, 1f);
                                                let _e2579 = (1f - max(fma(_e2529.z, _e1938.z, fma(_e2529.x, _e1938.x, (_e2529.y * _e1938.y))), 0f));
                                                let _e2581 = select(_e2579, 0f, (_e2579 < 0f));
                                                let _e2584 = pow(select(_e2581, 1f, (_e2581 > 1f)), 5f);
                                                let _e2585 = fma((1f - _e2508), _e2584, _e2508);
                                                let _e2586 = fma((1f - _e2509), _e2584, _e2509);
                                                let _e2587 = fma((1f - _e2510), _e2584, _e2510);
                                                let _e2594 = (((_e2530 * _e2530) / (pow(fma((_e2541 * _e2541), fma(_e2530, _e2530, -1f), 1f), 2f) * 3.1415927f)) * ((_e2554 / fma(_e2554, _e2566, _e2564)) * (_e2561 / fma(_e2561, _e2566, _e2564))));
                                                let _e2599 = fma((4f * _e2554), _e2561, 0.0001f);
                                                phi_3536_ = vec3<f32>(fma((fma((((1f - _e2585) * _e2504) * _e1370), 0.31830987f, ((_e2594 * _e2585) / _e2599)) * (_e2459.member_1.x * _e2502)), _e2561, _e1954.x), fma((fma((((1f - _e2586) * _e2504) * _e1372), 0.31830987f, ((_e2594 * _e2586) / _e2599)) * (_e2459.member_1.y * _e2502)), _e2561, _e1954.y), fma((fma((((1f - _e2587) * _e2504) * _e1374), 0.31830987f, ((_e2594 * _e2587) / _e2599)) * (_e2459.member_1.z * _e2502)), _e2561, _e1954.z));
                                            }
                                            let _e2620 = phi_3536_;
                                            phi_7264_ = _e1950;
                                            phi_3874_ = _e2620;
                                            phi_3875_ = select(true, false, _e2496);
                                            break;
                                        }
                                        case 2: {
                                            if (_e124 >= 13u) {
                                                phi_6285_ = (_e2012.member_1 <= (_e124 - 13u));
                                            } else {
                                                phi_6285_ = false;
                                            }
                                            let _e2123 = phi_6285_;
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
                                                    phi_6335_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6335_ = (_e2220 * (1f / _e2227));
                                                }
                                                let _e2232 = phi_6335_;
                                                let _e2243 = fma(_e2112.x, _e2193.member_1.z, fma(_e2110.x, _e2193.member_1.y, (_e2108.x * _e2193.member_1.x)));
                                                let _e2244 = fma(_e2112.y, _e2193.member_1.z, fma(_e2110.y, _e2193.member_1.y, (_e2108.y * _e2193.member_1.x)));
                                                let _e2245 = fma(_e2112.z, _e2193.member_1.z, fma(_e2110.z, _e2193.member_1.y, (_e2108.z * _e2193.member_1.x)));
                                                let _e2250 = sqrt(fma(_e2245, _e2245, fma(_e2243, _e2243, (_e2244 * _e2244))));
                                                if (_e2250 == 0f) {
                                                    phi_6370_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6370_ = (vec3<f32>(_e2243, _e2244, _e2245) * (1f / _e2250));
                                                }
                                                let _e2255 = phi_6370_;
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
                                                phi_3872_ = vec3<f32>();
                                            } else {
                                                let _e2288 = (_e2193.member_5 * _e2283.member_10);
                                                let _e2292 = fma(-(_e686.z), _e295.member_3, 1f);
                                                let _e2296 = fma(0.4f, _e2292, (_e1370 * _e1382));
                                                let _e2297 = fma(0.4f, _e2292, (_e1372 * _e1382));
                                                let _e2298 = fma(0.4f, _e2292, (_e1374 * _e1382));
                                                let _e2305 = (_e1938 + _e2283.member_2);
                                                let _e2312 = sqrt(fma(_e2305.z, _e2305.z, fma(_e2305.x, _e2305.x, (_e2305.y * _e2305.y))));
                                                if (_e2312 == 0f) {
                                                    phi_6405_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6405_ = (_e2305 * (1f / _e2312));
                                                }
                                                let _e2317 = phi_6405_;
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
                                                phi_3872_ = vec3<f32>(fma((fma((((1f - _e2373) * _e2292) * _e1370), 0.31830987f, ((_e2382 * _e2373) / _e2387)) * (_e2193.member_4.x * _e2288)), _e2349, _e1954.x), fma((fma((((1f - _e2374) * _e2292) * _e1372), 0.31830987f, ((_e2382 * _e2374) / _e2387)) * (_e2193.member_4.y * _e2288)), _e2349, _e1954.y), fma((fma((((1f - _e2375) * _e2292) * _e1374), 0.31830987f, ((_e2382 * _e2375) / _e2387)) * (_e2193.member_4.z * _e2288)), _e2349, _e1954.z));
                                            }
                                            let _e2408 = phi_3872_;
                                            phi_7264_ = _e1950;
                                            phi_3874_ = _e2408;
                                            phi_3875_ = select(true, false, _e2285);
                                            break;
                                        }
                                        default: {
                                            phi_7264_ = _e1950;
                                            phi_3874_ = vec3<f32>();
                                            phi_3875_ = bool();
                                            break;
                                        }
                                    }
                                    let _e3320 = phi_7264_;
                                    let _e3322 = phi_3874_;
                                    let _e3324 = phi_3875_;
                                    phi_7261_ = _e3320;
                                    phi_2377_ = select(_e3322, _e1954, vec3(select(true, false, _e3324)));
                                    phi_3884_ = true;
                                    break;
                                }
                                default: {
                                    phi_7261_ = _e1950;
                                    phi_2377_ = vec3<f32>();
                                    phi_3884_ = bool();
                                    break;
                                }
                            }
                            let _e3329 = phi_7261_;
                            let _e3331 = phi_2377_;
                            let _e3333 = phi_3884_;
                            continue;
                            continuing {
                                phi_7205_ = _e3329;
                                phi_2373_ = _e1967;
                                phi_2376_ = _e3331;
                                phi_7260_ = _e3329;
                                break if !(_e3333);
                            }
                        }
                        let _e3336 = phi_7260_;
                        phi_7288_ = _e3336;
                        if _e3336 {
                            break;
                        }
                        let _e3338 = fma(-(_e686.z), _e295.member_3, 1f);
                        let _e3342 = fma(0.04f, _e3338, (_e1370 * _e1382));
                        let _e3343 = fma(0.04f, _e3338, (_e1372 * _e1382));
                        let _e3344 = fma(0.04f, _e3338, (_e1374 * _e1382));
                        let _e3356 = fma(-(_e686.y), _e295.member_4, 1f);
                        let _e3363 = (1f - max(fma(_e1934.z, _e1938.z, fma(_e1934.x, _e1938.x, (_e1934.y * _e1938.y))), 0f));
                        let _e3365 = select(_e3363, 0f, (_e3363 < 0f));
                        let _e3368 = pow(select(_e3365, 1f, (_e3365 > 1f)), 5f);
                        let _e3369 = fma((max(_e3356, _e3342) - _e3342), _e3368, _e3342);
                        let _e3370 = fma((max(_e3356, _e3343) - _e3343), _e3368, _e3343);
                        let _e3371 = fma((max(_e3356, _e3344) - _e3344), _e3368, _e3344);
                        let _e3391 = local_4;
                        let _e3395 = local_5;
                        let _e3399 = local_6;
                        phi_7296_ = _e3336;
                        phi_4001_ = vec4<f32>(fma(_e1392, _e295.member_1, fma(fma(((1f - _e3369) * _e3338), (_e1401.x * _e1370), (_e1749.x * fma(_e3369, _e1765.x, _e1765.y))), _e1386, _e3391.x)), fma(_e1394, _e295.member_1, fma(fma(((1f - _e3370) * _e3338), (_e1401.y * _e1372), (_e1749.y * fma(_e3370, _e1765.x, _e1765.y))), _e1386, _e3395.y)), fma(_e1396, _e295.member_1, fma(fma(((1f - _e3371) * _e3338), (_e1401.z * _e1374), (_e1749.z * fma(_e3371, _e1765.x, _e1765.y))), _e1386, _e3399.z)), 1f);
                    } else {
                        phi_7296_ = false;
                        phi_4001_ = (vec4<f32>((_e126.x * _e492.x), (_e126.y * _e492.y), (_e126.z * _e492.z), (_e126.w * _e492.w)) * _e295.member_2);
                    }
                    let _e3407 = phi_7296_;
                    let _e3409 = phi_4001_;
                    global_20 = _e3409;
                    phi_7288_ = _e3407;
                    break;
                }
                case 1: {
                    let _e1907 = sqrt(fma(_e127.x, _e127.x, (_e127.y * _e127.y)));
                    if (_e1907 == 0f) {
                        phi_6030_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6030_ = (vec3<f32>(_e127.x, _e127.y, 0f) * (1f / _e1907));
                    }
                    let _e1912 = phi_6030_;
                    global_20 = vec4<f32>(((_e1912.x + 1f) * 0.5f), ((_e1912.y + 1f) * 0.5f), ((_e1912.z + 1f) * 0.5f), 1f);
                    phi_7288_ = false;
                    break;
                }
                case 2: {
                    let _e1886 = sqrt(fma(_e128.x, _e128.x, (_e128.y * _e128.y)));
                    if (_e1886 == 0f) {
                        phi_5981_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5981_ = (vec3<f32>(_e128.x, _e128.y, 0f) * (1f / _e1886));
                    }
                    let _e1891 = phi_5981_;
                    global_20 = vec4<f32>(((_e1891.x + 1f) * 0.5f), ((_e1891.y + 1f) * 0.5f), ((_e1891.z + 1f) * 0.5f), 1f);
                    phi_7288_ = false;
                    break;
                }
                case 3: {
                    if _e1728 {
                        phi_5932_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5932_ = (_e1355 * (1f / _e1727));
                    }
                    let _e1870 = phi_5932_;
                    global_20 = vec4<f32>(((_e1870.x + 1f) * 0.5f), ((_e1870.y + 1f) * 0.5f), ((_e1870.z + 1f) * 0.5f), 1f);
                    phi_7288_ = false;
                    break;
                }
                case 4: {
                    global_20 = _e126;
                    phi_7288_ = false;
                    break;
                }
                case 5: {
                    let _e1851 = sqrt(fma(_e129.z, _e129.z, fma(_e129.x, _e129.x, (_e129.y * _e129.y))));
                    if (_e1851 == 0f) {
                        phi_5883_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5883_ = (_e129 * (1f / _e1851));
                    }
                    let _e1856 = phi_5883_;
                    global_20 = vec4<f32>(((_e1856.x + 1f) * 0.5f), ((_e1856.y + 1f) * 0.5f), ((_e1856.z + 1f) * 0.5f), 1f);
                    phi_7288_ = false;
                    break;
                }
                case 6: {
                    let _e1829 = sqrt(fma(_e1353.z, _e1353.z, fma(_e1353.x, _e1353.x, (_e1353.y * _e1353.y))));
                    if (_e1829 == 0f) {
                        phi_5834_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5834_ = (_e1353 * (1f / _e1829));
                    }
                    let _e1834 = phi_5834_;
                    global_20 = vec4<f32>(((_e1834.x + 1f) * 0.5f), ((_e1834.y + 1f) * 0.5f), ((_e1834.z + 1f) * 0.5f), 1f);
                    phi_7288_ = false;
                    break;
                }
                case 7: {
                    let _e1807 = sqrt(fma(_e130.z, _e130.z, fma(_e130.x, _e130.x, (_e130.y * _e130.y))));
                    if (_e1807 == 0f) {
                        phi_5785_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5785_ = (_e130 * (1f / _e1807));
                    }
                    let _e1812 = phi_5785_;
                    global_20 = vec4<f32>(((_e1812.x + 1f) * 0.5f), ((_e1812.y + 1f) * 0.5f), ((_e1812.z + 1f) * 0.5f), 1f);
                    phi_7288_ = false;
                    break;
                }
                case 8: {
                    let _e1785 = sqrt(fma(_e131.z, _e131.z, fma(_e131.x, _e131.x, (_e131.y * _e131.y))));
                    if (_e1785 == 0f) {
                        phi_5736_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5736_ = (_e131 * (1f / _e1785));
                    }
                    let _e1790 = phi_5736_;
                    global_20 = vec4<f32>(((_e1790.x + 1f) * 0.5f), ((_e1790.y + 1f) * 0.5f), ((_e1790.z + 1f) * 0.5f), 1f);
                    phi_7288_ = false;
                    break;
                }
                case 9: {
                    global_20 = vec4<f32>(_e1401.x, _e1401.y, _e1401.z, 1f);
                    phi_7288_ = false;
                    break;
                }
                case 10: {
                    global_20 = vec4<f32>(_e1749.x, _e1749.y, _e1749.z, 1f);
                    phi_7288_ = false;
                    break;
                }
                case 11: {
                    global_20 = vec4<f32>(_e1765.x, _e1765.y, 1f, 1f);
                    phi_7288_ = false;
                    break;
                }
                case 12: {
                    global_20 = (vec4<f32>(_e1359, _e1362, _e1365, (_e492.w * _e295.member_2.w)) * _e126);
                    phi_7288_ = false;
                    break;
                }
                case 13: {
                    global_20 = vec4<f32>(_e1379, _e1379, _e1379, 1f);
                    phi_7288_ = false;
                    break;
                }
                case 14: {
                    global_20 = vec4<f32>(_e1382, _e1382, _e1382, 1f);
                    phi_7288_ = false;
                    break;
                }
                case 15: {
                    global_20 = vec4<f32>(_e1386, _e1386, _e1386, 1f);
                    phi_7288_ = false;
                    break;
                }
                case 16: {
                    global_20 = vec4<f32>((_e1392 * _e295.member_1), (_e1394 * _e295.member_1), (_e1396 * _e295.member_1), 1f);
                    phi_7288_ = false;
                    break;
                }
                case 17: {
                    global_20 = vec4<f32>(_e1268.x, _e1268.y, _e1268.z, 1f);
                    phi_7288_ = false;
                    break;
                }
                case 18: {
                    global_20 = vec4<f32>(_e295.member.x, _e295.member.y, _e295.member.z, 1f);
                    phi_7288_ = false;
                    break;
                }
                case 19: {
                    global_20 = vec4<f32>(_e295.member_1, _e295.member_1, _e295.member_1, 1f);
                    phi_7288_ = false;
                    break;
                }
                default: {
                    phi_7288_ = false;
                    break;
                }
            }
            let _e3411 = phi_7288_;
            if _e3411 {
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
