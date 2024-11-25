struct type_10 {
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

struct VertexOutput {
    @location(0) member: vec3<f32>,
    @location(1) member_1: f32,
    @builtin(position) member_2: vec4<f32>,
}

@group(0) @binding(0) 
var<storage> global: type_10;
var<private> global_1: u32;
var<private> global_2: u32;
var<private> global_3: f32;
var<private> global_4: vec3<f32>;
var<private> global_5: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var local_2: array<vec3<f32>, 36>;
    var phi_796_: bool;
    var phi_165_: type_24;
    var phi_824_: bool;
    var phi_326_: type_24;
    var phi_327_: type_24;
    var phi_350_: type_24;
    var phi_377_: bool;
    var phi_383_: type_24;
    var phi_384_: type_24;
    var phi_407_: type_24;
    var phi_430_: bool;
    var phi_451_: type_22;
    var phi_856_: bool;
    var phi_459_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e84 = global_1;
            let _e85 = global_2;
            let _e87 = arrayLength((&global.member));
            local_2 = array<vec3<f32>, 36>(vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f));
            if (_e85 < 36u) {
            } else {
                break;
            }
            let _e90 = local_2[_e85];
            if (_e87 >= 2u) {
                phi_796_ = (_e84 <= (_e87 - 2u));
            } else {
                phi_796_ = false;
            }
            let _e95 = phi_796_;
            if _e95 {
                let _e98 = global.member[_e84];
                let _e102 = global.member[(_e84 + 1u)];
                phi_165_ = type_24(_e98, _e102);
            } else {
                phi_165_ = type_24(4294967295u, 4294967295u);
            }
            let _e105 = phi_165_;
            if (_e87 >= 86u) {
                phi_824_ = (_e105.member <= (_e87 - 86u));
            } else {
                phi_824_ = false;
            }
            let _e112 = phi_824_;
            if _e112 {
                let _e115 = global.member[_e105.member];
                let _e120 = global.member[(_e105.member + 1u)];
                let _e125 = global.member[(_e105.member + 2u)];
                let _e130 = global.member[(_e105.member + 3u)];
                let _e136 = global.member[(_e105.member + 4u)];
                let _e141 = global.member[(_e105.member + 5u)];
                let _e146 = global.member[(_e105.member + 6u)];
                let _e151 = global.member[(_e105.member + 7u)];
                let _e157 = global.member[(_e105.member + 8u)];
                let _e162 = global.member[(_e105.member + 9u)];
                let _e167 = global.member[(_e105.member + 10u)];
                let _e172 = global.member[(_e105.member + 11u)];
                let _e178 = global.member[(_e105.member + 12u)];
                let _e183 = global.member[(_e105.member + 13u)];
                let _e188 = global.member[(_e105.member + 14u)];
                let _e193 = global.member[(_e105.member + 15u)];
                let _e200 = global.member[(_e105.member + 16u)];
                let _e205 = global.member[(_e105.member + 17u)];
                let _e210 = global.member[(_e105.member + 18u)];
                let _e215 = global.member[(_e105.member + 19u)];
                let _e221 = global.member[(_e105.member + 20u)];
                let _e226 = global.member[(_e105.member + 21u)];
                let _e231 = global.member[(_e105.member + 22u)];
                let _e236 = global.member[(_e105.member + 23u)];
                let _e242 = global.member[(_e105.member + 24u)];
                let _e247 = global.member[(_e105.member + 25u)];
                let _e252 = global.member[(_e105.member + 26u)];
                let _e257 = global.member[(_e105.member + 27u)];
                let _e263 = global.member[(_e105.member + 28u)];
                let _e268 = global.member[(_e105.member + 29u)];
                let _e273 = global.member[(_e105.member + 30u)];
                let _e278 = global.member[(_e105.member + 31u)];
                let _e285 = global.member[(_e105.member + 32u)];
                let _e290 = global.member[(_e105.member + 33u)];
                let _e295 = global.member[(_e105.member + 34u)];
                local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_326_ = type_24(0u, 6u);
                loop {
                    let _e300 = phi_326_;
                    if (_e300.member < _e300.member_1) {
                        phi_327_ = type_24((_e300.member + 1u), _e300.member_1);
                        phi_350_ = type_24(1u, _e300.member);
                    } else {
                        phi_327_ = _e300;
                        phi_350_ = type_24(0u, type_24().member_1);
                    }
                    let _e313 = phi_327_;
                    let _e315 = phi_350_;
                    switch bitcast<i32>(_e315.member) {
                        case 0: {
                            phi_377_ = false;
                            break;
                        }
                        case 1: {
                            let _e320 = ((_e105.member + 35u) + (_e315.member_1 * 4u));
                            let _e323 = global.member[_e320];
                            let _e328 = global.member[(_e320 + 1u)];
                            let _e333 = global.member[(_e320 + 2u)];
                            let _e338 = global.member[(_e320 + 3u)];
                            local_1[_e315.member_1] = vec4<f32>(bitcast<f32>(_e323), bitcast<f32>(_e328), bitcast<f32>(_e333), bitcast<f32>(_e338));
                            phi_377_ = true;
                            break;
                        }
                        default: {
                            phi_377_ = bool();
                            break;
                        }
                    }
                    let _e343 = phi_377_;
                    continue;
                    continuing {
                        phi_326_ = _e313;
                        break if !(_e343);
                    }
                }
                let _e345 = local_1;
                local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                phi_383_ = type_24(0u, 8u);
                loop {
                    let _e348 = phi_383_;
                    if (_e348.member < _e348.member_1) {
                        phi_384_ = type_24((_e348.member + 1u), _e348.member_1);
                        phi_407_ = type_24(1u, _e348.member);
                    } else {
                        phi_384_ = _e348;
                        phi_407_ = type_24(0u, type_24().member_1);
                    }
                    let _e361 = phi_384_;
                    let _e363 = phi_407_;
                    switch bitcast<i32>(_e363.member) {
                        case 0: {
                            phi_430_ = false;
                            break;
                        }
                        case 1: {
                            let _e368 = ((_e105.member + 59u) + (_e363.member_1 * 3u));
                            let _e371 = global.member[_e368];
                            let _e376 = global.member[(_e368 + 1u)];
                            let _e381 = global.member[(_e368 + 2u)];
                            local[_e363.member_1] = vec3<f32>(bitcast<f32>(_e371), bitcast<f32>(_e376), bitcast<f32>(_e381));
                            phi_430_ = true;
                            break;
                        }
                        default: {
                            phi_430_ = bool();
                            break;
                        }
                    }
                    let _e386 = phi_430_;
                    continue;
                    continuing {
                        phi_383_ = _e361;
                        break if !(_e386);
                    }
                }
                let _e388 = local;
                let _e392 = global.member[(_e105.member + 83u)];
                let _e397 = global.member[(_e105.member + 84u)];
                let _e402 = global.member[(_e105.member + 85u)];
                phi_451_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e115), bitcast<f32>(_e120), bitcast<f32>(_e125), bitcast<f32>(_e130)), vec4<f32>(bitcast<f32>(_e136), bitcast<f32>(_e141), bitcast<f32>(_e146), bitcast<f32>(_e151)), vec4<f32>(bitcast<f32>(_e157), bitcast<f32>(_e162), bitcast<f32>(_e167), bitcast<f32>(_e172)), vec4<f32>(bitcast<f32>(_e178), bitcast<f32>(_e183), bitcast<f32>(_e188), bitcast<f32>(_e193))), type_20(vec4<f32>(bitcast<f32>(_e200), bitcast<f32>(_e205), bitcast<f32>(_e210), bitcast<f32>(_e215)), vec4<f32>(bitcast<f32>(_e221), bitcast<f32>(_e226), bitcast<f32>(_e231), bitcast<f32>(_e236)), vec4<f32>(bitcast<f32>(_e242), bitcast<f32>(_e247), bitcast<f32>(_e252), bitcast<f32>(_e257)), vec4<f32>(bitcast<f32>(_e263), bitcast<f32>(_e268), bitcast<f32>(_e273), bitcast<f32>(_e278))), vec3<f32>(bitcast<f32>(_e285), bitcast<f32>(_e290), bitcast<f32>(_e295)), type_21(_e388, _e345, vec3<f32>(bitcast<f32>(_e392), bitcast<f32>(_e397), bitcast<f32>(_e402))));
            } else {
                phi_451_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
            }
            let _e408 = phi_451_;
            if (_e87 >= 1u) {
                phi_856_ = (_e105.member_1 <= (_e87 - 1u));
            } else {
                phi_856_ = false;
            }
            let _e415 = phi_856_;
            if _e415 {
                let _e418 = global.member[_e105.member_1];
                phi_459_ = bitcast<f32>(_e418);
            } else {
                phi_459_ = 0f;
            }
            let _e421 = phi_459_;
            global_3 = _e421;
            global_4 = _e90;
            global_5 = vec4<f32>((fma(fma(_e408.member.member_3.x, _e408.member_1.member_2.w, fma(_e408.member.member_2.x, _e408.member_1.member_2.z, fma(_e408.member.member.x, _e408.member_1.member_2.x, (_e408.member.member_1.x * _e408.member_1.member_2.y)))), _e90.z, fma(fma(_e408.member.member_3.x, _e408.member_1.member.w, fma(_e408.member.member_2.x, _e408.member_1.member.z, fma(_e408.member.member.x, _e408.member_1.member.x, (_e408.member.member_1.x * _e408.member_1.member.y)))), _e90.x, (fma(_e408.member.member_3.x, _e408.member_1.member_1.w, fma(_e408.member.member_2.x, _e408.member_1.member_1.z, fma(_e408.member.member.x, _e408.member_1.member_1.x, (_e408.member.member_1.x * _e408.member_1.member_1.y)))) * _e90.y))) + fma(_e408.member.member_3.x, _e408.member_1.member_3.w, fma(_e408.member.member_2.x, _e408.member_1.member_3.z, fma(_e408.member.member.x, _e408.member_1.member_3.x, (_e408.member.member_1.x * _e408.member_1.member_3.y))))), (fma(fma(_e408.member.member_3.y, _e408.member_1.member_2.w, fma(_e408.member.member_2.y, _e408.member_1.member_2.z, fma(_e408.member.member.y, _e408.member_1.member_2.x, (_e408.member.member_1.y * _e408.member_1.member_2.y)))), _e90.z, fma(fma(_e408.member.member_3.y, _e408.member_1.member.w, fma(_e408.member.member_2.y, _e408.member_1.member.z, fma(_e408.member.member.y, _e408.member_1.member.x, (_e408.member.member_1.y * _e408.member_1.member.y)))), _e90.x, (fma(_e408.member.member_3.y, _e408.member_1.member_1.w, fma(_e408.member.member_2.y, _e408.member_1.member_1.z, fma(_e408.member.member.y, _e408.member_1.member_1.x, (_e408.member.member_1.y * _e408.member_1.member_1.y)))) * _e90.y))) + fma(_e408.member.member_3.y, _e408.member_1.member_3.w, fma(_e408.member.member_2.y, _e408.member_1.member_3.z, fma(_e408.member.member.y, _e408.member_1.member_3.x, (_e408.member.member_1.y * _e408.member_1.member_3.y))))), (fma(fma(_e408.member.member_3.z, _e408.member_1.member_2.w, fma(_e408.member.member_2.z, _e408.member_1.member_2.z, fma(_e408.member.member.z, _e408.member_1.member_2.x, (_e408.member.member_1.z * _e408.member_1.member_2.y)))), _e90.z, fma(fma(_e408.member.member_3.z, _e408.member_1.member.w, fma(_e408.member.member_2.z, _e408.member_1.member.z, fma(_e408.member.member.z, _e408.member_1.member.x, (_e408.member.member_1.z * _e408.member_1.member.y)))), _e90.x, (fma(_e408.member.member_3.z, _e408.member_1.member_1.w, fma(_e408.member.member_2.z, _e408.member_1.member_1.z, fma(_e408.member.member.z, _e408.member_1.member_1.x, (_e408.member.member_1.z * _e408.member_1.member_1.y)))) * _e90.y))) + fma(_e408.member.member_3.z, _e408.member_1.member_3.w, fma(_e408.member.member_2.z, _e408.member_1.member_3.z, fma(_e408.member.member.z, _e408.member_1.member_3.x, (_e408.member.member_1.z * _e408.member_1.member_3.y))))), (fma(fma(_e408.member.member_3.w, _e408.member_1.member_2.w, fma(_e408.member.member_2.w, _e408.member_1.member_2.z, fma(_e408.member.member.w, _e408.member_1.member_2.x, (_e408.member.member_1.w * _e408.member_1.member_2.y)))), _e90.z, fma(fma(_e408.member.member_3.w, _e408.member_1.member.w, fma(_e408.member.member_2.w, _e408.member_1.member.z, fma(_e408.member.member.w, _e408.member_1.member.x, (_e408.member.member_1.w * _e408.member_1.member.y)))), _e90.x, (fma(_e408.member.member_3.w, _e408.member_1.member_1.w, fma(_e408.member.member_2.w, _e408.member_1.member_1.z, fma(_e408.member.member.w, _e408.member_1.member_1.x, (_e408.member.member_1.w * _e408.member_1.member_1.y)))) * _e90.y))) + fma(_e408.member.member_3.w, _e408.member_1.member_3.w, fma(_e408.member.member_2.w, _e408.member_1.member_3.z, fma(_e408.member.member.w, _e408.member_1.member_3.x, (_e408.member.member_1.w * _e408.member_1.member_3.y))))));
            break;
        }
    }
    return;
}

@vertex 
fn convolutionprefilter_environment_cubemap_vertex(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_1 = param;
    global_2 = param_1;
    function();
    let _e8 = global_5.y;
    global_5.y = -(_e8);
    let _e10 = global_4;
    let _e11 = global_3;
    let _e12 = global_5;
    return VertexOutput(_e10, _e11, _e12);
}
