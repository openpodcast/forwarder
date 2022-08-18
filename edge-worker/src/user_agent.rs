use once_cell::sync::Lazy;
use std::collections::HashMap;
use worker::*;

/// Lookup table of user agents
/// Source: https://github.com/opawg/podcast-rss-useragents/blob/master/src/rss-ua.json
/// Each config consists of a pattern and a cleaned up user agent string.
static USER_AGENTS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut user_agents = HashMap::new();
    user_agents.insert("Acast".to_string(), "Acast".to_string());
    user_agents.insert("Aggregator/".to_string(), "Aggregator".to_string());
    user_agents.insert("AhrefsBot".to_string(), "AhrefsSiteAudit bot".to_string());
    user_agents.insert(
        "AirPodcasts/".to_string(),
        "AirPodcasts-unknown".to_string(),
    );
    user_agents.insert("Airr Podcatcher".to_string(), "Airr".to_string());
    user_agents.insert(
        "Amazon Music Podcast".to_string(),
        "Amazon Music Podcasts".to_string(),
    );
    user_agents.insert("AntennaPod/".to_string(), "AntennaPod".to_string());
    user_agents.insert(
        "anytime_podcast_player".to_string(),
        "Anytime podcast player".to_string(),
    );
    user_agents.insert("iTunes/".to_string(), "Apple iTunes".to_string());
    user_agents.insert(
        "itunesstored/1.0".to_string(),
        "Apple iTunes Store".to_string(),
    );
    user_agents.insert("iTMS".to_string(), "Apple Podcasts - directory".to_string());
    user_agents.insert(
        "Podcasts/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Balados/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Balados/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Podcasti/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Podcastit/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Podcasturi/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Podcasty/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Podcast’ler/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Podkaster/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Podcaster/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Podcast/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Podcastok/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Подкасти/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "Подкасты/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "פודקאסטים/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "البودكاست/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "पॉडकास्ट/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert(
        "พ็อดคาสท์/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert("播客/".to_string(), "Apple Podcasts - via app".to_string());
    user_agents.insert(
        "팟캐스트/".to_string(),
        "Apple Podcasts - via app".to_string(),
    );
    user_agents.insert("special_archiver".to_string(), "archive.org".to_string());
    user_agents.insert("Audacy-Podcast-Scraper".to_string(), "Audacy".to_string());
    user_agents.insert("audius".to_string(), "Audius".to_string());
    user_agents.insert("AvailableOnBot".to_string(), "AvailableOn".to_string());
    user_agents.insert("BazQux/".to_string(), "BazQux Reader".to_string());
    user_agents.insert("BeyondPod".to_string(), "BeyondPod".to_string());
    user_agents.insert("bingbot/".to_string(), "BingBot".to_string());
    user_agents.insert("Bitcast/".to_string(), "Bitcast".to_string());
    user_agents.insert("bitcastbot".to_string(), "Bitcast".to_string());
    user_agents.insert("Blogtrottr/".to_string(), "Blogtrottr".to_string());
    user_agents.insert(
        "RawVoice Generator/".to_string(),
        "Blubrry Podcasting".to_string(),
    );
    user_agents.insert("Breaker/".to_string(), "Breaker".to_string());
    user_agents.insert("anytime.amugofjava.me.uk".to_string(), "Breez".to_string());
    user_agents.insert("briefings.fm".to_string(), "briefings.fm".to_string());
    user_agents.insert("Bullhorn Server".to_string(), "Bullhorn".to_string());
    user_agents.insert("Castamatic/".to_string(), "Castamatic".to_string());
    user_agents.insert("CastboxFeedParser".to_string(), "Castbox".to_string());
    user_agents.insert("CastBox".to_string(), "Castbox".to_string());
    user_agents.insert(
        "CastFeedValidator".to_string(),
        "CastFeedValidator".to_string(),
    );
    user_agents.insert("Tentacles".to_string(), "Castro".to_string());
    user_agents.insert(
        "Mozilla/5.0 +https://chartable.com/crawler Trackable/".to_string(),
        "Chartable".to_string(),
    );
    user_agents.insert(
        "Podcast-CriticalMention/".to_string(),
        "Critical Mention".to_string(),
    );
    user_agents.insert("CurioCaster/".to_string(), "CurioCaster".to_string());
    user_agents.insert("DataForSeoBot".to_string(), "DataForSEO".to_string());
    user_agents.insert("Deezer Podcasters/".to_string(), "Deezer".to_string());
    user_agents.insert("DEVONthink".to_string(), "DEVONthink".to_string());
    user_agents.insert("dlvr.it/".to_string(), "dlvr.it".to_string());
    user_agents.insert("DoggCatcher".to_string(), "DoggCatcher".to_string());
    user_agents.insert("Downcast/".to_string(), "Downcast".to_string());
    user_agents.insert("edgar".to_string(), "Edgar".to_string());
    user_agents.insert("Entale bot".to_string(), "Entale".to_string());
    user_agents.insert("facebookexternalhit/".to_string(), "Facebook".to_string());
    user_agents.insert("podcastbot".to_string(), "Facebook Podcasts".to_string());
    user_agents.insert("Feed Wrangler/".to_string(), "Feed Wrangler".to_string());
    user_agents.insert("Feedbin".to_string(), "Feedbin".to_string());
    user_agents.insert("feeder.co".to_string(), "Feeder".to_string());
    user_agents.insert("Feeder /".to_string(), "Feeder".to_string());
    user_agents.insert("Feedly".to_string(), "Feedly".to_string());
    user_agents.insert("Feedspot/".to_string(), "Feedspot".to_string());
    user_agents.insert("ffydpoll".to_string(), "Ffyd".to_string());
    user_agents.insert("FreshRSS".to_string(), "FreshRSS".to_string());
    user_agents.insert("Fusebox".to_string(), "Fusebox".to_string());
    user_agents.insert("FYEO/".to_string(), "FYEO".to_string());
    user_agents.insert("fyyd/".to_string(), "Fyyd".to_string());
    user_agents.insert("fyyd-poll".to_string(), "Fyyd".to_string());
    user_agents.insert("Goodpods".to_string(), "Goodpods".to_string());
    user_agents.insert(
        "FeedFetcher-Google".to_string(),
        "Google Feedfetcher".to_string(),
    );
    user_agents.insert(
        "Googlebot".to_string(),
        "Google Podcasts and Search".to_string(),
    );
    user_agents.insert("GEfektBot/1".to_string(), "Govoren Efekt Bot".to_string());
    user_agents.insert("gPodder/".to_string(), "gPodder".to_string());
    user_agents.insert("hackney/".to_string(), "Hackney-unknown".to_string());
    user_agents.insert("Headliner".to_string(), "Headliner".to_string());
    user_agents.insert("Hypefactors".to_string(), "Hypefactors".to_string());
    user_agents.insert("Buck/".to_string(), "Hypefactors".to_string());
    user_agents.insert(
        "iCatcher".to_string(),
        "iCatcher! Podcast Player".to_string(),
    );
    user_agents.insert(
        "Mozilla/5.0 (Linux;) AppleWebKit/ Chrome/ Safari".to_string(),
        "iHeartRadio".to_string(),
    );
    user_agents.insert("inoreader.com".to_string(), "Inoreader".to_string());
    user_agents.insert("Instacast/".to_string(), "Instacast".to_string());
    user_agents.insert("iVoox".to_string(), "iVoox".to_string());
    user_agents.insert("Krzana bot".to_string(), "Krzana".to_string());
    user_agents.insert("Leaf/".to_string(), "Leaf-unknown".to_string());
    user_agents.insert(
        "life-radio-konsole-app".to_string(),
        "Life Radio Konsole App".to_string(),
    );
    user_agents.insert("Liferea/".to_string(), "Liferea".to_string());
    user_agents.insert("Lisnybot".to_string(), "Lisny".to_string());
    user_agents.insert("ListenAppBot".to_string(), "Listen App".to_string());
    user_agents.insert("ListenNotes".to_string(), "Listen Notes".to_string());
    user_agents.insert("Luminary/".to_string(), "Luminary".to_string());
    user_agents.insert("Micro.blog/".to_string(), "Micro.blog".to_string());
    user_agents.insert("MissinglettrBot/".to_string(), "MissingLettr".to_string());
    user_agents.insert(
        "MixerBox Podcast Crawler".to_string(),
        "MixerBox".to_string(),
    );
    user_agents.insert("MuckRackFeedParser".to_string(), "Muck Rack".to_string());
    user_agents.insert("mypodapp.net".to_string(), "My Pod".to_string());
    user_agents.insert("NetNewsWire".to_string(), "NetNewsWire".to_string());
    user_agents.insert("Netvibes".to_string(), "Netvibes".to_string());
    user_agents.insert("News Explorer/".to_string(), "News Explorer".to_string());
    user_agents.insert("NewsBlur Feed Fetcher".to_string(), "NewsBlur".to_string());
    user_agents.insert("Newsify Feed Fetcher".to_string(), "Newsify".to_string());
    user_agents.insert("NewsNow/".to_string(), "NewsNow".to_string());
    user_agents.insert("NextCloud-News/".to_string(), "Nextcloud".to_string());
    user_agents.insert("NRCAudioBot/".to_string(), "NRC Audio".to_string());
    user_agents.insert(
        "Office 365 Connectors".to_string(),
        "Office 365".to_string(),
    );
    user_agents.insert("Overcast/".to_string(), "Overcast".to_string());
    user_agents.insert("OwlTail/".to_string(), "OwlTail".to_string());
    user_agents.insert("PandoraRSSCrawler".to_string(), "Pandora".to_string());
    user_agents.insert("PaperLiBot/".to_string(), "Paper.li".to_string());
    user_agents.insert("PetalBot".to_string(), "PetalBot".to_string());
    user_agents.insert("Playapod/".to_string(), "Playapod".to_string());
    user_agents.insert(
        "PlayerFM/1.0 Podcast Sync".to_string(),
        "Player FM".to_string(),
    );
    user_agents.insert("Plex/".to_string(), "Plex".to_string());
    user_agents.insert("plex".to_string(), "Plex".to_string());
    user_agents.insert("Plex Media Providers".to_string(), "Plex".to_string());
    user_agents.insert("PocketCasts/".to_string(), "Pocket Casts".to_string());
    user_agents.insert("Swoot/".to_string(), "Pod Hero".to_string());
    user_agents.insert(
        "Mozilla/5.0 (compatible; Podalong/".to_string(),
        "Podalong".to_string(),
    );
    user_agents.insert("Podbay/".to_string(), "Podbay".to_string());
    user_agents.insert("PodbeanFeedReader/".to_string(), "Podbean".to_string());
    user_agents.insert("PodcastGuru".to_string(), "Podcast Guru".to_string());
    user_agents.insert("Podcastindex.org/".to_string(), "Podcast Index".to_string());
    user_agents.insert(
        "PodcastRepublic/".to_string(),
        "Podcast Republic".to_string(),
    );
    user_agents.insert("PodcastAddict/".to_string(), "PodcastAddict".to_string());
    user_agents.insert("Podcastly/".to_string(), "Podcastly".to_string());
    user_agents.insert("Podcastly/".to_string(), "Podcastly".to_string());
    user_agents.insert("PodcastScraper".to_string(), "PodcastScraper".to_string());
    user_agents.insert("Podchaser-Parser".to_string(), "Podchaser".to_string());
    user_agents.insert("Podchaser".to_string(), "Podchaser".to_string());
    user_agents.insert("podCloud/".to_string(), "podCloud".to_string());
    user_agents.insert("PodCruncher".to_string(), "PodCruncher".to_string());
    user_agents.insert("PodEngine/".to_string(), "PodEngine".to_string());
    user_agents.insert("podfollowbot/".to_string(), "Podfollow".to_string());
    user_agents.insert("podfriend".to_string(), "Podfriend".to_string());
    user_agents.insert("PodheroBot/".to_string(), "Podhero".to_string());
    user_agents.insert("PodHound/".to_string(), "PodHound".to_string());
    user_agents.insert("Podimo/".to_string(), "Podimo".to_string());
    user_agents.insert("Podinstall".to_string(), "Podinstall".to_string());
    user_agents.insert("Podkicker".to_string(), "Podkicker".to_string());
    user_agents.insert("PodLink".to_string(), "PodLink".to_string());
    user_agents.insert("PodBotLP/".to_string(), "PodLP".to_string());
    user_agents.insert("PodMN/".to_string(), "PodMN".to_string());
    user_agents.insert("PodMust/".to_string(), "PodMust".to_string());
    user_agents.insert("Podmust/".to_string(), "Podmust".to_string());
    user_agents.insert("PodnewsBot".to_string(), "PodnewsBot".to_string());
    user_agents.insert("PodParadise".to_string(), "PodParadise".to_string());
    user_agents.insert("Podplay-Podcast-Sync/".to_string(), "Podplay".to_string());
    user_agents.insert("Podsights/".to_string(), "Podsights".to_string());
    user_agents.insert("Podtail/".to_string(), "Podtail".to_string());
    user_agents.insert(
        "Mozilla/5.0 (compatible; Podtail/".to_string(),
        "Podtail".to_string(),
    );
    user_agents.insert("podtail".to_string(), "Podtail".to_string());
    user_agents.insert("Podtrac Feed Scanner".to_string(), "Podtrac".to_string());
    user_agents.insert("Podverse/Feed Parser".to_string(), "Podverse".to_string());
    user_agents.insert("Podyssey App".to_string(), "Podyssey App".to_string());
    user_agents.insert(
        "Radical-Edward".to_string(),
        "Radical-Edward Podcast Discovery".to_string(),
    );
    user_agents.insert("axios/0.19.1".to_string(), "radio.com".to_string());
    user_agents.insert("RadioCut/".to_string(), "Radiocut".to_string());
    user_agents.insert("Radioline".to_string(), "Radioline".to_string());
    user_agents.insert("RadioPublic-Web/".to_string(), "RadioPublic".to_string());
    user_agents.insert("reason/".to_string(), "Reason".to_string());
    user_agents.insert("Reedah/1".to_string(), "Reedah".to_string());
    user_agents.insert("Reeder/".to_string(), "Reeder".to_string());
    user_agents.insert("Repod/".to_string(), "Repod".to_string());
    user_agents.insert("rssapi.net".to_string(), "RSS API".to_string());
    user_agents.insert("RSSOwl/".to_string(), "RSSOwl".to_string());
    user_agents.insert("RSSRadio".to_string(), "RSSRadio".to_string());
    user_agents.insert("R6_FeedFetcher".to_string(), "Salesforce".to_string());
    user_agents.insert("sp-agent".to_string(), "Samsung Podcasts".to_string());
    user_agents.insert(
        "semantic-visions.com".to_string(),
        "Semantic Visions".to_string(),
    );
    user_agents.insert("SemrushBot".to_string(), "SEMrushBot".to_string());
    user_agents.insert("SEOkicks".to_string(), "SEOkicks".to_string());
    user_agents.insert("SerendeputyBot/".to_string(), "Serendeputy".to_string());
    user_agents.insert("Shadow".to_string(), "Shadow".to_string());
    user_agents.insert("SismicsReaderBot".to_string(), "Sismics Reader".to_string());
    user_agents.insert("Slackbot".to_string(), "Slackbot".to_string());
    user_agents.insert("SocialBeeAgent".to_string(), "SocialBeeAgent".to_string());
    user_agents.insert("Sonnet/".to_string(), "Sonnet".to_string());
    user_agents.insert("Spotify/".to_string(), "Spotify".to_string());
    user_agents.insert("Spreaker/".to_string(), "Spreaker".to_string());
    user_agents.insert("StitcherBot".to_string(), "Stitcher".to_string());
    user_agents.insert("Subcast/".to_string(), "Subcast-unknown".to_string());
    user_agents.insert("Superfeedr bot".to_string(), "Superfeedr".to_string());
    user_agents.insert("TapTapes".to_string(), "Taptapes".to_string());
    user_agents.insert("theoldreader.com".to_string(), "The Old Reader".to_string());
    user_agents.insert(
        "tweetedtimes.com".to_string(),
        "The Tweeted Times".to_string(),
    );
    user_agents.insert("Tiny Tiny RSS".to_string(), "Tiny Tiny RSS".to_string());
    user_agents.insert("TPA/".to_string(), "TPA-unknown".to_string());
    user_agents.insert("trendictionbot".to_string(), "Trendiction Bot".to_string());
    user_agents.insert("Tumult".to_string(), "Tumult".to_string());
    user_agents.insert("TuneInRssParser/".to_string(), "TuneIn".to_string());
    user_agents.insert("um-IC/".to_string(), "Ubermetrics".to_string());
    user_agents.insert("verbbot/".to_string(), "Verb.fm".to_string());
    user_agents.insert("VictorReader".to_string(), "Victor Reader".to_string());
    user_agents.insert("Vienna/".to_string(), "ViennaRSS".to_string());
    user_agents.insert("Vodacast".to_string(), "Vodacast".to_string());
    user_agents.insert("VurblBot/".to_string(), "Vurbl".to_string());
    user_agents.insert("Winds:".to_string(), "Winds".to_string());
    user_agents.insert(
        "russ(xiaoyuzhou)/1.0".to_string(),
        "Xiao Yu Zhou".to_string(),
    );
    user_agents.insert("Russ".to_string(), "Xiao Yu Zhou".to_string());
    user_agents.insert("YandexBot/".to_string(), "YandexBot".to_string());
    user_agents.insert("Zapier".to_string(), "Zapier".to_string());
    user_agents.insert("ZoominfoBot".to_string(), "Zoominfo".to_string());
    user_agents
});

/// Try to return a canonical user agent from the `user-agent` header
pub fn from(request: Request) -> Result<String> {
    let ua_string = request.headers().get("user-agent")?;
    let ua_string = match ua_string {
        Some(ua) => ua,
        None => {
            return Err(Error::RustError(
                "Cannot read user agent from request".to_string(),
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
            return Some(agent.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        assert_eq!(
            lookup("Spotify/8.6.88.1104 Android/30 (SM-A525F)"),
            Some("Spotify".to_string())
        );
        assert_eq!(
            lookup("Spotify/8.6.82 iOS/15.1 (iPhone12,1)"),
            Some("Spotify".to_string())
        );
    }
}
