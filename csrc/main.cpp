#include <iostream>

using namespace std;

extern "C" {

int extern_c_main(int argc, const char* const* argv) {
   cout << "Hello, ccargo!!\n";
   for (int i = 0; i < argc; i++) {
      cout << i << ": " << argv[i] << endl;
   }
   return 0;
}

}
