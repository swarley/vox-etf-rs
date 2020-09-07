# frozen_string_literal: true

require 'vox/etf/version'
require 'rutie'

module Vox
  module ETF
    Rutie.new(:vox_etf).init('Init_vox_etf', File.expand_path(File.join(__dir__, '..')))
  end
end
