# /mypy: ignore-errors
import typing


G = typing.List[typing.List[int]]


def floyd_warshall(g: G, inf: int = 1 << 63) -> G:
    n = len(g)
    for k in range(n):
        for i in range(n):
            for j in range(n):
                if g[i][k] == inf or g[k][j] == inf:
                    continue
                g[i][j] = min(g[i][j], g[i][k] + g[k][j])
    return g


def main() -> None:
    n, m = map(int, input().split())
    inf = 1 << 60
    g = [[inf] * n for _ in range(n)]
    for i in range(n):
        g[i][i] = 0
    for _ in range(m):
        i, j, d = map(int, input().split())
        g[i][j] = min(g[i][j], d)
    g = floyd_warshall(g, inf)
    for i in range(n):
        if g[i][i] < 0:
            print("NEGATIVE CYCLE")
            return
    for row in g:
        row = ["INF" if x == inf else x for x in row]
        print(*row)


main()

