# Hosting the web version on Cloudflare Pages

The browser version of Khata is also hosted on Cloudflare Pages (a global CDN, with your own domain),
next to the GitHub Pages demo. Deployment is automatic; this is the one-time setup (about 10 minutes).

`.github/workflows/web-cloudflare.yml` builds the web app and deploys it:

- **automatically after every successful release** (the Release workflow calls it once the release is
  published, so a failed release never changes the live site),
- or by hand (Actions › Web (Cloudflare Pages) › Run workflow), which deploys the current `main`.

It does not run on ordinary pushes. Until the two secrets below exist it skips itself without failing.

## One-time setup

1. **Account id.** Cloudflare dashboard › *Workers & Pages*. Copy the **Account ID** from the right
   sidebar (it is also the long hex string in the dashboard URL).
2. **API token.** Dashboard › your profile (top right) › *API Tokens* › *Create Token* › *Create
   Custom Token*:
   - Permissions: **Account** › **Cloudflare Pages** › **Edit**
   - Account Resources: *Include* › your account
   - Create the token and copy it (it is shown once).
3. **GitHub secrets.** Repository › *Settings › Secrets and variables › Actions › New repository
   secret*, twice:
   - `CLOUDFLARE_API_TOKEN` = the token
   - `CLOUDFLARE_ACCOUNT_ID` = the account id
4. **First deployment.** *Actions › Web (Cloudflare Pages) › Run workflow.* It creates the Pages
   project (named `khata`; set a repository *variable* `CLOUDFLARE_PROJECT` to use another name) and
   deploys. The site is live at `https://khata.pages.dev`.
5. **Your domain.** Dashboard › *Workers & Pages* › `khata` › *Custom domains* › *Set up a custom
   domain*. Enter, for example, `khata.yourdomain.com` and confirm. Because the domain's DNS is on
   Cloudflare, the CNAME record and the HTTPS certificate are created for you. (A bare domain such
   as `yourdomain.com` also works when the zone is on Cloudflare.)

From now on every release deploys by itself. The `*.pages.dev` address keeps working.

## Good to know

- The build is about 13 MB (5 MB compressed), under Cloudflare's 25 MiB per-file limit; the workflow
  fails with a clear message if a file ever exceeds it.
- `packaging/web/_headers` is copied into the deployed site: it sets the `application/wasm` type and
  long-lived caching for the hashed files, and keeps `index.html` fresh.
- GitHub Pages and Cloudflare are updated together by each release, so both serve the same version.
  To update Cloudflare between releases, run the workflow by hand.
- Cloudflare has said it is steering new projects toward "Workers Static Assets" but Pages remains
  supported. Moving later means replacing the deploy command with a Workers asset deploy; the build
  and the site stay the same.
