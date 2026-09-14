import { relativeFrom } from './project-file-utils.mjs';

function countErrors(resources) {
  return resources.filter((resource) => resource?.error).length;
}

export function printProjectStructuredDataReport(projectData) {
  const contentResources = [
    projectData.content.index,
    projectData.content.pages.index,
    ...projectData.content.pages.pageIndexes,
    ...projectData.content.pages.pageLocales,
    projectData.content.blog.index,
    ...projectData.content.blog.posts,
    projectData.content.redirects,
    projectData.content.goneRoutes,
    projectData.content.invariants,
    projectData.content.websiteMetadata,
  ].filter(Boolean);

  const distResources = [
    ...projectData.dist.htmlFiles,
    projectData.dist.sitemap,
  ].filter(Boolean);

  console.log('Project structured data report');
  console.log('');

  console.log('content');
  console.log(`- exists: ${projectData.content.exists ? 'yes' : 'no'}`);
  console.log(`- files: ${projectData.content.files.length}`);
  console.log(`- json files: ${projectData.content.jsonFiles.length}`);
  console.log(`- content index: ${projectData.content.index.exists ? 'found' : 'missing'}`);
  console.log(`- pages index: ${projectData.content.pages.index?.exists ? 'found' : 'missing'}`);
  console.log(`- page indexes: ${projectData.content.pages.pageIndexes.length}`);
  console.log(`- page locales: ${projectData.content.pages.pageLocales.length}`);
  console.log(`- blog index: ${projectData.content.blog.index?.exists ? 'found' : 'missing'}`);
  console.log(`- blog posts: ${projectData.content.blog.posts.length}`);
  console.log(`- redirects: ${projectData.content.redirects.exists ? 'found' : 'missing'}`);
  console.log(`- gone routes: ${projectData.content.goneRoutes.exists ? 'found' : 'missing'}`);
  console.log(`- invariants: ${projectData.content.invariants.exists ? 'found' : 'missing'}`);
  console.log(`- website metadata: ${projectData.content.websiteMetadata.exists ? 'found' : 'missing'}`);
  console.log(`- read errors: ${countErrors(contentResources)}`);

  console.log('');

  console.log('dist');
  console.log(`- exists: ${projectData.dist.exists ? 'yes' : 'no'}`);
  console.log(`- files: ${projectData.dist.files.length}`);
  console.log(`- html files: ${projectData.dist.htmlFiles.length}`);
  console.log(`- xml files: ${projectData.dist.xmlFiles.length}`);
  console.log(`- sitemap: ${projectData.dist.sitemap.exists ? 'found' : 'missing'}`);
  console.log(`- sitemap locations: ${projectData.dist.sitemap.locations.length}`);
  console.log(`- read errors: ${countErrors(distResources)}`);

  console.log('');
  console.log('sample html page data');

  const firstHtmlFile = projectData.dist.htmlFiles.find((htmlFile) => htmlFile.route);

  if (firstHtmlFile) {
    console.log(`- file: ${firstHtmlFile.relativePath}`);
    console.log(`- route: ${firstHtmlFile.route ?? 'none'}`);
    console.log(`- lang: ${firstHtmlFile.pageData?.htmlLang ?? 'none'}`);
    console.log(`- title: ${firstHtmlFile.pageData?.title ?? 'none'}`);
    console.log(`- headings: ${firstHtmlFile.pageData?.headings.length ?? 0}`);
    console.log(`- anchors: ${firstHtmlFile.pageData?.anchors.length ?? 0}`);
    console.log(`- metas: ${firstHtmlFile.pageData?.metas.length ?? 0}`);
    console.log(`- links: ${firstHtmlFile.pageData?.links.length ?? 0}`);
  } else {
    console.log('- none');
  }

  console.log('');
  console.log('sample content page locale data');

  const firstPageLocale = projectData.content.pages.pageLocales[0];

  if (firstPageLocale) {
    console.log(`- file: ${firstPageLocale.relativePath}`);
    console.log(`- page id: ${firstPageLocale.pageId}`);
    console.log(`- locale: ${firstPageLocale.locale}`);
    console.log(`- slug: ${firstPageLocale.data?.slug ?? 'none'}`);
    console.log(`- seo title: ${firstPageLocale.data?.seo?.title ?? 'none'}`);
    console.log(`- blocks: ${Array.isArray(firstPageLocale.data?.blocks) ? firstPageLocale.data.blocks.length : 0}`);
  } else {
    console.log('- none');
  }

  const erroredResources = [...contentResources, ...distResources].filter((resource) => resource.error);

  if (erroredResources.length > 0) {
    console.log('');
    console.log('errors');

    for (const resource of erroredResources) {
      console.log(`- ${relativeFrom(projectData.rootDir, resource.filePath)}: ${resource.error}`);
    }
  }
}
