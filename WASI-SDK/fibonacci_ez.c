#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

unsigned long fib(unsigned long i) {
  if (i <= 1) {
    return i;
  }
  return fib(i - 1) + fib(i - 2);
}

int main(int argc, char *argv[]) {
	int a=fib(10);
	printf("%d",a);
}
