import sys
import typing

sys.setrecursionlimit(1 << 20)


def lowlink_undirected(
    n: int,
    edges: typing.List[typing.Tuple[int, int]],
) -> typing.Tuple[typing.List[int], typing.List[int]]:
    graph: typing.List[typing.List[typing.Tuple[int, int]]] = [
        [] for _ in range(n)
    ]
    for i, (u, v) in enumerate(edges):
        graph[u].append((v, i))
        graph[v].append((u, i))
    order = [-1] * n
    lowlink = [-1] * n
    ord = 0

    def dfs(u: int, edge_id_to_u: int) -> None:
        nonlocal ord
        order[u] = lowlink[u] = ord
        ord += 1
        for v, edge_id in graph[u]:
            if edge_id == edge_id_to_u:
                continue
            if order[v] != -1:
                lowlink[u] = min(lowlink[u], order[v])
                continue
            dfs(v, edge_id)
            lowlink[u] = min(lowlink[u], lowlink[v])

    for i in range(n):
        if order[i] == -1:
            dfs(i, -1)
    return order, lowlink


def bridges_lowlink(
    n: int,
    edges: typing.List[typing.Tuple[int, int]],
) -> typing.List[int]:
    # bridge is defined on undirected graph.
    # https://cp-algorithms.com/graph/bridge-searching.html
    order, lowlink = lowlink_undirected(n, edges)
    bridge_ids: typing.List[int] = []
    for i, (u, v) in enumerate(edges):
        if order[u] > order[v]:
            u, v = v, u
        if lowlink[v] > order[u]:
            bridge_ids.append(i)
    return bridge_ids


def main() -> None:
    n, m = map(int, input().split())
    edges = [tuple(map(int, input().split())) for _ in range(m)]
    bridge_ids = bridges_lowlink(n, edges)

    bridges = []
    for i in bridge_ids:
        u, v = edges[i]
        if u > v:
            u, v = v, u
        bridges.append((u, v))
    bridges.sort()
    for u, v in bridges:
        print(u, v)


main()
