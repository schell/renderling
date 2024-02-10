struct type_3 {
    member: array<u32>,
}

@group(0) @binding(0) 
var<storage> global: type_3;
var<private> global_1: vec4<f32>;
var<private> global_2: u32;
var<private> global_3: vec4<f32>;

fn function() {
    var phi_2249_: vec3<f32>;

    let _e44 = global_1;
    let _e46 = arrayLength((&global.member));
    let _e47 = global_2;
    if (_e47 < _e46) {
        let _e51 = global.member[_e47];
        let _e53 = (_e47 + 1u);
        if (_e53 < _e46) {
            let _e57 = global.member[_e53];
            if ((_e47 + 2u) < _e46) {
                if ((_e47 + 3u) < _e46) {
                    let _e63 = (_e47 + 4u);
                    if (_e63 < _e46) {
                        let _e67 = global.member[_e63];
                        if (_e67 < _e46) {
                            if ((_e67 + 1u) < _e46) {
                                let _e71 = (_e67 + 2u);
                                if (_e71 < _e46) {
                                    let _e75 = global.member[_e71];
                                    if (_e75 < _e46) {
                                        let _e79 = global.member[_e75];
                                        let _e80 = bitcast<f32>(_e79);
                                        let _e81 = (_e75 + 1u);
                                        if (_e81 < _e46) {
                                            let _e85 = global.member[_e81];
                                            let _e86 = bitcast<f32>(_e85);
                                            let _e87 = (_e75 + 2u);
                                            if (_e87 < _e46) {
                                                let _e91 = global.member[_e87];
                                                let _e92 = bitcast<f32>(_e91);
                                                let _e93 = (_e75 + 3u);
                                                if (_e93 < _e46) {
                                                    let _e97 = global.member[_e93];
                                                    let _e98 = bitcast<f32>(_e97);
                                                    let _e99 = (_e75 + 4u);
                                                    if (_e99 < _e46) {
                                                        let _e103 = global.member[_e99];
                                                        let _e104 = bitcast<f32>(_e103);
                                                        let _e105 = (_e75 + 5u);
                                                        if (_e105 < _e46) {
                                                            let _e109 = global.member[_e105];
                                                            let _e110 = bitcast<f32>(_e109);
                                                            let _e111 = (_e75 + 6u);
                                                            if (_e111 < _e46) {
                                                                let _e115 = global.member[_e111];
                                                                let _e116 = bitcast<f32>(_e115);
                                                                let _e117 = (_e75 + 7u);
                                                                if (_e117 < _e46) {
                                                                    let _e121 = global.member[_e117];
                                                                    let _e122 = bitcast<f32>(_e121);
                                                                    let _e123 = (_e75 + 8u);
                                                                    if (_e123 < _e46) {
                                                                        let _e127 = global.member[_e123];
                                                                        let _e128 = bitcast<f32>(_e127);
                                                                        let _e129 = (_e75 + 9u);
                                                                        if (_e129 < _e46) {
                                                                            let _e133 = global.member[_e129];
                                                                            let _e134 = bitcast<f32>(_e133);
                                                                            let _e135 = (_e75 + 10u);
                                                                            if (_e135 < _e46) {
                                                                                let _e139 = global.member[_e135];
                                                                                let _e140 = bitcast<f32>(_e139);
                                                                                let _e141 = (_e75 + 11u);
                                                                                if (_e141 < _e46) {
                                                                                    let _e145 = global.member[_e141];
                                                                                    let _e146 = bitcast<f32>(_e145);
                                                                                    let _e147 = (_e75 + 12u);
                                                                                    if (_e147 < _e46) {
                                                                                        let _e151 = global.member[_e147];
                                                                                        let _e152 = bitcast<f32>(_e151);
                                                                                        let _e153 = (_e75 + 13u);
                                                                                        if (_e153 < _e46) {
                                                                                            let _e157 = global.member[_e153];
                                                                                            let _e158 = bitcast<f32>(_e157);
                                                                                            let _e159 = (_e75 + 14u);
                                                                                            if (_e159 < _e46) {
                                                                                                let _e163 = global.member[_e159];
                                                                                                let _e164 = bitcast<f32>(_e163);
                                                                                                let _e165 = (_e75 + 15u);
                                                                                                if (_e165 < _e46) {
                                                                                                    let _e169 = global.member[_e165];
                                                                                                    let _e170 = bitcast<f32>(_e169);
                                                                                                    let _e171 = (_e75 + 16u);
                                                                                                    if (_e171 < _e46) {
                                                                                                        let _e175 = global.member[_e171];
                                                                                                        let _e176 = bitcast<f32>(_e175);
                                                                                                        let _e177 = (_e75 + 17u);
                                                                                                        if (_e177 < _e46) {
                                                                                                            let _e181 = global.member[_e177];
                                                                                                            let _e182 = bitcast<f32>(_e181);
                                                                                                            let _e183 = (_e75 + 18u);
                                                                                                            if (_e183 < _e46) {
                                                                                                                let _e187 = global.member[_e183];
                                                                                                                let _e188 = bitcast<f32>(_e187);
                                                                                                                let _e189 = (_e75 + 19u);
                                                                                                                if (_e189 < _e46) {
                                                                                                                    let _e193 = global.member[_e189];
                                                                                                                    let _e194 = bitcast<f32>(_e193);
                                                                                                                    let _e195 = (_e75 + 20u);
                                                                                                                    if (_e195 < _e46) {
                                                                                                                        let _e199 = global.member[_e195];
                                                                                                                        let _e200 = bitcast<f32>(_e199);
                                                                                                                        let _e201 = (_e75 + 21u);
                                                                                                                        if (_e201 < _e46) {
                                                                                                                            let _e205 = global.member[_e201];
                                                                                                                            let _e206 = bitcast<f32>(_e205);
                                                                                                                            let _e207 = (_e75 + 22u);
                                                                                                                            if (_e207 < _e46) {
                                                                                                                                let _e211 = global.member[_e207];
                                                                                                                                let _e212 = bitcast<f32>(_e211);
                                                                                                                                let _e213 = (_e75 + 23u);
                                                                                                                                if (_e213 < _e46) {
                                                                                                                                    let _e217 = global.member[_e213];
                                                                                                                                    let _e218 = bitcast<f32>(_e217);
                                                                                                                                    let _e219 = (_e75 + 24u);
                                                                                                                                    if (_e219 < _e46) {
                                                                                                                                        let _e223 = global.member[_e219];
                                                                                                                                        let _e224 = bitcast<f32>(_e223);
                                                                                                                                        let _e225 = (_e75 + 25u);
                                                                                                                                        if (_e225 < _e46) {
                                                                                                                                            let _e229 = global.member[_e225];
                                                                                                                                            let _e230 = bitcast<f32>(_e229);
                                                                                                                                            let _e231 = (_e75 + 26u);
                                                                                                                                            if (_e231 < _e46) {
                                                                                                                                                let _e235 = global.member[_e231];
                                                                                                                                                let _e236 = bitcast<f32>(_e235);
                                                                                                                                                let _e237 = (_e75 + 27u);
                                                                                                                                                if (_e237 < _e46) {
                                                                                                                                                    let _e241 = global.member[_e237];
                                                                                                                                                    let _e242 = bitcast<f32>(_e241);
                                                                                                                                                    let _e243 = (_e75 + 28u);
                                                                                                                                                    if (_e243 < _e46) {
                                                                                                                                                        let _e247 = global.member[_e243];
                                                                                                                                                        let _e248 = bitcast<f32>(_e247);
                                                                                                                                                        let _e249 = (_e75 + 29u);
                                                                                                                                                        if (_e249 < _e46) {
                                                                                                                                                            let _e253 = global.member[_e249];
                                                                                                                                                            let _e254 = bitcast<f32>(_e253);
                                                                                                                                                            let _e255 = (_e75 + 30u);
                                                                                                                                                            if (_e255 < _e46) {
                                                                                                                                                                let _e259 = global.member[_e255];
                                                                                                                                                                let _e260 = bitcast<f32>(_e259);
                                                                                                                                                                let _e261 = (_e75 + 31u);
                                                                                                                                                                if (_e261 < _e46) {
                                                                                                                                                                    let _e265 = global.member[_e261];
                                                                                                                                                                    let _e266 = bitcast<f32>(_e265);
                                                                                                                                                                    if ((_e75 + 32u) < _e46) {
                                                                                                                                                                        if ((_e75 + 33u) < _e46) {
                                                                                                                                                                            if ((_e75 + 34u) < _e46) {
                                                                                                                                                                                let _e279 = (((2f * _e44.x) / bitcast<f32>(_e51)) - 1f);
                                                                                                                                                                                let _e281 = ((((2f * _e44.y) / bitcast<f32>(_e57)) - 1f) * -1f);
                                                                                                                                                                                let _e294 = fma(_e152, _e194, fma(_e128, _e188, fma(_e80, _e176, (_e104 * _e182))));
                                                                                                                                                                                let _e295 = fma(_e158, _e194, fma(_e134, _e188, fma(_e86, _e176, (_e110 * _e182))));
                                                                                                                                                                                let _e296 = fma(_e164, _e194, fma(_e140, _e188, fma(_e92, _e176, (_e116 * _e182))));
                                                                                                                                                                                let _e297 = fma(_e170, _e194, fma(_e146, _e188, fma(_e98, _e176, (_e122 * _e182))));
                                                                                                                                                                                let _e310 = fma(_e152, _e218, fma(_e128, _e212, fma(_e80, _e200, (_e104 * _e206))));
                                                                                                                                                                                let _e311 = fma(_e158, _e218, fma(_e134, _e212, fma(_e86, _e200, (_e110 * _e206))));
                                                                                                                                                                                let _e312 = fma(_e164, _e218, fma(_e140, _e212, fma(_e92, _e200, (_e116 * _e206))));
                                                                                                                                                                                let _e313 = fma(_e170, _e218, fma(_e146, _e212, fma(_e98, _e200, (_e122 * _e206))));
                                                                                                                                                                                let _e326 = fma(_e152, _e242, fma(_e128, _e236, fma(_e80, _e224, (_e104 * _e230))));
                                                                                                                                                                                let _e327 = fma(_e158, _e242, fma(_e134, _e236, fma(_e86, _e224, (_e110 * _e230))));
                                                                                                                                                                                let _e328 = fma(_e164, _e242, fma(_e140, _e236, fma(_e92, _e224, (_e116 * _e230))));
                                                                                                                                                                                let _e329 = fma(_e170, _e242, fma(_e146, _e236, fma(_e98, _e224, (_e122 * _e230))));
                                                                                                                                                                                let _e342 = fma(_e152, _e266, fma(_e128, _e260, fma(_e80, _e248, (_e104 * _e254))));
                                                                                                                                                                                let _e343 = fma(_e158, _e266, fma(_e134, _e260, fma(_e86, _e248, (_e110 * _e254))));
                                                                                                                                                                                let _e344 = fma(_e164, _e266, fma(_e140, _e260, fma(_e92, _e248, (_e116 * _e254))));
                                                                                                                                                                                let _e345 = fma(_e170, _e266, fma(_e146, _e260, fma(_e98, _e248, (_e122 * _e254))));
                                                                                                                                                                                let _e348 = fma(_e328, _e345, -((_e344 * _e329)));
                                                                                                                                                                                let _e351 = fma(_e312, _e345, -((_e344 * _e313)));
                                                                                                                                                                                let _e354 = fma(_e312, _e329, -((_e328 * _e313)));
                                                                                                                                                                                let _e357 = fma(_e327, _e345, -((_e343 * _e329)));
                                                                                                                                                                                let _e360 = fma(_e311, _e345, -((_e343 * _e313)));
                                                                                                                                                                                let _e363 = fma(_e311, _e329, -((_e327 * _e313)));
                                                                                                                                                                                let _e366 = fma(_e327, _e344, -((_e343 * _e328)));
                                                                                                                                                                                let _e369 = fma(_e311, _e344, -((_e343 * _e312)));
                                                                                                                                                                                let _e372 = fma(_e311, _e328, -((_e327 * _e312)));
                                                                                                                                                                                let _e375 = fma(_e326, _e345, -((_e342 * _e329)));
                                                                                                                                                                                let _e378 = fma(_e310, _e345, -((_e342 * _e313)));
                                                                                                                                                                                let _e381 = fma(_e310, _e329, -((_e326 * _e313)));
                                                                                                                                                                                let _e384 = fma(_e326, _e344, -((_e342 * _e328)));
                                                                                                                                                                                let _e387 = fma(_e310, _e344, -((_e342 * _e312)));
                                                                                                                                                                                let _e390 = fma(_e310, _e328, -((_e326 * _e312)));
                                                                                                                                                                                let _e393 = fma(_e326, _e343, -((_e342 * _e327)));
                                                                                                                                                                                let _e396 = fma(_e310, _e343, -((_e342 * _e311)));
                                                                                                                                                                                let _e399 = fma(_e310, _e327, -((_e326 * _e311)));
                                                                                                                                                                                let _e412 = fma(_e313, _e366, fma(_e311, _e348, -((_e312 * _e357))));
                                                                                                                                                                                let _e444 = fma(_e313, _e393, fma(_e310, _e357, -((_e311 * _e375))));
                                                                                                                                                                                let _e467 = (fma(_e313, _e384, fma(_e310, _e348, -((_e312 * _e375)))) * -1f);
                                                                                                                                                                                let _e473 = (fma(_e312, _e393, fma(_e310, _e366, -((_e311 * _e384)))) * -1f);
                                                                                                                                                                                let _e480 = (1f / fma(_e297, _e473, fma(_e296, _e444, fma(_e294, _e412, (_e295 * _e467)))));
                                                                                                                                                                                let _e481 = (vec4<f32>(_e412, (fma(_e297, _e366, fma(_e295, _e348, -((_e296 * _e357)))) * -1f), fma(_e297, _e369, fma(_e295, _e351, -((_e296 * _e360)))), (fma(_e297, _e372, fma(_e295, _e354, -((_e296 * _e363)))) * -1f)) * _e480);
                                                                                                                                                                                let _e482 = (vec4<f32>(_e467, fma(_e297, _e384, fma(_e294, _e348, -((_e296 * _e375)))), (fma(_e297, _e387, fma(_e294, _e351, -((_e296 * _e378)))) * -1f), fma(_e297, _e390, fma(_e294, _e354, -((_e296 * _e381))))) * _e480);
                                                                                                                                                                                let _e483 = (vec4<f32>(_e444, (fma(_e297, _e393, fma(_e294, _e357, -((_e295 * _e375)))) * -1f), fma(_e297, _e396, fma(_e294, _e360, -((_e295 * _e378)))), (fma(_e297, _e399, fma(_e294, _e363, -((_e295 * _e381)))) * -1f)) * _e480);
                                                                                                                                                                                let _e484 = (vec4<f32>(_e473, fma(_e296, _e393, fma(_e294, _e366, -((_e295 * _e384)))), (fma(_e296, _e396, fma(_e294, _e369, -((_e295 * _e387)))) * -1f), fma(_e296, _e399, fma(_e294, _e372, -((_e295 * _e390))))) * _e480);
                                                                                                                                                                                let _e497 = fma(_e481.x, _e279, (_e482.x * _e281));
                                                                                                                                                                                let _e498 = fma(_e481.y, _e279, (_e482.y * _e281));
                                                                                                                                                                                let _e499 = fma(_e481.z, _e279, (_e482.z * _e281));
                                                                                                                                                                                let _e500 = fma(_e481.w, _e279, (_e482.w * _e281));
                                                                                                                                                                                let _e512 = (_e500 + _e484.w);
                                                                                                                                                                                let _e524 = ((_e500 + _e483.w) + _e484.w);
                                                                                                                                                                                let _e529 = (vec3<f32>((((_e497 + _e483.x) + _e484.x) / _e524), (((_e498 + _e483.y) + _e484.y) / _e524), (((_e499 + _e483.z) + _e484.z) / _e524)) - vec3<f32>(((_e497 + _e484.x) / _e512), ((_e498 + _e484.y) / _e512), ((_e499 + _e484.z) / _e512)));
                                                                                                                                                                                let _e536 = sqrt(fma(_e529.z, _e529.z, fma(_e529.x, _e529.x, (_e529.y * _e529.y))));
                                                                                                                                                                                if (_e536 == 0f) {
                                                                                                                                                                                    phi_2249_ = vec3<f32>(0f, 0f, 0f);
                                                                                                                                                                                } else {
                                                                                                                                                                                    phi_2249_ = (_e529 * (1f / _e536));
                                                                                                                                                                                }
                                                                                                                                                                                let _e541 = phi_2249_;
                                                                                                                                                                                global_3 = vec4<f32>(_e541.x, _e541.y, _e541.z, 1f);
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
                }
            }
        }
    }
    return;
}

@fragment 
fn sdfraymarchraymarch_rays(@builtin(position) param: vec4<f32>, @location(0) @interpolate(flat) param_1: u32) -> @location(0) vec4<f32> {
    global_1 = param;
    global_2 = param_1;
    function();
    let _e5 = global_3;
    return _e5;
}
