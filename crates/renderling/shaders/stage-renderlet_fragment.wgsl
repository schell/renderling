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
    var phi_689_: u32;
    var phi_4542_: bool;
    var phi_817_: type_33;
    var phi_821_: type_33;
    var phi_4579_: bool;
    var phi_861_: u32;
    var phi_870_: u32;
    var phi_883_: type_15;
    var phi_4601_: f32;
    var phi_4614_: bool;
    var phi_937_: f32;
    var phi_932_: f32;
    var phi_938_: f32;
    var phi_4631_: bool;
    var phi_903_: f32;
    var phi_940_: f32;
    var phi_4649_: f32;
    var phi_4662_: bool;
    var phi_995_: f32;
    var phi_990_: f32;
    var phi_996_: f32;
    var phi_4679_: bool;
    var phi_961_: f32;
    var phi_998_: f32;
    var phi_4715_: bool;
    var phi_1081_: u32;
    var phi_1090_: u32;
    var phi_1103_: type_15;
    var phi_4736_: f32;
    var phi_4749_: bool;
    var phi_1157_: f32;
    var phi_1152_: f32;
    var phi_1158_: f32;
    var phi_4766_: bool;
    var phi_1123_: f32;
    var phi_1160_: f32;
    var phi_4784_: f32;
    var phi_4797_: bool;
    var phi_1215_: f32;
    var phi_1210_: f32;
    var phi_1216_: f32;
    var phi_4814_: bool;
    var phi_1181_: f32;
    var phi_1218_: f32;
    var phi_4850_: bool;
    var phi_1301_: u32;
    var phi_1310_: u32;
    var phi_1323_: type_15;
    var phi_4871_: f32;
    var phi_4884_: bool;
    var phi_1377_: f32;
    var phi_1372_: f32;
    var phi_1378_: f32;
    var phi_4901_: bool;
    var phi_1343_: f32;
    var phi_1380_: f32;
    var phi_4919_: f32;
    var phi_4932_: bool;
    var phi_1435_: f32;
    var phi_1430_: f32;
    var phi_1436_: f32;
    var phi_4949_: bool;
    var phi_1401_: f32;
    var phi_1438_: f32;
    var phi_4985_: bool;
    var phi_1521_: u32;
    var phi_1530_: u32;
    var phi_1543_: type_15;
    var phi_5006_: f32;
    var phi_5019_: bool;
    var phi_1597_: f32;
    var phi_1592_: f32;
    var phi_1598_: f32;
    var phi_5036_: bool;
    var phi_1563_: f32;
    var phi_1600_: f32;
    var phi_5054_: f32;
    var phi_5067_: bool;
    var phi_1655_: f32;
    var phi_1650_: f32;
    var phi_1656_: f32;
    var phi_5084_: bool;
    var phi_1621_: f32;
    var phi_1658_: f32;
    var phi_5120_: bool;
    var phi_1741_: u32;
    var phi_1750_: u32;
    var phi_1763_: type_15;
    var phi_5141_: f32;
    var phi_5154_: bool;
    var phi_1817_: f32;
    var phi_1812_: f32;
    var phi_1818_: f32;
    var phi_5171_: bool;
    var phi_1783_: f32;
    var phi_1820_: f32;
    var phi_5189_: f32;
    var phi_5202_: bool;
    var phi_1875_: f32;
    var phi_1870_: f32;
    var phi_1876_: f32;
    var phi_5219_: bool;
    var phi_1841_: f32;
    var phi_1878_: f32;
    var phi_5277_: vec3<f32>;
    var phi_5312_: vec3<f32>;
    var phi_5347_: vec3<f32>;
    var phi_5382_: vec3<f32>;
    var phi_5417_: vec3<f32>;
    var phi_1972_: vec3<f32>;
    var phi_1973_: vec3<f32>;
    var phi_5449_: bool;
    var phi_2180_: type_14;
    var phi_2181_: type_14;
    var phi_2204_: type_14;
    var phi_2231_: bool;
    var phi_2237_: type_14;
    var phi_2238_: type_14;
    var phi_2261_: type_14;
    var phi_2284_: bool;
    var phi_2305_: type_25;
    var phi_5521_: vec3<f32>;
    var phi_5580_: vec3<f32>;
    var phi_5654_: vec3<f32>;
    var phi_5714_: vec3<f32>;
    var phi_5763_: vec3<f32>;
    var phi_5812_: vec3<f32>;
    var phi_5861_: vec3<f32>;
    var phi_5910_: vec3<f32>;
    var phi_5959_: vec3<f32>;
    var phi_6008_: vec3<f32>;
    var phi_6047_: vec3<f32>;
    var phi_6082_: vec3<f32>;
    var phi_7183_: bool;
    var phi_2372_: type_14;
    var phi_2375_: vec3<f32>;
    var phi_2373_: type_14;
    var phi_2398_: type_14;
    var phi_6108_: u32;
    var phi_6127_: bool;
    var phi_2415_: u32;
    var phi_6151_: bool;
    var phi_2427_: u32;
    var phi_2441_: type_30;
    var phi_6183_: bool;
    var phi_2491_: type_31;
    var phi_6263_: bool;
    var phi_3595_: type_37;
    var phi_6313_: vec3<f32>;
    var phi_6348_: vec3<f32>;
    var phi_6383_: vec3<f32>;
    var phi_3850_: vec3<f32>;
    var phi_6475_: bool;
    var phi_3342_: type_34;
    var phi_6522_: vec3<f32>;
    var phi_6557_: vec3<f32>;
    var phi_3532_: vec3<f32>;
    var phi_6649_: bool;
    var phi_2539_: type_34;
    var phi_6696_: vec3<f32>;
    var phi_6731_: vec3<f32>;
    var phi_2780_: u32;
    var phi_2789_: u32;
    var phi_6840_: bool;
    var phi_6843_: bool;
    var phi_6844_: bool;
    var phi_7161_: bool;
    var phi_2968_: type_35;
    var phi_2971_: f32;
    var phi_2983_: bool;
    var phi_3011_: type_36;
    var phi_3023_: type_35;
    var phi_2969_: type_35;
    var phi_3026_: type_36;
    var phi_3037_: type_35;
    var phi_3040_: f32;
    var phi_3052_: bool;
    var phi_3080_: type_36;
    var phi_3092_: type_35;
    var phi_3038_: type_35;
    var phi_3095_: type_36;
    var phi_6862_: f32;
    var phi_6875_: bool;
    var phi_3161_: f32;
    var phi_3156_: f32;
    var phi_3162_: f32;
    var phi_6892_: bool;
    var phi_3127_: f32;
    var phi_3164_: f32;
    var phi_6910_: f32;
    var phi_6923_: bool;
    var phi_3217_: f32;
    var phi_3212_: f32;
    var phi_3218_: f32;
    var phi_6940_: bool;
    var phi_3183_: f32;
    var phi_3220_: f32;
    var phi_3041_: f32;
    var phi_3281_: bool;
    var phi_7145_: bool;
    var phi_7237_: bool;
    var phi_2972_: f32;
    var phi_3282_: bool;
    var phi_7236_: bool;
    var local_2: f32;
    var phi_7264_: bool;
    var phi_3287_: f32;
    var phi_7263_: bool;
    var phi_3288_: f32;
    var phi_7242_: bool;
    var phi_3852_: vec3<f32>;
    var phi_3853_: bool;
    var phi_7239_: bool;
    var phi_2376_: vec3<f32>;
    var phi_3862_: bool;
    var phi_7238_: bool;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var phi_7273_: bool;
    var phi_3979_: vec4<f32>;
    var phi_7265_: bool;
    var local_6: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e121 = arrayLength((&global.member));
            let _e123 = arrayLength((&global_1.member));
            let _e124 = global_2;
            let _e125 = global_3;
            let _e126 = global_4;
            let _e127 = global_5;
            let _e128 = global_6;
            let _e129 = global_7;
            let _e130 = global_8;
            let _e131 = global_9;
            let _e135 = global.member[(_e124 + 9u)];
            let _e139 = global.member[(_e124 + 11u)];
            let _e143 = global.member[(_e124 + 17u)];
            let _e146 = global.member[_e143];
            let _e150 = global.member[(_e143 + 1u)];
            let _e154 = global.member[(_e143 + 4u)];
            switch bitcast<i32>(_e154) {
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
                case 3: {
                    phi_689_ = 3u;
                    break;
                }
                case 4: {
                    phi_689_ = 4u;
                    break;
                }
                case 5: {
                    phi_689_ = 5u;
                    break;
                }
                case 6: {
                    phi_689_ = 6u;
                    break;
                }
                case 7: {
                    phi_689_ = 7u;
                    break;
                }
                case 8: {
                    phi_689_ = 8u;
                    break;
                }
                case 9: {
                    phi_689_ = 9u;
                    break;
                }
                case 10: {
                    phi_689_ = 10u;
                    break;
                }
                case 11: {
                    phi_689_ = 11u;
                    break;
                }
                case 12: {
                    phi_689_ = 12u;
                    break;
                }
                case 13: {
                    phi_689_ = 13u;
                    break;
                }
                case 14: {
                    phi_689_ = 14u;
                    break;
                }
                case 15: {
                    phi_689_ = 15u;
                    break;
                }
                case 16: {
                    phi_689_ = 16u;
                    break;
                }
                case 17: {
                    phi_689_ = 17u;
                    break;
                }
                case 18: {
                    phi_689_ = 18u;
                    break;
                }
                case 19: {
                    phi_689_ = 19u;
                    break;
                }
                default: {
                    phi_689_ = 0u;
                    break;
                }
            }
            let _e157 = phi_689_;
            let _e161 = global.member[(_e143 + 5u)];
            if (_e139 == 4294967295u) {
                phi_821_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, false, 0f);
            } else {
                if (_e121 >= 22u) {
                    phi_4542_ = (_e139 <= (_e121 - 22u));
                } else {
                    phi_4542_ = false;
                }
                let _e168 = phi_4542_;
                if _e168 {
                    let _e171 = global.member[_e139];
                    let _e176 = global.member[(_e139 + 1u)];
                    let _e181 = global.member[(_e139 + 2u)];
                    let _e187 = global.member[(_e139 + 3u)];
                    let _e192 = global.member[(_e139 + 4u)];
                    let _e197 = global.member[(_e139 + 5u)];
                    let _e202 = global.member[(_e139 + 6u)];
                    let _e207 = global.member[(_e139 + 7u)];
                    let _e213 = global.member[(_e139 + 8u)];
                    let _e218 = global.member[(_e139 + 9u)];
                    let _e223 = global.member[(_e139 + 10u)];
                    let _e227 = global.member[(_e139 + 11u)];
                    let _e231 = global.member[(_e139 + 12u)];
                    let _e235 = global.member[(_e139 + 13u)];
                    let _e239 = global.member[(_e139 + 14u)];
                    let _e243 = global.member[(_e139 + 15u)];
                    let _e247 = global.member[(_e139 + 16u)];
                    let _e251 = global.member[(_e139 + 17u)];
                    let _e255 = global.member[(_e139 + 18u)];
                    let _e259 = global.member[(_e139 + 19u)];
                    let _e263 = global.member[(_e139 + 20u)];
                    let _e268 = global.member[(_e139 + 21u)];
                    phi_817_ = type_33(vec3<f32>(bitcast<f32>(_e171), bitcast<f32>(_e176), bitcast<f32>(_e181)), bitcast<f32>(_e187), vec4<f32>(bitcast<f32>(_e192), bitcast<f32>(_e197), bitcast<f32>(_e202), bitcast<f32>(_e207)), bitcast<f32>(_e213), bitcast<f32>(_e218), _e223, _e227, _e231, _e235, _e239, _e243, _e247, _e251, _e255, _e259, (_e263 == 1u), bitcast<f32>(_e268));
                } else {
                    phi_817_ = type_33(vec3<f32>(0f, 0f, 0f), 1f, vec4<f32>(1f, 1f, 1f, 1f), 1f, 1f, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 4294967295u, 0u, 0u, 0u, 0u, 0u, true, 0f);
                }
                let _e272 = phi_817_;
                phi_821_ = type_33(_e272.member, _e272.member_1, _e272.member_2, _e272.member_3, _e272.member_4, _e272.member_5, _e272.member_6, _e272.member_7, _e272.member_8, _e272.member_9, _e272.member_10, _e272.member_11, _e272.member_12, _e272.member_13, _e272.member_14, (_e272.member_15 && (_e161 == 1u)), _e272.member_16);
            }
            let _e294 = phi_821_;
            let _e298 = select(_e127, _e126, vec2((_e294.member_10 == 0u)));
            let _e300 = (_e121 >= 8u);
            if _e300 {
                phi_4579_ = (_e294.member_5 <= (_e121 - 8u));
            } else {
                phi_4579_ = false;
            }
            let _e304 = phi_4579_;
            if _e304 {
                let _e307 = global.member[_e294.member_5];
                let _e311 = global.member[(_e294.member_5 + 1u)];
                let _e316 = global.member[(_e294.member_5 + 2u)];
                let _e320 = global.member[(_e294.member_5 + 3u)];
                let _e325 = global.member[(_e294.member_5 + 4u)];
                let _e329 = global.member[(_e294.member_5 + 5u)];
                let _e333 = global.member[(_e294.member_5 + 6u)];
                switch bitcast<i32>(_e333) {
                    case 0: {
                        phi_861_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_861_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_861_ = 2u;
                        break;
                    }
                    default: {
                        phi_861_ = 0u;
                        break;
                    }
                }
                let _e336 = phi_861_;
                let _e340 = global.member[(_e294.member_5 + 7u)];
                switch bitcast<i32>(_e340) {
                    case 0: {
                        phi_870_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_870_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_870_ = 2u;
                        break;
                    }
                    default: {
                        phi_870_ = 0u;
                        break;
                    }
                }
                let _e343 = phi_870_;
                phi_883_ = type_15(type_14(_e336, _e343), vec2<u32>(_e307, _e311), vec2<u32>(_e316, _e320), _e325, _e329);
            } else {
                phi_883_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e347 = phi_883_;
            switch bitcast<i32>(_e347.member.member) {
                case 1: {
                    let _e385 = abs(_e298.x);
                    let _e387 = (_e385 % 1f);
                    if (_e385 >= 1f) {
                        phi_4631_ = select(true, false, (_e387 == 0f));
                    } else {
                        phi_4631_ = true;
                    }
                    let _e391 = phi_4631_;
                    let _e392 = select(1f, _e387, _e391);
                    if (select(-1f, 1f, (_e298.x >= 0f)) > 0f) {
                        phi_903_ = _e392;
                    } else {
                        phi_903_ = (1f - _e392);
                    }
                    let _e396 = phi_903_;
                    phi_940_ = _e396;
                    break;
                }
                case 2: {
                    let _e359 = abs(_e298.x);
                    let _e366 = ((select(select(u32(_e359), 0u, (_e359 < 0f)), 4294967295u, (_e359 > 4294967000f)) % 2u) == 0u);
                    let _e368 = (_e359 % 1f);
                    if (_e359 >= 1f) {
                        phi_4614_ = select(true, false, (_e368 == 0f));
                    } else {
                        phi_4614_ = true;
                    }
                    let _e372 = phi_4614_;
                    let _e373 = select(1f, _e368, _e372);
                    if (select(-1f, 1f, (_e298.x >= 0f)) > 0f) {
                        if _e366 {
                            phi_932_ = _e373;
                        } else {
                            phi_932_ = (1f - _e373);
                        }
                        let _e380 = phi_932_;
                        phi_938_ = _e380;
                    } else {
                        if _e366 {
                            phi_937_ = (1f - _e373);
                        } else {
                            phi_937_ = _e373;
                        }
                        let _e377 = phi_937_;
                        phi_938_ = _e377;
                    }
                    let _e382 = phi_938_;
                    phi_940_ = _e382;
                    break;
                }
                case 0: {
                    if (_e298.x > 1f) {
                        phi_4601_ = 0.9999999f;
                    } else {
                        phi_4601_ = select(_e298.x, 0.00000011920929f, (_e298.x < 0f));
                    }
                    let _e356 = phi_4601_;
                    phi_940_ = _e356;
                    break;
                }
                default: {
                    phi_940_ = f32();
                    break;
                }
            }
            let _e398 = phi_940_;
            switch bitcast<i32>(_e347.member.member_1) {
                case 1: {
                    let _e436 = abs(_e298.y);
                    let _e438 = (_e436 % 1f);
                    if (_e436 >= 1f) {
                        phi_4679_ = select(true, false, (_e438 == 0f));
                    } else {
                        phi_4679_ = true;
                    }
                    let _e442 = phi_4679_;
                    let _e443 = select(1f, _e438, _e442);
                    if (select(-1f, 1f, (_e298.y >= 0f)) > 0f) {
                        phi_961_ = _e443;
                    } else {
                        phi_961_ = (1f - _e443);
                    }
                    let _e447 = phi_961_;
                    phi_998_ = _e447;
                    break;
                }
                case 2: {
                    let _e410 = abs(_e298.y);
                    let _e417 = ((select(select(u32(_e410), 0u, (_e410 < 0f)), 4294967295u, (_e410 > 4294967000f)) % 2u) == 0u);
                    let _e419 = (_e410 % 1f);
                    if (_e410 >= 1f) {
                        phi_4662_ = select(true, false, (_e419 == 0f));
                    } else {
                        phi_4662_ = true;
                    }
                    let _e423 = phi_4662_;
                    let _e424 = select(1f, _e419, _e423);
                    if (select(-1f, 1f, (_e298.y >= 0f)) > 0f) {
                        if _e417 {
                            phi_990_ = _e424;
                        } else {
                            phi_990_ = (1f - _e424);
                        }
                        let _e431 = phi_990_;
                        phi_996_ = _e431;
                    } else {
                        if _e417 {
                            phi_995_ = (1f - _e424);
                        } else {
                            phi_995_ = _e424;
                        }
                        let _e428 = phi_995_;
                        phi_996_ = _e428;
                    }
                    let _e433 = phi_996_;
                    phi_998_ = _e433;
                    break;
                }
                case 0: {
                    if (_e298.y > 1f) {
                        phi_4649_ = 0.9999999f;
                    } else {
                        phi_4649_ = select(_e298.y, 0.00000011920929f, (_e298.y < 0f));
                    }
                    let _e407 = phi_4649_;
                    phi_998_ = _e407;
                    break;
                }
                default: {
                    phi_998_ = f32();
                    break;
                }
            }
            let _e449 = phi_998_;
            let _e453 = (_e398 * f32(_e347.member_2.x));
            let _e462 = (_e449 * f32(_e347.member_2.y));
            let _e474 = f32(_e146);
            let _e475 = f32(_e150);
            let _e482 = vec3<f32>((f32((select(select(u32(_e453), 0u, (_e453 < 0f)), 4294967295u, (_e453 > 4294967000f)) + _e347.member_1.x)) / _e474), (f32((select(select(u32(_e462), 0u, (_e462 < 0f)), 4294967295u, (_e462 > 4294967000f)) + _e347.member_1.y)) / _e475), f32(_e347.member_3));
            let _e488 = textureSampleLevel(global_11, global_10, vec2<f32>(_e482.x, _e482.y), i32(_e482.z), 0f);
            let _e491 = select(_e488, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e294.member_5 == 4294967295u)));
            let _e495 = select(_e127, _e126, vec2((_e294.member_11 == 0u)));
            if _e300 {
                phi_4715_ = (_e294.member_6 <= (_e121 - 8u));
            } else {
                phi_4715_ = false;
            }
            let _e500 = phi_4715_;
            if _e500 {
                let _e503 = global.member[_e294.member_6];
                let _e507 = global.member[(_e294.member_6 + 1u)];
                let _e512 = global.member[(_e294.member_6 + 2u)];
                let _e516 = global.member[(_e294.member_6 + 3u)];
                let _e521 = global.member[(_e294.member_6 + 4u)];
                let _e525 = global.member[(_e294.member_6 + 5u)];
                let _e529 = global.member[(_e294.member_6 + 6u)];
                switch bitcast<i32>(_e529) {
                    case 0: {
                        phi_1081_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1081_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1081_ = 2u;
                        break;
                    }
                    default: {
                        phi_1081_ = 0u;
                        break;
                    }
                }
                let _e532 = phi_1081_;
                let _e536 = global.member[(_e294.member_6 + 7u)];
                switch bitcast<i32>(_e536) {
                    case 0: {
                        phi_1090_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1090_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1090_ = 2u;
                        break;
                    }
                    default: {
                        phi_1090_ = 0u;
                        break;
                    }
                }
                let _e539 = phi_1090_;
                phi_1103_ = type_15(type_14(_e532, _e539), vec2<u32>(_e503, _e507), vec2<u32>(_e512, _e516), _e521, _e525);
            } else {
                phi_1103_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e543 = phi_1103_;
            switch bitcast<i32>(_e543.member.member) {
                case 1: {
                    let _e581 = abs(_e495.x);
                    let _e583 = (_e581 % 1f);
                    if (_e581 >= 1f) {
                        phi_4766_ = select(true, false, (_e583 == 0f));
                    } else {
                        phi_4766_ = true;
                    }
                    let _e587 = phi_4766_;
                    let _e588 = select(1f, _e583, _e587);
                    if (select(-1f, 1f, (_e495.x >= 0f)) > 0f) {
                        phi_1123_ = _e588;
                    } else {
                        phi_1123_ = (1f - _e588);
                    }
                    let _e592 = phi_1123_;
                    phi_1160_ = _e592;
                    break;
                }
                case 2: {
                    let _e555 = abs(_e495.x);
                    let _e562 = ((select(select(u32(_e555), 0u, (_e555 < 0f)), 4294967295u, (_e555 > 4294967000f)) % 2u) == 0u);
                    let _e564 = (_e555 % 1f);
                    if (_e555 >= 1f) {
                        phi_4749_ = select(true, false, (_e564 == 0f));
                    } else {
                        phi_4749_ = true;
                    }
                    let _e568 = phi_4749_;
                    let _e569 = select(1f, _e564, _e568);
                    if (select(-1f, 1f, (_e495.x >= 0f)) > 0f) {
                        if _e562 {
                            phi_1152_ = _e569;
                        } else {
                            phi_1152_ = (1f - _e569);
                        }
                        let _e576 = phi_1152_;
                        phi_1158_ = _e576;
                    } else {
                        if _e562 {
                            phi_1157_ = (1f - _e569);
                        } else {
                            phi_1157_ = _e569;
                        }
                        let _e573 = phi_1157_;
                        phi_1158_ = _e573;
                    }
                    let _e578 = phi_1158_;
                    phi_1160_ = _e578;
                    break;
                }
                case 0: {
                    if (_e495.x > 1f) {
                        phi_4736_ = 0.9999999f;
                    } else {
                        phi_4736_ = select(_e495.x, 0.00000011920929f, (_e495.x < 0f));
                    }
                    let _e552 = phi_4736_;
                    phi_1160_ = _e552;
                    break;
                }
                default: {
                    phi_1160_ = f32();
                    break;
                }
            }
            let _e594 = phi_1160_;
            switch bitcast<i32>(_e543.member.member_1) {
                case 1: {
                    let _e632 = abs(_e495.y);
                    let _e634 = (_e632 % 1f);
                    if (_e632 >= 1f) {
                        phi_4814_ = select(true, false, (_e634 == 0f));
                    } else {
                        phi_4814_ = true;
                    }
                    let _e638 = phi_4814_;
                    let _e639 = select(1f, _e634, _e638);
                    if (select(-1f, 1f, (_e495.y >= 0f)) > 0f) {
                        phi_1181_ = _e639;
                    } else {
                        phi_1181_ = (1f - _e639);
                    }
                    let _e643 = phi_1181_;
                    phi_1218_ = _e643;
                    break;
                }
                case 2: {
                    let _e606 = abs(_e495.y);
                    let _e613 = ((select(select(u32(_e606), 0u, (_e606 < 0f)), 4294967295u, (_e606 > 4294967000f)) % 2u) == 0u);
                    let _e615 = (_e606 % 1f);
                    if (_e606 >= 1f) {
                        phi_4797_ = select(true, false, (_e615 == 0f));
                    } else {
                        phi_4797_ = true;
                    }
                    let _e619 = phi_4797_;
                    let _e620 = select(1f, _e615, _e619);
                    if (select(-1f, 1f, (_e495.y >= 0f)) > 0f) {
                        if _e613 {
                            phi_1210_ = _e620;
                        } else {
                            phi_1210_ = (1f - _e620);
                        }
                        let _e627 = phi_1210_;
                        phi_1216_ = _e627;
                    } else {
                        if _e613 {
                            phi_1215_ = (1f - _e620);
                        } else {
                            phi_1215_ = _e620;
                        }
                        let _e624 = phi_1215_;
                        phi_1216_ = _e624;
                    }
                    let _e629 = phi_1216_;
                    phi_1218_ = _e629;
                    break;
                }
                case 0: {
                    if (_e495.y > 1f) {
                        phi_4784_ = 0.9999999f;
                    } else {
                        phi_4784_ = select(_e495.y, 0.00000011920929f, (_e495.y < 0f));
                    }
                    let _e603 = phi_4784_;
                    phi_1218_ = _e603;
                    break;
                }
                default: {
                    phi_1218_ = f32();
                    break;
                }
            }
            let _e645 = phi_1218_;
            let _e649 = (_e594 * f32(_e543.member_2.x));
            let _e658 = (_e645 * f32(_e543.member_2.y));
            let _e676 = vec3<f32>((f32((select(select(u32(_e649), 0u, (_e649 < 0f)), 4294967295u, (_e649 > 4294967000f)) + _e543.member_1.x)) / _e474), (f32((select(select(u32(_e658), 0u, (_e658 < 0f)), 4294967295u, (_e658 > 4294967000f)) + _e543.member_1.y)) / _e475), f32(_e543.member_3));
            let _e682 = textureSampleLevel(global_11, global_10, vec2<f32>(_e676.x, _e676.y), i32(_e676.z), 0f);
            let _e685 = select(_e682, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e294.member_6 == 4294967295u)));
            let _e689 = select(_e127, _e126, vec2((_e294.member_12 == 0u)));
            if _e300 {
                phi_4850_ = (_e294.member_7 <= (_e121 - 8u));
            } else {
                phi_4850_ = false;
            }
            let _e694 = phi_4850_;
            if _e694 {
                let _e697 = global.member[_e294.member_7];
                let _e701 = global.member[(_e294.member_7 + 1u)];
                let _e706 = global.member[(_e294.member_7 + 2u)];
                let _e710 = global.member[(_e294.member_7 + 3u)];
                let _e715 = global.member[(_e294.member_7 + 4u)];
                let _e719 = global.member[(_e294.member_7 + 5u)];
                let _e723 = global.member[(_e294.member_7 + 6u)];
                switch bitcast<i32>(_e723) {
                    case 0: {
                        phi_1301_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1301_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1301_ = 2u;
                        break;
                    }
                    default: {
                        phi_1301_ = 0u;
                        break;
                    }
                }
                let _e726 = phi_1301_;
                let _e730 = global.member[(_e294.member_7 + 7u)];
                switch bitcast<i32>(_e730) {
                    case 0: {
                        phi_1310_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1310_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1310_ = 2u;
                        break;
                    }
                    default: {
                        phi_1310_ = 0u;
                        break;
                    }
                }
                let _e733 = phi_1310_;
                phi_1323_ = type_15(type_14(_e726, _e733), vec2<u32>(_e697, _e701), vec2<u32>(_e706, _e710), _e715, _e719);
            } else {
                phi_1323_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e737 = phi_1323_;
            switch bitcast<i32>(_e737.member.member) {
                case 1: {
                    let _e775 = abs(_e689.x);
                    let _e777 = (_e775 % 1f);
                    if (_e775 >= 1f) {
                        phi_4901_ = select(true, false, (_e777 == 0f));
                    } else {
                        phi_4901_ = true;
                    }
                    let _e781 = phi_4901_;
                    let _e782 = select(1f, _e777, _e781);
                    if (select(-1f, 1f, (_e689.x >= 0f)) > 0f) {
                        phi_1343_ = _e782;
                    } else {
                        phi_1343_ = (1f - _e782);
                    }
                    let _e786 = phi_1343_;
                    phi_1380_ = _e786;
                    break;
                }
                case 2: {
                    let _e749 = abs(_e689.x);
                    let _e756 = ((select(select(u32(_e749), 0u, (_e749 < 0f)), 4294967295u, (_e749 > 4294967000f)) % 2u) == 0u);
                    let _e758 = (_e749 % 1f);
                    if (_e749 >= 1f) {
                        phi_4884_ = select(true, false, (_e758 == 0f));
                    } else {
                        phi_4884_ = true;
                    }
                    let _e762 = phi_4884_;
                    let _e763 = select(1f, _e758, _e762);
                    if (select(-1f, 1f, (_e689.x >= 0f)) > 0f) {
                        if _e756 {
                            phi_1372_ = _e763;
                        } else {
                            phi_1372_ = (1f - _e763);
                        }
                        let _e770 = phi_1372_;
                        phi_1378_ = _e770;
                    } else {
                        if _e756 {
                            phi_1377_ = (1f - _e763);
                        } else {
                            phi_1377_ = _e763;
                        }
                        let _e767 = phi_1377_;
                        phi_1378_ = _e767;
                    }
                    let _e772 = phi_1378_;
                    phi_1380_ = _e772;
                    break;
                }
                case 0: {
                    if (_e689.x > 1f) {
                        phi_4871_ = 0.9999999f;
                    } else {
                        phi_4871_ = select(_e689.x, 0.00000011920929f, (_e689.x < 0f));
                    }
                    let _e746 = phi_4871_;
                    phi_1380_ = _e746;
                    break;
                }
                default: {
                    phi_1380_ = f32();
                    break;
                }
            }
            let _e788 = phi_1380_;
            switch bitcast<i32>(_e737.member.member_1) {
                case 1: {
                    let _e826 = abs(_e689.y);
                    let _e828 = (_e826 % 1f);
                    if (_e826 >= 1f) {
                        phi_4949_ = select(true, false, (_e828 == 0f));
                    } else {
                        phi_4949_ = true;
                    }
                    let _e832 = phi_4949_;
                    let _e833 = select(1f, _e828, _e832);
                    if (select(-1f, 1f, (_e689.y >= 0f)) > 0f) {
                        phi_1401_ = _e833;
                    } else {
                        phi_1401_ = (1f - _e833);
                    }
                    let _e837 = phi_1401_;
                    phi_1438_ = _e837;
                    break;
                }
                case 2: {
                    let _e800 = abs(_e689.y);
                    let _e807 = ((select(select(u32(_e800), 0u, (_e800 < 0f)), 4294967295u, (_e800 > 4294967000f)) % 2u) == 0u);
                    let _e809 = (_e800 % 1f);
                    if (_e800 >= 1f) {
                        phi_4932_ = select(true, false, (_e809 == 0f));
                    } else {
                        phi_4932_ = true;
                    }
                    let _e813 = phi_4932_;
                    let _e814 = select(1f, _e809, _e813);
                    if (select(-1f, 1f, (_e689.y >= 0f)) > 0f) {
                        if _e807 {
                            phi_1430_ = _e814;
                        } else {
                            phi_1430_ = (1f - _e814);
                        }
                        let _e821 = phi_1430_;
                        phi_1436_ = _e821;
                    } else {
                        if _e807 {
                            phi_1435_ = (1f - _e814);
                        } else {
                            phi_1435_ = _e814;
                        }
                        let _e818 = phi_1435_;
                        phi_1436_ = _e818;
                    }
                    let _e823 = phi_1436_;
                    phi_1438_ = _e823;
                    break;
                }
                case 0: {
                    if (_e689.y > 1f) {
                        phi_4919_ = 0.9999999f;
                    } else {
                        phi_4919_ = select(_e689.y, 0.00000011920929f, (_e689.y < 0f));
                    }
                    let _e797 = phi_4919_;
                    phi_1438_ = _e797;
                    break;
                }
                default: {
                    phi_1438_ = f32();
                    break;
                }
            }
            let _e839 = phi_1438_;
            let _e843 = (_e788 * f32(_e737.member_2.x));
            let _e852 = (_e839 * f32(_e737.member_2.y));
            let _e870 = vec3<f32>((f32((select(select(u32(_e843), 0u, (_e843 < 0f)), 4294967295u, (_e843 > 4294967000f)) + _e737.member_1.x)) / _e474), (f32((select(select(u32(_e852), 0u, (_e852 < 0f)), 4294967295u, (_e852 > 4294967000f)) + _e737.member_1.y)) / _e475), f32(_e737.member_3));
            let _e876 = textureSampleLevel(global_11, global_10, vec2<f32>(_e870.x, _e870.y), i32(_e870.z), 0f);
            let _e877 = (_e294.member_7 == 4294967295u);
            let _e879 = select(_e876, vec4<f32>(1f, 1f, 1f, 1f), vec4(_e877));
            let _e883 = select(_e127, _e126, vec2((_e294.member_13 == 0u)));
            if _e300 {
                phi_4985_ = (_e294.member_8 <= (_e121 - 8u));
            } else {
                phi_4985_ = false;
            }
            let _e888 = phi_4985_;
            if _e888 {
                let _e891 = global.member[_e294.member_8];
                let _e895 = global.member[(_e294.member_8 + 1u)];
                let _e900 = global.member[(_e294.member_8 + 2u)];
                let _e904 = global.member[(_e294.member_8 + 3u)];
                let _e909 = global.member[(_e294.member_8 + 4u)];
                let _e913 = global.member[(_e294.member_8 + 5u)];
                let _e917 = global.member[(_e294.member_8 + 6u)];
                switch bitcast<i32>(_e917) {
                    case 0: {
                        phi_1521_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1521_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1521_ = 2u;
                        break;
                    }
                    default: {
                        phi_1521_ = 0u;
                        break;
                    }
                }
                let _e920 = phi_1521_;
                let _e924 = global.member[(_e294.member_8 + 7u)];
                switch bitcast<i32>(_e924) {
                    case 0: {
                        phi_1530_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1530_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1530_ = 2u;
                        break;
                    }
                    default: {
                        phi_1530_ = 0u;
                        break;
                    }
                }
                let _e927 = phi_1530_;
                phi_1543_ = type_15(type_14(_e920, _e927), vec2<u32>(_e891, _e895), vec2<u32>(_e900, _e904), _e909, _e913);
            } else {
                phi_1543_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e931 = phi_1543_;
            switch bitcast<i32>(_e931.member.member) {
                case 1: {
                    let _e969 = abs(_e883.x);
                    let _e971 = (_e969 % 1f);
                    if (_e969 >= 1f) {
                        phi_5036_ = select(true, false, (_e971 == 0f));
                    } else {
                        phi_5036_ = true;
                    }
                    let _e975 = phi_5036_;
                    let _e976 = select(1f, _e971, _e975);
                    if (select(-1f, 1f, (_e883.x >= 0f)) > 0f) {
                        phi_1563_ = _e976;
                    } else {
                        phi_1563_ = (1f - _e976);
                    }
                    let _e980 = phi_1563_;
                    phi_1600_ = _e980;
                    break;
                }
                case 2: {
                    let _e943 = abs(_e883.x);
                    let _e950 = ((select(select(u32(_e943), 0u, (_e943 < 0f)), 4294967295u, (_e943 > 4294967000f)) % 2u) == 0u);
                    let _e952 = (_e943 % 1f);
                    if (_e943 >= 1f) {
                        phi_5019_ = select(true, false, (_e952 == 0f));
                    } else {
                        phi_5019_ = true;
                    }
                    let _e956 = phi_5019_;
                    let _e957 = select(1f, _e952, _e956);
                    if (select(-1f, 1f, (_e883.x >= 0f)) > 0f) {
                        if _e950 {
                            phi_1592_ = _e957;
                        } else {
                            phi_1592_ = (1f - _e957);
                        }
                        let _e964 = phi_1592_;
                        phi_1598_ = _e964;
                    } else {
                        if _e950 {
                            phi_1597_ = (1f - _e957);
                        } else {
                            phi_1597_ = _e957;
                        }
                        let _e961 = phi_1597_;
                        phi_1598_ = _e961;
                    }
                    let _e966 = phi_1598_;
                    phi_1600_ = _e966;
                    break;
                }
                case 0: {
                    if (_e883.x > 1f) {
                        phi_5006_ = 0.9999999f;
                    } else {
                        phi_5006_ = select(_e883.x, 0.00000011920929f, (_e883.x < 0f));
                    }
                    let _e940 = phi_5006_;
                    phi_1600_ = _e940;
                    break;
                }
                default: {
                    phi_1600_ = f32();
                    break;
                }
            }
            let _e982 = phi_1600_;
            switch bitcast<i32>(_e931.member.member_1) {
                case 1: {
                    let _e1020 = abs(_e883.y);
                    let _e1022 = (_e1020 % 1f);
                    if (_e1020 >= 1f) {
                        phi_5084_ = select(true, false, (_e1022 == 0f));
                    } else {
                        phi_5084_ = true;
                    }
                    let _e1026 = phi_5084_;
                    let _e1027 = select(1f, _e1022, _e1026);
                    if (select(-1f, 1f, (_e883.y >= 0f)) > 0f) {
                        phi_1621_ = _e1027;
                    } else {
                        phi_1621_ = (1f - _e1027);
                    }
                    let _e1031 = phi_1621_;
                    phi_1658_ = _e1031;
                    break;
                }
                case 2: {
                    let _e994 = abs(_e883.y);
                    let _e1001 = ((select(select(u32(_e994), 0u, (_e994 < 0f)), 4294967295u, (_e994 > 4294967000f)) % 2u) == 0u);
                    let _e1003 = (_e994 % 1f);
                    if (_e994 >= 1f) {
                        phi_5067_ = select(true, false, (_e1003 == 0f));
                    } else {
                        phi_5067_ = true;
                    }
                    let _e1007 = phi_5067_;
                    let _e1008 = select(1f, _e1003, _e1007);
                    if (select(-1f, 1f, (_e883.y >= 0f)) > 0f) {
                        if _e1001 {
                            phi_1650_ = _e1008;
                        } else {
                            phi_1650_ = (1f - _e1008);
                        }
                        let _e1015 = phi_1650_;
                        phi_1656_ = _e1015;
                    } else {
                        if _e1001 {
                            phi_1655_ = (1f - _e1008);
                        } else {
                            phi_1655_ = _e1008;
                        }
                        let _e1012 = phi_1655_;
                        phi_1656_ = _e1012;
                    }
                    let _e1017 = phi_1656_;
                    phi_1658_ = _e1017;
                    break;
                }
                case 0: {
                    if (_e883.y > 1f) {
                        phi_5054_ = 0.9999999f;
                    } else {
                        phi_5054_ = select(_e883.y, 0.00000011920929f, (_e883.y < 0f));
                    }
                    let _e991 = phi_5054_;
                    phi_1658_ = _e991;
                    break;
                }
                default: {
                    phi_1658_ = f32();
                    break;
                }
            }
            let _e1033 = phi_1658_;
            let _e1037 = (_e982 * f32(_e931.member_2.x));
            let _e1046 = (_e1033 * f32(_e931.member_2.y));
            let _e1064 = vec3<f32>((f32((select(select(u32(_e1037), 0u, (_e1037 < 0f)), 4294967295u, (_e1037 > 4294967000f)) + _e931.member_1.x)) / _e474), (f32((select(select(u32(_e1046), 0u, (_e1046 < 0f)), 4294967295u, (_e1046 > 4294967000f)) + _e931.member_1.y)) / _e475), f32(_e931.member_3));
            let _e1070 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1064.x, _e1064.y), i32(_e1064.z), 0f);
            let _e1077 = select(_e127, _e126, vec2((_e294.member_14 == 0u)));
            if _e300 {
                phi_5120_ = (_e294.member_9 <= (_e121 - 8u));
            } else {
                phi_5120_ = false;
            }
            let _e1082 = phi_5120_;
            if _e1082 {
                let _e1085 = global.member[_e294.member_9];
                let _e1089 = global.member[(_e294.member_9 + 1u)];
                let _e1094 = global.member[(_e294.member_9 + 2u)];
                let _e1098 = global.member[(_e294.member_9 + 3u)];
                let _e1103 = global.member[(_e294.member_9 + 4u)];
                let _e1107 = global.member[(_e294.member_9 + 5u)];
                let _e1111 = global.member[(_e294.member_9 + 6u)];
                switch bitcast<i32>(_e1111) {
                    case 0: {
                        phi_1741_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1741_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1741_ = 2u;
                        break;
                    }
                    default: {
                        phi_1741_ = 0u;
                        break;
                    }
                }
                let _e1114 = phi_1741_;
                let _e1118 = global.member[(_e294.member_9 + 7u)];
                switch bitcast<i32>(_e1118) {
                    case 0: {
                        phi_1750_ = 0u;
                        break;
                    }
                    case 1: {
                        phi_1750_ = 1u;
                        break;
                    }
                    case 2: {
                        phi_1750_ = 2u;
                        break;
                    }
                    default: {
                        phi_1750_ = 0u;
                        break;
                    }
                }
                let _e1121 = phi_1750_;
                phi_1763_ = type_15(type_14(_e1114, _e1121), vec2<u32>(_e1085, _e1089), vec2<u32>(_e1094, _e1098), _e1103, _e1107);
            } else {
                phi_1763_ = type_15(type_14(0u, 0u), vec2<u32>(0u, 0u), vec2<u32>(0u, 0u), 0u, 0u);
            }
            let _e1125 = phi_1763_;
            switch bitcast<i32>(_e1125.member.member) {
                case 1: {
                    let _e1163 = abs(_e1077.x);
                    let _e1165 = (_e1163 % 1f);
                    if (_e1163 >= 1f) {
                        phi_5171_ = select(true, false, (_e1165 == 0f));
                    } else {
                        phi_5171_ = true;
                    }
                    let _e1169 = phi_5171_;
                    let _e1170 = select(1f, _e1165, _e1169);
                    if (select(-1f, 1f, (_e1077.x >= 0f)) > 0f) {
                        phi_1783_ = _e1170;
                    } else {
                        phi_1783_ = (1f - _e1170);
                    }
                    let _e1174 = phi_1783_;
                    phi_1820_ = _e1174;
                    break;
                }
                case 2: {
                    let _e1137 = abs(_e1077.x);
                    let _e1144 = ((select(select(u32(_e1137), 0u, (_e1137 < 0f)), 4294967295u, (_e1137 > 4294967000f)) % 2u) == 0u);
                    let _e1146 = (_e1137 % 1f);
                    if (_e1137 >= 1f) {
                        phi_5154_ = select(true, false, (_e1146 == 0f));
                    } else {
                        phi_5154_ = true;
                    }
                    let _e1150 = phi_5154_;
                    let _e1151 = select(1f, _e1146, _e1150);
                    if (select(-1f, 1f, (_e1077.x >= 0f)) > 0f) {
                        if _e1144 {
                            phi_1812_ = _e1151;
                        } else {
                            phi_1812_ = (1f - _e1151);
                        }
                        let _e1158 = phi_1812_;
                        phi_1818_ = _e1158;
                    } else {
                        if _e1144 {
                            phi_1817_ = (1f - _e1151);
                        } else {
                            phi_1817_ = _e1151;
                        }
                        let _e1155 = phi_1817_;
                        phi_1818_ = _e1155;
                    }
                    let _e1160 = phi_1818_;
                    phi_1820_ = _e1160;
                    break;
                }
                case 0: {
                    if (_e1077.x > 1f) {
                        phi_5141_ = 0.9999999f;
                    } else {
                        phi_5141_ = select(_e1077.x, 0.00000011920929f, (_e1077.x < 0f));
                    }
                    let _e1134 = phi_5141_;
                    phi_1820_ = _e1134;
                    break;
                }
                default: {
                    phi_1820_ = f32();
                    break;
                }
            }
            let _e1176 = phi_1820_;
            switch bitcast<i32>(_e1125.member.member_1) {
                case 1: {
                    let _e1214 = abs(_e1077.y);
                    let _e1216 = (_e1214 % 1f);
                    if (_e1214 >= 1f) {
                        phi_5219_ = select(true, false, (_e1216 == 0f));
                    } else {
                        phi_5219_ = true;
                    }
                    let _e1220 = phi_5219_;
                    let _e1221 = select(1f, _e1216, _e1220);
                    if (select(-1f, 1f, (_e1077.y >= 0f)) > 0f) {
                        phi_1841_ = _e1221;
                    } else {
                        phi_1841_ = (1f - _e1221);
                    }
                    let _e1225 = phi_1841_;
                    phi_1878_ = _e1225;
                    break;
                }
                case 2: {
                    let _e1188 = abs(_e1077.y);
                    let _e1195 = ((select(select(u32(_e1188), 0u, (_e1188 < 0f)), 4294967295u, (_e1188 > 4294967000f)) % 2u) == 0u);
                    let _e1197 = (_e1188 % 1f);
                    if (_e1188 >= 1f) {
                        phi_5202_ = select(true, false, (_e1197 == 0f));
                    } else {
                        phi_5202_ = true;
                    }
                    let _e1201 = phi_5202_;
                    let _e1202 = select(1f, _e1197, _e1201);
                    if (select(-1f, 1f, (_e1077.y >= 0f)) > 0f) {
                        if _e1195 {
                            phi_1870_ = _e1202;
                        } else {
                            phi_1870_ = (1f - _e1202);
                        }
                        let _e1209 = phi_1870_;
                        phi_1876_ = _e1209;
                    } else {
                        if _e1195 {
                            phi_1875_ = (1f - _e1202);
                        } else {
                            phi_1875_ = _e1202;
                        }
                        let _e1206 = phi_1875_;
                        phi_1876_ = _e1206;
                    }
                    let _e1211 = phi_1876_;
                    phi_1878_ = _e1211;
                    break;
                }
                case 0: {
                    if (_e1077.y > 1f) {
                        phi_5189_ = 0.9999999f;
                    } else {
                        phi_5189_ = select(_e1077.y, 0.00000011920929f, (_e1077.y < 0f));
                    }
                    let _e1185 = phi_5189_;
                    phi_1878_ = _e1185;
                    break;
                }
                default: {
                    phi_1878_ = f32();
                    break;
                }
            }
            let _e1227 = phi_1878_;
            let _e1231 = (_e1176 * f32(_e1125.member_2.x));
            let _e1240 = (_e1227 * f32(_e1125.member_2.y));
            let _e1258 = vec3<f32>((f32((select(select(u32(_e1231), 0u, (_e1231 < 0f)), 4294967295u, (_e1231 > 4294967000f)) + _e1125.member_1.x)) / _e474), (f32((select(select(u32(_e1240), 0u, (_e1240 < 0f)), 4294967295u, (_e1240 > 4294967000f)) + _e1125.member_1.y)) / _e475), f32(_e1125.member_3));
            let _e1264 = textureSampleLevel(global_11, global_10, vec2<f32>(_e1258.x, _e1258.y), i32(_e1258.z), 0f);
            let _e1267 = select(_e1264, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e294.member_9 == 4294967295u)));
            if _e877 {
                phi_1972_ = vec3<f32>(0f, 0f, 0f);
                phi_1973_ = _e128;
            } else {
                let _e1271 = fma(_e879.x, 2f, -1f);
                let _e1272 = fma(_e879.y, 2f, -1f);
                let _e1273 = fma(_e879.z, 2f, -1f);
                let _e1278 = sqrt(fma(_e1273, _e1273, fma(_e1271, _e1271, (_e1272 * _e1272))));
                if (_e1278 == 0f) {
                    phi_5277_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5277_ = (vec3<f32>(_e1271, _e1272, _e1273) * (1f / _e1278));
                }
                let _e1283 = phi_5277_;
                let _e1290 = sqrt(fma(_e129.z, _e129.z, fma(_e129.x, _e129.x, (_e129.y * _e129.y))));
                if (_e1290 == 0f) {
                    phi_5312_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5312_ = (_e129 * (1f / _e1290));
                }
                let _e1295 = phi_5312_;
                let _e1302 = sqrt(fma(_e130.z, _e130.z, fma(_e130.x, _e130.x, (_e130.y * _e130.y))));
                if (_e1302 == 0f) {
                    phi_5347_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5347_ = (_e130 * (1f / _e1302));
                }
                let _e1307 = phi_5347_;
                let _e1314 = sqrt(fma(_e128.z, _e128.z, fma(_e128.x, _e128.x, (_e128.y * _e128.y))));
                if (_e1314 == 0f) {
                    phi_5382_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5382_ = (_e128 * (1f / _e1314));
                }
                let _e1319 = phi_5382_;
                let _e1338 = fma(_e1319.x, _e1283.z, fma(_e1295.x, _e1283.x, (_e1307.x * _e1283.y)));
                let _e1339 = fma(_e1319.y, _e1283.z, fma(_e1295.y, _e1283.x, (_e1307.y * _e1283.y)));
                let _e1340 = fma(_e1319.z, _e1283.z, fma(_e1295.z, _e1283.x, (_e1307.z * _e1283.y)));
                let _e1345 = sqrt(fma(_e1340, _e1340, fma(_e1338, _e1338, (_e1339 * _e1339))));
                if (_e1345 == 0f) {
                    phi_5417_ = vec3<f32>(0f, 0f, 0f);
                } else {
                    phi_5417_ = (vec3<f32>(_e1338, _e1339, _e1340) * (1f / _e1345));
                }
                let _e1350 = phi_5417_;
                phi_1972_ = _e1283;
                phi_1973_ = _e1350;
            }
            let _e1352 = phi_1972_;
            let _e1354 = phi_1973_;
            let _e1358 = (_e491.x * _e294.member_2.x);
            let _e1361 = (_e491.y * _e294.member_2.y);
            let _e1364 = (_e491.z * _e294.member_2.z);
            let _e1369 = (_e1358 * _e125.x);
            let _e1371 = (_e1361 * _e125.y);
            let _e1373 = (_e1364 * _e125.z);
            let _e1378 = (_e685.y * _e294.member_4);
            let _e1381 = (_e685.z * _e294.member_3);
            let _e1385 = fma(_e294.member_16, (select(_e1070, vec4<f32>(1f, 1f, 1f, 1f), vec4((_e294.member_8 == 4294967295u))).x - 1f), 1f);
            let _e1391 = (_e1267.x * _e294.member.x);
            let _e1393 = (_e1267.y * _e294.member.y);
            let _e1395 = (_e1267.z * _e294.member.z);
            let _e1400 = textureSampleLevel(global_12, global_13, _e1354, 0f);
            if (_e121 >= 86u) {
                phi_5449_ = (_e135 <= (_e121 - 86u));
            } else {
                phi_5449_ = false;
            }
            let _e1408 = phi_5449_;
            if _e1408 {
                let _e1411 = global.member[_e135];
                let _e1416 = global.member[(_e135 + 1u)];
                let _e1421 = global.member[(_e135 + 2u)];
                let _e1426 = global.member[(_e135 + 3u)];
                let _e1432 = global.member[(_e135 + 4u)];
                let _e1437 = global.member[(_e135 + 5u)];
                let _e1442 = global.member[(_e135 + 6u)];
                let _e1447 = global.member[(_e135 + 7u)];
                let _e1453 = global.member[(_e135 + 8u)];
                let _e1458 = global.member[(_e135 + 9u)];
                let _e1463 = global.member[(_e135 + 10u)];
                let _e1468 = global.member[(_e135 + 11u)];
                let _e1474 = global.member[(_e135 + 12u)];
                let _e1479 = global.member[(_e135 + 13u)];
                let _e1484 = global.member[(_e135 + 14u)];
                let _e1489 = global.member[(_e135 + 15u)];
                let _e1496 = global.member[(_e135 + 16u)];
                let _e1501 = global.member[(_e135 + 17u)];
                let _e1506 = global.member[(_e135 + 18u)];
                let _e1511 = global.member[(_e135 + 19u)];
                let _e1517 = global.member[(_e135 + 20u)];
                let _e1522 = global.member[(_e135 + 21u)];
                let _e1527 = global.member[(_e135 + 22u)];
                let _e1532 = global.member[(_e135 + 23u)];
                let _e1538 = global.member[(_e135 + 24u)];
                let _e1543 = global.member[(_e135 + 25u)];
                let _e1548 = global.member[(_e135 + 26u)];
                let _e1553 = global.member[(_e135 + 27u)];
                let _e1559 = global.member[(_e135 + 28u)];
                let _e1564 = global.member[(_e135 + 29u)];
                let _e1569 = global.member[(_e135 + 30u)];
                let _e1574 = global.member[(_e135 + 31u)];
                let _e1581 = global.member[(_e135 + 32u)];
                let _e1586 = global.member[(_e135 + 33u)];
                let _e1591 = global.member[(_e135 + 34u)];
                local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_2180_ = type_14(0u, 6u);
                loop {
                    let _e1596 = phi_2180_;
                    if (_e1596.member < _e1596.member_1) {
                        phi_2181_ = type_14((_e1596.member + 1u), _e1596.member_1);
                        phi_2204_ = type_14(1u, _e1596.member);
                    } else {
                        phi_2181_ = _e1596;
                        phi_2204_ = type_14(0u, type_14().member_1);
                    }
                    let _e1609 = phi_2181_;
                    let _e1611 = phi_2204_;
                    switch bitcast<i32>(_e1611.member) {
                        case 0: {
                            phi_2231_ = false;
                            break;
                        }
                        case 1: {
                            let _e1616 = ((_e135 + 35u) + (_e1611.member_1 * 4u));
                            let _e1619 = global.member[_e1616];
                            let _e1624 = global.member[(_e1616 + 1u)];
                            let _e1629 = global.member[(_e1616 + 2u)];
                            let _e1634 = global.member[(_e1616 + 3u)];
                            local_1[_e1611.member_1] = vec4<f32>(bitcast<f32>(_e1619), bitcast<f32>(_e1624), bitcast<f32>(_e1629), bitcast<f32>(_e1634));
                            phi_2231_ = true;
                            break;
                        }
                        default: {
                            phi_2231_ = bool();
                            break;
                        }
                    }
                    let _e1639 = phi_2231_;
                    continue;
                    continuing {
                        phi_2180_ = _e1609;
                        break if !(_e1639);
                    }
                }
                let _e1641 = local_1;
                local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                phi_2237_ = type_14(0u, 8u);
                loop {
                    let _e1644 = phi_2237_;
                    if (_e1644.member < _e1644.member_1) {
                        phi_2238_ = type_14((_e1644.member + 1u), _e1644.member_1);
                        phi_2261_ = type_14(1u, _e1644.member);
                    } else {
                        phi_2238_ = _e1644;
                        phi_2261_ = type_14(0u, type_14().member_1);
                    }
                    let _e1657 = phi_2238_;
                    let _e1659 = phi_2261_;
                    switch bitcast<i32>(_e1659.member) {
                        case 0: {
                            phi_2284_ = false;
                            break;
                        }
                        case 1: {
                            let _e1664 = ((_e135 + 59u) + (_e1659.member_1 * 3u));
                            let _e1667 = global.member[_e1664];
                            let _e1672 = global.member[(_e1664 + 1u)];
                            let _e1677 = global.member[(_e1664 + 2u)];
                            local[_e1659.member_1] = vec3<f32>(bitcast<f32>(_e1667), bitcast<f32>(_e1672), bitcast<f32>(_e1677));
                            phi_2284_ = true;
                            break;
                        }
                        default: {
                            phi_2284_ = bool();
                            break;
                        }
                    }
                    let _e1682 = phi_2284_;
                    continue;
                    continuing {
                        phi_2237_ = _e1657;
                        break if !(_e1682);
                    }
                }
                let _e1684 = local;
                let _e1688 = global.member[(_e135 + 83u)];
                let _e1693 = global.member[(_e135 + 84u)];
                let _e1698 = global.member[(_e135 + 85u)];
                phi_2305_ = type_25(type_23(vec4<f32>(bitcast<f32>(_e1411), bitcast<f32>(_e1416), bitcast<f32>(_e1421), bitcast<f32>(_e1426)), vec4<f32>(bitcast<f32>(_e1432), bitcast<f32>(_e1437), bitcast<f32>(_e1442), bitcast<f32>(_e1447)), vec4<f32>(bitcast<f32>(_e1453), bitcast<f32>(_e1458), bitcast<f32>(_e1463), bitcast<f32>(_e1468)), vec4<f32>(bitcast<f32>(_e1474), bitcast<f32>(_e1479), bitcast<f32>(_e1484), bitcast<f32>(_e1489))), type_23(vec4<f32>(bitcast<f32>(_e1496), bitcast<f32>(_e1501), bitcast<f32>(_e1506), bitcast<f32>(_e1511)), vec4<f32>(bitcast<f32>(_e1517), bitcast<f32>(_e1522), bitcast<f32>(_e1527), bitcast<f32>(_e1532)), vec4<f32>(bitcast<f32>(_e1538), bitcast<f32>(_e1543), bitcast<f32>(_e1548), bitcast<f32>(_e1553)), vec4<f32>(bitcast<f32>(_e1559), bitcast<f32>(_e1564), bitcast<f32>(_e1569), bitcast<f32>(_e1574))), vec3<f32>(bitcast<f32>(_e1581), bitcast<f32>(_e1586), bitcast<f32>(_e1591)), type_24(_e1684, _e1641, vec3<f32>(bitcast<f32>(_e1688), bitcast<f32>(_e1693), bitcast<f32>(_e1698))));
            } else {
                phi_2305_ = type_25(type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_23(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_24(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
            }
            let _e1704 = phi_2305_;
            let _e1706 = (_e1704.member_2 - _e131);
            let _e1713 = sqrt(fma(_e1706.z, _e1706.z, fma(_e1706.x, _e1706.x, (_e1706.y * _e1706.y))));
            let _e1714 = (_e1713 == 0f);
            if _e1714 {
                phi_5521_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5521_ = (_e1706 * (1f / _e1713));
            }
            let _e1718 = phi_5521_;
            let _e1719 = -(_e1718);
            let _e1726 = sqrt(fma(_e1354.z, _e1354.z, fma(_e1354.x, _e1354.x, (_e1354.y * _e1354.y))));
            let _e1727 = (_e1726 == 0f);
            if _e1727 {
                phi_5580_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5580_ = (_e1354 * (1f / _e1726));
            }
            let _e1731 = phi_5580_;
            let _e1741 = (2f * fma(_e1731.z, _e1719.z, fma(_e1731.x, _e1719.x, (_e1731.y * _e1719.y))));
            let _e1748 = textureSampleLevel(global_14, global_15, (_e1719 - vec3<f32>((_e1741 * _e1731.x), (_e1741 * _e1731.y), (_e1741 * _e1731.z))), (_e1378 * 4f));
            if _e1714 {
                phi_5654_ = vec3<f32>(0f, 0f, 0f);
            } else {
                phi_5654_ = (_e1706 * (1f / _e1713));
            }
            let _e1755 = phi_5654_;
            let _e1764 = textureSampleLevel(global_16, global_17, vec2<f32>(max(fma(_e1354.z, _e1755.z, fma(_e1354.x, _e1755.x, (_e1354.y * _e1755.y))), 0f), _e1378), 0f);
            switch bitcast<i32>(_e157) {
                case 0: {
                    if _e294.member_15 {
                        if _e1727 {
                            phi_6047_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_6047_ = (_e1354 * (1f / _e1726));
                        }
                        let _e1933 = phi_6047_;
                        if _e1714 {
                            phi_6082_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            phi_6082_ = (_e1706 * (1f / _e1713));
                        }
                        let _e1937 = phi_6082_;
                        let _e1940 = global_1.member[0u];
                        let _e1943 = global_1.member[1u];
                        let _e1946 = global_1.member[2u];
                        phi_7183_ = false;
                        phi_2372_ = type_14(0u, _e1943);
                        phi_2375_ = vec3<f32>(0f, 0f, 0f);
                        loop {
                            let _e1949 = phi_7183_;
                            let _e1951 = phi_2372_;
                            let _e1953 = phi_2375_;
                            local_3 = _e1953;
                            local_4 = _e1953;
                            local_5 = _e1953;
                            if (_e1951.member < _e1951.member_1) {
                                phi_2373_ = type_14((_e1951.member + 1u), _e1951.member_1);
                                phi_2398_ = type_14(1u, _e1951.member);
                            } else {
                                phi_2373_ = _e1951;
                                phi_2398_ = type_14(0u, type_14().member_1);
                            }
                            let _e1966 = phi_2373_;
                            let _e1968 = phi_2398_;
                            switch bitcast<i32>(_e1968.member) {
                                case 0: {
                                    phi_7239_ = _e1949;
                                    phi_2376_ = vec3<f32>();
                                    phi_3862_ = false;
                                    break;
                                }
                                case 1: {
                                    if (_e1968.member_1 >= _e1943) {
                                        phi_6108_ = 4294967295u;
                                    } else {
                                        phi_6108_ = (_e1940 + _e1968.member_1);
                                    }
                                    let _e1975 = phi_6108_;
                                    if (_e123 >= 1u) {
                                        phi_6127_ = (_e1975 <= (_e123 - 1u));
                                    } else {
                                        phi_6127_ = false;
                                    }
                                    let _e1980 = phi_6127_;
                                    if _e1980 {
                                        let _e1983 = global_1.member[_e1975];
                                        phi_2415_ = _e1983;
                                    } else {
                                        phi_2415_ = 4294967295u;
                                    }
                                    let _e1985 = phi_2415_;
                                    if (_e123 >= 4u) {
                                        phi_6151_ = (_e1985 <= (_e123 - 4u));
                                    } else {
                                        phi_6151_ = false;
                                    }
                                    let _e1990 = phi_6151_;
                                    if _e1990 {
                                        let _e1993 = global_1.member[_e1985];
                                        switch bitcast<i32>(_e1993) {
                                            case 0: {
                                                phi_2427_ = 0u;
                                                break;
                                            }
                                            case 1: {
                                                phi_2427_ = 1u;
                                                break;
                                            }
                                            case 2: {
                                                phi_2427_ = 2u;
                                                break;
                                            }
                                            default: {
                                                phi_2427_ = 0u;
                                                break;
                                            }
                                        }
                                        let _e1996 = phi_2427_;
                                        let _e2000 = global_1.member[(_e1985 + 1u)];
                                        let _e2004 = global_1.member[(_e1985 + 2u)];
                                        let _e2008 = global_1.member[(_e1985 + 3u)];
                                        phi_2441_ = type_30(_e1996, _e2000, _e2004, _e2008);
                                    } else {
                                        phi_2441_ = type_30(0u, 4294967295u, 4294967295u, 4294967295u);
                                    }
                                    let _e2011 = phi_2441_;
                                    if (_e123 >= 10u) {
                                        phi_6183_ = (_e2011.member_2 <= (_e123 - 10u));
                                    } else {
                                        phi_6183_ = false;
                                    }
                                    let _e2017 = phi_6183_;
                                    if _e2017 {
                                        let _e2020 = global_1.member[_e2011.member_2];
                                        let _e2025 = global_1.member[(_e2011.member_2 + 1u)];
                                        let _e2030 = global_1.member[(_e2011.member_2 + 2u)];
                                        let _e2036 = global_1.member[(_e2011.member_2 + 3u)];
                                        let _e2041 = global_1.member[(_e2011.member_2 + 4u)];
                                        let _e2046 = global_1.member[(_e2011.member_2 + 5u)];
                                        let _e2051 = global_1.member[(_e2011.member_2 + 6u)];
                                        let _e2057 = global_1.member[(_e2011.member_2 + 7u)];
                                        let _e2062 = global_1.member[(_e2011.member_2 + 8u)];
                                        let _e2067 = global_1.member[(_e2011.member_2 + 9u)];
                                        phi_2491_ = type_31(vec3<f32>(bitcast<f32>(_e2020), bitcast<f32>(_e2025), bitcast<f32>(_e2030)), vec4<f32>(bitcast<f32>(_e2036), bitcast<f32>(_e2041), bitcast<f32>(_e2046), bitcast<f32>(_e2051)), vec3<f32>(bitcast<f32>(_e2057), bitcast<f32>(_e2062), bitcast<f32>(_e2067)));
                                    } else {
                                        phi_2491_ = type_31(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                    }
                                    let _e2072 = phi_2491_;
                                    let _e2080 = (_e2072.member_1.x + _e2072.member_1.x);
                                    let _e2081 = (_e2072.member_1.y + _e2072.member_1.y);
                                    let _e2082 = (_e2072.member_1.z + _e2072.member_1.z);
                                    let _e2084 = (_e2072.member_1.z * _e2082);
                                    let _e2085 = (_e2072.member_1.w * _e2080);
                                    let _e2086 = (_e2072.member_1.w * _e2081);
                                    let _e2087 = (_e2072.member_1.w * _e2082);
                                    let _e2107 = (vec4<f32>((1f - fma(_e2072.member_1.y, _e2081, _e2084)), fma(_e2072.member_1.x, _e2081, _e2087), fma(_e2072.member_1.x, _e2082, -(_e2086)), 0f) * _e2072.member_2.x);
                                    let _e2109 = (vec4<f32>(fma(_e2072.member_1.x, _e2081, -(_e2087)), (1f - fma(_e2072.member_1.x, _e2080, _e2084)), fma(_e2072.member_1.y, _e2082, _e2085), 0f) * _e2072.member_2.y);
                                    let _e2111 = (vec4<f32>(fma(_e2072.member_1.x, _e2082, _e2086), fma(_e2072.member_1.y, _e2082, -(_e2085)), (1f - fma(_e2072.member_1.x, _e2080, (_e2072.member_1.y * _e2081))), 0f) * _e2072.member_2.z);
                                    switch bitcast<i32>(_e2011.member) {
                                        case 0: {
                                            if (_e123 >= 8u) {
                                                phi_6649_ = (_e2011.member_1 <= (_e123 - 8u));
                                            } else {
                                                phi_6649_ = false;
                                            }
                                            let _e2609 = phi_6649_;
                                            if _e2609 {
                                                let _e2612 = global_1.member[_e2011.member_1];
                                                let _e2617 = global_1.member[(_e2011.member_1 + 1u)];
                                                let _e2622 = global_1.member[(_e2011.member_1 + 2u)];
                                                let _e2628 = global_1.member[(_e2011.member_1 + 3u)];
                                                let _e2633 = global_1.member[(_e2011.member_1 + 4u)];
                                                let _e2638 = global_1.member[(_e2011.member_1 + 5u)];
                                                let _e2643 = global_1.member[(_e2011.member_1 + 6u)];
                                                let _e2649 = global_1.member[(_e2011.member_1 + 7u)];
                                                phi_2539_ = type_34(vec3<f32>(bitcast<f32>(_e2612), bitcast<f32>(_e2617), bitcast<f32>(_e2622)), vec4<f32>(bitcast<f32>(_e2628), bitcast<f32>(_e2633), bitcast<f32>(_e2638), bitcast<f32>(_e2643)), bitcast<f32>(_e2649));
                                            } else {
                                                phi_2539_ = type_34(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2653 = phi_2539_;
                                            let _e2675 = fma(_e2111.x, _e2653.member.z, fma(_e2109.x, _e2653.member.y, (_e2107.x * _e2653.member.x)));
                                            let _e2676 = fma(_e2111.y, _e2653.member.z, fma(_e2109.y, _e2653.member.y, (_e2107.y * _e2653.member.x)));
                                            let _e2677 = fma(_e2111.z, _e2653.member.z, fma(_e2109.z, _e2653.member.y, (_e2107.z * _e2653.member.x)));
                                            let _e2682 = sqrt(fma(_e2677, _e2677, fma(_e2675, _e2675, (_e2676 * _e2676))));
                                            if (_e2682 == 0f) {
                                                phi_6696_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6696_ = (vec3<f32>(_e2675, _e2676, _e2677) * (1f / _e2682));
                                            }
                                            let _e2687 = phi_6696_;
                                            let _e2689 = -(_e2687.x);
                                            let _e2691 = -(_e2687.y);
                                            let _e2693 = -(_e2687.z);
                                            let _e2694 = -(_e2687);
                                            let _e2696 = fma(-(_e685.z), _e294.member_3, 1f);
                                            let _e2700 = fma(0.4f, _e2696, (_e1369 * _e1381));
                                            let _e2701 = fma(0.4f, _e2696, (_e1371 * _e1381));
                                            let _e2702 = fma(0.4f, _e2696, (_e1373 * _e1381));
                                            let _e2710 = (_e1937 + vec3<f32>(_e2689, _e2691, _e2693));
                                            let _e2717 = sqrt(fma(_e2710.z, _e2710.z, fma(_e2710.x, _e2710.x, (_e2710.y * _e2710.y))));
                                            if (_e2717 == 0f) {
                                                phi_6731_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_6731_ = (_e2710 * (1f / _e2717));
                                            }
                                            let _e2722 = phi_6731_;
                                            let _e2723 = (_e1378 * _e1378);
                                            let _e2734 = max(fma(_e1933.z, _e2722.z, fma(_e1933.x, _e2722.x, (_e1933.y * _e2722.y))), 0f);
                                            let _e2747 = max(fma(_e1933.z, _e1937.z, fma(_e1933.x, _e1937.x, (_e1933.y * _e1937.y))), 0f);
                                            let _e2753 = fma(_e1933.z, _e2694.z, fma(_e1933.x, _e2694.x, (_e1933.y * _e2694.y)));
                                            let _e2754 = max(_e2753, 0f);
                                            let _e2755 = fma(_e685.y, _e294.member_4, 1f);
                                            let _e2756 = (_e2755 * _e2755);
                                            let _e2757 = (_e2756 * 0.125f);
                                            let _e2759 = fma(-(_e2756), 0.125f, 1f);
                                            let _e2772 = (1f - max(fma(_e2722.z, _e1937.z, fma(_e2722.x, _e1937.x, (_e2722.y * _e1937.y))), 0f));
                                            let _e2774 = select(_e2772, 0f, (_e2772 < 0f));
                                            let _e2777 = pow(select(_e2774, 1f, (_e2774 > 1f)), 5f);
                                            let _e2778 = fma((1f - _e2700), _e2777, _e2700);
                                            let _e2779 = fma((1f - _e2701), _e2777, _e2701);
                                            let _e2780 = fma((1f - _e2702), _e2777, _e2702);
                                            let _e2787 = (((_e2723 * _e2723) / (pow(fma((_e2734 * _e2734), fma(_e2723, _e2723, -1f), 1f), 2f) * 3.1415927f)) * ((_e2747 / fma(_e2747, _e2759, _e2757)) * (_e2754 / fma(_e2754, _e2759, _e2757))));
                                            let _e2794 = max(fma(_e1933.z, _e2693, fma(_e1933.x, _e2689, (_e1933.y * _e2691))), 0f);
                                            let _e2796 = fma((4f * _e2747), _e2794, 0.0001f);
                                            if ((_e2011.member_3 == 4294967295u) != true) {
                                                let _e2817 = global_1.member[_e2011.member_3];
                                                let _e2821 = global_1.member[(_e2011.member_3 + 1u)];
                                                let _e2825 = global_1.member[(_e2011.member_3 + 2u)];
                                                let _e2829 = global_1.member[(_e2011.member_3 + 3u)];
                                                let _e2833 = global_1.member[(_e2011.member_3 + 4u)];
                                                let _e2838 = global_1.member[(_e2011.member_3 + 5u)];
                                                let _e2843 = global_1.member[(_e2011.member_3 + 6u)];
                                                let _e2848 = global_1.member[select(_e2825, 4294967295u, (0u >= _e2829))];
                                                let _e2851 = global_1.member[_e2848];
                                                let _e2855 = global_1.member[(_e2848 + 1u)];
                                                let _e2859 = global_1.member[(_e2848 + 2u)];
                                                let _e2863 = global_1.member[(_e2848 + 3u)];
                                                let _e2867 = global_1.member[(_e2848 + 4u)];
                                                let _e2871 = global_1.member[(_e2848 + 6u)];
                                                switch bitcast<i32>(_e2871) {
                                                    case 0: {
                                                        phi_2780_ = 0u;
                                                        break;
                                                    }
                                                    case 1: {
                                                        phi_2780_ = 1u;
                                                        break;
                                                    }
                                                    case 2: {
                                                        phi_2780_ = 2u;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_2780_ = 0u;
                                                        break;
                                                    }
                                                }
                                                let _e2874 = phi_2780_;
                                                let _e2878 = global_1.member[(_e2848 + 7u)];
                                                switch bitcast<i32>(_e2878) {
                                                    case 0: {
                                                        phi_2789_ = 0u;
                                                        break;
                                                    }
                                                    case 1: {
                                                        phi_2789_ = 1u;
                                                        break;
                                                    }
                                                    case 2: {
                                                        phi_2789_ = 2u;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_2789_ = 0u;
                                                        break;
                                                    }
                                                }
                                                let _e2881 = phi_2789_;
                                                let _e2884 = global_1.member[_e1946];
                                                let _e2888 = global_1.member[(_e1946 + 1u)];
                                                let _e2890 = select(_e2817, 4294967295u, (0u >= _e2821));
                                                let _e2893 = global_1.member[_e2890];
                                                let _e2898 = global_1.member[(_e2890 + 1u)];
                                                let _e2903 = global_1.member[(_e2890 + 2u)];
                                                let _e2908 = global_1.member[(_e2890 + 3u)];
                                                let _e2913 = global_1.member[(_e2890 + 4u)];
                                                let _e2918 = global_1.member[(_e2890 + 5u)];
                                                let _e2923 = global_1.member[(_e2890 + 6u)];
                                                let _e2928 = global_1.member[(_e2890 + 7u)];
                                                let _e2933 = global_1.member[(_e2890 + 8u)];
                                                let _e2938 = global_1.member[(_e2890 + 9u)];
                                                let _e2943 = global_1.member[(_e2890 + 10u)];
                                                let _e2948 = global_1.member[(_e2890 + 11u)];
                                                let _e2953 = global_1.member[(_e2890 + 12u)];
                                                let _e2958 = global_1.member[(_e2890 + 13u)];
                                                let _e2963 = global_1.member[(_e2890 + 14u)];
                                                let _e2968 = global_1.member[(_e2890 + 15u)];
                                                let _e2988 = (bitcast<f32>(_e2968) + fma(bitcast<f32>(_e2948), _e131.z, fma(bitcast<f32>(_e2928), _e131.y, (bitcast<f32>(_e2908) * _e131.x))));
                                                let _e2989 = ((bitcast<f32>(_e2953) + fma(bitcast<f32>(_e2933), _e131.z, fma(bitcast<f32>(_e2913), _e131.y, (bitcast<f32>(_e2893) * _e131.x)))) / _e2988);
                                                let _e2990 = ((bitcast<f32>(_e2958) + fma(bitcast<f32>(_e2938), _e131.z, fma(bitcast<f32>(_e2918), _e131.y, (bitcast<f32>(_e2898) * _e131.x)))) / _e2988);
                                                let _e2991 = ((bitcast<f32>(_e2963) + fma(bitcast<f32>(_e2943), _e131.z, fma(bitcast<f32>(_e2923), _e131.y, (bitcast<f32>(_e2903) * _e131.x)))) / _e2988);
                                                if (abs(_e2989) <= 1f) {
                                                    let _e2995 = (abs(_e2990) <= 1f);
                                                    if _e2995 {
                                                        phi_6840_ = (abs(_e2991) <= 1f);
                                                    } else {
                                                        phi_6840_ = bool();
                                                    }
                                                    let _e2999 = phi_6840_;
                                                    phi_6843_ = _e2999;
                                                    phi_6844_ = select(true, false, _e2995);
                                                } else {
                                                    phi_6843_ = bool();
                                                    phi_6844_ = true;
                                                }
                                                let _e3002 = phi_6843_;
                                                let _e3004 = phi_6844_;
                                                if select(_e3002, false, _e3004) {
                                                    let _e3008 = bitcast<i32>(_e2843);
                                                    let _e3009 = (_e3008 / 2i);
                                                    let _e3010 = f32(_e2859);
                                                    let _e3011 = f32(_e2863);
                                                    let _e3015 = type_35((_e3008 / -2i), _e3009, false);
                                                    phi_7161_ = _e1949;
                                                    phi_2968_ = _e3015;
                                                    phi_2971_ = 0f;
                                                    loop {
                                                        let _e3017 = phi_7161_;
                                                        let _e3019 = phi_2968_;
                                                        let _e3021 = phi_2971_;
                                                        local_2 = _e3021;
                                                        if _e3019.member_2 {
                                                            phi_2983_ = true;
                                                        } else {
                                                            phi_2983_ = ((_e3019.member <= _e3019.member_1) != true);
                                                        }
                                                        let _e3028 = phi_2983_;
                                                        if _e3028 {
                                                            phi_2969_ = _e3019;
                                                            phi_3026_ = type_36(0u, type_36().member_1);
                                                        } else {
                                                            if (_e3019.member < _e3019.member_1) {
                                                                let _e3036 = (_e3019.member + 1i);
                                                                if select(false, true, ((false == (_e3036 > _e3019.member)) != false)) {
                                                                    phi_3011_ = type_36(0u, type_36().member_1);
                                                                } else {
                                                                    phi_3011_ = type_36(1u, _e3036);
                                                                }
                                                                let _e3046 = phi_3011_;
                                                                switch bitcast<i32>(_e3046.member) {
                                                                    case 0: {
                                                                        phi_7236_ = true;
                                                                        break;
                                                                    }
                                                                    case 1: {
                                                                        break;
                                                                    }
                                                                    default: {
                                                                        break;
                                                                    }
                                                                }
                                                                phi_3023_ = type_35(_e3046.member_1, _e3019.member_1, _e3019.member_2);
                                                            } else {
                                                                phi_3023_ = type_35(_e3019.member, _e3019.member_1, true);
                                                            }
                                                            let _e3055 = phi_3023_;
                                                            phi_2969_ = _e3055;
                                                            phi_3026_ = type_36(1u, _e3019.member);
                                                        }
                                                        let _e3061 = phi_2969_;
                                                        let _e3063 = phi_3026_;
                                                        switch bitcast<i32>(_e3063.member) {
                                                            case 0: {
                                                                phi_7237_ = _e3017;
                                                                phi_2972_ = f32();
                                                                phi_3282_ = false;
                                                                break;
                                                            }
                                                            case 1: {
                                                                phi_3037_ = _e3015;
                                                                phi_3040_ = _e3021;
                                                                loop {
                                                                    let _e3068 = phi_3037_;
                                                                    let _e3070 = phi_3040_;
                                                                    local_6 = _e3070;
                                                                    if _e3068.member_2 {
                                                                        phi_3052_ = true;
                                                                    } else {
                                                                        phi_3052_ = ((_e3068.member <= _e3068.member_1) != true);
                                                                    }
                                                                    let _e3077 = phi_3052_;
                                                                    if _e3077 {
                                                                        phi_3038_ = _e3068;
                                                                        phi_3095_ = type_36(0u, type_36().member_1);
                                                                    } else {
                                                                        if (_e3068.member < _e3068.member_1) {
                                                                            let _e3085 = (_e3068.member + 1i);
                                                                            if select(false, true, ((false == (_e3085 > _e3068.member)) != false)) {
                                                                                phi_3080_ = type_36(0u, type_36().member_1);
                                                                            } else {
                                                                                phi_3080_ = type_36(1u, _e3085);
                                                                            }
                                                                            let _e3095 = phi_3080_;
                                                                            switch bitcast<i32>(_e3095.member) {
                                                                                case 0: {
                                                                                    phi_7145_ = true;
                                                                                    break;
                                                                                }
                                                                                case 1: {
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            phi_3092_ = type_35(_e3095.member_1, _e3068.member_1, _e3068.member_2);
                                                                        } else {
                                                                            phi_3092_ = type_35(_e3068.member, _e3068.member_1, true);
                                                                        }
                                                                        let _e3104 = phi_3092_;
                                                                        phi_3038_ = _e3104;
                                                                        phi_3095_ = type_36(1u, _e3068.member);
                                                                    }
                                                                    let _e3110 = phi_3038_;
                                                                    let _e3112 = phi_3095_;
                                                                    switch bitcast<i32>(_e3112.member) {
                                                                        case 0: {
                                                                            phi_3041_ = f32();
                                                                            phi_3281_ = false;
                                                                            break;
                                                                        }
                                                                        case 1: {
                                                                            let _e3120 = fma((_e2989 + 1f), 0.5f, (f32(_e3063.member_1) * (1f / _e3010)));
                                                                            let _e3121 = fma(fma(_e2990, -1f, 1f), 0.5f, (f32(_e3112.member_1) * (1f / _e3011)));
                                                                            switch bitcast<i32>(_e2874) {
                                                                                case 1: {
                                                                                    let _e3156 = abs(_e3120);
                                                                                    let _e3158 = (_e3156 % 1f);
                                                                                    if (_e3156 >= 1f) {
                                                                                        phi_6892_ = select(true, false, (_e3158 == 0f));
                                                                                    } else {
                                                                                        phi_6892_ = true;
                                                                                    }
                                                                                    let _e3162 = phi_6892_;
                                                                                    let _e3163 = select(1f, _e3158, _e3162);
                                                                                    if (select(-1f, 1f, (_e3120 >= 0f)) > 0f) {
                                                                                        phi_3127_ = _e3163;
                                                                                    } else {
                                                                                        phi_3127_ = (1f - _e3163);
                                                                                    }
                                                                                    let _e3167 = phi_3127_;
                                                                                    phi_3164_ = _e3167;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e3130 = abs(_e3120);
                                                                                    let _e3137 = ((select(select(u32(_e3130), 0u, (_e3130 < 0f)), 4294967295u, (_e3130 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e3139 = (_e3130 % 1f);
                                                                                    if (_e3130 >= 1f) {
                                                                                        phi_6875_ = select(true, false, (_e3139 == 0f));
                                                                                    } else {
                                                                                        phi_6875_ = true;
                                                                                    }
                                                                                    let _e3143 = phi_6875_;
                                                                                    let _e3144 = select(1f, _e3139, _e3143);
                                                                                    if (select(-1f, 1f, (_e3120 >= 0f)) > 0f) {
                                                                                        if _e3137 {
                                                                                            phi_3156_ = _e3144;
                                                                                        } else {
                                                                                            phi_3156_ = (1f - _e3144);
                                                                                        }
                                                                                        let _e3151 = phi_3156_;
                                                                                        phi_3162_ = _e3151;
                                                                                    } else {
                                                                                        if _e3137 {
                                                                                            phi_3161_ = (1f - _e3144);
                                                                                        } else {
                                                                                            phi_3161_ = _e3144;
                                                                                        }
                                                                                        let _e3148 = phi_3161_;
                                                                                        phi_3162_ = _e3148;
                                                                                    }
                                                                                    let _e3153 = phi_3162_;
                                                                                    phi_3164_ = _e3153;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e3120 > 1f) {
                                                                                        phi_6862_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_6862_ = select(_e3120, 0.00000011920929f, (_e3120 < 0f));
                                                                                    }
                                                                                    let _e3127 = phi_6862_;
                                                                                    phi_3164_ = _e3127;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3164_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e3169 = phi_3164_;
                                                                            switch bitcast<i32>(_e2881) {
                                                                                case 1: {
                                                                                    let _e3204 = abs(_e3121);
                                                                                    let _e3206 = (_e3204 % 1f);
                                                                                    if (_e3204 >= 1f) {
                                                                                        phi_6940_ = select(true, false, (_e3206 == 0f));
                                                                                    } else {
                                                                                        phi_6940_ = true;
                                                                                    }
                                                                                    let _e3210 = phi_6940_;
                                                                                    let _e3211 = select(1f, _e3206, _e3210);
                                                                                    if (select(-1f, 1f, (_e3121 >= 0f)) > 0f) {
                                                                                        phi_3183_ = _e3211;
                                                                                    } else {
                                                                                        phi_3183_ = (1f - _e3211);
                                                                                    }
                                                                                    let _e3215 = phi_3183_;
                                                                                    phi_3220_ = _e3215;
                                                                                    break;
                                                                                }
                                                                                case 2: {
                                                                                    let _e3178 = abs(_e3121);
                                                                                    let _e3185 = ((select(select(u32(_e3178), 0u, (_e3178 < 0f)), 4294967295u, (_e3178 > 4294967000f)) % 2u) == 0u);
                                                                                    let _e3187 = (_e3178 % 1f);
                                                                                    if (_e3178 >= 1f) {
                                                                                        phi_6923_ = select(true, false, (_e3187 == 0f));
                                                                                    } else {
                                                                                        phi_6923_ = true;
                                                                                    }
                                                                                    let _e3191 = phi_6923_;
                                                                                    let _e3192 = select(1f, _e3187, _e3191);
                                                                                    if (select(-1f, 1f, (_e3121 >= 0f)) > 0f) {
                                                                                        if _e3185 {
                                                                                            phi_3212_ = _e3192;
                                                                                        } else {
                                                                                            phi_3212_ = (1f - _e3192);
                                                                                        }
                                                                                        let _e3199 = phi_3212_;
                                                                                        phi_3218_ = _e3199;
                                                                                    } else {
                                                                                        if _e3185 {
                                                                                            phi_3217_ = (1f - _e3192);
                                                                                        } else {
                                                                                            phi_3217_ = _e3192;
                                                                                        }
                                                                                        let _e3196 = phi_3217_;
                                                                                        phi_3218_ = _e3196;
                                                                                    }
                                                                                    let _e3201 = phi_3218_;
                                                                                    phi_3220_ = _e3201;
                                                                                    break;
                                                                                }
                                                                                case 0: {
                                                                                    if (_e3121 > 1f) {
                                                                                        phi_6910_ = 0.9999999f;
                                                                                    } else {
                                                                                        phi_6910_ = select(_e3121, 0.00000011920929f, (_e3121 < 0f));
                                                                                    }
                                                                                    let _e3175 = phi_6910_;
                                                                                    phi_3220_ = _e3175;
                                                                                    break;
                                                                                }
                                                                                default: {
                                                                                    phi_3220_ = f32();
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e3217 = phi_3220_;
                                                                            let _e3218 = (_e3169 * _e3010);
                                                                            let _e3224 = (_e3217 * _e3011);
                                                                            let _e3239 = vec3<f32>((f32((select(select(u32(_e3218), 0u, (_e3218 < 0f)), 4294967295u, (_e3218 > 4294967000f)) + _e2851)) / f32(_e2884)), (f32((select(select(u32(_e3224), 0u, (_e3224 < 0f)), 4294967295u, (_e3224 > 4294967000f)) + _e2855)) / f32(_e2888)), f32(_e2867));
                                                                            let _e3245 = textureSampleLevel(global_19, global_18, vec2<f32>(_e3239.x, _e3239.y), i32(_e3239.z), 0f);
                                                                            phi_3041_ = (_e3070 + select(0f, 1f, ((_e2991 - max((bitcast<f32>(_e2838) * (1f - _e2753)), bitcast<f32>(_e2833))) > _e3245.x)));
                                                                            phi_3281_ = true;
                                                                            break;
                                                                        }
                                                                        default: {
                                                                            phi_3041_ = f32();
                                                                            phi_3281_ = bool();
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e3255 = phi_3041_;
                                                                    let _e3257 = phi_3281_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_3037_ = _e3110;
                                                                        phi_3040_ = _e3255;
                                                                        phi_7145_ = _e3017;
                                                                        break if !(_e3257);
                                                                    }
                                                                }
                                                                let _e3260 = phi_7145_;
                                                                phi_7236_ = _e3260;
                                                                if _e3260 {
                                                                    break;
                                                                }
                                                                phi_7237_ = _e3260;
                                                                let _e3588 = local_6;
                                                                phi_2972_ = _e3588;
                                                                phi_3282_ = true;
                                                                break;
                                                            }
                                                            default: {
                                                                phi_7237_ = _e3017;
                                                                phi_2972_ = f32();
                                                                phi_3282_ = bool();
                                                                break;
                                                            }
                                                        }
                                                        let _e3262 = phi_7237_;
                                                        let _e3264 = phi_2972_;
                                                        let _e3266 = phi_3282_;
                                                        continue;
                                                        continuing {
                                                            phi_7161_ = _e3262;
                                                            phi_2968_ = _e3061;
                                                            phi_2971_ = _e3264;
                                                            phi_7236_ = _e3262;
                                                            break if !(_e3266);
                                                        }
                                                    }
                                                    let _e3269 = phi_7236_;
                                                    phi_7238_ = _e3269;
                                                    if _e3269 {
                                                        break;
                                                    }
                                                    let _e3274 = local_2;
                                                    phi_7264_ = _e3269;
                                                    phi_3287_ = (_e3274 / (f32((_e3009 * 2i)) + 1f));
                                                } else {
                                                    phi_7264_ = _e1949;
                                                    phi_3287_ = 0f;
                                                }
                                                let _e3277 = phi_7264_;
                                                let _e3279 = phi_3287_;
                                                phi_7263_ = _e3277;
                                                phi_3288_ = _e3279;
                                            } else {
                                                phi_7263_ = _e1949;
                                                phi_3288_ = 0f;
                                            }
                                            let _e3281 = phi_7263_;
                                            let _e3283 = phi_3288_;
                                            let _e3284 = (1f - _e3283);
                                            phi_7242_ = _e3281;
                                            phi_3852_ = vec3<f32>(fma(((fma((((1f - _e2778) * _e2696) * _e1369), 0.31830987f, ((_e2787 * _e2778) / _e2796)) * (_e2653.member_1.x * _e2653.member_2)) * _e2794), _e3284, _e1953.x), fma(((fma((((1f - _e2779) * _e2696) * _e1371), 0.31830987f, ((_e2787 * _e2779) / _e2796)) * (_e2653.member_1.y * _e2653.member_2)) * _e2794), _e3284, _e1953.y), fma(((fma((((1f - _e2780) * _e2696) * _e1373), 0.31830987f, ((_e2787 * _e2780) / _e2796)) * (_e2653.member_1.z * _e2653.member_2)) * _e2794), _e3284, _e1953.z));
                                            phi_3853_ = true;
                                            break;
                                        }
                                        case 1: {
                                            if (_e123 >= 8u) {
                                                phi_6475_ = (_e2011.member_1 <= (_e123 - 8u));
                                            } else {
                                                phi_6475_ = false;
                                            }
                                            let _e2397 = phi_6475_;
                                            if _e2397 {
                                                let _e2400 = global_1.member[_e2011.member_1];
                                                let _e2405 = global_1.member[(_e2011.member_1 + 1u)];
                                                let _e2410 = global_1.member[(_e2011.member_1 + 2u)];
                                                let _e2416 = global_1.member[(_e2011.member_1 + 3u)];
                                                let _e2421 = global_1.member[(_e2011.member_1 + 4u)];
                                                let _e2426 = global_1.member[(_e2011.member_1 + 5u)];
                                                let _e2431 = global_1.member[(_e2011.member_1 + 6u)];
                                                let _e2437 = global_1.member[(_e2011.member_1 + 7u)];
                                                phi_3342_ = type_34(vec3<f32>(bitcast<f32>(_e2400), bitcast<f32>(_e2405), bitcast<f32>(_e2410)), vec4<f32>(bitcast<f32>(_e2416), bitcast<f32>(_e2421), bitcast<f32>(_e2426), bitcast<f32>(_e2431)), bitcast<f32>(_e2437));
                                            } else {
                                                phi_3342_ = type_34(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2441 = phi_3342_;
                                            let _e2470 = (vec3<f32>((_e2072.member.x + fma(_e2111.x, _e2441.member.z, fma(_e2109.x, _e2441.member.y, (_e2107.x * _e2441.member.x)))), (_e2072.member.y + fma(_e2111.y, _e2441.member.z, fma(_e2109.y, _e2441.member.y, (_e2107.y * _e2441.member.x)))), (_e2072.member.z + fma(_e2111.z, _e2441.member.z, fma(_e2109.z, _e2441.member.y, (_e2107.z * _e2441.member.x))))) - _e131);
                                            let _e2477 = sqrt(fma(_e2470.z, _e2470.z, fma(_e2470.x, _e2470.x, (_e2470.y * _e2470.y))));
                                            let _e2478 = (_e2477 == 0f);
                                            if _e2478 {
                                                phi_3532_ = vec3<f32>();
                                            } else {
                                                if _e2478 {
                                                    phi_6522_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6522_ = (_e2470 * (1f / _e2477));
                                                }
                                                let _e2482 = phi_6522_;
                                                let _e2484 = (_e2441.member_2 / (_e2477 * _e2477));
                                                let _e2486 = fma(-(_e685.z), _e294.member_3, 1f);
                                                let _e2490 = fma(0.4f, _e2486, (_e1369 * _e1381));
                                                let _e2491 = fma(0.4f, _e2486, (_e1371 * _e1381));
                                                let _e2492 = fma(0.4f, _e2486, (_e1373 * _e1381));
                                                let _e2499 = (_e1937 + _e2482);
                                                let _e2506 = sqrt(fma(_e2499.z, _e2499.z, fma(_e2499.x, _e2499.x, (_e2499.y * _e2499.y))));
                                                if (_e2506 == 0f) {
                                                    phi_6557_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6557_ = (_e2499 * (1f / _e2506));
                                                }
                                                let _e2511 = phi_6557_;
                                                let _e2512 = (_e1378 * _e1378);
                                                let _e2523 = max(fma(_e1933.z, _e2511.z, fma(_e1933.x, _e2511.x, (_e1933.y * _e2511.y))), 0f);
                                                let _e2536 = max(fma(_e1933.z, _e1937.z, fma(_e1933.x, _e1937.x, (_e1933.y * _e1937.y))), 0f);
                                                let _e2543 = max(fma(_e1933.z, _e2482.z, fma(_e1933.x, _e2482.x, (_e1933.y * _e2482.y))), 0f);
                                                let _e2544 = fma(_e685.y, _e294.member_4, 1f);
                                                let _e2545 = (_e2544 * _e2544);
                                                let _e2546 = (_e2545 * 0.125f);
                                                let _e2548 = fma(-(_e2545), 0.125f, 1f);
                                                let _e2561 = (1f - max(fma(_e2511.z, _e1937.z, fma(_e2511.x, _e1937.x, (_e2511.y * _e1937.y))), 0f));
                                                let _e2563 = select(_e2561, 0f, (_e2561 < 0f));
                                                let _e2566 = pow(select(_e2563, 1f, (_e2563 > 1f)), 5f);
                                                let _e2567 = fma((1f - _e2490), _e2566, _e2490);
                                                let _e2568 = fma((1f - _e2491), _e2566, _e2491);
                                                let _e2569 = fma((1f - _e2492), _e2566, _e2492);
                                                let _e2576 = (((_e2512 * _e2512) / (pow(fma((_e2523 * _e2523), fma(_e2512, _e2512, -1f), 1f), 2f) * 3.1415927f)) * ((_e2536 / fma(_e2536, _e2548, _e2546)) * (_e2543 / fma(_e2543, _e2548, _e2546))));
                                                let _e2581 = fma((4f * _e2536), _e2543, 0.0001f);
                                                phi_3532_ = vec3<f32>(fma((fma((((1f - _e2567) * _e2486) * _e1369), 0.31830987f, ((_e2576 * _e2567) / _e2581)) * (_e2441.member_1.x * _e2484)), _e2543, _e1953.x), fma((fma((((1f - _e2568) * _e2486) * _e1371), 0.31830987f, ((_e2576 * _e2568) / _e2581)) * (_e2441.member_1.y * _e2484)), _e2543, _e1953.y), fma((fma((((1f - _e2569) * _e2486) * _e1373), 0.31830987f, ((_e2576 * _e2569) / _e2581)) * (_e2441.member_1.z * _e2484)), _e2543, _e1953.z));
                                            }
                                            let _e2602 = phi_3532_;
                                            phi_7242_ = _e1949;
                                            phi_3852_ = _e2602;
                                            phi_3853_ = select(true, false, _e2478);
                                            break;
                                        }
                                        case 2: {
                                            if (_e123 >= 13u) {
                                                phi_6263_ = (_e2011.member_1 <= (_e123 - 13u));
                                            } else {
                                                phi_6263_ = false;
                                            }
                                            let _e2122 = phi_6263_;
                                            if _e2122 {
                                                let _e2125 = global_1.member[_e2011.member_1];
                                                let _e2130 = global_1.member[(_e2011.member_1 + 1u)];
                                                let _e2135 = global_1.member[(_e2011.member_1 + 2u)];
                                                let _e2141 = global_1.member[(_e2011.member_1 + 3u)];
                                                let _e2146 = global_1.member[(_e2011.member_1 + 4u)];
                                                let _e2151 = global_1.member[(_e2011.member_1 + 5u)];
                                                let _e2157 = global_1.member[(_e2011.member_1 + 6u)];
                                                let _e2162 = global_1.member[(_e2011.member_1 + 7u)];
                                                let _e2167 = global_1.member[(_e2011.member_1 + 8u)];
                                                let _e2172 = global_1.member[(_e2011.member_1 + 9u)];
                                                let _e2177 = global_1.member[(_e2011.member_1 + 10u)];
                                                let _e2182 = global_1.member[(_e2011.member_1 + 11u)];
                                                let _e2188 = global_1.member[(_e2011.member_1 + 12u)];
                                                phi_3595_ = type_37(vec3<f32>(bitcast<f32>(_e2125), bitcast<f32>(_e2130), bitcast<f32>(_e2135)), vec3<f32>(bitcast<f32>(_e2141), bitcast<f32>(_e2146), bitcast<f32>(_e2151)), bitcast<f32>(_e2157), bitcast<f32>(_e2162), vec4<f32>(bitcast<f32>(_e2167), bitcast<f32>(_e2172), bitcast<f32>(_e2177), bitcast<f32>(_e2182)), bitcast<f32>(_e2188));
                                            } else {
                                                phi_3595_ = type_37(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 1.0471976f, 1.5707964f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                            }
                                            let _e2192 = phi_3595_;
                                            let _e2224 = (vec3<f32>((_e2072.member.x + fma(_e2111.x, _e2192.member.z, fma(_e2109.x, _e2192.member.y, (_e2107.x * _e2192.member.x)))), (_e2072.member.y + fma(_e2111.y, _e2192.member.z, fma(_e2109.y, _e2192.member.y, (_e2107.y * _e2192.member.x)))), (_e2072.member.z + fma(_e2111.z, _e2192.member.z, fma(_e2109.z, _e2192.member.y, (_e2107.z * _e2192.member.x))))) - _e131);
                                            let _e2231 = sqrt(fma(_e2224.z, _e2224.z, fma(_e2224.x, _e2224.x, (_e2224.y * _e2224.y))));
                                            let _e2232 = (_e2231 == 0f);
                                            if _e2232 {
                                                phi_3850_ = vec3<f32>();
                                            } else {
                                                if _e2232 {
                                                    phi_6313_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6313_ = (_e2224 * (1f / _e2231));
                                                }
                                                let _e2236 = phi_6313_;
                                                let _e2246 = fma(_e2111.x, _e2192.member_1.z, fma(_e2109.x, _e2192.member_1.y, (_e2107.x * _e2192.member_1.x)));
                                                let _e2247 = fma(_e2111.y, _e2192.member_1.z, fma(_e2109.y, _e2192.member_1.y, (_e2107.y * _e2192.member_1.x)));
                                                let _e2248 = fma(_e2111.z, _e2192.member_1.z, fma(_e2109.z, _e2192.member_1.y, (_e2107.z * _e2192.member_1.x)));
                                                let _e2253 = sqrt(fma(_e2248, _e2248, fma(_e2246, _e2246, (_e2247 * _e2247))));
                                                if (_e2253 == 0f) {
                                                    phi_6348_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6348_ = (vec3<f32>(_e2246, _e2247, _e2248) * (1f / _e2253));
                                                }
                                                let _e2258 = phi_6348_;
                                                let _e2270 = ((fma(_e2236.z, _e2258.z, fma(_e2236.x, _e2258.x, (_e2236.y * _e2258.y))) - _e2192.member_3) / (_e2192.member_2 - _e2192.member_3));
                                                let _e2272 = select(_e2270, 0f, (_e2270 < 0f));
                                                let _e2275 = (_e2192.member_5 * select(_e2272, 1f, (_e2272 > 1f)));
                                                let _e2277 = fma(-(_e685.z), _e294.member_3, 1f);
                                                let _e2281 = fma(0.4f, _e2277, (_e1369 * _e1381));
                                                let _e2282 = fma(0.4f, _e2277, (_e1371 * _e1381));
                                                let _e2283 = fma(0.4f, _e2277, (_e1373 * _e1381));
                                                let _e2290 = (_e1937 + _e2236);
                                                let _e2297 = sqrt(fma(_e2290.z, _e2290.z, fma(_e2290.x, _e2290.x, (_e2290.y * _e2290.y))));
                                                if (_e2297 == 0f) {
                                                    phi_6383_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_6383_ = (_e2290 * (1f / _e2297));
                                                }
                                                let _e2302 = phi_6383_;
                                                let _e2303 = (_e1378 * _e1378);
                                                let _e2314 = max(fma(_e1933.z, _e2302.z, fma(_e1933.x, _e2302.x, (_e1933.y * _e2302.y))), 0f);
                                                let _e2327 = max(fma(_e1933.z, _e1937.z, fma(_e1933.x, _e1937.x, (_e1933.y * _e1937.y))), 0f);
                                                let _e2331 = max(fma(_e1933.z, _e2236.z, fma(_e1933.x, _e2236.x, (_e1933.y * _e2236.y))), 0f);
                                                let _e2332 = fma(_e685.y, _e294.member_4, 1f);
                                                let _e2333 = (_e2332 * _e2332);
                                                let _e2334 = (_e2333 * 0.125f);
                                                let _e2336 = fma(-(_e2333), 0.125f, 1f);
                                                let _e2349 = (1f - max(fma(_e2302.z, _e1937.z, fma(_e2302.x, _e1937.x, (_e2302.y * _e1937.y))), 0f));
                                                let _e2351 = select(_e2349, 0f, (_e2349 < 0f));
                                                let _e2354 = pow(select(_e2351, 1f, (_e2351 > 1f)), 5f);
                                                let _e2355 = fma((1f - _e2281), _e2354, _e2281);
                                                let _e2356 = fma((1f - _e2282), _e2354, _e2282);
                                                let _e2357 = fma((1f - _e2283), _e2354, _e2283);
                                                let _e2364 = (((_e2303 * _e2303) / (pow(fma((_e2314 * _e2314), fma(_e2303, _e2303, -1f), 1f), 2f) * 3.1415927f)) * ((_e2327 / fma(_e2327, _e2336, _e2334)) * (_e2331 / fma(_e2331, _e2336, _e2334))));
                                                let _e2369 = fma((4f * _e2327), _e2331, 0.0001f);
                                                phi_3850_ = vec3<f32>(fma((fma((((1f - _e2355) * _e2277) * _e1369), 0.31830987f, ((_e2364 * _e2355) / _e2369)) * (_e2192.member_4.x * _e2275)), _e2331, _e1953.x), fma((fma((((1f - _e2356) * _e2277) * _e1371), 0.31830987f, ((_e2364 * _e2356) / _e2369)) * (_e2192.member_4.y * _e2275)), _e2331, _e1953.y), fma((fma((((1f - _e2357) * _e2277) * _e1373), 0.31830987f, ((_e2364 * _e2357) / _e2369)) * (_e2192.member_4.z * _e2275)), _e2331, _e1953.z));
                                            }
                                            let _e2390 = phi_3850_;
                                            phi_7242_ = _e1949;
                                            phi_3852_ = _e2390;
                                            phi_3853_ = select(true, false, _e2232);
                                            break;
                                        }
                                        default: {
                                            phi_7242_ = _e1949;
                                            phi_3852_ = vec3<f32>();
                                            phi_3853_ = bool();
                                            break;
                                        }
                                    }
                                    let _e3293 = phi_7242_;
                                    let _e3295 = phi_3852_;
                                    let _e3297 = phi_3853_;
                                    phi_7239_ = _e3293;
                                    phi_2376_ = select(_e3295, _e1953, vec3(select(true, false, _e3297)));
                                    phi_3862_ = true;
                                    break;
                                }
                                default: {
                                    phi_7239_ = _e1949;
                                    phi_2376_ = vec3<f32>();
                                    phi_3862_ = bool();
                                    break;
                                }
                            }
                            let _e3302 = phi_7239_;
                            let _e3304 = phi_2376_;
                            let _e3306 = phi_3862_;
                            continue;
                            continuing {
                                phi_7183_ = _e3302;
                                phi_2372_ = _e1966;
                                phi_2375_ = _e3304;
                                phi_7238_ = _e3302;
                                break if !(_e3306);
                            }
                        }
                        let _e3309 = phi_7238_;
                        phi_7265_ = _e3309;
                        if _e3309 {
                            break;
                        }
                        let _e3311 = fma(-(_e685.z), _e294.member_3, 1f);
                        let _e3315 = fma(0.04f, _e3311, (_e1369 * _e1381));
                        let _e3316 = fma(0.04f, _e3311, (_e1371 * _e1381));
                        let _e3317 = fma(0.04f, _e3311, (_e1373 * _e1381));
                        let _e3329 = fma(-(_e685.y), _e294.member_4, 1f);
                        let _e3336 = (1f - max(fma(_e1933.z, _e1937.z, fma(_e1933.x, _e1937.x, (_e1933.y * _e1937.y))), 0f));
                        let _e3338 = select(_e3336, 0f, (_e3336 < 0f));
                        let _e3341 = pow(select(_e3338, 1f, (_e3338 > 1f)), 5f);
                        let _e3342 = fma((max(_e3329, _e3315) - _e3315), _e3341, _e3315);
                        let _e3343 = fma((max(_e3329, _e3316) - _e3316), _e3341, _e3316);
                        let _e3344 = fma((max(_e3329, _e3317) - _e3317), _e3341, _e3317);
                        let _e3364 = local_3;
                        let _e3368 = local_4;
                        let _e3372 = local_5;
                        phi_7273_ = _e3309;
                        phi_3979_ = vec4<f32>(fma(_e1391, _e294.member_1, fma(fma(((1f - _e3342) * _e3311), (_e1400.x * _e1369), (_e1748.x * fma(_e3342, _e1764.x, _e1764.y))), _e1385, _e3364.x)), fma(_e1393, _e294.member_1, fma(fma(((1f - _e3343) * _e3311), (_e1400.y * _e1371), (_e1748.y * fma(_e3343, _e1764.x, _e1764.y))), _e1385, _e3368.y)), fma(_e1395, _e294.member_1, fma(fma(((1f - _e3344) * _e3311), (_e1400.z * _e1373), (_e1748.z * fma(_e3344, _e1764.x, _e1764.y))), _e1385, _e3372.z)), 1f);
                    } else {
                        phi_7273_ = false;
                        phi_3979_ = (vec4<f32>((_e125.x * _e491.x), (_e125.y * _e491.y), (_e125.z * _e491.z), (_e125.w * _e491.w)) * _e294.member_2);
                    }
                    let _e3380 = phi_7273_;
                    let _e3382 = phi_3979_;
                    global_20 = _e3382;
                    phi_7265_ = _e3380;
                    break;
                }
                case 1: {
                    let _e1906 = sqrt(fma(_e126.x, _e126.x, (_e126.y * _e126.y)));
                    if (_e1906 == 0f) {
                        phi_6008_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_6008_ = (vec3<f32>(_e126.x, _e126.y, 0f) * (1f / _e1906));
                    }
                    let _e1911 = phi_6008_;
                    global_20 = vec4<f32>(((_e1911.x + 1f) * 0.5f), ((_e1911.y + 1f) * 0.5f), ((_e1911.z + 1f) * 0.5f), 1f);
                    phi_7265_ = false;
                    break;
                }
                case 2: {
                    let _e1885 = sqrt(fma(_e127.x, _e127.x, (_e127.y * _e127.y)));
                    if (_e1885 == 0f) {
                        phi_5959_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5959_ = (vec3<f32>(_e127.x, _e127.y, 0f) * (1f / _e1885));
                    }
                    let _e1890 = phi_5959_;
                    global_20 = vec4<f32>(((_e1890.x + 1f) * 0.5f), ((_e1890.y + 1f) * 0.5f), ((_e1890.z + 1f) * 0.5f), 1f);
                    phi_7265_ = false;
                    break;
                }
                case 3: {
                    if _e1727 {
                        phi_5910_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5910_ = (_e1354 * (1f / _e1726));
                    }
                    let _e1869 = phi_5910_;
                    global_20 = vec4<f32>(((_e1869.x + 1f) * 0.5f), ((_e1869.y + 1f) * 0.5f), ((_e1869.z + 1f) * 0.5f), 1f);
                    phi_7265_ = false;
                    break;
                }
                case 4: {
                    global_20 = _e125;
                    phi_7265_ = false;
                    break;
                }
                case 5: {
                    let _e1850 = sqrt(fma(_e128.z, _e128.z, fma(_e128.x, _e128.x, (_e128.y * _e128.y))));
                    if (_e1850 == 0f) {
                        phi_5861_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5861_ = (_e128 * (1f / _e1850));
                    }
                    let _e1855 = phi_5861_;
                    global_20 = vec4<f32>(((_e1855.x + 1f) * 0.5f), ((_e1855.y + 1f) * 0.5f), ((_e1855.z + 1f) * 0.5f), 1f);
                    phi_7265_ = false;
                    break;
                }
                case 6: {
                    let _e1828 = sqrt(fma(_e1352.z, _e1352.z, fma(_e1352.x, _e1352.x, (_e1352.y * _e1352.y))));
                    if (_e1828 == 0f) {
                        phi_5812_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5812_ = (_e1352 * (1f / _e1828));
                    }
                    let _e1833 = phi_5812_;
                    global_20 = vec4<f32>(((_e1833.x + 1f) * 0.5f), ((_e1833.y + 1f) * 0.5f), ((_e1833.z + 1f) * 0.5f), 1f);
                    phi_7265_ = false;
                    break;
                }
                case 7: {
                    let _e1806 = sqrt(fma(_e129.z, _e129.z, fma(_e129.x, _e129.x, (_e129.y * _e129.y))));
                    if (_e1806 == 0f) {
                        phi_5763_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5763_ = (_e129 * (1f / _e1806));
                    }
                    let _e1811 = phi_5763_;
                    global_20 = vec4<f32>(((_e1811.x + 1f) * 0.5f), ((_e1811.y + 1f) * 0.5f), ((_e1811.z + 1f) * 0.5f), 1f);
                    phi_7265_ = false;
                    break;
                }
                case 8: {
                    let _e1784 = sqrt(fma(_e130.z, _e130.z, fma(_e130.x, _e130.x, (_e130.y * _e130.y))));
                    if (_e1784 == 0f) {
                        phi_5714_ = vec3<f32>(0f, 0f, 0f);
                    } else {
                        phi_5714_ = (_e130 * (1f / _e1784));
                    }
                    let _e1789 = phi_5714_;
                    global_20 = vec4<f32>(((_e1789.x + 1f) * 0.5f), ((_e1789.y + 1f) * 0.5f), ((_e1789.z + 1f) * 0.5f), 1f);
                    phi_7265_ = false;
                    break;
                }
                case 9: {
                    global_20 = vec4<f32>(_e1400.x, _e1400.y, _e1400.z, 1f);
                    phi_7265_ = false;
                    break;
                }
                case 10: {
                    global_20 = vec4<f32>(_e1748.x, _e1748.y, _e1748.z, 1f);
                    phi_7265_ = false;
                    break;
                }
                case 11: {
                    global_20 = vec4<f32>(_e1764.x, _e1764.y, 1f, 1f);
                    phi_7265_ = false;
                    break;
                }
                case 12: {
                    global_20 = (vec4<f32>(_e1358, _e1361, _e1364, (_e491.w * _e294.member_2.w)) * _e125);
                    phi_7265_ = false;
                    break;
                }
                case 13: {
                    global_20 = vec4<f32>(_e1378, _e1378, _e1378, 1f);
                    phi_7265_ = false;
                    break;
                }
                case 14: {
                    global_20 = vec4<f32>(_e1381, _e1381, _e1381, 1f);
                    phi_7265_ = false;
                    break;
                }
                case 15: {
                    global_20 = vec4<f32>(_e1385, _e1385, _e1385, 1f);
                    phi_7265_ = false;
                    break;
                }
                case 16: {
                    global_20 = vec4<f32>((_e1391 * _e294.member_1), (_e1393 * _e294.member_1), (_e1395 * _e294.member_1), 1f);
                    phi_7265_ = false;
                    break;
                }
                case 17: {
                    global_20 = vec4<f32>(_e1267.x, _e1267.y, _e1267.z, 1f);
                    phi_7265_ = false;
                    break;
                }
                case 18: {
                    global_20 = vec4<f32>(_e294.member.x, _e294.member.y, _e294.member.z, 1f);
                    phi_7265_ = false;
                    break;
                }
                case 19: {
                    global_20 = vec4<f32>(_e294.member_1, _e294.member_1, _e294.member_1, 1f);
                    phi_7265_ = false;
                    break;
                }
                default: {
                    phi_7265_ = false;
                    break;
                }
            }
            let _e3384 = phi_7265_;
            if _e3384 {
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
