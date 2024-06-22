#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

bool is_pangramV1(char *line) {
  int alphabet[26] = {0};
  int unique_cnt = 0;

  while (*line != '\n') {
    if (isalpha(*line)) {
      int idx = tolower(*line) - 'a';
      if (alphabet[idx] == 0) {
        alphabet[idx] = 1;
        unique_cnt += 1;
      }
    }
    line++;
  }

  return unique_cnt == 26;
}

bool is_pangram(char *line) {
  int val = 0;

  while (*line != '\n') {
    if (isalpha(*line)) {
      val |= 1 << (*line - 'a');
    }
    line++;
  }

  return val == 0x3ffffff;
}

int main() {
  char *buffer = NULL;
  int len = 0;
  ssize_t read;

  while ((read = getline(&buffer, &len, stdin)) != -1) {
    if (is_pangram(buffer)) {
      printf("%s", buffer);
    }
  }

  free(buffer);
  return 0;
}
