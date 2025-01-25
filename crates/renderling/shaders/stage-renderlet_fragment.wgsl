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

struct type_31 {
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

struct type_32 {
    member: type_24,
    member_1: vec2<u32>,
    member_2: vec2<u32>,
    member_3: u32,
    member_4: u32,
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
    var phi_746_: u32;
    var phi_3775_: bool;
    var phi_900_: type_31;
    var phi_904_: type_31;
    var phi_3812_: bool;
    var phi_944_: u32;
    var phi_953_: u32;
    var phi_966_: type_32;
    var phi_3834_: f32;
    var phi_3847_: bool;
    var phi_1020_: f32;
    var phi_1015_: f32;
    var phi_1021_: f32;
    var phi_3864_: bool;
    var phi_986_: f32;
    var phi_1023_: f32;
    var phi_3882_: f32;
    var phi_3895_: bool;
    var phi_1078_: f32;
    var phi_1073_: f32;
    var phi_1079_: f32;
    var phi_3912_: bool;
    var phi_1044_: f32;
    var phi_1081_: f32;
    var phi_3948_: bool;
    var phi_1164_: u32;
    var phi_1173_: u32;
    var phi_1186_: type_32;
    var phi_3969_: f32;
    var phi_3982_: bool;
    var phi_1240_: f32;
    var phi_1235_: f32;
    var phi_1241_: f32;
    var phi_3999_: bool;
    var phi_1206_: f32;
    var phi_1243_: f32;
    var phi_4017_: f32;
    var phi_4030_: bool;
    var phi_1298_: f32;
    var phi_1293_: f32;
    var phi_1299_: f32;
    var phi_4047_: bool;
    var phi_1264_: f32;
    var phi_1301_: f32;
    var phi_4083_: bool;
    var phi_1384_: u32;
    var phi_1393_: u32;
    var phi_1406_: type_32;
    var phi_4104_: f32;
    var phi_4117_: bool;
    var phi_1460_: f32;
    var phi_1455_: f32;
    var phi_1461_: f32;
    var phi_4134_: bool;
    var phi_1426_: f32;
    var phi_1463_: f32;
    var phi_4152_: f32;
    var phi_4165_: bool;
    var phi_1518_: f32;
    var phi_1513_: f32;
    var phi_1519_: f32;
    var phi_4182_: bool;
    var phi_1484_: f32;
    var phi_1521_: f32;
    var phi_4218_: bool;
    var phi_1604_: u32;
    var phi_1613_: u32;
    var phi_1626_: type_32;
    var phi_4239_: f32;
    var phi_4252_: bool;
    var phi_1680_: f32;
    var phi_1675_: f32;
    var phi_1681_: f32;
    var phi_4269_: bool;
    var phi_1646_: f32;
    var phi_1683_: f32;
    var phi_4287_: f32;
    var phi_4300_: bool;
    var phi_1738_: f32;
    var phi_1733_: f32;
    var phi_1739_: f32;
    var phi_4317_: bool;
    var phi_1704_: f32;
    var phi_1741_: f32;
    var phi_4353_: bool;
    var phi_1824_: u32;
    var phi_1833_: u32;
    var phi_1846_: type_32;
    var phi_4374_: f32;
    var phi_4387_: bool;
    var phi_1900_: f32;
    var phi_1895_: f32;
    var phi_1901_: f32;
    var phi_4404_: bool;
    var phi_1866_: f32;
    var phi_1903_: f32;
    var phi_4422_: f32;
    var phi_4435_: bool;
    var phi_1958_: f32;
    var phi_1953_: f32;
    var phi_1959_: f32;
    var phi_4452_: bool;
    var phi_1924_: f32;
    var phi_1961_: f32;
    var phi_4510_: vec3<f32>;
    var phi_4545_: vec3<f32>;
    var phi_4580_: vec3<f32>;
    var phi_4615_: vec3<f32>;
    var phi_4650_: vec3<f32>;
    var phi_2055_: vec3<f32>;
    var phi_2056_: vec3<f32>;
    var phi_4682_: bool;
    var phi_2263_: type_24;
    var phi_2264_: type_24;
    var phi_2287_: type_24;
    var phi_2314_: bool;
    var phi_2320_: type_24;
    var phi_2321_: type_24;
    var phi_2344_: type_24;
    var phi_2367_: bool;
    var phi_2388_: type_22;
    var phi_4754_: vec3<f32>;
    var phi_4813_: vec3<f32>;
    var phi_4887_: vec3<f32>;
    var phi_4947_: vec3<f32>;
    var phi_4996_: vec3<f32>;
    var phi_5045_: vec3<f32>;
    var phi_5094_: vec3<f32>;
    var phi_5143_: vec3<f32>;
    var phi_5192_: vec3<f32>;
    var phi_5241_: vec3<f32>;
    var phi_3164_: vec4<f32>;

