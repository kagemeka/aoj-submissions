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
    a = [0] * (2 * n)
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


def solve(
  n: int,
  q: typing.List[
    typing.Tuple[int],
  ],
) -> typing.NoReturn:
  seg = SegmentTree(17)
  n = len(seg)
  def add(i, x):
    i += n
    while i:
      seg[i] += x 
      i //= 2


  def sum_(l, r) -> int:
    l += n; r += n
    s = 0
    while l < r:
      if l & 1:
        s += seg[l]
        l += 1
      l //= 2
      if r & 1:
        r -= 1
        s += seg[r]
      r //= 2
    return s
        

  for com, x, y in q:
    if com == 0:
      add(x - 1, y)
    else:
      print(sum_(x - 1, y))
      


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
  solve(n, q)


main()
