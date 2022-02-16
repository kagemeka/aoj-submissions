import sys
from heapq import heappush, heappop 
from bisect import bisect_left as bi_l, bisect_right as bi_r
from collections import deque, Counter, defaultdict
import itertools 
import string 
import math 
from operator import xor, or_
from functools import lru_cache, reduce
sys.setrecursionlimit(10**7)
inf = float('inf')
MOD = 10**9+7
# MOD = 998244353


using_numpy = 1
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
  def scc():
    n, m = map(int, sys.stdin.readline().split())
    g = GeometryTopology.Graph(n)
    for _ in range(m): g.add_edge(*map(int, sys.stdin.readline().split()))
    r = g.scc()
    q, *uv = map(int, sys.stdin.read().split())
    res = []
    for u, v in zip(*[iter(uv)] * 2):
      res.append(1 if r[u]==r[v] else 0)
    
    print(*res, sep='\n')
    # print(*r)
    
    # print(flush=True)
    # *y, = map(int, sys.stdin.read().split())
    # print(sum(y[i] != res[i] for i in range(q)))
  


if __name__ == '__main__':
  # AtCoder.ABC179.f()
  # AtCoder.ABC061.d_3()
  AOJ.scc()
  # Codeforces.CR676div2.a()

