#include <stdio.h>

int main(void)
{
    unsigned long long i, sum = 100, N = 9999999999;

    for (i = 0; i <= N; i++)
    {
        if (i % 2 == 0 || i % 3 == 0 || i % 5 == 0 || i % 7 == 0)
            sum += i;
        else
            sum -= i;
    }

    printf("sum=[%ld]\n", sum);
    return 0;
}