#include <stdio.h>
#include <stdint.h>
#include "custom_header.h"

int main(void) {
    int32_t r1a = Math_abs_diff(7, 3);
    int32_t r1b = Math_abs_diff(3, 7);

    int32_t r2 = Client_sum_abs_diffs(7, 3, 2, 9);

    Custom_Struct_Point p;
    p.x = 5;
    p.y = 8;
    int32_t r3 = Geom_point_x(&p);

    printf("Example 1a: Math_abs_diff(7, 3)\n");
    printf("Expected: 4\n");
    printf("Got:      %d\n\n", r1a);

    printf("Example 1b: Math_abs_diff(3, 7)\n");
    printf("Expected: 4\n");
    printf("Got:      %d\n\n", r1b);

    printf("Example 2: Client_sum_abs_diffs(7, 3, 2, 9)\n");
    printf("Expected: 11\n");
    printf("Got:      %d\n\n", r2);

    printf("Example 3: Geom_point_x({x = 5, y = 8})\n");
    printf("Expected: 5\n");
    printf("Got:      %d\n", r3);

    return 0;
}