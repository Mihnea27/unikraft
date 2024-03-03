#include <uk/rsalloc.h>
#include <uk/alloc_impl.h>
#include <uk/arch/limits.h>
#include <stddef.h>
#include <string.h>

#define MAX_ALLOCATIONS 128

struct Allocation {
	size_t used;
	size_t start;
	size_t size;
};

struct RsAlloc {
	struct Allocation allocations[MAX_ALLOCATIONS];
	size_t heap_start;
	size_t heap_end;
};

extern void* rsalloc_malloc(struct RsAlloc* alloc, size_t size);
extern void rsalloc_free(struct RsAlloc* alloc, void* ptr);
extern void rsalloc_init(struct RsAlloc* alloc, void* heap_base, size_t size);


static void *uk_rsalloc_malloc(struct uk_alloc* a, size_t size)
{
	struct RsAlloc* b;
	b = (struct RsAlloc*)&a->priv;
	return rsalloc_malloc(b, size);
}

static void uk_rsalloc_free(struct uk_alloc* a, void* ptr)
{
	struct RsAlloc* b;
	b = (struct RsAlloc*)&a->priv;
	rsalloc_free(b, ptr);
}


struct uk_alloc* uk_rsalloc_init(void* base, size_t len)
{
	struct uk_alloc* a;
	struct RsAlloc* b;

	size_t metalen;

	if (len < __PAGE_SIZE) {
		return NULL;
	}

	metalen = sizeof(*a) + sizeof(*b);

	if (metalen > len) {
		uk_pr_err("Not enough space");
		return NULL;
	}

	a = (struct uk_alloc*)base;
	b = (struct RsAlloc*)&a->priv;

	memset(a, 0, metalen);

	rsalloc_init(b, base + metalen, base + len);
	uk_pr_info("Initialise rsallocator\n");
	uk_alloc_init_malloc_ifmalloc(a, uk_rsalloc_malloc, uk_rsalloc_free,
					NULL, NULL, NULL);
	return a;
}
