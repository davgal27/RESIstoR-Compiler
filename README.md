A compiler pipeline for **ResIR**, a small intermediate representation. Built in Rust for CPS2000 - Compiler Theory and Practice.

## Usage

```sh
cargo run -- <input.resir> <output.c>
```

Then compile the generated C:

```sh
gcc -std=c11 -Wall -Wextra -pedantic output.c -o output
```

## Example

Input (`abs_diff.resir`):

```
function Math::abs_diff(%a: i32, %b: i32) -> i32 {
    locals { %zero: i32; %d: i32; %is_neg: bool; %r: i32; }
    entry bb0;
    bb0:
        %zero   = const 0;
        %d      = bin sub %a, %b;
        %is_neg = bin lt %d, %zero;
        cjump %is_neg, bb1, bb2;
    bb1:
        %r = un neg %d;
        return %r;
    bb2:
        return %d;
}
```

Generated C:

```c
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
```
