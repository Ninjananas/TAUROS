#define KERNEL_VA 0xffffffff80000000
#define KERNEL_VA_LOW 0x80000000
#define KERNEL_VA_HIGH 0xffffffff
#define TO_PA(a) (a - KERNEL_VA)

.global mb2_info_pa
.global p4_table

.section .data
.type mb2_info_pa, @object
mb2_info_pa:
	.long 0


.section .text
.code32
.global _start
.type _start, @function
_start:
	/* Store PA of multiboot2 information */
	movl %ebx, TO_PA(mb2_info_pa)

	/*  init basic stack */
	movl $TO_PA(stack_top), %esp

	/* Perform startup checks */
	call check_multiboot
	call check_long_mode

	call map_kernel
	call temporary_direct_map
	call enable_paging

	/* load 64-bit GDT */
	movl $TO_PA(gdt64pointer), %eax
	lgdt (%eax)

	/* load cs with gdt64code */
	movl $TO_PA(farjump64), %eax
	ljmp *(%eax)

error:
	movl $0x4f524f45, (0xb8000)
	movl $0x4f3a4f52, (0xb8004)
	movl $0x4f204f20, (0xb8008)
	movb %al, (0xb800a)
	hlt

check_multiboot:
	cmpl $0x36d76289, %eax
	jne .no_multiboot
	ret

.no_multiboot:
	movb $'0', %al
	jmp error

check_long_mode:
	//  test if extended processor info in available
	movl $0x80000000, %eax	# implicit argument for cpuid
	cpuid		# get highest supported argument
	cmpl $0x80000001, %eax	# it needs to be at least 0x80000001
	jb .no_long_mode	# if it's less, the CPU is too old for long mode

	//  use extended info to test if long mode is available
	movl $0x80000001, %eax	# argument for extended processor info
	cpuid		# returns various feature bits in ecx and edx
	test $(1 << 29), %edx	# test if the LM-bit is set in the D-register
	jz .no_long_mode	# If it's not set, there is no long mode
	ret

.no_long_mode:
	movb $'1', %al
	jmp error

enable_paging:
	// load P4 to cr3 register
	movl $TO_PA(p4_table), %eax
	movl %eax, %cr3

	// Enable PAE-flag in cr4
	movl %cr4, %eax
	orl $(1 << 5), %eax
	movl %eax, %cr4


	//  set the long mode bit in the EFER MSR (model specific register)
	mov $0xC0000080, %ecx
	rdmsr
	// Long Mode Enable
	orl $(1 << 8), %eax
	// No Execute Enable
	orl $(1 << 11), %eax
	wrmsr

	//  enable paging in the cr0 register
	movl %cr0, %eax
	// Paging
	orl $(1 << 31), %eax
	// Write protect
	orl $(1 << 16), %eax

	movl %eax, %cr0

	ret


init_p4_p3_511_510:
	pushl %eax
	pushl %ebx
	pushl %ecx
	pushl %edx

	/* MAP p4 -> p3_511 */
	// address of p3_511 table
	movl $TO_PA(p3_511), %edx
	// set present and writable
	orl $0b11, %edx
	// %eax = address of p4 table
	movl $TO_PA(p4_table), %eax
	// write entry for identity mapping
	movl %edx, (%eax)
	// get last entry (4088 = 511*8)
	// and write entry
	movl %edx, 4088(%eax)

	/* MAP p3_511 -> p2_511_510 */
	// address of p2_511_510 table
	movl $TO_PA(p2_511_510), %edx
	// set present and writable
	orl $0b11, %edx
	// %eax = address of p3_511 table
	movl $TO_PA(p3_511), %eax
	// write entry for identity mapping
	movl %edx, (%eax)
	// get (almost) last entry (4080 = 510*8)
	// and write entry
	movl %edx, 4080(%eax)

	popl %edx
	popl %ecx
	popl %ebx
	popl %eax
	ret

init_zero_p2:
	pushl %eax
	pushl %ebx
	pushl %ecx

	movl $511, %ecx
.init_zero_p2_loop:
	movl $8, %ebx
	imull %ecx, %ebx
	movl $TO_PA(p2_511_510), %eax
	addl %ebx, %eax
	movl $0, (%eax)
	loop .init_zero_p2_loop

	popl %ecx
	popl %ebx
	popl %eax
	ret

