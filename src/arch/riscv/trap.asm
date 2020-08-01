.option norvc
.altmacro

.macro save i, base
    sd x\i, ((\i)*8)(\base)
.endm

.macro rest i, base
    ld x\i, ((\i)*8)(\base)
.endm

.text
.global trap_vector
trap_vector:
    # mscratch contains a pointer to a trap::Context
    # NOTE: If there is an exception IN the kernel we have a problem. However this should never happen...
    # TODO: Make sure that this ACTUALLY never happens OR add runtime check here...
    csrrw x31, mscratch, x31

    # Save all registers to the context struct
    # NOTE: x0 is hardwired to zero, so we do not have to save it. In fact we use that slot to save the kernel stack pointer
    .set    i, 1
	.rept   30
		save   %i, x31
		.set   i, i+1
	.endr

    # store PC, original x31 and mstatus to the context
    csrr t0, mscratch
    csrr t1, mepc
    csrr t2, mstatus
    sd t0, 31*8(x31)
    sd t1, 32*8(x31)
    sd t2, 33*8(x31)

    # load kernel SP from the struct
    ld sp, 0*8(x31)

    # restore calle-saved registers (and GP, TP and RA)
    ld s0,   0*8(sp)
    ld s1,   1*8(sp)
    ld s2,   2*8(sp)
    ld s3,   3*8(sp)
    ld s4,   4*8(sp)
    ld s5,   5*8(sp)
    ld s6,   6*8(sp)
    ld s7,   7*8(sp)
    ld s8,   8*8(sp)
    ld s9,   9*8(sp)
    ld s10, 10*8(sp)
    ld s11, 11*8(sp)
    ld gp,  12*8(sp)
    ld tp,  13*8(sp)
    ld ra,  14*8(sp)

    add sp, sp, 15*8

    ret


# This takes one argument in a0: *mut trap::Context
.global trap_context_run
trap_context_run:
    # Save calle-saved registers (and GP, TP and RA)
    add sp, sp, -15*8

    sd s0,   0*8(sp)
    sd s1,   1*8(sp)
    sd s2,   2*8(sp)
    sd s3,   3*8(sp)
    sd s4,   4*8(sp)
    sd s5,   5*8(sp)
    sd s6,   6*8(sp)
    sd s7,   7*8(sp)
    sd s8,   8*8(sp)
    sd s9,   9*8(sp)
    sd s10, 10*8(sp)
    sd s11, 11*8(sp)
    sd gp,  12*8(sp)
    sd tp,  13*8(sp)
    sd ra,  14*8(sp)

    # Save kernel SP to the trap::Context struct.
    sd sp, 0*8(a0)

    # Load pc and status from structure
    ld t0, 32*8(a0)
    ld t1, 33*8(a0)
    csrw mepc, t0
    csrw mstatus, t1

    # Save the struct pointer in mscratch
    csrw mscratch, a0

    # Restore the tasks registers. The register holding the pointer must be restored last
    mv x31, a0
    .set    i, 1
	.rept   31
		rest   %i, x31
		.set   i, i+1
	.endr

    mret