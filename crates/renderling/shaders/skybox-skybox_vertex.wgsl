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
var<private> global_3: u32;
var<private> global_4: vec3<f32>;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var local_2: array<vec3<f32>, 36>;
    var phi_730_: bool;
    var phi_301_: type_23;
    var phi_302_: type_23;
    var phi_325_: type_23;
    var phi_352_: bool;
    var phi_358_: type_23;
    var phi_359_: type_23;
    var phi_382_: type_23;
    var phi_405_: bool;
    var phi_413_: type_21;

    switch bitcast<i32>(0u) {
        default: {
            let _e78 = global_3;
            let _e79 = global;
            let _e81 = arrayLength((&global_1.member));
            if (_e81 >= 83u) {
                phi_730_ = (_e78 <= (_e81 - 83u));
            } else {
                phi_730_ = false;
            }
            let _e86 = phi_730_;
            if _e86 {
                let _e89 = global_1.member[_e78];
                let _e94 = global_1.member[(_e78 + 1u)];
                let _e99 = global_1.member[(_e78 + 2u)];
                let _e104 = global_1.member[(_e78 + 3u)];
                let _e110 = global_1.member[(_e78 + 4u)];
                let _e115 = global_1.member[(_e78 + 5u)];
                let _e120 = global_1.member[(_e78 + 6u)];
                let _e125 = global_1.member[(_e78 + 7u)];
                let _e131 = global_1.member[(_e78 + 8u)];
                let _e136 = global_1.member[(_e78 + 9u)];
                let _e141 = global_1.member[(_e78 + 10u)];
                let _e146 = global_1.member[(_e78 + 11u)];
                let _e152 = global_1.member[(_e78 + 12u)];
                let _e157 = global_1.member[(_e78 + 13u)];
                let _e162 = global_1.member[(_e78 + 14u)];
                let _e167 = global_1.member[(_e78 + 15u)];
                let _e174 = global_1.member[(_e78 + 16u)];
                let _e179 = global_1.member[(_e78 + 17u)];
                let _e184 = global_1.member[(_e78 + 18u)];
                let _e189 = global_1.member[(_e78 + 19u)];
                let _e195 = global_1.member[(_e78 + 20u)];
                let _e200 = global_1.member[(_e78 + 21u)];
                let _e205 = global_1.member[(_e78 + 22u)];
                let _e210 = global_1.member[(_e78 + 23u)];
                let _e216 = global_1.member[(_e78 + 24u)];
                let _e221 = global_1.member[(_e78 + 25u)];
                let _e226 = global_1.member[(_e78 + 26u)];
                let _e231 = global_1.member[(_e78 + 27u)];
                let _e237 = global_1.member[(_e78 + 28u)];
                let _e242 = global_1.member[(_e78 + 29u)];
                let _e247 = global_1.member[(_e78 + 30u)];
                let _e252 = global_1.member[(_e78 + 31u)];
                let _e259 = global_1.member[(_e78 + 32u)];
                let _e264 = global_1.member[(_e78 + 33u)];
                let _e269 = global_1.member[(_e78 + 34u)];
                local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_301_ = type_23(0u, 6u);
                loop {
                    let _e274 = phi_301_;
                    if (_e274.member < _e274.member_1) {
                        phi_302_ = type_23((_e274.member + 1u), _e274.member_1);
                        phi_325_ = type_23(1u, _e274.member);
                    } else {
                        phi_302_ = _e274;
                        phi_325_ = type_23(0u, type_23().member_1);
                    }
                    let _e287 = phi_302_;
                    let _e289 = phi_325_;
                    switch bitcast<i32>(_e289.member) {
                        case 0: {
                            phi_352_ = false;
                            break;
                        }
                        case 1: {
                            let _e294 = ((_e78 + 35u) + (_e289.member_1 * 4u));
                            let _e297 = global_1.member[_e294];
                            let _e302 = global_1.member[(_e294 + 1u)];
                            let _e307 = global_1.member[(_e294 + 2u)];
                            let _e312 = global_1.member[(_e294 + 3u)];
                            local_1[_e289.member_1] = vec4<f32>(bitcast<f32>(_e297), bitcast<f32>(_e302), bitcast<f32>(_e307), bitcast<f32>(_e312));
                            phi_352_ = true;
                            break;
                        }
                        default: {
                            phi_352_ = bool();
                            break;
                        }
                    }
                    let _e317 = phi_352_;
                    continue;
                    continuing {
                        phi_301_ = _e287;
                        break if !(_e317);
                    }
                }
                let _e319 = local_1;
                local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                phi_358_ = type_23(0u, 8u);
                loop {
                    let _e322 = phi_358_;
                    if (_e322.member < _e322.member_1) {
                        phi_359_ = type_23((_e322.member + 1u), _e322.member_1);
                        phi_382_ = type_23(1u, _e322.member);
                    } else {
                        phi_359_ = _e322;
                        phi_382_ = type_23(0u, type_23().member_1);
                    }
                    let _e335 = phi_359_;
                    let _e337 = phi_382_;
                    switch bitcast<i32>(_e337.member) {
                        case 0: {
                            phi_405_ = false;
                            break;
                        }
                        case 1: {
                            let _e342 = ((_e78 + 59u) + (_e337.member_1 * 3u));
                            let _e345 = global_1.member[_e342];
                            let _e350 = global_1.member[(_e342 + 1u)];
                            let _e355 = global_1.member[(_e342 + 2u)];
                            local[_e337.member_1] = vec3<f32>(bitcast<f32>(_e345), bitcast<f32>(_e350), bitcast<f32>(_e355));
                            phi_405_ = true;
                            break;
                        }
                        default: {
                            phi_405_ = bool();
                            break;
                        }
                    }
                    let _e360 = phi_405_;
                    continue;
                    continuing {
                        phi_358_ = _e335;
                        break if !(_e360);
                    }
                }
                let _e362 = local;
                phi_413_ = type_21(type_19(vec4<f32>(bitcast<f32>(_e89), bitcast<f32>(_e94), bitcast<f32>(_e99), bitcast<f32>(_e104)), vec4<f32>(bitcast<f32>(_e110), bitcast<f32>(_e115), bitcast<f32>(_e120), bitcast<f32>(_e125)), vec4<f32>(bitcast<f32>(_e131), bitcast<f32>(_e136), bitcast<f32>(_e141), bitcast<f32>(_e146)), vec4<f32>(bitcast<f32>(_e152), bitcast<f32>(_e157), bitcast<f32>(_e162), bitcast<f32>(_e167))), type_19(vec4<f32>(bitcast<f32>(_e174), bitcast<f32>(_e179), bitcast<f32>(_e184), bitcast<f32>(_e189)), vec4<f32>(bitcast<f32>(_e195), bitcast<f32>(_e200), bitcast<f32>(_e205), bitcast<f32>(_e210)), vec4<f32>(bitcast<f32>(_e216), bitcast<f32>(_e221), bitcast<f32>(_e226), bitcast<f32>(_e231)), vec4<f32>(bitcast<f32>(_e237), bitcast<f32>(_e242), bitcast<f32>(_e247), bitcast<f32>(_e252))), type_20(_e362, _e319), vec3<f32>(bitcast<f32>(_e259), bitcast<f32>(_e264), bitcast<f32>(_e269)));
            } else {
                phi_413_ = type_21(type_19(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_19(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
            }
            let _e366 = phi_413_;
            local_2 = array<vec3<f32>, 36>(vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f));
            if (_e79 < 36u) {
            } else {
                break;
            }
            let _e374 = local_2[_e79];
            global_4 = _e374;
            let _e441 = (fma(fma(_e366.member.member_2.w, _e366.member_1.member_2.z, fma(_e366.member.member.w, _e366.member_1.member_2.x, (_e366.member.member_1.w * _e366.member_1.member_2.y))), _e374.z, fma(fma(_e366.member.member_2.w, _e366.member_1.member.z, fma(_e366.member.member.w, _e366.member_1.member.x, (_e366.member.member_1.w * _e366.member_1.member.y))), _e374.x, (fma(_e366.member.member_2.w, _e366.member_1.member_1.z, fma(_e366.member.member.w, _e366.member_1.member_1.x, (_e366.member.member_1.w * _e366.member_1.member_1.y))) * _e374.y))) + _e366.member.member_3.w);
            global_2 = vec4<f32>((fma(fma(_e366.member.member_2.x, _e366.member_1.member_2.z, fma(_e366.member.member.x, _e366.member_1.member_2.x, (_e366.member.member_1.x * _e366.member_1.member_2.y))), _e374.z, fma(fma(_e366.member.member_2.x, _e366.member_1.member.z, fma(_e366.member.member.x, _e366.member_1.member.x, (_e366.member.member_1.x * _e366.member_1.member.y))), _e374.x, (fma(_e366.member.member_2.x, _e366.member_1.member_1.z, fma(_e366.member.member.x, _e366.member_1.member_1.x, (_e366.member.member_1.x * _e366.member_1.member_1.y))) * _e374.y))) + _e366.member.member_3.x), (fma(fma(_e366.member.member_2.y, _e366.member_1.member_2.z, fma(_e366.member.member.y, _e366.member_1.member_2.x, (_e366.member.member_1.y * _e366.member_1.member_2.y))), _e374.z, fma(fma(_e366.member.member_2.y, _e366.member_1.member.z, fma(_e366.member.member.y, _e366.member_1.member.x, (_e366.member.member_1.y * _e366.member_1.member.y))), _e374.x, (fma(_e366.member.member_2.y, _e366.member_1.member_1.z, fma(_e366.member.member.y, _e366.member_1.member_1.x, (_e366.member.member_1.y * _e366.member_1.member_1.y))) * _e374.y))) + _e366.member.member_3.y), _e441, _e441);
            break;
        }
    }
    return;
}

@vertex 
fn skyboxskybox_vertex(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_3 = param;
    global = param_1;
    function();
    let _e7 = global_2.y;
    global_2.y = -(_e7);
    let _e9 = global_4;
    let _e10 = global_2;
    return VertexOutput(_e9, _e10);
}
