#include <stddef.h>
#include <stdint.h>
#include <assert.h>

uint64_t fast_crc64(void * src, size_t length);

uint64_t table_crc64(void * src, size_t length);

int main () {
	assert(fast_crc64("hello, world", 12) == 242422964339304333);
	return 0;
}
