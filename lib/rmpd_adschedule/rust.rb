require 'fiddle'

module RmpdAdschedule
  module Rust
    @lib = Fiddle.dlopen("#{File.dirname(__FILE__)}/../../rust/target/release/librmpd_adschedule.so")
    @ffi_calculate = Fiddle::Function.new(@lib['ffi_calculate'], [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOIDP)

    def self.calculate(data)
      json = data.to_json
      result = calculate!(json)
      JSON.parse(result)
    end

    def self.calculate!(jsondata)
      raise TypeError, "wrong argument type #{object.class} (expected String)" unless jsondata.is_a? String
      ptr = @ffi_calculate.call(jsondata)
      result = ptr.to_s
      LibC.free(ptr)
      result
    end

    module LibC
      @libc = if RUBY_PLATFORM =~ /_64/
                Fiddle.dlopen('/lib64/libc.so.6')
              else
                Fiddle.dlopen('/lib/libc.so.6')
              end
      @libc_free = Fiddle::Function.new(@libc['free'], [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOID)

      def self.free(ptr)
        @libc_free.call(ptr)
      end
    end
  end
end
