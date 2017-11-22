ENTRY(start)

SECTIONS {
	. = 1M;

	.boot :
	{
		/* Ensures that the multiboot header is in the beginning */
		KEEP(*(.multiboot_header))
	}

	.text :
	{
		*(.text)
	}
}
