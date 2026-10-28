import { expect, test } from 'playwright/test'

const port = process.env.SERVER_PORT || '8765'
const url = `http://127.0.0.1:${port}`

test.use({
  launchOptions: {
    executablePath: process.env.PLAYWRIGHT_EXECUTABLE_PATH || undefined
  }
})

function trackPageErrors(page) {
  const errors = []
  page.on('pageerror', error => errors.push(error.message))
  return () => expect(errors, 'page should not raise uncaught JavaScript errors').toEqual([])
}

async function expectedBuildDate(page, isoDate) {
  return page.evaluate(value => {
    const date = new Date(value)
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
  }, isoDate)
}

test('renders build metadata and its commit link', async ({ page }) => {
  const expectNoPageErrors = trackPageErrors(page)
  await page.goto(url, { waitUntil: 'domcontentloaded' })
  await expect(page.locator('.year'), 'WebAssembly should set the current year')
    .toHaveText(/^\d{4}$/)

  const buildMeta = page.locator('.build-meta')
  await expect(buildMeta, 'build metadata should render deployment details')
    .toContainText('Deployed: ')
  const buildMetaLink = buildMeta.locator('a[href*="/commit/"]')
  await expect(buildMetaLink, 'build metadata should link to its commit')
    .toHaveAttribute('href', /^https:\/\/github\.com\/rickycodes\/www\/commit\/.+/)
  const buildMetaHref = await buildMetaLink.getAttribute('href')
  const metadata = await page.evaluate(async () => (await fetch('build-meta.json')).json())
  const expectedDate = await expectedBuildDate(page, metadata.built_at_utc)

  expect(buildMetaHref.split('/').pop(), 'commit link should match build-meta.json')
    .toBe(metadata.git_sha)
  expect(await buildMeta.textContent(), 'metadata should preserve its date and punctuation')
    .toBe(`Deployed: ${expectedDate},${metadata.git_sha}.`)
  await expect(buildMetaLink, 'commit link should open in a new tab')
    .toHaveAttribute('target', '_blank')
  expectNoPageErrors()
})

test('uses a turquoise light-mode toggle with a grayscale icon', async ({ page }) => {
  const expectNoPageErrors = trackPageErrors(page)
  await page.goto(url, { waitUntil: 'domcontentloaded' })
  await expect(page.locator('.year'), 'WebAssembly should initialize before testing the toggle')
    .toHaveText(/^\d{4}$/)
  const themeToggle = page.locator('.theme-toggle')
  await expect(themeToggle, 'theme toggle should be present').toBeVisible()
  await page.evaluate(() => document.documentElement.setAttribute('data-theme', 'light'))

  const toggleColors = await themeToggle.evaluate(element => {
    const style = getComputedStyle(element)
    const icon = getComputedStyle(element.querySelector('.theme-toggle-icon'))
    return { background: style.backgroundColor, filter: style.filter, iconFilter: icon.filter }
  })
  expect(toggleColors.background, 'light toggle should use the gradient turquoise')
    .toBe('rgb(217, 255, 255)')
  expect(toggleColors.filter, 'the toggle background should not be grayscale-filtered')
    .toBe('none')
  expect(toggleColors.iconFilter, 'the toggle icon should remain grayscale')
    .toBe('grayscale(1)')

  const beforeTheme = await themeToggle.getAttribute('aria-pressed')
  await themeToggle.click()
  await expect(themeToggle, 'clicking the toggle should change its pressed state')
    .not.toHaveAttribute('aria-pressed', beforeTheme)
  expectNoPageErrors()
})

