#include <stdint.h>
#include "customheader.h"

int32_t Client_sum_abs_diffs(int32_t a, int32_t b, int32_t c, int32_t d) {
	int32_t dx;
	int32_t dy;
	int32_t r;

	goto bb0;

bb0:
    dx = Math_abs_diff(a, b);
    dy = Math_abs_diff(c, d);
    r = dx + dy;
	return r;
}

int main(void) {
    return 0;
}
