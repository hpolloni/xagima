#ifndef SINGLETON_HPP
#define SINGLETON_HPP

template<class T>
class singleton {
public:
    static T& instance() {
        static T instance;
        return instance;
    }
};

#endif