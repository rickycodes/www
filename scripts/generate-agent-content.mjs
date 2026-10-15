import { readFile, readdir, writeFile } from 'node:fs/promises';
import TurndownService from 'turndown';

const baseUrl = new URL(process.argv[2] || 'https://ricky.codes/');
const partials = new URL('../src/partials/', import.meta.url);
const output = new URL('../static/', import.meta.url);
const markdown = new TurndownService({ headingStyle: 'atx', bulletListMarker: '-' });

// Resolve URLs against the HTML page, including its interactive project anchors.
markdown.addRule('absoluteLinks', {
  filter: (node) => node.nodeName === 'A' && node.hasAttribute('href'),
  replacement: (content, node) =>
    `[${content}](<${new URL(node.getAttribute('href'), baseUrl).href}>)`,
});
markdown.addRule('lazyImages', {
  filter: 'img',
  replacement: (_content, node) => {
    const src = node.getAttribute('data-src') || node.getAttribute('src');
    const alt = markdown.escape(node.getAttribute('alt') || '');
    return src ? `![${alt}](<${new URL(src, baseUrl).href}>)` : '';
  },
});
markdown.addRule('workHistoryHeading', {
  filter: 'summary',
  replacement: (content) => `\n\n## ${content}\n\n`,
});
markdown.addRule('definitionTerms', {
  filter: 'dt',
  replacement: (content) => `\n\n**${content}**\n\n`,
});
markdown.addRule('omitControls', {
  filter: (node) => (node.getAttribute('class') || '').split(/\s+/).includes('close-link'),
  replacement: () => '',
});

// Read the content partials directly so hidden dialogs and collapsed work history
// remain available without executing JavaScript or maintaining a second biography.
const projects = (await readdir(new URL('projects/', partials)))
  .filter((name) => name.endsWith('.html')).sort();
const files = ['about.html', 'cv.html', ...projects.map((name) => `projects/${name}`), 'colophon.html'];
const sections = await Promise.all(files.map(async (name) =>
  markdown.turndown(await readFile(new URL(name, partials), 'utf8'))));
await writeFile(new URL('index.md', output), `${sections.join('\n\n')}\n`);

await writeFile(new URL('llms.txt', output), `# Ricky Miller

> Personal website and portfolio of Ricky Miller, a software developer based in Ottawa, Ontario, Canada.

## Website

- [Full website content](${new URL('index.md', baseUrl)}): Markdown generated from the same HTML partials as the website, including biography, work history, project descriptions, screenshots, links, and colophon.
- [Homepage](${baseUrl}): Interactive HTML version of the portfolio.
- [CV](${new URL('cv.pdf', baseUrl)}): Downloadable PDF résumé.

## Optional

- [GitHub](https://github.com/rickycodes): Public software projects and contributions.
- [Website source](https://github.com/rickycodes/www): Source code and build tooling.

## Access and reuse

- [Robots policy](${new URL('robots.txt', baseUrl)}): Search and assistant access is allowed; named model-training crawlers are disallowed.
- The site permits AI input and search while disallowing AI training, as declared by its robots.txt content signal.
`);
console.log('Generated static/index.md and static/llms.txt');
