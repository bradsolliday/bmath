#pragma once

#include "emulator.h"
#include <array>
#include <string>
#include <assert.h>

template<typename StateImpl>
class EuclidEmulator : Emulator<StateImpl, std::array<Rule<std::string>, 5>> {
public:
	EuclidEmulator() :
		Emulator<StateImpl, std::array<Rule<std::string>, 5>>(
			0, 5, /*std::array<Rule<std::string>, 5>*/{
			Rule<std::string>{ "ab",  "", 1, 2 },
			Rule<std::string>{ "", "c", 0, 0 },
			Rule<std::string>{ "a", "b", 2, 3 },
			Rule<std::string>{ "c", "a", 3, 4 },
			Rule<std::string>{ "b", "b", 0, 5 }
	}) {}
	int emulate(int a, int b) const {
		assert(a > 0 && b > 0);
		Rule<std::string> r{ "ab", "", 1, 2 };
		return Emulator<StateImpl, std::array<Rule<std::string>, 5>>::
			emulate(a, b);
	}
};