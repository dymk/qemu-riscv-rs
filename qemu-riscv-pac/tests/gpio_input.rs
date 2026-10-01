use qemu_riscv_pac::gpio::RegisterBlock;

#[test]
fn input_validity_has_its_own_read_only_register() {
    assert_eq!(core::mem::size_of::<RegisterBlock>(), 0x1c);
    for mask in [0, 1 << 2, 1 << 31, u32::MAX] {
        let words = [1 << 2, 0, 0, 0, 0, 0, mask];
        // SAFETY: RegisterBlock is seven u32 registers; words has that layout and lifetime.
        let registers = unsafe { &*words.as_ptr().cast::<RegisterBlock>() };
        assert_eq!(
            registers.input_initialized() as *const _ as usize - registers as *const _ as usize,
            0x18
        );
        assert_eq!(
            registers
                .input_initialized()
                .read()
                .input_initialized()
                .bits(),
            mask
        );
        assert_eq!(registers.in_().read().bits(), 1 << 2);
    }
}
