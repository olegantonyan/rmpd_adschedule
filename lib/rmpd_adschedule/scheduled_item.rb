require 'json'

module RmpdAdschedule
  class ScheduledItem
    class << self
      def from_hash(h)
        new(h['id'], h['begin_date'], h['end_date'], h['schedule'], h['distance'])
      end

      def array_from_hashes(arr)
        arr.map { |i| from_hash(i) }
      end
    end

    attr_reader :id, :begin_date, :end_date, :schedule, :distance

    def initialize(id, begin_date, end_date, schedule, distance)
      @id = id
      @begin_date = begin_date
      @end_date = end_date
      @schedule = schedule
      @distance = distance
    end

    def possible?
      distance > 0
    end

    def to_s
      "#ScheduledItem {id: #{id}, begin_date: #{begin_date}, end_date: #{end_date}, schedule: #{schedule}, distance: #{distance}}"
    end
  end
end
