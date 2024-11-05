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
var<private> global_2: u32;
var<private> global_3: vec3<f32>;
var<private> global_4: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var local_2: array<vec3<f32>, 36>;
    var phi_726_: bool;
    var phi_305_: type_23;
    var phi_321_: type_23;
    var phi_322_: type_23;
    var phi_335_: type_23;
    var phi_351_: type_23;
    var phi_352_: type_23;
    var phi_378_: type_21;
    var phi_379_: bool;
    var phi_336_: type_23;
    var phi_399_: type_21;
    var phi_400_: bool;
    var phi_306_: type_23;
    var phi_405_: type_21;
    var local_3: type_21;
    var local_4: type_21;

    let _e77 = global_2;
    let _e78 = global;
    let _e80 = arrayLength((&global_1.member));
    if (_e80 >= 83u) {
        phi_726_ = (_e77 <= (_e80 - 83u));
    } else {
        phi_726_ = false;
    }
    let _e85 = phi_726_;
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
        phi_305_ = type_23(0u, 6u);
        loop {
            let _e273 = phi_305_;
            if (_e273.member < _e273.member_1) {
                phi_321_ = type_23((_e273.member + 1u), _e273.member_1);
                phi_322_ = type_23(1u, _e273.member);
            } else {
                phi_321_ = _e273;
                phi_322_ = type_23(0u, type_23().member_1);
            }
            let _e286 = phi_321_;
            let _e288 = phi_322_;
            switch bitcast<i32>(_e288.member) {
                case 0: {
                    let _e292 = local_1;
                    local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                    phi_335_ = type_23(0u, 8u);
                    loop {
                        let _e295 = phi_335_;
                        if (_e295.member < _e295.member_1) {
                            phi_351_ = type_23((_e295.member + 1u), _e295.member_1);
                            phi_352_ = type_23(1u, _e295.member);
                        } else {
                            phi_351_ = _e295;
                            phi_352_ = type_23(0u, type_23().member_1);
                        }
                        let _e308 = phi_351_;
                        let _e310 = phi_352_;
                        switch bitcast<i32>(_e310.member) {
                            case 0: {
                                let _e314 = local;
                                phi_378_ = type_21(type_19(vec4<f32>(bitcast<f32>(_e88), bitcast<f32>(_e93), bitcast<f32>(_e98), bitcast<f32>(_e103)), vec4<f32>(bitcast<f32>(_e109), bitcast<f32>(_e114), bitcast<f32>(_e119), bitcast<f32>(_e124)), vec4<f32>(bitcast<f32>(_e130), bitcast<f32>(_e135), bitcast<f32>(_e140), bitcast<f32>(_e145)), vec4<f32>(bitcast<f32>(_e151), bitcast<f32>(_e156), bitcast<f32>(_e161), bitcast<f32>(_e166))), type_19(vec4<f32>(bitcast<f32>(_e173), bitcast<f32>(_e178), bitcast<f32>(_e183), bitcast<f32>(_e188)), vec4<f32>(bitcast<f32>(_e194), bitcast<f32>(_e199), bitcast<f32>(_e204), bitcast<f32>(_e209)), vec4<f32>(bitcast<f32>(_e215), bitcast<f32>(_e220), bitcast<f32>(_e225), bitcast<f32>(_e230)), vec4<f32>(bitcast<f32>(_e236), bitcast<f32>(_e241), bitcast<f32>(_e246), bitcast<f32>(_e251))), type_20(_e314, _e292), vec3<f32>(bitcast<f32>(_e258), bitcast<f32>(_e263), bitcast<f32>(_e268)));
                                phi_379_ = false;
                                phi_336_ = type_23();
                                break;
                            }
                            case 1: {
                                let _e318 = ((_e77 + 59u) + (_e310.member_1 * 3u));
                                let _e321 = global_1.member[_e318];
                                let _e326 = global_1.member[(_e318 + 1u)];
                                let _e331 = global_1.member[(_e318 + 2u)];
                                local[_e310.member_1] = vec3<f32>(bitcast<f32>(_e321), bitcast<f32>(_e326), bitcast<f32>(_e331));
                                phi_378_ = type_21();
                                phi_379_ = true;
                                phi_336_ = _e308;
                                break;
                            }
                            default: {
                                phi_378_ = type_21();
                                phi_379_ = false;
                                phi_336_ = type_23();
                                break;
                            }
                        }
                        let _e336 = phi_378_;
                        let _e338 = phi_379_;
                        let _e340 = phi_336_;
                        local_3 = _e336;
                        continue;
                        continuing {
                            phi_335_ = _e340;
                            break if !(_e338);
                        }
                    }
                    let _e462 = local_3;
                    phi_399_ = _e462;
                    phi_400_ = false;
                    phi_306_ = type_23();
                    break;
                }
                case 1: {
                    let _e343 = ((_e77 + 35u) + (_e288.member_1 * 4u));
                    let _e346 = global_1.member[_e343];
                    let _e351 = global_1.member[(_e343 + 1u)];
                    let _e356 = global_1.member[(_e343 + 2u)];
                    let _e361 = global_1.member[(_e343 + 3u)];
                    local_1[_e288.member_1] = vec4<f32>(bitcast<f32>(_e346), bitcast<f32>(_e351), bitcast<f32>(_e356), bitcast<f32>(_e361));
                    phi_399_ = type_21();
                    phi_400_ = true;
                    phi_306_ = _e286;
                    break;
                }
                default: {
                    phi_399_ = type_21();
                    phi_400_ = false;
                    phi_306_ = type_23();
                    break;
                }
            }
            let _e366 = phi_399_;
            let _e368 = phi_400_;
            let _e370 = phi_306_;
            local_4 = _e366;
            continue;
            continuing {
                phi_305_ = _e370;
                break if !(_e368);
            }
        }
        let _e467 = local_4;
        phi_405_ = _e467;
    } else {
        phi_405_ = type_21(type_19(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_19(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
    }
    let _e373 = phi_405_;
    local_2 = array<vec3<f32>, 36>(vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f));
    if (_e78 < 36u) {
        let _e381 = local_2[_e78];
        global_3 = _e381;
        let _e448 = (fma(fma(_e373.member.member_2.w, _e373.member_1.member_2.z, fma(_e373.member.member.w, _e373.member_1.member_2.x, (_e373.member.member_1.w * _e373.member_1.member_2.y))), _e381.z, fma(fma(_e373.member.member_2.w, _e373.member_1.member.z, fma(_e373.member.member.w, _e373.member_1.member.x, (_e373.member.member_1.w * _e373.member_1.member.y))), _e381.x, (fma(_e373.member.member_2.w, _e373.member_1.member_1.z, fma(_e373.member.member.w, _e373.member_1.member_1.x, (_e373.member.member_1.w * _e373.member_1.member_1.y))) * _e381.y))) + _e373.member.member_3.w);
        global_4 = vec4<f32>((fma(fma(_e373.member.member_2.x, _e373.member_1.member_2.z, fma(_e373.member.member.x, _e373.member_1.member_2.x, (_e373.member.member_1.x * _e373.member_1.member_2.y))), _e381.z, fma(fma(_e373.member.member_2.x, _e373.member_1.member.z, fma(_e373.member.member.x, _e373.member_1.member.x, (_e373.member.member_1.x * _e373.member_1.member.y))), _e381.x, (fma(_e373.member.member_2.x, _e373.member_1.member_1.z, fma(_e373.member.member.x, _e373.member_1.member_1.x, (_e373.member.member_1.x * _e373.member_1.member_1.y))) * _e381.y))) + _e373.member.member_3.x), (fma(fma(_e373.member.member_2.y, _e373.member_1.member_2.z, fma(_e373.member.member.y, _e373.member_1.member_2.x, (_e373.member.member_1.y * _e373.member_1.member_2.y))), _e381.z, fma(fma(_e373.member.member_2.y, _e373.member_1.member.z, fma(_e373.member.member.y, _e373.member_1.member.x, (_e373.member.member_1.y * _e373.member_1.member.y))), _e381.x, (fma(_e373.member.member_2.y, _e373.member_1.member_1.z, fma(_e373.member.member.y, _e373.member_1.member_1.x, (_e373.member.member_1.y * _e373.member_1.member_1.y))) * _e381.y))) + _e373.member.member_3.y), _e448, _e448);
    }
    return;
}

@vertex 
fn skyboxskybox_vertex(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_2 = param;
    global = param_1;
    function();
    let _e7 = global_4.y;
    global_4.y = -(_e7);
    let _e9 = global_3;
    let _e10 = global_4;
    return VertexOutput(_e9, _e10);
}
