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
    var phi_767_: bool;
    var phi_165_: type_24;
    var phi_795_: bool;
    var phi_326_: type_24;
    var phi_327_: type_24;
    var phi_342_: type_24;
    var phi_369_: bool;
    var phi_375_: type_24;
    var phi_376_: type_24;
    var phi_391_: type_24;
    var phi_414_: bool;
    var phi_422_: type_22;
    var phi_827_: bool;
    var phi_430_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e81 = global_1;
            let _e82 = global_2;
            let _e84 = arrayLength((&global.member));
            local_2 = array<vec3<f32>, 36>(vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f));
            if (_e82 < 36u) {
            } else {
                break;
            }
            let _e87 = local_2[_e82];
            if (_e84 >= 2u) {
                phi_767_ = (_e81 <= (_e84 - 2u));
            } else {
                phi_767_ = false;
            }
            let _e92 = phi_767_;
            if _e92 {
                let _e95 = global.member[_e81];
                let _e99 = global.member[(_e81 + 1u)];
                phi_165_ = type_24(_e95, _e99);
            } else {
                phi_165_ = type_24(4294967295u, 4294967295u);
            }
            let _e102 = phi_165_;
            if (_e84 >= 83u) {
                phi_795_ = (_e102.member <= (_e84 - 83u));
            } else {
                phi_795_ = false;
            }
            let _e109 = phi_795_;
            if _e109 {
                let _e112 = global.member[_e102.member];
                let _e117 = global.member[(_e102.member + 1u)];
                let _e122 = global.member[(_e102.member + 2u)];
                let _e127 = global.member[(_e102.member + 3u)];
                let _e133 = global.member[(_e102.member + 4u)];
                let _e138 = global.member[(_e102.member + 5u)];
                let _e143 = global.member[(_e102.member + 6u)];
                let _e148 = global.member[(_e102.member + 7u)];
                let _e154 = global.member[(_e102.member + 8u)];
                let _e159 = global.member[(_e102.member + 9u)];
                let _e164 = global.member[(_e102.member + 10u)];
                let _e169 = global.member[(_e102.member + 11u)];
                let _e175 = global.member[(_e102.member + 12u)];
                let _e180 = global.member[(_e102.member + 13u)];
                let _e185 = global.member[(_e102.member + 14u)];
                let _e190 = global.member[(_e102.member + 15u)];
                let _e197 = global.member[(_e102.member + 16u)];
                let _e202 = global.member[(_e102.member + 17u)];
                let _e207 = global.member[(_e102.member + 18u)];
                let _e212 = global.member[(_e102.member + 19u)];
                let _e218 = global.member[(_e102.member + 20u)];
                let _e223 = global.member[(_e102.member + 21u)];
                let _e228 = global.member[(_e102.member + 22u)];
                let _e233 = global.member[(_e102.member + 23u)];
                let _e239 = global.member[(_e102.member + 24u)];
                let _e244 = global.member[(_e102.member + 25u)];
                let _e249 = global.member[(_e102.member + 26u)];
                let _e254 = global.member[(_e102.member + 27u)];
                let _e260 = global.member[(_e102.member + 28u)];
                let _e265 = global.member[(_e102.member + 29u)];
                let _e270 = global.member[(_e102.member + 30u)];
                let _e275 = global.member[(_e102.member + 31u)];
                let _e282 = global.member[(_e102.member + 32u)];
                let _e287 = global.member[(_e102.member + 33u)];
                let _e292 = global.member[(_e102.member + 34u)];
                local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_326_ = type_24(0u, 6u);
                loop {
                    let _e297 = phi_326_;
                    if (_e297.member < _e297.member_1) {
                        phi_327_ = type_24((_e297.member + 1u), _e297.member_1);
                        phi_342_ = type_24(1u, _e297.member);
                    } else {
                        phi_327_ = _e297;
                        phi_342_ = type_24(0u, type_24().member_1);
                    }
                    let _e310 = phi_327_;
                    let _e312 = phi_342_;
                    switch bitcast<i32>(_e312.member) {
                        case 0: {
                            phi_369_ = false;
                            break;
                        }
                        case 1: {
                            let _e317 = ((_e102.member + 35u) + (_e312.member_1 * 4u));
                            let _e320 = global.member[_e317];
                            let _e325 = global.member[(_e317 + 1u)];
                            let _e330 = global.member[(_e317 + 2u)];
                            let _e335 = global.member[(_e317 + 3u)];
                            local_1[_e312.member_1] = vec4<f32>(bitcast<f32>(_e320), bitcast<f32>(_e325), bitcast<f32>(_e330), bitcast<f32>(_e335));
                            phi_369_ = true;
                            break;
                        }
                        default: {
                            phi_369_ = bool();
                            break;
                        }
                    }
                    let _e340 = phi_369_;
                    continue;
                    continuing {
                        phi_326_ = _e310;
                        break if !(_e340);
                    }
                }
                let _e342 = local_1;
                local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                phi_375_ = type_24(0u, 8u);
                loop {
                    let _e345 = phi_375_;
                    if (_e345.member < _e345.member_1) {
                        phi_376_ = type_24((_e345.member + 1u), _e345.member_1);
                        phi_391_ = type_24(1u, _e345.member);
                    } else {
                        phi_376_ = _e345;
                        phi_391_ = type_24(0u, type_24().member_1);
                    }
                    let _e358 = phi_376_;
                    let _e360 = phi_391_;
                    switch bitcast<i32>(_e360.member) {
                        case 0: {
                            phi_414_ = false;
                            break;
                        }
                        case 1: {
                            let _e365 = ((_e102.member + 59u) + (_e360.member_1 * 3u));
                            let _e368 = global.member[_e365];
                            let _e373 = global.member[(_e365 + 1u)];
                            let _e378 = global.member[(_e365 + 2u)];
                            local[_e360.member_1] = vec3<f32>(bitcast<f32>(_e368), bitcast<f32>(_e373), bitcast<f32>(_e378));
                            phi_414_ = true;
                            break;
                        }
                        default: {
                            phi_414_ = bool();
                            break;
                        }
                    }
                    let _e383 = phi_414_;
                    continue;
                    continuing {
                        phi_375_ = _e358;
                        break if !(_e383);
                    }
                }
                let _e385 = local;
                phi_422_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e112), bitcast<f32>(_e117), bitcast<f32>(_e122), bitcast<f32>(_e127)), vec4<f32>(bitcast<f32>(_e133), bitcast<f32>(_e138), bitcast<f32>(_e143), bitcast<f32>(_e148)), vec4<f32>(bitcast<f32>(_e154), bitcast<f32>(_e159), bitcast<f32>(_e164), bitcast<f32>(_e169)), vec4<f32>(bitcast<f32>(_e175), bitcast<f32>(_e180), bitcast<f32>(_e185), bitcast<f32>(_e190))), type_20(vec4<f32>(bitcast<f32>(_e197), bitcast<f32>(_e202), bitcast<f32>(_e207), bitcast<f32>(_e212)), vec4<f32>(bitcast<f32>(_e218), bitcast<f32>(_e223), bitcast<f32>(_e228), bitcast<f32>(_e233)), vec4<f32>(bitcast<f32>(_e239), bitcast<f32>(_e244), bitcast<f32>(_e249), bitcast<f32>(_e254)), vec4<f32>(bitcast<f32>(_e260), bitcast<f32>(_e265), bitcast<f32>(_e270), bitcast<f32>(_e275))), type_21(_e385, _e342), vec3<f32>(bitcast<f32>(_e282), bitcast<f32>(_e287), bitcast<f32>(_e292)));
            } else {
                phi_422_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
            }
            let _e389 = phi_422_;
            if (_e84 >= 1u) {
                phi_827_ = (_e102.member_1 <= (_e84 - 1u));
            } else {
                phi_827_ = false;
            }
            let _e396 = phi_827_;
            if _e396 {
                let _e399 = global.member[_e102.member_1];
                phi_430_ = bitcast<f32>(_e399);
            } else {
                phi_430_ = 0f;
            }
            let _e402 = phi_430_;
            global_3 = _e402;
            global_4 = _e87;
            global_5 = vec4<f32>((fma(fma(_e389.member.member_3.x, _e389.member_1.member_2.w, fma(_e389.member.member_2.x, _e389.member_1.member_2.z, fma(_e389.member.member.x, _e389.member_1.member_2.x, (_e389.member.member_1.x * _e389.member_1.member_2.y)))), _e87.z, fma(fma(_e389.member.member_3.x, _e389.member_1.member.w, fma(_e389.member.member_2.x, _e389.member_1.member.z, fma(_e389.member.member.x, _e389.member_1.member.x, (_e389.member.member_1.x * _e389.member_1.member.y)))), _e87.x, (fma(_e389.member.member_3.x, _e389.member_1.member_1.w, fma(_e389.member.member_2.x, _e389.member_1.member_1.z, fma(_e389.member.member.x, _e389.member_1.member_1.x, (_e389.member.member_1.x * _e389.member_1.member_1.y)))) * _e87.y))) + fma(_e389.member.member_3.x, _e389.member_1.member_3.w, fma(_e389.member.member_2.x, _e389.member_1.member_3.z, fma(_e389.member.member.x, _e389.member_1.member_3.x, (_e389.member.member_1.x * _e389.member_1.member_3.y))))), (fma(fma(_e389.member.member_3.y, _e389.member_1.member_2.w, fma(_e389.member.member_2.y, _e389.member_1.member_2.z, fma(_e389.member.member.y, _e389.member_1.member_2.x, (_e389.member.member_1.y * _e389.member_1.member_2.y)))), _e87.z, fma(fma(_e389.member.member_3.y, _e389.member_1.member.w, fma(_e389.member.member_2.y, _e389.member_1.member.z, fma(_e389.member.member.y, _e389.member_1.member.x, (_e389.member.member_1.y * _e389.member_1.member.y)))), _e87.x, (fma(_e389.member.member_3.y, _e389.member_1.member_1.w, fma(_e389.member.member_2.y, _e389.member_1.member_1.z, fma(_e389.member.member.y, _e389.member_1.member_1.x, (_e389.member.member_1.y * _e389.member_1.member_1.y)))) * _e87.y))) + fma(_e389.member.member_3.y, _e389.member_1.member_3.w, fma(_e389.member.member_2.y, _e389.member_1.member_3.z, fma(_e389.member.member.y, _e389.member_1.member_3.x, (_e389.member.member_1.y * _e389.member_1.member_3.y))))), (fma(fma(_e389.member.member_3.z, _e389.member_1.member_2.w, fma(_e389.member.member_2.z, _e389.member_1.member_2.z, fma(_e389.member.member.z, _e389.member_1.member_2.x, (_e389.member.member_1.z * _e389.member_1.member_2.y)))), _e87.z, fma(fma(_e389.member.member_3.z, _e389.member_1.member.w, fma(_e389.member.member_2.z, _e389.member_1.member.z, fma(_e389.member.member.z, _e389.member_1.member.x, (_e389.member.member_1.z * _e389.member_1.member.y)))), _e87.x, (fma(_e389.member.member_3.z, _e389.member_1.member_1.w, fma(_e389.member.member_2.z, _e389.member_1.member_1.z, fma(_e389.member.member.z, _e389.member_1.member_1.x, (_e389.member.member_1.z * _e389.member_1.member_1.y)))) * _e87.y))) + fma(_e389.member.member_3.z, _e389.member_1.member_3.w, fma(_e389.member.member_2.z, _e389.member_1.member_3.z, fma(_e389.member.member.z, _e389.member_1.member_3.x, (_e389.member.member_1.z * _e389.member_1.member_3.y))))), (fma(fma(_e389.member.member_3.w, _e389.member_1.member_2.w, fma(_e389.member.member_2.w, _e389.member_1.member_2.z, fma(_e389.member.member.w, _e389.member_1.member_2.x, (_e389.member.member_1.w * _e389.member_1.member_2.y)))), _e87.z, fma(fma(_e389.member.member_3.w, _e389.member_1.member.w, fma(_e389.member.member_2.w, _e389.member_1.member.z, fma(_e389.member.member.w, _e389.member_1.member.x, (_e389.member.member_1.w * _e389.member_1.member.y)))), _e87.x, (fma(_e389.member.member_3.w, _e389.member_1.member_1.w, fma(_e389.member.member_2.w, _e389.member_1.member_1.z, fma(_e389.member.member.w, _e389.member_1.member_1.x, (_e389.member.member_1.w * _e389.member_1.member_1.y)))) * _e87.y))) + fma(_e389.member.member_3.w, _e389.member_1.member_3.w, fma(_e389.member.member_2.w, _e389.member_1.member_3.z, fma(_e389.member.member.w, _e389.member_1.member_3.x, (_e389.member.member_1.w * _e389.member_1.member_3.y))))));
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
