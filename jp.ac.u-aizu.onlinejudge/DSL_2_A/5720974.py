# from __future__ import (
#   annotations,
# )

import typing
import sys


import dataclasses


# T = typing.TypeVar('T')
@dataclasses.dataclass
class Monoid(
  # typing.Generic[T],
):
  fn: typing.Callable
  e: typing.Callable



class SegTree(
  # typing.Generic[T],
):
  def __getitem__(
    self,
    lr: typing.Tuple[int, int],
  ) -> int:
    m = self.__m 
    l, r = self.__l, self.__r 
    i = self.__i
    ql, qr = lr
    if qr <= l or r <= ql:
      return m.e()
    if ql <= l and r <= qr:
      return self.__a[i]
    c = (l + r) // 2
    self.__i = i << 1
    self.__r = c
    lmin = self[lr]
    self.__i = i << 1 | 1
    self.__l, self.__r = c, r
    rmin = self[lr]
    self.__l, self.__i = l, i
    return m.fn(lmin, rmin)


  def __init__(
    self,
    monoid: Monoid,
    n: int,
  ) -> typing.NoReturn:
    self.__m = monoid
    n = (n - 1).bit_length()
    self.__n = n = 1 << n
    self.__a = [
      monoid.e() 
      for _ in range(n << 1)
    ]
    self.__l, self.__r = 0, n
    self.__i = 1


  def __len__(self) -> int:
    return self.__n


  def __repr__(self) -> str:
    return str(self.__a)


  def __setitem__(
    self,
    i: int,
    x#: T,
  ) -> typing.NoReturn:
    i += self.__n
    self.__a[i] = x; i >>= 1
    while i:
      self.__update(i)
      i >>= 1
  
  
  def __update(
    self,
    i: int,
  ) -> typing.NoReturn:
    a = self.__a
    a[i] = self.__m.fn(
      a[i << 1],
      a[i << 1 | 1],
    ) 


  @classmethod
  def from_array(
    cls,
    monoid: Monoid,
    arr: typing.List[int],
  ): #-> SegTree:
    n = len(arr)
    seg = cls(monoid, n)
    for i in range(n):
      seg[i] = arr[i]
    return seg  



class RMQ():
  def __init__(
    self,
    n: int,
    inf: int,
  ) -> typing.NoReturn:
    self.__inf = inf
    self.__seg = SegTree(
      self.__monoid,
      n,
    )


  @property
  def __monoid(
    self,
  ) -> Monoid:
    return Monoid(
      self.__fn,
      self.__identity,
    )


  def __fn(
    self,
    x: int, 
    y: int,
  ) -> int:
    return min(x, y)
  

  def __identity(
    self,
  ) -> int:
    return self.__inf


  def __setitem__(
    self,
    i: int,
    x: int,
  ) -> typing.NoReturn:
    self.__seg[i] = x


  def __getitem__(
    self,
    lr: typing.Tuple[int, int],
  ) -> int:
    return self.__seg[lr]



def solve(
  q: typing.List[
    typing.Tuple[int],
  ],
) -> typing.NoReturn:
  inf = (1 << 31) - 1
  rmq = RMQ(1 << 18, inf)
  for com, x, y in q:
    if com == 0:
      rmq[x] = y
    else:
      print(rmq[x, y + 1])
      


def main() -> typing.NoReturn:
  n, q = map(
    int, 
    input().split(),
  )
  q = [
    tuple(
      int(x)
      for x in (
        sys.stdin.readline()
        .split()
      )
    )
    for _ in range(q)
  ]
  solve(q)


main()
