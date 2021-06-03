FILE_DIR=MyShellFiles

all:

       ifeq ($(wildcard $(FILE_DIR)),)
		mkdir $(FILE_DIR)
		mv Cargo.toml $(FILE_DIR)
		mv main.rs $(FILE_DIR)
		cargo new shell
		cp -rf $(FILE_DIR)/Cargo.toml shell
		cp -rf $(FILE_DIR)/main.rs shell/src
		cd shell && cargo build && cargo run
       else
		cd shell && cargo run
       endif

