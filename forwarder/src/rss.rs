use regex::Regex;
use std::collections::HashMap;
use url::Url;

const LINK_REGEX: &str = r#"<link>(?P<url>.*?)</link>"#;
const ENCLOSURE_URL_REGEX: &str = r#"<enclosure.*url=("|')(?P<url>.*?)("|')"#;

/// Set an optional path prefix if specified
///
/// # Example
///
/// ```
/// let path_prefix = Some("/r/");
/// ```
/// produces URLs like:
/// example.com/podcast.mp3 -> example.com/r/podcast.mp3
fn set_prefix<'a>(url: &'a mut Url, prefix: &str) -> &'a mut Url {
    let old_path = url.path();
    let path_with_prefix = prefix.to_string() + old_path;
    url.set_path(&path_with_prefix);
    url
}

/// Replaces the domain of mp3 links inside RSS enclosure elements
/// as well as the link elements
pub struct Replacer {
    /// Override for the `<link>` element
    link_url: Url,
    /// New URL prefix for replaced mp3 URLs
    forward_url: Url,
    /// Optional path prefix for replaced URLs
    path_prefix: Option<String>,
    /// Regex for finding `<link>` elements
    link_regex: Regex,
    /// Regex for finding mp3 links
    enclosure_regex: Regex,
}

impl Replacer {
    #[must_use]
    /// Construct a replacer with the given forwarder domain
    pub fn new(link_url: Url, forward_url: Url, path_prefix: Option<&str>) -> Self {
        Self {
            link_url,
            forward_url,
            path_prefix: path_prefix.map(Into::into),
            link_regex: Regex::new(LINK_REGEX).unwrap(),
            enclosure_regex: Regex::new(ENCLOSURE_URL_REGEX).unwrap(),
        }
    }

    /// Extract all links from an arbitrary string input
    fn extract(&self, input: &str) -> Vec<String> {
        self.enclosure_regex
            .captures_iter(input)
            .filter_map(|c| c.name("url").map(|m| m.as_str().to_owned()))
            .collect()
    }

    /// Extract all valid MP3 links from an arbitrary string input
    fn extract_mp3s(&self, input: &str) -> Vec<Url> {
        let links = self.extract(input);
        links
            .into_iter()
            .map(|link| Url::parse(&link))
            .filter_map(Result::ok)
            .filter(|url| url.path().ends_with("mp3"))
            .filter(|url| url.scheme() == "http" || url.scheme() == "https")
            .collect()
    }

    /// Replace `<link>` elements with the new domain
    fn replace_links(&self, input: &str) -> String {
        self.link_regex
            .replace_all(input, |_caps: &regex::Captures| {
                format!("<link>{}</link>", self.link_url)
            })
            .to_string()
    }

    /// Replaces the domain of all mp3 links which were found
    /// Uses an ad-hoc lookup table for replacing old with new links
    ///
    /// This is pretty inefficient as we iterate over the input N times where N
    /// is the number of links
    #[must_use]
    pub fn replace(&self, mut input: String) -> String {
        let mp3s = self.extract_mp3s(&input);
        let lookup_table: HashMap<String, String> = mp3s
            .into_iter()
            .map(|orig| {
                let mut replaced = self.forward_url.clone();

                replaced.set_path(orig.path());

                // Set optional prefix if specified
                // example.com/podcast.mp3 -> example.com/r/podcast.mp3
                if let Some(prefix) = &self.path_prefix {
                    set_prefix(&mut replaced, prefix);
                }

                replaced.query_pairs_mut().append_pair("ref", orig.as_str());

                (
                    orig.as_str().to_string(),
                    // Escape `&` character as HTML entities to make feed readable in browser
                    // See https://stackoverflow.com/a/17918240/270334
                    // See https://docs.rs/html-escape/latest/html_escape/
                    html_escape::encode_text(replaced.as_str()).to_string(),
                )
            })
            .collect();

        for (mp3, replaced) in lookup_table {
            input = input.replace(&mp3, &replaced);
        }
        // replace `<link>` elements with the new host
        self.replace_links(input.as_str())
    }

