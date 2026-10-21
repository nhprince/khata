# Hosting: khata.bond (website) and app.khata.bond (web app)

| Address | What | Hosted on | Updated by |
|---|---|---|---|
| https://khata.bond (and www) | the landing page in `site/` | Cloudflare Pages | `.github/workflows/site-landing.yml`, on every push that changes `site/` |
| https://app.khata.bond | the web app (the browser version of Khata) | GitHub Pages | `.github/workflows/site-app.yml`, on every push that changes the app or what it is built from |
| GitHub Releases | installers for Windows, macOS, Linux | GitHub | `.github/workflows/release.yml`, when you run *Release* |

Why two hosts? A GitHub Pages site can serve only one domain, and Cloudflare Pages rejects files over
25 MiB, but the app's `.wasm` file is about 40 MB. So the small website goes to Cloudflare Pages and the
big app to GitHub Pages.

## One-time setup (about 30 minutes; follow the order)

### 1. Give GitHub access to Cloudflare Pages
1. **Account ID:** in the Cloudflare dashboard it is the long code right after `dash.cloudflare.com/` in
   the address bar (also on *Workers & Pages* in the right sidebar).
2. **API token:** profile icon (top right) › *API Tokens* › *Create Token* › *Create Custom Token*.
   Permissions: **Account › Cloudflare Pages › Edit**. Account Resources: *Include › your account*.
   Create it and copy it (shown once). A token you already made for Cloudflare Pages works too.
3. **GitHub:** repository › *Settings › Secrets and variables › Actions › New repository secret*, twice:
   `CLOUDFLARE_API_TOKEN` (the token) and `CLOUDFLARE_ACCOUNT_ID` (the account id).

### 2. Deploy the website once
Push this project (the `site/` folder triggers it), or run *Actions › Site (khata.bond) › Run workflow*.
It checks the site, creates the Cloudflare Pages project `khata` (first run only), deploys, and tests the
deployment. The run's *Summary* page shows your project's real address. It may carry a suffix, for example
`khata-c7l.pages.dev`, because `khata.pages.dev` belongs to someone else: use the address from the
summary (or the one in the Cloudflare dashboard), never `khata.pages.dev`. Only `khata.bond` is the
public address.

### 3. Move the domain (the order matters)
1. **Cloudflare › DNS › Records:** delete the record `khata.bond  CNAME  nhprince.github.io`.
   Do **not** touch the MX and TXT records (they are your email routing).
2. **GitHub › repository Settings › Pages › Custom domain:** replace `khata.bond` with
   `app.khata.bond` and **Save**.
3. **Cloudflare › DNS › Records › Add record:** Type `CNAME`, Name `app`, Target `nhprince.github.io`,
   Proxy status **DNS only** (grey cloud), TTL Auto. Save.
4. **Cloudflare › Workers & Pages › `khata` › Custom domains › Set up a custom domain:** enter
   `khata.bond`, continue, and **Activate domain** (Cloudflare creates the DNS record and the
   certificate itself). Repeat for `www.khata.bond`.
5. **GitHub › Settings › Pages:** press *Check again* until the DNS check for `app.khata.bond` passes,
   wait for the certificate (usually 15 minutes, sometimes an hour), then tick **Enforce HTTPS**.
6. **Deploy the app to the new address:** *Actions › App (app.khata.bond) › Run workflow* (from now on it
   runs by itself on every relevant push).

Then check: `https://khata.bond`, `https://www.khata.bond` and `https://app.khata.bond`.

### Optional
- **Send www to the bare domain:** Cloudflare › *Rules › Redirect Rules › Create rule*, when hostname
  equals `www.khata.bond`, redirect (301, keep the query string) to
  `concat("https://khata.bond", http.request.uri.path)`.
- **Cloudflare CDN in front of the app:** after step 5, switch the `app` record to *Proxied*, set
  *SSL/TLS* to *Full (strict)*, and add a cache rule for paths ending `.wasm` or `.js` (Edge TTL 1 month;
  the file names change with every build). GitHub's own CDN is already fast, so this is a bonus.

## Troubleshooting
- **`khata.pages.dev` shows somebody else's site.** Cloudflare Pages names are global; yours is the one
  with the suffix shown in the workflow summary. Ignore the other one and use `khata.bond`.
- **GitHub: "DNS check unsuccessful" for app.khata.bond.** The record must be named exactly `app`
  (not `app.khata.bond.khata.bond`), target `nhprince.github.io`, grey cloud, and there must be no
  other A, AAAA or CNAME record for `app`.
- **"Domain is not eligible for HTTPS at this time."** The certificate is still being issued. Wait; if it
  is still stuck after a few hours, *Remove* the custom domain in GitHub, save, and add it again.
- **Cloudflare: "A CNAME record with that host already exists" when adding `khata.bond`.** Step 1 was
  skipped; delete the old `khata.bond` record and retry.
- **The website workflow only shows a notice.** The two Cloudflare secrets are missing (step 1).
- **"www.khata.bond is improperly configured" in GitHub.** That warning belongs to the old setup, where
  the bare domain pointed at GitHub. After step 2 it disappears, because GitHub only serves `app`.

## Editing the website
Everything lives in `site/` (`index.html`, `css/style.css`, `js/main.js`). Run
`python3 packaging/check-site.py` to check it; a push to `main` publishes it in about a minute.
The download buttons read the latest GitHub release automatically, so a new release updates them
without touching the site.
