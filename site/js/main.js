/* Khata website: mobile menu, "for your system" highlighting, and live download links from the
   latest GitHub release. Everything degrades gracefully: without JavaScript the buttons simply
   point to the releases page. No cookies, no tracking. */
(function () {
  'use strict';

  var REPO = 'nhprince/khata';

  // ---- mobile menu ----
  var toggle = document.querySelector('.nav-toggle');
  var nav = document.getElementById('nav');
  function closeMenu() {
    if (!nav || !toggle) return;
    nav.classList.remove('open');
    toggle.setAttribute('aria-expanded', 'false');
  }
  if (toggle && nav) {
    toggle.addEventListener('click', function () {
      var open = nav.classList.toggle('open');
      toggle.setAttribute('aria-expanded', String(open));
    });
    nav.addEventListener('click', function (e) {
      if (e.target.closest && e.target.closest('a')) closeMenu();
    });
    document.addEventListener('keydown', function (e) {
      if (e.key === 'Escape' && nav.classList.contains('open')) {
        closeMenu();
        toggle.focus();
      }
    });
  }

  // ---- which system is the visitor on? ----
  function detectOS() {
    var platform = (navigator.userAgentData && navigator.userAgentData.platform) || navigator.platform || '';
    var s = (platform + ' ' + navigator.userAgent).toLowerCase();
    if (/android|iphone|ipad|ipod/.test(s)) return 'web';
    if (/windows|win32|win64/.test(s)) return 'windows';
    if (/macintosh|macintel|macos|mac os/.test(s)) return 'macos';
    if (/linux|x11|cros/.test(s)) return 'linux';
    return '';
  }
  var NAMES = { windows: 'Windows', macos: 'macOS', linux: 'Linux' };
  var os = detectOS();
  var current = os && document.querySelector('.dl[data-os="' + os + '"]');
  if (current) current.classList.add('is-current');
  var heroCta = document.querySelector('[data-download-cta]');
  if (heroCta && NAMES[os]) heroCta.textContent = 'Download for ' + NAMES[os];

  // ---- latest release from GitHub ----
  function megabytes(bytes) {
    return bytes ? ' (' + (bytes / 1048576).toFixed(bytes > 10485760 ? 0 : 1) + ' MB)' : '';
  }
  function classify(asset) {
    var n = asset.name || '';
    if (/sha256|third_party/i.test(n)) return null;
    if (/\.msi$/i.test(n)) return { os: 'windows', label: 'Installer (.msi)', rank: 0 };
    if (/win.*\.zip$/i.test(n)) return { os: 'windows', label: 'Portable (.zip)', rank: 1 };
    if (/\.dmg$/i.test(n)) return { os: 'macos', label: 'Disk image (.dmg)', rank: 0 };
    if (/\.appimage$/i.test(n)) return { os: 'linux', label: 'AppImage', rank: 0 };
    if (/\.deb$/i.test(n)) return { os: 'linux', label: '.deb (Debian, Ubuntu)', rank: 1 };
    if (/\.rpm$/i.test(n)) return { os: 'linux', label: '.rpm (Fedora, openSUSE)', rank: 2 };
    if (/\.tar\.gz$/i.test(n) && !/source/i.test(n)) return { os: 'linux', label: '.tar.gz', rank: 3 };
    return null;
  }
  function render(release) {
    // Two shapes are accepted: /release.json (written by the Release workflow: tag, assets[].url) and
    // GitHub's API (tag_name, assets[].browser_download_url).
    if (!release || !release.assets) return;
    var tagName = release.tag || release.tag_name;
    var groups = { windows: [], macos: [], linux: [] };
    release.assets.forEach(function (a) {
      var c = classify(a);
      var url = a.url || a.browser_download_url || '';
      if (c && url.indexOf('https://github.com/' + REPO + '/') === 0) {
        groups[c.os].push({ label: c.label, rank: c.rank, url: url, size: a.size });
      }
    });
    Object.keys(groups).forEach(function (key) {
      var list = groups[key].sort(function (a, b) { return a.rank - b.rank; });
      var box = document.querySelector('.dl[data-os="' + key + '"] [data-links]');
      if (!box || !list.length) return;
      while (box.firstChild) box.removeChild(box.firstChild);
      list.forEach(function (item, i) {
        var a = document.createElement('a');
        a.className = 'btn ' + (i === 0 ? 'btn-primary' : 'btn-ghost');
        a.href = item.url;
        a.rel = 'noopener';
        a.textContent = item.label + megabytes(item.size);
        box.appendChild(a);
      });
    });
    var label = document.querySelector('[data-release-version]');
    if (label && tagName) {
      var when = release.published_at ? new Date(release.published_at) : null;
      label.textContent = 'Latest version: ' + tagName +
        (when && !isNaN(when) ? ', released ' + when.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' }) : '') + '.';
    }
  }
  function fromJson(url, options) {
    return fetch(url, options).then(function (r) {
      if (!r.ok) throw new Error('HTTP ' + r.status);
      return r.json();
    });
  }
  function load() {
    // 1. the file the Release workflow put on this very site (instant, no rate limit);
    // 2. otherwise ask GitHub (remembered for 10 minutes in this browser tab).
    return fromJson('/release.json', { cache: 'no-cache' }).catch(function () {
      var KEY = 'khata-latest-release';
      try {
        var cached = JSON.parse(sessionStorage.getItem(KEY) || 'null');
        if (cached && Date.now() - cached.t < 600000) return cached.data;
      } catch (e) { /* storage may be unavailable */ }
      return fromJson('https://api.github.com/repos/' + REPO + '/releases/latest', { headers: { Accept: 'application/vnd.github+json' } })
        .then(function (data) {
          try { sessionStorage.setItem(KEY, JSON.stringify({ t: Date.now(), data: data })); } catch (e) { /* ignore */ }
          return data;
        });
    });
  }
  if (window.fetch) load().then(render).catch(function () { /* keep the default links */ });
})();
