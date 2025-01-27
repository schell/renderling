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
    member_2: vec3<f32>,
}

struct type_22 {
    member: type_20,
    member_1: type_20,
    member_2: vec3<f32>,
    member_3: type_21,
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
@group(2) @binding(0) 
var<storage> global_1: type_11;
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
@group(2) @binding(1) 
var global_18: texture_depth_2d;
@group(2) @binding(2) 
var global_19: sampler;
var<private> global_20: vec4<f32>;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var phi_632_: u32;
    var phi_4081_: bool;
    var phi_786_: type_32;
    var phi_790_: type_32;
    var phi_4118_: bool;
    var phi_830_: u32;
    var phi_839_: u32;
    var phi_852_: type_33;
    var phi_4140_: f32;
    var phi_4153_: bool;
    var phi_906_: f32;
    var phi_901_: f32;
    var phi_907_: f32;
    var phi_4170_: bool;
    var phi_872_: f32;
    var phi_909_: f32;
    var phi_4188_: f32;
    var phi_4201_: bool;
    var phi_964_: f32;
    var phi_959_: f32;
    var phi_965_: f32;
    var phi_4218_: bool;
    var phi_930_: f32;
    var phi_967_: f32;
    var phi_4254_: bool;
    var phi_1050_: u32;
    var phi_1059_: u32;
    var phi_1072_: type_33;
    var phi_4275_: f32;
    var phi_4288_: bool;
    var phi_1126_: f32;
    var phi_1121_: f32;
    var phi_1127_: f32;
    var phi_4305_: bool;
    var phi_1092_: f32;
    var phi_1129_: f32;
    var phi_4323_: f32;
    var phi_4336_: bool;
    var phi_1184_: f32;
    var phi_1179_: f32;
    var phi_1185_: f32;
    var phi_4353_: bool;
    var phi_1150_: f32;
    var phi_1187_: f32;
    var phi_4389_: bool;
    var phi_1270_: u32;
    var phi_1279_: u32;
    var phi_1292_: type_33;
    var phi_4410_: f32;
    var phi_4423_: bool;
    var phi_1346_: f32;
    var phi_1341_: f32;
    var phi_1347_: f32;
    var phi_4440_: bool;
    var phi_1312_: f32;
    var phi_1349_: f32;
    var phi_4458_: f32;
    var phi_4471_: bool;
    var phi_1404_: f32;
    var phi_1399_: f32;
    var phi_1405_: f32;
    var phi_4488_: bool;
    var phi_1370_: f32;
    var phi_1407_: f32;
    var phi_4524_: bool;
    var phi_1490_: u32;
    var phi_1499_: u32;
    var phi_1512_: type_33;
    var phi_4545_: f32;
    var phi_4558_: bool;
    var phi_1566_: f32;
    var phi_1561_: f32;
    var phi_1567_: f32;
    var phi_4575_: bool;
    var phi_1532_: f32;
    var phi_1569_: f32;
    var phi_4593_: f32;
    var phi_4606_: bool;
    var phi_1624_: f32;
    var phi_1619_: f32;
    var phi_1625_: f32;
    var phi_4623_: bool;
    var phi_1590_: f32;
    var phi_1627_: f32;
    var phi_4659_: bool;
    var phi_1710_: u32;
    var phi_1719_: u32;
    var phi_1732_: type_33;
    var phi_4680_: f32;
    var phi_4693_: bool;
    var phi_1786_: f32;
    var phi_1781_: f32;
    var phi_1787_: f32;
    var phi_4710_: bool;
    var phi_1752_: f32;
    var phi_1789_: f32;
    var phi_4728_: f32;
    var phi_4741_: bool;
    var phi_1844_: f32;
    var phi_1839_: f32;
    var phi_1845_: f32;
    var phi_4758_: bool;
    var phi_1810_: f32;
    var phi_1847_: f32;
    var phi_4816_: vec3<f32>;
    var phi_4851_: vec3<f32>;
    var phi_4886_: vec3<f32>;
    var phi_4921_: vec3<f32>;
    var phi_4956_: vec3<f32>;
    var phi_1941_: vec3<f32>;
    var phi_1942_: vec3<f32>;
    var phi_4988_: bool;
    var phi_2149_: type_24;
    var phi_2150_: type_24;
    var phi_2173_: type_24;
    var phi_2200_: bool;
    var phi_2206_: type_24;
    var phi_2207_: type_24;
    var phi_2230_: type_24;
    var phi_2253_: bool;
    var phi_2274_: type_22;
    var phi_5060_: vec3<f32>;
    var phi_5119_: vec3<f32>;
    var phi_5193_: vec3<f32>;
    var phi_5253_: vec3<f32>;
    var phi_5302_: vec3<f32>;
    var phi_5351_: vec3<f32>;
    var phi_5400_: vec3<f32>;
    var phi_5449_: vec3<f32>;
    var phi_5498_: vec3<f32>;
    var phi_5547_: vec3<f32>;
    var phi_5586_: vec3<f32>;
    var phi_5621_: vec3<f32>;
    var phi_2314_: type_24;
    var phi_2317_: vec3<f32>;
    var phi_2315_: type_24;
    var phi_2340_: type_24;
    var phi_5638_: u32;
    var phi_5657_: bool;
    var phi_2357_: u32;
    var phi_5689_: bool;
    var phi_2374_: u32;
    var phi_2384_: type_34;
    var phi_5719_: bool;
    var phi_2434_: type_29;
    var phi_5799_: bool;
    var phi_3071_: type_36;
    var phi_5849_: vec3<f32>;
    var phi_5884_: vec3<f32>;
    var phi_5919_: vec3<f32>;
    var phi_3326_: vec3<f32>;
    var phi_6011_: bool;
    var phi_2818_: type_35;
    var phi_6058_: vec3<f32>;
    var phi_6093_: vec3<f32>;
    var phi_3008_: vec3<f32>;
    var phi_6185_: bool;
    var phi_2482_: type_35;
    var phi_6232_: vec3<f32>;
    var phi_6267_: vec3<f32>;
    var phi_3328_: vec3<f32>;
    var phi_3329_: bool;
    var phi_3338_: vec3<f32>;
    var phi_2318_: vec3<f32>;
    var phi_3340_: bool;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var phi_3457_: vec4<f32>;

