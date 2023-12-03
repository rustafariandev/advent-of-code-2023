#!/bin/perl
#
$a = 0;
use Data::Dumper;
GAME: while (<>) {
    chomp($_);
    my ($game, $rest) = split(/\: /, $_, 2);
    my %merge = map { $_ => 0} qw(red green blue);
    for my $draw (split(/; /, $rest)) {
        my %draw = (
            (map { $_ => 0} qw(red green blue)) 
        );
        for my $spec (split(/\s*,\s*/, $draw)) {
            my ($count, $color) = split(" ", $spec, 2);
            $draw{$color} = $count;
        }
        
        while (my ($k, $v) = each %draw) {
            $merge{$k} = $v if $merge{$k} < $v;
        }
    }
    my $num = 1;
    $num *= $_ for (values %merge);
    $a += $num;
}
print "$a\n";
