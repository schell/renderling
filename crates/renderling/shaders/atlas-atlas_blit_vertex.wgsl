struct type_3 {
    member: array<u32>,
}

struct VertexOutput {
    @location(0) member: vec2<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
var<private> global_1: u32;
@group(0) @binding(0) 
var<storage> global_2: type_3;
var<private> global_3: vec2<f32>;
var<private> global_4: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);

fn function() {
    var local: array<vec2<f32>, 6>;
    var local_1: array<vec4<f32>, 6>;
    var phi_120_: u32;
    var phi_129_: u32;
    var phi_406_: f32;
    var phi_388_: bool;
    var phi_210_: f32;
    var phi_205_: f32;
    var phi_211_: f32;
    var phi_371_: bool;
    var phi_176_: f32;
    var phi_213_: f32;
    var phi_454_: f32;
    var phi_436_: bool;
    var phi_266_: f32;
    var phi_261_: f32;
    var phi_267_: f32;
    var phi_419_: bool;
    var phi_232_: f32;
    var phi_269_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e36 = global;
            let _e37 = global_1;
            local = array<vec2<f32>, 6>(vec2<f32>(0f, 1f), vec2<f32>(1f, 1f), vec2<f32>(1f, 0f), vec2<f32>(1f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 1f));
            let _e38 = (_e36 < 6u);
            if _e38 {
            } else {
                break;
            }
            let _e40 = local[_e36];
            global_3 = _e40;
            let _e43 = global_2.member[_e37];
            let _e47 = global_2.member[(_e37 + 1u)];
            let _e50 = global_2.member[_e43];
            let _e54 = global_2.member[(_e43 + 1u)];
            let _e58 = global_2.member[(_e43 + 2u)];
            let _e62 = global_2.member[(_e43 + 3u)];
            let _e66 = global_2.member[(_e43 + 6u)];
            switch bitcast<i32>(_e66) {
                case 0: {
                    phi_120_ = 0u;
                    break;
                }
                case 1: {
                    phi_120_ = 1u;
                    break;
                }
                case 2: {
                    phi_120_ = 2u;
                    break;
                }
                default: {
                    phi_120_ = 0u;
                    break;
                }
            }
            let _e69 = phi_120_;
            let _e73 = global_2.member[(_e43 + 7u)];
            switch bitcast<i32>(_e73) {
                case 0: {
                    phi_129_ = 0u;
                    break;
                }
                case 1: {
                    phi_129_ = 1u;
                    break;
                }
                case 2: {
                    phi_129_ = 2u;
                    break;
                }
                default: {
                    phi_129_ = 0u;
                    break;
                }
            }
            let _e76 = phi_129_;
            let _e79 = global_2.member[_e47];
            let _e83 = global_2.member[(_e47 + 1u)];
            local_1 = array<vec4<f32>, 6>(vec4<f32>(-1f, -1f, 0.5f, 1f), vec4<f32>(1f, -1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(1f, 1f, 0.5f, 1f), vec4<f32>(-1f, 1f, 0.5f, 1f), vec4<f32>(-1f, -1f, 0.5f, 1f));
            if _e38 {
            } else {
                break;
            }
            let _e85 = local_1[_e36];
            let _e90 = ((_e85.x + 1f) * 0.5f);
            let _e91 = (fma(_e85.y, -1f, 1f) * 0.5f);
            switch bitcast<i32>(_e69) {
                case 1: {
                    let _e126 = abs(_e90);
                    let _e128 = (_e126 % 1f);
                    if (_e126 >= 1f) {
                        phi_371_ = select(true, false, (_e128 == 0f));
                    } else {
                        phi_371_ = true;
                    }
                    let _e132 = phi_371_;
                    let _e133 = select(1f, _e128, _e132);
                    if (select(-1f, 1f, (_e90 >= 0f)) > 0f) {
                        phi_176_ = _e133;
                    } else {
                        phi_176_ = (1f - _e133);
                    }
                    let _e137 = phi_176_;
                    phi_213_ = _e137;
                    break;
                }
                case 2: {
                    let _e100 = abs(_e90);
                    let _e107 = ((select(select(u32(_e100), 0u, (_e100 < 0f)), 4294967295u, (_e100 > 4294967000f)) % 2u) == 0u);
                    let _e109 = (_e100 % 1f);
                    if (_e100 >= 1f) {
                        phi_388_ = select(true, false, (_e109 == 0f));
                    } else {
                        phi_388_ = true;
                    }
                    let _e113 = phi_388_;
                    let _e114 = select(1f, _e109, _e113);
                    if (select(-1f, 1f, (_e90 >= 0f)) > 0f) {
                        if _e107 {
                            phi_205_ = _e114;
                        } else {
                            phi_205_ = (1f - _e114);
                        }
                        let _e121 = phi_205_;
                        phi_211_ = _e121;
                    } else {
                        if _e107 {
                            phi_210_ = (1f - _e114);
                        } else {
                            phi_210_ = _e114;
                        }
                        let _e118 = phi_210_;
                        phi_211_ = _e118;
                    }
                    let _e123 = phi_211_;
                    phi_213_ = _e123;
                    break;
                }
                case 0: {
                    if (_e90 > 1f) {
                        phi_406_ = 0.9999999f;
                    } else {
                        phi_406_ = select(_e90, 0.00000011920929f, (_e90 < 0f));
                    }
                    let _e97 = phi_406_;
                    phi_213_ = _e97;
                    break;
                }
                default: {
                    phi_213_ = f32();
                    break;
                }
            }
            let _e139 = phi_213_;
            switch bitcast<i32>(_e76) {
                case 1: {
                    let _e174 = abs(_e91);
                    let _e176 = (_e174 % 1f);
                    if (_e174 >= 1f) {
                        phi_419_ = select(true, false, (_e176 == 0f));
                    } else {
                        phi_419_ = true;
                    }
                    let _e180 = phi_419_;
                    let _e181 = select(1f, _e176, _e180);
                    if (select(-1f, 1f, (_e91 >= 0f)) > 0f) {
                        phi_232_ = _e181;
                    } else {
                        phi_232_ = (1f - _e181);
                    }
                    let _e185 = phi_232_;
                    phi_269_ = _e185;
                    break;
                }
                case 2: {
                    let _e148 = abs(_e91);
                    let _e155 = ((select(select(u32(_e148), 0u, (_e148 < 0f)), 4294967295u, (_e148 > 4294967000f)) % 2u) == 0u);
                    let _e157 = (_e148 % 1f);
                    if (_e148 >= 1f) {
                        phi_436_ = select(true, false, (_e157 == 0f));
                    } else {
                        phi_436_ = true;
                    }
                    let _e161 = phi_436_;
                    let _e162 = select(1f, _e157, _e161);
                    if (select(-1f, 1f, (_e91 >= 0f)) > 0f) {
                        if _e155 {
                            phi_261_ = _e162;
                        } else {
                            phi_261_ = (1f - _e162);
                        }
                        let _e169 = phi_261_;
                        phi_267_ = _e169;
                    } else {
                        if _e155 {
                            phi_266_ = (1f - _e162);
                        } else {
                            phi_266_ = _e162;
                        }
                        let _e166 = phi_266_;
                        phi_267_ = _e166;
                    }
                    let _e171 = phi_267_;
                    phi_269_ = _e171;
                    break;
                }
                case 0: {
                    if (_e91 > 1f) {
                        phi_454_ = 0.9999999f;
                    } else {
                        phi_454_ = select(_e91, 0.00000011920929f, (_e91 < 0f));
                    }
                    let _e145 = phi_454_;
                    phi_269_ = _e145;
                    break;
                }
                default: {
                    phi_269_ = f32();
                    break;
                }
            }
            let _e187 = phi_269_;
            let _e189 = (_e139 * f32(_e58));
            let _e196 = (_e187 * f32(_e62));
            global_4 = vec4<f32>(fma((f32((select(select(u32(_e189), 0u, (_e189 < 0f)), 4294967295u, (_e189 > 4294967000f)) + _e50)) / f32(_e79)), 2f, -1f), (fma((f32((select(select(u32(_e196), 0u, (_e196 < 0f)), 4294967295u, (_e196 > 4294967000f)) + _e54)) / f32(_e83)), 2f, -1f) * -1f), _e85.z, _e85.w);
            break;
        }
    }
    return;
}

@vertex 
fn atlasatlas_blit_vertex(@builtin(vertex_index) param: u32, @builtin(instance_index) param_1: u32) -> VertexOutput {
    global = param;
    global_1 = param_1;
    function();
    let _e7 = global_4.y;
    global_4.y = -(_e7);
    let _e9 = global_3;
    let _e10 = global_4;
    return VertexOutput(_e9, _e10);
}
