require 'rmpd_adschedule/version'
require 'rmpd_adschedule/item'
require 'rmpd_adschedule/rust'

module RmpdAdschedule
  def self.say_hello
    arr = [
      #Item.new(1, '01.02.2016', '01.02.2016', '10:00:00', '11:00:00', 7),
      #Item.new(1, '01.02.2016', '01.02.2016', '10:00:00', '16:00:00', 16),
      #Item.new(1, '02.02.2016', '13.02.2016', '10:00:00', '18:00:00', 100),
      #Item.new(1, '21.02.2016', '19.03.2016', '10:00:00', '18:00:00', 100)
    ]
    1.upto(200) { arr << Item.new(rand(1..9000), '01.02.2016', '01.02.2016', '10:00:00', '19:00:00', rand(10..100)) }
    r = Rust.calculate(arr.map(&:to_hash))
    puts r
  end
end
