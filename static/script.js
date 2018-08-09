/* global IntersectionObserver */
(function () {
  var loadDeferredStyles = function () {
    var addStylesNode = document.querySelector('.deferred')
    var replacement = document.createElement('div')
    replacement.innerHTML = addStylesNode.textContent
    document.body.appendChild(replacement)
    addStylesNode.parentElement.removeChild(addStylesNode)
  }

  var raf = window.requestAnimationFrame || window.mozRequestAnimationFrame ||
    window.webkitRequestAnimationFrame || window.msRequestAnimationFrame

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

  var init = function () {
    window.dataLayer = window.dataLayer || []
    function gtag () { window.dataLayer.push(arguments) }
    gtag('js', new Date())
    gtag('config', 'UA-71959023-1')
    if (raf) raf(function () { window.setTimeout(loadDeferredStyles, 0) })
    else window.addEventListener('load', loadDeferredStyles)
    lazyLoadImages()
  }

  document.addEventListener('DOMContentLoaded', init)
})()
