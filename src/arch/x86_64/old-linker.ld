
ENTRY(start)

SECTIONS {
	 . = 1M;
	 .boot :
	 {
		/* Ensure multiboot loader is at the beginning */
	        *(.multiboot_header)
	}

	.text :
	{
		*(.text)
	}
}
