#ifndef CUSTOM_HEADER_H
#define CUSTOM_HEADER_H
#include <stdint.h>

typedef struct {
    int32_t x;
    int32_t y;
} Custom_Struct_Point;

int32_t Math_abs_diff(int32_t a, int32_t b);
int32_t Client_sum_abs_diffs(int32_t a, int32_t b, int32_t c, int32_t d);
int32_t Geom_point_x(Custom_Struct_Point *p);

#endif