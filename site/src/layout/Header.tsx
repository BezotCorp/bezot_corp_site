import { Link, useLocation } from 'react-router-dom';
import type { SiteRuntime } from '../application/site-contract';
import { useSite } from '../application/use-site';

function getAlternatePath(site: SiteRuntime, pathname: string, targetLocale: string) {
  const route = site.resolveRoute(pathname);

  if (route.kind === 'page') {
    return route.seo.alternates.find((alternate) => alternate.locale === targetLocale)?.path ?? site.getPagePath('home', targetLocale);
  }

  return site.getPagePath('home', targetLocale);
}

export function Header() {
  const site = useSite();
  const location = useLocation();
  const locale = site.getActiveLocale(location.pathname);
  const isFrench = locale === 'fr-fr';

  const homePath = site.getPagePath('home', locale);
  const projectsPath = site.getPagePath('projects', locale);
  const blogPath = site.getPagePath('blog', locale);
  const contactPath = site.getPagePath('contact', locale);

  const frPath = getAlternatePath(site, location.pathname, 'fr-fr') ?? '/fr-fr/';
  const enPath = getAlternatePath(site, location.pathname, 'en-us') ?? '/en-us/';

  return (
    <header className="site-header">
      {homePath && (
        <Link to={homePath} className="brand">
          Bezot Corp
        </Link>
      )}

      <nav aria-label={isFrench ? 'Navigation principale' : 'Main navigation'}>
        {homePath && <Link to={homePath}>{isFrench ? 'Accueil' : 'Home'}</Link>}
        {projectsPath && <Link to={projectsPath}>{isFrench ? 'Projets' : 'Projects'}</Link>}
        {blogPath && <Link to={blogPath}>Blog</Link>}
        {contactPath && <Link to={contactPath}>Contact</Link>}
      </nav>

      <div className="locale-switch" aria-label={isFrench ? 'Changer de langue' : 'Change language'}>
        <Link to={frPath} lang="fr" aria-current={locale === 'fr-fr' ? 'page' : undefined}>
          FR
        </Link>
        <Link to={enPath} lang="en" aria-current={locale === 'en-us' ? 'page' : undefined}>
          EN
        </Link>
      </div>
    </header>
  );
}
