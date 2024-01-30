#include "array-stack.h"

template <typename T>
ArrayStack<T>::ArrayStack() {
    this->size = 0;
    this->capacity = 1; // 初期化: 配列の初期容量を1に設定
    this->array = new T[capacity]; // 配列を初期容量で初期化
}

template <typename T>
ArrayStack<T>::~ArrayStack() {
    delete[] array;
}

template <typename T>
T ArrayStack<T>::get(int index) {
    return array[index];
}
template <typename T>
T ArrayStack<T>::set(int index, T value) {
    array[index] = value;
    return old_value;
}

template <typename T>
void ArrayStack<T>::add(int index, T value) {
    if (size + 1 >= capacity){ 
        resize();
    }
    for (int i = size - 1; i >= index; i--) {
        array[i + 1] = array[i];
    }
    array[index] = value;
    size++;
}

template <typename T>
T ArrayStack<T>::remove(int index) {
    T removed_value = array[index];
    for (int i = index + 1; i < size; i++) {
        array[i - 1] = array[i];
    }
    size--;
    if (capacity / 3 >= size) {
        resize();
    }
    return removed_value;
}

template <typename T>
void ArrayStack<T>::resize() {
    capacity *= 2; // 配列の容量を2倍にする
    T *new_array = new T[capacity]; // 新しい配列を新しい容量で初期化
    for (int i = 0; i < size; i++) {
        new_array[i] = array[i];
    }
    delete[] array;
    array = new_array;
}

template <typename T>
int ArrayStack<T>::getSize() {
    return size;
}