    let _e100 = arrayLength((&global.member));
    let _e101 = global_2;
    let _e102 = global_3;
    let _e103 = global_4;
    let _e104 = global_5;
    let _e105 = global_6;
    let _e106 = global_7;
    let _e107 = global_8;
    let _e108 = global_9;
    let _e111 = global_1.member[0u];
    let _e114 = global_1.member[_e111];
    let _e119 = global_1.member[(_e111 + 1u)];
    let _e124 = global_1.member[(_e111 + 2u)];
    let _e129 = global_1.member[(_e111 + 3u)];
    let _e134 = global_1.member[(_e111 + 4u)];
    let _e139 = global_1.member[(_e111 + 5u)];
    let _e144 = global_1.member[(_e111 + 6u)];
    let _e149 = global_1.member[(_e111 + 7u)];
    let _e154 = global_1.member[(_e111 + 8u)];
    let _e159 = global_1.member[(_e111 + 9u)];
    let _e164 = global_1.member[(_e111 + 10u)];
    let _e169 = global_1.member[(_e111 + 11u)];
    let _e174 = global_1.member[(_e111 + 12u)];
    let _e179 = global_1.member[(_e111 + 13u)];
    let _e184 = global_1.member[(_e111 + 14u)];
    let _e189 = global_1.member[(_e111 + 15u)];
    let _e209 = (bitcast<f32>(_e189) + fma(bitcast<f32>(_e169), _e108.z, fma(bitcast<f32>(_e149), _e108.y, (bitcast<f32>(_e129) * _e108.x))));
    let _e216 = global.member[(_e101 + 9u)];
    let _e220 = global.member[(_e101 + 11u)];
    let _e224 = global.member[(_e101 + 17u)];
    let _e227 = global.member[_e224];
    let _e231 = global.member[(_e224 + 1u)];
    let _e235 = global.member[(_e224 + 4u)];
    switch bitcast<i32>(_e235) {
        case 0: {
            phi_746_ = 0u;
            break;
        }
        case 1: {
            phi_746_ = 1u;
            break;
        }
        case 2: {
            phi_746_ = 2u;
            break;
        }
        case 3: {
            phi_746_ = 3u;
            break;
        }
        case 4: {
            phi_746_ = 4u;
            break;
        }
        case 5: {
            phi_746_ = 5u;
            break;
        }
        case 6: {
            phi_746_ = 6u;
            break;
        }
        case 7: {
            phi_746_ = 7u;
            break;
        }
        case 8: {
            phi_746_ = 8u;
            break;
        }
        case 9: {
            phi_746_ = 9u;
            break;
        }
        case 10: {
            phi_746_ = 10u;
            break;
        }
        case 11: {
            phi_746_ = 11u;
            break;
        }
        case 12: {
            phi_746_ = 12u;
            break;
        }
        case 13: {
            phi_746_ = 13u;
            break;
        }
        case 14: {
            phi_746_ = 14u;
            break;
        }
        case 15: {
            phi_746_ = 15u;
            break;
        }
        case 16: {
            phi_746_ = 16u;
            break;
        }
        case 17: {
            phi_746_ = 17u;
            break;
        }
        case 18: {
            phi_746_ = 18u;
            break;
        }
        case 19: {
            phi_746_ = 19u;
            break;
        }
        default: {
            phi_746_ = 0u;
            break;
        }
    }
    let _e238 = phi_746_;
    let _e242 = global.member[(_e224 + 5u)];
    if (_e220 == 4294967295u) {
        phi_904_ = type_31(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e100 >= 22u) {
            phi_3775_ = (_e220 <= (_e100 - 22u));
        } else {
            phi_3775_ = false;
        }
        let _e249 = phi_3775_;
        if _e249 {
            let _e252 = global.member[_e220];
            let _e257 = global.member[(_e220 + 1u)];
            let _e262 = global.member[(_e220 + 2u)];
            let _e268 = global.member[(_e220 + 3u)];
            let _e273 = global.member[(_e220 + 4u)];
            let _e278 = global.member[(_e220 + 5u)];
            let _e283 = global.member[(_e220 + 6u)];
            let _e288 = global.member[(_e220 + 7u)];
            let _e294 = global.member[(_e220 + 8u)];
            let _e299 = global.member[(_e220 + 9u)];
            let _e304 = global.member[(_e220 + 10u)];
            let _e308 = global.member[(_e220 + 11u)];
            let _e312 = global.member[(_e220 + 12u)];
            let _e316 = global.member[(_e220 + 13u)];
            let _e320 = global.member[(_e220 + 14u)];
            let _e324 = global.member[(_e220 + 15u)];
            let _e328 = global.member[(_e220 + 16u)];
            let _e332 = global.member[(_e220 + 17u)];
            let _e336 = global.member[(_e220 + 18u)];
            let _e340 = global.member[(_e220 + 19u)];
            let _e344 = global.member[(_e220 + 20u)];
            let _e349 = global.member[(_e220 + 21u)];
            phi_900_ = type_31(vec3<f32>(bitcast<f32>(_e252), bitcast<f32>(_e257), bitcast<f32>(_e262)), bitcast<f32>(_e268), vec4<f32>(bitcast<f32>(_e273), bitcast<f32>(_e278), bitcast<f32>(_e283), bitcast<f32>(_e288)), bitcast<f32>(_e294), bitcast<f32>(_e299), _e304, _e308, _e312, _e316, _e320, _e324, _e328, _e332, _e336, _e340, (_e344 == 1u), bitcast<f32>(_e349));
        } else {
            phi_900_ = type_31(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e353 = phi_900_;
        phi_904_ = type_31(_e353.member, _e353.member_1, _e353.member_2, _e353.member_3, _e353.member_4, _e353.member_5, _e353.member_6, _e353.member_7, _e353.member_8, _e353.member_9, _e353.member_10, _e353.member_11, _e353.member_12, _e353.member_13, _e353.member_14, (_e353.member_15 && (_e242 == 1u)), _e353.member_16);
    }
    let _e375 = phi_904_;
    let _e379 = select(_e104, _e103, vec2((_e375.member_10 == 0u)));
    let _e381 = (_e100 >= 8u);
    if _e381 {
        phi_3812_ = (_e375.member_5 <= (_e100 - 8u));
    } else {
        phi_3812_ = false;
    }
    let _e385 = phi_3812_;
    if _e385 {
        let _e388 = global.member[_e375.member_5];
        let _e392 = global.member[(_e375.member_5 + 1u)];
        let _e397 = global.member[(_e375.member_5 + 2u)];
        let _e401 = global.member[(_e375.member_5 + 3u)];
        let _e406 = global.member[(_e375.member_5 + 4u)];
        let _e410 = global.member[(_e375.member_5 + 5u)];
        let _e414 = global.member[(_e375.member_5 + 6u)];
        switch bitcast<i32>(_e414) {
            case 0: {
                phi_944_ = 0u;
                break;
            }
            case 1: {
                phi_944_ = 1u;
                break;
            }
            case 2: {
                phi_944_ = 2u;
                break;
            }
            default: {
                phi_944_ = 0u;
                break;
            }
        }
        let _e417 = phi_944_;
        let _e421 = global.member[(_e375.member_5 + 7u)];
        switch bitcast<i32>(_e421) {
            case 0: {
                phi_953_ = 0u;
                break;
            }
            case 1: {
                phi_953_ = 1u;
                break;
            }
            case 2: {
                phi_953_ = 2u;
                break;
            }
            default: {
                phi_953_ = 0u;
                break;
            }
        }
        let _e424 = phi_953_;
        phi_966_ = type_32(type_24(_e417, _e424), vec2<u32>(_e388, _e392), vec2<u32>(_e397, _e401), _e406, _e410);
    } else {
        phi_966_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e428 = phi_966_;
    switch bitcast<i32>(_e428.member.member) {
        case 1: {
            let _e466 = abs(_e379.x);
            let _e468 = (_e466 % 1f);
            if (_e466 >= 1f) {
                phi_3864_ = select(true, false, (_e468 == 0f));
            } else {
                phi_3864_ = true;
            }
            let _e472 = phi_3864_;
            let _e473 = select(1f, _e468, _e472);
            if (select(-1f, 1f, (_e379.x >= 0f)) > 0f) {
                phi_986_ = _e473;
            } else {
                phi_986_ = (1f - _e473);
            }
            let _e477 = phi_986_;
            phi_1023_ = _e477;
            break;
        }
        case 2: {
            let _e440 = abs(_e379.x);
            let _e447 = ((select(select(u32(_e440), 0u, (_e440 < 0f)), 4294967295u, (_e440 > 4294967000f)) % 2u) == 0u);
            let _e449 = (_e440 % 1f);
            if (_e440 >= 1f) {
                phi_3847_ = select(true, false, (_e449 == 0f));
            } else {
                phi_3847_ = true;
            }
            let _e453 = phi_3847_;
            let _e454 = select(1f, _e449, _e453);
            if (select(-1f, 1f, (_e379.x >= 0f)) > 0f) {
                if _e447 {
                    phi_1015_ = _e454;
                } else {
                    phi_1015_ = (1f - _e454);
                }
                let _e461 = phi_1015_;
                phi_1021_ = _e461;
            } else {
                if _e447 {
                    phi_1020_ = (1f - _e454);
                } else {
                    phi_1020_ = _e454;
                }
                let _e458 = phi_1020_;
                phi_1021_ = _e458;
            }
            let _e463 = phi_1021_;
            phi_1023_ = _e463;
            break;
        }
        case 0: {
            if (_e379.x > 1f) {
                phi_3834_ = 0.9999999f;
            } else {
                phi_3834_ = select(_e379.x, 0.00000011920929f, (_e379.x < 0f));
            }
            let _e437 = phi_3834_;
            phi_1023_ = _e437;
            break;
        }
        default: {
            phi_1023_ = f32();
            break;
        }
    }
    let _e479 = phi_1023_;
    switch bitcast<i32>(_e428.member.member_1) {
        case 1: {
            let _e517 = abs(_e379.y);
            let _e519 = (_e517 % 1f);
            if (_e517 >= 1f) {
                phi_3912_ = select(true, false, (_e519 == 0f));
            } else {
                phi_3912_ = true;
            }
            let _e523 = phi_3912_;
            let _e524 = select(1f, _e519, _e523);
            if (select(-1f, 1f, (_e379.y >= 0f)) > 0f) {
                phi_1044_ = _e524;
            } else {
                phi_1044_ = (1f - _e524);
            }
            let _e528 = phi_1044_;
            phi_1081_ = _e528;
            break;
        }
        case 2: {
            let _e491 = abs(_e379.y);
            let _e498 = ((select(select(u32(_e491), 0u, (_e491 < 0f)), 4294967295u, (_e491 > 4294967000f)) % 2u) == 0u);
            let _e500 = (_e491 % 1f);
            if (_e491 >= 1f) {
                phi_3895_ = select(true, false, (_e500 == 0f));
            } else {
                phi_3895_ = true;
            }
            let _e504 = phi_3895_;
            let _e505 = select(1f, _e500, _e504);
            if (select(-1f, 1f, (_e379.y >= 0f)) > 0f) {
                if _e498 {
                    phi_1073_ = _e505;
                } else {
                    phi_1073_ = (1f - _e505);
                }
                let _e512 = phi_1073_;
                phi_1079_ = _e512;
            } else {
                if _e498 {
                    phi_1078_ = (1f - _e505);
                } else {
                    phi_1078_ = _e505;
                }
                let _e509 = phi_1078_;
                phi_1079_ = _e509;
            }
            let _e514 = phi_1079_;
            phi_1081_ = _e514;
            break;
        }
        case 0: {
            if (_e379.y > 1f) {
                phi_3882_ = 0.9999999f;
            } else {
                phi_3882_ = select(_e379.y, 0.00000011920929f, (_e379.y < 0f));
            }
            let _e488 = phi_3882_;
            phi_1081_ = _e488;
            break;
        }
        default: {
            phi_1081_ = f32();
            break;
        }
    }
    let _e530 = phi_1081_;
    let _e534 = (_e479 * f32(_e428.member_2.x));
    let _e543 = (_e530 * f32(_e428.member_2.y));
    let _e555 = f32(_e227);
    let _e556 = f32(_e231);
    let _e563 = vec3<f32>((f32((select(select(u32(_e534), 0u, (_e534 < 0f)), 4294967295u, (_e534 > 4294967000f)) + _e428.member_1.x)) / _e555), (f32((select(select(u32(_e543), 0u, (_e543 < 0f)), 4294967295u, (_e543 > 4294967000f)) + _e428.member_1.y)) / _e556), f32(_e428.member_3));
    let _e569 = textureSampleLevel(global_11, global_10, vec2<f32>(_e563.x, _e563.y), i32(_e563.z), 0f);
    let _e572 = select(_e569, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e375.member_5 == 4294967295u)));
    let _e576 = select(_e104, _e103, vec2((_e375.member_11 == 0u)));
    if _e381 {
        phi_3948_ = (_e375.member_6 <= (_e100 - 8u));
    } else {
        phi_3948_ = false;
    }
    let _e581 = phi_3948_;
    if _e581 {
        let _e584 = global.member[_e375.member_6];
        let _e588 = global.member[(_e375.member_6 + 1u)];
        let _e593 = global.member[(_e375.member_6 + 2u)];
        let _e597 = global.member[(_e375.member_6 + 3u)];
        let _e602 = global.member[(_e375.member_6 + 4u)];
        let _e606 = global.member[(_e375.member_6 + 5u)];
        let _e610 = global.member[(_e375.member_6 + 6u)];
        switch bitcast<i32>(_e610) {
            case 0: {
                phi_1164_ = 0u;
                break;
            }
            case 1: {
                phi_1164_ = 1u;
                break;
            }
            case 2: {
                phi_1164_ = 2u;
                break;
            }
            default: {
                phi_1164_ = 0u;
                break;
            }
        }
        let _e613 = phi_1164_;
        let _e617 = global.member[(_e375.member_6 + 7u)];
        switch bitcast<i32>(_e617) {
            case 0: {
                phi_1173_ = 0u;
                break;
            }
            case 1: {
                phi_1173_ = 1u;
                break;
            }
            case 2: {
                phi_1173_ = 2u;
                break;
            }
            default: {
                phi_1173_ = 0u;
                break;
            }
        }
        let _e620 = phi_1173_;
        phi_1186_ = type_32(type_24(_e613, _e620), vec2<u32>(_e584, _e588), vec2<u32>(_e593, _e597), _e602, _e606);
    } else {
        phi_1186_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e624 = phi_1186_;
    switch bitcast<i32>(_e624.member.member) {
        case 1: {
            let _e662 = abs(_e576.x);
            let _e664 = (_e662 % 1f);
            if (_e662 >= 1f) {
                phi_3999_ = select(true, false, (_e664 == 0f));
            } else {
                phi_3999_ = true;
            }
            let _e668 = phi_3999_;
            let _e669 = select(1f, _e664, _e668);
            if (select(-1f, 1f, (_e576.x >= 0f)) > 0f) {
                phi_1206_ = _e669;
            } else {
                phi_1206_ = (1f - _e669);
            }
            let _e673 = phi_1206_;
            phi_1243_ = _e673;
            break;
        }
        case 2: {
            let _e636 = abs(_e576.x);
            let _e643 = ((select(select(u32(_e636), 0u, (_e636 < 0f)), 4294967295u, (_e636 > 4294967000f)) % 2u) == 0u);
            let _e645 = (_e636 % 1f);
            if (_e636 >= 1f) {
                phi_3982_ = select(true, false, (_e645 == 0f));
            } else {
                phi_3982_ = true;
            }
            let _e649 = phi_3982_;
            let _e650 = select(1f, _e645, _e649);
            if (select(-1f, 1f, (_e576.x >= 0f)) > 0f) {
                if _e643 {
                    phi_1235_ = _e650;
                } else {
                    phi_1235_ = (1f - _e650);
                }
                let _e657 = phi_1235_;
                phi_1241_ = _e657;
            } else {
                if _e643 {
                    phi_1240_ = (1f - _e650);
                } else {
                    phi_1240_ = _e650;
                }
                let _e654 = phi_1240_;
                phi_1241_ = _e654;
            }
            let _e659 = phi_1241_;
            phi_1243_ = _e659;
            break;
        }
        case 0: {
            if (_e576.x > 1f) {
                phi_3969_ = 0.9999999f;
            } else {
                phi_3969_ = select(_e576.x, 0.00000011920929f, (_e576.x < 0f));
            }
            let _e633 = phi_3969_;
            phi_1243_ = _e633;
            break;
        }
        default: {
            phi_1243_ = f32();
            break;
        }
    }
    let _e675 = phi_1243_;
    switch bitcast<i32>(_e624.member.member_1) {
        case 1: {
            let _e713 = abs(_e576.y);
            let _e715 = (_e713 % 1f);
            if (_e713 >= 1f) {
                phi_4047_ = select(true, false, (_e715 == 0f));
            } else {
                phi_4047_ = true;
            }
            let _e719 = phi_4047_;
            let _e720 = select(1f, _e715, _e719);
            if (select(-1f, 1f, (_e576.y >= 0f)) > 0f) {
                phi_1264_ = _e720;
            } else {
                phi_1264_ = (1f - _e720);
            }
            let _e724 = phi_1264_;
            phi_1301_ = _e724;
            break;
        }
        case 2: {
            let _e687 = abs(_e576.y);
            let _e694 = ((select(select(u32(_e687), 0u, (_e687 < 0f)), 4294967295u, (_e687 > 4294967000f)) % 2u) == 0u);
            let _e696 = (_e687 % 1f);
            if (_e687 >= 1f) {
                phi_4030_ = select(true, false, (_e696 == 0f));
            } else {
                phi_4030_ = true;
            }
            let _e700 = phi_4030_;
            let _e701 = select(1f, _e696, _e700);
            if (select(-1f, 1f, (_e576.y >= 0f)) > 0f) {
                if _e694 {
                    phi_1293_ = _e701;
                } else {
                    phi_1293_ = (1f - _e701);
                }
                let _e708 = phi_1293_;
                phi_1299_ = _e708;
            } else {
                if _e694 {
                    phi_1298_ = (1f - _e701);
                } else {
                    phi_1298_ = _e701;
                }
                let _e705 = phi_1298_;
                phi_1299_ = _e705;
            }
            let _e710 = phi_1299_;
            phi_1301_ = _e710;
            break;
        }
        case 0: {
            if (_e576.y > 1f) {
                phi_4017_ = 0.9999999f;
            } else {
                phi_4017_ = select(_e576.y, 0.00000011920929f, (_e576.y < 0f));
            }
            let _e684 = phi_4017_;
            phi_1301_ = _e684;
            break;
        }
        default: {
            phi_1301_ = f32();
            break;
        }
    }
    let _e726 = phi_1301_;
    let _e730 = (_e675 * f32(_e624.member_2.x));
    let _e739 = (_e726 * f32(_e624.member_2.y));
    let _e757 = vec3<f32>((f32((select(select(u32(_e730), 0u, (_e730 < 0f)), 4294967295u, (_e730 > 4294967000f)) + _e624.member_1.x)) / _e555), (f32((select(select(u32(_e739), 0u, (_e739 < 0f)), 4294967295u, (_e739 > 4294967000f)) + _e624.member_1.y)) / _e556), f32(_e624.member_3));
    let _e763 = textureSampleLevel(global_11, global_10, vec2<f32>(_e757.x, _e757.y), i32(_e757.z), 0f);
    let _e766 = select(_e763, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e375.member_6 == 4294967295u)));
    let _e770 = select(_e104, _e103, vec2((_e375.member_12 == 0u)));
    if _e381 {
        phi_4083_ = (_e375.member_7 <= (_e100 - 8u));
    } else {
        phi_4083_ = false;
    }
    let _e775 = phi_4083_;
    if _e775 {
        let _e778 = global.member[_e375.member_7];
        let _e782 = global.member[(_e375.member_7 + 1u)];
        let _e787 = global.member[(_e375.member_7 + 2u)];
        let _e791 = global.member[(_e375.member_7 + 3u)];
        let _e796 = global.member[(_e375.member_7 + 4u)];
        let _e800 = global.member[(_e375.member_7 + 5u)];
        let _e804 = global.member[(_e375.member_7 + 6u)];
        switch bitcast<i32>(_e804) {
            case 0: {
                phi_1384_ = 0u;
                break;
            }
            case 1: {
                phi_1384_ = 1u;
                break;
            }
            case 2: {
                phi_1384_ = 2u;
                break;
            }
            default: {
                phi_1384_ = 0u;
                break;
            }
        }
        let _e807 = phi_1384_;
        let _e811 = global.member[(_e375.member_7 + 7u)];
        switch bitcast<i32>(_e811) {
            case 0: {
                phi_1393_ = 0u;
                break;
            }
            case 1: {
                phi_1393_ = 1u;
                break;
            }
            case 2: {
                phi_1393_ = 2u;
                break;
            }
            default: {
                phi_1393_ = 0u;
                break;
            }
        }
        let _e814 = phi_1393_;
        phi_1406_ = type_32(type_24(_e807, _e814), vec2<u32>(_e778, _e782), vec2<u32>(_e787, _e791), _e796, _e800);
    } else {
        phi_1406_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e818 = phi_1406_;
    switch bitcast<i32>(_e818.member.member) {
        case 1: {
            let _e856 = abs(_e770.x);
            let _e858 = (_e856 % 1f);
            if (_e856 >= 1f) {
                phi_4134_ = select(true, false, (_e858 == 0f));
            } else {
                phi_4134_ = true;
            }
            let _e862 = phi_4134_;
            let _e863 = select(1f, _e858, _e862);
            if (select(-1f, 1f, (_e770.x >= 0f)) > 0f) {
                phi_1426_ = _e863;
            } else {
                phi_1426_ = (1f - _e863);
            }
            let _e867 = phi_1426_;
            phi_1463_ = _e867;
            break;
        }
        case 2: {
            let _e830 = abs(_e770.x);
            let _e837 = ((select(select(u32(_e830), 0u, (_e830 < 0f)), 4294967295u, (_e830 > 4294967000f)) % 2u) == 0u);
            let _e839 = (_e830 % 1f);
            if (_e830 >= 1f) {
                phi_4117_ = select(true, false, (_e839 == 0f));
            } else {
                phi_4117_ = true;
            }
            let _e843 = phi_4117_;
            let _e844 = select(1f, _e839, _e843);
            if (select(-1f, 1f, (_e770.x >= 0f)) > 0f) {
                if _e837 {
                    phi_1455_ = _e844;
                } else {
                    phi_1455_ = (1f - _e844);
                }
                let _e851 = phi_1455_;
                phi_1461_ = _e851;
            } else {
                if _e837 {
                    phi_1460_ = (1f - _e844);
                } else {
                    phi_1460_ = _e844;
                }
                let _e848 = phi_1460_;
                phi_1461_ = _e848;
            }
            let _e853 = phi_1461_;
            phi_1463_ = _e853;
            break;
        }
        case 0: {
            if (_e770.x > 1f) {
                phi_4104_ = 0.9999999f;
            } else {
                phi_4104_ = select(_e770.x, 0.00000011920929f, (_e770.x < 0f));
            }
            let _e827 = phi_4104_;
            phi_1463_ = _e827;
            break;
        }
        default: {
            phi_1463_ = f32();
            break;
        }
    }
    let _e869 = phi_1463_;
    switch bitcast<i32>(_e818.member.member_1) {
        case 1: {
            let _e907 = abs(_e770.y);
            let _e909 = (_e907 % 1f);
            if (_e907 >= 1f) {
                phi_4182_ = select(true, false, (_e909 == 0f));
            } else {
                phi_4182_ = true;
            }
            let _e913 = phi_4182_;
            let _e914 = select(1f, _e909, _e913);
            if (select(-1f, 1f, (_e770.y >= 0f)) > 0f) {
                phi_1484_ = _e914;
            } else {
                phi_1484_ = (1f - _e914);
            }
            let _e918 = phi_1484_;
            phi_1521_ = _e918;
            break;
        }
        case 2: {
            let _e881 = abs(_e770.y);
            let _e888 = ((select(select(u32(_e881), 0u, (_e881 < 0f)), 4294967295u, (_e881 > 4294967000f)) % 2u) == 0u);
            let _e890 = (_e881 % 1f);
            if (_e881 >= 1f) {
                phi_4165_ = select(true, false, (_e890 == 0f));
            } else {
                phi_4165_ = true;
            }
            let _e894 = phi_4165_;
            let _e895 = select(1f, _e890, _e894);
            if (select(-1f, 1f, (_e770.y >= 0f)) > 0f) {
                if _e888 {
                    phi_1513_ = _e895;
                } else {
                    phi_1513_ = (1f - _e895);
                }
                let _e902 = phi_1513_;
                phi_1519_ = _e902;
            } else {
                if _e888 {
                    phi_1518_ = (1f - _e895);
                } else {
                    phi_1518_ = _e895;
                }
                let _e899 = phi_1518_;
                phi_1519_ = _e899;
            }
            let _e904 = phi_1519_;
            phi_1521_ = _e904;
            break;
        }
        case 0: {
            if (_e770.y > 1f) {
                phi_4152_ = 0.9999999f;
            } else {
                phi_4152_ = select(_e770.y, 0.00000011920929f, (_e770.y < 0f));
            }
            let _e878 = phi_4152_;
            phi_1521_ = _e878;
            break;
        }
        default: {
            phi_1521_ = f32();
            break;
        }
    }
    let _e920 = phi_1521_;
    let _e924 = (_e869 * f32(_e818.member_2.x));
    let _e933 = (_e920 * f32(_e818.member_2.y));
    let _e951 = vec3<f32>((f32((select(select(u32(_e924), 0u, (_e924 < 0f)), 4294967295u, (_e924 > 4294967000f)) + _e818.member_1.x)) / _e555), (f32((select(select(u32(_e933), 0u, (_e933 < 0f)), 4294967295u, (_e933 > 4294967000f)) + _e818.member_1.y)) / _e556), f32(_e818.member_3));
    let _e957 = textureSampleLevel(global_11, global_10, vec2<f32>(_e951.x, _e951.y), i32(_e951.z), 0f);
    let _e958 = (_e375.member_7 == 4294967295u);
    let _e960 = select(_e957, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e958));
    let _e964 = select(_e104, _e103, vec2((_e375.member_13 == 0u)));
    if _e381 {
        phi_4218_ = (_e375.member_8 <= (_e100 - 8u));
    } else {
        phi_4218_ = false;
    }
    let _e969 = phi_4218_;
    if _e969 {
        let _e972 = global.member[_e375.member_8];
        let _e976 = global.member[(_e375.member_8 + 1u)];
        let _e981 = global.member[(_e375.member_8 + 2u)];
        let _e985 = global.member[(_e375.member_8 + 3u)];
        let _e990 = global.member[(_e375.member_8 + 4u)];
        let _e994 = global.member[(_e375.member_8 + 5u)];
        let _e998 = global.member[(_e375.member_8 + 6u)];
        switch bitcast<i32>(_e998) {
            case 0: {
                phi_1604_ = 0u;
                break;
            }
            case 1: {
                phi_1604_ = 1u;
                break;
            }
            case 2: {
                phi_1604_ = 2u;
                break;
            }
            default: {
                phi_1604_ = 0u;
                break;
            }
        }
        let _e1001 = phi_1604_;
        let _e1005 = global.member[(_e375.member_8 + 7u)];
        switch bitcast<i32>(_e1005) {
            case 0: {
                phi_1613_ = 0u;
                break;
            }
            case 1: {
                phi_1613_ = 1u;
                break;
            }
            case 2: {
                phi_1613_ = 2u;
                break;
            }
            default: {
                phi_1613_ = 0u;
                break;
            }
        }
        let _e1008 = phi_1613_;
        phi_1626_ = type_32(type_24(_e1001, _e1008), vec2<u32>(_e972, _e976), vec2<u32>(_e981, _e985), _e990, _e994);
    } else {
        phi_1626_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1012 = phi_1626_;
    switch bitcast<i32>(_e1012.member.member) {
        case 1: {
            let _e1050 = abs(_e964.x);
            let _e1052 = (_e1050 % 1f);
            if (_e1050 >= 1f) {
                phi_4269_ = select(true, false, (_e1052 == 0f));
            } else {
                phi_4269_ = true;
            }
            let _e1056 = phi_4269_;
            let _e1057 = select(1f, _e1052, _e1056);
            if (select(-1f, 1f, (_e964.x >= 0f)) > 0f) {
                phi_1646_ = _e1057;
            } else {
                phi_1646_ = (1f - _e1057);
            }
            let _e1061 = phi_1646_;
            phi_1683_ = _e1061;
            break;
        }
        case 2: {
            let _e1024 = abs(_e964.x);
            let _e1031 = ((select(select(u32(_e1024), 0u, (_e1024 < 0f)), 4294967295u, (_e1024 > 4294967000f)) % 2u) == 0u);
            let _e1033 = (_e1024 % 1f);
            if (_e1024 >= 1f) {
                phi_4252_ = select(true, false, (_e1033 == 0f));
            } else {
                phi_4252_ = true;
            }
            let _e1037 = phi_4252_;
            let _e1038 = select(1f, _e1033, _e1037);
            if (select(-1f, 1f, (_e964.x >= 0f)) > 0f) {
                if _e1031 {
                    phi_1675_ = _e1038;
                } else {
                    phi_1675_ = (1f - _e1038);
                }
                let _e1045 = phi_1675_;
                phi_1681_ = _e1045;
            } else {
                if _e1031 {
                    phi_1680_ = (1f - _e1038);
                } else {
                    phi_1680_ = _e1038;
                }
                let _e1042 = phi_1680_;
                phi_1681_ = _e1042;
            }
            let _e1047 = phi_1681_;
            phi_1683_ = _e1047;
            break;
        }
        case 0: {
            if (_e964.x > 1f) {
                phi_4239_ = 0.9999999f;
            } else {
                phi_4239_ = select(_e964.x, 0.00000011920929f, (_e964.x < 0f));
            }
            let _e1021 = phi_4239_;
            phi_1683_ = _e1021;
            break;
        }
        default: {
            phi_1683_ = f32();
            break;
        }
    }
    let _e1063 = phi_1683_;
    switch bitcast<i32>(_e1012.member.member_1) {
        case 1: {
            let _e1101 = abs(_e964.y);
            let _e1103 = (_e1101 % 1f);
            if (_e1101 >= 1f) {
                phi_4317_ = select(true, false, (_e1103 == 0f));
            } else {
                phi_4317_ = true;
            }
            let _e1107 = phi_4317_;
            let _e1108 = select(1f, _e1103, _e1107);
            if (select(-1f, 1f, (_e964.y >= 0f)) > 0f) {
                phi_1704_ = _e1108;
            } else {
                phi_1704_ = (1f - _e1108);
            }
            let _e1112 = phi_1704_;
            phi_1741_ = _e1112;
            break;
        }
        case 2: {
            let _e1075 = abs(_e964.y);
            let _e1082 = ((select(select(u32(_e1075), 0u, (_e1075 < 0f)), 4294967295u, (_e1075 > 4294967000f)) % 2u) == 0u);
            let _e1084 = (_e1075 % 1f);
            if (_e1075 >= 1f) {
                phi_4300_ = select(true, false, (_e1084 == 0f));
            } else {
                phi_4300_ = true;
            }
            let _e1088 = phi_4300_;
            let _e1089 = select(1f, _e1084, _e1088);
            if (select(-1f, 1f, (_e964.y >= 0f)) > 0f) {
                if _e1082 {
                    phi_1733_ = _e1089;
                } else {
                    phi_1733_ = (1f - _e1089);
                }
                let _e1096 = phi_1733_;
                phi_1739_ = _e1096;
            } else {
                if _e1082 {
                    phi_1738_ = (1f - _e1089);
                } else {
                    phi_1738_ = _e1089;
                }
                let _e1093 = phi_1738_;
                phi_1739_ = _e1093;
            }
            let _e1098 = phi_1739_;
            phi_1741_ = _e1098;
            break;
        }
        case 0: {
            if (_e964.y > 1f) {
                phi_4287_ = 0.9999999f;
            } else {
                phi_4287_ = select(_e964.y, 0.00000011920929f, (_e964.y < 0f));
            }
            let _e1072 = phi_4287_;
            phi_1741_ = _e1072;
            break;
        }
        default: {
            phi_1741_ = f32();
            break;
        }
    }
    let _e1114 = phi_1741_;
    let _e1118 = (_e1063 * f32(_e1012.member_2.x));
    let _e1127 = (_e1114 * f32(_e1012.member_2.y));
    let _e1145 = vec3<f32>((f32((select(select(u32(_e1118), 0u, (_e1118 < 0f)), 4294967295u, (_e1118 > 4294967000f)) + _e1012.member_1.x)) / _e555), (f32((select(select(u32(_e1127), 0u, (_e1127 < 0f)), 4294967295u, (_e1127 > 4294967000f)) + _e1012.member_1.y)) / _e556), f32(_e1012.member_3));
    let _e1151 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1145.x, _e1145.y), i32(_e1145.z), 0f);
    let _e1158 = select(_e104, _e103, vec2((_e375.member_14 == 0u)));
    if _e381 {
        phi_4353_ = (_e375.member_9 <= (_e100 - 8u));
    } else {
        phi_4353_ = false;
    }
    let _e1163 = phi_4353_;
    if _e1163 {
        let _e1166 = global.member[_e375.member_9];
        let _e1170 = global.member[(_e375.member_9 + 1u)];
        let _e1175 = global.member[(_e375.member_9 + 2u)];
        let _e1179 = global.member[(_e375.member_9 + 3u)];
        let _e1184 = global.member[(_e375.member_9 + 4u)];
        let _e1188 = global.member[(_e375.member_9 + 5u)];
        let _e1192 = global.member[(_e375.member_9 + 6u)];
        switch bitcast<i32>(_e1192) {
            case 0: {
                phi_1824_ = 0u;
                break;
            }
            case 1: {
                phi_1824_ = 1u;
                break;
            }
            case 2: {
                phi_1824_ = 2u;
                break;
            }
            default: {
                phi_1824_ = 0u;
                break;
            }
        }
        let _e1195 = phi_1824_;
        let _e1199 = global.member[(_e375.member_9 + 7u)];
        switch bitcast<i32>(_e1199) {
            case 0: {
                phi_1833_ = 0u;
                break;
            }
            case 1: {
                phi_1833_ = 1u;
                break;
            }
            case 2: {
                phi_1833_ = 2u;
                break;
            }
            default: {
                phi_1833_ = 0u;
                break;
            }
        }
        let _e1202 = phi_1833_;
        phi_1846_ = type_32(type_24(_e1195, _e1202), vec2<u32>(_e1166, _e1170), vec2<u32>(_e1175, _e1179), _e1184, _e1188);
    } else {
        phi_1846_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1206 = phi_1846_;
    switch bitcast<i32>(_e1206.member.member) {
        case 1: {
            let _e1244 = abs(_e1158.x);
            let _e1246 = (_e1244 % 1f);
            if (_e1244 >= 1f) {
                phi_4404_ = select(true, false, (_e1246 == 0f));
            } else {
                phi_4404_ = true;
            }
            let _e1250 = phi_4404_;
            let _e1251 = select(1f, _e1246, _e1250);
            if (select(-1f, 1f, (_e1158.x >= 0f)) > 0f) {
                phi_1866_ = _e1251;
            } else {
                phi_1866_ = (1f - _e1251);
            }
            let _e1255 = phi_1866_;
            phi_1903_ = _e1255;
            break;
        }
        case 2: {
            let _e1218 = abs(_e1158.x);
            let _e1225 = ((select(select(u32(_e1218), 0u, (_e1218 < 0f)), 4294967295u, (_e1218 > 4294967000f)) % 2u) == 0u);
            let _e1227 = (_e1218 % 1f);
            if (_e1218 >= 1f) {
                phi_4387_ = select(true, false, (_e1227 == 0f));
            } else {
                phi_4387_ = true;
            }
            let _e1231 = phi_4387_;
            let _e1232 = select(1f, _e1227, _e1231);
            if (select(-1f, 1f, (_e1158.x >= 0f)) > 0f) {
                if _e1225 {
                    phi_1895_ = _e1232;
                } else {
                    phi_1895_ = (1f - _e1232);
                }
                let _e1239 = phi_1895_;
                phi_1901_ = _e1239;
            } else {
                if _e1225 {
                    phi_1900_ = (1f - _e1232);
                } else {
                    phi_1900_ = _e1232;
                }
                let _e1236 = phi_1900_;
                phi_1901_ = _e1236;
            }
            let _e1241 = phi_1901_;
            phi_1903_ = _e1241;
            break;
        }
        case 0: {
            if (_e1158.x > 1f) {
                phi_4374_ = 0.9999999f;
            } else {
                phi_4374_ = select(_e1158.x, 0.00000011920929f, (_e1158.x < 0f));
            }
            let _e1215 = phi_4374_;
            phi_1903_ = _e1215;
            break;
        }
        default: {
            phi_1903_ = f32();
            break;
        }
    }
    let _e1257 = phi_1903_;
    switch bitcast<i32>(_e1206.member.member_1) {
        case 1: {
            let _e1295 = abs(_e1158.y);
            let _e1297 = (_e1295 % 1f);
            if (_e1295 >= 1f) {
                phi_4452_ = select(true, false, (_e1297 == 0f));
            } else {
                phi_4452_ = true;
            }
            let _e1301 = phi_4452_;
            let _e1302 = select(1f, _e1297, _e1301);
            if (select(-1f, 1f, (_e1158.y >= 0f)) > 0f) {
                phi_1924_ = _e1302;
            } else {
                phi_1924_ = (1f - _e1302);
            }
            let _e1306 = phi_1924_;
            phi_1961_ = _e1306;
            break;
        }
        case 2: {
            let _e1269 = abs(_e1158.y);
            let _e1276 = ((select(select(u32(_e1269), 0u, (_e1269 < 0f)), 4294967295u, (_e1269 > 4294967000f)) % 2u) == 0u);
            let _e1278 = (_e1269 % 1f);
            if (_e1269 >= 1f) {
                phi_4435_ = select(true, false, (_e1278 == 0f));
            } else {
                phi_4435_ = true;
            }
            let _e1282 = phi_4435_;
            let _e1283 = select(1f, _e1278, _e1282);
            if (select(-1f, 1f, (_e1158.y >= 0f)) > 0f) {
                if _e1276 {
                    phi_1953_ = _e1283;
                } else {
                    phi_1953_ = (1f - _e1283);
                }
                let _e1290 = phi_1953_;
                phi_1959_ = _e1290;
            } else {
                if _e1276 {
                    phi_1958_ = (1f - _e1283);
                } else {
                    phi_1958_ = _e1283;
                }
                let _e1287 = phi_1958_;
                phi_1959_ = _e1287;
            }
            let _e1292 = phi_1959_;
            phi_1961_ = _e1292;
            break;
        }
        case 0: {
            if (_e1158.y > 1f) {
                phi_4422_ = 0.9999999f;
            } else {
                phi_4422_ = select(_e1158.y, 0.00000011920929f, (_e1158.y < 0f));
            }
            let _e1266 = phi_4422_;
            phi_1961_ = _e1266;
            break;
        }
        default: {
            phi_1961_ = f32();
            break;
        }
    }
    let _e1308 = phi_1961_;
    let _e1312 = (_e1257 * f32(_e1206.member_2.x));
    let _e1321 = (_e1308 * f32(_e1206.member_2.y));
    let _e1339 = vec3<f32>((f32((select(select(u32(_e1312), 0u, (_e1312 < 0f)), 4294967295u, (_e1312 > 4294967000f)) + _e1206.member_1.x)) / _e555), (f32((select(select(u32(_e1321), 0u, (_e1321 < 0f)), 4294967295u, (_e1321 > 4294967000f)) + _e1206.member_1.y)) / _e556), f32(_e1206.member_3));
    let _e1345 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1339.x, _e1339.y), i32(_e1339.z), 0f);
    let _e1348 = select(_e1345, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e375.member_9 == 4294967295u)));
    if _e958 {
        phi_2055_ = vec3<f32>(0f, 0f, 0f);
        phi_2056_ = _e105;
    } else {
        let _e1352 = fma(_e960.x, 2f, -1f);
        let _e1353 = fma(_e960.y, 2f, -1f);
        let _e1354 = fma(_e960.z, 2f, -1f);
        let _e1359 = sqrt(fma(_e1354, _e1354, fma(_e1352, _e1352, (_e1353 * _e1353))));
        if (_e1359 == 0f) {
            phi_4510_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4510_ = (vec3<f32>(_e1352, _e1353, _e1354) * (1f / _e1359));
        }
        let _e1364 = phi_4510_;
        let _e1371 = sqrt(fma(_e106.z, _e106.z, fma(_e106.x, _e106.x, (_e106.y * _e106.y))));
        if (_e1371 == 0f) {
            phi_4545_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4545_ = (_e106 * (1f / _e1371));
        }
        let _e1376 = phi_4545_;
        let _e1383 = sqrt(fma(_e107.z, _e107.z, fma(_e107.x, _e107.x, (_e107.y * _e107.y))));
        if (_e1383 == 0f) {
            phi_4580_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4580_ = (_e107 * (1f / _e1383));
        }
        let _e1388 = phi_4580_;
        let _e1395 = sqrt(fma(_e105.z, _e105.z, fma(_e105.x, _e105.x, (_e105.y * _e105.y))));
        if (_e1395 == 0f) {
            phi_4615_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4615_ = (_e105 * (1f / _e1395));
        }
        let _e1400 = phi_4615_;
        let _e1419 = fma(_e1400.x, _e1364.z, fma(_e1376.x, _e1364.x, (_e1388.x * _e1364.y)));
        let _e1420 = fma(_e1400.y, _e1364.z, fma(_e1376.y, _e1364.x, (_e1388.y * _e1364.y)));
        let _e1421 = fma(_e1400.z, _e1364.z, fma(_e1376.z, _e1364.x, (_e1388.z * _e1364.y)));
        let _e1426 = sqrt(fma(_e1421, _e1421, fma(_e1419, _e1419, (_e1420 * _e1420))));
        if (_e1426 == 0f) {
            phi_4650_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4650_ = (vec3<f32>(_e1419, _e1420, _e1421) * (1f / _e1426));
        }
        let _e1431 = phi_4650_;
        phi_2055_ = _e1364;
        phi_2056_ = _e1431;
    }
    let _e1433 = phi_2055_;
    let _e1435 = phi_2056_;
    let _e1453 = (_e766.y * _e375.member_4);
    let _e1456 = (_e766.z * _e375.member_3);
    let _e1460 = fma(_e375.member_16, (select(_e1151, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e375.member_8 == 4294967295u))).x - 1f), 1f);
    let _e1475 = textureSampleLevel(global_12, global_13, _e1435, 0f);
    if (_e100 >= 86u) {
        phi_4682_ = (_e216 <= (_e100 - 86u));
    } else {
        phi_4682_ = false;
    }
    let _e1483 = phi_4682_;
    if _e1483 {
        let _e1486 = global.member[_e216];
        let _e1491 = global.member[(_e216 + 1u)];
        let _e1496 = global.member[(_e216 + 2u)];
        let _e1501 = global.member[(_e216 + 3u)];
        let _e1507 = global.member[(_e216 + 4u)];
        let _e1512 = global.member[(_e216 + 5u)];
        let _e1517 = global.member[(_e216 + 6u)];
        let _e1522 = global.member[(_e216 + 7u)];
        let _e1528 = global.member[(_e216 + 8u)];
        let _e1533 = global.member[(_e216 + 9u)];
        let _e1538 = global.member[(_e216 + 10u)];
        let _e1543 = global.member[(_e216 + 11u)];
        let _e1549 = global.member[(_e216 + 12u)];
        let _e1554 = global.member[(_e216 + 13u)];
        let _e1559 = global.member[(_e216 + 14u)];
        let _e1564 = global.member[(_e216 + 15u)];
        let _e1571 = global.member[(_e216 + 16u)];
        let _e1576 = global.member[(_e216 + 17u)];
        let _e1581 = global.member[(_e216 + 18u)];
        let _e1586 = global.member[(_e216 + 19u)];
        let _e1592 = global.member[(_e216 + 20u)];
        let _e1597 = global.member[(_e216 + 21u)];
        let _e1602 = global.member[(_e216 + 22u)];
        let _e1607 = global.member[(_e216 + 23u)];
        let _e1613 = global.member[(_e216 + 24u)];
        let _e1618 = global.member[(_e216 + 25u)];
        let _e1623 = global.member[(_e216 + 26u)];
        let _e1628 = global.member[(_e216 + 27u)];
        let _e1634 = global.member[(_e216 + 28u)];
        let _e1639 = global.member[(_e216 + 29u)];
        let _e1644 = global.member[(_e216 + 30u)];
        let _e1649 = global.member[(_e216 + 31u)];
        let _e1656 = global.member[(_e216 + 32u)];
        let _e1661 = global.member[(_e216 + 33u)];
        let _e1666 = global.member[(_e216 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_2263_ = type_24(0u, 6u);
        loop {
            let _e1671 = phi_2263_;
            if (_e1671.member < _e1671.member_1) {
                phi_2264_ = type_24((_e1671.member + 1u), _e1671.member_1);
                phi_2287_ = type_24(1u, _e1671.member);
            } else {
                phi_2264_ = _e1671;
                phi_2287_ = type_24(0u, type_24().member_1);
            }
            let _e1684 = phi_2264_;
            let _e1686 = phi_2287_;
            switch bitcast<i32>(_e1686.member) {
                case 0: {
                    phi_2314_ = false;
                    break;
                }
                case 1: {
                    let _e1691 = ((_e216 + 35u) + (_e1686.member_1 * 4u));
                    let _e1694 = global.member[_e1691];
                    let _e1699 = global.member[(_e1691 + 1u)];
                    let _e1704 = global.member[(_e1691 + 2u)];
                    let _e1709 = global.member[(_e1691 + 3u)];
                    local_1[_e1686.member_1] = vec4<f32>(bitcast<f32>(_e1694), bitcast<f32>(_e1699), bitcast<f32>(_e1704), bitcast<f32>(_e1709));
                    phi_2314_ = true;
                    break;
                }
                default: {
                    phi_2314_ = bool();
                    break;
                }
            }
            let _e1714 = phi_2314_;
            continue;
            continuing {
                phi_2263_ = _e1684;
                break if !(_e1714);
            }
        }
        let _e1716 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2320_ = type_24(0u, 8u);
        loop {
            let _e1719 = phi_2320_;
            if (_e1719.member < _e1719.member_1) {
                phi_2321_ = type_24((_e1719.member + 1u), _e1719.member_1);
                phi_2344_ = type_24(1u, _e1719.member);
            } else {
                phi_2321_ = _e1719;
                phi_2344_ = type_24(0u, type_24().member_1);
            }
            let _e1732 = phi_2321_;
            let _e1734 = phi_2344_;
            switch bitcast<i32>(_e1734.member) {
                case 0: {
                    phi_2367_ = false;
                    break;
                }
                case 1: {
                    let _e1739 = ((_e216 + 59u) + (_e1734.member_1 * 3u));
                    let _e1742 = global.member[_e1739];
                    let _e1747 = global.member[(_e1739 + 1u)];
                    let _e1752 = global.member[(_e1739 + 2u)];
                    local[_e1734.member_1] = vec3<f32>(bitcast<f32>(_e1742), bitcast<f32>(_e1747), bitcast<f32>(_e1752));
                    phi_2367_ = true;
                    break;
                }
                default: {
                    phi_2367_ = bool();
                    break;
                }
            }
            let _e1757 = phi_2367_;
            continue;
            continuing {
                phi_2320_ = _e1732;
                break if !(_e1757);
            }
        }
        let _e1759 = local;
        let _e1763 = global.member[(_e216 + 83u)];
        let _e1768 = global.member[(_e216 + 84u)];
        let _e1773 = global.member[(_e216 + 85u)];
        phi_2388_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1486), bitcast<f32>(_e1491), bitcast<f32>(_e1496), bitcast<f32>(_e1501)), vec4<f32>(bitcast<f32>(_e1507), bitcast<f32>(_e1512), bitcast<f32>(_e1517), bitcast<f32>(_e1522)), vec4<f32>(bitcast<f32>(_e1528), bitcast<f32>(_e1533), bitcast<f32>(_e1538), bitcast<f32>(_e1543)), vec4<f32>(bitcast<f32>(_e1549), bitcast<f32>(_e1554), bitcast<f32>(_e1559), bitcast<f32>(_e1564))), type_20(vec4<f32>(bitcast<f32>(_e1571), bitcast<f32>(_e1576), bitcast<f32>(_e1581), bitcast<f32>(_e1586)), vec4<f32>(bitcast<f32>(_e1592), bitcast<f32>(_e1597), bitcast<f32>(_e1602), bitcast<f32>(_e1607)), vec4<f32>(bitcast<f32>(_e1613), bitcast<f32>(_e1618), bitcast<f32>(_e1623), bitcast<f32>(_e1628)), vec4<f32>(bitcast<f32>(_e1634), bitcast<f32>(_e1639), bitcast<f32>(_e1644), bitcast<f32>(_e1649))), vec3<f32>(bitcast<f32>(_e1656), bitcast<f32>(_e1661), bitcast<f32>(_e1666)), type_21(_e1759, _e1716, vec3<f32>(bitcast<f32>(_e1763), bitcast<f32>(_e1768), bitcast<f32>(_e1773))));
    } else {
        phi_2388_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e1779 = phi_2388_;
    let _e1781 = (_e1779.member_2 - _e108);
    let _e1788 = sqrt(fma(_e1781.z, _e1781.z, fma(_e1781.x, _e1781.x, (_e1781.y * _e1781.y))));
    let _e1789 = (_e1788 == 0f);
    if _e1789 {
        phi_4754_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4754_ = (_e1781 * (1f / _e1788));
    }
    let _e1793 = phi_4754_;
    let _e1794 = -(_e1793);
    let _e1801 = sqrt(fma(_e1435.z, _e1435.z, fma(_e1435.x, _e1435.x, (_e1435.y * _e1435.y))));
    let _e1802 = (_e1801 == 0f);
    if _e1802 {
        phi_4813_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4813_ = (_e1435 * (1f / _e1801));
    }
    let _e1806 = phi_4813_;
    let _e1816 = (2f * fma(_e1806.z, _e1794.z, fma(_e1806.x, _e1794.x, (_e1806.y * _e1794.y))));
    let _e1823 = textureSampleLevel(global_14, global_15, (_e1794 - vec3<f32>((_e1816 * _e1806.x), (_e1816 * _e1806.y), (_e1816 * _e1806.z))), (_e1453 * 4f));
    if _e1789 {
        phi_4887_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4887_ = (_e1781 * (1f / _e1788));
    }
    let _e1830 = phi_4887_;
    let _e1839 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1435.z, _e1830.z, fma(_e1435.x, _e1830.x, (_e1435.y * _e1830.y))), 0f), _e1453), 0f);
    switch bitcast<i32>(_e238) {
        case 0: {
            if _e375.member_15 {
                let _e2012 = textureSampleLevel(global_18, global_19, vec2<f32>(fma(((bitcast<f32>(_e174) + fma(bitcast<f32>(_e154), _e108.z, fma(bitcast<f32>(_e134), _e108.y, (bitcast<f32>(_e114) * _e108.x)))) / _e209), 0.5f, 0.5f), fma(((bitcast<f32>(_e179) + fma(bitcast<f32>(_e159), _e108.z, fma(bitcast<f32>(_e139), _e108.y, (bitcast<f32>(_e119) * _e108.x)))) / _e209), -0.5f, 0.5f)), i32(0f));
                let _e2016 = select(0f, 1f, (((bitcast<f32>(_e184) + fma(bitcast<f32>(_e164), _e108.z, fma(bitcast<f32>(_e144), _e108.y, (bitcast<f32>(_e124) * _e108.x)))) / _e209) > vec4(_e2012).x));
                phi_3164_ = vec4<f32>(_e2016, _e2016, _e2016, 1f);
            } else {
                phi_3164_ = (vec4<f32>((_e102.x * _e572.x), (_e102.y * _e572.y), (_e102.z * _e572.z), (_e102.w * _e572.w)) * _e375.member_2);
            }
            let _e2019 = phi_3164_;
            global_20 = _e2019;
            break;
        }
        case 1: {
            let _e1981 = sqrt(fma(_e103.x, _e103.x, (_e103.y * _e103.y)));
            if (_e1981 == 0f) {
                phi_5241_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5241_ = (vec3<f32>(_e103.x, _e103.y, 0f) * (1f / _e1981));
            }
            let _e1986 = phi_5241_;
            global_20 = vec4<f32>(((_e1986.x + 1f) * 0.5f), ((_e1986.y + 1f) * 0.5f), ((_e1986.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1960 = sqrt(fma(_e104.x, _e104.x, (_e104.y * _e104.y)));
            if (_e1960 == 0f) {
                phi_5192_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5192_ = (vec3<f32>(_e104.x, _e104.y, 0f) * (1f / _e1960));
            }
            let _e1965 = phi_5192_;
            global_20 = vec4<f32>(((_e1965.x + 1f) * 0.5f), ((_e1965.y + 1f) * 0.5f), ((_e1965.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1802 {
                phi_5143_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5143_ = (_e1435 * (1f / _e1801));
            }
            let _e1944 = phi_5143_;
            global_20 = vec4<f32>(((_e1944.x + 1f) * 0.5f), ((_e1944.y + 1f) * 0.5f), ((_e1944.z + 1f) * 0.5f), 1f);
            break;
        }
        case 4: {
            global_20 = _e102;
            break;
        }
        case 5: {
            let _e1925 = sqrt(fma(_e105.z, _e105.z, fma(_e105.x, _e105.x, (_e105.y * _e105.y))));
            if (_e1925 == 0f) {
                phi_5094_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5094_ = (_e105 * (1f / _e1925));
            }
            let _e1930 = phi_5094_;
            global_20 = vec4<f32>(((_e1930.x + 1f) * 0.5f), ((_e1930.y + 1f) * 0.5f), ((_e1930.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1903 = sqrt(fma(_e1433.z, _e1433.z, fma(_e1433.x, _e1433.x, (_e1433.y * _e1433.y))));
            if (_e1903 == 0f) {
                phi_5045_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5045_ = (_e1433 * (1f / _e1903));
            }
            let _e1908 = phi_5045_;
            global_20 = vec4<f32>(((_e1908.x + 1f) * 0.5f), ((_e1908.y + 1f) * 0.5f), ((_e1908.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1881 = sqrt(fma(_e106.z, _e106.z, fma(_e106.x, _e106.x, (_e106.y * _e106.y))));
            if (_e1881 == 0f) {
                phi_4996_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_4996_ = (_e106 * (1f / _e1881));
            }
            let _e1886 = phi_4996_;
            global_20 = vec4<f32>(((_e1886.x + 1f) * 0.5f), ((_e1886.y + 1f) * 0.5f), ((_e1886.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1859 = sqrt(fma(_e107.z, _e107.z, fma(_e107.x, _e107.x, (_e107.y * _e107.y))));
            if (_e1859 == 0f) {
                phi_4947_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_4947_ = (_e107 * (1f / _e1859));
            }
            let _e1864 = phi_4947_;
            global_20 = vec4<f32>(((_e1864.x + 1f) * 0.5f), ((_e1864.y + 1f) * 0.5f), ((_e1864.z + 1f) * 0.5f), 1f);
            break;
        }
        case 9: {
            global_20 = vec4<f32>(_e1475.x, _e1475.y, _e1475.z, 1f);
            break;
        }
        case 10: {
            global_20 = vec4<f32>(_e1823.x, _e1823.y, _e1823.z, 1f);
            break;
        }
        case 11: {
            global_20 = vec4<f32>(_e1839.x, _e1839.y, 1f, 1f);
            break;
        }
        case 12: {
            global_20 = (vec4<f32>((_e572.x * _e375.member_2.x), (_e572.y * _e375.member_2.y), (_e572.z * _e375.member_2.z), (_e572.w * _e375.member_2.w)) * _e102);
            break;
        }
        case 13: {
            global_20 = vec4<f32>(_e1453, _e1453, _e1453, 1f);
            break;
        }
        case 14: {
            global_20 = vec4<f32>(_e1456, _e1456, _e1456, 1f);
            break;
        }
        case 15: {
            global_20 = vec4<f32>(_e1460, _e1460, _e1460, 1f);
            break;
        }
        case 16: {
            global_20 = vec4<f32>(((_e1348.x * _e375.member.x) * _e375.member_1), ((_e1348.y * _e375.member.y) * _e375.member_1), ((_e1348.z * _e375.member.z) * _e375.member_1), 1f);
            break;
        }
        case 17: {
            global_20 = vec4<f32>(_e1348.x, _e1348.y, _e1348.z, 1f);
            break;
        }
        case 18: {
            global_20 = vec4<f32>(_e375.member.x, _e375.member.y, _e375.member.z, 1f);
            break;
        }
        case 19: {
            global_20 = vec4<f32>(_e375.member_1, _e375.member_1, _e375.member_1, 1f);
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
