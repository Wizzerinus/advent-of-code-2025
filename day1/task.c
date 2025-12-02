#include <stdio.h>

int main() {
  FILE *f = fopen("./input1.txt", "rt");

  int got = 0, mult = 1, ptr = 50, count = 0, count2 = 0;
  char current = 0;
  while (fread(&current, 1, 1, f) != 0) {
    if (current == 'R') {
      mult = -1;
    } else if (current >= '0' && current <= '9') {
      got = got * 10 + current - '0';
    } else if (current == '\n') {
      int new_val = ptr + got * mult;
      int old_ptr = ptr;
      ptr = (new_val % 100 + 100) % 100;

      if (ptr == 0) {
        count++;
      }
      if (new_val >= 100) {
        count2 += new_val / 100;
      } else if (new_val < 0) {
        count2 += (-new_val) / 100 + (old_ptr != 0);
      } else if (ptr == 0) {
        count2++;
      }
      got = 0;
      mult = 1;
    } else if (current != 'L') {
      return -2;
    }
  }

  printf("Task 1: %d\n", count);
  printf("Task 2: %d\n", count2);

  fclose(f);
  return 0;
}