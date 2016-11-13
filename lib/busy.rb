require_relative 'busy/version'
require 'fiddle'
require 'rack'
require 'rack/builder'

module Busy
  def self.extract_rack_proxy(rack_proxy)
    puts "rack_proxy is a: #{rack_proxy.class}"
    puts "rack_proxy.@body is a: #{rack_proxy.instance_variable_get(:@body).class}"
    b = nil
    rack_proxy.each do |body|
      puts "body is a: #{rack_proxy.instance_variable_get(:@body).class}"
      b = body
    end

    b
  end
end

library = Fiddle::dlopen('target/debug/libbusy.dylib')

Fiddle::Function.new(library['initialize_busy'], [], Fiddle::TYPE_VOIDP).call
