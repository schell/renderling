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
}

struct type_22 {
    member: type_20,
    member_1: type_20,
    member_2: type_21,
    member_3: vec3<f32>,
}

struct type_24 {
    member: u32,
    member_1: u32,
}

struct type_28 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_30 {
    member: vec2<u32>,
    member_1: vec2<u32>,
    member_2: type_24,
    member_3: u32,
    member_4: bool,
    member_5: bool,
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
var<private> global_1: u32;
var<private> global_2: u32;
var<private> global_3: u32;
var<private> global_4: vec4<f32>;
var<private> global_5: vec2<f32>;
var<private> global_6: vec2<f32>;
var<private> global_7: vec3<f32>;
var<private> global_8: vec3<f32>;
var<private> global_9: vec3<f32>;
var<private> global_10: vec3<f32>;
@group(1) @binding(1) 
var global_11: sampler;
@group(1) @binding(0) 
var global_12: texture_2d_array<f32>;
@group(1) @binding(2) 
var global_13: texture_cube<f32>;
@group(1) @binding(3) 
var global_14: sampler;
@group(1) @binding(4) 
var global_15: texture_cube<f32>;
@group(1) @binding(5) 
var global_16: sampler;
@group(1) @binding(6) 
var global_17: texture_2d<f32>;
@group(1) @binding(7) 
var global_18: sampler;
var<private> global_19: vec4<f32>;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var phi_3692_: bool;
    var phi_500_: u32;
    var phi_526_: type_30;
    var phi_3743_: bool;
    var phi_645_: type_32;
    var phi_649_: type_32;
    var phi_3776_: bool;
    var phi_689_: u32;
    var phi_698_: u32;
    var phi_711_: type_33;
    var phi_3832_: f32;
    var phi_3814_: bool;
    var phi_765_: f32;
    var phi_760_: f32;
    var phi_766_: f32;
    var phi_3797_: bool;
    var phi_731_: f32;
    var phi_768_: f32;
    var phi_3880_: f32;
    var phi_3862_: bool;
    var phi_823_: f32;
    var phi_818_: f32;
    var phi_824_: f32;
    var phi_3845_: bool;
    var phi_789_: f32;
    var phi_826_: f32;
    var phi_3912_: bool;
    var phi_909_: u32;
    var phi_918_: u32;
    var phi_931_: type_33;
    var phi_3967_: f32;
    var phi_3949_: bool;
    var phi_985_: f32;
    var phi_980_: f32;
    var phi_986_: f32;
    var phi_3932_: bool;
    var phi_951_: f32;
    var phi_988_: f32;
    var phi_4015_: f32;
    var phi_3997_: bool;
    var phi_1043_: f32;
    var phi_1038_: f32;
    var phi_1044_: f32;
    var phi_3980_: bool;
    var phi_1009_: f32;
    var phi_1046_: f32;
    var phi_4047_: bool;
    var phi_1129_: u32;
    var phi_1138_: u32;
    var phi_1151_: type_33;
    var phi_4102_: f32;
    var phi_4084_: bool;
    var phi_1205_: f32;
    var phi_1200_: f32;
    var phi_1206_: f32;
    var phi_4067_: bool;
    var phi_1171_: f32;
    var phi_1208_: f32;
    var phi_4150_: f32;
    var phi_4132_: bool;
    var phi_1263_: f32;
    var phi_1258_: f32;
    var phi_1264_: f32;
    var phi_4115_: bool;
    var phi_1229_: f32;
    var phi_1266_: f32;
    var phi_4182_: bool;
    var phi_1349_: u32;
    var phi_1358_: u32;
    var phi_1371_: type_33;
    var phi_4237_: f32;
    var phi_4219_: bool;
    var phi_1425_: f32;
    var phi_1420_: f32;
    var phi_1426_: f32;
    var phi_4202_: bool;
    var phi_1391_: f32;
    var phi_1428_: f32;
    var phi_4285_: f32;
    var phi_4267_: bool;
    var phi_1483_: f32;
    var phi_1478_: f32;
    var phi_1484_: f32;
    var phi_4250_: bool;
    var phi_1449_: f32;
    var phi_1486_: f32;
    var phi_4317_: bool;
    var phi_1569_: u32;
    var phi_1578_: u32;
    var phi_1591_: type_33;
    var phi_4372_: f32;
    var phi_4354_: bool;
    var phi_1645_: f32;
    var phi_1640_: f32;
    var phi_1646_: f32;
    var phi_4337_: bool;
    var phi_1611_: f32;
    var phi_1648_: f32;
    var phi_4420_: f32;
    var phi_4402_: bool;
    var phi_1703_: f32;
    var phi_1698_: f32;
    var phi_1704_: f32;
    var phi_4385_: bool;
    var phi_1669_: f32;
    var phi_1706_: f32;
    var phi_4474_: vec3<f32>;
    var phi_4509_: vec3<f32>;
    var phi_4544_: vec3<f32>;
    var phi_4579_: vec3<f32>;
    var phi_4614_: vec3<f32>;
    var phi_1800_: vec3<f32>;
    var phi_1801_: vec3<f32>;
    var phi_4646_: bool;
    var phi_2007_: type_24;
    var phi_2008_: type_24;
    var phi_2023_: type_24;
    var phi_2050_: bool;
    var phi_2056_: type_24;
    var phi_2057_: type_24;
    var phi_2072_: type_24;
    var phi_2095_: bool;
    var phi_2103_: type_22;
    var phi_4718_: vec3<f32>;
    var phi_4777_: vec3<f32>;
    var phi_4851_: vec3<f32>;
    var phi_5996_: vec3<f32>;
    var phi_5947_: vec3<f32>;
    var phi_5898_: vec3<f32>;
    var phi_5849_: vec3<f32>;
    var phi_5800_: vec3<f32>;
    var phi_5751_: vec3<f32>;
    var phi_5702_: vec3<f32>;
    var phi_4901_: vec3<f32>;
    var phi_4936_: vec3<f32>;
    var phi_2143_: type_24;
    var phi_2146_: vec3<f32>;
    var phi_2144_: type_24;
    var phi_2161_: type_24;
    var phi_4953_: u32;
    var phi_4972_: bool;
    var phi_2178_: u32;
    var phi_5004_: bool;
    var phi_2195_: u32;
    var phi_2205_: type_34;
    var phi_5034_: bool;
    var phi_2255_: type_28;
    var phi_5463_: bool;
    var phi_2753_: type_36;
    var phi_5513_: vec3<f32>;
    var phi_5548_: vec3<f32>;
    var phi_5583_: vec3<f32>;
    var phi_2999_: vec3<f32>;
    var phi_5290_: bool;
    var phi_2505_: type_35;
    var phi_5336_: vec3<f32>;
    var phi_5371_: vec3<f32>;
    var phi_2690_: vec3<f32>;
    var phi_5115_: bool;
    var phi_2303_: type_35;
    var phi_5163_: vec3<f32>;
    var phi_5198_: vec3<f32>;
    var phi_3001_: vec3<f32>;
    var phi_3002_: bool;
    var phi_3011_: vec3<f32>;
    var phi_2147_: vec3<f32>;
    var phi_3013_: bool;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var phi_3125_: vec4<f32>;

