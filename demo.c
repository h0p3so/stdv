#include "stdv.h"

#include <stdio.h>

struct Sheet {
	FILE *file;
	char *source;
	size_t length;
};

int main () {
	struct Sheet **sheets = NULL;

	struct Sheet *s = calloc(1, sizeof(struct Sheet));
	s->source = "some source";

	struct Sheet *ss = stdv_put(sheets, s);


	printf("before free\n");
	printf("%s: %s\n", "src", s->source);
	printf("%s: %s\n", "src", ss->source);
	printf("%s: %s\n", "src", sheets[0]->source);

	bool rm = stdv_pop_and_free(sheets);
	printf("ok rm: %d\n", rm);

	printf("s add : %p %d\n", s, s == NULL);
	printf("ss add: %p\n", ss);
	printf("ss add: %p\n", sheets[0]);


	return 0;
}
