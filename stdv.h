#ifndef STDV_H
#define STDV_H

#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

/*
 *  _____________________________________
 * / private fields; the api user is not \
 * \ meant to invoke any of these code   /
 *  -------------------------------------
 *         \   ^__^
 *          \  (oo)\_______
 *             (__)\       )\/\
 *                 ||----w |
 *                 ||     ||
 */
#define _STDV_INIT_CAPACITY 64
#define _STDV_GROWTH_FACTOR 2
#define _STDV_GET_HEADER(v) ((_stdv_header*) (v) - 1)

typedef struct {
	uint64_t cap;
	uint64_t len;
	bool is_mut;
} _stdv_header;

/* takes the pointer to the array and checks if it needs to be created (allocated) or resized.
 * In both cases it will return the new or same address position for the array.
 */
void *_stdv_needs_to_grow (void *vec, const size_t membsz)
{
	if (vec == NULL)
	{
		_stdv_header *header = (_stdv_header*) malloc(sizeof(_stdv_header) + _STDV_INIT_CAPACITY * membsz);
		header-> len = 0;
		header-> cap = _STDV_INIT_CAPACITY;
		header->is_mut = true;
		vec = (void*) (header + 1);
		return vec;
	}

	_stdv_header *header = _STDV_GET_HEADER(vec);
	if (header->len < header->cap)
	{
		return vec;
	}

	header->cap *= _STDV_GROWTH_FACTOR;
	header = (_stdv_header*) realloc(header, sizeof(*header) + header->cap * membsz);
	vec = (void*) (header + 1);
	return vec;
}

bool _stdv_free (void **address)
{
	if (*address == NULL)
	{
		return false;
	}

	printf("freeing: %p\n", *address);

	free(*address);
	*address = NULL;
	return true;
}

/*
 *  ___________________________
 * < actual api-user interface >
 *  ---------------------------
 *         \   ^__^
 *          \  (oo)\_______
 *             (__)\       )\/\
 *                 ||----w |
 *                 ||     ||
 */

#define stdv_empty(vec)           ((vec) ? (stdv_size(vec) == 0) : 1)
#define stdv_size(vec)            (_STDV_GET_HEADER(vec)->len)
#define stdv_capacity(vec)        (_STDV_GET_HEADER(vec)->cap)
#define stdv_is_mut(vec)          (_STDV_GET_HEADER(vec)->is_mut)
#define stdv_reserve(vec, newcap) 1
#define stdv_shrink(vec)          2
#define stdv_clear(vec)           3

#define stdv_put(vec, a)          ((vec) = _stdv_needs_to_grow(vec, sizeof(*vec)), (vec)[_STDV_GET_HEADER(vec)->len++] = (a))
#define stdv_put_ptr(vec, a)      ((vec) = _stdv_needs_to_grow(vec, sizeof(*vec)), (vec)[_STDV_GET_HEADER(vec)->len++] = (a), &((vec)[_STDV_GET_HEADER(vec)->len -  1]))

#define stdv_pop(vec)             ((vec)[--_STDV_GET_HEADER(vec)->len])
#define stdv_pop_ptr(vec)         (&((vec)[--_STDV_GET_HEADER(vec)->len]))

/* this macro does not modify the value of the original owner (the variable which
 * called malloc/calloc), therefore, once this macro is called, the original variable
 * will be useless
 */
#define stdv_pop_and_free(vec)    (_stdv_free((void*) (&(vec)[--_STDV_GET_HEADER(vec)->len])))

#endif
