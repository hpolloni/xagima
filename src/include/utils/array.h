#ifndef ARRAY_H
#define ARRAY_H

#include <stddef.h>

template <typename T, size_t size = 256>
class array {
private:
  mutable T data[size];
  mutable size_t len = 0;
public:
  array() : len(0) {}
  
  constexpr void add(const T& elem) noexcept {
    data[len++] = elem;
  }

  constexpr size_t length() noexcept {
    return len;
  }

  constexpr void add(const T& elem) const noexcept {
    data[len++] = elem;
  }

  constexpr size_t length() const noexcept {
    return len;
  }

  constexpr T operator[] (int index) {
    return data[index];
  }

  constexpr T operator[] (int index) const noexcept {
    return data[index];
  }

  constexpr T* begin() const noexcept {
    return &data[0];
  }

  constexpr T* end() const noexcept {
    return &data[len];
  }
};

#endif
