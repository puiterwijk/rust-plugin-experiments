#include <stdio.h>
#include <stdint.h>

uint32_t activate() {
    printf("Guest C was activated!\n");

    return 42;
}
