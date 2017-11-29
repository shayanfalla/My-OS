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
		*(.text .text.*)
	}

	.rodata : 
	{
		*(.rodata .rodata.*)
	}

	.data.rel.ro : 
	{
		*(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro*)
	}
}
