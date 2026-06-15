#ifndef STDV_H
#define STDV_H

/*
 * cvec_pop(v)             — remove and return last element
 * cvec_reserve(v, n)      — pre-allocate space for n elements
 * cvec_free(v)            — release all memory
 */

#include <stdint.h>
#include <stdlib.h>
#include <assert.h>

typedef struct {
	uint64_t cap;
	uint64_t len;
} _stdvm;

#define STDV_INIT_CAP      64
#define STDV_GROWTH_FACTOR 2

#define STDV_PUSH(vec, a) do {                                              \
	if (vec == NULL) {                                                      \
		_stdvm *m = malloc(STDV_INIT_CAP * sizeof(*vec) + sizeof(_stdvm));  \
		m->cap = STDV_INIT_CAP;                                             \
		m->len = 0;                                                         \
		vec = (void*) (m + 1);                                              \
	}                                                                       \
	_stdvm *m = (_stdvm*) (vec) - 1;                                        \
	if (m->len >= m->cap) {                                                 \
		m->cap *= STDV_GROWTH_FACTOR;                                       \
		m = (_stdvm*) realloc(m, sizeof(*vec) * m->cap + sizeof(_stdvm));   \
		vec = (void*) (m + 1);                                              \
	}                                                                       \
	(vec)[m->len++] = (a);                                                  \
} while (0)

#define STDV_GET(vec, at, dest) do {                                        \
	assert(at < STDV_LEN(vec));                                             \
	dest = (vec)[at];                                                       \
} while (0)

#define STDV_GET_AS_PTR(vec, at, ptr) do {                                  \
	assert(at < STDV_LEN(vec));                                             \
	dest = &((vec)[at]);                                                    \
} while (0)

#define STDV_GET_OR(vec, at, dest, or) do {                                 \
	if (at < STDV_LEN(vec)) { dest = (vec)[at]; }                           \
	else { dest = or; }                                                     \
} while (0)

#define STDV_POP(vec) do {                                                  \
	assert(STDV_LEN(vec) > 0);                                              \
	_stdvm *m = (_stdvm*) (vec) - 1;                                        \
	m->len--;                                                               \
} while (0)

#define STDV_LEN(vec) ((_stdvm*)(vec) - 1)->len
#define STDV_CAP(vec) ((_stdvm*)(vec) - 1)->cap

#endif

