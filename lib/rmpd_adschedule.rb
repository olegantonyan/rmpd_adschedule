require 'rmpd_adschedule/version'
require 'rmpd_adschedule/item'

require 'ffi'
require 'rake'

module RmpdAdschedule
  def self.say_hello
    i = Item.new(2, '01.02.2016', '04.02.2016', '10:00:00', '18:00:00', 14)
    j = Item.new(5, '01.02.2016', '04.02.2016', '9:00:00', '18:00:00', 14)
    arr = [i, j].map(&:to_hash)
    r = Rust.calculate(arr)
    puts r
  end

  module Rust
    class << self
      extend FFI::Library
      ffi_lib "#{File.dirname(__FILE__)}/../rust/target/release/librmpd_adschedule.so"
      attach_function :ffi_calculate, [:string], :pointer

      def calculate(data)
        json = data.to_json
        result = calculate!(json)
        JSON.parse(result)
      end

      private

      def calculate!(input_data)
        raise TypeError, "wrong argument type #{object.class} (expected String)" unless input_data.is_a? String
        ptr = ffi_calculate(input_data)
        result = String.new(ptr.read_string)
        LibC.free(ptr)
        result
      end
    end

    module LibC
      extend FFI::Library
      ffi_lib FFI::Library::LIBC
      attach_function :free, [:pointer], :void
    end
  end
end
