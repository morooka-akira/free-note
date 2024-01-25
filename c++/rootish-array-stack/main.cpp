#include <iostream>
#include "array-stack.h"

using namespace std;

int main() {
    // ArrayStackを初期化してサンプル実装する
    ArrayStack *stack = new ArrayStack();
    stack->add(0, 1);
    stack->add(0, 2);
    stack->add(0, 3);
    stack->add(0, 4);
    stack->add(0, 5);

    cout << stack->get(0) << endl;
    cout << stack->get(1) << endl;
    cout << stack->get(2) << endl;

    cout << stack->remove(0) << endl;
    cout << stack->remove(0) << endl;
    cout << stack->remove(0) << endl;

    return 0;
}