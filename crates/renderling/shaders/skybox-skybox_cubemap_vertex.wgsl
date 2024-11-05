struct type_10 {
    member: array<u32>,
}

struct type_19 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_20 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
}

struct type_21 {
    member: type_19,
    member_1: type_19,
    member_2: type_20,
    member_3: vec3<f32>,
}

struct type_23 {
    member: u32,
    member_1: u32,
}

struct VertexOutput {
    @location(0) member: vec3<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
@group(0) @binding(0) 
var<storage> global_1: type_10;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: vec3<f32>;
var<private> global_4: u32;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var local_2: array<vec3<f32>, 36>;
    var phi_690_: bool;
    var phi_517_: type_23;
    var phi_533_: type_23;
    var phi_534_: type_23;
    var phi_547_: type_23;
    var phi_563_: type_23;
    var phi_564_: type_23;
    var phi_590_: type_21;
    var phi_591_: bool;
    var phi_548_: type_23;
    var phi_611_: type_21;
    var phi_612_: bool;
    var phi_518_: type_23;
    var phi_617_: type_21;
    var local_3: type_21;
    var local_4: type_21;

    let _e77 = global_4;
    let _e78 = global;
    let _e80 = arrayLength((&global_1.member));
    if (_e80 >= 83u) {
        phi_690_ = (_e77 <= (_e80 - 83u));
    } else {
        phi_690_ = false;
    }
    let _e85 = phi_690_;
    if _e85 {
        let _e88 = global_1.member[_e77];
        let _e93 = global_1.member[(_e77 + 1u)];
        let _e98 = global_1.member[(_e77 + 2u)];
        let _e103 = global_1.member[(_e77 + 3u)];
        let _e109 = global_1.member[(_e77 + 4u)];
        let _e114 = global_1.member[(_e77 + 5u)];
        let _e119 = global_1.member[(_e77 + 6u)];
        let _e124 = global_1.member[(_e77 + 7u)];
        let _e130 = global_1.member[(_e77 + 8u)];
        let _e135 = global_1.member[(_e77 + 9u)];
        let _e140 = global_1.member[(_e77 + 10u)];
        let _e145 = global_1.member[(_e77 + 11u)];
        let _e151 = global_1.member[(_e77 + 12u)];
        let _e156 = global_1.member[(_e77 + 13u)];
        let _e161 = global_1.member[(_e77 + 14u)];
        let _e166 = global_1.member[(_e77 + 15u)];
        let _e173 = global_1.member[(_e77 + 16u)];
        let _e178 = global_1.member[(_e77 + 17u)];
        let _e183 = global_1.member[(_e77 + 18u)];
        let _e188 = global_1.member[(_e77 + 19u)];
        let _e194 = global_1.member[(_e77 + 20u)];
        let _e199 = global_1.member[(_e77 + 21u)];
        let _e204 = global_1.member[(_e77 + 22u)];
        let _e209 = global_1.member[(_e77 + 23u)];
        let _e215 = global_1.member[(_e77 + 24u)];
        let _e220 = global_1.member[(_e77 + 25u)];
        let _e225 = global_1.member[(_e77 + 26u)];
        let _e230 = global_1.member[(_e77 + 27u)];
        let _e236 = global_1.member[(_e77 + 28u)];
        let _e241 = global_1.member[(_e77 + 29u)];
        let _e246 = global_1.member[(_e77 + 30u)];
        let _e251 = global_1.member[(_e77 + 31u)];
        let _e258 = global_1.member[(_e77 + 32u)];
        let _e263 = global_1.member[(_e77 + 33u)];
        let _e268 = global_1.member[(_e77 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_517_ = type_23(0u, 6u);
        loop {
            let _e273 = phi_517_;
            if (_e273.member < _e273.member_1) {
                phi_533_ = type_23((_e273.member + 1u), _e273.member_1);
                phi_534_ = type_23(1u, _e273.member);
            } else {
                phi_533_ = _e273;
                phi_534_ = type_23(0u, type_23().member_1);
            }
            let _e286 = phi_533_;
            let _e288 = phi_534_;
            switch bitcast<i32>(_e288.member) {
                case 0: {
                    let _e292 = local_1;
                    local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                    phi_547_ = type_23(0u, 8u);
                    loop {
                        let _e295 = phi_547_;
                        if (_e295.member < _e295.member_1) {
                            phi_563_ = type_23((_e295.member + 1u), _e295.member_1);
                            phi_564_ = type_23(1u, _e295.member);
                        } else {
                            phi_563_ = _e295;
                            phi_564_ = type_23(0u, type_23().member_1);
                        }
                        let _e308 = phi_563_;
                        let _e310 = phi_564_;
                        switch bitcast<i32>(_e310.member) {
                            case 0: {
                                let _e314 = local;
                                phi_590_ = type_21(type_19(vec4<f32>(bitcast<f32>(_e88), bitcast<f32>(_e93), bitcast<f32>(_e98), bitcast<f32>(_e103)), vec4<f32>(bitcast<f32>(_e109), bitcast<f32>(_e114), bitcast<f32>(_e119), bitcast<f32>(_e124)), vec4<f32>(bitcast<f32>(_e130), bitcast<f32>(_e135), bitcast<f32>(_e140), bitcast<f32>(_e145)), vec4<f32>(bitcast<f32>(_e151), bitcast<f32>(_e156), bitcast<f32>(_e161), bitcast<f32>(_e166))), type_19(vec4<f32>(bitcast<f32>(_e173), bitcast<f32>(_e178), bitcast<f32>(_e183), bitcast<f32>(_e188)), vec4<f32>(bitcast<f32>(_e194), bitcast<f32>(_e199), bitcast<f32>(_e204), bitcast<f32>(_e209)), vec4<f32>(bitcast<f32>(_e215), bitcast<f32>(_e220), bitcast<f32>(_e225), bitcast<f32>(_e230)), vec4<f32>(bitcast<f32>(_e236), bitcast<f32>(_e241), bitcast<f32>(_e246), bitcast<f32>(_e251))), type_20(_e314, _e292), vec3<f32>(bitcast<f32>(_e258), bitcast<f32>(_e263), bitcast<f32>(_e268)));
                                phi_591_ = false;
                                phi_548_ = type_23();
                                break;
                            }
                            case 1: {
                                let _e318 = ((_e77 + 59u) + (_e310.member_1 * 3u));
                                let _e321 = global_1.member[_e318];
                                let _e326 = global_1.member[(_e318 + 1u)];
                                let _e331 = global_1.member[(_e318 + 2u)];
                                local[_e310.member_1] = vec3<f32>(bitcast<f32>(_e321), bitcast<f32>(_e326), bitcast<f32>(_e331));
                                phi_590_ = type_21();
                                phi_591_ = true;
                                phi_548_ = _e308;
                                break;
                            }
                            default: {
                                phi_590_ = type_21();
                                phi_591_ = false;
                                phi_548_ = type_23();
                                break;
                            }
                        }
                        let _e336 = phi_590_;
                        let _e338 = phi_591_;
                        let _e340 = phi_548_;
                        local_3 = _e336;
                        continue;
                        continuing {
                            phi_547_ = _e340;
                            break if !(_e338);
                        }
                    }
                    let _e515 = local_3;
                    phi_611_ = _e515;
                    phi_612_ = false;
                    phi_518_ = type_23();
                    break;
                }
                case 1: {
                    let _e343 = ((_e77 + 35u) + (_e288.member_1 * 4u));
                    let _e346 = global_1.member[_e343];
                    let _e351 = global_1.member[(_e343 + 1u)];
                    let _e356 = global_1.member[(_e343 + 2u)];
                    let _e361 = global_1.member[(_e343 + 3u)];
                    local_1[_e288.member_1] = vec4<f32>(bitcast<f32>(_e346), bitcast<f32>(_e351), bitcast<f32>(_e356), bitcast<f32>(_e361));
                    phi_611_ = type_21();
                    phi_612_ = true;
                    phi_518_ = _e286;
                    break;
                }
                default: {
                    phi_611_ = type_21();
                    phi_612_ = false;
                    phi_518_ = type_23();
                    break;
                }
            }
            let _e366 = phi_611_;
            let _e368 = phi_612_;
            let _e370 = phi_518_;
            local_4 = _e366;
            continue;
            continuing {
                phi_517_ = _e370;
                break if !(_e368);
            }
        }
        let _e520 = local_4;
        phi_617_ = _e520;
    } else {
        phi_617_ = type_21(type_19(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_19(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
    }
    let _e373 = phi_617_;
    local_2 = array<vec3<f32>, 36>(vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f));
    if (_e78 < 36u) {
        let _e378 = local_2[_e78];
        global_3 = _e378;
        global_2 = vec4<f32>((fma(fma(_e373.member.member_3.x, _e373.member_1.member_2.w, fma(_e373.member.member_2.x, _e373.member_1.member_2.z, fma(_e373.member.member.x, _e373.member_1.member_2.x, (_e373.member.member_1.x * _e373.member_1.member_2.y)))), _e378.z, fma(fma(_e373.member.member_3.x, _e373.member_1.member.w, fma(_e373.member.member_2.x, _e373.member_1.member.z, fma(_e373.member.member.x, _e373.member_1.member.x, (_e373.member.member_1.x * _e373.member_1.member.y)))), _e378.x, (fma(_e373.member.member_3.x, _e373.member_1.member_1.w, fma(_e373.member.member_2.x, _e373.member_1.member_1.z, fma(_e373.member.member.x, _e373.member_1.member_1.x, (_e373.member.member_1.x * _e373.member_1.member_1.y)))) * _e378.y))) + fma(_e373.member.member_3.x, _e373.member_1.member_3.w, fma(_e373.member.member_2.x, _e373.member_1.member_3.z, fma(_e373.member.member.x, _e373.member_1.member_3.x, (_e373.member.member_1.x * _e373.member_1.member_3.y))))), (fma(fma(_e373.member.member_3.y, _e373.member_1.member_2.w, fma(_e373.member.member_2.y, _e373.member_1.member_2.z, fma(_e373.member.member.y, _e373.member_1.member_2.x, (_e373.member.member_1.y * _e373.member_1.member_2.y)))), _e378.z, fma(fma(_e373.member.member_3.y, _e373.member_1.member.w, fma(_e373.member.member_2.y, _e373.member_1.member.z, fma(_e373.member.member.y, _e373.member_1.member.x, (_e373.member.member_1.y * _e373.member_1.member.y)))), _e378.x, (fma(_e373.member.member_3.y, _e373.member_1.member_1.w, fma(_e373.member.member_2.y, _e373.member_1.member_1.z, fma(_e373.member.member.y, _e373.member_1.member_1.x, (_e373.member.member_1.y * _e373.member_1.member_1.y)))) * _e378.y))) + fma(_e373.member.member_3.y, _e373.member_1.member_3.w, fma(_e373.member.member_2.y, _e373.member_1.member_3.z, fma(_e373.member.member.y, _e373.member_1.member_3.x, (_e373.member.member_1.y * _e373.member_1.member_3.y))))), (fma(fma(_e373.member.member_3.z, _e373.member_1.member_2.w, fma(_e373.member.member_2.z, _e373.member_1.member_2.z, fma(_e373.member.member.z, _e373.member_1.member_2.x, (_e373.member.member_1.z * _e373.member_1.member_2.y)))), _e378.z, fma(fma(_e373.member.member_3.z, _e373.member_1.member.w, fma(_e373.member.member_2.z, _e373.member_1.member.z, fma(_e373.member.member.z, _e373.member_1.member.x, (_e373.member.member_1.z * _e373.member_1.member.y)))), _e378.x, (fma(_e373.member.member_3.z, _e373.member_1.member_1.w, fma(_e373.member.member_2.z, _e373.member_1.member_1.z, fma(_e373.member.member.z, _e373.member_1.member_1.x, (_e373.member.member_1.z * _e373.member_1.member_1.y)))) * _e378.y))) + fma(_e373.member.member_3.z, _e373.member_1.member_3.w, fma(_e373.member.member_2.z, _e373.member_1.member_3.z, fma(_e373.member.member.z, _e373.member_1.member_3.x, (_e373.member.member_1.z * _e373.member_1.member_3.y))))), (fma(fma(_e373.member.member_3.w, _e373.member_1.member_2.w, fma(_e373.member.member_2.w, _e373.member_1.member_2.z, fma(_e373.member.member.w, _e373.member_1.member_2.x, (_e373.member.member_1.w * _e373.member_1.member_2.y)))), _e378.z, fma(fma(_e373.member.member_3.w, _e373.member_1.member.w, fma(_e373.member.member_2.w, _e373.member_1.member.z, fma(_e373.member.member.w, _e373.member_1.member.x, (_e373.member.member_1.w * _e373.member_1.member.y)))), _e378.x, (fma(_e373.member.member_3.w, _e373.member_1.member_1.w, fma(_e373.member.member_2.w, _e373.member_1.member_1.z, fma(_e373.member.member.w, _e373.member_1.member_1.x, (_e373.member.member_1.w * _e373.member_1.member_1.y)))) * _e378.y))) + fma(_e373.member.member_3.w, _e373.member_1.member_3.w, fma(_e373.member.member_2.w, _e373.member_1.member_3.z, fma(_e373.member.member.w, _e373.member_1.member_3.x, (_e373.member.member_1.w * _e373.member_1.member_3.y))))));
    }
    return;
}

@vertex 
fn skyboxskybox_cubemap_vertex(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_4 = param;
    global = param_1;
    function();
    let _e7 = global_2.y;
    global_2.y = -(_e7);
    let _e9 = global_3;
    let _e10 = global_2;
    return VertexOutput(_e9, _e10);
}
