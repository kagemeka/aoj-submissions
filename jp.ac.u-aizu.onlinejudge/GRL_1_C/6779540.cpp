#include <bits/stdc++.h>

using namespace std;

template <typename G, typename F>
auto floyd_warshall(G g, F f) -> G {
  int n = g.size();
  for (int k = 0; k < n; ++k) {
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        g[i][j] = f(g[i][j], g[i][k], g[k][j]);
      }
    }
  }
  return g;
}

auto main() -> int {
  ios_base::sync_with_stdio(false);
  cin.tie(nullptr);

  int n, m;
  cin >> n >> m;

  long inf = 1l << 60;
  vector<vector<long>> g(n, vector<long>(n, inf));
  while (m--) {
    int s, t, d;
    cin >> s >> t >> d;
    g[s][t] = d;
  }
  for (int i = 0; i < n; i++) g[i][i] = 0;

  g = floyd_warshall(g, [](long x, long y, long z) { return min(x, y + z); });

  for (int i = 0; i < n; i++) {
    if (g[i][i] < 0) {
      cout << "NEGATIVE CYCLE" << endl;
      return 0;
    }
  }
  for (auto& row : g) {
    for (int i = 0; i < n; i++) {
      if (row[i] == inf) cout << "INF";
      else cout << row[i];
      cout << " \n"[i == n - 1];
    }
  }
}

