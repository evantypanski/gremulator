pub fn add(accumulator: &mut u8, val: Option<u8>, flags: &mut u8) {
    let result: u16;
    let to_add = val.unwrap_or(*accumulator);

    // Not using wrapping_add to check more easily result > 0xff later
    result = *accumulator as u16 + to_add as u16;

    set_flags(
        flags,
        result & 0xff == 0,
        false,
        (((*accumulator & 0xf) + (to_add & 0xf)) & 0x10) == 0x10,
        result > 0xff,
    );

    *accumulator = result as u8;
}

pub fn add_carry(accumulator: &mut u8, val: Option<u8>, flags: &mut u8) {
    let result: u16;
    let carry = (*flags & 0b00010000) >> 4;
    let to_add = val.unwrap_or(*accumulator) + carry;

    // Not using wrapping_add to check more easily result > 0xff later
    result = *accumulator as u16 + to_add as u16;

    set_flags(
        flags,
        result & 0xff == 0,
        false,
        (((*accumulator & 0xf) + (to_add & 0xf)) & 0x10) == 0x10,
        result > 0xff,
    );

    *accumulator = result as u8;
}

pub fn sub(accumulator: &mut u8, val: Option<u8>, flags: &mut u8) {
    let result: u8;
    let to_sub = val.unwrap_or(*accumulator);

    result = (*accumulator).wrapping_sub(to_sub);

    set_flags(
        flags,
        result == 0,
        true,
        (*accumulator & 0xf) < (to_sub & 0xf),
        *accumulator < to_sub,
    );

    *accumulator = result;
}

pub fn and(accumulator: &mut u8, val: Option<u8>, flags: &mut u8) {
    let to_and = val.unwrap_or(*accumulator);
    *accumulator = *accumulator & to_and;
    set_flags(flags, *accumulator == 0, false, true, false);
}

pub fn xor(accumulator: &mut u8, val: Option<u8>, flags: &mut u8) {
    let to_xor = val.unwrap_or(*accumulator);
    *accumulator = *accumulator ^ to_xor;
    set_flags(flags, *accumulator == 0, false, false, false);
}

pub fn or(accumulator: &mut u8, val: Option<u8>, flags: &mut u8) {
    let to_or = val.unwrap_or(*accumulator);
    *accumulator = *accumulator | to_or;
    set_flags(flags, *accumulator == 0, false, false, false);
}

pub fn cp(accumulator: u8, val: Option<u8>, flags: &mut u8) {
    let mut mutable_accumulator = accumulator;
    sub(&mut mutable_accumulator, val, flags);
}

