struct type_3 {
    member: array<u32>,
}

struct VertexOutput {
    @location(0) member: vec3<f32>,
    @builtin(position) member_1: vec4<f32>,
}

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_2: u32;
var<private> global_3: u32;
var<private> global_4: vec3<f32>;

fn function() {
    var local: array<vec3<f32>, 36>;

    let _e53 = global_3;
    let _e54 = global_2;
    let _e56 = arrayLength((&global.member));
    if (_e53 < _e56) {
        let _e60 = global.member[_e53];
        let _e61 = bitcast<f32>(_e60);
        let _e62 = (_e53 + 1u);
        if (_e62 < _e56) {
            let _e66 = global.member[_e62];
            let _e67 = bitcast<f32>(_e66);
            let _e68 = (_e53 + 2u);
            if (_e68 < _e56) {
                let _e72 = global.member[_e68];
                let _e73 = bitcast<f32>(_e72);
                let _e74 = (_e53 + 3u);
                if (_e74 < _e56) {
                    let _e78 = global.member[_e74];
                    let _e79 = bitcast<f32>(_e78);
                    let _e80 = (_e53 + 4u);
                    if (_e80 < _e56) {
                        let _e84 = global.member[_e80];
                        let _e85 = bitcast<f32>(_e84);
                        let _e86 = (_e53 + 5u);
                        if (_e86 < _e56) {
                            let _e90 = global.member[_e86];
                            let _e91 = bitcast<f32>(_e90);
                            let _e92 = (_e53 + 6u);
                            if (_e92 < _e56) {
                                let _e96 = global.member[_e92];
                                let _e97 = bitcast<f32>(_e96);
                                let _e98 = (_e53 + 7u);
                                if (_e98 < _e56) {
                                    let _e102 = global.member[_e98];
                                    let _e103 = bitcast<f32>(_e102);
                                    let _e104 = (_e53 + 8u);
                                    if (_e104 < _e56) {
                                        let _e108 = global.member[_e104];
                                        let _e109 = bitcast<f32>(_e108);
                                        let _e110 = (_e53 + 9u);
                                        if (_e110 < _e56) {
                                            let _e114 = global.member[_e110];
                                            let _e115 = bitcast<f32>(_e114);
                                            let _e116 = (_e53 + 10u);
                                            if (_e116 < _e56) {
                                                let _e120 = global.member[_e116];
                                                let _e121 = bitcast<f32>(_e120);
                                                let _e122 = (_e53 + 11u);
                                                if (_e122 < _e56) {
                                                    let _e126 = global.member[_e122];
                                                    let _e127 = bitcast<f32>(_e126);
                                                    let _e128 = (_e53 + 12u);
                                                    if (_e128 < _e56) {
                                                        let _e132 = global.member[_e128];
                                                        let _e133 = bitcast<f32>(_e132);
                                                        let _e134 = (_e53 + 13u);
                                                        if (_e134 < _e56) {
                                                            let _e138 = global.member[_e134];
                                                            let _e139 = bitcast<f32>(_e138);
                                                            let _e140 = (_e53 + 14u);
                                                            if (_e140 < _e56) {
                                                                let _e144 = global.member[_e140];
                                                                let _e145 = bitcast<f32>(_e144);
                                                                let _e146 = (_e53 + 15u);
                                                                if (_e146 < _e56) {
                                                                    let _e150 = global.member[_e146];
                                                                    let _e151 = bitcast<f32>(_e150);
                                                                    let _e152 = (_e53 + 16u);
                                                                    if (_e152 < _e56) {
                                                                        let _e156 = global.member[_e152];
                                                                        let _e157 = bitcast<f32>(_e156);
                                                                        let _e158 = (_e53 + 17u);
                                                                        if (_e158 < _e56) {
                                                                            let _e162 = global.member[_e158];
                                                                            let _e163 = bitcast<f32>(_e162);
                                                                            let _e164 = (_e53 + 18u);
                                                                            if (_e164 < _e56) {
                                                                                let _e168 = global.member[_e164];
                                                                                let _e169 = bitcast<f32>(_e168);
                                                                                let _e170 = (_e53 + 19u);
                                                                                if (_e170 < _e56) {
                                                                                    let _e174 = global.member[_e170];
                                                                                    let _e175 = bitcast<f32>(_e174);
                                                                                    let _e176 = (_e53 + 20u);
                                                                                    if (_e176 < _e56) {
                                                                                        let _e180 = global.member[_e176];
                                                                                        let _e181 = bitcast<f32>(_e180);
                                                                                        let _e182 = (_e53 + 21u);
                                                                                        if (_e182 < _e56) {
                                                                                            let _e186 = global.member[_e182];
                                                                                            let _e187 = bitcast<f32>(_e186);
                                                                                            let _e188 = (_e53 + 22u);
                                                                                            if (_e188 < _e56) {
                                                                                                let _e192 = global.member[_e188];
                                                                                                let _e193 = bitcast<f32>(_e192);
                                                                                                let _e194 = (_e53 + 23u);
                                                                                                if (_e194 < _e56) {
                                                                                                    let _e198 = global.member[_e194];
                                                                                                    let _e199 = bitcast<f32>(_e198);
                                                                                                    let _e200 = (_e53 + 24u);
                                                                                                    if (_e200 < _e56) {
                                                                                                        let _e204 = global.member[_e200];
                                                                                                        let _e205 = bitcast<f32>(_e204);
                                                                                                        let _e206 = (_e53 + 25u);
                                                                                                        if (_e206 < _e56) {
                                                                                                            let _e210 = global.member[_e206];
                                                                                                            let _e211 = bitcast<f32>(_e210);
                                                                                                            let _e212 = (_e53 + 26u);
                                                                                                            if (_e212 < _e56) {
                                                                                                                let _e216 = global.member[_e212];
                                                                                                                let _e217 = bitcast<f32>(_e216);
                                                                                                                let _e218 = (_e53 + 27u);
                                                                                                                if (_e218 < _e56) {
                                                                                                                    let _e222 = global.member[_e218];
                                                                                                                    let _e223 = bitcast<f32>(_e222);
                                                                                                                    let _e224 = (_e53 + 28u);
                                                                                                                    if (_e224 < _e56) {
                                                                                                                        let _e228 = global.member[_e224];
                                                                                                                        let _e229 = bitcast<f32>(_e228);
                                                                                                                        let _e230 = (_e53 + 29u);
                                                                                                                        if (_e230 < _e56) {
                                                                                                                            let _e234 = global.member[_e230];
                                                                                                                            let _e235 = bitcast<f32>(_e234);
                                                                                                                            let _e236 = (_e53 + 30u);
                                                                                                                            if (_e236 < _e56) {
                                                                                                                                let _e240 = global.member[_e236];
                                                                                                                                let _e241 = bitcast<f32>(_e240);
                                                                                                                                let _e242 = (_e53 + 31u);
                                                                                                                                if (_e242 < _e56) {
                                                                                                                                    let _e246 = global.member[_e242];
                                                                                                                                    let _e247 = bitcast<f32>(_e246);
                                                                                                                                    if ((_e53 + 32u) < _e56) {
                                                                                                                                        if ((_e53 + 33u) < _e56) {
                                                                                                                                            if ((_e53 + 34u) < _e56) {
                                                                                                                                                local = array<vec3<f32>, 36>(vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f));
                                                                                                                                                if (_e54 < 36u) {
                                                                                                                                                    let _e256 = local[_e54];
                                                                                                                                                    global_4 = _e256;
                                                                                                                                                    global_1 = vec4<f32>((fma(fma(_e133, _e223, fma(_e109, _e217, fma(_e61, _e205, (_e85 * _e211)))), _e256.z, fma(fma(_e133, _e175, fma(_e109, _e169, fma(_e61, _e157, (_e85 * _e163)))), _e256.x, (fma(_e133, _e199, fma(_e109, _e193, fma(_e61, _e181, (_e85 * _e187)))) * _e256.y))) + fma(_e133, _e247, fma(_e109, _e241, fma(_e61, _e229, (_e85 * _e235))))), (fma(fma(_e139, _e223, fma(_e115, _e217, fma(_e67, _e205, (_e91 * _e211)))), _e256.z, fma(fma(_e139, _e175, fma(_e115, _e169, fma(_e67, _e157, (_e91 * _e163)))), _e256.x, (fma(_e139, _e199, fma(_e115, _e193, fma(_e67, _e181, (_e91 * _e187)))) * _e256.y))) + fma(_e139, _e247, fma(_e115, _e241, fma(_e67, _e229, (_e91 * _e235))))), (fma(fma(_e145, _e223, fma(_e121, _e217, fma(_e73, _e205, (_e97 * _e211)))), _e256.z, fma(fma(_e145, _e175, fma(_e121, _e169, fma(_e73, _e157, (_e97 * _e163)))), _e256.x, (fma(_e145, _e199, fma(_e121, _e193, fma(_e73, _e181, (_e97 * _e187)))) * _e256.y))) + fma(_e145, _e247, fma(_e121, _e241, fma(_e73, _e229, (_e97 * _e235))))), (fma(fma(_e151, _e223, fma(_e127, _e217, fma(_e79, _e205, (_e103 * _e211)))), _e256.z, fma(fma(_e151, _e175, fma(_e127, _e169, fma(_e79, _e157, (_e103 * _e163)))), _e256.x, (fma(_e151, _e199, fma(_e127, _e193, fma(_e79, _e181, (_e103 * _e187)))) * _e256.y))) + fma(_e151, _e247, fma(_e127, _e241, fma(_e79, _e229, (_e103 * _e235))))));
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
fn skyboxvertex_cubemap(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_3 = param;
    global_2 = param_1;
    function();
    let _e7 = global_1.y;
    global_1.y = -(_e7);
    let _e9 = global_4;
    let _e10 = global_1;
    return VertexOutput(_e9, _e10);
}
