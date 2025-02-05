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

struct type_28 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_30 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_32 {
    member: type_24,
    member_1: vec2<u32>,
    member_2: vec2<u32>,
    member_3: u32,
    member_4: u32,
}

struct type_34 {
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
    var phi_668_: u32;
    var phi_4117_: bool;
    var phi_822_: type_34;
    var phi_826_: type_34;
    var phi_4154_: bool;
    var phi_866_: u32;
    var phi_875_: u32;
    var phi_888_: type_32;
    var phi_4176_: f32;
    var phi_4189_: bool;
    var phi_942_: f32;
    var phi_937_: f32;
    var phi_943_: f32;
    var phi_4206_: bool;
    var phi_908_: f32;
    var phi_945_: f32;
    var phi_4224_: f32;
    var phi_4237_: bool;
    var phi_1000_: f32;
    var phi_995_: f32;
    var phi_1001_: f32;
    var phi_4254_: bool;
    var phi_966_: f32;
    var phi_1003_: f32;
    var phi_4290_: bool;
    var phi_1086_: u32;
    var phi_1095_: u32;
    var phi_1108_: type_32;
    var phi_4311_: f32;
    var phi_4324_: bool;
    var phi_1162_: f32;
    var phi_1157_: f32;
    var phi_1163_: f32;
    var phi_4341_: bool;
    var phi_1128_: f32;
    var phi_1165_: f32;
    var phi_4359_: f32;
    var phi_4372_: bool;
    var phi_1220_: f32;
    var phi_1215_: f32;
    var phi_1221_: f32;
    var phi_4389_: bool;
    var phi_1186_: f32;
    var phi_1223_: f32;
    var phi_4425_: bool;
    var phi_1306_: u32;
    var phi_1315_: u32;
    var phi_1328_: type_32;
    var phi_4446_: f32;
    var phi_4459_: bool;
    var phi_1382_: f32;
    var phi_1377_: f32;
    var phi_1383_: f32;
    var phi_4476_: bool;
    var phi_1348_: f32;
    var phi_1385_: f32;
    var phi_4494_: f32;
    var phi_4507_: bool;
    var phi_1440_: f32;
    var phi_1435_: f32;
    var phi_1441_: f32;
    var phi_4524_: bool;
    var phi_1406_: f32;
    var phi_1443_: f32;
    var phi_4560_: bool;
    var phi_1526_: u32;
    var phi_1535_: u32;
    var phi_1548_: type_32;
    var phi_4581_: f32;
    var phi_4594_: bool;
    var phi_1602_: f32;
    var phi_1597_: f32;
    var phi_1603_: f32;
    var phi_4611_: bool;
    var phi_1568_: f32;
    var phi_1605_: f32;
    var phi_4629_: f32;
    var phi_4642_: bool;
    var phi_1660_: f32;
    var phi_1655_: f32;
    var phi_1661_: f32;
    var phi_4659_: bool;
    var phi_1626_: f32;
    var phi_1663_: f32;
    var phi_4695_: bool;
    var phi_1746_: u32;
    var phi_1755_: u32;
    var phi_1768_: type_32;
    var phi_4716_: f32;
    var phi_4729_: bool;
    var phi_1822_: f32;
    var phi_1817_: f32;
    var phi_1823_: f32;
    var phi_4746_: bool;
    var phi_1788_: f32;
    var phi_1825_: f32;
    var phi_4764_: f32;
    var phi_4777_: bool;
    var phi_1880_: f32;
    var phi_1875_: f32;
    var phi_1881_: f32;
    var phi_4794_: bool;
    var phi_1846_: f32;
    var phi_1883_: f32;
    var phi_4852_: vec3<f32>;
    var phi_4887_: vec3<f32>;
    var phi_4922_: vec3<f32>;
    var phi_4957_: vec3<f32>;
    var phi_4992_: vec3<f32>;
    var phi_1977_: vec3<f32>;
    var phi_1978_: vec3<f32>;
    var phi_5024_: bool;
    var phi_2185_: type_24;
    var phi_2186_: type_24;
    var phi_2209_: type_24;
    var phi_2236_: bool;
    var phi_2242_: type_24;
    var phi_2243_: type_24;
    var phi_2266_: type_24;
    var phi_2289_: bool;
    var phi_2310_: type_22;
    var phi_5096_: vec3<f32>;
    var phi_5155_: vec3<f32>;
    var phi_5229_: vec3<f32>;
    var phi_5289_: vec3<f32>;
    var phi_5338_: vec3<f32>;
    var phi_5387_: vec3<f32>;
    var phi_5436_: vec3<f32>;
    var phi_5485_: vec3<f32>;
    var phi_5534_: vec3<f32>;
    var phi_5583_: vec3<f32>;
    var phi_5622_: vec3<f32>;
    var phi_5657_: vec3<f32>;
    var phi_2350_: type_24;
    var phi_2353_: vec3<f32>;
    var phi_2351_: type_24;
    var phi_2376_: type_24;
    var phi_5674_: u32;
    var phi_5693_: bool;
    var phi_2393_: u32;
    var phi_5725_: bool;
    var phi_2410_: u32;
    var phi_2424_: type_28;
    var phi_5757_: bool;
    var phi_2474_: type_30;
    var phi_5837_: bool;
    var phi_3138_: type_36;
    var phi_5887_: vec3<f32>;
    var phi_5922_: vec3<f32>;
    var phi_5957_: vec3<f32>;
    var phi_3393_: vec3<f32>;
    var phi_6049_: bool;
    var phi_2885_: type_35;
    var phi_6096_: vec3<f32>;
    var phi_6131_: vec3<f32>;
    var phi_3075_: vec3<f32>;
    var phi_6223_: bool;
    var phi_2522_: type_35;
    var phi_6270_: vec3<f32>;
    var phi_6305_: vec3<f32>;
    var phi_2831_: f32;
    var phi_3395_: vec3<f32>;
    var phi_3396_: bool;
    var phi_3405_: vec3<f32>;
    var phi_2354_: vec3<f32>;
    var phi_3407_: bool;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var phi_3524_: vec4<f32>;

