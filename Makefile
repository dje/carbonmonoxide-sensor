.PHONY : openocd
openocd :
	openocd

.PHONY : run
run : openocd
	cargo run
