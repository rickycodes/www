// WebAssembly Feature Detection
(function () {
  if ('WebAssembly' in window) {
    var script = document.createElement('script')
    script.setAttribute('src', 'rickycodes.js')
    document.body.appendChild(script)
  } else {
    // womp womp
    var els = ['._projects', '.projects', '.coord']
    els.forEach(function (selector) {
      var el = document.querySelector(selector)
      el.style.display = 'none'
    })
  }
})()
