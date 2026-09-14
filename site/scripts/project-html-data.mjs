export function extractAttribute(tag, attributeName) {
  const pattern = new RegExp(`${attributeName}=["']([^"']*)["']`, 'i');
  const match = tag.match(pattern);

  return match?.[1]?.trim() ?? '';
}

export function extractTags(html, tagName) {
  return html.match(new RegExp(`<${tagName}(?:\\s|>)[^>]*>`, 'gi')) ?? [];
}

export function extractHtmlPageData(html) {
  const htmlTag = html.match(/<html(?:\s|>)[^>]*>/i)?.[0] ?? '';
  const title = html.match(/<title>([^<]*)<\/title>/i)?.[1]?.trim() ?? '';

  const headings = [...html.matchAll(/<h([1-6])\b[^>]*>(.*?)<\/h\1>/gis)].map((match) => ({
    level: Number(match[1]),
    text: match[2].replace(/<[^>]+>/g, '').trim(),
  }));

  const anchors = extractTags(html, 'a').map((tag) => ({
    tag,
    href: extractAttribute(tag, 'href'),
  }));

  const links = extractTags(html, 'link').map((tag) => ({
    tag,
    rel: extractAttribute(tag, 'rel'),
    href: extractAttribute(tag, 'href'),
  }));

  const metas = extractTags(html, 'meta').map((tag) => ({
    tag,
    name: extractAttribute(tag, 'name'),
    property: extractAttribute(tag, 'property'),
    content: extractAttribute(tag, 'content'),
  }));

  return {
    htmlLang: extractAttribute(htmlTag, 'lang'),
    title,
    headings,
    anchors,
    links,
    metas,
  };
}

export function htmlRelativePathToRoute(relativePath) {
  if (relativePath === 'index.html') {
    return '/';
  }

  if (!relativePath.endsWith('/index.html')) {
    return null;
  }

  return `/${relativePath.slice(0, -'index.html'.length)}`;
}
