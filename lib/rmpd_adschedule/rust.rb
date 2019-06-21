require 'fiddle'

module RmpdAdschedule
  module Rust
    @lib = Fiddle.dlopen("#{File.dirname(__FILE__)}/../../rust/target/release/librmpd_adschedule.so")
    @ffi_calculate = Fiddle::Function.new(@lib['ffi_calculate'], [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOIDP)
    @ffi_free = Fiddle::Function.new(@lib['ffi_free'], [Fiddle::TYPE_VOIDP], Fiddle::TYPE_VOID)

    module_function

    def calculate(data)
      json = data.to_json
      result = calculate!(json)
      JSON.parse(result)
    end

    def calculate!(jsondata)
      raise TypeError, "wrong argument type #{jsondata.class} (expected String)" unless jsondata.is_a?(String)
      ptr = @ffi_calculate.call(jsondata)
      result = ptr.to_s
      @ffi_free.call(ptr)
      result
    end
  end
end
