# Plugins

Plugins let an instance add custom behaviour without forking arcadia and without merging anything
upstream. A plugin is a **separate service** that arcadia calls over HTTP, so it can be written in
any language and deployed independently.

For now, plugins can add **external sources** (scrapers), like the built in TMDB, MusicBrainz, ISBN
and Comic Vine ones.

## Configuration

Copy `plugins.yml.example` to `plugins.yml`, and declare your plugins:

```yaml
scrapers:
  - id: anidb
    label: AniDB
    placeholder: AniDB url
    content_types:
      - tv_show
    url: http://anidb-plugin:9000/scrape
    timeout_seconds: 30
```

| Field | Description |
| --- | --- |
| `id` | Identifier used in the route `/api/external-sources/<id>`. Must not clash with a built in source (`tmdb`, `musicbrainz`, `isbn`, `comic-vine`). |
| `label` | Name displayed in the interface. |
| `placeholder` | Placeholder of the input field displayed in the interface. |
| `content_types` | Content types the source applies to: `movie`, `video`, `tv_show`, `music`, `podcast`, `software`, `book`, `live_performance`, `collection`. |
| `url` | Endpoint of the plugin. |
| `timeout_seconds` | Optional, defaults to `30`. |

`plugins.yml` is read once at startup. When the file does not exist, no plugin is registered.

Both `plugins.yml` and `compose.override.yml` are git ignored. Declare the plugin services themselves in `compose.override.yml`, which Docker Compose loads automatically on top of `compose.yml`:

```yaml
services:
  backend:
    volumes:
      - ./plugins.yml:/app/plugins.yml:ro

  anidb-plugin:
    build: ../anidb-plugin
    restart: unless-stopped
```

The backend runs from `/app`, which is where it looks for `plugins.yml`.

## Writing a scraper plugin

The declared endpoint is called with the identifier the user typed as the `url` query parameter:

```
GET http://anidb-plugin:9000/scrape?url=https://anidb.net/anime/1234
```

It must answer with JSON:

```json
{
  "title_group": { "name": "...", "description": "...", "content_type": "tv_show", "...": "..." },
  "edition_group": null,
  "affiliated_artists": [
    {
      "name": "Some Person",
      "aliases": [],
      "description": "",
      "pictures": ["https://example.com/picture.jpg"],
      "roles": ["director"],
      "nickname": null
    }
  ]
}
```

`title_group` follows the `UserCreatedTitleGroup` schema and `edition_group` the
`UserCreatedEditionGroup` one; both are documented in the OpenAPI specification. All three fields are optional.

Artists are given **by name**: arcadia creates them, merges the roles of an artist appearing several
times, and returns real affiliated artists to
the interface. Arcadia also checks beforehand whether a title group already has the submitted link,
and appends that link to the scraped title group.

Pictures are rehosted if image rehosting is enabled.

Anything else the plugin needs (API keys, caching, rate limiting) is its own business.
