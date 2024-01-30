#include "rootish-array-stack.h"
#include <math.h>

RootishArrayStack::RootishArrayStack() {
    size = 0;
    capacity = 1;
    blocks = new ArrayStack();
}

RootishArrayStack::~RootishArrayStack() {
    delete blocks;
}

// indexからblockをお求める
int RootishArrayStack::i2b(int index) {
    double db = (-3.0 + sqrt(9 + 8 * index)) / 2.0;
    int b = ceil(db);
    return b;
}

int RootishArrayStack::get(int index) {
    int b = i2b(index);
    int j = index - b * (b + 1) / 2;
    return blocks->get[j].get();
}

void RootishArrayStack::grow() {
    int* newBlock = new int[blocks->size() + 1];
    blocks->add(blocks->size(), newBlock);
    capacity++;
}