# require 'mkmf'
# create_makefile 'rmpd_adschedule'
# File.write('Makefile', "all:\n\tcargo build --release\nclean:\n\trm -rf target\ninstall: ;")

raise 'You have to install Rust with Cargo (https://www.rust-lang.org/)' if !system('cargo --version') || system('rustc --version')
