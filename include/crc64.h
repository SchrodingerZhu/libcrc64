#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

uint64_t fast_crc64(const void * src, size_t length);

uint64_t table_crc64(const void * src, size_t length);

#ifdef __cplusplus
}
#endif