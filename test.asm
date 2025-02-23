
    @include "my.asm"
    mov r0, 'a'
label: macro_rules! silly ( arg1: reg, arg2: imm, arg3: reg, arg4: mem) {
    mov %arg1, %arg2
    lea %arg3, %arg4
    .asciiz "Yap!\y"
}
    const memloc = 0xff
    silly!(r0, 3, r4, [(memloc + 3)])
    lea r0, [(memloc + 3)]
add r0, (((( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33) + (( ( 6 * 3 ) + (3 + 3) * 5) & ( 6 * 3 ) + (3 + 3) * 5) * 2 + (3 * 4 + 2) & 33))
    const c = 23