test('preloads project images on hover and focus and opens and closes projects', async ({ page }) => {
  const expectNoPageErrors = trackPageErrors(page)
  await page.goto(url, { waitUntil: 'domcontentloaded' })
  await expect(page.locator('.year'), 'WebAssembly should initialize before project interactions')
    .toHaveText(/^\d{4}$/)

  const projectLink = page.locator('._projects .project.link').first()
  const projectHref = await projectLink.getAttribute('href')
  expect(projectHref, 'first project link should have a hash target').toMatch(/^#.+/)
  const projectHash = projectHref.slice(1)
  const projectImage = page.locator(`.projects .project.${projectHash} img[data-src]`).first()
  const imageSource = await projectImage.getAttribute('data-src')
  expect(imageSource, 'first project should contain a lazy image').toBeTruthy()

  await projectLink.hover()
  const preloadedImage = page.locator(`.projects .project.${projectHash} img`).first()
  await expect(preloadedImage, 'hovering a project link should load its image')
    .not.toHaveAttribute('data-src')
  await expect(preloadedImage, 'preloaded image should use its data-src URL')
    .toHaveAttribute('src', imageSource)

  await projectLink.click()
  await expect(page.locator('body'), 'clicking a project link should open its project')
    .toHaveAttribute('data-project', projectHash)
  await page.keyboard.press('Escape')
  await expect(page.locator('body'), 'Escape should close the open project')
    .not.toHaveAttribute('data-project')

  const focusedProjectLink = page.locator('._projects .project.link').nth(1)
  const focusedHash = (await focusedProjectLink.getAttribute('href')).slice(1)
  const focusedImage = page.locator(`.projects .project.${focusedHash} img[data-src]`).first()
  const focusedImageSource = await focusedImage.getAttribute('data-src')
  expect(focusedImageSource, 'second project should contain a lazy image').toBeTruthy()
  await focusedProjectLink.focus()
  const preloadedFocusedImage = page.locator(`.projects .project.${focusedHash} img`).first()
  await expect(preloadedFocusedImage, 'focusing a project link should load its image')
    .not.toHaveAttribute('data-src')
  await expect(preloadedFocusedImage, 'focused preview should use its data-src URL')
    .toHaveAttribute('src', focusedImageSource)
  expectNoPageErrors()
})

test('handles metadata request failure and opens projects from the URL hash', async ({ page }) => {
  const expectNoPageErrors = trackPageErrors(page)
  const projectHash = 'metamask'
  await page.route('**/build-meta.json', route => route.abort())
  await page.goto(`${url}/#${projectHash}`, { waitUntil: 'domcontentloaded' })

  await expect(page.locator('.year'), 'WebAssembly should still initialize after metadata failure')
    .toHaveText(/^\d{4}$/)
  await expect(page.locator('.build-meta'), 'failed metadata request should show the fallback')
    .toHaveText('Deployed: unknown')
  await expect(page.locator('body'), 'URL hash should open the matching project at startup')
    .toHaveAttribute('data-project', projectHash)
  await expect(page.locator(`.projects .project.${projectHash} img`).first(), 'initial project should preload its image')
    .not.toHaveAttribute('data-src')
  expectNoPageErrors()
})

test('eagerly loads lazy images when IntersectionObserver is unavailable', async ({ page }) => {
  const expectNoPageErrors = trackPageErrors(page)
  await page.addInitScript(() => {
    Object.defineProperty(window, 'IntersectionObserver', { configurable: true, value: undefined })
    document.addEventListener('DOMContentLoaded', () => {
      const image = document.createElement('img')
      image.id = 'browser-smoke-lazy-image'
      image.setAttribute('src', '#')
      image.setAttribute('data-src', 'data:image/gif;base64,R0lGODlhAQABAAD/ACwAAAAAAQABAAACADs=')
      document.body.appendChild(image)
    })
  })
  await page.goto(url, { waitUntil: 'domcontentloaded' })

  await expect(page.locator('.year'), 'WebAssembly should initialize without IntersectionObserver')
    .toHaveText(/^\d{4}$/)
  await expect(page.locator('#browser-smoke-lazy-image'), 'fallback should load the image eagerly')
    .not.toHaveAttribute('data-src')
  expectNoPageErrors()
})
