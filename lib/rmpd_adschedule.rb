require 'rmpd_adschedule/version'

require 'fiddle'
require 'fiddle/import'

module RmpdAdschedule
  extend Fiddle::Importer

  begin
    dlload './rust/target/release/librmpd_adschedule.so'
  rescue Fiddle::DLError
    dlload './rust/target/debug/librmpd_adschedule.so'
  end

  extern 'void say_hello()'
end
