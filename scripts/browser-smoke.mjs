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
  const themeToggle = page.locator('.theme-toggle')
  const beforeTheme = await themeToggle.getAttribute('aria-pressed')
  await themeToggle.click()
  const afterTheme = await themeToggle.getAttribute('aria-pressed')
  if (beforeTheme === afterTheme) throw new Error('theme toggle did not update aria-pressed')

  const projectLink = page.locator('._projects .project.link').first()
  const projectHash = (await projectLink.getAttribute('href')).slice(1)
  await projectLink.click()
  await page.waitForFunction(hash => document.body.getAttribute('data-project') === hash, projectHash)

  await page.keyboard.press('Escape')
  await page.waitForFunction(() => !document.body.hasAttribute('data-project'))
} catch (error) {
  const details = diagnostics()
  throw new Error(`${error.message}${details.length ? `\n${details.join('\n')}` : ''}`)
} finally {
  await browser.close()
}

if (pageErrors.length > 0) throw new Error(pageErrors.join(' | '))

console.log('Browser smoke test passed: Wasm initialized, theme toggled, project opened and closed.')
