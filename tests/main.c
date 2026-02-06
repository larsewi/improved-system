#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include <leech2.h>

int main(int argc, char *argv[]) {
  if (argc < 3) {
    return EXIT_FAILURE;
  }

  int ret = lch_init(argv[1]);
  if (ret != 0) {
    return EXIT_FAILURE;
  }

  if (strcmp(argv[2], "commit") == 0) {
    ret = lch_commit();
  }
  else {
    return EXIT_FAILURE;
  }

  return (ret == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}
