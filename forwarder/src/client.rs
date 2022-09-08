use once_cell::sync::Lazy;
use std::collections::HashMap;
use worker::{Error, Request, Result};

/// Podcast Client information
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Client {
    /// Name of the Podcast client
    name: String,
    /// Whether the Podcast client is a bot
    bot: bool,
}

impl Client {
    /// Create a new `Client` from a name and optionally set a bot flag
    fn new(name: &str) -> Self {
        // At some point we might move bot detection to the user agent lookup
        // table or alternatively do it with post-processing
        // For now we just check if the name contains "bot"
        let bot = name.to_lowercase().contains("bot");
        Self {
            name: name.to_string(),
            bot,
        }
    }

    /// Return the name of the client
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return whether the client is a bot
    pub fn is_bot(&self) -> bool {
        self.bot
    }
}

/// Lookup table of user agents and the corresponding Podcast clients
/// Source: <https://github.com/opawg/podcast-rss-useragents/blob/master/src/rss-ua.json>
/// Each config consists of a pattern and a sanitized client name.
static USER_AGENTS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    [
        ("Acast", "Acast"),
        ("Aggregator/", "Aggregator"),
        ("AhrefsBot", "AhrefsSiteAudit [bot]"),
        ("AirPodcasts/", "AirPodcasts-unknown"),
        ("Airr Podcatcher", "Airr"),
        ("Amazon Music Podcast", "Amazon Music Podcasts"),
        ("AntennaPod/", "AntennaPod"),
        ("anytime_podcast_player", "Anytime podcast player"),
        ("iTunes/", "Apple iTunes"),
        ("itunes", "Apple iTunes Store"),
        ("iTMS", "Apple Podcasts - directory"),
        ("Pocket Casts", "Pocket Casts"),
        ("Podcasts/", "Apple Podcasts - via app"),
        ("AppleCoreMedia/", "Apple Podcasts - via app"),
        ("Balados/", "Apple Podcasts - via app"),
        ("Balados/", "Apple Podcasts - via app"),
        ("Podcasti/", "Apple Podcasts - via app"),
        ("Podcastit/", "Apple Podcasts - via app"),
        ("Podcasturi/", "Apple Podcasts - via app"),
        ("Podcasty/", "Apple Podcasts - via app"),
        ("Podcast’ler/", "Apple Podcasts - via app"),
        ("Podkaster/", "Apple Podcasts - via app"),
        ("PodkiteCrawler/", "Podkite"),
        ("Podcaster/", "Apple Podcasts - via app"),
        ("Podcast/", "Apple Podcasts - via app"),
        ("Podcastok/", "Apple Podcasts - via app"),
        ("Подкасти/", "Apple Podcasts - via app"),
        ("Подкасты/", "Apple Podcasts - via app"),
        ("פודקאסטים/", "Apple Podcasts - via app"),
        ("البودكاست/", "Apple Podcasts - via app"),
        ("पॉडकास्ट/", "Apple Podcasts - via app"),
        ("พ็อดคาสท์/", "Apple Podcasts - via app"),
        ("播客/", "Apple Podcasts - via app"),
        ("팟캐스트/", "Apple Podcasts - via app"),
        ("special_archiver", "archive.org"),
        ("Audacy-Podcast-Scraper", "Audacy"),
        ("audius", "Audius"),
        ("AvailableOnBot", "AvailableOn"),
        ("BazQux/", "BazQux Reader"),
        ("BeyondPod", "BeyondPod"),
        ("bingbot/", "BingBot"),
        ("Bitcast/", "Bitcast"),
        ("bitcastbot", "Bitcast"),
        ("Blogtrottr/", "Blogtrottr"),
        ("RawVoice Generator/", "Blubrry Podcasting"),
        ("Breaker/", "Breaker"),
        ("anytime.amugofjava.me.uk", "Breez"),
        ("briefings.fm", "briefings.fm"),
        ("Bullhorn Server", "Bullhorn"),
        ("Castamatic/", "Castamatic"),
        ("CastboxFeedParser", "Castbox"),
        ("CastBox", "Castbox"),
        ("CastFeedValidator", "CastFeedValidator"),
        ("Tentacles", "Castro"),
        (
            "Mozilla/5.0 +https://chartable.com/crawler Trackable/",
            "Chartable",
        ),
        ("Podcast-CriticalMention/", "Critical Mention"),
        ("CurioCaster/", "CurioCaster"),
        ("DataForSeoBot", "DataForSEO"),
        ("Deezer Podcasters/", "Deezer"),
        ("DEVONthink", "DEVONthink"),
        ("dlvr.it/", "dlvr.it"),
        ("DoggCatcher", "DoggCatcher"),
        ("Downcast/", "Downcast"),
        ("edgar", "Edgar"),
        ("Entale bot", "Entale"),
        ("facebookexternalhit/", "Facebook"),
        ("podcastbot", "Facebook Podcasts"),
        ("Feed Wrangler/", "Feed Wrangler"),
        ("Feedbin", "Feedbin"),
        ("feeder.co", "Feeder"),
        ("Feeder /", "Feeder"),
        ("Feedly", "Feedly"),
        ("Feedspot/", "Feedspot"),
        ("ffydpoll", "Ffyd"),
        ("FreshRSS", "FreshRSS"),
        ("Fusebox", "Fusebox"),
        ("FYEO/", "FYEO"),
        ("fyyd/", "Fyyd"),
        ("fyyd-poll", "Fyyd"),
        ("Goodpods", "Goodpods"),
        ("FeedFetcher-Google", "Google Feedfetcher"),
        ("Googlebot", "Google Podcasts and Search [bot]"),
        ("GEfektBot/1", "Govoren Efekt Bot"),
        ("gPodder/", "gPodder"),
        ("GSA/", "Google Podcasts Android"),
        ("GooglePodcasts/", "Google Podcasts iOS"),
        ("Google-Podcast", "Google Play Music Podcasts"),
        ("hackney/", "Hackney-unknown"),
        ("Headliner", "Headliner"),
        ("Hypefactors", "Hypefactors"),
        ("Buck/", "Hypefactors"),
        ("iCatcher", "iCatcher! Podcast Player"),
        (
            "Mozilla/5.0 (Linux;) AppleWebKit/ Chrome/ Safari",
            "iHeartRadio",
        ),
        ("inoreader.com", "Inoreader"),
        ("Instacast/", "Instacast"),
        ("iVoox", "iVoox"),
        ("Krzana bot", "Krzana bot"),
        ("Leaf/", "Leaf-unknown"),
        ("life-radio-konsole-app", "Life Radio Konsole App"),
        ("Liferea/", "Liferea"),
        ("Lisnybot", "Lisny"),
        ("ListenAppBot", "Listen App"),
        ("ListenNotes", "Listen Notes"),
        ("Luminary/", "Luminary"),
        ("Micro.blog/", "Micro.blog"),
        ("MissinglettrBot/", "MissingLettr"),
        ("MixerBox Podcast Crawler", "MixerBox"),
        ("MuckRackFeedParser", "Muck Rack"),
        ("mypodapp.net", "My Pod"),
        ("NetNewsWire", "NetNewsWire"),
        ("Netvibes", "Netvibes"),
        ("News Explorer/", "News Explorer"),
        ("NewsBlur Feed Fetcher", "NewsBlur"),
        ("Newsify Feed Fetcher", "Newsify"),
        ("NewsNow/", "NewsNow"),
        ("NextCloud-News/", "Nextcloud"),
        ("NRCAudioBot/", "NRC Audio"),
        ("Office 365 Connectors", "Office 365"),
        ("Overcast/", "Overcast"),
        ("OwlTail/", "OwlTail"),
        ("PandoraRSSCrawler", "Pandora"),
        ("PaperLiBot/", "Paper.li"),
        ("PetalBot", "PetalBot"),
        ("Playapod/", "Playapod"),
        ("PlayerFM/1.0 Podcast Sync", "Player FM"),
        ("Plex/", "Plex"),
        ("plex", "Plex"),
        ("Plex Media Providers", "Plex"),
        ("PocketCasts/", "Pocket Casts"),
        ("Swoot/", "Pod Hero"),
        ("Mozilla/5.0 (compatible; Podalong/", "Podalong"),
        ("Podbay/", "Podbay"),
        ("PodbeanFeedReader/", "Podbean"),
        ("Podbean/", "Podbean"),
        ("PodcastGuru", "Podcast Guru"),
        ("Podcastindex.org/", "Podcast Index"),
        ("PodcastRepublic/", "Podcast Republic"),
        ("PodcastAddict/", "PodcastAddict"),
        ("Podcastly/", "Podcastly"),
        ("Podcastly/", "Podcastly"),
        ("PodcastScraper", "PodcastScraper"),
        ("Podchaser-Parser", "Podchaser"),
        ("Podchaser", "Podchaser"),
        ("podCloud/", "podCloud"),
        ("PodCruncher", "PodCruncher"),
        ("PodEngine/", "PodEngine"),
        ("podfollowbot/", "Podfollow"),
        ("podfriend", "Podfriend"),
        ("PodheroBot/", "Podhero"),
        ("PodHound/", "PodHound"),
        ("Podimo/", "Podimo"),
        ("Podinstall", "Podinstall"),
        ("Podkicker", "Podkicker"),
        ("PodLink", "PodLink"),
        ("PodBotLP/", "PodLP"),
        ("PodMN/", "PodMN"),
        ("PodMust/", "PodMust"),
        ("Podmust/", "Podmust"),
        ("PodnewsBot", "PodnewsBot"),
        ("PodParadise", "PodParadise"),
        ("Podplay-Podcast-Sync/", "Podplay"),
        ("Podsights/", "Podsights"),
        ("Podtail/", "Podtail"),
        ("Mozilla/5.0 (compatible; Podtail/", "Podtail"),
        ("podtail", "Podtail"),
        ("Podtrac Feed Scanner", "Podtrac"),
        ("Podverse/Feed Parser", "Podverse"),
        ("Podyssey App", "Podyssey App"),
        ("Radical-Edward", "Radical-Edward Podcast Discovery"),
        ("axios/0.19.1", "radio.com"),
        ("RadioCut/", "Radiocut"),
        ("radiofeed/", "Radiofeed"),
        ("Radioline", "Radioline"),
        ("RadioPublic-Web/", "RadioPublic"),
        ("reason/", "Reason"),
        ("RedCircle", "RedCircle"),
        ("Reedah/1", "Reedah"),
        ("Reeder/", "Reeder"),
        ("Repod/", "Repod"),
        ("Rephonic/", "Rephonic"),
        ("rssapi.net", "RSS API"),
        ("RSSOwl/", "RSSOwl"),
        ("RSSRadio", "RSSRadio"),
        ("R6_FeedFetcher", "Salesforce"),
        ("sp-agent", "Samsung Podcasts"),
        ("semantic-visions.com", "Semantic Visions"),
        ("SemrushBot", "SEMrushBot"),
        ("SEOkicks", "SEOkicks"),
        ("SerendeputyBot/", "Serendeputy"),
        ("Shadow", "Shadow"),
        ("SismicsReaderBot", "Sismics Reader"),
        ("Slackbot", "Slackbot"),
        ("SocialBeeAgent", "SocialBeeAgent"),
        ("Sonnet/", "Sonnet"),
        ("Spotify/", "Spotify"),
        ("Spreaker/", "Spreaker"),
        ("StitcherBot", "Stitcher"),
        ("Subcast/", "Subcast-unknown"),
        ("Superfeedr bot", "Superfeedr"),
        ("taddy.org/", "taddy"),
        ("TapTapes", "Taptapes"),
        ("theoldreader.com", "The Old Reader"),
        ("tweetedtimes.com", "The Tweeted Times"),
        ("Expanse, a Palo Alto Networks company", "Expanse [bot]"),
        ("Tiny Tiny RSS", "Tiny Tiny RSS"),
        ("TPA/", "TPA-unknown"),
        ("trendictionbot", "Trendiction Bot"),
        ("Tumult", "Tumult"),
        ("TuneInRssParser/", "TuneIn"),
        ("um-IC/", "Ubermetrics"),
        ("verbbot/", "Verb.fm"),
        ("VictorReader", "Victor Reader"),
        ("Vienna/", "ViennaRSS"),
        ("Vodacast", "Vodacast"),
        ("VurblBot/", "Vurbl"),
        ("Winds:", "Winds"),
        ("russ(xiaoyuzhou)/1.0", "Xiao Yu Zhou"),
        ("Russ", "Xiao Yu Zhou"),
        ("YandexBot/", "YandexBot"),
        ("Zapier", "Zapier"),
        ("ZoominfoBot", "Zoominfo"),
        // Scripts and libraries get categorized as bots
        ("axios", "Script [bot]"),
        ("Go-http-client/", "Script [bot]"),
        ("node-fetch/", "Script [bot]"),
        ("lychee/", "Script [bot]"),
        ("python-requests/", "Script [bot]"),
        ("Ruby", "Script [bot]"),
        ("UniversalFeedParser/", "Script [bot]"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect()
});

