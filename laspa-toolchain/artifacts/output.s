	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 13, 0
	.section	__TEXT,__literal8,8byte_literals
	.p2align	3, 0x0                          ## -- Begin function main
LCPI0_0:
	.quad	0x405ec00000000000              ## double 123
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_main
	.p2align	4, 0x90
_main:                                  ## @main
	.cfi_startproc
## %bb.0:                               ## %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movsd	LCPI0_0(%rip), %xmm0            ## xmm0 = mem[0],zero
	callq	_collatz
	popq	%rax
	retq
	.cfi_endproc
                                        ## -- End function
	.section	__TEXT,__literal8,8byte_literals
	.p2align	3, 0x0                          ## -- Begin function collatz
LCPI1_0:
	.quad	0x3ff0000000000000              ## double 1
LCPI1_1:
	.quad	0x4000000000000000              ## double 2
LCPI1_2:
	.quad	0x4008000000000000              ## double 3
LCPI1_3:
	.quad	0x0000000000000000              ## double 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_collatz
	.p2align	4, 0x90
_collatz:                               ## @collatz
	.cfi_startproc
## %bb.0:                               ## %entry
	pushq	%rax
	.cfi_def_cfa_offset 16
	movsd	%xmm0, (%rsp)
	jmp	LBB1_1
	.p2align	4, 0x90
LBB1_4:                                 ## %else_block
                                        ##   in Loop: Header=BB1_1 Depth=1
	movsd	(%rsp), %xmm0                   ## xmm0 = mem[0],zero
	mulsd	LCPI1_2(%rip), %xmm0
	addsd	LCPI1_0(%rip), %xmm0
LBB1_5:                                 ## %end_if
                                        ##   in Loop: Header=BB1_1 Depth=1
	movsd	%xmm0, (%rsp)
	movsd	(%rsp), %xmm0                   ## xmm0 = mem[0],zero
	callq	_print_f64
LBB1_1:                                 ## %loop_cond
                                        ## =>This Inner Loop Header: Depth=1
	movsd	LCPI1_1(%rip), %xmm1            ## xmm1 = mem[0],zero
	movsd	(%rsp), %xmm0                   ## xmm0 = mem[0],zero
	ucomisd	LCPI1_0(%rip), %xmm0
	jbe	LBB1_6
## %bb.2:                               ## %loop_body
                                        ##   in Loop: Header=BB1_1 Depth=1
	movsd	(%rsp), %xmm0                   ## xmm0 = mem[0],zero
	callq	_fmod
	ucomisd	LCPI1_3(%rip), %xmm0
	jne	LBB1_4
	jp	LBB1_4
## %bb.3:                               ## %then_block
                                        ##   in Loop: Header=BB1_1 Depth=1
	movsd	(%rsp), %xmm0                   ## xmm0 = mem[0],zero
	divsd	LCPI1_1(%rip), %xmm0
	jmp	LBB1_5
LBB1_6:                                 ## %loop_end
	movsd	(%rsp), %xmm0                   ## xmm0 = mem[0],zero
	popq	%rax
	retq
	.cfi_endproc
                                        ## -- End function
.subsections_via_symbols
