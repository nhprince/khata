# Cloudflare in front of the web demo (CDN + your own domain)

The browser version of Khata is published to GitHub Pages by every release. Cloudflare can serve it
to visitors through its global CDN, on your own domain, with no extra deployment: Cloudflare simply
sits in front of GitHub Pages. Every release updates GitHub Pages and visitors get the new version
automatically.

**Why not Cloudflare Pages?** Cloudflare Pages rejects any file over 25 MiB. The web build's
`.wasm` file is far larger (the whole site is about 42 MB, 16 MB zipped), so it cannot be uploaded
there. GitHub Pages accepts files up to 100 MB, so it hosts the site and Cloudflare accelerates it.

## One-time setup (about 15 minutes; the order matters)

1. **GitHub: set the domain.** Repository *Settings › Pages › Custom domain*: enter
   `khata.yourdomain.com` (use your own domain) and save.
2. **Cloudflare: add the DNS record.** Your domain › *DNS › Records › Add record*:
   Type `CNAME`, Name `khata`, Target `nhprince.github.io`, Proxy status **DNS only** (grey cloud).
3. **GitHub: wait for the check, then turn on HTTPS.** Back in *Settings › Pages*, wait until the
   DNS check passes (a few minutes), then tick **Enforce HTTPS**. GitHub issues the certificate (it can
   take up to an hour).
4. **Cloudflare: switch it on.** Change the record to **Proxied** (orange cloud), then *SSL/TLS ›
   Overview* and choose **Full (strict)**. (Doing this before step 3 finishes can break GitHub's
   certificate issuance, and the *Flexible* mode causes redirect loops.)
5. **Cloudflare: cache the big files (recommended).** *Caching › Cache Rules › Create rule*:
   - Custom filter expression:
     `(ends_with(http.request.uri.path, ".wasm")) or (ends_with(http.request.uri.path, ".js"))`
   - Cache eligibility: **Eligible for cache**; Edge TTL: **1 month**; Browser TTL: **1 month**.
   The file names contain a hash that changes with every build, so long caching is safe, and the page
   itself (`index.html`) stays uncached so a new release shows up immediately.

Check it: open `https://khata.yourdomain.com`; in the browser's network tab the response headers
show `server: cloudflare`, and `cf-cache-status: HIT` on the `.wasm` after the second load.

The old address `https://nhprince.github.io/khata/` keeps working too. If you used a bare
domain instead of a subdomain, GitHub's documentation lists the A records to use; Cloudflare's
proxy works the same way.
