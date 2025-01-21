#!/bin/sh

native(){
	find \
		./src \
		-type f \
		-name '*.rs' |
		xargs \
			--max-args=2 \
			--max-procs=1 \
			./rs-find-empty-line
}

runtime_wazero(){
	find \
		./src \
		-type f \
		-name '*.rs' |
		sed \
			's,^,/guest.d/,' |
		xargs \
			--max-args=2 \
			--max-procs=1 \
			wazero \
				run \
				-mount "${PWD}:/guest.d:ro" \
				./rs-find-empty-line.wasm
}

native
#runtime_wazero
