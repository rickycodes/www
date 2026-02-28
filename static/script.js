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
    var formatBuiltAt = function (isoString) {
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
        var details = []
        var builtAt = formatBuiltAt(meta.built_at_utc)
        if (meta.git_sha) details.push(meta.git_sha)
        if (meta.runner_os && meta.runner_arch) details.push(meta.runner_os + '/' + meta.runner_arch)
        if (meta.cpu_cores) details.push(meta.cpu_cores + ' cores')

        if (builtAt && details.length > 0) {
          el.innerHTML = 'Built: ' + builtAt + ',<br>' + details.join(' • ') + '.'
        } else if (builtAt) {
          el.textContent = 'Built: ' + builtAt + '.'
        } else if (details.length > 0) {
          el.textContent = 'Built: ' + details.join(' • ') + '.'
        } else {
          el.textContent = 'Built: unknown'
        }
      })
      .catch(function () {
        el.textContent = 'Built: unknown'
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
