#ifndef STDV_H
#define STDV_H

#include <stdint.h>
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
typedef struct {
	uint64_t cap;
	uint64_t len;
} _StdvHeader;

#define _STDV_INIT_CAP             64
#define _STDV_GROWTH_FACTOR        2
#define _STDV_GET_STDVHEADER(vec) ((_StdvHeader*) (vec) - 1)

void *_stdv_needs_to_grow (void *vec, size_t membsz) {
	if (vec == NULL) {
		_StdvHeader *header = (_StdvHeader*) malloc(_STDV_INIT_CAP * membsz + sizeof(_StdvHeader));
		header->len         = 0;
		header->cap         = _STDV_INIT_CAP;
		vec                 = (void*)(header + 1);
		return vec;
	}

	_StdvHeader *header = ((_StdvHeader*) (vec) - 1);
	if (header->len < header->cap) {
		return vec;
	}

	header->cap *= _STDV_GROWTH_FACTOR;
	header       = realloc(header, header->cap * membsz + sizeof(_StdvHeader));
	vec          = (void*) (header + 1);
	return vec;
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
#define stdv_put(vec, x)  ((vec) = _stdv_needs_to_grow(vec, sizeof(*vec)), (vec)[_STDV_GET_STDVHEADER(vec)->len++] = x)
#define stdv_len_u64(vec) ((uint64_t) _STDV_GET_STDVHEADER(vec)->len)
#define stdv_cap_u64(vec) ((uint64_t) _STDV_GET_STDVHEADER(vec)->cap)
#define stdv_pop(vec)     ((vec)[--_STDV_GET_STDVHEADER(vec)->len])

#endif
