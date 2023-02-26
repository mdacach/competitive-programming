#include <bits/stdc++.h>

auto solve_testcase() -> void {
    int n; std::cin >> n;
    int ones = 0;
    for (int i = 0; i < n; ++i) {
        int x; std::cin >> x;
        ones += (x == 1);
    }
    int pairs = ones / 2;
    n -= 2 * pairs;
    std::cout << n + pairs << '\n';
}

auto main() -> int32_t {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int t;
    std::cin >> t;
    while (t--) solve_testcase();
}
