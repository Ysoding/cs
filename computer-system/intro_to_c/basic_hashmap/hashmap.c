#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// using liner proing &

#define DEFALT_BUCKETS 8
#define MAX_KEY_SIZE 128

typedef struct hashmap {
  int n_bucket;
  int cnt;
  void **buckets;
} HashMap;

typedef struct node {
  char *key;
  void *val;
} Node;

int hash(char *str, int m);
void rehash(HashMap *map);
HashMap *Hashmap_new();
void Hashmap_set(HashMap *map, char *key, void *value);
bool Hashmap_exists(HashMap *map, char *key);
void *Hashmap_get(HashMap *map, char *key);
void Hashmap_delete(HashMap *map, char *key);
void Hashmap_free(HashMap *map);
void node_free(Node *node);
Node *node_new(char *key, void *val);

HashMap *Hashmap_new() {
  HashMap *res = (HashMap *)malloc(sizeof(HashMap));
  res->n_bucket = DEFALT_BUCKETS;
  res->buckets = calloc(DEFALT_BUCKETS, sizeof(void *));
  res->cnt = 0;
  return res;
}

// https://theartincode.stanis.me/008-djb2/
// djb2 hash
int hash(char *str, int m) {
  unsigned long hash = 5381;

  while (*str != '\0') {
    hash = ((hash << 5) + hash) + (int)(*str);
    str++;
  }

  return hash % m;
}

void rehash(HashMap *map) {
  void **old_buckets = map->buckets;
  int old_n = map->n_bucket;

  map->n_bucket *= 2;
  map->buckets = calloc(map->n_bucket, sizeof(void *));
  map->cnt = 0;

  Node *cur_node;
  for (int i = 0; i < old_n; i++) {
    cur_node = (Node *)old_buckets[i];
    if (cur_node != NULL) {
      // FIXME: 优化直接使用原先的node 而不是释放copy
      Hashmap_set(map, cur_node->key, cur_node->val);
      free(cur_node->key);
      free(old_buckets[i]);
    }
  }

  free(old_buckets);
}

// if the key already exists, update value
void Hashmap_set(HashMap *map, char *key, void *value) {
  if (map->cnt > map->n_bucket * 0.75) {
    rehash(map);
  }

  int idx = hash(key, map->n_bucket);
  int start_idx = idx;
  Node *cur_node;

  while (map->buckets[idx] != NULL) {
    cur_node = (Node *)map->buckets[idx];
    if (strcmp(cur_node->key, key) == 0) {
      cur_node->val = value;
      return;
    }

    idx = (idx + 1) % map->n_bucket;
    if (idx == start_idx) {
      // FIXME: fail as we rehash when the load factor exceeds 0.75
      assert(1 == 2);
      return;
    }
  }

  map->buckets[idx] = node_new(key, value);
  map->cnt += 1;
}

bool Hashmap_exists(HashMap *map, char *key) {
  int idx = hash(key, map->n_bucket);
  int start_idx = idx;

  while (map->buckets[idx] != NULL) {
    Node *cur_node = (Node *)map->buckets[idx];
    if (strcmp(cur_node->key, key) == 0) {
      return true;
    }

    idx = (idx + 1) % map->n_bucket;
    if (idx == start_idx) {
      return false;
    }
  }

  return false;
}

void *Hashmap_get(HashMap *map, char *key) {
  int idx = hash(key, map->n_bucket);
  Node *n = (Node *)map->buckets[idx];
  if (n == NULL) {
    return NULL;
  }

  while (n != NULL && strcmp(n->key, key) != 0) {
    idx = (idx + 1) % map->n_bucket;
    n = (Node *)map->buckets[idx];
  }

  if (n == NULL) {
    return NULL;
  }

  return n->val;
}

void Hashmap_delete(HashMap *map, char *key) {
  int idx = hash(key, map->n_bucket);
  Node *n = (Node *)map->buckets[idx];
  if (n == NULL) {
    return;
  }
  node_free(n);
  map->buckets[idx] = NULL;
  map->cnt -= 1;
}

void Hashmap_free(HashMap *map) {
  for (int i = 0; i < map->n_bucket; i++) {
    node_free(map->buckets[i]);
  }
  free(map->buckets);
  free(map);
}

Node *node_new(char *key, void *val) {
  Node *res = (Node *)malloc(sizeof(Node));

  int n = strlen(key) + 1;
  char *k = (char *)malloc(n);
  strncpy(k, key, n);

  res->key = k;
  res->val = val;
}

void node_free(Node *node) {
  if (node == NULL) {
    return;
  }
  free(node->key);
  free(node);
}

int main() {
  HashMap *h = Hashmap_new();

  // basic get/set functionality
  int a = 5;
  float b = 7.2;
  Hashmap_set(h, "item a", &a);
  Hashmap_set(h, "item b", &b);
  assert(Hashmap_exists(h, "item a"));
  assert(Hashmap_exists(h, "item b"));
  assert(!Hashmap_exists(h, "item c"));
  assert(Hashmap_get(h, "item a") == &a);
  assert(Hashmap_get(h, "item b") == &b);

  // using the same key should override the previous value
  int c = 20;
  Hashmap_set(h, "item a", &c);
  assert(Hashmap_get(h, "item a") == &c);

  // basic delete functionality
  Hashmap_delete(h, "item a");
  assert(Hashmap_get(h, "item a") == NULL);
  assert(!Hashmap_exists(h, "item a"));

  // handle collisions correctly
  // note: this doesn't necessarily test expansion
  int i, n = DEFALT_BUCKETS * 10, ns[n];
  char key[MAX_KEY_SIZE];
  for (i = 0; i < n; i++) {
    ns[i] = i;
    sprintf(key, "item %d", i);
    Hashmap_set(h, key, &ns[i]);
  }
  for (i = 0; i < n; i++) {
    sprintf(key, "item %d", i);
    void *val = Hashmap_get(h, key);
    assert(val == &ns[i]);
  }

  Hashmap_free(h);
  /*
     stretch goals:
     - expand the underlying array if we start to get a lot of collisions
     - support non-string keys
     - try different hash functions
     - switch from chaining to open addressing
     - use a sophisticated rehashing scheme to avoid clustered collisions
     - implement some features from Python dicts, such as reducing space use,
     maintaing key ordering etc. see https://www.youtube.com/watch?v=npw4s1QTmPg
     for ideas
     */
  printf("ok\n");
  return 0;
}