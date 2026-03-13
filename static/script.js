/* global IntersectionObserver */
(function () {
  var getBuildVersion = function () {
    var current = document.currentScript
    if (!current) return null
    var src = current.getAttribute('src') || ''
    var match = src.match(/[?&]v=([^&]+)/)
    return match ? match[1] : null
  }

  var buildVersion = getBuildVersion()
  window.__BUILD_VERSION = buildVersion

  var withBuildVersion = function (url) {
    if (!buildVersion) return url
    if (/^https?:\/\//.test(url)) return url
    return url + (url.indexOf('?') === -1 ? '?v=' : '&v=') + buildVersion
  }

  var lazyLoadImages = function () {
    var lazyImages = [].slice.call(document.querySelectorAll('img[data-src]'))
    if ('IntersectionObserver' in window) {
      let lazyImageObserver = new IntersectionObserver(function (entries, observer) {
        entries.forEach(function (entry) {
          if (entry.isIntersecting) {
            let lazyImage = entry.target
            lazyImage.src = lazyImage.dataset.src
            lazyImageObserver.unobserve(lazyImage)
          }
        })
      })
      lazyImages.forEach(function (img) {
        lazyImageObserver.observe(img)
      })
    } else {
      lazyImages.forEach(function (img) {
        img.setAttribute('src', img.getAttribute('data-src'))
        img.onload = function () {
          this.removeAttribute('data-src')
        }
      })
    }
  }

  var attachScript = function (source) {
    var script = document.createElement('script')
    script.setAttribute('src', source.src)
    'type' in source && script.setAttribute('type', source.type)
    document.body.appendChild(script)
  }

  var attachScripts = function () {
    var scripts = [
      { src: withBuildVersion('detect.js') },
      { type: 'async', src: 'https://www.googletagmanager.com/gtag/js?id=UA-71959023-1' }
    ]

    scripts.forEach(attachScript)
  }

  var setupGTag = function () {
    window.dataLayer = window.dataLayer || []
    function gtag () { window.dataLayer.push(arguments) }
    gtag('js', new Date())
    gtag('config', 'UA-71959023-1')
  }

  var renderBuildMeta = function () {
    var el = document.querySelector('.build-meta')
    if (!el) return
    var DETAILS_PREFIX = 'Shipped: '
    var COMMIT_BASE_URL = 'https://github.com/rickycodes/www/commit/'

    var buildDetails = function (meta) {
      var items = []

      if (meta.git_sha) {
        var sha = meta.git_sha
        items.push({ type: 'link', href: COMMIT_BASE_URL + sha, text: sha })
      }
      if (meta.runner_os && meta.runner_arch) {
        var runner = meta.runner_os + '/' + meta.runner_arch
        items.push({ type: 'text', text: runner })
      }
      if (meta.cpu_cores) {
        var cores = meta.cpu_cores + ' cores'
        items.push({ type: 'text', text: cores })
      }

      return items
    }

    var appendDetails = function (el, items) {
      for (var i = 0; i < items.length; i++) {
        if (i > 0) el.appendChild(document.createTextNode(' • '))
        var item = items[i]
        if (item.type === 'link') {
          var link = document.createElement('a')
          link.href = item.href
          link.target = '_blank'
          link.rel = 'noopener'
          link.textContent = item.text
          el.appendChild(link)
        } else {
          el.appendChild(document.createTextNode(item.text))
        }
      }
    }

    var renderMeta = function (el, builtAt, details) {
      while (el.firstChild) el.removeChild(el.firstChild)

      if (builtAt && details.length > 0) {
        el.appendChild(document.createTextNode(DETAILS_PREFIX + builtAt + ','))
        el.appendChild(document.createElement('br'))
        appendDetails(el, details)
        el.appendChild(document.createTextNode('.'))
        return
      }

      if (builtAt) {
        el.textContent = DETAILS_PREFIX + builtAt + '.'
        return
      }

      if (details.length > 0) {
        el.appendChild(document.createTextNode(DETAILS_PREFIX))
        appendDetails(el, details)
        el.appendChild(document.createTextNode('.'))
        return
      }

      el.textContent = `${DETAILS_PREFIX}unknown`
    }

    var formatShippedAt = function (isoString) {
      if (!isoString) return null
      var date = new Date(isoString)
      if (Number.isNaN(date.getTime())) return isoString
      return new Intl.DateTimeFormat('en-CA', {
        dateStyle: 'medium',
        timeStyle: 'short',
        timeZone: 'America/Toronto'
      }).format(date) + ' ET'
    }

    fetch('build-meta.json', { cache: 'no-store' })
      .then(function (response) {
        if (!response.ok) throw new Error('missing build metadata')
        return response.json()
      })
      .then(function (meta) {
        var builtAt = formatShippedAt(meta.built_at_utc)
        var details = buildDetails(meta)
        renderMeta(el, builtAt, details)
      })
      .catch(function () {
        el.textContent = `${DETAILS_PREFIX}unknown`
      })
  }

  var initialize = function () {
    setupGTag()
    renderBuildMeta()
    lazyLoadImages()
    attachScripts()
  }

  document.addEventListener('DOMContentLoaded', initialize)
})()
