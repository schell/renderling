struct type_2 {
    member: array<u32>,
}

struct type_7 {
    member: u32,
    member_1: u32,
}

var<private> global: vec3<u32>;
@group(0) @binding(0) 
var<storage, read_write> global_1: type_2;

fn function() {
    var phi_504_: bool;
    var phi_221_: u32;
    var phi_520_: bool;
    var phi_239_: type_7;
    var phi_546_: u32;
    var phi_565_: bool;
    var phi_258_: f32;
    var phi_276_: u32;
    var phi_587_: bool;
    var phi_294_: type_7;
    var phi_613_: u32;
    var phi_632_: bool;
    var phi_313_: f32;
    var phi_331_: u32;
    var phi_654_: bool;
    var phi_349_: type_7;
    var phi_680_: u32;
    var phi_699_: bool;
    var phi_368_: f32;
    var phi_386_: u32;
    var phi_721_: bool;
    var phi_404_: type_7;
    var phi_747_: u32;
    var phi_766_: bool;
    var phi_423_: f32;
    var phi_443_: u32;
    var phi_788_: bool;
    var phi_461_: type_7;
    var phi_814_: u32;

    let _e14 = arrayLength((&global_1.member));
    let _e15 = global;
    let _e18 = global_1.member[0u];
    let _e21 = global_1.member[1u];
    let _e24 = global_1.member[2u];
    let _e27 = global_1.member[3u];
    let _e30 = global_1.member[4u];
    let _e31 = (_e24 & 31u);
    let _e33 = (_e18 >> bitcast<u32>(_e31));
    if (_e15.x < _e33) {
        phi_504_ = (_e15.y < (_e21 >> bitcast<u32>(_e31)));
    } else {
        phi_504_ = false;
    }
    let _e41 = phi_504_;
    if (_e41 != true) {
    } else {
        let _e44 = (_e15.x * 2u);
        let _e45 = (_e15.y * 2u);
        let _e46 = (_e24 - 1u);
        let _e47 = (_e46 >= _e30);
        if _e47 {
            phi_221_ = 4294967295u;
        } else {
            phi_221_ = (_e27 + (2u * _e46));
        }
        let _e51 = phi_221_;
        let _e52 = (_e14 >= 2u);
        if _e52 {
            phi_520_ = (_e51 <= (_e14 - 2u));
        } else {
            phi_520_ = false;
        }
        let _e56 = phi_520_;
        if _e56 {
            let _e59 = global_1.member[_e51];
            let _e63 = global_1.member[(_e51 + 1u)];
            phi_239_ = type_7(_e59, _e63);
        } else {
            phi_239_ = type_7(4294967295u, 0u);
        }
        let _e66 = phi_239_;
        let _e71 = (_e18 >> bitcast<u32>((_e46 & 31u)));
        let _e72 = (_e45 * _e71);
        let _e73 = (_e72 + _e44);
        if (_e73 >= _e66.member_1) {
            phi_546_ = 4294967295u;
        } else {
            phi_546_ = (_e66.member + _e73);
        }
        let _e77 = phi_546_;
        let _e78 = (_e14 >= 1u);
        if _e78 {
            phi_565_ = (_e77 <= (_e14 - 1u));
        } else {
            phi_565_ = false;
        }
        let _e82 = phi_565_;
        if _e82 {
            let _e85 = global_1.member[_e77];
            phi_258_ = bitcast<f32>(_e85);
        } else {
            phi_258_ = 0f;
        }
        let _e88 = phi_258_;
        let _e89 = (_e44 + 1u);
        if _e47 {
            phi_276_ = 4294967295u;
        } else {
            phi_276_ = (_e27 + (2u * _e46));
        }
        let _e93 = phi_276_;
        if _e52 {
            phi_587_ = (_e93 <= (_e14 - 2u));
        } else {
            phi_587_ = false;
        }
        let _e97 = phi_587_;
        if _e97 {
            let _e100 = global_1.member[_e93];
            let _e104 = global_1.member[(_e93 + 1u)];
            phi_294_ = type_7(_e100, _e104);
        } else {
            phi_294_ = type_7(4294967295u, 0u);
        }
        let _e107 = phi_294_;
        let _e110 = (_e72 + _e89);
        if (_e110 >= _e107.member_1) {
            phi_613_ = 4294967295u;
        } else {
            phi_613_ = (_e107.member + _e110);
        }
        let _e114 = phi_613_;
        if _e78 {
            phi_632_ = (_e114 <= (_e14 - 1u));
        } else {
            phi_632_ = false;
        }
        let _e118 = phi_632_;
        if _e118 {
            let _e121 = global_1.member[_e114];
            phi_313_ = bitcast<f32>(_e121);
        } else {
            phi_313_ = 0f;
        }
        let _e124 = phi_313_;
        if _e47 {
            phi_331_ = 4294967295u;
        } else {
            phi_331_ = (_e27 + (2u * _e46));
        }
        let _e129 = phi_331_;
        if _e52 {
            phi_654_ = (_e129 <= (_e14 - 2u));
        } else {
            phi_654_ = false;
        }
        let _e133 = phi_654_;
        if _e133 {
            let _e136 = global_1.member[_e129];
            let _e140 = global_1.member[(_e129 + 1u)];
            phi_349_ = type_7(_e136, _e140);
        } else {
            phi_349_ = type_7(4294967295u, 0u);
        }
        let _e143 = phi_349_;
        let _e146 = ((_e45 + 1u) * _e71);
        let _e147 = (_e146 + _e44);
        if (_e147 >= _e143.member_1) {
            phi_680_ = 4294967295u;
        } else {
            phi_680_ = (_e143.member + _e147);
        }
        let _e151 = phi_680_;
        if _e78 {
            phi_699_ = (_e151 <= (_e14 - 1u));
        } else {
            phi_699_ = false;
        }
        let _e155 = phi_699_;
        if _e155 {
            let _e158 = global_1.member[_e151];
            phi_368_ = bitcast<f32>(_e158);
        } else {
            phi_368_ = 0f;
        }
        let _e161 = phi_368_;
        if _e47 {
            phi_386_ = 4294967295u;
        } else {
            phi_386_ = (_e27 + (2u * _e46));
        }
        let _e165 = phi_386_;
        if _e52 {
            phi_721_ = (_e165 <= (_e14 - 2u));
        } else {
            phi_721_ = false;
        }
        let _e169 = phi_721_;
        if _e169 {
            let _e172 = global_1.member[_e165];
            let _e176 = global_1.member[(_e165 + 1u)];
            phi_404_ = type_7(_e172, _e176);
        } else {
            phi_404_ = type_7(4294967295u, 0u);
        }
        let _e179 = phi_404_;
        let _e182 = (_e146 + _e89);
        if (_e182 >= _e179.member_1) {
            phi_747_ = 4294967295u;
        } else {
            phi_747_ = (_e179.member + _e182);
        }
        let _e186 = phi_747_;
        if _e78 {
            phi_766_ = (_e186 <= (_e14 - 1u));
        } else {
            phi_766_ = false;
        }
        let _e190 = phi_766_;
        if _e190 {
            let _e193 = global_1.member[_e186];
            phi_423_ = bitcast<f32>(_e193);
        } else {
            phi_423_ = 0f;
        }
        let _e196 = phi_423_;
        if (_e24 >= _e30) {
            phi_443_ = 4294967295u;
        } else {
            phi_443_ = (_e27 + (2u * _e24));
        }
        let _e204 = phi_443_;
        if _e52 {
            phi_788_ = (_e204 <= (_e14 - 2u));
        } else {
            phi_788_ = false;
        }
        let _e208 = phi_788_;
        if _e208 {
            let _e211 = global_1.member[_e204];
            let _e215 = global_1.member[(_e204 + 1u)];
            phi_461_ = type_7(_e211, _e215);
        } else {
            phi_461_ = type_7(4294967295u, 0u);
        }
        let _e218 = phi_461_;
        let _e222 = ((_e15.y * _e33) + _e15.x);
        if (_e222 >= _e218.member_1) {
            phi_814_ = 4294967295u;
        } else {
            phi_814_ = (_e218.member + _e222);
        }
        let _e226 = phi_814_;
        global_1.member[_e226] = bitcast<u32>(max(max(max(_e88, _e124), _e161), _e196));
    }
    return;
}

@compute @workgroup_size(32, 32, 1) 
fn cullcompute_downsample_depth_pyramid(@builtin(global_invocation_id) param: vec3<u32>) {
    global = param;
    function();
}
