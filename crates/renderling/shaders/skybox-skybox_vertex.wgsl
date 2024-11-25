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
    member_2: vec3<f32>,
}

struct type_21 {
    member: type_19,
    member_1: type_19,
    member_2: vec3<f32>,
    member_3: type_20,
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
    var phi_743_: bool;
    var phi_301_: type_23;
    var phi_302_: type_23;
    var phi_325_: type_23;
    var phi_352_: bool;
    var phi_358_: type_23;
    var phi_359_: type_23;
    var phi_382_: type_23;
    var phi_405_: bool;
    var phi_426_: type_21;

    switch bitcast<i32>(0u) {
        default: {
            let _e81 = global_3;
            let _e82 = global;
            let _e84 = arrayLength((&global_1.member));
            if (_e84 >= 86u) {
                phi_743_ = (_e81 <= (_e84 - 86u));
            } else {
                phi_743_ = false;
            }
            let _e89 = phi_743_;
            if _e89 {
                let _e92 = global_1.member[_e81];
                let _e97 = global_1.member[(_e81 + 1u)];
                let _e102 = global_1.member[(_e81 + 2u)];
                let _e107 = global_1.member[(_e81 + 3u)];
                let _e113 = global_1.member[(_e81 + 4u)];
                let _e118 = global_1.member[(_e81 + 5u)];
                let _e123 = global_1.member[(_e81 + 6u)];
                let _e128 = global_1.member[(_e81 + 7u)];
                let _e134 = global_1.member[(_e81 + 8u)];
                let _e139 = global_1.member[(_e81 + 9u)];
                let _e144 = global_1.member[(_e81 + 10u)];
                let _e149 = global_1.member[(_e81 + 11u)];
                let _e155 = global_1.member[(_e81 + 12u)];
                let _e160 = global_1.member[(_e81 + 13u)];
                let _e165 = global_1.member[(_e81 + 14u)];
                let _e170 = global_1.member[(_e81 + 15u)];
                let _e177 = global_1.member[(_e81 + 16u)];
                let _e182 = global_1.member[(_e81 + 17u)];
                let _e187 = global_1.member[(_e81 + 18u)];
                let _e192 = global_1.member[(_e81 + 19u)];
                let _e198 = global_1.member[(_e81 + 20u)];
                let _e203 = global_1.member[(_e81 + 21u)];
                let _e208 = global_1.member[(_e81 + 22u)];
                let _e213 = global_1.member[(_e81 + 23u)];
                let _e219 = global_1.member[(_e81 + 24u)];
                let _e224 = global_1.member[(_e81 + 25u)];
                let _e229 = global_1.member[(_e81 + 26u)];
                let _e234 = global_1.member[(_e81 + 27u)];
                let _e240 = global_1.member[(_e81 + 28u)];
                let _e245 = global_1.member[(_e81 + 29u)];
                let _e250 = global_1.member[(_e81 + 30u)];
                let _e255 = global_1.member[(_e81 + 31u)];
                let _e262 = global_1.member[(_e81 + 32u)];
                let _e267 = global_1.member[(_e81 + 33u)];
                let _e272 = global_1.member[(_e81 + 34u)];
                local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_301_ = type_23(0u, 6u);
                loop {
                    let _e277 = phi_301_;
                    if (_e277.member < _e277.member_1) {
                        phi_302_ = type_23((_e277.member + 1u), _e277.member_1);
                        phi_325_ = type_23(1u, _e277.member);
                    } else {
                        phi_302_ = _e277;
                        phi_325_ = type_23(0u, type_23().member_1);
                    }
                    let _e290 = phi_302_;
                    let _e292 = phi_325_;
                    switch bitcast<i32>(_e292.member) {
                        case 0: {
                            phi_352_ = false;
                            break;
                        }
                        case 1: {
                            let _e297 = ((_e81 + 35u) + (_e292.member_1 * 4u));
                            let _e300 = global_1.member[_e297];
                            let _e305 = global_1.member[(_e297 + 1u)];
                            let _e310 = global_1.member[(_e297 + 2u)];
                            let _e315 = global_1.member[(_e297 + 3u)];
                            local_1[_e292.member_1] = vec4<f32>(bitcast<f32>(_e300), bitcast<f32>(_e305), bitcast<f32>(_e310), bitcast<f32>(_e315));
                            phi_352_ = true;
                            break;
                        }
                        default: {
                            phi_352_ = bool();
                            break;
                        }
                    }
                    let _e320 = phi_352_;
                    continue;
                    continuing {
                        phi_301_ = _e290;
                        break if !(_e320);
                    }
                }
                let _e322 = local_1;
                local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                phi_358_ = type_23(0u, 8u);
                loop {
                    let _e325 = phi_358_;
                    if (_e325.member < _e325.member_1) {
                        phi_359_ = type_23((_e325.member + 1u), _e325.member_1);
                        phi_382_ = type_23(1u, _e325.member);
                    } else {
                        phi_359_ = _e325;
                        phi_382_ = type_23(0u, type_23().member_1);
                    }
                    let _e338 = phi_359_;
                    let _e340 = phi_382_;
                    switch bitcast<i32>(_e340.member) {
                        case 0: {
                            phi_405_ = false;
                            break;
                        }
                        case 1: {
                            let _e345 = ((_e81 + 59u) + (_e340.member_1 * 3u));
                            let _e348 = global_1.member[_e345];
                            let _e353 = global_1.member[(_e345 + 1u)];
                            let _e358 = global_1.member[(_e345 + 2u)];
                            local[_e340.member_1] = vec3<f32>(bitcast<f32>(_e348), bitcast<f32>(_e353), bitcast<f32>(_e358));
                            phi_405_ = true;
                            break;
                        }
                        default: {
                            phi_405_ = bool();
                            break;
                        }
                    }
                    let _e363 = phi_405_;
                    continue;
                    continuing {
                        phi_358_ = _e338;
                        break if !(_e363);
                    }
                }
                let _e365 = local;
                let _e369 = global_1.member[(_e81 + 83u)];
                let _e374 = global_1.member[(_e81 + 84u)];
                let _e379 = global_1.member[(_e81 + 85u)];
                phi_426_ = type_21(type_19(vec4<f32>(bitcast<f32>(_e92), bitcast<f32>(_e97), bitcast<f32>(_e102), bitcast<f32>(_e107)), vec4<f32>(bitcast<f32>(_e113), bitcast<f32>(_e118), bitcast<f32>(_e123), bitcast<f32>(_e128)), vec4<f32>(bitcast<f32>(_e134), bitcast<f32>(_e139), bitcast<f32>(_e144), bitcast<f32>(_e149)), vec4<f32>(bitcast<f32>(_e155), bitcast<f32>(_e160), bitcast<f32>(_e165), bitcast<f32>(_e170))), type_19(vec4<f32>(bitcast<f32>(_e177), bitcast<f32>(_e182), bitcast<f32>(_e187), bitcast<f32>(_e192)), vec4<f32>(bitcast<f32>(_e198), bitcast<f32>(_e203), bitcast<f32>(_e208), bitcast<f32>(_e213)), vec4<f32>(bitcast<f32>(_e219), bitcast<f32>(_e224), bitcast<f32>(_e229), bitcast<f32>(_e234)), vec4<f32>(bitcast<f32>(_e240), bitcast<f32>(_e245), bitcast<f32>(_e250), bitcast<f32>(_e255))), vec3<f32>(bitcast<f32>(_e262), bitcast<f32>(_e267), bitcast<f32>(_e272)), type_20(_e365, _e322, vec3<f32>(bitcast<f32>(_e369), bitcast<f32>(_e374), bitcast<f32>(_e379))));
            } else {
                phi_426_ = type_21(type_19(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_19(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), vec3<f32>(0f, 0f, 0f), type_20(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)), vec3<f32>(0f, 0f, 0f)));
            }
            let _e385 = phi_426_;
            local_2 = array<vec3<f32>, 36>(vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f));
            if (_e82 < 36u) {
            } else {
                break;
            }
            let _e393 = local_2[_e82];
            global_4 = _e393;
            let _e460 = (fma(fma(_e385.member.member_2.w, _e385.member_1.member_2.z, fma(_e385.member.member.w, _e385.member_1.member_2.x, (_e385.member.member_1.w * _e385.member_1.member_2.y))), _e393.z, fma(fma(_e385.member.member_2.w, _e385.member_1.member.z, fma(_e385.member.member.w, _e385.member_1.member.x, (_e385.member.member_1.w * _e385.member_1.member.y))), _e393.x, (fma(_e385.member.member_2.w, _e385.member_1.member_1.z, fma(_e385.member.member.w, _e385.member_1.member_1.x, (_e385.member.member_1.w * _e385.member_1.member_1.y))) * _e393.y))) + _e385.member.member_3.w);
            global_2 = vec4<f32>((fma(fma(_e385.member.member_2.x, _e385.member_1.member_2.z, fma(_e385.member.member.x, _e385.member_1.member_2.x, (_e385.member.member_1.x * _e385.member_1.member_2.y))), _e393.z, fma(fma(_e385.member.member_2.x, _e385.member_1.member.z, fma(_e385.member.member.x, _e385.member_1.member.x, (_e385.member.member_1.x * _e385.member_1.member.y))), _e393.x, (fma(_e385.member.member_2.x, _e385.member_1.member_1.z, fma(_e385.member.member.x, _e385.member_1.member_1.x, (_e385.member.member_1.x * _e385.member_1.member_1.y))) * _e393.y))) + _e385.member.member_3.x), (fma(fma(_e385.member.member_2.y, _e385.member_1.member_2.z, fma(_e385.member.member.y, _e385.member_1.member_2.x, (_e385.member.member_1.y * _e385.member_1.member_2.y))), _e393.z, fma(fma(_e385.member.member_2.y, _e385.member_1.member.z, fma(_e385.member.member.y, _e385.member_1.member.x, (_e385.member.member_1.y * _e385.member_1.member.y))), _e393.x, (fma(_e385.member.member_2.y, _e385.member_1.member_1.z, fma(_e385.member.member.y, _e385.member_1.member_1.x, (_e385.member.member_1.y * _e385.member_1.member_1.y))) * _e393.y))) + _e385.member.member_3.y), _e460, _e460);
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
