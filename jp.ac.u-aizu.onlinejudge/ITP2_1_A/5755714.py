import typing


def main() -> typing.NoReturn:
  n = int(input())
  a = []
  for _ in range(n):
    *q, = map(
      int, input().split(),
    )
    if q[0] == 0:
      a.append(q[1])
      continue
    if q[0] == 1:
      print(a[q[1]])
      continue
    a.pop()


main()
