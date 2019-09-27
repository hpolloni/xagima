#ifndef OSTREAM_HPP
#define OSTREAM_HPP

#include <stdint.h>

template<class U>
class ostream {
private:
  U _under;
  mutable uint8_t _base = 10;
public:
  ostream(U _u) : _under(_u) {}

  constexpr auto write(char c) const noexcept {
    _under.write(c);
  }

  constexpr auto base() const noexcept {
    return _base;
  }

  constexpr auto hex() const noexcept {
    auto new_os = ostream{_under};
    new_os._base = 16;
    return new_os;
  }
};

template<typename U>
constexpr const ostream<U>& operator<<(const ostream<U>& os, uint64_t num) {
  if (num == 0) {
    os.write('0');
    return os;
  }
  char buf[20] = {};
  char* e = buf;
  char* s = buf;
  while(num) {
    uint8_t digit = num % os.base();
    if (digit <= 9) {
      *e = digit + '0';
    } else {
      *e = digit - 10 + 'A';
    }
    num /= os.base();
    e++;
  }
  *e = 0;
  while (e != s) {
    e--;
    os.write(*e);
  }
  return os;
}

template<typename U>
constexpr const ostream<U>& operator<<(const ostream<U>& os, uint16_t num) {
  os << static_cast<uint64_t>(num);
  return os;
}

template<typename U>
constexpr const ostream<U>& operator<<(const ostream<U>& os, uint32_t num) {
  os << static_cast<uint64_t>(num);
  return os;
}

template<typename U>
constexpr const ostream<U>& operator<<(const ostream<U>& os, int64_t num) {
  if(num < 0) {
    os.write('-');
    num = -num;
  }
  os << static_cast<uint64_t>(num);
  return os;
}

template<typename U>
constexpr const ostream<U>& operator<<(const ostream<U>& os, char arg) {
  os.write(arg);
  return os;
}

template<typename U>
constexpr const ostream<U>& operator<<(const ostream<U>& os, const char* arg) {
  while(*arg) {
    os.write(*arg);
    arg++;
  }
  return os;
}

#endif
