#include <iostream>
#include "utils.h"

using namespace std;

void make_test(bool result, bool expect) {
    if (result == expect)
        cout << "✅ Test passed" << endl;
    else
        cout << "❌ Test failed" << endl;
}