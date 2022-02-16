import sys
from heapq import heappush, heappop, heapify
# from bisect import bisect_left as bi_l, bisect_right as bi_r
# from collections import deque, Counter, defaultdict
# import itertools 
# import string 
# import math 
# from operator import xor, or_
from functools import lru_cache, reduce
# sys.setrecursionlimit(10**7)
inf = float('inf')
MOD = 10**9+7
# MOD = 998244353


# using_numpy = 1
# import numpy as np
# from scipy.sparse.csgraph import (
#   shortest_path, csgraph_to_dense, maximum_flow, minimum_spanning_tree,
#   connected_components
# )
# from scipy.sparse import csr_matrix
# from scipy.spatial import ConvexHull
# from scipy import optimize
# from scipy.special import comb
# from scipy.ndimage import distance_transform_cdt
# from numba import njit, i8
# import networkx as nx


class Algebra:
  class Modular(int):
    def __init__(self, n, mod=MOD):
      self.value = n
      self.mod = mod

    def __str__(self): return f'{self.value}'

    def __add__(self, other): 
      return self.__class__((self.value + other.value) % self.mod)
    def __sub__(self, x): return self.__class__((self.value - x.value) % self.mod)
    def __mul__(self, x): return self.__class__((self.value * x.value) % self.mod)
    def __pow__(self, x): return self.__class__(pow(self.value, x.value, self.mod))

    def __lt__(self, x): return self.value < x.value
    def __le__(self, x): return self.value <= x.value
    def __eq__(self, x): return self.value == x.value 
    def __ne__(self, x): return self.value != x.value
    def __gt__(self, x): return self.value > x.value
    def __ge__(self, x): return self.value >= x.value
  

  
  class SemiGroup:
    pass 
  class Monoid:
    pass
  class Group:
    pass 
  class SemiRing:
    pass
  class Ring:
    pass


  @staticmethod
  def identity(n):
    if using_numpy:
      return np.identity(n, dtype=np.int64)
    else:
      a = [[0]*n for _ in range(n)]
      for i in range(n): a[i][i] = 1
      return a

  @staticmethod
  def dot(a, b):
    if using_numpy:
      return np.dot(a, b)
    else:
      h, w, l = len(a), len(b[0]), len(b)
      assert len(a[0]) == l
      c = [[0]*w for _ in range(h)]
      for i in range(h):
        for j in range(w):
          for k in range(l):
            c[i][j] += a[i][k]*b[k][j]
      return c 
    
  @classmethod
  def matrix_pow(cls, a, n, mod=10**9+7):
    m = len(a)
    b = cls.identity(m)
    while n:
      if n&1: b = cls.dot(b, a)
      n >>= 1; a = cls.dot(a, a)
      if using_numpy:
        a %= mod; b %= mod
      else:
        for i in range(m):
          for j in range(m):
            a[i][j] %= mod
            b[i][j] %= mod
    return b

  @staticmethod
  def bitwise_dot(a, b):
    if using_numpy:
      return np.bitwise_xor.reduce(a[:,None,:] & b.T[None,:,:], axis=-1)
    else:
      h, w, l = len(a), len(b[0]), len(b)
      assert len(a[0]) == l
      c = [[0]*w for _ in range(h)]
      for i in range(h):
        for j in range(w):
          for k in range(l):
            c[i][j] ^= a[i][k]&b[k][j]
      return c 

  @classmethod
  def bitwise_mat_pow(cls, a, n):
    if n==0: return np.eye(len(a), dtype=np.uint32)*((1<<32)-1)
    res = cls.bitwise_mat_pow(a, n//2)
    res = cls.bitwise_dot(res, res)
    return cls.bitwise_dot(res, a) if n&1 else res


  @staticmethod 
  def cumprod(a, mod):
    l = len(a); sql = int(np.sqrt(l)+1)
    a = np.resize(a, sql**2).reshape(sql, sql)
    for i in range(sql-1): a[:, i+1] *= a[:, i]; a[:, i+1] %= mod
    for i in range(sql-1): a[i+1] *= a[i, -1]; a[i+1] %= mod
    return np.ravel(a)[:l]  
  
  @classmethod 
  def generate_fac_ifac(cls, n, p=MOD):
    if using_numpy:
      fac = np.arange(n+1); fac[0] = 1; fac = cls.cumprod(fac, p)
      ifac = np.arange(n+1, 0, -1); ifac[0] = pow(int(fac[-1]), p-2, p)
      ifac = cls.cumprod(ifac, p)[n::-1]
    else:
      fac = [None]*(n+1); fac[0] = 1
      for i in range(n): fac[i+1] = fac[i]*(i+1)%p
      ifac = [None]*(n+1); ifac[n] = pow(fac[n], p-2, p)
      for i in range(n, 0, -1): ifac[i-1] = ifac[i]*i%p
    return fac, ifac 

  class Kitamasa:
    pass


mint = Algebra.Modular


class NumberTheory:
  class PrimeNumbers: # pn
    def __init__(self, n=2*10**6):
      self.is_prime, self.prime_nums = self.find(n)
    
    def __call__(self, n): return self.is_prime[n]
    def __iter__(self): return iter(self.prime_nums)
    def __getitem__(self, key): return self.prime_nums[key]

    @staticmethod
    def find(n): # Sieve of eratosthenes
      if using_numpy:
        is_prime = np.ones(n+1, dtype=np.bool); is_prime[:2] = 0
        for i in range(2, int(n**.5)+1):
          if is_prime[i]: is_prime[i*2::i] = 0
        prime_nums = np.flatnonzero(is_prime)
      else:
        is_prime = [True]*(n+1); is_prime[0] = is_prime[1] = 0
        for i in range(2, int(n**.5)+1):
          if not is_prime[i]: continue
          for j in range(i*2, n+1, i): is_prime[j] = 0
        prime_nums = [i for i in range(2, n+1) if is_prime[i]]
      return is_prime, prime_nums 

    @lru_cache(maxsize=None)
    def factorize(self, n):
      res = defaultdict(int)
      if n < 2: return res
      for p in self:
        if p*p > n: break
        while n%p == 0: res[p] += 1; n //= p
        if n == 1: return res
      res[n] = 1; return res

    def factorize_factorial(self, n):
      res = defaultdict(int)
      for i in range(2, n+1):
        for p, c in self.factorize(i).items(): res[p] += c
      return res
  
  @classmethod
  @lru_cache(maxsize=None)
  def gcd(cls, a, b): return cls.gcd(b, a%b) if b else abs(a)
  @classmethod
  def lcm(cls, a, b): return abs(a // cls.gcd(a, b) * b)

  @staticmethod
  def find_divisors(n):
    divisors = []
    for i in range(1, int(n**.5)+1):
      if n%i: continue
      divisors.append(i)
      j = n // i
      if j != i: divisors.append(j)
    return sorted(divisors)

  @staticmethod
  def base_convert(n, b):
    if not n: return [0]
    res = []
    while n:
      n, r = divmod(n, b)
      if r < 0: n += 1; r -= b
      res.append(r)
    return res



class Combinatorics:
  @classmethod
  @lru_cache(maxsize=None)
  def choose(cls, n, r, mod=None):
    if r > n or r < 0: return 0
    if r == 0: return 1
    res = cls.choose(n-1,r,mod) + cls.choose(n-1,r-1,mod)
    if mod: res %= mod
    return res 

  class CombinationsMod:
    def __init__(self, n=2*10**6, mod=MOD):
      self.__mod = mod
      self.__fac, self.__ifac = Algebra.generate_fac_ifac(n, mod)

    def __call__(self, n, r): return self.__choose(n, r)

    def __choose(self, n, r):
      bl = (0<=r) & (r<=n)
      p = self.__mod
      return bl * self.__fac[n] * self.__ifac[r] % p * self.__ifac[n-r] % p

    def make_nchoose_table(self, n):
      p = self.__mod
      r = len(self.__fac)-1
      if using_numpy:
        n_choose = np.arange(n+1, n-r, -1); n_choose[0] = 1
        n_choose = Algebra.cumprod(n_choose, p)*self.__ifac%p
      else:
        n_choose = [None]*(r+1); n_choose[0] = 1
        for i in range(r): n_choose[i+1] = n_choose[i]*(n-i)%p
        for i in range(1,r+1): n_choose[i] = n_choose[i]*self.__ifac[i]%p
      return n_choose

  @classmethod
  def permutations(cls, a, r=None, i=0):
    a = list(a); n = len(a)
    if r is None: r = n
    res = []
    if r > n or i > r: return res
    if i == r: return [tuple(a[:r])]
    for j in range(i, n): a[i],a[j] = a[j],a[i]; res += cls.permutations(a, r, i+1)
    return res
  
  @staticmethod 
  def combinations(a, r):
    a = tuple(a)
    n = len(a)
    if r > n: return
    indices = list(range(r))
    yield a[:r]
    while True:
      for i in range(r-1, -1, -1):
        if indices[i] != i+n-r: break
      else: return
      indices[i] += 1
      for j in range(i+1, r): indices[j] = indices[j-1]+1
      yield tuple(a[i] for i in indices)



class DP:
  @staticmethod 
  def LIS(a):
    res = [inf] * len(a)
    for x in a: res[bi_l(res, x)] = x
    return res


class String:
  @staticmethod
  def z_algorithm(s):
    n = len(s)
    a = [0] * n; a[0] = n
    l = r = -1
    for i in range(1, n):
      if r >= i: a[i] = min(a[i-l], r-i)
      while i + a[i] < n and s[i+a[i]] == s[a[i]]: a[i] += 1
      if i+a[i] >= r: l, r = i, i+a[i]
    return a


class GeometryTopology:
  class Graph:
    class __Edge:
      def __init__(self, weight=1, capacity=1, **args):
        self.weight = weight
        self.capacity = capacity
    
    class __Node:
      def __init__(self, **args):
        pass

    def __init__(self, n=0):
      self.__N = n
      self.nodes = [None] * n
      self.edges = [{} for _ in range(n)]

    def add_node_info(self, v, **args): self.nodes[v] = self.__Node(**args)

    def add_edge(self, u, v, **args): self.edges[u][v] = self.__Edge(**args)

    def get_size(self): return self.__N

    def bfs(self, src=0):
      assert 0 <= src < self.__N
      self.__depth = self.__lv = lv = [None]*self.__N; lv[src] = 0 # depth in tree, or level in general graph.
      self.__dist = dist = [inf]*self.__N; dist[src] = 0
      self.__parent = par = [None]*self.__N; par[src] = src
      q = deque([src])
      while q:
        u = q.popleft()
        for v, e in self.edges[u].items():
          if e.capacity == 0 or lv[v] is not None: continue
          lv[v] = lv[u] + 1
          dist[v] = dist[u] + e.weight # only tree
          par[v] = u
          q.append(v)
      return lv 
 
    def dinic(self, src, sink):
      def flow_to_sink(u, flow_in):
        if u == sink: return flow_in
        flow = 0
        for v, e in self.edges[u].items():
          if e.capacity == 0 or self.__lv[v] <= self.__lv[u]: continue 
          f = flow_to_sink(v, min(flow_in, e.capacity))
          if not f: continue
          self.edges[u][v].capacity -= f
          if u in self.edges[v]: self.edges[v][u].capacity += f
          else: self.add_edge(v, u, capacity=f)
          flow_in -= f
          flow += f
        return flow

      flow = 0
      while True:
        self.bfs(src)
        if self.__lv[sink] is None: return flow
        flow += flow_to_sink(src, inf)
      
    def ford_fulkerson(self):
      pass

    def push_relabel(self):
      pass

    def floyd_warshall(self):
      n = self.__N
      d = [[inf]*n for _ in range(n)]
      for u in range(n):
        d[u][u] = 0
        for v, e in self.edges[u].items(): d[u][v] = e.weight
      for w in range(n):
        for u in range(n):
          for v in range(n):
            d[u][v] = min(d[u][v], d[u][w]+d[w][v])
      return d
    
    def dijkstra(self, src, paths_cnt=False, mod=None):
      dist = [inf] * self.__N; dist[src] = 0
      visited = [False] * self.__N
      paths = [0] * self.__N; paths[src] = 1
      q = [(0, src)]
      while q:
        d, u = heappop(q)
        if visited[u]: continue
        visited[u] = True
        for v, e in self.edges[u].items():
          dv = d + e.weight
          if dv > dist[v]: continue
          elif dv == dist[v]:
            paths[v] += paths[u]
            if mod: paths[v] %= mod
            continue 
          paths[v], dist[v] = paths[u], dv
          heappush(q, (dv, v))
      if paths_cnt: return dist, paths 
      else: return dist

    def astar(self, src, tgt, heuristic_func):
      cost = [inf] * self.__N
      q = [(heuristic_func(src, tgt), 0, src)]
      while q:
        _, c, u = heappop(q)
        if u == tgt: return c
        if cost[u] != inf: continue 
        cost[u] = c
        for v, e in self.edges[u].items():
          if cost[v] != inf: continue
          h = heuristic_func(v, tgt)
          nc = c + e.weight
          heappush(q, (h+nc, nc, v))        
      return inf
    
    def bellman_ford(self, src):
      n = self.__N
      d = [inf] * n; d[src] = 0
      for _ in range(n-1): 
        for u in range(n):
          for v, e in self.edges[u].items(): d[v] = min(d[v], d[u]+e.weight)
      
      for u in range(n):
        for v, e in self.edges[u].items():
          if d[u]+e.weight < d[v]: raise Exception('found negative cycle.')
      
      return d 
    
    
    def find_ancestors(self): # tree doubling.
      self.__ancestors = ancestors = [self.__parent]
      for _ in range(max(self.__depth).bit_length()):
        ancestors.append([ancestors[-1][u] for u in ancestors[-1]])


    def find_dist(self, u, v):
      return self.__dist[u]+self.__dist[v]-2*self.__dist[self.__find_lca(u, v)]

    
    def __find_lca(self, u, v):
      du, dv = self.__depth[u], self.__depth[v]
      if du > dv:
        u, v = v, u 
        du, dv = dv, du 
      
      d = dv - du
      for i in range(d.bit_length()): # up-stream
        if d>>i&1: v = self.__ancestors[i][v]
      if v == u: return v

      for i in range(du.bit_length()-1, -1, -1): # find direct child of LCA.
        nu, nv = self.__ancestors[i][u], self.__ancestors[i][v]
        if nu == nv: continue 
        u, v = nu, nv 
      
      return self.__ancestors[0][u]

    def init_dsu(self): # disjoint set union (union-find)
      n = self.__N
      self.parent = list(range(n))
      self.rank = [0] * n 
      self.size = [1] * n 

    def find(self, u):
      if self.parent[u] == u: return u
      self.parent[u] = self.find(self.parent[u])
      return self.parent[u]

    def unite(self, u, v):
      u, v = self.find(u), self.find(v)
      if u == v: return 
      if self.rank[u] < self.rank[v]: u,v = v,u
      self.parent[v] = u 
      self.size[u] += self.size[v]
      self.rank[u] = max(self.rank[u], self.rank[v]+1)

  

    def scc(self): # strongly connected components
      n = self.__N
      visited, q, root, r = [False]*n, [], [None]*n, 0
      gg = self.__class__(n)
      for u in range(n):
        for v in self.edges[u]: gg.add_edge(v, u)
      
      def dfs(u):
        if visited[u]: return
        visited[u] = True 
        for v in self.edges[u]: dfs(v)
        q.append(u)

      def rev_dfs(u, r):
        if root[u] is not None: return 
        root[u] = r 
        for v in gg.edges[u]: rev_dfs(v, r)

      for u in range(n): dfs(u)
      for u in q[::-1]: rev_dfs(u, r); r += 1
      return root


    def kruskal(self): # minimum spanning tree
      n = self.__N 
      uf = self.__class__(n); uf.init_dsu()
      edges = sorted([(u,v,e.weight) for u in range(n) for v,e in self.edges[u].items()], key=lambda x: x[2])
      g = self.__class__(n)
      d = 0
      for u, v, w in edges:
        if uf.find(u)==uf.find(v): continue 
        uf.unite(u, v); g.add_edge(u, v, weight=w); d += w
      return g, d
    
    def prim(self): # minimum spanning tree 
      n = self.__N
      g = self.__class__(n)
      visited, dist = [False]*n, 0 
      q = [(0, (0, 0))]
      while q:
        d, (w, u) = heappop(q)
        if visited[u]: continue 
        visited[u] = True; dist += d; g.add_edge(w,u, weight=d)  
        for v, e in self.edges[u].items():
          if not visited[v]: heappush(q, (e.weight, (u,v)))
      return g, dist 


  @staticmethod
  def triangle_area(p0, p1, p2, signed=False):
    x1, y1, x2, y2 = p1[0]-p0[0], p1[1]-p0[1], p2[0]-p0[0], p2[1]-p0[1]
    return (x1*y2 - x2*y1)/2 if signed else abs(x1*y2 - x2*y1)/2 
  
  @classmethod
  def intersect(cls, seg1, seg2):
    (p1, p2), (p3, p4) = seg1, seg2
    t1 = cls.triangle_area(p1, p2, p3, signed=True)
    t2 = cls.triangle_area(p1, p2, p4, signed=True)
    t3 = cls.triangle_area(p3, p4, p1, signed=True)
    t4 = cls.triangle_area(p3, p4, p2, signed=True)
    return (t1*t2<0) & (t3*t4<0)
  

def cumxor(a): return reduce(xor, a, 0)
def cumor(a): return reduce(or_, a, 0)

def bit_count(n):
  cnt = 0
  while n: cnt += n&1; n >>= 1 
  return cnt




class AOJ:
  @staticmethod 
  def ALDS1_12_A():
    n, *a = map(int, sys.stdin.read().split())
    g = GeometryTopology.Graph(n)
    for i in range(n-1):
      for j in range(i+1, n):
        if a[i*n+j] == -1: continue 
        g.add_edge(i,j, weight=a[i*n+j])
        g.add_edge(j,i, weight=a[i*n+j])
    # _, d = g.kruskal()
    _, d = g.prim()
    print(d)

  @staticmethod 
  def GRL_3_C(): # strongly connected components 
    n, m = map(int, sys.stdin.readline().split())
    g = GeometryTopology.Graph(n)
    for _ in range(m): g.add_edge(*map(int, sys.stdin.readline().split()))
    r = g.scc()
    q, *uv = map(int, sys.stdin.read().split())
    for u, v in zip(*[iter(uv)] * 2): print(int(r[u]==r[v]))


class YosupoJudge:
  @staticmethod 
  def Manhattan_MST():
      n, *xy = map(int, sys.stdin.read().split())
      g = GeometryTopology.Graph(n)
      # for x, y in g.add_edge()
      
      # def make(a):
      #   b = sorted(enumerate(a), key=lambda x: x[1])
      #   for i in range(n-1):
      #     u, v, w = b[i][0], b[i+1][0], b[i+1][1]-b[i][1]
      #     if not v in g.edges[u]: g.add_edge(u, v, weight = w)
      #     else: g.edges[u][v].weight = min(g.edges[u][v].weight, w)
      # make(x); make(y)
      # mst, d = g.kruskal()
      # print(d)
      # for u in range(n):
      #   for v in mst.edges[u]:
      #     print(u, v)

if __name__ == '__main__':
  # AtCoder.ABC179.f()

  # AtCoder.ABC060.d()
  # AtCoder.ABC065.d_3()
  # YosupoJudge.Manhattan_MST()
  AOJ.ALDS1_12_A()

  pass 
