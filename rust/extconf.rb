require 'mkmf'
# have_library('rmpd_adschedule')
create_makefile 'rust/rmpd_adschedule'

system("cd #{File.dirname(__FILE__)} && cargo build --release")
