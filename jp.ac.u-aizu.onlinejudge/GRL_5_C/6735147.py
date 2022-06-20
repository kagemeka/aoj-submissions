# /mypy: ignore-errors
from re import I
import typing

E = typing.Tuple[int, int]
L = typing.List


# (undirected edges, root) -> parent, depth
# unlike in Rust, we don't generalize in Python to avoid overhead
def tree_bfs(e: typing.List[E], r: int) -> typing.Tuple[L[int], L[int]]:
    n = len(e) + 1
    g = _ue_to_g(n, e)
    p = [-1] * n
    d = [0] * n
    q = [r]
    for u in q:
        for v in g[u]:
            if v == p[u]:
                continue
            p[v] = u
            d[v] = d[u] + 1
            q.append(v)
    return p, d


def _ue_to_g(n: int, e: typing.List[E]) -> typing.List[typing.List[int]]:
    # undirected edges to graph
    g = [[] for _ in range(n)]
    for u, v in e:
        g[u].append(v)
        g[v].append(u)
    return g


T = typing.TypeVar("T")
ED = typing.Tuple[int, int, T]


def _ued_to_g(
    n: int,  # vertex size
    e: typing.List[ED],
) -> typing.List[typing.List[typing.Tuple[int, T]]]:
    # undirected edges with data to graph
    g = [[] for _ in range(n)]
    for u, v, d in e:
        g[u].append((v, d))
        g[v].append((u, d))
    return g


def doubling(
    e: typing.List[typing.Tuple[int, int]],
) -> typing.Callable[[int, int], int]:
    # binary lifting
    n = len(e) + 1
    r = 0  # root
    p, dep = tree_bfs(e, r)
    k = max(1, max(dep).bit_length())
    a = [[n] * n for _ in range(k)]
    a[0] = p
    a[0][r] = r
    for i in range(k - 1):
        for j in range(n):
            a[i + 1][j] = a[i][a[i][j]]

    def get(u: int, v: int) -> int:
        if dep[u] > dep[v]:
            u, v = v, u
        d = dep[v] - dep[u]
        for i in range(d.bit_length()):
            if d >> i & 1:
                v = a[i][v]
        if v == u:
            return u
        for f in a[::-1]:
            nu, nv = f[u], f[v]
            if nu != nv:
                u, v = nu, nv
        return p[u]

    return get


class UF:
    # Union Find
    _a: typing.List[int]  # root: neg-size, other: parent

    def __init__(self, n: int) -> None:
        self._a = [-1] * n

    def __len__(self) -> int:
        return len(self._a)

    def root(self, u: int) -> int:
        if self._a[u] < 0:
            return u
        self._a[u] = self.root(self._a[u])
        return self._a[u]

    def unite(self, u: int, v: int) -> None:
        u, v = self.root(u), self.root(v)
        if u == v:
            return
        if self._a[u] > self._a[v]:
            u, v = v, u
        self._a[u] += self._a[v]
        self._a[v] = u

    def size_of(self, u: int) -> int:
        return -self._a[self.root(u)]


# class Root(typing.Sized, typing.Protocol):
#     def root(self, u: int) -> int:
#         ...

# def same(uf: Root, u: int, v: int) -> bool:
#     return uf.root(u) == uf.root(v)


# def labels(uf: Root) -> list[int]:
#     n = len(uf)
#     lb = [-1] * n  # label
#     l = 0
#     for i in range(n):
#         r = uf.root(i)
#         if lb[r] == -1:
#             lb[r] = l
#             l += 1
#         lb[i] = lb[r]
#     return lb


def tarjan(
    e: typing.List[E],
    r: int,
    qs: typing.List[typing.Tuple[int, int]],  # queries
) -> typing.List[int]:
    # tarjan's offline algorithm
    n = len(e) + 1
    g = _ue_to_g(n, e)
    q = _ued_to_g(n, [(u, v, i) for i, (u, v) in enumerate(qs)])
    uf = UF(n)
    a = [-1] * n  # current ancestor
    lca = [n] * len(qs)

    def dfs(u: int) -> None:
        a[u] = u
        for v in g[u]:
            if a[v] != -1:  # visited
                continue
            dfs(v)
            uf.unite(u, v)
            a[uf.root(u)] = u

        for v, i in q[u]:
            if a[v] != -1:
                lca[i] = a[uf.root(v)]

    dfs(r)
    return lca


def main() -> None:
    n = int(input())
    edges = []
    for i in range(n):
        k, *c = map(int, input().split())
        for v in c:
            edges.append((i, v))

    q = int(input())
    q = [tuple(map(int, input().split())) for _ in range(q)]

    # get = doubling(edges)

    # res = []
    # for u, v in q:
    #     res.append(get(u, v))
    res = tarjan(edges, 0, q)
    print(*res, sep="\n")


import sys

sys.setrecursionlimit(1 << 25)
main()

