
    @include "my.asm"
label: macro_rules! silly ( arg1: reg, arg2: imm, arg3: reg, arg4: mem) {
    mov %arg1, %arg2
    lea %arg2, %arg4
    .asciiz "Yap!"
}
    const memloc = 0xff
    lea r0, [(memloc + 3)]
add r0, (((( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33) + (( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33))
