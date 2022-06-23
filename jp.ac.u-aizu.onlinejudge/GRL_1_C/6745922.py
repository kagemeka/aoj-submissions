# /mypy: ignore-errors
import typing

G = typing.List[typing.List[int]]


def floyd_warshall(g: G, inf: int = 1 << 63) -> G:
    n = len(g)
    for k in range(n):
        for i in range(n):
            d0 = g[i][k]
            if d0 == inf:
                continue
            for j in range(n):
                d1 = g[k][j]
                if d1 == inf or d0 + d1 >= g[i][j]:
                    continue
                g[i][j] = d0 + d1
    return g


def main() -> None:
    n, m = map(int, input().split())
    inf = 1 << 32
    g = [[inf] * n for _ in range(n)]
    for i in range(n):
        g[i][i] = 0
    for _ in range(m):
        i, j, d = map(int, input().split())
        g[i][j] = min(g[i][j], d)
    g = floyd_warshall(g, inf)
    if any(g[i][i] < 0 for i in range(n)):
        print("NEGATIVE CYCLE")
        return
    for row in g:
        print(" ".join("INF" if x == inf else str(x) for x in row))


main()

