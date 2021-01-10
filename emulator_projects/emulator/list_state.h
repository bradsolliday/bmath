#pragma once

#include <algorithm>
#include <list>

template<typename CharIterable>
class ListState {
private:
	std::list<char> state;

	void copy_before(
		std::list<char>::iterator location,
		const CharIterable& insertion
	) {
		for (char c : insertion)
			state.insert(location, c);
	}

	void remove_after(
		std::list<char>::iterator location,
		const CharIterable& deletion
	) {
		auto location_end = [&] {
			auto out = location;
			for (auto x : deletion)
				++out;
			return out;
		}();
		state.erase(location, location_end);
	}

public:
	ListState(int m, int n) : state(m, 'a') {
		state.resize(m + n, 'b');
	}

	bool find_and_replace(
		const CharIterable& seek,
		const CharIterable& replacement)
	{
		using std::begin, std::end;

		const auto location = std::search(
			begin(state), end(state), // search within
			begin(seek), end(seek)); // search for
		if (location == end(state) && state.size() != 0)
			return false;

		// If here, seek has been found starting at location
		copy_before(location, replacement);
		remove_after(location, seek);
		return true;
	}
	size_t size() const { return state.size(); }
};