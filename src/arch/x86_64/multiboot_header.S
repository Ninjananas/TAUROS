.section .multiboot_header
.align 64
header_start:
	.long 0xe85250d6	# magic number (multiboot 2)
	.long 0		# architecture 0 (protected mode i386)
	.long header_end - header_start # header length
	##  checksum
	.long -(0xe85250d6 + (header_end - header_start))
	
	##  insert optional multiboot tags here
	
	##  required end tag
	.word 0    		# type
	.word 0    		# flags
	.long 8    		# size
header_end:	