    /// Dummy replacer for testing
    #[cfg(test)]
    fn dummy() -> Self {
        Self {
            link_url: Url::parse("http://example.com/podcast").unwrap(),
            forward_url: Url::parse("http://test_dummy.com").unwrap(),
            path_prefix: None,
            link_regex: Regex::new(LINK_REGEX).unwrap(),
            enclosure_regex: Regex::new(ENCLOSURE_URL_REGEX).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_set_prefix() {
        let mut url = Url::parse("http://example.com/podcast.mp3").unwrap();
        let expected = Url::parse("http://example.com/r/podcast.mp3").unwrap();
        let actual = set_prefix(&mut url, "/r");
        assert_eq!(actual, &expected);
    }

    #[test]
    fn test_extract_link_from_enclosure() {
        let enclosure = r#"<enclosure url="https://example.com/podcast.mp3?awCollectionId=omr_abd3eb&amp;awEpisodeId=585475&amp;source=feed&amp;v=1636509931" type="audio/mpeg" length="96950025"/>"#;
        let link = Replacer::dummy().extract(enclosure);
        assert_eq!(
            vec!["https://example.com/podcast.mp3?awCollectionId=omr_abd3eb&amp;awEpisodeId=585475&amp;source=feed&amp;v=1636509931"], link
        );
    }

    #[test]
    fn test_extract_link_from_item() {
        let item = r#"
        <item>
            <itunes:episodeType>full</itunes:episodeType>
            <itunes:title>#05 Team Lead - der einzige Ausweg</itunes:title>
            <title>#05 Team Lead - der einzige Ausweg</title>

            <itunes:episode>5</itunes:episode>
            <itunes:season>1</itunes:season>
            <itunes:author>Wolfgang Gassler, Andy Grunwald</itunes:author>
            
            <description><![CDATA[<p><span>Engineering Manager oder Team-Lead: Eine Position die sehr motivierend, aber auch abschreckend wirken kann.</span></p><p><span>Was erwartet einen? Was ist die Aufgabe einer Engineering Managerin? Wie verändert sich der Arbeitsalltag? Ist die Stelle überhaupt etwas für mich? Und was passiert, wenn ich doch lieber Software Entwickeln möchte? Gibt es einen alternativen Karrierepfad?</span></p><p><span>All das und noch über viel mehr Erfahrungen sprechen Andy und Wolfgang in Episode 05 vom Engineering Kiosk.</span></p><p><span>Bonus: Warum Andy Muskelkater im Arsch hat</span></p><p><br></p><p><span>Feedback an </span><a href="mailto:stehtisch@engineeringkiosk.dev" rel="nofollow">stehtisch@engineeringkiosk.dev</a><span> oder via Twitter an </span><a href="https://twitter.com/EngKiosk" rel="nofollow">https://twitter.com/EngKiosk</a></p><p><br></p><h2><span>Erwähnte Artikel</span></h2><ul><li><span>Mitchell&#39;s New Role at HashiCorp: </span><a href="https://www.hashicorp.com/blog/mitchell-s-new-role-at-hashicorp" rel="nofollow">https://www.hashicorp.com/blog/mitchell-s-new-role-at-hashicorp</a></li><li><span>Tom Bartel mit &#34;A Year Ago, I Stepped Away From a Leadership Position. Here Are 7 Things I Learned From That&#34;: </span><a href="https://www.tombartel.me/blog/leadership-position-to-individual-contributor-what-i-learned/" rel="nofollow">https://www.tombartel.me/blog/leadership-position-to-individual-contributor-what-i-learned/ </a></li><li><span>&#34;What is a Staff (or Staff-Plus or Principal) Engineer?&#34;: </span><a href="https://mikemcquaid.com/2021/10/01/what-is-a-staff-plus-principal-engineer/" rel="nofollow">https://mikemcquaid.com/2021/10/01/what-is-a-staff-plus-principal-engineer/ </a></li></ul><p><br></p><h2><span>Bücher über das Engineering Management</span></h2><ul><li><span>&#34;The Managers Path&#34; von Camille Fournier</span></li><li>&#34;Turn the ship around&#34; oder &#34;Reiß das Ruder rum!&#34; von David Marquet</li><li><span>&#34;Drive&#34; von Daniel H. Pink</span></li><li><span>&#34;Start with Why&#34; von Simon Sinek</span></li><li><span>&#34;High Output Management&#34; von Andrew S. Grove</span></li><li><span>&#34;An elegant puzzle&#34; von Will Larson</span></li></ul><p><br></p><h2><span>Sprungmarken</span></h2><p><span>(00:55) Hörer Feedback</span></p><p><span>(01:43) Wann bist du das erste mal in eine Teamlead-Stelle gerutscht?</span></p><p><span>(03:15) Kann man bereits Aufgaben eines Teamleads übernehmen, ohne ein Teamlead zu sein?</span></p><p><span>(04:18) Wie viel Zeit hast du mit Hands-On und wie viel mit People Management verbracht?</span></p><p><span>(04:52) Wie lang warst du Individual Contributor bevor du Teamleiter wurdest?</span></p><p><span>(05:42) Was hat sich am meisten an deinem Arbeitsalltag geändert?</span></p><p><span>(09:22) Was ist ein 1 on 1 Meeting und warum ist dies sinnvoll?</span></p><p><span>(13:27) Was ist eine gute Teamgröße für den Start als Engineering Manager?</span></p><p><span>(14:50) Woher wusstest du, was du als neuer Engineering Manager machen musst?</span></p><p><span>(20:51) Empfehlungen um die Entscheidung &#34;Möchte ich den Job einer Engineering Managerin machen?&#34; treffen zu können</span></p><p><span>(24:25) Feedback-Loop eines Software Engineers und eines Engineering Managers</span></p><p><span>(25:50) Was solltest du nicht wollen, wenn du ein Engineering Manager werden möchtest?</span></p><p><span>(27:42) Ist es ab und zu notwendig, seine eigene Entscheidung im Team durchzusetzen?</span></p><p><span>(28:36) Schwierige Konversationen als Engineering Manager</span></p><p><span>(30:49) Ist es ein Rückschritt wenn man als Engineering Manager zurück zum Software Engineer wechselt?</span></p><p><span>(34:42) Wie sieht ein möglicher Karriereweg aus, wenn der Engineering Manager-Weg nichts für mich ist?</span></p><p><br></p><h2><span>Hosts</span></h2><ul><li><span>Wolfgang Gassler (https://twitter.com/schafele)</span></li><li><span>Andy Grunwald (https://twitter.com/andygrunwald)</span></li></ul><p><br></p><p><span>Engineering Kiosk Podcast: Anfragen an </span><a href="mailto:stehtisch@engineeringkiosk.dev" rel="nofollow">stehtisch@engineeringkiosk.dev</a><span> oder via Twitter an </span><a href="https://twitter.com/EngKiosk" rel="nofollow">https://twitter.com/EngKiosk</a></p>]]></description>
            <content:encoded>&lt;p&gt;&lt;span&gt;Engineering Manager oder Team-Lead: Eine Position die sehr motivierend, aber auch abschreckend wirken kann.&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;Was erwartet einen? Was ist die Aufgabe einer Engineering Managerin? Wie verändert sich der Arbeitsalltag? Ist die Stelle überhaupt etwas für mich? Und was passiert, wenn ich doch lieber Software Entwickeln möchte? Gibt es einen alternativen Karrierepfad?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;All das und noch über viel mehr Erfahrungen sprechen Andy und Wolfgang in Episode 05 vom Engineering Kiosk.&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;Bonus: Warum Andy Muskelkater im Arsch hat&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;br&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;Feedback an &lt;/span&gt;&lt;a href=&#34;mailto:stehtisch@engineeringkiosk.dev&#34; rel=&#34;nofollow&#34;&gt;stehtisch@engineeringkiosk.dev&lt;/a&gt;&lt;span&gt; oder via Twitter an &lt;/span&gt;&lt;a href=&#34;https://twitter.com/EngKiosk&#34; rel=&#34;nofollow&#34;&gt;https://twitter.com/EngKiosk&lt;/a&gt;&lt;/p&gt;&lt;p&gt;&lt;br&gt;&lt;/p&gt;&lt;h2&gt;&lt;span&gt;Erwähnte Artikel&lt;/span&gt;&lt;/h2&gt;&lt;ul&gt;&lt;li&gt;&lt;span&gt;Mitchell&amp;#39;s New Role at HashiCorp: &lt;/span&gt;&lt;a href=&#34;https://www.hashicorp.com/blog/mitchell-s-new-role-at-hashicorp&#34; rel=&#34;nofollow&#34;&gt;https://www.hashicorp.com/blog/mitchell-s-new-role-at-hashicorp&lt;/a&gt;&lt;/li&gt;&lt;li&gt;&lt;span&gt;Tom Bartel mit &amp;#34;A Year Ago, I Stepped Away From a Leadership Position. Here Are 7 Things I Learned From That&amp;#34;: &lt;/span&gt;&lt;a href=&#34;https://www.tombartel.me/blog/leadership-position-to-individual-contributor-what-i-learned/&#34; rel=&#34;nofollow&#34;&gt;https://www.tombartel.me/blog/leadership-position-to-individual-contributor-what-i-learned/ &lt;/a&gt;&lt;/li&gt;&lt;li&gt;&lt;span&gt;&amp;#34;What is a Staff (or Staff-Plus or Principal) Engineer?&amp;#34;: &lt;/span&gt;&lt;a href=&#34;https://mikemcquaid.com/2021/10/01/what-is-a-staff-plus-principal-engineer/&#34; rel=&#34;nofollow&#34;&gt;https://mikemcquaid.com/2021/10/01/what-is-a-staff-plus-principal-engineer/ &lt;/a&gt;&lt;/li&gt;&lt;/ul&gt;&lt;p&gt;&lt;br&gt;&lt;/p&gt;&lt;h2&gt;&lt;span&gt;Bücher über das Engineering Management&lt;/span&gt;&lt;/h2&gt;&lt;ul&gt;&lt;li&gt;&lt;span&gt;&amp;#34;The Managers Path&amp;#34; von Camille Fournier&lt;/span&gt;&lt;/li&gt;&lt;li&gt;&amp;#34;Turn the ship around&amp;#34; oder &amp;#34;Reiß das Ruder rum!&amp;#34; von David Marquet&lt;/li&gt;&lt;li&gt;&lt;span&gt;&amp;#34;Drive&amp;#34; von Daniel H. Pink&lt;/span&gt;&lt;/li&gt;&lt;li&gt;&lt;span&gt;&amp;#34;Start with Why&amp;#34; von Simon Sinek&lt;/span&gt;&lt;/li&gt;&lt;li&gt;&lt;span&gt;&amp;#34;High Output Management&amp;#34; von Andrew S. Grove&lt;/span&gt;&lt;/li&gt;&lt;li&gt;&lt;span&gt;&amp;#34;An elegant puzzle&amp;#34; von Will Larson&lt;/span&gt;&lt;/li&gt;&lt;/ul&gt;&lt;p&gt;&lt;br&gt;&lt;/p&gt;&lt;h2&gt;&lt;span&gt;Sprungmarken&lt;/span&gt;&lt;/h2&gt;&lt;p&gt;&lt;span&gt;(00:55) Hörer Feedback&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(01:43) Wann bist du das erste mal in eine Teamlead-Stelle gerutscht?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(03:15) Kann man bereits Aufgaben eines Teamleads übernehmen, ohne ein Teamlead zu sein?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(04:18) Wie viel Zeit hast du mit Hands-On und wie viel mit People Management verbracht?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(04:52) Wie lang warst du Individual Contributor bevor du Teamleiter wurdest?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(05:42) Was hat sich am meisten an deinem Arbeitsalltag geändert?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(09:22) Was ist ein 1 on 1 Meeting und warum ist dies sinnvoll?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(13:27) Was ist eine gute Teamgröße für den Start als Engineering Manager?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(14:50) Woher wusstest du, was du als neuer Engineering Manager machen musst?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(20:51) Empfehlungen um die Entscheidung &amp;#34;Möchte ich den Job einer Engineering Managerin machen?&amp;#34; treffen zu können&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(24:25) Feedback-Loop eines Software Engineers und eines Engineering Managers&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(25:50) Was solltest du nicht wollen, wenn du ein Engineering Manager werden möchtest?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(27:42) Ist es ab und zu notwendig, seine eigene Entscheidung im Team durchzusetzen?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(28:36) Schwierige Konversationen als Engineering Manager&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(30:49) Ist es ein Rückschritt wenn man als Engineering Manager zurück zum Software Engineer wechselt?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;(34:42) Wie sieht ein möglicher Karriereweg aus, wenn der Engineering Manager-Weg nichts für mich ist?&lt;/span&gt;&lt;/p&gt;&lt;p&gt;&lt;br&gt;&lt;/p&gt;&lt;h2&gt;&lt;span&gt;Hosts&lt;/span&gt;&lt;/h2&gt;&lt;ul&gt;&lt;li&gt;&lt;span&gt;Wolfgang Gassler (https://twitter.com/schafele)&lt;/span&gt;&lt;/li&gt;&lt;li&gt;&lt;span&gt;Andy Grunwald (https://twitter.com/andygrunwald)&lt;/span&gt;&lt;/li&gt;&lt;/ul&gt;&lt;p&gt;&lt;br&gt;&lt;/p&gt;&lt;p&gt;&lt;span&gt;Engineering Kiosk Podcast: Anfragen an &lt;/span&gt;&lt;a href=&#34;mailto:stehtisch@engineeringkiosk.dev&#34; rel=&#34;nofollow&#34;&gt;stehtisch@engineeringkiosk.dev&lt;/a&gt;&lt;span&gt; oder via Twitter an &lt;/span&gt;&lt;a href=&#34;https://twitter.com/EngKiosk&#34; rel=&#34;nofollow&#34;&gt;https://twitter.com/EngKiosk&lt;/a&gt;&lt;/p&gt;</content:encoded>
            
            <enclosure length="38602292" type="audio/mpeg" url="https://stream.redcircle.com/episodes/41cfb14d-7091-482a-9d05-eb21219897ab/stream.link"/>
            
            <guid isPermaLink="false">5f7fb175-4381-4dd4-a207-d5ef6c679706</guid>
            <link>https://redcircle.com/shows/0ecfdfd7-fda1-4c3d-9515-476727f9df5e/episodes/41cfb14d-7091-482a-9d05-eb21219897ab</link>
            <pubDate>Tue, 01 Feb 2022 10:05:00 &#43;0000</pubDate>
            <itunes:image href="https://media.redcircle.com/images/2022/2/1/10/46f20898-97dd-49a6-873e-d784dd5b6da6_ep05_cover.jpg"/>
            <itunes:duration>2412</itunes:duration>
            
            <itunes:explicit>no</itunes:explicit>
        </item> 
        "#;
        let link = Replacer::dummy().extract(item);
        assert_eq!(
            vec!["https://stream.redcircle.com/episodes/41cfb14d-7091-482a-9d05-eb21219897ab/stream.link"], link

        );
    }

    #[test]
    fn test_extract_multiple_links() {
        let item = r#"
                <enclosure length="38602292" type="audio/mpeg" url="https://stream.redcircle.com/episodes/41cfb14d-7091-482a-9d05-eb21219897ab/stream.link"/>
                <enclosure url="https://stream.redcircle.com/episodes/08ff2242-89e4-4533-8498-93d201ed6679/stream.link"/>
                <enclosure url="https://foo.de/e/podcat.link?bar=baz&foo=123"/>

        "#;
        let links = Replacer::dummy().extract(item);
        assert_eq!(
            vec![
                "https://stream.redcircle.com/episodes/41cfb14d-7091-482a-9d05-eb21219897ab/stream.link",
                "https://stream.redcircle.com/episodes/08ff2242-89e4-4533-8498-93d201ed6679/stream.link",
                "https://foo.de/e/podcat.link?bar=baz&foo=123",
            ], links
        );
    }

    #[test]
    fn test_extract_mp3() {
        let item = r#"
        <item>
            <title>Some Title</title>
            <itunes:title>Itunes Title</itunes:title>
            <description>Some Description</description>
            <pubDate>Wed, 10 Nov 2021 02:00:22 +0000</pubDate>
            <link>https://example.com/episode-1</link>
            <guid isPermaLink="false">197c6fd449acabee3ac0b7179aee7527</guid>
            <content:encoded>
            <![CDATA[<a href="https://www.example.com/disclaimer/">https://www.example.com/disclaimer/</a></p>]]>
            </content:encoded>
            <itunes:image href="https://example.com/image.jpg"/>
            <itunes:episode>1</itunes:episode>
            <itunes:episodeType>full</itunes:episodeType>
            <itunes:subtitle></itunes:subtitle>
            <itunes:summary>Itunes Summary</itunes:summary>
            <itunes:explicit>no</itunes:explicit>
            <itunes:keywords>foo,bar,baz</itunes:keywords>
            <itunes:author>John Doe</itunes:author>
            <enclosure url="https://example.com/podcast.mp3?awCollectionId=omr_abd3eb&amp;awEpisodeId=585475&amp;source=feed&amp;v=1636509931" type="audio/mpeg" length="96950025"/>
            <itunes:duration>6880</itunes:duration>
        </item>
        "#;
        let expected = Url::parse("https://example.com/podcast.mp3?awCollectionId=omr_abd3eb&amp;awEpisodeId=585475&amp;source=feed&amp;v=1636509931").unwrap();
        let links = Replacer::dummy().extract_mp3s(item);
        assert_eq!(vec![expected], links);
    }

    #[test]
    fn test_fake_mp3s() {
        let expected = Url::parse("https://example.com/podcast.mp3").unwrap();
        let input = r#"
                <enclosure length="38602292" type="audio/mpeg" url="https://stream.redcircle.com/episodes/41cfb14d-7091-482a-9d05-eb21219897ab/stream.link"/>
                <enclosure url="https://stream.redcircle.com/episodes/08ff2242-89e4-4533-8498-93d201ed6679/stream.link"/>
                <enclosure url="https://foo.de/e/podcat.link?bar=baz&foo=123"/>
                <enclosure url="https://example.com/podcast.mp3" type="audio/mpeg" length="96950025"/>
                <enclosure url="mailto://example.com/podcast.mp3" type="audio/mpeg" length="96950025"/>
                <enclosure url="file:///example.com/podcast.mp3" type="audio/mpeg" length="96950025"/>
        "#;
        let links = Replacer::dummy().extract_mp3s(input);
        assert_eq!(vec![expected], links);
    }

    #[test]
    fn test_replace_mp3() {
        let old_mp3 = r#"<enclosure url="https://example.com/podcast.mp3" type="audio/mpeg" length="96950025"/>"#;
        let new_mp3 = r#"<enclosure url="http://foo.org/podcast.mp3?ref=https%3A%2F%2Fexample.com%2Fpodcast.mp3" type="audio/mpeg" length="96950025"/>"#;
        let output = Replacer::new(
            Url::parse("http://example.com/podcast").unwrap(),
            Url::parse("http://foo.org").unwrap(),
            None,
        )
        .replace(old_mp3.to_string());
        assert_eq!(output, new_mp3);
    }

    #[test]
    fn test_replace_multiple_mp3() {
        let input = r#"
            <enclosure url="https://example.com/podcast1.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="http://example.com/podcast2.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="https://foo.com/some/podcast3.mp3?bla=blub123" type="audio/mpeg" length="96950025"></enclosure>
            <enclosure url="ftp://example.com/fake_podcast.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="example.com/podcast4.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="lol" type="audio/mpeg" length="96950025"/>
            <enclosure />
        "#;
        let expected = r#"
            <enclosure url="https://example.org/podcast1.mp3?ref=https%3A%2F%2Fexample.com%2Fpodcast1.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="https://example.org/podcast2.mp3?ref=http%3A%2F%2Fexample.com%2Fpodcast2.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="https://example.org/some/podcast3.mp3?ref=https%3A%2F%2Ffoo.com%2Fsome%2Fpodcast3.mp3%3Fbla%3Dblub123" type="audio/mpeg" length="96950025"></enclosure>
            <enclosure url="ftp://example.com/fake_podcast.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="example.com/podcast4.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="lol" type="audio/mpeg" length="96950025"/>
            <enclosure />
        "#;
        let output = Replacer::new(
            Url::parse("http://example.com/podcast").unwrap(),
            Url::parse("https://example.org").unwrap(),
            None,
        )
        .replace(input.to_string());
        assert_eq!(output, expected);
    }

    #[test]
    fn test_replace_multiple_mp3_with_path_prefix() {
        let input = r#"
            <enclosure url="https://example.com/podcast1.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="http://example.com/podcast2.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="https://example.org/podcast3.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="https://foo.com/some/podcast3.mp3?bla=blub123" type="audio/mpeg" length="96950025"></enclosure>
            <enclosure />
        "#;
        let expected = r#"
            <enclosure url="https://example.org/r/podcast1.mp3?ref=https%3A%2F%2Fexample.com%2Fpodcast1.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="https://example.org/r/podcast2.mp3?ref=http%3A%2F%2Fexample.com%2Fpodcast2.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="https://example.org/r/podcast3.mp3?ref=https%3A%2F%2Fexample.org%2Fpodcast3.mp3" type="audio/mpeg" length="96950025"/>
            <enclosure url="https://example.org/r/some/podcast3.mp3?ref=https%3A%2F%2Ffoo.com%2Fsome%2Fpodcast3.mp3%3Fbla%3Dblub123" type="audio/mpeg" length="96950025"></enclosure>
            <enclosure />
        "#;
        let output = Replacer::new(
            Url::parse("http://example.com/podcast").unwrap(),
            Url::parse("https://example.org").unwrap(),
            Some("/r"),
        )
        .replace(input.to_string());
        assert_eq!(output, expected);
    }

    #[test]
    fn test_replace_podcast_link() {
        let input = "<link>https://redcircle.com/shows/open-podcast</link>";
        let expected = "<link>https://example.org/</link>";
        let output = Replacer::new(
            Url::parse("http://example.com/podcast").unwrap(),
            Url::parse("https://example.org").unwrap(),
            None,
        )
        .replace(input.to_string());
        assert_eq!(output, expected);
    }
}
