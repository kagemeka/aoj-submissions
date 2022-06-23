# mypy: ignore-errors
import typing

T = typing.TypeVar("T")
G = typing.List[typing.List[T]]


def floyd_warshall(g: G) -> G:
    n = len(g)
    for k in range(n):
        for i in range(n):
            for j in range(n):
                g[i][j] = min(g[i][j], g[i][k] + g[k][j])
    return g


def main() -> None:
    n, m = map(int, input().split())
    inf = float("inf")
    g = [[inf] * n for _ in range(n)]
    for i in range(n):
        g[i][i] = 0
    for _ in range(m):
        i, j, d = map(int, input().split())
        g[i][j] = min(g[i][j], d)
    g = floyd_warshall(g)
    if any(g[i][i] < 0 for i in range(n)):
        print("NEGATIVE CYCLE")
        return
    for row in g:
        print(" ".join("INF" if x == inf else str(x) for x in row))


main()

