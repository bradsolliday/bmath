#pragma once

#include <vector>
#include <algorithm>
#include <iterator>

template<typename CharIterable>
class VectorState {
private:
	std::vector<char> state;
public:
	VectorState(int m, int n) : state() {
		state.reserve(m + n);
		state.resize(m, 'a');
		state.resize(m + n, 'b');
	}
	bool find_and_replace(
		const CharIterable& seek,
		const CharIterable& replacement) {
		using std::begin, std::end;

		const auto location = std::search(
			begin(state), end(state),
			begin(seek), end(seek));
		if (location == end(state) && state.size() != 0)
			return false;

		
		std::vector<char>::iterator after_erased =
			state.erase(location, [&] {
			auto seek_location_end = location;
			std::advance(seek_location_end, seek.size());
			return seek_location_end;
			}());
		state.insert(after_erased, begin(replacement), end(replacement));
		return true;
	}
	size_t size() {
		return state.size();
	}
};