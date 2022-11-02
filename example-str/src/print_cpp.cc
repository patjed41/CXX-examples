#include "example-str/include/print_cpp.h"
#include <iostream>

void print_cpp(rust::Str message) {
  std::cout << message << std::endl;
  print_rust("hello from C++");
}
