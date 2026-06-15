#define STDV_IMPLEMENTATION
#include "stdv.h"

#include <stdio.h>

int main () {
	int *numbers = NULL;
	int a = stdv_put(numbers, 5);

	printf("%d\n", numbers[0]);
	printf("%ld\n", stdv_len_u64(numbers));
	printf("%ld\n", stdv_cap_u64(numbers));

	puts("-*-");
	stdv_put(numbers, 543);
	printf("%d\n", numbers[1]);
	printf("%ld\n", stdv_len_u64(numbers));
	printf("%ld\n", stdv_cap_u64(numbers));

	puts("-*-");
	a = stdv_pop(numbers);
	printf("poped: %d\n", a);
	printf("%ld\n", stdv_len_u64(numbers));
	printf("%ld\n", stdv_cap_u64(numbers));

	return 0;
}
