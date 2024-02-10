struct type_3 {
    member: array<u32>,
}

struct VertexOutput {
    @location(0) member: vec3<f32>,
    @builtin(position) member_1: vec4<f32>,
}

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: u32;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: u32;
var<private> global_4: vec3<f32>;

fn function() {
    var local: array<vec3<f32>, 36>;

    let _e53 = global_3;
    let _e54 = global_1;
    let _e56 = arrayLength((&global.member));
    if (_e53 < _e56) {
        let _e60 = global.member[_e53];
        let _e61 = bitcast<f32>(_e60);
        let _e62 = (_e53 + 1u);
        if (_e62 < _e56) {
            let _e66 = global.member[_e62];
            let _e67 = bitcast<f32>(_e66);
            if ((_e53 + 2u) < _e56) {
                let _e70 = (_e53 + 3u);
                if (_e70 < _e56) {
                    let _e74 = global.member[_e70];
                    let _e75 = bitcast<f32>(_e74);
                    let _e76 = (_e53 + 4u);
                    if (_e76 < _e56) {
                        let _e80 = global.member[_e76];
                        let _e81 = bitcast<f32>(_e80);
                        let _e82 = (_e53 + 5u);
                        if (_e82 < _e56) {
                            let _e86 = global.member[_e82];
                            let _e87 = bitcast<f32>(_e86);
                            if ((_e53 + 6u) < _e56) {
                                let _e90 = (_e53 + 7u);
                                if (_e90 < _e56) {
                                    let _e94 = global.member[_e90];
                                    let _e95 = bitcast<f32>(_e94);
                                    let _e96 = (_e53 + 8u);
                                    if (_e96 < _e56) {
                                        let _e100 = global.member[_e96];
                                        let _e101 = bitcast<f32>(_e100);
                                        let _e102 = (_e53 + 9u);
                                        if (_e102 < _e56) {
                                            let _e106 = global.member[_e102];
                                            let _e107 = bitcast<f32>(_e106);
                                            if ((_e53 + 10u) < _e56) {
                                                let _e110 = (_e53 + 11u);
                                                if (_e110 < _e56) {
                                                    let _e114 = global.member[_e110];
                                                    let _e115 = bitcast<f32>(_e114);
                                                    let _e116 = (_e53 + 12u);
                                                    if (_e116 < _e56) {
                                                        let _e120 = global.member[_e116];
                                                        let _e122 = (_e53 + 13u);
                                                        if (_e122 < _e56) {
                                                            let _e126 = global.member[_e122];
                                                            if ((_e53 + 14u) < _e56) {
                                                                let _e130 = (_e53 + 15u);
                                                                if (_e130 < _e56) {
                                                                    let _e134 = global.member[_e130];
                                                                    let _e136 = (_e53 + 16u);
                                                                    if (_e136 < _e56) {
                                                                        let _e140 = global.member[_e136];
                                                                        let _e141 = bitcast<f32>(_e140);
                                                                        let _e142 = (_e53 + 17u);
                                                                        if (_e142 < _e56) {
                                                                            let _e146 = global.member[_e142];
                                                                            let _e147 = bitcast<f32>(_e146);
                                                                            let _e148 = (_e53 + 18u);
                                                                            if (_e148 < _e56) {
                                                                                let _e152 = global.member[_e148];
                                                                                let _e153 = bitcast<f32>(_e152);
                                                                                if ((_e53 + 19u) < _e56) {
                                                                                    let _e156 = (_e53 + 20u);
                                                                                    if (_e156 < _e56) {
                                                                                        let _e160 = global.member[_e156];
                                                                                        let _e161 = bitcast<f32>(_e160);
                                                                                        let _e162 = (_e53 + 21u);
                                                                                        if (_e162 < _e56) {
                                                                                            let _e166 = global.member[_e162];
                                                                                            let _e167 = bitcast<f32>(_e166);
                                                                                            let _e168 = (_e53 + 22u);
                                                                                            if (_e168 < _e56) {
                                                                                                let _e172 = global.member[_e168];
                                                                                                let _e173 = bitcast<f32>(_e172);
                                                                                                if ((_e53 + 23u) < _e56) {
                                                                                                    let _e176 = (_e53 + 24u);
                                                                                                    if (_e176 < _e56) {
                                                                                                        let _e180 = global.member[_e176];
                                                                                                        let _e181 = bitcast<f32>(_e180);
                                                                                                        let _e182 = (_e53 + 25u);
                                                                                                        if (_e182 < _e56) {
                                                                                                            let _e186 = global.member[_e182];
                                                                                                            let _e187 = bitcast<f32>(_e186);
                                                                                                            let _e188 = (_e53 + 26u);
                                                                                                            if (_e188 < _e56) {
                                                                                                                let _e192 = global.member[_e188];
                                                                                                                let _e193 = bitcast<f32>(_e192);
                                                                                                                if ((_e53 + 27u) < _e56) {
                                                                                                                    if ((_e53 + 28u) < _e56) {
                                                                                                                        if ((_e53 + 29u) < _e56) {
                                                                                                                            if ((_e53 + 30u) < _e56) {
                                                                                                                                if ((_e53 + 31u) < _e56) {
                                                                                                                                    if ((_e53 + 32u) < _e56) {
                                                                                                                                        if ((_e53 + 33u) < _e56) {
                                                                                                                                            if ((_e53 + 34u) < _e56) {
                                                                                                                                                local = array<vec3<f32>, 36>(vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f));
                                                                                                                                                if (_e54 < 36u) {
                                                                                                                                                    let _e212 = local[_e54];
                                                                                                                                                    global_4 = _e212;
                                                                                                                                                    let _e254 = (fma(fma(_e115, _e193, fma(_e75, _e181, (_e95 * _e187))), _e212.z, fma(fma(_e115, _e153, fma(_e75, _e141, (_e95 * _e147))), _e212.x, (fma(_e115, _e173, fma(_e75, _e161, (_e95 * _e167))) * _e212.y))) + bitcast<f32>(_e134));
                                                                                                                                                    global_2 = vec4<f32>((fma(fma(_e101, _e193, fma(_e61, _e181, (_e81 * _e187))), _e212.z, fma(fma(_e101, _e153, fma(_e61, _e141, (_e81 * _e147))), _e212.x, (fma(_e101, _e173, fma(_e61, _e161, (_e81 * _e167))) * _e212.y))) + bitcast<f32>(_e120)), (fma(fma(_e107, _e193, fma(_e67, _e181, (_e87 * _e187))), _e212.z, fma(fma(_e107, _e153, fma(_e67, _e141, (_e87 * _e147))), _e212.x, (fma(_e107, _e173, fma(_e67, _e161, (_e87 * _e167))) * _e212.y))) + bitcast<f32>(_e126)), _e254, _e254);
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}

@vertex 
fn skyboxvertex(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_3 = param;
    global_1 = param_1;
    function();
    let _e7 = global_2.y;
    global_2.y = -(_e7);
    let _e9 = global_4;
    let _e10 = global_2;
    return VertexOutput(_e9, _e10);
}
