use once_cell::sync::Lazy;
use std::collections::HashMap;
use worker::{Error, Request, Result};

/// Lookup table of user agents
/// Source: <https://github.com/opawg/podcast-rss-useragents/blob/master/src/rss-ua.json>
/// Each config consists of a pattern and a cleaned up user agent string.
static USER_AGENTS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut user_agents = HashMap::new();
    user_agents.insert("Acast".to_owned(), "Acast".to_owned());
    user_agents.insert("Aggregator/".to_owned(), "Aggregator".to_owned());
    user_agents.insert("AhrefsBot".to_owned(), "AhrefsSiteAudit bot".to_owned());
    user_agents.insert("AirPodcasts/".to_owned(), "AirPodcasts-unknown".to_owned());
    user_agents.insert("Airr Podcatcher".to_owned(), "Airr".to_owned());
    user_agents.insert(
        "Amazon Music Podcast".to_owned(),
        "Amazon Music Podcasts".to_owned(),
    );
    user_agents.insert(
        "AmazonMusic/".to_owned(),
        "Amazon Music Podcasts".to_owned(),
    );
    user_agents.insert("AntennaPod/".to_owned(), "AntennaPod".to_owned());
    user_agents.insert(
        "anytime_podcast_player".to_owned(),
        "Anytime podcast player".to_owned(),
    );
    user_agents.insert("iTunes/".to_owned(), "Apple iTunes".to_owned());
    user_agents.insert(
        "itunesstored/1.0".to_owned(),
        "Apple iTunes Store".to_owned(),
    );
    user_agents.insert("iTMS".to_owned(), "Apple Podcasts - directory".to_owned());
    user_agents.insert(
        "Podcasts/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert("Balados/".to_owned(), "Apple Podcasts - via app".to_owned());
    user_agents.insert("Balados/".to_owned(), "Apple Podcasts - via app".to_owned());
    user_agents.insert(
        "Podcasti/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert(
        "Podcastit/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert(
        "Podcasturi/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert(
        "Podcasty/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert(
        "Podcast’ler/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert(
        "Podkaster/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert(
        "Podcaster/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert("Podcast/".to_owned(), "Apple Podcasts - via app".to_owned());
    user_agents.insert(
        "Podcastok/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert(
        "Подкасти/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert(
        "Подкасты/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert(
        "פודקאסטים/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert(
        "البودكاست/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert("पॉडकास्ट/".to_owned(), "Apple Podcasts - via app".to_owned());
    user_agents.insert("พ็อดคาสท์/".to_owned(), "Apple Podcasts - via app".to_owned());
    user_agents.insert("播客/".to_owned(), "Apple Podcasts - via app".to_owned());
    user_agents.insert(
        "팟캐스트/".to_owned(),
        "Apple Podcasts - via app".to_owned(),
    );
    user_agents.insert("special_archiver".to_owned(), "archive.org".to_owned());
    user_agents.insert("Audacy-Podcast-Scraper".to_owned(), "Audacy".to_owned());
    user_agents.insert("audius".to_owned(), "Audius".to_owned());
    user_agents.insert("AvailableOnBot".to_owned(), "AvailableOn".to_owned());
    user_agents.insert("BazQux/".to_owned(), "BazQux Reader".to_owned());
    user_agents.insert("BeyondPod".to_owned(), "BeyondPod".to_owned());
    user_agents.insert("bingbot/".to_owned(), "BingBot".to_owned());
    user_agents.insert("Bitcast/".to_owned(), "Bitcast".to_owned());
    user_agents.insert("bitcastbot".to_owned(), "Bitcast".to_owned());
    user_agents.insert("Blogtrottr/".to_owned(), "Blogtrottr".to_owned());
    user_agents.insert(
        "RawVoice Generator/".to_owned(),
        "Blubrry Podcasting".to_owned(),
    );
    user_agents.insert("Breaker/".to_owned(), "Breaker".to_owned());
    user_agents.insert("anytime.amugofjava.me.uk".to_owned(), "Breez".to_owned());
    user_agents.insert("briefings.fm".to_owned(), "briefings.fm".to_owned());
    user_agents.insert("Bullhorn Server".to_owned(), "Bullhorn".to_owned());
    user_agents.insert("Castamatic/".to_owned(), "Castamatic".to_owned());
    user_agents.insert("CastboxFeedParser".to_owned(), "Castbox".to_owned());
    user_agents.insert("CastBox".to_owned(), "Castbox".to_owned());
    user_agents.insert(
        "CastFeedValidator".to_owned(),
        "CastFeedValidator".to_owned(),
    );
    user_agents.insert("Tentacles".to_owned(), "Castro".to_owned());
    user_agents.insert(
        "Mozilla/5.0 +https://chartable.com/crawler Trackable/".to_owned(),
        "Chartable".to_owned(),
    );
    user_agents.insert(
        "Podcast-CriticalMention/".to_owned(),
        "Critical Mention".to_owned(),
    );
    user_agents.insert("CurioCaster/".to_owned(), "CurioCaster".to_owned());
    user_agents.insert("DataForSeoBot".to_owned(), "DataForSEO".to_owned());
    user_agents.insert("Deezer Podcasters/".to_owned(), "Deezer".to_owned());
    user_agents.insert("DEVONthink".to_owned(), "DEVONthink".to_owned());
    user_agents.insert("dlvr.it/".to_owned(), "dlvr.it".to_owned());
    user_agents.insert("DoggCatcher".to_owned(), "DoggCatcher".to_owned());
    user_agents.insert("Downcast/".to_owned(), "Downcast".to_owned());
    user_agents.insert("edgar".to_owned(), "Edgar".to_owned());
    user_agents.insert("Entale bot".to_owned(), "Entale".to_owned());
    user_agents.insert("facebookexternalhit/".to_owned(), "Facebook".to_owned());
    user_agents.insert("podcastbot".to_owned(), "Facebook Podcasts".to_owned());
    user_agents.insert("Feed Wrangler/".to_owned(), "Feed Wrangler".to_owned());
    user_agents.insert("Feedbin".to_owned(), "Feedbin".to_owned());
    user_agents.insert("feeder.co".to_owned(), "Feeder".to_owned());
    user_agents.insert("Feeder /".to_owned(), "Feeder".to_owned());
    user_agents.insert("Feedly".to_owned(), "Feedly".to_owned());
    user_agents.insert("Feedspot/".to_owned(), "Feedspot".to_owned());
    user_agents.insert("ffydpoll".to_owned(), "Ffyd".to_owned());
    user_agents.insert("FreshRSS".to_owned(), "FreshRSS".to_owned());
    user_agents.insert("Fusebox".to_owned(), "Fusebox".to_owned());
    user_agents.insert("FYEO/".to_owned(), "FYEO".to_owned());
    user_agents.insert("fyyd/".to_owned(), "Fyyd".to_owned());
    user_agents.insert("fyyd-poll".to_owned(), "Fyyd".to_owned());
    user_agents.insert("Goodpods".to_owned(), "Goodpods".to_owned());
    user_agents.insert(
        "FeedFetcher-Google".to_owned(),
        "Google Feedfetcher".to_owned(),
    );
    user_agents.insert(
        "Googlebot".to_owned(),
        "Google Podcasts and Search".to_owned(),
    );
    user_agents.insert("GEfektBot/1".to_owned(), "Govoren Efekt Bot".to_owned());
    user_agents.insert("gPodder/".to_owned(), "gPodder".to_owned());
    user_agents.insert("hackney/".to_owned(), "Hackney-unknown".to_owned());
    user_agents.insert("Headliner".to_owned(), "Headliner".to_owned());
    user_agents.insert("Hypefactors".to_owned(), "Hypefactors".to_owned());
    user_agents.insert("Buck/".to_owned(), "Hypefactors".to_owned());
    user_agents.insert("iCatcher".to_owned(), "iCatcher! Podcast Player".to_owned());
    user_agents.insert(
        "Mozilla/5.0 (Linux;) AppleWebKit/ Chrome/ Safari".to_owned(),
        "iHeartRadio".to_owned(),
    );
    user_agents.insert("inoreader.com".to_owned(), "Inoreader".to_owned());
    user_agents.insert("Instacast/".to_owned(), "Instacast".to_owned());
    user_agents.insert("iVoox".to_owned(), "iVoox".to_owned());
    user_agents.insert("Krzana bot".to_owned(), "Krzana".to_owned());
    user_agents.insert("Leaf/".to_owned(), "Leaf-unknown".to_owned());
    user_agents.insert(
        "life-radio-konsole-app".to_owned(),
        "Life Radio Konsole App".to_owned(),
    );
    user_agents.insert("Liferea/".to_owned(), "Liferea".to_owned());
    user_agents.insert("Lisnybot".to_owned(), "Lisny".to_owned());
    user_agents.insert("ListenAppBot".to_owned(), "Listen App".to_owned());
    user_agents.insert("ListenNotes".to_owned(), "Listen Notes".to_owned());
    user_agents.insert("Luminary/".to_owned(), "Luminary".to_owned());
    user_agents.insert("Micro.blog/".to_owned(), "Micro.blog".to_owned());
    user_agents.insert("MissinglettrBot/".to_owned(), "MissingLettr".to_owned());
    user_agents.insert("MixerBox Podcast Crawler".to_owned(), "MixerBox".to_owned());
    user_agents.insert("MuckRackFeedParser".to_owned(), "Muck Rack".to_owned());
    user_agents.insert("mypodapp.net".to_owned(), "My Pod".to_owned());
    user_agents.insert("NetNewsWire".to_owned(), "NetNewsWire".to_owned());
    user_agents.insert("Netvibes".to_owned(), "Netvibes".to_owned());
    user_agents.insert("News Explorer/".to_owned(), "News Explorer".to_owned());
    user_agents.insert("NewsBlur Feed Fetcher".to_owned(), "NewsBlur".to_owned());
    user_agents.insert("Newsify Feed Fetcher".to_owned(), "Newsify".to_owned());
    user_agents.insert("NewsNow/".to_owned(), "NewsNow".to_owned());
    user_agents.insert("NextCloud-News/".to_owned(), "Nextcloud".to_owned());
    user_agents.insert("NRCAudioBot/".to_owned(), "NRC Audio".to_owned());
    user_agents.insert("Office 365 Connectors".to_owned(), "Office 365".to_owned());
    user_agents.insert("Overcast/".to_owned(), "Overcast".to_owned());
    user_agents.insert("OwlTail/".to_owned(), "OwlTail".to_owned());
    user_agents.insert("PandoraRSSCrawler".to_owned(), "Pandora".to_owned());
    user_agents.insert("PaperLiBot/".to_owned(), "Paper.li".to_owned());
    user_agents.insert("PetalBot".to_owned(), "PetalBot".to_owned());
    user_agents.insert("Playapod/".to_owned(), "Playapod".to_owned());
    user_agents.insert(
        "PlayerFM/1.0 Podcast Sync".to_owned(),
        "Player FM".to_owned(),
    );
    user_agents.insert("Plex/".to_owned(), "Plex".to_owned());
    user_agents.insert("plex".to_owned(), "Plex".to_owned());
    user_agents.insert("Plex Media Providers".to_owned(), "Plex".to_owned());
    user_agents.insert("PocketCasts/".to_owned(), "Pocket Casts".to_owned());
    user_agents.insert("Swoot/".to_owned(), "Pod Hero".to_owned());
    user_agents.insert(
        "Mozilla/5.0 (compatible; Podalong/".to_owned(),
        "Podalong".to_owned(),
    );
    user_agents.insert("Podbay/".to_owned(), "Podbay".to_owned());
    user_agents.insert("PodbeanFeedReader/".to_owned(), "Podbean".to_owned());
    user_agents.insert("PodcastGuru".to_owned(), "Podcast Guru".to_owned());
    user_agents.insert("Podcastindex.org/".to_owned(), "Podcast Index".to_owned());
    user_agents.insert("PodcastRepublic/".to_owned(), "Podcast Republic".to_owned());
    user_agents.insert("PodcastAddict/".to_owned(), "PodcastAddict".to_owned());
    user_agents.insert("Podcastly/".to_owned(), "Podcastly".to_owned());
    user_agents.insert("Podcastly/".to_owned(), "Podcastly".to_owned());
    user_agents.insert("PodcastScraper".to_owned(), "PodcastScraper".to_owned());
    user_agents.insert("Podchaser-Parser".to_owned(), "Podchaser".to_owned());
    user_agents.insert("Podchaser".to_owned(), "Podchaser".to_owned());
    user_agents.insert("podCloud/".to_owned(), "podCloud".to_owned());
    user_agents.insert("PodCruncher".to_owned(), "PodCruncher".to_owned());
    user_agents.insert("PodEngine/".to_owned(), "PodEngine".to_owned());
    user_agents.insert("podfollowbot/".to_owned(), "Podfollow".to_owned());
    user_agents.insert("podfriend".to_owned(), "Podfriend".to_owned());
    user_agents.insert("PodheroBot/".to_owned(), "Podhero".to_owned());
    user_agents.insert("PodHound/".to_owned(), "PodHound".to_owned());
    user_agents.insert("Podimo/".to_owned(), "Podimo".to_owned());
    user_agents.insert("Podinstall".to_owned(), "Podinstall".to_owned());
    user_agents.insert("Podkicker".to_owned(), "Podkicker".to_owned());
    user_agents.insert("PodLink".to_owned(), "PodLink".to_owned());
    user_agents.insert("PodBotLP/".to_owned(), "PodLP".to_owned());
    user_agents.insert("PodMN/".to_owned(), "PodMN".to_owned());
    user_agents.insert("PodMust/".to_owned(), "PodMust".to_owned());
    user_agents.insert("Podmust/".to_owned(), "Podmust".to_owned());
    user_agents.insert("PodnewsBot".to_owned(), "PodnewsBot".to_owned());
    user_agents.insert("PodParadise".to_owned(), "PodParadise".to_owned());
    user_agents.insert("Podplay-Podcast-Sync/".to_owned(), "Podplay".to_owned());
    user_agents.insert("Podsights/".to_owned(), "Podsights".to_owned());
    user_agents.insert("Podtail/".to_owned(), "Podtail".to_owned());
    user_agents.insert(
        "Mozilla/5.0 (compatible; Podtail/".to_owned(),
        "Podtail".to_owned(),
    );
    user_agents.insert("podtail".to_owned(), "Podtail".to_owned());
    user_agents.insert("Podtrac Feed Scanner".to_owned(), "Podtrac".to_owned());
    user_agents.insert("Podverse/Feed Parser".to_owned(), "Podverse".to_owned());
    user_agents.insert("Podyssey App".to_owned(), "Podyssey App".to_owned());
    user_agents.insert(
        "Radical-Edward".to_owned(),
        "Radical-Edward Podcast Discovery".to_owned(),
    );
    user_agents.insert("axios/0.19.1".to_owned(), "radio.com".to_owned());
    user_agents.insert("RadioCut/".to_owned(), "Radiocut".to_owned());
    user_agents.insert("Radioline".to_owned(), "Radioline".to_owned());
    user_agents.insert("RadioPublic-Web/".to_owned(), "RadioPublic".to_owned());
    user_agents.insert("reason/".to_owned(), "Reason".to_owned());
    user_agents.insert("Reedah/1".to_owned(), "Reedah".to_owned());
    user_agents.insert("Reeder/".to_owned(), "Reeder".to_owned());
    user_agents.insert("Repod/".to_owned(), "Repod".to_owned());
    user_agents.insert("rssapi.net".to_owned(), "RSS API".to_owned());
    user_agents.insert("RSSOwl/".to_owned(), "RSSOwl".to_owned());
    user_agents.insert("RSSRadio".to_owned(), "RSSRadio".to_owned());
    user_agents.insert("R6_FeedFetcher".to_owned(), "Salesforce".to_owned());
    user_agents.insert("sp-agent".to_owned(), "Samsung Podcasts".to_owned());
    user_agents.insert(
        "semantic-visions.com".to_owned(),
        "Semantic Visions".to_owned(),
    );
    user_agents.insert("SemrushBot".to_owned(), "SEMrushBot".to_owned());
    user_agents.insert("SEOkicks".to_owned(), "SEOkicks".to_owned());
    user_agents.insert("SerendeputyBot/".to_owned(), "Serendeputy".to_owned());
    user_agents.insert("Shadow".to_owned(), "Shadow".to_owned());
    user_agents.insert("SismicsReaderBot".to_owned(), "Sismics Reader".to_owned());
    user_agents.insert("Slackbot".to_owned(), "Slackbot".to_owned());
    user_agents.insert("SocialBeeAgent".to_owned(), "SocialBeeAgent".to_owned());
    user_agents.insert("Sonnet/".to_owned(), "Sonnet".to_owned());
    user_agents.insert("Spotify/".to_owned(), "Spotify".to_owned());
    user_agents.insert("Spreaker/".to_owned(), "Spreaker".to_owned());
    user_agents.insert("StitcherBot".to_owned(), "Stitcher".to_owned());
    user_agents.insert("Subcast/".to_owned(), "Subcast-unknown".to_owned());
    user_agents.insert("Superfeedr bot".to_owned(), "Superfeedr".to_owned());
    user_agents.insert("TapTapes".to_owned(), "Taptapes".to_owned());
    user_agents.insert("theoldreader.com".to_owned(), "The Old Reader".to_owned());
    user_agents.insert(
        "tweetedtimes.com".to_owned(),
        "The Tweeted Times".to_owned(),
    );
    user_agents.insert("Tiny Tiny RSS".to_owned(), "Tiny Tiny RSS".to_owned());
    user_agents.insert("TPA/".to_owned(), "TPA-unknown".to_owned());
    user_agents.insert("trendictionbot".to_owned(), "Trendiction Bot".to_owned());
    user_agents.insert("Tumult".to_owned(), "Tumult".to_owned());
    user_agents.insert("TuneInRssParser/".to_owned(), "TuneIn".to_owned());
    user_agents.insert("um-IC/".to_owned(), "Ubermetrics".to_owned());
    user_agents.insert("verbbot/".to_owned(), "Verb.fm".to_owned());
    user_agents.insert("VictorReader".to_owned(), "Victor Reader".to_owned());
    user_agents.insert("Vienna/".to_owned(), "ViennaRSS".to_owned());
    user_agents.insert("Vodacast".to_owned(), "Vodacast".to_owned());
    user_agents.insert("VurblBot/".to_owned(), "Vurbl".to_owned());
    user_agents.insert("Winds:".to_owned(), "Winds".to_owned());
    user_agents.insert("russ(xiaoyuzhou)/1.0".to_owned(), "Xiao Yu Zhou".to_owned());
    user_agents.insert("Russ".to_owned(), "Xiao Yu Zhou".to_owned());
    user_agents.insert("YandexBot/".to_owned(), "YandexBot".to_owned());
    user_agents.insert("Zapier".to_owned(), "Zapier".to_owned());
    user_agents.insert("ZoominfoBot".to_owned(), "Zoominfo".to_owned());
    user_agents
});

/// Try to return a canonical user agent from the `user-agent` header
pub fn from(request: &Request) -> Result<String> {
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
fn lookup(user_agent: &str) -> Option<String> {
    for (pattern, agent) in USER_AGENTS.iter() {
        if user_agent.starts_with(pattern) {
            return Some(agent.clone());
        }
    }
    None
}

/// Get user agent from request
pub fn client(request: &Request) -> String {
    from(request).unwrap_or_else(|_| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        assert_eq!(
            lookup("Spotify/8.6.88.1104 Android/30 (SM-A525F)"),
            Some("Spotify".to_owned())
        );
        assert_eq!(
            lookup("Spotify/8.6.82 iOS/15.1 (iPhone12,1)"),
            Some("Spotify".to_owned())
        );
        assert_eq!(
            lookup("AmazonMusic/9.16.0 iPhone12,1 CFNetwork/1128.0.1 Darwin/19.6.0"),
            Some("Amazon Music Podcasts".to_owned())
        );
    }
}
