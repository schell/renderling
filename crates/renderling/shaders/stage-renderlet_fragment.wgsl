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
    var phi_631_: u32;
    var phi_4084_: bool;
    var phi_785_: type_32;
    var phi_789_: type_32;
    var phi_4121_: bool;
    var phi_829_: u32;
    var phi_838_: u32;
    var phi_851_: type_33;
    var phi_4143_: f32;
    var phi_4156_: bool;
    var phi_905_: f32;
    var phi_900_: f32;
    var phi_906_: f32;
    var phi_4173_: bool;
    var phi_871_: f32;
    var phi_908_: f32;
    var phi_4191_: f32;
    var phi_4204_: bool;
    var phi_963_: f32;
    var phi_958_: f32;
    var phi_964_: f32;
    var phi_4221_: bool;
    var phi_929_: f32;
    var phi_966_: f32;
    var phi_4257_: bool;
    var phi_1049_: u32;
    var phi_1058_: u32;
    var phi_1071_: type_33;
    var phi_4278_: f32;
    var phi_4291_: bool;
    var phi_1125_: f32;
    var phi_1120_: f32;
    var phi_1126_: f32;
    var phi_4308_: bool;
    var phi_1091_: f32;
    var phi_1128_: f32;
    var phi_4326_: f32;
    var phi_4339_: bool;
    var phi_1183_: f32;
    var phi_1178_: f32;
    var phi_1184_: f32;
    var phi_4356_: bool;
    var phi_1149_: f32;
    var phi_1186_: f32;
    var phi_4392_: bool;
    var phi_1269_: u32;
    var phi_1278_: u32;
    var phi_1291_: type_33;
    var phi_4413_: f32;
    var phi_4426_: bool;
    var phi_1345_: f32;
    var phi_1340_: f32;
    var phi_1346_: f32;
    var phi_4443_: bool;
    var phi_1311_: f32;
    var phi_1348_: f32;
    var phi_4461_: f32;
    var phi_4474_: bool;
    var phi_1403_: f32;
    var phi_1398_: f32;
    var phi_1404_: f32;
    var phi_4491_: bool;
    var phi_1369_: f32;
    var phi_1406_: f32;
    var phi_4527_: bool;
    var phi_1489_: u32;
    var phi_1498_: u32;
    var phi_1511_: type_33;
    var phi_4548_: f32;
    var phi_4561_: bool;
    var phi_1565_: f32;
    var phi_1560_: f32;
    var phi_1566_: f32;
    var phi_4578_: bool;
    var phi_1531_: f32;
    var phi_1568_: f32;
    var phi_4596_: f32;
    var phi_4609_: bool;
    var phi_1623_: f32;
    var phi_1618_: f32;
    var phi_1624_: f32;
    var phi_4626_: bool;
    var phi_1589_: f32;
    var phi_1626_: f32;
    var phi_4662_: bool;
    var phi_1709_: u32;
    var phi_1718_: u32;
    var phi_1731_: type_33;
    var phi_4683_: f32;
    var phi_4696_: bool;
    var phi_1785_: f32;
    var phi_1780_: f32;
    var phi_1786_: f32;
    var phi_4713_: bool;
    var phi_1751_: f32;
    var phi_1788_: f32;
    var phi_4731_: f32;
    var phi_4744_: bool;
    var phi_1843_: f32;
    var phi_1838_: f32;
    var phi_1844_: f32;
    var phi_4761_: bool;
    var phi_1809_: f32;
    var phi_1846_: f32;
    var phi_4819_: vec3<f32>;
    var phi_4854_: vec3<f32>;
    var phi_4889_: vec3<f32>;
    var phi_4924_: vec3<f32>;
    var phi_4959_: vec3<f32>;
    var phi_1940_: vec3<f32>;
    var phi_1941_: vec3<f32>;
    var phi_4991_: bool;
    var phi_2148_: type_24;
    var phi_2149_: type_24;
    var phi_2172_: type_24;
    var phi_2199_: bool;
    var phi_2205_: type_24;
    var phi_2206_: type_24;
    var phi_2229_: type_24;
    var phi_2252_: bool;
    var phi_2273_: type_22;
    var phi_5063_: vec3<f32>;
    var phi_5122_: vec3<f32>;
    var phi_5196_: vec3<f32>;
    var phi_5256_: vec3<f32>;
    var phi_5305_: vec3<f32>;
    var phi_5354_: vec3<f32>;
    var phi_5403_: vec3<f32>;
    var phi_5452_: vec3<f32>;
    var phi_5501_: vec3<f32>;
    var phi_5550_: vec3<f32>;
    var phi_5589_: vec3<f32>;
    var phi_5624_: vec3<f32>;
    var phi_2313_: type_24;
    var phi_2316_: vec3<f32>;
    var phi_2314_: type_24;
    var phi_2339_: type_24;
    var phi_5641_: u32;
    var phi_5660_: bool;
    var phi_2356_: u32;
    var phi_5692_: bool;
    var phi_2373_: u32;
    var phi_2383_: type_34;
    var phi_5722_: bool;
    var phi_2433_: type_29;
    var phi_5802_: bool;
    var phi_3072_: type_36;
    var phi_5852_: vec3<f32>;
    var phi_5887_: vec3<f32>;
    var phi_5922_: vec3<f32>;
    var phi_3327_: vec3<f32>;
    var phi_6014_: bool;
    var phi_2819_: type_35;
    var phi_6061_: vec3<f32>;
    var phi_6096_: vec3<f32>;
    var phi_3009_: vec3<f32>;
    var phi_6188_: bool;
    var phi_2481_: type_35;
    var phi_6235_: vec3<f32>;
    var phi_6270_: vec3<f32>;
    var phi_3329_: vec3<f32>;
    var phi_3330_: bool;
    var phi_3339_: vec3<f32>;
    var phi_2317_: vec3<f32>;
    var phi_3341_: bool;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var phi_3458_: vec4<f32>;

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
            phi_631_ = 0u;
            break;
        }
        case 1: {
            phi_631_ = 1u;
            break;
        }
        case 2: {
            phi_631_ = 2u;
            break;
        }
        case 3: {
            phi_631_ = 3u;
            break;
        }
        case 4: {
            phi_631_ = 4u;
            break;
        }
        case 5: {
            phi_631_ = 5u;
            break;
        }
        case 6: {
            phi_631_ = 6u;
            break;
        }
        case 7: {
            phi_631_ = 7u;
            break;
        }
        case 8: {
            phi_631_ = 8u;
            break;
        }
        case 9: {
            phi_631_ = 9u;
            break;
        }
        case 10: {
            phi_631_ = 10u;
            break;
        }
        case 11: {
            phi_631_ = 11u;
            break;
        }
        case 12: {
            phi_631_ = 12u;
            break;
        }
        case 13: {
            phi_631_ = 13u;
            break;
        }
        case 14: {
            phi_631_ = 14u;
            break;
        }
        case 15: {
            phi_631_ = 15u;
            break;
        }
        case 16: {
            phi_631_ = 16u;
            break;
        }
        case 17: {
            phi_631_ = 17u;
            break;
        }
        case 18: {
            phi_631_ = 18u;
            break;
        }
        case 19: {
            phi_631_ = 19u;
            break;
        }
        default: {
            phi_631_ = 0u;
            break;
        }
    }
    let _e151 = phi_631_;
    let _e155 = global.member[(_e137 + 5u)];
    let _e160 = global.member[(_e137 + 9u)];
    let _e164 = global.member[(_e137 + 10u)];
    if (_e133 == 4294967295u) {
        phi_789_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e117 >= 22u) {
            phi_4084_ = (_e133 <= (_e117 - 22u));
        } else {
            phi_4084_ = false;
        }
        let _e170 = phi_4084_;
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
            phi_785_ = type_32(vec3<f32>(bitcast<f32>(_e173), bitcast<f32>(_e178), bitcast<f32>(_e183)), bitcast<f32>(_e189), vec4<f32>(bitcast<f32>(_e194), bitcast<f32>(_e199), bitcast<f32>(_e204), bitcast<f32>(_e209)), bitcast<f32>(_e215), bitcast<f32>(_e220), _e225, _e229, _e233, _e237, _e241, _e245, _e249, _e253, _e257, _e261, (_e265 == 1u), bitcast<f32>(_e270));
        } else {
            phi_785_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e274 = phi_785_;
        phi_789_ = type_32(_e274.member, _e274.member_1, _e274.member_2, _e274.member_3, _e274.member_4, _e274.member_5, _e274.member_6, _e274.member_7, _e274.member_8, _e274.member_9, _e274.member_10, _e274.member_11, _e274.member_12, _e274.member_13, _e274.member_14, (_e274.member_15 && (_e155 == 1u)), _e274.member_16);
    }
    let _e296 = phi_789_;
    let _e300 = select(_e121, _e120, vec2((_e296.member_10 == 0u)));
    let _e302 = (_e117 >= 8u);
    if _e302 {
        phi_4121_ = (_e296.member_5 <= (_e117 - 8u));
    } else {
        phi_4121_ = false;
    }
    let _e306 = phi_4121_;
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
                phi_829_ = 0u;
                break;
            }
            case 1: {
                phi_829_ = 1u;
                break;
            }
            case 2: {
                phi_829_ = 2u;
                break;
            }
            default: {
                phi_829_ = 0u;
                break;
            }
        }
        let _e338 = phi_829_;
        let _e342 = global.member[(_e296.member_5 + 7u)];
        switch bitcast<i32>(_e342) {
            case 0: {
                phi_838_ = 0u;
                break;
            }
            case 1: {
                phi_838_ = 1u;
                break;
            }
            case 2: {
                phi_838_ = 2u;
                break;
            }
            default: {
                phi_838_ = 0u;
                break;
            }
        }
        let _e345 = phi_838_;
        phi_851_ = type_33(type_24(_e338, _e345), vec2<u32>(_e309, _e313), vec2<u32>(_e318, _e322), _e327, _e331);
    } else {
        phi_851_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e349 = phi_851_;
    switch bitcast<i32>(_e349.member.member) {
        case 1: {
            let _e387 = abs(_e300.x);
            let _e389 = (_e387 % 1f);
            if (_e387 >= 1f) {
                phi_4173_ = select(true, false, (_e389 == 0f));
            } else {
                phi_4173_ = true;
            }
            let _e393 = phi_4173_;
            let _e394 = select(1f, _e389, _e393);
            if (select(-1f, 1f, (_e300.x >= 0f)) > 0f) {
                phi_871_ = _e394;
            } else {
                phi_871_ = (1f - _e394);
            }
            let _e398 = phi_871_;
            phi_908_ = _e398;
            break;
        }
        case 2: {
            let _e361 = abs(_e300.x);
            let _e368 = ((select(select(u32(_e361), 0u, (_e361 < 0f)), 4294967295u, (_e361 > 4294967000f)) % 2u) == 0u);
            let _e370 = (_e361 % 1f);
            if (_e361 >= 1f) {
                phi_4156_ = select(true, false, (_e370 == 0f));
            } else {
                phi_4156_ = true;
            }
            let _e374 = phi_4156_;
            let _e375 = select(1f, _e370, _e374);
            if (select(-1f, 1f, (_e300.x >= 0f)) > 0f) {
                if _e368 {
                    phi_900_ = _e375;
                } else {
                    phi_900_ = (1f - _e375);
                }
                let _e382 = phi_900_;
                phi_906_ = _e382;
            } else {
                if _e368 {
                    phi_905_ = (1f - _e375);
                } else {
                    phi_905_ = _e375;
                }
                let _e379 = phi_905_;
                phi_906_ = _e379;
            }
            let _e384 = phi_906_;
            phi_908_ = _e384;
            break;
        }
        case 0: {
            if (_e300.x > 1f) {
                phi_4143_ = 0.9999999f;
            } else {
                phi_4143_ = select(_e300.x, 0.00000011920929f, (_e300.x < 0f));
            }
            let _e358 = phi_4143_;
            phi_908_ = _e358;
            break;
        }
        default: {
            phi_908_ = f32();
            break;
        }
    }
    let _e400 = phi_908_;
    switch bitcast<i32>(_e349.member.member_1) {
        case 1: {
            let _e438 = abs(_e300.y);
            let _e440 = (_e438 % 1f);
            if (_e438 >= 1f) {
                phi_4221_ = select(true, false, (_e440 == 0f));
            } else {
                phi_4221_ = true;
            }
            let _e444 = phi_4221_;
            let _e445 = select(1f, _e440, _e444);
            if (select(-1f, 1f, (_e300.y >= 0f)) > 0f) {
                phi_929_ = _e445;
            } else {
                phi_929_ = (1f - _e445);
            }
            let _e449 = phi_929_;
            phi_966_ = _e449;
            break;
        }
        case 2: {
            let _e412 = abs(_e300.y);
            let _e419 = ((select(select(u32(_e412), 0u, (_e412 < 0f)), 4294967295u, (_e412 > 4294967000f)) % 2u) == 0u);
            let _e421 = (_e412 % 1f);
            if (_e412 >= 1f) {
                phi_4204_ = select(true, false, (_e421 == 0f));
            } else {
                phi_4204_ = true;
            }
            let _e425 = phi_4204_;
            let _e426 = select(1f, _e421, _e425);
            if (select(-1f, 1f, (_e300.y >= 0f)) > 0f) {
                if _e419 {
                    phi_958_ = _e426;
                } else {
                    phi_958_ = (1f - _e426);
                }
                let _e433 = phi_958_;
                phi_964_ = _e433;
            } else {
                if _e419 {
                    phi_963_ = (1f - _e426);
                } else {
                    phi_963_ = _e426;
                }
                let _e430 = phi_963_;
                phi_964_ = _e430;
            }
            let _e435 = phi_964_;
            phi_966_ = _e435;
            break;
        }
        case 0: {
            if (_e300.y > 1f) {
                phi_4191_ = 0.9999999f;
            } else {
                phi_4191_ = select(_e300.y, 0.00000011920929f, (_e300.y < 0f));
            }
            let _e409 = phi_4191_;
            phi_966_ = _e409;
            break;
        }
        default: {
            phi_966_ = f32();
            break;
        }
    }
    let _e451 = phi_966_;
    let _e455 = (_e400 * f32(_e349.member_2.x));
    let _e464 = (_e451 * f32(_e349.member_2.y));
    let _e476 = f32(_e140);
    let _e477 = f32(_e144);
    let _e484 = vec3<f32>((f32((select(select(u32(_e455), 0u, (_e455 < 0f)), 4294967295u, (_e455 > 4294967000f)) + _e349.member_1.x)) / _e476), (f32((select(select(u32(_e464), 0u, (_e464 < 0f)), 4294967295u, (_e464 > 4294967000f)) + _e349.member_1.y)) / _e477), f32(_e349.member_3));
    let _e490 = textureSampleLevel(global_11, global_10, vec2<f32>(_e484.x, _e484.y), i32(_e484.z), 0f);
    let _e493 = select(_e490, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e296.member_5 == 4294967295u)));
    let _e497 = select(_e121, _e120, vec2((_e296.member_11 == 0u)));
    if _e302 {
        phi_4257_ = (_e296.member_6 <= (_e117 - 8u));
    } else {
        phi_4257_ = false;
    }
    let _e502 = phi_4257_;
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
                phi_1049_ = 0u;
                break;
            }
            case 1: {
                phi_1049_ = 1u;
                break;
            }
            case 2: {
                phi_1049_ = 2u;
                break;
            }
            default: {
                phi_1049_ = 0u;
                break;
            }
        }
        let _e534 = phi_1049_;
        let _e538 = global.member[(_e296.member_6 + 7u)];
        switch bitcast<i32>(_e538) {
            case 0: {
                phi_1058_ = 0u;
                break;
            }
            case 1: {
                phi_1058_ = 1u;
                break;
            }
            case 2: {
                phi_1058_ = 2u;
                break;
            }
            default: {
                phi_1058_ = 0u;
                break;
            }
        }
        let _e541 = phi_1058_;
        phi_1071_ = type_33(type_24(_e534, _e541), vec2<u32>(_e505, _e509), vec2<u32>(_e514, _e518), _e523, _e527);
    } else {
        phi_1071_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e545 = phi_1071_;
    switch bitcast<i32>(_e545.member.member) {
        case 1: {
            let _e583 = abs(_e497.x);
            let _e585 = (_e583 % 1f);
            if (_e583 >= 1f) {
                phi_4308_ = select(true, false, (_e585 == 0f));
            } else {
                phi_4308_ = true;
            }
            let _e589 = phi_4308_;
            let _e590 = select(1f, _e585, _e589);
            if (select(-1f, 1f, (_e497.x >= 0f)) > 0f) {
                phi_1091_ = _e590;
            } else {
                phi_1091_ = (1f - _e590);
            }
            let _e594 = phi_1091_;
            phi_1128_ = _e594;
            break;
        }
        case 2: {
            let _e557 = abs(_e497.x);
            let _e564 = ((select(select(u32(_e557), 0u, (_e557 < 0f)), 4294967295u, (_e557 > 4294967000f)) % 2u) == 0u);
            let _e566 = (_e557 % 1f);
            if (_e557 >= 1f) {
                phi_4291_ = select(true, false, (_e566 == 0f));
            } else {
                phi_4291_ = true;
            }
            let _e570 = phi_4291_;
            let _e571 = select(1f, _e566, _e570);
            if (select(-1f, 1f, (_e497.x >= 0f)) > 0f) {
                if _e564 {
                    phi_1120_ = _e571;
                } else {
                    phi_1120_ = (1f - _e571);
                }
                let _e578 = phi_1120_;
                phi_1126_ = _e578;
            } else {
                if _e564 {
                    phi_1125_ = (1f - _e571);
                } else {
                    phi_1125_ = _e571;
                }
                let _e575 = phi_1125_;
                phi_1126_ = _e575;
            }
            let _e580 = phi_1126_;
            phi_1128_ = _e580;
            break;
        }
        case 0: {
            if (_e497.x > 1f) {
                phi_4278_ = 0.9999999f;
            } else {
                phi_4278_ = select(_e497.x, 0.00000011920929f, (_e497.x < 0f));
            }
            let _e554 = phi_4278_;
            phi_1128_ = _e554;
            break;
        }
        default: {
            phi_1128_ = f32();
            break;
        }
    }
    let _e596 = phi_1128_;
    switch bitcast<i32>(_e545.member.member_1) {
        case 1: {
            let _e634 = abs(_e497.y);
            let _e636 = (_e634 % 1f);
            if (_e634 >= 1f) {
                phi_4356_ = select(true, false, (_e636 == 0f));
            } else {
                phi_4356_ = true;
            }
            let _e640 = phi_4356_;
            let _e641 = select(1f, _e636, _e640);
            if (select(-1f, 1f, (_e497.y >= 0f)) > 0f) {
                phi_1149_ = _e641;
            } else {
                phi_1149_ = (1f - _e641);
            }
            let _e645 = phi_1149_;
            phi_1186_ = _e645;
            break;
        }
        case 2: {
            let _e608 = abs(_e497.y);
            let _e615 = ((select(select(u32(_e608), 0u, (_e608 < 0f)), 4294967295u, (_e608 > 4294967000f)) % 2u) == 0u);
            let _e617 = (_e608 % 1f);
            if (_e608 >= 1f) {
                phi_4339_ = select(true, false, (_e617 == 0f));
            } else {
                phi_4339_ = true;
            }
            let _e621 = phi_4339_;
            let _e622 = select(1f, _e617, _e621);
            if (select(-1f, 1f, (_e497.y >= 0f)) > 0f) {
                if _e615 {
                    phi_1178_ = _e622;
                } else {
                    phi_1178_ = (1f - _e622);
                }
                let _e629 = phi_1178_;
                phi_1184_ = _e629;
            } else {
                if _e615 {
                    phi_1183_ = (1f - _e622);
                } else {
                    phi_1183_ = _e622;
                }
                let _e626 = phi_1183_;
                phi_1184_ = _e626;
            }
            let _e631 = phi_1184_;
            phi_1186_ = _e631;
            break;
        }
        case 0: {
            if (_e497.y > 1f) {
                phi_4326_ = 0.9999999f;
            } else {
                phi_4326_ = select(_e497.y, 0.00000011920929f, (_e497.y < 0f));
            }
            let _e605 = phi_4326_;
            phi_1186_ = _e605;
            break;
        }
        default: {
            phi_1186_ = f32();
            break;
        }
    }
    let _e647 = phi_1186_;
    let _e651 = (_e596 * f32(_e545.member_2.x));
    let _e660 = (_e647 * f32(_e545.member_2.y));
    let _e678 = vec3<f32>((f32((select(select(u32(_e651), 0u, (_e651 < 0f)), 4294967295u, (_e651 > 4294967000f)) + _e545.member_1.x)) / _e476), (f32((select(select(u32(_e660), 0u, (_e660 < 0f)), 4294967295u, (_e660 > 4294967000f)) + _e545.member_1.y)) / _e477), f32(_e545.member_3));
    let _e684 = textureSampleLevel(global_11, global_10, vec2<f32>(_e678.x, _e678.y), i32(_e678.z), 0f);
    let _e687 = select(_e684, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e296.member_6 == 4294967295u)));
    let _e691 = select(_e121, _e120, vec2((_e296.member_12 == 0u)));
    if _e302 {
        phi_4392_ = (_e296.member_7 <= (_e117 - 8u));
    } else {
        phi_4392_ = false;
    }
    let _e696 = phi_4392_;
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
                phi_1269_ = 0u;
                break;
            }
            case 1: {
                phi_1269_ = 1u;
                break;
            }
            case 2: {
                phi_1269_ = 2u;
                break;
            }
            default: {
                phi_1269_ = 0u;
                break;
            }
        }
        let _e728 = phi_1269_;
        let _e732 = global.member[(_e296.member_7 + 7u)];
        switch bitcast<i32>(_e732) {
            case 0: {
                phi_1278_ = 0u;
                break;
            }
            case 1: {
                phi_1278_ = 1u;
                break;
            }
            case 2: {
                phi_1278_ = 2u;
                break;
            }
            default: {
                phi_1278_ = 0u;
                break;
            }
        }
        let _e735 = phi_1278_;
        phi_1291_ = type_33(type_24(_e728, _e735), vec2<u32>(_e699, _e703), vec2<u32>(_e708, _e712), _e717, _e721);
    } else {
        phi_1291_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e739 = phi_1291_;
    switch bitcast<i32>(_e739.member.member) {
        case 1: {
            let _e777 = abs(_e691.x);
            let _e779 = (_e777 % 1f);
            if (_e777 >= 1f) {
                phi_4443_ = select(true, false, (_e779 == 0f));
            } else {
                phi_4443_ = true;
            }
            let _e783 = phi_4443_;
            let _e784 = select(1f, _e779, _e783);
            if (select(-1f, 1f, (_e691.x >= 0f)) > 0f) {
                phi_1311_ = _e784;
            } else {
                phi_1311_ = (1f - _e784);
            }
            let _e788 = phi_1311_;
            phi_1348_ = _e788;
            break;
        }
        case 2: {
            let _e751 = abs(_e691.x);
            let _e758 = ((select(select(u32(_e751), 0u, (_e751 < 0f)), 4294967295u, (_e751 > 4294967000f)) % 2u) == 0u);
            let _e760 = (_e751 % 1f);
            if (_e751 >= 1f) {
                phi_4426_ = select(true, false, (_e760 == 0f));
            } else {
                phi_4426_ = true;
            }
            let _e764 = phi_4426_;
            let _e765 = select(1f, _e760, _e764);
            if (select(-1f, 1f, (_e691.x >= 0f)) > 0f) {
                if _e758 {
                    phi_1340_ = _e765;
                } else {
                    phi_1340_ = (1f - _e765);
                }
                let _e772 = phi_1340_;
                phi_1346_ = _e772;
            } else {
                if _e758 {
                    phi_1345_ = (1f - _e765);
                } else {
                    phi_1345_ = _e765;
                }
                let _e769 = phi_1345_;
                phi_1346_ = _e769;
            }
            let _e774 = phi_1346_;
            phi_1348_ = _e774;
            break;
        }
        case 0: {
            if (_e691.x > 1f) {
                phi_4413_ = 0.9999999f;
            } else {
                phi_4413_ = select(_e691.x, 0.00000011920929f, (_e691.x < 0f));
            }
            let _e748 = phi_4413_;
            phi_1348_ = _e748;
            break;
        }
        default: {
            phi_1348_ = f32();
            break;
        }
    }
    let _e790 = phi_1348_;
    switch bitcast<i32>(_e739.member.member_1) {
        case 1: {
            let _e828 = abs(_e691.y);
            let _e830 = (_e828 % 1f);
            if (_e828 >= 1f) {
                phi_4491_ = select(true, false, (_e830 == 0f));
            } else {
                phi_4491_ = true;
            }
            let _e834 = phi_4491_;
            let _e835 = select(1f, _e830, _e834);
            if (select(-1f, 1f, (_e691.y >= 0f)) > 0f) {
                phi_1369_ = _e835;
            } else {
                phi_1369_ = (1f - _e835);
            }
            let _e839 = phi_1369_;
            phi_1406_ = _e839;
            break;
        }
        case 2: {
            let _e802 = abs(_e691.y);
            let _e809 = ((select(select(u32(_e802), 0u, (_e802 < 0f)), 4294967295u, (_e802 > 4294967000f)) % 2u) == 0u);
            let _e811 = (_e802 % 1f);
            if (_e802 >= 1f) {
                phi_4474_ = select(true, false, (_e811 == 0f));
            } else {
                phi_4474_ = true;
            }
            let _e815 = phi_4474_;
            let _e816 = select(1f, _e811, _e815);
            if (select(-1f, 1f, (_e691.y >= 0f)) > 0f) {
                if _e809 {
                    phi_1398_ = _e816;
                } else {
                    phi_1398_ = (1f - _e816);
                }
                let _e823 = phi_1398_;
                phi_1404_ = _e823;
            } else {
                if _e809 {
                    phi_1403_ = (1f - _e816);
                } else {
                    phi_1403_ = _e816;
                }
                let _e820 = phi_1403_;
                phi_1404_ = _e820;
            }
            let _e825 = phi_1404_;
            phi_1406_ = _e825;
            break;
        }
        case 0: {
            if (_e691.y > 1f) {
                phi_4461_ = 0.9999999f;
            } else {
                phi_4461_ = select(_e691.y, 0.00000011920929f, (_e691.y < 0f));
            }
            let _e799 = phi_4461_;
            phi_1406_ = _e799;
            break;
        }
        default: {
            phi_1406_ = f32();
            break;
        }
    }
    let _e841 = phi_1406_;
    let _e845 = (_e790 * f32(_e739.member_2.x));
    let _e854 = (_e841 * f32(_e739.member_2.y));
    let _e872 = vec3<f32>((f32((select(select(u32(_e845), 0u, (_e845 < 0f)), 4294967295u, (_e845 > 4294967000f)) + _e739.member_1.x)) / _e476), (f32((select(select(u32(_e854), 0u, (_e854 < 0f)), 4294967295u, (_e854 > 4294967000f)) + _e739.member_1.y)) / _e477), f32(_e739.member_3));
    let _e878 = textureSampleLevel(global_11, global_10, vec2<f32>(_e872.x, _e872.y), i32(_e872.z), 0f);
    let _e879 = (_e296.member_7 == 4294967295u);
    let _e881 = select(_e878, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e879));
    let _e885 = select(_e121, _e120, vec2((_e296.member_13 == 0u)));
    if _e302 {
        phi_4527_ = (_e296.member_8 <= (_e117 - 8u));
    } else {
        phi_4527_ = false;
    }
    let _e890 = phi_4527_;
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
                phi_1489_ = 0u;
                break;
            }
            case 1: {
                phi_1489_ = 1u;
                break;
            }
            case 2: {
                phi_1489_ = 2u;
                break;
            }
            default: {
                phi_1489_ = 0u;
                break;
            }
        }
        let _e922 = phi_1489_;
        let _e926 = global.member[(_e296.member_8 + 7u)];
        switch bitcast<i32>(_e926) {
            case 0: {
                phi_1498_ = 0u;
                break;
            }
            case 1: {
                phi_1498_ = 1u;
                break;
            }
            case 2: {
                phi_1498_ = 2u;
                break;
            }
            default: {
                phi_1498_ = 0u;
                break;
            }
        }
        let _e929 = phi_1498_;
        phi_1511_ = type_33(type_24(_e922, _e929), vec2<u32>(_e893, _e897), vec2<u32>(_e902, _e906), _e911, _e915);
    } else {
        phi_1511_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e933 = phi_1511_;
    switch bitcast<i32>(_e933.member.member) {
        case 1: {
            let _e971 = abs(_e885.x);
            let _e973 = (_e971 % 1f);
            if (_e971 >= 1f) {
                phi_4578_ = select(true, false, (_e973 == 0f));
            } else {
                phi_4578_ = true;
            }
            let _e977 = phi_4578_;
            let _e978 = select(1f, _e973, _e977);
            if (select(-1f, 1f, (_e885.x >= 0f)) > 0f) {
                phi_1531_ = _e978;
            } else {
                phi_1531_ = (1f - _e978);
            }
            let _e982 = phi_1531_;
            phi_1568_ = _e982;
            break;
        }
        case 2: {
            let _e945 = abs(_e885.x);
            let _e952 = ((select(select(u32(_e945), 0u, (_e945 < 0f)), 4294967295u, (_e945 > 4294967000f)) % 2u) == 0u);
            let _e954 = (_e945 % 1f);
            if (_e945 >= 1f) {
                phi_4561_ = select(true, false, (_e954 == 0f));
            } else {
                phi_4561_ = true;
            }
            let _e958 = phi_4561_;
            let _e959 = select(1f, _e954, _e958);
            if (select(-1f, 1f, (_e885.x >= 0f)) > 0f) {
                if _e952 {
                    phi_1560_ = _e959;
                } else {
                    phi_1560_ = (1f - _e959);
                }
                let _e966 = phi_1560_;
                phi_1566_ = _e966;
            } else {
                if _e952 {
                    phi_1565_ = (1f - _e959);
                } else {
                    phi_1565_ = _e959;
                }
                let _e963 = phi_1565_;
                phi_1566_ = _e963;
            }
            let _e968 = phi_1566_;
            phi_1568_ = _e968;
            break;
        }
        case 0: {
            if (_e885.x > 1f) {
                phi_4548_ = 0.9999999f;
            } else {
                phi_4548_ = select(_e885.x, 0.00000011920929f, (_e885.x < 0f));
            }
            let _e942 = phi_4548_;
            phi_1568_ = _e942;
            break;
        }
        default: {
            phi_1568_ = f32();
            break;
        }
    }
    let _e984 = phi_1568_;
    switch bitcast<i32>(_e933.member.member_1) {
        case 1: {
            let _e1022 = abs(_e885.y);
            let _e1024 = (_e1022 % 1f);
            if (_e1022 >= 1f) {
                phi_4626_ = select(true, false, (_e1024 == 0f));
            } else {
                phi_4626_ = true;
            }
            let _e1028 = phi_4626_;
            let _e1029 = select(1f, _e1024, _e1028);
            if (select(-1f, 1f, (_e885.y >= 0f)) > 0f) {
                phi_1589_ = _e1029;
            } else {
                phi_1589_ = (1f - _e1029);
            }
            let _e1033 = phi_1589_;
            phi_1626_ = _e1033;
            break;
        }
        case 2: {
            let _e996 = abs(_e885.y);
            let _e1003 = ((select(select(u32(_e996), 0u, (_e996 < 0f)), 4294967295u, (_e996 > 4294967000f)) % 2u) == 0u);
            let _e1005 = (_e996 % 1f);
            if (_e996 >= 1f) {
                phi_4609_ = select(true, false, (_e1005 == 0f));
            } else {
                phi_4609_ = true;
            }
            let _e1009 = phi_4609_;
            let _e1010 = select(1f, _e1005, _e1009);
            if (select(-1f, 1f, (_e885.y >= 0f)) > 0f) {
                if _e1003 {
                    phi_1618_ = _e1010;
                } else {
                    phi_1618_ = (1f - _e1010);
                }
                let _e1017 = phi_1618_;
                phi_1624_ = _e1017;
            } else {
                if _e1003 {
                    phi_1623_ = (1f - _e1010);
                } else {
                    phi_1623_ = _e1010;
                }
                let _e1014 = phi_1623_;
                phi_1624_ = _e1014;
            }
            let _e1019 = phi_1624_;
            phi_1626_ = _e1019;
            break;
        }
        case 0: {
            if (_e885.y > 1f) {
                phi_4596_ = 0.9999999f;
            } else {
                phi_4596_ = select(_e885.y, 0.00000011920929f, (_e885.y < 0f));
            }
            let _e993 = phi_4596_;
            phi_1626_ = _e993;
            break;
        }
        default: {
            phi_1626_ = f32();
            break;
        }
    }
    let _e1035 = phi_1626_;
    let _e1039 = (_e984 * f32(_e933.member_2.x));
    let _e1048 = (_e1035 * f32(_e933.member_2.y));
    let _e1066 = vec3<f32>((f32((select(select(u32(_e1039), 0u, (_e1039 < 0f)), 4294967295u, (_e1039 > 4294967000f)) + _e933.member_1.x)) / _e476), (f32((select(select(u32(_e1048), 0u, (_e1048 < 0f)), 4294967295u, (_e1048 > 4294967000f)) + _e933.member_1.y)) / _e477), f32(_e933.member_3));
    let _e1072 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1066.x, _e1066.y), i32(_e1066.z), 0f);
    let _e1079 = select(_e121, _e120, vec2((_e296.member_14 == 0u)));
    if _e302 {
        phi_4662_ = (_e296.member_9 <= (_e117 - 8u));
    } else {
        phi_4662_ = false;
    }
    let _e1084 = phi_4662_;
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
                phi_1709_ = 0u;
                break;
            }
            case 1: {
                phi_1709_ = 1u;
                break;
            }
            case 2: {
                phi_1709_ = 2u;
                break;
            }
            default: {
                phi_1709_ = 0u;
                break;
            }
        }
        let _e1116 = phi_1709_;
        let _e1120 = global.member[(_e296.member_9 + 7u)];
        switch bitcast<i32>(_e1120) {
            case 0: {
                phi_1718_ = 0u;
                break;
            }
            case 1: {
                phi_1718_ = 1u;
                break;
            }
            case 2: {
                phi_1718_ = 2u;
                break;
            }
            default: {
                phi_1718_ = 0u;
                break;
            }
        }
        let _e1123 = phi_1718_;
        phi_1731_ = type_33(type_24(_e1116, _e1123), vec2<u32>(_e1087, _e1091), vec2<u32>(_e1096, _e1100), _e1105, _e1109);
    } else {
        phi_1731_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1127 = phi_1731_;
    switch bitcast<i32>(_e1127.member.member) {
        case 1: {
            let _e1165 = abs(_e1079.x);
            let _e1167 = (_e1165 % 1f);
            if (_e1165 >= 1f) {
                phi_4713_ = select(true, false, (_e1167 == 0f));
            } else {
                phi_4713_ = true;
            }
            let _e1171 = phi_4713_;
            let _e1172 = select(1f, _e1167, _e1171);
            if (select(-1f, 1f, (_e1079.x >= 0f)) > 0f) {
                phi_1751_ = _e1172;
            } else {
                phi_1751_ = (1f - _e1172);
            }
            let _e1176 = phi_1751_;
            phi_1788_ = _e1176;
            break;
        }
        case 2: {
            let _e1139 = abs(_e1079.x);
            let _e1146 = ((select(select(u32(_e1139), 0u, (_e1139 < 0f)), 4294967295u, (_e1139 > 4294967000f)) % 2u) == 0u);
            let _e1148 = (_e1139 % 1f);
            if (_e1139 >= 1f) {
                phi_4696_ = select(true, false, (_e1148 == 0f));
            } else {
                phi_4696_ = true;
            }
            let _e1152 = phi_4696_;
            let _e1153 = select(1f, _e1148, _e1152);
            if (select(-1f, 1f, (_e1079.x >= 0f)) > 0f) {
                if _e1146 {
                    phi_1780_ = _e1153;
                } else {
                    phi_1780_ = (1f - _e1153);
                }
                let _e1160 = phi_1780_;
                phi_1786_ = _e1160;
            } else {
                if _e1146 {
                    phi_1785_ = (1f - _e1153);
                } else {
                    phi_1785_ = _e1153;
                }
                let _e1157 = phi_1785_;
                phi_1786_ = _e1157;
            }
            let _e1162 = phi_1786_;
            phi_1788_ = _e1162;
            break;
        }
        case 0: {
            if (_e1079.x > 1f) {
                phi_4683_ = 0.9999999f;
            } else {
                phi_4683_ = select(_e1079.x, 0.00000011920929f, (_e1079.x < 0f));
            }
            let _e1136 = phi_4683_;
            phi_1788_ = _e1136;
            break;
        }
        default: {
            phi_1788_ = f32();
            break;
        }
    }
    let _e1178 = phi_1788_;
    switch bitcast<i32>(_e1127.member.member_1) {
        case 1: {
            let _e1216 = abs(_e1079.y);
            let _e1218 = (_e1216 % 1f);
            if (_e1216 >= 1f) {
                phi_4761_ = select(true, false, (_e1218 == 0f));
            } else {
                phi_4761_ = true;
            }
            let _e1222 = phi_4761_;
            let _e1223 = select(1f, _e1218, _e1222);
            if (select(-1f, 1f, (_e1079.y >= 0f)) > 0f) {
                phi_1809_ = _e1223;
            } else {
                phi_1809_ = (1f - _e1223);
            }
            let _e1227 = phi_1809_;
            phi_1846_ = _e1227;
            break;
        }
        case 2: {
            let _e1190 = abs(_e1079.y);
            let _e1197 = ((select(select(u32(_e1190), 0u, (_e1190 < 0f)), 4294967295u, (_e1190 > 4294967000f)) % 2u) == 0u);
            let _e1199 = (_e1190 % 1f);
            if (_e1190 >= 1f) {
                phi_4744_ = select(true, false, (_e1199 == 0f));
            } else {
                phi_4744_ = true;
            }
            let _e1203 = phi_4744_;
            let _e1204 = select(1f, _e1199, _e1203);
            if (select(-1f, 1f, (_e1079.y >= 0f)) > 0f) {
                if _e1197 {
                    phi_1838_ = _e1204;
                } else {
                    phi_1838_ = (1f - _e1204);
                }
                let _e1211 = phi_1838_;
                phi_1844_ = _e1211;
            } else {
                if _e1197 {
                    phi_1843_ = (1f - _e1204);
                } else {
                    phi_1843_ = _e1204;
                }
                let _e1208 = phi_1843_;
                phi_1844_ = _e1208;
            }
            let _e1213 = phi_1844_;
            phi_1846_ = _e1213;
            break;
        }
        case 0: {
            if (_e1079.y > 1f) {
                phi_4731_ = 0.9999999f;
            } else {
                phi_4731_ = select(_e1079.y, 0.00000011920929f, (_e1079.y < 0f));
            }
            let _e1187 = phi_4731_;
            phi_1846_ = _e1187;
            break;
        }
        default: {
            phi_1846_ = f32();
            break;
        }
    }
    let _e1229 = phi_1846_;
    let _e1233 = (_e1178 * f32(_e1127.member_2.x));
    let _e1242 = (_e1229 * f32(_e1127.member_2.y));
    let _e1260 = vec3<f32>((f32((select(select(u32(_e1233), 0u, (_e1233 < 0f)), 4294967295u, (_e1233 > 4294967000f)) + _e1127.member_1.x)) / _e476), (f32((select(select(u32(_e1242), 0u, (_e1242 < 0f)), 4294967295u, (_e1242 > 4294967000f)) + _e1127.member_1.y)) / _e477), f32(_e1127.member_3));
    let _e1266 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1260.x, _e1260.y), i32(_e1260.z), 0f);
    let _e1269 = select(_e1266, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e296.member_9 == 4294967295u)));
    if _e879 {
        phi_1940_ = vec3<f32>(0f, 0f, 0f);
        phi_1941_ = _e122;
    } else {
        let _e1273 = fma(_e881.x, 2f, -1f);
        let _e1274 = fma(_e881.y, 2f, -1f);
        let _e1275 = fma(_e881.z, 2f, -1f);
        let _e1280 = sqrt(fma(_e1275, _e1275, fma(_e1273, _e1273, (_e1274 * _e1274))));
        if (_e1280 == 0f) {
            phi_4819_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4819_ = (vec3<f32>(_e1273, _e1274, _e1275) * (1f / _e1280));
        }
        let _e1285 = phi_4819_;
        let _e1292 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
        if (_e1292 == 0f) {
            phi_4854_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4854_ = (_e123 * (1f / _e1292));
        }
        let _e1297 = phi_4854_;
        let _e1304 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
        if (_e1304 == 0f) {
            phi_4889_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4889_ = (_e124 * (1f / _e1304));
        }
        let _e1309 = phi_4889_;
        let _e1316 = sqrt(fma(_e122.z, _e122.z, fma(_e122.x, _e122.x, (_e122.y * _e122.y))));
        if (_e1316 == 0f) {
            phi_4924_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4924_ = (_e122 * (1f / _e1316));
        }
        let _e1321 = phi_4924_;
        let _e1340 = fma(_e1321.x, _e1285.z, fma(_e1297.x, _e1285.x, (_e1309.x * _e1285.y)));
        let _e1341 = fma(_e1321.y, _e1285.z, fma(_e1297.y, _e1285.x, (_e1309.y * _e1285.y)));
        let _e1342 = fma(_e1321.z, _e1285.z, fma(_e1297.z, _e1285.x, (_e1309.z * _e1285.y)));
        let _e1347 = sqrt(fma(_e1342, _e1342, fma(_e1340, _e1340, (_e1341 * _e1341))));
        if (_e1347 == 0f) {
            phi_4959_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4959_ = (vec3<f32>(_e1340, _e1341, _e1342) * (1f / _e1347));
        }
        let _e1352 = phi_4959_;
        phi_1940_ = _e1285;
        phi_1941_ = _e1352;
    }
    let _e1354 = phi_1940_;
    let _e1356 = phi_1941_;
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
        phi_4991_ = (_e129 <= (_e117 - 86u));
    } else {
        phi_4991_ = false;
    }
    let _e1410 = phi_4991_;
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
        phi_2148_ = type_24(0u, 6u);
        loop {
            let _e1598 = phi_2148_;
            if (_e1598.member < _e1598.member_1) {
                phi_2149_ = type_24((_e1598.member + 1u), _e1598.member_1);
                phi_2172_ = type_24(1u, _e1598.member);
            } else {
                phi_2149_ = _e1598;
                phi_2172_ = type_24(0u, type_24().member_1);
            }
            let _e1611 = phi_2149_;
            let _e1613 = phi_2172_;
            switch bitcast<i32>(_e1613.member) {
                case 0: {
                    phi_2199_ = false;
                    break;
                }
                case 1: {
                    let _e1618 = ((_e129 + 35u) + (_e1613.member_1 * 4u));
                    let _e1621 = global.member[_e1618];
                    let _e1626 = global.member[(_e1618 + 1u)];
                    let _e1631 = global.member[(_e1618 + 2u)];
                    let _e1636 = global.member[(_e1618 + 3u)];
                    local_1[_e1613.member_1] = vec4<f32>(bitcast<f32>(_e1621), bitcast<f32>(_e1626), bitcast<f32>(_e1631), bitcast<f32>(_e1636));
                    phi_2199_ = true;
                    break;
                }
                default: {
                    phi_2199_ = bool();
                    break;
                }
            }
            let _e1641 = phi_2199_;
            continue;
            continuing {
                phi_2148_ = _e1611;
                break if !(_e1641);
            }
        }
        let _e1643 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2205_ = type_24(0u, 8u);
        loop {
            let _e1646 = phi_2205_;
            if (_e1646.member < _e1646.member_1) {
                phi_2206_ = type_24((_e1646.member + 1u), _e1646.member_1);
                phi_2229_ = type_24(1u, _e1646.member);
            } else {
                phi_2206_ = _e1646;
                phi_2229_ = type_24(0u, type_24().member_1);
            }
            let _e1659 = phi_2206_;
            let _e1661 = phi_2229_;
            switch bitcast<i32>(_e1661.member) {
                case 0: {
                    phi_2252_ = false;
                    break;
                }
                case 1: {
                    let _e1666 = ((_e129 + 59u) + (_e1661.member_1 * 3u));
                    let _e1669 = global.member[_e1666];
                    let _e1674 = global.member[(_e1666 + 1u)];
                    let _e1679 = global.member[(_e1666 + 2u)];
                    local[_e1661.member_1] = vec3<f32>(bitcast<f32>(_e1669), bitcast<f32>(_e1674), bitcast<f32>(_e1679));
                    phi_2252_ = true;
                    break;
                }
                default: {
                    phi_2252_ = bool();
                    break;
                }
            }
            let _e1684 = phi_2252_;
            continue;
            continuing {
                phi_2205_ = _e1659;
                break if !(_e1684);
            }
        }
        let _e1686 = local;
        let _e1690 = global.member[(_e129 + 83u)];
        let _e1695 = global.member[(_e129 + 84u)];
        let _e1700 = global.member[(_e129 + 85u)];
        phi_2273_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1413), bitcast<f32>(_e1418), bitcast<f32>(_e1423), bitcast<f32>(_e1428)), vec4<f32>(bitcast<f32>(_e1434), bitcast<f32>(_e1439), bitcast<f32>(_e1444), bitcast<f32>(_e1449)), vec4<f32>(bitcast<f32>(_e1455), bitcast<f32>(_e1460), bitcast<f32>(_e1465), bitcast<f32>(_e1470)), vec4<f32>(bitcast<f32>(_e1476), bitcast<f32>(_e1481), bitcast<f32>(_e1486), bitcast<f32>(_e1491))), type_20(vec4<f32>(bitcast<f32>(_e1498), bitcast<f32>(_e1503), bitcast<f32>(_e1508), bitcast<f32>(_e1513)), vec4<f32>(bitcast<f32>(_e1519), bitcast<f32>(_e1524), bitcast<f32>(_e1529), bitcast<f32>(_e1534)), vec4<f32>(bitcast<f32>(_e1540), bitcast<f32>(_e1545), bitcast<f32>(_e1550), bitcast<f32>(_e1555)), vec4<f32>(bitcast<f32>(_e1561), bitcast<f32>(_e1566), bitcast<f32>(_e1571), bitcast<f32>(_e1576))), vec3<f32>(bitcast<f32>(_e1583), bitcast<f32>(_e1588), bitcast<f32>(_e1593)), type_21(_e1686, _e1643, vec3<f32>(bitcast<f32>(_e1690), bitcast<f32>(_e1695), bitcast<f32>(_e1700))));
    } else {
        phi_2273_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e1706 = phi_2273_;
    let _e1708 = (_e1706.member_2 - _e125);
    let _e1715 = sqrt(fma(_e1708.z, _e1708.z, fma(_e1708.x, _e1708.x, (_e1708.y * _e1708.y))));
    let _e1716 = (_e1715 == 0f);
    if _e1716 {
        phi_5063_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5063_ = (_e1708 * (1f / _e1715));
    }
    let _e1720 = phi_5063_;
    let _e1721 = -(_e1720);
    let _e1728 = sqrt(fma(_e1356.z, _e1356.z, fma(_e1356.x, _e1356.x, (_e1356.y * _e1356.y))));
    let _e1729 = (_e1728 == 0f);
    if _e1729 {
        phi_5122_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5122_ = (_e1356 * (1f / _e1728));
    }
    let _e1733 = phi_5122_;
    let _e1743 = (2f * fma(_e1733.z, _e1721.z, fma(_e1733.x, _e1721.x, (_e1733.y * _e1721.y))));
    let _e1750 = textureSampleLevel(global_14, global_15, (_e1721 - vec3<f32>((_e1743 * _e1733.x), (_e1743 * _e1733.y), (_e1743 * _e1733.z))), (_e1380 * 4f));
    if _e1716 {
        phi_5196_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5196_ = (_e1708 * (1f / _e1715));
    }
    let _e1757 = phi_5196_;
    let _e1766 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1356.z, _e1757.z, fma(_e1356.x, _e1757.x, (_e1356.y * _e1757.y))), 0f), _e1380), 0f);
    switch bitcast<i32>(_e151) {
        case 0: {
            if _e296.member_15 {
                if _e1729 {
                    phi_5589_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5589_ = (_e1356 * (1f / _e1728));
                }
                let _e1935 = phi_5589_;
                if _e1716 {
                    phi_5624_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5624_ = (_e1708 * (1f / _e1715));
                }
                let _e1939 = phi_5624_;
                phi_2313_ = type_24(0u, _e164);
                phi_2316_ = vec3<f32>(0f, 0f, 0f);
                loop {
                    let _e1942 = phi_2313_;
                    let _e1944 = phi_2316_;
                    local_2 = _e1944;
                    local_3 = _e1944;
                    local_4 = _e1944;
                    if (_e1942.member < _e1942.member_1) {
                        phi_2314_ = type_24((_e1942.member + 1u), _e1942.member_1);
                        phi_2339_ = type_24(1u, _e1942.member);
                    } else {
                        phi_2314_ = _e1942;
                        phi_2339_ = type_24(0u, type_24().member_1);
                    }
                    let _e1957 = phi_2314_;
                    let _e1959 = phi_2339_;
                    switch bitcast<i32>(_e1959.member) {
                        case 0: {
                            phi_2317_ = vec3<f32>();
                            phi_3341_ = false;
                            break;
                        }
                        case 1: {
                            if (_e1959.member_1 >= _e164) {
                                phi_5641_ = 4294967295u;
                            } else {
                                phi_5641_ = (_e160 + _e1959.member_1);
                            }
                            let _e1966 = phi_5641_;
                            if (_e117 >= 1u) {
                                phi_5660_ = (_e1966 <= (_e117 - 1u));
                            } else {
                                phi_5660_ = false;
                            }
                            let _e1971 = phi_5660_;
                            if _e1971 {
                                let _e1974 = global.member[_e1966];
                                phi_2356_ = _e1974;
                            } else {
                                phi_2356_ = 4294967295u;
                            }
                            let _e1976 = phi_2356_;
                            let _e1977 = (_e1976 == 4294967295u);
                            if _e1977 {
                                phi_3339_ = vec3<f32>();
                            } else {
                                if (_e117 >= 3u) {
                                    phi_5692_ = (_e1976 <= (_e117 - 3u));
                                } else {
                                    phi_5692_ = false;
                                }
                                let _e1982 = phi_5692_;
                                if _e1982 {
                                    let _e1985 = global.member[_e1976];
                                    switch bitcast<i32>(_e1985) {
                                        case 0: {
                                            phi_2373_ = 0u;
                                            break;
                                        }
                                        case 1: {
                                            phi_2373_ = 1u;
                                            break;
                                        }
                                        case 2: {
                                            phi_2373_ = 2u;
                                            break;
                                        }
                                        default: {
                                            phi_2373_ = 0u;
                                            break;
                                        }
                                    }
                                    let _e1988 = phi_2373_;
                                    let _e1992 = global.member[(_e1976 + 1u)];
                                    let _e1996 = global.member[(_e1976 + 2u)];
                                    phi_2383_ = type_34(_e1988, _e1992, _e1996);
                                } else {
                                    phi_2383_ = type_34(0u, 4294967295u, 4294967295u);
                                }
                                let _e1999 = phi_2383_;
                                if (_e117 >= 10u) {
                                    phi_5722_ = (_e1999.member_2 <= (_e117 - 10u));
                                } else {
                                    phi_5722_ = false;
                                }
                                let _e2005 = phi_5722_;
                                if _e2005 {
                                    let _e2008 = global.member[_e1999.member_2];
                                    let _e2013 = global.member[(_e1999.member_2 + 1u)];
                                    let _e2018 = global.member[(_e1999.member_2 + 2u)];
                                    let _e2024 = global.member[(_e1999.member_2 + 3u)];
                                    let _e2029 = global.member[(_e1999.member_2 + 4u)];
                                    let _e2034 = global.member[(_e1999.member_2 + 5u)];
                                    let _e2039 = global.member[(_e1999.member_2 + 6u)];
                                    let _e2045 = global.member[(_e1999.member_2 + 7u)];
                                    let _e2050 = global.member[(_e1999.member_2 + 8u)];
                                    let _e2055 = global.member[(_e1999.member_2 + 9u)];
                                    phi_2433_ = type_29(vec3<f32>(bitcast<f32>(_e2008), bitcast<f32>(_e2013), bitcast<f32>(_e2018)), vec4<f32>(bitcast<f32>(_e2024), bitcast<f32>(_e2029), bitcast<f32>(_e2034), bitcast<f32>(_e2039)), vec3<f32>(bitcast<f32>(_e2045), bitcast<f32>(_e2050), bitcast<f32>(_e2055)));
                                } else {
                                    phi_2433_ = type_29(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e2060 = phi_2433_;
                                let _e2068 = (_e2060.member_1.x + _e2060.member_1.x);
                                let _e2069 = (_e2060.member_1.y + _e2060.member_1.y);
                                let _e2070 = (_e2060.member_1.z + _e2060.member_1.z);
                                let _e2072 = (_e2060.member_1.z * _e2070);
                                let _e2073 = (_e2060.member_1.w * _e2068);
                                let _e2074 = (_e2060.member_1.w * _e2069);
                                let _e2075 = (_e2060.member_1.w * _e2070);
                                let _e2095 = (vec4<f32>((1f - fma(_e2060.member_1.y, _e2069, _e2072)), fma(_e2060.member_1.x, _e2069, _e2075), fma(_e2060.member_1.x, _e2070, -(_e2074)), 0f) * _e2060.member_2.x);
                                let _e2097 = (vec4<f32>(fma(_e2060.member_1.x, _e2069, -(_e2075)), (1f - fma(_e2060.member_1.x, _e2068, _e2072)), fma(_e2060.member_1.y, _e2070, _e2073), 0f) * _e2060.member_2.y);
                                let _e2099 = (vec4<f32>(fma(_e2060.member_1.x, _e2070, _e2074), fma(_e2060.member_1.y, _e2070, -(_e2073)), (1f - fma(_e2060.member_1.x, _e2068, (_e2060.member_1.y * _e2069))), 0f) * _e2060.member_2.z);
                                switch bitcast<i32>(_e1999.member) {
                                    case 0: {
                                        if _e302 {
                                            phi_6188_ = (_e1999.member_1 <= (_e117 - 8u));
                                        } else {
                                            phi_6188_ = false;
                                        }
                                        let _e2595 = phi_6188_;
                                        if _e2595 {
                                            let _e2598 = global.member[_e1999.member_1];
                                            let _e2603 = global.member[(_e1999.member_1 + 1u)];
                                            let _e2608 = global.member[(_e1999.member_1 + 2u)];
                                            let _e2614 = global.member[(_e1999.member_1 + 3u)];
                                            let _e2619 = global.member[(_e1999.member_1 + 4u)];
                                            let _e2624 = global.member[(_e1999.member_1 + 5u)];
                                            let _e2629 = global.member[(_e1999.member_1 + 6u)];
                                            let _e2635 = global.member[(_e1999.member_1 + 7u)];
                                            phi_2481_ = type_35(vec3<f32>(bitcast<f32>(_e2598), bitcast<f32>(_e2603), bitcast<f32>(_e2608)), vec4<f32>(bitcast<f32>(_e2614), bitcast<f32>(_e2619), bitcast<f32>(_e2624), bitcast<f32>(_e2629)), bitcast<f32>(_e2635));
                                        } else {
                                            phi_2481_ = type_35(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2639 = phi_2481_;
                                        let _e2661 = fma(_e2099.x, _e2639.member.z, fma(_e2097.x, _e2639.member.y, (_e2095.x * _e2639.member.x)));
                                        let _e2662 = fma(_e2099.y, _e2639.member.z, fma(_e2097.y, _e2639.member.y, (_e2095.y * _e2639.member.x)));
                                        let _e2663 = fma(_e2099.z, _e2639.member.z, fma(_e2097.z, _e2639.member.y, (_e2095.z * _e2639.member.x)));
                                        let _e2668 = sqrt(fma(_e2663, _e2663, fma(_e2661, _e2661, (_e2662 * _e2662))));
                                        if (_e2668 == 0f) {
                                            phi_6235_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6235_ = (vec3<f32>(_e2661, _e2662, _e2663) * (1f / _e2668));
                                        }
                                        let _e2673 = phi_6235_;
                                        let _e2675 = -(_e2673.x);
                                        let _e2677 = -(_e2673.y);
                                        let _e2679 = -(_e2673.z);
                                        let _e2680 = -(_e2673);
                                        let _e2682 = fma(-(_e687.z), _e296.member_3, 1f);
                                        let _e2686 = fma(0.4f, _e2682, (_e1371 * _e1383));
                                        let _e2687 = fma(0.4f, _e2682, (_e1373 * _e1383));
                                        let _e2688 = fma(0.4f, _e2682, (_e1375 * _e1383));
                                        let _e2696 = (_e1939 + vec3<f32>(_e2675, _e2677, _e2679));
                                        let _e2703 = sqrt(fma(_e2696.z, _e2696.z, fma(_e2696.x, _e2696.x, (_e2696.y * _e2696.y))));
                                        if (_e2703 == 0f) {
                                            phi_6270_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6270_ = (_e2696 * (1f / _e2703));
                                        }
                                        let _e2708 = phi_6270_;
                                        let _e2709 = (_e1380 * _e1380);
                                        let _e2720 = max(fma(_e1935.z, _e2708.z, fma(_e1935.x, _e2708.x, (_e1935.y * _e2708.y))), 0f);
                                        let _e2733 = max(fma(_e1935.z, _e1939.z, fma(_e1935.x, _e1939.x, (_e1935.y * _e1939.y))), 0f);
                                        let _e2739 = fma(_e1935.z, _e2680.z, fma(_e1935.x, _e2680.x, (_e1935.y * _e2680.y)));
                                        let _e2740 = max(_e2739, 0f);
                                        let _e2741 = fma(_e687.y, _e296.member_4, 1f);
                                        let _e2742 = (_e2741 * _e2741);
                                        let _e2743 = (_e2742 * 0.125f);
                                        let _e2745 = fma(-(_e2742), 0.125f, 1f);
                                        let _e2758 = (1f - max(fma(_e2708.z, _e1939.z, fma(_e2708.x, _e1939.x, (_e2708.y * _e1939.y))), 0f));
                                        let _e2760 = select(_e2758, 0f, (_e2758 < 0f));
                                        let _e2763 = pow(select(_e2760, 1f, (_e2760 > 1f)), 5f);
                                        let _e2764 = fma((1f - _e2686), _e2763, _e2686);
                                        let _e2765 = fma((1f - _e2687), _e2763, _e2687);
                                        let _e2766 = fma((1f - _e2688), _e2763, _e2688);
                                        let _e2773 = (((_e2709 * _e2709) / (pow(fma((_e2720 * _e2720), fma(_e2709, _e2709, -1f), 1f), 2f) * 3.1415927f)) * ((_e2733 / fma(_e2733, _e2745, _e2743)) * (_e2740 / fma(_e2740, _e2745, _e2743))));
                                        let _e2780 = max(fma(_e1935.z, _e2679, fma(_e1935.x, _e2675, (_e1935.y * _e2677))), 0f);
                                        let _e2782 = fma((4f * _e2733), _e2780, 0.0001f);
                                        let _e2800 = global_1.member[0u];
                                        let _e2803 = global_1.member[1u];
                                        let _e2807 = global_1.member[2u];
                                        let _e2811 = global_1.member[_e2800];
                                        let _e2816 = global_1.member[(_e2800 + 1u)];
                                        let _e2821 = global_1.member[(_e2800 + 2u)];
                                        let _e2826 = global_1.member[(_e2800 + 3u)];
                                        let _e2831 = global_1.member[(_e2800 + 4u)];
                                        let _e2836 = global_1.member[(_e2800 + 5u)];
                                        let _e2841 = global_1.member[(_e2800 + 6u)];
                                        let _e2846 = global_1.member[(_e2800 + 7u)];
                                        let _e2851 = global_1.member[(_e2800 + 8u)];
                                        let _e2856 = global_1.member[(_e2800 + 9u)];
                                        let _e2861 = global_1.member[(_e2800 + 10u)];
                                        let _e2866 = global_1.member[(_e2800 + 11u)];
                                        let _e2871 = global_1.member[(_e2800 + 12u)];
                                        let _e2876 = global_1.member[(_e2800 + 13u)];
                                        let _e2881 = global_1.member[(_e2800 + 14u)];
                                        let _e2886 = global_1.member[(_e2800 + 15u)];
                                        let _e2906 = (bitcast<f32>(_e2886) + fma(bitcast<f32>(_e2866), _e125.z, fma(bitcast<f32>(_e2846), _e125.y, (bitcast<f32>(_e2826) * _e125.x))));
                                        let _e2914 = textureSampleLevel(global_18, global_19, vec2<f32>(fma(((bitcast<f32>(_e2871) + fma(bitcast<f32>(_e2851), _e125.z, fma(bitcast<f32>(_e2831), _e125.y, (bitcast<f32>(_e2811) * _e125.x)))) / _e2906), 0.5f, 0.5f), fma(((bitcast<f32>(_e2876) + fma(bitcast<f32>(_e2856), _e125.z, fma(bitcast<f32>(_e2836), _e125.y, (bitcast<f32>(_e2816) * _e125.x)))) / _e2906), -0.5f, 0.5f)), i32(0f));
                                        let _e2923 = (1f - select(0f, 1f, ((((bitcast<f32>(_e2881) + fma(bitcast<f32>(_e2861), _e125.z, fma(bitcast<f32>(_e2841), _e125.y, (bitcast<f32>(_e2821) * _e125.x)))) / _e2906) - max((bitcast<f32>(_e2807) * (1f - _e2739)), bitcast<f32>(_e2803))) > vec4(_e2914).x)));
                                        phi_3329_ = vec3<f32>(fma(((fma((((1f - _e2764) * _e2682) * _e1371), 0.31830987f, ((_e2773 * _e2764) / _e2782)) * (_e2639.member_1.x * _e2639.member_2)) * _e2780), _e2923, _e1944.x), fma(((fma((((1f - _e2765) * _e2682) * _e1373), 0.31830987f, ((_e2773 * _e2765) / _e2782)) * (_e2639.member_1.y * _e2639.member_2)) * _e2780), _e2923, _e1944.y), fma(((fma((((1f - _e2766) * _e2682) * _e1375), 0.31830987f, ((_e2773 * _e2766) / _e2782)) * (_e2639.member_1.z * _e2639.member_2)) * _e2780), _e2923, _e1944.z));
                                        phi_3330_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if _e302 {
                                            phi_6014_ = (_e1999.member_1 <= (_e117 - 8u));
                                        } else {
                                            phi_6014_ = false;
                                        }
                                        let _e2384 = phi_6014_;
                                        if _e2384 {
                                            let _e2387 = global.member[_e1999.member_1];
                                            let _e2392 = global.member[(_e1999.member_1 + 1u)];
                                            let _e2397 = global.member[(_e1999.member_1 + 2u)];
                                            let _e2403 = global.member[(_e1999.member_1 + 3u)];
                                            let _e2408 = global.member[(_e1999.member_1 + 4u)];
                                            let _e2413 = global.member[(_e1999.member_1 + 5u)];
                                            let _e2418 = global.member[(_e1999.member_1 + 6u)];
                                            let _e2424 = global.member[(_e1999.member_1 + 7u)];
                                            phi_2819_ = type_35(vec3<f32>(bitcast<f32>(_e2387), bitcast<f32>(_e2392), bitcast<f32>(_e2397)), vec4<f32>(bitcast<f32>(_e2403), bitcast<f32>(_e2408), bitcast<f32>(_e2413), bitcast<f32>(_e2418)), bitcast<f32>(_e2424));
                                        } else {
                                            phi_2819_ = type_35(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2428 = phi_2819_;
                                        let _e2457 = (vec3<f32>((_e2060.member.x + fma(_e2099.x, _e2428.member.z, fma(_e2097.x, _e2428.member.y, (_e2095.x * _e2428.member.x)))), (_e2060.member.y + fma(_e2099.y, _e2428.member.z, fma(_e2097.y, _e2428.member.y, (_e2095.y * _e2428.member.x)))), (_e2060.member.z + fma(_e2099.z, _e2428.member.z, fma(_e2097.z, _e2428.member.y, (_e2095.z * _e2428.member.x))))) - _e125);
                                        let _e2464 = sqrt(fma(_e2457.z, _e2457.z, fma(_e2457.x, _e2457.x, (_e2457.y * _e2457.y))));
                                        let _e2465 = (_e2464 == 0f);
                                        if _e2465 {
                                            phi_3009_ = vec3<f32>();
                                        } else {
                                            if _e2465 {
                                                phi_6061_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6061_ = (_e2457 * (1f / _e2464));
                                            }
                                            let _e2469 = phi_6061_;
                                            let _e2471 = (_e2428.member_2 / (_e2464 * _e2464));
                                            let _e2473 = fma(-(_e687.z), _e296.member_3, 1f);
                                            let _e2477 = fma(0.4f, _e2473, (_e1371 * _e1383));
                                            let _e2478 = fma(0.4f, _e2473, (_e1373 * _e1383));
                                            let _e2479 = fma(0.4f, _e2473, (_e1375 * _e1383));
                                            let _e2486 = (_e1939 + _e2469);
                                            let _e2493 = sqrt(fma(_e2486.z, _e2486.z, fma(_e2486.x, _e2486.x, (_e2486.y * _e2486.y))));
                                            if (_e2493 == 0f) {
                                                phi_6096_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6096_ = (_e2486 * (1f / _e2493));
                                            }
                                            let _e2498 = phi_6096_;
                                            let _e2499 = (_e1380 * _e1380);
                                            let _e2510 = max(fma(_e1935.z, _e2498.z, fma(_e1935.x, _e2498.x, (_e1935.y * _e2498.y))), 0f);
                                            let _e2523 = max(fma(_e1935.z, _e1939.z, fma(_e1935.x, _e1939.x, (_e1935.y * _e1939.y))), 0f);
                                            let _e2530 = max(fma(_e1935.z, _e2469.z, fma(_e1935.x, _e2469.x, (_e1935.y * _e2469.y))), 0f);
                                            let _e2531 = fma(_e687.y, _e296.member_4, 1f);
                                            let _e2532 = (_e2531 * _e2531);
                                            let _e2533 = (_e2532 * 0.125f);
                                            let _e2535 = fma(-(_e2532), 0.125f, 1f);
                                            let _e2548 = (1f - max(fma(_e2498.z, _e1939.z, fma(_e2498.x, _e1939.x, (_e2498.y * _e1939.y))), 0f));
                                            let _e2550 = select(_e2548, 0f, (_e2548 < 0f));
                                            let _e2553 = pow(select(_e2550, 1f, (_e2550 > 1f)), 5f);
                                            let _e2554 = fma((1f - _e2477), _e2553, _e2477);
                                            let _e2555 = fma((1f - _e2478), _e2553, _e2478);
                                            let _e2556 = fma((1f - _e2479), _e2553, _e2479);
                                            let _e2563 = (((_e2499 * _e2499) / (pow(fma((_e2510 * _e2510), fma(_e2499, _e2499, -1f), 1f), 2f) * 3.1415927f)) * ((_e2523 / fma(_e2523, _e2535, _e2533)) * (_e2530 / fma(_e2530, _e2535, _e2533))));
                                            let _e2568 = fma((4f * _e2523), _e2530, 0.0001f);
                                            phi_3009_ = vec3<f32>(fma((fma((((1f - _e2554) * _e2473) * _e1371), 0.31830987f, ((_e2563 * _e2554) / _e2568)) * (_e2428.member_1.x * _e2471)), _e2530, _e1944.x), fma((fma((((1f - _e2555) * _e2473) * _e1373), 0.31830987f, ((_e2563 * _e2555) / _e2568)) * (_e2428.member_1.y * _e2471)), _e2530, _e1944.y), fma((fma((((1f - _e2556) * _e2473) * _e1375), 0.31830987f, ((_e2563 * _e2556) / _e2568)) * (_e2428.member_1.z * _e2471)), _e2530, _e1944.z));
                                        }
                                        let _e2589 = phi_3009_;
                                        phi_3329_ = _e2589;
                                        phi_3330_ = select(true, false, _e2465);
                                        break;
                                    }
                                    case 2: {
                                        if (_e117 >= 13u) {
                                            phi_5802_ = (_e1999.member_1 <= (_e117 - 13u));
                                        } else {
                                            phi_5802_ = false;
                                        }
                                        let _e2110 = phi_5802_;
                                        if _e2110 {
                                            let _e2113 = global.member[_e1999.member_1];
                                            let _e2118 = global.member[(_e1999.member_1 + 1u)];
                                            let _e2123 = global.member[(_e1999.member_1 + 2u)];
                                            let _e2129 = global.member[(_e1999.member_1 + 3u)];
                                            let _e2134 = global.member[(_e1999.member_1 + 4u)];
                                            let _e2139 = global.member[(_e1999.member_1 + 5u)];
                                            let _e2145 = global.member[(_e1999.member_1 + 6u)];
                                            let _e2150 = global.member[(_e1999.member_1 + 7u)];
                                            let _e2155 = global.member[(_e1999.member_1 + 8u)];
                                            let _e2160 = global.member[(_e1999.member_1 + 9u)];
                                            let _e2165 = global.member[(_e1999.member_1 + 10u)];
                                            let _e2170 = global.member[(_e1999.member_1 + 11u)];
                                            let _e2176 = global.member[(_e1999.member_1 + 12u)];
                                            phi_3072_ = type_36(vec3<f32>(bitcast<f32>(_e2113), bitcast<f32>(_e2118), bitcast<f32>(_e2123)), vec3<f32>(bitcast<f32>(_e2129), bitcast<f32>(_e2134), bitcast<f32>(_e2139)), bitcast<f32>(_e2145), bitcast<f32>(_e2150), vec4<f32>(bitcast<f32>(_e2155), bitcast<f32>(_e2160), bitcast<f32>(_e2165), bitcast<f32>(_e2170)), bitcast<f32>(_e2176));
                                        } else {
                                            phi_3072_ = type_36(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2180 = phi_3072_;
                                        let _e2212 = (vec3<f32>((_e2060.member.x + fma(_e2099.x, _e2180.member.z, fma(_e2097.x, _e2180.member.y, (_e2095.x * _e2180.member.x)))), (_e2060.member.y + fma(_e2099.y, _e2180.member.z, fma(_e2097.y, _e2180.member.y, (_e2095.y * _e2180.member.x)))), (_e2060.member.z + fma(_e2099.z, _e2180.member.z, fma(_e2097.z, _e2180.member.y, (_e2095.z * _e2180.member.x))))) - _e125);
                                        let _e2219 = sqrt(fma(_e2212.z, _e2212.z, fma(_e2212.x, _e2212.x, (_e2212.y * _e2212.y))));
                                        let _e2220 = (_e2219 == 0f);
                                        if _e2220 {
                                            phi_3327_ = vec3<f32>();
                                        } else {
                                            if _e2220 {
                                                phi_5852_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5852_ = (_e2212 * (1f / _e2219));
                                            }
                                            let _e2224 = phi_5852_;
                                            let _e2234 = fma(_e2099.x, _e2180.member_1.z, fma(_e2097.x, _e2180.member_1.y, (_e2095.x * _e2180.member_1.x)));
                                            let _e2235 = fma(_e2099.y, _e2180.member_1.z, fma(_e2097.y, _e2180.member_1.y, (_e2095.y * _e2180.member_1.x)));
                                            let _e2236 = fma(_e2099.z, _e2180.member_1.z, fma(_e2097.z, _e2180.member_1.y, (_e2095.z * _e2180.member_1.x)));
                                            let _e2241 = sqrt(fma(_e2236, _e2236, fma(_e2234, _e2234, (_e2235 * _e2235))));
                                            if (_e2241 == 0f) {
                                                phi_5887_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5887_ = (vec3<f32>(_e2234, _e2235, _e2236) * (1f / _e2241));
                                            }
                                            let _e2246 = phi_5887_;
                                            let _e2258 = ((fma(_e2224.z, _e2246.z, fma(_e2224.x, _e2246.x, (_e2224.y * _e2246.y))) - _e2180.member_3) / (_e2180.member_2 - _e2180.member_3));
                                            let _e2260 = select(_e2258, 0f, (_e2258 < 0f));
                                            let _e2263 = (_e2180.member_5 * select(_e2260, 1f, (_e2260 > 1f)));
                                            let _e2265 = fma(-(_e687.z), _e296.member_3, 1f);
                                            let _e2269 = fma(0.4f, _e2265, (_e1371 * _e1383));
                                            let _e2270 = fma(0.4f, _e2265, (_e1373 * _e1383));
                                            let _e2271 = fma(0.4f, _e2265, (_e1375 * _e1383));
                                            let _e2278 = (_e1939 + _e2224);
                                            let _e2285 = sqrt(fma(_e2278.z, _e2278.z, fma(_e2278.x, _e2278.x, (_e2278.y * _e2278.y))));
                                            if (_e2285 == 0f) {
                                                phi_5922_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5922_ = (_e2278 * (1f / _e2285));
                                            }
                                            let _e2290 = phi_5922_;
                                            let _e2291 = (_e1380 * _e1380);
                                            let _e2302 = max(fma(_e1935.z, _e2290.z, fma(_e1935.x, _e2290.x, (_e1935.y * _e2290.y))), 0f);
                                            let _e2315 = max(fma(_e1935.z, _e1939.z, fma(_e1935.x, _e1939.x, (_e1935.y * _e1939.y))), 0f);
                                            let _e2319 = max(fma(_e1935.z, _e2224.z, fma(_e1935.x, _e2224.x, (_e1935.y * _e2224.y))), 0f);
                                            let _e2320 = fma(_e687.y, _e296.member_4, 1f);
                                            let _e2321 = (_e2320 * _e2320);
                                            let _e2322 = (_e2321 * 0.125f);
                                            let _e2324 = fma(-(_e2321), 0.125f, 1f);
                                            let _e2337 = (1f - max(fma(_e2290.z, _e1939.z, fma(_e2290.x, _e1939.x, (_e2290.y * _e1939.y))), 0f));
                                            let _e2339 = select(_e2337, 0f, (_e2337 < 0f));
                                            let _e2342 = pow(select(_e2339, 1f, (_e2339 > 1f)), 5f);
                                            let _e2343 = fma((1f - _e2269), _e2342, _e2269);
                                            let _e2344 = fma((1f - _e2270), _e2342, _e2270);
                                            let _e2345 = fma((1f - _e2271), _e2342, _e2271);
                                            let _e2352 = (((_e2291 * _e2291) / (pow(fma((_e2302 * _e2302), fma(_e2291, _e2291, -1f), 1f), 2f) * 3.1415927f)) * ((_e2315 / fma(_e2315, _e2324, _e2322)) * (_e2319 / fma(_e2319, _e2324, _e2322))));
                                            let _e2357 = fma((4f * _e2315), _e2319, 0.0001f);
                                            phi_3327_ = vec3<f32>(fma((fma((((1f - _e2343) * _e2265) * _e1371), 0.31830987f, ((_e2352 * _e2343) / _e2357)) * (_e2180.member_4.x * _e2263)), _e2319, _e1944.x), fma((fma((((1f - _e2344) * _e2265) * _e1373), 0.31830987f, ((_e2352 * _e2344) / _e2357)) * (_e2180.member_4.y * _e2263)), _e2319, _e1944.y), fma((fma((((1f - _e2345) * _e2265) * _e1375), 0.31830987f, ((_e2352 * _e2345) / _e2357)) * (_e2180.member_4.z * _e2263)), _e2319, _e1944.z));
                                        }
                                        let _e2378 = phi_3327_;
                                        phi_3329_ = _e2378;
                                        phi_3330_ = select(true, false, _e2220);
                                        break;
                                    }
                                    default: {
                                        phi_3329_ = vec3<f32>();
                                        phi_3330_ = bool();
                                        break;
                                    }
                                }
                                let _e2932 = phi_3329_;
                                let _e2934 = phi_3330_;
                                phi_3339_ = select(_e2932, _e1944, vec3(select(true, false, _e2934)));
                            }
                            let _e2939 = phi_3339_;
                            phi_2317_ = _e2939;
                            phi_3341_ = select(true, false, _e1977);
                            break;
                        }
                        default: {
                            phi_2317_ = vec3<f32>();
                            phi_3341_ = bool();
                            break;
                        }
                    }
                    let _e2942 = phi_2317_;
                    let _e2944 = phi_3341_;
                    continue;
                    continuing {
                        phi_2313_ = _e1957;
                        phi_2316_ = _e2942;
                        break if !(_e2944);
                    }
                }
                let _e2947 = fma(-(_e687.z), _e296.member_3, 1f);
                let _e2951 = fma(0.04f, _e2947, (_e1371 * _e1383));
                let _e2952 = fma(0.04f, _e2947, (_e1373 * _e1383));
                let _e2953 = fma(0.04f, _e2947, (_e1375 * _e1383));
                let _e2965 = fma(-(_e687.y), _e296.member_4, 1f);
                let _e2972 = (1f - max(fma(_e1935.z, _e1939.z, fma(_e1935.x, _e1939.x, (_e1935.y * _e1939.y))), 0f));
                let _e2974 = select(_e2972, 0f, (_e2972 < 0f));
                let _e2977 = pow(select(_e2974, 1f, (_e2974 > 1f)), 5f);
                let _e2978 = fma((max(_e2965, _e2951) - _e2951), _e2977, _e2951);
                let _e2979 = fma((max(_e2965, _e2952) - _e2952), _e2977, _e2952);
                let _e2980 = fma((max(_e2965, _e2953) - _e2953), _e2977, _e2953);
                let _e3000 = local_2;
                let _e3004 = local_3;
                let _e3008 = local_4;
                phi_3458_ = vec4<f32>(fma(_e1393, _e296.member_1, fma(fma(((1f - _e2978) * _e2947), (_e1402.x * _e1371), (_e1750.x * fma(_e2978, _e1766.x, _e1766.y))), _e1387, _e3000.x)), fma(_e1395, _e296.member_1, fma(fma(((1f - _e2979) * _e2947), (_e1402.y * _e1373), (_e1750.y * fma(_e2979, _e1766.x, _e1766.y))), _e1387, _e3004.y)), fma(_e1397, _e296.member_1, fma(fma(((1f - _e2980) * _e2947), (_e1402.z * _e1375), (_e1750.z * fma(_e2980, _e1766.x, _e1766.y))), _e1387, _e3008.z)), 1f);
            } else {
                phi_3458_ = (vec4<f32>((_e119.x * _e493.x), (_e119.y * _e493.y), (_e119.z * _e493.z), (_e119.w * _e493.w)) * _e296.member_2);
            }
            let _e3016 = phi_3458_;
            global_20 = _e3016;
            break;
        }
        case 1: {
            let _e1908 = sqrt(fma(_e120.x, _e120.x, (_e120.y * _e120.y)));
            if (_e1908 == 0f) {
                phi_5550_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5550_ = (vec3<f32>(_e120.x, _e120.y, 0f) * (1f / _e1908));
            }
            let _e1913 = phi_5550_;
            global_20 = vec4<f32>(((_e1913.x + 1f) * 0.5f), ((_e1913.y + 1f) * 0.5f), ((_e1913.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1887 = sqrt(fma(_e121.x, _e121.x, (_e121.y * _e121.y)));
            if (_e1887 == 0f) {
                phi_5501_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5501_ = (vec3<f32>(_e121.x, _e121.y, 0f) * (1f / _e1887));
            }
            let _e1892 = phi_5501_;
            global_20 = vec4<f32>(((_e1892.x + 1f) * 0.5f), ((_e1892.y + 1f) * 0.5f), ((_e1892.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1729 {
                phi_5452_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5452_ = (_e1356 * (1f / _e1728));
            }
            let _e1871 = phi_5452_;
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
                phi_5403_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5403_ = (_e122 * (1f / _e1852));
            }
            let _e1857 = phi_5403_;
            global_20 = vec4<f32>(((_e1857.x + 1f) * 0.5f), ((_e1857.y + 1f) * 0.5f), ((_e1857.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1830 = sqrt(fma(_e1354.z, _e1354.z, fma(_e1354.x, _e1354.x, (_e1354.y * _e1354.y))));
            if (_e1830 == 0f) {
                phi_5354_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5354_ = (_e1354 * (1f / _e1830));
            }
            let _e1835 = phi_5354_;
            global_20 = vec4<f32>(((_e1835.x + 1f) * 0.5f), ((_e1835.y + 1f) * 0.5f), ((_e1835.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1808 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
            if (_e1808 == 0f) {
                phi_5305_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5305_ = (_e123 * (1f / _e1808));
            }
            let _e1813 = phi_5305_;
            global_20 = vec4<f32>(((_e1813.x + 1f) * 0.5f), ((_e1813.y + 1f) * 0.5f), ((_e1813.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1786 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
            if (_e1786 == 0f) {
                phi_5256_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5256_ = (_e124 * (1f / _e1786));
            }
            let _e1791 = phi_5256_;
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
