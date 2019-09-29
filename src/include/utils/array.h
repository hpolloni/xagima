#ifndef ARRAY_H
#define ARRAY_H

#include <stddef.h>

template <typename T, size_t size = 64>
class array {
private:
  union holder {
    T val;
    holder() {}
  };

  mutable holder data[size];
  mutable size_t len = 0;

public:
  array() : len(0) {}
  
  constexpr void add(const T& elem) {
    data[len++].val = elem;
  }

  constexpr size_t length() {
    return len;
  }

  constexpr void add(const T& elem) const {
    data[len++].val = elem;
  }

  constexpr size_t length() const {
    return len;
  }

  constexpr T operator[] (int index) {
    return data[index].val;
  }

  constexpr T operator[] (int index) const {
    return data[index].val;
  }

  constexpr T* begin() const {
    return &data[0].val;
  }

  constexpr T* end() const {
    return &data[len].val;
  }
};

#endif
