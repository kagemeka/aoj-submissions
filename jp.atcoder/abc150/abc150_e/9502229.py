import sys

MOD = 10 ** 9 + 7

def make_power_2_table(n=10**6, mod=MOD):
    power = [None] * (n + 1)
    power[0] = 1
    for i in range(n):
        power[i+1] = power[i] * 2 % mod
    return power

power_2 = make_power_2_table(2*10**5)
inv_2 = pow(2, MOD-2, MOD)

n, *c = map(int, sys.stdin.read().split())
c.sort()

def main():
    res = 0
    for i in range(n):
        sum_d = (power_2[n-i-1] + power_2[n-(i+1)] * inv_2 % MOD * (n-(i+1)) % MOD) % MOD
        l_comb = power_2[i]
        res += c[i] * l_comb % MOD * sum_d % MOD
        res %= MOD

    return res * power_2[n] % MOD

if __name__ == '__main__':
    ans = main()
    print(ans)
