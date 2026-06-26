#ifndef CUSTOMHEADER_H
#define CUSTOMHEADER_H

#include <stdint.h>
#include <stdbool.h>

/* For extern type Custom::Struct::Point */
typedef struct Custom_Struct_Point {
    int32_t x;
    int32_t y;
} Custom_Struct_Point;

/* Helper used by generated call example */
static inline int32_t Math_abs_diff(int32_t a, int32_t b) {
    int32_t d = a - b;
    if (d < 0) {
        return -d;
    }
    return d;
}

#endif