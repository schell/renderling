struct type_9 {
    member: array<u32>,
}

struct type_15 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_16 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
}

struct type_17 {
    member: type_15,
    member_1: type_15,
    member_2: type_16,
    member_3: vec3<f32>,
}

struct type_19 {
    member: u32,
    member_1: u32,
}

struct type_23 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_25 {
    member: array<type_23>,
}

struct type_31 {
    member: vec2<u32>,
    member_1: vec2<u32>,
    member_2: type_19,
    member_3: u32,
    member_4: bool,
    member_5: bool,
    member_6: bool,
    member_7: bool,
}

struct type_32 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_33 {
    member: bool,
    member_1: u32,
}

@group(0) @binding(0) 
var<storage> global: type_9;
@group(0) @binding(1) 
var<storage> global_1: type_9;
@group(0) @binding(2) 
var<storage, read_write> global_2: type_25;
var<private> global_3: vec3<u32>;

fn function() {
    var local: array<vec4<f32>, 6>;
    var local_1: array<vec3<f32>, 8>;
    var local_2: array<f32, 3>;
    var local_3: array<f32, 3>;
    var local_4: array<f32, 3>;
    var local_5: array<f32, 3>;
    var local_6: array<vec3<f32>, 8>;
    var local_7: array<vec4<f32>, 6>;
    var phi_631_: u32;
    var phi_667_: type_31;
    var phi_2131_: bool;
    var phi_832_: type_19;
    var phi_833_: type_19;
    var phi_856_: type_19;
    var phi_883_: bool;
    var phi_889_: type_19;
    var phi_890_: type_19;
    var phi_913_: type_19;
    var phi_936_: bool;
    var phi_944_: type_17;
    var phi_2163_: bool;
    var phi_995_: type_32;
    var phi_1127_: type_19;
    var phi_1128_: type_19;
    var phi_1151_: type_19;
    var phi_1195_: bool;
    var phi_1199_: bool;
    var phi_1200_: bool;
    var phi_2727_: bool;
    var phi_2032_: bool;
    var phi_2754_: bool;
    var phi_1207_: type_19;
    var phi_1208_: type_19;
    var phi_1231_: type_19;
    var phi_1241_: type_19;
    var phi_1244_: i32;
    var phi_1242_: type_19;
    var phi_1267_: type_19;
    var phi_1308_: i32;
    var phi_1245_: i32;
    var phi_1309_: bool;
    var phi_2748_: bool;
    var local_8: i32;
    var phi_1316_: type_19;
    var phi_1319_: i32;
    var phi_1317_: type_19;
    var phi_1342_: type_19;
    var phi_1383_: i32;
    var phi_1320_: i32;
    var phi_1384_: bool;
    var phi_2755_: bool;
    var local_9: i32;
    var phi_2762_: bool;
    var phi_1390_: bool;
    var phi_1391_: bool;
    var phi_2761_: bool;
    var phi_1392_: bool;
    var phi_1393_: bool;
    var phi_1394_: bool;
    var phi_1395_: bool;
    var phi_2760_: bool;
    var phi_2035_: bool;
    var phi_2034_: bool;
    var phi_2033_: bool;
    var phi_1418_: type_33;
    var phi_1419_: type_33;
    var local_10: u32;
    var phi_1425_: type_33;
    var phi_1426_: type_33;
    var phi_1690_: u32;
    var phi_2546_: bool;
    var phi_1708_: type_19;
    var phi_2572_: u32;

    switch bitcast<i32>(0u) {
        default: {
            let _e90 = arrayLength((&global.member));
            let _e92 = arrayLength((&global_1.member));
            let _e95 = global_3;
            if (_e95.x >= arrayLength((&global_2.member))) {
            } else {
                let _e101 = global_2.member[_e95.x].member_3;
                let _e104 = global.member[_e101];
                let _e109 = global.member[(_e101 + 2u)];
                let _e113 = global.member[(_e101 + 3u)];
                let _e114 = bitcast<f32>(_e113);
                let _e118 = global.member[(_e101 + 4u)];
                let _e119 = bitcast<f32>(_e118);
                let _e123 = global.member[(_e101 + 5u)];
                let _e124 = bitcast<f32>(_e123);
                let _e128 = global.member[(_e101 + 6u)];
                let _e129 = bitcast<f32>(_e128);
                let _e133 = global.member[(_e101 + 7u)];
                let _e137 = global.member[(_e101 + 8u)];
                let _e141 = global.member[(_e101 + 9u)];
                let _e145 = global.member[(_e101 + 10u)];
                global_2.member[_e95.x].member = select(_e137, _e109, (_e133 == 4294967295u));
                global_2.member[_e95.x].member_1 = select(0u, 1u, (_e104 == 1u));
                if (_e129 == 0f) {
                } else {
                    if select(false, true, (_e90 >= 11u)) {
                        let _e160 = global.member[0u];
                        let _e163 = global.member[1u];
                        let _e167 = global.member[2u];
                        let _e170 = global.member[3u];
                        let _e174 = global.member[4u];
                        switch bitcast<i32>(_e174) {
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
                        let _e177 = phi_631_;
                        let _e180 = global.member[5u];
                        let _e184 = global.member[6u];
                        let _e188 = global.member[7u];
                        let _e192 = global.member[8u];
                        let _e196 = global.member[9u];
                        let _e199 = global.member[10u];
                        phi_667_ = type_31(vec2<u32>(_e160, _e163), vec2<u32>(_e167, _e170), type_19(_e196, _e199), _e177, (_e180 == 1u), (_e184 == 1u), (_e188 == 1u), (_e192 == 1u));
                    } else {
                        phi_667_ = type_31(vec2<u32>(0u, 0u), vec2<u32>(1u, 1u), type_19(4294967295u, 0u), 0u, true, true, true, false);
                    }
                    let _e203 = phi_667_;
                    if _e203.member_6 {
                        if (_e90 >= 83u) {
                            phi_2131_ = (_e141 <= (_e90 - 83u));
                        } else {
                            phi_2131_ = false;
                        }
                        let _e209 = phi_2131_;
                        if _e209 {
                            let _e212 = global.member[_e141];
                            let _e217 = global.member[(_e141 + 1u)];
                            let _e222 = global.member[(_e141 + 2u)];
                            let _e227 = global.member[(_e141 + 3u)];
                            let _e233 = global.member[(_e141 + 4u)];
                            let _e238 = global.member[(_e141 + 5u)];
                            let _e243 = global.member[(_e141 + 6u)];
                            let _e248 = global.member[(_e141 + 7u)];
                            let _e254 = global.member[(_e141 + 8u)];
                            let _e259 = global.member[(_e141 + 9u)];
                            let _e264 = global.member[(_e141 + 10u)];
                            let _e269 = global.member[(_e141 + 11u)];
                            let _e275 = global.member[(_e141 + 12u)];
                            let _e280 = global.member[(_e141 + 13u)];
                            let _e285 = global.member[(_e141 + 14u)];
                            let _e290 = global.member[(_e141 + 15u)];
                            let _e297 = global.member[(_e141 + 16u)];
                            let _e302 = global.member[(_e141 + 17u)];
                            let _e307 = global.member[(_e141 + 18u)];
                            let _e312 = global.member[(_e141 + 19u)];
                            let _e318 = global.member[(_e141 + 20u)];
                            let _e323 = global.member[(_e141 + 21u)];
                            let _e328 = global.member[(_e141 + 22u)];
                            let _e333 = global.member[(_e141 + 23u)];
                            let _e339 = global.member[(_e141 + 24u)];
                            let _e344 = global.member[(_e141 + 25u)];
                            let _e349 = global.member[(_e141 + 26u)];
                            let _e354 = global.member[(_e141 + 27u)];
                            let _e360 = global.member[(_e141 + 28u)];
                            let _e365 = global.member[(_e141 + 29u)];
                            let _e370 = global.member[(_e141 + 30u)];
                            let _e375 = global.member[(_e141 + 31u)];
                            let _e382 = global.member[(_e141 + 32u)];
                            let _e387 = global.member[(_e141 + 33u)];
                            let _e392 = global.member[(_e141 + 34u)];
                            local_7 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            phi_832_ = type_19(0u, 6u);
                            loop {
                                let _e397 = phi_832_;
                                if (_e397.member < _e397.member_1) {
                                    phi_833_ = type_19((_e397.member + 1u), _e397.member_1);
                                    phi_856_ = type_19(1u, _e397.member);
                                } else {
                                    phi_833_ = _e397;
                                    phi_856_ = type_19(0u, type_19().member_1);
                                }
                                let _e410 = phi_833_;
                                let _e412 = phi_856_;
                                switch bitcast<i32>(_e412.member) {
                                    case 0: {
                                        phi_883_ = false;
                                        break;
                                    }
                                    case 1: {
                                        let _e417 = ((_e141 + 35u) + (_e412.member_1 * 4u));
                                        let _e420 = global.member[_e417];
                                        let _e425 = global.member[(_e417 + 1u)];
                                        let _e430 = global.member[(_e417 + 2u)];
                                        let _e435 = global.member[(_e417 + 3u)];
                                        local_7[_e412.member_1] = vec4<f32>(bitcast<f32>(_e420), bitcast<f32>(_e425), bitcast<f32>(_e430), bitcast<f32>(_e435));
                                        phi_883_ = true;
                                        break;
                                    }
                                    default: {
                                        phi_883_ = bool();
                                        break;
                                    }
                                }
                                let _e440 = phi_883_;
                                continue;
                                continuing {
                                    phi_832_ = _e410;
                                    break if !(_e440);
                                }
                            }
                            let _e442 = local_7;
                            local_6 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            phi_889_ = type_19(0u, 8u);
                            loop {
                                let _e445 = phi_889_;
                                if (_e445.member < _e445.member_1) {
                                    phi_890_ = type_19((_e445.member + 1u), _e445.member_1);
                                    phi_913_ = type_19(1u, _e445.member);
                                } else {
                                    phi_890_ = _e445;
                                    phi_913_ = type_19(0u, type_19().member_1);
                                }
                                let _e458 = phi_890_;
                                let _e460 = phi_913_;
                                switch bitcast<i32>(_e460.member) {
                                    case 0: {
                                        phi_936_ = false;
                                        break;
                                    }
                                    case 1: {
                                        let _e465 = ((_e141 + 59u) + (_e460.member_1 * 3u));
                                        let _e468 = global.member[_e465];
                                        let _e473 = global.member[(_e465 + 1u)];
                                        let _e478 = global.member[(_e465 + 2u)];
                                        local_6[_e460.member_1] = vec3<f32>(bitcast<f32>(_e468), bitcast<f32>(_e473), bitcast<f32>(_e478));
                                        phi_936_ = true;
                                        break;
                                    }
                                    default: {
                                        phi_936_ = bool();
                                        break;
                                    }
                                }
                                let _e483 = phi_936_;
                                continue;
                                continuing {
                                    phi_889_ = _e458;
                                    break if !(_e483);
                                }
                            }
                            let _e485 = local_6;
                            phi_944_ = type_17(type_15(vec4<f32>(bitcast<f32>(_e212), bitcast<f32>(_e217), bitcast<f32>(_e222), bitcast<f32>(_e227)), vec4<f32>(bitcast<f32>(_e233), bitcast<f32>(_e238), bitcast<f32>(_e243), bitcast<f32>(_e248)), vec4<f32>(bitcast<f32>(_e254), bitcast<f32>(_e259), bitcast<f32>(_e264), bitcast<f32>(_e269)), vec4<f32>(bitcast<f32>(_e275), bitcast<f32>(_e280), bitcast<f32>(_e285), bitcast<f32>(_e290))), type_15(vec4<f32>(bitcast<f32>(_e297), bitcast<f32>(_e302), bitcast<f32>(_e307), bitcast<f32>(_e312)), vec4<f32>(bitcast<f32>(_e318), bitcast<f32>(_e323), bitcast<f32>(_e328), bitcast<f32>(_e333)), vec4<f32>(bitcast<f32>(_e339), bitcast<f32>(_e344), bitcast<f32>(_e349), bitcast<f32>(_e354)), vec4<f32>(bitcast<f32>(_e360), bitcast<f32>(_e365), bitcast<f32>(_e370), bitcast<f32>(_e375))), type_16(_e485, _e442), vec3<f32>(bitcast<f32>(_e382), bitcast<f32>(_e387), bitcast<f32>(_e392)));
                        } else {
                            phi_944_ = type_17(type_15(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_15(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_16(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
                        }
                        let _e489 = phi_944_;
                        if (_e90 >= 10u) {
                            phi_2163_ = (_e145 <= (_e90 - 10u));
                        } else {
                            phi_2163_ = false;
                        }
                        let _e497 = phi_2163_;
                        if _e497 {
                            let _e500 = global.member[_e145];
                            let _e505 = global.member[(_e145 + 1u)];
                            let _e510 = global.member[(_e145 + 2u)];
                            let _e516 = global.member[(_e145 + 3u)];
                            let _e521 = global.member[(_e145 + 4u)];
                            let _e526 = global.member[(_e145 + 5u)];
                            let _e531 = global.member[(_e145 + 6u)];
                            let _e537 = global.member[(_e145 + 7u)];
                            let _e542 = global.member[(_e145 + 8u)];
                            let _e547 = global.member[(_e145 + 9u)];
                            phi_995_ = type_32(vec3<f32>(bitcast<f32>(_e500), bitcast<f32>(_e505), bitcast<f32>(_e510)), vec4<f32>(bitcast<f32>(_e516), bitcast<f32>(_e521), bitcast<f32>(_e526), bitcast<f32>(_e531)), vec3<f32>(bitcast<f32>(_e537), bitcast<f32>(_e542), bitcast<f32>(_e547)));
                        } else {
                            phi_995_ = type_32(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                        }
                        let _e552 = phi_995_;
                        let _e560 = (_e552.member_1.x + _e552.member_1.x);
                        let _e561 = (_e552.member_1.y + _e552.member_1.y);
                        let _e562 = (_e552.member_1.z + _e552.member_1.z);
                        let _e564 = (_e552.member_1.z * _e562);
                        let _e565 = (_e552.member_1.w * _e560);
                        let _e566 = (_e552.member_1.w * _e561);
                        let _e567 = (_e552.member_1.w * _e562);
                        let _e587 = (vec4<f32>((1f - fma(_e552.member_1.y, _e561, _e564)), fma(_e552.member_1.x, _e561, _e567), fma(_e552.member_1.x, _e562, -(_e566)), 0f) * _e552.member_2.x);
                        let _e589 = (vec4<f32>(fma(_e552.member_1.x, _e561, -(_e567)), (1f - fma(_e552.member_1.x, _e560, _e564)), fma(_e552.member_1.y, _e562, _e565), 0f) * _e552.member_2.y);
                        let _e591 = (vec4<f32>(fma(_e552.member_1.x, _e562, _e566), fma(_e552.member_1.y, _e562, -(_e565)), (1f - fma(_e552.member_1.x, _e560, (_e552.member_1.y * _e561))), 0f) * _e552.member_2.z);
                        let _e613 = (_e552.member.x + fma(_e591.x, _e124, fma(_e589.x, _e119, (_e587.x * _e114))));
                        let _e614 = (_e552.member.y + fma(_e591.y, _e124, fma(_e589.y, _e119, (_e587.y * _e114))));
                        let _e615 = (_e552.member.z + fma(_e591.z, _e124, fma(_e589.z, _e119, (_e587.z * _e114))));
                        let _e616 = vec3<f32>(_e613, _e614, _e615);
                        let _e619 = (max(_e552.member_2.x, max(_e552.member_2.y, _e552.member_2.z)) * _e129);
                        let _e621 = sqrt((_e619 * _e619));
                        local_1 = _e489.member_2.member;
                        local = _e489.member_2.member_1;
                        let _e626 = local[0u][0u];
                        let _e629 = local[0u][1u];
                        let _e634 = local[0u][2u];
                        let _e638 = local[0u][3u];
                        let _e640 = -(_e621);
                        if ((fma(_e634, _e615, fma(_e626, _e613, (_e629 * _e614))) + _e638) < _e640) {
                            phi_1426_ = type_33(true, 0u);
                        } else {
                            phi_1127_ = type_19(0u, 6u);
                            loop {
                                let _e643 = phi_1127_;
                                if (_e643.member < _e643.member_1) {
                                    phi_1128_ = type_19((_e643.member + 1u), _e643.member_1);
                                    phi_1151_ = type_19(1u, _e643.member);
                                } else {
                                    phi_1128_ = _e643;
                                    phi_1151_ = type_19(0u, type_19().member_1);
                                }
                                let _e656 = phi_1128_;
                                let _e658 = phi_1151_;
                                local_10 = _e658.member_1;
                                switch bitcast<i32>(_e658.member) {
                                    case 0: {
                                        phi_1199_ = false;
                                        phi_1200_ = true;
                                        break;
                                    }
                                    case 1: {
                                        if (_e658.member_1 != 0u) {
                                            if (_e658.member_1 < 6u) {
                                            } else {
                                                phi_2727_ = true;
                                                phi_2032_ = bool();
                                                break;
                                            }
                                            let _e666 = local[_e658.member_1][0u];
                                            let _e669 = local[_e658.member_1][1u];
                                            let _e674 = local[_e658.member_1][2u];
                                            let _e678 = local[_e658.member_1][3u];
                                            phi_1195_ = select(true, false, ((fma(_e674, _e615, fma(_e666, _e613, (_e669 * _e614))) + _e678) < _e640));
                                        } else {
                                            phi_1195_ = true;
                                        }
                                        let _e683 = phi_1195_;
                                        phi_1199_ = _e683;
                                        phi_1200_ = false;
                                        break;
                                    }
                                    default: {
                                        phi_1199_ = bool();
                                        phi_1200_ = bool();
                                        break;
                                    }
                                }
                                let _e685 = phi_1199_;
                                let _e687 = phi_1200_;
                                continue;
                                continuing {
                                    phi_1127_ = _e656;
                                    phi_2727_ = false;
                                    phi_2032_ = _e687;
                                    break if !(_e685);
                                }
                            }
                            let _e690 = phi_2727_;
                            let _e692 = phi_2032_;
                            if _e690 {
                                break;
                            }
                            if _e692 {
                                let _e693 = vec3(_e621);
                                let _e694 = (_e616 - _e693);
                                let _e695 = (_e616 + _e693);
                                phi_2754_ = _e690;
                                phi_1207_ = type_19(0u, 3u);
                                loop {
                                    let _e697 = phi_2754_;
                                    let _e699 = phi_1207_;
                                    if (_e699.member < _e699.member_1) {
                                        phi_1208_ = type_19((_e699.member + 1u), _e699.member_1);
                                        phi_1231_ = type_19(1u, _e699.member);
                                    } else {
                                        phi_1208_ = _e699;
                                        phi_1231_ = type_19(0u, type_19().member_1);
                                    }
                                    let _e712 = phi_1208_;
                                    let _e714 = phi_1231_;
                                    switch bitcast<i32>(_e714.member) {
                                        case 0: {
                                            phi_2761_ = _e697;
                                            phi_1392_ = false;
                                            phi_1393_ = true;
                                            phi_1394_ = false;
                                            phi_1395_ = false;
                                            break;
                                        }
                                        case 1: {
                                            phi_1241_ = type_19(0u, 8u);
                                            phi_1244_ = 0i;
                                            loop {
                                                let _e719 = phi_1241_;
                                                let _e721 = phi_1244_;
                                                local_8 = _e721;
                                                if (_e719.member < _e719.member_1) {
                                                    phi_1242_ = type_19((_e719.member + 1u), _e719.member_1);
                                                    phi_1267_ = type_19(1u, _e719.member);
                                                } else {
                                                    phi_1242_ = _e719;
                                                    phi_1267_ = type_19(0u, type_19().member_1);
                                                }
                                                let _e734 = phi_1242_;
                                                let _e736 = phi_1267_;
                                                switch bitcast<i32>(_e736.member) {
                                                    case 0: {
                                                        phi_1245_ = i32();
                                                        phi_1309_ = false;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if (_e736.member_1 < 8u) {
                                                        } else {
                                                            phi_2748_ = true;
                                                            break;
                                                        }
                                                        let _e743 = local_1[_e736.member_1][0u];
                                                        let _e746 = local_1[_e736.member_1][1u];
                                                        let _e749 = local_1[_e736.member_1][2u];
                                                        local_2 = array<f32, 3>(_e743, _e746, _e749);
                                                        let _e751 = (_e714.member_1 < 3u);
                                                        if _e751 {
                                                        } else {
                                                            phi_2748_ = true;
                                                            break;
                                                        }
                                                        let _e753 = local_2[_e714.member_1];
                                                        local_3 = array<f32, 3>(_e694.x, _e694.y, _e694.z);
                                                        if _e751 {
                                                        } else {
                                                            phi_2748_ = true;
                                                            break;
                                                        }
                                                        let _e759 = local_3[_e714.member_1];
                                                        if (_e753 < _e759) {
                                                            phi_1308_ = (_e721 + 1i);
                                                        } else {
                                                            phi_1308_ = _e721;
                                                        }
                                                        let _e763 = phi_1308_;
                                                        phi_1245_ = _e763;
                                                        phi_1309_ = true;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_1245_ = i32();
                                                        phi_1309_ = bool();
                                                        break;
                                                    }
                                                }
                                                let _e765 = phi_1245_;
                                                let _e767 = phi_1309_;
                                                continue;
                                                continuing {
                                                    phi_1241_ = _e734;
                                                    phi_1244_ = _e765;
                                                    phi_2748_ = _e697;
                                                    break if !(_e767);
                                                }
                                            }
                                            let _e770 = phi_2748_;
                                            phi_2760_ = _e770;
                                            phi_2035_ = bool();
                                            phi_2034_ = bool();
                                            phi_2033_ = bool();
                                            if _e770 {
                                                break;
                                            }
                                            let _e772 = local_8;
                                            let _e773 = (_e772 == 8i);
                                            if _e773 {
                                                phi_2762_ = _e770;
                                                phi_1390_ = false;
                                                phi_1391_ = false;
                                            } else {
                                                phi_1316_ = type_19(0u, 8u);
                                                phi_1319_ = 0i;
                                                loop {
                                                    let _e775 = phi_1316_;
                                                    let _e777 = phi_1319_;
                                                    local_9 = _e777;
                                                    if (_e775.member < _e775.member_1) {
                                                        phi_1317_ = type_19((_e775.member + 1u), _e775.member_1);
                                                        phi_1342_ = type_19(1u, _e775.member);
                                                    } else {
                                                        phi_1317_ = _e775;
                                                        phi_1342_ = type_19(0u, type_19().member_1);
                                                    }
                                                    let _e790 = phi_1317_;
                                                    let _e792 = phi_1342_;
                                                    switch bitcast<i32>(_e792.member) {
                                                        case 0: {
                                                            phi_1320_ = i32();
                                                            phi_1384_ = false;
                                                            break;
                                                        }
                                                        case 1: {
                                                            if (_e792.member_1 < 8u) {
                                                            } else {
                                                                phi_2755_ = true;
                                                                break;
                                                            }
                                                            let _e799 = local_1[_e792.member_1][0u];
                                                            let _e802 = local_1[_e792.member_1][1u];
                                                            let _e805 = local_1[_e792.member_1][2u];
                                                            local_4 = array<f32, 3>(_e799, _e802, _e805);
                                                            let _e807 = (_e714.member_1 < 3u);
                                                            if _e807 {
                                                            } else {
                                                                phi_2755_ = true;
                                                                break;
                                                            }
                                                            let _e809 = local_4[_e714.member_1];
                                                            local_5 = array<f32, 3>(_e695.x, _e695.y, _e695.z);
                                                            if _e807 {
                                                            } else {
                                                                phi_2755_ = true;
                                                                break;
                                                            }
                                                            let _e815 = local_5[_e714.member_1];
                                                            if (_e809 > _e815) {
                                                                phi_1383_ = (_e777 + 1i);
                                                            } else {
                                                                phi_1383_ = _e777;
                                                            }
                                                            let _e819 = phi_1383_;
                                                            phi_1320_ = _e819;
                                                            phi_1384_ = true;
                                                            break;
                                                        }
                                                        default: {
                                                            phi_1320_ = i32();
                                                            phi_1384_ = bool();
                                                            break;
                                                        }
                                                    }
                                                    let _e821 = phi_1320_;
                                                    let _e823 = phi_1384_;
                                                    continue;
                                                    continuing {
                                                        phi_1316_ = _e790;
                                                        phi_1319_ = _e821;
                                                        phi_2755_ = _e770;
                                                        break if !(_e823);
                                                    }
                                                }
                                                let _e826 = phi_2755_;
                                                phi_2760_ = _e826;
                                                phi_2035_ = bool();
                                                phi_2034_ = bool();
                                                phi_2033_ = bool();
                                                if _e826 {
                                                    break;
                                                }
                                                let _e828 = local_9;
                                                let _e829 = (_e828 == 8i);
                                                phi_2762_ = _e826;
                                                phi_1390_ = select(true, false, _e829);
                                                phi_1391_ = _e829;
                                            }
                                            let _e832 = phi_2762_;
                                            let _e834 = phi_1390_;
                                            let _e836 = phi_1391_;
                                            phi_2761_ = _e832;
                                            phi_1392_ = _e834;
                                            phi_1393_ = false;
                                            phi_1394_ = _e773;
                                            phi_1395_ = _e836;
                                            break;
                                        }
                                        default: {
                                            phi_2761_ = _e697;
                                            phi_1392_ = bool();
                                            phi_1393_ = bool();
                                            phi_1394_ = bool();
                                            phi_1395_ = bool();
                                            break;
                                        }
                                    }
                                    let _e838 = phi_2761_;
                                    let _e840 = phi_1392_;
                                    let _e842 = phi_1393_;
                                    let _e844 = phi_1394_;
                                    let _e846 = phi_1395_;
                                    continue;
                                    continuing {
                                        phi_2754_ = _e838;
                                        phi_1207_ = _e712;
                                        phi_2760_ = _e838;
                                        phi_2035_ = _e846;
                                        phi_2034_ = _e844;
                                        phi_2033_ = _e842;
                                        break if !(_e840);
                                    }
                                }
                                let _e849 = phi_2760_;
                                let _e851 = phi_2035_;
                                let _e853 = phi_2034_;
                                let _e855 = phi_2033_;
                                if _e849 {
                                    break;
                                }
                                let _e856 = select(_e853, false, _e855);
                                if select(true, false, select(_e856, true, select(select(_e851, false, _e855), false, _e856))) {
                                    phi_1418_ = type_33(false, 0u);
                                } else {
                                    phi_1418_ = type_33(true, 0u);
                                }
                                let _e862 = phi_1418_;
                                phi_1419_ = _e862;
                            } else {
                                phi_1419_ = type_33();
                            }
                            let _e864 = phi_1419_;
                            if select(true, false, _e692) {
                                let _e867 = local_10;
                                phi_1425_ = type_33(true, _e867);
                            } else {
                                phi_1425_ = _e864;
                            }
                            let _e870 = phi_1425_;
                            phi_1426_ = _e870;
                        }
                        let _e872 = phi_1426_;
                        if (_e872.member != true) {
                            global_2.member[_e95.x].member_1 = 1u;
                            if _e203.member_7 {
                                let _e878 = global_1.member[0u];
                                let _e881 = global_1.member[1u];
                                let _e884 = global_1.member[3u];
                                let _e887 = global_1.member[4u];
                                let _e888 = f32(_e878);
                                let _e930 = fma(_e489.member.member_3.z, _e489.member_1.member.w, fma(_e489.member.member_2.z, _e489.member_1.member.z, fma(_e489.member.member.z, _e489.member_1.member.x, (_e489.member.member_1.z * _e489.member_1.member.y))));
                                let _e931 = fma(_e489.member.member_3.w, _e489.member_1.member.w, fma(_e489.member.member_2.w, _e489.member_1.member.z, fma(_e489.member.member.w, _e489.member_1.member.x, (_e489.member.member_1.w * _e489.member_1.member.y))));
                                let _e951 = fma(_e489.member.member_3.z, _e489.member_1.member_1.w, fma(_e489.member.member_2.z, _e489.member_1.member_1.z, fma(_e489.member.member.z, _e489.member_1.member_1.x, (_e489.member.member_1.z * _e489.member_1.member_1.y))));
                                let _e952 = fma(_e489.member.member_3.w, _e489.member_1.member_1.w, fma(_e489.member.member_2.w, _e489.member_1.member_1.z, fma(_e489.member.member.w, _e489.member_1.member_1.x, (_e489.member.member_1.w * _e489.member_1.member_1.y))));
                                let _e972 = fma(_e489.member.member_3.z, _e489.member_1.member_2.w, fma(_e489.member.member_2.z, _e489.member_1.member_2.z, fma(_e489.member.member.z, _e489.member_1.member_2.x, (_e489.member.member_1.z * _e489.member_1.member_2.y))));
                                let _e973 = fma(_e489.member.member_3.w, _e489.member_1.member_2.w, fma(_e489.member.member_2.w, _e489.member_1.member_2.z, fma(_e489.member.member.w, _e489.member_1.member_2.x, (_e489.member.member_1.w * _e489.member_1.member_2.y))));
                                let _e993 = fma(_e489.member.member_3.z, _e489.member_1.member_3.w, fma(_e489.member.member_2.z, _e489.member_1.member_3.z, fma(_e489.member.member.z, _e489.member_1.member_3.x, (_e489.member.member_1.z * _e489.member_1.member_3.y))));
                                let _e994 = fma(_e489.member.member_3.w, _e489.member_1.member_3.w, fma(_e489.member.member_2.w, _e489.member_1.member_3.z, fma(_e489.member.member.w, _e489.member_1.member_3.x, (_e489.member.member_1.w * _e489.member_1.member_3.y))));
                                let _e1006 = (fma(_e973, _e615, fma(_e931, _e613, (_e952 * _e614))) + _e994);
                                let _e1012 = fma(_e621, _e489.member_2.member_1[5].x, _e613);
                                let _e1013 = fma(_e621, _e489.member_2.member_1[5].y, _e614);
                                let _e1014 = fma(_e621, _e489.member_2.member_1[5].z, _e615);
                                let _e1029 = fma(_e621, _e489.member_2.member_1[0].x, _e613);
                                let _e1030 = fma(_e621, _e489.member_2.member_1[0].y, _e614);
                                let _e1031 = fma(_e621, _e489.member_2.member_1[0].z, _e615);
                                let _e1048 = (vec2<f32>(((((fma(fma(_e489.member.member_3.x, _e489.member_1.member_2.w, fma(_e489.member.member_2.x, _e489.member_1.member_2.z, fma(_e489.member.member.x, _e489.member_1.member_2.x, (_e489.member.member_1.x * _e489.member_1.member_2.y)))), _e615, fma(fma(_e489.member.member_3.x, _e489.member_1.member.w, fma(_e489.member.member_2.x, _e489.member_1.member.z, fma(_e489.member.member.x, _e489.member_1.member.x, (_e489.member.member_1.x * _e489.member_1.member.y)))), _e613, (fma(_e489.member.member_3.x, _e489.member_1.member_1.w, fma(_e489.member.member_2.x, _e489.member_1.member_1.z, fma(_e489.member.member.x, _e489.member_1.member_1.x, (_e489.member.member_1.x * _e489.member_1.member_1.y)))) * _e614))) + fma(_e489.member.member_3.x, _e489.member_1.member_3.w, fma(_e489.member.member_2.x, _e489.member_1.member_3.z, fma(_e489.member.member.x, _e489.member_1.member_3.x, (_e489.member.member_1.x * _e489.member_1.member_3.y))))) / _e1006) + 1f) * 0.5f), fma((-1f - ((fma(fma(_e489.member.member_3.y, _e489.member_1.member_2.w, fma(_e489.member.member_2.y, _e489.member_1.member_2.z, fma(_e489.member.member.y, _e489.member_1.member_2.x, (_e489.member.member_1.y * _e489.member_1.member_2.y)))), _e615, fma(fma(_e489.member.member_3.y, _e489.member_1.member.w, fma(_e489.member.member_2.y, _e489.member_1.member.z, fma(_e489.member.member.y, _e489.member_1.member.x, (_e489.member.member_1.y * _e489.member_1.member.y)))), _e613, (fma(_e489.member.member_3.y, _e489.member_1.member_1.w, fma(_e489.member.member_2.y, _e489.member_1.member_1.z, fma(_e489.member.member.y, _e489.member_1.member_1.x, (_e489.member.member_1.y * _e489.member_1.member_1.y)))) * _e614))) + fma(_e489.member.member_3.y, _e489.member_1.member_3.w, fma(_e489.member.member_2.y, _e489.member_1.member_3.z, fma(_e489.member.member.y, _e489.member_1.member_3.x, (_e489.member.member_1.y * _e489.member_1.member_3.y))))) / _e1006)), 0.5f, 1f)) * vec2<f32>(_e888, f32(_e881)));
                                let _e1049 = (_e621 / _e1006);
                                let _e1051 = -(_e888);
                                let _e1055 = vec3<f32>(fma(_e1051, _e1049, _e1048.x), fma(_e1051, _e1049, _e1048.y), ((_e993 + fma(_e972, _e1014, fma(_e951, _e1013, (_e930 * _e1012)))) / (_e994 + fma(_e973, _e1014, fma(_e952, _e1013, (_e931 * _e1012))))));
                                let _e1058 = vec3<f32>(fma(_e888, _e1049, _e1048.x), fma(_e888, _e1049, _e1048.y), ((_e993 + fma(_e972, _e1031, fma(_e951, _e1030, (_e930 * _e1029)))) / (_e994 + fma(_e973, _e1031, fma(_e952, _e1030, (_e931 * _e1029))))));
                                let _e1059 = min(_e1055, _e1058);
                                let _e1060 = max(_e1055, _e1058);
                                let _e1065 = (_e1060.x - _e1059.x);
                                let _e1066 = (_e1060.y - _e1059.y);
                                let _e1070 = floor(log2(select(_e1066, _e1065, (_e1065 > _e1066))));
                                let _e1075 = select(select(u32(_e1070), 0u, (_e1070 < 0f)), 4294967295u, (_e1070 > 4294967000f));
                                let _e1076 = (_e887 - 1u);
                                let _e1078 = select(_e1075, _e1076, (_e1075 > _e1076));
                                let _e1084 = round(((_e1059.x + _e1060.x) * 0.5f));
                                let _e1090 = (_e1078 & 31u);
                                let _e1093 = round(((_e1059.y + _e1060.y) * 0.5f));
                                if (_e1078 >= _e887) {
                                    phi_1690_ = 4294967295u;
                                } else {
                                    phi_1690_ = (_e884 + (2u * _e1078));
                                }
                                let _e1105 = phi_1690_;
                                if (_e92 >= 2u) {
                                    phi_2546_ = (_e1105 <= (_e92 - 2u));
                                } else {
                                    phi_2546_ = false;
                                }
                                let _e1110 = phi_2546_;
                                if _e1110 {
                                    let _e1113 = global_1.member[_e1105];
                                    let _e1117 = global_1.member[(_e1105 + 1u)];
                                    phi_1708_ = type_19(_e1113, _e1117);
                                } else {
                                    phi_1708_ = type_19(4294967295u, 0u);
                                }
                                let _e1120 = phi_1708_;
                                let _e1126 = (((select(select(u32(_e1093), 0u, (_e1093 < 0f)), 4294967295u, (_e1093 > 4294967000f)) >> bitcast<u32>(_e1090)) * (_e878 >> bitcast<u32>(_e1090))) + (select(select(u32(_e1084), 0u, (_e1084 < 0f)), 4294967295u, (_e1084 > 4294967000f)) >> bitcast<u32>(_e1090)));
                                if (_e1126 >= _e1120.member_1) {
                                    phi_2572_ = 4294967295u;
                                } else {
                                    phi_2572_ = (_e1120.member + _e1126);
                                }
                                let _e1130 = phi_2572_;
                                let _e1133 = global_1.member[_e1130];
                                if select((_e1059.z > 1f), true, (_e1059.z > bitcast<f32>(_e1133))) {
                                    global_2.member[_e95.x].member_1 = 0u;
                                }
                            }
                        } else {
                            global_2.member[_e95.x].member_1 = 0u;
                        }
                    }
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(32, 1, 1) 
fn cullcompute_culling(@builtin(global_invocation_id) param: vec3<u32>) {
    global_3 = param;
    function();
}
