# mypy: ignore-errors
from re import I
import typing

E = typing.Tuple[int, int]
L = typing.List


# (undirected edges, root) -> parent, depth
# unlike in Rust, we don't generalize in Python to avoid overhead
def tree_bfs(e: list[E], r: int) -> typing.Tuple[L[int], L[int]]:
    n = len(e) + 1
    g = [[] for _ in range(n)]
    for u, v in e:
        g[u].append(v)
        g[v].append(u)
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


# binary lifting
def doubling(
    e: typing.List[typing.Tuple[int, int]],
) -> typing.Callable[[int, int], int]:
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


def main() -> None:
    n = int(input())
    edges = []
    for i in range(n):
        k, *c = map(int, input().split())
        for v in c:
            edges.append((i, v))

    get = doubling(edges)
    q = int(input())

    res = []
    for _ in range(q):
        u, v = map(int, input().split())
        res.append(get(u, v))
    print(*res, sep="\n")


main()