    let _e117 = arrayLength((&global.member));
    let _e118 = global_2;
    let _e119 = global_3;
    let _e120 = global_4;
    let _e121 = global_5;
    let _e122 = global_6;
    let _e123 = global_7;
    let _e124 = global_8;
    let _e125 = global_9;
    let _e129 = global.member[(_e118 + 9u)];
    let _e133 = global.member[(_e118 + 11u)];
    let _e137 = global.member[(_e118 + 17u)];
    let _e140 = global.member[_e137];
    let _e144 = global.member[(_e137 + 1u)];
    let _e148 = global.member[(_e137 + 4u)];
    switch bitcast<i32>(_e148) {
        case 0: {
            phi_668_ = 0u;
            break;
        }
        case 1: {
            phi_668_ = 1u;
            break;
        }
        case 2: {
            phi_668_ = 2u;
            break;
        }
        case 3: {
            phi_668_ = 3u;
            break;
        }
        case 4: {
            phi_668_ = 4u;
            break;
        }
        case 5: {
            phi_668_ = 5u;
            break;
        }
        case 6: {
            phi_668_ = 6u;
            break;
        }
        case 7: {
            phi_668_ = 7u;
            break;
        }
        case 8: {
            phi_668_ = 8u;
            break;
        }
        case 9: {
            phi_668_ = 9u;
            break;
        }
        case 10: {
            phi_668_ = 10u;
            break;
        }
        case 11: {
            phi_668_ = 11u;
            break;
        }
        case 12: {
            phi_668_ = 12u;
            break;
        }
        case 13: {
            phi_668_ = 13u;
            break;
        }
        case 14: {
            phi_668_ = 14u;
            break;
        }
        case 15: {
            phi_668_ = 15u;
            break;
        }
        case 16: {
            phi_668_ = 16u;
            break;
        }
        case 17: {
            phi_668_ = 17u;
            break;
        }
        case 18: {
            phi_668_ = 18u;
            break;
        }
        case 19: {
            phi_668_ = 19u;
            break;
        }
        default: {
            phi_668_ = 0u;
            break;
        }
    }
    let _e151 = phi_668_;
    let _e155 = global.member[(_e137 + 5u)];
    let _e160 = global.member[(_e137 + 9u)];
    let _e164 = global.member[(_e137 + 10u)];
    if (_e133 == 4294967295u) {
        phi_826_ = type_34(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e117 >= 22u) {
            phi_4117_ = (_e133 <= (_e117 - 22u));
        } else {
            phi_4117_ = false;
        }
        let _e170 = phi_4117_;
        if _e170 {
            let _e173 = global.member[_e133];
            let _e178 = global.member[(_e133 + 1u)];
            let _e183 = global.member[(_e133 + 2u)];
            let _e189 = global.member[(_e133 + 3u)];
            let _e194 = global.member[(_e133 + 4u)];
            let _e199 = global.member[(_e133 + 5u)];
            let _e204 = global.member[(_e133 + 6u)];
            let _e209 = global.member[(_e133 + 7u)];
            let _e215 = global.member[(_e133 + 8u)];
            let _e220 = global.member[(_e133 + 9u)];
            let _e225 = global.member[(_e133 + 10u)];
            let _e229 = global.member[(_e133 + 11u)];
            let _e233 = global.member[(_e133 + 12u)];
            let _e237 = global.member[(_e133 + 13u)];
            let _e241 = global.member[(_e133 + 14u)];
            let _e245 = global.member[(_e133 + 15u)];
            let _e249 = global.member[(_e133 + 16u)];
            let _e253 = global.member[(_e133 + 17u)];
            let _e257 = global.member[(_e133 + 18u)];
            let _e261 = global.member[(_e133 + 19u)];
            let _e265 = global.member[(_e133 + 20u)];
            let _e270 = global.member[(_e133 + 21u)];
            phi_822_ = type_34(vec3<f32>(bitcast<f32>(_e173), bitcast<f32>(_e178), bitcast<f32>(_e183)), bitcast<f32>(_e189), vec4<f32>(bitcast<f32>(_e194), bitcast<f32>(_e199), bitcast<f32>(_e204), bitcast<f32>(_e209)), bitcast<f32>(_e215), bitcast<f32>(_e220), _e225, _e229, _e233, _e237, _e241, _e245, _e249, _e253, _e257, _e261, (_e265 == 1u), bitcast<f32>(_e270));
        } else {
            phi_822_ = type_34(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e274 = phi_822_;
        phi_826_ = type_34(_e274.member, _e274.member_1, _e274.member_2, _e274.member_3, _e274.member_4, _e274.member_5, _e274.member_6, _e274.member_7, _e274.member_8, _e274.member_9, _e274.member_10, _e274.member_11, _e274.member_12, _e274.member_13, _e274.member_14, (_e274.member_15 && (_e155 == 1u)), _e274.member_16);
    }
    let _e296 = phi_826_;
    let _e300 = select(_e121, _e120, vec2((_e296.member_10 == 0u)));
    let _e302 = (_e117 >= 8u);
    if _e302 {
        phi_4154_ = (_e296.member_5 <= (_e117 - 8u));
    } else {
        phi_4154_ = false;
    }
    let _e306 = phi_4154_;
    if _e306 {
        let _e309 = global.member[_e296.member_5];
        let _e313 = global.member[(_e296.member_5 + 1u)];
        let _e318 = global.member[(_e296.member_5 + 2u)];
        let _e322 = global.member[(_e296.member_5 + 3u)];
        let _e327 = global.member[(_e296.member_5 + 4u)];
        let _e331 = global.member[(_e296.member_5 + 5u)];
        let _e335 = global.member[(_e296.member_5 + 6u)];
        switch bitcast<i32>(_e335) {
            case 0: {
                phi_866_ = 0u;
                break;
            }
            case 1: {
                phi_866_ = 1u;
                break;
            }
            case 2: {
                phi_866_ = 2u;
                break;
            }
            default: {
                phi_866_ = 0u;
                break;
            }
        }
        let _e338 = phi_866_;
        let _e342 = global.member[(_e296.member_5 + 7u)];
        switch bitcast<i32>(_e342) {
            case 0: {
                phi_875_ = 0u;
                break;
            }
            case 1: {
                phi_875_ = 1u;
                break;
            }
            case 2: {
                phi_875_ = 2u;
                break;
            }
            default: {
                phi_875_ = 0u;
                break;
            }
        }
        let _e345 = phi_875_;
        phi_888_ = type_32(type_24(_e338, _e345), vec2<u32>(_e309, _e313), vec2<u32>(_e318, _e322), _e327, _e331);
    } else {
        phi_888_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e349 = phi_888_;
    switch bitcast<i32>(_e349.member.member) {
        case 1: {
            let _e387 = abs(_e300.x);
            let _e389 = (_e387 % 1f);
            if (_e387 >= 1f) {
                phi_4206_ = select(true, false, (_e389 == 0f));
            } else {
                phi_4206_ = true;
            }
            let _e393 = phi_4206_;
            let _e394 = select(1f, _e389, _e393);
            if (select(-1f, 1f, (_e300.x >= 0f)) > 0f) {
                phi_908_ = _e394;
            } else {
                phi_908_ = (1f - _e394);
            }
            let _e398 = phi_908_;
            phi_945_ = _e398;
            break;
        }
        case 2: {
            let _e361 = abs(_e300.x);
            let _e368 = ((select(select(u32(_e361), 0u, (_e361 < 0f)), 4294967295u, (_e361 > 4294967000f)) % 2u) == 0u);
            let _e370 = (_e361 % 1f);
            if (_e361 >= 1f) {
                phi_4189_ = select(true, false, (_e370 == 0f));
            } else {
                phi_4189_ = true;
            }
            let _e374 = phi_4189_;
            let _e375 = select(1f, _e370, _e374);
            if (select(-1f, 1f, (_e300.x >= 0f)) > 0f) {
                if _e368 {
                    phi_937_ = _e375;
                } else {
                    phi_937_ = (1f - _e375);
                }
                let _e382 = phi_937_;
                phi_943_ = _e382;
            } else {
                if _e368 {
                    phi_942_ = (1f - _e375);
                } else {
                    phi_942_ = _e375;
                }
                let _e379 = phi_942_;
                phi_943_ = _e379;
            }
            let _e384 = phi_943_;
            phi_945_ = _e384;
            break;
        }
        case 0: {
            if (_e300.x > 1f) {
                phi_4176_ = 0.9999999f;
            } else {
                phi_4176_ = select(_e300.x, 0.00000011920929f, (_e300.x < 0f));
            }
            let _e358 = phi_4176_;
            phi_945_ = _e358;
            break;
        }
        default: {
            phi_945_ = f32();
            break;
        }
    }
    let _e400 = phi_945_;
    switch bitcast<i32>(_e349.member.member_1) {
        case 1: {
            let _e438 = abs(_e300.y);
            let _e440 = (_e438 % 1f);
            if (_e438 >= 1f) {
                phi_4254_ = select(true, false, (_e440 == 0f));
            } else {
                phi_4254_ = true;
            }
            let _e444 = phi_4254_;
            let _e445 = select(1f, _e440, _e444);
            if (select(-1f, 1f, (_e300.y >= 0f)) > 0f) {
                phi_966_ = _e445;
            } else {
                phi_966_ = (1f - _e445);
            }
            let _e449 = phi_966_;
            phi_1003_ = _e449;
            break;
        }
        case 2: {
            let _e412 = abs(_e300.y);
            let _e419 = ((select(select(u32(_e412), 0u, (_e412 < 0f)), 4294967295u, (_e412 > 4294967000f)) % 2u) == 0u);
            let _e421 = (_e412 % 1f);
            if (_e412 >= 1f) {
                phi_4237_ = select(true, false, (_e421 == 0f));
            } else {
                phi_4237_ = true;
            }
            let _e425 = phi_4237_;
            let _e426 = select(1f, _e421, _e425);
            if (select(-1f, 1f, (_e300.y >= 0f)) > 0f) {
                if _e419 {
                    phi_995_ = _e426;
                } else {
                    phi_995_ = (1f - _e426);
                }
                let _e433 = phi_995_;
                phi_1001_ = _e433;
            } else {
                if _e419 {
                    phi_1000_ = (1f - _e426);
                } else {
                    phi_1000_ = _e426;
                }
                let _e430 = phi_1000_;
                phi_1001_ = _e430;
            }
            let _e435 = phi_1001_;
            phi_1003_ = _e435;
            break;
        }
        case 0: {
            if (_e300.y > 1f) {
                phi_4224_ = 0.9999999f;
            } else {
                phi_4224_ = select(_e300.y, 0.00000011920929f, (_e300.y < 0f));
            }
            let _e409 = phi_4224_;
            phi_1003_ = _e409;
            break;
        }
        default: {
            phi_1003_ = f32();
            break;
        }
    }
    let _e451 = phi_1003_;
    let _e455 = (_e400 * f32(_e349.member_2.x));
    let _e464 = (_e451 * f32(_e349.member_2.y));
    let _e476 = f32(_e140);
    let _e477 = f32(_e144);
    let _e484 = vec3<f32>((f32((select(select(u32(_e455), 0u, (_e455 < 0f)), 4294967295u, (_e455 > 4294967000f)) + _e349.member_1.x)) / _e476), (f32((select(select(u32(_e464), 0u, (_e464 < 0f)), 4294967295u, (_e464 > 4294967000f)) + _e349.member_1.y)) / _e477), f32(_e349.member_3));
    let _e490 = textureSampleLevel(global_11, global_10, vec2<f32>(_e484.x, _e484.y), i32(_e484.z), 0f);
    let _e493 = select(_e490, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e296.member_5 == 4294967295u)));
    let _e497 = select(_e121, _e120, vec2((_e296.member_11 == 0u)));
    if _e302 {
        phi_4290_ = (_e296.member_6 <= (_e117 - 8u));
    } else {
        phi_4290_ = false;
    }
    let _e502 = phi_4290_;
    if _e502 {
        let _e505 = global.member[_e296.member_6];
        let _e509 = global.member[(_e296.member_6 + 1u)];
        let _e514 = global.member[(_e296.member_6 + 2u)];
        let _e518 = global.member[(_e296.member_6 + 3u)];
        let _e523 = global.member[(_e296.member_6 + 4u)];
        let _e527 = global.member[(_e296.member_6 + 5u)];
        let _e531 = global.member[(_e296.member_6 + 6u)];
        switch bitcast<i32>(_e531) {
            case 0: {
                phi_1086_ = 0u;
                break;
            }
            case 1: {
                phi_1086_ = 1u;
                break;
            }
            case 2: {
                phi_1086_ = 2u;
                break;
            }
            default: {
                phi_1086_ = 0u;
                break;
            }
        }
        let _e534 = phi_1086_;
        let _e538 = global.member[(_e296.member_6 + 7u)];
        switch bitcast<i32>(_e538) {
            case 0: {
                phi_1095_ = 0u;
                break;
            }
            case 1: {
                phi_1095_ = 1u;
                break;
            }
            case 2: {
                phi_1095_ = 2u;
                break;
            }
            default: {
                phi_1095_ = 0u;
                break;
            }
        }
        let _e541 = phi_1095_;
        phi_1108_ = type_32(type_24(_e534, _e541), vec2<u32>(_e505, _e509), vec2<u32>(_e514, _e518), _e523, _e527);
    } else {
        phi_1108_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e545 = phi_1108_;
    switch bitcast<i32>(_e545.member.member) {
        case 1: {
            let _e583 = abs(_e497.x);
            let _e585 = (_e583 % 1f);
            if (_e583 >= 1f) {
                phi_4341_ = select(true, false, (_e585 == 0f));
            } else {
                phi_4341_ = true;
            }
            let _e589 = phi_4341_;
            let _e590 = select(1f, _e585, _e589);
            if (select(-1f, 1f, (_e497.x >= 0f)) > 0f) {
                phi_1128_ = _e590;
            } else {
                phi_1128_ = (1f - _e590);
            }
            let _e594 = phi_1128_;
            phi_1165_ = _e594;
            break;
        }
        case 2: {
            let _e557 = abs(_e497.x);
            let _e564 = ((select(select(u32(_e557), 0u, (_e557 < 0f)), 4294967295u, (_e557 > 4294967000f)) % 2u) == 0u);
            let _e566 = (_e557 % 1f);
            if (_e557 >= 1f) {
                phi_4324_ = select(true, false, (_e566 == 0f));
            } else {
                phi_4324_ = true;
            }
            let _e570 = phi_4324_;
            let _e571 = select(1f, _e566, _e570);
            if (select(-1f, 1f, (_e497.x >= 0f)) > 0f) {
                if _e564 {
                    phi_1157_ = _e571;
                } else {
                    phi_1157_ = (1f - _e571);
                }
                let _e578 = phi_1157_;
                phi_1163_ = _e578;
            } else {
                if _e564 {
                    phi_1162_ = (1f - _e571);
                } else {
                    phi_1162_ = _e571;
                }
                let _e575 = phi_1162_;
                phi_1163_ = _e575;
            }
            let _e580 = phi_1163_;
            phi_1165_ = _e580;
            break;
        }
        case 0: {
            if (_e497.x > 1f) {
                phi_4311_ = 0.9999999f;
            } else {
                phi_4311_ = select(_e497.x, 0.00000011920929f, (_e497.x < 0f));
            }
            let _e554 = phi_4311_;
            phi_1165_ = _e554;
            break;
        }
        default: {
            phi_1165_ = f32();
            break;
        }
    }
    let _e596 = phi_1165_;
    switch bitcast<i32>(_e545.member.member_1) {
        case 1: {
            let _e634 = abs(_e497.y);
            let _e636 = (_e634 % 1f);
            if (_e634 >= 1f) {
                phi_4389_ = select(true, false, (_e636 == 0f));
            } else {
                phi_4389_ = true;
            }
            let _e640 = phi_4389_;
            let _e641 = select(1f, _e636, _e640);
            if (select(-1f, 1f, (_e497.y >= 0f)) > 0f) {
                phi_1186_ = _e641;
            } else {
                phi_1186_ = (1f - _e641);
            }
            let _e645 = phi_1186_;
            phi_1223_ = _e645;
            break;
        }
        case 2: {
            let _e608 = abs(_e497.y);
            let _e615 = ((select(select(u32(_e608), 0u, (_e608 < 0f)), 4294967295u, (_e608 > 4294967000f)) % 2u) == 0u);
            let _e617 = (_e608 % 1f);
            if (_e608 >= 1f) {
                phi_4372_ = select(true, false, (_e617 == 0f));
            } else {
                phi_4372_ = true;
            }
            let _e621 = phi_4372_;
            let _e622 = select(1f, _e617, _e621);
            if (select(-1f, 1f, (_e497.y >= 0f)) > 0f) {
                if _e615 {
                    phi_1215_ = _e622;
                } else {
                    phi_1215_ = (1f - _e622);
                }
                let _e629 = phi_1215_;
                phi_1221_ = _e629;
            } else {
                if _e615 {
                    phi_1220_ = (1f - _e622);
                } else {
                    phi_1220_ = _e622;
                }
                let _e626 = phi_1220_;
                phi_1221_ = _e626;
            }
            let _e631 = phi_1221_;
            phi_1223_ = _e631;
            break;
        }
        case 0: {
            if (_e497.y > 1f) {
                phi_4359_ = 0.9999999f;
            } else {
                phi_4359_ = select(_e497.y, 0.00000011920929f, (_e497.y < 0f));
            }
            let _e605 = phi_4359_;
            phi_1223_ = _e605;
            break;
        }
        default: {
            phi_1223_ = f32();
            break;
        }
    }
    let _e647 = phi_1223_;
    let _e651 = (_e596 * f32(_e545.member_2.x));
    let _e660 = (_e647 * f32(_e545.member_2.y));
    let _e678 = vec3<f32>((f32((select(select(u32(_e651), 0u, (_e651 < 0f)), 4294967295u, (_e651 > 4294967000f)) + _e545.member_1.x)) / _e476), (f32((select(select(u32(_e660), 0u, (_e660 < 0f)), 4294967295u, (_e660 > 4294967000f)) + _e545.member_1.y)) / _e477), f32(_e545.member_3));
    let _e684 = textureSampleLevel(global_11, global_10, vec2<f32>(_e678.x, _e678.y), i32(_e678.z), 0f);
    let _e687 = select(_e684, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e296.member_6 == 4294967295u)));
    let _e691 = select(_e121, _e120, vec2((_e296.member_12 == 0u)));
    if _e302 {
        phi_4425_ = (_e296.member_7 <= (_e117 - 8u));
    } else {
        phi_4425_ = false;
    }
    let _e696 = phi_4425_;
    if _e696 {
        let _e699 = global.member[_e296.member_7];
        let _e703 = global.member[(_e296.member_7 + 1u)];
        let _e708 = global.member[(_e296.member_7 + 2u)];
        let _e712 = global.member[(_e296.member_7 + 3u)];
        let _e717 = global.member[(_e296.member_7 + 4u)];
        let _e721 = global.member[(_e296.member_7 + 5u)];
        let _e725 = global.member[(_e296.member_7 + 6u)];
        switch bitcast<i32>(_e725) {
            case 0: {
                phi_1306_ = 0u;
                break;
            }
            case 1: {
                phi_1306_ = 1u;
                break;
            }
            case 2: {
                phi_1306_ = 2u;
                break;
            }
            default: {
                phi_1306_ = 0u;
                break;
            }
        }
        let _e728 = phi_1306_;
        let _e732 = global.member[(_e296.member_7 + 7u)];
        switch bitcast<i32>(_e732) {
            case 0: {
                phi_1315_ = 0u;
                break;
            }
            case 1: {
                phi_1315_ = 1u;
                break;
            }
            case 2: {
                phi_1315_ = 2u;
                break;
            }
            default: {
                phi_1315_ = 0u;
                break;
            }
        }
        let _e735 = phi_1315_;
        phi_1328_ = type_32(type_24(_e728, _e735), vec2<u32>(_e699, _e703), vec2<u32>(_e708, _e712), _e717, _e721);
    } else {
        phi_1328_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e739 = phi_1328_;
    switch bitcast<i32>(_e739.member.member) {
        case 1: {
            let _e777 = abs(_e691.x);
            let _e779 = (_e777 % 1f);
            if (_e777 >= 1f) {
                phi_4476_ = select(true, false, (_e779 == 0f));
            } else {
                phi_4476_ = true;
            }
            let _e783 = phi_4476_;
            let _e784 = select(1f, _e779, _e783);
            if (select(-1f, 1f, (_e691.x >= 0f)) > 0f) {
                phi_1348_ = _e784;
            } else {
                phi_1348_ = (1f - _e784);
            }
            let _e788 = phi_1348_;
            phi_1385_ = _e788;
            break;
        }
        case 2: {
            let _e751 = abs(_e691.x);
            let _e758 = ((select(select(u32(_e751), 0u, (_e751 < 0f)), 4294967295u, (_e751 > 4294967000f)) % 2u) == 0u);
            let _e760 = (_e751 % 1f);
            if (_e751 >= 1f) {
                phi_4459_ = select(true, false, (_e760 == 0f));
            } else {
                phi_4459_ = true;
            }
            let _e764 = phi_4459_;
            let _e765 = select(1f, _e760, _e764);
            if (select(-1f, 1f, (_e691.x >= 0f)) > 0f) {
                if _e758 {
                    phi_1377_ = _e765;
                } else {
                    phi_1377_ = (1f - _e765);
                }
                let _e772 = phi_1377_;
                phi_1383_ = _e772;
            } else {
                if _e758 {
                    phi_1382_ = (1f - _e765);
                } else {
                    phi_1382_ = _e765;
                }
                let _e769 = phi_1382_;
                phi_1383_ = _e769;
            }
            let _e774 = phi_1383_;
            phi_1385_ = _e774;
            break;
        }
        case 0: {
            if (_e691.x > 1f) {
                phi_4446_ = 0.9999999f;
            } else {
                phi_4446_ = select(_e691.x, 0.00000011920929f, (_e691.x < 0f));
            }
            let _e748 = phi_4446_;
            phi_1385_ = _e748;
            break;
        }
        default: {
            phi_1385_ = f32();
            break;
        }
    }
    let _e790 = phi_1385_;
    switch bitcast<i32>(_e739.member.member_1) {
        case 1: {
            let _e828 = abs(_e691.y);
            let _e830 = (_e828 % 1f);
            if (_e828 >= 1f) {
                phi_4524_ = select(true, false, (_e830 == 0f));
            } else {
                phi_4524_ = true;
            }
            let _e834 = phi_4524_;
            let _e835 = select(1f, _e830, _e834);
            if (select(-1f, 1f, (_e691.y >= 0f)) > 0f) {
                phi_1406_ = _e835;
            } else {
                phi_1406_ = (1f - _e835);
            }
            let _e839 = phi_1406_;
            phi_1443_ = _e839;
            break;
        }
        case 2: {
            let _e802 = abs(_e691.y);
            let _e809 = ((select(select(u32(_e802), 0u, (_e802 < 0f)), 4294967295u, (_e802 > 4294967000f)) % 2u) == 0u);
            let _e811 = (_e802 % 1f);
            if (_e802 >= 1f) {
                phi_4507_ = select(true, false, (_e811 == 0f));
            } else {
                phi_4507_ = true;
            }
            let _e815 = phi_4507_;
            let _e816 = select(1f, _e811, _e815);
            if (select(-1f, 1f, (_e691.y >= 0f)) > 0f) {
                if _e809 {
                    phi_1435_ = _e816;
                } else {
                    phi_1435_ = (1f - _e816);
                }
                let _e823 = phi_1435_;
                phi_1441_ = _e823;
            } else {
                if _e809 {
                    phi_1440_ = (1f - _e816);
                } else {
                    phi_1440_ = _e816;
                }
                let _e820 = phi_1440_;
                phi_1441_ = _e820;
            }
            let _e825 = phi_1441_;
            phi_1443_ = _e825;
            break;
        }
        case 0: {
            if (_e691.y > 1f) {
                phi_4494_ = 0.9999999f;
            } else {
                phi_4494_ = select(_e691.y, 0.00000011920929f, (_e691.y < 0f));
            }
            let _e799 = phi_4494_;
            phi_1443_ = _e799;
            break;
        }
        default: {
            phi_1443_ = f32();
            break;
        }
    }
    let _e841 = phi_1443_;
    let _e845 = (_e790 * f32(_e739.member_2.x));
    let _e854 = (_e841 * f32(_e739.member_2.y));
    let _e872 = vec3<f32>((f32((select(select(u32(_e845), 0u, (_e845 < 0f)), 4294967295u, (_e845 > 4294967000f)) + _e739.member_1.x)) / _e476), (f32((select(select(u32(_e854), 0u, (_e854 < 0f)), 4294967295u, (_e854 > 4294967000f)) + _e739.member_1.y)) / _e477), f32(_e739.member_3));
    let _e878 = textureSampleLevel(global_11, global_10, vec2<f32>(_e872.x, _e872.y), i32(_e872.z), 0f);
    let _e879 = (_e296.member_7 == 4294967295u);
    let _e881 = select(_e878, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e879));
    let _e885 = select(_e121, _e120, vec2((_e296.member_13 == 0u)));
    if _e302 {
        phi_4560_ = (_e296.member_8 <= (_e117 - 8u));
    } else {
        phi_4560_ = false;
    }
    let _e890 = phi_4560_;
    if _e890 {
        let _e893 = global.member[_e296.member_8];
        let _e897 = global.member[(_e296.member_8 + 1u)];
        let _e902 = global.member[(_e296.member_8 + 2u)];
        let _e906 = global.member[(_e296.member_8 + 3u)];
        let _e911 = global.member[(_e296.member_8 + 4u)];
        let _e915 = global.member[(_e296.member_8 + 5u)];
        let _e919 = global.member[(_e296.member_8 + 6u)];
        switch bitcast<i32>(_e919) {
            case 0: {
                phi_1526_ = 0u;
                break;
            }
            case 1: {
                phi_1526_ = 1u;
                break;
            }
            case 2: {
                phi_1526_ = 2u;
                break;
            }
            default: {
                phi_1526_ = 0u;
                break;
            }
        }
        let _e922 = phi_1526_;
        let _e926 = global.member[(_e296.member_8 + 7u)];
        switch bitcast<i32>(_e926) {
            case 0: {
                phi_1535_ = 0u;
                break;
            }
            case 1: {
                phi_1535_ = 1u;
                break;
            }
            case 2: {
                phi_1535_ = 2u;
                break;
            }
            default: {
                phi_1535_ = 0u;
                break;
            }
        }
        let _e929 = phi_1535_;
        phi_1548_ = type_32(type_24(_e922, _e929), vec2<u32>(_e893, _e897), vec2<u32>(_e902, _e906), _e911, _e915);
    } else {
        phi_1548_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e933 = phi_1548_;
    switch bitcast<i32>(_e933.member.member) {
        case 1: {
            let _e971 = abs(_e885.x);
            let _e973 = (_e971 % 1f);
            if (_e971 >= 1f) {
                phi_4611_ = select(true, false, (_e973 == 0f));
            } else {
                phi_4611_ = true;
            }
            let _e977 = phi_4611_;
            let _e978 = select(1f, _e973, _e977);
            if (select(-1f, 1f, (_e885.x >= 0f)) > 0f) {
                phi_1568_ = _e978;
            } else {
                phi_1568_ = (1f - _e978);
            }
            let _e982 = phi_1568_;
            phi_1605_ = _e982;
            break;
        }
        case 2: {
            let _e945 = abs(_e885.x);
            let _e952 = ((select(select(u32(_e945), 0u, (_e945 < 0f)), 4294967295u, (_e945 > 4294967000f)) % 2u) == 0u);
            let _e954 = (_e945 % 1f);
            if (_e945 >= 1f) {
                phi_4594_ = select(true, false, (_e954 == 0f));
            } else {
                phi_4594_ = true;
            }
            let _e958 = phi_4594_;
            let _e959 = select(1f, _e954, _e958);
            if (select(-1f, 1f, (_e885.x >= 0f)) > 0f) {
                if _e952 {
                    phi_1597_ = _e959;
                } else {
                    phi_1597_ = (1f - _e959);
                }
                let _e966 = phi_1597_;
                phi_1603_ = _e966;
            } else {
                if _e952 {
                    phi_1602_ = (1f - _e959);
                } else {
                    phi_1602_ = _e959;
                }
                let _e963 = phi_1602_;
                phi_1603_ = _e963;
            }
            let _e968 = phi_1603_;
            phi_1605_ = _e968;
            break;
        }
        case 0: {
            if (_e885.x > 1f) {
                phi_4581_ = 0.9999999f;
            } else {
                phi_4581_ = select(_e885.x, 0.00000011920929f, (_e885.x < 0f));
            }
            let _e942 = phi_4581_;
            phi_1605_ = _e942;
            break;
        }
        default: {
            phi_1605_ = f32();
            break;
        }
    }
    let _e984 = phi_1605_;
    switch bitcast<i32>(_e933.member.member_1) {
        case 1: {
            let _e1022 = abs(_e885.y);
            let _e1024 = (_e1022 % 1f);
            if (_e1022 >= 1f) {
                phi_4659_ = select(true, false, (_e1024 == 0f));
            } else {
                phi_4659_ = true;
            }
            let _e1028 = phi_4659_;
            let _e1029 = select(1f, _e1024, _e1028);
            if (select(-1f, 1f, (_e885.y >= 0f)) > 0f) {
                phi_1626_ = _e1029;
            } else {
                phi_1626_ = (1f - _e1029);
            }
            let _e1033 = phi_1626_;
            phi_1663_ = _e1033;
            break;
        }
        case 2: {
            let _e996 = abs(_e885.y);
            let _e1003 = ((select(select(u32(_e996), 0u, (_e996 < 0f)), 4294967295u, (_e996 > 4294967000f)) % 2u) == 0u);
            let _e1005 = (_e996 % 1f);
            if (_e996 >= 1f) {
                phi_4642_ = select(true, false, (_e1005 == 0f));
            } else {
                phi_4642_ = true;
            }
            let _e1009 = phi_4642_;
            let _e1010 = select(1f, _e1005, _e1009);
            if (select(-1f, 1f, (_e885.y >= 0f)) > 0f) {
                if _e1003 {
                    phi_1655_ = _e1010;
                } else {
                    phi_1655_ = (1f - _e1010);
                }
                let _e1017 = phi_1655_;
                phi_1661_ = _e1017;
            } else {
                if _e1003 {
                    phi_1660_ = (1f - _e1010);
                } else {
                    phi_1660_ = _e1010;
                }
                let _e1014 = phi_1660_;
                phi_1661_ = _e1014;
            }
            let _e1019 = phi_1661_;
            phi_1663_ = _e1019;
            break;
        }
        case 0: {
            if (_e885.y > 1f) {
                phi_4629_ = 0.9999999f;
            } else {
                phi_4629_ = select(_e885.y, 0.00000011920929f, (_e885.y < 0f));
            }
            let _e993 = phi_4629_;
            phi_1663_ = _e993;
            break;
        }
        default: {
            phi_1663_ = f32();
            break;
        }
    }
    let _e1035 = phi_1663_;
    let _e1039 = (_e984 * f32(_e933.member_2.x));
    let _e1048 = (_e1035 * f32(_e933.member_2.y));
    let _e1066 = vec3<f32>((f32((select(select(u32(_e1039), 0u, (_e1039 < 0f)), 4294967295u, (_e1039 > 4294967000f)) + _e933.member_1.x)) / _e476), (f32((select(select(u32(_e1048), 0u, (_e1048 < 0f)), 4294967295u, (_e1048 > 4294967000f)) + _e933.member_1.y)) / _e477), f32(_e933.member_3));
    let _e1072 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1066.x, _e1066.y), i32(_e1066.z), 0f);
    let _e1079 = select(_e121, _e120, vec2((_e296.member_14 == 0u)));
    if _e302 {
        phi_4695_ = (_e296.member_9 <= (_e117 - 8u));
    } else {
        phi_4695_ = false;
    }
    let _e1084 = phi_4695_;
    if _e1084 {
        let _e1087 = global.member[_e296.member_9];
        let _e1091 = global.member[(_e296.member_9 + 1u)];
        let _e1096 = global.member[(_e296.member_9 + 2u)];
        let _e1100 = global.member[(_e296.member_9 + 3u)];
        let _e1105 = global.member[(_e296.member_9 + 4u)];
        let _e1109 = global.member[(_e296.member_9 + 5u)];
        let _e1113 = global.member[(_e296.member_9 + 6u)];
        switch bitcast<i32>(_e1113) {
            case 0: {
                phi_1746_ = 0u;
                break;
            }
            case 1: {
                phi_1746_ = 1u;
                break;
            }
            case 2: {
                phi_1746_ = 2u;
                break;
            }
            default: {
                phi_1746_ = 0u;
                break;
            }
        }
        let _e1116 = phi_1746_;
        let _e1120 = global.member[(_e296.member_9 + 7u)];
        switch bitcast<i32>(_e1120) {
            case 0: {
                phi_1755_ = 0u;
                break;
            }
            case 1: {
                phi_1755_ = 1u;
                break;
            }
            case 2: {
                phi_1755_ = 2u;
                break;
            }
            default: {
                phi_1755_ = 0u;
                break;
            }
        }
        let _e1123 = phi_1755_;
        phi_1768_ = type_32(type_24(_e1116, _e1123), vec2<u32>(_e1087, _e1091), vec2<u32>(_e1096, _e1100), _e1105, _e1109);
    } else {
        phi_1768_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1127 = phi_1768_;
    switch bitcast<i32>(_e1127.member.member) {
        case 1: {
            let _e1165 = abs(_e1079.x);
            let _e1167 = (_e1165 % 1f);
            if (_e1165 >= 1f) {
                phi_4746_ = select(true, false, (_e1167 == 0f));
            } else {
                phi_4746_ = true;
            }
            let _e1171 = phi_4746_;
            let _e1172 = select(1f, _e1167, _e1171);
            if (select(-1f, 1f, (_e1079.x >= 0f)) > 0f) {
                phi_1788_ = _e1172;
            } else {
                phi_1788_ = (1f - _e1172);
            }
            let _e1176 = phi_1788_;
            phi_1825_ = _e1176;
            break;
        }
        case 2: {
            let _e1139 = abs(_e1079.x);
            let _e1146 = ((select(select(u32(_e1139), 0u, (_e1139 < 0f)), 4294967295u, (_e1139 > 4294967000f)) % 2u) == 0u);
            let _e1148 = (_e1139 % 1f);
            if (_e1139 >= 1f) {
                phi_4729_ = select(true, false, (_e1148 == 0f));
            } else {
                phi_4729_ = true;
            }
            let _e1152 = phi_4729_;
            let _e1153 = select(1f, _e1148, _e1152);
            if (select(-1f, 1f, (_e1079.x >= 0f)) > 0f) {
                if _e1146 {
                    phi_1817_ = _e1153;
                } else {
                    phi_1817_ = (1f - _e1153);
                }
                let _e1160 = phi_1817_;
                phi_1823_ = _e1160;
            } else {
                if _e1146 {
                    phi_1822_ = (1f - _e1153);
                } else {
                    phi_1822_ = _e1153;
                }
                let _e1157 = phi_1822_;
                phi_1823_ = _e1157;
            }
            let _e1162 = phi_1823_;
            phi_1825_ = _e1162;
            break;
        }
        case 0: {
            if (_e1079.x > 1f) {
                phi_4716_ = 0.9999999f;
            } else {
                phi_4716_ = select(_e1079.x, 0.00000011920929f, (_e1079.x < 0f));
            }
            let _e1136 = phi_4716_;
            phi_1825_ = _e1136;
            break;
        }
        default: {
            phi_1825_ = f32();
            break;
        }
    }
    let _e1178 = phi_1825_;
    switch bitcast<i32>(_e1127.member.member_1) {
        case 1: {
            let _e1216 = abs(_e1079.y);
            let _e1218 = (_e1216 % 1f);
            if (_e1216 >= 1f) {
                phi_4794_ = select(true, false, (_e1218 == 0f));
            } else {
                phi_4794_ = true;
            }
            let _e1222 = phi_4794_;
            let _e1223 = select(1f, _e1218, _e1222);
            if (select(-1f, 1f, (_e1079.y >= 0f)) > 0f) {
                phi_1846_ = _e1223;
            } else {
                phi_1846_ = (1f - _e1223);
            }
            let _e1227 = phi_1846_;
            phi_1883_ = _e1227;
            break;
        }
        case 2: {
            let _e1190 = abs(_e1079.y);
            let _e1197 = ((select(select(u32(_e1190), 0u, (_e1190 < 0f)), 4294967295u, (_e1190 > 4294967000f)) % 2u) == 0u);
            let _e1199 = (_e1190 % 1f);
            if (_e1190 >= 1f) {
                phi_4777_ = select(true, false, (_e1199 == 0f));
            } else {
                phi_4777_ = true;
            }
            let _e1203 = phi_4777_;
            let _e1204 = select(1f, _e1199, _e1203);
            if (select(-1f, 1f, (_e1079.y >= 0f)) > 0f) {
                if _e1197 {
                    phi_1875_ = _e1204;
                } else {
                    phi_1875_ = (1f - _e1204);
                }
                let _e1211 = phi_1875_;
                phi_1881_ = _e1211;
            } else {
                if _e1197 {
                    phi_1880_ = (1f - _e1204);
                } else {
                    phi_1880_ = _e1204;
                }
                let _e1208 = phi_1880_;
                phi_1881_ = _e1208;
            }
            let _e1213 = phi_1881_;
            phi_1883_ = _e1213;
            break;
        }
        case 0: {
            if (_e1079.y > 1f) {
                phi_4764_ = 0.9999999f;
            } else {
                phi_4764_ = select(_e1079.y, 0.00000011920929f, (_e1079.y < 0f));
            }
            let _e1187 = phi_4764_;
            phi_1883_ = _e1187;
            break;
        }
        default: {
            phi_1883_ = f32();
            break;
        }
    }
    let _e1229 = phi_1883_;
    let _e1233 = (_e1178 * f32(_e1127.member_2.x));
    let _e1242 = (_e1229 * f32(_e1127.member_2.y));
    let _e1260 = vec3<f32>((f32((select(select(u32(_e1233), 0u, (_e1233 < 0f)), 4294967295u, (_e1233 > 4294967000f)) + _e1127.member_1.x)) / _e476), (f32((select(select(u32(_e1242), 0u, (_e1242 < 0f)), 4294967295u, (_e1242 > 4294967000f)) + _e1127.member_1.y)) / _e477), f32(_e1127.member_3));
    let _e1266 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1260.x, _e1260.y), i32(_e1260.z), 0f);
    let _e1269 = select(_e1266, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e296.member_9 == 4294967295u)));
    if _e879 {
        phi_1977_ = vec3<f32>(0f, 0f, 0f);
        phi_1978_ = _e122;
    } else {
        let _e1273 = fma(_e881.x, 2f, -1f);
        let _e1274 = fma(_e881.y, 2f, -1f);
        let _e1275 = fma(_e881.z, 2f, -1f);
        let _e1280 = sqrt(fma(_e1275, _e1275, fma(_e1273, _e1273, (_e1274 * _e1274))));
        if (_e1280 == 0f) {
            phi_4852_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4852_ = (vec3<f32>(_e1273, _e1274, _e1275) * (1f / _e1280));
        }
        let _e1285 = phi_4852_;
        let _e1292 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
        if (_e1292 == 0f) {
            phi_4887_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4887_ = (_e123 * (1f / _e1292));
        }
        let _e1297 = phi_4887_;
        let _e1304 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
        if (_e1304 == 0f) {
            phi_4922_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4922_ = (_e124 * (1f / _e1304));
        }
        let _e1309 = phi_4922_;
        let _e1316 = sqrt(fma(_e122.z, _e122.z, fma(_e122.x, _e122.x, (_e122.y * _e122.y))));
        if (_e1316 == 0f) {
            phi_4957_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4957_ = (_e122 * (1f / _e1316));
        }
        let _e1321 = phi_4957_;
        let _e1340 = fma(_e1321.x, _e1285.z, fma(_e1297.x, _e1285.x, (_e1309.x * _e1285.y)));
        let _e1341 = fma(_e1321.y, _e1285.z, fma(_e1297.y, _e1285.x, (_e1309.y * _e1285.y)));
        let _e1342 = fma(_e1321.z, _e1285.z, fma(_e1297.z, _e1285.x, (_e1309.z * _e1285.y)));
        let _e1347 = sqrt(fma(_e1342, _e1342, fma(_e1340, _e1340, (_e1341 * _e1341))));
        if (_e1347 == 0f) {
            phi_4992_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4992_ = (vec3<f32>(_e1340, _e1341, _e1342) * (1f / _e1347));
        }
        let _e1352 = phi_4992_;
        phi_1977_ = _e1285;
        phi_1978_ = _e1352;
    }
    let _e1354 = phi_1977_;
    let _e1356 = phi_1978_;
    let _e1360 = (_e493.x * _e296.member_2.x);
    let _e1363 = (_e493.y * _e296.member_2.y);
    let _e1366 = (_e493.z * _e296.member_2.z);
    let _e1371 = (_e1360 * _e119.x);
    let _e1373 = (_e1363 * _e119.y);
    let _e1375 = (_e1366 * _e119.z);
    let _e1380 = (_e687.y * _e296.member_4);
    let _e1383 = (_e687.z * _e296.member_3);
    let _e1387 = fma(_e296.member_16, (select(_e1072, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e296.member_8 == 4294967295u))).x - 1f), 1f);
    let _e1393 = (_e1269.x * _e296.member.x);
    let _e1395 = (_e1269.y * _e296.member.y);
    let _e1397 = (_e1269.z * _e296.member.z);
    let _e1402 = textureSampleLevel(global_12, global_13, _e1356, 0f);
    if (_e117 >= 86u) {
        phi_5024_ = (_e129 <= (_e117 - 86u));
    } else {
        phi_5024_ = false;
    }
    let _e1410 = phi_5024_;
    if _e1410 {
        let _e1413 = global.member[_e129];
        let _e1418 = global.member[(_e129 + 1u)];
        let _e1423 = global.member[(_e129 + 2u)];
        let _e1428 = global.member[(_e129 + 3u)];
        let _e1434 = global.member[(_e129 + 4u)];
        let _e1439 = global.member[(_e129 + 5u)];
        let _e1444 = global.member[(_e129 + 6u)];
        let _e1449 = global.member[(_e129 + 7u)];
        let _e1455 = global.member[(_e129 + 8u)];
        let _e1460 = global.member[(_e129 + 9u)];
        let _e1465 = global.member[(_e129 + 10u)];
        let _e1470 = global.member[(_e129 + 11u)];
        let _e1476 = global.member[(_e129 + 12u)];
        let _e1481 = global.member[(_e129 + 13u)];
        let _e1486 = global.member[(_e129 + 14u)];
        let _e1491 = global.member[(_e129 + 15u)];
        let _e1498 = global.member[(_e129 + 16u)];
        let _e1503 = global.member[(_e129 + 17u)];
        let _e1508 = global.member[(_e129 + 18u)];
        let _e1513 = global.member[(_e129 + 19u)];
        let _e1519 = global.member[(_e129 + 20u)];
        let _e1524 = global.member[(_e129 + 21u)];
        let _e1529 = global.member[(_e129 + 22u)];
        let _e1534 = global.member[(_e129 + 23u)];
        let _e1540 = global.member[(_e129 + 24u)];
        let _e1545 = global.member[(_e129 + 25u)];
        let _e1550 = global.member[(_e129 + 26u)];
        let _e1555 = global.member[(_e129 + 27u)];
        let _e1561 = global.member[(_e129 + 28u)];
        let _e1566 = global.member[(_e129 + 29u)];
        let _e1571 = global.member[(_e129 + 30u)];
        let _e1576 = global.member[(_e129 + 31u)];
        let _e1583 = global.member[(_e129 + 32u)];
        let _e1588 = global.member[(_e129 + 33u)];
        let _e1593 = global.member[(_e129 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_2185_ = type_24(0u, 6u);
        loop {
            let _e1598 = phi_2185_;
            if (_e1598.member < _e1598.member_1) {
                phi_2186_ = type_24((_e1598.member + 1u), _e1598.member_1);
                phi_2209_ = type_24(1u, _e1598.member);
            } else {
                phi_2186_ = _e1598;
                phi_2209_ = type_24(0u, type_24().member_1);
            }
            let _e1611 = phi_2186_;
            let _e1613 = phi_2209_;
            switch bitcast<i32>(_e1613.member) {
                case 0: {
                    phi_2236_ = false;
                    break;
                }
                case 1: {
                    let _e1618 = ((_e129 + 35u) + (_e1613.member_1 * 4u));
                    let _e1621 = global.member[_e1618];
                    let _e1626 = global.member[(_e1618 + 1u)];
                    let _e1631 = global.member[(_e1618 + 2u)];
                    let _e1636 = global.member[(_e1618 + 3u)];
                    local_1[_e1613.member_1] = vec4<f32>(bitcast<f32>(_e1621), bitcast<f32>(_e1626), bitcast<f32>(_e1631), bitcast<f32>(_e1636));
                    phi_2236_ = true;
                    break;
                }
                default: {
                    phi_2236_ = bool();
                    break;
                }
            }
            let _e1641 = phi_2236_;
            continue;
            continuing {
                phi_2185_ = _e1611;
                break if !(_e1641);
            }
        }
        let _e1643 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2242_ = type_24(0u, 8u);
        loop {
            let _e1646 = phi_2242_;
            if (_e1646.member < _e1646.member_1) {
                phi_2243_ = type_24((_e1646.member + 1u), _e1646.member_1);
                phi_2266_ = type_24(1u, _e1646.member);
            } else {
                phi_2243_ = _e1646;
                phi_2266_ = type_24(0u, type_24().member_1);
            }
            let _e1659 = phi_2243_;
            let _e1661 = phi_2266_;
            switch bitcast<i32>(_e1661.member) {
                case 0: {
                    phi_2289_ = false;
                    break;
                }
                case 1: {
                    let _e1666 = ((_e129 + 59u) + (_e1661.member_1 * 3u));
                    let _e1669 = global.member[_e1666];
                    let _e1674 = global.member[(_e1666 + 1u)];
                    let _e1679 = global.member[(_e1666 + 2u)];
                    local[_e1661.member_1] = vec3<f32>(bitcast<f32>(_e1669), bitcast<f32>(_e1674), bitcast<f32>(_e1679));
                    phi_2289_ = true;
                    break;
                }
                default: {
                    phi_2289_ = bool();
                    break;
                }
            }
            let _e1684 = phi_2289_;
            continue;
            continuing {
                phi_2242_ = _e1659;
                break if !(_e1684);
            }
        }
        let _e1686 = local;
        let _e1690 = global.member[(_e129 + 83u)];
        let _e1695 = global.member[(_e129 + 84u)];
        let _e1700 = global.member[(_e129 + 85u)];
        phi_2310_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1413), bitcast<f32>(_e1418), bitcast<f32>(_e1423), bitcast<f32>(_e1428)), vec4<f32>(bitcast<f32>(_e1434), bitcast<f32>(_e1439), bitcast<f32>(_e1444), bitcast<f32>(_e1449)), vec4<f32>(bitcast<f32>(_e1455), bitcast<f32>(_e1460), bitcast<f32>(_e1465), bitcast<f32>(_e1470)), vec4<f32>(bitcast<f32>(_e1476), bitcast<f32>(_e1481), bitcast<f32>(_e1486), bitcast<f32>(_e1491))), type_20(vec4<f32>(bitcast<f32>(_e1498), bitcast<f32>(_e1503), bitcast<f32>(_e1508), bitcast<f32>(_e1513)), vec4<f32>(bitcast<f32>(_e1519), bitcast<f32>(_e1524), bitcast<f32>(_e1529), bitcast<f32>(_e1534)), vec4<f32>(bitcast<f32>(_e1540), bitcast<f32>(_e1545), bitcast<f32>(_e1550), bitcast<f32>(_e1555)), vec4<f32>(bitcast<f32>(_e1561), bitcast<f32>(_e1566), bitcast<f32>(_e1571), bitcast<f32>(_e1576))), vec3<f32>(bitcast<f32>(_e1583), bitcast<f32>(_e1588), bitcast<f32>(_e1593)), type_21(_e1686, _e1643, vec3<f32>(bitcast<f32>(_e1690), bitcast<f32>(_e1695), bitcast<f32>(_e1700))));
    } else {
        phi_2310_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e1706 = phi_2310_;
    let _e1708 = (_e1706.member_2 - _e125);
    let _e1715 = sqrt(fma(_e1708.z, _e1708.z, fma(_e1708.x, _e1708.x, (_e1708.y * _e1708.y))));
    let _e1716 = (_e1715 == 0f);
    if _e1716 {
        phi_5096_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5096_ = (_e1708 * (1f / _e1715));
    }
    let _e1720 = phi_5096_;
    let _e1721 = -(_e1720);
    let _e1728 = sqrt(fma(_e1356.z, _e1356.z, fma(_e1356.x, _e1356.x, (_e1356.y * _e1356.y))));
    let _e1729 = (_e1728 == 0f);
    if _e1729 {
        phi_5155_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5155_ = (_e1356 * (1f / _e1728));
    }
    let _e1733 = phi_5155_;
    let _e1743 = (2f * fma(_e1733.z, _e1721.z, fma(_e1733.x, _e1721.x, (_e1733.y * _e1721.y))));
    let _e1750 = textureSampleLevel(global_14, global_15, (_e1721 - vec3<f32>((_e1743 * _e1733.x), (_e1743 * _e1733.y), (_e1743 * _e1733.z))), (_e1380 * 4f));
    if _e1716 {
        phi_5229_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5229_ = (_e1708 * (1f / _e1715));
    }
    let _e1757 = phi_5229_;
    let _e1766 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1356.z, _e1757.z, fma(_e1356.x, _e1757.x, (_e1356.y * _e1757.y))), 0f), _e1380), 0f);
    switch bitcast<i32>(_e151) {
        case 0: {
            if _e296.member_15 {
                if _e1729 {
                    phi_5622_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5622_ = (_e1356 * (1f / _e1728));
                }
                let _e1935 = phi_5622_;
                if _e1716 {
                    phi_5657_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5657_ = (_e1708 * (1f / _e1715));
                }
                let _e1939 = phi_5657_;
                phi_2350_ = type_24(0u, _e164);
                phi_2353_ = vec3<f32>(0f, 0f, 0f);
                loop {
                    let _e1942 = phi_2350_;
                    let _e1944 = phi_2353_;
                    local_2 = _e1944;
                    local_3 = _e1944;
                    local_4 = _e1944;
                    if (_e1942.member < _e1942.member_1) {
                        phi_2351_ = type_24((_e1942.member + 1u), _e1942.member_1);
                        phi_2376_ = type_24(1u, _e1942.member);
                    } else {
                        phi_2351_ = _e1942;
                        phi_2376_ = type_24(0u, type_24().member_1);
                    }
                    let _e1957 = phi_2351_;
                    let _e1959 = phi_2376_;
                    switch bitcast<i32>(_e1959.member) {
                        case 0: {
                            phi_2354_ = vec3<f32>();
                            phi_3407_ = false;
                            break;
                        }
                        case 1: {
                            if (_e1959.member_1 >= _e164) {
                                phi_5674_ = 4294967295u;
                            } else {
                                phi_5674_ = (_e160 + _e1959.member_1);
                            }
                            let _e1966 = phi_5674_;
                            if (_e117 >= 1u) {
                                phi_5693_ = (_e1966 <= (_e117 - 1u));
                            } else {
                                phi_5693_ = false;
                            }
                            let _e1971 = phi_5693_;
                            if _e1971 {
                                let _e1974 = global.member[_e1966];
                                phi_2393_ = _e1974;
                            } else {
                                phi_2393_ = 4294967295u;
                            }
                            let _e1976 = phi_2393_;
                            let _e1977 = (_e1976 == 4294967295u);
                            if _e1977 {
                                phi_3405_ = vec3<f32>();
                            } else {
                                if (_e117 >= 4u) {
                                    phi_5725_ = (_e1976 <= (_e117 - 4u));
                                } else {
                                    phi_5725_ = false;
                                }
                                let _e1982 = phi_5725_;
                                if _e1982 {
                                    let _e1985 = global.member[_e1976];
                                    switch bitcast<i32>(_e1985) {
                                        case 0: {
                                            phi_2410_ = 0u;
                                            break;
                                        }
                                        case 1: {
                                            phi_2410_ = 1u;
                                            break;
                                        }
                                        case 2: {
                                            phi_2410_ = 2u;
                                            break;
                                        }
                                        default: {
                                            phi_2410_ = 0u;
                                            break;
                                        }
                                    }
                                    let _e1988 = phi_2410_;
                                    let _e1992 = global.member[(_e1976 + 1u)];
                                    let _e1996 = global.member[(_e1976 + 2u)];
                                    let _e2000 = global.member[(_e1976 + 3u)];
                                    phi_2424_ = type_28(_e1988, _e1992, _e1996, _e2000);
                                } else {
                                    phi_2424_ = type_28(0u, 4294967295u, 4294967295u, 4294967295u);
                                }
                                let _e2003 = phi_2424_;
                                if (_e117 >= 10u) {
                                    phi_5757_ = (_e2003.member_2 <= (_e117 - 10u));
                                } else {
                                    phi_5757_ = false;
                                }
                                let _e2009 = phi_5757_;
                                if _e2009 {
                                    let _e2012 = global.member[_e2003.member_2];
                                    let _e2017 = global.member[(_e2003.member_2 + 1u)];
                                    let _e2022 = global.member[(_e2003.member_2 + 2u)];
                                    let _e2028 = global.member[(_e2003.member_2 + 3u)];
                                    let _e2033 = global.member[(_e2003.member_2 + 4u)];
                                    let _e2038 = global.member[(_e2003.member_2 + 5u)];
                                    let _e2043 = global.member[(_e2003.member_2 + 6u)];
                                    let _e2049 = global.member[(_e2003.member_2 + 7u)];
                                    let _e2054 = global.member[(_e2003.member_2 + 8u)];
                                    let _e2059 = global.member[(_e2003.member_2 + 9u)];
                                    phi_2474_ = type_30(vec3<f32>(bitcast<f32>(_e2012), bitcast<f32>(_e2017), bitcast<f32>(_e2022)), vec4<f32>(bitcast<f32>(_e2028), bitcast<f32>(_e2033), bitcast<f32>(_e2038), bitcast<f32>(_e2043)), vec3<f32>(bitcast<f32>(_e2049), bitcast<f32>(_e2054), bitcast<f32>(_e2059)));
                                } else {
                                    phi_2474_ = type_30(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e2064 = phi_2474_;
                                let _e2072 = (_e2064.member_1.x + _e2064.member_1.x);
                                let _e2073 = (_e2064.member_1.y + _e2064.member_1.y);
                                let _e2074 = (_e2064.member_1.z + _e2064.member_1.z);
                                let _e2076 = (_e2064.member_1.z * _e2074);
                                let _e2077 = (_e2064.member_1.w * _e2072);
                                let _e2078 = (_e2064.member_1.w * _e2073);
                                let _e2079 = (_e2064.member_1.w * _e2074);
                                let _e2099 = (vec4<f32>((1f - fma(_e2064.member_1.y, _e2073, _e2076)), fma(_e2064.member_1.x, _e2073, _e2079), fma(_e2064.member_1.x, _e2074, -(_e2078)), 0f) * _e2064.member_2.x);
                                let _e2101 = (vec4<f32>(fma(_e2064.member_1.x, _e2073, -(_e2079)), (1f - fma(_e2064.member_1.x, _e2072, _e2076)), fma(_e2064.member_1.y, _e2074, _e2077), 0f) * _e2064.member_2.y);
                                let _e2103 = (vec4<f32>(fma(_e2064.member_1.x, _e2074, _e2078), fma(_e2064.member_1.y, _e2074, -(_e2077)), (1f - fma(_e2064.member_1.x, _e2072, (_e2064.member_1.y * _e2073))), 0f) * _e2064.member_2.z);
                                switch bitcast<i32>(_e2003.member) {
                                    case 0: {
                                        if _e302 {
                                            phi_6223_ = (_e2003.member_1 <= (_e117 - 8u));
                                        } else {
                                            phi_6223_ = false;
                                        }
                                        let _e2599 = phi_6223_;
                                        if _e2599 {
                                            let _e2602 = global.member[_e2003.member_1];
                                            let _e2607 = global.member[(_e2003.member_1 + 1u)];
                                            let _e2612 = global.member[(_e2003.member_1 + 2u)];
                                            let _e2618 = global.member[(_e2003.member_1 + 3u)];
                                            let _e2623 = global.member[(_e2003.member_1 + 4u)];
                                            let _e2628 = global.member[(_e2003.member_1 + 5u)];
                                            let _e2633 = global.member[(_e2003.member_1 + 6u)];
                                            let _e2639 = global.member[(_e2003.member_1 + 7u)];
                                            phi_2522_ = type_35(vec3<f32>(bitcast<f32>(_e2602), bitcast<f32>(_e2607), bitcast<f32>(_e2612)), vec4<f32>(bitcast<f32>(_e2618), bitcast<f32>(_e2623), bitcast<f32>(_e2628), bitcast<f32>(_e2633)), bitcast<f32>(_e2639));
                                        } else {
                                            phi_2522_ = type_35(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2643 = phi_2522_;
                                        let _e2665 = fma(_e2103.x, _e2643.member.z, fma(_e2101.x, _e2643.member.y, (_e2099.x * _e2643.member.x)));
                                        let _e2666 = fma(_e2103.y, _e2643.member.z, fma(_e2101.y, _e2643.member.y, (_e2099.y * _e2643.member.x)));
                                        let _e2667 = fma(_e2103.z, _e2643.member.z, fma(_e2101.z, _e2643.member.y, (_e2099.z * _e2643.member.x)));
                                        let _e2672 = sqrt(fma(_e2667, _e2667, fma(_e2665, _e2665, (_e2666 * _e2666))));
                                        if (_e2672 == 0f) {
                                            phi_6270_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6270_ = (vec3<f32>(_e2665, _e2666, _e2667) * (1f / _e2672));
                                        }
                                        let _e2677 = phi_6270_;
                                        let _e2679 = -(_e2677.x);
                                        let _e2681 = -(_e2677.y);
                                        let _e2683 = -(_e2677.z);
                                        let _e2684 = -(_e2677);
                                        let _e2686 = fma(-(_e687.z), _e296.member_3, 1f);
                                        let _e2690 = fma(0.4f, _e2686, (_e1371 * _e1383));
                                        let _e2691 = fma(0.4f, _e2686, (_e1373 * _e1383));
                                        let _e2692 = fma(0.4f, _e2686, (_e1375 * _e1383));
                                        let _e2700 = (_e1939 + vec3<f32>(_e2679, _e2681, _e2683));
                                        let _e2707 = sqrt(fma(_e2700.z, _e2700.z, fma(_e2700.x, _e2700.x, (_e2700.y * _e2700.y))));
                                        if (_e2707 == 0f) {
                                            phi_6305_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6305_ = (_e2700 * (1f / _e2707));
                                        }
                                        let _e2712 = phi_6305_;
                                        let _e2713 = (_e1380 * _e1380);
                                        let _e2724 = max(fma(_e1935.z, _e2712.z, fma(_e1935.x, _e2712.x, (_e1935.y * _e2712.y))), 0f);
                                        let _e2737 = max(fma(_e1935.z, _e1939.z, fma(_e1935.x, _e1939.x, (_e1935.y * _e1939.y))), 0f);
                                        let _e2743 = fma(_e1935.z, _e2684.z, fma(_e1935.x, _e2684.x, (_e1935.y * _e2684.y)));
                                        let _e2744 = max(_e2743, 0f);
                                        let _e2745 = fma(_e687.y, _e296.member_4, 1f);
                                        let _e2746 = (_e2745 * _e2745);
                                        let _e2747 = (_e2746 * 0.125f);
                                        let _e2749 = fma(-(_e2746), 0.125f, 1f);
                                        let _e2762 = (1f - max(fma(_e2712.z, _e1939.z, fma(_e2712.x, _e1939.x, (_e2712.y * _e1939.y))), 0f));
                                        let _e2764 = select(_e2762, 0f, (_e2762 < 0f));
                                        let _e2767 = pow(select(_e2764, 1f, (_e2764 > 1f)), 5f);
                                        let _e2768 = fma((1f - _e2690), _e2767, _e2690);
                                        let _e2769 = fma((1f - _e2691), _e2767, _e2691);
                                        let _e2770 = fma((1f - _e2692), _e2767, _e2692);
                                        let _e2777 = (((_e2713 * _e2713) / (pow(fma((_e2724 * _e2724), fma(_e2713, _e2713, -1f), 1f), 2f) * 3.1415927f)) * ((_e2737 / fma(_e2737, _e2749, _e2747)) * (_e2744 / fma(_e2744, _e2749, _e2747))));
                                        let _e2784 = max(fma(_e1935.z, _e2683, fma(_e1935.x, _e2679, (_e1935.y * _e2681))), 0f);
                                        let _e2786 = fma((4f * _e2737), _e2784, 0.0001f);
                                        if ((_e2003.member_3 == 4294967295u) != true) {
                                            let _e2807 = global_1.member[_e2003.member_3];
                                            let _e2812 = global_1.member[(_e2003.member_3 + 1u)];
                                            let _e2817 = global_1.member[(_e2003.member_3 + 2u)];
                                            let _e2822 = global_1.member[(_e2003.member_3 + 3u)];
                                            let _e2827 = global_1.member[(_e2003.member_3 + 4u)];
                                            let _e2832 = global_1.member[(_e2003.member_3 + 5u)];
                                            let _e2837 = global_1.member[(_e2003.member_3 + 6u)];
                                            let _e2842 = global_1.member[(_e2003.member_3 + 7u)];
                                            let _e2847 = global_1.member[(_e2003.member_3 + 8u)];
                                            let _e2852 = global_1.member[(_e2003.member_3 + 9u)];
                                            let _e2857 = global_1.member[(_e2003.member_3 + 10u)];
                                            let _e2862 = global_1.member[(_e2003.member_3 + 11u)];
                                            let _e2867 = global_1.member[(_e2003.member_3 + 12u)];
                                            let _e2872 = global_1.member[(_e2003.member_3 + 13u)];
                                            let _e2877 = global_1.member[(_e2003.member_3 + 14u)];
                                            let _e2882 = global_1.member[(_e2003.member_3 + 15u)];
                                            let _e2887 = global_1.member[(_e2003.member_3 + 18u)];
                                            let _e2892 = global_1.member[(_e2003.member_3 + 19u)];
                                            let _e2912 = (bitcast<f32>(_e2882) + fma(bitcast<f32>(_e2862), _e125.z, fma(bitcast<f32>(_e2842), _e125.y, (bitcast<f32>(_e2822) * _e125.x))));
                                            let _e2920 = textureSampleLevel(global_18, global_19, vec2<f32>(fma(((bitcast<f32>(_e2867) + fma(bitcast<f32>(_e2847), _e125.z, fma(bitcast<f32>(_e2827), _e125.y, (bitcast<f32>(_e2807) * _e125.x)))) / _e2912), 0.5f, 0.5f), fma(((bitcast<f32>(_e2872) + fma(bitcast<f32>(_e2852), _e125.z, fma(bitcast<f32>(_e2832), _e125.y, (bitcast<f32>(_e2812) * _e125.x)))) / _e2912), -0.5f, 0.5f)), i32(0f));
                                            phi_2831_ = select(0f, 1f, ((((bitcast<f32>(_e2877) + fma(bitcast<f32>(_e2857), _e125.z, fma(bitcast<f32>(_e2837), _e125.y, (bitcast<f32>(_e2817) * _e125.x)))) / _e2912) - max((bitcast<f32>(_e2892) * (1f - _e2743)), bitcast<f32>(_e2887))) > vec4(_e2920).x));
                                        } else {
                                            phi_2831_ = 0f;
                                        }
                                        let _e2930 = phi_2831_;
                                        let _e2931 = (1f - _e2930);
                                        phi_3395_ = vec3<f32>(fma(((fma((((1f - _e2768) * _e2686) * _e1371), 0.31830987f, ((_e2777 * _e2768) / _e2786)) * (_e2643.member_1.x * _e2643.member_2)) * _e2784), _e2931, _e1944.x), fma(((fma((((1f - _e2769) * _e2686) * _e1373), 0.31830987f, ((_e2777 * _e2769) / _e2786)) * (_e2643.member_1.y * _e2643.member_2)) * _e2784), _e2931, _e1944.y), fma(((fma((((1f - _e2770) * _e2686) * _e1375), 0.31830987f, ((_e2777 * _e2770) / _e2786)) * (_e2643.member_1.z * _e2643.member_2)) * _e2784), _e2931, _e1944.z));
                                        phi_3396_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if _e302 {
                                            phi_6049_ = (_e2003.member_1 <= (_e117 - 8u));
                                        } else {
                                            phi_6049_ = false;
                                        }
                                        let _e2388 = phi_6049_;
                                        if _e2388 {
                                            let _e2391 = global.member[_e2003.member_1];
                                            let _e2396 = global.member[(_e2003.member_1 + 1u)];
                                            let _e2401 = global.member[(_e2003.member_1 + 2u)];
                                            let _e2407 = global.member[(_e2003.member_1 + 3u)];
                                            let _e2412 = global.member[(_e2003.member_1 + 4u)];
                                            let _e2417 = global.member[(_e2003.member_1 + 5u)];
                                            let _e2422 = global.member[(_e2003.member_1 + 6u)];
                                            let _e2428 = global.member[(_e2003.member_1 + 7u)];
                                            phi_2885_ = type_35(vec3<f32>(bitcast<f32>(_e2391), bitcast<f32>(_e2396), bitcast<f32>(_e2401)), vec4<f32>(bitcast<f32>(_e2407), bitcast<f32>(_e2412), bitcast<f32>(_e2417), bitcast<f32>(_e2422)), bitcast<f32>(_e2428));
                                        } else {
                                            phi_2885_ = type_35(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2432 = phi_2885_;
                                        let _e2461 = (vec3<f32>((_e2064.member.x + fma(_e2103.x, _e2432.member.z, fma(_e2101.x, _e2432.member.y, (_e2099.x * _e2432.member.x)))), (_e2064.member.y + fma(_e2103.y, _e2432.member.z, fma(_e2101.y, _e2432.member.y, (_e2099.y * _e2432.member.x)))), (_e2064.member.z + fma(_e2103.z, _e2432.member.z, fma(_e2101.z, _e2432.member.y, (_e2099.z * _e2432.member.x))))) - _e125);
                                        let _e2468 = sqrt(fma(_e2461.z, _e2461.z, fma(_e2461.x, _e2461.x, (_e2461.y * _e2461.y))));
                                        let _e2469 = (_e2468 == 0f);
                                        if _e2469 {
                                            phi_3075_ = vec3<f32>();
                                        } else {
                                            if _e2469 {
                                                phi_6096_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6096_ = (_e2461 * (1f / _e2468));
                                            }
                                            let _e2473 = phi_6096_;
                                            let _e2475 = (_e2432.member_2 / (_e2468 * _e2468));
                                            let _e2477 = fma(-(_e687.z), _e296.member_3, 1f);
                                            let _e2481 = fma(0.4f, _e2477, (_e1371 * _e1383));
                                            let _e2482 = fma(0.4f, _e2477, (_e1373 * _e1383));
                                            let _e2483 = fma(0.4f, _e2477, (_e1375 * _e1383));
                                            let _e2490 = (_e1939 + _e2473);
                                            let _e2497 = sqrt(fma(_e2490.z, _e2490.z, fma(_e2490.x, _e2490.x, (_e2490.y * _e2490.y))));
                                            if (_e2497 == 0f) {
                                                phi_6131_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6131_ = (_e2490 * (1f / _e2497));
                                            }
                                            let _e2502 = phi_6131_;
                                            let _e2503 = (_e1380 * _e1380);
                                            let _e2514 = max(fma(_e1935.z, _e2502.z, fma(_e1935.x, _e2502.x, (_e1935.y * _e2502.y))), 0f);
                                            let _e2527 = max(fma(_e1935.z, _e1939.z, fma(_e1935.x, _e1939.x, (_e1935.y * _e1939.y))), 0f);
                                            let _e2534 = max(fma(_e1935.z, _e2473.z, fma(_e1935.x, _e2473.x, (_e1935.y * _e2473.y))), 0f);
                                            let _e2535 = fma(_e687.y, _e296.member_4, 1f);
                                            let _e2536 = (_e2535 * _e2535);
                                            let _e2537 = (_e2536 * 0.125f);
                                            let _e2539 = fma(-(_e2536), 0.125f, 1f);
                                            let _e2552 = (1f - max(fma(_e2502.z, _e1939.z, fma(_e2502.x, _e1939.x, (_e2502.y * _e1939.y))), 0f));
                                            let _e2554 = select(_e2552, 0f, (_e2552 < 0f));
                                            let _e2557 = pow(select(_e2554, 1f, (_e2554 > 1f)), 5f);
                                            let _e2558 = fma((1f - _e2481), _e2557, _e2481);
                                            let _e2559 = fma((1f - _e2482), _e2557, _e2482);
                                            let _e2560 = fma((1f - _e2483), _e2557, _e2483);
                                            let _e2567 = (((_e2503 * _e2503) / (pow(fma((_e2514 * _e2514), fma(_e2503, _e2503, -1f), 1f), 2f) * 3.1415927f)) * ((_e2527 / fma(_e2527, _e2539, _e2537)) * (_e2534 / fma(_e2534, _e2539, _e2537))));
                                            let _e2572 = fma((4f * _e2527), _e2534, 0.0001f);
                                            phi_3075_ = vec3<f32>(fma((fma((((1f - _e2558) * _e2477) * _e1371), 0.31830987f, ((_e2567 * _e2558) / _e2572)) * (_e2432.member_1.x * _e2475)), _e2534, _e1944.x), fma((fma((((1f - _e2559) * _e2477) * _e1373), 0.31830987f, ((_e2567 * _e2559) / _e2572)) * (_e2432.member_1.y * _e2475)), _e2534, _e1944.y), fma((fma((((1f - _e2560) * _e2477) * _e1375), 0.31830987f, ((_e2567 * _e2560) / _e2572)) * (_e2432.member_1.z * _e2475)), _e2534, _e1944.z));
                                        }
                                        let _e2593 = phi_3075_;
                                        phi_3395_ = _e2593;
                                        phi_3396_ = select(true, false, _e2469);
                                        break;
                                    }
                                    case 2: {
                                        if (_e117 >= 13u) {
                                            phi_5837_ = (_e2003.member_1 <= (_e117 - 13u));
                                        } else {
                                            phi_5837_ = false;
                                        }
                                        let _e2114 = phi_5837_;
                                        if _e2114 {
                                            let _e2117 = global.member[_e2003.member_1];
                                            let _e2122 = global.member[(_e2003.member_1 + 1u)];
                                            let _e2127 = global.member[(_e2003.member_1 + 2u)];
                                            let _e2133 = global.member[(_e2003.member_1 + 3u)];
                                            let _e2138 = global.member[(_e2003.member_1 + 4u)];
                                            let _e2143 = global.member[(_e2003.member_1 + 5u)];
                                            let _e2149 = global.member[(_e2003.member_1 + 6u)];
                                            let _e2154 = global.member[(_e2003.member_1 + 7u)];
                                            let _e2159 = global.member[(_e2003.member_1 + 8u)];
                                            let _e2164 = global.member[(_e2003.member_1 + 9u)];
                                            let _e2169 = global.member[(_e2003.member_1 + 10u)];
                                            let _e2174 = global.member[(_e2003.member_1 + 11u)];
                                            let _e2180 = global.member[(_e2003.member_1 + 12u)];
                                            phi_3138_ = type_36(vec3<f32>(bitcast<f32>(_e2117), bitcast<f32>(_e2122), bitcast<f32>(_e2127)), vec3<f32>(bitcast<f32>(_e2133), bitcast<f32>(_e2138), bitcast<f32>(_e2143)), bitcast<f32>(_e2149), bitcast<f32>(_e2154), vec4<f32>(bitcast<f32>(_e2159), bitcast<f32>(_e2164), bitcast<f32>(_e2169), bitcast<f32>(_e2174)), bitcast<f32>(_e2180));
                                        } else {
                                            phi_3138_ = type_36(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2184 = phi_3138_;
                                        let _e2216 = (vec3<f32>((_e2064.member.x + fma(_e2103.x, _e2184.member.z, fma(_e2101.x, _e2184.member.y, (_e2099.x * _e2184.member.x)))), (_e2064.member.y + fma(_e2103.y, _e2184.member.z, fma(_e2101.y, _e2184.member.y, (_e2099.y * _e2184.member.x)))), (_e2064.member.z + fma(_e2103.z, _e2184.member.z, fma(_e2101.z, _e2184.member.y, (_e2099.z * _e2184.member.x))))) - _e125);
                                        let _e2223 = sqrt(fma(_e2216.z, _e2216.z, fma(_e2216.x, _e2216.x, (_e2216.y * _e2216.y))));
                                        let _e2224 = (_e2223 == 0f);
                                        if _e2224 {
                                            phi_3393_ = vec3<f32>();
                                        } else {
                                            if _e2224 {
                                                phi_5887_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5887_ = (_e2216 * (1f / _e2223));
                                            }
                                            let _e2228 = phi_5887_;
                                            let _e2238 = fma(_e2103.x, _e2184.member_1.z, fma(_e2101.x, _e2184.member_1.y, (_e2099.x * _e2184.member_1.x)));
                                            let _e2239 = fma(_e2103.y, _e2184.member_1.z, fma(_e2101.y, _e2184.member_1.y, (_e2099.y * _e2184.member_1.x)));
                                            let _e2240 = fma(_e2103.z, _e2184.member_1.z, fma(_e2101.z, _e2184.member_1.y, (_e2099.z * _e2184.member_1.x)));
                                            let _e2245 = sqrt(fma(_e2240, _e2240, fma(_e2238, _e2238, (_e2239 * _e2239))));
                                            if (_e2245 == 0f) {
                                                phi_5922_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5922_ = (vec3<f32>(_e2238, _e2239, _e2240) * (1f / _e2245));
                                            }
                                            let _e2250 = phi_5922_;
                                            let _e2262 = ((fma(_e2228.z, _e2250.z, fma(_e2228.x, _e2250.x, (_e2228.y * _e2250.y))) - _e2184.member_3) / (_e2184.member_2 - _e2184.member_3));
                                            let _e2264 = select(_e2262, 0f, (_e2262 < 0f));
                                            let _e2267 = (_e2184.member_5 * select(_e2264, 1f, (_e2264 > 1f)));
                                            let _e2269 = fma(-(_e687.z), _e296.member_3, 1f);
                                            let _e2273 = fma(0.4f, _e2269, (_e1371 * _e1383));
                                            let _e2274 = fma(0.4f, _e2269, (_e1373 * _e1383));
                                            let _e2275 = fma(0.4f, _e2269, (_e1375 * _e1383));
                                            let _e2282 = (_e1939 + _e2228);
                                            let _e2289 = sqrt(fma(_e2282.z, _e2282.z, fma(_e2282.x, _e2282.x, (_e2282.y * _e2282.y))));
                                            if (_e2289 == 0f) {
                                                phi_5957_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5957_ = (_e2282 * (1f / _e2289));
                                            }
                                            let _e2294 = phi_5957_;
                                            let _e2295 = (_e1380 * _e1380);
                                            let _e2306 = max(fma(_e1935.z, _e2294.z, fma(_e1935.x, _e2294.x, (_e1935.y * _e2294.y))), 0f);
                                            let _e2319 = max(fma(_e1935.z, _e1939.z, fma(_e1935.x, _e1939.x, (_e1935.y * _e1939.y))), 0f);
                                            let _e2323 = max(fma(_e1935.z, _e2228.z, fma(_e1935.x, _e2228.x, (_e1935.y * _e2228.y))), 0f);
                                            let _e2324 = fma(_e687.y, _e296.member_4, 1f);
                                            let _e2325 = (_e2324 * _e2324);
                                            let _e2326 = (_e2325 * 0.125f);
                                            let _e2328 = fma(-(_e2325), 0.125f, 1f);
                                            let _e2341 = (1f - max(fma(_e2294.z, _e1939.z, fma(_e2294.x, _e1939.x, (_e2294.y * _e1939.y))), 0f));
                                            let _e2343 = select(_e2341, 0f, (_e2341 < 0f));
                                            let _e2346 = pow(select(_e2343, 1f, (_e2343 > 1f)), 5f);
                                            let _e2347 = fma((1f - _e2273), _e2346, _e2273);
                                            let _e2348 = fma((1f - _e2274), _e2346, _e2274);
                                            let _e2349 = fma((1f - _e2275), _e2346, _e2275);
                                            let _e2356 = (((_e2295 * _e2295) / (pow(fma((_e2306 * _e2306), fma(_e2295, _e2295, -1f), 1f), 2f) * 3.1415927f)) * ((_e2319 / fma(_e2319, _e2328, _e2326)) * (_e2323 / fma(_e2323, _e2328, _e2326))));
                                            let _e2361 = fma((4f * _e2319), _e2323, 0.0001f);
                                            phi_3393_ = vec3<f32>(fma((fma((((1f - _e2347) * _e2269) * _e1371), 0.31830987f, ((_e2356 * _e2347) / _e2361)) * (_e2184.member_4.x * _e2267)), _e2323, _e1944.x), fma((fma((((1f - _e2348) * _e2269) * _e1373), 0.31830987f, ((_e2356 * _e2348) / _e2361)) * (_e2184.member_4.y * _e2267)), _e2323, _e1944.y), fma((fma((((1f - _e2349) * _e2269) * _e1375), 0.31830987f, ((_e2356 * _e2349) / _e2361)) * (_e2184.member_4.z * _e2267)), _e2323, _e1944.z));
                                        }
                                        let _e2382 = phi_3393_;
                                        phi_3395_ = _e2382;
                                        phi_3396_ = select(true, false, _e2224);
                                        break;
                                    }
                                    default: {
                                        phi_3395_ = vec3<f32>();
                                        phi_3396_ = bool();
                                        break;
                                    }
                                }
                                let _e2940 = phi_3395_;
                                let _e2942 = phi_3396_;
                                phi_3405_ = select(_e2940, _e1944, vec3(select(true, false, _e2942)));
                            }
                            let _e2947 = phi_3405_;
                            phi_2354_ = _e2947;
                            phi_3407_ = select(true, false, _e1977);
                            break;
                        }
                        default: {
                            phi_2354_ = vec3<f32>();
                            phi_3407_ = bool();
                            break;
                        }
                    }
                    let _e2950 = phi_2354_;
                    let _e2952 = phi_3407_;
                    continue;
                    continuing {
                        phi_2350_ = _e1957;
                        phi_2353_ = _e2950;
                        break if !(_e2952);
                    }
                }
                let _e2955 = fma(-(_e687.z), _e296.member_3, 1f);
                let _e2959 = fma(0.04f, _e2955, (_e1371 * _e1383));
                let _e2960 = fma(0.04f, _e2955, (_e1373 * _e1383));
                let _e2961 = fma(0.04f, _e2955, (_e1375 * _e1383));
                let _e2973 = fma(-(_e687.y), _e296.member_4, 1f);
                let _e2980 = (1f - max(fma(_e1935.z, _e1939.z, fma(_e1935.x, _e1939.x, (_e1935.y * _e1939.y))), 0f));
                let _e2982 = select(_e2980, 0f, (_e2980 < 0f));
                let _e2985 = pow(select(_e2982, 1f, (_e2982 > 1f)), 5f);
                let _e2986 = fma((max(_e2973, _e2959) - _e2959), _e2985, _e2959);
                let _e2987 = fma((max(_e2973, _e2960) - _e2960), _e2985, _e2960);
                let _e2988 = fma((max(_e2973, _e2961) - _e2961), _e2985, _e2961);
                let _e3008 = local_2;
                let _e3012 = local_3;
                let _e3016 = local_4;
                phi_3524_ = vec4<f32>(fma(_e1393, _e296.member_1, fma(fma(((1f - _e2986) * _e2955), (_e1402.x * _e1371), (_e1750.x * fma(_e2986, _e1766.x, _e1766.y))), _e1387, _e3008.x)), fma(_e1395, _e296.member_1, fma(fma(((1f - _e2987) * _e2955), (_e1402.y * _e1373), (_e1750.y * fma(_e2987, _e1766.x, _e1766.y))), _e1387, _e3012.y)), fma(_e1397, _e296.member_1, fma(fma(((1f - _e2988) * _e2955), (_e1402.z * _e1375), (_e1750.z * fma(_e2988, _e1766.x, _e1766.y))), _e1387, _e3016.z)), 1f);
            } else {
                phi_3524_ = (vec4<f32>((_e119.x * _e493.x), (_e119.y * _e493.y), (_e119.z * _e493.z), (_e119.w * _e493.w)) * _e296.member_2);
            }
            let _e3024 = phi_3524_;
            global_20 = _e3024;
            break;
        }
        case 1: {
            let _e1908 = sqrt(fma(_e120.x, _e120.x, (_e120.y * _e120.y)));
            if (_e1908 == 0f) {
                phi_5583_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5583_ = (vec3<f32>(_e120.x, _e120.y, 0f) * (1f / _e1908));
            }
            let _e1913 = phi_5583_;
            global_20 = vec4<f32>(((_e1913.x + 1f) * 0.5f), ((_e1913.y + 1f) * 0.5f), ((_e1913.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1887 = sqrt(fma(_e121.x, _e121.x, (_e121.y * _e121.y)));
            if (_e1887 == 0f) {
                phi_5534_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5534_ = (vec3<f32>(_e121.x, _e121.y, 0f) * (1f / _e1887));
            }
            let _e1892 = phi_5534_;
            global_20 = vec4<f32>(((_e1892.x + 1f) * 0.5f), ((_e1892.y + 1f) * 0.5f), ((_e1892.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1729 {
                phi_5485_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5485_ = (_e1356 * (1f / _e1728));
            }
            let _e1871 = phi_5485_;
            global_20 = vec4<f32>(((_e1871.x + 1f) * 0.5f), ((_e1871.y + 1f) * 0.5f), ((_e1871.z + 1f) * 0.5f), 1f);
            break;
        }
        case 4: {
            global_20 = _e119;
            break;
        }
        case 5: {
            let _e1852 = sqrt(fma(_e122.z, _e122.z, fma(_e122.x, _e122.x, (_e122.y * _e122.y))));
            if (_e1852 == 0f) {
                phi_5436_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5436_ = (_e122 * (1f / _e1852));
            }
            let _e1857 = phi_5436_;
            global_20 = vec4<f32>(((_e1857.x + 1f) * 0.5f), ((_e1857.y + 1f) * 0.5f), ((_e1857.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1830 = sqrt(fma(_e1354.z, _e1354.z, fma(_e1354.x, _e1354.x, (_e1354.y * _e1354.y))));
            if (_e1830 == 0f) {
                phi_5387_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5387_ = (_e1354 * (1f / _e1830));
            }
            let _e1835 = phi_5387_;
            global_20 = vec4<f32>(((_e1835.x + 1f) * 0.5f), ((_e1835.y + 1f) * 0.5f), ((_e1835.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1808 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
            if (_e1808 == 0f) {
                phi_5338_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5338_ = (_e123 * (1f / _e1808));
            }
            let _e1813 = phi_5338_;
            global_20 = vec4<f32>(((_e1813.x + 1f) * 0.5f), ((_e1813.y + 1f) * 0.5f), ((_e1813.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1786 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
            if (_e1786 == 0f) {
                phi_5289_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5289_ = (_e124 * (1f / _e1786));
            }
            let _e1791 = phi_5289_;
            global_20 = vec4<f32>(((_e1791.x + 1f) * 0.5f), ((_e1791.y + 1f) * 0.5f), ((_e1791.z + 1f) * 0.5f), 1f);
            break;
        }
        case 9: {
            global_20 = vec4<f32>(_e1402.x, _e1402.y, _e1402.z, 1f);
            break;
        }
        case 10: {
            global_20 = vec4<f32>(_e1750.x, _e1750.y, _e1750.z, 1f);
            break;
        }
        case 11: {
            global_20 = vec4<f32>(_e1766.x, _e1766.y, 1f, 1f);
            break;
        }
        case 12: {
            global_20 = (vec4<f32>(_e1360, _e1363, _e1366, (_e493.w * _e296.member_2.w)) * _e119);
            break;
        }
        case 13: {
            global_20 = vec4<f32>(_e1380, _e1380, _e1380, 1f);
            break;
        }
        case 14: {
            global_20 = vec4<f32>(_e1383, _e1383, _e1383, 1f);
            break;
        }
        case 15: {
            global_20 = vec4<f32>(_e1387, _e1387, _e1387, 1f);
            break;
        }
        case 16: {
            global_20 = vec4<f32>((_e1393 * _e296.member_1), (_e1395 * _e296.member_1), (_e1397 * _e296.member_1), 1f);
            break;
        }
        case 17: {
            global_20 = vec4<f32>(_e1269.x, _e1269.y, _e1269.z, 1f);
            break;
        }
        case 18: {
            global_20 = vec4<f32>(_e296.member.x, _e296.member.y, _e296.member.z, 1f);
            break;
        }
        case 19: {
            global_20 = vec4<f32>(_e296.member_1, _e296.member_1, _e296.member_1, 1f);
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
