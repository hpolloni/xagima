#include <string.h>

extern "C" void* memmove(void* dstptr, const void* srcptr, size_t size) {
  unsigned char* dst = (unsigned char*) dstptr;
  const unsigned char* src = (const unsigned char*) srcptr;
  if (dst < src) {
    for (size_t i = 0; i < size; i++)
      dst[i] = src[i];
  } else {
    for (size_t i = size; i != 0; i--)
      dst[i-1] = src[i-1];
  }
  return dstptr;
}

extern "C" void* memcpy(void* dstptr, const void* srcptr, size_t size) {
  unsigned char* dst = (unsigned char*) dstptr;
  const unsigned char* src = (const unsigned char*) srcptr;
  for (size_t i = 0; i < size; i++)
    dst[i] = src[i];
  return dstptr;
}

extern "C" void* memset(void* bufptr, int value, size_t size) {
  unsigned char* buf = (unsigned char*) bufptr;
  for (size_t i = 0; i < size; i++)
    buf[i] = (unsigned char) value;
  return bufptr;
}