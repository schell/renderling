struct type_3 {
    member: array<u32>,
}

struct VertexOutput {
    @location(0) member: vec3<f32>,
    @location(1) member_1: f32,
    @builtin(position) member_2: vec4<f32>,
}

var<private> global: u32;
var<private> global_1: u32;
@group(0) @binding(0) 
var<storage> global_2: type_3;
var<private> global_3: f32;
var<private> global_4: vec3<f32>;
var<private> global_5: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    var local: array<vec3<f32>, 36>;

    let _e54 = global;
    let _e55 = global_1;
    let _e57 = arrayLength((&global_2.member));
    local = array<vec3<f32>, 36>(vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, 0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, 0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, 0.5f, -0.5f), vec3<f32>(0.5f, -0.5f, -0.5f), vec3<f32>(-0.5f, -0.5f, -0.5f));
    if (_e55 < 36u) {
        let _e60 = local[_e55];
        if (_e54 < _e57) {
            let _e64 = global_2.member[_e54];
            let _e65 = (_e54 + 1u);
            if (_e65 < _e57) {
                let _e69 = global_2.member[_e65];
                if (_e64 < _e57) {
                    let _e73 = global_2.member[_e64];
                    let _e74 = bitcast<f32>(_e73);
                    let _e75 = (_e64 + 1u);
                    if (_e75 < _e57) {
                        let _e79 = global_2.member[_e75];
                        let _e80 = bitcast<f32>(_e79);
                        let _e81 = (_e64 + 2u);
                        if (_e81 < _e57) {
                            let _e85 = global_2.member[_e81];
                            let _e86 = bitcast<f32>(_e85);
                            let _e87 = (_e64 + 3u);
                            if (_e87 < _e57) {
                                let _e91 = global_2.member[_e87];
                                let _e92 = bitcast<f32>(_e91);
                                let _e93 = (_e64 + 4u);
                                if (_e93 < _e57) {
                                    let _e97 = global_2.member[_e93];
                                    let _e98 = bitcast<f32>(_e97);
                                    let _e99 = (_e64 + 5u);
                                    if (_e99 < _e57) {
                                        let _e103 = global_2.member[_e99];
                                        let _e104 = bitcast<f32>(_e103);
                                        let _e105 = (_e64 + 6u);
                                        if (_e105 < _e57) {
                                            let _e109 = global_2.member[_e105];
                                            let _e110 = bitcast<f32>(_e109);
                                            let _e111 = (_e64 + 7u);
                                            if (_e111 < _e57) {
                                                let _e115 = global_2.member[_e111];
                                                let _e116 = bitcast<f32>(_e115);
                                                let _e117 = (_e64 + 8u);
                                                if (_e117 < _e57) {
                                                    let _e121 = global_2.member[_e117];
                                                    let _e122 = bitcast<f32>(_e121);
                                                    let _e123 = (_e64 + 9u);
                                                    if (_e123 < _e57) {
                                                        let _e127 = global_2.member[_e123];
                                                        let _e128 = bitcast<f32>(_e127);
                                                        let _e129 = (_e64 + 10u);
                                                        if (_e129 < _e57) {
                                                            let _e133 = global_2.member[_e129];
                                                            let _e134 = bitcast<f32>(_e133);
                                                            let _e135 = (_e64 + 11u);
                                                            if (_e135 < _e57) {
                                                                let _e139 = global_2.member[_e135];
                                                                let _e140 = bitcast<f32>(_e139);
                                                                let _e141 = (_e64 + 12u);
                                                                if (_e141 < _e57) {
                                                                    let _e145 = global_2.member[_e141];
                                                                    let _e146 = bitcast<f32>(_e145);
                                                                    let _e147 = (_e64 + 13u);
                                                                    if (_e147 < _e57) {
                                                                        let _e151 = global_2.member[_e147];
                                                                        let _e152 = bitcast<f32>(_e151);
                                                                        let _e153 = (_e64 + 14u);
                                                                        if (_e153 < _e57) {
                                                                            let _e157 = global_2.member[_e153];
                                                                            let _e158 = bitcast<f32>(_e157);
                                                                            let _e159 = (_e64 + 15u);
                                                                            if (_e159 < _e57) {
                                                                                let _e163 = global_2.member[_e159];
                                                                                let _e164 = bitcast<f32>(_e163);
                                                                                let _e165 = (_e64 + 16u);
                                                                                if (_e165 < _e57) {
                                                                                    let _e169 = global_2.member[_e165];
                                                                                    let _e170 = bitcast<f32>(_e169);
                                                                                    let _e171 = (_e64 + 17u);
                                                                                    if (_e171 < _e57) {
                                                                                        let _e175 = global_2.member[_e171];
                                                                                        let _e176 = bitcast<f32>(_e175);
                                                                                        let _e177 = (_e64 + 18u);
                                                                                        if (_e177 < _e57) {
                                                                                            let _e181 = global_2.member[_e177];
                                                                                            let _e182 = bitcast<f32>(_e181);
                                                                                            let _e183 = (_e64 + 19u);
                                                                                            if (_e183 < _e57) {
                                                                                                let _e187 = global_2.member[_e183];
                                                                                                let _e188 = bitcast<f32>(_e187);
                                                                                                let _e189 = (_e64 + 20u);
                                                                                                if (_e189 < _e57) {
                                                                                                    let _e193 = global_2.member[_e189];
                                                                                                    let _e194 = bitcast<f32>(_e193);
                                                                                                    let _e195 = (_e64 + 21u);
                                                                                                    if (_e195 < _e57) {
                                                                                                        let _e199 = global_2.member[_e195];
                                                                                                        let _e200 = bitcast<f32>(_e199);
                                                                                                        let _e201 = (_e64 + 22u);
                                                                                                        if (_e201 < _e57) {
                                                                                                            let _e205 = global_2.member[_e201];
                                                                                                            let _e206 = bitcast<f32>(_e205);
                                                                                                            let _e207 = (_e64 + 23u);
                                                                                                            if (_e207 < _e57) {
                                                                                                                let _e211 = global_2.member[_e207];
                                                                                                                let _e212 = bitcast<f32>(_e211);
                                                                                                                let _e213 = (_e64 + 24u);
                                                                                                                if (_e213 < _e57) {
                                                                                                                    let _e217 = global_2.member[_e213];
                                                                                                                    let _e218 = bitcast<f32>(_e217);
                                                                                                                    let _e219 = (_e64 + 25u);
                                                                                                                    if (_e219 < _e57) {
                                                                                                                        let _e223 = global_2.member[_e219];
                                                                                                                        let _e224 = bitcast<f32>(_e223);
                                                                                                                        let _e225 = (_e64 + 26u);
                                                                                                                        if (_e225 < _e57) {
                                                                                                                            let _e229 = global_2.member[_e225];
                                                                                                                            let _e230 = bitcast<f32>(_e229);
                                                                                                                            let _e231 = (_e64 + 27u);
                                                                                                                            if (_e231 < _e57) {
                                                                                                                                let _e235 = global_2.member[_e231];
                                                                                                                                let _e236 = bitcast<f32>(_e235);
                                                                                                                                let _e237 = (_e64 + 28u);
                                                                                                                                if (_e237 < _e57) {
                                                                                                                                    let _e241 = global_2.member[_e237];
                                                                                                                                    let _e242 = bitcast<f32>(_e241);
                                                                                                                                    let _e243 = (_e64 + 29u);
                                                                                                                                    if (_e243 < _e57) {
                                                                                                                                        let _e247 = global_2.member[_e243];
                                                                                                                                        let _e248 = bitcast<f32>(_e247);
                                                                                                                                        let _e249 = (_e64 + 30u);
                                                                                                                                        if (_e249 < _e57) {
                                                                                                                                            let _e253 = global_2.member[_e249];
                                                                                                                                            let _e254 = bitcast<f32>(_e253);
                                                                                                                                            let _e255 = (_e64 + 31u);
                                                                                                                                            if (_e255 < _e57) {
                                                                                                                                                let _e259 = global_2.member[_e255];
                                                                                                                                                let _e260 = bitcast<f32>(_e259);
                                                                                                                                                if ((_e64 + 32u) < _e57) {
                                                                                                                                                    if ((_e64 + 33u) < _e57) {
                                                                                                                                                        if ((_e64 + 34u) < _e57) {
                                                                                                                                                            if (_e69 < _e57) {
                                                                                                                                                                let _e270 = global_2.member[_e69];
                                                                                                                                                                global_3 = bitcast<f32>(_e270);
                                                                                                                                                                global_4 = _e60;
                                                                                                                                                                global_5 = vec4<f32>((fma(fma(_e146, _e236, fma(_e122, _e230, fma(_e74, _e218, (_e98 * _e224)))), _e60.z, fma(fma(_e146, _e188, fma(_e122, _e182, fma(_e74, _e170, (_e98 * _e176)))), _e60.x, (fma(_e146, _e212, fma(_e122, _e206, fma(_e74, _e194, (_e98 * _e200)))) * _e60.y))) + fma(_e146, _e260, fma(_e122, _e254, fma(_e74, _e242, (_e98 * _e248))))), (fma(fma(_e152, _e236, fma(_e128, _e230, fma(_e80, _e218, (_e104 * _e224)))), _e60.z, fma(fma(_e152, _e188, fma(_e128, _e182, fma(_e80, _e170, (_e104 * _e176)))), _e60.x, (fma(_e152, _e212, fma(_e128, _e206, fma(_e80, _e194, (_e104 * _e200)))) * _e60.y))) + fma(_e152, _e260, fma(_e128, _e254, fma(_e80, _e242, (_e104 * _e248))))), (fma(fma(_e158, _e236, fma(_e134, _e230, fma(_e86, _e218, (_e110 * _e224)))), _e60.z, fma(fma(_e158, _e188, fma(_e134, _e182, fma(_e86, _e170, (_e110 * _e176)))), _e60.x, (fma(_e158, _e212, fma(_e134, _e206, fma(_e86, _e194, (_e110 * _e200)))) * _e60.y))) + fma(_e158, _e260, fma(_e134, _e254, fma(_e86, _e242, (_e110 * _e248))))), (fma(fma(_e164, _e236, fma(_e140, _e230, fma(_e92, _e218, (_e116 * _e224)))), _e60.z, fma(fma(_e164, _e188, fma(_e140, _e182, fma(_e92, _e170, (_e116 * _e176)))), _e60.x, (fma(_e164, _e212, fma(_e140, _e206, fma(_e92, _e194, (_e116 * _e200)))) * _e60.y))) + fma(_e164, _e260, fma(_e140, _e254, fma(_e92, _e242, (_e116 * _e248))))));
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
            }
        }
    }
    return;
}

@vertex 
fn convolutionvertex_prefilter_environment_cubemap(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global = param;
    global_1 = param_1;
    function();
    let _e8 = global_5.y;
    global_5.y = -(_e8);
    let _e10 = global_4;
    let _e11 = global_3;
    let _e12 = global_5;
    return VertexOutput(_e10, _e11, _e12);
}
