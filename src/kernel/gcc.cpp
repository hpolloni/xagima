// Functions needed for C++ to function properly
#include <stdint.h>
#include <assert.h>
#include <stdlib.h>

extern "C" void __cxa_pure_virtual()
{
    // This needs to be here in order for pure virtual functions to work.
    // It is only called in case a pure virtual function call cannot be made.
    assert(false);
}

#if UINT32_MAX == UINTPTR_MAX
#define STACK_CHK_GUARD 0xDEAD574C
#else
#define STACK_CHK_GUARD 0xBADD00D574C70457 
#endif

uintptr_t __stack_chk_guard = STACK_CHK_GUARD;

extern "C" void __stack_chk_fail(void) {
	assert(false && "Stack smashing detected");
}

namespace __cxxabiv1 
{
    // TODO: correctly implement guarding a static variable
	
	/* The ABI requires a 64-bit type.  */
	__extension__ typedef int __guard __attribute__((mode(__DI__)));
 
	extern "C" int __cxa_guard_acquire (__guard *);
	extern "C" void __cxa_guard_release (__guard *);
	extern "C" void __cxa_guard_abort (__guard *);
 
	extern "C" int __cxa_guard_acquire (__guard *g) {
		return !*(char *)(g);
	}
 
	extern "C" void __cxa_guard_release (__guard *g) {
		*(char *)g = 1;
	}
 
	extern "C" void __cxa_guard_abort (__guard *) {
 
	}
}


void *operator new(size_t size) {
    return malloc(size);
}
 
void *operator new[](size_t size) {
    return malloc(size);
}
 
void operator delete(void *p) {
    free(p);
}
 
void operator delete[](void *p) {
    free(p);
}