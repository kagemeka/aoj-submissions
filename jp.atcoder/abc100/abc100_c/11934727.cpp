#include <bits/stdc++.h>
using namespace std;

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  int res = 0;
  for (int i = 0; i < n; i++) {
    int a; cin >> a;
    while (a % 2 == 0) {
      res++;
      a /= 2;
    }
  }
  cout << res << '\n';
  return 0;
}
