# Interesting metrics we want to track

## Server-Side metrics

- How many people are listening to your podcast episodes?
- Are listeners skipping over your episode intro or your second mid-roll?
- How are your shows and episodes performing over time?
- How many listeners do you have in each country and city?

## API metrics

- How many new followers did you get last month?

## RSS Feed metrics

- Duration of each episode
- Description length per episode

## Sources

- https://podcasters.apple.com/support/840-check-your-podcast-performance

# Apple Podcast server-side metrics

https://help.apple.com/itc/podcasts_reading_server_requests/#/itcc0e1eaa94

Discusses streaming vs downloading.

> A streaming request from iTunes desktop client looks similar to a download request, except that the GET request for the episode is not preceded by a GET request for the podcast XML file.

Similar to the download requests, you can perform a more advanced analysis by adding the following line of code to the LogFormat line in your httpd.conf configuration to specify the range of bytes for each GET request compared to the total file size.
\"%{Content-Range}o\"
