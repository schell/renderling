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
    var phi_768_: bool;
    var phi_165_: type_24;
    var phi_796_: bool;
    var phi_326_: type_24;
    var phi_342_: type_24;
    var phi_343_: type_24;
    var phi_356_: type_24;
    var phi_372_: type_24;
    var phi_373_: type_24;
    var phi_399_: type_22;
    var phi_400_: bool;
    var phi_357_: type_24;
    var phi_420_: type_22;
    var phi_421_: bool;
    var phi_327_: type_24;
    var phi_426_: type_22;
    var phi_828_: bool;
    var phi_434_: f32;
    var local_3: type_22;
    var local_4: type_22;

    let _e80 = global_1;
    let _e81 = global_2;
    let _e83 = arrayLength((&global.member));
    local_2 = array<vec3<f32>, 36>(vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f));
    if (_e81 < 36u) {
        let _e86 = local_2[_e81];
        if (_e83 >= 2u) {
            phi_768_ = (_e80 <= (_e83 - 2u));
        } else {
            phi_768_ = false;
        }
        let _e91 = phi_768_;
        if _e91 {
            let _e94 = global.member[_e80];
            let _e98 = global.member[(_e80 + 1u)];
            phi_165_ = type_24(_e94, _e98);
        } else {
            phi_165_ = type_24(4294967295u, 4294967295u);
        }
        let _e101 = phi_165_;
        if (_e83 >= 83u) {
            phi_796_ = (_e101.member <= (_e83 - 83u));
        } else {
            phi_796_ = false;
        }
        let _e108 = phi_796_;
        if _e108 {
            let _e111 = global.member[_e101.member];
            let _e116 = global.member[(_e101.member + 1u)];
            let _e121 = global.member[(_e101.member + 2u)];
            let _e126 = global.member[(_e101.member + 3u)];
            let _e132 = global.member[(_e101.member + 4u)];
            let _e137 = global.member[(_e101.member + 5u)];
            let _e142 = global.member[(_e101.member + 6u)];
            let _e147 = global.member[(_e101.member + 7u)];
            let _e153 = global.member[(_e101.member + 8u)];
            let _e158 = global.member[(_e101.member + 9u)];
            let _e163 = global.member[(_e101.member + 10u)];
            let _e168 = global.member[(_e101.member + 11u)];
            let _e174 = global.member[(_e101.member + 12u)];
            let _e179 = global.member[(_e101.member + 13u)];
            let _e184 = global.member[(_e101.member + 14u)];
            let _e189 = global.member[(_e101.member + 15u)];
            let _e196 = global.member[(_e101.member + 16u)];
            let _e201 = global.member[(_e101.member + 17u)];
            let _e206 = global.member[(_e101.member + 18u)];
            let _e211 = global.member[(_e101.member + 19u)];
            let _e217 = global.member[(_e101.member + 20u)];
            let _e222 = global.member[(_e101.member + 21u)];
            let _e227 = global.member[(_e101.member + 22u)];
            let _e232 = global.member[(_e101.member + 23u)];
            let _e238 = global.member[(_e101.member + 24u)];
            let _e243 = global.member[(_e101.member + 25u)];
            let _e248 = global.member[(_e101.member + 26u)];
            let _e253 = global.member[(_e101.member + 27u)];
            let _e259 = global.member[(_e101.member + 28u)];
            let _e264 = global.member[(_e101.member + 29u)];
            let _e269 = global.member[(_e101.member + 30u)];
            let _e274 = global.member[(_e101.member + 31u)];
            let _e281 = global.member[(_e101.member + 32u)];
            let _e286 = global.member[(_e101.member + 33u)];
            let _e291 = global.member[(_e101.member + 34u)];
            local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
            phi_326_ = type_24(0u, 6u);
            loop {
                let _e296 = phi_326_;
                if (_e296.member < _e296.member_1) {
                    phi_342_ = type_24((_e296.member + 1u), _e296.member_1);
                    phi_343_ = type_24(1u, _e296.member);
                } else {
                    phi_342_ = _e296;
                    phi_343_ = type_24(0u, type_24().member_1);
                }
                let _e309 = phi_342_;
                let _e311 = phi_343_;
                switch bitcast<i32>(_e311.member) {
                    case 0: {
                        let _e315 = local_1;
                        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                        phi_356_ = type_24(0u, 8u);
                        loop {
                            let _e318 = phi_356_;
                            if (_e318.member < _e318.member_1) {
                                phi_372_ = type_24((_e318.member + 1u), _e318.member_1);
                                phi_373_ = type_24(1u, _e318.member);
                            } else {
                                phi_372_ = _e318;
                                phi_373_ = type_24(0u, type_24().member_1);
                            }
                            let _e331 = phi_372_;
                            let _e333 = phi_373_;
                            switch bitcast<i32>(_e333.member) {
                                case 0: {
                                    let _e337 = local;
                                    phi_399_ = type_22(type_20(vec4<f32>(bitcast<f32>(_e111), bitcast<f32>(_e116), bitcast<f32>(_e121), bitcast<f32>(_e126)), vec4<f32>(bitcast<f32>(_e132), bitcast<f32>(_e137), bitcast<f32>(_e142), bitcast<f32>(_e147)), vec4<f32>(bitcast<f32>(_e153), bitcast<f32>(_e158), bitcast<f32>(_e163), bitcast<f32>(_e168)), vec4<f32>(bitcast<f32>(_e174), bitcast<f32>(_e179), bitcast<f32>(_e184), bitcast<f32>(_e189))), type_20(vec4<f32>(bitcast<f32>(_e196), bitcast<f32>(_e201), bitcast<f32>(_e206), bitcast<f32>(_e211)), vec4<f32>(bitcast<f32>(_e217), bitcast<f32>(_e222), bitcast<f32>(_e227), bitcast<f32>(_e232)), vec4<f32>(bitcast<f32>(_e238), bitcast<f32>(_e243), bitcast<f32>(_e248), bitcast<f32>(_e253)), vec4<f32>(bitcast<f32>(_e259), bitcast<f32>(_e264), bitcast<f32>(_e269), bitcast<f32>(_e274))), type_21(_e337, _e315), vec3<f32>(bitcast<f32>(_e281), bitcast<f32>(_e286), bitcast<f32>(_e291)));
                                    phi_400_ = false;
                                    phi_357_ = type_24();
                                    break;
                                }
                                case 1: {
                                    let _e341 = ((_e101.member + 59u) + (_e333.member_1 * 3u));
                                    let _e344 = global.member[_e341];
                                    let _e349 = global.member[(_e341 + 1u)];
                                    let _e354 = global.member[(_e341 + 2u)];
                                    local[_e333.member_1] = vec3<f32>(bitcast<f32>(_e344), bitcast<f32>(_e349), bitcast<f32>(_e354));
                                    phi_399_ = type_22();
                                    phi_400_ = true;
                                    phi_357_ = _e331;
                                    break;
                                }
                                default: {
                                    phi_399_ = type_22();
                                    phi_400_ = false;
                                    phi_357_ = type_24();
                                    break;
                                }
                            }
                            let _e359 = phi_399_;
                            let _e361 = phi_400_;
                            let _e363 = phi_357_;
                            local_3 = _e359;
                            continue;
                            continuing {
                                phi_356_ = _e363;
                                break if !(_e361);
                            }
                        }
                        let _e548 = local_3;
                        phi_420_ = _e548;
                        phi_421_ = false;
                        phi_327_ = type_24();
                        break;
                    }
                    case 1: {
                        let _e366 = ((_e101.member + 35u) + (_e311.member_1 * 4u));
                        let _e369 = global.member[_e366];
                        let _e374 = global.member[(_e366 + 1u)];
                        let _e379 = global.member[(_e366 + 2u)];
                        let _e384 = global.member[(_e366 + 3u)];
                        local_1[_e311.member_1] = vec4<f32>(bitcast<f32>(_e369), bitcast<f32>(_e374), bitcast<f32>(_e379), bitcast<f32>(_e384));
                        phi_420_ = type_22();
                        phi_421_ = true;
                        phi_327_ = _e309;
                        break;
                    }
                    default: {
                        phi_420_ = type_22();
                        phi_421_ = false;
                        phi_327_ = type_24();
                        break;
                    }
                }
                let _e389 = phi_420_;
                let _e391 = phi_421_;
                let _e393 = phi_327_;
                local_4 = _e389;
                continue;
                continuing {
                    phi_326_ = _e393;
                    break if !(_e391);
                }
            }
            let _e553 = local_4;
            phi_426_ = _e553;
        } else {
            phi_426_ = type_22(type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_20(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_21(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
        }
        let _e396 = phi_426_;
        if (_e83 >= 1u) {
            phi_828_ = (_e101.member_1 <= (_e83 - 1u));
        } else {
            phi_828_ = false;
        }
        let _e403 = phi_828_;
        if _e403 {
            let _e406 = global.member[_e101.member_1];
            phi_434_ = bitcast<f32>(_e406);
        } else {
            phi_434_ = 0f;
        }
        let _e409 = phi_434_;
        global_3 = _e409;
        global_4 = _e86;
        global_5 = vec4<f32>((fma(fma(_e396.member.member_3.x, _e396.member_1.member_2.w, fma(_e396.member.member_2.x, _e396.member_1.member_2.z, fma(_e396.member.member.x, _e396.member_1.member_2.x, (_e396.member.member_1.x * _e396.member_1.member_2.y)))), _e86.z, fma(fma(_e396.member.member_3.x, _e396.member_1.member.w, fma(_e396.member.member_2.x, _e396.member_1.member.z, fma(_e396.member.member.x, _e396.member_1.member.x, (_e396.member.member_1.x * _e396.member_1.member.y)))), _e86.x, (fma(_e396.member.member_3.x, _e396.member_1.member_1.w, fma(_e396.member.member_2.x, _e396.member_1.member_1.z, fma(_e396.member.member.x, _e396.member_1.member_1.x, (_e396.member.member_1.x * _e396.member_1.member_1.y)))) * _e86.y))) + fma(_e396.member.member_3.x, _e396.member_1.member_3.w, fma(_e396.member.member_2.x, _e396.member_1.member_3.z, fma(_e396.member.member.x, _e396.member_1.member_3.x, (_e396.member.member_1.x * _e396.member_1.member_3.y))))), (fma(fma(_e396.member.member_3.y, _e396.member_1.member_2.w, fma(_e396.member.member_2.y, _e396.member_1.member_2.z, fma(_e396.member.member.y, _e396.member_1.member_2.x, (_e396.member.member_1.y * _e396.member_1.member_2.y)))), _e86.z, fma(fma(_e396.member.member_3.y, _e396.member_1.member.w, fma(_e396.member.member_2.y, _e396.member_1.member.z, fma(_e396.member.member.y, _e396.member_1.member.x, (_e396.member.member_1.y * _e396.member_1.member.y)))), _e86.x, (fma(_e396.member.member_3.y, _e396.member_1.member_1.w, fma(_e396.member.member_2.y, _e396.member_1.member_1.z, fma(_e396.member.member.y, _e396.member_1.member_1.x, (_e396.member.member_1.y * _e396.member_1.member_1.y)))) * _e86.y))) + fma(_e396.member.member_3.y, _e396.member_1.member_3.w, fma(_e396.member.member_2.y, _e396.member_1.member_3.z, fma(_e396.member.member.y, _e396.member_1.member_3.x, (_e396.member.member_1.y * _e396.member_1.member_3.y))))), (fma(fma(_e396.member.member_3.z, _e396.member_1.member_2.w, fma(_e396.member.member_2.z, _e396.member_1.member_2.z, fma(_e396.member.member.z, _e396.member_1.member_2.x, (_e396.member.member_1.z * _e396.member_1.member_2.y)))), _e86.z, fma(fma(_e396.member.member_3.z, _e396.member_1.member.w, fma(_e396.member.member_2.z, _e396.member_1.member.z, fma(_e396.member.member.z, _e396.member_1.member.x, (_e396.member.member_1.z * _e396.member_1.member.y)))), _e86.x, (fma(_e396.member.member_3.z, _e396.member_1.member_1.w, fma(_e396.member.member_2.z, _e396.member_1.member_1.z, fma(_e396.member.member.z, _e396.member_1.member_1.x, (_e396.member.member_1.z * _e396.member_1.member_1.y)))) * _e86.y))) + fma(_e396.member.member_3.z, _e396.member_1.member_3.w, fma(_e396.member.member_2.z, _e396.member_1.member_3.z, fma(_e396.member.member.z, _e396.member_1.member_3.x, (_e396.member.member_1.z * _e396.member_1.member_3.y))))), (fma(fma(_e396.member.member_3.w, _e396.member_1.member_2.w, fma(_e396.member.member_2.w, _e396.member_1.member_2.z, fma(_e396.member.member.w, _e396.member_1.member_2.x, (_e396.member.member_1.w * _e396.member_1.member_2.y)))), _e86.z, fma(fma(_e396.member.member_3.w, _e396.member_1.member.w, fma(_e396.member.member_2.w, _e396.member_1.member.z, fma(_e396.member.member.w, _e396.member_1.member.x, (_e396.member.member_1.w * _e396.member_1.member.y)))), _e86.x, (fma(_e396.member.member_3.w, _e396.member_1.member_1.w, fma(_e396.member.member_2.w, _e396.member_1.member_1.z, fma(_e396.member.member.w, _e396.member_1.member_1.x, (_e396.member.member_1.w * _e396.member_1.member_1.y)))) * _e86.y))) + fma(_e396.member.member_3.w, _e396.member_1.member_3.w, fma(_e396.member.member_2.w, _e396.member_1.member_3.z, fma(_e396.member.member.w, _e396.member_1.member_3.x, (_e396.member.member_1.w * _e396.member_1.member_3.y))))));
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
