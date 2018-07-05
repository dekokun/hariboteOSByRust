#!/usr/bin/ruby

file = open(ARGV[0], "r")

print "pub static HANKAKU:[[u8; 16]; 256] = [\n"

file.each do |line|
  if line =~ /^char 0x/
    print "["
    file.each do |data|
      break if /^\s*$/ =~ data
        printf "%#x, ", "0b" + data.chomp.tr(".","0").tr("*","1")
    end
    print "],\n"
  end
end

print "];\n"
