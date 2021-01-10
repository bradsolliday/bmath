#pragma once

#include <iostream>
#include <ranges>
#include <chrono>
#include <numeric>
#include <cassert>
#include <utility>

template<typename EmulatorImpl>
std::chrono::duration<double> measure_gcd_time(
	EmulatorImpl em,
	int arg1,
	int arg2
)
{
	auto start = std::chrono::steady_clock::now();
	int emulated_gcd = em.emulate(arg1, arg2);
	auto end = std::chrono::steady_clock::now();

	assert(emulated_gcd == std::gcd(arg1, arg2));

	return end - start;
}

struct ArgPair {
	int arg1;
	int arg2;
};

template<
	typename EmulatorImpl,
	std::ranges::output_range<ArgPair> Range>
std::ostream& measure_performance(
	std::ostream& os,
	EmulatorImpl em,
	Range args_list)
{

	for (auto [arg1, arg2] : args_list) {
		double time_elapsed = measure_gcd_time(em, arg1, arg2).count();
		os << '\n' << arg1 << ',' << arg2 << ',' << time_elapsed;
	}
	return os;
}