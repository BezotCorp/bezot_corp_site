import { useEffect } from "react";
import { Link } from "react-router-dom";
import { useSite } from "../application/use-site";
import { MainLayout } from "../layout/MainLayout";
import { applySeo } from "../seo";

type Props = {
  locale: string;
};

export function NotFoundPage({ locale }: Props) {
  const site = useSite();
  const isFrench = locale === "fr-fr";

  useEffect(() => {
    applySeo(site.getNotFoundSeo(locale), site.metadata);
  }, [locale, site]);

  return (
    <MainLayout>
      <main className="page">
        <section>
          <h1>{isFrench ? "Page introuvable" : "Page not found"}</h1>
          <p>
            {isFrench
              ? "Cette URL ne correspond à aucune page publiée."
              : "This URL does not match any published page."}
          </p>
          <p>
            <Link to={`/${locale}`}>
              {isFrench ? "Retour à l'accueil" : "Back to home"}
            </Link>
          </p>
        </section>
      </main>
    </MainLayout>
  );
}
