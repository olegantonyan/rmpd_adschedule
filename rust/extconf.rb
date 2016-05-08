require 'mkmf'

create_makefile 'rmpd_adschedule'

makefile = "all:\n  cd rust/ && cargo build --release"
File.write('Makefile', makefile)
# system("cd #{File.dirname(__FILE__)} && cargo build --release")
