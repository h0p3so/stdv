#include "stdvec.h"

#include <stdio.h>

int main () {
	int *vec = NULL;
	STDV_PUSH(vec, 43);

	printf("value at 0: %d\n", vec[0]);
	printf("len: %ld\n", STDV_LEN(vec));
	printf("cap: %ld\n", STDV_CAP(vec));

	
	/*printf("-----------------------------------------------\n");
	STDVEC_PUSH(vec, 44);
	printf("value at 1: %d\n", STDVEC_GET(vec, 1));
	printf("len: %ld\n", STDVEC_LEN(vec));
	printf("cap: %ld\n", STDVEC_CAP(vec));*/

	return 0;
}
