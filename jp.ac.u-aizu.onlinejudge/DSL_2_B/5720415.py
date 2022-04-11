from __future__ import (
  annotations,
)
import typing



class SegTree():

  def  __init__(
    self,
    height: int,
  ) -> typing.NoReturn:
    n = 1 << height
    self.__a = [0] * (n << 1)
    self.__n = n


  def __len__(self) -> int:
    return self.__n


  def __getitem__(
    self,
    i: int,
  ) -> int:
    return self.__a[i]


  def __setitem__(
    self,
    i: int,
    x: int,
  ) -> typing.NoReturn:
    self.__a[i] = x



class RSQ():

  def __init__(
    self,
    height: int,
  ) -> typing.NoReturn:
    seg = SegTree(height)
    self.__seg = seg


  def add(
    self,
    i: int,
    x: int,
  ) -> typing.NoReturn:
    seg = self.__seg
    n = len(seg)
    i += n
    seg[i] += x; i >>= 1
    while i:
      seg[i] = seg[i << 1 | 1]
      seg[i] += seg[i << 1]
      i >>= 1


  def sum(
    self,
    l: int,
    r: int,
    sl: int,
    sr: int,
    i: int,
  ) -> int:
    if r <= sl or sr <= l:
      return 0
    if l <= sl and sr <= r:
      return self.__seg[i]
    sc = (sl + sr) // 2
    lsum = self.sum(
      l, r, sl, sc, i << 1,
    )
    rsum = self.sum(
      l, r, sc, sr, i << 1 | 1,
    )
    return lsum + rsum






def solve(
  q: typing.List[
    typing.Iterator[int],
  ],
) -> typing.NoReturn:
  h = 17
  rsq = RSQ(h)
  for com, x, y in q:
    if com == 0:
      rsq.add(x - 1, y)
    else:
      print(rsq.sum(
        x - 1, y, 0, 1 << h, 1,
      ))




def main() -> typing.NoReturn:
  n, q = map(
    int,
    input().split(),
  )
  q = [
    map(int, input().split())
    for _ in range(q)
  ]
  solve(q)


main()
