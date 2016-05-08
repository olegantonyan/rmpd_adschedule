require 'mkmf'

create_makefile 'rmpd_adschedule'

File.write('Makefile', "all:\n\tcargo build --release")
# system("cd #{File.dirname(__FILE__)} && cargo build --release")
