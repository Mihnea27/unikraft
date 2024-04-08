#ifndef __UKRSALLOC_H__
#define __UKRSALLOC_H__

#include <uk/alloc.h>
#include <stddef.h>

struct uk_alloc* uk_rsalloc_init(void* base, size_t len);


#endif // __UK_RSALLOC_H__
