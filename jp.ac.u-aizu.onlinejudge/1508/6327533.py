from __future__ import (
    annotations,
)
import typing
from typing import (
    Optional,
)
import dataclasses
import enum


class State(enum.IntEnum):
    NONE = enum.auto()
    LEFT = enum.auto()
    RIGHT = enum.auto()


@dataclasses.dataclass
class Node:
    parent: Optional[Node] = None
    left: Optional[Node] = None
    right: Optional[Node] = None
    value: Optional[int] = None
    size: int = 1
    mn: int = 1 << 50

    def rotate(
        self,
    ) -> typing.NoReturn:
        p = self.parent
        pp = p.parent
        if pp and pp.left is p:
            pp.left = self
        if pp and pp.right is p:
            pp.right = self
        self.parent = pp

        if p.left is self:
            c = self.right
            p.left = c
            self.right = p
        else:
            c = self.left
            p.right = c
            self.left = p
        if c:
            c.parent = p

        p.parent = self
        p.update()
        self.update()

    def splay(
        self,
    ) -> typing.NoReturn:
        ss = self.__state()
        while ss != State.NONE:
            p = self.parent
            ps = p.__state()
            if ps == State.NONE:
                self.rotate()
            elif ss == ps:
                p.rotate()
                self.rotate()
            else:
                self.rotate()
                self.rotate()
            ss = self.__state()

    def __state(
        self,
    ) -> State:
        p = self.parent
        if not p:
            return State.NONE
        if p.left is self:
            return State.LEFT
        return State.RIGHT

    def update(
        self,
    ) -> typing.NoReturn:
        s = 1
        m = self.value
        if self.left:
            m = min(m, self.left.mn)
            s += self.left.size
        if self.right:
            m = min(m, self.right.mn)
            s += self.right.size
        self.size = s
        self.mn = m


@dataclasses.dataclass
class SplayArray:
    root: typing.Optional[Node] = None

    def __get(
        self,
        i: int,
    ) -> Node:
        u = self.root
        while 1:
            j = u.left.size if u.left else 0
            if i < j:
                u = u.left
                continue
            if i > j:
                u = u.right
                i -= j + 1
                continue
            u.splay()
            self.root = u
            return u

    def __getitem__(
        self,
        i: int,
    ) -> int:
        return self.__get(i).value

    def __setitem__(
        self,
        i: int,
        x: int,
    ) -> typing.NoReturn:
        u = self.__get(i)
        u.value = x
        u.update()

    @classmethod
    def from_size(
        cls,
        n: int,
    ) -> SplayArray:
        a = [Node() for _ in range(n)]
        for i in range(n - 1):
            a[i].parent = a[i + 1]
            a[i + 1].left = a[i]
            a[i + 1].update()
        return cls(a[-1])

    def join(
        self,
        rhs: SplayArray,
    ) -> typing.NoReturn:
        u = self.root
        v = rhs.root
        if not u:
            self.root = v
            return
        if not v:
            return
        u = self.__get(u.size - 1)
        u.right = v
        v.parent = u
        u.update()
        self.root = u

    def split(
        self,
        i: int,
    ) -> SplayArray:
        u = self.root
        if i == 0:
            rhs = SplayArray(u)
            self.root = None
            return rhs
        if i == u.size:
            return SplayArray(None)
        v = self.__get(i)
        u = v.left
        v.left = None
        u.parent = None
        v.update()
        self.root = u
        return SplayArray(v)

    def insert(
        self,
        i: int,
        v: int,
    ) -> typing.NoReturn:
        rhs = self.split(i)
        v = Node(value=v)
        v.update()
        self.join(SplayArray(v))
        self.join(rhs)

    def delete(
        self,
        i: int,
    ) -> typing.NoReturn:
        u = self.__get(i)
        v = u.right
        u = u.left
        if u:
            u.parent = None
        if v:
            v.parent = None
        self.root = u
        self.join(SplayArray(v))


import sys


def main() -> typing.NoReturn:
    n, q = map(
        int,
        input().split(),
    )
    a = [Node() for _ in range(n)]

    for i in range(n):
        a[i].value = int(
            sys.stdin.readline().rstrip(),
        )
    a[0].update()
    for i in range(n - 1):
        a[i + 1].left = a[i]
        a[i].parent = a[i + 1]
        a[i + 1].update()

    a = SplayArray(a[-1])

    for _ in range(q):
        x, y, z = map(
            int,
            sys.stdin.readline().split(),
        )
        if x == 0:
            v = a[z]
            a.delete(z)
            a.insert(y, v)
            continue
        if x == 1:
            rhs = a.split(z + 1)
            m = a.split(y)
            print(m.root.mn)
            a.join(m)
            a.join(rhs)
            continue
        a[y] = z


main()
