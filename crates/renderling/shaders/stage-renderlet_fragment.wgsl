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
    var phi_742_: u32;
    var phi_4052_: bool;
    var phi_896_: type_32;
    var phi_900_: type_32;
    var phi_4089_: bool;
    var phi_940_: u32;
    var phi_949_: u32;
    var phi_962_: type_33;
    var phi_4111_: f32;
    var phi_4124_: bool;
    var phi_1016_: f32;
    var phi_1011_: f32;
    var phi_1017_: f32;
    var phi_4141_: bool;
    var phi_982_: f32;
    var phi_1019_: f32;
    var phi_4159_: f32;
    var phi_4172_: bool;
    var phi_1074_: f32;
    var phi_1069_: f32;
    var phi_1075_: f32;
    var phi_4189_: bool;
    var phi_1040_: f32;
    var phi_1077_: f32;
    var phi_4225_: bool;
    var phi_1160_: u32;
    var phi_1169_: u32;
    var phi_1182_: type_33;
    var phi_4246_: f32;
    var phi_4259_: bool;
    var phi_1236_: f32;
    var phi_1231_: f32;
    var phi_1237_: f32;
    var phi_4276_: bool;
    var phi_1202_: f32;
    var phi_1239_: f32;
    var phi_4294_: f32;
    var phi_4307_: bool;
    var phi_1294_: f32;
    var phi_1289_: f32;
    var phi_1295_: f32;
    var phi_4324_: bool;
    var phi_1260_: f32;
    var phi_1297_: f32;
    var phi_4360_: bool;
    var phi_1380_: u32;
    var phi_1389_: u32;
    var phi_1402_: type_33;
    var phi_4381_: f32;
    var phi_4394_: bool;
    var phi_1456_: f32;
    var phi_1451_: f32;
    var phi_1457_: f32;
    var phi_4411_: bool;
    var phi_1422_: f32;
    var phi_1459_: f32;
    var phi_4429_: f32;
    var phi_4442_: bool;
    var phi_1514_: f32;
    var phi_1509_: f32;
    var phi_1515_: f32;
    var phi_4459_: bool;
    var phi_1480_: f32;
    var phi_1517_: f32;
    var phi_4495_: bool;
    var phi_1600_: u32;
    var phi_1609_: u32;
    var phi_1622_: type_33;
    var phi_4516_: f32;
    var phi_4529_: bool;
    var phi_1676_: f32;
    var phi_1671_: f32;
    var phi_1677_: f32;
    var phi_4546_: bool;
    var phi_1642_: f32;
    var phi_1679_: f32;
    var phi_4564_: f32;
    var phi_4577_: bool;
    var phi_1734_: f32;
    var phi_1729_: f32;
    var phi_1735_: f32;
    var phi_4594_: bool;
    var phi_1700_: f32;
    var phi_1737_: f32;
    var phi_4630_: bool;
    var phi_1820_: u32;
    var phi_1829_: u32;
    var phi_1842_: type_33;
    var phi_4651_: f32;
    var phi_4664_: bool;
    var phi_1896_: f32;
    var phi_1891_: f32;
    var phi_1897_: f32;
    var phi_4681_: bool;
    var phi_1862_: f32;
    var phi_1899_: f32;
    var phi_4699_: f32;
    var phi_4712_: bool;
    var phi_1954_: f32;
    var phi_1949_: f32;
    var phi_1955_: f32;
    var phi_4729_: bool;
    var phi_1920_: f32;
    var phi_1957_: f32;
    var phi_4787_: vec3<f32>;
    var phi_4822_: vec3<f32>;
    var phi_4857_: vec3<f32>;
    var phi_4892_: vec3<f32>;
    var phi_4927_: vec3<f32>;
    var phi_2051_: vec3<f32>;
    var phi_2052_: vec3<f32>;
    var phi_4959_: bool;
    var phi_2259_: type_24;
    var phi_2260_: type_24;
    var phi_2283_: type_24;
    var phi_2310_: bool;
    var phi_2316_: type_24;
    var phi_2317_: type_24;
    var phi_2340_: type_24;
    var phi_2363_: bool;
    var phi_2384_: type_22;
    var phi_5031_: vec3<f32>;
    var phi_5090_: vec3<f32>;
    var phi_5164_: vec3<f32>;
    var phi_5224_: vec3<f32>;
    var phi_5273_: vec3<f32>;
    var phi_5322_: vec3<f32>;
    var phi_5371_: vec3<f32>;
    var phi_5420_: vec3<f32>;
    var phi_5469_: vec3<f32>;
    var phi_5518_: vec3<f32>;
    var phi_5557_: vec3<f32>;
    var phi_5592_: vec3<f32>;
    var phi_2424_: type_24;
    var phi_2427_: vec3<f32>;
    var phi_2425_: type_24;
    var phi_2450_: type_24;
    var phi_5609_: u32;
    var phi_5628_: bool;
    var phi_2467_: u32;
    var phi_5660_: bool;
    var phi_2484_: u32;
    var phi_2494_: type_34;
    var phi_5690_: bool;
    var phi_2544_: type_29;
    var phi_5770_: bool;
    var phi_3052_: type_36;
    var phi_5820_: vec3<f32>;
    var phi_5855_: vec3<f32>;
    var phi_5890_: vec3<f32>;
    var phi_3307_: vec3<f32>;
    var phi_5982_: bool;
    var phi_2799_: type_35;
    var phi_6029_: vec3<f32>;
    var phi_6064_: vec3<f32>;
    var phi_2989_: vec3<f32>;
    var phi_6156_: bool;
    var phi_2592_: type_35;
    var phi_6203_: vec3<f32>;
    var phi_6238_: vec3<f32>;
    var phi_3309_: vec3<f32>;
    var phi_3310_: bool;
    var phi_3319_: vec3<f32>;
    var phi_2428_: vec3<f32>;
    var phi_3321_: bool;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var phi_3443_: vec4<f32>;

    let _e116 = arrayLength((&global.member));
    let _e117 = global_2;
    let _e118 = global_3;
    let _e119 = global_4;
    let _e120 = global_5;
    let _e121 = global_6;
    let _e122 = global_7;
    let _e123 = global_8;
    let _e124 = global_9;
    let _e127 = global_1.member[0u];
    let _e131 = global_1.member[1u];
    let _e135 = global_1.member[2u];
    let _e139 = global_1.member[3u];
    let _e143 = global_1.member[4u];
    let _e147 = global_1.member[5u];
    let _e151 = global_1.member[6u];
    let _e155 = global_1.member[7u];
    let _e159 = global_1.member[8u];
    let _e163 = global_1.member[9u];
    let _e167 = global_1.member[10u];
    let _e171 = global_1.member[11u];
    let _e175 = global_1.member[12u];
    let _e179 = global_1.member[13u];
    let _e183 = global_1.member[14u];
    let _e187 = global_1.member[15u];
    let _e207 = (bitcast<f32>(_e187) + fma(bitcast<f32>(_e171), _e124.z, fma(bitcast<f32>(_e155), _e124.y, (bitcast<f32>(_e139) * _e124.x))));
    let _e214 = global.member[(_e117 + 9u)];
    let _e218 = global.member[(_e117 + 11u)];
    let _e222 = global.member[(_e117 + 17u)];
    let _e225 = global.member[_e222];
    let _e229 = global.member[(_e222 + 1u)];
    let _e233 = global.member[(_e222 + 4u)];
    switch bitcast<i32>(_e233) {
        case 0: {
            phi_742_ = 0u;
            break;
        }
        case 1: {
            phi_742_ = 1u;
            break;
        }
        case 2: {
            phi_742_ = 2u;
            break;
        }
        case 3: {
            phi_742_ = 3u;
            break;
        }
        case 4: {
            phi_742_ = 4u;
            break;
        }
        case 5: {
            phi_742_ = 5u;
            break;
        }
        case 6: {
            phi_742_ = 6u;
            break;
        }
        case 7: {
            phi_742_ = 7u;
            break;
        }
        case 8: {
            phi_742_ = 8u;
            break;
        }
        case 9: {
            phi_742_ = 9u;
            break;
        }
        case 10: {
            phi_742_ = 10u;
            break;
        }
        case 11: {
            phi_742_ = 11u;
            break;
        }
        case 12: {
            phi_742_ = 12u;
            break;
        }
        case 13: {
            phi_742_ = 13u;
            break;
        }
        case 14: {
            phi_742_ = 14u;
            break;
        }
        case 15: {
            phi_742_ = 15u;
            break;
        }
        case 16: {
            phi_742_ = 16u;
            break;
        }
        case 17: {
            phi_742_ = 17u;
            break;
        }
        case 18: {
            phi_742_ = 18u;
            break;
        }
        case 19: {
            phi_742_ = 19u;
            break;
        }
        default: {
            phi_742_ = 0u;
            break;
        }
    }
    let _e236 = phi_742_;
    let _e240 = global.member[(_e222 + 5u)];
    let _e245 = global.member[(_e222 + 9u)];
    let _e249 = global.member[(_e222 + 10u)];
    if (_e218 == 4294967295u) {
        phi_900_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e116 >= 22u) {
            phi_4052_ = (_e218 <= (_e116 - 22u));
        } else {
            phi_4052_ = false;
        }
        let _e255 = phi_4052_;
        if _e255 {
            let _e258 = global.member[_e218];
            let _e263 = global.member[(_e218 + 1u)];
            let _e268 = global.member[(_e218 + 2u)];
            let _e274 = global.member[(_e218 + 3u)];
            let _e279 = global.member[(_e218 + 4u)];
            let _e284 = global.member[(_e218 + 5u)];
            let _e289 = global.member[(_e218 + 6u)];
            let _e294 = global.member[(_e218 + 7u)];
            let _e300 = global.member[(_e218 + 8u)];
            let _e305 = global.member[(_e218 + 9u)];
            let _e310 = global.member[(_e218 + 10u)];
            let _e314 = global.member[(_e218 + 11u)];
            let _e318 = global.member[(_e218 + 12u)];
            let _e322 = global.member[(_e218 + 13u)];
            let _e326 = global.member[(_e218 + 14u)];
            let _e330 = global.member[(_e218 + 15u)];
            let _e334 = global.member[(_e218 + 16u)];
            let _e338 = global.member[(_e218 + 17u)];
            let _e342 = global.member[(_e218 + 18u)];
            let _e346 = global.member[(_e218 + 19u)];
            let _e350 = global.member[(_e218 + 20u)];
            let _e355 = global.member[(_e218 + 21u)];
            phi_896_ = type_32(vec3<f32>(bitcast<f32>(_e258), bitcast<f32>(_e263), bitcast<f32>(_e268)), bitcast<f32>(_e274), vec4<f32>(bitcast<f32>(_e279), bitcast<f32>(_e284), bitcast<f32>(_e289), bitcast<f32>(_e294)), bitcast<f32>(_e300), bitcast<f32>(_e305), _e310, _e314, _e318, _e322, _e326, _e330, _e334, _e338, _e342, _e346, (_e350 == 1u), bitcast<f32>(_e355));
        } else {
            phi_896_ = type_32(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e359 = phi_896_;
        phi_900_ = type_32(_e359.member, _e359.member_1, _e359.member_2, _e359.member_3, _e359.member_4, _e359.member_5, _e359.member_6, _e359.member_7, _e359.member_8, _e359.member_9, _e359.member_10, _e359.member_11, _e359.member_12, _e359.member_13, _e359.member_14, (_e359.member_15 && (_e240 == 1u)), _e359.member_16);
    }
    let _e381 = phi_900_;
    let _e385 = select(_e120, _e119, vec2((_e381.member_10 == 0u)));
    let _e387 = (_e116 >= 8u);
    if _e387 {
        phi_4089_ = (_e381.member_5 <= (_e116 - 8u));
    } else {
        phi_4089_ = false;
    }
    let _e391 = phi_4089_;
    if _e391 {
        let _e394 = global.member[_e381.member_5];
        let _e398 = global.member[(_e381.member_5 + 1u)];
        let _e403 = global.member[(_e381.member_5 + 2u)];
        let _e407 = global.member[(_e381.member_5 + 3u)];
        let _e412 = global.member[(_e381.member_5 + 4u)];
        let _e416 = global.member[(_e381.member_5 + 5u)];
        let _e420 = global.member[(_e381.member_5 + 6u)];
        switch bitcast<i32>(_e420) {
            case 0: {
                phi_940_ = 0u;
                break;
            }
            case 1: {
                phi_940_ = 1u;
                break;
            }
            case 2: {
                phi_940_ = 2u;
                break;
            }
            default: {
                phi_940_ = 0u;
                break;
            }
        }
        let _e423 = phi_940_;
        let _e427 = global.member[(_e381.member_5 + 7u)];
        switch bitcast<i32>(_e427) {
            case 0: {
                phi_949_ = 0u;
                break;
            }
            case 1: {
                phi_949_ = 1u;
                break;
            }
            case 2: {
                phi_949_ = 2u;
                break;
            }
            default: {
                phi_949_ = 0u;
                break;
            }
        }
        let _e430 = phi_949_;
        phi_962_ = type_33(type_24(_e423, _e430), vec2<u32>(_e394, _e398), vec2<u32>(_e403, _e407), _e412, _e416);
    } else {
        phi_962_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e434 = phi_962_;
    switch bitcast<i32>(_e434.member.member) {
        case 1: {
            let _e472 = abs(_e385.x);
            let _e474 = (_e472 % 1f);
            if (_e472 >= 1f) {
                phi_4141_ = select(true, false, (_e474 == 0f));
            } else {
                phi_4141_ = true;
            }
            let _e478 = phi_4141_;
            let _e479 = select(1f, _e474, _e478);
            if (select(-1f, 1f, (_e385.x >= 0f)) > 0f) {
                phi_982_ = _e479;
            } else {
                phi_982_ = (1f - _e479);
            }
            let _e483 = phi_982_;
            phi_1019_ = _e483;
            break;
        }
        case 2: {
            let _e446 = abs(_e385.x);
            let _e453 = ((select(select(u32(_e446), 0u, (_e446 < 0f)), 4294967295u, (_e446 > 4294967000f)) % 2u) == 0u);
            let _e455 = (_e446 % 1f);
            if (_e446 >= 1f) {
                phi_4124_ = select(true, false, (_e455 == 0f));
            } else {
                phi_4124_ = true;
            }
            let _e459 = phi_4124_;
            let _e460 = select(1f, _e455, _e459);
            if (select(-1f, 1f, (_e385.x >= 0f)) > 0f) {
                if _e453 {
                    phi_1011_ = _e460;
                } else {
                    phi_1011_ = (1f - _e460);
                }
                let _e467 = phi_1011_;
                phi_1017_ = _e467;
            } else {
                if _e453 {
                    phi_1016_ = (1f - _e460);
                } else {
                    phi_1016_ = _e460;
                }
                let _e464 = phi_1016_;
                phi_1017_ = _e464;
            }
            let _e469 = phi_1017_;
            phi_1019_ = _e469;
            break;
        }
        case 0: {
            if (_e385.x > 1f) {
                phi_4111_ = 0.9999999f;
            } else {
                phi_4111_ = select(_e385.x, 0.00000011920929f, (_e385.x < 0f));
            }
            let _e443 = phi_4111_;
            phi_1019_ = _e443;
            break;
        }
        default: {
            phi_1019_ = f32();
            break;
        }
    }
    let _e485 = phi_1019_;
    switch bitcast<i32>(_e434.member.member_1) {
        case 1: {
            let _e523 = abs(_e385.y);
            let _e525 = (_e523 % 1f);
            if (_e523 >= 1f) {
                phi_4189_ = select(true, false, (_e525 == 0f));
            } else {
                phi_4189_ = true;
            }
            let _e529 = phi_4189_;
            let _e530 = select(1f, _e525, _e529);
            if (select(-1f, 1f, (_e385.y >= 0f)) > 0f) {
                phi_1040_ = _e530;
            } else {
                phi_1040_ = (1f - _e530);
            }
            let _e534 = phi_1040_;
            phi_1077_ = _e534;
            break;
        }
        case 2: {
            let _e497 = abs(_e385.y);
            let _e504 = ((select(select(u32(_e497), 0u, (_e497 < 0f)), 4294967295u, (_e497 > 4294967000f)) % 2u) == 0u);
            let _e506 = (_e497 % 1f);
            if (_e497 >= 1f) {
                phi_4172_ = select(true, false, (_e506 == 0f));
            } else {
                phi_4172_ = true;
            }
            let _e510 = phi_4172_;
            let _e511 = select(1f, _e506, _e510);
            if (select(-1f, 1f, (_e385.y >= 0f)) > 0f) {
                if _e504 {
                    phi_1069_ = _e511;
                } else {
                    phi_1069_ = (1f - _e511);
                }
                let _e518 = phi_1069_;
                phi_1075_ = _e518;
            } else {
                if _e504 {
                    phi_1074_ = (1f - _e511);
                } else {
                    phi_1074_ = _e511;
                }
                let _e515 = phi_1074_;
                phi_1075_ = _e515;
            }
            let _e520 = phi_1075_;
            phi_1077_ = _e520;
            break;
        }
        case 0: {
            if (_e385.y > 1f) {
                phi_4159_ = 0.9999999f;
            } else {
                phi_4159_ = select(_e385.y, 0.00000011920929f, (_e385.y < 0f));
            }
            let _e494 = phi_4159_;
            phi_1077_ = _e494;
            break;
        }
        default: {
            phi_1077_ = f32();
            break;
        }
    }
    let _e536 = phi_1077_;
    let _e540 = (_e485 * f32(_e434.member_2.x));
    let _e549 = (_e536 * f32(_e434.member_2.y));
    let _e561 = f32(_e225);
    let _e562 = f32(_e229);
    let _e569 = vec3<f32>((f32((select(select(u32(_e540), 0u, (_e540 < 0f)), 4294967295u, (_e540 > 4294967000f)) + _e434.member_1.x)) / _e561), (f32((select(select(u32(_e549), 0u, (_e549 < 0f)), 4294967295u, (_e549 > 4294967000f)) + _e434.member_1.y)) / _e562), f32(_e434.member_3));
    let _e575 = textureSampleLevel(global_11, global_10, vec2<f32>(_e569.x, _e569.y), i32(_e569.z), 0f);
    let _e578 = select(_e575, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e381.member_5 == 4294967295u)));
    let _e582 = select(_e120, _e119, vec2((_e381.member_11 == 0u)));
    if _e387 {
        phi_4225_ = (_e381.member_6 <= (_e116 - 8u));
    } else {
        phi_4225_ = false;
    }
    let _e587 = phi_4225_;
    if _e587 {
        let _e590 = global.member[_e381.member_6];
        let _e594 = global.member[(_e381.member_6 + 1u)];
        let _e599 = global.member[(_e381.member_6 + 2u)];
        let _e603 = global.member[(_e381.member_6 + 3u)];
        let _e608 = global.member[(_e381.member_6 + 4u)];
        let _e612 = global.member[(_e381.member_6 + 5u)];
        let _e616 = global.member[(_e381.member_6 + 6u)];
        switch bitcast<i32>(_e616) {
            case 0: {
                phi_1160_ = 0u;
                break;
            }
            case 1: {
                phi_1160_ = 1u;
                break;
            }
            case 2: {
                phi_1160_ = 2u;
                break;
            }
            default: {
                phi_1160_ = 0u;
                break;
            }
        }
        let _e619 = phi_1160_;
        let _e623 = global.member[(_e381.member_6 + 7u)];
        switch bitcast<i32>(_e623) {
            case 0: {
                phi_1169_ = 0u;
                break;
            }
            case 1: {
                phi_1169_ = 1u;
                break;
            }
            case 2: {
                phi_1169_ = 2u;
                break;
            }
            default: {
                phi_1169_ = 0u;
                break;
            }
        }
        let _e626 = phi_1169_;
        phi_1182_ = type_33(type_24(_e619, _e626), vec2<u32>(_e590, _e594), vec2<u32>(_e599, _e603), _e608, _e612);
    } else {
        phi_1182_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e630 = phi_1182_;
    switch bitcast<i32>(_e630.member.member) {
        case 1: {
            let _e668 = abs(_e582.x);
            let _e670 = (_e668 % 1f);
            if (_e668 >= 1f) {
                phi_4276_ = select(true, false, (_e670 == 0f));
            } else {
                phi_4276_ = true;
            }
            let _e674 = phi_4276_;
            let _e675 = select(1f, _e670, _e674);
            if (select(-1f, 1f, (_e582.x >= 0f)) > 0f) {
                phi_1202_ = _e675;
            } else {
                phi_1202_ = (1f - _e675);
            }
            let _e679 = phi_1202_;
            phi_1239_ = _e679;
            break;
        }
        case 2: {
            let _e642 = abs(_e582.x);
            let _e649 = ((select(select(u32(_e642), 0u, (_e642 < 0f)), 4294967295u, (_e642 > 4294967000f)) % 2u) == 0u);
            let _e651 = (_e642 % 1f);
            if (_e642 >= 1f) {
                phi_4259_ = select(true, false, (_e651 == 0f));
            } else {
                phi_4259_ = true;
            }
            let _e655 = phi_4259_;
            let _e656 = select(1f, _e651, _e655);
            if (select(-1f, 1f, (_e582.x >= 0f)) > 0f) {
                if _e649 {
                    phi_1231_ = _e656;
                } else {
                    phi_1231_ = (1f - _e656);
                }
                let _e663 = phi_1231_;
                phi_1237_ = _e663;
            } else {
                if _e649 {
                    phi_1236_ = (1f - _e656);
                } else {
                    phi_1236_ = _e656;
                }
                let _e660 = phi_1236_;
                phi_1237_ = _e660;
            }
            let _e665 = phi_1237_;
            phi_1239_ = _e665;
            break;
        }
        case 0: {
            if (_e582.x > 1f) {
                phi_4246_ = 0.9999999f;
            } else {
                phi_4246_ = select(_e582.x, 0.00000011920929f, (_e582.x < 0f));
            }
            let _e639 = phi_4246_;
            phi_1239_ = _e639;
            break;
        }
        default: {
            phi_1239_ = f32();
            break;
        }
    }
    let _e681 = phi_1239_;
    switch bitcast<i32>(_e630.member.member_1) {
        case 1: {
            let _e719 = abs(_e582.y);
            let _e721 = (_e719 % 1f);
            if (_e719 >= 1f) {
                phi_4324_ = select(true, false, (_e721 == 0f));
            } else {
                phi_4324_ = true;
            }
            let _e725 = phi_4324_;
            let _e726 = select(1f, _e721, _e725);
            if (select(-1f, 1f, (_e582.y >= 0f)) > 0f) {
                phi_1260_ = _e726;
            } else {
                phi_1260_ = (1f - _e726);
            }
            let _e730 = phi_1260_;
            phi_1297_ = _e730;
            break;
        }
        case 2: {
            let _e693 = abs(_e582.y);
            let _e700 = ((select(select(u32(_e693), 0u, (_e693 < 0f)), 4294967295u, (_e693 > 4294967000f)) % 2u) == 0u);
            let _e702 = (_e693 % 1f);
            if (_e693 >= 1f) {
                phi_4307_ = select(true, false, (_e702 == 0f));
            } else {
                phi_4307_ = true;
            }
            let _e706 = phi_4307_;
            let _e707 = select(1f, _e702, _e706);
            if (select(-1f, 1f, (_e582.y >= 0f)) > 0f) {
                if _e700 {
                    phi_1289_ = _e707;
                } else {
                    phi_1289_ = (1f - _e707);
                }
                let _e714 = phi_1289_;
                phi_1295_ = _e714;
            } else {
                if _e700 {
                    phi_1294_ = (1f - _e707);
                } else {
                    phi_1294_ = _e707;
                }
                let _e711 = phi_1294_;
                phi_1295_ = _e711;
            }
            let _e716 = phi_1295_;
            phi_1297_ = _e716;
            break;
        }
        case 0: {
            if (_e582.y > 1f) {
                phi_4294_ = 0.9999999f;
            } else {
                phi_4294_ = select(_e582.y, 0.00000011920929f, (_e582.y < 0f));
            }
            let _e690 = phi_4294_;
            phi_1297_ = _e690;
            break;
        }
        default: {
            phi_1297_ = f32();
            break;
        }
    }
    let _e732 = phi_1297_;
    let _e736 = (_e681 * f32(_e630.member_2.x));
    let _e745 = (_e732 * f32(_e630.member_2.y));
    let _e763 = vec3<f32>((f32((select(select(u32(_e736), 0u, (_e736 < 0f)), 4294967295u, (_e736 > 4294967000f)) + _e630.member_1.x)) / _e561), (f32((select(select(u32(_e745), 0u, (_e745 < 0f)), 4294967295u, (_e745 > 4294967000f)) + _e630.member_1.y)) / _e562), f32(_e630.member_3));
    let _e769 = textureSampleLevel(global_11, global_10, vec2<f32>(_e763.x, _e763.y), i32(_e763.z), 0f);
    let _e772 = select(_e769, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e381.member_6 == 4294967295u)));
    let _e776 = select(_e120, _e119, vec2((_e381.member_12 == 0u)));
    if _e387 {
        phi_4360_ = (_e381.member_7 <= (_e116 - 8u));
    } else {
        phi_4360_ = false;
    }
    let _e781 = phi_4360_;
    if _e781 {
        let _e784 = global.member[_e381.member_7];
        let _e788 = global.member[(_e381.member_7 + 1u)];
        let _e793 = global.member[(_e381.member_7 + 2u)];
        let _e797 = global.member[(_e381.member_7 + 3u)];
        let _e802 = global.member[(_e381.member_7 + 4u)];
        let _e806 = global.member[(_e381.member_7 + 5u)];
        let _e810 = global.member[(_e381.member_7 + 6u)];
        switch bitcast<i32>(_e810) {
            case 0: {
                phi_1380_ = 0u;
                break;
            }
            case 1: {
                phi_1380_ = 1u;
                break;
            }
            case 2: {
                phi_1380_ = 2u;
                break;
            }
            default: {
                phi_1380_ = 0u;
                break;
            }
        }
        let _e813 = phi_1380_;
        let _e817 = global.member[(_e381.member_7 + 7u)];
        switch bitcast<i32>(_e817) {
            case 0: {
                phi_1389_ = 0u;
                break;
            }
            case 1: {
                phi_1389_ = 1u;
                break;
            }
            case 2: {
                phi_1389_ = 2u;
                break;
            }
            default: {
                phi_1389_ = 0u;
                break;
            }
        }
        let _e820 = phi_1389_;
        phi_1402_ = type_33(type_24(_e813, _e820), vec2<u32>(_e784, _e788), vec2<u32>(_e793, _e797), _e802, _e806);
    } else {
        phi_1402_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e824 = phi_1402_;
    switch bitcast<i32>(_e824.member.member) {
        case 1: {
            let _e862 = abs(_e776.x);
            let _e864 = (_e862 % 1f);
            if (_e862 >= 1f) {
                phi_4411_ = select(true, false, (_e864 == 0f));
            } else {
                phi_4411_ = true;
            }
            let _e868 = phi_4411_;
            let _e869 = select(1f, _e864, _e868);
            if (select(-1f, 1f, (_e776.x >= 0f)) > 0f) {
                phi_1422_ = _e869;
            } else {
                phi_1422_ = (1f - _e869);
            }
            let _e873 = phi_1422_;
            phi_1459_ = _e873;
            break;
        }
        case 2: {
            let _e836 = abs(_e776.x);
            let _e843 = ((select(select(u32(_e836), 0u, (_e836 < 0f)), 4294967295u, (_e836 > 4294967000f)) % 2u) == 0u);
            let _e845 = (_e836 % 1f);
            if (_e836 >= 1f) {
                phi_4394_ = select(true, false, (_e845 == 0f));
            } else {
                phi_4394_ = true;
            }
            let _e849 = phi_4394_;
            let _e850 = select(1f, _e845, _e849);
            if (select(-1f, 1f, (_e776.x >= 0f)) > 0f) {
                if _e843 {
                    phi_1451_ = _e850;
                } else {
                    phi_1451_ = (1f - _e850);
                }
                let _e857 = phi_1451_;
                phi_1457_ = _e857;
            } else {
                if _e843 {
                    phi_1456_ = (1f - _e850);
                } else {
                    phi_1456_ = _e850;
                }
                let _e854 = phi_1456_;
                phi_1457_ = _e854;
            }
            let _e859 = phi_1457_;
            phi_1459_ = _e859;
            break;
        }
        case 0: {
            if (_e776.x > 1f) {
                phi_4381_ = 0.9999999f;
            } else {
                phi_4381_ = select(_e776.x, 0.00000011920929f, (_e776.x < 0f));
            }
            let _e833 = phi_4381_;
            phi_1459_ = _e833;
            break;
        }
        default: {
            phi_1459_ = f32();
            break;
        }
    }
    let _e875 = phi_1459_;
    switch bitcast<i32>(_e824.member.member_1) {
        case 1: {
            let _e913 = abs(_e776.y);
            let _e915 = (_e913 % 1f);
            if (_e913 >= 1f) {
                phi_4459_ = select(true, false, (_e915 == 0f));
            } else {
                phi_4459_ = true;
            }
            let _e919 = phi_4459_;
            let _e920 = select(1f, _e915, _e919);
            if (select(-1f, 1f, (_e776.y >= 0f)) > 0f) {
                phi_1480_ = _e920;
            } else {
                phi_1480_ = (1f - _e920);
            }
            let _e924 = phi_1480_;
            phi_1517_ = _e924;
            break;
        }
        case 2: {
            let _e887 = abs(_e776.y);
            let _e894 = ((select(select(u32(_e887), 0u, (_e887 < 0f)), 4294967295u, (_e887 > 4294967000f)) % 2u) == 0u);
            let _e896 = (_e887 % 1f);
            if (_e887 >= 1f) {
                phi_4442_ = select(true, false, (_e896 == 0f));
            } else {
                phi_4442_ = true;
            }
            let _e900 = phi_4442_;
            let _e901 = select(1f, _e896, _e900);
            if (select(-1f, 1f, (_e776.y >= 0f)) > 0f) {
                if _e894 {
                    phi_1509_ = _e901;
                } else {
                    phi_1509_ = (1f - _e901);
                }
                let _e908 = phi_1509_;
                phi_1515_ = _e908;
            } else {
                if _e894 {
                    phi_1514_ = (1f - _e901);
                } else {
                    phi_1514_ = _e901;
                }
                let _e905 = phi_1514_;
                phi_1515_ = _e905;
            }
            let _e910 = phi_1515_;
            phi_1517_ = _e910;
            break;
        }
        case 0: {
            if (_e776.y > 1f) {
                phi_4429_ = 0.9999999f;
            } else {
                phi_4429_ = select(_e776.y, 0.00000011920929f, (_e776.y < 0f));
            }
            let _e884 = phi_4429_;
            phi_1517_ = _e884;
            break;
        }
        default: {
            phi_1517_ = f32();
            break;
        }
    }
    let _e926 = phi_1517_;
    let _e930 = (_e875 * f32(_e824.member_2.x));
    let _e939 = (_e926 * f32(_e824.member_2.y));
    let _e957 = vec3<f32>((f32((select(select(u32(_e930), 0u, (_e930 < 0f)), 4294967295u, (_e930 > 4294967000f)) + _e824.member_1.x)) / _e561), (f32((select(select(u32(_e939), 0u, (_e939 < 0f)), 4294967295u, (_e939 > 4294967000f)) + _e824.member_1.y)) / _e562), f32(_e824.member_3));
    let _e963 = textureSampleLevel(global_11, global_10, vec2<f32>(_e957.x, _e957.y), i32(_e957.z), 0f);
    let _e964 = (_e381.member_7 == 4294967295u);
    let _e966 = select(_e963, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e964));
    let _e970 = select(_e120, _e119, vec2((_e381.member_13 == 0u)));
    if _e387 {
        phi_4495_ = (_e381.member_8 <= (_e116 - 8u));
    } else {
        phi_4495_ = false;
    }
    let _e975 = phi_4495_;
    if _e975 {
        let _e978 = global.member[_e381.member_8];
        let _e982 = global.member[(_e381.member_8 + 1u)];
        let _e987 = global.member[(_e381.member_8 + 2u)];
        let _e991 = global.member[(_e381.member_8 + 3u)];
        let _e996 = global.member[(_e381.member_8 + 4u)];
        let _e1000 = global.member[(_e381.member_8 + 5u)];
        let _e1004 = global.member[(_e381.member_8 + 6u)];
        switch bitcast<i32>(_e1004) {
            case 0: {
                phi_1600_ = 0u;
                break;
            }
            case 1: {
                phi_1600_ = 1u;
                break;
            }
            case 2: {
                phi_1600_ = 2u;
                break;
            }
            default: {
                phi_1600_ = 0u;
                break;
            }
        }
        let _e1007 = phi_1600_;
        let _e1011 = global.member[(_e381.member_8 + 7u)];
        switch bitcast<i32>(_e1011) {
            case 0: {
                phi_1609_ = 0u;
                break;
            }
            case 1: {
                phi_1609_ = 1u;
                break;
            }
            case 2: {
                phi_1609_ = 2u;
                break;
            }
            default: {
                phi_1609_ = 0u;
                break;
            }
        }
        let _e1014 = phi_1609_;
        phi_1622_ = type_33(type_24(_e1007, _e1014), vec2<u32>(_e978, _e982), vec2<u32>(_e987, _e991), _e996, _e1000);
    } else {
        phi_1622_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1018 = phi_1622_;
    switch bitcast<i32>(_e1018.member.member) {
        case 1: {
            let _e1056 = abs(_e970.x);
            let _e1058 = (_e1056 % 1f);
            if (_e1056 >= 1f) {
                phi_4546_ = select(true, false, (_e1058 == 0f));
            } else {
                phi_4546_ = true;
            }
            let _e1062 = phi_4546_;
            let _e1063 = select(1f, _e1058, _e1062);
            if (select(-1f, 1f, (_e970.x >= 0f)) > 0f) {
                phi_1642_ = _e1063;
            } else {
                phi_1642_ = (1f - _e1063);
            }
            let _e1067 = phi_1642_;
            phi_1679_ = _e1067;
            break;
        }
        case 2: {
            let _e1030 = abs(_e970.x);
            let _e1037 = ((select(select(u32(_e1030), 0u, (_e1030 < 0f)), 4294967295u, (_e1030 > 4294967000f)) % 2u) == 0u);
            let _e1039 = (_e1030 % 1f);
            if (_e1030 >= 1f) {
                phi_4529_ = select(true, false, (_e1039 == 0f));
            } else {
                phi_4529_ = true;
            }
            let _e1043 = phi_4529_;
            let _e1044 = select(1f, _e1039, _e1043);
            if (select(-1f, 1f, (_e970.x >= 0f)) > 0f) {
                if _e1037 {
                    phi_1671_ = _e1044;
                } else {
                    phi_1671_ = (1f - _e1044);
                }
                let _e1051 = phi_1671_;
                phi_1677_ = _e1051;
            } else {
                if _e1037 {
                    phi_1676_ = (1f - _e1044);
                } else {
                    phi_1676_ = _e1044;
                }
                let _e1048 = phi_1676_;
                phi_1677_ = _e1048;
            }
            let _e1053 = phi_1677_;
            phi_1679_ = _e1053;
            break;
        }
        case 0: {
            if (_e970.x > 1f) {
                phi_4516_ = 0.9999999f;
            } else {
                phi_4516_ = select(_e970.x, 0.00000011920929f, (_e970.x < 0f));
            }
            let _e1027 = phi_4516_;
            phi_1679_ = _e1027;
            break;
        }
        default: {
            phi_1679_ = f32();
            break;
        }
    }
    let _e1069 = phi_1679_;
    switch bitcast<i32>(_e1018.member.member_1) {
        case 1: {
            let _e1107 = abs(_e970.y);
            let _e1109 = (_e1107 % 1f);
            if (_e1107 >= 1f) {
                phi_4594_ = select(true, false, (_e1109 == 0f));
            } else {
                phi_4594_ = true;
            }
            let _e1113 = phi_4594_;
            let _e1114 = select(1f, _e1109, _e1113);
            if (select(-1f, 1f, (_e970.y >= 0f)) > 0f) {
                phi_1700_ = _e1114;
            } else {
                phi_1700_ = (1f - _e1114);
            }
            let _e1118 = phi_1700_;
            phi_1737_ = _e1118;
            break;
        }
        case 2: {
            let _e1081 = abs(_e970.y);
            let _e1088 = ((select(select(u32(_e1081), 0u, (_e1081 < 0f)), 4294967295u, (_e1081 > 4294967000f)) % 2u) == 0u);
            let _e1090 = (_e1081 % 1f);
            if (_e1081 >= 1f) {
                phi_4577_ = select(true, false, (_e1090 == 0f));
            } else {
                phi_4577_ = true;
            }
            let _e1094 = phi_4577_;
            let _e1095 = select(1f, _e1090, _e1094);
            if (select(-1f, 1f, (_e970.y >= 0f)) > 0f) {
                if _e1088 {
                    phi_1729_ = _e1095;
                } else {
                    phi_1729_ = (1f - _e1095);
                }
                let _e1102 = phi_1729_;
                phi_1735_ = _e1102;
            } else {
                if _e1088 {
                    phi_1734_ = (1f - _e1095);
                } else {
                    phi_1734_ = _e1095;
                }
                let _e1099 = phi_1734_;
                phi_1735_ = _e1099;
            }
            let _e1104 = phi_1735_;
            phi_1737_ = _e1104;
            break;
        }
        case 0: {
            if (_e970.y > 1f) {
                phi_4564_ = 0.9999999f;
            } else {
                phi_4564_ = select(_e970.y, 0.00000011920929f, (_e970.y < 0f));
            }
            let _e1078 = phi_4564_;
            phi_1737_ = _e1078;
            break;
        }
        default: {
            phi_1737_ = f32();
            break;
        }
    }
    let _e1120 = phi_1737_;
    let _e1124 = (_e1069 * f32(_e1018.member_2.x));
    let _e1133 = (_e1120 * f32(_e1018.member_2.y));
    let _e1151 = vec3<f32>((f32((select(select(u32(_e1124), 0u, (_e1124 < 0f)), 4294967295u, (_e1124 > 4294967000f)) + _e1018.member_1.x)) / _e561), (f32((select(select(u32(_e1133), 0u, (_e1133 < 0f)), 4294967295u, (_e1133 > 4294967000f)) + _e1018.member_1.y)) / _e562), f32(_e1018.member_3));
    let _e1157 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1151.x, _e1151.y), i32(_e1151.z), 0f);
    let _e1164 = select(_e120, _e119, vec2((_e381.member_14 == 0u)));
    if _e387 {
        phi_4630_ = (_e381.member_9 <= (_e116 - 8u));
    } else {
        phi_4630_ = false;
    }
    let _e1169 = phi_4630_;
    if _e1169 {
        let _e1172 = global.member[_e381.member_9];
        let _e1176 = global.member[(_e381.member_9 + 1u)];
        let _e1181 = global.member[(_e381.member_9 + 2u)];
        let _e1185 = global.member[(_e381.member_9 + 3u)];
        let _e1190 = global.member[(_e381.member_9 + 4u)];
        let _e1194 = global.member[(_e381.member_9 + 5u)];
        let _e1198 = global.member[(_e381.member_9 + 6u)];
        switch bitcast<i32>(_e1198) {
            case 0: {
                phi_1820_ = 0u;
                break;
            }
            case 1: {
                phi_1820_ = 1u;
                break;
            }
            case 2: {
                phi_1820_ = 2u;
                break;
            }
            default: {
                phi_1820_ = 0u;
                break;
            }
        }
        let _e1201 = phi_1820_;
        let _e1205 = global.member[(_e381.member_9 + 7u)];
        switch bitcast<i32>(_e1205) {
            case 0: {
                phi_1829_ = 0u;
                break;
            }
            case 1: {
                phi_1829_ = 1u;
                break;
            }
            case 2: {
                phi_1829_ = 2u;
                break;
            }
            default: {
                phi_1829_ = 0u;
                break;
            }
        }
        let _e1208 = phi_1829_;
        phi_1842_ = type_33(type_24(_e1201, _e1208), vec2<u32>(_e1172, _e1176), vec2<u32>(_e1181, _e1185), _e1190, _e1194);
    } else {
        phi_1842_ = type_33(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1212 = phi_1842_;
    switch bitcast<i32>(_e1212.member.member) {
        case 1: {
            let _e1250 = abs(_e1164.x);
            let _e1252 = (_e1250 % 1f);
            if (_e1250 >= 1f) {
                phi_4681_ = select(true, false, (_e1252 == 0f));
            } else {
                phi_4681_ = true;
            }
            let _e1256 = phi_4681_;
            let _e1257 = select(1f, _e1252, _e1256);
            if (select(-1f, 1f, (_e1164.x >= 0f)) > 0f) {
                phi_1862_ = _e1257;
            } else {
                phi_1862_ = (1f - _e1257);
            }
            let _e1261 = phi_1862_;
            phi_1899_ = _e1261;
            break;
        }
        case 2: {
            let _e1224 = abs(_e1164.x);
            let _e1231 = ((select(select(u32(_e1224), 0u, (_e1224 < 0f)), 4294967295u, (_e1224 > 4294967000f)) % 2u) == 0u);
            let _e1233 = (_e1224 % 1f);
            if (_e1224 >= 1f) {
                phi_4664_ = select(true, false, (_e1233 == 0f));
            } else {
                phi_4664_ = true;
            }
            let _e1237 = phi_4664_;
            let _e1238 = select(1f, _e1233, _e1237);
            if (select(-1f, 1f, (_e1164.x >= 0f)) > 0f) {
                if _e1231 {
                    phi_1891_ = _e1238;
                } else {
                    phi_1891_ = (1f - _e1238);
                }
                let _e1245 = phi_1891_;
                phi_1897_ = _e1245;
            } else {
                if _e1231 {
                    phi_1896_ = (1f - _e1238);
                } else {
                    phi_1896_ = _e1238;
                }
                let _e1242 = phi_1896_;
                phi_1897_ = _e1242;
            }
            let _e1247 = phi_1897_;
            phi_1899_ = _e1247;
            break;
        }
        case 0: {
            if (_e1164.x > 1f) {
                phi_4651_ = 0.9999999f;
            } else {
                phi_4651_ = select(_e1164.x, 0.00000011920929f, (_e1164.x < 0f));
            }
            let _e1221 = phi_4651_;
            phi_1899_ = _e1221;
            break;
        }
        default: {
            phi_1899_ = f32();
            break;
        }
    }
    let _e1263 = phi_1899_;
    switch bitcast<i32>(_e1212.member.member_1) {
        case 1: {
            let _e1301 = abs(_e1164.y);
            let _e1303 = (_e1301 % 1f);
            if (_e1301 >= 1f) {
                phi_4729_ = select(true, false, (_e1303 == 0f));
            } else {
                phi_4729_ = true;
            }
            let _e1307 = phi_4729_;
            let _e1308 = select(1f, _e1303, _e1307);
            if (select(-1f, 1f, (_e1164.y >= 0f)) > 0f) {
                phi_1920_ = _e1308;
            } else {
                phi_1920_ = (1f - _e1308);
            }
            let _e1312 = phi_1920_;
            phi_1957_ = _e1312;
            break;
        }
        case 2: {
            let _e1275 = abs(_e1164.y);
            let _e1282 = ((select(select(u32(_e1275), 0u, (_e1275 < 0f)), 4294967295u, (_e1275 > 4294967000f)) % 2u) == 0u);
            let _e1284 = (_e1275 % 1f);
            if (_e1275 >= 1f) {
                phi_4712_ = select(true, false, (_e1284 == 0f));
            } else {
                phi_4712_ = true;
            }
            let _e1288 = phi_4712_;
            let _e1289 = select(1f, _e1284, _e1288);
            if (select(-1f, 1f, (_e1164.y >= 0f)) > 0f) {
                if _e1282 {
                    phi_1949_ = _e1289;
                } else {
                    phi_1949_ = (1f - _e1289);
                }
                let _e1296 = phi_1949_;
                phi_1955_ = _e1296;
            } else {
                if _e1282 {
                    phi_1954_ = (1f - _e1289);
                } else {
                    phi_1954_ = _e1289;
                }
                let _e1293 = phi_1954_;
                phi_1955_ = _e1293;
            }
            let _e1298 = phi_1955_;
            phi_1957_ = _e1298;
            break;
        }
        case 0: {
            if (_e1164.y > 1f) {
                phi_4699_ = 0.9999999f;
            } else {
                phi_4699_ = select(_e1164.y, 0.00000011920929f, (_e1164.y < 0f));
            }
            let _e1272 = phi_4699_;
            phi_1957_ = _e1272;
            break;
        }
        default: {
            phi_1957_ = f32();
            break;
        }
    }
    let _e1314 = phi_1957_;
    let _e1318 = (_e1263 * f32(_e1212.member_2.x));
    let _e1327 = (_e1314 * f32(_e1212.member_2.y));
    let _e1345 = vec3<f32>((f32((select(select(u32(_e1318), 0u, (_e1318 < 0f)), 4294967295u, (_e1318 > 4294967000f)) + _e1212.member_1.x)) / _e561), (f32((select(select(u32(_e1327), 0u, (_e1327 < 0f)), 4294967295u, (_e1327 > 4294967000f)) + _e1212.member_1.y)) / _e562), f32(_e1212.member_3));
    let _e1351 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1345.x, _e1345.y), i32(_e1345.z), 0f);
    let _e1354 = select(_e1351, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e381.member_9 == 4294967295u)));
    if _e964 {
        phi_2051_ = vec3<f32>(0f, 0f, 0f);
        phi_2052_ = _e121;
    } else {
        let _e1358 = fma(_e966.x, 2f, -1f);
        let _e1359 = fma(_e966.y, 2f, -1f);
        let _e1360 = fma(_e966.z, 2f, -1f);
        let _e1365 = sqrt(fma(_e1360, _e1360, fma(_e1358, _e1358, (_e1359 * _e1359))));
        if (_e1365 == 0f) {
            phi_4787_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4787_ = (vec3<f32>(_e1358, _e1359, _e1360) * (1f / _e1365));
        }
        let _e1370 = phi_4787_;
        let _e1377 = sqrt(fma(_e122.z, _e122.z, fma(_e122.x, _e122.x, (_e122.y * _e122.y))));
        if (_e1377 == 0f) {
            phi_4822_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4822_ = (_e122 * (1f / _e1377));
        }
        let _e1382 = phi_4822_;
        let _e1389 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
        if (_e1389 == 0f) {
            phi_4857_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4857_ = (_e123 * (1f / _e1389));
        }
        let _e1394 = phi_4857_;
        let _e1401 = sqrt(fma(_e121.z, _e121.z, fma(_e121.x, _e121.x, (_e121.y * _e121.y))));
        if (_e1401 == 0f) {
            phi_4892_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4892_ = (_e121 * (1f / _e1401));
        }
        let _e1406 = phi_4892_;
        let _e1425 = fma(_e1406.x, _e1370.z, fma(_e1382.x, _e1370.x, (_e1394.x * _e1370.y)));
        let _e1426 = fma(_e1406.y, _e1370.z, fma(_e1382.y, _e1370.x, (_e1394.y * _e1370.y)));
        let _e1427 = fma(_e1406.z, _e1370.z, fma(_e1382.z, _e1370.x, (_e1394.z * _e1370.y)));
        let _e1432 = sqrt(fma(_e1427, _e1427, fma(_e1425, _e1425, (_e1426 * _e1426))));
        if (_e1432 == 0f) {
            phi_4927_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4927_ = (vec3<f32>(_e1425, _e1426, _e1427) * (1f / _e1432));
        }
        let _e1437 = phi_4927_;
        phi_2051_ = _e1370;
        phi_2052_ = _e1437;
    }
    let _e1439 = phi_2051_;
    let _e1441 = phi_2052_;
    let _e1445 = (_e578.x * _e381.member_2.x);
    let _e1448 = (_e578.y * _e381.member_2.y);
    let _e1451 = (_e578.z * _e381.member_2.z);
    let _e1456 = (_e1445 * _e118.x);
    let _e1458 = (_e1448 * _e118.y);
    let _e1460 = (_e1451 * _e118.z);
    let _e1465 = (_e772.y * _e381.member_4);
    let _e1468 = (_e772.z * _e381.member_3);
    let _e1472 = fma(_e381.member_16, (select(_e1157, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e381.member_8 == 4294967295u))).x - 1f), 1f);
    let _e1478 = (_e1354.x * _e381.member.x);
    let _e1480 = (_e1354.y * _e381.member.y);
    let _e1482 = (_e1354.z * _e381.member.z);
    let _e1487 = textureSampleLevel(global_12, global_13, _e1441, 0f);
    if (_e116 >= 86u) {
        phi_4959_ = (_e214 <= (_e116 - 86u));
    } else {
        phi_4959_ = false;
    }
    let _e1495 = phi_4959_;
    if _e1495 {
        let _e1498 = global.member[_e214];
        let _e1503 = global.member[(_e214 + 1u)];
        let _e1508 = global.member[(_e214 + 2u)];
        let _e1513 = global.member[(_e214 + 3u)];
        let _e1519 = global.member[(_e214 + 4u)];
        let _e1524 = global.member[(_e214 + 5u)];
        let _e1529 = global.member[(_e214 + 6u)];
        let _e1534 = global.member[(_e214 + 7u)];
        let _e1540 = global.member[(_e214 + 8u)];
        let _e1545 = global.member[(_e214 + 9u)];
        let _e1550 = global.member[(_e214 + 10u)];
        let _e1555 = global.member[(_e214 + 11u)];
        let _e1561 = global.member[(_e214 + 12u)];
        let _e1566 = global.member[(_e214 + 13u)];
        let _e1571 = global.member[(_e214 + 14u)];
        let _e1576 = global.member[(_e214 + 15u)];
        let _e1583 = global.member[(_e214 + 16u)];
        let _e1588 = global.member[(_e214 + 17u)];
        let _e1593 = global.member[(_e214 + 18u)];
        let _e1598 = global.member[(_e214 + 19u)];
        let _e1604 = global.member[(_e214 + 20u)];
        let _e1609 = global.member[(_e214 + 21u)];
        let _e1614 = global.member[(_e214 + 22u)];
        let _e1619 = global.member[(_e214 + 23u)];
        let _e1625 = global.member[(_e214 + 24u)];
        let _e1630 = global.member[(_e214 + 25u)];
        let _e1635 = global.member[(_e214 + 26u)];
        let _e1640 = global.member[(_e214 + 27u)];
        let _e1646 = global.member[(_e214 + 28u)];
        let _e1651 = global.member[(_e214 + 29u)];
        let _e1656 = global.member[(_e214 + 30u)];
        let _e1661 = global.member[(_e214 + 31u)];
        let _e1668 = global.member[(_e214 + 32u)];
        let _e1673 = global.member[(_e214 + 33u)];
        let _e1678 = global.member[(_e214 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_2259_ = type_24(0u, 6u);
        loop {
            let _e1683 = phi_2259_;
            if (_e1683.member < _e1683.member_1) {
                phi_2260_ = type_24((_e1683.member + 1u), _e1683.member_1);
                phi_2283_ = type_24(1u, _e1683.member);
            } else {
                phi_2260_ = _e1683;
                phi_2283_ = type_24(0u, type_24().member_1);
            }
            let _e1696 = phi_2260_;
            let _e1698 = phi_2283_;
            switch bitcast<i32>(_e1698.member) {
                case 0: {
                    phi_2310_ = false;
                    break;
                }
                case 1: {
                    let _e1703 = ((_e214 + 35u) + (_e1698.member_1 * 4u));
                    let _e1706 = global.member[_e1703];
                    let _e1711 = global.member[(_e1703 + 1u)];
                    let _e1716 = global.member[(_e1703 + 2u)];
                    let _e1721 = global.member[(_e1703 + 3u)];
                    local_1[_e1698.member_1] = vec4<f32>(bitcast<f32>(_e1706), bitcast<f32>(_e1711), bitcast<f32>(_e1716), bitcast<f32>(_e1721));
                    phi_2310_ = true;
                    break;
                }
                default: {
                    phi_2310_ = bool();
                    break;
                }
            }
            let _e1726 = phi_2310_;
            continue;
            continuing {
                phi_2259_ = _e1696;
                break if !(_e1726);
            }
        }
        let _e1728 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2316_ = type_24(0u, 8u);
        loop {
            let _e1731 = phi_2316_;
            if (_e1731.member < _e1731.member_1) {
                phi_2317_ = type_24((_e1731.member + 1u), _e1731.member_1);
                phi_2340_ = type_24(1u, _e1731.member);
            } else {
                phi_2317_ = _e1731;
                phi_2340_ = type_24(0u, type_24().member_1);
            }
            let _e1744 = phi_2317_;
            let _e1746 = phi_2340_;
            switch bitcast<i32>(_e1746.member) {
                case 0: {
                    phi_2363_ = false;
                    break;
                }
                case 1: {
                    let _e1751 = ((_e214 + 59u) + (_e1746.member_1 * 3u));
                    let _e1754 = global.member[_e1751];
                    let _e1759 = global.member[(_e1751 + 1u)];
                    let _e1764 = global.member[(_e1751 + 2u)];
                    local[_e1746.member_1] = vec3<f32>(bitcast<f32>(_e1754), bitcast<f32>(_e1759), bitcast<f32>(_e1764));
                    phi_2363_ = true;
                    break;
                }
                default: {
                    phi_2363_ = bool();
                    break;
                }
            }
            let _e1769 = phi_2363_;
            continue;
            continuing {
                phi_2316_ = _e1744;
                break if !(_e1769);
            }
        }
        let _e1771 = local;
        let _e1775 = global.member[(_e214 + 83u)];
        let _e1780 = global.member[(_e214 + 84u)];
        let _e1785 = global.member[(_e214 + 85u)];
        phi_2384_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1498), bitcast<f32>(_e1503), bitcast<f32>(_e1508), bitcast<f32>(_e1513)), vec4<f32>(bitcast<f32>(_e1519), bitcast<f32>(_e1524), bitcast<f32>(_e1529), bitcast<f32>(_e1534)), vec4<f32>(bitcast<f32>(_e1540), bitcast<f32>(_e1545), bitcast<f32>(_e1550), bitcast<f32>(_e1555)), vec4<f32>(bitcast<f32>(_e1561), bitcast<f32>(_e1566), bitcast<f32>(_e1571), bitcast<f32>(_e1576))), type_20(vec4<f32>(bitcast<f32>(_e1583), bitcast<f32>(_e1588), bitcast<f32>(_e1593), bitcast<f32>(_e1598)), vec4<f32>(bitcast<f32>(_e1604), bitcast<f32>(_e1609), bitcast<f32>(_e1614), bitcast<f32>(_e1619)), vec4<f32>(bitcast<f32>(_e1625), bitcast<f32>(_e1630), bitcast<f32>(_e1635), bitcast<f32>(_e1640)), vec4<f32>(bitcast<f32>(_e1646), bitcast<f32>(_e1651), bitcast<f32>(_e1656), bitcast<f32>(_e1661))), vec3<f32>(bitcast<f32>(_e1668), bitcast<f32>(_e1673), bitcast<f32>(_e1678)), type_21(_e1771, _e1728, vec3<f32>(bitcast<f32>(_e1775), bitcast<f32>(_e1780), bitcast<f32>(_e1785))));
    } else {
        phi_2384_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e1791 = phi_2384_;
    let _e1793 = (_e1791.member_2 - _e124);
    let _e1800 = sqrt(fma(_e1793.z, _e1793.z, fma(_e1793.x, _e1793.x, (_e1793.y * _e1793.y))));
    let _e1801 = (_e1800 == 0f);
    if _e1801 {
        phi_5031_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5031_ = (_e1793 * (1f / _e1800));
    }
    let _e1805 = phi_5031_;
    let _e1806 = -(_e1805);
    let _e1813 = sqrt(fma(_e1441.z, _e1441.z, fma(_e1441.x, _e1441.x, (_e1441.y * _e1441.y))));
    let _e1814 = (_e1813 == 0f);
    if _e1814 {
        phi_5090_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5090_ = (_e1441 * (1f / _e1813));
    }
    let _e1818 = phi_5090_;
    let _e1828 = (2f * fma(_e1818.z, _e1806.z, fma(_e1818.x, _e1806.x, (_e1818.y * _e1806.y))));
    let _e1835 = textureSampleLevel(global_14, global_15, (_e1806 - vec3<f32>((_e1828 * _e1818.x), (_e1828 * _e1818.y), (_e1828 * _e1818.z))), (_e1465 * 4f));
    if _e1801 {
        phi_5164_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_5164_ = (_e1793 * (1f / _e1800));
    }
    let _e1842 = phi_5164_;
    let _e1851 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1441.z, _e1842.z, fma(_e1441.x, _e1842.x, (_e1441.y * _e1842.y))), 0f), _e1465), 0f);
    switch bitcast<i32>(_e236) {
        case 0: {
            if _e381.member_15 {
                if _e1814 {
                    phi_5557_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5557_ = (_e1441 * (1f / _e1813));
                }
                let _e2020 = phi_5557_;
                if _e1801 {
                    phi_5592_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5592_ = (_e1793 * (1f / _e1800));
                }
                let _e2024 = phi_5592_;
                phi_2424_ = type_24(0u, _e249);
                phi_2427_ = vec3<f32>(0f, 0f, 0f);
                loop {
                    let _e2027 = phi_2424_;
                    let _e2029 = phi_2427_;
                    local_2 = _e2029;
                    local_3 = _e2029;
                    local_4 = _e2029;
                    if (_e2027.member < _e2027.member_1) {
                        phi_2425_ = type_24((_e2027.member + 1u), _e2027.member_1);
                        phi_2450_ = type_24(1u, _e2027.member);
                    } else {
                        phi_2425_ = _e2027;
                        phi_2450_ = type_24(0u, type_24().member_1);
                    }
                    let _e2042 = phi_2425_;
                    let _e2044 = phi_2450_;
                    switch bitcast<i32>(_e2044.member) {
                        case 0: {
                            phi_2428_ = vec3<f32>();
                            phi_3321_ = false;
                            break;
                        }
                        case 1: {
                            if (_e2044.member_1 >= _e249) {
                                phi_5609_ = 4294967295u;
                            } else {
                                phi_5609_ = (_e245 + _e2044.member_1);
                            }
                            let _e2051 = phi_5609_;
                            if (_e116 >= 1u) {
                                phi_5628_ = (_e2051 <= (_e116 - 1u));
                            } else {
                                phi_5628_ = false;
                            }
                            let _e2056 = phi_5628_;
                            if _e2056 {
                                let _e2059 = global.member[_e2051];
                                phi_2467_ = _e2059;
                            } else {
                                phi_2467_ = 4294967295u;
                            }
                            let _e2061 = phi_2467_;
                            let _e2062 = (_e2061 == 4294967295u);
                            if _e2062 {
                                phi_3319_ = vec3<f32>();
                            } else {
                                if (_e116 >= 3u) {
                                    phi_5660_ = (_e2061 <= (_e116 - 3u));
                                } else {
                                    phi_5660_ = false;
                                }
                                let _e2067 = phi_5660_;
                                if _e2067 {
                                    let _e2070 = global.member[_e2061];
                                    switch bitcast<i32>(_e2070) {
                                        case 0: {
                                            phi_2484_ = 0u;
                                            break;
                                        }
                                        case 1: {
                                            phi_2484_ = 1u;
                                            break;
                                        }
                                        case 2: {
                                            phi_2484_ = 2u;
                                            break;
                                        }
                                        default: {
                                            phi_2484_ = 0u;
                                            break;
                                        }
                                    }
                                    let _e2073 = phi_2484_;
                                    let _e2077 = global.member[(_e2061 + 1u)];
                                    let _e2081 = global.member[(_e2061 + 2u)];
                                    phi_2494_ = type_34(_e2073, _e2077, _e2081);
                                } else {
                                    phi_2494_ = type_34(0u, 4294967295u, 4294967295u);
                                }
                                let _e2084 = phi_2494_;
                                if (_e116 >= 10u) {
                                    phi_5690_ = (_e2084.member_2 <= (_e116 - 10u));
                                } else {
                                    phi_5690_ = false;
                                }
                                let _e2090 = phi_5690_;
                                if _e2090 {
                                    let _e2093 = global.member[_e2084.member_2];
                                    let _e2098 = global.member[(_e2084.member_2 + 1u)];
                                    let _e2103 = global.member[(_e2084.member_2 + 2u)];
                                    let _e2109 = global.member[(_e2084.member_2 + 3u)];
                                    let _e2114 = global.member[(_e2084.member_2 + 4u)];
                                    let _e2119 = global.member[(_e2084.member_2 + 5u)];
                                    let _e2124 = global.member[(_e2084.member_2 + 6u)];
                                    let _e2130 = global.member[(_e2084.member_2 + 7u)];
                                    let _e2135 = global.member[(_e2084.member_2 + 8u)];
                                    let _e2140 = global.member[(_e2084.member_2 + 9u)];
                                    phi_2544_ = type_29(vec3<f32>(bitcast<f32>(_e2093), bitcast<f32>(_e2098), bitcast<f32>(_e2103)), vec4<f32>(bitcast<f32>(_e2109), bitcast<f32>(_e2114), bitcast<f32>(_e2119), bitcast<f32>(_e2124)), vec3<f32>(bitcast<f32>(_e2130), bitcast<f32>(_e2135), bitcast<f32>(_e2140)));
                                } else {
                                    phi_2544_ = type_29(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e2145 = phi_2544_;
                                let _e2153 = (_e2145.member_1.x + _e2145.member_1.x);
                                let _e2154 = (_e2145.member_1.y + _e2145.member_1.y);
                                let _e2155 = (_e2145.member_1.z + _e2145.member_1.z);
                                let _e2157 = (_e2145.member_1.z * _e2155);
                                let _e2158 = (_e2145.member_1.w * _e2153);
                                let _e2159 = (_e2145.member_1.w * _e2154);
                                let _e2160 = (_e2145.member_1.w * _e2155);
                                let _e2180 = (vec4<f32>((1f - fma(_e2145.member_1.y, _e2154, _e2157)), fma(_e2145.member_1.x, _e2154, _e2160), fma(_e2145.member_1.x, _e2155, -(_e2159)), 0f) * _e2145.member_2.x);
                                let _e2182 = (vec4<f32>(fma(_e2145.member_1.x, _e2154, -(_e2160)), (1f - fma(_e2145.member_1.x, _e2153, _e2157)), fma(_e2145.member_1.y, _e2155, _e2158), 0f) * _e2145.member_2.y);
                                let _e2184 = (vec4<f32>(fma(_e2145.member_1.x, _e2155, _e2159), fma(_e2145.member_1.y, _e2155, -(_e2158)), (1f - fma(_e2145.member_1.x, _e2153, (_e2145.member_1.y * _e2154))), 0f) * _e2145.member_2.z);
                                switch bitcast<i32>(_e2084.member) {
                                    case 0: {
                                        if _e387 {
                                            phi_6156_ = (_e2084.member_1 <= (_e116 - 8u));
                                        } else {
                                            phi_6156_ = false;
                                        }
                                        let _e2680 = phi_6156_;
                                        if _e2680 {
                                            let _e2683 = global.member[_e2084.member_1];
                                            let _e2688 = global.member[(_e2084.member_1 + 1u)];
                                            let _e2693 = global.member[(_e2084.member_1 + 2u)];
                                            let _e2699 = global.member[(_e2084.member_1 + 3u)];
                                            let _e2704 = global.member[(_e2084.member_1 + 4u)];
                                            let _e2709 = global.member[(_e2084.member_1 + 5u)];
                                            let _e2714 = global.member[(_e2084.member_1 + 6u)];
                                            let _e2720 = global.member[(_e2084.member_1 + 7u)];
                                            phi_2592_ = type_35(vec3<f32>(bitcast<f32>(_e2683), bitcast<f32>(_e2688), bitcast<f32>(_e2693)), vec4<f32>(bitcast<f32>(_e2699), bitcast<f32>(_e2704), bitcast<f32>(_e2709), bitcast<f32>(_e2714)), bitcast<f32>(_e2720));
                                        } else {
                                            phi_2592_ = type_35(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2724 = phi_2592_;
                                        let _e2746 = fma(_e2184.x, _e2724.member.z, fma(_e2182.x, _e2724.member.y, (_e2180.x * _e2724.member.x)));
                                        let _e2747 = fma(_e2184.y, _e2724.member.z, fma(_e2182.y, _e2724.member.y, (_e2180.y * _e2724.member.x)));
                                        let _e2748 = fma(_e2184.z, _e2724.member.z, fma(_e2182.z, _e2724.member.y, (_e2180.z * _e2724.member.x)));
                                        let _e2753 = sqrt(fma(_e2748, _e2748, fma(_e2746, _e2746, (_e2747 * _e2747))));
                                        if (_e2753 == 0f) {
                                            phi_6203_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6203_ = (vec3<f32>(_e2746, _e2747, _e2748) * (1f / _e2753));
                                        }
                                        let _e2758 = phi_6203_;
                                        let _e2760 = -(_e2758.x);
                                        let _e2762 = -(_e2758.y);
                                        let _e2764 = -(_e2758.z);
                                        let _e2765 = -(_e2758);
                                        let _e2767 = fma(-(_e772.z), _e381.member_3, 1f);
                                        let _e2771 = fma(0.4f, _e2767, (_e1456 * _e1468));
                                        let _e2772 = fma(0.4f, _e2767, (_e1458 * _e1468));
                                        let _e2773 = fma(0.4f, _e2767, (_e1460 * _e1468));
                                        let _e2781 = (_e2024 + vec3<f32>(_e2760, _e2762, _e2764));
                                        let _e2788 = sqrt(fma(_e2781.z, _e2781.z, fma(_e2781.x, _e2781.x, (_e2781.y * _e2781.y))));
                                        if (_e2788 == 0f) {
                                            phi_6238_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6238_ = (_e2781 * (1f / _e2788));
                                        }
                                        let _e2793 = phi_6238_;
                                        let _e2794 = (_e1465 * _e1465);
                                        let _e2805 = max(fma(_e2020.z, _e2793.z, fma(_e2020.x, _e2793.x, (_e2020.y * _e2793.y))), 0f);
                                        let _e2818 = max(fma(_e2020.z, _e2024.z, fma(_e2020.x, _e2024.x, (_e2020.y * _e2024.y))), 0f);
                                        let _e2825 = max(fma(_e2020.z, _e2765.z, fma(_e2020.x, _e2765.x, (_e2020.y * _e2765.y))), 0f);
                                        let _e2826 = fma(_e772.y, _e381.member_4, 1f);
                                        let _e2827 = (_e2826 * _e2826);
                                        let _e2828 = (_e2827 * 0.125f);
                                        let _e2830 = fma(-(_e2827), 0.125f, 1f);
                                        let _e2843 = (1f - max(fma(_e2793.z, _e2024.z, fma(_e2793.x, _e2024.x, (_e2793.y * _e2024.y))), 0f));
                                        let _e2845 = select(_e2843, 0f, (_e2843 < 0f));
                                        let _e2848 = pow(select(_e2845, 1f, (_e2845 > 1f)), 5f);
                                        let _e2849 = fma((1f - _e2771), _e2848, _e2771);
                                        let _e2850 = fma((1f - _e2772), _e2848, _e2772);
                                        let _e2851 = fma((1f - _e2773), _e2848, _e2773);
                                        let _e2858 = (((_e2794 * _e2794) / (pow(fma((_e2805 * _e2805), fma(_e2794, _e2794, -1f), 1f), 2f) * 3.1415927f)) * ((_e2818 / fma(_e2818, _e2830, _e2828)) * (_e2825 / fma(_e2825, _e2830, _e2828))));
                                        let _e2865 = max(fma(_e2020.z, _e2764, fma(_e2020.x, _e2760, (_e2020.y * _e2762))), 0f);
                                        let _e2867 = fma((4f * _e2818), _e2865, 0.0001f);
                                        phi_3309_ = vec3<f32>(fma((fma((((1f - _e2849) * _e2767) * _e1456), 0.31830987f, ((_e2858 * _e2849) / _e2867)) * (_e2724.member_1.x * _e2724.member_2)), _e2865, _e2029.x), fma((fma((((1f - _e2850) * _e2767) * _e1458), 0.31830987f, ((_e2858 * _e2850) / _e2867)) * (_e2724.member_1.y * _e2724.member_2)), _e2865, _e2029.y), fma((fma((((1f - _e2851) * _e2767) * _e1460), 0.31830987f, ((_e2858 * _e2851) / _e2867)) * (_e2724.member_1.z * _e2724.member_2)), _e2865, _e2029.z));
                                        phi_3310_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if _e387 {
                                            phi_5982_ = (_e2084.member_1 <= (_e116 - 8u));
                                        } else {
                                            phi_5982_ = false;
                                        }
                                        let _e2469 = phi_5982_;
                                        if _e2469 {
                                            let _e2472 = global.member[_e2084.member_1];
                                            let _e2477 = global.member[(_e2084.member_1 + 1u)];
                                            let _e2482 = global.member[(_e2084.member_1 + 2u)];
                                            let _e2488 = global.member[(_e2084.member_1 + 3u)];
                                            let _e2493 = global.member[(_e2084.member_1 + 4u)];
                                            let _e2498 = global.member[(_e2084.member_1 + 5u)];
                                            let _e2503 = global.member[(_e2084.member_1 + 6u)];
                                            let _e2509 = global.member[(_e2084.member_1 + 7u)];
                                            phi_2799_ = type_35(vec3<f32>(bitcast<f32>(_e2472), bitcast<f32>(_e2477), bitcast<f32>(_e2482)), vec4<f32>(bitcast<f32>(_e2488), bitcast<f32>(_e2493), bitcast<f32>(_e2498), bitcast<f32>(_e2503)), bitcast<f32>(_e2509));
                                        } else {
                                            phi_2799_ = type_35(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2513 = phi_2799_;
                                        let _e2542 = (vec3<f32>((_e2145.member.x + fma(_e2184.x, _e2513.member.z, fma(_e2182.x, _e2513.member.y, (_e2180.x * _e2513.member.x)))), (_e2145.member.y + fma(_e2184.y, _e2513.member.z, fma(_e2182.y, _e2513.member.y, (_e2180.y * _e2513.member.x)))), (_e2145.member.z + fma(_e2184.z, _e2513.member.z, fma(_e2182.z, _e2513.member.y, (_e2180.z * _e2513.member.x))))) - _e124);
                                        let _e2549 = sqrt(fma(_e2542.z, _e2542.z, fma(_e2542.x, _e2542.x, (_e2542.y * _e2542.y))));
                                        let _e2550 = (_e2549 == 0f);
                                        if _e2550 {
                                            phi_2989_ = vec3<f32>();
                                        } else {
                                            if _e2550 {
                                                phi_6029_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6029_ = (_e2542 * (1f / _e2549));
                                            }
                                            let _e2554 = phi_6029_;
                                            let _e2556 = (_e2513.member_2 / (_e2549 * _e2549));
                                            let _e2558 = fma(-(_e772.z), _e381.member_3, 1f);
                                            let _e2562 = fma(0.4f, _e2558, (_e1456 * _e1468));
                                            let _e2563 = fma(0.4f, _e2558, (_e1458 * _e1468));
                                            let _e2564 = fma(0.4f, _e2558, (_e1460 * _e1468));
                                            let _e2571 = (_e2024 + _e2554);
                                            let _e2578 = sqrt(fma(_e2571.z, _e2571.z, fma(_e2571.x, _e2571.x, (_e2571.y * _e2571.y))));
                                            if (_e2578 == 0f) {
                                                phi_6064_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6064_ = (_e2571 * (1f / _e2578));
                                            }
                                            let _e2583 = phi_6064_;
                                            let _e2584 = (_e1465 * _e1465);
                                            let _e2595 = max(fma(_e2020.z, _e2583.z, fma(_e2020.x, _e2583.x, (_e2020.y * _e2583.y))), 0f);
                                            let _e2608 = max(fma(_e2020.z, _e2024.z, fma(_e2020.x, _e2024.x, (_e2020.y * _e2024.y))), 0f);
                                            let _e2615 = max(fma(_e2020.z, _e2554.z, fma(_e2020.x, _e2554.x, (_e2020.y * _e2554.y))), 0f);
                                            let _e2616 = fma(_e772.y, _e381.member_4, 1f);
                                            let _e2617 = (_e2616 * _e2616);
                                            let _e2618 = (_e2617 * 0.125f);
                                            let _e2620 = fma(-(_e2617), 0.125f, 1f);
                                            let _e2633 = (1f - max(fma(_e2583.z, _e2024.z, fma(_e2583.x, _e2024.x, (_e2583.y * _e2024.y))), 0f));
                                            let _e2635 = select(_e2633, 0f, (_e2633 < 0f));
                                            let _e2638 = pow(select(_e2635, 1f, (_e2635 > 1f)), 5f);
                                            let _e2639 = fma((1f - _e2562), _e2638, _e2562);
                                            let _e2640 = fma((1f - _e2563), _e2638, _e2563);
                                            let _e2641 = fma((1f - _e2564), _e2638, _e2564);
                                            let _e2648 = (((_e2584 * _e2584) / (pow(fma((_e2595 * _e2595), fma(_e2584, _e2584, -1f), 1f), 2f) * 3.1415927f)) * ((_e2608 / fma(_e2608, _e2620, _e2618)) * (_e2615 / fma(_e2615, _e2620, _e2618))));
                                            let _e2653 = fma((4f * _e2608), _e2615, 0.0001f);
                                            phi_2989_ = vec3<f32>(fma((fma((((1f - _e2639) * _e2558) * _e1456), 0.31830987f, ((_e2648 * _e2639) / _e2653)) * (_e2513.member_1.x * _e2556)), _e2615, _e2029.x), fma((fma((((1f - _e2640) * _e2558) * _e1458), 0.31830987f, ((_e2648 * _e2640) / _e2653)) * (_e2513.member_1.y * _e2556)), _e2615, _e2029.y), fma((fma((((1f - _e2641) * _e2558) * _e1460), 0.31830987f, ((_e2648 * _e2641) / _e2653)) * (_e2513.member_1.z * _e2556)), _e2615, _e2029.z));
                                        }
                                        let _e2674 = phi_2989_;
                                        phi_3309_ = _e2674;
                                        phi_3310_ = select(true, false, _e2550);
                                        break;
                                    }
                                    case 2: {
                                        if (_e116 >= 13u) {
                                            phi_5770_ = (_e2084.member_1 <= (_e116 - 13u));
                                        } else {
                                            phi_5770_ = false;
                                        }
                                        let _e2195 = phi_5770_;
                                        if _e2195 {
                                            let _e2198 = global.member[_e2084.member_1];
                                            let _e2203 = global.member[(_e2084.member_1 + 1u)];
                                            let _e2208 = global.member[(_e2084.member_1 + 2u)];
                                            let _e2214 = global.member[(_e2084.member_1 + 3u)];
                                            let _e2219 = global.member[(_e2084.member_1 + 4u)];
                                            let _e2224 = global.member[(_e2084.member_1 + 5u)];
                                            let _e2230 = global.member[(_e2084.member_1 + 6u)];
                                            let _e2235 = global.member[(_e2084.member_1 + 7u)];
                                            let _e2240 = global.member[(_e2084.member_1 + 8u)];
                                            let _e2245 = global.member[(_e2084.member_1 + 9u)];
                                            let _e2250 = global.member[(_e2084.member_1 + 10u)];
                                            let _e2255 = global.member[(_e2084.member_1 + 11u)];
                                            let _e2261 = global.member[(_e2084.member_1 + 12u)];
                                            phi_3052_ = type_36(vec3<f32>(bitcast<f32>(_e2198), bitcast<f32>(_e2203), bitcast<f32>(_e2208)), vec3<f32>(bitcast<f32>(_e2214), bitcast<f32>(_e2219), bitcast<f32>(_e2224)), bitcast<f32>(_e2230), bitcast<f32>(_e2235), vec4<f32>(bitcast<f32>(_e2240), bitcast<f32>(_e2245), bitcast<f32>(_e2250), bitcast<f32>(_e2255)), bitcast<f32>(_e2261));
                                        } else {
                                            phi_3052_ = type_36(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2265 = phi_3052_;
                                        let _e2297 = (vec3<f32>((_e2145.member.x + fma(_e2184.x, _e2265.member.z, fma(_e2182.x, _e2265.member.y, (_e2180.x * _e2265.member.x)))), (_e2145.member.y + fma(_e2184.y, _e2265.member.z, fma(_e2182.y, _e2265.member.y, (_e2180.y * _e2265.member.x)))), (_e2145.member.z + fma(_e2184.z, _e2265.member.z, fma(_e2182.z, _e2265.member.y, (_e2180.z * _e2265.member.x))))) - _e124);
                                        let _e2304 = sqrt(fma(_e2297.z, _e2297.z, fma(_e2297.x, _e2297.x, (_e2297.y * _e2297.y))));
                                        let _e2305 = (_e2304 == 0f);
                                        if _e2305 {
                                            phi_3307_ = vec3<f32>();
                                        } else {
                                            if _e2305 {
                                                phi_5820_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5820_ = (_e2297 * (1f / _e2304));
                                            }
                                            let _e2309 = phi_5820_;
                                            let _e2319 = fma(_e2184.x, _e2265.member_1.z, fma(_e2182.x, _e2265.member_1.y, (_e2180.x * _e2265.member_1.x)));
                                            let _e2320 = fma(_e2184.y, _e2265.member_1.z, fma(_e2182.y, _e2265.member_1.y, (_e2180.y * _e2265.member_1.x)));
                                            let _e2321 = fma(_e2184.z, _e2265.member_1.z, fma(_e2182.z, _e2265.member_1.y, (_e2180.z * _e2265.member_1.x)));
                                            let _e2326 = sqrt(fma(_e2321, _e2321, fma(_e2319, _e2319, (_e2320 * _e2320))));
                                            if (_e2326 == 0f) {
                                                phi_5855_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5855_ = (vec3<f32>(_e2319, _e2320, _e2321) * (1f / _e2326));
                                            }
                                            let _e2331 = phi_5855_;
                                            let _e2343 = ((fma(_e2309.z, _e2331.z, fma(_e2309.x, _e2331.x, (_e2309.y * _e2331.y))) - _e2265.member_3) / (_e2265.member_2 - _e2265.member_3));
                                            let _e2345 = select(_e2343, 0f, (_e2343 < 0f));
                                            let _e2348 = (_e2265.member_5 * select(_e2345, 1f, (_e2345 > 1f)));
                                            let _e2350 = fma(-(_e772.z), _e381.member_3, 1f);
                                            let _e2354 = fma(0.4f, _e2350, (_e1456 * _e1468));
                                            let _e2355 = fma(0.4f, _e2350, (_e1458 * _e1468));
                                            let _e2356 = fma(0.4f, _e2350, (_e1460 * _e1468));
                                            let _e2363 = (_e2024 + _e2309);
                                            let _e2370 = sqrt(fma(_e2363.z, _e2363.z, fma(_e2363.x, _e2363.x, (_e2363.y * _e2363.y))));
                                            if (_e2370 == 0f) {
                                                phi_5890_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5890_ = (_e2363 * (1f / _e2370));
                                            }
                                            let _e2375 = phi_5890_;
                                            let _e2376 = (_e1465 * _e1465);
                                            let _e2387 = max(fma(_e2020.z, _e2375.z, fma(_e2020.x, _e2375.x, (_e2020.y * _e2375.y))), 0f);
                                            let _e2400 = max(fma(_e2020.z, _e2024.z, fma(_e2020.x, _e2024.x, (_e2020.y * _e2024.y))), 0f);
                                            let _e2404 = max(fma(_e2020.z, _e2309.z, fma(_e2020.x, _e2309.x, (_e2020.y * _e2309.y))), 0f);
                                            let _e2405 = fma(_e772.y, _e381.member_4, 1f);
                                            let _e2406 = (_e2405 * _e2405);
                                            let _e2407 = (_e2406 * 0.125f);
                                            let _e2409 = fma(-(_e2406), 0.125f, 1f);
                                            let _e2422 = (1f - max(fma(_e2375.z, _e2024.z, fma(_e2375.x, _e2024.x, (_e2375.y * _e2024.y))), 0f));
                                            let _e2424 = select(_e2422, 0f, (_e2422 < 0f));
                                            let _e2427 = pow(select(_e2424, 1f, (_e2424 > 1f)), 5f);
                                            let _e2428 = fma((1f - _e2354), _e2427, _e2354);
                                            let _e2429 = fma((1f - _e2355), _e2427, _e2355);
                                            let _e2430 = fma((1f - _e2356), _e2427, _e2356);
                                            let _e2437 = (((_e2376 * _e2376) / (pow(fma((_e2387 * _e2387), fma(_e2376, _e2376, -1f), 1f), 2f) * 3.1415927f)) * ((_e2400 / fma(_e2400, _e2409, _e2407)) * (_e2404 / fma(_e2404, _e2409, _e2407))));
                                            let _e2442 = fma((4f * _e2400), _e2404, 0.0001f);
                                            phi_3307_ = vec3<f32>(fma((fma((((1f - _e2428) * _e2350) * _e1456), 0.31830987f, ((_e2437 * _e2428) / _e2442)) * (_e2265.member_4.x * _e2348)), _e2404, _e2029.x), fma((fma((((1f - _e2429) * _e2350) * _e1458), 0.31830987f, ((_e2437 * _e2429) / _e2442)) * (_e2265.member_4.y * _e2348)), _e2404, _e2029.y), fma((fma((((1f - _e2430) * _e2350) * _e1460), 0.31830987f, ((_e2437 * _e2430) / _e2442)) * (_e2265.member_4.z * _e2348)), _e2404, _e2029.z));
                                        }
                                        let _e2463 = phi_3307_;
                                        phi_3309_ = _e2463;
                                        phi_3310_ = select(true, false, _e2305);
                                        break;
                                    }
                                    default: {
                                        phi_3309_ = vec3<f32>();
                                        phi_3310_ = bool();
                                        break;
                                    }
                                }
                                let _e2888 = phi_3309_;
                                let _e2890 = phi_3310_;
                                phi_3319_ = select(_e2888, _e2029, vec3(select(true, false, _e2890)));
                            }
                            let _e2895 = phi_3319_;
                            phi_2428_ = _e2895;
                            phi_3321_ = select(true, false, _e2062);
                            break;
                        }
                        default: {
                            phi_2428_ = vec3<f32>();
                            phi_3321_ = bool();
                            break;
                        }
                    }
                    let _e2898 = phi_2428_;
                    let _e2900 = phi_3321_;
                    continue;
                    continuing {
                        phi_2424_ = _e2042;
                        phi_2427_ = _e2898;
                        break if !(_e2900);
                    }
                }
                let _e2903 = fma(-(_e772.z), _e381.member_3, 1f);
                let _e2907 = fma(0.04f, _e2903, (_e1456 * _e1468));
                let _e2908 = fma(0.04f, _e2903, (_e1458 * _e1468));
                let _e2909 = fma(0.04f, _e2903, (_e1460 * _e1468));
                let _e2921 = fma(-(_e772.y), _e381.member_4, 1f);
                let _e2928 = (1f - max(fma(_e2020.z, _e2024.z, fma(_e2020.x, _e2024.x, (_e2020.y * _e2024.y))), 0f));
                let _e2930 = select(_e2928, 0f, (_e2928 < 0f));
                let _e2933 = pow(select(_e2930, 1f, (_e2930 > 1f)), 5f);
                let _e2934 = fma((max(_e2921, _e2907) - _e2907), _e2933, _e2907);
                let _e2935 = fma((max(_e2921, _e2908) - _e2908), _e2933, _e2908);
                let _e2936 = fma((max(_e2921, _e2909) - _e2909), _e2933, _e2909);
                let _e2957 = textureSampleLevel(global_18, global_19, vec2<f32>(fma(((bitcast<f32>(_e175) + fma(bitcast<f32>(_e159), _e124.z, fma(bitcast<f32>(_e143), _e124.y, (bitcast<f32>(_e127) * _e124.x)))) / _e207), 0.5f, 0.5f), fma(((bitcast<f32>(_e179) + fma(bitcast<f32>(_e163), _e124.z, fma(bitcast<f32>(_e147), _e124.y, (bitcast<f32>(_e131) * _e124.x)))) / _e207), 0.5f, 0.5f)), i32(0f));
                let _e2965 = (1f - select(0f, 1f, (fma(((bitcast<f32>(_e183) + fma(bitcast<f32>(_e167), _e124.z, fma(bitcast<f32>(_e151), _e124.y, (bitcast<f32>(_e135) * _e124.x)))) / _e207), 0.5f, 0.5f) > vec4(_e2957).x)));
                let _e2967 = local_2;
                let _e2971 = local_3;
                let _e2975 = local_4;
                phi_3443_ = vec4<f32>(fma(_e1478, _e381.member_1, fma(fma(((1f - _e2934) * _e2903), (_e1487.x * _e1456), (_e1835.x * fma(_e2934, _e1851.x, _e1851.y))), _e1472, (_e2967.x * _e2965))), fma(_e1480, _e381.member_1, fma(fma(((1f - _e2935) * _e2903), (_e1487.y * _e1458), (_e1835.y * fma(_e2935, _e1851.x, _e1851.y))), _e1472, (_e2971.y * _e2965))), fma(_e1482, _e381.member_1, fma(fma(((1f - _e2936) * _e2903), (_e1487.z * _e1460), (_e1835.z * fma(_e2936, _e1851.x, _e1851.y))), _e1472, (_e2975.z * _e2965))), 1f);
            } else {
                phi_3443_ = (vec4<f32>((_e118.x * _e578.x), (_e118.y * _e578.y), (_e118.z * _e578.z), (_e118.w * _e578.w)) * _e381.member_2);
            }
            let _e2986 = phi_3443_;
            global_20 = _e2986;
            break;
        }
        case 1: {
            let _e1993 = sqrt(fma(_e119.x, _e119.x, (_e119.y * _e119.y)));
            if (_e1993 == 0f) {
                phi_5518_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5518_ = (vec3<f32>(_e119.x, _e119.y, 0f) * (1f / _e1993));
            }
            let _e1998 = phi_5518_;
            global_20 = vec4<f32>(((_e1998.x + 1f) * 0.5f), ((_e1998.y + 1f) * 0.5f), ((_e1998.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1972 = sqrt(fma(_e120.x, _e120.x, (_e120.y * _e120.y)));
            if (_e1972 == 0f) {
                phi_5469_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5469_ = (vec3<f32>(_e120.x, _e120.y, 0f) * (1f / _e1972));
            }
            let _e1977 = phi_5469_;
            global_20 = vec4<f32>(((_e1977.x + 1f) * 0.5f), ((_e1977.y + 1f) * 0.5f), ((_e1977.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1814 {
                phi_5420_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5420_ = (_e1441 * (1f / _e1813));
            }
            let _e1956 = phi_5420_;
            global_20 = vec4<f32>(((_e1956.x + 1f) * 0.5f), ((_e1956.y + 1f) * 0.5f), ((_e1956.z + 1f) * 0.5f), 1f);
            break;
        }
        case 4: {
            global_20 = _e118;
            break;
        }
        case 5: {
            let _e1937 = sqrt(fma(_e121.z, _e121.z, fma(_e121.x, _e121.x, (_e121.y * _e121.y))));
            if (_e1937 == 0f) {
                phi_5371_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5371_ = (_e121 * (1f / _e1937));
            }
            let _e1942 = phi_5371_;
            global_20 = vec4<f32>(((_e1942.x + 1f) * 0.5f), ((_e1942.y + 1f) * 0.5f), ((_e1942.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1915 = sqrt(fma(_e1439.z, _e1439.z, fma(_e1439.x, _e1439.x, (_e1439.y * _e1439.y))));
            if (_e1915 == 0f) {
                phi_5322_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5322_ = (_e1439 * (1f / _e1915));
            }
            let _e1920 = phi_5322_;
            global_20 = vec4<f32>(((_e1920.x + 1f) * 0.5f), ((_e1920.y + 1f) * 0.5f), ((_e1920.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1893 = sqrt(fma(_e122.z, _e122.z, fma(_e122.x, _e122.x, (_e122.y * _e122.y))));
            if (_e1893 == 0f) {
                phi_5273_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5273_ = (_e122 * (1f / _e1893));
            }
            let _e1898 = phi_5273_;
            global_20 = vec4<f32>(((_e1898.x + 1f) * 0.5f), ((_e1898.y + 1f) * 0.5f), ((_e1898.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1871 = sqrt(fma(_e123.z, _e123.z, fma(_e123.x, _e123.x, (_e123.y * _e123.y))));
            if (_e1871 == 0f) {
                phi_5224_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5224_ = (_e123 * (1f / _e1871));
            }
            let _e1876 = phi_5224_;
            global_20 = vec4<f32>(((_e1876.x + 1f) * 0.5f), ((_e1876.y + 1f) * 0.5f), ((_e1876.z + 1f) * 0.5f), 1f);
            break;
        }
        case 9: {
            global_20 = vec4<f32>(_e1487.x, _e1487.y, _e1487.z, 1f);
            break;
        }
        case 10: {
            global_20 = vec4<f32>(_e1835.x, _e1835.y, _e1835.z, 1f);
            break;
        }
        case 11: {
            global_20 = vec4<f32>(_e1851.x, _e1851.y, 1f, 1f);
            break;
        }
        case 12: {
            global_20 = (vec4<f32>(_e1445, _e1448, _e1451, (_e578.w * _e381.member_2.w)) * _e118);
            break;
        }
        case 13: {
            global_20 = vec4<f32>(_e1465, _e1465, _e1465, 1f);
            break;
        }
        case 14: {
            global_20 = vec4<f32>(_e1468, _e1468, _e1468, 1f);
            break;
        }
        case 15: {
            global_20 = vec4<f32>(_e1472, _e1472, _e1472, 1f);
            break;
        }
        case 16: {
            global_20 = vec4<f32>((_e1478 * _e381.member_1), (_e1480 * _e381.member_1), (_e1482 * _e381.member_1), 1f);
            break;
        }
        case 17: {
            global_20 = vec4<f32>(_e1354.x, _e1354.y, _e1354.z, 1f);
            break;
        }
        case 18: {
            global_20 = vec4<f32>(_e381.member.x, _e381.member.y, _e381.member.z, 1f);
            break;
        }
        case 19: {
            global_20 = vec4<f32>(_e381.member_1, _e381.member_1, _e381.member_1, 1f);
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