map_kernel_page:
	/*
	Only handles pages at levels 1 and 2
	parameters:
	@eax: physical address of page to map
	@ebx: additional (low ) bits to or to the page
	@ecx: additional (high) bits to or to the page
	*/
	pushl %eax
	pushl %ebx
	pushl %ecx
	pushl %edx

	pushl %ecx
	pushl %ebx

	//// Get p1 and p2 entries addresses
	movl %eax, %ebx
	addl $KERNEL_VA_LOW, %ebx
	movl %ebx, %ecx

	// ebx = index of p2 entry
	shrl $21, %ebx
	andl $511, %ebx

	// ecx = index of p1 entry
	shrl $12, %ecx
	andl $511, %ecx


	// ecx = address of p1 entry
	imull $8, %ecx
	addl $TO_PA(p1_tables_511_510), %ecx
	movl %ebx, %edx
	imull $4096, %edx
	addl %edx, %ecx

	// ebx = address of p2 entry
	imull $8, %ebx
	addl $TO_PA(p2_511_510), %ebx

	/// Init p2 entry if not set
	cmpl $0, (%ebx)
	jne .map_p1
	movl %ecx, %edx
	orl $0b11 ,%edx
	movl %edx, (%ebx)
.map_p1:
	/// Write low bits
	orl $1, %eax
	popl %ebx // low or

	orl %ebx, %eax
	movl %eax, (%ecx)

	/// Write high bits
	xorl %eax, %eax
	popl %ebx // high or
	orl %ebx, %eax
	movl %eax, 4(%ecx)

	popl %edx
	popl %ecx
	popl %ebx
	popl %eax
	ret

map_kernel_part:
	/*
	Maps a part of kernel, takes parameters:
	@eax: start (physical) address of part to map
	@ebx: end address of part to map
	@ecx: additional (low) bits to or to the pages
	@edx: additional (high) bits to or to the pages
	Maps PA 'a' to VA 'a + KERNEL_VA'
	Physical addresses of kernel are considered small enough
	to fit in 32-bit registers
	*/

	/// Count how many pages to create and place it in %ebx (instead of end address)
	pushl %edx
	pushl %ecx

	subl %eax, %ebx
	xchgl %eax, %ebx
	xorl %edx, %edx
	movl $4096, %ecx
	idivl %ecx
	// test remainder of division in edx
	// the number of pages is in eax
	testl %edx, %edx
	jnz .error_align
	// Number of pages now in ebx, and start address in eax
	xchgl %eax, %ebx
	// restore registers
	popl %ecx
	popl %edx

	xchgl %ebx, %ecx
	xchgl %ecx, %edx
	// Counter in %edx, args correctly placed for call to map_kernel_page
.map_part_loop:
	dec %edx
	call map_kernel_page
	addl $4096, %eax
	testl %edx, %edx
	jnz .map_part_loop
	ret


map_kernel:
	/// Init p4 and p3_511
	call init_p4_p3_511_510

	/// Set whole p2_511_510 table to 0
	call init_zero_p2

	/// Map text section
	movl $TO_PA(kernel_start), %eax
	movl $TO_PA(kernel_text_end), %ebx
	movl $0b1, %ecx // present
	movl $0, %edx // Not no-execute
	call map_kernel_part

	/// Map rodata section
	movl $TO_PA(kernel_text_end), %eax
	movl $TO_PA(kernel_ro_end), %ebx
	movl $0b1, %ecx // present
	movl $0x80000000, %edx // No execute
	call map_kernel_part

	/// Map rw sections
	movl $TO_PA(kernel_ro_end), %eax
	movl $TO_PA(kernel_end), %ebx
	movl $0b11, %ecx // present and writable
	movl $0x80000000, %edx // No execute
	call map_kernel_part

	ret

.error_align:
	movb $'2', %al
	jmp error

temporary_direct_map:
	// present + writable + huge
	movl $0b10000011, TO_PA(p3_511 + 2032)
	// No exec
	movl $0x80000000, TO_PA(p3_511 + 2036)
	ret

.section .rodata
.align 4096
gdt64:
	.quad 0			#zero entry
gdt64code:
	.quad ((1<<43) | (1<<44) | (1<<47) | (1<<53)) #code segment
gdt64pointer:
	.word gdt64pointer - gdt64 - 1
	.quad TO_PA(gdt64)
farjump64:
	.long TO_PA(long_mode_start)
	.word gdt64code - gdt64


.section .bss
.align 16
stack_bottom:
	/*
	Stack should be located directly below rodata,
	so overflows result in page fault
	*/
	.skip 65536
stack_top:

.align 4096
p4_table:
	.skip 4096
p3_511:
	.skip 4096
p2_511_510:
	.skip 4096
p1_tables_511_510:
	.skip 4096 * 512
