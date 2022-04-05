use crate::opc;
use crate::{Cpu, SimpleMemory};
#[test]
fn test_no_repeated_instructions() {
    let _opc_arr = opc::init_opc_array();
}

#[test]
fn test_run() {
    let buf = [0xa2, 0x01, 0xe8, 0xe0, 0x10, 0xd0, 0xfb];

    let mem = SimpleMemory::from_rom(&buf);

    let mut cpu = Cpu::with_mem(mem);

    assert_eq!(cpu.pc(), 0);

    cpu.run(&mut |cpu: &Cpu<SimpleMemory>| {
        println!("x: {}, ir: {:?} z:{}", cpu.x(), cpu.ir(), cpu.z_flag());
        cpu.pc() as usize >= buf.len()
    })
    .unwrap();

    assert_eq!(cpu.x(), 0x10);
}
