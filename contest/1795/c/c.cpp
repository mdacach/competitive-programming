#include <bits/stdc++.h>

#define int int64_t

auto solve() -> void {
    int n;
    std::cin >> n;
    std::vector<int> tea_amounts(n + 1);
    std::vector<int> taster_capacity(n + 1);

    for (int i = 1; i <= n; ++i) std::cin >> tea_amounts[i];
    for (int i = 1; i <= n; ++i) std::cin >> taster_capacity[i];

    std::vector<int> prefix_sum_tasters(n + 1);
    for (int i = 1; i <= n; ++i)
        prefix_sum_tasters[i] = prefix_sum_tasters[i - 1] + taster_capacity[i];

    std::vector<int> difference_array(n + 2);

    std::vector<int> from_partial_drinks(n + 1);

    auto update_state = [&](int tea_index) {
        int need_to_remove = prefix_sum_tasters[tea_index - 1];

        int tea_amount = tea_amounts[tea_index];

        auto start = std::begin(prefix_sum_tasters) + tea_index;
        auto end = std::end(prefix_sum_tasters);
        auto last_full_taster = std::prev(std::upper_bound(start, end, tea_amounts[tea_index] + need_to_remove));
        int last_index = last_full_taster - std::begin(prefix_sum_tasters);

        // the next guy will drink partially
        int leftover_tea = tea_amount - (*last_full_taster - need_to_remove);
        if (leftover_tea > 0) {
            if (std::next(last_full_taster) != end) {
                from_partial_drinks[last_index + 1] += leftover_tea;
            }
        }

        difference_array[tea_index] += 1;
        difference_array[last_index + 1] -= 1;
    };

    for (int tea = 1; tea <= n; ++tea) {
        update_state(tea);
    }

    std::vector<int> answer(n + 1);
    int multiple_count = 0;
    for (int i = 1; i <= n; ++i) {
        multiple_count += difference_array[i];
        answer[i] += multiple_count * taster_capacity[i] + from_partial_drinks[i];
    }

    for (int i = 1; i <= n; ++i)
        std::cout << answer[i] << " ";
    std::cout << '\n';
}

int32_t main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int t;
    std::cin >> t;
    while (t--) {
        solve();
    }
}