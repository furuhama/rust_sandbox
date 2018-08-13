#!/usr/bin/env ruby

File.open('seed_data.txt', 'w') do |file|
  1_000_000.times do |idx|
    file.puts("test #{idx}")
  end
end
