use crate::util::*;

#[test]
pub fn test_z_flag() {
    let mut cpu = new_cpu_empty_mem();
    cpu.update_z_flag_with(0x00);
    assert!(cpu.z_flag());
    assert!(!cpu.n_flag());
}

#[test]
pub fn test_n_flag() {
    let mut cpu = new_cpu_empty_mem();
    cpu.update_n_flag_with(0b10110000);
    assert!(cpu.n_flag());
    assert!(!cpu.z_flag());
}

#[test]
pub fn test_c_flag() {
    let mut cpu = new_cpu_empty_mem();
    cpu.update_c_flag_with(u8::MAX, 1);
    assert!(cpu.c_flag());
    cpu.update_c_flag_with(u8::MAX - 1, 1);
    assert!(!cpu.c_flag());
}

#[test]
pub fn test_v_flag() {
    let mut cpu = new_cpu_empty_mem();
    // (+64) + (+65) = -127 in two's complement logic, which is an overflow error
    cpu.update_v_flag_with(0b01000000, 0b01000001);
    assert!(cpu.v_flag());

    cpu.reset_flags();
    // (-1) + (-1) = -2 which is correct, so no flag
    let res: i8 = cpu.update_v_flag_with(0b11111111, 0b11111111) as i8;
    assert!(!cpu.v_flag());
    assert_eq!(res, -2);

    cpu.reset_flags();
    // (-64) + (-65) = +127 in two's complement logic, which is an overflow error
    cpu.update_v_flag_with(0b11000000, 0b10111111);
    assert!(cpu.v_flag());
}
