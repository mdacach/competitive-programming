#include <bits/stdc++.h>

auto solve_testcase() -> void {
    int a1, a2, a3, a4;
    std::cin >> a1 >> a2 >> a3 >> a4;
    int total = a1 + a2 + a3 + a4;

    int max = 0;
    max += a1;
    int alice_mood = a1;
    int bob_mood = a1;

    if (a1 >= 1) {
        int neutral = std::min(a2, a3);
        max += 2 * neutral;
        a2 -= neutral;
        a3 -= neutral;
    }

    if (a2 <= a1) {
        max += a2;
        alice_mood -= a2;
        bob_mood += a2;
    } else {
        max += a1;
        alice_mood -= a1;
        bob_mood += a1;
    }

    if (a3 <= a1) {
        max += a3;
        bob_mood -= a3;
        alice_mood += a3;
    } else {
        max += a1;
        bob_mood -= a1;
        alice_mood += a1;
    }

    if (a4 <= std::min(alice_mood, bob_mood)) {
        max += a4;
    } else {
        max += std::min(alice_mood, bob_mood);
    }
    if (max != total) {
        max += 1;
    }
    std::cout << max << '\n';
}

auto main() -> int32_t {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int t;
    std::cin >> t;
    while (t--) solve_testcase();
}
