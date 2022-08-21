# Edge worker

Reads an RSS feeds and replaces mp3 links with a custom URL on the fly.
We don't use a [full RSS parser][rss] yet as we could run into a lot of edge-cases.
Instead, we just extract the URLS with a regex and replace them.

## Usage

Run `make help` for a full list of commands.

```bash
# compile project to WebAssembly
make build

# run Worker in a development environment
wrangler dev

# deploy Worker to Cloudflare
wrangler login
wrangler publish

# test request
curl -A "Spotify/1.0" -c - https://forwarder.mre.workers.dev
```

## Implementation steps

1. RSS feed edge worker replaces `<enclosure url="URL" ... />` elements by new URL of edge worker with the original URL encoded.
   Important: URLs have to end with `.mp3`
2. requests are forwarded to original url by edge worker

## Matomo Instance for Testing

Instance URL: https://piwik.inlupus.at/matomo.php?idsite=15&rec=1

[Matomo API Docs](https://developer.matomo.org/api-reference/tracking-api)

## RSS Examples

- [Engineering Kiosk RSS (redcircle)](https://feeds.redcircle.com/0ecfdfd7-fda1-4c3d-9515-476727f9df5e)
- [Doppelgaenger Podcast with more than 100 episodes (podigee)](https://doppelgaenger.podigee.io/feed/mp3)

## Resources

- [DB of User agents](https://github.com/opawg/podcast-rss-useragents/blob/master/src/rss-ua.json)
- [insights about a tagged tracking approach](https://soundsprofitable.com/update/rss-useragents)

[rss]: https://github.com/emilyskidsister/pyrocast/blob/master/loader/src/rss.rs
