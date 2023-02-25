#include <bits/stdc++.h>

const int N = 2 * 100'005;

bool is_black[N];
int remaining_moves[N];
int max_distance_to_white_below[N];

std::vector<int> adj[N];

bool is_possible;

void visit(int current, int parent = -1) {
    int max_moves_down = 0;
    for (auto neighbor: adj[current]) {
        if (neighbor == parent) continue;
        visit(neighbor, current);
        if (is_black[neighbor]) continue;
        max_moves_down = std::max(max_moves_down, max_distance_to_white_below[neighbor] + 1);
    }
    max_distance_to_white_below[current] = max_moves_down;
    if (remaining_moves[current] == 0) return; // no chip left to move here
    if (max_moves_down >= remaining_moves[current])
        return; // we can move this chip down, which will not hurt any others

    // otherwise, we are forced to move it to its parent
    if (parent == -1) { // no parent to move to
        is_possible = false;
        return;
    }
    if (is_black[parent]) { // parent had a chip already
        is_possible = false;
        return;
    }
    remaining_moves[parent] = remaining_moves[current] - 1;
    is_black[parent] = true;
}

void solve() {
    int n;
    std::cin >> n;
    for (int i = 0; i < n - 1; ++i) {
        int a, b;
        std::cin >> a >> b;
        adj[a].push_back(b);
        adj[b].push_back(a);
    }

    std::vector<int> starting_chips;
    int number_chips;
    std::cin >> number_chips;
    for (int i = 0; i < number_chips; ++i) {
        int v;
        std::cin >> v;
        is_black[v] = true;
        starting_chips.push_back(v);
    }

    auto populate_number_moves = [&](int total_moves) {
        // figure out how many moves each chip does
        if (total_moves % number_chips == 0) {
            // each chip moves the same amount
            for (const auto v: starting_chips) {
                remaining_moves[v] = total_moves / number_chips;
            }
        } else {
            // some chips will move one less
            for (const auto v: starting_chips) {
                remaining_moves[v] = total_moves / number_chips;
            }
            int leftover_moves = total_moves % number_chips;
            assert(leftover_moves < std::size(starting_chips));
            for (int i = 0; i < leftover_moves; ++i)
                remaining_moves[starting_chips[i]] += 1;
        }
    };

    int arbitrary_root = 1;

    auto check = [&](int total_moves) {
        populate_number_moves(total_moves);
        is_possible = true;
        visit(arbitrary_root);
        return is_possible;
    };


    int l = 0;
    int r = n + 1;
    while (r - l > 1) {
        int m = l + (r - l) / 2;
        if (check(m)) {
            l = m;
        } else {
            r = m;
        }

        // clean up for same tree
        for (int v = 1; v <= n; ++v) {
            remaining_moves[v] = 0;
            is_black[v] = false;
        }
        for (auto v: starting_chips) is_black[v] = true;
    }
    std::cout << l << '\n';

    // clean up for next test
    for (int v = 1; v <= n; ++v) {
        is_black[v] = false;
        remaining_moves[v] = 0;
        max_distance_to_white_below[v] = 0;
        adj[v].clear();
    }
}

int main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int t;
    std::cin >> t;
    while (t--) solve();

}