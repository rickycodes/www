const featureKey = 'WebAssembly';
const ASCII = 'cat.txt';
const src = 'rickycodes.js';

const options = [
  "It is certain.",
  "It is decidedly so.",
  "Without a doubt.",
  "Yes - definitely.",
  "You may rely on it.",
  "As I see it, yes.",
  "Most likely.",
  "Outlook good.",
  "Yes.",
  "Signs point to yes.",
  "Reply hazy, try again.",
  "Ask again later.",
  "Better not tell you now.",
  "Cannot predict now.",
  "Concentrate and ask again.",
  "Don't count on it.",
  "My reply is no.",
  "My sources say no.",
  "Outlook not so good.",
  "Very doubtful."
];

const ask = q => console.log(options[Math.floor(Math.random() * options.length)]);

// WebAssembly Feature Detection
(async function () {
  if (featureKey in window) {
    var script = document.createElement('script')
    await script.setAttribute('src', src)
    await fetch(ASCII)
      .then(response => response.text())
      .then(data => console.log(data))
    await document.body.appendChild(script)
  } else {
    // womp womp
    var els = ['._projects', '.projects', '.coord']
    els.forEach(function (selector) {
      var el = document.querySelector(selector)
      el.style.display = 'none'
    })
  }
})();
