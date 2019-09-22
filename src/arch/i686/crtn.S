.section .init
	/* crtend.o's .init section here. */
	popl %ebp
	ret

.section .fini
	/* crtend.o's .fini section here. */
	popl %ebp
	ret
