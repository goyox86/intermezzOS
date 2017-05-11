ENTRY(start)

SECTIONS {
        . = 1m;

        .boot :
        {
                /*making sure the multiboot headeris at the beggining */
                *(.multiboot_header)
        }

        .text :
        {
                *(.text)
        }
}
