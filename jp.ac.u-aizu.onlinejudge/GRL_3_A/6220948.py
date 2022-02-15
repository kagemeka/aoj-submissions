import sys
import typing

sys.setrecursionlimit(1 << 20)


def articulation_points_lowlink(
    n: int,
    edges: typing.List[typing.Tuple[int, int]],
) -> typing.List[int]:
    # articulation point is defined on undirected graph.
    # https://cp-algorithms.com/graph/cutpoints.html
    graph: typing.List[typing.List[typing.Tuple[int, int]]] = [
        [] for _ in range(n)
    ]
    for i, (u, v) in enumerate(edges):
        graph[u].append((v, i))
        graph[v].append((u, i))
    order = [-1] * n
    lowlink = [-1] * n
    ord = 0
    is_articulation = [False] * n

    def dfs(u: int, edge_id_to_u: int) -> None:
        nonlocal ord
        order[u] = lowlink[u] = ord
        ord += 1
        num_childs = 0
        for v, edge_id in graph[u]:
            if edge_id == edge_id_to_u:
                continue
            if order[v] != -1:
                lowlink[u] = min(lowlink[u], order[v])
                continue
            num_childs += 1
            dfs(v, edge_id)
            lowlink[u] = min(lowlink[u], lowlink[v])
            is_articulation[u] |= edge_id_to_u != -1 and lowlink[v] >= order[u]
        is_articulation[u] |= edge_id_to_u == -1 and num_childs >= 2

    for i in range(n):
        if order[i] == -1:
            dfs(i, -1)
    return [i for i in range(n) if is_articulation[i]]


def main() -> None:
    n, m = map(int, input().split())
    edges = [tuple(map(int, input().split())) for _ in range(m)]

    points = articulation_points_lowlink(n, edges)
    for p in points:
        print(p)
    # print(*points, sep='\n')


main()
