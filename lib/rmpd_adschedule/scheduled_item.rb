require 'json'

module RmpdAdschedule
  class ScheduledItem
    attr_accessor :id, :begin_date, :end_date, :schedule, :overlap

    def initialize(id, begin_date, end_date, schedule, overlap)
      self.id = id
      self.begin_date = begin_date
      self.end_date = end_date
      self.schedule = schedule
      self.overlap = overlap
    end

    def to_s
      "#ScheduledItem {id: #{id}, begin_date: #{begin_date}, end_date: #{end_date}, schedule: #{schedule}, overlap: #{overlap}}"
    end

    def self.from_hash(h)
      new(h['id'], h['begin_date'], h['end_date'], h['schedule'], h['overlap'])
    end

    def self.array_from_hashes(arr)
      arr.map { |i| from_hash(i) }
    end
  end
end
