require 'rmpd_adschedule/version'
require 'rmpd_adschedule/item'
require 'rmpd_adschedule/rust'

module RmpdAdschedule
  def self.say_hello
    arr = (1..1000).map { |i| Item.new(i, '01.02.2016', '04.02.2016', '10:00:00', '18:00:00', 14) }.map(&:to_hash)
    r = Rust.calculate(arr)
    puts r
    loop do
      a = Rust.calculate(arr)
      a
    end
  end
end
