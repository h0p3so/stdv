#include "stdv.h"

#include <stdio.h>

int main () {
	int *numbers = NULL;
	stdv_put(numbers, 0);

	printf("%ld\n", stdv_capacity(numbers));
	stdv_reserve(numbers, 32);
	printf("%ld\n", stdv_capacity(numbers));
	stdv_reserve(numbers, 65);
	printf("%ld\n", stdv_capacity(numbers));

	stdv_shrink(numbers);
	printf("%ld %p\n", stdv_capacity(numbers), numbers);

	stdv_free(numbers);
	return 0;
}
