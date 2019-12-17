#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int sum_of_primes(int hi);
int main() {
  printf("Hello, World!\n");
  printf("2 + 3 + 5 + 7 + 11 + ... + 83 + 89 + 97 == %d\n", sum_of_primes(100));
}

int sum_of_primes(int hi) {
  bool arr[hi + 1];
  for (int i=0; i < hi; i++) {
    arr[i] = true;
  }
  for (int p=2; p*p < hi; p++) {
    if (!arr[p]) continue;
    for (int c=p*p; c < hi; c += p) {
      arr[c] = false;
    }
  }

  int sum = 0;
  for (int p=2; p < hi; p++) {
    if (arr[p]) {
      sum += p;
    }
  }
  return sum;
}
