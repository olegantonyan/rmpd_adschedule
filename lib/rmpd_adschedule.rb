require 'rmpd_adschedule/version'
require 'rmpd_adschedule/item'
require 'rmpd_adschedule/rust'

module RmpdAdschedule
  def self.say_hello
    arr = [
      Item.new(1, '01.02.2016', '01.02.2016', '10:00:00', '18:00:00', 14),
      Item.new(1, '02.02.2016', '10.02.2016', '10:00:00', '18:00:00', 14),
      Item.new(1, '02.02.2016', '13.02.2016', '10:00:00', '18:00:00', 14),
      Item.new(1, '21.02.2016', '19.03.2016', '10:00:00', '18:00:00', 14)
    ]
    r = Rust.calculate(arr.map(&:to_hash))
    puts r
  end
end
