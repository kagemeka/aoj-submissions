import typing 


def main() -> typing.NoReturn:
  n = int(input())
  a = [None] * (1 << 18)
  i = 0 
  for _ in range(n):
    *q, = map(
      int, input().split(),
    )
    if q[0] == 0:
      a[i] = q[1]
      i += 1
      continue  
    if q[0] == 1:
      print(a[q[1]])
      continue
    i -= 1


main()
