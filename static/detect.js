const featureKey = 'WebAssembly'
const ASCII = 'cat.txt'
const buildVersion = window.__BUILD_VERSION || null

const withBuildVersion = function (url) {
  return buildVersion ? `${url}?v=${buildVersion}` : url
}

const options = [
  'It is certain.',
  'It is decidedly so.',
  'Without a doubt.',
  'Yes - definitely.',
  'You may rely on it.',
  'As I see it, yes.',
  'Most likely.',
  'Outlook good.',
  'Yes.',
  'Signs point to yes.',
  'Reply hazy, try again.',
  'Ask again later.',
  'Better not tell you now.',
  'Cannot predict now.',
  'Concentrate and ask again.',
  "Don't count on it.",
  'My reply is no.',
  'My sources say no.',
  'Outlook not so good.',
  'Very doubtful.'
]

window.ask = function () {
  console.log(options[Math.floor(Math.random() * options.length)])
}

const hideInteractiveElements = function () {
  const selectors = ['._projects', '.projects', '.coord']
  selectors.forEach(function (selector) {
    const element = document.querySelector(selector)
    if (element) element.style.display = 'none'
  })
}

if (featureKey in window) {
  try {
    const response = await fetch(withBuildVersion(ASCII))
    console.log(await response.text())

    const module = await import(withBuildVersion('./rickycodes.js'))
    await module.default({ module_or_path: withBuildVersion('./rickycodes_bg.wasm') })
  } catch (error) {
    console.error('Unable to initialize the WebAssembly application.', error)
    hideInteractiveElements()
  }
} else {
  hideInteractiveElements()
}