    let _e117 = arrayLength((&global.member));
    let _e118 = global_1;
    let _e119 = global_2;
    let _e120 = global_3;
    let _e121 = global_4;
    let _e122 = global_5;
    let _e123 = global_6;
    let _e124 = global_7;
    let _e125 = global_8;
    let _e126 = global_9;
    let _e127 = global_10;
    if (_e117 >= 9u) {
        phi_3692_ = (_e120 <= (_e117 - 9u));
    } else {
        phi_3692_ = false;
    }
    let _e132 = phi_3692_;
    if _e132 {
        let _e135 = global.member[_e120];
        let _e139 = global.member[(_e120 + 1u)];
        let _e144 = global.member[(_e120 + 2u)];
        let _e148 = global.member[(_e120 + 3u)];
        let _e153 = global.member[(_e120 + 4u)];
        switch bitcast<i32>(_e153) {
            case 0: {
                phi_500_ = 0u;
                break;
            }
            case 1: {
                phi_500_ = 1u;
                break;
            }
            case 2: {
                phi_500_ = 2u;
                break;
            }
            case 3: {
                phi_500_ = 3u;
                break;
            }
            case 4: {
                phi_500_ = 4u;
                break;
            }
            case 5: {
                phi_500_ = 5u;
                break;
            }
            case 6: {
                phi_500_ = 6u;
                break;
            }
            case 7: {
                phi_500_ = 7u;
                break;
            }
            case 8: {
                phi_500_ = 8u;
                break;
            }
            case 9: {
                phi_500_ = 9u;
                break;
            }
            case 10: {
                phi_500_ = 10u;
                break;
            }
            case 11: {
                phi_500_ = 11u;
                break;
            }
            case 12: {
                phi_500_ = 12u;
                break;
            }
            case 13: {
                phi_500_ = 13u;
                break;
            }
            case 14: {
                phi_500_ = 14u;
                break;
            }
            case 15: {
                phi_500_ = 15u;
                break;
            }
            case 16: {
                phi_500_ = 16u;
                break;
            }
            case 17: {
                phi_500_ = 17u;
                break;
            }
            case 18: {
                phi_500_ = 18u;
                break;
            }
            case 19: {
                phi_500_ = 19u;
                break;
            }
            default: {
                phi_500_ = 0u;
                break;
            }
        }
        let _e156 = phi_500_;
        let _e160 = global.member[(_e120 + 5u)];
        let _e165 = global.member[(_e120 + 6u)];
        let _e170 = global.member[(_e120 + 7u)];
        let _e174 = global.member[(_e120 + 8u)];
        phi_526_ = type_30(vec2<u32>(_e135, _e139), vec2<u32>(_e144, _e148), type_24(_e170, _e174), _e156, (_e160 == 1u), (_e165 == 1u));
    } else {
        phi_526_ = type_30(vec2<u32>(0u, 0u), vec2<u32>(1u, 1u), type_24(4294967295u, 0u), 0u, true, true);
    }
    let _e178 = phi_526_;
    if (_e119 == 4294967295u) {
        phi_649_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e117 >= 22u) {
            phi_3743_ = (_e119 <= (_e117 - 22u));
        } else {
            phi_3743_ = false;
        }
        let _e191 = phi_3743_;
        if _e191 {
            let _e194 = global.member[_e119];
            let _e199 = global.member[(_e119 + 1u)];
            let _e204 = global.member[(_e119 + 2u)];
            let _e210 = global.member[(_e119 + 3u)];
            let _e215 = global.member[(_e119 + 4u)];
            let _e220 = global.member[(_e119 + 5u)];
            let _e225 = global.member[(_e119 + 6u)];
            let _e230 = global.member[(_e119 + 7u)];
            let _e236 = global.member[(_e119 + 8u)];
            let _e241 = global.member[(_e119 + 9u)];
            let _e246 = global.member[(_e119 + 10u)];
            let _e250 = global.member[(_e119 + 11u)];
            let _e254 = global.member[(_e119 + 12u)];
            let _e258 = global.member[(_e119 + 13u)];
            let _e262 = global.member[(_e119 + 14u)];
            let _e266 = global.member[(_e119 + 15u)];
            let _e270 = global.member[(_e119 + 16u)];
            let _e274 = global.member[(_e119 + 17u)];
            let _e278 = global.member[(_e119 + 18u)];
            let _e282 = global.member[(_e119 + 19u)];
            let _e286 = global.member[(_e119 + 20u)];
            let _e291 = global.member[(_e119 + 21u)];
            phi_645_ = type_32(vec3<f32>(bitcast<f32>(_e194), bitcast<f32>(_e199), bitcast<f32>(_e204)), bitcast<f32>(_e210), vec4<f32>(bitcast<f32>(_e215), bitcast<f32>(_e220), bitcast<f32>(_e225), bitcast<f32>(_e230)), bitcast<f32>(_e236), bitcast<f32>(_e241), _e246, _e250, _e254, _e258, _e262, _e266, _e270, _e274, _e278, _e282, (_e286 == 1u), bitcast<f32>(_e291));
        } else {
            phi_645_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e295 = phi_645_;
        phi_649_ = type_32(_e295.member, _e295.member_1, _e295.member_2, _e295.member_3, _e295.member_4, _e295.member_5, _e295.member_6, _e295.member_7, _e295.member_8, _e295.member_9, _e295.member_10, _e295.member_11, _e295.member_12, _e295.member_13, _e295.member_14, (_e295.member_15 && _e178.member_4), _e295.member_16);
    }
    let _e317 = phi_649_;
    let _e321 = select(_e123, _e122, vec2((_e317.member_10 == 0u)));
    let _e323 = (_e117 >= 8u);
    if _e323 {
        phi_3776_ = (_e317.member_5 <= (_e117 - 8u));
    } else {
        phi_3776_ = false;
    }
    let _e327 = phi_3776_;
    if _e327 {
        let _e330 = global.member[_e317.member_5];
        let _e334 = global.member[(_e317.member_5 + 1u)];
        let _e339 = global.member[(_e317.member_5 + 2u)];
        let _e343 = global.member[(_e317.member_5 + 3u)];
        let _e348 = global.member[(_e317.member_5 + 4u)];
        let _e352 = global.member[(_e317.member_5 + 5u)];
        let _e356 = global.member[(_e317.member_5 + 6u)];
        switch bitcast<i32>(_e356) {
            case 0: {
                phi_689_ = 0u;
                break;
            }
            case 1: {
                phi_689_ = 1u;
                break;
            }
            case 2: {
                phi_689_ = 2u;
                break;
            }
            default: {
                phi_689_ = 0u;
                break;
            }
        }
        let _e359 = phi_689_;
        let _e363 = global.member[(_e317.member_5 + 7u)];
        switch bitcast<i32>(_e363) {
            case 0: {
                phi_698_ = 0u;
                break;
            }
            case 1: {
                phi_698_ = 1u;
                break;
            }
            case 2: {
                phi_698_ = 2u;
                break;
            }
            default: {
                phi_698_ = 0u;
                break;
            }
        }
        let _e366 = phi_698_;
        phi_711_ = type_33(type_24(_e359, _e366), vec2<u32>(_e330, _e334), vec2<u32>(_e339, _e343), _e348, _e352);
    } else {
        phi_711_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e370 = phi_711_;
    switch bitcast<i32>(_e370.member.member) {
        case 1: {
            let _e408 = abs(_e321.x);
            let _e410 = (_e408 % 1f);
            if (_e408 >= 1f) {
                phi_3797_ = select(true, false, (_e410 == 0f));
            } else {
                phi_3797_ = true;
            }
            let _e414 = phi_3797_;
            let _e415 = select(1f, _e410, _e414);
            if (select(-1f, 1f, (_e321.x >= 0f)) > 0f) {
                phi_731_ = _e415;
            } else {
                phi_731_ = (1f - _e415);
            }
            let _e419 = phi_731_;
            phi_768_ = _e419;
            break;
        }
        case 2: {
            let _e382 = abs(_e321.x);
            let _e389 = ((select(select(u32(_e382), 0u, (_e382 < 0f)), 4294967295u, (_e382 > 4294967000f)) % 2u) == 0u);
            let _e391 = (_e382 % 1f);
            if (_e382 >= 1f) {
                phi_3814_ = select(true, false, (_e391 == 0f));
            } else {
                phi_3814_ = true;
            }
            let _e395 = phi_3814_;
            let _e396 = select(1f, _e391, _e395);
            if (select(-1f, 1f, (_e321.x >= 0f)) > 0f) {
                if _e389 {
                    phi_760_ = _e396;
                } else {
                    phi_760_ = (1f - _e396);
                }
                let _e403 = phi_760_;
                phi_766_ = _e403;
            } else {
                if _e389 {
                    phi_765_ = (1f - _e396);
                } else {
                    phi_765_ = _e396;
                }
                let _e400 = phi_765_;
                phi_766_ = _e400;
            }
            let _e405 = phi_766_;
            phi_768_ = _e405;
            break;
        }
        case 0: {
            if (_e321.x > 1f) {
                phi_3832_ = 0.9999999f;
            } else {
                phi_3832_ = select(_e321.x, 0.00000011920929f, (_e321.x < 0f));
            }
            let _e379 = phi_3832_;
            phi_768_ = _e379;
            break;
        }
        default: {
            phi_768_ = f32();
            break;
        }
    }
    let _e421 = phi_768_;
    switch bitcast<i32>(_e370.member.member_1) {
        case 1: {
            let _e459 = abs(_e321.y);
            let _e461 = (_e459 % 1f);
            if (_e459 >= 1f) {
                phi_3845_ = select(true, false, (_e461 == 0f));
            } else {
                phi_3845_ = true;
            }
            let _e465 = phi_3845_;
            let _e466 = select(1f, _e461, _e465);
            if (select(-1f, 1f, (_e321.y >= 0f)) > 0f) {
                phi_789_ = _e466;
            } else {
                phi_789_ = (1f - _e466);
            }
            let _e470 = phi_789_;
            phi_826_ = _e470;
            break;
        }
        case 2: {
            let _e433 = abs(_e321.y);
            let _e440 = ((select(select(u32(_e433), 0u, (_e433 < 0f)), 4294967295u, (_e433 > 4294967000f)) % 2u) == 0u);
            let _e442 = (_e433 % 1f);
            if (_e433 >= 1f) {
                phi_3862_ = select(true, false, (_e442 == 0f));
            } else {
                phi_3862_ = true;
            }
            let _e446 = phi_3862_;
            let _e447 = select(1f, _e442, _e446);
            if (select(-1f, 1f, (_e321.y >= 0f)) > 0f) {
                if _e440 {
                    phi_818_ = _e447;
                } else {
                    phi_818_ = (1f - _e447);
                }
                let _e454 = phi_818_;
                phi_824_ = _e454;
            } else {
                if _e440 {
                    phi_823_ = (1f - _e447);
                } else {
                    phi_823_ = _e447;
                }
                let _e451 = phi_823_;
                phi_824_ = _e451;
            }
            let _e456 = phi_824_;
            phi_826_ = _e456;
            break;
        }
        case 0: {
            if (_e321.y > 1f) {
                phi_3880_ = 0.9999999f;
            } else {
                phi_3880_ = select(_e321.y, 0.00000011920929f, (_e321.y < 0f));
            }
            let _e430 = phi_3880_;
            phi_826_ = _e430;
            break;
        }
        default: {
            phi_826_ = f32();
            break;
        }
    }
    let _e472 = phi_826_;
    let _e476 = (_e421 * f32(_e370.member_2.x));
    let _e485 = (_e472 * f32(_e370.member_2.y));
    let _e498 = f32(_e178.member.x);
    let _e500 = f32(_e178.member.y);
    let _e507 = vec3<f32>((f32((select(select(u32(_e476), 0u, (_e476 < 0f)), 4294967295u, (_e476 > 4294967000f)) + _e370.member_1.x)) / _e498), (f32((select(select(u32(_e485), 0u, (_e485 < 0f)), 4294967295u, (_e485 > 4294967000f)) + _e370.member_1.y)) / _e500), f32(_e370.member_3));
    let _e513 = textureSampleLevel(global_12, global_11, vec2<f32>(_e507.x, _e507.y), i32(_e507.z), 0f);
    let _e516 = select(_e513, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e317.member_5 == 4294967295u)));
    let _e520 = select(_e123, _e122, vec2((_e317.member_11 == 0u)));
    if _e323 {
        phi_3912_ = (_e317.member_6 <= (_e117 - 8u));
    } else {
        phi_3912_ = false;
    }
    let _e525 = phi_3912_;
    if _e525 {
        let _e528 = global.member[_e317.member_6];
        let _e532 = global.member[(_e317.member_6 + 1u)];
        let _e537 = global.member[(_e317.member_6 + 2u)];
        let _e541 = global.member[(_e317.member_6 + 3u)];
        let _e546 = global.member[(_e317.member_6 + 4u)];
        let _e550 = global.member[(_e317.member_6 + 5u)];
        let _e554 = global.member[(_e317.member_6 + 6u)];
        switch bitcast<i32>(_e554) {
            case 0: {
                phi_909_ = 0u;
                break;
            }
            case 1: {
                phi_909_ = 1u;
                break;
            }
            case 2: {
                phi_909_ = 2u;
                break;
            }
            default: {
                phi_909_ = 0u;
                break;
            }
        }
        let _e557 = phi_909_;
        let _e561 = global.member[(_e317.member_6 + 7u)];
        switch bitcast<i32>(_e561) {
            case 0: {
                phi_918_ = 0u;
                break;
            }
            case 1: {
                phi_918_ = 1u;
                break;
            }
            case 2: {
                phi_918_ = 2u;
                break;
            }
            default: {
                phi_918_ = 0u;
                break;
            }
        }
        let _e564 = phi_918_;
        phi_931_ = type_33(type_24(_e557, _e564), vec2<u32>(_e528, _e532), vec2<u32>(_e537, _e541), _e546, _e550);
    } else {
        phi_931_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e568 = phi_931_;
    switch bitcast<i32>(_e568.member.member) {
        case 1: {
            let _e606 = abs(_e520.x);
            let _e608 = (_e606 % 1f);
            if (_e606 >= 1f) {
                phi_3932_ = select(true, false, (_e608 == 0f));
            } else {
                phi_3932_ = true;
            }
            let _e612 = phi_3932_;
            let _e613 = select(1f, _e608, _e612);
            if (select(-1f, 1f, (_e520.x >= 0f)) > 0f) {
                phi_951_ = _e613;
            } else {
                phi_951_ = (1f - _e613);
            }
            let _e617 = phi_951_;
            phi_988_ = _e617;
            break;
        }
        case 2: {
            let _e580 = abs(_e520.x);
            let _e587 = ((select(select(u32(_e580), 0u, (_e580 < 0f)), 4294967295u, (_e580 > 4294967000f)) % 2u) == 0u);
            let _e589 = (_e580 % 1f);
            if (_e580 >= 1f) {
                phi_3949_ = select(true, false, (_e589 == 0f));
            } else {
                phi_3949_ = true;
            }
            let _e593 = phi_3949_;
            let _e594 = select(1f, _e589, _e593);
            if (select(-1f, 1f, (_e520.x >= 0f)) > 0f) {
                if _e587 {
                    phi_980_ = _e594;
                } else {
                    phi_980_ = (1f - _e594);
                }
                let _e601 = phi_980_;
                phi_986_ = _e601;
            } else {
                if _e587 {
                    phi_985_ = (1f - _e594);
                } else {
                    phi_985_ = _e594;
                }
                let _e598 = phi_985_;
                phi_986_ = _e598;
            }
            let _e603 = phi_986_;
            phi_988_ = _e603;
            break;
        }
        case 0: {
            if (_e520.x > 1f) {
                phi_3967_ = 0.9999999f;
            } else {
                phi_3967_ = select(_e520.x, 0.00000011920929f, (_e520.x < 0f));
            }
            let _e577 = phi_3967_;
            phi_988_ = _e577;
            break;
        }
        default: {
            phi_988_ = f32();
            break;
        }
    }
    let _e619 = phi_988_;
    switch bitcast<i32>(_e568.member.member_1) {
        case 1: {
            let _e657 = abs(_e520.y);
            let _e659 = (_e657 % 1f);
            if (_e657 >= 1f) {
                phi_3980_ = select(true, false, (_e659 == 0f));
            } else {
                phi_3980_ = true;
            }
            let _e663 = phi_3980_;
            let _e664 = select(1f, _e659, _e663);
            if (select(-1f, 1f, (_e520.y >= 0f)) > 0f) {
                phi_1009_ = _e664;
            } else {
                phi_1009_ = (1f - _e664);
            }
            let _e668 = phi_1009_;
            phi_1046_ = _e668;
            break;
        }
        case 2: {
            let _e631 = abs(_e520.y);
            let _e638 = ((select(select(u32(_e631), 0u, (_e631 < 0f)), 4294967295u, (_e631 > 4294967000f)) % 2u) == 0u);
            let _e640 = (_e631 % 1f);
            if (_e631 >= 1f) {
                phi_3997_ = select(true, false, (_e640 == 0f));
            } else {
                phi_3997_ = true;
            }
            let _e644 = phi_3997_;
            let _e645 = select(1f, _e640, _e644);
            if (select(-1f, 1f, (_e520.y >= 0f)) > 0f) {
                if _e638 {
                    phi_1038_ = _e645;
                } else {
                    phi_1038_ = (1f - _e645);
                }
                let _e652 = phi_1038_;
                phi_1044_ = _e652;
            } else {
                if _e638 {
                    phi_1043_ = (1f - _e645);
                } else {
                    phi_1043_ = _e645;
                }
                let _e649 = phi_1043_;
                phi_1044_ = _e649;
            }
            let _e654 = phi_1044_;
            phi_1046_ = _e654;
            break;
        }
        case 0: {
            if (_e520.y > 1f) {
                phi_4015_ = 0.9999999f;
            } else {
                phi_4015_ = select(_e520.y, 0.00000011920929f, (_e520.y < 0f));
            }
            let _e628 = phi_4015_;
            phi_1046_ = _e628;
            break;
        }
        default: {
            phi_1046_ = f32();
            break;
        }
    }
    let _e670 = phi_1046_;
    let _e674 = (_e619 * f32(_e568.member_2.x));
    let _e683 = (_e670 * f32(_e568.member_2.y));
    let _e701 = vec3<f32>((f32((select(select(u32(_e674), 0u, (_e674 < 0f)), 4294967295u, (_e674 > 4294967000f)) + _e568.member_1.x)) / _e498), (f32((select(select(u32(_e683), 0u, (_e683 < 0f)), 4294967295u, (_e683 > 4294967000f)) + _e568.member_1.y)) / _e500), f32(_e568.member_3));
    let _e707 = textureSampleLevel(global_12, global_11, vec2<f32>(_e701.x, _e701.y), i32(_e701.z), 0f);
    let _e710 = select(_e707, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e317.member_6 == 4294967295u)));
    let _e714 = select(_e123, _e122, vec2((_e317.member_12 == 0u)));
    if _e323 {
        phi_4047_ = (_e317.member_7 <= (_e117 - 8u));
    } else {
        phi_4047_ = false;
    }
    let _e719 = phi_4047_;
    if _e719 {
        let _e722 = global.member[_e317.member_7];
        let _e726 = global.member[(_e317.member_7 + 1u)];
        let _e731 = global.member[(_e317.member_7 + 2u)];
        let _e735 = global.member[(_e317.member_7 + 3u)];
        let _e740 = global.member[(_e317.member_7 + 4u)];
        let _e744 = global.member[(_e317.member_7 + 5u)];
        let _e748 = global.member[(_e317.member_7 + 6u)];
        switch bitcast<i32>(_e748) {
            case 0: {
                phi_1129_ = 0u;
                break;
            }
            case 1: {
                phi_1129_ = 1u;
                break;
            }
            case 2: {
                phi_1129_ = 2u;
                break;
            }
            default: {
                phi_1129_ = 0u;
                break;
            }
        }
        let _e751 = phi_1129_;
        let _e755 = global.member[(_e317.member_7 + 7u)];
        switch bitcast<i32>(_e755) {
            case 0: {
                phi_1138_ = 0u;
                break;
            }
            case 1: {
                phi_1138_ = 1u;
                break;
            }
            case 2: {
                phi_1138_ = 2u;
                break;
            }
            default: {
                phi_1138_ = 0u;
                break;
            }
        }
        let _e758 = phi_1138_;
        phi_1151_ = type_33(type_24(_e751, _e758), vec2<u32>(_e722, _e726), vec2<u32>(_e731, _e735), _e740, _e744);
    } else {
        phi_1151_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e762 = phi_1151_;
    switch bitcast<i32>(_e762.member.member) {
        case 1: {
            let _e800 = abs(_e714.x);
            let _e802 = (_e800 % 1f);
            if (_e800 >= 1f) {
                phi_4067_ = select(true, false, (_e802 == 0f));
            } else {
                phi_4067_ = true;
            }
            let _e806 = phi_4067_;
            let _e807 = select(1f, _e802, _e806);
            if (select(-1f, 1f, (_e714.x >= 0f)) > 0f) {
                phi_1171_ = _e807;
            } else {
                phi_1171_ = (1f - _e807);
            }
            let _e811 = phi_1171_;
            phi_1208_ = _e811;
            break;
        }
        case 2: {
            let _e774 = abs(_e714.x);
            let _e781 = ((select(select(u32(_e774), 0u, (_e774 < 0f)), 4294967295u, (_e774 > 4294967000f)) % 2u) == 0u);
            let _e783 = (_e774 % 1f);
            if (_e774 >= 1f) {
                phi_4084_ = select(true, false, (_e783 == 0f));
            } else {
                phi_4084_ = true;
            }
            let _e787 = phi_4084_;
            let _e788 = select(1f, _e783, _e787);
            if (select(-1f, 1f, (_e714.x >= 0f)) > 0f) {
                if _e781 {
                    phi_1200_ = _e788;
                } else {
                    phi_1200_ = (1f - _e788);
                }
                let _e795 = phi_1200_;
                phi_1206_ = _e795;
            } else {
                if _e781 {
                    phi_1205_ = (1f - _e788);
                } else {
                    phi_1205_ = _e788;
                }
                let _e792 = phi_1205_;
                phi_1206_ = _e792;
            }
            let _e797 = phi_1206_;
            phi_1208_ = _e797;
            break;
        }
        case 0: {
            if (_e714.x > 1f) {
                phi_4102_ = 0.9999999f;
            } else {
                phi_4102_ = select(_e714.x, 0.00000011920929f, (_e714.x < 0f));
            }
            let _e771 = phi_4102_;
            phi_1208_ = _e771;
            break;
        }
        default: {
            phi_1208_ = f32();
            break;
        }
    }
    let _e813 = phi_1208_;
    switch bitcast<i32>(_e762.member.member_1) {
        case 1: {
            let _e851 = abs(_e714.y);
            let _e853 = (_e851 % 1f);
            if (_e851 >= 1f) {
                phi_4115_ = select(true, false, (_e853 == 0f));
            } else {
                phi_4115_ = true;
            }
            let _e857 = phi_4115_;
            let _e858 = select(1f, _e853, _e857);
            if (select(-1f, 1f, (_e714.y >= 0f)) > 0f) {
                phi_1229_ = _e858;
            } else {
                phi_1229_ = (1f - _e858);
            }
            let _e862 = phi_1229_;
            phi_1266_ = _e862;
            break;
        }
        case 2: {
            let _e825 = abs(_e714.y);
            let _e832 = ((select(select(u32(_e825), 0u, (_e825 < 0f)), 4294967295u, (_e825 > 4294967000f)) % 2u) == 0u);
            let _e834 = (_e825 % 1f);
            if (_e825 >= 1f) {
                phi_4132_ = select(true, false, (_e834 == 0f));
            } else {
                phi_4132_ = true;
            }
            let _e838 = phi_4132_;
            let _e839 = select(1f, _e834, _e838);
            if (select(-1f, 1f, (_e714.y >= 0f)) > 0f) {
                if _e832 {
                    phi_1258_ = _e839;
                } else {
                    phi_1258_ = (1f - _e839);
                }
                let _e846 = phi_1258_;
                phi_1264_ = _e846;
            } else {
                if _e832 {
                    phi_1263_ = (1f - _e839);
                } else {
                    phi_1263_ = _e839;
                }
                let _e843 = phi_1263_;
                phi_1264_ = _e843;
            }
            let _e848 = phi_1264_;
            phi_1266_ = _e848;
            break;
        }
        case 0: {
            if (_e714.y > 1f) {
                phi_4150_ = 0.9999999f;
            } else {
                phi_4150_ = select(_e714.y, 0.00000011920929f, (_e714.y < 0f));
            }
            let _e822 = phi_4150_;
            phi_1266_ = _e822;
            break;
        }
        default: {
            phi_1266_ = f32();
            break;
        }
    }
    let _e864 = phi_1266_;
    let _e868 = (_e813 * f32(_e762.member_2.x));
    let _e877 = (_e864 * f32(_e762.member_2.y));
    let _e895 = vec3<f32>((f32((select(select(u32(_e868), 0u, (_e868 < 0f)), 4294967295u, (_e868 > 4294967000f)) + _e762.member_1.x)) / _e498), (f32((select(select(u32(_e877), 0u, (_e877 < 0f)), 4294967295u, (_e877 > 4294967000f)) + _e762.member_1.y)) / _e500), f32(_e762.member_3));
    let _e901 = textureSampleLevel(global_12, global_11, vec2<f32>(_e895.x, _e895.y), i32(_e895.z), 0f);
    let _e902 = (_e317.member_7 == 4294967295u);
    let _e904 = select(_e901, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e902));
    let _e908 = select(_e123, _e122, vec2((_e317.member_13 == 0u)));
    if _e323 {
        phi_4182_ = (_e317.member_8 <= (_e117 - 8u));
    } else {
        phi_4182_ = false;
    }
    let _e913 = phi_4182_;
    if _e913 {
        let _e916 = global.member[_e317.member_8];
        let _e920 = global.member[(_e317.member_8 + 1u)];
        let _e925 = global.member[(_e317.member_8 + 2u)];
        let _e929 = global.member[(_e317.member_8 + 3u)];
        let _e934 = global.member[(_e317.member_8 + 4u)];
        let _e938 = global.member[(_e317.member_8 + 5u)];
        let _e942 = global.member[(_e317.member_8 + 6u)];
        switch bitcast<i32>(_e942) {
            case 0: {
                phi_1349_ = 0u;
                break;
            }
            case 1: {
                phi_1349_ = 1u;
                break;
            }
            case 2: {
                phi_1349_ = 2u;
                break;
            }
            default: {
                phi_1349_ = 0u;
                break;
            }
        }
        let _e945 = phi_1349_;
        let _e949 = global.member[(_e317.member_8 + 7u)];
        switch bitcast<i32>(_e949) {
            case 0: {
                phi_1358_ = 0u;
                break;
            }
            case 1: {
                phi_1358_ = 1u;
                break;
            }
            case 2: {
                phi_1358_ = 2u;
                break;
            }
            default: {
                phi_1358_ = 0u;
                break;
            }
        }
        let _e952 = phi_1358_;
        phi_1371_ = type_33(type_24(_e945, _e952), vec2<u32>(_e916, _e920), vec2<u32>(_e925, _e929), _e934, _e938);
    } else {
        phi_1371_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e956 = phi_1371_;
    switch bitcast<i32>(_e956.member.member) {
        case 1: {
            let _e994 = abs(_e908.x);
            let _e996 = (_e994 % 1f);
            if (_e994 >= 1f) {
                phi_4202_ = select(true, false, (_e996 == 0f));
            } else {
                phi_4202_ = true;
            }
            let _e1000 = phi_4202_;
            let _e1001 = select(1f, _e996, _e1000);
            if (select(-1f, 1f, (_e908.x >= 0f)) > 0f) {
                phi_1391_ = _e1001;
            } else {
                phi_1391_ = (1f - _e1001);
            }
            let _e1005 = phi_1391_;
            phi_1428_ = _e1005;
            break;
        }
        case 2: {
            let _e968 = abs(_e908.x);
            let _e975 = ((select(select(u32(_e968), 0u, (_e968 < 0f)), 4294967295u, (_e968 > 4294967000f)) % 2u) == 0u);
            let _e977 = (_e968 % 1f);
            if (_e968 >= 1f) {
                phi_4219_ = select(true, false, (_e977 == 0f));
            } else {
                phi_4219_ = true;
            }
            let _e981 = phi_4219_;
            let _e982 = select(1f, _e977, _e981);
            if (select(-1f, 1f, (_e908.x >= 0f)) > 0f) {
                if _e975 {
                    phi_1420_ = _e982;
                } else {
                    phi_1420_ = (1f - _e982);
                }
                let _e989 = phi_1420_;
                phi_1426_ = _e989;
            } else {
                if _e975 {
                    phi_1425_ = (1f - _e982);
                } else {
                    phi_1425_ = _e982;
                }
                let _e986 = phi_1425_;
                phi_1426_ = _e986;
            }
            let _e991 = phi_1426_;
            phi_1428_ = _e991;
            break;
        }
        case 0: {
            if (_e908.x > 1f) {
                phi_4237_ = 0.9999999f;
            } else {
                phi_4237_ = select(_e908.x, 0.00000011920929f, (_e908.x < 0f));
            }
            let _e965 = phi_4237_;
            phi_1428_ = _e965;
            break;
        }
        default: {
            phi_1428_ = f32();
            break;
        }
    }
    let _e1007 = phi_1428_;
    switch bitcast<i32>(_e956.member.member_1) {
        case 1: {
            let _e1045 = abs(_e908.y);
            let _e1047 = (_e1045 % 1f);
            if (_e1045 >= 1f) {
                phi_4250_ = select(true, false, (_e1047 == 0f));
            } else {
                phi_4250_ = true;
            }
            let _e1051 = phi_4250_;
            let _e1052 = select(1f, _e1047, _e1051);
            if (select(-1f, 1f, (_e908.y >= 0f)) > 0f) {
                phi_1449_ = _e1052;
            } else {
                phi_1449_ = (1f - _e1052);
            }
            let _e1056 = phi_1449_;
            phi_1486_ = _e1056;
            break;
        }
        case 2: {
            let _e1019 = abs(_e908.y);
            let _e1026 = ((select(select(u32(_e1019), 0u, (_e1019 < 0f)), 4294967295u, (_e1019 > 4294967000f)) % 2u) == 0u);
            let _e1028 = (_e1019 % 1f);
            if (_e1019 >= 1f) {
                phi_4267_ = select(true, false, (_e1028 == 0f));
            } else {
                phi_4267_ = true;
            }
            let _e1032 = phi_4267_;
            let _e1033 = select(1f, _e1028, _e1032);
            if (select(-1f, 1f, (_e908.y >= 0f)) > 0f) {
                if _e1026 {
                    phi_1478_ = _e1033;
                } else {
                    phi_1478_ = (1f - _e1033);
                }
                let _e1040 = phi_1478_;
                phi_1484_ = _e1040;
            } else {
                if _e1026 {
                    phi_1483_ = (1f - _e1033);
                } else {
                    phi_1483_ = _e1033;
                }
                let _e1037 = phi_1483_;
                phi_1484_ = _e1037;
            }
            let _e1042 = phi_1484_;
            phi_1486_ = _e1042;
            break;
        }
        case 0: {
            if (_e908.y > 1f) {
                phi_4285_ = 0.9999999f;
            } else {
                phi_4285_ = select(_e908.y, 0.00000011920929f, (_e908.y < 0f));
            }
            let _e1016 = phi_4285_;
            phi_1486_ = _e1016;
            break;
        }
        default: {
            phi_1486_ = f32();
            break;
        }
    }
    let _e1058 = phi_1486_;
    let _e1062 = (_e1007 * f32(_e956.member_2.x));
    let _e1071 = (_e1058 * f32(_e956.member_2.y));
    let _e1089 = vec3<f32>((f32((select(select(u32(_e1062), 0u, (_e1062 < 0f)), 4294967295u, (_e1062 > 4294967000f)) + _e956.member_1.x)) / _e498), (f32((select(select(u32(_e1071), 0u, (_e1071 < 0f)), 4294967295u, (_e1071 > 4294967000f)) + _e956.member_1.y)) / _e500), f32(_e956.member_3));
    let _e1095 = textureSampleLevel(global_12, global_11, vec2<f32>(_e1089.x, _e1089.y), i32(_e1089.z), 0f);
    let _e1102 = select(_e123, _e122, vec2((_e317.member_14 == 0u)));
    if _e323 {
        phi_4317_ = (_e317.member_9 <= (_e117 - 8u));
    } else {
        phi_4317_ = false;
    }
    let _e1107 = phi_4317_;
    if _e1107 {
        let _e1110 = global.member[_e317.member_9];
        let _e1114 = global.member[(_e317.member_9 + 1u)];
        let _e1119 = global.member[(_e317.member_9 + 2u)];
        let _e1123 = global.member[(_e317.member_9 + 3u)];
        let _e1128 = global.member[(_e317.member_9 + 4u)];
        let _e1132 = global.member[(_e317.member_9 + 5u)];
        let _e1136 = global.member[(_e317.member_9 + 6u)];
        switch bitcast<i32>(_e1136) {
            case 0: {
                phi_1569_ = 0u;
                break;
            }
            case 1: {
                phi_1569_ = 1u;
                break;
            }
            case 2: {
                phi_1569_ = 2u;
                break;
            }
            default: {
                phi_1569_ = 0u;
                break;
            }
        }
        let _e1139 = phi_1569_;
        let _e1143 = global.member[(_e317.member_9 + 7u)];
        switch bitcast<i32>(_e1143) {
            case 0: {
                phi_1578_ = 0u;
                break;
            }
            case 1: {
                phi_1578_ = 1u;
                break;
            }
            case 2: {
                phi_1578_ = 2u;
                break;
            }
            default: {
                phi_1578_ = 0u;
                break;
            }
        }
        let _e1146 = phi_1578_;
        phi_1591_ = type_33(type_24(_e1139, _e1146), vec2<u32>(_e1110, _e1114), vec2<u32>(_e1119, _e1123), _e1128, _e1132);
    } else {
        phi_1591_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1150 = phi_1591_;
    switch bitcast<i32>(_e1150.member.member) {
        case 1: {
            let _e1188 = abs(_e1102.x);
            let _e1190 = (_e1188 % 1f);
            if (_e1188 >= 1f) {
                phi_4337_ = select(true, false, (_e1190 == 0f));
            } else {
                phi_4337_ = true;
            }
            let _e1194 = phi_4337_;
            let _e1195 = select(1f, _e1190, _e1194);
            if (select(-1f, 1f, (_e1102.x >= 0f)) > 0f) {
                phi_1611_ = _e1195;
            } else {
                phi_1611_ = (1f - _e1195);
            }
            let _e1199 = phi_1611_;
            phi_1648_ = _e1199;
            break;
        }
        case 2: {
            let _e1162 = abs(_e1102.x);
            let _e1169 = ((select(select(u32(_e1162), 0u, (_e1162 < 0f)), 4294967295u, (_e1162 > 4294967000f)) % 2u) == 0u);
            let _e1171 = (_e1162 % 1f);
            if (_e1162 >= 1f) {
                phi_4354_ = select(true, false, (_e1171 == 0f));
            } else {
                phi_4354_ = true;
            }
            let _e1175 = phi_4354_;
            let _e1176 = select(1f, _e1171, _e1175);
            if (select(-1f, 1f, (_e1102.x >= 0f)) > 0f) {
                if _e1169 {
                    phi_1640_ = _e1176;
                } else {
                    phi_1640_ = (1f - _e1176);
                }
                let _e1183 = phi_1640_;
                phi_1646_ = _e1183;
            } else {
                if _e1169 {
                    phi_1645_ = (1f - _e1176);
                } else {
                    phi_1645_ = _e1176;
                }
                let _e1180 = phi_1645_;
                phi_1646_ = _e1180;
            }
            let _e1185 = phi_1646_;
            phi_1648_ = _e1185;
            break;
        }
        case 0: {
            if (_e1102.x > 1f) {
                phi_4372_ = 0.9999999f;
            } else {
                phi_4372_ = select(_e1102.x, 0.00000011920929f, (_e1102.x < 0f));
            }
            let _e1159 = phi_4372_;
            phi_1648_ = _e1159;
            break;
        }
        default: {
            phi_1648_ = f32();
            break;
        }
    }
    let _e1201 = phi_1648_;
    switch bitcast<i32>(_e1150.member.member_1) {
        case 1: {
            let _e1239 = abs(_e1102.y);
            let _e1241 = (_e1239 % 1f);
            if (_e1239 >= 1f) {
                phi_4385_ = select(true, false, (_e1241 == 0f));
            } else {
                phi_4385_ = true;
            }
            let _e1245 = phi_4385_;
            let _e1246 = select(1f, _e1241, _e1245);
            if (select(-1f, 1f, (_e1102.y >= 0f)) > 0f) {
                phi_1669_ = _e1246;
            } else {
                phi_1669_ = (1f - _e1246);
            }
            let _e1250 = phi_1669_;
            phi_1706_ = _e1250;
            break;
        }
        case 2: {
            let _e1213 = abs(_e1102.y);
            let _e1220 = ((select(select(u32(_e1213), 0u, (_e1213 < 0f)), 4294967295u, (_e1213 > 4294967000f)) % 2u) == 0u);
            let _e1222 = (_e1213 % 1f);
            if (_e1213 >= 1f) {
                phi_4402_ = select(true, false, (_e1222 == 0f));
            } else {
                phi_4402_ = true;
            }
            let _e1226 = phi_4402_;
            let _e1227 = select(1f, _e1222, _e1226);
            if (select(-1f, 1f, (_e1102.y >= 0f)) > 0f) {
                if _e1220 {
                    phi_1698_ = _e1227;
                } else {
                    phi_1698_ = (1f - _e1227);
                }
                let _e1234 = phi_1698_;
                phi_1704_ = _e1234;
            } else {
                if _e1220 {
                    phi_1703_ = (1f - _e1227);
                } else {
                    phi_1703_ = _e1227;
                }
                let _e1231 = phi_1703_;
                phi_1704_ = _e1231;
            }
            let _e1236 = phi_1704_;
            phi_1706_ = _e1236;
            break;
        }
        case 0: {
            if (_e1102.y > 1f) {
                phi_4420_ = 0.9999999f;
            } else {
                phi_4420_ = select(_e1102.y, 0.00000011920929f, (_e1102.y < 0f));
            }
            let _e1210 = phi_4420_;
            phi_1706_ = _e1210;
            break;
        }
        default: {
            phi_1706_ = f32();
            break;
        }
    }
    let _e1252 = phi_1706_;
    let _e1256 = (_e1201 * f32(_e1150.member_2.x));
    let _e1265 = (_e1252 * f32(_e1150.member_2.y));
    let _e1283 = vec3<f32>((f32((select(select(u32(_e1256), 0u, (_e1256 < 0f)), 4294967295u, (_e1256 > 4294967000f)) + _e1150.member_1.x)) / _e498), (f32((select(select(u32(_e1265), 0u, (_e1265 < 0f)), 4294967295u, (_e1265 > 4294967000f)) + _e1150.member_1.y)) / _e500), f32(_e1150.member_3));
    let _e1289 = textureSampleLevel(global_12, global_11, vec2<f32>(_e1283.x, _e1283.y), i32(_e1283.z), 0f);
    let _e1292 = select(_e1289, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e317.member_9 == 4294967295u)));
    if _e902 {
        phi_1800_ = vec3<f32>(0f, 0f, 0f);
        phi_1801_ = _e124;
    } else {
        let _e1296 = fma(_e904.x, 2f, -1f);
        let _e1297 = fma(_e904.y, 2f, -1f);
        let _e1298 = fma(_e904.z, 2f, -1f);
        let _e1303 = sqrt(fma(_e1298, _e1298, fma(_e1296, _e1296, (_e1297 * _e1297))));
        if (_e1303 == 0f) {
            phi_4474_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4474_ = (vec3<f32>(_e1296, _e1297, _e1298) * (1f / _e1303));
        }
        let _e1308 = phi_4474_;
        let _e1315 = sqrt(fma(_e125.z, _e125.z, fma(_e125.x, _e125.x, (_e125.y * _e125.y))));
        if (_e1315 == 0f) {
            phi_4509_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4509_ = (_e125 * (1f / _e1315));
        }
        let _e1320 = phi_4509_;
        let _e1327 = sqrt(fma(_e126.z, _e126.z, fma(_e126.x, _e126.x, (_e126.y * _e126.y))));
        if (_e1327 == 0f) {
            phi_4544_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4544_ = (_e126 * (1f / _e1327));
        }
        let _e1332 = phi_4544_;
        let _e1339 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
        if (_e1339 == 0f) {
            phi_4579_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4579_ = (_e124 * (1f / _e1339));
        }
        let _e1344 = phi_4579_;
        let _e1363 = fma(_e1344.x, _e1308.z, fma(_e1320.x, _e1308.x, (_e1332.x * _e1308.y)));
        let _e1364 = fma(_e1344.y, _e1308.z, fma(_e1320.y, _e1308.x, (_e1332.y * _e1308.y)));
        let _e1365 = fma(_e1344.z, _e1308.z, fma(_e1320.z, _e1308.x, (_e1332.z * _e1308.y)));
        let _e1370 = sqrt(fma(_e1365, _e1365, fma(_e1363, _e1363, (_e1364 * _e1364))));
        if (_e1370 == 0f) {
            phi_4614_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4614_ = (vec3<f32>(_e1363, _e1364, _e1365) * (1f / _e1370));
        }
        let _e1375 = phi_4614_;
        phi_1800_ = _e1308;
        phi_1801_ = _e1375;
    }
    let _e1377 = phi_1800_;
    let _e1379 = phi_1801_;
    let _e1383 = (_e516.x * _e317.member_2.x);
    let _e1386 = (_e516.y * _e317.member_2.y);
    let _e1389 = (_e516.z * _e317.member_2.z);
    let _e1394 = (_e1383 * _e121.x);
    let _e1396 = (_e1386 * _e121.y);
    let _e1398 = (_e1389 * _e121.z);
    let _e1403 = (_e710.y * _e317.member_4);
    let _e1406 = (_e710.z * _e317.member_3);
    let _e1410 = fma(_e317.member_16, (select(_e1095, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e317.member_8 == 4294967295u))).x - 1f), 1f);
    let _e1416 = (_e1292.x * _e317.member.x);
    let _e1418 = (_e1292.y * _e317.member.y);
    let _e1420 = (_e1292.z * _e317.member.z);
    let _e1425 = textureSampleLevel(global_13, global_14, _e1379, 0f);
    if (_e117 >= 83u) {
        phi_4646_ = (_e118 <= (_e117 - 83u));
    } else {
        phi_4646_ = false;
    }
    let _e1433 = phi_4646_;
    if _e1433 {
        let _e1436 = global.member[_e118];
        let _e1441 = global.member[(_e118 + 1u)];
        let _e1446 = global.member[(_e118 + 2u)];
        let _e1451 = global.member[(_e118 + 3u)];
        let _e1457 = global.member[(_e118 + 4u)];
        let _e1462 = global.member[(_e118 + 5u)];
        let _e1467 = global.member[(_e118 + 6u)];
        let _e1472 = global.member[(_e118 + 7u)];
        let _e1478 = global.member[(_e118 + 8u)];
        let _e1483 = global.member[(_e118 + 9u)];
        let _e1488 = global.member[(_e118 + 10u)];
        let _e1493 = global.member[(_e118 + 11u)];
        let _e1499 = global.member[(_e118 + 12u)];
        let _e1504 = global.member[(_e118 + 13u)];
        let _e1509 = global.member[(_e118 + 14u)];
        let _e1514 = global.member[(_e118 + 15u)];
        let _e1521 = global.member[(_e118 + 16u)];
        let _e1526 = global.member[(_e118 + 17u)];
        let _e1531 = global.member[(_e118 + 18u)];
        let _e1536 = global.member[(_e118 + 19u)];
        let _e1542 = global.member[(_e118 + 20u)];
        let _e1547 = global.member[(_e118 + 21u)];
        let _e1552 = global.member[(_e118 + 22u)];
        let _e1557 = global.member[(_e118 + 23u)];
        let _e1563 = global.member[(_e118 + 24u)];
        let _e1568 = global.member[(_e118 + 25u)];
        let _e1573 = global.member[(_e118 + 26u)];
        let _e1578 = global.member[(_e118 + 27u)];
        let _e1584 = global.member[(_e118 + 28u)];
        let _e1589 = global.member[(_e118 + 29u)];
        let _e1594 = global.member[(_e118 + 30u)];
        let _e1599 = global.member[(_e118 + 31u)];
        let _e1606 = global.member[(_e118 + 32u)];
        let _e1611 = global.member[(_e118 + 33u)];
        let _e1616 = global.member[(_e118 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_2007_ = type_24(0u, 6u);
        loop {
            let _e1621 = phi_2007_;
            if (_e1621.member < _e1621.member_1) {
                phi_2008_ = type_24((_e1621.member + 1u), _e1621.member_1);
                phi_2023_ = type_24(1u, _e1621.member);
            } else {
                phi_2008_ = _e1621;
                phi_2023_ = type_24(0u, type_24().member_1);
            }
            let _e1634 = phi_2008_;
            let _e1636 = phi_2023_;
            switch bitcast<i32>(_e1636.member) {
                case 0: {
                    phi_2050_ = false;
                    break;
                }
                case 1: {
                    let _e1641 = ((_e118 + 35u) + (_e1636.member_1 * 4u));
                    let _e1644 = global.member[_e1641];
                    let _e1649 = global.member[(_e1641 + 1u)];
                    let _e1654 = global.member[(_e1641 + 2u)];
                    let _e1659 = global.member[(_e1641 + 3u)];
                    local_1[_e1636.member_1] = vec4<f32>(bitcast<f32>(_e1644), bitcast<f32>(_e1649), bitcast<f32>(_e1654), bitcast<f32>(_e1659));
                    phi_2050_ = true;
                    break;
                }
                default: {
                    phi_2050_ = bool();
                    break;
                }
            }
            let _e1664 = phi_2050_;
            continue;
            continuing {
                phi_2007_ = _e1634;
                break if !(_e1664);
            }
        }
        let _e1666 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2056_ = type_24(0u, 8u);
        loop {
            let _e1669 = phi_2056_;
            if (_e1669.member < _e1669.member_1) {
                phi_2057_ = type_24((_e1669.member + 1u), _e1669.member_1);
                phi_2072_ = type_24(1u, _e1669.member);
            } else {
                phi_2057_ = _e1669;
                phi_2072_ = type_24(0u, type_24().member_1);
            }
            let _e1682 = phi_2057_;
            let _e1684 = phi_2072_;
            switch bitcast<i32>(_e1684.member) {
                case 0: {
                    phi_2095_ = false;
                    break;
                }
                case 1: {
                    let _e1689 = ((_e118 + 59u) + (_e1684.member_1 * 3u));
                    let _e1692 = global.member[_e1689];
                    let _e1697 = global.member[(_e1689 + 1u)];
                    let _e1702 = global.member[(_e1689 + 2u)];
                    local[_e1684.member_1] = vec3<f32>(bitcast<f32>(_e1692), bitcast<f32>(_e1697), bitcast<f32>(_e1702));
                    phi_2095_ = true;
                    break;
                }
                default: {
                    phi_2095_ = bool();
                    break;
                }
            }
            let _e1707 = phi_2095_;
            continue;
            continuing {
                phi_2056_ = _e1682;
                break if !(_e1707);
            }
        }
        let _e1709 = local;
        phi_2103_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1436), bitcast<f32>(_e1441), bitcast<f32>(_e1446), bitcast<f32>(_e1451)), vec4<f32>(bitcast<f32>(_e1457), bitcast<f32>(_e1462), bitcast<f32>(_e1467), bitcast<f32>(_e1472)), vec4<f32>(bitcast<f32>(_e1478), bitcast<f32>(_e1483), bitcast<f32>(_e1488), bitcast<f32>(_e1493)), vec4<f32>(bitcast<f32>(_e1499), bitcast<f32>(_e1504), bitcast<f32>(_e1509), bitcast<f32>(_e1514))), type_20(vec4<f32>(bitcast<f32>(_e1521), bitcast<f32>(_e1526), bitcast<f32>(_e1531), bitcast<f32>(_e1536)), vec4<f32>(bitcast<f32>(_e1542), bitcast<f32>(_e1547), bitcast<f32>(_e1552), bitcast<f32>(_e1557)), vec4<f32>(bitcast<f32>(_e1563), bitcast<f32>(_e1568), bitcast<f32>(_e1573), bitcast<f32>(_e1578)), vec4<f32>(bitcast<f32>(_e1584), bitcast<f32>(_e1589), bitcast<f32>(_e1594), bitcast<f32>(_e1599))), type_21(_e1709, _e1666), vec3<f32>(bitcast<f32>(_e1606), bitcast<f32>(_e1611), bitcast<f32>(_e1616)));
    } else {
        phi_2103_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
    }
    let _e1713 = phi_2103_;
    let _e1715 = (_e1713.member_3 - _e127);
    let _e1722 = sqrt(fma(_e1715.z, _e1715.z, fma(_e1715.x, _e1715.x, (_e1715.y * _e1715.y))));
    let _e1723 = (_e1722 == 0f);
    if _e1723 {
        phi_4718_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4718_ = (_e1715 * (1f / _e1722));
    }
    let _e1727 = phi_4718_;
    let _e1728 = -(_e1727);
    let _e1735 = sqrt(fma(_e1379.z, _e1379.z, fma(_e1379.x, _e1379.x, (_e1379.y * _e1379.y))));
    let _e1736 = (_e1735 == 0f);
    if _e1736 {
        phi_4777_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4777_ = (_e1379 * (1f / _e1735));
    }
    let _e1740 = phi_4777_;
    let _e1750 = (2f * fma(_e1740.z, _e1728.z, fma(_e1740.x, _e1728.x, (_e1740.y * _e1728.y))));
    let _e1757 = textureSampleLevel(global_15, global_16, (_e1728 - vec3<f32>((_e1750 * _e1740.x), (_e1750 * _e1740.y), (_e1750 * _e1740.z))), (_e1403 * 4f));
    if _e1723 {
        phi_4851_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4851_ = (_e1715 * (1f / _e1722));
    }
    let _e1764 = phi_4851_;
    let _e1773 = textureSampleLevel(global_17, global_18, vec2<f32>(max(fma(_e1379.z, _e1764.z, fma(_e1379.x, _e1764.x, (_e1379.y * _e1764.y))), 0f), _e1403), 0f);
    switch bitcast<i32>(_e178.member_3) {
        case 0: {
            if _e317.member_15 {
                if _e1736 {
                    phi_4901_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_4901_ = (_e1379 * (1f / _e1735));
                }
                let _e1942 = phi_4901_;
                if _e1723 {
                    phi_4936_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_4936_ = (_e1715 * (1f / _e1722));
                }
                let _e1946 = phi_4936_;
                phi_2143_ = type_24(0u, _e178.member_2.member_1);
                phi_2146_ = vec3<f32>(0f, 0f, 0f);
                loop {
                    let _e1949 = phi_2143_;
                    let _e1951 = phi_2146_;
                    local_2 = _e1951;
                    local_3 = _e1951;
                    local_4 = _e1951;
                    if (_e1949.member < _e1949.member_1) {
                        phi_2144_ = type_24((_e1949.member + 1u), _e1949.member_1);
                        phi_2161_ = type_24(1u, _e1949.member);
                    } else {
                        phi_2144_ = _e1949;
                        phi_2161_ = type_24(0u, type_24().member_1);
                    }
                    let _e1964 = phi_2144_;
                    let _e1966 = phi_2161_;
                    switch bitcast<i32>(_e1966.member) {
                        case 0: {
                            phi_2147_ = vec3<f32>();
                            phi_3013_ = false;
                            break;
                        }
                        case 1: {
                            if (_e1966.member_1 >= _e178.member_2.member_1) {
                                phi_4953_ = 4294967295u;
                            } else {
                                phi_4953_ = (_e178.member_2.member + _e1966.member_1);
                            }
                            let _e1973 = phi_4953_;
                            if (_e117 >= 1u) {
                                phi_4972_ = (_e1973 <= (_e117 - 1u));
                            } else {
                                phi_4972_ = false;
                            }
                            let _e1978 = phi_4972_;
                            if _e1978 {
                                let _e1981 = global.member[_e1973];
                                phi_2178_ = _e1981;
                            } else {
                                phi_2178_ = 4294967295u;
                            }
                            let _e1983 = phi_2178_;
                            let _e1984 = (_e1983 == 4294967295u);
                            if _e1984 {
                                phi_3011_ = vec3<f32>();
                            } else {
                                if (_e117 >= 3u) {
                                    phi_5004_ = (_e1983 <= (_e117 - 3u));
                                } else {
                                    phi_5004_ = false;
                                }
                                let _e1989 = phi_5004_;
                                if _e1989 {
                                    let _e1992 = global.member[_e1983];
                                    switch bitcast<i32>(_e1992) {
                                        case 0: {
                                            phi_2195_ = 0u;
                                            break;
                                        }
                                        case 1: {
                                            phi_2195_ = 1u;
                                            break;
                                        }
                                        case 2: {
                                            phi_2195_ = 2u;
                                            break;
                                        }
                                        default: {
                                            phi_2195_ = 0u;
                                            break;
                                        }
                                    }
                                    let _e1995 = phi_2195_;
                                    let _e1999 = global.member[(_e1983 + 1u)];
                                    let _e2003 = global.member[(_e1983 + 2u)];
                                    phi_2205_ = type_34(_e1995, _e1999, _e2003);
                                } else {
                                    phi_2205_ = type_34(0u, 4294967295u, 4294967295u);
                                }
                                let _e2006 = phi_2205_;
                                if (_e117 >= 10u) {
                                    phi_5034_ = (_e2006.member_2 <= (_e117 - 10u));
                                } else {
                                    phi_5034_ = false;
                                }
                                let _e2012 = phi_5034_;
                                if _e2012 {
                                    let _e2015 = global.member[_e2006.member_2];
                                    let _e2020 = global.member[(_e2006.member_2 + 1u)];
                                    let _e2025 = global.member[(_e2006.member_2 + 2u)];
                                    let _e2031 = global.member[(_e2006.member_2 + 3u)];
                                    let _e2036 = global.member[(_e2006.member_2 + 4u)];
                                    let _e2041 = global.member[(_e2006.member_2 + 5u)];
                                    let _e2046 = global.member[(_e2006.member_2 + 6u)];
                                    let _e2052 = global.member[(_e2006.member_2 + 7u)];
                                    let _e2057 = global.member[(_e2006.member_2 + 8u)];
                                    let _e2062 = global.member[(_e2006.member_2 + 9u)];
                                    phi_2255_ = type_28(vec3<f32>(bitcast<f32>(_e2015), bitcast<f32>(_e2020), bitcast<f32>(_e2025)), vec4<f32>(bitcast<f32>(_e2031), bitcast<f32>(_e2036), bitcast<f32>(_e2041), bitcast<f32>(_e2046)), vec3<f32>(bitcast<f32>(_e2052), bitcast<f32>(_e2057), bitcast<f32>(_e2062)));
                                } else {
                                    phi_2255_ = type_28(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e2067 = phi_2255_;
                                let _e2075 = (_e2067.member_1.x + _e2067.member_1.x);
                                let _e2076 = (_e2067.member_1.y + _e2067.member_1.y);
                                let _e2077 = (_e2067.member_1.z + _e2067.member_1.z);
                                let _e2079 = (_e2067.member_1.z * _e2077);
                                let _e2080 = (_e2067.member_1.w * _e2075);
                                let _e2081 = (_e2067.member_1.w * _e2076);
                                let _e2082 = (_e2067.member_1.w * _e2077);
                                let _e2102 = (vec4<f32>((1f - fma(_e2067.member_1.y, _e2076, _e2079)), fma(_e2067.member_1.x, _e2076, _e2082), fma(_e2067.member_1.x, _e2077, -(_e2081)), 0f) * _e2067.member_2.x);
                                let _e2104 = (vec4<f32>(fma(_e2067.member_1.x, _e2076, -(_e2082)), (1f - fma(_e2067.member_1.x, _e2075, _e2079)), fma(_e2067.member_1.y, _e2077, _e2080), 0f) * _e2067.member_2.y);
                                let _e2106 = (vec4<f32>(fma(_e2067.member_1.x, _e2077, _e2081), fma(_e2067.member_1.y, _e2077, -(_e2080)), (1f - fma(_e2067.member_1.x, _e2075, (_e2067.member_1.y * _e2076))), 0f) * _e2067.member_2.z);
                                switch bitcast<i32>(_e2006.member) {
                                    case 0: {
                                        if _e323 {
                                            phi_5115_ = (_e2006.member_1 <= (_e117 - 8u));
                                        } else {
                                            phi_5115_ = false;
                                        }
                                        let _e2602 = phi_5115_;
                                        if _e2602 {
                                            let _e2605 = global.member[_e2006.member_1];
                                            let _e2610 = global.member[(_e2006.member_1 + 1u)];
                                            let _e2615 = global.member[(_e2006.member_1 + 2u)];
                                            let _e2621 = global.member[(_e2006.member_1 + 3u)];
                                            let _e2626 = global.member[(_e2006.member_1 + 4u)];
                                            let _e2631 = global.member[(_e2006.member_1 + 5u)];
                                            let _e2636 = global.member[(_e2006.member_1 + 6u)];
                                            let _e2642 = global.member[(_e2006.member_1 + 7u)];
                                            phi_2303_ = type_35(vec3<f32>(bitcast<f32>(_e2605), bitcast<f32>(_e2610), bitcast<f32>(_e2615)), vec4<f32>(bitcast<f32>(_e2621), bitcast<f32>(_e2626), bitcast<f32>(_e2631), bitcast<f32>(_e2636)), bitcast<f32>(_e2642));
                                        } else {
                                            phi_2303_ = type_35(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2646 = phi_2303_;
                                        let _e2668 = fma(_e2106.x, _e2646.member.z, fma(_e2104.x, _e2646.member.y, (_e2102.x * _e2646.member.x)));
                                        let _e2669 = fma(_e2106.y, _e2646.member.z, fma(_e2104.y, _e2646.member.y, (_e2102.y * _e2646.member.x)));
                                        let _e2670 = fma(_e2106.z, _e2646.member.z, fma(_e2104.z, _e2646.member.y, (_e2102.z * _e2646.member.x)));
                                        let _e2675 = sqrt(fma(_e2670, _e2670, fma(_e2668, _e2668, (_e2669 * _e2669))));
                                        if (_e2675 == 0f) {
                                            phi_5163_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_5163_ = (vec3<f32>(_e2668, _e2669, _e2670) * (1f / _e2675));
                                        }
                                        let _e2680 = phi_5163_;
                                        let _e2682 = -(_e2680.x);
                                        let _e2684 = -(_e2680.y);
                                        let _e2686 = -(_e2680.z);
                                        let _e2687 = -(_e2680);
                                        let _e2691 = fma(fma(_e1383, _e121.x, -0.4f), _e1406, 0.4f);
                                        let _e2692 = fma(fma(_e1386, _e121.y, -0.4f), _e1406, 0.4f);
                                        let _e2693 = fma(fma(_e1389, _e121.z, -0.4f), _e1406, 0.4f);
                                        let _e2701 = (_e1946 + vec3<f32>(_e2682, _e2684, _e2686));
                                        let _e2708 = sqrt(fma(_e2701.z, _e2701.z, fma(_e2701.x, _e2701.x, (_e2701.y * _e2701.y))));
                                        if (_e2708 == 0f) {
                                            phi_5198_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_5198_ = (_e2701 * (1f / _e2708));
                                        }
                                        let _e2713 = phi_5198_;
                                        let _e2714 = (_e1403 * _e1403);
                                        let _e2725 = max(fma(_e1942.z, _e2713.z, fma(_e1942.x, _e2713.x, (_e1942.y * _e2713.y))), 0f);
                                        let _e2738 = max(fma(_e1942.z, _e1946.z, fma(_e1942.x, _e1946.x, (_e1942.y * _e1946.y))), 0f);
                                        let _e2745 = max(fma(_e1942.z, _e2687.z, fma(_e1942.x, _e2687.x, (_e1942.y * _e2687.y))), 0f);
                                        let _e2746 = fma(_e710.y, _e317.member_4, 1f);
                                        let _e2747 = (_e2746 * _e2746);
                                        let _e2748 = (_e2747 * 0.125f);
                                        let _e2750 = fma(-(_e2747), 0.125f, 1f);
                                        let _e2763 = (1f - max(fma(_e2713.z, _e1946.z, fma(_e2713.x, _e1946.x, (_e2713.y * _e1946.y))), 0f));
                                        let _e2765 = select(_e2763, 0f, (_e2763 < 0f));
                                        let _e2768 = pow(select(_e2765, 1f, (_e2765 > 1f)), 5f);
                                        let _e2769 = fma((1f - _e2691), _e2768, _e2691);
                                        let _e2770 = fma((1f - _e2692), _e2768, _e2692);
                                        let _e2771 = fma((1f - _e2693), _e2768, _e2693);
                                        let _e2776 = fma(-(_e710.z), _e317.member_3, 1f);
                                        let _e2780 = (((_e2714 * _e2714) / (pow(fma((_e2725 * _e2725), fma(_e2714, _e2714, -1f), 1f), 2f) * 3.1415927f)) * ((_e2738 / fma(_e2738, _e2750, _e2748)) * (_e2745 / fma(_e2745, _e2750, _e2748))));
                                        let _e2787 = max(fma(_e1942.z, _e2686, fma(_e1942.x, _e2682, (_e1942.y * _e2684))), 0f);
                                        let _e2789 = fma((4f * _e2738), _e2787, 0.0001f);
                                        phi_3001_ = vec3<f32>(fma((fma((((1f - _e2769) * _e2776) * _e1394), 0.31830987f, ((_e2780 * _e2769) / _e2789)) * (_e2646.member_1.x * _e2646.member_2)), _e2787, _e1951.x), fma((fma((((1f - _e2770) * _e2776) * _e1396), 0.31830987f, ((_e2780 * _e2770) / _e2789)) * (_e2646.member_1.y * _e2646.member_2)), _e2787, _e1951.y), fma((fma((((1f - _e2771) * _e2776) * _e1398), 0.31830987f, ((_e2780 * _e2771) / _e2789)) * (_e2646.member_1.z * _e2646.member_2)), _e2787, _e1951.z));
                                        phi_3002_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if _e323 {
                                            phi_5290_ = (_e2006.member_1 <= (_e117 - 8u));
                                        } else {
                                            phi_5290_ = false;
                                        }
                                        let _e2391 = phi_5290_;
                                        if _e2391 {
                                            let _e2394 = global.member[_e2006.member_1];
                                            let _e2399 = global.member[(_e2006.member_1 + 1u)];
                                            let _e2404 = global.member[(_e2006.member_1 + 2u)];
                                            let _e2410 = global.member[(_e2006.member_1 + 3u)];
                                            let _e2415 = global.member[(_e2006.member_1 + 4u)];
                                            let _e2420 = global.member[(_e2006.member_1 + 5u)];
                                            let _e2425 = global.member[(_e2006.member_1 + 6u)];
                                            let _e2431 = global.member[(_e2006.member_1 + 7u)];
                                            phi_2505_ = type_35(vec3<f32>(bitcast<f32>(_e2394), bitcast<f32>(_e2399), bitcast<f32>(_e2404)), vec4<f32>(bitcast<f32>(_e2410), bitcast<f32>(_e2415), bitcast<f32>(_e2420), bitcast<f32>(_e2425)), bitcast<f32>(_e2431));
                                        } else {
                                            phi_2505_ = type_35(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2435 = phi_2505_;
                                        let _e2464 = (vec3<f32>((_e2067.member.x + fma(_e2106.x, _e2435.member.z, fma(_e2104.x, _e2435.member.y, (_e2102.x * _e2435.member.x)))), (_e2067.member.y + fma(_e2106.y, _e2435.member.z, fma(_e2104.y, _e2435.member.y, (_e2102.y * _e2435.member.x)))), (_e2067.member.z + fma(_e2106.z, _e2435.member.z, fma(_e2104.z, _e2435.member.y, (_e2102.z * _e2435.member.x))))) - _e127);
                                        let _e2471 = sqrt(fma(_e2464.z, _e2464.z, fma(_e2464.x, _e2464.x, (_e2464.y * _e2464.y))));
                                        let _e2472 = (_e2471 == 0f);
                                        if _e2472 {
                                            phi_2690_ = vec3<f32>();
                                        } else {
                                            if _e2472 {
                                                phi_5336_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5336_ = (_e2464 * (1f / _e2471));
                                            }
                                            let _e2476 = phi_5336_;
                                            let _e2478 = (_e2435.member_2 / (_e2471 * _e2471));
                                            let _e2482 = fma(fma(_e1383, _e121.x, -0.4f), _e1406, 0.4f);
                                            let _e2483 = fma(fma(_e1386, _e121.y, -0.4f), _e1406, 0.4f);
                                            let _e2484 = fma(fma(_e1389, _e121.z, -0.4f), _e1406, 0.4f);
                                            let _e2491 = (_e1946 + _e2476);
                                            let _e2498 = sqrt(fma(_e2491.z, _e2491.z, fma(_e2491.x, _e2491.x, (_e2491.y * _e2491.y))));
                                            if (_e2498 == 0f) {
                                                phi_5371_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5371_ = (_e2491 * (1f / _e2498));
                                            }
                                            let _e2503 = phi_5371_;
                                            let _e2504 = (_e1403 * _e1403);
                                            let _e2515 = max(fma(_e1942.z, _e2503.z, fma(_e1942.x, _e2503.x, (_e1942.y * _e2503.y))), 0f);
                                            let _e2528 = max(fma(_e1942.z, _e1946.z, fma(_e1942.x, _e1946.x, (_e1942.y * _e1946.y))), 0f);
                                            let _e2535 = max(fma(_e1942.z, _e2476.z, fma(_e1942.x, _e2476.x, (_e1942.y * _e2476.y))), 0f);
                                            let _e2536 = fma(_e710.y, _e317.member_4, 1f);
                                            let _e2537 = (_e2536 * _e2536);
                                            let _e2538 = (_e2537 * 0.125f);
                                            let _e2540 = fma(-(_e2537), 0.125f, 1f);
                                            let _e2553 = (1f - max(fma(_e2503.z, _e1946.z, fma(_e2503.x, _e1946.x, (_e2503.y * _e1946.y))), 0f));
                                            let _e2555 = select(_e2553, 0f, (_e2553 < 0f));
                                            let _e2558 = pow(select(_e2555, 1f, (_e2555 > 1f)), 5f);
                                            let _e2559 = fma((1f - _e2482), _e2558, _e2482);
                                            let _e2560 = fma((1f - _e2483), _e2558, _e2483);
                                            let _e2561 = fma((1f - _e2484), _e2558, _e2484);
                                            let _e2566 = fma(-(_e710.z), _e317.member_3, 1f);
                                            let _e2570 = (((_e2504 * _e2504) / (pow(fma((_e2515 * _e2515), fma(_e2504, _e2504, -1f), 1f), 2f) * 3.1415927f)) * ((_e2528 / fma(_e2528, _e2540, _e2538)) * (_e2535 / fma(_e2535, _e2540, _e2538))));
                                            let _e2575 = fma((4f * _e2528), _e2535, 0.0001f);
                                            phi_2690_ = vec3<f32>(fma((fma((((1f - _e2559) * _e2566) * _e1394), 0.31830987f, ((_e2570 * _e2559) / _e2575)) * (_e2435.member_1.x * _e2478)), _e2535, _e1951.x), fma((fma((((1f - _e2560) * _e2566) * _e1396), 0.31830987f, ((_e2570 * _e2560) / _e2575)) * (_e2435.member_1.y * _e2478)), _e2535, _e1951.y), fma((fma((((1f - _e2561) * _e2566) * _e1398), 0.31830987f, ((_e2570 * _e2561) / _e2575)) * (_e2435.member_1.z * _e2478)), _e2535, _e1951.z));
                                        }
                                        let _e2596 = phi_2690_;
                                        phi_3001_ = _e2596;
                                        phi_3002_ = select(true, false, _e2472);
                                        break;
                                    }
                                    case 2: {
                                        if (_e117 >= 13u) {
                                            phi_5463_ = (_e2006.member_1 <= (_e117 - 13u));
                                        } else {
                                            phi_5463_ = false;
                                        }
                                        let _e2117 = phi_5463_;
                                        if _e2117 {
                                            let _e2120 = global.member[_e2006.member_1];
                                            let _e2125 = global.member[(_e2006.member_1 + 1u)];
                                            let _e2130 = global.member[(_e2006.member_1 + 2u)];
                                            let _e2136 = global.member[(_e2006.member_1 + 3u)];
                                            let _e2141 = global.member[(_e2006.member_1 + 4u)];
                                            let _e2146 = global.member[(_e2006.member_1 + 5u)];
                                            let _e2152 = global.member[(_e2006.member_1 + 6u)];
                                            let _e2157 = global.member[(_e2006.member_1 + 7u)];
                                            let _e2162 = global.member[(_e2006.member_1 + 8u)];
                                            let _e2167 = global.member[(_e2006.member_1 + 9u)];
                                            let _e2172 = global.member[(_e2006.member_1 + 10u)];
                                            let _e2177 = global.member[(_e2006.member_1 + 11u)];
                                            let _e2183 = global.member[(_e2006.member_1 + 12u)];
                                            phi_2753_ = type_36(vec3<f32>(bitcast<f32>(_e2120), bitcast<f32>(_e2125), bitcast<f32>(_e2130)), vec3<f32>(bitcast<f32>(_e2136), bitcast<f32>(_e2141), bitcast<f32>(_e2146)), bitcast<f32>(_e2152), bitcast<f32>(_e2157), vec4<f32>(bitcast<f32>(_e2162), bitcast<f32>(_e2167), bitcast<f32>(_e2172), bitcast<f32>(_e2177)), bitcast<f32>(_e2183));
                                        } else {
                                            phi_2753_ = type_36(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2187 = phi_2753_;
                                        let _e2219 = (vec3<f32>((_e2067.member.x + fma(_e2106.x, _e2187.member.z, fma(_e2104.x, _e2187.member.y, (_e2102.x * _e2187.member.x)))), (_e2067.member.y + fma(_e2106.y, _e2187.member.z, fma(_e2104.y, _e2187.member.y, (_e2102.y * _e2187.member.x)))), (_e2067.member.z + fma(_e2106.z, _e2187.member.z, fma(_e2104.z, _e2187.member.y, (_e2102.z * _e2187.member.x))))) - _e127);
                                        let _e2226 = sqrt(fma(_e2219.z, _e2219.z, fma(_e2219.x, _e2219.x, (_e2219.y * _e2219.y))));
                                        let _e2227 = (_e2226 == 0f);
                                        if _e2227 {
                                            phi_2999_ = vec3<f32>();
                                        } else {
                                            if _e2227 {
                                                phi_5513_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5513_ = (_e2219 * (1f / _e2226));
                                            }
                                            let _e2231 = phi_5513_;
                                            let _e2241 = fma(_e2106.x, _e2187.member_1.z, fma(_e2104.x, _e2187.member_1.y, (_e2102.x * _e2187.member_1.x)));
                                            let _e2242 = fma(_e2106.y, _e2187.member_1.z, fma(_e2104.y, _e2187.member_1.y, (_e2102.y * _e2187.member_1.x)));
                                            let _e2243 = fma(_e2106.z, _e2187.member_1.z, fma(_e2104.z, _e2187.member_1.y, (_e2102.z * _e2187.member_1.x)));
                                            let _e2248 = sqrt(fma(_e2243, _e2243, fma(_e2241, _e2241, (_e2242 * _e2242))));
                                            if (_e2248 == 0f) {
                                                phi_5548_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5548_ = (vec3<f32>(_e2241, _e2242, _e2243) * (1f / _e2248));
                                            }
                                            let _e2253 = phi_5548_;
                                            let _e2265 = ((fma(_e2231.z, _e2253.z, fma(_e2231.x, _e2253.x, (_e2231.y * _e2253.y))) - _e2187.member_3) / (_e2187.member_2 - _e2187.member_3));
                                            let _e2267 = select(_e2265, 0f, (_e2265 < 0f));
                                            let _e2270 = (_e2187.member_5 * select(_e2267, 1f, (_e2267 > 1f)));
                                            let _e2274 = fma(fma(_e1383, _e121.x, -0.4f), _e1406, 0.4f);
                                            let _e2275 = fma(fma(_e1386, _e121.y, -0.4f), _e1406, 0.4f);
                                            let _e2276 = fma(fma(_e1389, _e121.z, -0.4f), _e1406, 0.4f);
                                            let _e2283 = (_e1946 + _e2231);
                                            let _e2290 = sqrt(fma(_e2283.z, _e2283.z, fma(_e2283.x, _e2283.x, (_e2283.y * _e2283.y))));
                                            if (_e2290 == 0f) {
                                                phi_5583_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5583_ = (_e2283 * (1f / _e2290));
                                            }
                                            let _e2295 = phi_5583_;
                                            let _e2296 = (_e1403 * _e1403);
                                            let _e2307 = max(fma(_e1942.z, _e2295.z, fma(_e1942.x, _e2295.x, (_e1942.y * _e2295.y))), 0f);
                                            let _e2320 = max(fma(_e1942.z, _e1946.z, fma(_e1942.x, _e1946.x, (_e1942.y * _e1946.y))), 0f);
                                            let _e2324 = max(fma(_e1942.z, _e2231.z, fma(_e1942.x, _e2231.x, (_e1942.y * _e2231.y))), 0f);
                                            let _e2325 = fma(_e710.y, _e317.member_4, 1f);
                                            let _e2326 = (_e2325 * _e2325);
                                            let _e2327 = (_e2326 * 0.125f);
                                            let _e2329 = fma(-(_e2326), 0.125f, 1f);
                                            let _e2342 = (1f - max(fma(_e2295.z, _e1946.z, fma(_e2295.x, _e1946.x, (_e2295.y * _e1946.y))), 0f));
                                            let _e2344 = select(_e2342, 0f, (_e2342 < 0f));
                                            let _e2347 = pow(select(_e2344, 1f, (_e2344 > 1f)), 5f);
                                            let _e2348 = fma((1f - _e2274), _e2347, _e2274);
                                            let _e2349 = fma((1f - _e2275), _e2347, _e2275);
                                            let _e2350 = fma((1f - _e2276), _e2347, _e2276);
                                            let _e2355 = fma(-(_e710.z), _e317.member_3, 1f);
                                            let _e2359 = (((_e2296 * _e2296) / (pow(fma((_e2307 * _e2307), fma(_e2296, _e2296, -1f), 1f), 2f) * 3.1415927f)) * ((_e2320 / fma(_e2320, _e2329, _e2327)) * (_e2324 / fma(_e2324, _e2329, _e2327))));
                                            let _e2364 = fma((4f * _e2320), _e2324, 0.0001f);
                                            phi_2999_ = vec3<f32>(fma((fma((((1f - _e2348) * _e2355) * _e1394), 0.31830987f, ((_e2359 * _e2348) / _e2364)) * (_e2187.member_4.x * _e2270)), _e2324, _e1951.x), fma((fma((((1f - _e2349) * _e2355) * _e1396), 0.31830987f, ((_e2359 * _e2349) / _e2364)) * (_e2187.member_4.y * _e2270)), _e2324, _e1951.y), fma((fma((((1f - _e2350) * _e2355) * _e1398), 0.31830987f, ((_e2359 * _e2350) / _e2364)) * (_e2187.member_4.z * _e2270)), _e2324, _e1951.z));
                                        }
                                        let _e2385 = phi_2999_;
                                        phi_3001_ = _e2385;
                                        phi_3002_ = select(true, false, _e2227);
                                        break;
                                    }
                                    default: {
                                        phi_3001_ = vec3<f32>();
                                        phi_3002_ = bool();
                                        break;
                                    }
                                }
                                let _e2810 = phi_3001_;
                                let _e2812 = phi_3002_;
                                phi_3011_ = select(_e2810, _e1951, vec3(select(true, false, _e2812)));
                            }
                            let _e2817 = phi_3011_;
                            phi_2147_ = _e2817;
                            phi_3013_ = select(true, false, _e1984);
                            break;
                        }
                        default: {
                            phi_2147_ = vec3<f32>();
                            phi_3013_ = bool();
                            break;
                        }
                    }
                    let _e2820 = phi_2147_;
                    let _e2822 = phi_3013_;
                    continue;
                    continuing {
                        phi_2143_ = _e1964;
                        phi_2146_ = _e2820;
                        break if !(_e2822);
                    }
                }
                let _e2827 = fma(fma(_e1383, _e121.x, -0.04f), _e1406, 0.04f);
                let _e2828 = fma(fma(_e1386, _e121.y, -0.04f), _e1406, 0.04f);
                let _e2829 = fma(fma(_e1389, _e121.z, -0.04f), _e1406, 0.04f);
                let _e2841 = fma(-(_e710.y), _e317.member_4, 1f);
                let _e2848 = (1f - max(fma(_e1942.z, _e1946.z, fma(_e1942.x, _e1946.x, (_e1942.y * _e1946.y))), 0f));
                let _e2850 = select(_e2848, 0f, (_e2848 < 0f));
                let _e2853 = pow(select(_e2850, 1f, (_e2850 > 1f)), 5f);
                let _e2854 = fma((max(_e2841, _e2827) - _e2827), _e2853, _e2827);
                let _e2855 = fma((max(_e2841, _e2828) - _e2828), _e2853, _e2828);
                let _e2856 = fma((max(_e2841, _e2829) - _e2829), _e2853, _e2829);
                let _e2861 = fma(-(_e710.z), _e317.member_3, 1f);
                let _e2878 = local_2;
                let _e2882 = local_3;
                let _e2886 = local_4;
                phi_3125_ = vec4<f32>(fma(_e1416, _e317.member_1, fma(fma(((1f - _e2854) * _e2861), (_e1425.x * _e1394), (_e1757.x * fma(_e2854, _e1773.x, _e1773.y))), _e1410, _e2878.x)), fma(_e1418, _e317.member_1, fma(fma(((1f - _e2855) * _e2861), (_e1425.y * _e1396), (_e1757.y * fma(_e2855, _e1773.x, _e1773.y))), _e1410, _e2882.y)), fma(_e1420, _e317.member_1, fma(fma(((1f - _e2856) * _e2861), (_e1425.z * _e1398), (_e1757.z * fma(_e2856, _e1773.x, _e1773.y))), _e1410, _e2886.z)), 1f);
            } else {
                phi_3125_ = (vec4<f32>((_e121.x * _e516.x), (_e121.y * _e516.y), (_e121.z * _e516.z), (_e121.w * _e516.w)) * _e317.member_2);
            }
            let _e2894 = phi_3125_;
            global_19 = _e2894;
            break;
        }
        case 1: {
            let _e1915 = sqrt(fma(_e122.x, _e122.x, (_e122.y * _e122.y)));
            if (_e1915 == 0f) {
                phi_5702_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5702_ = (vec3<f32>(_e122.x, _e122.y, 0f) * (1f / _e1915));
            }
            let _e1920 = phi_5702_;
            global_19 = vec4<f32>(((_e1920.x + 1f) * 0.5f), ((_e1920.y + 1f) * 0.5f), ((_e1920.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1894 = sqrt(fma(_e123.x, _e123.x, (_e123.y * _e123.y)));
            if (_e1894 == 0f) {
                phi_5751_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5751_ = (vec3<f32>(_e123.x, _e123.y, 0f) * (1f / _e1894));
            }
            let _e1899 = phi_5751_;
            global_19 = vec4<f32>(((_e1899.x + 1f) * 0.5f), ((_e1899.y + 1f) * 0.5f), ((_e1899.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1736 {
                phi_5800_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5800_ = (_e1379 * (1f / _e1735));
            }
            let _e1878 = phi_5800_;
            global_19 = vec4<f32>(((_e1878.x + 1f) * 0.5f), ((_e1878.y + 1f) * 0.5f), ((_e1878.z + 1f) * 0.5f), 1f);
            break;
        }
        case 4: {
            global_19 = _e121;
            break;
        }
        case 5: {
            let _e1859 = sqrt(fma(_e124.z, _e124.z, fma(_e124.x, _e124.x, (_e124.y * _e124.y))));
            if (_e1859 == 0f) {
                phi_5849_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5849_ = (_e124 * (1f / _e1859));
            }
            let _e1864 = phi_5849_;
            global_19 = vec4<f32>(((_e1864.x + 1f) * 0.5f), ((_e1864.y + 1f) * 0.5f), ((_e1864.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1837 = sqrt(fma(_e1377.z, _e1377.z, fma(_e1377.x, _e1377.x, (_e1377.y * _e1377.y))));
            if (_e1837 == 0f) {
                phi_5898_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5898_ = (_e1377 * (1f / _e1837));
            }
            let _e1842 = phi_5898_;
            global_19 = vec4<f32>(((_e1842.x + 1f) * 0.5f), ((_e1842.y + 1f) * 0.5f), ((_e1842.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1815 = sqrt(fma(_e125.z, _e125.z, fma(_e125.x, _e125.x, (_e125.y * _e125.y))));
            if (_e1815 == 0f) {
                phi_5947_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5947_ = (_e125 * (1f / _e1815));
            }
            let _e1820 = phi_5947_;
            global_19 = vec4<f32>(((_e1820.x + 1f) * 0.5f), ((_e1820.y + 1f) * 0.5f), ((_e1820.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1793 = sqrt(fma(_e126.z, _e126.z, fma(_e126.x, _e126.x, (_e126.y * _e126.y))));
            if (_e1793 == 0f) {
                phi_5996_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5996_ = (_e126 * (1f / _e1793));
            }
            let _e1798 = phi_5996_;
            global_19 = vec4<f32>(((_e1798.x + 1f) * 0.5f), ((_e1798.y + 1f) * 0.5f), ((_e1798.z + 1f) * 0.5f), 1f);
            break;
        }
        case 9: {
            global_19 = vec4<f32>(_e1425.x, _e1425.y, _e1425.z, 1f);
            break;
        }
        case 10: {
            global_19 = vec4<f32>(_e1757.x, _e1757.y, _e1757.z, 1f);
            break;
        }
        case 11: {
            global_19 = vec4<f32>(_e1773.x, _e1773.y, 1f, 1f);
            break;
        }
        case 12: {
            global_19 = (vec4<f32>(_e1383, _e1386, _e1389, (_e516.w * _e317.member_2.w)) * _e121);
            break;
        }
        case 13: {
            global_19 = vec4<f32>(_e1403, _e1403, _e1403, 1f);
            break;
        }
        case 14: {
            global_19 = vec4<f32>(_e1406, _e1406, _e1406, 1f);
            break;
        }
        case 15: {
            global_19 = vec4<f32>(_e1410, _e1410, _e1410, 1f);
            break;
        }
        case 16: {
            global_19 = vec4<f32>((_e1416 * _e317.member_1), (_e1418 * _e317.member_1), (_e1420 * _e317.member_1), 1f);
            break;
        }
        case 17: {
            global_19 = vec4<f32>(_e1292.x, _e1292.y, _e1292.z, 1f);
            break;
        }
        case 18: {
            global_19 = vec4<f32>(_e317.member.x, _e317.member.y, _e317.member.z, 1f);
            break;
        }
        case 19: {
            global_19 = vec4<f32>(_e317.member_1, _e317.member_1, _e317.member_1, 1f);
            break;
        }
        default: {
            break;
        }
    }
    return;
}

@fragment 
fn stagerenderlet_fragment(@location(0) @interpolate(flat) param: u32, @location(1) @interpolate(flat) param_1: u32, @location(2) @interpolate(flat) param_2: u32, @location(3) param_3: vec4<f32>, @location(4) param_4: vec2<f32>, @location(5) param_5: vec2<f32>, @location(6) param_6: vec3<f32>, @location(7) param_7: vec3<f32>, @location(8) param_8: vec3<f32>, @location(9) param_9: vec3<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    global_2 = param_1;
    global_3 = param_2;
    global_4 = param_3;
    global_5 = param_4;
    global_6 = param_5;
    global_7 = param_6;
    global_8 = param_7;
    global_9 = param_8;
    global_10 = param_9;
    function();
    let _e21 = global_19;
    return _e21;
}
