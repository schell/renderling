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

struct type_33 {
    member: u32,
    member_1: u32,
    member_2: u32,
}

struct type_34 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: f32,
}

struct type_35 {
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
var<private> global_2: vec4<f32>;
var<private> global_3: vec2<f32>;
var<private> global_4: vec2<f32>;
var<private> global_5: vec3<f32>;
var<private> global_6: vec3<f32>;
var<private> global_7: vec3<f32>;
var<private> global_8: vec3<f32>;
@group(1) @binding(1) 
var global_9: sampler;
@group(1) @binding(0) 
var global_10: texture_2d_array<f32>;
@group(1) @binding(2) 
var global_11: texture_cube<f32>;
@group(1) @binding(3) 
var global_12: sampler;
@group(1) @binding(4) 
var global_13: texture_cube<f32>;
@group(1) @binding(5) 
var global_14: sampler;
@group(1) @binding(6) 
var global_15: texture_2d<f32>;
@group(1) @binding(7) 
var global_16: sampler;
var<private> global_17: vec4<f32>;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var phi_610_: u32;
    var phi_3876_: bool;
    var phi_764_: type_31;
    var phi_768_: type_31;
    var phi_3913_: bool;
    var phi_808_: u32;
    var phi_817_: u32;
    var phi_830_: type_32;
    var phi_3935_: f32;
    var phi_3948_: bool;
    var phi_884_: f32;
    var phi_879_: f32;
    var phi_885_: f32;
    var phi_3965_: bool;
    var phi_850_: f32;
    var phi_887_: f32;
    var phi_3983_: f32;
    var phi_3996_: bool;
    var phi_942_: f32;
    var phi_937_: f32;
    var phi_943_: f32;
    var phi_4013_: bool;
    var phi_908_: f32;
    var phi_945_: f32;
    var phi_4049_: bool;
    var phi_1028_: u32;
    var phi_1037_: u32;
    var phi_1050_: type_32;
    var phi_4070_: f32;
    var phi_4083_: bool;
    var phi_1104_: f32;
    var phi_1099_: f32;
    var phi_1105_: f32;
    var phi_4100_: bool;
    var phi_1070_: f32;
    var phi_1107_: f32;
    var phi_4118_: f32;
    var phi_4131_: bool;
    var phi_1162_: f32;
    var phi_1157_: f32;
    var phi_1163_: f32;
    var phi_4148_: bool;
    var phi_1128_: f32;
    var phi_1165_: f32;
    var phi_4184_: bool;
    var phi_1248_: u32;
    var phi_1257_: u32;
    var phi_1270_: type_32;
    var phi_4205_: f32;
    var phi_4218_: bool;
    var phi_1324_: f32;
    var phi_1319_: f32;
    var phi_1325_: f32;
    var phi_4235_: bool;
    var phi_1290_: f32;
    var phi_1327_: f32;
    var phi_4253_: f32;
    var phi_4266_: bool;
    var phi_1382_: f32;
    var phi_1377_: f32;
    var phi_1383_: f32;
    var phi_4283_: bool;
    var phi_1348_: f32;
    var phi_1385_: f32;
    var phi_4319_: bool;
    var phi_1468_: u32;
    var phi_1477_: u32;
    var phi_1490_: type_32;
    var phi_4340_: f32;
    var phi_4353_: bool;
    var phi_1544_: f32;
    var phi_1539_: f32;
    var phi_1545_: f32;
    var phi_4370_: bool;
    var phi_1510_: f32;
    var phi_1547_: f32;
    var phi_4388_: f32;
    var phi_4401_: bool;
    var phi_1602_: f32;
    var phi_1597_: f32;
    var phi_1603_: f32;
    var phi_4418_: bool;
    var phi_1568_: f32;
    var phi_1605_: f32;
    var phi_4454_: bool;
    var phi_1688_: u32;
    var phi_1697_: u32;
    var phi_1710_: type_32;
    var phi_4475_: f32;
    var phi_4488_: bool;
    var phi_1764_: f32;
    var phi_1759_: f32;
    var phi_1765_: f32;
    var phi_4505_: bool;
    var phi_1730_: f32;
    var phi_1767_: f32;
    var phi_4523_: f32;
    var phi_4536_: bool;
    var phi_1822_: f32;
    var phi_1817_: f32;
    var phi_1823_: f32;
    var phi_4553_: bool;
    var phi_1788_: f32;
    var phi_1825_: f32;
    var phi_4611_: vec3<f32>;
    var phi_4646_: vec3<f32>;
    var phi_4681_: vec3<f32>;
    var phi_4716_: vec3<f32>;
    var phi_4751_: vec3<f32>;
    var phi_1919_: vec3<f32>;
    var phi_1920_: vec3<f32>;
    var phi_4783_: bool;
    var phi_2127_: type_24;
    var phi_2128_: type_24;
    var phi_2151_: type_24;
    var phi_2178_: bool;
    var phi_2184_: type_24;
    var phi_2185_: type_24;
    var phi_2208_: type_24;
    var phi_2231_: bool;
    var phi_2252_: type_22;
    var phi_4855_: vec3<f32>;
    var phi_4914_: vec3<f32>;
    var phi_4988_: vec3<f32>;
    var phi_5048_: vec3<f32>;
    var phi_5097_: vec3<f32>;
    var phi_5146_: vec3<f32>;
    var phi_5195_: vec3<f32>;
    var phi_5244_: vec3<f32>;
    var phi_5293_: vec3<f32>;
    var phi_5342_: vec3<f32>;
    var phi_5381_: vec3<f32>;
    var phi_5416_: vec3<f32>;
    var phi_2292_: type_24;
    var phi_2295_: vec3<f32>;
    var phi_2293_: type_24;
    var phi_2318_: type_24;
    var phi_5433_: u32;
    var phi_5452_: bool;
    var phi_2335_: u32;
    var phi_5484_: bool;
    var phi_2352_: u32;
    var phi_2362_: type_33;
    var phi_5514_: bool;
    var phi_2412_: type_29;
    var phi_5594_: bool;
    var phi_2920_: type_35;
    var phi_5644_: vec3<f32>;
    var phi_5679_: vec3<f32>;
    var phi_5714_: vec3<f32>;
    var phi_3175_: vec3<f32>;
    var phi_5806_: bool;
    var phi_2667_: type_34;
    var phi_5853_: vec3<f32>;
    var phi_5888_: vec3<f32>;
    var phi_2857_: vec3<f32>;
    var phi_5980_: bool;
    var phi_2460_: type_34;
    var phi_6027_: vec3<f32>;
    var phi_6062_: vec3<f32>;
    var phi_3177_: vec3<f32>;
    var phi_3178_: bool;
    var phi_3187_: vec3<f32>;
    var phi_2296_: vec3<f32>;
    var phi_3189_: bool;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var phi_3306_: vec4<f32>;

