int old(int *nums, int n) {
  int total = 0;
  for (int i = 0; i < n; i++) {
    total += nums[i];
  }
  return total;
}

int opt1(int *nums, int n) {
  int t1 = 0, t2 = 0, t3 = 0, t4 = 0;
  for (int i = 0; i < n; i += 4) {
    t1 += nums[i];
    t2 += nums[i + 1];
    t3 += nums[i + 2];
    t4 += nums[i + 3];
  }
  return t1 + t2 + t3 + t4;
}

int sum(int *nums, int n) {
  opt1(nums, n);
}

