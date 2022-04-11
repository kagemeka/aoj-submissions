import typing
import sys



class SegmentTree():
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
    a = [0] * (n << 1)
    self.__n = n
    self.__a = a


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



class RMQ():
  def __init__(
    self,
    height: int,
    inf: int,
  ) -> typing.NoReturn:
    seg = SegmentTree(height)
    n = len(seg)
    for i in range(n << 1):
      seg[i] = inf
    self.__inf = inf
    self.__seg = seg


  def __setitem__(
    self,
    i: int,
    x: int,
  ) -> typing.NoReturn:
    seg = self.__seg
    i += len(seg)
    seg[i] = x
    i >>= 1
    while i:
      seg[i] = min(
        seg[i << 1],
        seg[i << 1 | 1],
      )
      i >>= 1


  def min(
    self,
    l: int,
    r: int,
  ) -> int:
    seg = self.__seg
    n = len(seg)
    l += n; r += n
    mn = self.__inf
    while l < r:
      if l & 1:
        mn = min(mn, seg[l])
        l += 1
      l >>= 1
      if r & 1:
        r -= 1;
        mn = min(mn, seg[r])
      r >>= 1
    return mn



def solve(
  q: typing.List[
    typing.Tuple[int],
  ],
) -> typing.NoReturn:
  inf = (1 << 31) - 1
  rmq = RMQ(17, inf)
  for com, x, y in q:
    if com == 0:
      rmq[x - 1] = y
    else:
      print(rmq.min(x - 1, y))



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
