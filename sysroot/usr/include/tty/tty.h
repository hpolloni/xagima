#ifndef TTY_HPP
#define TTY_HPP

#include <stdint.h>
#include <stddef.h>
#include <utils/ostream.h>

namespace tty {
    class default_tty {
    public:
        void write(char c) const;
    };

    extern const ostream<default_tty> out;
};


#endif
