#ifndef DEVICE_MANAGER_H
#define DEVICE_MANAGER_H

#include <utils/array.h>
#include <utils/singleton.h>

namespace device {

  class driver;
  class device_manager : public singleton<device_manager> {
  private:
    array<driver*> _drivers;
  public:
    void add_driver(driver* dev) {
      _drivers.add(dev);
    }

    auto drivers() const {
      return _drivers;
    }
  };

  class driver {
  public:
    driver() {
      device_manager::instance().add_driver(this);
    }

    virtual void init() = 0;
    virtual bool is_present() = 0;
    virtual const char* name() = 0;
  };

  void init();
};

#endif

