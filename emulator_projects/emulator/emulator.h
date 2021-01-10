#pragma once

template<typename Pattern>
struct Rule {
	Pattern seek;
	Pattern replacement;
	size_t found;
	size_t missing;
};

// StateImpl should be a state_implementation (has constructor, find_and_replace, and size)
// list should be a rule_list (indexable array of rules)
template<typename StateImpl, typename RuleList>
class Emulator {
private:
	const size_t initial_rule;
	const size_t terminal_rule;
	const RuleList rules;
protected:
	Emulator(size_t initial_rule, size_t terminal_rule, RuleList rules)
		: initial_rule{ initial_rule },
		terminal_rule{ terminal_rule },
		rules{ rules } {}
public:
	int emulate(int a, int b) const {
		StateImpl state(a, b);
		size_t rule_num = initial_rule;
		while (rule_num != terminal_rule) {
			// perhaps instead should be const Rule<RuleList::Type>&
			const auto/*typename RuleList::Type*/& rule = rules[rule_num]; // can this be const auto&?
			bool is_found = state.find_and_replace(rule.seek, rule.replacement);
			rule_num = is_found ? rule.found : rule.missing;
		}
		return state.size();
	}
};