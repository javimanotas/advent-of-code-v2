run:
	cargo run --release -p aoc_$(YEAR) --bin day$(DAY) -- $(YEAR) $(DAY)

run-debug:
	cargo run -p aoc_$(YEAR) --bin day$(DAY) -- $(YEAR) $(DAY)
