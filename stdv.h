#ifndef STDV_H
#define STDV_H

#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#define STDV_STD_INIT_CAP           64
#define STDV_POS_OF_THE_LAST(v, o)  (stdv_size(v) - 1 - o)

#define stdv_create(membsz, cap)    (_stdv_create(membsz, cap))
#define stdv_free(v)                ((void) ((v) ? free((_stdv_header*)(v) - 1) : (void)0), (v)=NULL)

#define stdv_get(v, p)              ((v)[p])
#define stdv_get_or(v, p, or)       (((v) && (p) < stdv_size(v)) ? stdv_get(v, p) : (or))

#define stdv_front(v)               ((v)[0])
#define stdv_front_or(v, or)        (((v) && stdv_size(v) > 0) ? stdv_front(v) : (or))

#define stdv_back(v)                ((v)[stdv_size(v) - 1])
#define stdv_back_or(v, or)         (((v) && stdv_size(v) > 0) ? stdv_back(v) : (or))

#define stdv_pbeg(v)                (&(v)[0])
#define stdv_pbeg_or(v, or)         (((v) && stdv_size(v) > 0) ? stdv_pbeg(v) : (or))

#define stdv_pend(v)                (&(v)[_STDV_GET_HEADER(v)->len - 1])
#define stdv_pend_or(v, or)         (((v) && stdv_size(v) > 0) ? stdv_pend(v) : (or))

#define stdv_pget(v, p)             (&(v)[p])
#define stdv_pget_or(v, p, or)      (((v) && (p) < stdv_size(v)) ? stdv_pget(v, p) : (or))

#define stdv_empty(v)               ((v) ? (stdv_size(v) == 0) : 1)
#define stdv_size(v)                (_STDV_GET_HEADER(v)->len)
#define stdv_capacity(v)            (_STDV_GET_HEADER(v)->cap)
#define stdv_reserve(v, newcap)     ((v) = ((v) ? _stdv_try_reserve(v, sizeof(*v), newcap) : stdv_create(sizeof(*v), newcap)))
#define stdv_shrink(v)              ((v) = _stdv_shrink(v, sizeof(*v)))

#define stdv_pop_and_free(v)        (_stdv_free_element((void*) (&(v)[--_STDV_GET_HEADER(v)->len])))
#define stdv_pop_ptr(v)             (&((v)[--_STDV_GET_HEADER(v)->len]))
#define stdv_pop(v)                 ((v)[--_STDV_GET_HEADER(v)->len])

#define stdv_put(v, a)              ((v) = _stdv_may_grow(v, sizeof(*v)), (v)[_STDV_GET_HEADER(v)->len++] = (a))
#define stdv_put_ptr(v, a)          ((v) = _stdv_may_grow(v, sizeof(*v)), (v)[_STDV_GET_HEADER(v)->len++] = (a), &((v)[_STDV_GET_HEADER(v)->len -  1]))

#define stdv_erase(v, p) do {                                                      \
	memmove(v + p, v + p + 1, sizeof(*v) * (_STDV_GET_HEADER(v)->len - p - 1));    \
	_STDV_GET_HEADER(v)->len--;                                                    \
} while (0)

#define stdv_insert(v, p, a) do {                                                  \
	v = _stdv_may_grow(v, sizeof(*v));                                             \
	memmove(v + p + 1, v + p, sizeof(*v) * _STDV_GET_HEADER(v)->len - p);          \
	_STDV_GET_HEADER(v)->len++;                                                    \
	v[p] = a;                                                                      \
} while (0)

#define _STDV_GROWTH_FACTOR 2
#define _STDV_GET_HEADER(v) ((_stdv_header*) (v) - 1)

#define _STDV_MAY_BE_UNUSED __attribute__ ((unused))

typedef struct
{
	size_t cap;
	size_t len;
} _stdv_header;

static inline size_t _stdv_next_power2 (size_t n)
{
#ifdef STDV_ALWAYS_POWER_2
	if (n == 0)
	{
		return 1;
	}
	if ((n & (n - 1)) == 0)
	{
		return n;
	}

	n--;
	n |= n >> 1;
	n |= n >> 2;
	n |= n >> 4;
	n |= n >> 8;
	n |= n >> 16;
	return ++n;
#else
	return (n == 0) ? 1 : n;
#endif
}

_STDV_MAY_BE_UNUSED static void *_stdv_create (const size_t membsz, const size_t initcap)
{
	const size_t cap = _stdv_next_power2(initcap);
	_stdv_header *header = (_stdv_header*) malloc(sizeof(_stdv_header) + cap * membsz);
	header->len = 0;
	header->cap = cap;
	return ((void*) (header + 1));
}

_STDV_MAY_BE_UNUSED static void *_stdv_grow (void *vec, const size_t membsz, const size_t growth_factor)
{
	if (vec == NULL)
	{
		return vec;
	}
	_stdv_header *header = _STDV_GET_HEADER(vec);
	header->cap *= growth_factor;
	header = (_stdv_header*) realloc(header, sizeof(*header) + header->cap * membsz);
	vec = (void*) (header + 1);
	return vec;
}

_STDV_MAY_BE_UNUSED static void *_stdv_may_grow (void *vec, const size_t membsz)
{
	if (vec == NULL)
	{
		return _stdv_create(membsz, STDV_STD_INIT_CAP);
	}
	_stdv_header *header = _STDV_GET_HEADER(vec);
	if (header->len < header->cap)
	{
		return vec;
	}
	return _stdv_grow(vec, membsz, _STDV_GROWTH_FACTOR);
}

_STDV_MAY_BE_UNUSED static bool _stdv_free_element (void **address)
{
	if (*address == NULL)
	{
		return false;
	}

	free(*address);
	*address = NULL;
	return true;
}

_STDV_MAY_BE_UNUSED static void *_stdv_try_reserve (void *vec, size_t membsz, size_t newcap)
{
	_stdv_header *header = _STDV_GET_HEADER(vec);
	if (newcap < header->cap)
	{
		return vec;
	}

	header->cap = _stdv_next_power2(newcap);
	return _stdv_grow(vec, membsz, 1);
}

_STDV_MAY_BE_UNUSED static void *_stdv_shrink (void *vec, size_t membsz)
{
	if (vec == NULL)
	{
		return vec;
	}

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
