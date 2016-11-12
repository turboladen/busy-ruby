require_relative 'busy/version'
require 'fiddle'
require 'rack'
require 'rack/builder'

module Busy
  # Your code goes here...
end

library = Fiddle::dlopen('target/debug/libbusy.dylib')

Fiddle::Function.new(library['initialize_busy'], [], Fiddle::TYPE_VOIDP).call
