import { Link, useLocation } from 'react-router-dom';
import { useSite } from '../application/use-site';
import { NotFoundPage } from './NotFoundPage';
import { PageTemplate } from '../templates/PageTemplate';

function RootLanguagePage() {
  const site = useSite();

  return (
    <main>
      <h1>{site.metadata.name}</h1>
      <p>Choose your language.</p>

      <nav aria-label="Language selection">
        {site.metadata.locales.map((locale, index) => (
          <span key={locale}>
            {index > 0 ? ' | ' : null}
            <Link to={`/${locale}/`}>
              {locale === 'fr-fr' ? 'Français' : locale === 'en-us' ? 'English' : locale}
            </Link>
          </span>
        ))}
      </nav>
    </main>
  );
}

export function PageRenderer() {
  const site = useSite();
  const location = useLocation();
  const match = site.resolveRoute(location.pathname);

  if (match.kind === 'root') {
    return <RootLanguagePage />;
  }

  if (match.kind !== 'page') {
    return <NotFoundPage locale={match.locale} />;
  }

  const blocks = site.isPostEntry(match.entry)
    ? [
        match.content.blocks[0],
        {
          type: 'post_meta',
          props: {
            author: match.entry.author,
            publishedAt: match.entry.publishedAt,
            updatedAt: match.entry.updatedAt,
            locale: match.locale,
          },
        },
        ...match.content.blocks.slice(1),
      ].filter(Boolean)
    : match.content.blocks;

  return <PageTemplate page={{ ...match.content, blocks }} seo={match.seo} />;
}
