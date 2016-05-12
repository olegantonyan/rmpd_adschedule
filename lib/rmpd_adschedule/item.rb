require 'json'

module RmpdAdschedule
  class Item
    DATE_FORMAT = '%d.%m.%Y'.freeze
    TIME_FORMAT = '%H:%M:%S'.freeze

    attr_accessor :id, :begin_date, :end_date, :begin_time, :end_time, :playbacks_count

    # rubocop: disable Metrics/ParameterLists
    def initialize(id, begin_date, end_date, begin_time, end_time, playbacks_count)
      self.id = id
      self.begin_date = date_indifferent(begin_date)
      self.end_date = date_indifferent(end_date)
      self.begin_time = time_indifferent(begin_time)
      self.end_time = time_indifferent(end_time)
      self.playbacks_count = playbacks_count
    end
    # rubocop: enable Metrics/ParameterLists

    def to_json
      to_hash.to_json
    end

    def to_hash
      { id: id,
        begin_date: begin_date,
        end_date: end_date,
        begin_time: begin_time,
        end_time: end_time,
        playbacks_count: playbacks_count }
    end

    private

    def time_indifferent(tm)
      return tm if tm.is_a?(String)
      tm.strftime(TIME_FORMAT)
    end

    def date_indifferent(dt)
      return dt if dt.is_a?(String)
      dt.strftime(DATE_FORMAT)
    end
  end
end