fn set_flags(flags: &mut u8, z: bool, n: bool, h: bool, c: bool) {
    *flags = ((z as u8) << 7) | ((n as u8) << 6) | ((h as u8) << 5) | ((c as u8) << 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_simple_add_calculates_correctly() {
        let mut accumulator = 40;
        let val = 2;
        let mut flags = 0;
        add(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 42);
        // Also ensure flags don't get set as baseline
        assert_eq!(flags, 0);
    }

    #[test]
    fn add_missing_value_adds_accumulator_to_itself() {
        let mut accumulator = 5;
        let mut flags = 0;
        add(&mut accumulator, None, &mut flags);
        assert_eq!(accumulator, 10);
    }

    #[test]
    fn add_zero_flag_sets_properly() {
        let mut accumulator = 0;
        let val = 0;
        let mut flags = 0;
        add(&mut accumulator, Some(val), &mut flags);
        assert_eq!(flags, 0b10000000);
    }

    #[test]
    fn add_half_carry_flag_sets_properly() {
        let mut accumulator = 0xf;
        let val = 1;
        let mut flags = 0;
        add(&mut accumulator, Some(val), &mut flags);
        assert_eq!(flags, 0b00100000);
    }

    #[test]
    fn add_overflow_handles_properly() {
        let mut accumulator = 0xf0;
        let val = 0xf0;
        let mut flags = 0;
        add(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 224);
        assert_eq!(flags, 0b00010000);
    }

    #[test]
    fn add_carry_to_zero_sets_both_flags() {
        let mut accumulator = 255;
        let val = 1;
        let mut flags = 0;
        add(&mut accumulator, Some(val), &mut flags);
        println!("{}", accumulator);
        assert_eq!(flags, 0b10110000);
    }

    #[test]
    fn adc_carry_applies_properly() {
        let mut accumulator = 40;
        let val = 1;
        // Only carry flag set
        let mut flags = 0b00010000;
        add_carry(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 42);
        // Flags should now clear.
        assert_eq!(flags, 0);
    }

    #[test]
    fn sub_simple_subtraction_calculates_correctly() {
        let mut accumulator = 45;
        let val = 3;
        let mut flags = 0;
        sub(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 42);
        // Also ensure only subtraction flag is set
        assert_eq!(flags, 0b01000000);
    }

    #[test]
    fn sub_missing_value_subtracts_accumulator_from_itself() {
        let mut accumulator = 5;
        let mut flags = 0;
        sub(&mut accumulator, None, &mut flags);
        assert_eq!(accumulator, 0);
    }

    #[test]
    fn sub_zero_flag_sets_properly() {
        let mut accumulator = 45;
        let val = 45;
        let mut flags = 0;
        sub(&mut accumulator, Some(val), &mut flags);
        assert_eq!(flags, 0b11000000);
    }

    #[test]
    fn sub_half_carry_flag_sets_properly() {
        let mut accumulator = 0x10;
        let val = 10;
        let mut flags = 0;
        sub(&mut accumulator, Some(val), &mut flags);
        assert_eq!(flags, 0b01100000);
    }

    #[test]
    fn sub_overflow_handles_properly() {
        let mut accumulator = 0x10;
        let val = 0x20;
        let mut flags = 0;
        sub(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 240);
        assert_eq!(flags, 0b01010000);
    }

    #[test]
    fn and_simple_and_calculates_correctly() {
        let mut accumulator = 0b1111;
        let val = 0b10101010;
        let mut flags = 0;
        and(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 0b1010);
        // Also ensure flags get baseline set
        assert_eq!(flags, 0b00100000);
    }

    #[test]
    fn and_missing_value_ands_accumulator_with_itself() {
        let mut accumulator = 0b1111;
        let mut flags = 0;
        and(&mut accumulator, None, &mut flags);
        assert_eq!(accumulator, 0b1111);
    }

    #[test]
    fn and_zero_flag_sets_properly() {
        let mut accumulator = 0xf;
        let val = 0xf0;
        let mut flags = 0;
        and(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 0);
        // Also ensure flags get baseline set
        assert_eq!(flags, 0b10100000);
    }

    #[test]
    fn or_simple_or_calculates_correctly() {
        let mut accumulator = 0b01010101;
        let val = 0b10101010;
        let mut flags = 0;
        or(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 0xff);
        // Also ensure flags get baseline set
        assert_eq!(flags, 0b00000000);
    }

    #[test]
    fn or_missing_value_ors_accumulator_with_itself() {
        let mut accumulator = 0b1111;
        let mut flags = 0;
        or(&mut accumulator, None, &mut flags);
        assert_eq!(accumulator, 0b1111);
    }

    #[test]
    fn or_zero_flag_sets_properly() {
        let mut accumulator = 0;
        let val = 0;
        let mut flags = 0;
        or(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 0);
        // Also ensure flags get baseline set
        assert_eq!(flags, 0b10000000);
    }

    #[test]
    fn cp_does_not_change_accumulator_value() {
        let accumulator = 30;
        let val = 3;
        let mut flags = 0;
        cp(accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 30);
        // Also ensure only subtraction flag is set
        assert_eq!(flags, 0b01000000);
    }

    #[test]
    fn xor_simple_xor_calculates_correctly() {
        let mut accumulator = 0b01010101;
        let val = 0b11111111;
        let mut flags = 0;
        xor(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 0b10101010);
        // Also ensure flags get baseline set
        assert_eq!(flags, 0b00000000);
    }

    #[test]
    fn xor_missing_value_xors_accumulator_with_itself() {
        let mut accumulator = 0b1111;
        let mut flags = 0;
        xor(&mut accumulator, None, &mut flags);
        assert_eq!(accumulator, 0);
    }

    #[test]
    fn xor_zero_flag_sets_properly() {
        let mut accumulator = 0;
        let val = 0;
        let mut flags = 0;
        xor(&mut accumulator, Some(val), &mut flags);
        assert_eq!(accumulator, 0);
        // Also ensure flags get baseline set
        assert_eq!(flags, 0b10000000);
    }

    #[test]
    fn cp_missing_value_tests_against_self() {
        let accumulator = 5;
        let mut flags = 0;
        cp(accumulator, None, &mut flags);
        assert_eq!(flags, 0b11000000);
    }

    #[test]
    fn cp_zero_flag_sets_properly() {
        let accumulator = 45;
        let val = 45;
        let mut flags = 0;
        cp(accumulator, Some(val), &mut flags);
        assert_eq!(flags, 0b11000000);
    }

    #[test]
    fn cp_half_carry_flag_sets_properly() {
        let accumulator = 0x10;
        let val = 10;
        let mut flags = 0;
        cp(accumulator, Some(val), &mut flags);
        assert_eq!(flags, 0b01100000);
    }

    #[test]
    fn cp_carry_flag_sets_properly() {
        let accumulator = 0x10;
        let val = 0x20;
        let mut flags = 0;
        cp(accumulator, Some(val), &mut flags);
        assert_eq!(flags, 0b01010000);
    }
}
