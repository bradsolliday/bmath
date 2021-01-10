#pragma once

#include <string>

class StringState {
private:
	std::string state;
public:
	StringState(int m, int n) : state() {
		state.reserve(m + n);
		state.resize(m, 'a');
		state.resize(m + n, 'b');
	}
	bool find_and_replace(
		const std::string& seek,
		const std::string& replacement
	) {
		size_t location = state.find(seek);
		if (location == std::string::npos && state.size() != 0)
			return false;
		state.replace(location, seek.size(), replacement);
		return true;
	}
	size_t size() const {
		return state.size();
	}
};