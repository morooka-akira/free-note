#include "array-stack.h"

ArrayStack::ArrayStack() {
    this->size = 0;
    this->capacity = 1; // 初期化: 配列の初期容量を1に設定
    this->array = new int[capacity]; // 配列を初期容量で初期化
}

ArrayStack::~ArrayStack() {
    delete[] array;
}

int ArrayStack::get(int index) {
    return array[index];
}

int ArrayStack::set(int index, int value) {
    int old_value = array[index];
    array[index] = value;
    return old_value;
}

void ArrayStack::add(int index, int value) {
    if (size + 1 >= capacity){ 
        resize();
    }
    for (int i = size - 1; i >= index; i--) {
        array[i + 1] = array[i];
    }
    array[index] = value;
    size++;
}

int ArrayStack::remove(int index) {
    int removed_value = array[index];
    for (int i = index + 1; i < size; i++) {
        array[i - 1] = array[i];
    }
    size--;
    if (capacity / 3 >= size) {
        resize();
    }
    return removed_value;
}

void ArrayStack::resize() {
    capacity *= 2; // 配列の容量を2倍にする
    int *new_array = new int[capacity]; // 新しい配列を新しい容量で初期化
    for (int i = 0; i < size; i++) {
        new_array[i] = array[i];
    }
    delete[] array;
    array = new_array;
}
