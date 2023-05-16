#include <stdio.h>
#include <stdlib.h>
int main()
{

    char *s = "10000000000";
    char *endpoing;
    unsigned long long num = strtoull(s, &endpoing, 0);
    printf("%llu\n", num);
    unsigned long long i = 0;
    while (1)
    {
        i = i + 1;
        if (i > num)
            break;
    }
    printf("%llu\n", i);
}