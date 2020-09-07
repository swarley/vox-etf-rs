require 'bundler/gem_tasks'
require 'rspec/core/rake_task'

RSpec::Core::RakeTask.new(:spec)

require 'thermite/tasks'

# @!visibility private
module Thermite
  # @!visibility private
  class Config
    def ruby_extension_path
      ruby_path('lib/vox/etf', shared_library)
    end
  end
end

Thermite::Tasks.new

task test: %w[thermite:build thermite:test spec]
