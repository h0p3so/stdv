#include "stdv.h"
#include <stdio.h>

void info (int *numbers)
{
	printf("cap: %ld\n", stdv_capacity(numbers));
	printf("len: %ld\n", stdv_size(numbers));
	for (size_t i = 0; i < stdv_size(numbers); i++)
	{
		printf("%ld) %d\n", i, numbers[i]);
	}
	printf("---------------\n");
}


int main ()
{
	int *numbers = stdv_create(sizeof(int), 3);

	stdv_put(numbers, 0);
	info(numbers);

	stdv_put(numbers, 5);
	info(numbers);

	stdv_insert(numbers, STDV_POS_OF_THE_LAST(numbers, 1), 69);
	info(numbers);
	return 0;
}
