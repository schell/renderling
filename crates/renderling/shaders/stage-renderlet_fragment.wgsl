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
    var phi_749_: u32;
    var phi_4061_: bool;
    var phi_903_: type_32;
    var phi_907_: type_32;
    var phi_4098_: bool;
    var phi_947_: u32;
    var phi_956_: u32;
    var phi_969_: type_33;
    var phi_4120_: f32;
    var phi_4133_: bool;
    var phi_1023_: f32;
    var phi_1018_: f32;
    var phi_1024_: f32;
    var phi_4150_: bool;
    var phi_989_: f32;
    var phi_1026_: f32;
    var phi_4168_: f32;
    var phi_4181_: bool;
    var phi_1081_: f32;
    var phi_1076_: f32;
    var phi_1082_: f32;
    var phi_4198_: bool;
    var phi_1047_: f32;
    var phi_1084_: f32;
    var phi_4234_: bool;
    var phi_1167_: u32;
    var phi_1176_: u32;
    var phi_1189_: type_33;
    var phi_4255_: f32;
    var phi_4268_: bool;
    var phi_1243_: f32;
    var phi_1238_: f32;
    var phi_1244_: f32;
    var phi_4285_: bool;
    var phi_1209_: f32;
    var phi_1246_: f32;
    var phi_4303_: f32;
    var phi_4316_: bool;
    var phi_1301_: f32;
    var phi_1296_: f32;
    var phi_1302_: f32;
    var phi_4333_: bool;
    var phi_1267_: f32;
    var phi_1304_: f32;
    var phi_4369_: bool;
    var phi_1387_: u32;
    var phi_1396_: u32;
    var phi_1409_: type_33;
    var phi_4390_: f32;
    var phi_4403_: bool;
    var phi_1463_: f32;
    var phi_1458_: f32;
    var phi_1464_: f32;
    var phi_4420_: bool;
    var phi_1429_: f32;
    var phi_1466_: f32;
    var phi_4438_: f32;
    var phi_4451_: bool;
    var phi_1521_: f32;
    var phi_1516_: f32;
    var phi_1522_: f32;
    var phi_4468_: bool;
    var phi_1487_: f32;
    var phi_1524_: f32;
    var phi_4504_: bool;
    var phi_1607_: u32;
    var phi_1616_: u32;
    var phi_1629_: type_33;
    var phi_4525_: f32;
    var phi_4538_: bool;
    var phi_1683_: f32;
    var phi_1678_: f32;
    var phi_1684_: f32;
    var phi_4555_: bool;
    var phi_1649_: f32;
    var phi_1686_: f32;
    var phi_4573_: f32;
    var phi_4586_: bool;
    var phi_1741_: f32;
    var phi_1736_: f32;
    var phi_1742_: f32;
    var phi_4603_: bool;
    var phi_1707_: f32;
    var phi_1744_: f32;
    var phi_4639_: bool;
    var phi_1827_: u32;
    var phi_1836_: u32;
    var phi_1849_: type_33;
    var phi_4660_: f32;
    var phi_4673_: bool;
    var phi_1903_: f32;
    var phi_1898_: f32;
    var phi_1904_: f32;
    var phi_4690_: bool;
    var phi_1869_: f32;
    var phi_1906_: f32;
    var phi_4708_: f32;
    var phi_4721_: bool;
    var phi_1961_: f32;
    var phi_1956_: f32;
    var phi_1962_: f32;
    var phi_4738_: bool;
    var phi_1927_: f32;
    var phi_1964_: f32;
    var phi_4796_: vec3<f32>;
    var phi_4831_: vec3<f32>;
    var phi_4866_: vec3<f32>;
    var phi_4901_: vec3<f32>;
    var phi_4936_: vec3<f32>;
    var phi_2058_: vec3<f32>;
    var phi_2059_: vec3<f32>;
    var phi_4968_: bool;
    var phi_2266_: type_24;
    var phi_2267_: type_24;
    var phi_2290_: type_24;
    var phi_2317_: bool;
    var phi_2323_: type_24;
    var phi_2324_: type_24;
    var phi_2347_: type_24;
    var phi_2370_: bool;
    var phi_2391_: type_22;
    var phi_5040_: vec3<f32>;
    var phi_5099_: vec3<f32>;
    var phi_5173_: vec3<f32>;
    var phi_5233_: vec3<f32>;
    var phi_5282_: vec3<f32>;
    var phi_5331_: vec3<f32>;
    var phi_5380_: vec3<f32>;
    var phi_5429_: vec3<f32>;
    var phi_5478_: vec3<f32>;
    var phi_5527_: vec3<f32>;
    var phi_5566_: vec3<f32>;
    var phi_5601_: vec3<f32>;
    var phi_2431_: type_24;
    var phi_2434_: vec3<f32>;
    var phi_2432_: type_24;
    var phi_2457_: type_24;
    var phi_5618_: u32;
    var phi_5637_: bool;
    var phi_2474_: u32;
    var phi_5669_: bool;
    var phi_2491_: u32;
    var phi_2501_: type_34;
    var phi_5699_: bool;
    var phi_2551_: type_29;
    var phi_5779_: bool;
    var phi_3059_: type_36;
    var phi_5829_: vec3<f32>;
    var phi_5864_: vec3<f32>;
    var phi_5899_: vec3<f32>;
    var phi_3314_: vec3<f32>;
    var phi_5991_: bool;
    var phi_2806_: type_35;
    var phi_6038_: vec3<f32>;
    var phi_6073_: vec3<f32>;
    var phi_2996_: vec3<f32>;
    var phi_6165_: bool;
    var phi_2599_: type_35;
    var phi_6212_: vec3<f32>;
    var phi_6247_: vec3<f32>;
    var phi_3316_: vec3<f32>;
    var phi_3317_: bool;
    var phi_3326_: vec3<f32>;
    var phi_2435_: vec3<f32>;
    var phi_3328_: bool;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var phi_3450_: vec4<f32>;

    let _e117 = arrayLength((&global.member));
    let _e118 = global_2;
    let _e119 = global_3;
    let _e120 = global_4;
    let _e121 = global_5;
    let _e122 = global_6;
    let _e123 = global_7;
    let _e124 = global_8;
    let _e125 = global_9;
    let _e128 = global_1.member[0u];
    let _e131 = global_1.member[_e128];
    let _e136 = global_1.member[(_e128 + 1u)];
    let _e141 = global_1.member[(_e128 + 2u)];
    let _e146 = global_1.member[(_e128 + 3u)];
    let _e151 = global_1.member[(_e128 + 4u)];
    let _e156 = global_1.member[(_e128 + 5u)];
    let _e161 = global_1.member[(_e128 + 6u)];
    let _e166 = global_1.member[(_e128 + 7u)];
    let _e171 = global_1.member[(_e128 + 8u)];
    let _e176 = global_1.member[(_e128 + 9u)];
    let _e181 = global_1.member[(_e128 + 10u)];
    let _e186 = global_1.member[(_e128 + 11u)];
    let _e191 = global_1.member[(_e128 + 12u)];
    let _e196 = global_1.member[(_e128 + 13u)];
    let _e201 = global_1.member[(_e128 + 14u)];
    let _e206 = global_1.member[(_e128 + 15u)];
    let _e226 = (bitcast<f32>(_e206) + fma(bitcast<f32>(_e186), _e125.z, fma(bitcast<f32>(_e166), _e125.y, (bitcast<f32>(_e146) * _e125.x))));
    let _e233 = global.member[(_e118 + 9u)];
    let _e237 = global.member[(_e118 + 11u)];
    let _e241 = global.member[(_e118 + 17u)];
    let _e244 = global.member[_e241];
    let _e248 = global.member[(_e241 + 1u)];
    let _e252 = global.member[(_e241 + 4u)];
    switch bitcast<i32>(_e252) {
        case 0: {
            phi_749_ = 0u;
            break;
        }
        case 1: {
            phi_749_ = 1u;
            break;
        }
        case 2: {
            phi_749_ = 2u;
            break;
        }
        case 3: {
            phi_749_ = 3u;
            break;
        }
        case 4: {
            phi_749_ = 4u;
            break;
        }
        case 5: {
            phi_749_ = 5u;
            break;
        }
        case 6: {
            phi_749_ = 6u;
            break;
        }
        case 7: {
            phi_749_ = 7u;
            break;
        }
        case 8: {
            phi_749_ = 8u;
            break;
        }
        case 9: {
            phi_749_ = 9u;
            break;
        }
        case 10: {
            phi_749_ = 10u;
            break;
        }
        case 11: {
            phi_749_ = 11u;
            break;
        }
        case 12: {
            phi_749_ = 12u;
            break;
        }
        case 13: {
            phi_749_ = 13u;
            break;
        }
        case 14: {
            phi_749_ = 14u;
            break;
        }
        case 15: {
            phi_749_ = 15u;
            break;
        }
        case 16: {
            phi_749_ = 16u;
            break;
        }
        case 17: {
            phi_749_ = 17u;
            break;
        }
        case 18: {
            phi_749_ = 18u;
            break;
        }
        case 19: {
            phi_749_ = 19u;
            break;
        }
        default: {
            phi_749_ = 0u;
            break;
        }
    }
    let _e255 = phi_749_;
    let _e259 = global.member[(_e241 + 5u)];
    let _e264 = global.member[(_e241 + 9u)];
    let _e268 = global.member[(_e241 + 10u)];
    if (_e237 == 4294967295u) {
        phi_907_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e117 >= 22u) {
            phi_4061_ = (_e237 <= (_e117 - 22u));
        } else {
            phi_4061_ = false;
        }
        let _e274 = phi_4061_;
        if _e274 {
            let _e277 = global.member[_e237];
            let _e282 = global.member[(_e237 + 1u)];
            let _e287 = global.member[(_e237 + 2u)];
            let _e293 = global.member[(_e237 + 3u)];
            let _e298 = global.member[(_e237 + 4u)];
            let _e303 = global.member[(_e237 + 5u)];
            let _e308 = global.member[(_e237 + 6u)];
            let _e313 = global.member[(_e237 + 7u)];
            let _e319 = global.member[(_e237 + 8u)];
            let _e324 = global.member[(_e237 + 9u)];
            let _e329 = global.member[(_e237 + 10u)];
            let _e333 = global.member[(_e237 + 11u)];
            let _e337 = global.member[(_e237 + 12u)];
            let _e341 = global.member[(_e237 + 13u)];
            let _e345 = global.member[(_e237 + 14u)];
            let _e349 = global.member[(_e237 + 15u)];
            let _e353 = global.member[(_e237 + 16u)];
            let _e357 = global.member[(_e237 + 17u)];
            let _e361 = global.member[(_e237 + 18u)];
            let _e365 = global.member[(_e237 + 19u)];
            let _e369 = global.member[(_e237 + 20u)];
            let _e374 = global.member[(_e237 + 21u)];
            phi_903_ = type_32(vec3<f32>(bitcast<f32>(_e277), bitcast<f32>(_e282), bitcast<f32>(_e287)), bitcast<f32>(_e293), vec4<f32>(bitcast<f32>(_e298), bitcast<f32>(_e303), bitcast<f32>(_e308), bitcast<f32>(_e313)), bitcast<f32>(_e319), bitcast<f32>(_e324), _e329, _e333, _e337, _e341, _e345, _e349, _e353, _e357, _e361, _e365, (_e369 == 1u), bitcast<f32>(_e374));
        } else {
            phi_903_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e378 = phi_903_;
        phi_907_ = type_32(_e378.member, _e378.member_1, _e378.member_2, _e378.member_3, _e378.member_4, _e378.member_5, _e378.member_6, _e378.member_7, _e378.member_8, _e378.member_9, _e378.member_10, _e378.member_11, _e378.member_12, _e378.member_13, _e378.member_14, (_e378.member_15 && (_e259 == 1u)), _e378.member_16);
    }
    let _e400 = phi_907_;
    let _e404 = select(_e121, _e120, vec2((_e400.member_10 == 0u)));
    let _e406 = (_e117 >= 8u);
    if _e406 {
        phi_4098_ = (_e400.member_5 <= (_e117 - 8u));
    } else {
        phi_4098_ = false;
    }
    let _e410 = phi_4098_;
    if _e410 {
        let _e413 = global.member[_e400.member_5];
        let _e417 = global.member[(_e400.member_5 + 1u)];
        let _e422 = global.member[(_e400.member_5 + 2u)];
        let _e426 = global.member[(_e400.member_5 + 3u)];
        let _e431 = global.member[(_e400.member_5 + 4u)];
        let _e435 = global.member[(_e400.member_5 + 5u)];
        let _e439 = global.member[(_e400.member_5 + 6u)];
        switch bitcast<i32>(_e439) {
            case 0: {
                phi_947_ = 0u;
                break;
            }
            case 1: {
                phi_947_ = 1u;
                break;
            }
            case 2: {
                phi_947_ = 2u;
                break;
            }
            default: {
                phi_947_ = 0u;
                break;
            }
        }
        let _e442 = phi_947_;
        let _e446 = global.member[(_e400.member_5 + 7u)];
        switch bitcast<i32>(_e446) {
            case 0: {
                phi_956_ = 0u;
                break;
            }
            case 1: {
                phi_956_ = 1u;
                break;
            }
            case 2: {
                phi_956_ = 2u;
                break;
            }
            default: {
                phi_956_ = 0u;
                break;
            }
        }
        let _e449 = phi_956_;
        phi_969_ = type_33(type_24(_e442, _e449), vec2<u32>(_e413, _e417), vec2<u32>(_e422, _e426), _e431, _e435);
    } else {
        phi_969_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e453 = phi_969_;
    switch bitcast<i32>(_e453.member.member) {
        case 1: {
            let _e491 = abs(_e404.x);
            let _e493 = (_e491 % 1f);
            if (_e491 >= 1f) {
                phi_4150_ = select(true, false, (_e493 == 0f));
            } else {
                phi_4150_ = true;
            }
            let _e497 = phi_4150_;
            let _e498 = select(1f, _e493, _e497);
            if (select(-1f, 1f, (_e404.x >= 0f)) > 0f) {
                phi_989_ = _e498;
            } else {
                phi_989_ = (1f - _e498);
            }
            let _e502 = phi_989_;
            phi_1026_ = _e502;
            break;
        }
        case 2: {
            let _e465 = abs(_e404.x);
            let _e472 = ((select(select(u32(_e465), 0u, (_e465 < 0f)), 4294967295u, (_e465 > 4294967000f)) % 2u) == 0u);
            let _e474 = (_e465 % 1f);
            if (_e465 >= 1f) {
                phi_4133_ = select(true, false, (_e474 == 0f));
            } else {
                phi_4133_ = true;
            }
            let _e478 = phi_4133_;
            let _e479 = select(1f, _e474, _e478);
            if (select(-1f, 1f, (_e404.x >= 0f)) > 0f) {
                if _e472 {
                    phi_1018_ = _e479;
                } else {
                    phi_1018_ = (1f - _e479);
                }
                let _e486 = phi_1018_;
                phi_1024_ = _e486;
            } else {
                if _e472 {
                    phi_1023_ = (1f - _e479);
                } else {
                    phi_1023_ = _e479;
                }
                let _e483 = phi_1023_;
                phi_1024_ = _e483;
            }
            let _e488 = phi_1024_;
            phi_1026_ = _e488;
            break;
        }
        case 0: {
            if (_e404.x > 1f) {
                phi_4120_ = 0.9999999f;
            } else {
                phi_4120_ = select(_e404.x, 0.00000011920929f, (_e404.x < 0f));
            }
            let _e462 = phi_4120_;
            phi_1026_ = _e462;
            break;
        }
        default: {
            phi_1026_ = f32();
            break;
        }
    }
    let _e504 = phi_1026_;
    switch bitcast<i32>(_e453.member.member_1) {
        case 1: {
            let _e542 = abs(_e404.y);
            let _e544 = (_e542 % 1f);
            if (_e542 >= 1f) {
                phi_4198_ = select(true, false, (_e544 == 0f));
            } else {
                phi_4198_ = true;
            }
            let _e548 = phi_4198_;
            let _e549 = select(1f, _e544, _e548);
            if (select(-1f, 1f, (_e404.y >= 0f)) > 0f) {
                phi_1047_ = _e549;
            } else {
                phi_1047_ = (1f - _e549);
            }
            let _e553 = phi_1047_;
            phi_1084_ = _e553;
            break;
        }
        case 2: {
            let _e516 = abs(_e404.y);
            let _e523 = ((select(select(u32(_e516), 0u, (_e516 < 0f)), 4294967295u, (_e516 > 4294967000f)) % 2u) == 0u);
            let _e525 = (_e516 % 1f);
            if (_e516 >= 1f) {
                phi_4181_ = select(true, false, (_e525 == 0f));
            } else {
                phi_4181_ = true;
            }
            let _e529 = phi_4181_;
            let _e530 = select(1f, _e525, _e529);
            if (select(-1f, 1f, (_e404.y >= 0f)) > 0f) {
                if _e523 {
                    phi_1076_ = _e530;
                } else {
                    phi_1076_ = (1f - _e530);
                }
                let _e537 = phi_1076_;
                phi_1082_ = _e537;
            } else {
                if _e523 {
                    phi_1081_ = (1f - _e530);
                } else {
                    phi_1081_ = _e530;
                }
                let _e534 = phi_1081_;
                phi_1082_ = _e534;
            }
            let _e539 = phi_1082_;
            phi_1084_ = _e539;
            break;
        }
        case 0: {
            if (_e404.y > 1f) {
                phi_4168_ = 0.9999999f;
            } else {
                phi_4168_ = select(_e404.y, 0.00000011920929f, (_e404.y < 0f));
            }
            let _e513 = phi_4168_;
            phi_1084_ = _e513;
            break;
        }
        default: {
            phi_1084_ = f32();
            break;
        }
    }
    let _e555 = phi_1084_;
    let _e559 = (_e504 * f32(_e453.member_2.x));
    let _e568 = (_e555 * f32(_e453.member_2.y));
    let _e580 = f32(_e244);
    let _e581 = f32(_e248);
    let _e588 = vec3<f32>((f32((select(select(u32(_e559), 0u, (_e559 < 0f)), 4294967295u, (_e559 > 4294967000f)) + _e453.member_1.x)) / _e580), (f32((select(select(u32(_e568), 0u, (_e568 < 0f)), 4294967295u, (_e568 > 4294967000f)) + _e453.member_1.y)) / _e581), f32(_e453.member_3));
    let _e594 = textureSampleLevel(global_11, global_10, vec2<f32>(_e588.x, _e588.y), i32(_e588.z), 0f);
    let _e597 = select(_e594, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e400.member_5 == 4294967295u)));
    let _e601 = select(_e121, _e120, vec2((_e400.member_11 == 0u)));
    if _e406 {
        phi_4234_ = (_e400.member_6 <= (_e117 - 8u));
    } else {
        phi_4234_ = false;
    }
    let _e606 = phi_4234_;
    if _e606 {
        let _e609 = global.member[_e400.member_6];
        let _e613 = global.member[(_e400.member_6 + 1u)];
        let _e618 = global.member[(_e400.member_6 + 2u)];
        let _e622 = global.member[(_e400.member_6 + 3u)];
        let _e627 = global.member[(_e400.member_6 + 4u)];
        let _e631 = global.member[(_e400.member_6 + 5u)];
        let _e635 = global.member[(_e400.member_6 + 6u)];
        switch bitcast<i32>(_e635) {
            case 0: {
                phi_1167_ = 0u;
                break;
            }
            case 1: {
                phi_1167_ = 1u;
                break;
            }
            case 2: {
                phi_1167_ = 2u;
                break;
            }
            default: {
                phi_1167_ = 0u;
                break;
            }
        }
        let _e638 = phi_1167_;
        let _e642 = global.member[(_e400.member_6 + 7u)];
        switch bitcast<i32>(_e642) {
            case 0: {
                phi_1176_ = 0u;
                break;
            }
            case 1: {
                phi_1176_ = 1u;
                break;
            }
            case 2: {
                phi_1176_ = 2u;
                break;
            }
            default: {
                phi_1176_ = 0u;
                break;
            }
        }
        let _e645 = phi_1176_;
        phi_1189_ = type_33(type_24(_e638, _e645), vec2<u32>(_e609, _e613), vec2<u32>(_e618, _e622), _e627, _e631);
    } else {
        phi_1189_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e649 = phi_1189_;
    switch bitcast<i32>(_e649.member.member) {
        case 1: {
            let _e687 = abs(_e601.x);
            let _e689 = (_e687 % 1f);
            if (_e687 >= 1f) {
                phi_4285_ = select(true, false, (_e689 == 0f));
            } else {
                phi_4285_ = true;
            }
            let _e693 = phi_4285_;
            let _e694 = select(1f, _e689, _e693);
            if (select(-1f, 1f, (_e601.x >= 0f)) > 0f) {
                phi_1209_ = _e694;
            } else {
                phi_1209_ = (1f - _e694);
            }
            let _e698 = phi_1209_;
            phi_1246_ = _e698;
            break;
        }
        case 2: {
            let _e661 = abs(_e601.x);
            let _e668 = ((select(select(u32(_e661), 0u, (_e661 < 0f)), 4294967295u, (_e661 > 4294967000f)) % 2u) == 0u);
            let _e670 = (_e661 % 1f);
            if (_e661 >= 1f) {
                phi_4268_ = select(true, false, (_e670 == 0f));
            } else {
                phi_4268_ = true;
            }
            let _e674 = phi_4268_;
            let _e675 = select(1f, _e670, _e674);
            if (select(-1f, 1f, (_e601.x >= 0f)) > 0f) {
                if _e668 {
                    phi_1238_ = _e675;
                } else {
                    phi_1238_ = (1f - _e675);
                }
                let _e682 = phi_1238_;
                phi_1244_ = _e682;
            } else {
                if _e668 {
                    phi_1243_ = (1f - _e675);
                } else {
                    phi_1243_ = _e675;
                }
                let _e679 = phi_1243_;
                phi_1244_ = _e679;
            }
            let _e684 = phi_1244_;
            phi_1246_ = _e684;
            break;
        }
        case 0: {
            if (_e601.x > 1f) {
                phi_4255_ = 0.9999999f;
            } else {
                phi_4255_ = select(_e601.x, 0.00000011920929f, (_e601.x < 0f));
            }
            let _e658 = phi_4255_;
            phi_1246_ = _e658;
            break;
        }
        default: {
            phi_1246_ = f32();
            break;
        }
    }
    let _e700 = phi_1246_;
    switch bitcast<i32>(_e649.member.member_1) {
        case 1: {
            let _e738 = abs(_e601.y);
            let _e740 = (_e738 % 1f);
            if (_e738 >= 1f) {
                phi_4333_ = select(true, false, (_e740 == 0f));
            } else {
                phi_4333_ = true;
            }
            let _e744 = phi_4333_;
            let _e745 = select(1f, _e740, _e744);
            if (select(-1f, 1f, (_e601.y >= 0f)) > 0f) {
                phi_1267_ = _e745;
            } else {
                phi_1267_ = (1f - _e745);
            }
            let _e749 = phi_1267_;
            phi_1304_ = _e749;
            break;
        }
        case 2: {
            let _e712 = abs(_e601.y);
            let _e719 = ((select(select(u32(_e712), 0u, (_e712 < 0f)), 4294967295u, (_e712 > 4294967000f)) % 2u) == 0u);
            let _e721 = (_e712 % 1f);
            if (_e712 >= 1f) {
                phi_4316_ = select(true, false, (_e721 == 0f));
            } else {
                phi_4316_ = true;
            }
            let _e725 = phi_4316_;
            let _e726 = select(1f, _e721, _e725);
            if (select(-1f, 1f, (_e601.y >= 0f)) > 0f) {
                if _e719 {
                    phi_1296_ = _e726;
                } else {
                    phi_1296_ = (1f - _e726);
                }
                let _e733 = phi_1296_;
                phi_1302_ = _e733;
            } else {
                if _e719 {
                    phi_1301_ = (1f - _e726);
                } else {
                    phi_1301_ = _e726;
                }
                let _e730 = phi_1301_;
                phi_1302_ = _e730;
            }
            let _e735 = phi_1302_;
            phi_1304_ = _e735;
            break;
        }
        case 0: {
            if (_e601.y > 1f) {
                phi_4303_ = 0.9999999f;
            } else {
                phi_4303_ = select(_e601.y, 0.00000011920929f, (_e601.y < 0f));
            }
            let _e709 = phi_4303_;
            phi_1304_ = _e709;
            break;
        }
        default: {
            phi_1304_ = f32();
            break;
        }
    }
    let _e751 = phi_1304_;
    let _e755 = (_e700 * f32(_e649.member_2.x));
    let _e764 = (_e751 * f32(_e649.member_2.y));
    let _e782 = vec3<f32>((f32((select(select(u32(_e755), 0u, (_e755 < 0f)), 4294967295u, (_e755 > 4294967000f)) + _e649.member_1.x)) / _e580), (f32((select(select(u32(_e764), 0u, (_e764 < 0f)), 4294967295u, (_e764 > 4294967000f)) + _e649.member_1.y)) / _e581), f32(_e649.member_3));
    let _e788 = textureSampleLevel(global_11, global_10, vec2<f32>(_e782.x, _e782.y), i32(_e782.z), 0f);
    let _e791 = select(_e788, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e400.member_6 == 4294967295u)));
    let _e795 = select(_e121, _e120, vec2((_e400.member_12 == 0u)));
    if _e406 {
        phi_4369_ = (_e400.member_7 <= (_e117 - 8u));
    } else {
        phi_4369_ = false;
    }
    let _e800 = phi_4369_;
    if _e800 {
        let _e803 = global.member[_e400.member_7];
        let _e807 = global.member[(_e400.member_7 + 1u)];
        let _e812 = global.member[(_e400.member_7 + 2u)];
        let _e816 = global.member[(_e400.member_7 + 3u)];
        let _e821 = global.member[(_e400.member_7 + 4u)];
        let _e825 = global.member[(_e400.member_7 + 5u)];
        let _e829 = global.member[(_e400.member_7 + 6u)];
        switch bitcast<i32>(_e829) {
            case 0: {
                phi_1387_ = 0u;
                break;
            }
            case 1: {
                phi_1387_ = 1u;
                break;
            }
            case 2: {
                phi_1387_ = 2u;
                break;
            }
            default: {
                phi_1387_ = 0u;
                break;
            }
        }
        let _e832 = phi_1387_;
        let _e836 = global.member[(_e400.member_7 + 7u)];
        switch bitcast<i32>(_e836) {
            case 0: {
                phi_1396_ = 0u;
                break;
            }
            case 1: {
                phi_1396_ = 1u;
                break;
            }
            case 2: {
                phi_1396_ = 2u;
                break;
            }
            default: {
                phi_1396_ = 0u;
                break;
            }
        }
        let _e839 = phi_1396_;
        phi_1409_ = type_33(type_24(_e832, _e839), vec2<u32>(_e803, _e807), vec2<u32>(_e812, _e816), _e821, _e825);
    } else {
        phi_1409_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e843 = phi_1409_;
    switch bitcast<i32>(_e843.member.member) {
        case 1: {
            let _e881 = abs(_e795.x);
            let _e883 = (_e881 % 1f);
            if (_e881 >= 1f) {
                phi_4420_ = select(true, false, (_e883 == 0f));
            } else {
                phi_4420_ = true;
            }
            let _e887 = phi_4420_;
            let _e888 = select(1f, _e883, _e887);
            if (select(-1f, 1f, (_e795.x >= 0f)) > 0f) {
                phi_1429_ = _e888;
            } else {
                phi_1429_ = (1f - _e888);
            }
            let _e892 = phi_1429_;
            phi_1466_ = _e892;
            break;
        }
        case 2: {
            let _e855 = abs(_e795.x);
            let _e862 = ((select(select(u32(_e855), 0u, (_e855 < 0f)), 4294967295u, (_e855 > 4294967000f)) % 2u) == 0u);
            let _e864 = (_e855 % 1f);
            if (_e855 >= 1f) {
                phi_4403_ = select(true, false, (_e864 == 0f));
            } else {
                phi_4403_ = true;
            }
            let _e868 = phi_4403_;
            let _e869 = select(1f, _e864, _e868);
            if (select(-1f, 1f, (_e795.x >= 0f)) > 0f) {
                if _e862 {
                    phi_1458_ = _e869;
                } else {
                    phi_1458_ = (1f - _e869);
                }
                let _e876 = phi_1458_;
                phi_1464_ = _e876;
            } else {
                if _e862 {
                    phi_1463_ = (1f - _e869);
                } else {
                    phi_1463_ = _e869;
                }
                let _e873 = phi_1463_;
                phi_1464_ = _e873;
            }
            let _e878 = phi_1464_;
            phi_1466_ = _e878;
            break;
        }
        case 0: {
            if (_e795.x > 1f) {
                phi_4390_ = 0.9999999f;
            } else {
                phi_4390_ = select(_e795.x, 0.00000011920929f, (_e795.x < 0f));
            }
            let _e852 = phi_4390_;
            phi_1466_ = _e852;
            break;
        }
        default: {
            phi_1466_ = f32();
            break;
        }
    }
    let _e894 = phi_1466_;
    switch bitcast<i32>(_e843.member.member_1) {
        case 1: {
            let _e932 = abs(_e795.y);
            let _e934 = (_e932 % 1f);
            if (_e932 >= 1f) {
                phi_4468_ = select(true, false, (_e934 == 0f));
            } else {
                phi_4468_ = true;
            }
            let _e938 = phi_4468_;
            let _e939 = select(1f, _e934, _e938);
            if (select(-1f, 1f, (_e795.y >= 0f)) > 0f) {
                phi_1487_ = _e939;
            } else {
                phi_1487_ = (1f - _e939);
            }
            let _e943 = phi_1487_;
            phi_1524_ = _e943;
            break;
        }
        case 2: {
            let _e906 = abs(_e795.y);
            let _e913 = ((select(select(u32(_e906), 0u, (_e906 < 0f)), 4294967295u, (_e906 > 4294967000f)) % 2u) == 0u);
            let _e915 = (_e906 % 1f);
            if (_e906 >= 1f) {
                phi_4451_ = select(true, false, (_e915 == 0f));
            } else {
                phi_4451_ = true;
            }
            let _e919 = phi_4451_;
            let _e920 = select(1f, _e915, _e919);
            if (select(-1f, 1f, (_e795.y >= 0f)) > 0f) {
                if _e913 {
                    phi_1516_ = _e920;
                } else {
                    phi_1516_ = (1f - _e920);
                }
                let _e927 = phi_1516_;
                phi_1522_ = _e927;
            } else {
                if _e913 {
                    phi_1521_ = (1f - _e920);
                } else {
                    phi_1521_ = _e920;
                }
                let _e924 = phi_1521_;
                phi_1522_ = _e924;
            }
            let _e929 = phi_1522_;
            phi_1524_ = _e929;
            break;
        }
        case 0: {
            if (_e795.y > 1f) {
                phi_4438_ = 0.9999999f;
            } else {
                phi_4438_ = select(_e795.y, 0.00000011920929f, (_e795.y < 0f));
            }
            let _e903 = phi_4438_;
            phi_1524_ = _e903;
            break;
        }
        default: {
            phi_1524_ = f32();
            break;
        }
    }
    let _e945 = phi_1524_;
    let _e949 = (_e894 * f32(_e843.member_2.x));
    let _e958 = (_e945 * f32(_e843.member_2.y));
    let _e976 = vec3<f32>((f32((select(select(u32(_e949), 0u, (_e949 < 0f)), 4294967295u, (_e949 > 4294967000f)) + _e843.member_1.x)) / _e580), (f32((select(select(u32(_e958), 0u, (_e958 < 0f)), 4294967295u, (_e958 > 4294967000f)) + _e843.member_1.y)) / _e581), f32(_e843.member_3));
    let _e982 = textureSampleLevel(global_11, global_10, vec2<f32>(_e976.x, _e976.y), i32(_e976.z), 0f);
    let _e983 = (_e400.member_7 == 4294967295u);
    let _e985 = select(_e982, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e983));
    let _e989 = select(_e121, _e120, vec2((_e400.member_13 == 0u)));
    if _e406 {
        phi_4504_ = (_e400.member_8 <= (_e117 - 8u));
    } else {
        phi_4504_ = false;
    }
    let _e994 = phi_4504_;
    if _e994 {
        let _e997 = global.member[_e400.member_8];
        let _e1001 = global.member[(_e400.member_8 + 1u)];
        let _e1006 = global.member[(_e400.member_8 + 2u)];
        let _e1010 = global.member[(_e400.member_8 + 3u)];
        let _e1015 = global.member[(_e400.member_8 + 4u)];
        let _e1019 = global.member[(_e400.member_8 + 5u)];
        let _e1023 = global.member[(_e400.member_8 + 6u)];
        switch bitcast<i32>(_e1023) {
            case 0: {
                phi_1607_ = 0u;
                break;
            }
            case 1: {
                phi_1607_ = 1u;
                break;
            }
            case 2: {
                phi_1607_ = 2u;
                break;
            }
            default: {
                phi_1607_ = 0u;
                break;
            }
        }
        let _e1026 = phi_1607_;
        let _e1030 = global.member[(_e400.member_8 + 7u)];
        switch bitcast<i32>(_e1030) {
            case 0: {
                phi_1616_ = 0u;
                break;
            }
            case 1: {
                phi_1616_ = 1u;
                break;
            }
            case 2: {
                phi_1616_ = 2u;
                break;
            }
            default: {
                phi_1616_ = 0u;
                break;
            }
        }
        let _e1033 = phi_1616_;
        phi_1629_ = type_33(type_24(_e1026, _e1033), vec2<u32>(_e997, _e1001), vec2<u32>(_e1006, _e1010), _e1015, _e1019);
    } else {
        phi_1629_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1037 = phi_1629_;
    switch bitcast<i32>(_e1037.member.member) {
        case 1: {
            let _e1075 = abs(_e989.x);
            let _e1077 = (_e1075 % 1f);
            if (_e1075 >= 1f) {
                phi_4555_ = select(true, false, (_e1077 == 0f));
            } else {
                phi_4555_ = true;
            }
            let _e1081 = phi_4555_;
            let _e1082 = select(1f, _e1077, _e1081);
            if (select(-1f, 1f, (_e989.x >= 0f)) > 0f) {
                phi_1649_ = _e1082;
            } else {
                phi_1649_ = (1f - _e1082);
            }
            let _e1086 = phi_1649_;
            phi_1686_ = _e1086;
            break;
        }
        case 2: {
            let _e1049 = abs(_e989.x);
            let _e1056 = ((select(select(u32(_e1049), 0u, (_e1049 < 0f)), 4294967295u, (_e1049 > 4294967000f)) % 2u) == 0u);
            let _e1058 = (_e1049 % 1f);
            if (_e1049 >= 1f) {
                phi_4538_ = select(true, false, (_e1058 == 0f));
            } else {
                phi_4538_ = true;
            }
            let _e1062 = phi_4538_;
            let _e1063 = select(1f, _e1058, _e1062);
            if (select(-1f, 1f, (_e989.x >= 0f)) > 0f) {
                if _e1056 {
                    phi_1678_ = _e1063;
                } else {
                    phi_1678_ = (1f - _e1063);
                }
                let _e1070 = phi_1678_;
                phi_1684_ = _e1070;
            } else {
                if _e1056 {
                    phi_1683_ = (1f - _e1063);
                } else {
                    phi_1683_ = _e1063;
                }
                let _e1067 = phi_1683_;
                phi_1684_ = _e1067;
            }
            let _e1072 = phi_1684_;
            phi_1686_ = _e1072;
            break;
        }
        case 0: {
            if (_e989.x > 1f) {
                phi_4525_ = 0.9999999f;
            } else {
                phi_4525_ = select(_e989.x, 0.00000011920929f, (_e989.x < 0f));
            }
            let _e1046 = phi_4525_;
            phi_1686_ = _e1046;
            break;
        }
        default: {
            phi_1686_ = f32();
            break;
        }
    }
    let _e1088 = phi_1686_;
    switch bitcast<i32>(_e1037.member.member_1) {
        case 1: {
            let _e1126 = abs(_e989.y);
            let _e1128 = (_e1126 % 1f);
            if (_e1126 >= 1f) {
                phi_4603_ = select(true, false, (_e1128 == 0f));
            } else {
                phi_4603_ = true;
            }
            let _e1132 = phi_4603_;
            let _e1133 = select(1f, _e1128, _e1132);
            if (select(-1f, 1f, (_e989.y >= 0f)) > 0f) {
                phi_1707_ = _e1133;
            } else {
                phi_1707_ = (1f - _e1133);
            }
            let _e1137 = phi_1707_;
            phi_1744_ = _e1137;
            break;
        }
        case 2: {
            let _e1100 = abs(_e989.y);
            let _e1107 = ((select(select(u32(_e1100), 0u, (_e1100 < 0f)), 4294967295u, (_e1100 > 4294967000f)) % 2u) == 0u);
            let _e1109 = (_e1100 % 1f);
            if (_e1100 >= 1f) {
                phi_4586_ = select(true, false, (_e1109 == 0f));
            } else {
                phi_4586_ = true;
            }
            let _e1113 = phi_4586_;
            let _e1114 = select(1f, _e1109, _e1113);
            if (select(-1f, 1f, (_e989.y >= 0f)) > 0f) {
                if _e1107 {
                    phi_1736_ = _e1114;
                } else {
                    phi_1736_ = (1f - _e1114);
                }
                let _e1121 = phi_1736_;
                phi_1742_ = _e1121;
            } else {
                if _e1107 {
                    phi_1741_ = (1f - _e1114);
                } else {
                    phi_1741_ = _e1114;
                }
                let _e1118 = phi_1741_;
                phi_1742_ = _e1118;
            }
            let _e1123 = phi_1742_;
            phi_1744_ = _e1123;
            break;
        }
        case 0: {
            if (_e989.y > 1f) {
                phi_4573_ = 0.9999999f;
            } else {
                phi_4573_ = select(_e989.y, 0.00000011920929f, (_e989.y < 0f));
            }
            let _e1097 = phi_4573_;
            phi_1744_ = _e1097;
            break;
        }
        default: {
            phi_1744_ = f32();
            break;
        }
    }
    let _e1139 = phi_1744_;
    let _e1143 = (_e1088 * f32(_e1037.member_2.x));
    let _e1152 = (_e1139 * f32(_e1037.member_2.y));
    let _e1170 = vec3<f32>((f32((select(select(u32(_e1143), 0u, (_e1143 < 0f)), 4294967295u, (_e1143 > 4294967000f)) + _e1037.member_1.x)) / _e580), (f32((select(select(u32(_e1152), 0u, (_e1152 < 0f)), 4294967295u, (_e1152 > 4294967000f)) + _e1037.member_1.y)) / _e581), f32(_e1037.member_3));
    let _e1176 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1170.x, _e1170.y), i32(_e1170.z), 0f);
    let _e1183 = select(_e121, _e120, vec2((_e400.member_14 == 0u)));
    if _e406 {
        phi_4639_ = (_e400.member_9 <= (_e117 - 8u));
    } else {
        phi_4639_ = false;
    }
    let _e1188 = phi_4639_;
    if _e1188 {
        let _e1191 = global.member[_e400.member_9];
        let _e1195 = global.member[(_e400.member_9 + 1u)];
        let _e1200 = global.member[(_e400.member_9 + 2u)];
        let _e1204 = global.member[(_e400.member_9 + 3u)];
        let _e1209 = global.member[(_e400.member_9 + 4u)];
        let _e1213 = global.member[(_e400.member_9 + 5u)];
        let _e1217 = global.member[(_e400.member_9 + 6u)];
        switch bitcast<i32>(_e1217) {
            case 0: {
                phi_1827_ = 0u;
                break;
            }
            case 1: {
                phi_1827_ = 1u;
                break;
            }
            case 2: {
                phi_1827_ = 2u;
                break;
            }
            default: {
                phi_1827_ = 0u;
                break;
            }
        }
        let _e1220 = phi_1827_;
        let _e1224 = global.member[(_e400.member_9 + 7u)];
        switch bitcast<i32>(_e1224) {
            case 0: {
                phi_1836_ = 0u;
                break;
            }
            case 1: {
                phi_1836_ = 1u;
                break;
            }
            case 2: {
                phi_1836_ = 2u;
                break;
            }
            default: {
                phi_1836_ = 0u;
                break;
            }
        }
        let _e1227 = phi_1836_;
        phi_1849_ = type_33(type_24(_e1220, _e1227), vec2<u32>(_e1191, _e1195), vec2<u32>(_e1200, _e1204), _e1209, _e1213);
    } else {
        phi_1849_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1231 = phi_1849_;
    switch bitcast<i32>(_e1231.member.member) {
        case 1: {
            let _e1269 = abs(_e1183.x);
            let _e1271 = (_e1269 % 1f);
            if (_e1269 >= 1f) {
                phi_4690_ = select(true, false, (_e1271 == 0f));
            } else {
                phi_4690_ = true;
            }
            let _e1275 = phi_4690_;
            let _e1276 = select(1f, _e1271, _e1275);
            if (select(-1f, 1f, (_e1183.x >= 0f)) > 0f) {
                phi_1869_ = _e1276;
            } else {
                phi_1869_ = (1f - _e1276);
            }
            let _e1280 = phi_1869_;
            phi_1906_ = _e1280;
            break;
        }
        case 2: {
            let _e1243 = abs(_e1183.x);
            let _e1250 = ((select(select(u32(_e1243), 0u, (_e1243 < 0f)), 4294967295u, (_e1243 > 4294967000f)) % 2u) == 0u);
            let _e1252 = (_e1243 % 1f);
            if (_e1243 >= 1f) {
                phi_4673_ = select(true, false, (_e1252 == 0f));
            } else {
                phi_4673_ = true;
            }
            let _e1256 = phi_4673_;
            let _e1257 = select(1f, _e1252, _e1256);
            if (select(-1f, 1f, (_e1183.x >= 0f)) > 0f) {
                if _e1250 {
                    phi_1898_ = _e1257;
                } else {
                    phi_1898_ = (1f - _e1257);
                }
                let _e1264 = phi_1898_;
                phi_1904_ = _e1264;
            } else {
                if _e1250 {
                    phi_1903_ = (1f - _e1257);
                } else {
                    phi_1903_ = _e1257;
                }
                let _e1261 = phi_1903_;
                phi_1904_ = _e1261;
            }
            let _e1266 = phi_1904_;
            phi_1906_ = _e1266;
            break;
        }
        case 0: {
            if (_e1183.x > 1f) {
                phi_4660_ = 0.9999999f;
            } else {
                phi_4660_ = select(_e1183.x, 0.00000011920929f, (_e1183.x < 0f));
            }
            let _e1240 = phi_4660_;
            phi_1906_ = _e1240;
            break;
        }
        default: {
            phi_1906_ = f32();
            break;
        }
    }
    let _e1282 = phi_1906_;
    switch bitcast<i32>(_e1231.member.member_1) {
        case 1: {
            let _e1320 = abs(_e1183.y);
            let _e1322 = (_e1320 % 1f);
            if (_e1320 >= 1f) {
                phi_4738_ = select(true, false, (_e1322 == 0f));
            } else {
                phi_4738_ = true;
            }
            let _e1326 = phi_4738_;
            let _e1327 = select(1f, _e1322, _e1326);
            if (select(-1f, 1f, (_e1183.y >= 0f)) > 0f) {
                phi_1927_ = _e1327;
            } else {
                phi_1927_ = (1f - _e1327);
            }
            let _e1331 = phi_1927_;
            phi_1964_ = _e1331;
            break;
        }
        case 2: {
            let _e1294 = abs(_e1183.y);
            let _e1301 = ((select(select(u32(_e1294), 0u, (_e1294 < 0f)), 4294967295u, (_e1294 > 4294967000f)) % 2u) == 0u);
            let _e1303 = (_e1294 % 1f);
            if (_e1294 >= 1f) {
                phi_4721_ = select(true, false, (_e1303 == 0f));
            } else {
                phi_4721_ = true;
            }
            let _e1307 = phi_4721_;
            let _e1308 = select(1f, _e1303, _e1307);
            if (select(-1f, 1f, (_e1183.y >= 0f)) > 0f) {
                if _e1301 {
                    phi_1956_ = _e1308;
                } else {
                    phi_1956_ = (1f - _e1308);
                }
                let _e1315 = phi_1956_;
                phi_1962_ = _e1315;
            } else {
                if _e1301 {
                    phi_1961_ = (1f - _e1308);
                } else {
                    phi_1961_ = _e1308;
                }
                let _e1312 = phi_1961_;
                phi_1962_ = _e1312;
            }
            let _e1317 = phi_1962_;
            phi_1964_ = _e1317;
            break;
        }
        case 0: {
            if (_e1183.y > 1f) {
                phi_4708_ = 0.9999999f;
            } else {
                phi_4708_ = select(_e1183.y, 0.00000011920929f, (_e1183.y < 0f));
            }
            let _e1291 = phi_4708_;
            phi_1964_ = _e1291;
            break;
        }
        default: {
            phi_1964_ = f32();
            break;
        }
    }
    let _e1333 = phi_1964_;
    let _e1337 = (_e1282 * f32(_e1231.member_2.x));
    let _e1346 = (_e1333 * f32(_e1231.member_2.y));
    let _e1364 = vec3<f32>((f32((select(select(u32(_e1337), 0u, (_e1337 < 0f)), 4294967295u, (_e1337 > 4294967000f)) + _e1231.member_1.x)) / _e580), (f32((select(select(u32(_e1346), 0u, (_e1346 < 0f)), 4294967295u, (_e1346 > 4294967000f)) + _e1231.member_1.y)) / _e581), f32(_e1231.member_3));
    let _e1370 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1364.x, _e1364.y), i32(_e1364.z), 0f);
    let _e1373 = select(_e1370, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e400.member_9 == 4294967295u)));
    if _e983 {
        phi_2058_ = vec3<f32>(0f, 0f, 0f);
        phi_2059_ = _e122;
    } else {
        let _e1377 = fma(_e985.x, 2f, -1f);
        let _e1378 = fma(_e985.y, 2f, -1f);
        let _e1379 = fma(_e985.z, 2f, -1f);
        let _e1384 = sqrt(fma(_e1379, _e1379, fma(_e1377, _e1377, (_e1378 * _e1378))));
        if (_e1384 == 0f) {
            phi_4796_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4796_ = (vec3<f32>(_e1377, _e1378, _e1379) * (1f / _e1384));
        }
        let _e1389 = phi_4796_;
        let _e1396 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
        if (_e1396 == 0f) {
            phi_4831_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4831_ = (_e123 * (1f / _e1396));
        }
        let _e1401 = phi_4831_;
        let _e1408 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
        if (_e1408 == 0f) {
            phi_4866_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4866_ = (_e124 * (1f / _e1408));
        }
        let _e1413 = phi_4866_;
        let _e1420 = sqrt(fma(_e122.z, _e122.z, fma(_e122.x, _e122.x, (_e122.y * _e122.y))));
        if (_e1420 == 0f) {
            phi_4901_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4901_ = (_e122 * (1f / _e1420));
        }
        let _e1425 = phi_4901_;
        let _e1444 = fma(_e1425.x, _e1389.z, fma(_e1401.x, _e1389.x, (_e1413.x * _e1389.y)));
        let _e1445 = fma(_e1425.y, _e1389.z, fma(_e1401.y, _e1389.x, (_e1413.y * _e1389.y)));
        let _e1446 = fma(_e1425.z, _e1389.z, fma(_e1401.z, _e1389.x, (_e1413.z * _e1389.y)));
        let _e1451 = sqrt(fma(_e1446, _e1446, fma(_e1444, _e1444, (_e1445 * _e1445))));
        if (_e1451 == 0f) {
            phi_4936_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4936_ = (vec3<f32>(_e1444, _e1445, _e1446) * (1f / _e1451));
        }
        let _e1456 = phi_4936_;
        phi_2058_ = _e1389;
        phi_2059_ = _e1456;
    }
    let _e1458 = phi_2058_;
    let _e1460 = phi_2059_;
    let _e1464 = (_e597.x * _e400.member_2.x);
    let _e1467 = (_e597.y * _e400.member_2.y);
    let _e1470 = (_e597.z * _e400.member_2.z);
    let _e1475 = (_e1464 * _e119.x);
    let _e1477 = (_e1467 * _e119.y);
    let _e1479 = (_e1470 * _e119.z);
    let _e1484 = (_e791.y * _e400.member_4);
    let _e1487 = (_e791.z * _e400.member_3);
    let _e1491 = fma(_e400.member_16, (select(_e1176, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e400.member_8 == 4294967295u))).x - 1f), 1f);
    let _e1497 = (_e1373.x * _e400.member.x);
    let _e1499 = (_e1373.y * _e400.member.y);
    let _e1501 = (_e1373.z * _e400.member.z);
    let _e1506 = textureSampleLevel(global_12, global_13, _e1460, 0f);
    if (_e117 >= 86u) {
        phi_4968_ = (_e233 <= (_e117 - 86u));
    } else {
        phi_4968_ = false;
    }
    let _e1514 = phi_4968_;
    if _e1514 {
        let _e1517 = global.member[_e233];
        let _e1522 = global.member[(_e233 + 1u)];
        let _e1527 = global.member[(_e233 + 2u)];
        let _e1532 = global.member[(_e233 + 3u)];
        let _e1538 = global.member[(_e233 + 4u)];
        let _e1543 = global.member[(_e233 + 5u)];
        let _e1548 = global.member[(_e233 + 6u)];
        let _e1553 = global.member[(_e233 + 7u)];
        let _e1559 = global.member[(_e233 + 8u)];
        let _e1564 = global.member[(_e233 + 9u)];
        let _e1569 = global.member[(_e233 + 10u)];
        let _e1574 = global.member[(_e233 + 11u)];
        let _e1580 = global.member[(_e233 + 12u)];
        let _e1585 = global.member[(_e233 + 13u)];
        let _e1590 = global.member[(_e233 + 14u)];
        let _e1595 = global.member[(_e233 + 15u)];
        let _e1602 = global.member[(_e233 + 16u)];
        let _e1607 = global.member[(_e233 + 17u)];
        let _e1612 = global.member[(_e233 + 18u)];
        let _e1617 = global.member[(_e233 + 19u)];
        let _e1623 = global.member[(_e233 + 20u)];
        let _e1628 = global.member[(_e233 + 21u)];
        let _e1633 = global.member[(_e233 + 22u)];
        let _e1638 = global.member[(_e233 + 23u)];
        let _e1644 = global.member[(_e233 + 24u)];
        let _e1649 = global.member[(_e233 + 25u)];
        let _e1654 = global.member[(_e233 + 26u)];
        let _e1659 = global.member[(_e233 + 27u)];
        let _e1665 = global.member[(_e233 + 28u)];
        let _e1670 = global.member[(_e233 + 29u)];
        let _e1675 = global.member[(_e233 + 30u)];
        let _e1680 = global.member[(_e233 + 31u)];
        let _e1687 = global.member[(_e233 + 32u)];
        let _e1692 = global.member[(_e233 + 33u)];
        let _e1697 = global.member[(_e233 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_2266_ = type_24(0u, 6u);
        loop {
            let _e1702 = phi_2266_;
            if (_e1702.member < _e1702.member_1) {
                phi_2267_ = type_24((_e1702.member + 1u), _e1702.member_1);
                phi_2290_ = type_24(1u, _e1702.member);
            } else {
                phi_2267_ = _e1702;
                phi_2290_ = type_24(0u, type_24().member_1);
            }
            let _e1715 = phi_2267_;
            let _e1717 = phi_2290_;
            switch bitcast<i32>(_e1717.member) {
                case 0: {
                    phi_2317_ = false;
                    break;
                }
                case 1: {
                    let _e1722 = ((_e233 + 35u) + (_e1717.member_1 * 4u));
                    let _e1725 = global.member[_e1722];
                    let _e1730 = global.member[(_e1722 + 1u)];
                    let _e1735 = global.member[(_e1722 + 2u)];
                    let _e1740 = global.member[(_e1722 + 3u)];
                    local_1[_e1717.member_1] = vec4<f32>(bitcast<f32>(_e1725), bitcast<f32>(_e1730), bitcast<f32>(_e1735), bitcast<f32>(_e1740));
                    phi_2317_ = true;
                    break;
                }
                default: {
                    phi_2317_ = bool();
                    break;
                }
            }
            let _e1745 = phi_2317_;
            continue;
            continuing {
                phi_2266_ = _e1715;
                break if !(_e1745);
            }
        }
        let _e1747 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2323_ = type_24(0u, 8u);
        loop {
            let _e1750 = phi_2323_;
            if (_e1750.member < _e1750.member_1) {
                phi_2324_ = type_24((_e1750.member + 1u), _e1750.member_1);
                phi_2347_ = type_24(1u, _e1750.member);
            } else {
                phi_2324_ = _e1750;
                phi_2347_ = type_24(0u, type_24().member_1);
            }
            let _e1763 = phi_2324_;
            let _e1765 = phi_2347_;
            switch bitcast<i32>(_e1765.member) {
                case 0: {
                    phi_2370_ = false;
                    break;
                }
                case 1: {
                    let _e1770 = ((_e233 + 59u) + (_e1765.member_1 * 3u));
                    let _e1773 = global.member[_e1770];
                    let _e1778 = global.member[(_e1770 + 1u)];
                    let _e1783 = global.member[(_e1770 + 2u)];
                    local[_e1765.member_1] = vec3<f32>(bitcast<f32>(_e1773), bitcast<f32>(_e1778), bitcast<f32>(_e1783));
                    phi_2370_ = true;
                    break;
                }
                default: {
                    phi_2370_ = bool();
                    break;
                }
            }
            let _e1788 = phi_2370_;
            continue;
            continuing {
                phi_2323_ = _e1763;
                break if !(_e1788);
            }
        }
        let _e1790 = local;
        let _e1794 = global.member[(_e233 + 83u)];
        let _e1799 = global.member[(_e233 + 84u)];
        let _e1804 = global.member[(_e233 + 85u)];
        phi_2391_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1517), bitcast<f32>(_e1522), bitcast<f32>(_e1527), bitcast<f32>(_e1532)), vec4<f32>(bitcast<f32>(_e1538), bitcast<f32>(_e1543), bitcast<f32>(_e1548), bitcast<f32>(_e1553)), vec4<f32>(bitcast<f32>(_e1559), bitcast<f32>(_e1564), bitcast<f32>(_e1569), bitcast<f32>(_e1574)), vec4<f32>(bitcast<f32>(_e1580), bitcast<f32>(_e1585), bitcast<f32>(_e1590), bitcast<f32>(_e1595))), type_20(vec4<f32>(bitcast<f32>(_e1602), bitcast<f32>(_e1607), bitcast<f32>(_e1612), bitcast<f32>(_e1617)), vec4<f32>(bitcast<f32>(_e1623), bitcast<f32>(_e1628), bitcast<f32>(_e1633), bitcast<f32>(_e1638)), vec4<f32>(bitcast<f32>(_e1644), bitcast<f32>(_e1649), bitcast<f32>(_e1654), bitcast<f32>(_e1659)), vec4<f32>(bitcast<f32>(_e1665), bitcast<f32>(_e1670), bitcast<f32>(_e1675), bitcast<f32>(_e1680))), vec3<f32>(bitcast<f32>(_e1687), bitcast<f32>(_e1692), bitcast<f32>(_e1697)), type_21(_e1790, _e1747, vec3<f32>(bitcast<f32>(_e1794), bitcast<f32>(_e1799), bitcast<f32>(_e1804))));
    } else {
        phi_2391_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e1810 = phi_2391_;
    let _e1812 = (_e1810.member_2 - _e125);
    let _e1819 = sqrt(fma(_e1812.z, _e1812.z, fma(_e1812.x, _e1812.x, (_e1812.y * _e1812.y))));
    let _e1820 = (_e1819 == 0f);
    if _e1820 {
        phi_5040_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5040_ = (_e1812 * (1f / _e1819));
    }
    let _e1824 = phi_5040_;
    let _e1825 = -(_e1824);
    let _e1832 = sqrt(fma(_e1460.z, _e1460.z, fma(_e1460.x, _e1460.x, (_e1460.y * _e1460.y))));
    let _e1833 = (_e1832 == 0f);
    if _e1833 {
        phi_5099_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5099_ = (_e1460 * (1f / _e1832));
    }
    let _e1837 = phi_5099_;
    let _e1847 = (2f * fma(_e1837.z, _e1825.z, fma(_e1837.x, _e1825.x, (_e1837.y * _e1825.y))));
    let _e1854 = textureSampleLevel(global_14, global_15, (_e1825 - vec3<f32>((_e1847 * _e1837.x), (_e1847 * _e1837.y), (_e1847 * _e1837.z))), (_e1484 * 4f));
    if _e1820 {
        phi_5173_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5173_ = (_e1812 * (1f / _e1819));
    }
    let _e1861 = phi_5173_;
    let _e1870 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1460.z, _e1861.z, fma(_e1460.x, _e1861.x, (_e1460.y * _e1861.y))), 0f), _e1484), 0f);
    switch bitcast<i32>(_e255) {
        case 0: {
            if _e400.member_15 {
                if _e1833 {
                    phi_5566_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5566_ = (_e1460 * (1f / _e1832));
                }
                let _e2039 = phi_5566_;
                if _e1820 {
                    phi_5601_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5601_ = (_e1812 * (1f / _e1819));
                }
                let _e2043 = phi_5601_;
                phi_2431_ = type_24(0u, _e268);
                phi_2434_ = vec3<f32>(0f, 0f, 0f);
                loop {
                    let _e2046 = phi_2431_;
                    let _e2048 = phi_2434_;
                    local_2 = _e2048;
                    local_3 = _e2048;
                    local_4 = _e2048;
                    if (_e2046.member < _e2046.member_1) {
                        phi_2432_ = type_24((_e2046.member + 1u), _e2046.member_1);
                        phi_2457_ = type_24(1u, _e2046.member);
                    } else {
                        phi_2432_ = _e2046;
                        phi_2457_ = type_24(0u, type_24().member_1);
                    }
                    let _e2061 = phi_2432_;
                    let _e2063 = phi_2457_;
                    switch bitcast<i32>(_e2063.member) {
                        case 0: {
                            phi_2435_ = vec3<f32>();
                            phi_3328_ = false;
                            break;
                        }
                        case 1: {
                            if (_e2063.member_1 >= _e268) {
                                phi_5618_ = 4294967295u;
                            } else {
                                phi_5618_ = (_e264 + _e2063.member_1);
                            }
                            let _e2070 = phi_5618_;
                            if (_e117 >= 1u) {
                                phi_5637_ = (_e2070 <= (_e117 - 1u));
                            } else {
                                phi_5637_ = false;
                            }
                            let _e2075 = phi_5637_;
                            if _e2075 {
                                let _e2078 = global.member[_e2070];
                                phi_2474_ = _e2078;
                            } else {
                                phi_2474_ = 4294967295u;
                            }
                            let _e2080 = phi_2474_;
                            let _e2081 = (_e2080 == 4294967295u);
                            if _e2081 {
                                phi_3326_ = vec3<f32>();
                            } else {
                                if (_e117 >= 3u) {
                                    phi_5669_ = (_e2080 <= (_e117 - 3u));
                                } else {
                                    phi_5669_ = false;
                                }
                                let _e2086 = phi_5669_;
                                if _e2086 {
                                    let _e2089 = global.member[_e2080];
                                    switch bitcast<i32>(_e2089) {
                                        case 0: {
                                            phi_2491_ = 0u;
                                            break;
                                        }
                                        case 1: {
                                            phi_2491_ = 1u;
                                            break;
                                        }
                                        case 2: {
                                            phi_2491_ = 2u;
                                            break;
                                        }
                                        default: {
                                            phi_2491_ = 0u;
                                            break;
                                        }
                                    }
                                    let _e2092 = phi_2491_;
                                    let _e2096 = global.member[(_e2080 + 1u)];
                                    let _e2100 = global.member[(_e2080 + 2u)];
                                    phi_2501_ = type_34(_e2092, _e2096, _e2100);
                                } else {
                                    phi_2501_ = type_34(0u, 4294967295u, 4294967295u);
                                }
                                let _e2103 = phi_2501_;
                                if (_e117 >= 10u) {
                                    phi_5699_ = (_e2103.member_2 <= (_e117 - 10u));
                                } else {
                                    phi_5699_ = false;
                                }
                                let _e2109 = phi_5699_;
                                if _e2109 {
                                    let _e2112 = global.member[_e2103.member_2];
                                    let _e2117 = global.member[(_e2103.member_2 + 1u)];
                                    let _e2122 = global.member[(_e2103.member_2 + 2u)];
                                    let _e2128 = global.member[(_e2103.member_2 + 3u)];
                                    let _e2133 = global.member[(_e2103.member_2 + 4u)];
                                    let _e2138 = global.member[(_e2103.member_2 + 5u)];
                                    let _e2143 = global.member[(_e2103.member_2 + 6u)];
                                    let _e2149 = global.member[(_e2103.member_2 + 7u)];
                                    let _e2154 = global.member[(_e2103.member_2 + 8u)];
                                    let _e2159 = global.member[(_e2103.member_2 + 9u)];
                                    phi_2551_ = type_29(vec3<f32>(bitcast<f32>(_e2112), bitcast<f32>(_e2117), bitcast<f32>(_e2122)), vec4<f32>(bitcast<f32>(_e2128), bitcast<f32>(_e2133), bitcast<f32>(_e2138), bitcast<f32>(_e2143)), vec3<f32>(bitcast<f32>(_e2149), bitcast<f32>(_e2154), bitcast<f32>(_e2159)));
                                } else {
                                    phi_2551_ = type_29(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e2164 = phi_2551_;
                                let _e2172 = (_e2164.member_1.x + _e2164.member_1.x);
                                let _e2173 = (_e2164.member_1.y + _e2164.member_1.y);
                                let _e2174 = (_e2164.member_1.z + _e2164.member_1.z);
                                let _e2176 = (_e2164.member_1.z * _e2174);
                                let _e2177 = (_e2164.member_1.w * _e2172);
                                let _e2178 = (_e2164.member_1.w * _e2173);
                                let _e2179 = (_e2164.member_1.w * _e2174);
                                let _e2199 = (vec4<f32>((1f - fma(_e2164.member_1.y, _e2173, _e2176)), fma(_e2164.member_1.x, _e2173, _e2179), fma(_e2164.member_1.x, _e2174, -(_e2178)), 0f) * _e2164.member_2.x);
                                let _e2201 = (vec4<f32>(fma(_e2164.member_1.x, _e2173, -(_e2179)), (1f - fma(_e2164.member_1.x, _e2172, _e2176)), fma(_e2164.member_1.y, _e2174, _e2177), 0f) * _e2164.member_2.y);
                                let _e2203 = (vec4<f32>(fma(_e2164.member_1.x, _e2174, _e2178), fma(_e2164.member_1.y, _e2174, -(_e2177)), (1f - fma(_e2164.member_1.x, _e2172, (_e2164.member_1.y * _e2173))), 0f) * _e2164.member_2.z);
                                switch bitcast<i32>(_e2103.member) {
                                    case 0: {
                                        if _e406 {
                                            phi_6165_ = (_e2103.member_1 <= (_e117 - 8u));
                                        } else {
                                            phi_6165_ = false;
                                        }
                                        let _e2699 = phi_6165_;
                                        if _e2699 {
                                            let _e2702 = global.member[_e2103.member_1];
                                            let _e2707 = global.member[(_e2103.member_1 + 1u)];
                                            let _e2712 = global.member[(_e2103.member_1 + 2u)];
                                            let _e2718 = global.member[(_e2103.member_1 + 3u)];
                                            let _e2723 = global.member[(_e2103.member_1 + 4u)];
                                            let _e2728 = global.member[(_e2103.member_1 + 5u)];
                                            let _e2733 = global.member[(_e2103.member_1 + 6u)];
                                            let _e2739 = global.member[(_e2103.member_1 + 7u)];
                                            phi_2599_ = type_35(vec3<f32>(bitcast<f32>(_e2702), bitcast<f32>(_e2707), bitcast<f32>(_e2712)), vec4<f32>(bitcast<f32>(_e2718), bitcast<f32>(_e2723), bitcast<f32>(_e2728), bitcast<f32>(_e2733)), bitcast<f32>(_e2739));
                                        } else {
                                            phi_2599_ = type_35(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2743 = phi_2599_;
                                        let _e2765 = fma(_e2203.x, _e2743.member.z, fma(_e2201.x, _e2743.member.y, (_e2199.x * _e2743.member.x)));
                                        let _e2766 = fma(_e2203.y, _e2743.member.z, fma(_e2201.y, _e2743.member.y, (_e2199.y * _e2743.member.x)));
                                        let _e2767 = fma(_e2203.z, _e2743.member.z, fma(_e2201.z, _e2743.member.y, (_e2199.z * _e2743.member.x)));
                                        let _e2772 = sqrt(fma(_e2767, _e2767, fma(_e2765, _e2765, (_e2766 * _e2766))));
                                        if (_e2772 == 0f) {
                                            phi_6212_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6212_ = (vec3<f32>(_e2765, _e2766, _e2767) * (1f / _e2772));
                                        }
                                        let _e2777 = phi_6212_;
                                        let _e2779 = -(_e2777.x);
                                        let _e2781 = -(_e2777.y);
                                        let _e2783 = -(_e2777.z);
                                        let _e2784 = -(_e2777);
                                        let _e2786 = fma(-(_e791.z), _e400.member_3, 1f);
                                        let _e2790 = fma(0.4f, _e2786, (_e1475 * _e1487));
                                        let _e2791 = fma(0.4f, _e2786, (_e1477 * _e1487));
                                        let _e2792 = fma(0.4f, _e2786, (_e1479 * _e1487));
                                        let _e2800 = (_e2043 + vec3<f32>(_e2779, _e2781, _e2783));
                                        let _e2807 = sqrt(fma(_e2800.z, _e2800.z, fma(_e2800.x, _e2800.x, (_e2800.y * _e2800.y))));
                                        if (_e2807 == 0f) {
                                            phi_6247_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6247_ = (_e2800 * (1f / _e2807));
                                        }
                                        let _e2812 = phi_6247_;
                                        let _e2813 = (_e1484 * _e1484);
                                        let _e2824 = max(fma(_e2039.z, _e2812.z, fma(_e2039.x, _e2812.x, (_e2039.y * _e2812.y))), 0f);
                                        let _e2837 = max(fma(_e2039.z, _e2043.z, fma(_e2039.x, _e2043.x, (_e2039.y * _e2043.y))), 0f);
                                        let _e2844 = max(fma(_e2039.z, _e2784.z, fma(_e2039.x, _e2784.x, (_e2039.y * _e2784.y))), 0f);
                                        let _e2845 = fma(_e791.y, _e400.member_4, 1f);
                                        let _e2846 = (_e2845 * _e2845);
                                        let _e2847 = (_e2846 * 0.125f);
                                        let _e2849 = fma(-(_e2846), 0.125f, 1f);
                                        let _e2862 = (1f - max(fma(_e2812.z, _e2043.z, fma(_e2812.x, _e2043.x, (_e2812.y * _e2043.y))), 0f));
                                        let _e2864 = select(_e2862, 0f, (_e2862 < 0f));
                                        let _e2867 = pow(select(_e2864, 1f, (_e2864 > 1f)), 5f);
                                        let _e2868 = fma((1f - _e2790), _e2867, _e2790);
                                        let _e2869 = fma((1f - _e2791), _e2867, _e2791);
                                        let _e2870 = fma((1f - _e2792), _e2867, _e2792);
                                        let _e2877 = (((_e2813 * _e2813) / (pow(fma((_e2824 * _e2824), fma(_e2813, _e2813, -1f), 1f), 2f) * 3.1415927f)) * ((_e2837 / fma(_e2837, _e2849, _e2847)) * (_e2844 / fma(_e2844, _e2849, _e2847))));
                                        let _e2884 = max(fma(_e2039.z, _e2783, fma(_e2039.x, _e2779, (_e2039.y * _e2781))), 0f);
                                        let _e2886 = fma((4f * _e2837), _e2884, 0.0001f);
                                        phi_3316_ = vec3<f32>(fma((fma((((1f - _e2868) * _e2786) * _e1475), 0.31830987f, ((_e2877 * _e2868) / _e2886)) * (_e2743.member_1.x * _e2743.member_2)), _e2884, _e2048.x), fma((fma((((1f - _e2869) * _e2786) * _e1477), 0.31830987f, ((_e2877 * _e2869) / _e2886)) * (_e2743.member_1.y * _e2743.member_2)), _e2884, _e2048.y), fma((fma((((1f - _e2870) * _e2786) * _e1479), 0.31830987f, ((_e2877 * _e2870) / _e2886)) * (_e2743.member_1.z * _e2743.member_2)), _e2884, _e2048.z));
                                        phi_3317_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if _e406 {
                                            phi_5991_ = (_e2103.member_1 <= (_e117 - 8u));
                                        } else {
                                            phi_5991_ = false;
                                        }
                                        let _e2488 = phi_5991_;
                                        if _e2488 {
                                            let _e2491 = global.member[_e2103.member_1];
                                            let _e2496 = global.member[(_e2103.member_1 + 1u)];
                                            let _e2501 = global.member[(_e2103.member_1 + 2u)];
                                            let _e2507 = global.member[(_e2103.member_1 + 3u)];
                                            let _e2512 = global.member[(_e2103.member_1 + 4u)];
                                            let _e2517 = global.member[(_e2103.member_1 + 5u)];
                                            let _e2522 = global.member[(_e2103.member_1 + 6u)];
                                            let _e2528 = global.member[(_e2103.member_1 + 7u)];
                                            phi_2806_ = type_35(vec3<f32>(bitcast<f32>(_e2491), bitcast<f32>(_e2496), bitcast<f32>(_e2501)), vec4<f32>(bitcast<f32>(_e2507), bitcast<f32>(_e2512), bitcast<f32>(_e2517), bitcast<f32>(_e2522)), bitcast<f32>(_e2528));
                                        } else {
                                            phi_2806_ = type_35(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2532 = phi_2806_;
                                        let _e2561 = (vec3<f32>((_e2164.member.x + fma(_e2203.x, _e2532.member.z, fma(_e2201.x, _e2532.member.y, (_e2199.x * _e2532.member.x)))), (_e2164.member.y + fma(_e2203.y, _e2532.member.z, fma(_e2201.y, _e2532.member.y, (_e2199.y * _e2532.member.x)))), (_e2164.member.z + fma(_e2203.z, _e2532.member.z, fma(_e2201.z, _e2532.member.y, (_e2199.z * _e2532.member.x))))) - _e125);
                                        let _e2568 = sqrt(fma(_e2561.z, _e2561.z, fma(_e2561.x, _e2561.x, (_e2561.y * _e2561.y))));
                                        let _e2569 = (_e2568 == 0f);
                                        if _e2569 {
                                            phi_2996_ = vec3<f32>();
                                        } else {
                                            if _e2569 {
                                                phi_6038_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6038_ = (_e2561 * (1f / _e2568));
                                            }
                                            let _e2573 = phi_6038_;
                                            let _e2575 = (_e2532.member_2 / (_e2568 * _e2568));
                                            let _e2577 = fma(-(_e791.z), _e400.member_3, 1f);
                                            let _e2581 = fma(0.4f, _e2577, (_e1475 * _e1487));
                                            let _e2582 = fma(0.4f, _e2577, (_e1477 * _e1487));
                                            let _e2583 = fma(0.4f, _e2577, (_e1479 * _e1487));
                                            let _e2590 = (_e2043 + _e2573);
                                            let _e2597 = sqrt(fma(_e2590.z, _e2590.z, fma(_e2590.x, _e2590.x, (_e2590.y * _e2590.y))));
                                            if (_e2597 == 0f) {
                                                phi_6073_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6073_ = (_e2590 * (1f / _e2597));
                                            }
                                            let _e2602 = phi_6073_;
                                            let _e2603 = (_e1484 * _e1484);
                                            let _e2614 = max(fma(_e2039.z, _e2602.z, fma(_e2039.x, _e2602.x, (_e2039.y * _e2602.y))), 0f);
                                            let _e2627 = max(fma(_e2039.z, _e2043.z, fma(_e2039.x, _e2043.x, (_e2039.y * _e2043.y))), 0f);
                                            let _e2634 = max(fma(_e2039.z, _e2573.z, fma(_e2039.x, _e2573.x, (_e2039.y * _e2573.y))), 0f);
                                            let _e2635 = fma(_e791.y, _e400.member_4, 1f);
                                            let _e2636 = (_e2635 * _e2635);
                                            let _e2637 = (_e2636 * 0.125f);
                                            let _e2639 = fma(-(_e2636), 0.125f, 1f);
                                            let _e2652 = (1f - max(fma(_e2602.z, _e2043.z, fma(_e2602.x, _e2043.x, (_e2602.y * _e2043.y))), 0f));
                                            let _e2654 = select(_e2652, 0f, (_e2652 < 0f));
                                            let _e2657 = pow(select(_e2654, 1f, (_e2654 > 1f)), 5f);
                                            let _e2658 = fma((1f - _e2581), _e2657, _e2581);
                                            let _e2659 = fma((1f - _e2582), _e2657, _e2582);
                                            let _e2660 = fma((1f - _e2583), _e2657, _e2583);
                                            let _e2667 = (((_e2603 * _e2603) / (pow(fma((_e2614 * _e2614), fma(_e2603, _e2603, -1f), 1f), 2f) * 3.1415927f)) * ((_e2627 / fma(_e2627, _e2639, _e2637)) * (_e2634 / fma(_e2634, _e2639, _e2637))));
                                            let _e2672 = fma((4f * _e2627), _e2634, 0.0001f);
                                            phi_2996_ = vec3<f32>(fma((fma((((1f - _e2658) * _e2577) * _e1475), 0.31830987f, ((_e2667 * _e2658) / _e2672)) * (_e2532.member_1.x * _e2575)), _e2634, _e2048.x), fma((fma((((1f - _e2659) * _e2577) * _e1477), 0.31830987f, ((_e2667 * _e2659) / _e2672)) * (_e2532.member_1.y * _e2575)), _e2634, _e2048.y), fma((fma((((1f - _e2660) * _e2577) * _e1479), 0.31830987f, ((_e2667 * _e2660) / _e2672)) * (_e2532.member_1.z * _e2575)), _e2634, _e2048.z));
                                        }
                                        let _e2693 = phi_2996_;
                                        phi_3316_ = _e2693;
                                        phi_3317_ = select(true, false, _e2569);
                                        break;
                                    }
                                    case 2: {
                                        if (_e117 >= 13u) {
                                            phi_5779_ = (_e2103.member_1 <= (_e117 - 13u));
                                        } else {
                                            phi_5779_ = false;
                                        }
                                        let _e2214 = phi_5779_;
                                        if _e2214 {
                                            let _e2217 = global.member[_e2103.member_1];
                                            let _e2222 = global.member[(_e2103.member_1 + 1u)];
                                            let _e2227 = global.member[(_e2103.member_1 + 2u)];
                                            let _e2233 = global.member[(_e2103.member_1 + 3u)];
                                            let _e2238 = global.member[(_e2103.member_1 + 4u)];
                                            let _e2243 = global.member[(_e2103.member_1 + 5u)];
                                            let _e2249 = global.member[(_e2103.member_1 + 6u)];
                                            let _e2254 = global.member[(_e2103.member_1 + 7u)];
                                            let _e2259 = global.member[(_e2103.member_1 + 8u)];
                                            let _e2264 = global.member[(_e2103.member_1 + 9u)];
                                            let _e2269 = global.member[(_e2103.member_1 + 10u)];
                                            let _e2274 = global.member[(_e2103.member_1 + 11u)];
                                            let _e2280 = global.member[(_e2103.member_1 + 12u)];
                                            phi_3059_ = type_36(vec3<f32>(bitcast<f32>(_e2217), bitcast<f32>(_e2222), bitcast<f32>(_e2227)), vec3<f32>(bitcast<f32>(_e2233), bitcast<f32>(_e2238), bitcast<f32>(_e2243)), bitcast<f32>(_e2249), bitcast<f32>(_e2254), vec4<f32>(bitcast<f32>(_e2259), bitcast<f32>(_e2264), bitcast<f32>(_e2269), bitcast<f32>(_e2274)), bitcast<f32>(_e2280));
                                        } else {
                                            phi_3059_ = type_36(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2284 = phi_3059_;
                                        let _e2316 = (vec3<f32>((_e2164.member.x + fma(_e2203.x, _e2284.member.z, fma(_e2201.x, _e2284.member.y, (_e2199.x * _e2284.member.x)))), (_e2164.member.y + fma(_e2203.y, _e2284.member.z, fma(_e2201.y, _e2284.member.y, (_e2199.y * _e2284.member.x)))), (_e2164.member.z + fma(_e2203.z, _e2284.member.z, fma(_e2201.z, _e2284.member.y, (_e2199.z * _e2284.member.x))))) - _e125);
                                        let _e2323 = sqrt(fma(_e2316.z, _e2316.z, fma(_e2316.x, _e2316.x, (_e2316.y * _e2316.y))));
                                        let _e2324 = (_e2323 == 0f);
                                        if _e2324 {
                                            phi_3314_ = vec3<f32>();
                                        } else {
                                            if _e2324 {
                                                phi_5829_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5829_ = (_e2316 * (1f / _e2323));
                                            }
                                            let _e2328 = phi_5829_;
                                            let _e2338 = fma(_e2203.x, _e2284.member_1.z, fma(_e2201.x, _e2284.member_1.y, (_e2199.x * _e2284.member_1.x)));
                                            let _e2339 = fma(_e2203.y, _e2284.member_1.z, fma(_e2201.y, _e2284.member_1.y, (_e2199.y * _e2284.member_1.x)));
                                            let _e2340 = fma(_e2203.z, _e2284.member_1.z, fma(_e2201.z, _e2284.member_1.y, (_e2199.z * _e2284.member_1.x)));
                                            let _e2345 = sqrt(fma(_e2340, _e2340, fma(_e2338, _e2338, (_e2339 * _e2339))));
                                            if (_e2345 == 0f) {
                                                phi_5864_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5864_ = (vec3<f32>(_e2338, _e2339, _e2340) * (1f / _e2345));
                                            }
                                            let _e2350 = phi_5864_;
                                            let _e2362 = ((fma(_e2328.z, _e2350.z, fma(_e2328.x, _e2350.x, (_e2328.y * _e2350.y))) - _e2284.member_3) / (_e2284.member_2 - _e2284.member_3));
                                            let _e2364 = select(_e2362, 0f, (_e2362 < 0f));
                                            let _e2367 = (_e2284.member_5 * select(_e2364, 1f, (_e2364 > 1f)));
                                            let _e2369 = fma(-(_e791.z), _e400.member_3, 1f);
                                            let _e2373 = fma(0.4f, _e2369, (_e1475 * _e1487));
                                            let _e2374 = fma(0.4f, _e2369, (_e1477 * _e1487));
                                            let _e2375 = fma(0.4f, _e2369, (_e1479 * _e1487));
                                            let _e2382 = (_e2043 + _e2328);
                                            let _e2389 = sqrt(fma(_e2382.z, _e2382.z, fma(_e2382.x, _e2382.x, (_e2382.y * _e2382.y))));
                                            if (_e2389 == 0f) {
                                                phi_5899_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5899_ = (_e2382 * (1f / _e2389));
                                            }
                                            let _e2394 = phi_5899_;
                                            let _e2395 = (_e1484 * _e1484);
                                            let _e2406 = max(fma(_e2039.z, _e2394.z, fma(_e2039.x, _e2394.x, (_e2039.y * _e2394.y))), 0f);
                                            let _e2419 = max(fma(_e2039.z, _e2043.z, fma(_e2039.x, _e2043.x, (_e2039.y * _e2043.y))), 0f);
                                            let _e2423 = max(fma(_e2039.z, _e2328.z, fma(_e2039.x, _e2328.x, (_e2039.y * _e2328.y))), 0f);
                                            let _e2424 = fma(_e791.y, _e400.member_4, 1f);
                                            let _e2425 = (_e2424 * _e2424);
                                            let _e2426 = (_e2425 * 0.125f);
                                            let _e2428 = fma(-(_e2425), 0.125f, 1f);
                                            let _e2441 = (1f - max(fma(_e2394.z, _e2043.z, fma(_e2394.x, _e2043.x, (_e2394.y * _e2043.y))), 0f));
                                            let _e2443 = select(_e2441, 0f, (_e2441 < 0f));
                                            let _e2446 = pow(select(_e2443, 1f, (_e2443 > 1f)), 5f);
                                            let _e2447 = fma((1f - _e2373), _e2446, _e2373);
                                            let _e2448 = fma((1f - _e2374), _e2446, _e2374);
                                            let _e2449 = fma((1f - _e2375), _e2446, _e2375);
                                            let _e2456 = (((_e2395 * _e2395) / (pow(fma((_e2406 * _e2406), fma(_e2395, _e2395, -1f), 1f), 2f) * 3.1415927f)) * ((_e2419 / fma(_e2419, _e2428, _e2426)) * (_e2423 / fma(_e2423, _e2428, _e2426))));
                                            let _e2461 = fma((4f * _e2419), _e2423, 0.0001f);
                                            phi_3314_ = vec3<f32>(fma((fma((((1f - _e2447) * _e2369) * _e1475), 0.31830987f, ((_e2456 * _e2447) / _e2461)) * (_e2284.member_4.x * _e2367)), _e2423, _e2048.x), fma((fma((((1f - _e2448) * _e2369) * _e1477), 0.31830987f, ((_e2456 * _e2448) / _e2461)) * (_e2284.member_4.y * _e2367)), _e2423, _e2048.y), fma((fma((((1f - _e2449) * _e2369) * _e1479), 0.31830987f, ((_e2456 * _e2449) / _e2461)) * (_e2284.member_4.z * _e2367)), _e2423, _e2048.z));
                                        }
                                        let _e2482 = phi_3314_;
                                        phi_3316_ = _e2482;
                                        phi_3317_ = select(true, false, _e2324);
                                        break;
                                    }
                                    default: {
                                        phi_3316_ = vec3<f32>();
                                        phi_3317_ = bool();
                                        break;
                                    }
                                }
                                let _e2907 = phi_3316_;
                                let _e2909 = phi_3317_;
                                phi_3326_ = select(_e2907, _e2048, vec3(select(true, false, _e2909)));
                            }
                            let _e2914 = phi_3326_;
                            phi_2435_ = _e2914;
                            phi_3328_ = select(true, false, _e2081);
                            break;
                        }
                        default: {
                            phi_2435_ = vec3<f32>();
                            phi_3328_ = bool();
                            break;
                        }
                    }
                    let _e2917 = phi_2435_;
                    let _e2919 = phi_3328_;
                    continue;
                    continuing {
                        phi_2431_ = _e2061;
                        phi_2434_ = _e2917;
                        break if !(_e2919);
                    }
                }
                let _e2922 = fma(-(_e791.z), _e400.member_3, 1f);
                let _e2926 = fma(0.04f, _e2922, (_e1475 * _e1487));
                let _e2927 = fma(0.04f, _e2922, (_e1477 * _e1487));
                let _e2928 = fma(0.04f, _e2922, (_e1479 * _e1487));
                let _e2940 = fma(-(_e791.y), _e400.member_4, 1f);
                let _e2947 = (1f - max(fma(_e2039.z, _e2043.z, fma(_e2039.x, _e2043.x, (_e2039.y * _e2043.y))), 0f));
                let _e2949 = select(_e2947, 0f, (_e2947 < 0f));
                let _e2952 = pow(select(_e2949, 1f, (_e2949 > 1f)), 5f);
                let _e2953 = fma((max(_e2940, _e2926) - _e2926), _e2952, _e2926);
                let _e2954 = fma((max(_e2940, _e2927) - _e2927), _e2952, _e2927);
                let _e2955 = fma((max(_e2940, _e2928) - _e2928), _e2952, _e2928);
                let _e2975 = textureSampleLevel(global_18, global_19, vec2<f32>(fma(((bitcast<f32>(_e191) + fma(bitcast<f32>(_e171), _e125.z, fma(bitcast<f32>(_e151), _e125.y, (bitcast<f32>(_e131) * _e125.x)))) / _e226), 0.5f, 0.5f), fma(((bitcast<f32>(_e196) + fma(bitcast<f32>(_e176), _e125.z, fma(bitcast<f32>(_e156), _e125.y, (bitcast<f32>(_e136) * _e125.x)))) / _e226), -0.5f, 0.5f)), i32(0f));
                let _e2980 = (1f - select(0f, 1f, (((bitcast<f32>(_e201) + fma(bitcast<f32>(_e181), _e125.z, fma(bitcast<f32>(_e161), _e125.y, (bitcast<f32>(_e141) * _e125.x)))) / _e226) > vec4(_e2975).x)));
                let _e2985 = local_2;
                let _e2989 = local_3;
                let _e2993 = local_4;
                phi_3450_ = vec4<f32>((_e2980 * fma(_e1497, _e400.member_1, fma(fma(((1f - _e2953) * _e2922), (_e1506.x * _e1475), (_e1854.x * fma(_e2953, _e1870.x, _e1870.y))), _e1491, _e2985.x))), (_e2980 * fma(_e1499, _e400.member_1, fma(fma(((1f - _e2954) * _e2922), (_e1506.y * _e1477), (_e1854.y * fma(_e2954, _e1870.x, _e1870.y))), _e1491, _e2989.y))), (_e2980 * fma(_e1501, _e400.member_1, fma(fma(((1f - _e2955) * _e2922), (_e1506.z * _e1479), (_e1854.z * fma(_e2955, _e1870.x, _e1870.y))), _e1491, _e2993.z))), 1f);
            } else {
                phi_3450_ = (vec4<f32>((_e119.x * _e597.x), (_e119.y * _e597.y), (_e119.z * _e597.z), (_e119.w * _e597.w)) * _e400.member_2);
            }
            let _e3004 = phi_3450_;
            global_20 = _e3004;
            break;
        }
        case 1: {
            let _e2012 = sqrt(fma(_e120.x, _e120.x, (_e120.y * _e120.y)));
            if (_e2012 == 0f) {
                phi_5527_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5527_ = (vec3<f32>(_e120.x, _e120.y, 0f) * (1f / _e2012));
            }
            let _e2017 = phi_5527_;
            global_20 = vec4<f32>(((_e2017.x + 1f) * 0.5f), ((_e2017.y + 1f) * 0.5f), ((_e2017.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1991 = sqrt(fma(_e121.x, _e121.x, (_e121.y * _e121.y)));
            if (_e1991 == 0f) {
                phi_5478_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5478_ = (vec3<f32>(_e121.x, _e121.y, 0f) * (1f / _e1991));
            }
            let _e1996 = phi_5478_;
            global_20 = vec4<f32>(((_e1996.x + 1f) * 0.5f), ((_e1996.y + 1f) * 0.5f), ((_e1996.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1833 {
                phi_5429_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5429_ = (_e1460 * (1f / _e1832));
            }
            let _e1975 = phi_5429_;
            global_20 = vec4<f32>(((_e1975.x + 1f) * 0.5f), ((_e1975.y + 1f) * 0.5f), ((_e1975.z + 1f) * 0.5f), 1f);
            break;
        }
        case 4: {
            global_20 = _e119;
            break;
        }
        case 5: {
            let _e1956 = sqrt(fma(_e122.z, _e122.z, fma(_e122.x, _e122.x, (_e122.y * _e122.y))));
            if (_e1956 == 0f) {
                phi_5380_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5380_ = (_e122 * (1f / _e1956));
            }
            let _e1961 = phi_5380_;
            global_20 = vec4<f32>(((_e1961.x + 1f) * 0.5f), ((_e1961.y + 1f) * 0.5f), ((_e1961.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1934 = sqrt(fma(_e1458.z, _e1458.z, fma(_e1458.x, _e1458.x, (_e1458.y * _e1458.y))));
            if (_e1934 == 0f) {
                phi_5331_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5331_ = (_e1458 * (1f / _e1934));
            }
            let _e1939 = phi_5331_;
            global_20 = vec4<f32>(((_e1939.x + 1f) * 0.5f), ((_e1939.y + 1f) * 0.5f), ((_e1939.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1912 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
            if (_e1912 == 0f) {
                phi_5282_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5282_ = (_e123 * (1f / _e1912));
            }
            let _e1917 = phi_5282_;
            global_20 = vec4<f32>(((_e1917.x + 1f) * 0.5f), ((_e1917.y + 1f) * 0.5f), ((_e1917.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1890 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
            if (_e1890 == 0f) {
                phi_5233_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5233_ = (_e124 * (1f / _e1890));
            }
            let _e1895 = phi_5233_;
            global_20 = vec4<f32>(((_e1895.x + 1f) * 0.5f), ((_e1895.y + 1f) * 0.5f), ((_e1895.z + 1f) * 0.5f), 1f);
            break;
        }
        case 9: {
            global_20 = vec4<f32>(_e1506.x, _e1506.y, _e1506.z, 1f);
            break;
        }
        case 10: {
            global_20 = vec4<f32>(_e1854.x, _e1854.y, _e1854.z, 1f);
            break;
        }
        case 11: {
            global_20 = vec4<f32>(_e1870.x, _e1870.y, 1f, 1f);
            break;
        }
        case 12: {
            global_20 = (vec4<f32>(_e1464, _e1467, _e1470, (_e597.w * _e400.member_2.w)) * _e119);
            break;
        }
        case 13: {
            global_20 = vec4<f32>(_e1484, _e1484, _e1484, 1f);
            break;
        }
        case 14: {
            global_20 = vec4<f32>(_e1487, _e1487, _e1487, 1f);
            break;
        }
        case 15: {
            global_20 = vec4<f32>(_e1491, _e1491, _e1491, 1f);
            break;
        }
        case 16: {
            global_20 = vec4<f32>((_e1497 * _e400.member_1), (_e1499 * _e400.member_1), (_e1501 * _e400.member_1), 1f);
            break;
        }
        case 17: {
            global_20 = vec4<f32>(_e1373.x, _e1373.y, _e1373.z, 1f);
            break;
        }
        case 18: {
            global_20 = vec4<f32>(_e400.member.x, _e400.member.y, _e400.member.z, 1f);
            break;
        }
        case 19: {
            global_20 = vec4<f32>(_e400.member_1, _e400.member_1, _e400.member_1, 1f);
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
