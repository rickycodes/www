import { chromium } from 'playwright'

const port = process.env.SERVER_PORT || '8765'
const url = `http://127.0.0.1:${port}`
const browser = await chromium.launch({
  headless: true,
  executablePath: process.env.PLAYWRIGHT_EXECUTABLE_PATH || undefined
})
const page = await browser.newPage()
const pageErrors = []
const consoleErrors = []
const failedRequests = []

const diagnostics = () => [...pageErrors, ...consoleErrors, ...failedRequests]

page.on('pageerror', error => pageErrors.push(error.message))
page.on('console', message => {
  if (message.type() === 'error') consoleErrors.push(message.text())
})
page.on('requestfailed', request => {
  failedRequests.push(`${request.method()} ${request.url()}: ${request.failure()?.errorText}`)
})
try {
  await page.goto(url, { waitUntil: 'domcontentloaded' })
  await page.waitForFunction(() => {
    const year = document.querySelector('.year')
    return year && year.textContent.length === 4
  })

  const buildMeta = page.locator('.build-meta')
  await page.waitForFunction(() => {
    const meta = document.querySelector('.build-meta')
    return meta && meta.textContent.startsWith('Deployed: ') && meta.querySelector('a[href*="/commit/"]')
  })
  const buildMetaText = await buildMeta.textContent()
  const buildMetaLink = buildMeta.locator('a[href*="/commit/"]')
  const buildMetaHref = await buildMetaLink.getAttribute('href')
  const buildMetaSha = buildMetaHref.split('/').pop()
  const metadata = await page.evaluate(async () => (await fetch('build-meta.json')).json())
  const expectedBuildDate = await page.evaluate(isoDate => {
    const date = new Date(isoDate)
    const parts = new Intl.DateTimeFormat('en-CA', {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
      timeZone: 'America/Toronto'
    }).formatToParts(date)
    const getPart = type => parts.find(part => part.type === type).value
    const day = Number(getPart('day'))
    const lastTwo = day % 100
    const suffix = lastTwo >= 11 && lastTwo <= 13
      ? 'th'
      : ({ 1: 'st', 2: 'nd', 3: 'rd' }[day % 10] || 'th')
    const time = new Intl.DateTimeFormat('en-CA', {
      timeStyle: 'short',
      timeZone: 'America/Toronto'
    }).format(date)
    return `${getPart('month')} ${day}${suffix}, ${getPart('year')}, ${time} ET`
  }, metadata.built_at_utc)
  if (buildMetaSha !== metadata.git_sha) throw new Error('build metadata commit link did not match build-meta.json')
  if (buildMetaText !== `Deployed: ${expectedBuildDate},${metadata.git_sha}.`) {
    throw new Error(`build metadata output was unexpected: ${buildMetaText}`)
  }
  if (await buildMetaLink.getAttribute('target') !== '_blank') throw new Error('build metadata commit link target missing')

  const themeToggle = page.locator('.theme-toggle')
  const beforeTheme = await themeToggle.getAttribute('aria-pressed')
  await themeToggle.click()
  const afterTheme = await themeToggle.getAttribute('aria-pressed')
  if (beforeTheme === afterTheme) throw new Error('theme toggle did not update aria-pressed')

  const projectLink = page.locator('._projects .project.link').first()
  const projectHref = await projectLink.getAttribute('href')
  const projectHash = projectHref.slice(1)
  const projectImage = page.locator(`.projects .project.${projectHash} img[data-src]`).first()
  const imageSource = await projectImage.getAttribute('data-src')
  if (!imageSource) throw new Error('first project has no lazy image to preload')
  await projectLink.hover()
  await page.waitForFunction(({ selector, source }) => {
    const image = document.querySelector(selector)
    return image && !image.hasAttribute('data-src') && image.getAttribute('src') === source
  }, { selector: `.projects .project.${projectHash} img`, source: imageSource })

  await projectLink.click()
  await page.waitForFunction(hash => document.body.getAttribute('data-project') === hash, projectHash)

  await page.keyboard.press('Escape')
  await page.waitForFunction(() => !document.body.hasAttribute('data-project'))

  const focusedProjectLink = page.locator('._projects .project.link').nth(1)
  const focusedProjectHash = (await focusedProjectLink.getAttribute('href')).slice(1)
  const focusedProjectImage = page.locator(`.projects .project.${focusedProjectHash} img[data-src]`).first()
  const focusedImageSource = await focusedProjectImage.getAttribute('data-src')
  if (!focusedImageSource) throw new Error('second project has no lazy image to preload')
  await focusedProjectLink.focus()
  await page.waitForFunction(({ selector, source }) => {
    const image = document.querySelector(selector)
    return image && !image.hasAttribute('data-src') && image.getAttribute('src') === source
  }, { selector: `.projects .project.${focusedProjectHash} img`, source: focusedImageSource })

  const failedMetadataPage = await browser.newPage()
  await failedMetadataPage.route('**/build-meta.json', route => route.abort())
  await failedMetadataPage.goto(`${url}/${projectHref}`, { waitUntil: 'domcontentloaded' })
  await failedMetadataPage.waitForFunction(() => {
    const year = document.querySelector('.year')
    const meta = document.querySelector('.build-meta')
    const project = document.body.getAttribute('data-project')
    const image = document.querySelector(`.projects .project.${project} img`)
    return year && year.textContent.length === 4 && meta?.textContent === 'Deployed: unknown' &&
      project && image && !image.hasAttribute('data-src')
  }, projectHash)
  await failedMetadataPage.close()

  const observerFallbackPage = await browser.newPage()
  await observerFallbackPage.addInitScript(() => {
    Object.defineProperty(window, 'IntersectionObserver', { configurable: true, value: undefined })
    document.addEventListener('DOMContentLoaded', () => {
      const image = document.createElement('img')
      image.id = 'browser-smoke-lazy-image'
      image.setAttribute('src', '#')
      image.setAttribute('data-src', 'data:image/gif;base64,R0lGODlhAQABAAD/ACwAAAAAAQABAAACADs=')
      document.body.appendChild(image)
    })
  })
  await observerFallbackPage.goto(url, { waitUntil: 'domcontentloaded' })
  await observerFallbackPage.waitForFunction(() => {
    const year = document.querySelector('.year')
    const image = document.querySelector('#browser-smoke-lazy-image')
    return year && year.textContent.length === 4 && image && !image.hasAttribute('data-src')
  })
  await observerFallbackPage.close()
} catch (error) {
  const details = diagnostics()
  throw new Error(`${error.message}${details.length ? `\n${details.join('\n')}` : ''}`)
} finally {
  await browser.close()
}

if (pageErrors.length > 0) throw new Error(pageErrors.join(' | '))

console.log('Browser smoke test passed: Wasm initialized, metadata rendered, images preloaded, and project/theme interactions worked.')
