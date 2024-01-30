#include "array-stack.h"

#ifndef ROOTISH_ARRAY_STACK_H
#define ROOTISH_ARRAY_STACK_H

class RootishArrayStack {
public:
    RootishArrayStack();
    ~RootishArrayStack();
    int get(int index);
    int set(int index, int value);
    void add(int index, int value);
    int remove(int index);
private:
    int size;
    int capacity;
    ArrayStack* blocks;
    void grow();
    int i2b(int index);
};
#endif