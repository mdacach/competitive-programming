#include <bits/stdc++.h>

auto solve_testcase() -> void {
    int n, k;
    std::cin >> n >> k;
    std::vector<std::pair<int, int>> segments;
    for (int i = 0; i < n; ++i) {
        int l, r;
        std::cin >> l >> r;
        if (l <= k && k <= r) segments.emplace_back(l, r);
    }

    std::vector<int> points(55);
    // Can be improved with Delta Encoding, but as constraints are so low
    // it is not needed.
    for (auto [l, r]: segments) {
        for (int x = l; x <= r; ++x)
            points[x] += 1;
    }

    int max = 0;
    int second_max = 0;
    for (auto x: points) {
        if (x > max) {
            second_max = max;
            max = x;
        } else if (x > second_max) {
            second_max = x;
        }
    }
    std::cout << (points[k] == max && second_max < max ? "YES" : "NO") << '\n';
}

auto main() -> int32_t {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int t;
    std::cin >> t;
    while (t--) solve_testcase();
}