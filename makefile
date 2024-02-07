run:
	rustc ${script} -o cses && ./cses > out.txt

sol:
	cargo run --bin solution_file < input