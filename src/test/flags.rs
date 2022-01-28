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
