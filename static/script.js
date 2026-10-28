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

  const attachScript = function (source) {
    const script = document.createElement('script')
    script.setAttribute('src', source.src)
    if (source.module) script.setAttribute('type', 'module')
    if (source.async) script.async = true
    document.body.appendChild(script)
  }

  const attachScripts = function () {
    if (!('noModule' in HTMLScriptElement.prototype)) {
      ;['._projects', '.projects', '.coord'].forEach(function (selector) {
        const element = document.querySelector(selector)
        if (element) element.style.display = 'none'
      })
      return
    }

    const scripts = [
      { module: true, src: withBuildVersion('detect.js') },
      { async: true, src: 'https://www.googletagmanager.com/gtag/js?id=UA-71959023-1' }
    ]
    scripts.forEach(attachScript)
  }

  const setupGTag = function () {
    window.dataLayer = window.dataLayer || []
    function gtag () { window.dataLayer.push(arguments) }
    gtag('js', new Date())
    gtag('config', 'UA-71959023-1')
  }

  document.addEventListener('DOMContentLoaded', function () {
    setupGTag()
    attachScripts()
  })
})()
