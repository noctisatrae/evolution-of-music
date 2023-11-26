pipeline:
	echo "[PIPELINE] ===================================="
	make snapshot && make clean && make venv && make analysis 

snapshot:
	echo "[SNAPSHOT] ===================================="
	cargo run

clean:
	echo "[CLEANING] ===================================="
	cargo run -- --mode cleaning

analysis: 
	echo "[ANALYSING] ===================================="
	python ./snapshot/analysis.py

venv:
	echo "[STARTING VENV] ===================================="
	source .venv/bin/activate