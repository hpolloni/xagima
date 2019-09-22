#ifndef ERRNO_H
#define ERRNO_H

#define ENOMEM 12
#define EINVAL 22

// FIXME: this shouldn't be a global variable, but it's currently only used in dlmalloc
extern unsigned int errno;

#endif
