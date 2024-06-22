#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#define DEFAULT_CAPACITY 8

typedef struct DA {
  void **items;
  int length;
  int capacity;
} DA;

DA *DA_new(void) {
  DA *da = (DA *)malloc(sizeof(DA));
  da->length = 0;
  da->capacity = DEFAULT_CAPACITY;
  da->items = (void **)malloc(sizeof(void *) * da->capacity);
  return da;
}

void DA_free(DA *da) {
  free(da->items);
  free(da);
}

int DA_size(DA *da) { return da->length; }

void DA_push(DA *da, void *x) {
  if (da->length == da->capacity) {
    da->capacity <<= 1;
    da->items = realloc(da->items, da->capacity * sizeof(void *));
  }
  da->items[da->length++] = x;
}

void DA_set(DA *da, void *x, int i) { da->items[i] = x; }

void *DA_get(DA *da, int i) { return da->items[i]; }

void *DA_pop(DA *da) {
  if (da->length <= 0) {
    return NULL;
  }
  return da->items[--da->length];
}

int main() {
  DA *da = DA_new();

  int size = DA_size(da);
  assert(DA_size(da) == 0);

  // basic push and pop test
  int x = 5;
  float y = 12.4;

  DA_push(da, &x);
  DA_push(da, &y);
  assert(DA_size(da) == 2);

  assert(DA_pop(da) == &y);
  assert(DA_size(da) == 1);

  assert(DA_pop(da) == &x);
  assert(DA_size(da) == 0);
  assert(DA_pop(da) == NULL);

  // basic set/get test
  DA_push(da, &x);
  DA_set(da, &y, 0);
  assert(DA_get(da, 0) == &y);
  DA_pop(da);
  assert(DA_size(da) == 0);

  // expansion test
  DA *da2 = DA_new();
  DA_push(da2, &x);
  int i, n = 100 * DEFAULT_CAPACITY, arr[n];
  for (i = 0; i < n; i++) {
    arr[i] = i;
    DA_push(da, &arr[i]);
  }
  assert(DA_size(da) == n);
  for (i = 0; i < n; i++) {
    assert(DA_get(da, i) == &arr[i]);
  }
  for (; n; n--) DA_pop(da);
  assert(DA_size(da) == 0);
  assert(DA_pop(da2) == &x);

  int test = 100;
  DA_set(da, &test, 799);
  assert(DA_get(da, 799) == &test);

  DA_free(da);
  DA_free(da2);
  printf("OK\n");
  return 0;
}
