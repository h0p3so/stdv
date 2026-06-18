#include "stdv.h"
#include <stdio.h>

int main ()
{
	int *numbers = stdv_create(sizeof(int), 64);

	stdv_put(numbers, 0);
	stdv_put(numbers, 1);
	stdv_put(numbers, 2);
	stdv_put(numbers, 3);
	stdv_put(numbers, 4);
	stdv_put(numbers, 5);
	stdv_put(numbers, 6);
	stdv_put(numbers, 7);
	stdv_put(numbers, 8);
	stdv_put(numbers, 9);

	for (size_t i = 0; i < stdv_size(numbers); i++)
		printf("%ld) %d\n", i + 1,numbers[i]);
	puts("----");

	stdv_insert(numbers, 2, 69);
	for (size_t i = 0; i < stdv_size(numbers); i++)
		printf("%ld) %d\n", i + 1, numbers[i]);

	return 0;
}
