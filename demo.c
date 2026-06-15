#define STDV_IMPLEMENTATION
#include "stdv.h"

#include <stdio.h>

int main () {
	int *numbers = NULL;
	int a = stdv_put(numbers, 5);
	printf("is empty: %d\n", stdv_is_empty(numbers));

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

	puts("-*-");
	stdv_put(numbers, -69);
	printf("%d\n", stdv_get(numbers, 1));
	printf("%ld\n", stdv_len_u64(numbers));
	printf("%ld\n", stdv_cap_u64(numbers));


	puts("-*-");
	stdv_put(numbers, -9);

	printf("beg: %d\n", stdv_beg(numbers));
	printf("beg: %d\n", stdv_get(numbers, 1));
	printf("end: %d\n", stdv_end(numbers));

	puts("-*-");
	a = stdv_pop(numbers);
	a = stdv_pop(numbers);
	a = stdv_pop(numbers);
	printf("len: %ld\n", stdv_len_u64(numbers));
	printf("cap: %ld\n", stdv_cap_u64(numbers));

	puts("-----------------------------------------");

	printf("is empty: %d\n", stdv_is_empty(numbers));
	a = stdv_put(numbers, 1);
	printf("is empty: %d\n", stdv_is_empty(numbers));
	printf("%d\n", numbers[0]);
	printf("len: %ld\n", stdv_len_u64(numbers));
	printf("cap: %ld\n", stdv_cap_u64(numbers));
	puts("shrinking");
	stdv_shrink(numbers);
	printf("len: %ld\n", stdv_len_u64(numbers));
	printf("cap: %ld\n", stdv_cap_u64(numbers));

	return 0;
}
