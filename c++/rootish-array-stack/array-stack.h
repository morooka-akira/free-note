#ifndef ARRAY_STACK_H
#define ARRAY_STACK_H

class ArrayStack {
public:
    ArrayStack();
    ~ArrayStack();
    int get(int index);
    int set(int index, int value);
    void add(int index, int value);
    int remove(int index);
private:
    int size;
    int capacity;
    int* array;
    void resize();
};
#endif