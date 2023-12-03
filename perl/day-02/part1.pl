#!/bin/perl
#
$a = 0;
use Data::Dumper;
GAME: while (<>) {
    chomp($_);
    my ($game, $rest) = split(/\: /, $_, 2);
    for my $draw (split(/; /, $rest)) {
        my %draw = (
            (map { $_ => 0} qw(red green blue)) 
        );
        for my $spec (split(/\s*,\s*/, $draw)) {
            my ($count, $color) = split(" ", $spec, 2);
            $draw{$color} = $count;
        }

        if ($draw{red} > 12 || $draw{green} > 13 || $draw{blue} > 14 ){
            next GAME;
        }

    }
    my (undef, $num) = split(" ", $game);
    $a += $num;
}
print "$a\n";
