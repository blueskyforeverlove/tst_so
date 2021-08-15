#include <stdio.h>

extern unsigned int get_sum(unsigned int a, unsigned int b);

int main()
{
    unsigned int a = 1;
    unsigned int b = 2;
    printf("%u+%u=%u\n", a, b, get_sum(a,b));
    return 0;
}
