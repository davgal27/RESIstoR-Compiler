#include <stdint.h>
#include "custom_header.h"

int32_t Geom_point_x(Custom_Struct_Point *p) {
	int32_t *fp;
	int32_t v;

	goto bb0;

bb0:
    fp = &p->x;
    v = *fp;
	return v;
}
