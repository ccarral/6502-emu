use crate::util;

#[test]
fn test_stack() {
    let mut cpu = util::new_cpu_empty_mem();
    cpu.stack_push(0x80);
    assert_eq!(cpu.stack_pop(), 0x80);

    cpu.stack_push(0x90);
    cpu.stack_push(0xFE);
    cpu.stack_push(0x69);

    assert_eq!(cpu.stack_pop(), 0x69);
    assert_eq!(cpu.stack_pop(), 0xFE);
    assert_eq!(cpu.stack_pop(), 0x90);
}
