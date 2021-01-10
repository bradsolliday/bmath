#include "euclid_emulator.h"
#include "list_state.h"
#include "string_state.h"
#include "vector_state.h"
#include "performance_test.h"
#include <vector>
#include <iostream>
#include <string>
#include <ranges>
#include <filesystem>
#include <fstream>

template<typename EmulatorImpl, std::ranges::output_range<ArgPair> Range>
void output_performance(
	std::string file_name,
	EmulatorImpl em,
	Range args_list) {
	namespace fs = std::filesystem;
	const fs::path cwd = fs::current_path();
	const fs::path test_data_dir = cwd / "test_data";
	fs::create_directory(test_data_dir);
	fs::path test_file = test_data_dir / file_name;
	std::ofstream output_file_stream{ test_file };
	// Refactor measure_perfomance, so it only produces the measured data
	// and returns it as some data structure.
	// Seperate the printing to output_file_stream to another function
	measure_performance(output_file_stream, em, args_list);
}

int main() {
	EuclidEmulator<ListState<std::string>> list_euclid;
	EuclidEmulator<StringState> string_euclid;
	EuclidEmulator<VectorState<std::string>> vector_euclid;

	std::vector<ArgPair> args_list;
	for (int i = 1; i < 100; ++i) {
		for (int j = 1; j < 100; ++j) {
			args_list.emplace_back(ArgPair{ i, j });
		}
	}

	std::vector<ArgPair> args_path;
	for (int i = 1; i < 1000; ++i) {
		args_path.emplace_back(ArgPair{ i, 2 * i + 1 });
	}

	//output_performance("list_euclid.txt", list_euclid,
	//	               arg1_list, arg2_list);
	//output_performance("string_euclid.txt", string_euclid, args_list);
	std::cout << "starting fast one\n";
	output_performance("string_euclid_1d.txt", string_euclid, args_path);
	std::cout << "ending fast one\n";
	output_performance("list_euclid_1d.txt", list_euclid, args_path);
	std::cout << "On last one\n";
	output_performance("vector_euclid_1d.txt", vector_euclid, args_path);
	//output_performance("vector_euclid.txt", vector_euclid,
	//	               arg1_list, arg2_list);
}