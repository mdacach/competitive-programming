#include <iostream>
#include <vector>
#include <algorithm>

#define int int64_t

const int MOD = 998244353;
const int N = 300'005;

int64_t factorial[N];

int fast_exp(int b, int e) {
    int answer = 1;
    while (e > 0) {
        if (e & 1) {
            answer *= b;
            answer %= MOD;
        }
        b *= b;
        b %= MOD;
        e /= 2;
    }
    return answer;
}

int inverse(int a) {
    return fast_exp(a, MOD - 2);
}

int choose(int a, int b) {
    int num = factorial[a];
    int den1 = factorial[b];
    int den2 = factorial[a - b];
    return (num * inverse(den1)) % MOD * inverse(den2) % MOD;
}

int32_t main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    factorial[0] = 1;
    for (int i = 1; i < N; ++i) {
        factorial[i] = factorial[i - 1] * i;
        factorial[i] %= MOD;
    }

    int n;
    std::cin >> n;
    std::vector <std::vector<int>> groups;

    for (int i = 0; i < n;) {
        std::vector <int> this_group;
        for (int j = 0; j < 3; ++j) {
            int x;
            std::cin >> x;
            this_group.push_back(x);
        }
        i += 3;
        std::sort(std::rbegin(this_group), std::rend(this_group));
        groups.push_back(this_group);
    }

    int64_t ways = 1;
    for (auto &g: groups) {
        if (g[0] == g[2]) {
            ways *= 3;
        } else if (g[1] == g[2]) {
            ways *= 2;
        } else {
            ways *= 1;
        }
        ways %= MOD;
    }


    ways = (ways * choose(n / 3, n / 6)) % MOD;

    std::cout << ways << '\n';
}