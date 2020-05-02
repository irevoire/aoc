Meta
====

This directory contains an executable that generate a markdown from the problem statement.

Usage
-----

You can use this executable either with file or by piping data inside it.
An exemple of utilisation would be:
```
curl https://adventofcode.com/2019/day/1 | cargo run 
```

But if you want the complete problem statement with the second part you can 
either download the page from your browser and run:
```
cargo run day1.html
```
or get your cookie and then run:
```
curl 'https://adventofcode.com/2019/day/1' \
-XGET \
-H 'Cookie: session=WRITE YOUR OWN SESSION COOKIE HERE' \
-H 'Connection: keep-alive' | cargo run
```


Your cookie does not change that often so you can easily run script like that:
```
for day in {1..8}; do
curl "https://adventofcode.com/2019/day/$day" \
-XGET \
-H 'Cookie: session=WRITE YOUR OWN SESSION COOKIE HERE' | cargo run > ../day$day/README.md; done
```

Important
=========

You should also read the following message that is displayed in every page of the
Advent of Code:
```
Oh, hello!  Funny seeing you here.

I appreciate your enthusiasm, but you aren't going to find much down here.
There certainly aren't clues to any of the puzzles.  The best surprises don't
even appear in the source until you unlock them for real.

Please be careful with automated requests; I'm not a massive company, and I can
only take so much traffic.  Please be considerate so that everyone gets to play.

If you're curious about how Advent of Code works, it's running on some custom
Perl code. Other than a few integrations (auth, analytics, ads, social media),
I built the whole thing myself, including the design, animations, prose, and
all of the puzzles.

The puzzles are most of the work; preparing a new calendar and a new set of
puzzles each year takes all of my free time for 4-5 months. A lot of effort
went into building this thing - I hope you're enjoying playing it as much as I
enjoyed making it for you!

If you'd like to hang out, I'm @ericwastl on Twitter.

- Eric Wastl
```

**Please be careful with automated requests; I'm not a massive company, and I can
only take so much traffic.  Please be considerate so that everyone gets to play.**
