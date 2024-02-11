run:
	rustc ${script} -o cses && ./cses > out.txt

solve:
	cargo run --bin solution_file < input

test:
	cargo run --bin solution_file < inputs/${test}.txt \
	> outputs/${test}.txt
