#include <tty/tty.h>
#include <utils/singleton.h>

namespace tty {
  enum tty_color {
    COLOR_BLACK = 0,
    COLOR_BLUE = 1,
    COLOR_GREEN = 2, 
    COLOR_CYAN = 3,
    COLOR_RED = 4,
    COLOR_MAGENTA = 5,
    COLOR_BROWN = 6,
    COLOR_LIGHT_GREY = 7,
    COLOR_DARK_GREY = 8,
    COLOR_LIGHT_BLUE = 9,
    COLOR_LIGHT_GREEN = 10,
    COLOR_LIGHT_CYAN = 11,
    COLOR_LIGHT_RED = 12,
    COLOR_LIGHT_MAGENTA = 13,
    COLOR_LIGHT_BROWN = 14,
    COLOR_WHITE = 15,
  };

  constexpr static uint16_t vga_entry(unsigned char uc, uint8_t color) {
    return (uint16_t)uc | (uint16_t)color << 8;
  }

  constexpr uint8_t make_color(enum tty_color fg, enum tty_color bg) {
    return fg | bg << 4;
  }
  
  class tty_impl : public singleton<tty_impl> {
  private:
    const size_t vga_width = 80;
    const size_t vga_height = 25;
    uint16_t* const framebuffer = reinterpret_cast<uint16_t*>(0xB8000);
    const uint8_t color = make_color(COLOR_LIGHT_GREY, COLOR_BLACK);
    mutable uint8_t cursorX = 0;
    mutable uint8_t cursorY = 0;
  public:
    tty_impl() {
      clear();
    }
    
    constexpr void move_cursor(uint8_t x, uint8_t y) const noexcept {
      cursorX = x;
      cursorY = y;
      // TODO: Actually move the cursor
    }

    constexpr void clear() const noexcept {
      for (size_t y = 0; y < vga_height; y++) {
        for (size_t x = 0; x < vga_width; x++) {
          framebuffer[y * vga_width + x] = vga_entry(' ', color);
        }
      }
      move_cursor(0,0);
    }

    constexpr void write(char c) const noexcept {
      if (c == '\n') {
        cursorX = 0;
        cursorY++;
        return;
      }

      framebuffer[cursorY * vga_width + cursorX] = vga_entry(c, color);
      if (++cursorX == vga_width) {
        cursorX = 0;
        if (++cursorY == vga_height) {
          cursorY = 0;
        }
      }
    }
  };

  void default_tty::write(char c) const {
    tty_impl::instance().write(c);
  }

  const default_tty dev{};
  const ostream<default_tty> out = ostream{dev};
}