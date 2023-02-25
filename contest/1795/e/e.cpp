#include <iostream>
#include <vector>
#include <cassert>
#include <algorithm>

#define int int64_t

struct ConsecutiveSegment {
    int left, right, length;

    void add_next_consecutive_value() {
        right += 1;
        length += 1;
    }

    int cost_for_decreasing(int x) {
        int cost = 0;
        // all that are very positive will pay X
        // all numbers which are >= x will pay the full x
        int greater_or_equal_than_x = [&]() -> int64_t {
            if (right < x) return 0;
            if (left > x) return (right - left + 1) * x;
            return (right - x + 1) * x;
        }();

        // some will pay less
        // all that are between 0 and x will pay their sum
        int positive_smaller_than_x = [&]() -> int64_t {
            if (left >= x) {
                return 0;
            }
            int new_left = (left < 0 ? 0 : left);
            // sum from new_left to x or right
            int new_right = (right >= x ? x - 1 : right);
            int cost = new_right * (new_right + 1) / 2;
            int to_subtract = (new_left - 1) * new_left / 2;
            return cost - to_subtract;
        }();

        return greater_or_equal_than_x + positive_smaller_than_x;

        // some will pay nothing
    }

    void decrease_from_all(int x) {
        right -= x;
        left -= x;
    }
};

std::pair<ConsecutiveSegment, int> merge(ConsecutiveSegment &s1, const ConsecutiveSegment &s2) {
    // we need to merge both segments together.
    // we need to decrease from s1 so that they can become consecutive.
    int current_value = s1.right;
    int desired_value = s2.left - 1; // so that they fit perfectly
    assert(desired_value <= current_value);
    int to_decrease = current_value - desired_value;
    int cost = s1.cost_for_decreasing(to_decrease);
    s1.decrease_from_all(to_decrease);
    // now we merge those
    return {ConsecutiveSegment{s1.left, s2.right,
                               s1.length + s2.length}, cost};
}

std::vector<int> compute_cost_for_increasing(const std::vector<int> &values) {
    const int N = std::size(values);

    // compute the cost to make the sequence increasing until point P
    std::vector<int> cost_for_increasing_sequence_until(N);

    std::vector<ConsecutiveSegment> active_segments;
    auto first_segment = ConsecutiveSegment{
            values[0], values[0], 1
    };
    active_segments.push_back(first_segment);

    for (int current_index = 1; current_index < N; ++current_index) {
        // the costs are always increasing, so at least we have the same cost as before.
        cost_for_increasing_sequence_until[current_index] = cost_for_increasing_sequence_until[current_index - 1];

        int value = values[current_index];
        auto &last_segment = active_segments.back();
        if (value > last_segment.right) {
            // it's all good. with this value, the sequence still continues increasing.
            // we can just take it then.
            if (value == last_segment.right + 1) {
                // and it is even the next value exactly, we can just append it to this segment!
                last_segment.add_next_consecutive_value();

                // there's no additional cost, we are already good.
            } else {
                assert(value > last_segment.right);
                // this will be the beginning of a new Segment

                auto this_segment = ConsecutiveSegment{value, value, 1};
                active_segments.push_back(this_segment);

                // there's no additional cost.
            }
        } else {
            // we have a cost for decreasing this segment.
            int to_decrease = last_segment.right - value + 1;
            cost_for_increasing_sequence_until[current_index] += last_segment.cost_for_decreasing(to_decrease);
            last_segment.decrease_from_all(to_decrease);

            assert(last_segment.right + 1 == value);
            last_segment.add_next_consecutive_value();

            // we will need to do some house-cleaning now. in order for the sequence to be increasing,
            // we need to decrease 1 or many ConsecutiveSegments.
            if (std::size(active_segments) == 1) {
                // we only have one segment to worry about. this will be a constant time operation.
                // no additional costs, we are done already.
            } else {
                assert(std::size(active_segments) > 0);
                // we may need to change multiple segments.

                // we may have fucked up the remaining of the segments.
                // active_segments: (s1) (s2), if s1.right < s2.left - 1, we are good
                // otherwise, we must merge these two segments
                while (std::size(active_segments) >= 2) {
                    int size = std::size(active_segments);
                    auto &last_segment = active_segments[size - 1];
                    auto &previous_segment = active_segments[size - 2];
                    if (previous_segment.right < last_segment.left - 1) {
                        // we are OK. no need to change this guy
                        // TODO: compute cost here
                        break;
                    } else {
                        auto [new_segment, cost] = merge(previous_segment, last_segment);
                        cost_for_increasing_sequence_until[current_index] += cost;
                        active_segments.pop_back(); // remove last
                        active_segments.pop_back(); // remove previous
                        active_segments.push_back(new_segment);
                    }
                }
            }

        }
    }
    return cost_for_increasing_sequence_until;
}

int32_t main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    int t;
    std::cin >> t;
    while (t--) {
        int n;
        std::cin >> n;
        std::vector<int> v(n);
        for (auto &x: v) std::cin >> x;

        auto original = compute_cost_for_increasing(v);
        auto r = v;
        std::reverse(std::begin(r), std::end(r));
        auto reversed = compute_cost_for_increasing(r);

        int answer = 1e18;
        for (int last_monster = 0; last_monster < n; ++last_monster) {
            int cost_prefix = original[last_monster];
            int cost_suffix = reversed[n - last_monster - 1];
            answer = std::min(answer, cost_prefix + cost_suffix + v[last_monster]);
        }
        std::cout << answer << '\n';
    }
}