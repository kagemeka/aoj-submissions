#include <bits/stdc++.h>
using namespace std;

long long gcd(long long a, long long b) {return b ? gcd(b, a % b) : abs(a);}
long long lcm(long long a, long long b) {return abs(a / gcd(a, b) * b);}

int main() {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n; cin >> n;
  vector<int> x(n + 1);
  for (int i = 0; i < n + 1; i++) cin >> x[i];
  int d = abs(x[0] - x[1]);
  for (int i = 1; i < n; i++) {
    d = gcd(d, x[i+1] - x[i]);
  }
  cout << d << '\n';
  return 0;
}
