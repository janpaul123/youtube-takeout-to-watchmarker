# Youtube Takeout to Watchmarker

Youtube seems to not always mark videos that I've watched a while ago. This can be quite annoying. To fix this, I tried using the [Watchmarker browser extension](https://chrome.google.com/webstore/detail/watchmarker-for-youtube/pfkkfbfdhomeagojoahjmkojeeepcolc?hl=en). However, it does not seem to be able to pull in all watched videos from the Youtube API either.

However, when you download your data from Youtube (using [Takeout](https://takeout.google.com/settings/takeout/custom/youtube)) you get a file `watch-history.html`, which does contain your proper history.

To load this into Watchmarker, I wrote this program, which takes in `watch-history.html` and converts it into a `watchmarker-youtube.database` file, which you can import into Watchmarker using the Import button.

To run this program, first install [Rust](https://www.rust-lang.org/tools/install), then check out this repository, put your `watch-history.html` in the repository directory, run `cargo run --release`, and finally import the resulting `watchmarker-youtube.database` file into Watchmarker using the Import button on its Settings page.

**Note: when importing you might have to explicitly select Options > Format > All Files in the Import screen on Mac OS X.**

Limitations: this program currently does not support importing the actual date or number of views. It always shows up in Watchmarker with a hardcoded date, and with exactly 1 view.
