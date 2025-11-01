use std::arch::asm;

fn main() {
    unsafe {
        asm!("nop");
    }

    let x: u64;
    unsafe {
        // 1. 如果寄存器分配有冲突，则在内联汇编的代码块内会有 store 和 reload
        // 2. reg 是适用于任何架构的通用寄存器
        asm!("mov {}, 8964", out(reg) x);
    }
    assert_eq!(x, 8964);
    println!("x: {x}");

    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o, // 这个位置就是参数{0}
            in(reg) i,  // 这个位置就是参数{1}
        );
    }
    assert_eq!(o, 8);

    let mut x: u64 = 6;
    unsafe {
        asm!("add {0}, 4", inout(reg) x);
    }
    assert_eq!(x, 10);

    let x: u64 = 6;
    let y: u64;
    unsafe {
        // 可以在使用 inout 的情况下，指定不同的输入和输出
        asm!("add {0}, 4", inout(reg) x => y);
    }
    assert_eq!(y, 10);

    let mut a: u64 = 4;
    let b: u64 = 4;
    unsafe {
        asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);
    }
    assert_eq!(a, 8);

    let a = 0xffff_1111u32;
    let c: u32;
    unsafe {
        // 显式寄存器操作数无法用于格式化字符串中，例如我们之前使用的 {}，只能直接在字符串中使用 eax
        asm!("mov cx, ax", in("eax") a, out("ecx") c);
    }
    println!("c: {:#x}", c);

    println!("mul result: {}", mul(89, 64));

    let mut name_buf = [0_u8; 12];
    println!("cpuid string: {}", cpuid(&mut name_buf));

    // Multiply x by 6 using shifts and adds
    // 汇编的模板字符串里可以使用别名
    let mut x: u64 = 4;
    unsafe {
        asm!(
            "mov {tmp}, {xxx}",
            "shl {tmp}, 1",
            "shl {xxx}, 2",
            "add {xxx}, {tmp}",
            xxx = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    assert_eq!(x, 4 * 6);
}

fn mul(a: u64, b: u64) -> u128 {
    let lo: u64;
    let hi: u64;

    unsafe {
        asm!(
            // The x86 mul instruction takes rax as an implicit input and writes
            // the 128-bit result of the multiplication to rdx:rax.
            "mul {}",
            in(reg) a,
            inout("rax") b => lo,
            out("rdx") hi
        );
    }

    ((hi as u128) << 64) + (lo as u128)
}

/// `name_buf`: three entries of four bytes each
fn cpuid(name_buf: &mut [u8; 12]) -> &str {
    let a: u32;

    // String is stored as ascii in ebx, edx, ecx in order
    // Because ebx is reserved, the asm needs to preserve the value of it.
    // So we push and pop it around the main asm.
    // (in 64 bit mode for 64-bit processors, 32-bit processors would use ebx)
    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov [rdi], ebx",
            "mov [rdi + 4], edx",
            "mov [rdi + 8], ecx",
            "pop rbx",
            // We use a pointer to an array for storing the values to simplify
            // the Rust code at the cost of a couple more asm instructions
            // This is more explicit with how the asm works however, as opposed
            // to explicit register outputs such as `out("ecx") val`
            // The *pointer itself* is only an input even though it's written behind
            in("rdi") name_buf.as_mut_ptr(),
            inout("eax") 0 => a,
            // 即使 ecx\edx 从没有被读取，我们依然需要告知编译器这个寄存器被修改过
            out("ecx") _,
            out("edx") _,
        );
    }

    println!("cpuid: {a}");
    core::str::from_utf8(name_buf).unwrap_or("")
}
