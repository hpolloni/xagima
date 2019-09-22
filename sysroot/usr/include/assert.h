#ifndef ASSERT_H
#define ASSERT_H

#include <tty/tty.h>
#include <stdlib.h>

#define __string(x) #x
#define __s(x) __string(x)

#define assert(x) if (!(x)) { tty::out << "Assertion failed: " # x " " __FILE__ ":"  __s(__LINE__); abort(); }

#endif
