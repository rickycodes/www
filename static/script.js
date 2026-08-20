/* global IntersectionObserver */
(function () {
  const getBuildVersion = function () {
    const current = document.currentScript
    if (!current) return null
    const src = current.getAttribute('src') || ''
    const match = src.match(/[?&]v=([^&]+)/)
    return match ? match[1] : null
  }

  const buildVersion = getBuildVersion()
  window.__BUILD_VERSION = buildVersion

  const withBuildVersion = function (url) {
    if (!buildVersion) return url
    if (/^https?:\/\//.test(url)) return url
    return url + (url.indexOf('?') === -1 ? '?v=' : '&v=') + buildVersion
  }

  const lazyLoadImages = function () {
    const lazyImages = [].slice.call(document.querySelectorAll('img[data-src]'))
    if ('IntersectionObserver' in window) {
      const lazyImageObserver = new IntersectionObserver(function (entries, observer) {
        entries.forEach(function (entry) {
          if (entry.isIntersecting) {
            const lazyImage = entry.target
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

  const attachScript = function (source) {
    const script = document.createElement('script')
    script.setAttribute('src', source.src)
    'type' in source && script.setAttribute('type', source.type)
    document.body.appendChild(script)
  }

  const attachScripts = function () {
    const scripts = [
      { src: withBuildVersion('detect.js') },
      { type: 'async', src: 'https://www.googletagmanager.com/gtag/js?id=UA-71959023-1' }
    ]

    scripts.forEach(attachScript)
  }

  const setupGTag = function () {
    window.dataLayer = window.dataLayer || []
    function gtag () { window.dataLayer.push(arguments) }
    gtag('js', new Date())
    gtag('config', 'UA-71959023-1')
  }

  const renderBuildMeta = function () {
    const el = document.querySelector('.build-meta')
    if (!el) return
    const DETAILS_PREFIX = 'Deployed: '
    const COMMIT_BASE_URL = 'https://github.com/rickycodes/www/commit/'

    const buildDetails = function (meta) {
      const items = []

      if (meta.git_sha) {
        const sha = meta.git_sha
        items.push({ type: 'link', href: COMMIT_BASE_URL + sha, text: sha })
      }
      if (meta.runner_os && meta.runner_arch) {
        const runner = meta.runner_os + '/' + meta.runner_arch
        items.push({ type: 'text', text: runner })
      }
      if (meta.cpu_cores) {
        const cores = meta.cpu_cores + ' cores'
        items.push({ type: 'text', text: cores })
      }

      return items
    }

    const appendDetails = function (el, items) {
      for (let i = 0; i < items.length; i++) {
        if (i > 0) el.appendChild(document.createTextNode(' • '))
        const item = items[i]
        if (item.type === 'link') {
          const link = document.createElement('a')
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

    const renderMeta = function (el, builtAt, details) {
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

    const ordinalSuffix = function (day) {
      const lastTwo = day % 100
      if (lastTwo >= 11 && lastTwo <= 13) return 'th'
      switch (day % 10) {
        case 1: return 'st'
        case 2: return 'nd'
        case 3: return 'rd'
        default: return 'th'
      }
    }

    const formatShippedAt = function (isoString) {
      if (!isoString) return null
      const date = new Date(isoString)
      if (Number.isNaN(date.getTime())) return isoString

      const dateParts = new Intl.DateTimeFormat('en-CA', {
        day: 'numeric',
        month: 'short',
        year: 'numeric',
        timeZone: 'America/Toronto'
      }).formatToParts(date)
      const getPart = function (type) {
        return dateParts.find(function (part) { return part.type === type }).value
      }
      const day = Number(getPart('day'))
      const time = new Intl.DateTimeFormat('en-CA', {
        timeStyle: 'short',
        timeZone: 'America/Toronto'
      }).format(date) + ' ET'

      return getPart('month') + ' ' + day + ordinalSuffix(day) + ', ' + getPart('year') + ', ' + time
    }

    fetch('build-meta.json', { cache: 'no-store' })
      .then(function (response) {
        if (!response.ok) throw new Error('missing build metadata')
        return response.json()
      })
      .then(function (meta) {
        const builtAt = formatShippedAt(meta.built_at_utc)
        const details = buildDetails(meta)
        renderMeta(el, builtAt, details)
      })
      .catch(function () {
        el.textContent = `${DETAILS_PREFIX}unknown`
      })
  }

  const initialize = function () {
    setupGTag()
    renderBuildMeta()
    lazyLoadImages()
    attachScripts()
  }

  document.addEventListener('DOMContentLoaded', initialize)
})()
