struct type_2 {
    member: array<u32>,
}

struct type_10 {
    member: u32,
    member_1: u32,
}

struct type_15 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_17 {
    member: vec2<u32>,
    member_1: type_10,
    member_2: f32,
}

struct type_18 {
    member: type_10,
    member_1: u32,
    member_2: u32,
    member_3: u32,
    member_4: u32,
}

struct type_20 {
    member: vec3<u32>,
    member_1: type_10,
    member_2: u32,
    member_3: u32,
}

struct type_21 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: f32,
}

struct type_22 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
    member_3: f32,
    member_4: vec4<f32>,
    member_5: f32,
}

struct type_25 {
    member: array<atomic<u32>>,
}

var<private> global: vec3<u32>;
@group(0) @binding(0) 
var<storage> global_1: type_2;
@group(0) @binding(1) 
var<storage, read_write> global_2: type_25;
var<private> global_3: vec2<u32> = vec2<u32>(16u, 16u);

fn function() {
    var phi_747_: type_18;
    var phi_2201_: bool;
    var phi_777_: type_17;
    var phi_2244_: bool;
    var phi_816_: u32;
    var phi_891_: u32;
    var phi_2411_: bool;
    var phi_910_: type_10;
    var phi_3515_: bool;
    var phi_1236_: type_20;
    var phi_1279_: u32;
    var phi_1282_: type_10;
    var phi_1300_: u32;
    var phi_2497_: bool;
    var phi_1361_: type_15;
    var phi_2529_: bool;
    var phi_1601_: type_22;
    var phi_2883_: f32;
    var phi_2905_: bool;
    var phi_1449_: type_21;
    var phi_3258_: f32;
    var phi_3280_: bool;
    var phi_1407_: type_21;
    var phi_1692_: f32;
    var phi_1693_: f32;
    var phi_1704_: f32;
    var phi_3316_: u32;
    var phi_3503_: bool;
    var phi_3362_: u32;
    var phi_3387_: u32;
    var phi_1778_: type_10;
    var phi_1781_: u32;
    var phi_1783_: f32;
    var phi_1779_: type_10;
    var phi_1806_: type_10;
    var phi_3413_: u32;
    var phi_1782_: u32;
    var phi_1784_: f32;
    var phi_1825_: bool;
    var local: u32;
    var local_1: bool;
    var phi_3540_: bool;
    var phi_1851_: bool;
    var phi_3539_: bool;
    var phi_1855_: bool;
    var phi_3538_: bool;
    var phi_1859_: bool;
    var phi_3537_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e73 = arrayLength((&global_2.member));
            let _e74 = global;
            if select(false, true, (_e73 >= 6u)) {
                let _e79 = atomicLoad((&global_2.member[0u]));
                let _e82 = atomicLoad((&global_2.member[1u]));
                let _e85 = atomicLoad((&global_2.member[2u]));
                let _e88 = atomicLoad((&global_2.member[3u]));
                let _e91 = atomicLoad((&global_2.member[4u]));
                let _e94 = atomicLoad((&global_2.member[5u]));
                phi_747_ = type_18(type_10(_e79, _e82), _e85, _e88, _e91, _e94);
            } else {
                phi_747_ = type_18(type_10(4294967295u, 0u), 4294967295u, 4294967295u, 0u, 4294967295u);
            }
            let _e98 = phi_747_;
            if (_e73 >= 5u) {
                phi_2201_ = (_e98.member_4 <= (_e73 - 5u));
            } else {
                phi_2201_ = false;
            }
            let _e104 = phi_2201_;
            if _e104 {
                let _e107 = atomicLoad((&global_2.member[_e98.member_4]));
                let _e111 = atomicLoad((&global_2.member[(_e98.member_4 + 1u)]));
                let _e116 = atomicLoad((&global_2.member[(_e98.member_4 + 2u)]));
                let _e120 = atomicLoad((&global_2.member[(_e98.member_4 + 3u)]));
                let _e124 = atomicLoad((&global_2.member[(_e98.member_4 + 4u)]));
                phi_777_ = type_17(vec2<u32>(_e107, _e111), type_10(_e116, _e120), bitcast<f32>(_e124));
            } else {
                phi_777_ = type_17(vec2<u32>(0u, 0u), type_10(4294967295u, 0u), 0.25f);
            }
            let _e129 = phi_777_;
            if (_e74.x < _e129.member.x) {
                phi_2244_ = (_e74.y < _e129.member.y);
            } else {
                phi_2244_ = false;
            }
            let _e142 = phi_2244_;
            if _e142 {
                let _e144 = (_e74.x / 16u);
                let _e145 = (_e74.y / 16u);
                let _e146 = f32(_e129.member.x);
                let _e150 = global_3[0u];
                let _e154 = ceil((_e146 / f32(_e150)));
                let _e161 = ((_e145 * select(select(u32(_e154), 0u, (_e154 < 0f)), 4294967295u, (_e154 > 4294967000f))) + _e144);
                if (_e161 >= _e129.member_1.member_1) {
                    phi_816_ = 4294967295u;
                } else {
                    phi_816_ = (_e129.member_1.member + (9u * _e161));
                }
                let _e166 = phi_816_;
                let _e169 = atomicLoad((&global_2.member[_e166]));
                let _e173 = atomicLoad((&global_2.member[(_e166 + 1u)]));
                let _e175 = (f32(_e169) * 0.00000000023283064f);
                let _e177 = (f32(_e173) * 0.00000000023283064f);
                if (_e175 == _e177) {
                } else {
                    let _e183 = global_3[0u];
                    let _e185 = global_3[1u];
                    let _e189 = ceil((_e146 / f32(_e183)));
                    let _e190 = ceil((f32(_e129.member.y) / f32(_e185)));
                    let _e208 = ((((f32(_e144) + 0.5f) / f32(select(select(u32(_e189), 0u, (_e189 < 0f)), 4294967295u, (_e189 > 4294967000f)))) * 2f) - 1f);
                    let _e209 = (((1f - ((f32(_e145) + 0.5f) / f32(select(select(u32(_e190), 0u, (_e190 < 0f)), 4294967295u, (_e190 > 4294967000f))))) * 2f) - 1f);
                    let _e210 = global_3[0u];
                    let _e213 = ceil((_e146 / f32(_e210)));
                    let _e220 = ((_e145 * select(select(u32(_e213), 0u, (_e213 < 0f)), 4294967295u, (_e213 > 4294967000f))) + _e144);
                    if (_e220 >= _e129.member_1.member_1) {
                        phi_891_ = 4294967295u;
                    } else {
                        phi_891_ = (_e129.member_1.member + (9u * _e220));
                    }
                    let _e225 = phi_891_;
                    let _e226 = (_e225 + 3u);
                    if (_e73 >= 2u) {
                        phi_2411_ = (_e226 <= (_e73 - 2u));
                    } else {
                        phi_2411_ = false;
                    }
                    let _e231 = phi_2411_;
                    if _e231 {
                        let _e234 = atomicLoad((&global_2.member[_e226]));
                        let _e238 = atomicLoad((&global_2.member[(_e225 + 4u)]));
                        phi_910_ = type_10(_e234, _e238);
                    } else {
                        phi_910_ = type_10(4294967295u, 0u);
                    }
                    let _e241 = phi_910_;
                    let _e244 = (_e225 + 2u);
                    let _e247 = global_1.member[0u];
                    let _e250 = global_1.member[_e247];
                    let _e251 = bitcast<f32>(_e250);
                    let _e255 = global_1.member[(_e247 + 1u)];
                    let _e256 = bitcast<f32>(_e255);
                    let _e260 = global_1.member[(_e247 + 2u)];
                    let _e261 = bitcast<f32>(_e260);
                    let _e265 = global_1.member[(_e247 + 3u)];
                    let _e266 = bitcast<f32>(_e265);
                    let _e270 = global_1.member[(_e247 + 4u)];
                    let _e271 = bitcast<f32>(_e270);
                    let _e275 = global_1.member[(_e247 + 5u)];
                    let _e276 = bitcast<f32>(_e275);
                    let _e280 = global_1.member[(_e247 + 6u)];
                    let _e281 = bitcast<f32>(_e280);
                    let _e285 = global_1.member[(_e247 + 7u)];
                    let _e286 = bitcast<f32>(_e285);
                    let _e290 = global_1.member[(_e247 + 8u)];
                    let _e291 = bitcast<f32>(_e290);
                    let _e295 = global_1.member[(_e247 + 9u)];
                    let _e296 = bitcast<f32>(_e295);
                    let _e300 = global_1.member[(_e247 + 10u)];
                    let _e301 = bitcast<f32>(_e300);
                    let _e305 = global_1.member[(_e247 + 11u)];
                    let _e306 = bitcast<f32>(_e305);
                    let _e310 = global_1.member[(_e247 + 12u)];
                    let _e311 = bitcast<f32>(_e310);
                    let _e315 = global_1.member[(_e247 + 13u)];
                    let _e316 = bitcast<f32>(_e315);
                    let _e320 = global_1.member[(_e247 + 14u)];
                    let _e321 = bitcast<f32>(_e320);
                    let _e325 = global_1.member[(_e247 + 15u)];
                    let _e326 = bitcast<f32>(_e325);
                    let _e330 = global_1.member[(_e247 + 16u)];
                    let _e331 = bitcast<f32>(_e330);
                    let _e335 = global_1.member[(_e247 + 17u)];
                    let _e336 = bitcast<f32>(_e335);
                    let _e340 = global_1.member[(_e247 + 18u)];
                    let _e341 = bitcast<f32>(_e340);
                    let _e345 = global_1.member[(_e247 + 19u)];
                    let _e346 = bitcast<f32>(_e345);
                    let _e350 = global_1.member[(_e247 + 20u)];
                    let _e351 = bitcast<f32>(_e350);
                    let _e355 = global_1.member[(_e247 + 21u)];
                    let _e356 = bitcast<f32>(_e355);
                    let _e360 = global_1.member[(_e247 + 22u)];
                    let _e361 = bitcast<f32>(_e360);
                    let _e365 = global_1.member[(_e247 + 23u)];
                    let _e366 = bitcast<f32>(_e365);
                    let _e370 = global_1.member[(_e247 + 24u)];
                    let _e371 = bitcast<f32>(_e370);
                    let _e375 = global_1.member[(_e247 + 25u)];
                    let _e376 = bitcast<f32>(_e375);
                    let _e380 = global_1.member[(_e247 + 26u)];
                    let _e381 = bitcast<f32>(_e380);
                    let _e385 = global_1.member[(_e247 + 27u)];
                    let _e386 = bitcast<f32>(_e385);
                    let _e390 = global_1.member[(_e247 + 28u)];
                    let _e391 = bitcast<f32>(_e390);
                    let _e395 = global_1.member[(_e247 + 29u)];
                    let _e396 = bitcast<f32>(_e395);
                    let _e400 = global_1.member[(_e247 + 30u)];
                    let _e401 = bitcast<f32>(_e400);
                    let _e405 = global_1.member[(_e247 + 31u)];
                    let _e406 = bitcast<f32>(_e405);
                    let _e409 = atomicLoad((&global_2.member[0u]));
                    let _e412 = atomicLoad((&global_2.member[1u]));
                    phi_3515_ = false;
                    phi_1236_ = type_20(_e74, type_10(_e409, _e412), 0u, 256u);
                    loop {
                        let _e416 = phi_3515_;
                        let _e418 = phi_1236_;
                        let _e429 = ((_e418.member_2 * _e418.member_3) + (((_e418.member.y % 16u) * 16u) + (_e418.member.x % 16u)));
                        if (_e429 < _e418.member_1.member_1) {
                            if (_e429 >= _e418.member_1.member_1) {
                                phi_1279_ = 4294967295u;
                            } else {
                                phi_1279_ = (_e418.member_1.member + _e429);
                            }
                            let _e447 = phi_1279_;
                            phi_1282_ = type_10(1u, _e447);
                        } else {
                            phi_1282_ = type_10(0u, type_10().member_1);
                        }
                        let _e450 = phi_1282_;
                        switch bitcast<i32>(_e450.member) {
                            case 0: {
                                phi_3538_ = _e416;
                                phi_1859_ = false;
                                break;
                            }
                            case 1: {
                                let _e456 = atomicLoad((&global_2.member[_e450.member_1]));
                                let _e459 = atomicLoad((&global_2.member[_e456]));
                                switch bitcast<i32>(_e459) {
                                    case 0: {
                                        phi_1300_ = 0u;
                                        break;
                                    }
                                    case 1: {
                                        phi_1300_ = 1u;
                                        break;
                                    }
                                    case 2: {
                                        phi_1300_ = 2u;
                                        break;
                                    }
                                    default: {
                                        phi_1300_ = 0u;
                                        break;
                                    }
                                }
                                let _e462 = phi_1300_;
                                let _e466 = atomicLoad((&global_2.member[(_e456 + 1u)]));
                                let _e470 = atomicLoad((&global_2.member[(_e456 + 2u)]));
                                if (_e73 >= 10u) {
                                    phi_2497_ = (_e470 <= (_e73 - 10u));
                                } else {
                                    phi_2497_ = false;
                                }
                                let _e475 = phi_2497_;
                                if _e475 {
                                    let _e478 = atomicLoad((&global_2.member[_e470]));
                                    let _e483 = atomicLoad((&global_2.member[(_e470 + 1u)]));
                                    let _e488 = atomicLoad((&global_2.member[(_e470 + 2u)]));
                                    let _e494 = atomicLoad((&global_2.member[(_e470 + 3u)]));
                                    let _e499 = atomicLoad((&global_2.member[(_e470 + 4u)]));
                                    let _e504 = atomicLoad((&global_2.member[(_e470 + 5u)]));
                                    let _e509 = atomicLoad((&global_2.member[(_e470 + 6u)]));
                                    let _e515 = atomicLoad((&global_2.member[(_e470 + 7u)]));
                                    let _e520 = atomicLoad((&global_2.member[(_e470 + 8u)]));
                                    let _e525 = atomicLoad((&global_2.member[(_e470 + 9u)]));
                                    phi_1361_ = type_15(vec3<f32>(bitcast<f32>(_e478), bitcast<f32>(_e483), bitcast<f32>(_e488)), vec4<f32>(bitcast<f32>(_e494), bitcast<f32>(_e499), bitcast<f32>(_e504), bitcast<f32>(_e509)), vec3<f32>(bitcast<f32>(_e515), bitcast<f32>(_e520), bitcast<f32>(_e525)));
                                } else {
                                    phi_1361_ = type_15(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
                                }
                                let _e530 = phi_1361_;
                                switch bitcast<i32>(_e462) {
                                    case 0: {
                                        if (_e73 >= 8u) {
                                            phi_3280_ = (_e466 <= (_e73 - 8u));
                                        } else {
                                            phi_3280_ = false;
                                        }
                                        let _e1168 = phi_3280_;
                                        if _e1168 {
                                            let _e1171 = atomicLoad((&global_2.member[_e466]));
                                            let _e1176 = atomicLoad((&global_2.member[(_e466 + 1u)]));
                                            let _e1181 = atomicLoad((&global_2.member[(_e466 + 2u)]));
                                            let _e1187 = atomicLoad((&global_2.member[(_e466 + 3u)]));
                                            let _e1192 = atomicLoad((&global_2.member[(_e466 + 4u)]));
                                            let _e1197 = atomicLoad((&global_2.member[(_e466 + 5u)]));
                                            let _e1202 = atomicLoad((&global_2.member[(_e466 + 6u)]));
                                            let _e1208 = atomicLoad((&global_2.member[(_e466 + 7u)]));
                                            phi_1407_ = type_21(vec3<f32>(bitcast<f32>(_e1171), bitcast<f32>(_e1176), bitcast<f32>(_e1181)), vec4<f32>(bitcast<f32>(_e1187), bitcast<f32>(_e1192), bitcast<f32>(_e1197), bitcast<f32>(_e1202)), bitcast<f32>(_e1208));
                                        } else {
                                            phi_1407_ = type_21(vec3<f32>(0f, -1f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e1212 = phi_1407_;
                                        phi_1692_ = _e1212.member_2;
                                        phi_1693_ = 0f;
                                        break;
                                    }
                                    case 1: {
                                        if (_e73 >= 8u) {
                                            phi_2905_ = (_e466 <= (_e73 - 8u));
                                        } else {
                                            phi_2905_ = false;
                                        }
                                        let _e865 = phi_2905_;
                                        if _e865 {
                                            let _e868 = atomicLoad((&global_2.member[_e466]));
                                            let _e873 = atomicLoad((&global_2.member[(_e466 + 1u)]));
                                            let _e878 = atomicLoad((&global_2.member[(_e466 + 2u)]));
                                            let _e884 = atomicLoad((&global_2.member[(_e466 + 3u)]));
                                            let _e889 = atomicLoad((&global_2.member[(_e466 + 4u)]));
                                            let _e894 = atomicLoad((&global_2.member[(_e466 + 5u)]));
                                            let _e899 = atomicLoad((&global_2.member[(_e466 + 6u)]));
                                            let _e905 = atomicLoad((&global_2.member[(_e466 + 7u)]));
                                            phi_1449_ = type_21(vec3<f32>(bitcast<f32>(_e868), bitcast<f32>(_e873), bitcast<f32>(_e878)), vec4<f32>(bitcast<f32>(_e884), bitcast<f32>(_e889), bitcast<f32>(_e894), bitcast<f32>(_e899)), bitcast<f32>(_e905));
                                        } else {
                                            phi_1449_ = type_21(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e909 = phi_1449_;
                                        let _e917 = (_e530.member_1.x + _e530.member_1.x);
                                        let _e918 = (_e530.member_1.y + _e530.member_1.y);
                                        let _e919 = (_e530.member_1.z + _e530.member_1.z);
                                        let _e920 = (_e530.member_1.x * _e917);
                                        let _e921 = (_e530.member_1.x * _e918);
                                        let _e922 = (_e530.member_1.x * _e919);
                                        let _e923 = (_e530.member_1.y * _e918);
                                        let _e924 = (_e530.member_1.y * _e919);
                                        let _e925 = (_e530.member_1.z * _e919);
                                        let _e926 = (_e530.member_1.w * _e917);
                                        let _e927 = (_e530.member_1.w * _e918);
                                        let _e928 = (_e530.member_1.w * _e919);
                                        let _e945 = (vec4<f32>((1f - (_e923 + _e925)), (_e921 + _e928), (_e922 - _e927), 0f) * _e530.member_2.x);
                                        let _e947 = (vec4<f32>((_e921 - _e928), (1f - (_e920 + _e925)), (_e924 + _e926), 0f) * _e530.member_2.y);
                                        let _e949 = (vec4<f32>((_e922 + _e927), (_e924 - _e926), (1f - (_e920 + _e923)), 0f) * _e530.member_2.z);
                                        let _e981 = (_e530.member.x + ((_e949.x * _e909.member.z) + ((_e947.x * _e909.member.y) + (_e945.x * _e909.member.x))));
                                        let _e982 = (_e530.member.y + ((_e949.y * _e909.member.z) + ((_e947.y * _e909.member.y) + (_e945.y * _e909.member.x))));
                                        let _e983 = (_e530.member.z + ((_e949.z * _e909.member.z) + ((_e947.z * _e909.member.y) + (_e945.z * _e909.member.x))));
                                        let _e1119 = (((((_e266 * _e391) + (_e286 * _e396)) + (_e306 * _e401)) + (_e326 * _e406)) + ((((((_e266 * _e371) + (_e286 * _e376)) + (_e306 * _e381)) + (_e326 * _e386)) * _e983) + ((((((_e266 * _e351) + (_e286 * _e356)) + (_e306 * _e361)) + (_e326 * _e366)) * _e982) + (((((_e266 * _e331) + (_e286 * _e336)) + (_e306 * _e341)) + (_e326 * _e346)) * _e981))));
                                        let _e1120 = ((((((_e251 * _e391) + (_e271 * _e396)) + (_e291 * _e401)) + (_e311 * _e406)) + ((((((_e251 * _e371) + (_e271 * _e376)) + (_e291 * _e381)) + (_e311 * _e386)) * _e983) + ((((((_e251 * _e351) + (_e271 * _e356)) + (_e291 * _e361)) + (_e311 * _e366)) * _e982) + (((((_e251 * _e331) + (_e271 * _e336)) + (_e291 * _e341)) + (_e311 * _e346)) * _e981)))) / _e1119);
                                        let _e1121 = ((((((_e256 * _e391) + (_e276 * _e396)) + (_e296 * _e401)) + (_e316 * _e406)) + ((((((_e256 * _e371) + (_e276 * _e376)) + (_e296 * _e381)) + (_e316 * _e386)) * _e983) + ((((((_e256 * _e351) + (_e276 * _e356)) + (_e296 * _e361)) + (_e316 * _e366)) * _e982) + (((((_e256 * _e331) + (_e276 * _e336)) + (_e296 * _e341)) + (_e316 * _e346)) * _e981)))) / _e1119);
                                        let _e1122 = ((((((_e261 * _e391) + (_e281 * _e396)) + (_e301 * _e401)) + (_e321 * _e406)) + ((((((_e261 * _e371) + (_e281 * _e376)) + (_e301 * _e381)) + (_e321 * _e386)) * _e983) + ((((((_e261 * _e351) + (_e281 * _e356)) + (_e301 * _e361)) + (_e321 * _e366)) * _e982) + (((((_e261 * _e331) + (_e281 * _e336)) + (_e301 * _e341)) + (_e321 * _e346)) * _e981)))) / _e1119);
                                        let _e1123 = (_e208 - _e208);
                                        let _e1124 = (_e209 - _e209);
                                        let _e1125 = (_e175 - _e177);
                                        let _e1131 = sqrt((((_e1123 * _e1123) + (_e1124 * _e1124)) + (_e1125 * _e1125)));
                                        if (_e1131 <= 0.00000011920929f) {
                                            let _e1152 = (_e1120 - _e208);
                                            let _e1153 = (_e1121 - _e209);
                                            let _e1154 = (_e1122 - _e175);
                                            phi_3258_ = sqrt((((_e1152 * _e1152) + (_e1153 * _e1153)) + (_e1154 * _e1154)));
                                        } else {
                                            let _e1133 = (_e1120 - _e208);
                                            let _e1134 = (_e1121 - _e209);
                                            let _e1135 = (_e1122 - _e175);
                                            let _e1136 = (_e1122 - _e177);
                                            let _e1139 = ((_e1134 * _e1136) - (_e1134 * _e1135));
                                            let _e1142 = ((_e1135 * _e1133) - (_e1136 * _e1133));
                                            let _e1143 = (_e1133 * _e1134);
                                            let _e1144 = (_e1143 - _e1143);
                                            phi_3258_ = (sqrt((((_e1139 * _e1139) + (_e1142 * _e1142)) + (_e1144 * _e1144))) / _e1131);
                                        }
                                        let _e1162 = phi_3258_;
                                        phi_1692_ = _e909.member_2;
                                        phi_1693_ = _e1162;
                                        break;
                                    }
                                    case 2: {
                                        if (_e73 >= 13u) {
                                            phi_2529_ = (_e466 <= (_e73 - 13u));
                                        } else {
                                            phi_2529_ = false;
                                        }
                                        let _e536 = phi_2529_;
                                        if _e536 {
                                            let _e539 = atomicLoad((&global_2.member[_e466]));
                                            let _e544 = atomicLoad((&global_2.member[(_e466 + 1u)]));
                                            let _e549 = atomicLoad((&global_2.member[(_e466 + 2u)]));
                                            let _e555 = atomicLoad((&global_2.member[(_e466 + 3u)]));
                                            let _e560 = atomicLoad((&global_2.member[(_e466 + 4u)]));
                                            let _e565 = atomicLoad((&global_2.member[(_e466 + 5u)]));
                                            let _e571 = atomicLoad((&global_2.member[(_e466 + 6u)]));
                                            let _e576 = atomicLoad((&global_2.member[(_e466 + 7u)]));
                                            let _e581 = atomicLoad((&global_2.member[(_e466 + 8u)]));
                                            let _e586 = atomicLoad((&global_2.member[(_e466 + 9u)]));
                                            let _e591 = atomicLoad((&global_2.member[(_e466 + 10u)]));
                                            let _e596 = atomicLoad((&global_2.member[(_e466 + 11u)]));
                                            let _e602 = atomicLoad((&global_2.member[(_e466 + 12u)]));
                                            phi_1601_ = type_22(vec3<f32>(bitcast<f32>(_e539), bitcast<f32>(_e544), bitcast<f32>(_e549)), vec3<f32>(bitcast<f32>(_e555), bitcast<f32>(_e560), bitcast<f32>(_e565)), bitcast<f32>(_e571), bitcast<f32>(_e576), vec4<f32>(bitcast<f32>(_e581), bitcast<f32>(_e586), bitcast<f32>(_e591), bitcast<f32>(_e596)), bitcast<f32>(_e602));
                                        } else {
                                            phi_1601_ = type_22(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, -1f, 0f), 0.077143565f, 0.09075713f, vec4<f32>(1f, 1f, 1f, 1f), 1f);
                                        }
                                        let _e606 = phi_1601_;
                                        let _e614 = (_e530.member_1.x + _e530.member_1.x);
                                        let _e615 = (_e530.member_1.y + _e530.member_1.y);
                                        let _e616 = (_e530.member_1.z + _e530.member_1.z);
                                        let _e617 = (_e530.member_1.x * _e614);
                                        let _e618 = (_e530.member_1.x * _e615);
                                        let _e619 = (_e530.member_1.x * _e616);
                                        let _e620 = (_e530.member_1.y * _e615);
                                        let _e621 = (_e530.member_1.y * _e616);
                                        let _e622 = (_e530.member_1.z * _e616);
                                        let _e623 = (_e530.member_1.w * _e614);
                                        let _e624 = (_e530.member_1.w * _e615);
                                        let _e625 = (_e530.member_1.w * _e616);
                                        let _e642 = (vec4<f32>((1f - (_e620 + _e622)), (_e618 + _e625), (_e619 - _e624), 0f) * _e530.member_2.x);
                                        let _e644 = (vec4<f32>((_e618 - _e625), (1f - (_e617 + _e622)), (_e621 + _e623), 0f) * _e530.member_2.y);
                                        let _e646 = (vec4<f32>((_e619 + _e624), (_e621 - _e623), (1f - (_e617 + _e620)), 0f) * _e530.member_2.z);
                                        let _e678 = (_e530.member.x + ((_e646.x * _e606.member.z) + ((_e644.x * _e606.member.y) + (_e642.x * _e606.member.x))));
                                        let _e679 = (_e530.member.y + ((_e646.y * _e606.member.z) + ((_e644.y * _e606.member.y) + (_e642.y * _e606.member.x))));
                                        let _e680 = (_e530.member.z + ((_e646.z * _e606.member.z) + ((_e644.z * _e606.member.y) + (_e642.z * _e606.member.x))));
                                        let _e816 = (((((_e266 * _e391) + (_e286 * _e396)) + (_e306 * _e401)) + (_e326 * _e406)) + ((((((_e266 * _e371) + (_e286 * _e376)) + (_e306 * _e381)) + (_e326 * _e386)) * _e680) + ((((((_e266 * _e351) + (_e286 * _e356)) + (_e306 * _e361)) + (_e326 * _e366)) * _e679) + (((((_e266 * _e331) + (_e286 * _e336)) + (_e306 * _e341)) + (_e326 * _e346)) * _e678))));
                                        let _e817 = ((((((_e251 * _e391) + (_e271 * _e396)) + (_e291 * _e401)) + (_e311 * _e406)) + ((((((_e251 * _e371) + (_e271 * _e376)) + (_e291 * _e381)) + (_e311 * _e386)) * _e680) + ((((((_e251 * _e351) + (_e271 * _e356)) + (_e291 * _e361)) + (_e311 * _e366)) * _e679) + (((((_e251 * _e331) + (_e271 * _e336)) + (_e291 * _e341)) + (_e311 * _e346)) * _e678)))) / _e816);
                                        let _e818 = ((((((_e256 * _e391) + (_e276 * _e396)) + (_e296 * _e401)) + (_e316 * _e406)) + ((((((_e256 * _e371) + (_e276 * _e376)) + (_e296 * _e381)) + (_e316 * _e386)) * _e680) + ((((((_e256 * _e351) + (_e276 * _e356)) + (_e296 * _e361)) + (_e316 * _e366)) * _e679) + (((((_e256 * _e331) + (_e276 * _e336)) + (_e296 * _e341)) + (_e316 * _e346)) * _e678)))) / _e816);
                                        let _e819 = ((((((_e261 * _e391) + (_e281 * _e396)) + (_e301 * _e401)) + (_e321 * _e406)) + ((((((_e261 * _e371) + (_e281 * _e376)) + (_e301 * _e381)) + (_e321 * _e386)) * _e680) + ((((((_e261 * _e351) + (_e281 * _e356)) + (_e301 * _e361)) + (_e321 * _e366)) * _e679) + (((((_e261 * _e331) + (_e281 * _e336)) + (_e301 * _e341)) + (_e321 * _e346)) * _e678)))) / _e816);
                                        let _e820 = (_e208 - _e208);
                                        let _e821 = (_e209 - _e209);
                                        let _e822 = (_e175 - _e177);
                                        let _e828 = sqrt((((_e820 * _e820) + (_e821 * _e821)) + (_e822 * _e822)));
                                        if (_e828 <= 0.00000011920929f) {
                                            let _e849 = (_e817 - _e208);
                                            let _e850 = (_e818 - _e209);
                                            let _e851 = (_e819 - _e175);
                                            phi_2883_ = sqrt((((_e849 * _e849) + (_e850 * _e850)) + (_e851 * _e851)));
                                        } else {
                                            let _e830 = (_e817 - _e208);
                                            let _e831 = (_e818 - _e209);
                                            let _e832 = (_e819 - _e175);
                                            let _e833 = (_e819 - _e177);
                                            let _e836 = ((_e831 * _e833) - (_e831 * _e832));
                                            let _e839 = ((_e832 * _e830) - (_e833 * _e830));
                                            let _e840 = (_e830 * _e831);
                                            let _e841 = (_e840 - _e840);
                                            phi_2883_ = (sqrt((((_e836 * _e836) + (_e839 * _e839)) + (_e841 * _e841))) / _e828);
                                        }
                                        let _e859 = phi_2883_;
                                        phi_1692_ = _e606.member_5;
                                        phi_1693_ = _e859;
                                        break;
                                    }
                                    default: {
                                        phi_1692_ = f32();
                                        phi_1693_ = f32();
                                        break;
                                    }
                                }
                                let _e1215 = phi_1692_;
                                let _e1217 = phi_1693_;
                                if (_e1217 > 0.00000011920929f) {
                                    phi_1704_ = (_e1215 / (_e1217 * _e1217));
                                } else {
                                    phi_1704_ = 340282350000000000000000000000000000000f;
                                }
                                let _e1224 = phi_1704_;
                                if (sqrt((_e1215 / _e129.member_2)) >= _e1217) {
                                    if (_e244 < _e73) {
                                    } else {
                                        phi_3537_ = true;
                                        break;
                                    }
                                    let _e1229 = atomicAdd((&global_2.member[_e244]), 1u);
                                    let _e1231 = (_e1229 >= _e241.member_1);
                                    if _e1231 {
                                        let _e1237 = (_e225 + 7u);
                                        loop {
                                            let _e1238 = (_e1237 < _e73);
                                            local_1 = _e1238;
                                            if _e1238 {
                                            } else {
                                                phi_3503_ = true;
                                                break;
                                            }
                                            let _e1241 = atomicExchange((&global_2.member[_e1237]), 1u);
                                            continue;
                                            continuing {
                                                phi_3503_ = _e416;
                                                break if !(select(true, false, (_e1241 == 0u)));
                                            }
                                        }
                                        let _e1246 = phi_3503_;
                                        phi_3537_ = _e1246;
                                        if _e1246 {
                                            break;
                                        }
                                        let _e1249 = atomicLoad((&global_2.member[(_e225 + 5u)]));
                                        let _e1252 = atomicLoad((&global_2.member[(_e225 + 6u)]));
                                        let _e1255 = atomicLoad((&global_2.member[(_e225 + 8u)]));
                                        if (_e1255 >= _e1252) {
                                            phi_3362_ = 4294967295u;
                                        } else {
                                            phi_3362_ = (_e1249 + _e1255);
                                        }
                                        let _e1259 = phi_3362_;
                                        let _e1261 = atomicLoad((&global_2.member[_e1259]));
                                        let _e1263 = (_e1224 > bitcast<f32>(_e1261));
                                        if _e1263 {
                                            let _e1264 = atomicLoad((&global_2.member[(_e225 + 8u)]));
                                            if (_e1264 >= _e241.member_1) {
                                                phi_3387_ = 4294967295u;
                                            } else {
                                                phi_3387_ = (_e241.member + _e1264);
                                            }
                                            let _e1268 = phi_3387_;
                                            atomicStore((&global_2.member[_e1268]), _e456);
                                            phi_1778_ = type_10(0u, _e1252);
                                            phi_1781_ = _e1264;
                                            phi_1783_ = _e1224;
                                            loop {
                                                let _e1272 = phi_1778_;
                                                let _e1274 = phi_1781_;
                                                let _e1276 = phi_1783_;
                                                local = _e1274;
                                                if (_e1272.member < _e1272.member_1) {
                                                    phi_1779_ = type_10((_e1272.member + 1u), _e1272.member_1);
                                                    phi_1806_ = type_10(1u, _e1272.member);
                                                } else {
                                                    phi_1779_ = _e1272;
                                                    phi_1806_ = type_10(0u, type_10().member_1);
                                                }
                                                let _e1289 = phi_1779_;
                                                let _e1291 = phi_1806_;
                                                switch bitcast<i32>(_e1291.member) {
                                                    case 0: {
                                                        phi_1782_ = u32();
                                                        phi_1784_ = f32();
                                                        phi_1825_ = false;
                                                        break;
                                                    }
                                                    case 1: {
                                                        if (_e1291.member_1 >= _e1252) {
                                                            phi_3413_ = 4294967295u;
                                                        } else {
                                                            phi_3413_ = (_e1249 + _e1291.member_1);
                                                        }
                                                        let _e1298 = phi_3413_;
                                                        let _e1300 = atomicLoad((&global_2.member[_e1298]));
                                                        let _e1301 = bitcast<f32>(_e1300);
                                                        let _e1302 = (_e1301 < _e1276);
                                                        phi_1782_ = select(_e1274, _e1291.member_1, _e1302);
                                                        phi_1784_ = select(_e1276, _e1301, _e1302);
                                                        phi_1825_ = true;
                                                        break;
                                                    }
                                                    default: {
                                                        phi_1782_ = u32();
                                                        phi_1784_ = f32();
                                                        phi_1825_ = bool();
                                                        break;
                                                    }
                                                }
                                                let _e1306 = phi_1782_;
                                                let _e1308 = phi_1784_;
                                                let _e1310 = phi_1825_;
                                                continue;
                                                continuing {
                                                    phi_1778_ = _e1289;
                                                    phi_1781_ = _e1306;
                                                    phi_1783_ = _e1308;
                                                    break if !(_e1310);
                                                }
                                            }
                                            let _e1313 = local;
                                            atomicStore((&global_2.member[(_e225 + 8u)]), _e1313);
                                            let _e1315 = local_1;
                                            if _e1315 {
                                            } else {
                                                phi_3537_ = true;
                                                break;
                                            }
                                            loop {
                                                let _e1317 = atomicExchange((&global_2.member[_e1237]), 0u);
                                                if select(true, false, (_e1317 == 1u)) {
                                                    continue;
                                                } else {
                                                    break;
                                                }
                                            }
                                        }
                                        phi_3540_ = _e1246;
                                        phi_1851_ = _e1263;
                                    } else {
                                        if _e1231 {
                                            phi_3316_ = 4294967295u;
                                        } else {
                                            phi_3316_ = (_e241.member + _e1229);
                                        }
                                        let _e1234 = phi_3316_;
                                        atomicStore((&global_2.member[_e1234]), _e456);
                                        phi_3540_ = _e416;
                                        phi_1851_ = true;
                                    }
                                    let _e1321 = phi_3540_;
                                    let _e1323 = phi_1851_;
                                    phi_3539_ = _e1321;
                                    phi_1855_ = _e1323;
                                } else {
                                    phi_3539_ = _e416;
                                    phi_1855_ = true;
                                }
                                let _e1325 = phi_3539_;
                                let _e1327 = phi_1855_;
                                phi_3538_ = _e1325;
                                phi_1859_ = _e1327;
                                break;
                            }
                            default: {
                                phi_3538_ = _e416;
                                phi_1859_ = bool();
                                break;
                            }
                        }
                        let _e1329 = phi_3538_;
                        let _e1331 = phi_1859_;
                        continue;
                        continuing {
                            phi_3515_ = _e1329;
                            phi_1236_ = type_20(_e418.member, _e418.member_1, (_e418.member_2 + 1u), _e418.member_3);
                            phi_3537_ = _e1329;
                            break if !(_e1331);
                        }
                    }
                    let _e1334 = phi_3537_;
                    if _e1334 {
                        break;
                    }
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(16, 16, 1) 
fn lightlight_tiling_bin_lights(@builtin(global_invocation_id) param: vec3<u32>) {
    global = param;
    function();
}
