require 'rmpd_adschedule/version'
require 'rmpd_adschedule/item'
require 'rmpd_adschedule/scheduled_item'
require 'rmpd_adschedule/rust'

module RmpdAdschedule
  def self.calculate(items)
    result = Rust.calculate(items.map(&:to_hash))
    ScheduledItem.array_from_hashes(result)
  end

  def self.say_hello
    arr = [
      # Item.new(1, '01.02.2016', '01.02.2016', '10:00:00', '11:00:00', 7),
      # Item.new(1, '01.02.2016', '01.02.2016', '10:00:00', '16:00:00', 16),
      # Item.new(1, '02.02.2016', '13.02.2016', '10:00:00', '18:00:00', 100),
      # Item.new(1, '21.02.2016', '19.03.2016', '10:00:00', '18:00:00', 100)
    ]
    1.upto(4) { arr << Item.new(rand(1..9000), '01.02.2016', '03.02.2016', '10:00:00', '19:00:00', 300) }
    r = Rust.calculate(arr.map(&:to_hash))

    puts ScheduledItem.array_from_hashes(r)
  end
end
