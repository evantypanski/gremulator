pub fn add(accumulator: &mut u8, val: Option<u8>, flags: &mut u8) {
    let result: u16;
    let to_add = val.unwrap_or(*accumulator);

    result = *accumulator as u16 + to_add as u16;

    set_flags(flags,
              result & 0xff == 0,
              false,
              (((*accumulator & 0xf) + (to_add & 0xf)) & 0x10) == 0x10,
              result > 0xff);

    *accumulator = result as u8;
}

fn set_flags(flags: &mut u8, z: bool, n: bool, h: bool, c: bool) {
    *flags = ((z as u8) << 7)
        | ((n as u8) << 6)
        | ((h as u8) << 5)
        | ((c as u8) << 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_simple_add_adds_correctly() {
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
    fn add_carry_flag_sets_properly() {
        let mut accumulator = 0xf0;
        let val = 0xf0;
        let mut flags = 0;
        add(&mut accumulator, Some(val), &mut flags);
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
}