    let _e118 = arrayLength((&global.member));
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
            phi_632_ = 0u;
            break;
        }
        case 1: {
            phi_632_ = 1u;
            break;
        }
        case 2: {
            phi_632_ = 2u;
            break;
        }
        case 3: {
            phi_632_ = 3u;
            break;
        }
        case 4: {
            phi_632_ = 4u;
            break;
        }
        case 5: {
            phi_632_ = 5u;
            break;
        }
        case 6: {
            phi_632_ = 6u;
            break;
        }
        case 7: {
            phi_632_ = 7u;
            break;
        }
        case 8: {
            phi_632_ = 8u;
            break;
        }
        case 9: {
            phi_632_ = 9u;
            break;
        }
        case 10: {
            phi_632_ = 10u;
            break;
        }
        case 11: {
            phi_632_ = 11u;
            break;
        }
        case 12: {
            phi_632_ = 12u;
            break;
        }
        case 13: {
            phi_632_ = 13u;
            break;
        }
        case 14: {
            phi_632_ = 14u;
            break;
        }
        case 15: {
            phi_632_ = 15u;
            break;
        }
        case 16: {
            phi_632_ = 16u;
            break;
        }
        case 17: {
            phi_632_ = 17u;
            break;
        }
        case 18: {
            phi_632_ = 18u;
            break;
        }
        case 19: {
            phi_632_ = 19u;
            break;
        }
        default: {
            phi_632_ = 0u;
            break;
        }
    }
    let _e152 = phi_632_;
    let _e156 = global.member[(_e138 + 5u)];
    let _e161 = global.member[(_e138 + 9u)];
    let _e165 = global.member[(_e138 + 10u)];
    if (_e134 == 4294967295u) {
        phi_790_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e118 >= 22u) {
            phi_4081_ = (_e134 <= (_e118 - 22u));
        } else {
            phi_4081_ = false;
        }
        let _e171 = phi_4081_;
        if _e171 {
            let _e174 = global.member[_e134];
            let _e179 = global.member[(_e134 + 1u)];
            let _e184 = global.member[(_e134 + 2u)];
            let _e190 = global.member[(_e134 + 3u)];
            let _e195 = global.member[(_e134 + 4u)];
            let _e200 = global.member[(_e134 + 5u)];
            let _e205 = global.member[(_e134 + 6u)];
            let _e210 = global.member[(_e134 + 7u)];
            let _e216 = global.member[(_e134 + 8u)];
            let _e221 = global.member[(_e134 + 9u)];
            let _e226 = global.member[(_e134 + 10u)];
            let _e230 = global.member[(_e134 + 11u)];
            let _e234 = global.member[(_e134 + 12u)];
            let _e238 = global.member[(_e134 + 13u)];
            let _e242 = global.member[(_e134 + 14u)];
            let _e246 = global.member[(_e134 + 15u)];
            let _e250 = global.member[(_e134 + 16u)];
            let _e254 = global.member[(_e134 + 17u)];
            let _e258 = global.member[(_e134 + 18u)];
            let _e262 = global.member[(_e134 + 19u)];
            let _e266 = global.member[(_e134 + 20u)];
            let _e271 = global.member[(_e134 + 21u)];
            phi_786_ = type_32(vec3<f32>(bitcast<f32>(_e174), bitcast<f32>(_e179), bitcast<f32>(_e184)), bitcast<f32>(_e190), vec4<f32>(bitcast<f32>(_e195), bitcast<f32>(_e200), bitcast<f32>(_e205), bitcast<f32>(_e210)), bitcast<f32>(_e216), bitcast<f32>(_e221), _e226, _e230, _e234, _e238, _e242, _e246, _e250, _e254, _e258, _e262, (_e266 == 1u), bitcast<f32>(_e271));
        } else {
            phi_786_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e275 = phi_786_;
        phi_790_ = type_32(_e275.member, _e275.member_1, _e275.member_2, _e275.member_3, _e275.member_4, _e275.member_5, _e275.member_6, _e275.member_7, _e275.member_8, _e275.member_9, _e275.member_10, _e275.member_11, _e275.member_12, _e275.member_13, _e275.member_14, (_e275.member_15 && (_e156 == 1u)), _e275.member_16);
    }
    let _e297 = phi_790_;
    let _e301 = select(_e122, _e121, vec2((_e297.member_10 == 0u)));
    let _e303 = (_e118 >= 8u);
    if _e303 {
        phi_4118_ = (_e297.member_5 <= (_e118 - 8u));
    } else {
        phi_4118_ = false;
    }
    let _e307 = phi_4118_;
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
                phi_830_ = 0u;
                break;
            }
            case 1: {
                phi_830_ = 1u;
                break;
            }
            case 2: {
                phi_830_ = 2u;
                break;
            }
            default: {
                phi_830_ = 0u;
                break;
            }
        }
        let _e339 = phi_830_;
        let _e343 = global.member[(_e297.member_5 + 7u)];
        switch bitcast<i32>(_e343) {
            case 0: {
                phi_839_ = 0u;
                break;
            }
            case 1: {
                phi_839_ = 1u;
                break;
            }
            case 2: {
                phi_839_ = 2u;
                break;
            }
            default: {
                phi_839_ = 0u;
                break;
            }
        }
        let _e346 = phi_839_;
        phi_852_ = type_33(type_24(_e339, _e346), vec2<u32>(_e310, _e314), vec2<u32>(_e319, _e323), _e328, _e332);
    } else {
        phi_852_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e350 = phi_852_;
    switch bitcast<i32>(_e350.member.member) {
        case 1: {
            let _e388 = abs(_e301.x);
            let _e390 = (_e388 % 1f);
            if (_e388 >= 1f) {
                phi_4170_ = select(true, false, (_e390 == 0f));
            } else {
                phi_4170_ = true;
            }
            let _e394 = phi_4170_;
            let _e395 = select(1f, _e390, _e394);
            if (select(-1f, 1f, (_e301.x >= 0f)) > 0f) {
                phi_872_ = _e395;
            } else {
                phi_872_ = (1f - _e395);
            }
            let _e399 = phi_872_;
            phi_909_ = _e399;
            break;
        }
        case 2: {
            let _e362 = abs(_e301.x);
            let _e369 = ((select(select(u32(_e362), 0u, (_e362 < 0f)), 4294967295u, (_e362 > 4294967000f)) % 2u) == 0u);
            let _e371 = (_e362 % 1f);
            if (_e362 >= 1f) {
                phi_4153_ = select(true, false, (_e371 == 0f));
            } else {
                phi_4153_ = true;
            }
            let _e375 = phi_4153_;
            let _e376 = select(1f, _e371, _e375);
            if (select(-1f, 1f, (_e301.x >= 0f)) > 0f) {
                if _e369 {
                    phi_901_ = _e376;
                } else {
                    phi_901_ = (1f - _e376);
                }
                let _e383 = phi_901_;
                phi_907_ = _e383;
            } else {
                if _e369 {
                    phi_906_ = (1f - _e376);
                } else {
                    phi_906_ = _e376;
                }
                let _e380 = phi_906_;
                phi_907_ = _e380;
            }
            let _e385 = phi_907_;
            phi_909_ = _e385;
            break;
        }
        case 0: {
            if (_e301.x > 1f) {
                phi_4140_ = 0.9999999f;
            } else {
                phi_4140_ = select(_e301.x, 0.00000011920929f, (_e301.x < 0f));
            }
            let _e359 = phi_4140_;
            phi_909_ = _e359;
            break;
        }
        default: {
            phi_909_ = f32();
            break;
        }
    }
    let _e401 = phi_909_;
    switch bitcast<i32>(_e350.member.member_1) {
        case 1: {
            let _e439 = abs(_e301.y);
            let _e441 = (_e439 % 1f);
            if (_e439 >= 1f) {
                phi_4218_ = select(true, false, (_e441 == 0f));
            } else {
                phi_4218_ = true;
            }
            let _e445 = phi_4218_;
            let _e446 = select(1f, _e441, _e445);
            if (select(-1f, 1f, (_e301.y >= 0f)) > 0f) {
                phi_930_ = _e446;
            } else {
                phi_930_ = (1f - _e446);
            }
            let _e450 = phi_930_;
            phi_967_ = _e450;
            break;
        }
        case 2: {
            let _e413 = abs(_e301.y);
            let _e420 = ((select(select(u32(_e413), 0u, (_e413 < 0f)), 4294967295u, (_e413 > 4294967000f)) % 2u) == 0u);
            let _e422 = (_e413 % 1f);
            if (_e413 >= 1f) {
                phi_4201_ = select(true, false, (_e422 == 0f));
            } else {
                phi_4201_ = true;
            }
            let _e426 = phi_4201_;
            let _e427 = select(1f, _e422, _e426);
            if (select(-1f, 1f, (_e301.y >= 0f)) > 0f) {
                if _e420 {
                    phi_959_ = _e427;
                } else {
                    phi_959_ = (1f - _e427);
                }
                let _e434 = phi_959_;
                phi_965_ = _e434;
            } else {
                if _e420 {
                    phi_964_ = (1f - _e427);
                } else {
                    phi_964_ = _e427;
                }
                let _e431 = phi_964_;
                phi_965_ = _e431;
            }
            let _e436 = phi_965_;
            phi_967_ = _e436;
            break;
        }
        case 0: {
            if (_e301.y > 1f) {
                phi_4188_ = 0.9999999f;
            } else {
                phi_4188_ = select(_e301.y, 0.00000011920929f, (_e301.y < 0f));
            }
            let _e410 = phi_4188_;
            phi_967_ = _e410;
            break;
        }
        default: {
            phi_967_ = f32();
            break;
        }
    }
    let _e452 = phi_967_;
    let _e456 = (_e401 * f32(_e350.member_2.x));
    let _e465 = (_e452 * f32(_e350.member_2.y));
    let _e477 = f32(_e141);
    let _e478 = f32(_e145);
    let _e485 = vec3<f32>((f32((select(select(u32(_e456), 0u, (_e456 < 0f)), 4294967295u, (_e456 > 4294967000f)) + _e350.member_1.x)) / _e477), (f32((select(select(u32(_e465), 0u, (_e465 < 0f)), 4294967295u, (_e465 > 4294967000f)) + _e350.member_1.y)) / _e478), f32(_e350.member_3));
    let _e491 = textureSampleLevel(global_11, global_10, vec2<f32>(_e485.x, _e485.y), i32(_e485.z), 0f);
    let _e494 = select(_e491, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e297.member_5 == 4294967295u)));
    let _e498 = select(_e122, _e121, vec2((_e297.member_11 == 0u)));
    if _e303 {
        phi_4254_ = (_e297.member_6 <= (_e118 - 8u));
    } else {
        phi_4254_ = false;
    }
    let _e503 = phi_4254_;
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
                phi_1050_ = 0u;
                break;
            }
            case 1: {
                phi_1050_ = 1u;
                break;
            }
            case 2: {
                phi_1050_ = 2u;
                break;
            }
            default: {
                phi_1050_ = 0u;
                break;
            }
        }
        let _e535 = phi_1050_;
        let _e539 = global.member[(_e297.member_6 + 7u)];
        switch bitcast<i32>(_e539) {
            case 0: {
                phi_1059_ = 0u;
                break;
            }
            case 1: {
                phi_1059_ = 1u;
                break;
            }
            case 2: {
                phi_1059_ = 2u;
                break;
            }
            default: {
                phi_1059_ = 0u;
                break;
            }
        }
        let _e542 = phi_1059_;
        phi_1072_ = type_33(type_24(_e535, _e542), vec2<u32>(_e506, _e510), vec2<u32>(_e515, _e519), _e524, _e528);
    } else {
        phi_1072_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e546 = phi_1072_;
    switch bitcast<i32>(_e546.member.member) {
        case 1: {
            let _e584 = abs(_e498.x);
            let _e586 = (_e584 % 1f);
            if (_e584 >= 1f) {
                phi_4305_ = select(true, false, (_e586 == 0f));
            } else {
                phi_4305_ = true;
            }
            let _e590 = phi_4305_;
            let _e591 = select(1f, _e586, _e590);
            if (select(-1f, 1f, (_e498.x >= 0f)) > 0f) {
                phi_1092_ = _e591;
            } else {
                phi_1092_ = (1f - _e591);
            }
            let _e595 = phi_1092_;
            phi_1129_ = _e595;
            break;
        }
        case 2: {
            let _e558 = abs(_e498.x);
            let _e565 = ((select(select(u32(_e558), 0u, (_e558 < 0f)), 4294967295u, (_e558 > 4294967000f)) % 2u) == 0u);
            let _e567 = (_e558 % 1f);
            if (_e558 >= 1f) {
                phi_4288_ = select(true, false, (_e567 == 0f));
            } else {
                phi_4288_ = true;
            }
            let _e571 = phi_4288_;
            let _e572 = select(1f, _e567, _e571);
            if (select(-1f, 1f, (_e498.x >= 0f)) > 0f) {
                if _e565 {
                    phi_1121_ = _e572;
                } else {
                    phi_1121_ = (1f - _e572);
                }
                let _e579 = phi_1121_;
                phi_1127_ = _e579;
            } else {
                if _e565 {
                    phi_1126_ = (1f - _e572);
                } else {
                    phi_1126_ = _e572;
                }
                let _e576 = phi_1126_;
                phi_1127_ = _e576;
            }
            let _e581 = phi_1127_;
            phi_1129_ = _e581;
            break;
        }
        case 0: {
            if (_e498.x > 1f) {
                phi_4275_ = 0.9999999f;
            } else {
                phi_4275_ = select(_e498.x, 0.00000011920929f, (_e498.x < 0f));
            }
            let _e555 = phi_4275_;
            phi_1129_ = _e555;
            break;
        }
        default: {
            phi_1129_ = f32();
            break;
        }
    }
    let _e597 = phi_1129_;
    switch bitcast<i32>(_e546.member.member_1) {
        case 1: {
            let _e635 = abs(_e498.y);
            let _e637 = (_e635 % 1f);
            if (_e635 >= 1f) {
                phi_4353_ = select(true, false, (_e637 == 0f));
            } else {
                phi_4353_ = true;
            }
            let _e641 = phi_4353_;
            let _e642 = select(1f, _e637, _e641);
            if (select(-1f, 1f, (_e498.y >= 0f)) > 0f) {
                phi_1150_ = _e642;
            } else {
                phi_1150_ = (1f - _e642);
            }
            let _e646 = phi_1150_;
            phi_1187_ = _e646;
            break;
        }
        case 2: {
            let _e609 = abs(_e498.y);
            let _e616 = ((select(select(u32(_e609), 0u, (_e609 < 0f)), 4294967295u, (_e609 > 4294967000f)) % 2u) == 0u);
            let _e618 = (_e609 % 1f);
            if (_e609 >= 1f) {
                phi_4336_ = select(true, false, (_e618 == 0f));
            } else {
                phi_4336_ = true;
            }
            let _e622 = phi_4336_;
            let _e623 = select(1f, _e618, _e622);
            if (select(-1f, 1f, (_e498.y >= 0f)) > 0f) {
                if _e616 {
                    phi_1179_ = _e623;
                } else {
                    phi_1179_ = (1f - _e623);
                }
                let _e630 = phi_1179_;
                phi_1185_ = _e630;
            } else {
                if _e616 {
                    phi_1184_ = (1f - _e623);
                } else {
                    phi_1184_ = _e623;
                }
                let _e627 = phi_1184_;
                phi_1185_ = _e627;
            }
            let _e632 = phi_1185_;
            phi_1187_ = _e632;
            break;
        }
        case 0: {
            if (_e498.y > 1f) {
                phi_4323_ = 0.9999999f;
            } else {
                phi_4323_ = select(_e498.y, 0.00000011920929f, (_e498.y < 0f));
            }
            let _e606 = phi_4323_;
            phi_1187_ = _e606;
            break;
        }
        default: {
            phi_1187_ = f32();
            break;
        }
    }
    let _e648 = phi_1187_;
    let _e652 = (_e597 * f32(_e546.member_2.x));
    let _e661 = (_e648 * f32(_e546.member_2.y));
    let _e679 = vec3<f32>((f32((select(select(u32(_e652), 0u, (_e652 < 0f)), 4294967295u, (_e652 > 4294967000f)) + _e546.member_1.x)) / _e477), (f32((select(select(u32(_e661), 0u, (_e661 < 0f)), 4294967295u, (_e661 > 4294967000f)) + _e546.member_1.y)) / _e478), f32(_e546.member_3));
    let _e685 = textureSampleLevel(global_11, global_10, vec2<f32>(_e679.x, _e679.y), i32(_e679.z), 0f);
    let _e688 = select(_e685, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e297.member_6 == 4294967295u)));
    let _e692 = select(_e122, _e121, vec2((_e297.member_12 == 0u)));
    if _e303 {
        phi_4389_ = (_e297.member_7 <= (_e118 - 8u));
    } else {
        phi_4389_ = false;
    }
    let _e697 = phi_4389_;
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
                phi_1270_ = 0u;
                break;
            }
            case 1: {
                phi_1270_ = 1u;
                break;
            }
            case 2: {
                phi_1270_ = 2u;
                break;
            }
            default: {
                phi_1270_ = 0u;
                break;
            }
        }
        let _e729 = phi_1270_;
        let _e733 = global.member[(_e297.member_7 + 7u)];
        switch bitcast<i32>(_e733) {
            case 0: {
                phi_1279_ = 0u;
                break;
            }
            case 1: {
                phi_1279_ = 1u;
                break;
            }
            case 2: {
                phi_1279_ = 2u;
                break;
            }
            default: {
                phi_1279_ = 0u;
                break;
            }
        }
        let _e736 = phi_1279_;
        phi_1292_ = type_33(type_24(_e729, _e736), vec2<u32>(_e700, _e704), vec2<u32>(_e709, _e713), _e718, _e722);
    } else {
        phi_1292_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e740 = phi_1292_;
    switch bitcast<i32>(_e740.member.member) {
        case 1: {
            let _e778 = abs(_e692.x);
            let _e780 = (_e778 % 1f);
            if (_e778 >= 1f) {
                phi_4440_ = select(true, false, (_e780 == 0f));
            } else {
                phi_4440_ = true;
            }
            let _e784 = phi_4440_;
            let _e785 = select(1f, _e780, _e784);
            if (select(-1f, 1f, (_e692.x >= 0f)) > 0f) {
                phi_1312_ = _e785;
            } else {
                phi_1312_ = (1f - _e785);
            }
            let _e789 = phi_1312_;
            phi_1349_ = _e789;
            break;
        }
        case 2: {
            let _e752 = abs(_e692.x);
            let _e759 = ((select(select(u32(_e752), 0u, (_e752 < 0f)), 4294967295u, (_e752 > 4294967000f)) % 2u) == 0u);
            let _e761 = (_e752 % 1f);
            if (_e752 >= 1f) {
                phi_4423_ = select(true, false, (_e761 == 0f));
            } else {
                phi_4423_ = true;
            }
            let _e765 = phi_4423_;
            let _e766 = select(1f, _e761, _e765);
            if (select(-1f, 1f, (_e692.x >= 0f)) > 0f) {
                if _e759 {
                    phi_1341_ = _e766;
                } else {
                    phi_1341_ = (1f - _e766);
                }
                let _e773 = phi_1341_;
                phi_1347_ = _e773;
            } else {
                if _e759 {
                    phi_1346_ = (1f - _e766);
                } else {
                    phi_1346_ = _e766;
                }
                let _e770 = phi_1346_;
                phi_1347_ = _e770;
            }
            let _e775 = phi_1347_;
            phi_1349_ = _e775;
            break;
        }
        case 0: {
            if (_e692.x > 1f) {
                phi_4410_ = 0.9999999f;
            } else {
                phi_4410_ = select(_e692.x, 0.00000011920929f, (_e692.x < 0f));
            }
            let _e749 = phi_4410_;
            phi_1349_ = _e749;
            break;
        }
        default: {
            phi_1349_ = f32();
            break;
        }
    }
    let _e791 = phi_1349_;
    switch bitcast<i32>(_e740.member.member_1) {
        case 1: {
            let _e829 = abs(_e692.y);
            let _e831 = (_e829 % 1f);
            if (_e829 >= 1f) {
                phi_4488_ = select(true, false, (_e831 == 0f));
            } else {
                phi_4488_ = true;
            }
            let _e835 = phi_4488_;
            let _e836 = select(1f, _e831, _e835);
            if (select(-1f, 1f, (_e692.y >= 0f)) > 0f) {
                phi_1370_ = _e836;
            } else {
                phi_1370_ = (1f - _e836);
            }
            let _e840 = phi_1370_;
            phi_1407_ = _e840;
            break;
        }
        case 2: {
            let _e803 = abs(_e692.y);
            let _e810 = ((select(select(u32(_e803), 0u, (_e803 < 0f)), 4294967295u, (_e803 > 4294967000f)) % 2u) == 0u);
            let _e812 = (_e803 % 1f);
            if (_e803 >= 1f) {
                phi_4471_ = select(true, false, (_e812 == 0f));
            } else {
                phi_4471_ = true;
            }
            let _e816 = phi_4471_;
            let _e817 = select(1f, _e812, _e816);
            if (select(-1f, 1f, (_e692.y >= 0f)) > 0f) {
                if _e810 {
                    phi_1399_ = _e817;
                } else {
                    phi_1399_ = (1f - _e817);
                }
                let _e824 = phi_1399_;
                phi_1405_ = _e824;
            } else {
                if _e810 {
                    phi_1404_ = (1f - _e817);
                } else {
                    phi_1404_ = _e817;
                }
                let _e821 = phi_1404_;
                phi_1405_ = _e821;
            }
            let _e826 = phi_1405_;
            phi_1407_ = _e826;
            break;
        }
        case 0: {
            if (_e692.y > 1f) {
                phi_4458_ = 0.9999999f;
            } else {
                phi_4458_ = select(_e692.y, 0.00000011920929f, (_e692.y < 0f));
            }
            let _e800 = phi_4458_;
            phi_1407_ = _e800;
            break;
        }
        default: {
            phi_1407_ = f32();
            break;
        }
    }
    let _e842 = phi_1407_;
    let _e846 = (_e791 * f32(_e740.member_2.x));
    let _e855 = (_e842 * f32(_e740.member_2.y));
    let _e873 = vec3<f32>((f32((select(select(u32(_e846), 0u, (_e846 < 0f)), 4294967295u, (_e846 > 4294967000f)) + _e740.member_1.x)) / _e477), (f32((select(select(u32(_e855), 0u, (_e855 < 0f)), 4294967295u, (_e855 > 4294967000f)) + _e740.member_1.y)) / _e478), f32(_e740.member_3));
    let _e879 = textureSampleLevel(global_11, global_10, vec2<f32>(_e873.x, _e873.y), i32(_e873.z), 0f);
    let _e880 = (_e297.member_7 == 4294967295u);
    let _e882 = select(_e879, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e880));
    let _e886 = select(_e122, _e121, vec2((_e297.member_13 == 0u)));
    if _e303 {
        phi_4524_ = (_e297.member_8 <= (_e118 - 8u));
    } else {
        phi_4524_ = false;
    }
    let _e891 = phi_4524_;
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
                phi_1490_ = 0u;
                break;
            }
            case 1: {
                phi_1490_ = 1u;
                break;
            }
            case 2: {
                phi_1490_ = 2u;
                break;
            }
            default: {
                phi_1490_ = 0u;
                break;
            }
        }
        let _e923 = phi_1490_;
        let _e927 = global.member[(_e297.member_8 + 7u)];
        switch bitcast<i32>(_e927) {
            case 0: {
                phi_1499_ = 0u;
                break;
            }
            case 1: {
                phi_1499_ = 1u;
                break;
            }
            case 2: {
                phi_1499_ = 2u;
                break;
            }
            default: {
                phi_1499_ = 0u;
                break;
            }
        }
        let _e930 = phi_1499_;
        phi_1512_ = type_33(type_24(_e923, _e930), vec2<u32>(_e894, _e898), vec2<u32>(_e903, _e907), _e912, _e916);
    } else {
        phi_1512_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e934 = phi_1512_;
    switch bitcast<i32>(_e934.member.member) {
        case 1: {
            let _e972 = abs(_e886.x);
            let _e974 = (_e972 % 1f);
            if (_e972 >= 1f) {
                phi_4575_ = select(true, false, (_e974 == 0f));
            } else {
                phi_4575_ = true;
            }
            let _e978 = phi_4575_;
            let _e979 = select(1f, _e974, _e978);
            if (select(-1f, 1f, (_e886.x >= 0f)) > 0f) {
                phi_1532_ = _e979;
            } else {
                phi_1532_ = (1f - _e979);
            }
            let _e983 = phi_1532_;
            phi_1569_ = _e983;
            break;
        }
        case 2: {
            let _e946 = abs(_e886.x);
            let _e953 = ((select(select(u32(_e946), 0u, (_e946 < 0f)), 4294967295u, (_e946 > 4294967000f)) % 2u) == 0u);
            let _e955 = (_e946 % 1f);
            if (_e946 >= 1f) {
                phi_4558_ = select(true, false, (_e955 == 0f));
            } else {
                phi_4558_ = true;
            }
            let _e959 = phi_4558_;
            let _e960 = select(1f, _e955, _e959);
            if (select(-1f, 1f, (_e886.x >= 0f)) > 0f) {
                if _e953 {
                    phi_1561_ = _e960;
                } else {
                    phi_1561_ = (1f - _e960);
                }
                let _e967 = phi_1561_;
                phi_1567_ = _e967;
            } else {
                if _e953 {
                    phi_1566_ = (1f - _e960);
                } else {
                    phi_1566_ = _e960;
                }
                let _e964 = phi_1566_;
                phi_1567_ = _e964;
            }
            let _e969 = phi_1567_;
            phi_1569_ = _e969;
            break;
        }
        case 0: {
            if (_e886.x > 1f) {
                phi_4545_ = 0.9999999f;
            } else {
                phi_4545_ = select(_e886.x, 0.00000011920929f, (_e886.x < 0f));
            }
            let _e943 = phi_4545_;
            phi_1569_ = _e943;
            break;
        }
        default: {
            phi_1569_ = f32();
            break;
        }
    }
    let _e985 = phi_1569_;
    switch bitcast<i32>(_e934.member.member_1) {
        case 1: {
            let _e1023 = abs(_e886.y);
            let _e1025 = (_e1023 % 1f);
            if (_e1023 >= 1f) {
                phi_4623_ = select(true, false, (_e1025 == 0f));
            } else {
                phi_4623_ = true;
            }
            let _e1029 = phi_4623_;
            let _e1030 = select(1f, _e1025, _e1029);
            if (select(-1f, 1f, (_e886.y >= 0f)) > 0f) {
                phi_1590_ = _e1030;
            } else {
                phi_1590_ = (1f - _e1030);
            }
            let _e1034 = phi_1590_;
            phi_1627_ = _e1034;
            break;
        }
        case 2: {
            let _e997 = abs(_e886.y);
            let _e1004 = ((select(select(u32(_e997), 0u, (_e997 < 0f)), 4294967295u, (_e997 > 4294967000f)) % 2u) == 0u);
            let _e1006 = (_e997 % 1f);
            if (_e997 >= 1f) {
                phi_4606_ = select(true, false, (_e1006 == 0f));
            } else {
                phi_4606_ = true;
            }
            let _e1010 = phi_4606_;
            let _e1011 = select(1f, _e1006, _e1010);
            if (select(-1f, 1f, (_e886.y >= 0f)) > 0f) {
                if _e1004 {
                    phi_1619_ = _e1011;
                } else {
                    phi_1619_ = (1f - _e1011);
                }
                let _e1018 = phi_1619_;
                phi_1625_ = _e1018;
            } else {
                if _e1004 {
                    phi_1624_ = (1f - _e1011);
                } else {
                    phi_1624_ = _e1011;
                }
                let _e1015 = phi_1624_;
                phi_1625_ = _e1015;
            }
            let _e1020 = phi_1625_;
            phi_1627_ = _e1020;
            break;
        }
        case 0: {
            if (_e886.y > 1f) {
                phi_4593_ = 0.9999999f;
            } else {
                phi_4593_ = select(_e886.y, 0.00000011920929f, (_e886.y < 0f));
            }
            let _e994 = phi_4593_;
            phi_1627_ = _e994;
            break;
        }
        default: {
            phi_1627_ = f32();
            break;
        }
    }
    let _e1036 = phi_1627_;
    let _e1040 = (_e985 * f32(_e934.member_2.x));
    let _e1049 = (_e1036 * f32(_e934.member_2.y));
    let _e1067 = vec3<f32>((f32((select(select(u32(_e1040), 0u, (_e1040 < 0f)), 4294967295u, (_e1040 > 4294967000f)) + _e934.member_1.x)) / _e477), (f32((select(select(u32(_e1049), 0u, (_e1049 < 0f)), 4294967295u, (_e1049 > 4294967000f)) + _e934.member_1.y)) / _e478), f32(_e934.member_3));
    let _e1073 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1067.x, _e1067.y), i32(_e1067.z), 0f);
    let _e1080 = select(_e122, _e121, vec2((_e297.member_14 == 0u)));
    if _e303 {
        phi_4659_ = (_e297.member_9 <= (_e118 - 8u));
    } else {
        phi_4659_ = false;
    }
    let _e1085 = phi_4659_;
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
                phi_1710_ = 0u;
                break;
            }
            case 1: {
                phi_1710_ = 1u;
                break;
            }
            case 2: {
                phi_1710_ = 2u;
                break;
            }
            default: {
                phi_1710_ = 0u;
                break;
            }
        }
        let _e1117 = phi_1710_;
        let _e1121 = global.member[(_e297.member_9 + 7u)];
        switch bitcast<i32>(_e1121) {
            case 0: {
                phi_1719_ = 0u;
                break;
            }
            case 1: {
                phi_1719_ = 1u;
                break;
            }
            case 2: {
                phi_1719_ = 2u;
                break;
            }
            default: {
                phi_1719_ = 0u;
                break;
            }
        }
        let _e1124 = phi_1719_;
        phi_1732_ = type_33(type_24(_e1117, _e1124), vec2<u32>(_e1088, _e1092), vec2<u32>(_e1097, _e1101), _e1106, _e1110);
    } else {
        phi_1732_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1128 = phi_1732_;
    switch bitcast<i32>(_e1128.member.member) {
        case 1: {
            let _e1166 = abs(_e1080.x);
            let _e1168 = (_e1166 % 1f);
            if (_e1166 >= 1f) {
                phi_4710_ = select(true, false, (_e1168 == 0f));
            } else {
                phi_4710_ = true;
            }
            let _e1172 = phi_4710_;
            let _e1173 = select(1f, _e1168, _e1172);
            if (select(-1f, 1f, (_e1080.x >= 0f)) > 0f) {
                phi_1752_ = _e1173;
            } else {
                phi_1752_ = (1f - _e1173);
            }
            let _e1177 = phi_1752_;
            phi_1789_ = _e1177;
            break;
        }
        case 2: {
            let _e1140 = abs(_e1080.x);
            let _e1147 = ((select(select(u32(_e1140), 0u, (_e1140 < 0f)), 4294967295u, (_e1140 > 4294967000f)) % 2u) == 0u);
            let _e1149 = (_e1140 % 1f);
            if (_e1140 >= 1f) {
                phi_4693_ = select(true, false, (_e1149 == 0f));
            } else {
                phi_4693_ = true;
            }
            let _e1153 = phi_4693_;
            let _e1154 = select(1f, _e1149, _e1153);
            if (select(-1f, 1f, (_e1080.x >= 0f)) > 0f) {
                if _e1147 {
                    phi_1781_ = _e1154;
                } else {
                    phi_1781_ = (1f - _e1154);
                }
                let _e1161 = phi_1781_;
                phi_1787_ = _e1161;
            } else {
                if _e1147 {
                    phi_1786_ = (1f - _e1154);
                } else {
                    phi_1786_ = _e1154;
                }
                let _e1158 = phi_1786_;
                phi_1787_ = _e1158;
            }
            let _e1163 = phi_1787_;
            phi_1789_ = _e1163;
            break;
        }
        case 0: {
            if (_e1080.x > 1f) {
                phi_4680_ = 0.9999999f;
            } else {
                phi_4680_ = select(_e1080.x, 0.00000011920929f, (_e1080.x < 0f));
            }
            let _e1137 = phi_4680_;
            phi_1789_ = _e1137;
            break;
        }
        default: {
            phi_1789_ = f32();
            break;
        }
    }
    let _e1179 = phi_1789_;
    switch bitcast<i32>(_e1128.member.member_1) {
        case 1: {
            let _e1217 = abs(_e1080.y);
            let _e1219 = (_e1217 % 1f);
            if (_e1217 >= 1f) {
                phi_4758_ = select(true, false, (_e1219 == 0f));
            } else {
                phi_4758_ = true;
            }
            let _e1223 = phi_4758_;
            let _e1224 = select(1f, _e1219, _e1223);
            if (select(-1f, 1f, (_e1080.y >= 0f)) > 0f) {
                phi_1810_ = _e1224;
            } else {
                phi_1810_ = (1f - _e1224);
            }
            let _e1228 = phi_1810_;
            phi_1847_ = _e1228;
            break;
        }
        case 2: {
            let _e1191 = abs(_e1080.y);
            let _e1198 = ((select(select(u32(_e1191), 0u, (_e1191 < 0f)), 4294967295u, (_e1191 > 4294967000f)) % 2u) == 0u);
            let _e1200 = (_e1191 % 1f);
            if (_e1191 >= 1f) {
                phi_4741_ = select(true, false, (_e1200 == 0f));
            } else {
                phi_4741_ = true;
            }
            let _e1204 = phi_4741_;
            let _e1205 = select(1f, _e1200, _e1204);
            if (select(-1f, 1f, (_e1080.y >= 0f)) > 0f) {
                if _e1198 {
                    phi_1839_ = _e1205;
                } else {
                    phi_1839_ = (1f - _e1205);
                }
                let _e1212 = phi_1839_;
                phi_1845_ = _e1212;
            } else {
                if _e1198 {
                    phi_1844_ = (1f - _e1205);
                } else {
                    phi_1844_ = _e1205;
                }
                let _e1209 = phi_1844_;
                phi_1845_ = _e1209;
            }
            let _e1214 = phi_1845_;
            phi_1847_ = _e1214;
            break;
        }
        case 0: {
            if (_e1080.y > 1f) {
                phi_4728_ = 0.9999999f;
            } else {
                phi_4728_ = select(_e1080.y, 0.00000011920929f, (_e1080.y < 0f));
            }
            let _e1188 = phi_4728_;
            phi_1847_ = _e1188;
            break;
        }
        default: {
            phi_1847_ = f32();
            break;
        }
    }
    let _e1230 = phi_1847_;
    let _e1234 = (_e1179 * f32(_e1128.member_2.x));
    let _e1243 = (_e1230 * f32(_e1128.member_2.y));
    let _e1261 = vec3<f32>((f32((select(select(u32(_e1234), 0u, (_e1234 < 0f)), 4294967295u, (_e1234 > 4294967000f)) + _e1128.member_1.x)) / _e477), (f32((select(select(u32(_e1243), 0u, (_e1243 < 0f)), 4294967295u, (_e1243 > 4294967000f)) + _e1128.member_1.y)) / _e478), f32(_e1128.member_3));
    let _e1267 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1261.x, _e1261.y), i32(_e1261.z), 0f);
    let _e1270 = select(_e1267, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e297.member_9 == 4294967295u)));
    if _e880 {
        phi_1941_ = vec3<f32>(0f, 0f, 0f);
        phi_1942_ = _e123;
    } else {
        let _e1274 = fma(_e882.x, 2f, -1f);
        let _e1275 = fma(_e882.y, 2f, -1f);
        let _e1276 = fma(_e882.z, 2f, -1f);
        let _e1281 = sqrt(fma(_e1276, _e1276, fma(_e1274, _e1274, (_e1275 * _e1275))));
        if (_e1281 == 0f) {
            phi_4816_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4816_ = (vec3<f32>(_e1274, _e1275, _e1276) * (1f / _e1281));
        }
        let _e1286 = phi_4816_;
        let _e1293 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
        if (_e1293 == 0f) {
            phi_4851_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4851_ = (_e124 * (1f / _e1293));
        }
        let _e1298 = phi_4851_;
        let _e1305 = sqrt(fma(_e125.z, _e125.z, fma(_e125.x, _e125.x, (_e125.y * _e125.y))));
        if (_e1305 == 0f) {
            phi_4886_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4886_ = (_e125 * (1f / _e1305));
        }
        let _e1310 = phi_4886_;
        let _e1317 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
        if (_e1317 == 0f) {
            phi_4921_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4921_ = (_e123 * (1f / _e1317));
        }
        let _e1322 = phi_4921_;
        let _e1341 = fma(_e1322.x, _e1286.z, fma(_e1298.x, _e1286.x, (_e1310.x * _e1286.y)));
        let _e1342 = fma(_e1322.y, _e1286.z, fma(_e1298.y, _e1286.x, (_e1310.y * _e1286.y)));
        let _e1343 = fma(_e1322.z, _e1286.z, fma(_e1298.z, _e1286.x, (_e1310.z * _e1286.y)));
        let _e1348 = sqrt(fma(_e1343, _e1343, fma(_e1341, _e1341, (_e1342 * _e1342))));
        if (_e1348 == 0f) {
            phi_4956_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4956_ = (vec3<f32>(_e1341, _e1342, _e1343) * (1f / _e1348));
        }
        let _e1353 = phi_4956_;
        phi_1941_ = _e1286;
        phi_1942_ = _e1353;
    }
    let _e1355 = phi_1941_;
    let _e1357 = phi_1942_;
    let _e1361 = (_e494.x * _e297.member_2.x);
    let _e1364 = (_e494.y * _e297.member_2.y);
    let _e1367 = (_e494.z * _e297.member_2.z);
    let _e1372 = (_e1361 * _e120.x);
    let _e1374 = (_e1364 * _e120.y);
    let _e1376 = (_e1367 * _e120.z);
    let _e1381 = (_e688.y * _e297.member_4);
    let _e1384 = (_e688.z * _e297.member_3);
    let _e1388 = fma(_e297.member_16, (select(_e1073, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e297.member_8 == 4294967295u))).x - 1f), 1f);
    let _e1394 = (_e1270.x * _e297.member.x);
    let _e1396 = (_e1270.y * _e297.member.y);
    let _e1398 = (_e1270.z * _e297.member.z);
    let _e1403 = textureSampleLevel(global_12, global_13, _e1357, 0f);
    if (_e118 >= 86u) {
        phi_4988_ = (_e130 <= (_e118 - 86u));
    } else {
        phi_4988_ = false;
    }
    let _e1411 = phi_4988_;
    if _e1411 {
        let _e1414 = global.member[_e130];
        let _e1419 = global.member[(_e130 + 1u)];
        let _e1424 = global.member[(_e130 + 2u)];
        let _e1429 = global.member[(_e130 + 3u)];
        let _e1435 = global.member[(_e130 + 4u)];
        let _e1440 = global.member[(_e130 + 5u)];
        let _e1445 = global.member[(_e130 + 6u)];
        let _e1450 = global.member[(_e130 + 7u)];
        let _e1456 = global.member[(_e130 + 8u)];
        let _e1461 = global.member[(_e130 + 9u)];
        let _e1466 = global.member[(_e130 + 10u)];
        let _e1471 = global.member[(_e130 + 11u)];
        let _e1477 = global.member[(_e130 + 12u)];
        let _e1482 = global.member[(_e130 + 13u)];
        let _e1487 = global.member[(_e130 + 14u)];
        let _e1492 = global.member[(_e130 + 15u)];
        let _e1499 = global.member[(_e130 + 16u)];
        let _e1504 = global.member[(_e130 + 17u)];
        let _e1509 = global.member[(_e130 + 18u)];
        let _e1514 = global.member[(_e130 + 19u)];
        let _e1520 = global.member[(_e130 + 20u)];
        let _e1525 = global.member[(_e130 + 21u)];
        let _e1530 = global.member[(_e130 + 22u)];
        let _e1535 = global.member[(_e130 + 23u)];
        let _e1541 = global.member[(_e130 + 24u)];
        let _e1546 = global.member[(_e130 + 25u)];
        let _e1551 = global.member[(_e130 + 26u)];
        let _e1556 = global.member[(_e130 + 27u)];
        let _e1562 = global.member[(_e130 + 28u)];
        let _e1567 = global.member[(_e130 + 29u)];
        let _e1572 = global.member[(_e130 + 30u)];
        let _e1577 = global.member[(_e130 + 31u)];
        let _e1584 = global.member[(_e130 + 32u)];
        let _e1589 = global.member[(_e130 + 33u)];
        let _e1594 = global.member[(_e130 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_2149_ = type_24(0u, 6u);
        loop {
            let _e1599 = phi_2149_;
            if (_e1599.member < _e1599.member_1) {
                phi_2150_ = type_24((_e1599.member + 1u), _e1599.member_1);
                phi_2173_ = type_24(1u, _e1599.member);
            } else {
                phi_2150_ = _e1599;
                phi_2173_ = type_24(0u, type_24().member_1);
            }
            let _e1612 = phi_2150_;
            let _e1614 = phi_2173_;
            switch bitcast<i32>(_e1614.member) {
                case 0: {
                    phi_2200_ = false;
                    break;
                }
                case 1: {
                    let _e1619 = ((_e130 + 35u) + (_e1614.member_1 * 4u));
                    let _e1622 = global.member[_e1619];
                    let _e1627 = global.member[(_e1619 + 1u)];
                    let _e1632 = global.member[(_e1619 + 2u)];
                    let _e1637 = global.member[(_e1619 + 3u)];
                    local_1[_e1614.member_1] = vec4<f32>(bitcast<f32>(_e1622), bitcast<f32>(_e1627), bitcast<f32>(_e1632), bitcast<f32>(_e1637));
                    phi_2200_ = true;
                    break;
                }
                default: {
                    phi_2200_ = bool();
                    break;
                }
            }
            let _e1642 = phi_2200_;
            continue;
            continuing {
                phi_2149_ = _e1612;
                break if !(_e1642);
            }
        }
        let _e1644 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2206_ = type_24(0u, 8u);
        loop {
            let _e1647 = phi_2206_;
            if (_e1647.member < _e1647.member_1) {
                phi_2207_ = type_24((_e1647.member + 1u), _e1647.member_1);
                phi_2230_ = type_24(1u, _e1647.member);
            } else {
                phi_2207_ = _e1647;
                phi_2230_ = type_24(0u, type_24().member_1);
            }
            let _e1660 = phi_2207_;
            let _e1662 = phi_2230_;
            switch bitcast<i32>(_e1662.member) {
                case 0: {
                    phi_2253_ = false;
                    break;
                }
                case 1: {
                    let _e1667 = ((_e130 + 59u) + (_e1662.member_1 * 3u));
                    let _e1670 = global.member[_e1667];
                    let _e1675 = global.member[(_e1667 + 1u)];
                    let _e1680 = global.member[(_e1667 + 2u)];
                    local[_e1662.member_1] = vec3<f32>(bitcast<f32>(_e1670), bitcast<f32>(_e1675), bitcast<f32>(_e1680));
                    phi_2253_ = true;
                    break;
                }
                default: {
                    phi_2253_ = bool();
                    break;
                }
            }
            let _e1685 = phi_2253_;
            continue;
            continuing {
                phi_2206_ = _e1660;
                break if !(_e1685);
            }
        }
        let _e1687 = local;
        let _e1691 = global.member[(_e130 + 83u)];
        let _e1696 = global.member[(_e130 + 84u)];
        let _e1701 = global.member[(_e130 + 85u)];
        phi_2274_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1414), bitcast<f32>(_e1419), bitcast<f32>(_e1424), bitcast<f32>(_e1429)), vec4<f32>(bitcast<f32>(_e1435), bitcast<f32>(_e1440), bitcast<f32>(_e1445), bitcast<f32>(_e1450)), vec4<f32>(bitcast<f32>(_e1456), bitcast<f32>(_e1461), bitcast<f32>(_e1466), bitcast<f32>(_e1471)), vec4<f32>(bitcast<f32>(_e1477), bitcast<f32>(_e1482), bitcast<f32>(_e1487), bitcast<f32>(_e1492))), type_20(vec4<f32>(bitcast<f32>(_e1499), bitcast<f32>(_e1504), bitcast<f32>(_e1509), bitcast<f32>(_e1514)), vec4<f32>(bitcast<f32>(_e1520), bitcast<f32>(_e1525), bitcast<f32>(_e1530), bitcast<f32>(_e1535)), vec4<f32>(bitcast<f32>(_e1541), bitcast<f32>(_e1546), bitcast<f32>(_e1551), bitcast<f32>(_e1556)), vec4<f32>(bitcast<f32>(_e1562), bitcast<f32>(_e1567), bitcast<f32>(_e1572), bitcast<f32>(_e1577))), vec3<f32>(bitcast<f32>(_e1584), bitcast<f32>(_e1589), bitcast<f32>(_e1594)), type_21(_e1687, _e1644, vec3<f32>(bitcast<f32>(_e1691), bitcast<f32>(_e1696), bitcast<f32>(_e1701))));
    } else {
        phi_2274_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e1707 = phi_2274_;
    let _e1709 = (_e1707.member_2 - _e126);
    let _e1716 = sqrt(fma(_e1709.z, _e1709.z, fma(_e1709.x, _e1709.x, (_e1709.y * _e1709.y))));
    let _e1717 = (_e1716 == 0f);
    if _e1717 {
        phi_5060_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5060_ = (_e1709 * (1f / _e1716));
    }
    let _e1721 = phi_5060_;
    let _e1722 = -(_e1721);
    let _e1729 = sqrt(fma(_e1357.z, _e1357.z, fma(_e1357.x, _e1357.x, (_e1357.y * _e1357.y))));
    let _e1730 = (_e1729 == 0f);
    if _e1730 {
        phi_5119_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5119_ = (_e1357 * (1f / _e1729));
    }
    let _e1734 = phi_5119_;
    let _e1744 = (2f * fma(_e1734.z, _e1722.z, fma(_e1734.x, _e1722.x, (_e1734.y * _e1722.y))));
    let _e1751 = textureSampleLevel(global_14, global_15, (_e1722 - vec3<f32>((_e1744 * _e1734.x), (_e1744 * _e1734.y), (_e1744 * _e1734.z))), (_e1381 * 4f));
    if _e1717 {
        phi_5193_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5193_ = (_e1709 * (1f / _e1716));
    }
    let _e1758 = phi_5193_;
    let _e1767 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1357.z, _e1758.z, fma(_e1357.x, _e1758.x, (_e1357.y * _e1758.y))), 0f), _e1381), 0f);
    switch bitcast<i32>(_e152) {
        case 0: {
            if _e297.member_15 {
                if _e1730 {
                    phi_5586_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5586_ = (_e1357 * (1f / _e1729));
                }
                let _e1936 = phi_5586_;
                if _e1717 {
                    phi_5621_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5621_ = (_e1709 * (1f / _e1716));
                }
                let _e1940 = phi_5621_;
                phi_2314_ = type_24(0u, _e165);
                phi_2317_ = vec3<f32>(0f, 0f, 0f);
                loop {
                    let _e1943 = phi_2314_;
                    let _e1945 = phi_2317_;
                    local_2 = _e1945;
                    local_3 = _e1945;
                    local_4 = _e1945;
                    if (_e1943.member < _e1943.member_1) {
                        phi_2315_ = type_24((_e1943.member + 1u), _e1943.member_1);
                        phi_2340_ = type_24(1u, _e1943.member);
                    } else {
                        phi_2315_ = _e1943;
                        phi_2340_ = type_24(0u, type_24().member_1);
                    }
                    let _e1958 = phi_2315_;
                    let _e1960 = phi_2340_;
                    switch bitcast<i32>(_e1960.member) {
                        case 0: {
                            phi_2318_ = vec3<f32>();
                            phi_3340_ = false;
                            break;
                        }
                        case 1: {
                            if (_e1960.member_1 >= _e165) {
                                phi_5638_ = 4294967295u;
                            } else {
                                phi_5638_ = (_e161 + _e1960.member_1);
                            }
                            let _e1967 = phi_5638_;
                            if (_e118 >= 1u) {
                                phi_5657_ = (_e1967 <= (_e118 - 1u));
                            } else {
                                phi_5657_ = false;
                            }
                            let _e1972 = phi_5657_;
                            if _e1972 {
                                let _e1975 = global.member[_e1967];
                                phi_2357_ = _e1975;
                            } else {
                                phi_2357_ = 4294967295u;
                            }
                            let _e1977 = phi_2357_;
                            let _e1978 = (_e1977 == 4294967295u);
                            if _e1978 {
                                phi_3338_ = vec3<f32>();
                            } else {
                                if (_e118 >= 3u) {
                                    phi_5689_ = (_e1977 <= (_e118 - 3u));
                                } else {
                                    phi_5689_ = false;
                                }
                                let _e1983 = phi_5689_;
                                if _e1983 {
                                    let _e1986 = global.member[_e1977];
                                    switch bitcast<i32>(_e1986) {
                                        case 0: {
                                            phi_2374_ = 0u;
                                            break;
                                        }
                                        case 1: {
                                            phi_2374_ = 1u;
                                            break;
                                        }
                                        case 2: {
                                            phi_2374_ = 2u;
                                            break;
                                        }
                                        default: {
                                            phi_2374_ = 0u;
                                            break;
                                        }
                                    }
                                    let _e1989 = phi_2374_;
                                    let _e1993 = global.member[(_e1977 + 1u)];
                                    let _e1997 = global.member[(_e1977 + 2u)];
                                    phi_2384_ = type_34(_e1989, _e1993, _e1997);
                                } else {
                                    phi_2384_ = type_34(0u, 4294967295u, 4294967295u);
                                }
                                let _e2000 = phi_2384_;
                                if (_e118 >= 10u) {
                                    phi_5719_ = (_e2000.member_2 <= (_e118 - 10u));
                                } else {
                                    phi_5719_ = false;
                                }
                                let _e2006 = phi_5719_;
                                if _e2006 {
                                    let _e2009 = global.member[_e2000.member_2];
                                    let _e2014 = global.member[(_e2000.member_2 + 1u)];
                                    let _e2019 = global.member[(_e2000.member_2 + 2u)];
                                    let _e2025 = global.member[(_e2000.member_2 + 3u)];
                                    let _e2030 = global.member[(_e2000.member_2 + 4u)];
                                    let _e2035 = global.member[(_e2000.member_2 + 5u)];
                                    let _e2040 = global.member[(_e2000.member_2 + 6u)];
                                    let _e2046 = global.member[(_e2000.member_2 + 7u)];
                                    let _e2051 = global.member[(_e2000.member_2 + 8u)];
                                    let _e2056 = global.member[(_e2000.member_2 + 9u)];
                                    phi_2434_ = type_29(vec3<f32>(bitcast<f32>(_e2009), bitcast<f32>(_e2014), bitcast<f32>(_e2019)), vec4<f32>(bitcast<f32>(_e2025), bitcast<f32>(_e2030), bitcast<f32>(_e2035), bitcast<f32>(_e2040)), vec3<f32>(bitcast<f32>(_e2046), bitcast<f32>(_e2051), bitcast<f32>(_e2056)));
                                } else {
                                    phi_2434_ = type_29(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e2061 = phi_2434_;
                                let _e2069 = (_e2061.member_1.x + _e2061.member_1.x);
                                let _e2070 = (_e2061.member_1.y + _e2061.member_1.y);
                                let _e2071 = (_e2061.member_1.z + _e2061.member_1.z);
                                let _e2073 = (_e2061.member_1.z * _e2071);
                                let _e2074 = (_e2061.member_1.w * _e2069);
                                let _e2075 = (_e2061.member_1.w * _e2070);
                                let _e2076 = (_e2061.member_1.w * _e2071);
                                let _e2096 = (vec4<f32>((1f - fma(_e2061.member_1.y, _e2070, _e2073)), fma(_e2061.member_1.x, _e2070, _e2076), fma(_e2061.member_1.x, _e2071, -(_e2075)), 0f) * _e2061.member_2.x);
                                let _e2098 = (vec4<f32>(fma(_e2061.member_1.x, _e2070, -(_e2076)), (1f - fma(_e2061.member_1.x, _e2069, _e2073)), fma(_e2061.member_1.y, _e2071, _e2074), 0f) * _e2061.member_2.y);
                                let _e2100 = (vec4<f32>(fma(_e2061.member_1.x, _e2071, _e2075), fma(_e2061.member_1.y, _e2071, -(_e2074)), (1f - fma(_e2061.member_1.x, _e2069, (_e2061.member_1.y * _e2070))), 0f) * _e2061.member_2.z);
                                switch bitcast<i32>(_e2000.member) {
                                    case 0: {
                                        if _e303 {
                                            phi_6185_ = (_e2000.member_1 <= (_e118 - 8u));
                                        } else {
                                            phi_6185_ = false;
                                        }
                                        let _e2596 = phi_6185_;
                                        if _e2596 {
                                            let _e2599 = global.member[_e2000.member_1];
                                            let _e2604 = global.member[(_e2000.member_1 + 1u)];
                                            let _e2609 = global.member[(_e2000.member_1 + 2u)];
                                            let _e2615 = global.member[(_e2000.member_1 + 3u)];
                                            let _e2620 = global.member[(_e2000.member_1 + 4u)];
                                            let _e2625 = global.member[(_e2000.member_1 + 5u)];
                                            let _e2630 = global.member[(_e2000.member_1 + 6u)];
                                            let _e2636 = global.member[(_e2000.member_1 + 7u)];
                                            phi_2482_ = type_35(vec3<f32>(bitcast<f32>(_e2599), bitcast<f32>(_e2604), bitcast<f32>(_e2609)), vec4<f32>(bitcast<f32>(_e2615), bitcast<f32>(_e2620), bitcast<f32>(_e2625), bitcast<f32>(_e2630)), bitcast<f32>(_e2636));
                                        } else {
                                            phi_2482_ = type_35(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2640 = phi_2482_;
                                        let _e2662 = fma(_e2100.x, _e2640.member.z, fma(_e2098.x, _e2640.member.y, (_e2096.x * _e2640.member.x)));
                                        let _e2663 = fma(_e2100.y, _e2640.member.z, fma(_e2098.y, _e2640.member.y, (_e2096.y * _e2640.member.x)));
                                        let _e2664 = fma(_e2100.z, _e2640.member.z, fma(_e2098.z, _e2640.member.y, (_e2096.z * _e2640.member.x)));
                                        let _e2669 = sqrt(fma(_e2664, _e2664, fma(_e2662, _e2662, (_e2663 * _e2663))));
                                        if (_e2669 == 0f) {
                                            phi_6232_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6232_ = (vec3<f32>(_e2662, _e2663, _e2664) * (1f / _e2669));
                                        }
                                        let _e2674 = phi_6232_;
                                        let _e2676 = -(_e2674.x);
                                        let _e2678 = -(_e2674.y);
                                        let _e2680 = -(_e2674.z);
                                        let _e2681 = -(_e2674);
                                        let _e2683 = fma(-(_e688.z), _e297.member_3, 1f);
                                        let _e2687 = fma(0.4f, _e2683, (_e1372 * _e1384));
                                        let _e2688 = fma(0.4f, _e2683, (_e1374 * _e1384));
                                        let _e2689 = fma(0.4f, _e2683, (_e1376 * _e1384));
                                        let _e2697 = (_e1940 + vec3<f32>(_e2676, _e2678, _e2680));
                                        let _e2704 = sqrt(fma(_e2697.z, _e2697.z, fma(_e2697.x, _e2697.x, (_e2697.y * _e2697.y))));
                                        if (_e2704 == 0f) {
                                            phi_6267_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6267_ = (_e2697 * (1f / _e2704));
                                        }
                                        let _e2709 = phi_6267_;
                                        let _e2710 = (_e1381 * _e1381);
                                        let _e2721 = max(fma(_e1936.z, _e2709.z, fma(_e1936.x, _e2709.x, (_e1936.y * _e2709.y))), 0f);
                                        let _e2734 = max(fma(_e1936.z, _e1940.z, fma(_e1936.x, _e1940.x, (_e1936.y * _e1940.y))), 0f);
                                        let _e2740 = fma(_e1936.z, _e2681.z, fma(_e1936.x, _e2681.x, (_e1936.y * _e2681.y)));
                                        let _e2741 = max(_e2740, 0f);
                                        let _e2742 = fma(_e688.y, _e297.member_4, 1f);
                                        let _e2743 = (_e2742 * _e2742);
                                        let _e2744 = (_e2743 * 0.125f);
                                        let _e2746 = fma(-(_e2743), 0.125f, 1f);
                                        let _e2759 = (1f - max(fma(_e2709.z, _e1940.z, fma(_e2709.x, _e1940.x, (_e2709.y * _e1940.y))), 0f));
                                        let _e2761 = select(_e2759, 0f, (_e2759 < 0f));
                                        let _e2764 = pow(select(_e2761, 1f, (_e2761 > 1f)), 5f);
                                        let _e2765 = fma((1f - _e2687), _e2764, _e2687);
                                        let _e2766 = fma((1f - _e2688), _e2764, _e2688);
                                        let _e2767 = fma((1f - _e2689), _e2764, _e2689);
                                        let _e2774 = (((_e2710 * _e2710) / (pow(fma((_e2721 * _e2721), fma(_e2710, _e2710, -1f), 1f), 2f) * 3.1415927f)) * ((_e2734 / fma(_e2734, _e2746, _e2744)) * (_e2741 / fma(_e2741, _e2746, _e2744))));
                                        let _e2781 = max(fma(_e1936.z, _e2680, fma(_e1936.x, _e2676, (_e1936.y * _e2678))), 0f);
                                        let _e2783 = fma((4f * _e2734), _e2781, 0.0001f);
                                        let _e2801 = global_1.member[0u];
                                        let _e2804 = global_1.member[_e2801];
                                        let _e2809 = global_1.member[(_e2801 + 1u)];
                                        let _e2814 = global_1.member[(_e2801 + 2u)];
                                        let _e2819 = global_1.member[(_e2801 + 3u)];
                                        let _e2824 = global_1.member[(_e2801 + 4u)];
                                        let _e2829 = global_1.member[(_e2801 + 5u)];
                                        let _e2834 = global_1.member[(_e2801 + 6u)];
                                        let _e2839 = global_1.member[(_e2801 + 7u)];
                                        let _e2844 = global_1.member[(_e2801 + 8u)];
                                        let _e2849 = global_1.member[(_e2801 + 9u)];
                                        let _e2854 = global_1.member[(_e2801 + 10u)];
                                        let _e2859 = global_1.member[(_e2801 + 11u)];
                                        let _e2864 = global_1.member[(_e2801 + 12u)];
                                        let _e2869 = global_1.member[(_e2801 + 13u)];
                                        let _e2874 = global_1.member[(_e2801 + 14u)];
                                        let _e2879 = global_1.member[(_e2801 + 15u)];
                                        let _e2899 = (bitcast<f32>(_e2879) + fma(bitcast<f32>(_e2859), _e126.z, fma(bitcast<f32>(_e2839), _e126.y, (bitcast<f32>(_e2819) * _e126.x))));
                                        let _e2907 = textureSampleLevel(global_18, global_19, vec2<f32>(fma(((bitcast<f32>(_e2864) + fma(bitcast<f32>(_e2844), _e126.z, fma(bitcast<f32>(_e2824), _e126.y, (bitcast<f32>(_e2804) * _e126.x)))) / _e2899), 0.5f, 0.5f), fma(((bitcast<f32>(_e2869) + fma(bitcast<f32>(_e2849), _e126.z, fma(bitcast<f32>(_e2829), _e126.y, (bitcast<f32>(_e2809) * _e126.x)))) / _e2899), -0.5f, 0.5f)), i32(0f));
                                        let _e2916 = (1f - select(0f, 1f, ((((bitcast<f32>(_e2874) + fma(bitcast<f32>(_e2854), _e126.z, fma(bitcast<f32>(_e2834), _e126.y, (bitcast<f32>(_e2814) * _e126.x)))) / _e2899) - max((0.5f * (1f - _e2740)), 0.005f)) > vec4(_e2907).x)));
                                        phi_3328_ = vec3<f32>(fma(((fma((((1f - _e2765) * _e2683) * _e1372), 0.31830987f, ((_e2774 * _e2765) / _e2783)) * (_e2640.member_1.x * _e2640.member_2)) * _e2781), _e2916, _e1945.x), fma(((fma((((1f - _e2766) * _e2683) * _e1374), 0.31830987f, ((_e2774 * _e2766) / _e2783)) * (_e2640.member_1.y * _e2640.member_2)) * _e2781), _e2916, _e1945.y), fma(((fma((((1f - _e2767) * _e2683) * _e1376), 0.31830987f, ((_e2774 * _e2767) / _e2783)) * (_e2640.member_1.z * _e2640.member_2)) * _e2781), _e2916, _e1945.z));
                                        phi_3329_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if _e303 {
                                            phi_6011_ = (_e2000.member_1 <= (_e118 - 8u));
                                        } else {
                                            phi_6011_ = false;
                                        }
                                        let _e2385 = phi_6011_;
                                        if _e2385 {
                                            let _e2388 = global.member[_e2000.member_1];
                                            let _e2393 = global.member[(_e2000.member_1 + 1u)];
                                            let _e2398 = global.member[(_e2000.member_1 + 2u)];
                                            let _e2404 = global.member[(_e2000.member_1 + 3u)];
                                            let _e2409 = global.member[(_e2000.member_1 + 4u)];
                                            let _e2414 = global.member[(_e2000.member_1 + 5u)];
                                            let _e2419 = global.member[(_e2000.member_1 + 6u)];
                                            let _e2425 = global.member[(_e2000.member_1 + 7u)];
                                            phi_2818_ = type_35(vec3<f32>(bitcast<f32>(_e2388), bitcast<f32>(_e2393), bitcast<f32>(_e2398)), vec4<f32>(bitcast<f32>(_e2404), bitcast<f32>(_e2409), bitcast<f32>(_e2414), bitcast<f32>(_e2419)), bitcast<f32>(_e2425));
                                        } else {
                                            phi_2818_ = type_35(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2429 = phi_2818_;
                                        let _e2458 = (vec3<f32>((_e2061.member.x + fma(_e2100.x, _e2429.member.z, fma(_e2098.x, _e2429.member.y, (_e2096.x * _e2429.member.x)))), (_e2061.member.y + fma(_e2100.y, _e2429.member.z, fma(_e2098.y, _e2429.member.y, (_e2096.y * _e2429.member.x)))), (_e2061.member.z + fma(_e2100.z, _e2429.member.z, fma(_e2098.z, _e2429.member.y, (_e2096.z * _e2429.member.x))))) - _e126);
                                        let _e2465 = sqrt(fma(_e2458.z, _e2458.z, fma(_e2458.x, _e2458.x, (_e2458.y * _e2458.y))));
                                        let _e2466 = (_e2465 == 0f);
                                        if _e2466 {
                                            phi_3008_ = vec3<f32>();
                                        } else {
                                            if _e2466 {
                                                phi_6058_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6058_ = (_e2458 * (1f / _e2465));
                                            }
                                            let _e2470 = phi_6058_;
                                            let _e2472 = (_e2429.member_2 / (_e2465 * _e2465));
                                            let _e2474 = fma(-(_e688.z), _e297.member_3, 1f);
                                            let _e2478 = fma(0.4f, _e2474, (_e1372 * _e1384));
                                            let _e2479 = fma(0.4f, _e2474, (_e1374 * _e1384));
                                            let _e2480 = fma(0.4f, _e2474, (_e1376 * _e1384));
                                            let _e2487 = (_e1940 + _e2470);
                                            let _e2494 = sqrt(fma(_e2487.z, _e2487.z, fma(_e2487.x, _e2487.x, (_e2487.y * _e2487.y))));
                                            if (_e2494 == 0f) {
                                                phi_6093_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6093_ = (_e2487 * (1f / _e2494));
                                            }
                                            let _e2499 = phi_6093_;
                                            let _e2500 = (_e1381 * _e1381);
                                            let _e2511 = max(fma(_e1936.z, _e2499.z, fma(_e1936.x, _e2499.x, (_e1936.y * _e2499.y))), 0f);
                                            let _e2524 = max(fma(_e1936.z, _e1940.z, fma(_e1936.x, _e1940.x, (_e1936.y * _e1940.y))), 0f);
                                            let _e2531 = max(fma(_e1936.z, _e2470.z, fma(_e1936.x, _e2470.x, (_e1936.y * _e2470.y))), 0f);
                                            let _e2532 = fma(_e688.y, _e297.member_4, 1f);
                                            let _e2533 = (_e2532 * _e2532);
                                            let _e2534 = (_e2533 * 0.125f);
                                            let _e2536 = fma(-(_e2533), 0.125f, 1f);
                                            let _e2549 = (1f - max(fma(_e2499.z, _e1940.z, fma(_e2499.x, _e1940.x, (_e2499.y * _e1940.y))), 0f));
                                            let _e2551 = select(_e2549, 0f, (_e2549 < 0f));
                                            let _e2554 = pow(select(_e2551, 1f, (_e2551 > 1f)), 5f);
                                            let _e2555 = fma((1f - _e2478), _e2554, _e2478);
                                            let _e2556 = fma((1f - _e2479), _e2554, _e2479);
                                            let _e2557 = fma((1f - _e2480), _e2554, _e2480);
                                            let _e2564 = (((_e2500 * _e2500) / (pow(fma((_e2511 * _e2511), fma(_e2500, _e2500, -1f), 1f), 2f) * 3.1415927f)) * ((_e2524 / fma(_e2524, _e2536, _e2534)) * (_e2531 / fma(_e2531, _e2536, _e2534))));
                                            let _e2569 = fma((4f * _e2524), _e2531, 0.0001f);
                                            phi_3008_ = vec3<f32>(fma((fma((((1f - _e2555) * _e2474) * _e1372), 0.31830987f, ((_e2564 * _e2555) / _e2569)) * (_e2429.member_1.x * _e2472)), _e2531, _e1945.x), fma((fma((((1f - _e2556) * _e2474) * _e1374), 0.31830987f, ((_e2564 * _e2556) / _e2569)) * (_e2429.member_1.y * _e2472)), _e2531, _e1945.y), fma((fma((((1f - _e2557) * _e2474) * _e1376), 0.31830987f, ((_e2564 * _e2557) / _e2569)) * (_e2429.member_1.z * _e2472)), _e2531, _e1945.z));
                                        }
                                        let _e2590 = phi_3008_;
                                        phi_3328_ = _e2590;
                                        phi_3329_ = select(true, false, _e2466);
                                        break;
                                    }
                                    case 2: {
                                        if (_e118 >= 13u) {
                                            phi_5799_ = (_e2000.member_1 <= (_e118 - 13u));
                                        } else {
                                            phi_5799_ = false;
                                        }
                                        let _e2111 = phi_5799_;
                                        if _e2111 {
                                            let _e2114 = global.member[_e2000.member_1];
                                            let _e2119 = global.member[(_e2000.member_1 + 1u)];
                                            let _e2124 = global.member[(_e2000.member_1 + 2u)];
                                            let _e2130 = global.member[(_e2000.member_1 + 3u)];
                                            let _e2135 = global.member[(_e2000.member_1 + 4u)];
                                            let _e2140 = global.member[(_e2000.member_1 + 5u)];
                                            let _e2146 = global.member[(_e2000.member_1 + 6u)];
                                            let _e2151 = global.member[(_e2000.member_1 + 7u)];
                                            let _e2156 = global.member[(_e2000.member_1 + 8u)];
                                            let _e2161 = global.member[(_e2000.member_1 + 9u)];
                                            let _e2166 = global.member[(_e2000.member_1 + 10u)];
                                            let _e2171 = global.member[(_e2000.member_1 + 11u)];
                                            let _e2177 = global.member[(_e2000.member_1 + 12u)];
                                            phi_3071_ = type_36(vec3<f32>(bitcast<f32>(_e2114), bitcast<f32>(_e2119), bitcast<f32>(_e2124)), vec3<f32>(bitcast<f32>(_e2130), bitcast<f32>(_e2135), bitcast<f32>(_e2140)), bitcast<f32>(_e2146), bitcast<f32>(_e2151), vec4<f32>(bitcast<f32>(_e2156), bitcast<f32>(_e2161), bitcast<f32>(_e2166), bitcast<f32>(_e2171)), bitcast<f32>(_e2177));
                                        } else {
                                            phi_3071_ = type_36(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2181 = phi_3071_;
                                        let _e2213 = (vec3<f32>((_e2061.member.x + fma(_e2100.x, _e2181.member.z, fma(_e2098.x, _e2181.member.y, (_e2096.x * _e2181.member.x)))), (_e2061.member.y + fma(_e2100.y, _e2181.member.z, fma(_e2098.y, _e2181.member.y, (_e2096.y * _e2181.member.x)))), (_e2061.member.z + fma(_e2100.z, _e2181.member.z, fma(_e2098.z, _e2181.member.y, (_e2096.z * _e2181.member.x))))) - _e126);
                                        let _e2220 = sqrt(fma(_e2213.z, _e2213.z, fma(_e2213.x, _e2213.x, (_e2213.y * _e2213.y))));
                                        let _e2221 = (_e2220 == 0f);
                                        if _e2221 {
                                            phi_3326_ = vec3<f32>();
                                        } else {
                                            if _e2221 {
                                                phi_5849_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5849_ = (_e2213 * (1f / _e2220));
                                            }
                                            let _e2225 = phi_5849_;
                                            let _e2235 = fma(_e2100.x, _e2181.member_1.z, fma(_e2098.x, _e2181.member_1.y, (_e2096.x * _e2181.member_1.x)));
                                            let _e2236 = fma(_e2100.y, _e2181.member_1.z, fma(_e2098.y, _e2181.member_1.y, (_e2096.y * _e2181.member_1.x)));
                                            let _e2237 = fma(_e2100.z, _e2181.member_1.z, fma(_e2098.z, _e2181.member_1.y, (_e2096.z * _e2181.member_1.x)));
                                            let _e2242 = sqrt(fma(_e2237, _e2237, fma(_e2235, _e2235, (_e2236 * _e2236))));
                                            if (_e2242 == 0f) {
                                                phi_5884_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5884_ = (vec3<f32>(_e2235, _e2236, _e2237) * (1f / _e2242));
                                            }
                                            let _e2247 = phi_5884_;
                                            let _e2259 = ((fma(_e2225.z, _e2247.z, fma(_e2225.x, _e2247.x, (_e2225.y * _e2247.y))) - _e2181.member_3) / (_e2181.member_2 - _e2181.member_3));
                                            let _e2261 = select(_e2259, 0f, (_e2259 < 0f));
                                            let _e2264 = (_e2181.member_5 * select(_e2261, 1f, (_e2261 > 1f)));
                                            let _e2266 = fma(-(_e688.z), _e297.member_3, 1f);
                                            let _e2270 = fma(0.4f, _e2266, (_e1372 * _e1384));
                                            let _e2271 = fma(0.4f, _e2266, (_e1374 * _e1384));
                                            let _e2272 = fma(0.4f, _e2266, (_e1376 * _e1384));
                                            let _e2279 = (_e1940 + _e2225);
                                            let _e2286 = sqrt(fma(_e2279.z, _e2279.z, fma(_e2279.x, _e2279.x, (_e2279.y * _e2279.y))));
                                            if (_e2286 == 0f) {
                                                phi_5919_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5919_ = (_e2279 * (1f / _e2286));
                                            }
                                            let _e2291 = phi_5919_;
                                            let _e2292 = (_e1381 * _e1381);
                                            let _e2303 = max(fma(_e1936.z, _e2291.z, fma(_e1936.x, _e2291.x, (_e1936.y * _e2291.y))), 0f);
                                            let _e2316 = max(fma(_e1936.z, _e1940.z, fma(_e1936.x, _e1940.x, (_e1936.y * _e1940.y))), 0f);
                                            let _e2320 = max(fma(_e1936.z, _e2225.z, fma(_e1936.x, _e2225.x, (_e1936.y * _e2225.y))), 0f);
                                            let _e2321 = fma(_e688.y, _e297.member_4, 1f);
                                            let _e2322 = (_e2321 * _e2321);
                                            let _e2323 = (_e2322 * 0.125f);
                                            let _e2325 = fma(-(_e2322), 0.125f, 1f);
                                            let _e2338 = (1f - max(fma(_e2291.z, _e1940.z, fma(_e2291.x, _e1940.x, (_e2291.y * _e1940.y))), 0f));
                                            let _e2340 = select(_e2338, 0f, (_e2338 < 0f));
                                            let _e2343 = pow(select(_e2340, 1f, (_e2340 > 1f)), 5f);
                                            let _e2344 = fma((1f - _e2270), _e2343, _e2270);
                                            let _e2345 = fma((1f - _e2271), _e2343, _e2271);
                                            let _e2346 = fma((1f - _e2272), _e2343, _e2272);
                                            let _e2353 = (((_e2292 * _e2292) / (pow(fma((_e2303 * _e2303), fma(_e2292, _e2292, -1f), 1f), 2f) * 3.1415927f)) * ((_e2316 / fma(_e2316, _e2325, _e2323)) * (_e2320 / fma(_e2320, _e2325, _e2323))));
                                            let _e2358 = fma((4f * _e2316), _e2320, 0.0001f);
                                            phi_3326_ = vec3<f32>(fma((fma((((1f - _e2344) * _e2266) * _e1372), 0.31830987f, ((_e2353 * _e2344) / _e2358)) * (_e2181.member_4.x * _e2264)), _e2320, _e1945.x), fma((fma((((1f - _e2345) * _e2266) * _e1374), 0.31830987f, ((_e2353 * _e2345) / _e2358)) * (_e2181.member_4.y * _e2264)), _e2320, _e1945.y), fma((fma((((1f - _e2346) * _e2266) * _e1376), 0.31830987f, ((_e2353 * _e2346) / _e2358)) * (_e2181.member_4.z * _e2264)), _e2320, _e1945.z));
                                        }
                                        let _e2379 = phi_3326_;
                                        phi_3328_ = _e2379;
                                        phi_3329_ = select(true, false, _e2221);
                                        break;
                                    }
                                    default: {
                                        phi_3328_ = vec3<f32>();
                                        phi_3329_ = bool();
                                        break;
                                    }
                                }
                                let _e2925 = phi_3328_;
                                let _e2927 = phi_3329_;
                                phi_3338_ = select(_e2925, _e1945, vec3(select(true, false, _e2927)));
                            }
                            let _e2932 = phi_3338_;
                            phi_2318_ = _e2932;
                            phi_3340_ = select(true, false, _e1978);
                            break;
                        }
                        default: {
                            phi_2318_ = vec3<f32>();
                            phi_3340_ = bool();
                            break;
                        }
                    }
                    let _e2935 = phi_2318_;
                    let _e2937 = phi_3340_;
                    continue;
                    continuing {
                        phi_2314_ = _e1958;
                        phi_2317_ = _e2935;
                        break if !(_e2937);
                    }
                }
                let _e2940 = fma(-(_e688.z), _e297.member_3, 1f);
                let _e2944 = fma(0.04f, _e2940, (_e1372 * _e1384));
                let _e2945 = fma(0.04f, _e2940, (_e1374 * _e1384));
                let _e2946 = fma(0.04f, _e2940, (_e1376 * _e1384));
                let _e2958 = fma(-(_e688.y), _e297.member_4, 1f);
                let _e2965 = (1f - max(fma(_e1936.z, _e1940.z, fma(_e1936.x, _e1940.x, (_e1936.y * _e1940.y))), 0f));
                let _e2967 = select(_e2965, 0f, (_e2965 < 0f));
                let _e2970 = pow(select(_e2967, 1f, (_e2967 > 1f)), 5f);
                let _e2971 = fma((max(_e2958, _e2944) - _e2944), _e2970, _e2944);
                let _e2972 = fma((max(_e2958, _e2945) - _e2945), _e2970, _e2945);
                let _e2973 = fma((max(_e2958, _e2946) - _e2946), _e2970, _e2946);
                let _e2993 = local_2;
                let _e2997 = local_3;
                let _e3001 = local_4;
                phi_3457_ = vec4<f32>(fma(_e1394, _e297.member_1, fma(fma(((1f - _e2971) * _e2940), (_e1403.x * _e1372), (_e1751.x * fma(_e2971, _e1767.x, _e1767.y))), _e1388, _e2993.x)), fma(_e1396, _e297.member_1, fma(fma(((1f - _e2972) * _e2940), (_e1403.y * _e1374), (_e1751.y * fma(_e2972, _e1767.x, _e1767.y))), _e1388, _e2997.y)), fma(_e1398, _e297.member_1, fma(fma(((1f - _e2973) * _e2940), (_e1403.z * _e1376), (_e1751.z * fma(_e2973, _e1767.x, _e1767.y))), _e1388, _e3001.z)), 1f);
            } else {
                phi_3457_ = (vec4<f32>((_e120.x * _e494.x), (_e120.y * _e494.y), (_e120.z * _e494.z), (_e120.w * _e494.w)) * _e297.member_2);
            }
            let _e3009 = phi_3457_;
            global_20 = _e3009;
            break;
        }
        case 1: {
            let _e1909 = sqrt(fma(_e121.x, _e121.x, (_e121.y * _e121.y)));
            if (_e1909 == 0f) {
                phi_5547_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5547_ = (vec3<f32>(_e121.x, _e121.y, 0f) * (1f / _e1909));
            }
            let _e1914 = phi_5547_;
            global_20 = vec4<f32>(((_e1914.x + 1f) * 0.5f), ((_e1914.y + 1f) * 0.5f), ((_e1914.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1888 = sqrt(fma(_e122.x, _e122.x, (_e122.y * _e122.y)));
            if (_e1888 == 0f) {
                phi_5498_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5498_ = (vec3<f32>(_e122.x, _e122.y, 0f) * (1f / _e1888));
            }
            let _e1893 = phi_5498_;
            global_20 = vec4<f32>(((_e1893.x + 1f) * 0.5f), ((_e1893.y + 1f) * 0.5f), ((_e1893.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1730 {
                phi_5449_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5449_ = (_e1357 * (1f / _e1729));
            }
            let _e1872 = phi_5449_;
            global_20 = vec4<f32>(((_e1872.x + 1f) * 0.5f), ((_e1872.y + 1f) * 0.5f), ((_e1872.z + 1f) * 0.5f), 1f);
            break;
        }
        case 4: {
            global_20 = _e120;
            break;
        }
        case 5: {
            let _e1853 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
            if (_e1853 == 0f) {
                phi_5400_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5400_ = (_e123 * (1f / _e1853));
            }
            let _e1858 = phi_5400_;
            global_20 = vec4<f32>(((_e1858.x + 1f) * 0.5f), ((_e1858.y + 1f) * 0.5f), ((_e1858.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1831 = sqrt(fma(_e1355.z, _e1355.z, fma(_e1355.x, _e1355.x, (_e1355.y * _e1355.y))));
            if (_e1831 == 0f) {
                phi_5351_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5351_ = (_e1355 * (1f / _e1831));
            }
            let _e1836 = phi_5351_;
            global_20 = vec4<f32>(((_e1836.x + 1f) * 0.5f), ((_e1836.y + 1f) * 0.5f), ((_e1836.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1809 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
            if (_e1809 == 0f) {
                phi_5302_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5302_ = (_e124 * (1f / _e1809));
            }
            let _e1814 = phi_5302_;
            global_20 = vec4<f32>(((_e1814.x + 1f) * 0.5f), ((_e1814.y + 1f) * 0.5f), ((_e1814.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1787 = sqrt(fma(_e125.z, _e125.z, fma(_e125.x, _e125.x, (_e125.y * _e125.y))));
            if (_e1787 == 0f) {
                phi_5253_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5253_ = (_e125 * (1f / _e1787));
            }
            let _e1792 = phi_5253_;
            global_20 = vec4<f32>(((_e1792.x + 1f) * 0.5f), ((_e1792.y + 1f) * 0.5f), ((_e1792.z + 1f) * 0.5f), 1f);
            break;
        }
        case 9: {
            global_20 = vec4<f32>(_e1403.x, _e1403.y, _e1403.z, 1f);
            break;
        }
        case 10: {
            global_20 = vec4<f32>(_e1751.x, _e1751.y, _e1751.z, 1f);
            break;
        }
        case 11: {
            global_20 = vec4<f32>(_e1767.x, _e1767.y, 1f, 1f);
            break;
        }
        case 12: {
            global_20 = (vec4<f32>(_e1361, _e1364, _e1367, (_e494.w * _e297.member_2.w)) * _e120);
            break;
        }
        case 13: {
            global_20 = vec4<f32>(_e1381, _e1381, _e1381, 1f);
            break;
        }
        case 14: {
            global_20 = vec4<f32>(_e1384, _e1384, _e1384, 1f);
            break;
        }
        case 15: {
            global_20 = vec4<f32>(_e1388, _e1388, _e1388, 1f);
            break;
        }
        case 16: {
            global_20 = vec4<f32>((_e1394 * _e297.member_1), (_e1396 * _e297.member_1), (_e1398 * _e297.member_1), 1f);
            break;
        }
        case 17: {
            global_20 = vec4<f32>(_e1270.x, _e1270.y, _e1270.z, 1f);
            break;
        }
        case 18: {
            global_20 = vec4<f32>(_e297.member.x, _e297.member.y, _e297.member.z, 1f);
            break;
        }
        case 19: {
            global_20 = vec4<f32>(_e297.member_1, _e297.member_1, _e297.member_1, 1f);
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
