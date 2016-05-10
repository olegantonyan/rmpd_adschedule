require 'json'

module RmpdAdschedule
  class Item
    attr_accessor :id, :begin_date, :end_date, :begin_time, :end_time, :playbacks_count

    # rubocop: disable Metrics/ParameterLists
    def initialize(id, begin_date, end_date, begin_time, end_time, playbacks_count)
      self.id = id
      self.begin_date = begin_date
      self.end_date = end_date
      self.begin_time = begin_time
      self.end_time = end_time
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
  end
end
