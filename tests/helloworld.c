#include <stdint.h>
#include <stdlib.h>

#include <improved.h>

int main() {
  init(".bogus");
  int ret = commit();
  if (ret == -1) {
    return EXIT_FAILURE;
  }
  return EXIT_SUCCESS;
}
