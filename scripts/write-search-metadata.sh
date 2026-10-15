#!/usr/bin/env bash
set -euo pipefail

SITE_NAME="${1:-ricky.codes}"
BASE_URL="https://${SITE_NAME}"
TODAY_UTC="$(date -u +"%Y-%m-%d")"

cat > static/robots.txt <<EOF
User-agent: *
Allow: /
Content-Signal: search=yes, ai-input=yes, ai-train=no

# Allow assistant search and user-requested fetches. Training crawlers are
# disallowed in line with the Content-Signal above.
User-agent: OAI-SearchBot
Allow: /

User-agent: ChatGPT-User
Allow: /

User-agent: Claude-SearchBot
Allow: /

User-agent: Claude-User
Allow: /

User-agent: GPTBot
Disallow: /

User-agent: ClaudeBot
Disallow: /

User-agent: Google-Extended
Disallow: /

Sitemap: $BASE_URL/sitemap.xml
EOF

cat > static/sitemap.xml <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>$BASE_URL/</loc>
    <lastmod>$TODAY_UTC</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
</urlset>
EOF

node scripts/generate-agent-content.mjs "$BASE_URL"
