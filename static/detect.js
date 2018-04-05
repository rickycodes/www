// WebAssembly Feature Detection
(function () {
  if (window.hasOwnProperty('WebAssembly')) {
    var s = document.createElement('script')
    s.setAttribute('src', 'rickycodes.js')
    document.body.appendChild(s)
  } else {
    // womp womp
    document.querySelector('._projects').style.display = 'none'
  }
})()
