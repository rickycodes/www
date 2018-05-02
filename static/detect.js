// WebAssembly Feature Detection
(function () {
  if (window.hasOwnProperty('WebAssembly')) {
    var script = document.createElement('script')
    script.setAttribute('src', 'rickycodes.js')
    document.body.appendChild(script)
  } else {
    // womp womp
    var els = ['._projects', '.projects']
    els.forEach(function(selector) {
      var el = document.querySelector(selector)
      el.style.display = 'none'
    })
  }
})()
