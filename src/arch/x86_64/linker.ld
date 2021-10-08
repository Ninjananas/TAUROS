KERNEL_VA = 0xffffffff80000000;

PAGE_SIZE = 4K;
ENTRY(_start)
OUTPUT_FORMAT(elf64-x86-64)

SECTIONS {
	 /* Putting sections at PA 2MiB, though 1MiB is a conventional place
	 for kernels to be loaded at by the bootloader.*/

	 . = KERNEL_VA + 2M; /* end of VA - 2GiB + 2MiB */

	 kernel_start = .;

	 /* .text section comes first */
	 .text ALIGN(PAGE_SIZE) : AT (ADDR (.text) - KERNEL_VA)
	 {
	     *(.multiboot_header)
	     *(.text)
	     *(.text.*)
	 }

	 . = ALIGN(PAGE_SIZE);
	 kernel_text_end = .;

	/* Read-only data */
	 .rodata ALIGN(PAGE_SIZE) : AT (ADDR (.rodata) - KERNEL_VA)
	 {
	     *(.rodata)
	     *(.rodata.*)
	 }

	 . = ALIGN(PAGE_SIZE);
	 kernel_ro_end = .;

	/* Read-write data (uninitialised) and stack */
	 .bss ALIGN(PAGE_SIZE) : AT (ADDR (.bss) - KERNEL_VA)
	 {
	     *(.bss)
	     *(.bss.*)
	 }

	/* Read-write data (initialised) */
	 .data ALIGN(PAGE_SIZE) : AT (ADDR (.data) - KERNEL_VA)
	 {
	     *(.data)
	     *(.data.*)
	 }

	 . = ALIGN(PAGE_SIZE);
	 kernel_end = .;
}
