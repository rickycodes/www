// WebAssembly Feature Detection
(function () {
  if (window.hasOwnProperty('WebAssembly')) {
    var script = document.createElement('script')
    script.setAttribute('src', 'rickycodes.js')
    document.body.appendChild(script)
  } else {
    // womp womp
    document.querySelector('._projects').style.display = 'none'
  }
})()