/// Try to return a canonical user agent from the `user-agent` header
pub fn from(request: &Request) -> Result<Client> {
    let ua_string = request.headers().get("user-agent")?;
    let ua_string = match ua_string {
        Some(ua) => ua,
        None => {
            return Err(Error::RustError(
                "Cannot read user agent from request".to_owned(),
            ))
        }
    };
    lookup(&ua_string).ok_or_else(|| "Cannot read user agent".into())
}

/// Lookup the given user agent string in the table of known user agents
#[must_use]
fn lookup(user_agent: &str) -> Option<Client> {
    for (pattern, agent) in USER_AGENTS.iter() {
        if user_agent.contains(pattern) {
            return Some(Client::new(agent));
        }
    }
    None
}

/// Get Podcast client from request user agent
pub fn client(request: &Request) -> Client {
    from(request).unwrap_or_else(|_| Client::new("Unknown Podcast Client"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        assert_eq!(
            lookup("Spotify/8.6.88.1104 Android/30 (SM-A525F)").unwrap(),
            Client::new("Spotify")
        );
        assert_eq!(
            lookup("Spotify/8.6.82 iOS/15.1 (iPhone12,1)").unwrap(),
            Client::new("Spotify")
        );
        assert_eq!(
            lookup("AmazonMusic/9.16.0 iPhone12,1 CFNetwork/1128.0.1 Darwin/19.6.0").unwrap(),
            Client::new("Amazon Music Podcasts")
        );
        assert_eq!(lookup("Something Random"), None);
        assert_eq!(
            lookup("UA: Mozilla/5.0 (Linux; Android 10; Pixel 3a XL Build/QQ3A.200805.001; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/86.0.4240.198 Mobile Safari/537.36 GSA/11.38.8.23.arm64").unwrap(),
            Client::new("Google Podcasts Android")
        );
        assert_eq!(
            lookup("AppleCoreMedia/1.0.0.21G72 (Macintosh; U; Intel Mac OS X 12_5; en_us)")
                .unwrap(),
            Client::new("Apple Podcasts - via app")
        );

        assert_eq!(
            lookup("Expanse, a Palo Alto Networks company, searches across the global IPv4 space multiple times per day to identify customers&#39; presences on the Internet. If you would like to be excluded from our scans, please send IP addresses/domains to: scaninfo@paloaltonetworks.com")
                .unwrap(),
            Client::new("Expanse [bot]")
        );
    }
}
