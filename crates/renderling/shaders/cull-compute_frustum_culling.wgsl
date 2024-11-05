struct type_8 {
    member: array<u32>,
}

struct type_14 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_15 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
}

struct type_16 {
    member: type_14,
    member_1: type_14,
    member_2: type_15,
    member_3: vec3<f32>,
}

struct type_18 {
    member: u32,
    member_1: u32,
}

struct type_22 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_24 {
    member: array<type_22>,
}

struct type_29 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_30 {
    member: bool,
    member_1: u32,
}

@group(0) @binding(0) 
var<storage, read_write> global: type_8;
@group(0) @binding(1) 
var<storage, read_write> global_1: type_24;
var<private> global_2: vec3<u32>;

fn function() {
    var local: array<vec4<f32>, 6>;
    var local_1: array<vec3<f32>, 8>;
    var local_2: array<f32, 3>;
    var local_3: array<f32, 3>;
    var local_4: array<f32, 3>;
    var local_5: array<f32, 3>;
    var local_6: array<vec3<f32>, 8>;
    var local_7: array<vec4<f32>, 6>;
    var phi_1260_: bool;
    var phi_461_: type_18;
    var phi_477_: type_18;
    var phi_478_: type_18;
    var phi_491_: type_18;
    var phi_507_: type_18;
    var phi_508_: type_18;
    var phi_534_: type_16;
    var phi_535_: bool;
    var phi_492_: type_18;
    var phi_555_: type_16;
    var phi_556_: bool;
    var phi_462_: type_18;
    var phi_561_: type_16;
    var phi_1292_: bool;
    var phi_612_: type_29;
    var phi_744_: type_18;
    var phi_760_: type_18;
    var phi_761_: type_18;
    var phi_1039_: type_30;
    var phi_1041_: bool;
    var phi_1042_: type_30;
    var phi_1043_: bool;
    var phi_1044_: bool;
    var phi_1045_: type_30;
    var phi_1046_: bool;
    var phi_1051_: type_18;
    var phi_1053_: type_30;
    var phi_772_: type_18;
    var phi_788_: type_18;
    var phi_789_: type_18;
    var phi_799_: type_18;
    var phi_802_: i32;
    var phi_817_: type_18;
    var phi_818_: type_18;
    var phi_962_: i32;
    var phi_964_: type_18;
    var phi_965_: i32;
    var phi_966_: bool;
    var phi_967_: type_18;
    var phi_968_: i32;
    var phi_969_: bool;
    var phi_970_: type_18;
    var phi_971_: i32;
    var phi_832_: type_18;
    var phi_835_: i32;
    var phi_850_: type_18;
    var phi_851_: type_18;
    var phi_900_: i32;
    var phi_902_: type_18;
    var phi_903_: i32;
    var phi_904_: bool;
    var phi_905_: type_18;
    var phi_906_: i32;
    var phi_907_: bool;
    var phi_908_: type_18;
    var phi_909_: i32;
    var phi_866_: type_18;
    var phi_910_: bool;
    var phi_911_: bool;
    var phi_912_: bool;
    var phi_913_: bool;
    var phi_914_: type_18;
    var phi_915_: bool;
    var phi_833_: type_18;
    var phi_836_: i32;
    var phi_916_: bool;
    var phi_917_: bool;
    var phi_918_: bool;
    var phi_919_: bool;
    var phi_920_: type_18;
    var phi_928_: type_18;
    var phi_972_: bool;
    var phi_973_: bool;
    var phi_974_: bool;
    var phi_975_: bool;
    var phi_976_: type_18;
    var phi_977_: bool;
    var phi_800_: type_18;
    var phi_803_: i32;
    var phi_978_: bool;
    var phi_979_: bool;
    var phi_980_: bool;
    var phi_981_: bool;
    var phi_982_: type_18;
    var phi_989_: type_18;
    var phi_998_: type_30;
    var phi_1000_: type_30;
    var phi_773_: type_18;
    var phi_1054_: bool;
    var phi_1055_: type_30;
    var phi_1056_: bool;
    var phi_745_: type_18;
    var phi_1057_: bool;
    var phi_1058_: type_30;
    var local_8: type_16;
    var local_9: type_16;
    var local_10: bool;
    var local_11: bool;
    var local_12: bool;
    var local_13: bool;
    var local_14: type_18;
    var local_15: bool;
    var local_16: bool;
    var local_17: bool;
    var local_18: bool;
    var local_19: type_18;
    var local_20: bool;
    var local_21: type_30;
    var local_22: bool;
    var local_23: type_30;

    let _e81 = arrayLength((&global.member));
    let _e84 = global_2;
    if (_e84.x >= arrayLength((&global_1.member))) {
    } else {
        let _e90 = global_1.member[_e84.x].member_3;
        let _e93 = global.member[_e90];
        let _e98 = global.member[(_e90 + 2u)];
        let _e102 = global.member[(_e90 + 3u)];
        let _e103 = bitcast<f32>(_e102);
        let _e107 = global.member[(_e90 + 4u)];
        let _e108 = bitcast<f32>(_e107);
        let _e112 = global.member[(_e90 + 5u)];
        let _e113 = bitcast<f32>(_e112);
        let _e117 = global.member[(_e90 + 6u)];
        let _e118 = bitcast<f32>(_e117);
        let _e122 = global.member[(_e90 + 7u)];
        let _e126 = global.member[(_e90 + 8u)];
        let _e130 = global.member[(_e90 + 9u)];
        let _e134 = global.member[(_e90 + 10u)];
        global_1.member[_e84.x].member = select(_e126, _e98, (_e122 == 4294967295u));
        global_1.member[_e84.x].member_1 = select(0u, 1u, (_e93 == 1u));
        if (_e118 == 0f) {
        } else {
            if (_e81 >= 83u) {
                phi_1260_ = (_e130 <= (_e81 - 83u));
            } else {
                phi_1260_ = false;
            }
            let _e149 = phi_1260_;
            if _e149 {
                let _e152 = global.member[_e130];
                let _e157 = global.member[(_e130 + 1u)];
                let _e162 = global.member[(_e130 + 2u)];
                let _e167 = global.member[(_e130 + 3u)];
                let _e173 = global.member[(_e130 + 4u)];
                let _e178 = global.member[(_e130 + 5u)];
                let _e183 = global.member[(_e130 + 6u)];
                let _e188 = global.member[(_e130 + 7u)];
                let _e194 = global.member[(_e130 + 8u)];
                let _e199 = global.member[(_e130 + 9u)];
                let _e204 = global.member[(_e130 + 10u)];
                let _e209 = global.member[(_e130 + 11u)];
                let _e215 = global.member[(_e130 + 12u)];
                let _e220 = global.member[(_e130 + 13u)];
                let _e225 = global.member[(_e130 + 14u)];
                let _e230 = global.member[(_e130 + 15u)];
                let _e237 = global.member[(_e130 + 16u)];
                let _e242 = global.member[(_e130 + 17u)];
                let _e247 = global.member[(_e130 + 18u)];
                let _e252 = global.member[(_e130 + 19u)];
                let _e258 = global.member[(_e130 + 20u)];
                let _e263 = global.member[(_e130 + 21u)];
                let _e268 = global.member[(_e130 + 22u)];
                let _e273 = global.member[(_e130 + 23u)];
                let _e279 = global.member[(_e130 + 24u)];
                let _e284 = global.member[(_e130 + 25u)];
                let _e289 = global.member[(_e130 + 26u)];
                let _e294 = global.member[(_e130 + 27u)];
                let _e300 = global.member[(_e130 + 28u)];
                let _e305 = global.member[(_e130 + 29u)];
                let _e310 = global.member[(_e130 + 30u)];
                let _e315 = global.member[(_e130 + 31u)];
                let _e322 = global.member[(_e130 + 32u)];
                let _e327 = global.member[(_e130 + 33u)];
                let _e332 = global.member[(_e130 + 34u)];
                local_7 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                phi_461_ = type_18(0u, 6u);
                loop {
                    let _e337 = phi_461_;
                    if (_e337.member < _e337.member_1) {
                        phi_477_ = type_18((_e337.member + 1u), _e337.member_1);
                        phi_478_ = type_18(1u, _e337.member);
                    } else {
                        phi_477_ = _e337;
                        phi_478_ = type_18(0u, type_18().member_1);
                    }
                    let _e350 = phi_477_;
                    let _e352 = phi_478_;
                    switch bitcast<i32>(_e352.member) {
                        case 0: {
                            let _e379 = local_7;
                            local_6 = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                            phi_491_ = type_18(0u, 8u);
                            loop {
                                let _e382 = phi_491_;
                                if (_e382.member < _e382.member_1) {
                                    phi_507_ = type_18((_e382.member + 1u), _e382.member_1);
                                    phi_508_ = type_18(1u, _e382.member);
                                } else {
                                    phi_507_ = _e382;
                                    phi_508_ = type_18(0u, type_18().member_1);
                                }
                                let _e395 = phi_507_;
                                let _e397 = phi_508_;
                                switch bitcast<i32>(_e397.member) {
                                    case 0: {
                                        let _e419 = local_6;
                                        phi_534_ = type_16(type_14(vec4<f32>(bitcast<f32>(_e152), bitcast<f32>(_e157), bitcast<f32>(_e162), bitcast<f32>(_e167)), vec4<f32>(bitcast<f32>(_e173), bitcast<f32>(_e178), bitcast<f32>(_e183), bitcast<f32>(_e188)), vec4<f32>(bitcast<f32>(_e194), bitcast<f32>(_e199), bitcast<f32>(_e204), bitcast<f32>(_e209)), vec4<f32>(bitcast<f32>(_e215), bitcast<f32>(_e220), bitcast<f32>(_e225), bitcast<f32>(_e230))), type_14(vec4<f32>(bitcast<f32>(_e237), bitcast<f32>(_e242), bitcast<f32>(_e247), bitcast<f32>(_e252)), vec4<f32>(bitcast<f32>(_e258), bitcast<f32>(_e263), bitcast<f32>(_e268), bitcast<f32>(_e273)), vec4<f32>(bitcast<f32>(_e279), bitcast<f32>(_e284), bitcast<f32>(_e289), bitcast<f32>(_e294)), vec4<f32>(bitcast<f32>(_e300), bitcast<f32>(_e305), bitcast<f32>(_e310), bitcast<f32>(_e315))), type_15(_e419, _e379), vec3<f32>(bitcast<f32>(_e322), bitcast<f32>(_e327), bitcast<f32>(_e332)));
                                        phi_535_ = false;
                                        phi_492_ = type_18();
                                        break;
                                    }
                                    case 1: {
                                        let _e402 = ((_e130 + 59u) + (_e397.member_1 * 3u));
                                        let _e405 = global.member[_e402];
                                        let _e410 = global.member[(_e402 + 1u)];
                                        let _e415 = global.member[(_e402 + 2u)];
                                        local_6[_e397.member_1] = vec3<f32>(bitcast<f32>(_e405), bitcast<f32>(_e410), bitcast<f32>(_e415));
                                        phi_534_ = type_16();
                                        phi_535_ = true;
                                        phi_492_ = _e395;
                                        break;
                                    }
                                    default: {
                                        phi_534_ = type_16();
                                        phi_535_ = false;
                                        phi_492_ = type_18();
                                        break;
                                    }
                                }
                                let _e423 = phi_534_;
                                let _e425 = phi_535_;
                                let _e427 = phi_492_;
                                local_8 = _e423;
                                continue;
                                continuing {
                                    phi_491_ = _e427;
                                    break if !(_e425);
                                }
                            }
                            let _e907 = local_8;
                            phi_555_ = _e907;
                            phi_556_ = false;
                            phi_462_ = type_18();
                            break;
                        }
                        case 1: {
                            let _e357 = ((_e130 + 35u) + (_e352.member_1 * 4u));
                            let _e360 = global.member[_e357];
                            let _e365 = global.member[(_e357 + 1u)];
                            let _e370 = global.member[(_e357 + 2u)];
                            let _e375 = global.member[(_e357 + 3u)];
                            local_7[_e352.member_1] = vec4<f32>(bitcast<f32>(_e360), bitcast<f32>(_e365), bitcast<f32>(_e370), bitcast<f32>(_e375));
                            phi_555_ = type_16();
                            phi_556_ = true;
                            phi_462_ = _e350;
                            break;
                        }
                        default: {
                            phi_555_ = type_16();
                            phi_556_ = false;
                            phi_462_ = type_18();
                            break;
                        }
                    }
                    let _e430 = phi_555_;
                    let _e432 = phi_556_;
                    let _e434 = phi_462_;
                    local_9 = _e430;
                    continue;
                    continuing {
                        phi_461_ = _e434;
                        break if !(_e432);
                    }
                }
                let _e912 = local_9;
                phi_561_ = _e912;
            } else {
                phi_561_ = type_16(type_14(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_14(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_15(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
            }
            let _e437 = phi_561_;
            if (_e81 >= 10u) {
                phi_1292_ = (_e134 <= (_e81 - 10u));
            } else {
                phi_1292_ = false;
            }
            let _e443 = phi_1292_;
            if _e443 {
                let _e446 = global.member[_e134];
                let _e451 = global.member[(_e134 + 1u)];
                let _e456 = global.member[(_e134 + 2u)];
                let _e462 = global.member[(_e134 + 3u)];
                let _e467 = global.member[(_e134 + 4u)];
                let _e472 = global.member[(_e134 + 5u)];
                let _e477 = global.member[(_e134 + 6u)];
                let _e483 = global.member[(_e134 + 7u)];
                let _e488 = global.member[(_e134 + 8u)];
                let _e493 = global.member[(_e134 + 9u)];
                phi_612_ = type_29(vec3<f32>(bitcast<f32>(_e446), bitcast<f32>(_e451), bitcast<f32>(_e456)), vec4<f32>(bitcast<f32>(_e462), bitcast<f32>(_e467), bitcast<f32>(_e472), bitcast<f32>(_e477)), vec3<f32>(bitcast<f32>(_e483), bitcast<f32>(_e488), bitcast<f32>(_e493)));
            } else {
                phi_612_ = type_29(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
            }
            let _e498 = phi_612_;
            let _e506 = (_e498.member_1.x + _e498.member_1.x);
            let _e507 = (_e498.member_1.y + _e498.member_1.y);
            let _e508 = (_e498.member_1.z + _e498.member_1.z);
            let _e510 = (_e498.member_1.z * _e508);
            let _e511 = (_e498.member_1.w * _e506);
            let _e512 = (_e498.member_1.w * _e507);
            let _e513 = (_e498.member_1.w * _e508);
            let _e533 = (vec4<f32>((1f - fma(_e498.member_1.y, _e507, _e510)), fma(_e498.member_1.x, _e507, _e513), fma(_e498.member_1.x, _e508, -(_e512)), 0f) * _e498.member_2.x);
            let _e535 = (vec4<f32>(fma(_e498.member_1.x, _e507, -(_e513)), (1f - fma(_e498.member_1.x, _e506, _e510)), fma(_e498.member_1.y, _e508, _e511), 0f) * _e498.member_2.y);
            let _e537 = (vec4<f32>(fma(_e498.member_1.x, _e508, _e512), fma(_e498.member_1.y, _e508, -(_e511)), (1f - fma(_e498.member_1.x, _e506, (_e498.member_1.y * _e507))), 0f) * _e498.member_2.z);
            let _e559 = (_e498.member.x + fma(_e537.x, _e113, fma(_e535.x, _e108, (_e533.x * _e103))));
            let _e560 = (_e498.member.y + fma(_e537.y, _e113, fma(_e535.y, _e108, (_e533.y * _e103))));
            let _e561 = (_e498.member.z + fma(_e537.z, _e113, fma(_e535.z, _e108, (_e533.z * _e103))));
            let _e562 = vec3<f32>(_e559, _e560, _e561);
            let _e565 = (max(_e498.member_2.x, max(_e498.member_2.y, _e498.member_2.z)) * _e118);
            let _e567 = sqrt((_e565 * _e565));
            local_1 = _e437.member_2.member;
            local = _e437.member_2.member_1;
            let _e572 = local[0u][0u];
            let _e575 = local[0u][1u];
            let _e580 = local[0u][2u];
            let _e584 = local[0u][3u];
            let _e586 = -(_e567);
            if ((fma(_e580, _e561, fma(_e572, _e559, (_e575 * _e560))) + _e584) < _e586) {
                phi_1057_ = true;
                phi_1058_ = type_30(true, 0u);
            } else {
                phi_744_ = type_18(0u, 6u);
                loop {
                    let _e589 = phi_744_;
                    if (_e589.member < _e589.member_1) {
                        phi_760_ = type_18((_e589.member + 1u), _e589.member_1);
                        phi_761_ = type_18(1u, _e589.member);
                    } else {
                        phi_760_ = _e589;
                        phi_761_ = type_18(0u, type_18().member_1);
                    }
                    let _e602 = phi_760_;
                    let _e604 = phi_761_;
                    switch bitcast<i32>(_e604.member) {
                        case 0: {
                            let _e650 = vec3(_e567);
                            let _e651 = (_e562 - _e650);
                            let _e652 = (_e562 + _e650);
                            phi_772_ = type_18(0u, 3u);
                            loop {
                                let _e654 = phi_772_;
                                if (_e654.member < _e654.member_1) {
                                    phi_788_ = type_18((_e654.member + 1u), _e654.member_1);
                                    phi_789_ = type_18(1u, _e654.member);
                                } else {
                                    phi_788_ = _e654;
                                    phi_789_ = type_18(0u, type_18().member_1);
                                }
                                let _e667 = phi_788_;
                                let _e669 = phi_789_;
                                switch bitcast<i32>(_e669.member) {
                                    case 0: {
                                        phi_978_ = false;
                                        phi_979_ = true;
                                        phi_980_ = true;
                                        phi_981_ = false;
                                        phi_982_ = type_18();
                                        break;
                                    }
                                    case 1: {
                                        phi_799_ = type_18(0u, 8u);
                                        phi_802_ = 0i;
                                        loop {
                                            let _e674 = phi_799_;
                                            let _e676 = phi_802_;
                                            if (_e674.member < _e674.member_1) {
                                                phi_817_ = type_18((_e674.member + 1u), _e674.member_1);
                                                phi_818_ = type_18(1u, _e674.member);
                                            } else {
                                                phi_817_ = _e674;
                                                phi_818_ = type_18(0u, type_18().member_1);
                                            }
                                            let _e689 = phi_817_;
                                            let _e691 = phi_818_;
                                            switch bitcast<i32>(_e691.member) {
                                                case 0: {
                                                    if (_e676 == 8i) {
                                                        phi_916_ = true;
                                                        phi_917_ = false;
                                                        phi_918_ = false;
                                                        phi_919_ = false;
                                                        phi_920_ = type_18();
                                                    } else {
                                                        phi_832_ = type_18(0u, 8u);
                                                        phi_835_ = 0i;
                                                        loop {
                                                            let _e738 = phi_832_;
                                                            let _e740 = phi_835_;
                                                            if (_e738.member < _e738.member_1) {
                                                                phi_850_ = type_18((_e738.member + 1u), _e738.member_1);
                                                                phi_851_ = type_18(1u, _e738.member);
                                                            } else {
                                                                phi_850_ = _e738;
                                                                phi_851_ = type_18(0u, type_18().member_1);
                                                            }
                                                            let _e753 = phi_850_;
                                                            let _e755 = phi_851_;
                                                            switch bitcast<i32>(_e755.member) {
                                                                case 0: {
                                                                    let _e800 = (_e740 == 8i);
                                                                    if _e800 {
                                                                        phi_866_ = type_18();
                                                                    } else {
                                                                        phi_866_ = _e667;
                                                                    }
                                                                    let _e802 = phi_866_;
                                                                    phi_910_ = false;
                                                                    phi_911_ = select(false, true, _e800);
                                                                    phi_912_ = select(bool(), false, _e800);
                                                                    phi_913_ = select(true, false, _e800);
                                                                    phi_914_ = _e802;
                                                                    phi_915_ = false;
                                                                    phi_833_ = type_18();
                                                                    phi_836_ = i32();
                                                                    break;
                                                                }
                                                                case 1: {
                                                                    if (_e755.member_1 < 8u) {
                                                                        let _e762 = local_1[_e755.member_1][0u];
                                                                        let _e765 = local_1[_e755.member_1][1u];
                                                                        let _e768 = local_1[_e755.member_1][2u];
                                                                        local_4 = array<f32, 3>(_e762, _e765, _e768);
                                                                        let _e770 = (_e669.member_1 < 3u);
                                                                        if _e770 {
                                                                            let _e772 = local_4[_e669.member_1];
                                                                            local_5 = array<f32, 3>(_e652.x, _e652.y, _e652.z);
                                                                            if _e770 {
                                                                                let _e778 = local_5[_e669.member_1];
                                                                                if (_e772 > _e778) {
                                                                                    phi_900_ = (_e740 + 1i);
                                                                                } else {
                                                                                    phi_900_ = _e740;
                                                                                }
                                                                                let _e782 = phi_900_;
                                                                                phi_902_ = _e753;
                                                                                phi_903_ = _e782;
                                                                            } else {
                                                                                phi_902_ = type_18();
                                                                                phi_903_ = i32();
                                                                            }
                                                                            let _e784 = phi_902_;
                                                                            let _e786 = phi_903_;
                                                                            phi_904_ = select(false, true, _e770);
                                                                            phi_905_ = _e784;
                                                                            phi_906_ = _e786;
                                                                        } else {
                                                                            phi_904_ = false;
                                                                            phi_905_ = type_18();
                                                                            phi_906_ = i32();
                                                                        }
                                                                        let _e789 = phi_904_;
                                                                        let _e791 = phi_905_;
                                                                        let _e793 = phi_906_;
                                                                        phi_907_ = _e789;
                                                                        phi_908_ = _e791;
                                                                        phi_909_ = _e793;
                                                                    } else {
                                                                        phi_907_ = false;
                                                                        phi_908_ = type_18();
                                                                        phi_909_ = i32();
                                                                    }
                                                                    let _e795 = phi_907_;
                                                                    let _e797 = phi_908_;
                                                                    let _e799 = phi_909_;
                                                                    phi_910_ = false;
                                                                    phi_911_ = false;
                                                                    phi_912_ = bool();
                                                                    phi_913_ = false;
                                                                    phi_914_ = type_18();
                                                                    phi_915_ = _e795;
                                                                    phi_833_ = _e797;
                                                                    phi_836_ = _e799;
                                                                    break;
                                                                }
                                                                default: {
                                                                    phi_910_ = true;
                                                                    phi_911_ = false;
                                                                    phi_912_ = bool();
                                                                    phi_913_ = false;
                                                                    phi_914_ = type_18();
                                                                    phi_915_ = false;
                                                                    phi_833_ = type_18();
                                                                    phi_836_ = i32();
                                                                    break;
                                                                }
                                                            }
                                                            let _e807 = phi_910_;
                                                            let _e809 = phi_911_;
                                                            let _e811 = phi_912_;
                                                            let _e813 = phi_913_;
                                                            let _e815 = phi_914_;
                                                            let _e817 = phi_915_;
                                                            let _e819 = phi_833_;
                                                            let _e821 = phi_836_;
                                                            local_10 = _e809;
                                                            local_11 = _e811;
                                                            local_12 = _e807;
                                                            local_13 = _e813;
                                                            local_14 = _e815;
                                                            continue;
                                                            continuing {
                                                                phi_832_ = _e819;
                                                                phi_835_ = _e821;
                                                                break if !(_e817);
                                                            }
                                                        }
                                                        let _e967 = local_10;
                                                        phi_916_ = _e967;
                                                        let _e970 = local_11;
                                                        phi_917_ = _e970;
                                                        let _e973 = local_12;
                                                        phi_918_ = _e973;
                                                        let _e976 = local_13;
                                                        phi_919_ = _e976;
                                                        let _e979 = local_14;
                                                        phi_920_ = _e979;
                                                    }
                                                    let _e824 = phi_916_;
                                                    let _e826 = phi_917_;
                                                    let _e828 = phi_918_;
                                                    let _e830 = phi_919_;
                                                    let _e832 = phi_920_;
                                                    if _e824 {
                                                        phi_928_ = type_18();
                                                    } else {
                                                        phi_928_ = _e832;
                                                    }
                                                    let _e834 = phi_928_;
                                                    phi_972_ = select(_e828, false, _e824);
                                                    phi_973_ = select(false, true, _e824);
                                                    phi_974_ = select(bool(), _e826, _e824);
                                                    phi_975_ = select(_e830, false, _e824);
                                                    phi_976_ = _e834;
                                                    phi_977_ = false;
                                                    phi_800_ = type_18();
                                                    phi_803_ = i32();
                                                    break;
                                                }
                                                case 1: {
                                                    if (_e691.member_1 < 8u) {
                                                        let _e698 = local_1[_e691.member_1][0u];
                                                        let _e701 = local_1[_e691.member_1][1u];
                                                        let _e704 = local_1[_e691.member_1][2u];
                                                        local_2 = array<f32, 3>(_e698, _e701, _e704);
                                                        let _e706 = (_e669.member_1 < 3u);
                                                        if _e706 {
                                                            let _e708 = local_2[_e669.member_1];
                                                            local_3 = array<f32, 3>(_e651.x, _e651.y, _e651.z);
                                                            if _e706 {
                                                                let _e714 = local_3[_e669.member_1];
                                                                if (_e708 < _e714) {
                                                                    phi_962_ = (_e676 + 1i);
                                                                } else {
                                                                    phi_962_ = _e676;
                                                                }
                                                                let _e718 = phi_962_;
                                                                phi_964_ = _e689;
                                                                phi_965_ = _e718;
                                                            } else {
                                                                phi_964_ = type_18();
                                                                phi_965_ = i32();
                                                            }
                                                            let _e720 = phi_964_;
                                                            let _e722 = phi_965_;
                                                            phi_966_ = select(false, true, _e706);
                                                            phi_967_ = _e720;
                                                            phi_968_ = _e722;
                                                        } else {
                                                            phi_966_ = false;
                                                            phi_967_ = type_18();
                                                            phi_968_ = i32();
                                                        }
                                                        let _e725 = phi_966_;
                                                        let _e727 = phi_967_;
                                                        let _e729 = phi_968_;
                                                        phi_969_ = _e725;
                                                        phi_970_ = _e727;
                                                        phi_971_ = _e729;
                                                    } else {
                                                        phi_969_ = false;
                                                        phi_970_ = type_18();
                                                        phi_971_ = i32();
                                                    }
                                                    let _e731 = phi_969_;
                                                    let _e733 = phi_970_;
                                                    let _e735 = phi_971_;
                                                    phi_972_ = false;
                                                    phi_973_ = false;
                                                    phi_974_ = bool();
                                                    phi_975_ = false;
                                                    phi_976_ = type_18();
                                                    phi_977_ = _e731;
                                                    phi_800_ = _e733;
                                                    phi_803_ = _e735;
                                                    break;
                                                }
                                                default: {
                                                    phi_972_ = true;
                                                    phi_973_ = false;
                                                    phi_974_ = bool();
                                                    phi_975_ = false;
                                                    phi_976_ = type_18();
                                                    phi_977_ = false;
                                                    phi_800_ = type_18();
                                                    phi_803_ = i32();
                                                    break;
                                                }
                                            }
                                            let _e840 = phi_972_;
                                            let _e842 = phi_973_;
                                            let _e844 = phi_974_;
                                            let _e846 = phi_975_;
                                            let _e848 = phi_976_;
                                            let _e850 = phi_977_;
                                            let _e852 = phi_800_;
                                            let _e854 = phi_803_;
                                            local_15 = _e840;
                                            local_16 = _e842;
                                            local_17 = _e844;
                                            local_18 = _e846;
                                            local_19 = _e848;
                                            continue;
                                            continuing {
                                                phi_799_ = _e852;
                                                phi_802_ = _e854;
                                                break if !(_e850);
                                            }
                                        }
                                        let _e991 = local_15;
                                        phi_978_ = _e991;
                                        let _e994 = local_16;
                                        phi_979_ = _e994;
                                        let _e997 = local_17;
                                        phi_980_ = _e997;
                                        let _e1000 = local_18;
                                        phi_981_ = _e1000;
                                        let _e1003 = local_19;
                                        phi_982_ = _e1003;
                                        break;
                                    }
                                    default: {
                                        phi_978_ = true;
                                        phi_979_ = false;
                                        phi_980_ = bool();
                                        phi_981_ = false;
                                        phi_982_ = type_18();
                                        break;
                                    }
                                }
                                let _e857 = phi_978_;
                                let _e859 = phi_979_;
                                let _e861 = phi_980_;
                                let _e863 = phi_981_;
                                let _e865 = phi_982_;
                                if _e857 {
                                    phi_989_ = type_18();
                                } else {
                                    phi_989_ = _e865;
                                }
                                let _e867 = phi_989_;
                                let _e868 = select(_e859, false, _e857);
                                if _e868 {
                                    if select(_e861, bool(), _e857) {
                                        phi_998_ = type_30(false, 0u);
                                    } else {
                                        phi_998_ = type_30(true, 0u);
                                    }
                                    let _e872 = phi_998_;
                                    phi_1000_ = _e872;
                                    phi_773_ = type_18();
                                } else {
                                    phi_1000_ = type_30();
                                    phi_773_ = _e867;
                                }
                                let _e874 = phi_1000_;
                                let _e876 = phi_773_;
                                local_20 = select(false, true, _e868);
                                local_21 = _e874;
                                continue;
                                continuing {
                                    phi_772_ = _e876;
                                    break if !(select(select(_e863, false, _e857), false, _e868));
                                }
                            }
                            let _e1010 = local_20;
                            phi_1054_ = _e1010;
                            let _e1013 = local_21;
                            phi_1055_ = _e1013;
                            phi_1056_ = false;
                            phi_745_ = type_18();
                            break;
                        }
                        case 1: {
                            if (_e604.member_1 != 0u) {
                                if (_e604.member_1 < 6u) {
                                    let _e612 = local[_e604.member_1][0u];
                                    let _e615 = local[_e604.member_1][1u];
                                    let _e620 = local[_e604.member_1][2u];
                                    let _e624 = local[_e604.member_1][3u];
                                    let _e626 = ((fma(_e620, _e561, fma(_e612, _e559, (_e615 * _e560))) + _e624) < _e586);
                                    if _e626 {
                                        phi_1039_ = type_30(true, _e604.member_1);
                                    } else {
                                        phi_1039_ = type_30();
                                    }
                                    let _e629 = phi_1039_;
                                    phi_1041_ = select(false, true, _e626);
                                    phi_1042_ = _e629;
                                    phi_1043_ = select(true, false, _e626);
                                } else {
                                    phi_1041_ = false;
                                    phi_1042_ = type_30();
                                    phi_1043_ = false;
                                }
                                let _e633 = phi_1041_;
                                let _e635 = phi_1042_;
                                let _e637 = phi_1043_;
                                phi_1044_ = _e633;
                                phi_1045_ = _e635;
                                phi_1046_ = _e637;
                            } else {
                                phi_1044_ = false;
                                phi_1045_ = type_30();
                                phi_1046_ = true;
                            }
                            let _e639 = phi_1044_;
                            let _e641 = phi_1045_;
                            let _e643 = phi_1046_;
                            if _e643 {
                                phi_1051_ = _e602;
                                phi_1053_ = type_30();
                            } else {
                                phi_1051_ = type_18();
                                phi_1053_ = _e641;
                            }
                            let _e645 = phi_1051_;
                            let _e647 = phi_1053_;
                            phi_1054_ = select(_e639, false, _e643);
                            phi_1055_ = _e647;
                            phi_1056_ = select(false, true, _e643);
                            phi_745_ = _e645;
                            break;
                        }
                        default: {
                            phi_1054_ = false;
                            phi_1055_ = type_30();
                            phi_1056_ = false;
                            phi_745_ = type_18();
                            break;
                        }
                    }
                    let _e881 = phi_1054_;
                    let _e883 = phi_1055_;
                    let _e885 = phi_1056_;
                    let _e887 = phi_745_;
                    local_22 = _e881;
                    local_23 = _e883;
                    continue;
                    continuing {
                        phi_744_ = _e887;
                        break if !(_e885);
                    }
                }
                let _e1018 = local_22;
                phi_1057_ = _e1018;
                let _e1021 = local_23;
                phi_1058_ = _e1021;
            }
            let _e890 = phi_1057_;
            let _e892 = phi_1058_;
            if _e890 {
                if (_e892.member != true) {
                } else {
                    global_1.member[_e84.x].member_1 = 0u;
                }
            }
        }
    }
    return;
}

@compute @workgroup_size(32, 1, 1) 
fn cullcompute_frustum_culling(@builtin(global_invocation_id) param: vec3<u32>) {
    global_2 = param;
    function();
}
