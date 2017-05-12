require './lib/busy'

file_path = ARGV[0]
abort 'gimme a file' unless file_path

Busy::Server.rackup(File.expand_path(file_path))
