use crate::opc;
use crate::{Cpu, SimpleMemory};
#[test]
fn test_no_repeated_instructions() {
    let _opc_arr = opc::init_opc_array();
}

#[test]
fn test_run() {
    let buf = [0xa2, 0x03, 0xCA, 0xD0, 0xFD];
    let mem = SimpleMemory::from_rom(&buf);

    let mut cpu = Cpu::with_mem(mem);

    assert_eq!(cpu.pc(), 0);

    cpu.run(&mut |cpu: &Cpu<SimpleMemory>| {
        println!("{cpu}");
        println!("{:#40x?}", &cpu.mem.inner[0x04e5..0x04e5 + 3]);
        cpu.pc() as usize >= buf.len()
        // false
    })
    .unwrap();

    assert_eq!(cpu.x(), 0x00);
}
