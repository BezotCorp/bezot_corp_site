import { Link, useLocation } from "react-router-dom";
import { useSite } from "../application/use-site";

export function Footer() {
  const site = useSite();
  const location = useLocation();
  const locale = site.getActiveLocale(location.pathname);
  const isFrench = locale === "fr-fr";

  const privacyPath = site.getPagePath("privacy", locale);
  const legalNoticePath = site.getPagePath("legal-notice", locale);

  return (
    <footer className="site-footer">
      <p>© {new Date().getFullYear()} {site.metadata.name}. {isFrench ? "Tous droits réservés." : "All rights reserved."}</p>

      <nav aria-label={isFrench ? "Liens légaux" : "Legal links"}>
        {privacyPath && (
          <Link to={privacyPath}>
            {isFrench ? "Confidentialité" : "Privacy Policy"}
          </Link>
        )}

        {legalNoticePath && (
          <Link to={legalNoticePath}>
            {isFrench ? "Mentions légales" : "Legal Notice"}
          </Link>
        )}
      </nav>
    </footer>
  );
}
