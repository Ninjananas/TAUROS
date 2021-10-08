#define VA_OFFSET_KERNEL 0xffffffff80000000
#define VA_OFFSET_DMAP (VA_OFFSET_KERNEL - (0x40000000 * 256)) // 256 GiB below kernel

.global long_mode_start
.global DIRECT_MAP_OFFSET
.global rdrand

.section .data
.type DIRECT_MAP_OFFSET, @object
DIRECT_MAP_OFFSET:
	.quad VA_OFFSET_DMAP

.section .text
.code64
long_mode_start:
	// load 0 into all data segment registers
	xorw %ax, %ax
	movw %ax, %ss
	movw %ax, %ds
	movw %ax, %es
	movw %ax, %fs
	movw %ax, %gs

	// Jump in virtual address space
	movq $VA_OFFSET_KERNEL, %rax
	addq %rax, %rsp
	movabsq $.to_virtual, %rax
	jmp *%rax
.to_virtual:

	// Remove identity mapping
	movq %cr3, %rbx
	movq (%rbx), %rax
	movq $VA_OFFSET_KERNEL, %rcx
	addq %rcx, %rax
	addq %rcx, %rbx
	andq $0xfffffffffffff000, %rax
	movq $0, (%rax)
	movq $0, (%rbx)

	// call kernel
	call kernel_main

	hlt

rdrand:
	// test if rdrand instruction is available, if not return 1
	xorq %rax, %rax
	xorq %rcx, %rcx
	movl $1, %eax
	cpuid
	shrl $30, %ecx
	andl $1, %ecx
	jnz .ok

	movq $1, %rax
	ret
.ok:
	// number of tries
	movq $100, %rcx
retry:
	rdrand %rax
	jnc .done
	loop retry
.fail:
	// If rdrand keeps failing, return 0
	movq $0, %rax
.done:
	ret
