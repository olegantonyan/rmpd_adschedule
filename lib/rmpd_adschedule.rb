require 'rmpd_adschedule/version'

require 'fiddle'
require 'fiddle/import'
require 'rake'

module RmpdAdschedule
  extend Fiddle::Importer

  begin
    dlload "#{File.dirname(__FILE__)}/../rust/target/release/librmpd_adschedule.so"
  rescue Fiddle::DLError
    dlload "#{File.dirname(__FILE__)}/../rust/target/debug/librmpd_adschedule.so"
  end

  extern 'void say_hello()'
end
