import sys
import typing


class SegTree:
    def __getitem__(
        self,
        i: int,
    ) -> int:
        return self.__a[i]

    def __init__(
        self,
        height: int,
    ) -> typing.NoReturn:
        n = 1 << height
        self.__n = n
        self.__a = [0] * (n << 1)

    def __len__(self) -> int:
        return self.__n

    def __repr__(self) -> str:
        return str(self.__a)

    def __setitem__(
        self,
        i: int,
        x: int,
    ) -> typing.NoReturn:
        self.__a[i] = x


class RAQ:
    def __init__(
        self,
        height: int,
    ) -> typing.NoReturn:
        seg = SegTree(height)
        self.__seg = seg

    def add(
        self,
        l: int,
        r: int,
        x: int,
    ) -> typing.NoReturn:
        seg = self.__seg
        n = len(seg)
        l += n
        r += n
        while l < r:
            if l & 1:
                seg[l] += x
                l += 1
            if r & 1:
                r -= 1
                seg[r] += x
            l >>= 1
            r >>= 1

    def __getitem__(
        self,
        i: int,
    ) -> int:
        seg = self.__seg
        i += len(seg)
        s = 0
        while i:
            s += seg[i]
            i >>= 1
        return s


def solve(
    q: typing.List[
        typing.Tuple[int],
    ],
) -> typing.NoReturn:
    raq = RAQ(17)
    for x in q:
        if x[0] == 0:
            l, r, x = x[1:]
            raq.add(l - 1, r, x)
        else:
            i = x[1]
            print(raq[i - 1])


def main():
    n, q = map(int, input().split())
    q = [
        tuple(int(x) for x in (sys.stdin.readline().split())) for _ in range(q)
    ]
    solve(q)


main()
