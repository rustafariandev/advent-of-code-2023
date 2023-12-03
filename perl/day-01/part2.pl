#!/bin/perl
#
$a = 0;
%lookup = (
    ( map { $_ => $_ } 0..9 ),
    (
        one   => 1,
        two   => 2,
        three => 3,
        four  => 4,
        five  => 5,
        six   => 6,
        seven => 7,
        eight => 8,
        nine  => 9,
      )
);
$a = 0;
while (<>) {
    /(one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9).*/;
    $s = $lookup{$1} * 10;
    /.*(one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9)/;
    $s += $lookup{$1};
    print "$s $_\n";
    $a+=$s;
}

print $a;
