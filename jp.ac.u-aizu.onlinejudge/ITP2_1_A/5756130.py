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
class Node():
  parent: Optional[Node] = None
  left: Optional[Node] = None
  right: Optional[Node] = None
  value: Optional[int] = None
  size: int = 1


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
    if c: c.parent = p
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
    if self.left is not None:
      s += self.left.size
    if self.right is not None:
      s += self.right.size
    self.size = s



# with list
# @dataclasses.dataclass
class SplayTree():  

  def __getitem__(
    self,
    i: int,
  ) -> int:
    u = self.__root
    while 1:
      j = (
        u.left.size if u.left 
        else 0
      )
      if i < j:
        u = u.left
        continue
      if i > j:
        u = u.right
        i -= j + 1
        continue
      u.splay()
      self.__root = u
      return u.value


  def __setitem__(
    self,
    i: int,
    x: int,
  ) -> typing.NoReturn:
    u = self.__root
    while 1:
      j = (
        u.left.size if u.left 
        else 0
      )
      if i < j:
        u = u.left
        continue
      if i < j:
        u = u.left
        continue
      if i > j:
        u = u.right
        i -= j + 1
        continue
      u.splay()
      u.value = x 
      self.__root = u
      return
  

  def __init__(
    self,
    n: int,
  ) -> typing.NoReturn:
    a = [
      Node() for _ in range(n)
    ]
    for i in range(n - 1):
      a[i].parent = a[i + 1]
      a[i + 1].left = a[i]
      a[i + 1].update()
    self.__a = a
    self.__root = a[-1]




def test() -> typing.NoReturn:
  n = 1 << 18
  st = SplayTree(n)
  st[0] = 100

  print(st[0], st[1])
  print(st[0], st[1])
  print(st[0] == st[0])
  print(st[1] is st[2])
  print(st[0])
  print(st[1])
  print(st[1])
  print(st[2])


def main() -> typing.NoReturn:
  n = 1 << 18
  st = SplayTree(n) 
  n = int(input())
  i = 0 
  for _ in range(n):
    *q, = map(
      int, input().split(),
    )
    if q[0] == 0:
      st[i] = q[1]
      i += 1
      continue
    if q[0] == 1:
      print(st[q[1]])
      continue
    i -= 1


# test()
main()
