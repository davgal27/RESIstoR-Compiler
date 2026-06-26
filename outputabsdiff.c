#include <stdint.h>
#include <stdbool.h>

int32_t Math_abs_diff(int32_t a, int32_t b) {
	int32_t zero;
	int32_t d;
	bool is_neg;
	int32_t r;

	goto bb0;

bb0:
    zero = 0;
    d = a - b;
    is_neg = d < zero;
	if (is_neg) goto bb1; else goto bb2;

bb1:
    r = -d;
	return r;

bb2:
	return d;
}

int main(void) {
    return 0;
}
