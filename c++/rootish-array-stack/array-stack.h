#ifndef ARRAY_STACK_H
#define ARRAY_STACK_H

template <typename T>
class ArrayStack {
public:
    ArrayStack();
    ~ArrayStack();
    T get(int index);
    T set(int index, T value);
    void add(int index, T value);
    T remove(int index);
    int getSize();
private:
    int size;
    int capacity;
    T* array;
    void resize();
};
#endif