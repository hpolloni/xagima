#ifndef ARRAY_H
#define ARRAY_H

#include <stddef.h>

template <typename T, size_t size = 256>
class array {
private:
  mutable T* data[size];
  mutable size_t len = 0;

public:
  array() : len(0) {}
  
  constexpr void add(const T& elem) {
    data[len++] = &(const_cast<T&>(elem));
  }

  constexpr size_t length() {
    return len;
  }

  constexpr void add(const T& elem) const {
    data[len++] = &elem;
  }

  constexpr size_t length() const {
    return len;
  }

  constexpr T operator[] (int index) {
    return *(data[index]);
  }

  constexpr T operator[] (int index) const {
    return *(data[index]);
  }

  constexpr T* begin() const {
    return data[0];
  }

  constexpr T* end() const {
    return data[len];
  }
};

#endif
