
#include <assert.h>
#include <limits.h>
#include <stdio.h>

#include <ncurses.h>

int main(void)
{
    if (sizeof(chtype)*CHAR_BIT == 64) {
        puts("cargo:rustc-cfg=feature=\"wide_chtype\"");
    } else {
        /* We only support 32-bit and 64-bit chtype. */
        assert(sizeof(chtype)*CHAR_BIT == 32 && "unsupported size for chtype");
    }

#if defined(NCURSES_MOUSE_VERSION) && NCURSES_MOUSE_VERSION == 1
	puts("cargo:rustc-cfg=feature=\"mouse_v1\"");
#endif
    return 0;
}
    