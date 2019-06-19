require 'time'

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
      #Item.new(1, '25.05.2016', '25.06.2016', '09:00:00', '09:20:00', 4),
      #Item.new(2, '25.05.2016', '25.06.2016', '09:00:00', '09:20:00', 2)
      Item.new(1, '25.05.2016', '25.06.2016', '09:00:00', '23:00:00', 3),
      Item.new(2, '25.05.2016', '25.06.2016', '09:00:00', '23:00:00', 6),
      #Item.new(3, '25.05.2016', '25.06.2016', '09:00:00', '23:00:00', 10),
      #Item.new(4, '25.05.2016', '25.06.2016', '09:00:00', '23:00:00', 5),
      #Item.new(5, '25.05.2016', '25.06.2016', '09:00:00', '23:00:00', 7),
    ]
    make_item = -> (id, beg, en, per_hour) {
      b = Time.parse(beg)
      e = Time.parse(en)
      c = (e - b).to_i / 3600 * per_hour
      RmpdAdschedule::Item.new(id, '01.01.1970', '01.01.1970', b, e, c)
    }

    bar = [
      ['11:00', '16:00', 6],
      ['11:00', '23:00', 4],
      ['11:00', '23:00', 4],
      ['16:00', '23:00', 6],
      ['16:00', '23:00', 4],
      ['11:00', '23:00', 2],
      ['11:00', '23:00', 4],
      ['11:00', '23:00', 6],
      ['11:00', '23:00', 10],
      ['11:00', '14:00', 2],
      ['11:00', '14:00', 2],
      ['18:00', '23:00', 2],
      ['15:00', '17:00', 4],
      ['19:00', '23:00', 2],
      ['20:00', '23:00', 4]
    ].map.with_index(1) { |i, idx| make_item.call(idx, *i) }

    cafe2 = [
      ['09:00', '11:00', 8],
      ['15:00', '17:00', 8],
      ['09:00', '20:00', 4],
      ['09:00', '11:00', 6],
      ['15:00', '18:00', 6],
      ['12:00', '15:00', 4],
      ['18:00', '20:00', 7],
      ['11:00', '18:00', 3],
      ['15:00', '17:00', 7],
      ['18:00', '20:00', 5],
      ['09:00', '20:00', 2],
      ['10:00', '14:30', 3],
      ['17:45', '20:00', 3],
      ['16:45', '19:15', 4],
      ['12:30', '16:50', 4]
    ].map.with_index(1) { |i, idx| make_item.call(idx, *i) } # 28 сек, не удаётся раскидать

    cafe = [
      ['09:00', '11:00', 6],
      ['09:00', '20:00', 4],
      ['09:00', '11:00', 6],
      ['12:00', '15:00', 6],
      ['11:00', '18:00', 4],
      ['15:00', '17:00', 7],
      ['18:00', '20:00', 6],
      ['09:00', '20:00', 2],
      ['09:00', '13:30', 3],
      ['16:45', '19:15', 5],
      ['12:30', '16:50', 3]
    ].map.with_index(1) { |i, idx| make_item.call(idx, *i) } # 16 сек, не удаётся раскидать

    restaurant = [
      ['09:00', '13:30', 4],
      ['16:00', '18:00', 4],
      ['09:00', '13:30', 6],
      ['09:00', '13:30', 7],
      ['16:00', '20:00', 7],
      ['13:00', '15:00', 5],
      ['19:00', '20:00', 5],
      ['13:30', '16:00', 3],
      ['13:30', '23:00', 5],
      ['12:00', '20:00', 1],
      ['18:00', '21:00', 9],
      ['13:00', '16:55', 8],
      ['12:30', '16:50', 4],
      ['19:00', '23:00', 4],
      ['17:00', '21:00', 6],
      ['17:00', '21:30', 3],
      ['17:30', '22:00', 3]
    ].map.with_index(1) { |i, idx| make_item.call(idx, *i) } # не удаётся раскидать

    supermarket = [
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4],
      ['09:00', '23:00', 4]
    ].map.with_index(1) { |i, idx| make_item.call(idx, *i) } # не удаётся раскидать, через несколько минут прибил процесс т.к. надоело ждать :)

    #arr = supermarket

    r = Rust.calculate(arr.map(&:to_hash))

    puts ScheduledItem.array_from_hashes(r)
  end
end