    let _e113 = arrayLength((&global.member));
    let _e114 = global_1;
    let _e115 = global_2;
    let _e116 = global_3;
    let _e117 = global_4;
    let _e118 = global_5;
    let _e119 = global_6;
    let _e120 = global_7;
    let _e121 = global_8;
    let _e125 = global.member[(_e114 + 9u)];
    let _e129 = global.member[(_e114 + 11u)];
    let _e133 = global.member[(_e114 + 17u)];
    let _e136 = global.member[_e133];
    let _e140 = global.member[(_e133 + 1u)];
    let _e144 = global.member[(_e133 + 4u)];
    switch bitcast<i32>(_e144) {
        case 0: {
            phi_610_ = 0u;
            break;
        }
        case 1: {
            phi_610_ = 1u;
            break;
        }
        case 2: {
            phi_610_ = 2u;
            break;
        }
        case 3: {
            phi_610_ = 3u;
            break;
        }
        case 4: {
            phi_610_ = 4u;
            break;
        }
        case 5: {
            phi_610_ = 5u;
            break;
        }
        case 6: {
            phi_610_ = 6u;
            break;
        }
        case 7: {
            phi_610_ = 7u;
            break;
        }
        case 8: {
            phi_610_ = 8u;
            break;
        }
        case 9: {
            phi_610_ = 9u;
            break;
        }
        case 10: {
            phi_610_ = 10u;
            break;
        }
        case 11: {
            phi_610_ = 11u;
            break;
        }
        case 12: {
            phi_610_ = 12u;
            break;
        }
        case 13: {
            phi_610_ = 13u;
            break;
        }
        case 14: {
            phi_610_ = 14u;
            break;
        }
        case 15: {
            phi_610_ = 15u;
            break;
        }
        case 16: {
            phi_610_ = 16u;
            break;
        }
        case 17: {
            phi_610_ = 17u;
            break;
        }
        case 18: {
            phi_610_ = 18u;
            break;
        }
        case 19: {
            phi_610_ = 19u;
            break;
        }
        default: {
            phi_610_ = 0u;
            break;
        }
    }
    let _e147 = phi_610_;
    let _e151 = global.member[(_e133 + 5u)];
    let _e156 = global.member[(_e133 + 9u)];
    let _e160 = global.member[(_e133 + 10u)];
    if (_e129 == 4294967295u) {
        phi_768_ = type_31(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
    } else {
        if (_e113 >= 22u) {
            phi_3876_ = (_e129 <= (_e113 - 22u));
        } else {
            phi_3876_ = false;
        }
        let _e166 = phi_3876_;
        if _e166 {
            let _e169 = global.member[_e129];
            let _e174 = global.member[(_e129 + 1u)];
            let _e179 = global.member[(_e129 + 2u)];
            let _e185 = global.member[(_e129 + 3u)];
            let _e190 = global.member[(_e129 + 4u)];
            let _e195 = global.member[(_e129 + 5u)];
            let _e200 = global.member[(_e129 + 6u)];
            let _e205 = global.member[(_e129 + 7u)];
            let _e211 = global.member[(_e129 + 8u)];
            let _e216 = global.member[(_e129 + 9u)];
            let _e221 = global.member[(_e129 + 10u)];
            let _e225 = global.member[(_e129 + 11u)];
            let _e229 = global.member[(_e129 + 12u)];
            let _e233 = global.member[(_e129 + 13u)];
            let _e237 = global.member[(_e129 + 14u)];
            let _e241 = global.member[(_e129 + 15u)];
            let _e245 = global.member[(_e129 + 16u)];
            let _e249 = global.member[(_e129 + 17u)];
            let _e253 = global.member[(_e129 + 18u)];
            let _e257 = global.member[(_e129 + 19u)];
            let _e261 = global.member[(_e129 + 20u)];
            let _e266 = global.member[(_e129 + 21u)];
            phi_764_ = type_31(vec3<f32>(bitcast<f32>(_e169), bitcast<f32>(_e174), bitcast<f32>(_e179)), bitcast<f32>(_e185), vec4<f32>(bitcast<f32>(_e190), bitcast<f32>(_e195), bitcast<f32>(_e200), bitcast<f32>(_e205)), bitcast<f32>(_e211), bitcast<f32>(_e216), _e221, _e225, _e229, _e233, _e237, _e241, _e245, _e249, _e253, _e257, (_e261 == 1u), bitcast<f32>(_e266));
        } else {
            phi_764_ = type_31(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
        }
        let _e270 = phi_764_;
        phi_768_ = type_31(_e270.member, _e270.member_1, _e270.member_2, _e270.member_3, _e270.member_4, _e270.member_5, _e270.member_6, _e270.member_7, _e270.member_8, _e270.member_9, _e270.member_10, _e270.member_11, _e270.member_12, _e270.member_13, _e270.member_14, (_e270.member_15 && (_e151 == 1u)), _e270.member_16);
    }
    let _e292 = phi_768_;
    let _e296 = select(_e117, _e116, vec2((_e292.member_10 == 0u)));
    let _e298 = (_e113 >= 8u);
    if _e298 {
        phi_3913_ = (_e292.member_5 <= (_e113 - 8u));
    } else {
        phi_3913_ = false;
    }
    let _e302 = phi_3913_;
    if _e302 {
        let _e305 = global.member[_e292.member_5];
        let _e309 = global.member[(_e292.member_5 + 1u)];
        let _e314 = global.member[(_e292.member_5 + 2u)];
        let _e318 = global.member[(_e292.member_5 + 3u)];
        let _e323 = global.member[(_e292.member_5 + 4u)];
        let _e327 = global.member[(_e292.member_5 + 5u)];
        let _e331 = global.member[(_e292.member_5 + 6u)];
        switch bitcast<i32>(_e331) {
            case 0: {
                phi_808_ = 0u;
                break;
            }
            case 1: {
                phi_808_ = 1u;
                break;
            }
            case 2: {
                phi_808_ = 2u;
                break;
            }
            default: {
                phi_808_ = 0u;
                break;
            }
        }
        let _e334 = phi_808_;
        let _e338 = global.member[(_e292.member_5 + 7u)];
        switch bitcast<i32>(_e338) {
            case 0: {
                phi_817_ = 0u;
                break;
            }
            case 1: {
                phi_817_ = 1u;
                break;
            }
            case 2: {
                phi_817_ = 2u;
                break;
            }
            default: {
                phi_817_ = 0u;
                break;
            }
        }
        let _e341 = phi_817_;
        phi_830_ = type_32(type_24(_e334, _e341), vec2<u32>(_e305, _e309), vec2<u32>(_e314, _e318), _e323, _e327);
    } else {
        phi_830_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e345 = phi_830_;
    switch bitcast<i32>(_e345.member.member) {
        case 1: {
            let _e383 = abs(_e296.x);
            let _e385 = (_e383 % 1f);
            if (_e383 >= 1f) {
                phi_3965_ = select(true, false, (_e385 == 0f));
            } else {
                phi_3965_ = true;
            }
            let _e389 = phi_3965_;
            let _e390 = select(1f, _e385, _e389);
            if (select(-1f, 1f, (_e296.x >= 0f)) > 0f) {
                phi_850_ = _e390;
            } else {
                phi_850_ = (1f - _e390);
            }
            let _e394 = phi_850_;
            phi_887_ = _e394;
            break;
        }
        case 2: {
            let _e357 = abs(_e296.x);
            let _e364 = ((select(select(u32(_e357), 0u, (_e357 < 0f)), 4294967295u, (_e357 > 4294967000f)) % 2u) == 0u);
            let _e366 = (_e357 % 1f);
            if (_e357 >= 1f) {
                phi_3948_ = select(true, false, (_e366 == 0f));
            } else {
                phi_3948_ = true;
            }
            let _e370 = phi_3948_;
            let _e371 = select(1f, _e366, _e370);
            if (select(-1f, 1f, (_e296.x >= 0f)) > 0f) {
                if _e364 {
                    phi_879_ = _e371;
                } else {
                    phi_879_ = (1f - _e371);
                }
                let _e378 = phi_879_;
                phi_885_ = _e378;
            } else {
                if _e364 {
                    phi_884_ = (1f - _e371);
                } else {
                    phi_884_ = _e371;
                }
                let _e375 = phi_884_;
                phi_885_ = _e375;
            }
            let _e380 = phi_885_;
            phi_887_ = _e380;
            break;
        }
        case 0: {
            if (_e296.x > 1f) {
                phi_3935_ = 0.9999999f;
            } else {
                phi_3935_ = select(_e296.x, 0.00000011920929f, (_e296.x < 0f));
            }
            let _e354 = phi_3935_;
            phi_887_ = _e354;
            break;
        }
        default: {
            phi_887_ = f32();
            break;
        }
    }
    let _e396 = phi_887_;
    switch bitcast<i32>(_e345.member.member_1) {
        case 1: {
            let _e434 = abs(_e296.y);
            let _e436 = (_e434 % 1f);
            if (_e434 >= 1f) {
                phi_4013_ = select(true, false, (_e436 == 0f));
            } else {
                phi_4013_ = true;
            }
            let _e440 = phi_4013_;
            let _e441 = select(1f, _e436, _e440);
            if (select(-1f, 1f, (_e296.y >= 0f)) > 0f) {
                phi_908_ = _e441;
            } else {
                phi_908_ = (1f - _e441);
            }
            let _e445 = phi_908_;
            phi_945_ = _e445;
            break;
        }
        case 2: {
            let _e408 = abs(_e296.y);
            let _e415 = ((select(select(u32(_e408), 0u, (_e408 < 0f)), 4294967295u, (_e408 > 4294967000f)) % 2u) == 0u);
            let _e417 = (_e408 % 1f);
            if (_e408 >= 1f) {
                phi_3996_ = select(true, false, (_e417 == 0f));
            } else {
                phi_3996_ = true;
            }
            let _e421 = phi_3996_;
            let _e422 = select(1f, _e417, _e421);
            if (select(-1f, 1f, (_e296.y >= 0f)) > 0f) {
                if _e415 {
                    phi_937_ = _e422;
                } else {
                    phi_937_ = (1f - _e422);
                }
                let _e429 = phi_937_;
                phi_943_ = _e429;
            } else {
                if _e415 {
                    phi_942_ = (1f - _e422);
                } else {
                    phi_942_ = _e422;
                }
                let _e426 = phi_942_;
                phi_943_ = _e426;
            }
            let _e431 = phi_943_;
            phi_945_ = _e431;
            break;
        }
        case 0: {
            if (_e296.y > 1f) {
                phi_3983_ = 0.9999999f;
            } else {
                phi_3983_ = select(_e296.y, 0.00000011920929f, (_e296.y < 0f));
            }
            let _e405 = phi_3983_;
            phi_945_ = _e405;
            break;
        }
        default: {
            phi_945_ = f32();
            break;
        }
    }
    let _e447 = phi_945_;
    let _e451 = (_e396 * f32(_e345.member_2.x));
    let _e460 = (_e447 * f32(_e345.member_2.y));
    let _e472 = f32(_e136);
    let _e473 = f32(_e140);
    let _e480 = vec3<f32>((f32((select(select(u32(_e451), 0u, (_e451 < 0f)), 4294967295u, (_e451 > 4294967000f)) + _e345.member_1.x)) / _e472), (f32((select(select(u32(_e460), 0u, (_e460 < 0f)), 4294967295u, (_e460 > 4294967000f)) + _e345.member_1.y)) / _e473), f32(_e345.member_3));
    let _e486 = textureSampleLevel(global_10, global_9, vec2<f32>(_e480.x, _e480.y), i32(_e480.z), 0f);
    let _e489 = select(_e486, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e292.member_5 == 4294967295u)));
    let _e493 = select(_e117, _e116, vec2((_e292.member_11 == 0u)));
    if _e298 {
        phi_4049_ = (_e292.member_6 <= (_e113 - 8u));
    } else {
        phi_4049_ = false;
    }
    let _e498 = phi_4049_;
    if _e498 {
        let _e501 = global.member[_e292.member_6];
        let _e505 = global.member[(_e292.member_6 + 1u)];
        let _e510 = global.member[(_e292.member_6 + 2u)];
        let _e514 = global.member[(_e292.member_6 + 3u)];
        let _e519 = global.member[(_e292.member_6 + 4u)];
        let _e523 = global.member[(_e292.member_6 + 5u)];
        let _e527 = global.member[(_e292.member_6 + 6u)];
        switch bitcast<i32>(_e527) {
            case 0: {
                phi_1028_ = 0u;
                break;
            }
            case 1: {
                phi_1028_ = 1u;
                break;
            }
            case 2: {
                phi_1028_ = 2u;
                break;
            }
            default: {
                phi_1028_ = 0u;
                break;
            }
        }
        let _e530 = phi_1028_;
        let _e534 = global.member[(_e292.member_6 + 7u)];
        switch bitcast<i32>(_e534) {
            case 0: {
                phi_1037_ = 0u;
                break;
            }
            case 1: {
                phi_1037_ = 1u;
                break;
            }
            case 2: {
                phi_1037_ = 2u;
                break;
            }
            default: {
                phi_1037_ = 0u;
                break;
            }
        }
        let _e537 = phi_1037_;
        phi_1050_ = type_32(type_24(_e530, _e537), vec2<u32>(_e501, _e505), vec2<u32>(_e510, _e514), _e519, _e523);
    } else {
        phi_1050_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e541 = phi_1050_;
    switch bitcast<i32>(_e541.member.member) {
        case 1: {
            let _e579 = abs(_e493.x);
            let _e581 = (_e579 % 1f);
            if (_e579 >= 1f) {
                phi_4100_ = select(true, false, (_e581 == 0f));
            } else {
                phi_4100_ = true;
            }
            let _e585 = phi_4100_;
            let _e586 = select(1f, _e581, _e585);
            if (select(-1f, 1f, (_e493.x >= 0f)) > 0f) {
                phi_1070_ = _e586;
            } else {
                phi_1070_ = (1f - _e586);
            }
            let _e590 = phi_1070_;
            phi_1107_ = _e590;
            break;
        }
        case 2: {
            let _e553 = abs(_e493.x);
            let _e560 = ((select(select(u32(_e553), 0u, (_e553 < 0f)), 4294967295u, (_e553 > 4294967000f)) % 2u) == 0u);
            let _e562 = (_e553 % 1f);
            if (_e553 >= 1f) {
                phi_4083_ = select(true, false, (_e562 == 0f));
            } else {
                phi_4083_ = true;
            }
            let _e566 = phi_4083_;
            let _e567 = select(1f, _e562, _e566);
            if (select(-1f, 1f, (_e493.x >= 0f)) > 0f) {
                if _e560 {
                    phi_1099_ = _e567;
                } else {
                    phi_1099_ = (1f - _e567);
                }
                let _e574 = phi_1099_;
                phi_1105_ = _e574;
            } else {
                if _e560 {
                    phi_1104_ = (1f - _e567);
                } else {
                    phi_1104_ = _e567;
                }
                let _e571 = phi_1104_;
                phi_1105_ = _e571;
            }
            let _e576 = phi_1105_;
            phi_1107_ = _e576;
            break;
        }
        case 0: {
            if (_e493.x > 1f) {
                phi_4070_ = 0.9999999f;
            } else {
                phi_4070_ = select(_e493.x, 0.00000011920929f, (_e493.x < 0f));
            }
            let _e550 = phi_4070_;
            phi_1107_ = _e550;
            break;
        }
        default: {
            phi_1107_ = f32();
            break;
        }
    }
    let _e592 = phi_1107_;
    switch bitcast<i32>(_e541.member.member_1) {
        case 1: {
            let _e630 = abs(_e493.y);
            let _e632 = (_e630 % 1f);
            if (_e630 >= 1f) {
                phi_4148_ = select(true, false, (_e632 == 0f));
            } else {
                phi_4148_ = true;
            }
            let _e636 = phi_4148_;
            let _e637 = select(1f, _e632, _e636);
            if (select(-1f, 1f, (_e493.y >= 0f)) > 0f) {
                phi_1128_ = _e637;
            } else {
                phi_1128_ = (1f - _e637);
            }
            let _e641 = phi_1128_;
            phi_1165_ = _e641;
            break;
        }
        case 2: {
            let _e604 = abs(_e493.y);
            let _e611 = ((select(select(u32(_e604), 0u, (_e604 < 0f)), 4294967295u, (_e604 > 4294967000f)) % 2u) == 0u);
            let _e613 = (_e604 % 1f);
            if (_e604 >= 1f) {
                phi_4131_ = select(true, false, (_e613 == 0f));
            } else {
                phi_4131_ = true;
            }
            let _e617 = phi_4131_;
            let _e618 = select(1f, _e613, _e617);
            if (select(-1f, 1f, (_e493.y >= 0f)) > 0f) {
                if _e611 {
                    phi_1157_ = _e618;
                } else {
                    phi_1157_ = (1f - _e618);
                }
                let _e625 = phi_1157_;
                phi_1163_ = _e625;
            } else {
                if _e611 {
                    phi_1162_ = (1f - _e618);
                } else {
                    phi_1162_ = _e618;
                }
                let _e622 = phi_1162_;
                phi_1163_ = _e622;
            }
            let _e627 = phi_1163_;
            phi_1165_ = _e627;
            break;
        }
        case 0: {
            if (_e493.y > 1f) {
                phi_4118_ = 0.9999999f;
            } else {
                phi_4118_ = select(_e493.y, 0.00000011920929f, (_e493.y < 0f));
            }
            let _e601 = phi_4118_;
            phi_1165_ = _e601;
            break;
        }
        default: {
            phi_1165_ = f32();
            break;
        }
    }
    let _e643 = phi_1165_;
    let _e647 = (_e592 * f32(_e541.member_2.x));
    let _e656 = (_e643 * f32(_e541.member_2.y));
    let _e674 = vec3<f32>((f32((select(select(u32(_e647), 0u, (_e647 < 0f)), 4294967295u, (_e647 > 4294967000f)) + _e541.member_1.x)) / _e472), (f32((select(select(u32(_e656), 0u, (_e656 < 0f)), 4294967295u, (_e656 > 4294967000f)) + _e541.member_1.y)) / _e473), f32(_e541.member_3));
    let _e680 = textureSampleLevel(global_10, global_9, vec2<f32>(_e674.x, _e674.y), i32(_e674.z), 0f);
    let _e683 = select(_e680, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e292.member_6 == 4294967295u)));
    let _e687 = select(_e117, _e116, vec2((_e292.member_12 == 0u)));
    if _e298 {
        phi_4184_ = (_e292.member_7 <= (_e113 - 8u));
    } else {
        phi_4184_ = false;
    }
    let _e692 = phi_4184_;
    if _e692 {
        let _e695 = global.member[_e292.member_7];
        let _e699 = global.member[(_e292.member_7 + 1u)];
        let _e704 = global.member[(_e292.member_7 + 2u)];
        let _e708 = global.member[(_e292.member_7 + 3u)];
        let _e713 = global.member[(_e292.member_7 + 4u)];
        let _e717 = global.member[(_e292.member_7 + 5u)];
        let _e721 = global.member[(_e292.member_7 + 6u)];
        switch bitcast<i32>(_e721) {
            case 0: {
                phi_1248_ = 0u;
                break;
            }
            case 1: {
                phi_1248_ = 1u;
                break;
            }
            case 2: {
                phi_1248_ = 2u;
                break;
            }
            default: {
                phi_1248_ = 0u;
                break;
            }
        }
        let _e724 = phi_1248_;
        let _e728 = global.member[(_e292.member_7 + 7u)];
        switch bitcast<i32>(_e728) {
            case 0: {
                phi_1257_ = 0u;
                break;
            }
            case 1: {
                phi_1257_ = 1u;
                break;
            }
            case 2: {
                phi_1257_ = 2u;
                break;
            }
            default: {
                phi_1257_ = 0u;
                break;
            }
        }
        let _e731 = phi_1257_;
        phi_1270_ = type_32(type_24(_e724, _e731), vec2<u32>(_e695, _e699), vec2<u32>(_e704, _e708), _e713, _e717);
    } else {
        phi_1270_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e735 = phi_1270_;
    switch bitcast<i32>(_e735.member.member) {
        case 1: {
            let _e773 = abs(_e687.x);
            let _e775 = (_e773 % 1f);
            if (_e773 >= 1f) {
                phi_4235_ = select(true, false, (_e775 == 0f));
            } else {
                phi_4235_ = true;
            }
            let _e779 = phi_4235_;
            let _e780 = select(1f, _e775, _e779);
            if (select(-1f, 1f, (_e687.x >= 0f)) > 0f) {
                phi_1290_ = _e780;
            } else {
                phi_1290_ = (1f - _e780);
            }
            let _e784 = phi_1290_;
            phi_1327_ = _e784;
            break;
        }
        case 2: {
            let _e747 = abs(_e687.x);
            let _e754 = ((select(select(u32(_e747), 0u, (_e747 < 0f)), 4294967295u, (_e747 > 4294967000f)) % 2u) == 0u);
            let _e756 = (_e747 % 1f);
            if (_e747 >= 1f) {
                phi_4218_ = select(true, false, (_e756 == 0f));
            } else {
                phi_4218_ = true;
            }
            let _e760 = phi_4218_;
            let _e761 = select(1f, _e756, _e760);
            if (select(-1f, 1f, (_e687.x >= 0f)) > 0f) {
                if _e754 {
                    phi_1319_ = _e761;
                } else {
                    phi_1319_ = (1f - _e761);
                }
                let _e768 = phi_1319_;
                phi_1325_ = _e768;
            } else {
                if _e754 {
                    phi_1324_ = (1f - _e761);
                } else {
                    phi_1324_ = _e761;
                }
                let _e765 = phi_1324_;
                phi_1325_ = _e765;
            }
            let _e770 = phi_1325_;
            phi_1327_ = _e770;
            break;
        }
        case 0: {
            if (_e687.x > 1f) {
                phi_4205_ = 0.9999999f;
            } else {
                phi_4205_ = select(_e687.x, 0.00000011920929f, (_e687.x < 0f));
            }
            let _e744 = phi_4205_;
            phi_1327_ = _e744;
            break;
        }
        default: {
            phi_1327_ = f32();
            break;
        }
    }
    let _e786 = phi_1327_;
    switch bitcast<i32>(_e735.member.member_1) {
        case 1: {
            let _e824 = abs(_e687.y);
            let _e826 = (_e824 % 1f);
            if (_e824 >= 1f) {
                phi_4283_ = select(true, false, (_e826 == 0f));
            } else {
                phi_4283_ = true;
            }
            let _e830 = phi_4283_;
            let _e831 = select(1f, _e826, _e830);
            if (select(-1f, 1f, (_e687.y >= 0f)) > 0f) {
                phi_1348_ = _e831;
            } else {
                phi_1348_ = (1f - _e831);
            }
            let _e835 = phi_1348_;
            phi_1385_ = _e835;
            break;
        }
        case 2: {
            let _e798 = abs(_e687.y);
            let _e805 = ((select(select(u32(_e798), 0u, (_e798 < 0f)), 4294967295u, (_e798 > 4294967000f)) % 2u) == 0u);
            let _e807 = (_e798 % 1f);
            if (_e798 >= 1f) {
                phi_4266_ = select(true, false, (_e807 == 0f));
            } else {
                phi_4266_ = true;
            }
            let _e811 = phi_4266_;
            let _e812 = select(1f, _e807, _e811);
            if (select(-1f, 1f, (_e687.y >= 0f)) > 0f) {
                if _e805 {
                    phi_1377_ = _e812;
                } else {
                    phi_1377_ = (1f - _e812);
                }
                let _e819 = phi_1377_;
                phi_1383_ = _e819;
            } else {
                if _e805 {
                    phi_1382_ = (1f - _e812);
                } else {
                    phi_1382_ = _e812;
                }
                let _e816 = phi_1382_;
                phi_1383_ = _e816;
            }
            let _e821 = phi_1383_;
            phi_1385_ = _e821;
            break;
        }
        case 0: {
            if (_e687.y > 1f) {
                phi_4253_ = 0.9999999f;
            } else {
                phi_4253_ = select(_e687.y, 0.00000011920929f, (_e687.y < 0f));
            }
            let _e795 = phi_4253_;
            phi_1385_ = _e795;
            break;
        }
        default: {
            phi_1385_ = f32();
            break;
        }
    }
    let _e837 = phi_1385_;
    let _e841 = (_e786 * f32(_e735.member_2.x));
    let _e850 = (_e837 * f32(_e735.member_2.y));
    let _e868 = vec3<f32>((f32((select(select(u32(_e841), 0u, (_e841 < 0f)), 4294967295u, (_e841 > 4294967000f)) + _e735.member_1.x)) / _e472), (f32((select(select(u32(_e850), 0u, (_e850 < 0f)), 4294967295u, (_e850 > 4294967000f)) + _e735.member_1.y)) / _e473), f32(_e735.member_3));
    let _e874 = textureSampleLevel(global_10, global_9, vec2<f32>(_e868.x, _e868.y), i32(_e868.z), 0f);
    let _e875 = (_e292.member_7 == 4294967295u);
    let _e877 = select(_e874, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e875));
    let _e881 = select(_e117, _e116, vec2((_e292.member_13 == 0u)));
    if _e298 {
        phi_4319_ = (_e292.member_8 <= (_e113 - 8u));
    } else {
        phi_4319_ = false;
    }
    let _e886 = phi_4319_;
    if _e886 {
        let _e889 = global.member[_e292.member_8];
        let _e893 = global.member[(_e292.member_8 + 1u)];
        let _e898 = global.member[(_e292.member_8 + 2u)];
        let _e902 = global.member[(_e292.member_8 + 3u)];
        let _e907 = global.member[(_e292.member_8 + 4u)];
        let _e911 = global.member[(_e292.member_8 + 5u)];
        let _e915 = global.member[(_e292.member_8 + 6u)];
        switch bitcast<i32>(_e915) {
            case 0: {
                phi_1468_ = 0u;
                break;
            }
            case 1: {
                phi_1468_ = 1u;
                break;
            }
            case 2: {
                phi_1468_ = 2u;
                break;
            }
            default: {
                phi_1468_ = 0u;
                break;
            }
        }
        let _e918 = phi_1468_;
        let _e922 = global.member[(_e292.member_8 + 7u)];
        switch bitcast<i32>(_e922) {
            case 0: {
                phi_1477_ = 0u;
                break;
            }
            case 1: {
                phi_1477_ = 1u;
                break;
            }
            case 2: {
                phi_1477_ = 2u;
                break;
            }
            default: {
                phi_1477_ = 0u;
                break;
            }
        }
        let _e925 = phi_1477_;
        phi_1490_ = type_32(type_24(_e918, _e925), vec2<u32>(_e889, _e893), vec2<u32>(_e898, _e902), _e907, _e911);
    } else {
        phi_1490_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e929 = phi_1490_;
    switch bitcast<i32>(_e929.member.member) {
        case 1: {
            let _e967 = abs(_e881.x);
            let _e969 = (_e967 % 1f);
            if (_e967 >= 1f) {
                phi_4370_ = select(true, false, (_e969 == 0f));
            } else {
                phi_4370_ = true;
            }
            let _e973 = phi_4370_;
            let _e974 = select(1f, _e969, _e973);
            if (select(-1f, 1f, (_e881.x >= 0f)) > 0f) {
                phi_1510_ = _e974;
            } else {
                phi_1510_ = (1f - _e974);
            }
            let _e978 = phi_1510_;
            phi_1547_ = _e978;
            break;
        }
        case 2: {
            let _e941 = abs(_e881.x);
            let _e948 = ((select(select(u32(_e941), 0u, (_e941 < 0f)), 4294967295u, (_e941 > 4294967000f)) % 2u) == 0u);
            let _e950 = (_e941 % 1f);
            if (_e941 >= 1f) {
                phi_4353_ = select(true, false, (_e950 == 0f));
            } else {
                phi_4353_ = true;
            }
            let _e954 = phi_4353_;
            let _e955 = select(1f, _e950, _e954);
            if (select(-1f, 1f, (_e881.x >= 0f)) > 0f) {
                if _e948 {
                    phi_1539_ = _e955;
                } else {
                    phi_1539_ = (1f - _e955);
                }
                let _e962 = phi_1539_;
                phi_1545_ = _e962;
            } else {
                if _e948 {
                    phi_1544_ = (1f - _e955);
                } else {
                    phi_1544_ = _e955;
                }
                let _e959 = phi_1544_;
                phi_1545_ = _e959;
            }
            let _e964 = phi_1545_;
            phi_1547_ = _e964;
            break;
        }
        case 0: {
            if (_e881.x > 1f) {
                phi_4340_ = 0.9999999f;
            } else {
                phi_4340_ = select(_e881.x, 0.00000011920929f, (_e881.x < 0f));
            }
            let _e938 = phi_4340_;
            phi_1547_ = _e938;
            break;
        }
        default: {
            phi_1547_ = f32();
            break;
        }
    }
    let _e980 = phi_1547_;
    switch bitcast<i32>(_e929.member.member_1) {
        case 1: {
            let _e1018 = abs(_e881.y);
            let _e1020 = (_e1018 % 1f);
            if (_e1018 >= 1f) {
                phi_4418_ = select(true, false, (_e1020 == 0f));
            } else {
                phi_4418_ = true;
            }
            let _e1024 = phi_4418_;
            let _e1025 = select(1f, _e1020, _e1024);
            if (select(-1f, 1f, (_e881.y >= 0f)) > 0f) {
                phi_1568_ = _e1025;
            } else {
                phi_1568_ = (1f - _e1025);
            }
            let _e1029 = phi_1568_;
            phi_1605_ = _e1029;
            break;
        }
        case 2: {
            let _e992 = abs(_e881.y);
            let _e999 = ((select(select(u32(_e992), 0u, (_e992 < 0f)), 4294967295u, (_e992 > 4294967000f)) % 2u) == 0u);
            let _e1001 = (_e992 % 1f);
            if (_e992 >= 1f) {
                phi_4401_ = select(true, false, (_e1001 == 0f));
            } else {
                phi_4401_ = true;
            }
            let _e1005 = phi_4401_;
            let _e1006 = select(1f, _e1001, _e1005);
            if (select(-1f, 1f, (_e881.y >= 0f)) > 0f) {
                if _e999 {
                    phi_1597_ = _e1006;
                } else {
                    phi_1597_ = (1f - _e1006);
                }
                let _e1013 = phi_1597_;
                phi_1603_ = _e1013;
            } else {
                if _e999 {
                    phi_1602_ = (1f - _e1006);
                } else {
                    phi_1602_ = _e1006;
                }
                let _e1010 = phi_1602_;
                phi_1603_ = _e1010;
            }
            let _e1015 = phi_1603_;
            phi_1605_ = _e1015;
            break;
        }
        case 0: {
            if (_e881.y > 1f) {
                phi_4388_ = 0.9999999f;
            } else {
                phi_4388_ = select(_e881.y, 0.00000011920929f, (_e881.y < 0f));
            }
            let _e989 = phi_4388_;
            phi_1605_ = _e989;
            break;
        }
        default: {
            phi_1605_ = f32();
            break;
        }
    }
    let _e1031 = phi_1605_;
    let _e1035 = (_e980 * f32(_e929.member_2.x));
    let _e1044 = (_e1031 * f32(_e929.member_2.y));
    let _e1062 = vec3<f32>((f32((select(select(u32(_e1035), 0u, (_e1035 < 0f)), 4294967295u, (_e1035 > 4294967000f)) + _e929.member_1.x)) / _e472), (f32((select(select(u32(_e1044), 0u, (_e1044 < 0f)), 4294967295u, (_e1044 > 4294967000f)) + _e929.member_1.y)) / _e473), f32(_e929.member_3));
    let _e1068 = textureSampleLevel(global_10, global_9, vec2<f32>(_e1062.x, _e1062.y), i32(_e1062.z), 0f);
    let _e1075 = select(_e117, _e116, vec2((_e292.member_14 == 0u)));
    if _e298 {
        phi_4454_ = (_e292.member_9 <= (_e113 - 8u));
    } else {
        phi_4454_ = false;
    }
    let _e1080 = phi_4454_;
    if _e1080 {
        let _e1083 = global.member[_e292.member_9];
        let _e1087 = global.member[(_e292.member_9 + 1u)];
        let _e1092 = global.member[(_e292.member_9 + 2u)];
        let _e1096 = global.member[(_e292.member_9 + 3u)];
        let _e1101 = global.member[(_e292.member_9 + 4u)];
        let _e1105 = global.member[(_e292.member_9 + 5u)];
        let _e1109 = global.member[(_e292.member_9 + 6u)];
        switch bitcast<i32>(_e1109) {
            case 0: {
                phi_1688_ = 0u;
                break;
            }
            case 1: {
                phi_1688_ = 1u;
                break;
            }
            case 2: {
                phi_1688_ = 2u;
                break;
            }
            default: {
                phi_1688_ = 0u;
                break;
            }
        }
        let _e1112 = phi_1688_;
        let _e1116 = global.member[(_e292.member_9 + 7u)];
        switch bitcast<i32>(_e1116) {
            case 0: {
                phi_1697_ = 0u;
                break;
            }
            case 1: {
                phi_1697_ = 1u;
                break;
            }
            case 2: {
                phi_1697_ = 2u;
                break;
            }
            default: {
                phi_1697_ = 0u;
                break;
            }
        }
        let _e1119 = phi_1697_;
        phi_1710_ = type_32(type_24(_e1112, _e1119), vec2<u32>(_e1083, _e1087), vec2<u32>(_e1092, _e1096), _e1101, _e1105);
    } else {
        phi_1710_ = type_32(type_24(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
    }
    let _e1123 = phi_1710_;
    switch bitcast<i32>(_e1123.member.member) {
        case 1: {
            let _e1161 = abs(_e1075.x);
            let _e1163 = (_e1161 % 1f);
            if (_e1161 >= 1f) {
                phi_4505_ = select(true, false, (_e1163 == 0f));
            } else {
                phi_4505_ = true;
            }
            let _e1167 = phi_4505_;
            let _e1168 = select(1f, _e1163, _e1167);
            if (select(-1f, 1f, (_e1075.x >= 0f)) > 0f) {
                phi_1730_ = _e1168;
            } else {
                phi_1730_ = (1f - _e1168);
            }
            let _e1172 = phi_1730_;
            phi_1767_ = _e1172;
            break;
        }
        case 2: {
            let _e1135 = abs(_e1075.x);
            let _e1142 = ((select(select(u32(_e1135), 0u, (_e1135 < 0f)), 4294967295u, (_e1135 > 4294967000f)) % 2u) == 0u);
            let _e1144 = (_e1135 % 1f);
            if (_e1135 >= 1f) {
                phi_4488_ = select(true, false, (_e1144 == 0f));
            } else {
                phi_4488_ = true;
            }
            let _e1148 = phi_4488_;
            let _e1149 = select(1f, _e1144, _e1148);
            if (select(-1f, 1f, (_e1075.x >= 0f)) > 0f) {
                if _e1142 {
                    phi_1759_ = _e1149;
                } else {
                    phi_1759_ = (1f - _e1149);
                }
                let _e1156 = phi_1759_;
                phi_1765_ = _e1156;
            } else {
                if _e1142 {
                    phi_1764_ = (1f - _e1149);
                } else {
                    phi_1764_ = _e1149;
                }
                let _e1153 = phi_1764_;
                phi_1765_ = _e1153;
            }
            let _e1158 = phi_1765_;
            phi_1767_ = _e1158;
            break;
        }
        case 0: {
            if (_e1075.x > 1f) {
                phi_4475_ = 0.9999999f;
            } else {
                phi_4475_ = select(_e1075.x, 0.00000011920929f, (_e1075.x < 0f));
            }
            let _e1132 = phi_4475_;
            phi_1767_ = _e1132;
            break;
        }
        default: {
            phi_1767_ = f32();
            break;
        }
    }
    let _e1174 = phi_1767_;
    switch bitcast<i32>(_e1123.member.member_1) {
        case 1: {
            let _e1212 = abs(_e1075.y);
            let _e1214 = (_e1212 % 1f);
            if (_e1212 >= 1f) {
                phi_4553_ = select(true, false, (_e1214 == 0f));
            } else {
                phi_4553_ = true;
            }
            let _e1218 = phi_4553_;
            let _e1219 = select(1f, _e1214, _e1218);
            if (select(-1f, 1f, (_e1075.y >= 0f)) > 0f) {
                phi_1788_ = _e1219;
            } else {
                phi_1788_ = (1f - _e1219);
            }
            let _e1223 = phi_1788_;
            phi_1825_ = _e1223;
            break;
        }
        case 2: {
            let _e1186 = abs(_e1075.y);
            let _e1193 = ((select(select(u32(_e1186), 0u, (_e1186 < 0f)), 4294967295u, (_e1186 > 4294967000f)) % 2u) == 0u);
            let _e1195 = (_e1186 % 1f);
            if (_e1186 >= 1f) {
                phi_4536_ = select(true, false, (_e1195 == 0f));
            } else {
                phi_4536_ = true;
            }
            let _e1199 = phi_4536_;
            let _e1200 = select(1f, _e1195, _e1199);
            if (select(-1f, 1f, (_e1075.y >= 0f)) > 0f) {
                if _e1193 {
                    phi_1817_ = _e1200;
                } else {
                    phi_1817_ = (1f - _e1200);
                }
                let _e1207 = phi_1817_;
                phi_1823_ = _e1207;
            } else {
                if _e1193 {
                    phi_1822_ = (1f - _e1200);
                } else {
                    phi_1822_ = _e1200;
                }
                let _e1204 = phi_1822_;
                phi_1823_ = _e1204;
            }
            let _e1209 = phi_1823_;
            phi_1825_ = _e1209;
            break;
        }
        case 0: {
            if (_e1075.y > 1f) {
                phi_4523_ = 0.9999999f;
            } else {
                phi_4523_ = select(_e1075.y, 0.00000011920929f, (_e1075.y < 0f));
            }
            let _e1183 = phi_4523_;
            phi_1825_ = _e1183;
            break;
        }
        default: {
            phi_1825_ = f32();
            break;
        }
    }
    let _e1225 = phi_1825_;
    let _e1229 = (_e1174 * f32(_e1123.member_2.x));
    let _e1238 = (_e1225 * f32(_e1123.member_2.y));
    let _e1256 = vec3<f32>((f32((select(select(u32(_e1229), 0u, (_e1229 < 0f)), 4294967295u, (_e1229 > 4294967000f)) + _e1123.member_1.x)) / _e472), (f32((select(select(u32(_e1238), 0u, (_e1238 < 0f)), 4294967295u, (_e1238 > 4294967000f)) + _e1123.member_1.y)) / _e473), f32(_e1123.member_3));
    let _e1262 = textureSampleLevel(global_10, global_9, vec2<f32>(_e1256.x, _e1256.y), i32(_e1256.z), 0f);
    let _e1265 = select(_e1262, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e292.member_9 == 4294967295u)));
    if _e875 {
        phi_1919_ = vec3<f32>(0f, 0f, 0f);
        phi_1920_ = _e118;
    } else {
        let _e1269 = fma(_e877.x, 2f, -1f);
        let _e1270 = fma(_e877.y, 2f, -1f);
        let _e1271 = fma(_e877.z, 2f, -1f);
        let _e1276 = sqrt(fma(_e1271, _e1271, fma(_e1269, _e1269, (_e1270 * _e1270))));
        if (_e1276 == 0f) {
            phi_4611_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4611_ = (vec3<f32>(_e1269, _e1270, _e1271) * (1f / _e1276));
        }
        let _e1281 = phi_4611_;
        let _e1288 = sqrt(fma(_e119.z, _e119.z, fma(_e119.x, _e119.x, (_e119.y * _e119.y))));
        if (_e1288 == 0f) {
            phi_4646_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4646_ = (_e119 * (1f / _e1288));
        }
        let _e1293 = phi_4646_;
        let _e1300 = sqrt(fma(_e120.z, _e120.z, fma(_e120.x, _e120.x, (_e120.y * _e120.y))));
        if (_e1300 == 0f) {
            phi_4681_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4681_ = (_e120 * (1f / _e1300));
        }
        let _e1305 = phi_4681_;
        let _e1312 = sqrt(fma(_e118.z, _e118.z, fma(_e118.x, _e118.x, (_e118.y * _e118.y))));
        if (_e1312 == 0f) {
            phi_4716_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4716_ = (_e118 * (1f / _e1312));
        }
        let _e1317 = phi_4716_;
        let _e1336 = fma(_e1317.x, _e1281.z, fma(_e1293.x, _e1281.x, (_e1305.x * _e1281.y)));
        let _e1337 = fma(_e1317.y, _e1281.z, fma(_e1293.y, _e1281.x, (_e1305.y * _e1281.y)));
        let _e1338 = fma(_e1317.z, _e1281.z, fma(_e1293.z, _e1281.x, (_e1305.z * _e1281.y)));
        let _e1343 = sqrt(fma(_e1338, _e1338, fma(_e1336, _e1336, (_e1337 * _e1337))));
        if (_e1343 == 0f) {
            phi_4751_ = vec3<f32>(0f, 0f, 0f);
        } else {
            phi_4751_ = (vec3<f32>(_e1336, _e1337, _e1338) * (1f / _e1343));
        }
        let _e1348 = phi_4751_;
        phi_1919_ = _e1281;
        phi_1920_ = _e1348;
    }
    let _e1350 = phi_1919_;
    let _e1352 = phi_1920_;
    let _e1356 = (_e489.x * _e292.member_2.x);
    let _e1359 = (_e489.y * _e292.member_2.y);
    let _e1362 = (_e489.z * _e292.member_2.z);
    let _e1367 = (_e1356 * _e115.x);
    let _e1369 = (_e1359 * _e115.y);
    let _e1371 = (_e1362 * _e115.z);
    let _e1376 = (_e683.y * _e292.member_4);
    let _e1379 = (_e683.z * _e292.member_3);
    let _e1383 = fma(_e292.member_16, (select(_e1068, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e292.member_8 == 4294967295u))).x - 1f), 1f);
    let _e1389 = (_e1265.x * _e292.member.x);
    let _e1391 = (_e1265.y * _e292.member.y);
    let _e1393 = (_e1265.z * _e292.member.z);
    let _e1398 = textureSampleLevel(global_11, global_12, _e1352, 0f);
    if (_e113 >= 86u) {
        phi_4783_ = (_e125 <= (_e113 - 86u));
    } else {
        phi_4783_ = false;
    }
    let _e1406 = phi_4783_;
    if _e1406 {
        let _e1409 = global.member[_e125];
        let _e1414 = global.member[(_e125 + 1u)];
        let _e1419 = global.member[(_e125 + 2u)];
        let _e1424 = global.member[(_e125 + 3u)];
        let _e1430 = global.member[(_e125 + 4u)];
        let _e1435 = global.member[(_e125 + 5u)];
        let _e1440 = global.member[(_e125 + 6u)];
        let _e1445 = global.member[(_e125 + 7u)];
        let _e1451 = global.member[(_e125 + 8u)];
        let _e1456 = global.member[(_e125 + 9u)];
        let _e1461 = global.member[(_e125 + 10u)];
        let _e1466 = global.member[(_e125 + 11u)];
        let _e1472 = global.member[(_e125 + 12u)];
        let _e1477 = global.member[(_e125 + 13u)];
        let _e1482 = global.member[(_e125 + 14u)];
        let _e1487 = global.member[(_e125 + 15u)];
        let _e1494 = global.member[(_e125 + 16u)];
        let _e1499 = global.member[(_e125 + 17u)];
        let _e1504 = global.member[(_e125 + 18u)];
        let _e1509 = global.member[(_e125 + 19u)];
        let _e1515 = global.member[(_e125 + 20u)];
        let _e1520 = global.member[(_e125 + 21u)];
        let _e1525 = global.member[(_e125 + 22u)];
        let _e1530 = global.member[(_e125 + 23u)];
        let _e1536 = global.member[(_e125 + 24u)];
        let _e1541 = global.member[(_e125 + 25u)];
        let _e1546 = global.member[(_e125 + 26u)];
        let _e1551 = global.member[(_e125 + 27u)];
        let _e1557 = global.member[(_e125 + 28u)];
        let _e1562 = global.member[(_e125 + 29u)];
        let _e1567 = global.member[(_e125 + 30u)];
        let _e1572 = global.member[(_e125 + 31u)];
        let _e1579 = global.member[(_e125 + 32u)];
        let _e1584 = global.member[(_e125 + 33u)];
        let _e1589 = global.member[(_e125 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_2127_ = type_24(0u, 6u);
        loop {
            let _e1594 = phi_2127_;
            if (_e1594.member < _e1594.member_1) {
                phi_2128_ = type_24((_e1594.member + 1u), _e1594.member_1);
                phi_2151_ = type_24(1u, _e1594.member);
            } else {
                phi_2128_ = _e1594;
                phi_2151_ = type_24(0u, type_24().member_1);
            }
            let _e1607 = phi_2128_;
            let _e1609 = phi_2151_;
            switch bitcast<i32>(_e1609.member) {
                case 0: {
                    phi_2178_ = false;
                    break;
                }
                case 1: {
                    let _e1614 = ((_e125 + 35u) + (_e1609.member_1 * 4u));
                    let _e1617 = global.member[_e1614];
                    let _e1622 = global.member[(_e1614 + 1u)];
                    let _e1627 = global.member[(_e1614 + 2u)];
                    let _e1632 = global.member[(_e1614 + 3u)];
                    local_1[_e1609.member_1] = vec4<f32>(bitcast<f32>(_e1617), bitcast<f32>(_e1622), bitcast<f32>(_e1627), bitcast<f32>(_e1632));
                    phi_2178_ = true;
                    break;
                }
                default: {
                    phi_2178_ = bool();
                    break;
                }
            }
            let _e1637 = phi_2178_;
            continue;
            continuing {
                phi_2127_ = _e1607;
                break if !(_e1637);
            }
        }
        let _e1639 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_2184_ = type_24(0u, 8u);
        loop {
            let _e1642 = phi_2184_;
            if (_e1642.member < _e1642.member_1) {
                phi_2185_ = type_24((_e1642.member + 1u), _e1642.member_1);
                phi_2208_ = type_24(1u, _e1642.member);
            } else {
                phi_2185_ = _e1642;
                phi_2208_ = type_24(0u, type_24().member_1);
            }
            let _e1655 = phi_2185_;
            let _e1657 = phi_2208_;
            switch bitcast<i32>(_e1657.member) {
                case 0: {
                    phi_2231_ = false;
                    break;
                }
                case 1: {
                    let _e1662 = ((_e125 + 59u) + (_e1657.member_1 * 3u));
                    let _e1665 = global.member[_e1662];
                    let _e1670 = global.member[(_e1662 + 1u)];
                    let _e1675 = global.member[(_e1662 + 2u)];
                    local[_e1657.member_1] = vec3<f32>(bitcast<f32>(_e1665), bitcast<f32>(_e1670), bitcast<f32>(_e1675));
                    phi_2231_ = true;
                    break;
                }
                default: {
                    phi_2231_ = bool();
                    break;
                }
            }
            let _e1680 = phi_2231_;
            continue;
            continuing {
                phi_2184_ = _e1655;
                break if !(_e1680);
            }
        }
        let _e1682 = local;
        let _e1686 = global.member[(_e125 + 83u)];
        let _e1691 = global.member[(_e125 + 84u)];
        let _e1696 = global.member[(_e125 + 85u)];
        phi_2252_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e1409), bitcast<f32>(_e1414), bitcast<f32>(_e1419), bitcast<f32>(_e1424)), vec4<f32>(bitcast<f32>(_e1430), bitcast<f32>(_e1435), bitcast<f32>(_e1440), bitcast<f32>(_e1445)), vec4<f32>(bitcast<f32>(_e1451), bitcast<f32>(_e1456), bitcast<f32>(_e1461), bitcast<f32>(_e1466)), vec4<f32>(bitcast<f32>(_e1472), bitcast<f32>(_e1477), bitcast<f32>(_e1482), bitcast<f32>(_e1487))), type_20(vec4<f32>(bitcast<f32>(_e1494), bitcast<f32>(_e1499), bitcast<f32>(_e1504), bitcast<f32>(_e1509)), vec4<f32>(bitcast<f32>(_e1515), bitcast<f32>(_e1520), bitcast<f32>(_e1525), bitcast<f32>(_e1530)), vec4<f32>(bitcast<f32>(_e1536), bitcast<f32>(_e1541), bitcast<f32>(_e1546), bitcast<f32>(_e1551)), vec4<f32>(bitcast<f32>(_e1557), bitcast<f32>(_e1562), bitcast<f32>(_e1567), bitcast<f32>(_e1572))), vec3<f32>(bitcast<f32>(_e1579), bitcast<f32>(_e1584), bitcast<f32>(_e1589)), type_21(_e1682, _e1639, vec3<f32>(bitcast<f32>(_e1686), bitcast<f32>(_e1691), bitcast<f32>(_e1696))));
    } else {
        phi_2252_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
    }
    let _e1702 = phi_2252_;
    let _e1704 = (_e1702.member_2 - _e121);
    let _e1711 = sqrt(fma(_e1704.z, _e1704.z, fma(_e1704.x, _e1704.x, (_e1704.y * _e1704.y))));
    let _e1712 = (_e1711 == 0f);
    if _e1712 {
        phi_4855_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4855_ = (_e1704 * (1f / _e1711));
    }
    let _e1716 = phi_4855_;
    let _e1717 = -(_e1716);
    let _e1724 = sqrt(fma(_e1352.z, _e1352.z, fma(_e1352.x, _e1352.x, (_e1352.y * _e1352.y))));
    let _e1725 = (_e1724 == 0f);
    if _e1725 {
        phi_4914_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4914_ = (_e1352 * (1f / _e1724));
    }
    let _e1729 = phi_4914_;
    let _e1739 = (2f * fma(_e1729.z, _e1717.z, fma(_e1729.x, _e1717.x, (_e1729.y * _e1717.y))));
    let _e1746 = textureSampleLevel(global_13, global_14, (_e1717 - vec3<f32>((_e1739 * _e1729.x), (_e1739 * _e1729.y), (_e1739 * _e1729.z))), (_e1376 * 4f));
    if _e1712 {
        phi_4988_ = vec3<f32>(0f, 0f, 0f);
    } else {
        phi_4988_ = (_e1704 * (1f / _e1711));
    }
    let _e1753 = phi_4988_;
    let _e1762 = textureSampleLevel(global_15, global_16, vec2<f32>(max(fma(_e1352.z, _e1753.z, fma(_e1352.x, _e1753.x, (_e1352.y * _e1753.y))), 0f), _e1376), 0f);
    switch bitcast<i32>(_e147) {
        case 0: {
            if _e292.member_15 {
                if _e1725 {
                    phi_5381_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5381_ = (_e1352 * (1f / _e1724));
                }
                let _e1931 = phi_5381_;
                if _e1712 {
                    phi_5416_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5416_ = (_e1704 * (1f / _e1711));
                }
                let _e1935 = phi_5416_;
                phi_2292_ = type_24(0u, _e160);
                phi_2295_ = vec3<f32>(0f, 0f, 0f);
                loop {
                    let _e1938 = phi_2292_;
                    let _e1940 = phi_2295_;
                    local_2 = _e1940;
                    local_3 = _e1940;
                    local_4 = _e1940;
                    if (_e1938.member < _e1938.member_1) {
                        phi_2293_ = type_24((_e1938.member + 1u), _e1938.member_1);
                        phi_2318_ = type_24(1u, _e1938.member);
                    } else {
                        phi_2293_ = _e1938;
                        phi_2318_ = type_24(0u, type_24().member_1);
                    }
                    let _e1953 = phi_2293_;
                    let _e1955 = phi_2318_;
                    switch bitcast<i32>(_e1955.member) {
                        case 0: {
                            phi_2296_ = vec3<f32>();
                            phi_3189_ = false;
                            break;
                        }
                        case 1: {
                            if (_e1955.member_1 >= _e160) {
                                phi_5433_ = 4294967295u;
                            } else {
                                phi_5433_ = (_e156 + _e1955.member_1);
                            }
                            let _e1962 = phi_5433_;
                            if (_e113 >= 1u) {
                                phi_5452_ = (_e1962 <= (_e113 - 1u));
                            } else {
                                phi_5452_ = false;
                            }
                            let _e1967 = phi_5452_;
                            if _e1967 {
                                let _e1970 = global.member[_e1962];
                                phi_2335_ = _e1970;
                            } else {
                                phi_2335_ = 4294967295u;
                            }
                            let _e1972 = phi_2335_;
                            let _e1973 = (_e1972 == 4294967295u);
                            if _e1973 {
                                phi_3187_ = vec3<f32>();
                            } else {
                                if (_e113 >= 3u) {
                                    phi_5484_ = (_e1972 <= (_e113 - 3u));
                                } else {
                                    phi_5484_ = false;
                                }
                                let _e1978 = phi_5484_;
                                if _e1978 {
                                    let _e1981 = global.member[_e1972];
                                    switch bitcast<i32>(_e1981) {
                                        case 0: {
                                            phi_2352_ = 0u;
                                            break;
                                        }
                                        case 1: {
                                            phi_2352_ = 1u;
                                            break;
                                        }
                                        case 2: {
                                            phi_2352_ = 2u;
                                            break;
                                        }
                                        default: {
                                            phi_2352_ = 0u;
                                            break;
                                        }
                                    }
                                    let _e1984 = phi_2352_;
                                    let _e1988 = global.member[(_e1972 + 1u)];
                                    let _e1992 = global.member[(_e1972 + 2u)];
                                    phi_2362_ = type_33(_e1984, _e1988, _e1992);
                                } else {
                                    phi_2362_ = type_33(0u, 4294967295u, 4294967295u);
                                }
                                let _e1995 = phi_2362_;
                                if (_e113 >= 10u) {
                                    phi_5514_ = (_e1995.member_2 <= (_e113 - 10u));
                                } else {
                                    phi_5514_ = false;
                                }
                                let _e2001 = phi_5514_;
                                if _e2001 {
                                    let _e2004 = global.member[_e1995.member_2];
                                    let _e2009 = global.member[(_e1995.member_2 + 1u)];
                                    let _e2014 = global.member[(_e1995.member_2 + 2u)];
                                    let _e2020 = global.member[(_e1995.member_2 + 3u)];
                                    let _e2025 = global.member[(_e1995.member_2 + 4u)];
                                    let _e2030 = global.member[(_e1995.member_2 + 5u)];
                                    let _e2035 = global.member[(_e1995.member_2 + 6u)];
                                    let _e2041 = global.member[(_e1995.member_2 + 7u)];
                                    let _e2046 = global.member[(_e1995.member_2 + 8u)];
                                    let _e2051 = global.member[(_e1995.member_2 + 9u)];
                                    phi_2412_ = type_29(vec3<f32>(bitcast<f32>(_e2004), bitcast<f32>(_e2009), bitcast<f32>(_e2014)), vec4<f32>(bitcast<f32>(_e2020), bitcast<f32>(_e2025), bitcast<f32>(_e2030), bitcast<f32>(_e2035)), vec3<f32>(bitcast<f32>(_e2041), bitcast<f32>(_e2046), bitcast<f32>(_e2051)));
                                } else {
                                    phi_2412_ = type_29(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e2056 = phi_2412_;
                                let _e2064 = (_e2056.member_1.x + _e2056.member_1.x);
                                let _e2065 = (_e2056.member_1.y + _e2056.member_1.y);
                                let _e2066 = (_e2056.member_1.z + _e2056.member_1.z);
                                let _e2068 = (_e2056.member_1.z * _e2066);
                                let _e2069 = (_e2056.member_1.w * _e2064);
                                let _e2070 = (_e2056.member_1.w * _e2065);
                                let _e2071 = (_e2056.member_1.w * _e2066);
                                let _e2091 = (vec4<f32>((1f - fma(_e2056.member_1.y, _e2065, _e2068)), fma(_e2056.member_1.x, _e2065, _e2071), fma(_e2056.member_1.x, _e2066, -(_e2070)), 0f) * _e2056.member_2.x);
                                let _e2093 = (vec4<f32>(fma(_e2056.member_1.x, _e2065, -(_e2071)), (1f - fma(_e2056.member_1.x, _e2064, _e2068)), fma(_e2056.member_1.y, _e2066, _e2069), 0f) * _e2056.member_2.y);
                                let _e2095 = (vec4<f32>(fma(_e2056.member_1.x, _e2066, _e2070), fma(_e2056.member_1.y, _e2066, -(_e2069)), (1f - fma(_e2056.member_1.x, _e2064, (_e2056.member_1.y * _e2065))), 0f) * _e2056.member_2.z);
                                switch bitcast<i32>(_e1995.member) {
                                    case 0: {
                                        if _e298 {
                                            phi_5980_ = (_e1995.member_1 <= (_e113 - 8u));
                                        } else {
                                            phi_5980_ = false;
                                        }
                                        let _e2591 = phi_5980_;
                                        if _e2591 {
                                            let _e2594 = global.member[_e1995.member_1];
                                            let _e2599 = global.member[(_e1995.member_1 + 1u)];
                                            let _e2604 = global.member[(_e1995.member_1 + 2u)];
                                            let _e2610 = global.member[(_e1995.member_1 + 3u)];
                                            let _e2615 = global.member[(_e1995.member_1 + 4u)];
                                            let _e2620 = global.member[(_e1995.member_1 + 5u)];
                                            let _e2625 = global.member[(_e1995.member_1 + 6u)];
                                            let _e2631 = global.member[(_e1995.member_1 + 7u)];
                                            phi_2460_ = type_34(vec3<f32>(bitcast<f32>(_e2594), bitcast<f32>(_e2599), bitcast<f32>(_e2604)), vec4<f32>(bitcast<f32>(_e2610), bitcast<f32>(_e2615), bitcast<f32>(_e2620), bitcast<f32>(_e2625)), bitcast<f32>(_e2631));
                                        } else {
                                            phi_2460_ = type_34(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2635 = phi_2460_;
                                        let _e2657 = fma(_e2095.x, _e2635.member.z, fma(_e2093.x, _e2635.member.y, (_e2091.x * _e2635.member.x)));
                                        let _e2658 = fma(_e2095.y, _e2635.member.z, fma(_e2093.y, _e2635.member.y, (_e2091.y * _e2635.member.x)));
                                        let _e2659 = fma(_e2095.z, _e2635.member.z, fma(_e2093.z, _e2635.member.y, (_e2091.z * _e2635.member.x)));
                                        let _e2664 = sqrt(fma(_e2659, _e2659, fma(_e2657, _e2657, (_e2658 * _e2658))));
                                        if (_e2664 == 0f) {
                                            phi_6027_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6027_ = (vec3<f32>(_e2657, _e2658, _e2659) * (1f / _e2664));
                                        }
                                        let _e2669 = phi_6027_;
                                        let _e2671 = -(_e2669.x);
                                        let _e2673 = -(_e2669.y);
                                        let _e2675 = -(_e2669.z);
                                        let _e2676 = -(_e2669);
                                        let _e2678 = fma(-(_e683.z), _e292.member_3, 1f);
                                        let _e2682 = fma(0.4f, _e2678, (_e1367 * _e1379));
                                        let _e2683 = fma(0.4f, _e2678, (_e1369 * _e1379));
                                        let _e2684 = fma(0.4f, _e2678, (_e1371 * _e1379));
                                        let _e2692 = (_e1935 + vec3<f32>(_e2671, _e2673, _e2675));
                                        let _e2699 = sqrt(fma(_e2692.z, _e2692.z, fma(_e2692.x, _e2692.x, (_e2692.y * _e2692.y))));
                                        if (_e2699 == 0f) {
                                            phi_6062_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_6062_ = (_e2692 * (1f / _e2699));
                                        }
                                        let _e2704 = phi_6062_;
                                        let _e2705 = (_e1376 * _e1376);
                                        let _e2716 = max(fma(_e1931.z, _e2704.z, fma(_e1931.x, _e2704.x, (_e1931.y * _e2704.y))), 0f);
                                        let _e2729 = max(fma(_e1931.z, _e1935.z, fma(_e1931.x, _e1935.x, (_e1931.y * _e1935.y))), 0f);
                                        let _e2736 = max(fma(_e1931.z, _e2676.z, fma(_e1931.x, _e2676.x, (_e1931.y * _e2676.y))), 0f);
                                        let _e2737 = fma(_e683.y, _e292.member_4, 1f);
                                        let _e2738 = (_e2737 * _e2737);
                                        let _e2739 = (_e2738 * 0.125f);
                                        let _e2741 = fma(-(_e2738), 0.125f, 1f);
                                        let _e2754 = (1f - max(fma(_e2704.z, _e1935.z, fma(_e2704.x, _e1935.x, (_e2704.y * _e1935.y))), 0f));
                                        let _e2756 = select(_e2754, 0f, (_e2754 < 0f));
                                        let _e2759 = pow(select(_e2756, 1f, (_e2756 > 1f)), 5f);
                                        let _e2760 = fma((1f - _e2682), _e2759, _e2682);
                                        let _e2761 = fma((1f - _e2683), _e2759, _e2683);
                                        let _e2762 = fma((1f - _e2684), _e2759, _e2684);
                                        let _e2769 = (((_e2705 * _e2705) / (pow(fma((_e2716 * _e2716), fma(_e2705, _e2705, -1f), 1f), 2f) * 3.1415927f)) * ((_e2729 / fma(_e2729, _e2741, _e2739)) * (_e2736 / fma(_e2736, _e2741, _e2739))));
                                        let _e2776 = max(fma(_e1931.z, _e2675, fma(_e1931.x, _e2671, (_e1931.y * _e2673))), 0f);
                                        let _e2778 = fma((4f * _e2729), _e2776, 0.0001f);
                                        phi_3177_ = vec3<f32>(fma((fma((((1f - _e2760) * _e2678) * _e1367), 0.31830987f, ((_e2769 * _e2760) / _e2778)) * (_e2635.member_1.x * _e2635.member_2)), _e2776, _e1940.x), fma((fma((((1f - _e2761) * _e2678) * _e1369), 0.31830987f, ((_e2769 * _e2761) / _e2778)) * (_e2635.member_1.y * _e2635.member_2)), _e2776, _e1940.y), fma((fma((((1f - _e2762) * _e2678) * _e1371), 0.31830987f, ((_e2769 * _e2762) / _e2778)) * (_e2635.member_1.z * _e2635.member_2)), _e2776, _e1940.z));
                                        phi_3178_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if _e298 {
                                            phi_5806_ = (_e1995.member_1 <= (_e113 - 8u));
                                        } else {
                                            phi_5806_ = false;
                                        }
                                        let _e2380 = phi_5806_;
                                        if _e2380 {
                                            let _e2383 = global.member[_e1995.member_1];
                                            let _e2388 = global.member[(_e1995.member_1 + 1u)];
                                            let _e2393 = global.member[(_e1995.member_1 + 2u)];
                                            let _e2399 = global.member[(_e1995.member_1 + 3u)];
                                            let _e2404 = global.member[(_e1995.member_1 + 4u)];
                                            let _e2409 = global.member[(_e1995.member_1 + 5u)];
                                            let _e2414 = global.member[(_e1995.member_1 + 6u)];
                                            let _e2420 = global.member[(_e1995.member_1 + 7u)];
                                            phi_2667_ = type_34(vec3<f32>(bitcast<f32>(_e2383), bitcast<f32>(_e2388), bitcast<f32>(_e2393)), vec4<f32>(bitcast<f32>(_e2399), bitcast<f32>(_e2404), bitcast<f32>(_e2409), bitcast<f32>(_e2414)), bitcast<f32>(_e2420));
                                        } else {
                                            phi_2667_ = type_34(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2424 = phi_2667_;
                                        let _e2453 = (vec3<f32>((_e2056.member.x + fma(_e2095.x, _e2424.member.z, fma(_e2093.x, _e2424.member.y, (_e2091.x * _e2424.member.x)))), (_e2056.member.y + fma(_e2095.y, _e2424.member.z, fma(_e2093.y, _e2424.member.y, (_e2091.y * _e2424.member.x)))), (_e2056.member.z + fma(_e2095.z, _e2424.member.z, fma(_e2093.z, _e2424.member.y, (_e2091.z * _e2424.member.x))))) - _e121);
                                        let _e2460 = sqrt(fma(_e2453.z, _e2453.z, fma(_e2453.x, _e2453.x, (_e2453.y * _e2453.y))));
                                        let _e2461 = (_e2460 == 0f);
                                        if _e2461 {
                                            phi_2857_ = vec3<f32>();
                                        } else {
                                            if _e2461 {
                                                phi_5853_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5853_ = (_e2453 * (1f / _e2460));
                                            }
                                            let _e2465 = phi_5853_;
                                            let _e2467 = (_e2424.member_2 / (_e2460 * _e2460));
                                            let _e2469 = fma(-(_e683.z), _e292.member_3, 1f);
                                            let _e2473 = fma(0.4f, _e2469, (_e1367 * _e1379));
                                            let _e2474 = fma(0.4f, _e2469, (_e1369 * _e1379));
                                            let _e2475 = fma(0.4f, _e2469, (_e1371 * _e1379));
                                            let _e2482 = (_e1935 + _e2465);
                                            let _e2489 = sqrt(fma(_e2482.z, _e2482.z, fma(_e2482.x, _e2482.x, (_e2482.y * _e2482.y))));
                                            if (_e2489 == 0f) {
                                                phi_5888_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5888_ = (_e2482 * (1f / _e2489));
                                            }
                                            let _e2494 = phi_5888_;
                                            let _e2495 = (_e1376 * _e1376);
                                            let _e2506 = max(fma(_e1931.z, _e2494.z, fma(_e1931.x, _e2494.x, (_e1931.y * _e2494.y))), 0f);
                                            let _e2519 = max(fma(_e1931.z, _e1935.z, fma(_e1931.x, _e1935.x, (_e1931.y * _e1935.y))), 0f);
                                            let _e2526 = max(fma(_e1931.z, _e2465.z, fma(_e1931.x, _e2465.x, (_e1931.y * _e2465.y))), 0f);
                                            let _e2527 = fma(_e683.y, _e292.member_4, 1f);
                                            let _e2528 = (_e2527 * _e2527);
                                            let _e2529 = (_e2528 * 0.125f);
                                            let _e2531 = fma(-(_e2528), 0.125f, 1f);
                                            let _e2544 = (1f - max(fma(_e2494.z, _e1935.z, fma(_e2494.x, _e1935.x, (_e2494.y * _e1935.y))), 0f));
                                            let _e2546 = select(_e2544, 0f, (_e2544 < 0f));
                                            let _e2549 = pow(select(_e2546, 1f, (_e2546 > 1f)), 5f);
                                            let _e2550 = fma((1f - _e2473), _e2549, _e2473);
                                            let _e2551 = fma((1f - _e2474), _e2549, _e2474);
                                            let _e2552 = fma((1f - _e2475), _e2549, _e2475);
                                            let _e2559 = (((_e2495 * _e2495) / (pow(fma((_e2506 * _e2506), fma(_e2495, _e2495, -1f), 1f), 2f) * 3.1415927f)) * ((_e2519 / fma(_e2519, _e2531, _e2529)) * (_e2526 / fma(_e2526, _e2531, _e2529))));
                                            let _e2564 = fma((4f * _e2519), _e2526, 0.0001f);
                                            phi_2857_ = vec3<f32>(fma((fma((((1f - _e2550) * _e2469) * _e1367), 0.31830987f, ((_e2559 * _e2550) / _e2564)) * (_e2424.member_1.x * _e2467)), _e2526, _e1940.x), fma((fma((((1f - _e2551) * _e2469) * _e1369), 0.31830987f, ((_e2559 * _e2551) / _e2564)) * (_e2424.member_1.y * _e2467)), _e2526, _e1940.y), fma((fma((((1f - _e2552) * _e2469) * _e1371), 0.31830987f, ((_e2559 * _e2552) / _e2564)) * (_e2424.member_1.z * _e2467)), _e2526, _e1940.z));
                                        }
                                        let _e2585 = phi_2857_;
                                        phi_3177_ = _e2585;
                                        phi_3178_ = select(true, false, _e2461);
                                        break;
                                    }
                                    case 2: {
                                        if (_e113 >= 13u) {
                                            phi_5594_ = (_e1995.member_1 <= (_e113 - 13u));
                                        } else {
                                            phi_5594_ = false;
                                        }
                                        let _e2106 = phi_5594_;
                                        if _e2106 {
                                            let _e2109 = global.member[_e1995.member_1];
                                            let _e2114 = global.member[(_e1995.member_1 + 1u)];
                                            let _e2119 = global.member[(_e1995.member_1 + 2u)];
                                            let _e2125 = global.member[(_e1995.member_1 + 3u)];
                                            let _e2130 = global.member[(_e1995.member_1 + 4u)];
                                            let _e2135 = global.member[(_e1995.member_1 + 5u)];
                                            let _e2141 = global.member[(_e1995.member_1 + 6u)];
                                            let _e2146 = global.member[(_e1995.member_1 + 7u)];
                                            let _e2151 = global.member[(_e1995.member_1 + 8u)];
                                            let _e2156 = global.member[(_e1995.member_1 + 9u)];
                                            let _e2161 = global.member[(_e1995.member_1 + 10u)];
                                            let _e2166 = global.member[(_e1995.member_1 + 11u)];
                                            let _e2172 = global.member[(_e1995.member_1 + 12u)];
                                            phi_2920_ = type_35(vec3<f32>(bitcast<f32>(_e2109), bitcast<f32>(_e2114), bitcast<f32>(_e2119)), vec3<f32>(bitcast<f32>(_e2125), bitcast<f32>(_e2130), bitcast<f32>(_e2135)), bitcast<f32>(_e2141), bitcast<f32>(_e2146), vec4<f32>(bitcast<f32>(_e2151), bitcast<f32>(_e2156), bitcast<f32>(_e2161), bitcast<f32>(_e2166)), bitcast<f32>(_e2172));
                                        } else {
                                            phi_2920_ = type_35(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e2176 = phi_2920_;
                                        let _e2208 = (vec3<f32>((_e2056.member.x + fma(_e2095.x, _e2176.member.z, fma(_e2093.x, _e2176.member.y, (_e2091.x * _e2176.member.x)))), (_e2056.member.y + fma(_e2095.y, _e2176.member.z, fma(_e2093.y, _e2176.member.y, (_e2091.y * _e2176.member.x)))), (_e2056.member.z + fma(_e2095.z, _e2176.member.z, fma(_e2093.z, _e2176.member.y, (_e2091.z * _e2176.member.x))))) - _e121);
                                        let _e2215 = sqrt(fma(_e2208.z, _e2208.z, fma(_e2208.x, _e2208.x, (_e2208.y * _e2208.y))));
                                        let _e2216 = (_e2215 == 0f);
                                        if _e2216 {
                                            phi_3175_ = vec3<f32>();
                                        } else {
                                            if _e2216 {
                                                phi_5644_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5644_ = (_e2208 * (1f / _e2215));
                                            }
                                            let _e2220 = phi_5644_;
                                            let _e2230 = fma(_e2095.x, _e2176.member_1.z, fma(_e2093.x, _e2176.member_1.y, (_e2091.x * _e2176.member_1.x)));
                                            let _e2231 = fma(_e2095.y, _e2176.member_1.z, fma(_e2093.y, _e2176.member_1.y, (_e2091.y * _e2176.member_1.x)));
                                            let _e2232 = fma(_e2095.z, _e2176.member_1.z, fma(_e2093.z, _e2176.member_1.y, (_e2091.z * _e2176.member_1.x)));
                                            let _e2237 = sqrt(fma(_e2232, _e2232, fma(_e2230, _e2230, (_e2231 * _e2231))));
                                            if (_e2237 == 0f) {
                                                phi_5679_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5679_ = (vec3<f32>(_e2230, _e2231, _e2232) * (1f / _e2237));
                                            }
                                            let _e2242 = phi_5679_;
                                            let _e2254 = ((fma(_e2220.z, _e2242.z, fma(_e2220.x, _e2242.x, (_e2220.y * _e2242.y))) - _e2176.member_3) / (_e2176.member_2 - _e2176.member_3));
                                            let _e2256 = select(_e2254, 0f, (_e2254 < 0f));
                                            let _e2259 = (_e2176.member_5 * select(_e2256, 1f, (_e2256 > 1f)));
                                            let _e2261 = fma(-(_e683.z), _e292.member_3, 1f);
                                            let _e2265 = fma(0.4f, _e2261, (_e1367 * _e1379));
                                            let _e2266 = fma(0.4f, _e2261, (_e1369 * _e1379));
                                            let _e2267 = fma(0.4f, _e2261, (_e1371 * _e1379));
                                            let _e2274 = (_e1935 + _e2220);
                                            let _e2281 = sqrt(fma(_e2274.z, _e2274.z, fma(_e2274.x, _e2274.x, (_e2274.y * _e2274.y))));
                                            if (_e2281 == 0f) {
                                                phi_5714_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_5714_ = (_e2274 * (1f / _e2281));
                                            }
                                            let _e2286 = phi_5714_;
                                            let _e2287 = (_e1376 * _e1376);
                                            let _e2298 = max(fma(_e1931.z, _e2286.z, fma(_e1931.x, _e2286.x, (_e1931.y * _e2286.y))), 0f);
                                            let _e2311 = max(fma(_e1931.z, _e1935.z, fma(_e1931.x, _e1935.x, (_e1931.y * _e1935.y))), 0f);
                                            let _e2315 = max(fma(_e1931.z, _e2220.z, fma(_e1931.x, _e2220.x, (_e1931.y * _e2220.y))), 0f);
                                            let _e2316 = fma(_e683.y, _e292.member_4, 1f);
                                            let _e2317 = (_e2316 * _e2316);
                                            let _e2318 = (_e2317 * 0.125f);
                                            let _e2320 = fma(-(_e2317), 0.125f, 1f);
                                            let _e2333 = (1f - max(fma(_e2286.z, _e1935.z, fma(_e2286.x, _e1935.x, (_e2286.y * _e1935.y))), 0f));
                                            let _e2335 = select(_e2333, 0f, (_e2333 < 0f));
                                            let _e2338 = pow(select(_e2335, 1f, (_e2335 > 1f)), 5f);
                                            let _e2339 = fma((1f - _e2265), _e2338, _e2265);
                                            let _e2340 = fma((1f - _e2266), _e2338, _e2266);
                                            let _e2341 = fma((1f - _e2267), _e2338, _e2267);
                                            let _e2348 = (((_e2287 * _e2287) / (pow(fma((_e2298 * _e2298), fma(_e2287, _e2287, -1f), 1f), 2f) * 3.1415927f)) * ((_e2311 / fma(_e2311, _e2320, _e2318)) * (_e2315 / fma(_e2315, _e2320, _e2318))));
                                            let _e2353 = fma((4f * _e2311), _e2315, 0.0001f);
                                            phi_3175_ = vec3<f32>(fma((fma((((1f - _e2339) * _e2261) * _e1367), 0.31830987f, ((_e2348 * _e2339) / _e2353)) * (_e2176.member_4.x * _e2259)), _e2315, _e1940.x), fma((fma((((1f - _e2340) * _e2261) * _e1369), 0.31830987f, ((_e2348 * _e2340) / _e2353)) * (_e2176.member_4.y * _e2259)), _e2315, _e1940.y), fma((fma((((1f - _e2341) * _e2261) * _e1371), 0.31830987f, ((_e2348 * _e2341) / _e2353)) * (_e2176.member_4.z * _e2259)), _e2315, _e1940.z));
                                        }
                                        let _e2374 = phi_3175_;
                                        phi_3177_ = _e2374;
                                        phi_3178_ = select(true, false, _e2216);
                                        break;
                                    }
                                    default: {
                                        phi_3177_ = vec3<f32>();
                                        phi_3178_ = bool();
                                        break;
                                    }
                                }
                                let _e2799 = phi_3177_;
                                let _e2801 = phi_3178_;
                                phi_3187_ = select(_e2799, _e1940, vec3(select(true, false, _e2801)));
                            }
                            let _e2806 = phi_3187_;
                            phi_2296_ = _e2806;
                            phi_3189_ = select(true, false, _e1973);
                            break;
                        }
                        default: {
                            phi_2296_ = vec3<f32>();
                            phi_3189_ = bool();
                            break;
                        }
                    }
                    let _e2809 = phi_2296_;
                    let _e2811 = phi_3189_;
                    continue;
                    continuing {
                        phi_2292_ = _e1953;
                        phi_2295_ = _e2809;
                        break if !(_e2811);
                    }
                }
                let _e2814 = fma(-(_e683.z), _e292.member_3, 1f);
                let _e2818 = fma(0.04f, _e2814, (_e1367 * _e1379));
                let _e2819 = fma(0.04f, _e2814, (_e1369 * _e1379));
                let _e2820 = fma(0.04f, _e2814, (_e1371 * _e1379));
                let _e2832 = fma(-(_e683.y), _e292.member_4, 1f);
                let _e2839 = (1f - max(fma(_e1931.z, _e1935.z, fma(_e1931.x, _e1935.x, (_e1931.y * _e1935.y))), 0f));
                let _e2841 = select(_e2839, 0f, (_e2839 < 0f));
                let _e2844 = pow(select(_e2841, 1f, (_e2841 > 1f)), 5f);
                let _e2845 = fma((max(_e2832, _e2818) - _e2818), _e2844, _e2818);
                let _e2846 = fma((max(_e2832, _e2819) - _e2819), _e2844, _e2819);
                let _e2847 = fma((max(_e2832, _e2820) - _e2820), _e2844, _e2820);
                let _e2867 = local_2;
                let _e2871 = local_3;
                let _e2875 = local_4;
                phi_3306_ = vec4<f32>(fma(_e1389, _e292.member_1, fma(fma(((1f - _e2845) * _e2814), (_e1398.x * _e1367), (_e1746.x * fma(_e2845, _e1762.x, _e1762.y))), _e1383, _e2867.x)), fma(_e1391, _e292.member_1, fma(fma(((1f - _e2846) * _e2814), (_e1398.y * _e1369), (_e1746.y * fma(_e2846, _e1762.x, _e1762.y))), _e1383, _e2871.y)), fma(_e1393, _e292.member_1, fma(fma(((1f - _e2847) * _e2814), (_e1398.z * _e1371), (_e1746.z * fma(_e2847, _e1762.x, _e1762.y))), _e1383, _e2875.z)), 1f);
            } else {
                phi_3306_ = (vec4<f32>((_e115.x * _e489.x), (_e115.y * _e489.y), (_e115.z * _e489.z), (_e115.w * _e489.w)) * _e292.member_2);
            }
            let _e2883 = phi_3306_;
            global_17 = _e2883;
            break;
        }
        case 1: {
            let _e1904 = sqrt(fma(_e116.x, _e116.x, (_e116.y * _e116.y)));
            if (_e1904 == 0f) {
                phi_5342_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5342_ = (vec3<f32>(_e116.x, _e116.y, 0f) * (1f / _e1904));
            }
            let _e1909 = phi_5342_;
            global_17 = vec4<f32>(((_e1909.x + 1f) * 0.5f), ((_e1909.y + 1f) * 0.5f), ((_e1909.z + 1f) * 0.5f), 1f);
            break;
        }
        case 2: {
            let _e1883 = sqrt(fma(_e117.x, _e117.x, (_e117.y * _e117.y)));
            if (_e1883 == 0f) {
                phi_5293_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5293_ = (vec3<f32>(_e117.x, _e117.y, 0f) * (1f / _e1883));
            }
            let _e1888 = phi_5293_;
            global_17 = vec4<f32>(((_e1888.x + 1f) * 0.5f), ((_e1888.y + 1f) * 0.5f), ((_e1888.z + 1f) * 0.5f), 1f);
            break;
        }
        case 3: {
            if _e1725 {
                phi_5244_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5244_ = (_e1352 * (1f / _e1724));
            }
            let _e1867 = phi_5244_;
            global_17 = vec4<f32>(((_e1867.x + 1f) * 0.5f), ((_e1867.y + 1f) * 0.5f), ((_e1867.z + 1f) * 0.5f), 1f);
            break;
        }
        case 4: {
            global_17 = _e115;
            break;
        }
        case 5: {
            let _e1848 = sqrt(fma(_e118.z, _e118.z, fma(_e118.x, _e118.x, (_e118.y * _e118.y))));
            if (_e1848 == 0f) {
                phi_5195_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5195_ = (_e118 * (1f / _e1848));
            }
            let _e1853 = phi_5195_;
            global_17 = vec4<f32>(((_e1853.x + 1f) * 0.5f), ((_e1853.y + 1f) * 0.5f), ((_e1853.z + 1f) * 0.5f), 1f);
            break;
        }
        case 6: {
            let _e1826 = sqrt(fma(_e1350.z, _e1350.z, fma(_e1350.x, _e1350.x, (_e1350.y * _e1350.y))));
            if (_e1826 == 0f) {
                phi_5146_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5146_ = (_e1350 * (1f / _e1826));
            }
            let _e1831 = phi_5146_;
            global_17 = vec4<f32>(((_e1831.x + 1f) * 0.5f), ((_e1831.y + 1f) * 0.5f), ((_e1831.z + 1f) * 0.5f), 1f);
            break;
        }
        case 7: {
            let _e1804 = sqrt(fma(_e119.z, _e119.z, fma(_e119.x, _e119.x, (_e119.y * _e119.y))));
            if (_e1804 == 0f) {
                phi_5097_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5097_ = (_e119 * (1f / _e1804));
            }
            let _e1809 = phi_5097_;
            global_17 = vec4<f32>(((_e1809.x + 1f) * 0.5f), ((_e1809.y + 1f) * 0.5f), ((_e1809.z + 1f) * 0.5f), 1f);
            break;
        }
        case 8: {
            let _e1782 = sqrt(fma(_e120.z, _e120.z, fma(_e120.x, _e120.x, (_e120.y * _e120.y))));
            if (_e1782 == 0f) {
                phi_5048_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5048_ = (_e120 * (1f / _e1782));
            }
            let _e1787 = phi_5048_;
            global_17 = vec4<f32>(((_e1787.x + 1f) * 0.5f), ((_e1787.y + 1f) * 0.5f), ((_e1787.z + 1f) * 0.5f), 1f);
            break;
        }
        case 9: {
            global_17 = vec4<f32>(_e1398.x, _e1398.y, _e1398.z, 1f);
            break;
        }
        case 10: {
            global_17 = vec4<f32>(_e1746.x, _e1746.y, _e1746.z, 1f);
            break;
        }
        case 11: {
            global_17 = vec4<f32>(_e1762.x, _e1762.y, 1f, 1f);
            break;
        }
        case 12: {
            global_17 = (vec4<f32>(_e1356, _e1359, _e1362, (_e489.w * _e292.member_2.w)) * _e115);
            break;
        }
        case 13: {
            global_17 = vec4<f32>(_e1376, _e1376, _e1376, 1f);
            break;
        }
        case 14: {
            global_17 = vec4<f32>(_e1379, _e1379, _e1379, 1f);
            break;
        }
        case 15: {
            global_17 = vec4<f32>(_e1383, _e1383, _e1383, 1f);
            break;
        }
        case 16: {
            global_17 = vec4<f32>((_e1389 * _e292.member_1), (_e1391 * _e292.member_1), (_e1393 * _e292.member_1), 1f);
            break;
        }
        case 17: {
            global_17 = vec4<f32>(_e1265.x, _e1265.y, _e1265.z, 1f);
            break;
        }
        case 18: {
            global_17 = vec4<f32>(_e292.member.x, _e292.member.y, _e292.member.z, 1f);
            break;
        }
        case 19: {
            global_17 = vec4<f32>(_e292.member_1, _e292.member_1, _e292.member_1, 1f);
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
    global_1 = param;
    global_2 = param_1;
    global_3 = param_2;
    global_4 = param_3;
    global_5 = param_4;
    global_6 = param_5;
    global_7 = param_6;
    global_8 = param_7;
    function();
    let _e17 = global_17;
    return _e17;
}
