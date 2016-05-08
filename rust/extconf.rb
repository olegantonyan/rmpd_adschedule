require 'mkmf'

create_makefile 'rmpd_adschedule'

File.write('Makefile', "all:\n\tcargo build --release\nclean:\n\trm -rf target\ninstall:\n\techo 'install is not required'")
# system("cd #{File.dirname(__FILE__)} && cargo build --release")
