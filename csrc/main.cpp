#include <iostream>

using namespace std;

extern "C" {
   int c_main(int argc, const char* const* argv);
}

int c_main(const int argc, const char* const* const argv) {
   cout << "Hello, ccargo!!" << endl;
   for (int i = 0; i < argc; i++) {
      cout << i << ": " << argv[i] << endl;
   }
   return argc - 1;
}
