setup:
	cargo run -p aoc_setup -- $(YEAR) $(DAY)

run:
	cargo run --release -p aoc_$(YEAR) --bin day$(DAY) -- $(YEAR) $(DAY)

run-debug:
	cargo run -p aoc_$(YEAR) --bin day$(DAY) -- $(YEAR) $(DAY)
