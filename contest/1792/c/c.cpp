#include <bits/stdc++.h>

auto solve_testcase() -> void {
    int n;
    std::cin >> n;
    std::vector<int> values(n);
    for (auto &x: values) std::cin >> x;

    if (n % 2 == 0) {
        int middle_left = (n + 1) / 2;
        int middle_right = middle_left + 1;
        int start, end;
        for (int i = 0; i < n; ++i) {
            if (values[i] == middle_left) {
                start = i;
            }
            if (values[i] == middle_right) {
                end = i;
            }
        }
        bool found_left = false;
        bool found_right = false;
        int counter = 0;
        while (start < end) {
            if (start < 0) break;
            if (end >= n) break;

            if (values[start] == middle_left) {
                middle_left -= 1;
                found_left = true;
            }
            if (values[end] == middle_right) {
                middle_right += 1;
                found_right = true;
            }
            if (found_left && found_right) {
                counter += 1;
                found_left = false;
                found_right = false;
            }
            if (!found_left)
                start -= 1;
            if (!found_right)
                end += 1;
        }
        int remaining = n - counter * 2;
        std::cout << remaining / 2 << '\n';
    } else {
        int middle_left = (n + 1) / 2;
        int middle_right = middle_left;
        int start, end;
        for (int i = 0; i < n; ++i) {
            if (values[i] == middle_left) {
                start = i - 1;
                end = i + 1;
            }
        }
        bool found_left = false;
        bool found_right = false;
        int counter = 1;
        middle_left -= 1;
        middle_right += 1;
        while (true) {
            if (start < 0) break;
            if (end >= n) break;

            if (values[start] == middle_left) {
                middle_left -= 1;
                found_left = true;
            }
            if (values[end] == middle_right) {
                middle_right += 1;
                found_right = true;
            }
            if (found_left && found_right) {
                counter += 1;
                found_left = false;
                found_right = false;
            }
            if (!found_left)
                start -= 1;
            if (!found_right)
                end += 1;
        }
        int remaining = n - (counter * 2 - 1);
        std::cout << remaining / 2 << '\n';
    }
}

auto main() -> int32_t {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int t;
    std::cin >> t;
    while (t--) solve_testcase();
}
