#ifndef STDV_H
#define STDV_H

#include <stdio.h>				// TODO remove
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

/*
 *  ___________________________
 * < actual user-api methods   >
 *  ---------------------------
 *         \   ^__^
 *          \  (oo)\_______
 *             (__)\       )\/\
 *                 ||----w |
 *                 ||     ||
 */

#define stdv_at(vec, at)          ((vec)[at])
#define stdv_front(vec)           ((vec)[0])
#define stdv_back(vec)            ((vec)[_STDV_GET_HEADER(vec)->len - 1])
#define stdv_pbeg(vec)            (&(vec)[0])
#define stdv_pend(vec)            (&(vec)[_STDV_GET_HEADER(vec)->len - 1])
#define stdv_pat(vec, at)         (&(vec)[at])

#define stdv_empty(vec)           ((vec) ? (stdv_size(vec) == 0) : 1)
#define stdv_size(vec)            (_STDV_GET_HEADER(vec)->len)
#define stdv_capacity(vec)        (_STDV_GET_HEADER(vec)->cap)
#define stdv_reserve(vec, newcap) ((vec) = _stdv_try_reserve(vec, sizeof(*vec), newcap))
#define stdv_shrink(vec)          ((vec) = _stdv_shrink(vec, sizeof(*vec)))

/* this macro does not modify the value of the original owner (the variable which
 * called malloc/calloc), therefore, once this macro is called, the original variable
 * will be useless since it will be pointing to an address was already freed
 */
#define stdv_pop_and_free(vec)    (_stdv_free_element((void*) (&(vec)[--_STDV_GET_HEADER(vec)->len])))
#define stdv_put(vec, a)          ((vec) = _stdv_may_grow(vec, sizeof(*vec), 0), (vec)[_STDV_GET_HEADER(vec)->len++] = (a))
#define stdv_put_ptr(vec, a)      ((vec) = _stdv_may_grow(vec, sizeof(*vec), 0), (vec)[_STDV_GET_HEADER(vec)->len++] = (a), &((vec)[_STDV_GET_HEADER(vec)->len -  1]))
#define stdv_pop(vec)             ((vec)[--_STDV_GET_HEADER(vec)->len])
#define stdv_pop_ptr(vec)         (&((vec)[--_STDV_GET_HEADER(vec)->len]))
#define stdv_insert(vec, at, a)   ((vec) = _stdv_may_grow(vec, sizeof(*vec), 1), memmove(vec + at + 1, vec + at, sizeof(*vec) * ++_STDV_GET_HEADER(vec)->len - at - 2), (vec)[at] = (a), (a))
#define stdv_erase(vec, at)       (memmove(vec + at, vec + at + 1, sizeof(*vec) * --_STDV_GET_HEADER(vec)->len - at))
#define stdv_free(vec)            ((void) ((vec) ? free((_stdv_header*)(vec) - 1) : (void)0), (vec)=NULL)

/*
 *  _____________________________________
 * / private fields; the user-api is not \
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

typedef struct
{
	size_t cap;
	size_t len;
} _stdv_header;

static inline size_t _stdv_next_power2 (size_t n)
{
	n--;
	n |= n >> 1;
	n |= n >> 2;
	n |= n >> 4;
	n |= n >> 8;
	n |= n >> 16;
	return ++n;
}

static void *_stdv_grow (void *vec, const size_t membsz, const size_t growth_factor)
{
	_stdv_header *header = _STDV_GET_HEADER(vec);
	header->cap *= growth_factor;
	header = (_stdv_header*) realloc(header, sizeof(*header) + header->cap * membsz);
	vec = (void*) (header + 1);
	return vec;
}

static void *_stdv_may_grow (void *vec, const size_t membsz, const size_t extramemb)
{
	if (vec == NULL) {
		_stdv_header *header = (_stdv_header*) malloc(sizeof(_stdv_header) + _STDV_INIT_CAPACITY * membsz);
		header-> len = 0;
		header-> cap = _STDV_INIT_CAPACITY;
		vec = (void*) (header + 1);
		return vec;
	}

	_stdv_header *header = _STDV_GET_HEADER(vec);
	if ((header->len + extramemb) < header->cap)
	{
		return vec;
	}
	return _stdv_grow(vec, membsz, _STDV_GROWTH_FACTOR);
}

bool _stdv_free_element (void **address)
{
	if (*address == NULL)
	{
		return false;
	}

	free(*address);
	*address = NULL;
	return true;
}

void *_stdv_try_reserve (void *vec, size_t membsz, size_t newcap)
{
	_stdv_header *header = _STDV_GET_HEADER(vec);
	if (newcap < header->cap)
	{
		return vec;
	}

	header->cap = _stdv_next_power2(newcap);
	return _stdv_grow(vec, membsz, 1);
}

void *_stdv_shrink (void *vec, size_t membsz)
{
	_stdv_header *oldhead = _STDV_GET_HEADER(vec);
	const size_t len = oldhead->len;
	const size_t cap = _stdv_next_power2(len);

	_stdv_header *newhead = malloc(cap * membsz + sizeof(_stdv_header));
	memcpy(newhead + 1, vec, oldhead->len * membsz);
	stdv_free(vec);

	newhead->len = len;
	newhead->cap = cap;

	return newhead + 1;
}

#endif
