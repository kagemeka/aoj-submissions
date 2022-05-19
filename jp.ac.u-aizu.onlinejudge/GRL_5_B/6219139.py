import typing
# import dataclasses

S = typing.TypeVar('S')
F = typing.TypeVar('F')


# @dataclasses.dataclass
class Monoid(typing.Generic[S]):
    op: typing.Callable[[S, S], S]
    e: typing.Callable[[], S]

    def __init__(self, op, e) -> None:
        self.op = op
        self.e = e


# monoid operation must be commutative.
def rerooting(
    monoid: Monoid[S],
    map_: typing.Callable[[F, S], S],
    edges: typing.List[typing.Tuple[int, int, F]],
) -> typing.List[S]:
    n = len(edges) + 1
    graph: typing.List[typing.List[typing.Tuple[int, F]]] = [
        [] for _ in range(n)
    ]
    for u, v, f in edges:
        graph[u].append((v, f))
        graph[v].append((u, f))

    dp_from_childs = [monoid.e() for _ in range(n)]

    def tree_dp(u: int, parent: int) -> None:
        for v, f in graph[u]:
            if v == parent:
                continue
            tree_dp(v, u)
            dp_from_childs[u] = monoid.op(
                dp_from_childs[u],
                map_(f, dp_from_childs[v]),
            )

    tree_dp(0, -1)
    dp_from_parent = [monoid.e() for _ in range(n)]

    def reroot(u: int, parent: int) -> None:
        child_edges = [edge for edge in graph[u] if edge[0] != parent]
        deg = len(child_edges)
        dp_l = [monoid.e() for _ in range(deg)]
        dp_r = [monoid.e() for _ in range(deg)]
        for i, (v, f) in enumerate(child_edges[:-1]):
            dp_l[i + 1] = monoid.op(dp_l[i], map_(f, dp_from_childs[v]))
        for i, (v, f) in reversed(list(enumerate(child_edges[1:]))):
            dp_r[i] = monoid.op(dp_r[i + 1], map_(f, dp_from_childs[v]))
        for i, (v, f) in enumerate(child_edges):
            dp_from_parent[v] = map_(
                f,
                monoid.op(dp_from_parent[u], monoid.op(dp_l[i], dp_r[i])),
            )
            reroot(v, u)

    reroot(0, -1)
    return [monoid.op(dp_from_childs[i], dp_from_parent[i]) for i in range(n)]



def main() -> None:
    n = int(input())
    edges = [tuple(map(int, input().split())) for _ in range(n - 1)]
    dp = rerooting(
        Monoid[int](
            op=max,
            e=lambda: 0,
        ),
        map_=lambda f, x: x + f,
        edges=edges,
    )
    print(*dp, sep='\n')

main()
