#include <bits/stdc++.h>

auto solve_testcase() -> void {
    int n, m;
    std::cin >> n >> m;
    std::string first_tower;
    std::cin >> first_tower;
    std::string second_tower;
    std::cin >> second_tower;

    int bad_pairs = 0;
    for (int i = 1; i < n; ++i)
        if (first_tower[i] == first_tower[i - 1]) bad_pairs += 1;
    for (int i = 1; i < m; ++i)
        if (second_tower[i] == second_tower[i - 1]) bad_pairs += 1;
    if (bad_pairs == 0) {
        std::cout << "YES\n";
        return;
    }
    if (bad_pairs == 1 && first_tower.back() != second_tower.back()) {
        std::cout << "YES\n";
        return;
    }
    std::cout << "NO\n";
}

auto main() -> int32_t {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int t;
    std::cin >> t;
    while (t--) solve_testcase();
}