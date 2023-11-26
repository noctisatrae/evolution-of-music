BIN=.venv/bin/

pipeline:
	echo "[PIPELINE]"
	make snapshot && make clean && make analysis 

snapshot:
	echo "[SNAPSHOT]"
	cargo run

clean:
	echo "[CLEANING]"
	cargo run -- --mode cleaning

analysis: 
	echo "[ANALYSING]"
	$(BIN)/python ./snapshot/analysis.py

requirements:
	echo "[INSTALLING REQUIREMENTS]"
	$(BIN)/pip install -r ./snapshot/requirements.txt