require 'json'

module RmpdAdschedule
  class ScheduledItem
    class << self
      def from_hash(h)
        new(h['id'], h['begin_date'], h['end_date'], h['schedule'], h['overlap'])
      end

      def array_from_hashes(arr)
        arr.map { |i| from_hash(i) }
      end
    end

    attr_reader :id, :begin_date, :end_date, :schedule, :overlap

    def initialize(id, begin_date, end_date, schedule, overlap)
      @id = id
      @begin_date = begin_date
      @end_date = end_date
      @schedule = schedule
      @overlap = overlap
    end

    def to_s
      "#ScheduledItem {id: #{id}, begin_date: #{begin_date}, end_date: #{end_date}, schedule: #{schedule}, overlap: #{overlap}}"
    end
  end
end
